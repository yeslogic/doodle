#![allow(dead_code)]
#![allow(unused_imports)]

use std::num::NonZeroUsize;

use super::{GenType, TypedExpr};
use crate::Label;

pub(crate) trait QueryExternVar {
    fn query_extern_var(&self, var: &str, persistent: bool) -> VarInfo;
}

// REVIEW - should we cache variables we see that aren't `var`, to avoid repeated traversal?
impl<TypeRep> QueryExternVar for TypedExpr<TypeRep> {
    fn query_extern_var(&self, var: &str, persistent: bool) -> VarInfo {
        let mut ret = VarInfo::new();
        match self {
            TypedExpr::Var(_, lbl) => {
                if *lbl == var {
                    if persistent {
                        ret.add_persist();
                    } else {
                        ret.add_reference();
                    }
                }
            }
            TypedExpr::IntRel(_, _, lhs, rhs)
            | TypedExpr::Arith(_, _, lhs, rhs) => {
                ret += lhs.query_extern_var(var, persistent);
                ret += rhs.query_extern_var(var, persistent);
            }
            TypedExpr::AsU8(inner)
            | TypedExpr::AsU16(inner)
            | TypedExpr::AsU32(inner)
            | TypedExpr::AsU64(inner)
            | TypedExpr::AsChar(inner) => {
                ret += inner.query_extern_var(var, persistent)
            }
            _ => todo!(),
        }
        ret
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct VarInfo {
    n_references: u32,
    n_persists: u32,
}

impl std::ops::Add<VarInfo> for VarInfo {
    type Output = VarInfo;

    fn add(self, rhs: VarInfo) -> Self::Output {
        VarInfo {
            n_references: self.n_references + rhs.n_references,
            n_persists: self.n_persists + rhs.n_persists,
        }
    }
}

impl std::ops::AddAssign<VarInfo> for VarInfo {
    fn add_assign(&mut self, rhs: VarInfo) {
        self.n_references += rhs.n_references;
        self.n_persists += rhs.n_persists;
    }
}

impl VarInfo {
    pub fn new() -> Self {
        VarInfo {
            n_references: 0,
            n_persists: 0,
        }
    }

    pub const fn is_unused(&self) -> bool {
        self.n_references == 0 && self.n_persists == 0
    }

    pub fn add_reference(&mut self) {
        self.n_references += 1;
    }

    pub fn add_persist(&mut self) {
        self.n_persists += 1;
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
