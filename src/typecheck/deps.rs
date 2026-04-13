#![cfg(any())]
use std::collections::{BTreeMap, HashMap};

use crate::numeric::elaborator::IntType;

use super::{TCResult, TVar, UVar, error::TCError, error::TCErrorKind};

/// Helper type for associating a dependency path with a numeric-tree
///
/// The origin of the path is stored within [`DepGraph`] and is not explicitly
/// stored within the individual links.
#[derive(Clone, Debug, Default)]
pub struct DepPath {
    /// Each member is a TVar<-UVar dependency, successive links imply UVar->TVar resolution
    links: Vec<(TVar, UVar)>,
}

impl DepPath {
    pub const fn new() -> Self {
        Self { links: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.links.len()
    }

    pub fn push_link(&mut self, dep_pair: (TVar, UVar)) {
        self.links.push(dep_pair);
    }

    pub fn check_cycle(&self, terminus: TVar) -> TCResult<()> {
        if self.links.iter().any(|(n, _)| *n == terminus) {
            Err(TCError::from(TCErrorKind::CyclicDeps(
                terminus,
                self.clone(),
            )))
        } else {
            Ok(())
        }
    }

    pub fn deepest_tree(&self, origin: TVar) -> TVar {
        self.links.last().map(|(t, _)| *t).unwrap_or(origin)
    }
}

impl std::fmt::Display for DepPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.links)
    }
}

/// Helper struct for detecting potential cycles in the dependency-graph
/// for numeric-trees.
#[derive(Debug)]
pub struct DepGraph {
    origin: TVar,
    shortest_paths: HashMap<TVar, DepPath>,
}

impl DepGraph {
    pub fn new(origin: TVar) -> Self {
        Self {
            origin,
            shortest_paths: HashMap::from([(origin, DepPath::default())]),
        }
    }

    /// Registers a path-to-a-node in the dependency graph.
    ///
    /// If a cycle is detected, returns Err.
    ///
    /// Otherwise, returns `true` if the node has never been visited before, or `false`
    /// if it has already been visited.
    pub fn add_path(&mut self, node: TVar, via: DepPath) -> TCResult<bool> {
        use std::collections::hash_map::Entry;

        via.check_cycle(node)?;
        match self.shortest_paths.entry(node) {
            Entry::Occupied(mut occ) => {
                let prior = occ.get_mut();
                if via.len() < prior.len() {
                    *prior = via;
                }
                Ok(false)
            }
            Entry::Vacant(entry) => {
                entry.insert(via);
                Ok(true)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum VarSolution {
    #[default]
    Indefinite,
    Definite(IntType),
}

impl std::fmt::Display for VarSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarSolution::Indefinite => write!(f, "<unsolved>"),
            VarSolution::Definite(int_type) => write!(f, "{}", int_type),
        }
    }
}

impl VarSolution {
    /// Returns `true` if `self` is a definite `IntType` solution.
    pub(crate) fn is_definite(&self) -> bool {
        matches!(self, VarSolution::Definite(_))
    }

    /// Coerce `self` to an `IntType` if possible.
    pub(crate) fn coerce_int(self) -> Option<IntType> {
        match self {
            VarSolution::Definite(int_type) => Some(int_type),
            _ => None,
        }
    }
}

pub type DepSolutions = BTreeMap<UVar, VarSolution>;
