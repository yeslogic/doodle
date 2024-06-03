use std::fmt::Write;
use std::collections::HashMap;

use crate::Label;

/// Classification of type-entities that enclose other type-entities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum WrapperKind {
    /// ThisType :~ Vec<OtherType>
    Sequence,
    /// ThisType :~ Option<OtherType>
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
            NameAtom::Variant(vn) => write!(f, "{}", vn),
            NameAtom::Wrapped(wk) => write!(f, "denest_{}", wk.describe()),
        }
    }
}

pub(crate) struct OnDemand<'a, T, Arg = ()>
{
    elem: T,
    thunk: Box<dyn FnOnce(Arg) + 'a>
}

impl<'a, T, Arg> OnDemand<'a, T, Arg> {
    pub fn new(elem: T, thunk: Box<dyn FnOnce(Arg)>) -> Self {
        Self { elem, thunk }
    }

    pub fn pure(elem: T) -> OnDemand<'static, T, Arg> {
        OnDemand { elem, thunk: Box::new(|_| {}) }
    }

    pub fn extract(self, arg: Arg) -> T {
        (self.thunk)(arg);
        self.elem
    }
}

pub(crate) struct NameCtxt {
    stack: Vec<NameAtom>,
    table: HashMap<Label, Vec<NameAtom>>,
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
        self.stack.pop()?;
        Some(self)
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

    fn resolve<'a>(table: &HashMap<Label, Vec<NameAtom>>, identifier: Label, location: &[NameAtom], user_specified: bool) -> OnDemand<'a, Label, &'a mut NameCtxt> {
        if user_specified {
            return OnDemand::pure(identifier);
        }
        match table.get(&identifier) {
            None => {
                let loc = location.to_vec();
                let id = identifier.clone();
                OnDemand::new(identifier.clone(), Box::new(move |ctxt: &'a mut NameCtxt| { ctxt.table.insert(id, loc); () }))
            }
            Some(path) => {
                if NameCtxt::eq_path(&path[..], location) {
                    OnDemand::pure(identifier)
                } else {
                    Self::resolve(table, Label::Owned(format!("{}_dedup", identifier)), location, false)
                }
            }
        }
    }


    /// Constructs a locally-unique identifier-string from a path of atoms.
    fn generate_name(location: &Vec<NameAtom>) -> Label {
        let mut buffer = String::new();
        for atom in location.iter().rev() {
            match atom {
                NameAtom::Explicit(name) => underscore_join(&mut buffer, name),
                other => underscore_join(&mut buffer, other),
            }
        }
        Label::Owned(buffer)
    }

    /// Returns a delayed-evaluation value via [`OnDemand`] that dynamically appends the associated
    /// production as an entry to the table of whatever [`NameCtxt`] is later passed in.
    pub fn produce_name<'a>(&'a mut self, hint: Option<Label>) -> OnDemand<Label, &'a mut NameCtxt> {

        let (identifier, user_specified) = match hint {
            Some(id) => (id, true),
            None => (Self::generate_name(&self.stack), false),
        };
        Self::resolve(&self.table, identifier, &self.stack, user_specified)
    }
}

fn underscore_join(lhs: &mut String, rhs: impl std::fmt::Display) {
    if lhs.is_empty() {
        write!(lhs, "{}", rhs).expect("bad string write");
    } else {
        lhs.insert_str(0, format!("{}_", rhs).as_str());
    }
}
