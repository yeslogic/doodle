use std::cell::RefCell;
use std::fmt::Write;
use std::collections::{HashMap, HashSet};

use anyhow::anyhow;

use crate::output::Fragment;
use crate::Label;

/// Classification of type-entities that enclose other type-entities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum WrapperKind {
    /// ParentType :~ Vec<LocalType>
    Sequence,
    /// ParentType :~ Option<LocalType>
    Option,
}

impl WrapperKind {
    pub fn describe(&self) -> &'static str {
        match self {
            WrapperKind::Sequence => "seq",
            WrapperKind::Option => "opt",
        }
    }
}

/// Path-component of a hierarchically-derived identifier for a possibly-anonymous type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum NameAtom {
    /// Any type-entity given a name explicitly in the FormatModule
    Explicit(Label),
    /// Any type-entity accessed via a positional argument of a tuple
    Positional(usize),
    /// Any type-entity accessed via a field of a record
    RecordField(Label),
    /// Type-Entity accessed via an index-labeled branch of an abstract union-type
    BranchIx(usize),
    /// Type-Entity that is embedded within a variant of an existing enum
    Variant(Label),
    /// Type-Entity accessed via one of the [`WrapperKind`] constants defined above
    Wrapped(WrapperKind),
}

impl std::fmt::Display for NameAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameAtom::Explicit(name) => write!(f, "{}", name),
            NameAtom::Positional(pos) => write!(f, "index{}", pos),
            NameAtom::RecordField(fld) => write!(f, "{}", fld),
            NameAtom::BranchIx(ix) => write!(f, "case{}", ix),
            NameAtom::Variant(vn) => write!(f, "{}", vn),
            NameAtom::Wrapped(wk) => write!(f, "denest_{}", wk.describe()),
        }
    }
}

pub type PathLabel = Vec<NameAtom>;


pub(crate) struct OnDemand<T, Arg = ()>
{
    elem: T,
    thunk: Box<dyn FnOnce(Arg)>
}

impl<'a, T, Arg> OnDemand<T, Arg> {
    pub fn new(elem: T, thunk: Box<dyn FnOnce(Arg)>) -> Self {
        Self { elem, thunk }
    }

    pub fn pure(elem: T) -> OnDemand<T, Arg> {
        OnDemand { elem, thunk: Box::new(|_| {}) }
    }

    pub fn extract(self, arg: Arg) -> T {
        (self.thunk)(arg);
        self.elem
    }
}

#[derive(Debug)]
pub(crate) struct NameCtxt {
    stack: Vec<NameAtom>,
    table: HashMap<Label, RefCell<PHeap<PathLabel>>>,
}

#[derive(Debug)]
struct PHeap<T: Eq + std::hash::Hash> {
    fixed: Vec<T>,
    floating: std::collections::hash_set::HashSet<T>,
}

impl<T: Eq + std::hash::Hash> PHeap<T> {
    pub fn new() -> Self {
        Self { fixed: Vec::new(), floating: HashSet::new() }
    }

    pub fn fix(&mut self, elem: T) -> Result<usize, anyhow::Error> {
        for (i, elt0) in self.fixed.iter().enumerate() {
            if elt0 == &elem {
                return Ok(i);
            }
        }
        if self.floating.contains(&elem) {
            self.floating.remove(&elem);
            let ret = self.fixed.len();
            self.fixed.push(elem);
            Ok(ret)
        } else {
            Err(anyhow!("cannot promote floating-to-fixed element that is not in the floating-set already"))
        }
    }

    pub fn insert(&mut self, elem: T) {
        if !self.fixed.contains(&elem) {
            self.floating.insert(elem);
        }
    }

    fn get_fixed(&self, elem: &T) -> Option<usize> {
        self.fixed.iter().enumerate().find_map(|(ix, elt0)| (elt0 == elem).then_some(ix))
    }
}

impl NameCtxt {
    /// Constructs a novel, neutral [`NameCtxt`] value.
    pub fn new() -> Self {
        NameCtxt { stack: Vec::new(), table: HashMap::new() }
    }

    /// Unwraps the provided (or instantiates a novel) [`NameCtxt`] from a parameter of type `Option<NameCtxt>`,
    /// as appropriate.
    pub fn renew(this: Option<Self>) -> Self {
        this.unwrap_or_else(|| Self::new())
    }

    /// Pushes a given [`NameAtom`] to the top (i.e. deepest element) of the [`NameCtxt`] and returns the
    /// reborrowed receiver, for chaining with other operations
    pub fn push_atom(&mut self, atom: NameAtom) -> &mut Self {
        // eprintln!("{:?} + {:?}", self.stack, &atom);
        self.stack.push(atom);
        self
    }

    /// Takes two [`NameCtxt`]s and returns a new one, with the location of the former
    /// and the union of their two tables of established atom-chain<->string pairings.
    ///
    /// Left-biased in the path, and right-biased in the association table.
    pub fn merge(self, other: Self) -> Self {
        Self {
            stack: self.stack,
            table: other.table,
        }
    }

    /// Increments the index of the top-of-stack [`NameAtom::Positional`] by one,
    /// or pushes a novel Positional element whose index is `0` if some other element-type
    /// is on the top of the stack.
    ///
    /// Returns a reborrow of the receiver, for chaining with other operations.
    pub fn increment_index(&mut self) -> &mut Self {
        match self.stack.last_mut() {
            Some(NameAtom::Positional(ref mut ix)) => *ix += 1,
            _ => self.stack.push(NameAtom::Positional(0)),
        }
        self
    }

    /// Attempts to pop the top-of-stack [`NameAtom`].
    ///
    /// Returns a reborrow of the receiver if successful, for chaining with other operations.
    ///
    /// Will return None if there are no elements on the stack.
    pub fn try_escape(&mut self) -> Option<&mut Self> {
        // eprintln!("{:?} (<- -{:?})", &self.stack[..self.stack.len() - 1], &self.stack[self.stack.len() - 1]);
        self.stack.pop()?;
        Some(self)
    }

    /// Returns an owned copy (clone) of the current stack as a PathLabel.
    pub fn get_loc(&self) -> &PathLabel {
        &self.stack
    }

    /// Unconditionally pops the top-of-stack [`NameAtom`]
    ///
    /// Returns a reborrow of the receiver if successful, for chaining with other operations.
    ///
    /// Panics if there is no element to pop.
    pub fn escape(&mut self) -> &mut Self {
        self.try_escape().expect("cannot escape a nonexistent NameAtom")
    }

    /// Co-recursive equality test that short-circuits on first discrepancy or unfalsifiable equivalence between two borrowed [`NameCtxt`] values.
    ///
    /// Only checks the paths, not the association tables, and does so from the top of the stack to the bottom.
    ///
    /// # Note
    ///
    /// If two [`NameAtom::Explicit`] atoms are encountered at the same depth, ignores any later elements and determines equality
    /// based on the equality of the explicated names.
    pub fn eq_path(stack0: &[NameAtom], stack1: &[NameAtom]) -> bool {
        if stack0.len() != stack1.len() {
            return false;
        }
        // NOTE - compare in reverse order because
        for (elt0, elt1) in Iterator::zip(stack0.iter().rev(), stack1.iter().rev()) {
            match (elt0, elt1) {
                (NameAtom::Explicit(name0), NameAtom::Explicit(name1)) => return name0 == name1,
                (atom0, atom1) if atom0 != atom1 => return false,
                _ => continue,
            }
        }
        true
    }

    fn resolve(table: &mut HashMap<Label, RefCell<PHeap<PathLabel>>>, identifier: Label, location: &PathLabel) {
        table.entry(identifier).or_insert_with(|| RefCell::new(PHeap::new())).borrow_mut().insert(location.clone());
    }


    /// Constructs a locally-unique identifier-string from a path of atoms.
    pub(crate) fn generate_name(location: &PathLabel) -> Label {
        let mut buffer = Fragment::Empty;
        for atom in location.iter().rev() {
            match atom {
                NameAtom::Explicit(name) => {
                    underscore_join(&mut buffer, name);
                    break;
                }
                other => underscore_join(&mut buffer, other),
            }
        }
        let ret = buffer.to_string();
        Label::Owned(ret)
    }

    pub(crate) fn find_name_for(&self, loc: &PathLabel) -> Result<Label, anyhow::Error> {
        let rawname = Self::generate_name(loc);
        match self.table.get(&rawname) {
            None => Err(anyhow!("no raw-name found for {:?}", loc)),
            Some(heap) => {
                match heap.borrow_mut().fix(loc.to_vec()) {
                    Ok(ix) => Ok(dedup(rawname, ix)),
                    Err(e) => Err(anyhow!("error: {e}")),
                }
            }
        }
    }

    /// Returns a raw name and the current PathLabel to disambiguate between multiple options in the resolved
    /// PHeap registered with the raw name.
    pub fn produce_name<'a>(&mut self) -> PathLabel {
        let identifier = Self::generate_name(&self.stack);
        Self::resolve(&mut self.table, identifier.clone(), &self.stack);
        self.stack.clone()
    }
}

fn dedup(rawname: Label, ix: usize) -> Label {
    if ix == 0 {
        rawname
    } else {
        Label::Owned(format!("{}__dupX{}", rawname, ix))
    }
}

fn underscore_join(tail: &mut Fragment, prefix: impl std::fmt::Display) {
    let tmp = std::mem::replace(tail, Fragment::Empty);
    *tail = Fragment::intervene(Fragment::String(Label::Owned(format!("{}", prefix))), Fragment::Char('_'), tmp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup() {
        let overlap0 = vec![NameAtom::Explicit(Label::from("foo_bar"))];
        let overlap1 = vec![NameAtom::Explicit(Label::from("foo")), NameAtom::RecordField(Label::from("bar"))];
        let mut namectxt = NameCtxt::new();
        let _ = std::mem::replace(&mut namectxt.stack, overlap0.clone());
        let oput0 = namectxt.produce_name();
        let _ = std::mem::replace(&mut namectxt.stack, overlap1.clone());
        let oput1 = namectxt.produce_name();
        let name0 = namectxt.find_name_for(&oput0).unwrap();
        let name1 = namectxt.find_name_for(&oput1).unwrap();
        assert_ne!(name0, name1);
    }

    #[test]
    fn test_record_tree() {
        let ref mut ctxt = NameCtxt::new();
        let root = ctxt.push_atom(NameAtom::Explicit(Label::Borrowed("root"))).produce_name();
        let root_data = ctxt.push_atom(NameAtom::RecordField(Label::Borrowed("data"))).produce_name();
        let root_data_header = ctxt.push_atom(NameAtom::RecordField(Label::Borrowed("header"))).produce_name();
        let data_header = ctxt.push_atom(NameAtom::Explicit(Label::Borrowed("data.header"))).produce_name();
        let root_data_body = ctxt.escape().escape().push_atom(NameAtom::RecordField(Label::Borrowed("body"))).produce_name();
        let root_data_body0 = ctxt.increment_index().produce_name();
        let root_data_body1 = ctxt.increment_index().produce_name();
        let root_data_footer = ctxt.escape().escape().push_atom(NameAtom::RecordField(Label::Borrowed("footer"))).produce_name();
        let root_extra = ctxt.escape().escape().push_atom(NameAtom::RecordField("extra".into())).produce_name();
        let root_extra_branch0 = ctxt.push_atom(NameAtom::BranchIx(0)).produce_name();
        let root_extra_varyes = ctxt.escape().push_atom(NameAtom::Variant("Yes".into())).produce_name();
        let root_extra_varno = ctxt.escape().push_atom(NameAtom::Variant("No".into())).produce_name();
        assert_eq!(ctxt.find_name_for(&root).unwrap(), "root");
        assert_eq!(ctxt.find_name_for(&root_data).unwrap(), "root_data");
        assert_eq!(ctxt.find_name_for(&root_data_header).unwrap(), "root_data_header");
        assert_eq!(ctxt.find_name_for(&data_header).unwrap(), "data.header");
        assert_eq!(ctxt.find_name_for(&root_data_body).unwrap(), "root_data_body");
        assert_eq!(ctxt.find_name_for(&root_data_body0).unwrap(), "root_data_body_index0");
        assert_eq!(ctxt.find_name_for(&root_data_body1).unwrap(), "root_data_body_index1");
        assert_eq!(ctxt.find_name_for(&root_data_footer).unwrap(), "root_data_footer");
        assert_eq!(ctxt.find_name_for(&root_extra).unwrap(), "root_extra");
        assert_eq!(ctxt.find_name_for(&root_extra_branch0).unwrap(), "root_extra_case0");
        assert_eq!(ctxt.find_name_for(&root_extra_varyes).unwrap(), "root_extra_Yes");
        assert_eq!(ctxt.find_name_for(&root_extra_varno).unwrap(), "root_extra_No");
    }
}
