use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use anyhow::anyhow;

use crate::output::Fragment;
use crate::Label;

/// Classification of type-entities that enclose other type-entities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum WrapperKind {
    /// ParentType :~ Vec<LocalType>
    Sequence,
}

impl WrapperKind {
    pub fn describe(&self) -> &'static str {
        match self {
            WrapperKind::Sequence => "Seq",
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
    /// Type-Entity that is embedded within a variant of an existing enum
    Variant(Label),
    /// Type-Entity accessed via one of the [`WrapperKind`] constants defined above
    Wrapped(WrapperKind),
}

impl std::fmt::Display for NameAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameAtom::Explicit(name) => write!(f, "{}", name),
            NameAtom::Positional(pos) => write!(f, "ix{}", pos),
            NameAtom::RecordField(fld) => write!(f, "{}", fld),
            NameAtom::Variant(vn) => write!(f, "{}", vn),
            NameAtom::Wrapped(wk) => write!(f, "in{}", wk.describe()),
        }
    }
}

pub type PathLabel = Vec<NameAtom>;

#[derive(Debug)]
pub(crate) struct NameCtxt {
    stack: Vec<NameAtom>,
    table: HashMap<Label, RefCell<PHeap<PathLabel>>>,
}

/// Priority Heap: a loose collection of 'candidates' that are initially unsorted, but can be later promoted to the next available priority-slot,
/// which are immutable once assigned.
#[derive(Debug)]
struct PHeap<T: Eq + std::hash::Hash> {
    fixed: Vec<T>,
    floating: HashSet<T>,
}

impl<T: Eq + std::hash::Hash> PHeap<T> {
    /// Construts a new, initially-empty PHeap
    pub fn new() -> Self {
        Self {
            fixed: Vec::new(),
            floating: HashSet::new(),
        }
    }

    /// Given a value already in the `PHeap`, promotes it to the next available priority-slot and returns the corresponding
    /// index of said position.
    ///
    /// Idempotent, in that if the value already has a priority, this will not change and the same value will be returned as
    /// when it was originally promoted.
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
            Err(anyhow!(
                "cannot promote floating-to-fixed element that is not in the floating-set already"
            ))
        }
    }

    /// Push a new candidate to the `PHeap`, without assigning it a priority-slot
    pub fn insert(&mut self, elem: T) {
        if !self.fixed.contains(&elem) {
            self.floating.insert(elem);
        }
    }
}

impl NameCtxt {
    /// Constructs a novel, neutral [`NameCtxt`] value.
    pub fn new() -> Self {
        NameCtxt {
            stack: Vec::new(),
            table: HashMap::new(),
        }
    }

    /// Pushes a given [`NameAtom`] to the top (i.e. deepest element) of the [`NameCtxt`] and returns the
    /// reborrowed receiver, for chaining with other operations
    pub fn push_atom(&mut self, atom: NameAtom) -> &mut Self {
        // eprintln!("{:?} + {:?}", self.stack, &atom);
        self.stack.push(atom);
        self
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

    /// Returns a reference to the current stack as a borrowed PathLabel.
    pub fn get_loc(&self) -> &PathLabel {
        &self.stack
    }

    /// Unconditionally pops the top-of-stack [`NameAtom`]
    ///
    /// Returns a re-borrow of the receiver if successful, for chaining with other operations.
    ///
    /// Panics if there is no element to pop.
    pub fn escape(&mut self) -> &mut Self {
        match self.try_escape() {
            Some(this) => this,
            None => unreachable!("escape attempted on empty stack-NameCtxt"),
        }
    }

    /// Co-recursive equality test that short-circuits on first discrepancy or unfalsifiable equivalence between two borrowed [`NameCtxt`] values.
    ///
    /// Only checks the paths, not the association tables, and does so from the top of the stack to the bottom.
    ///
    /// # Note
    ///
    /// If two [`NameAtom::Explicit`] atoms are encountered at the same depth, ignores any later elements and determines equality
    /// based on the equality of the explicated names.
    #[allow(dead_code)]
    pub fn eq_path(stack0: &[NameAtom], stack1: &[NameAtom]) -> bool {
        if stack0.len() != stack1.len() {
            return false;
        }
        // NOTE - compare in reverse order because we want to encounter the deepest explicated label before anything prior
        for (elt0, elt1) in Iterator::zip(stack0.iter().rev(), stack1.iter().rev()) {
            match (elt0, elt1) {
                (NameAtom::Explicit(name0), NameAtom::Explicit(name1)) => return name0 == name1,
                (atom0, atom1) if atom0 != atom1 => return false,
                _ => continue,
            }
        }
        true
    }

    /// Inserts a delayed-priority association between `identifier` and `location` into `table`
    fn resolve(
        table: &mut HashMap<Label, RefCell<PHeap<PathLabel>>>,
        identifier: Label,
        location: &PathLabel,
    ) {
        table
            .entry(identifier)
            .or_insert_with(|| RefCell::new(PHeap::new()))
            .borrow_mut()
            .insert(location.clone());
    }

    /// Constructs a locally-unique identifier-string from a `PathLabel`
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

    /// Returns a globally-unique fixed-priority name for a given `PathLabel`
    ///
    /// The order in which competing candidates for a given name are passed into this method affects deduplication strategies
    /// and resulting identifiers, but otherwise the generation process for names is invariant.
    pub(crate) fn find_name_for(&self, loc: &PathLabel) -> Result<Label, anyhow::Error> {
        let rawname = Self::generate_name(loc);
        match self.table.get(&rawname) {
            None => Err(anyhow!("no raw-name found for {:?}", loc)),
            Some(heap) => match heap.borrow_mut().fix(loc.to_vec()) {
                Ok(ix) => Ok(dedup(rawname, ix)),
                Err(e) => Err(anyhow!("error: {e}")),
            },
        }
    }

    /// Registers the current PathLabel on-stack into the appropriate [`PHeap`] in the association-table,
    /// returning it for later promotion using [`NameCtxt::find_name_for`]
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

// prefixes a given string-tail with an intervening underscore, but leaves that separator out if either is the empty-string
fn underscore_join(tail: &mut Fragment, prefix: impl std::fmt::Display) {
    let tmp = std::mem::replace(tail, Fragment::Empty);
    *tail = Fragment::intervene(
        Fragment::String(Label::Owned(format!("{}", prefix))),
        Fragment::Char('_'),
        tmp,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup() {
        let overlap0 = vec![NameAtom::Explicit(Label::from("foo_bar"))];
        let overlap1 = vec![
            NameAtom::Explicit(Label::from("foo")),
            NameAtom::RecordField(Label::from("bar")),
        ];
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
        let root = ctxt
            .push_atom(NameAtom::Explicit(Label::Borrowed("root")))
            .produce_name();
        let root_data = ctxt
            .push_atom(NameAtom::RecordField(Label::Borrowed("data")))
            .produce_name();
        let root_data_header = ctxt
            .push_atom(NameAtom::RecordField(Label::Borrowed("header")))
            .produce_name();
        let data_header = ctxt
            .push_atom(NameAtom::Explicit(Label::Borrowed("data.header")))
            .produce_name();
        let root_data_body = ctxt
            .escape()
            .escape()
            .push_atom(NameAtom::RecordField(Label::Borrowed("body")))
            .produce_name();
        let root_data_body0 = ctxt.increment_index().produce_name();
        let root_data_body1 = ctxt.increment_index().produce_name();
        let root_data_footer = ctxt
            .escape()
            .escape()
            .push_atom(NameAtom::RecordField(Label::Borrowed("footer")))
            .produce_name();
        let root_extra = ctxt
            .escape()
            .escape()
            .push_atom(NameAtom::RecordField("extra".into()))
            .produce_name();
        let root_extra_varyes = ctxt
            .push_atom(NameAtom::Variant("Yes".into()))
            .produce_name();
        let root_extra_varno = ctxt
            .escape()
            .push_atom(NameAtom::Variant("No".into()))
            .produce_name();
        assert_eq!(ctxt.find_name_for(&root).unwrap(), "root");
        assert_eq!(ctxt.find_name_for(&root_data).unwrap(), "root_data");
        assert_eq!(
            ctxt.find_name_for(&root_data_header).unwrap(),
            "root_data_header"
        );
        assert_eq!(ctxt.find_name_for(&data_header).unwrap(), "data.header");
        assert_eq!(
            ctxt.find_name_for(&root_data_body).unwrap(),
            "root_data_body"
        );
        assert_eq!(
            ctxt.find_name_for(&root_data_body0).unwrap(),
            "root_data_body_ix0"
        );
        assert_eq!(
            ctxt.find_name_for(&root_data_body1).unwrap(),
            "root_data_body_ix1"
        );
        assert_eq!(
            ctxt.find_name_for(&root_data_footer).unwrap(),
            "root_data_footer"
        );
        assert_eq!(ctxt.find_name_for(&root_extra).unwrap(), "root_extra");
        assert_eq!(
            ctxt.find_name_for(&root_extra_varyes).unwrap(),
            "root_extra_Yes"
        );
        assert_eq!(
            ctxt.find_name_for(&root_extra_varno).unwrap(),
            "root_extra_No"
        );
    }
}
