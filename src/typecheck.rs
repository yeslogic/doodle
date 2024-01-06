use std::{collections::{HashSet, HashMap}, cmp::Ordering, rc::Rc};

use crate::{ValueType, GroundType, Label};

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct UVar(usize);

/// Unification type, equivalent to ValueType up to abstraction
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum UType {
    Var(UVar), // type-hole
    Ground(GroundType),
    Tuple(Vec<Rc<UType>>),
    Record(Vec<(Label, Rc<UType>)>),
    Seq(Rc<UType>),
}

/// Analogue of Mercury-style instantiation states, but for metavariables
pub(crate) enum UInst {
    Ground, // No free metavariables
    Bound(HashSet<UVar>),
    Free(UVar),
}

impl UType {
    pub fn iter_embedded(&self) -> Box<dyn Iterator<Item = Rc<UType>>> {
        match self {
            UType::Var(..) | UType::Ground(..) => Box::new(std::iter::empty()),
            UType::Tuple(ts) => Box::new(ts.iter().cloned()),
            UType::Record(fs) => Box::new(fs.iter().map(|(_l, t)| t.clone())),
            UType::Seq(t) => Box::new(std::iter::once(t.clone())),
        }

    }

    pub fn get_uinst(&self) -> UInst {
        match self {
            Self::Ground(..) => UInst::Ground,
            Self::Var(v) => UInst::Free(*v),
            Self::Record(fs) => {
                let mut vars = HashSet::new();
                for (_l, t) in fs.iter() {
                    match t.get_uinst() {
                        UInst::Free(var) => {
                            let _ = vars.insert(var);
                        },
                        UInst::Bound(vars0) => {
                            vars.extend(vars0.iter());
                        },
                        UInst::Ground => {}
                    }
                }
                if vars.is_empty() {
                    UInst::Ground
                } else {
                    UInst::Bound(vars)
                }
            }
            Self::Tuple(ts) => {
                let mut vars = HashSet::new();
                for t in ts.iter() {
                    match t.get_uinst() {
                        UInst::Free(var) => {
                            let _ = vars.insert(var);
                        },
                        UInst::Bound(vars0) => {
                            vars.extend(vars0.iter());
                        },
                        UInst::Ground => {}
                    }
                }
                if vars.is_empty() {
                    UInst::Ground
                } else {
                    UInst::Bound(vars)
                }
            }
            Self::Seq(t) => {
                match t.get_uinst() {
                    UInst::Ground => UInst::Ground,
                    UInst::Bound(vs) => UInst::Bound(vs),
                    UInst::Free(v) => UInst::Bound(HashSet::from([v]))
                }
            }
        }
    }

    /// Determines whether a particular UVar occurs within a UType, and at what nesting depth.
    ///
    /// Some(true) indicates direct equivalence to the UVar in question, which is a
    /// tautology rather than an infinite type if self is equated to the UVar
    /// being searched for.
    ///
    /// Some(false) indicates that the UVar in question occurs somewhere within the type,
    /// but inside of an N-layer nested structure of intervening constructors. This would constitute
    /// an infinite type if self is equated to the UVar being searched for.
    ///
    /// None indicates the variable does not directly occur in the type.
    ///
    /// Note that transitive inclusion is not checked, as this requires a typechecker context to expand
    /// indirect equivalences, so a return value of `None` does not necessarily rule out infinite types
    /// or tautologies.
    pub fn find_var_depth(&self, var: &UVar) -> Option<bool> {
        match self.get_uinst() {
            UInst::Ground => None,
            UInst::Bound(vs) => {
                if vs.contains(var) {
                    Some(false)
                } else {
                    None
                }
            }
            UInst::Free(v0) => {
                if v0 == *var {
                    Some(true)
                } else {
                    None
                }
            }
        }
    }
}

/// Representation of an inferred type that is either fully-known or partly-known
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum VType {
    Concrete(ValueType),
    Abstract(UType),
    IndefiniteUnion(VarMap),
}

pub(crate) struct TypeChecker {
    constraints: Vec<Constraints>,
    equivalences: Vec<HashSet<usize>>, // set of non-identity metavariables that are aliased to ?ix
    solutions: Solutions,
}

#[derive(Clone, Debug, Default)]
pub enum Constraints {
    #[default]
    Indefinite, // default value before union-type distinction is made
    Variant(VarMap), // for type metavariables that are inferred to be an indefinite union type, minimal list of variants and their types we have found
    Invariant(HashSet<Constraint>), // for all type metavariables, set of inferred constraints (other than variants)
}

impl Constraints {
    pub fn new() -> Self {
        Self::Indefinite
    }

    pub fn is_variant(&self) -> bool {
        matches!(self, Self::Variant(_))
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Constraint {
    Equiv(Rc<UType>), // direct equivalence with a UType, which should not be a bare `UType::Var` (that is what TypeChecker.equivalences is for)
    Elem(Vec<GroundType>), // implicit restriction to a narrowed set of ground-types (e.g. from `Expr::AsU32`)
}

#[derive(Clone)]
struct Solutions {
    substitutions: HashMap<usize, Rc<UType>>, // list of non-trivial (i.e. not `?n = ?n`) substitutions based on prior constraint-solving
}

impl Solutions {
    pub fn new() -> Self {
        Self { substitutions: HashMap::new() }
    }
}

type VarMap = HashMap<Label, Rc<UType>>;


impl TypeChecker {
    /// Constructs a typechecker with initially 0 metavariables.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            equivalences: Vec::new(),
            solutions: Solutions::new(),
        }
    }

    /// Attempt to resolve all existing constraints on a single metavariable.
    ///
    /// Returns a result whose value is `true` if the solution process converged to
    /// a concrete type, and `false` if it error::TypeErrorwas either irreducible or only partly solved.
    ///
    /// Returns an error if any two constraints on that metavariable form an insoluble pair.
    ///
    /// May update the typechecker's set of unsolved variables
    fn solve_variable(&mut self, ix: usize) -> Result<bool, ConstraintError> {
        todo!()
    }

    /// Partially solve for as many free metavariables in an input `UType` as possible
    ///
    /// If a variable has no known solutions, this acts as the identity function
    fn reduce(&self, t: &UType) -> Rc<UType> {
        match t {
            UType::Var(v) => {
                if let Some(t0) = self.solutions.substitutions.get(&v.0) {
                    self.reduce(t0)
                } else {
                    Rc::new(t.clone())
                }
            }
            UType::Ground(..) => Rc::new(t.clone()),
            UType::Tuple(ts) => {
                Rc::new(UType::Tuple(ts.iter().map(|elt| self.reduce(elt)).collect()))
            }
            UType::Record(fs) => {
                Rc::new(UType::Record(fs.iter().map(|(lbl, ft)| (lbl.clone(), self.reduce(ft))).collect()))
            }
            UType::Seq(t0) => {
                Rc::new(UType::Seq(self.reduce(t0.as_ref())))
            }
        }
    }

    /// Attempt to fully solve a `UType` until all free metavariables are eliminated
    ///
    /// Returns None if at least one metavariable is irreducible
    fn reify(&self, t: &UType) -> Option<ValueType> {
        match t {
            UType::Var(v) => {
                // FIXME -   is this enough?
                if let Some(t0) = self.solutions.substitutions.get(&v.0) {
                    self.reify(t0.as_ref())
                } else {
                    None
                }
            }
            UType::Ground(g) => {
                Some(ValueType::Ground(*g))
            }
            UType::Tuple(ts) => {
                let mut vts = Vec::with_capacity(ts.len());
                for elt in ts.iter() {
                    vts.push(self.reify(elt.as_ref())?);
                }
                Some(ValueType::Tuple(vts))
            }
            UType::Record(fs) => {
                let mut vfs = Vec::with_capacity(fs.len());
                for (lab, ft) in fs.iter() {
                    vfs.push((lab.clone(), self.reify(ft.as_ref())?));
                }
                Some(ValueType::Record(vfs))
            }
            UType::Seq(t0) => {
                Some(ValueType::Seq(Box::new(self.reify(t0.as_ref())?)))
            }
        }
    }


    fn unify_type(&mut self, left: Rc<UType>, right: Rc<UType>) -> Result<(), TypeError> {
        match (left.as_ref(), right.as_ref()) {
            (UType::Seq(e1), UType::Seq(e2)) => {
                self.unify_type(e1.clone(), e2.clone())
            }
            (UType::Ground(g1), UType::Ground(g2)) => {
                if g1 != g2 {
                    return Err(UnificationError::Unsatisfiable(left, right))
                }
                Ok(())
            }
            (UType::Tuple(ts1), UType::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    return Err(UnificationError::Unsatisfiable(left, right))
                }
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    self.unify_type(t1.clone(), t2.clone())?;
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
                    self.unify_type(f1.clone(), f2.clone())?;
                }
                Ok(())
            }
            (UType::Var(v1), UType::Var(v2)) => {
                if v1 != v2 {
                    self.equivalences[v1.0].insert(v2.0);
                    self.equivalences[v2.0].insert(v1.0);
                }
                Ok(())
            }
            (_, UType::Var(v)) => {
                if let Some(substitution) = self.solutions.substitutions.get(&v.0) {
                    return self.unify_type(left, substitution.clone());
                }
                assert!(!self.infinite_type(*v, left.clone(), true));
                self.solutions.substitutions.insert(v.0, left);
                Ok(())
            }
            (UType::Var(v), _) => {
                if let Some(substitution) = self.solutions.substitutions.get(&v.0) {
                    return self.unify_type(right, substitution.clone());
                }
                assert!(!self.infinite_type(*v, right.clone(), true));
                self.solutions.substitutions.insert(v.0, right);
                Ok(())
            }
            // all the remaining cases are mismatched UType constructors
            _ => Err(UnificationError::Unsatisfiable(left, right)),
        }
    }

    /// Returns `true` if `v := t` describes an infinite type, considering tautologies only in recursive calls
    ///
    /// Does not bother to check any variables other than v while traversing `t`, as those should be ruled out by a theoretical inductive hypothesis
    fn infinite_type(&self, v: UVar, t: Rc<UType>, is_top: bool) -> bool {
        match t.as_ref() {
            UType::Ground(..) => false,
            UType::Var(v0) => {
                if let Some(substitution) = self.solutions.substitutions.get(&v0.0) {
                    if substitution.as_ref() != &UType::Var(*v0) {
                        return self.infinite_type(v, substitution.clone(), false);
                    }
                }
                return v == *v0 && !is_top;
            }
            _ => {
                for sub_t in t.iter_embedded() {
                    if self.infinite_type(v, sub_t, false) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

pub(crate) type TypeError = UnificationError<Rc<UType>>;
pub(crate) type ConstraintError = UnificationError<Constraint>;

impl From<TypeError> for ConstraintError {
    fn from(value: TypeError) -> Self {
        match value {
            UnificationError::Incompatible(ix, lt, rt) => {
                let lc = Constraint::Equiv(lt);
                let rc = Constraint::Equiv(rt);
                UnificationError::Incompatible(ix, lc, rc)
            }
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
    Incompatible(usize, T, T), // two standalone equivalences on a UVar are incompatible with each other
    Unsatisfiable(T, T), // single equivalence cannot be satisfied
}

impl<T: std::fmt::Debug> std::fmt::Display for UnificationError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnificationError::Incompatible(ix, lhs, rhs) => {
                write!(f, "incompatible equivalences `?{ix} = {lhs:?}` && `?{ix} = {rhs:?}`")
            }
            UnificationError::Unsatisfiable(lhs, rhs) => {
                write!(f, "unsatisfiable equivalence  `{lhs:?} = {rhs:?}`")
            }
        }
    }
}

impl<T: std::fmt::Debug> std::error::Error for UnificationError<T> {}
