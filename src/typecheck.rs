use std::{collections::{HashSet, HashMap}, rc::Rc};

use crate::{ValueType, BaseType, Label, Format, FormatModule, Expr};

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct UVar(usize);

/// Unification type, equivalent to ValueType up to abstraction
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum UType {
    Var(UVar), // type-hole
    Base(BaseType),
    Tuple(Vec<Rc<UType>>),
    Record(Vec<(Label, Rc<UType>)>),
    Seq(Rc<UType>),
}

/// Analogue of Mercury-style instantiation states, but for metavariables
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UInst {
    Ground, // No free metavariables
    Bound(HashSet<UVar>),
    Free(UVar),
}

#[derive(Debug, Clone, Default)]
pub enum UScope<'a> {
    #[default]
    Empty,
    Multi(&'a UMultiScope<'a>),
    Single(USingleScope<'a>),
}

impl<'a> UScope<'a> {
    pub fn new() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Clone)]
pub struct UMultiScope<'a> {
    parent: &'a UScope<'a>,
    entries: Vec<(Label, UVar)>,
}

impl<'a> UMultiScope<'a> {
    pub fn new(parent: &'a UScope<'a>) -> Self {
        Self {
            parent,
            entries: Vec::new(),
        }
    }

    pub fn with_capacity(parent: &'a UScope<'a>, capacity: usize) -> Self {
        Self {
            parent,
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn into_record_constraint(self) -> Constraint {
        let mut fields = Vec::new();
        for (label, uv) in self.entries.into_iter() {
            let ut = UType::Var(uv);
            fields.push((label, Rc::new(ut)));
        }
        Constraint::Equiv(Rc::new(UType::Record(fields)))
    }

    pub fn push(&mut self, name: Label, v: UVar) {
        self.entries.push((name, v));
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, UVar)>) {
        for (name, metavar) in self.entries.iter().rev() {
            bindings.push((name.clone(), *metavar));
        }
        self.parent.get_bindings(bindings);
    }

    fn get_uvar_by_name(&self, name: &str) -> Option<UVar> {
        for (n, v) in self.entries.iter().rev() {
            if n == name {
                return Some(*v);
            }
        }
        self.parent.get_uvar_by_name(name)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct USingleScope<'a> {
    parent: &'a UScope<'a>,
    name: &'a str,
    uvar: UVar,
}

impl<'a> USingleScope<'a> {
    pub fn new(parent: &'a UScope<'a>, name: &'a str, uvar: UVar) -> USingleScope<'a> {
        Self {
            parent, name, uvar
        }
    }

    fn get_uvar_by_name(&self, name: &str) -> Option<UVar> {
        if self.name == name {
            return Some(self.uvar);
        }
        self.parent.get_uvar_by_name(name)
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, UVar)>) {
        bindings.push((self.name.into(), self.uvar));
        self.parent.get_bindings(bindings);
    }
}

impl<'a> UScope<'a> {
    fn get_uvar_by_name(&self, name: &str) -> Option<UVar> {
        match self {
            UScope::Empty => None,
            UScope::Multi(multi) => multi.get_uvar_by_name(name),
            UScope::Single(single) => single.get_uvar_by_name(name),
        }
    }

    pub fn get_bindings(&self, bindings: &mut Vec<(Label, UVar)>) {
        match self {
            UScope::Empty => {},
            UScope::Multi(multi) => multi.get_bindings(bindings),
            UScope::Single(single) => single.get_bindings(bindings),
        }
    }
}

impl UType {
    pub fn iter_embedded<'a>(&'a self) -> Box<dyn Iterator<Item = Rc<UType>> + 'a> {
        match self {
            UType::Var(..) | UType::Base(..) => Box::new(std::iter::empty()),
            UType::Tuple(ts) => Box::new(ts.iter().cloned()),
            UType::Record(fs) => Box::new(fs.iter().map(|(_l, t)| t.clone())),
            UType::Seq(t) => Box::new(std::iter::once(t.clone())),
        }

    }

    pub fn get_uinst(&self) -> UInst {
        match self {
            Self::Base(..) => UInst::Ground,
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
pub(crate) enum Constraints {
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
    Elem(Vec<BaseType>), // implicit restriction to a narrowed set of ground-types (e.g. from `Expr::AsU32`)
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
    #[cfg_attr(not(test), allow(dead_code))]
    fn uvar_sanity(&self) -> () {
        assert_eq!(self.constraints.len(), self.equivalences.len());
    }


    /// Constructs a typechecker with initially 0 metavariables.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            equivalences: Vec::new(),
            solutions: Solutions::new(),
        }
    }

    /// Attempts to add a new variant to the implied partial union-type of an existing metavariable.
    ///
    /// Returns `Ok(())` if successful.
    /// Returns `Err(e)` if unification with an identically named, pre-existing variant returned the unification error `e`.
    ///
    /// # Panics
    ///
    /// Will panic if called on a uix value (index stored within a UVar) corresponding to [`Constraints::Invariant`]
    /// constraints-object.
    pub fn add_constraints_variant(&mut self, uix: usize, cname: Label, inner: Rc<UType>) -> Result<(), TypeError> {
        let ref mut cnstrs = self.constraints[uix];
        match cnstrs {
            Constraints::Indefinite => {
                let mut vm = VarMap::new();
                vm.insert(cname, inner);
                *cnstrs = Constraints::Variant(vm);
                Ok(())
            }
            Constraints::Variant(vm) => {
                if let Some(prior) = vm.get(&cname) {
                    self.unify_type(prior.clone(), inner)?;
                } else {
                    vm.insert(cname, inner);
                }
                Ok(())
            }
            Constraints::Invariant(_) => panic!("Cannot add constraint to invariant constraints object (index: {uix})"),
        }
    }

    pub fn register_vars_format_union(&mut self, f: &Vec<Format>, module: &FormatModule) -> Option<usize> {
        let newvar = UVar(self.constraints.len());
    }

    /// Assigns new metavariables and simple constraints
    pub fn register_vars_format(&mut self, f: &Format, module: &FormatModule) -> Option<usize> {
        match f {
            Format::ItemVar(level, args) => {
                if !args.is_empty() {
                    for arg in args.iter() {
                        let _ = self.register_vars_expr(arg);
                    }
                }
                self.register_vars_format(module.get_format(*level), module)
            }
            Format::Fail => ,
            Format::EndOfInput => {},
            Format::Align(_n) => {},
            Format::Byte(_n) => {},
            Format::Variant(cname, inner) => {
                let new_uix = self.constraints.len();
                let mut constraints = Constraints::new();
                let inner_type =
                self.add_constraints_variant(uix, cname, inner);

            }
            Format::Union(_) => todo!(),
            Format::UnionNondet(_) => todo!(),
            Format::Tuple(_) => todo!(),
            Format::Record(_) => todo!(),
            Format::Repeat(_) => todo!(),
            Format::Repeat1(_) => todo!(),
            Format::RepeatCount(_, _) => todo!(),
            Format::RepeatUntilLast(_, _) => todo!(),
            Format::RepeatUntilSeq(_, _) => todo!(),
            Format::Peek(_) => todo!(),
            Format::PeekNot(_) => todo!(),
            Format::Slice(_, _) => todo!(),
            Format::Bits(_) => todo!(),
            Format::WithRelativeOffset(_, _) => todo!(),
            Format::Map(_, _) => todo!(),
            Format::Compute(_) => todo!(),
            Format::Let(_, _, _) => todo!(),
            Format::Match(_, _) => todo!(),
            Format::Dynamic(_, _, _) => todo!(),
            Format::Apply(_) => todo!(),
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
        // check to see if the variable has already been solved before anything else
        if let Some(t) = self.solutions.substitutions.get(&ix) {
            let mut occurs = HashSet::from([ix]);
            let res = self.reduce(t.as_ref(), &mut occurs);
            match t.get_uinst() {
                UInst::Ground => {
                    // variable is fully solved
                    return Ok(true);
                }
                _ => (),
            };
            // we couldn't fully solve the variable, so either a dependent variable was insoluble,
            // or we encountered a tautology/infinite type
            todo!()
        }
        todo!()
    }

    /// Partially solve for as many free metavariables in an input `UType` as possible,
    /// tracking any variables that have already been expanded thus-far to avoid constructing
    /// infinite types or looping on tautologies.
    ///
    /// If a variable has no known solutions, this acts as the identity function
    fn reduce(&self, t: &UType, occurs: &mut HashSet<usize>) -> Rc<UType> {
        match t {
            UType::Var(v) => {
                if !occurs.contains(&v.0) {
                    if let Some(t0) = self.solutions.substitutions.get(&v.0) {
                        occurs.insert(v.0);
                        return self.reduce(t0, occurs);
                    }
                }
                Rc::new(t.clone())
            }
            UType::Base(..) => Rc::new(t.clone()),
            UType::Tuple(ts) => {
                let mut ts0 = Vec::new();
                for elt in ts {
                    ts0.push(self.reduce(elt, occurs));
                }
                Rc::new(UType::Tuple(ts0))
            }
            UType::Record(fs) => {
                let mut fs0 = Vec::new();
                for (lbl, ft) in fs.iter() {
                    fs0.push((lbl.clone(), self.reduce(ft, occurs)))
                }
                Rc::new(UType::Record(fs0))
            }
            UType::Seq(t0) => {
                Rc::new(UType::Seq(self.reduce(t0.as_ref(), occurs)))
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
            UType::Base(g) => {
                Some(ValueType::Base(*g))
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
            (UType::Base(b1), UType::Base(b2)) => {
                if b1 != b2 {
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

    /// Returns `true` if `v := t` describes an infinite type, considering
    /// tautologies only in recursive calls (i.e. `v := Var(v)` at top-level is
    /// not ruled infinite)
    ///
    /// Does not bother to check any variables other than v while traversing
    /// `t`, as those should be ruled out by a theoretical inductive hypothesis
    fn infinite_type(&self, v: UVar, t: Rc<UType>, is_top: bool) -> bool {
        match t.as_ref() {
            UType::Base(..) => false,
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

    fn register_vars_expr(&self, arg: &Expr, ) {
        match arg {
            Expr::Var(_lbl) => {

            }
            Expr::Bool(_) => todo!(),
            Expr::U8(_) => todo!(),
            Expr::U16(_) => todo!(),
            Expr::U32(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::TupleProj(_, _) => todo!(),
            Expr::Record(_) => todo!(),
            Expr::RecordProj(_, _) => todo!(),
            Expr::Variant(_, _) => todo!(),
            Expr::Seq(_) => todo!(),
            Expr::Match(_, _) => todo!(),
            Expr::Lambda(_, _) => todo!(),
            Expr::BitAnd(_, _) => todo!(),
            Expr::BitOr(_, _) => todo!(),
            Expr::Eq(_, _) => todo!(),
            Expr::Ne(_, _) => todo!(),
            Expr::Lt(_, _) => todo!(),
            Expr::Gt(_, _) => todo!(),
            Expr::Lte(_, _) => todo!(),
            Expr::Gte(_, _) => todo!(),
            Expr::Mul(_, _) => todo!(),
            Expr::Div(_, _) => todo!(),
            Expr::Rem(_, _) => todo!(),
            Expr::Shl(_, _) => todo!(),
            Expr::Shr(_, _) => todo!(),
            Expr::Add(_, _) => todo!(),
            Expr::Sub(_, _) => todo!(),
            Expr::AsU8(_) => todo!(),
            Expr::AsU16(_) => todo!(),
            Expr::AsU32(_) => todo!(),
            Expr::U16Be(_) => todo!(),
            Expr::U16Le(_) => todo!(),
            Expr::U32Be(_) => todo!(),
            Expr::U32Le(_) => todo!(),
            Expr::AsChar(_) => todo!(),
            Expr::SeqLength(_) => todo!(),
            Expr::SubSeq(_, _, _) => todo!(),
            Expr::FlatMap(_, _) => todo!(),
            Expr::FlatMapAccum(_, _, _, _) => todo!(),
            Expr::Dup(_, _) => todo!(),
            Expr::Inflate(_) => todo!(),
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



pub(crate) fn typecheck(module: &FormatModule, f: &Format) -> Result<TypeChecker, UnificationError<UType>> {
    let mut tc = TypeChecker::new();
}

pub mod precheck {
    pub enum TFormat {

    }

}
