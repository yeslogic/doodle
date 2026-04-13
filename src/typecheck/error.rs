use std::rc::Rc;

use crate::typecheck::base_set::TryFromPrimIntError;

use super::base_set::{BaseSet, IntSet, TryFromBaseTypeError};
use super::*;

pub(crate) type TypeError = UnificationError<Rc<UType>>;

pub(crate) type ConstraintError = UnificationError<Constraint>;

impl From<TypeError> for ConstraintError {
    fn from(value: TypeError) -> Self {
        match value {
            UnificationError::Unsatisfiable(lt, rt) => {
                let lc = Constraint::Equiv(lt);
                let rc = Constraint::Equiv(rt);
                UnificationError::Unsatisfiable(lc, rc)
            }
        }
    }
}

#[derive(Clone, Debug)]
// Generic error in unification between two type-constraints, which are represented generically
pub enum UnificationError<T: std::fmt::Debug> {
    // Incompatible(UVar, T, T), // two independent assertions about a UVar are incompatible
    Unsatisfiable(T, T), // a single non-variable assertion is directly unsatisfiable
}

impl<T: std::fmt::Debug> std::fmt::Display for UnificationError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // UnificationError::Incompatible(var, lhs, rhs) => { write!( f, "incompatible equivalences `{var} = {lhs:?}` && `{var} = {rhs:?}`") }
            UnificationError::Unsatisfiable(lhs, rhs) => {
                write!(f, "unsatisfiable equivalence  `{lhs:?} = {rhs:?}`")
            }
        }
    }
}

impl<T: std::fmt::Debug> std::error::Error for UnificationError<T> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Marker enum to track which of an invariant and variant constraints came first
pub enum Polarity {
    /// Attempting to add variants onto an invariant metavariable
    PriorInvariant,
    /// Attempting to enforce invariant constraints on a Variant metavariable
    PriorVariant,
}

#[derive(Debug)]
pub enum CrossLayerNumericError {
    TryFromBase(TryFromBaseTypeError),
    TryFromPrim(TryFromPrimIntError),
    PrimNotInBaseSet(PrimInt, BaseSet),
    BaseNotInIntSet(BaseType, IntSet),
    DisjointSets(IntSet, BaseSet),
    NonMatching(PrimInt, BaseType),
    EmptyBase,
    EmptyInt,
}

impl std::fmt::Display for CrossLayerNumericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TryFromBase(err) => err.fmt(f),
            Self::TryFromPrim(err) => err.fmt(f),
            Self::DisjointSets(prims, uints) => {
                write!(
                    f,
                    "empty intersection of PrimIntSet and UintSet: `{}` ∩ `{}` = ∅",
                    prims, uints
                )
            }
            Self::PrimNotInBaseSet(prim, base) => {
                write!(f, "PrimInt not in BaseSet: `{prim}` ∉ `{base}`")
            }
            Self::BaseNotInIntSet(base, int) => {
                write!(f, "BaseType not in IntSet: `{base}` ∉ `{int}`")
            }
            Self::NonMatching(prim, base) => {
                write!(f, "non-matching PrimInt and BaseType: `{prim}` ≄ `{base}`")
            }
            Self::EmptyBase => {
                write!(f, "unification insoluble due to empty BaseSet`")
            }
            Self::EmptyInt => {
                write!(f, "unification insoluble due to empty IntSet")
            }
        }
    }
}

impl std::error::Error for CrossLayerNumericError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TryFromBase(err) => Some(err),
            _ => None,
        }
    }
}

impl From<TryFromBaseTypeError> for CrossLayerNumericError {
    fn from(err: TryFromBaseTypeError) -> Self {
        Self::TryFromBase(err)
    }
}

impl From<TryFromPrimIntError> for CrossLayerNumericError {
    fn from(err: TryFromPrimIntError) -> Self {
        Self::TryFromPrim(err)
    }
}

impl From<TryFromBaseTypeError> for TCErrorKind {
    fn from(err: TryFromBaseTypeError) -> Self {
        Self::CrossLayerNumeric(CrossLayerNumericError::TryFromBase(err))
    }
}

impl From<TryFromPrimIntError> for TCErrorKind {
    fn from(err: TryFromPrimIntError) -> Self {
        Self::CrossLayerNumeric(CrossLayerNumericError::TryFromPrim(err))
    }
}

impl From<TryFromBaseTypeError> for TCError {
    fn from(err: TryFromBaseTypeError) -> Self {
        Self {
            err: Box::new(TCErrorKind::CrossLayerNumeric(
                CrossLayerNumericError::TryFromBase(err),
            )),
            _trace: Vec::new(),
        }
    }
}

impl From<TryFromPrimIntError> for TCError {
    fn from(err: TryFromPrimIntError) -> Self {
        Self {
            err: Box::new(TCErrorKind::CrossLayerNumeric(
                CrossLayerNumericError::TryFromPrim(err),
            )),
            _trace: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct TCError {
    pub(crate) err: Box<TCErrorKind>,
    pub(crate) _trace: Vec<Box<dyn std::fmt::Debug + 'static + Send + Sync>>,
}

impl From<TCErrorKind> for TCError {
    fn from(value: TCErrorKind) -> Self {
        Self {
            err: Box::new(value),
            _trace: Vec::new(),
        }
    }
}

impl TCError {
    #[allow(dead_code)]
    pub(crate) fn with_trace<T>(mut self, trace: T) -> Self
    where
        T: std::fmt::Debug + Send + Sync + 'static,
    {
        self._trace.push(Box::new(trace));
        self
    }
}

impl std::fmt::Display for TCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} (", self.err)?;
        for item in self._trace.iter() {
            writeln!(f, "\t{item:?}")?;
        }
        write!(f, ")")
    }
}

impl std::error::Error for TCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.err.source()
    }
}

#[derive(Debug)]
pub enum TCErrorKind {
    /// Constraints-Constraints unification failed for ADT/non-ADT constraints-pair
    VarianceMismatch(UVar, VMId, VarMap, Constraint, Polarity),
    /// Failed Constraint-Constraint unification
    Unification(ConstraintError),
    /// Constraints on UVar imply an infinite type
    InfiniteType(UVar, Constraints),
    /// Base-Set constraint on given UVar has multiple solutions that cannot be tie-broken
    MultipleSolutions(UVar, BaseSet),
    MultipleIntSolutions(UVar, IntSet),
    /// Base-Set/Int-Set constraints on a given UVar has no solution
    NoSolution(UVar),
    /// No view-var in scope bound to the given label
    MissingView(Label),
    /// InferenceError occurred for a variable within a numeric subtree
    Inference(UVar, InferenceError),
    /// Contextually numeric variable found to have a non-numeric constraints
    NonNumeric(UVar, Constraints),
    /// Any issue reconciling BaseType numerics and IntType numerics
    CrossLayerNumeric(CrossLayerNumericError),
}

impl TCErrorKind {
    #[expect(dead_code)]
    pub(crate) fn with_trace<T>(self, trace: T) -> TCError
    where
        T: std::fmt::Debug + Send + Sync + 'static,
    {
        TCError::from(self).with_trace(trace)
    }
}

impl std::fmt::Display for TCErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TCErrorKind::VarianceMismatch(uv, vmid, vm, constraint, pol) => match pol {
                Polarity::PriorInvariant => write!(
                    f,
                    "prior constraint `{uv} {constraint}` precludes attempted unification `{uv} ⊇ {vmid} (:= {vm:?})`"
                ),
                Polarity::PriorVariant => write!(
                    f,
                    "attempted unification `{uv} {constraint}` precluded by prior constraint `{uv} ⊇ {vmid} (:= {vm:?})`"
                ),
            },
            TCErrorKind::Unification(c_err) => write!(f, "{c_err}"),
            TCErrorKind::InfiniteType(v, constraints) => match constraints {
                Constraints::Indefinite => {
                    unreachable!("indefinite constraint `{v} = ??` is not infinite")
                }
                Constraints::Variant(vmid) => write!(
                    f,
                    "`{v} ⊇ {vmid}` constitutes an infinite type ({v} or alias occurs within {vmid})"
                ),
                Constraints::Invariant(inv) => match inv {
                    Constraint::Equiv(t) => write!(
                        f,
                        "`{v} = {t:?}` is an infinite type ({v} or alias occurs within the rhs utype)"
                    ),
                    Constraint::NumTree(_) | Constraint::Elem(_) => {
                        unreachable!("`{v} {inv}` is not infinite, but we thought it was")
                    }
                    Constraint::Proj(ps) => {
                        write!(f, "`{v} ~ {ps:?}` constitutes an infinite type")
                    }
                },
            },
            TCErrorKind::MultipleSolutions(uv, set) => {
                write!(f, "no unique solution for `{uv} {}`", set.to_constraint())
            }
            TCErrorKind::MultipleIntSolutions(uvar, set) => {
                write!(f, "no unique solution for `{uvar} {}`", set.to_constraint())
            }
            TCErrorKind::NoSolution(uv) => write!(f, "no valid solutions for `{uv}`"),
            TCErrorKind::MissingView(lbl) => {
                write!(f, "view-based parse depends on unbound identifier `{lbl}`")
            }
            TCErrorKind::Inference(tree_v, err) => {
                write!(f, "inference failed for tree-var `{tree_v}`: {err}")
            }
            TCErrorKind::NonNumeric(v, c) => {
                write!(
                    f,
                    "non-numeric constraints found on variable in numeric-tree context: `{v} {c}`"
                )
            }
            TCErrorKind::CrossLayerNumeric(err) => {
                write!(f, "cross-layer numeric error: {err}")
            }
        }
    }
}

impl std::error::Error for TCErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unification(u_err) => Some(u_err),
            Self::Inference(_, err) => Some(err),
            _ => None,
        }
    }
}

impl From<TypeError> for TCErrorKind {
    fn from(value: TypeError) -> Self {
        Self::Unification(value.into())
    }
}

impl From<TypeError> for TCError {
    fn from(value: TypeError) -> Self {
        Self::from(TCErrorKind::Unification(value.into()))
    }
}

impl From<ConstraintError> for TCError {
    fn from(value: ConstraintError) -> Self {
        Self::from(TCErrorKind::Unification(value))
    }
}

impl<T> From<(ConstraintError, T)> for TCError
where
    T: std::fmt::Debug + 'static + Send + Sync,
{
    fn from(value: (ConstraintError, T)) -> Self {
        Self {
            err: Box::new(TCErrorKind::Unification(value.0)),
            _trace: vec![Box::new(value.1)],
        }
    }
}

impl From<ConstraintError> for TCErrorKind {
    fn from(value: ConstraintError) -> Self {
        Self::Unification(value)
    }
}

impl From<(UVar, InferenceError)> for TCErrorKind {
    fn from((var, err): (UVar, InferenceError)) -> Self {
        Self::Inference(var, err)
    }
}

impl From<(UVar, InferenceError)> for TCError {
    fn from(value: (UVar, InferenceError)) -> Self {
        TCError {
            err: Box::new(TCErrorKind::from(value)),
            _trace: Vec::new(),
        }
    }
}

impl From<CrossLayerNumericError> for TCErrorKind {
    fn from(value: CrossLayerNumericError) -> Self {
        Self::CrossLayerNumeric(value)
    }
}

impl From<CrossLayerNumericError> for TCError {
    fn from(value: CrossLayerNumericError) -> Self {
        TCError {
            err: Box::new(TCErrorKind::CrossLayerNumeric(value)),
            _trace: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum InferenceError {
    BadUnification(super::inference::Constraint, super::inference::Constraint),
    Ambiguous,
    NoSolution,
    MultipleSolutions,
    Eval(crate::numeric::core::EvalError),
    UnconstrainedVar(NVar),
    MissingGlobal(UVar),
    UnscopedVariable(Label),
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferenceError::BadUnification(cx1, cx2) => {
                write!(f, "constraints `{}` and `{}` cannot be unified", cx1, cx2)
            }
            InferenceError::Ambiguous => write!(
                f,
                "mixed-type binary operation must have out_rep on operation to avoid ambiguity"
            ),
            InferenceError::Eval(e) => {
                write!(f, "inference abandoned due to evaluation error: {}", e)
            }
            InferenceError::NoSolution => write!(
                f,
                "no valid assignment of PrimInt types produce a fully representable tree"
            ),
            InferenceError::MultipleSolutions => write!(
                f,
                "multiple assignments of PrimInt produce a fully representable tree, in absence of tie-breaking mechanism"
            ),
            InferenceError::UnconstrainedVar(v) => {
                write!(f, "unconstrained variable {v} cannot be solved")
            }
            InferenceError::MissingGlobal(v) => {
                write!(
                    f,
                    "tree-external metavariable {v} not found in DepSolutions lookup-table"
                )
            }
            InferenceError::UnscopedVariable(label) => {
                write!(f, "identifier {label} not found in scope provided")
            }
        }
    }
}

impl std::error::Error for InferenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InferenceError::Eval(e) => Some(e),
            _ => None,
        }
    }
}

pub type InferenceResult<T> = Result<T, InferenceError>;
