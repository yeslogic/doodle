use std::{rc::Rc, cell::RefCell, collections::HashSet};

use crate::{ValueType, GroundType, Label, error::TypeError};

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct UVar(usize);

/// Unification type, equivalent to ValueType up to abstraction
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum UType {
    Var(UVar), // type-hole
    Ground(GroundType),
    Tuple(Vec<UType>),
    Record(Vec<(Label, UType)>),
    Seq(Box<UType>),
}

/// Representation of an inferred type that is either fully-known or partly-known
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub(crate) enum VType {
    Concrete(ValueType),
    Abstract(UType),
}

pub(crate) struct TypeChecker<'a> {
    constraints: Vec<HashSet<Constraint>>,
    equivalence: Vec<HashSet<usize>>,
    solutions: Solutions,
}
struct Solutions {
    converged: HashSet<usize>,
    unsolved: HashSet<usize>,
}

impl Solutions {
    pub fn new() -> Self {
        Self { converged: HashSet::new(), unsolved: HashSet::new() }
    }
}


#[derive(Clone, Debug)]
pub(crate) enum Constraint {
    Equiv(UType), // direct equivalence with a UType
    Elem(Vec<GroundType>), // implicit restriction to a narrowed set of ground-types (e.g. from `Expr::AsU32`)
    ContainsVariants(HashMap<Label, UType>), // requirement of a minimal set of variants for a potentially larger overall union-type
}

impl<'a> TypeChecker<'a> {
    /// Attempt to resolve all existing constraints on a single metavariable.
    ///
    /// Returns a result whose value is `true` if the solution process converged to
    /// a concrete type, and `false` if it was either irreducible or only partly solved.
    ///
    /// Returns an error if any two constraints on that metavariable form an insoluble pair.
    ///
    /// May update the typechecker's set of unsolved variables
    fn solve_variable(&mut self, ix: usize) -> Result<bool, ConstraintError> {

    }

    fn unify_type(&mut self, left: &UType, right: &UType) -> Result<(), TypeError> {
        match (left, right) {
            (UType::Seq(e1), UType::Seq(e2)) => {
                self.unify_type(e1.as_ref(), e2.as_ref())
            }
            (UType::Tuple(ts1), UType::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    return Err(UnificationError::Unsatisfiable(left, right))
                }
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    self.unify_type(t1, t2)?;
                }
                Ok(())
            }
            (UType::Record(fs1), UType::Record(fs2)) => {
                if fs1.len() != fs2.len() {
                    return Err(UnificationError::Unsatisfiable(left.clone(), right.clone()))
                }
                for ((l1, f1), (l2, f2)) in Iterator::zip(fs1.iter(), fs2.iter()) {
                    if l1 != l2 {
                        return Err(UnificationError::Unsatisfiable(left.clone(), right.clone()))
                    }
                    self.unify_type(f1, f2)?;
                }
                Ok(())
            }
            (UType::Union(branches1), UType::Union(branches2)) => {
            }
        }
    }
}

pub(crate) type TypeError = UnificationError<UType>;
pub(crate) type ConstraintError = UnificationError<Constraint>;


#[derive(Clone, Debug)]
pub enum UnificationError<T: std::fmt::Debug> {
    Incompatible(usize, T, T), // two standalone equivalences on a UVar are incompatible with each other
    Unsatisfiable(T, T), // single equivalence cannot be satisfied
}

impl<T: Debug> std::fmt::Display for UnificationError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnificationError::Incompatible(ix, lhs, rhs) => {
                write!(f, "incompatible equivalences `?{ix} = {lhs:?}` && `?{ix} = {rhs:?}`"),
            }
            UnificationError::Unsatisfiable(lhs, rhs) => {
                write!(f, "unsatisfiable equivalence  `{lhs:?} = {rhs:?}`")
            }
        }
    }
}

impl<T: std::fmt::Debug> std::error::Error for UnificationError<T> {}
