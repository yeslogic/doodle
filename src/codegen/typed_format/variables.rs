#![allow(dead_code)]
#![allow(unused_imports)]

use super::{GenType, TypedExpr};
use crate::Label;

impl TypedExpr<GenType> {
    pub(crate) fn get_vars(&self) -> Variables {
        let mut vars = Variables::new();
        let ctxt = VarCtxt::new();
        self.poll_vars(&mut vars, ctxt);
        vars
    }

    /// Extends a collection with a list of all the variable-names used in an Expr, as well
    /// as whether their provenance at site-of-use is internal or external, and how they are
    /// accessed.
    ///
    /// If the expression is itself a raw `TypedExpr::Var`, returns the variable's verbatim identifier, otherwise None.
    fn poll_vars<'a>(&'a self, _vars: &mut Variables, _ctxt: VarCtxt<'a>) -> Option<&'a Label> {
        match self {
            TypedExpr::U8(..)
            | TypedExpr::U16(..)
            | TypedExpr::U32(..)
            | TypedExpr::U64(..)
            | TypedExpr::Bool(..) => None,

            _ => unimplemented!(),
        }
    }
}

// REVIEW - consider HashMap?
type ScopeContainer<K, V> = std::collections::BTreeMap<K, V>;

#[derive(Debug, Clone, Default)]
enum VarCtxt<'a> {
    #[default]
    Empty,
    SingleCtxt(Option<&'a VarCtxt<'a>>, &'a Label),
}

impl<'a> VarCtxt<'a> {
    pub fn new() -> Self {
        VarCtxt::Empty
    }
}

#[derive(Clone, Debug, Default)]
struct PerScope<T> {
    scopes: ScopeContainer<Provenance, T>,
}

impl<T> PerScope<T> {
    pub fn new() -> Self {
        PerScope {
            scopes: ScopeContainer::new(),
        }
    }

    pub fn insert(&mut self, provenance: Provenance, value: T) -> Option<T> {
        self.scopes.insert(provenance, value)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Default, Hash)]
enum Provenance {
    #[default]
    External,
    // De Bruijn index. 1 indicates usage-site is in the exact binding-scope of the definition site
    DeBruijn(std::num::NonZeroU32),
}

type VarStore<K, V> = std::collections::HashMap<K, V>;

#[derive(Debug, Clone)]
pub(crate) struct Variables {
    _store: VarStore<Label, PerScope<AccessKind>>,
}

impl Variables {
    pub fn new() -> Self {
        Variables {
            _store: VarStore::new(),
        }
    }

    fn entry(
        &mut self,
        vname: Label,
    ) -> std::collections::hash_map::Entry<'_, Label, PerScope<AccessKind>> {
        self._store.entry(vname)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) enum AccessKind {
    /// How `x` is accessed in `let y = x;`
    Rebind,
    /// How `x` is accessed in `match x { .. }`
    Switch,
}
