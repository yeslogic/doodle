use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{precedence::IntransitiveOrd, BaseType, Expr, Format, FormatModule, Label, ValueType};

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct UVar(usize);

impl std::fmt::Display for UVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.0)
    }
}

/// Unification type, equivalent to ValueType up to abstraction
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum UType {
    Empty,     // Reserved for value-free Formats
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
        let mut fields = Vec::with_capacity(self.entries.len());
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
        Self { parent, name, uvar }
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
            UScope::Empty => {}
            UScope::Multi(multi) => multi.get_bindings(bindings),
            UScope::Single(single) => single.get_bindings(bindings),
        }
    }
}

impl UType {
    pub fn iter_embedded<'a>(&'a self) -> Box<dyn Iterator<Item = Rc<UType>> + 'a> {
        match self {
            UType::Empty | UType::Var(..) | UType::Base(..) => Box::new(std::iter::empty()),
            UType::Tuple(ts) => Box::new(ts.iter().cloned()),
            UType::Record(fs) => Box::new(fs.iter().map(|(_l, t)| t.clone())),
            UType::Seq(t) => Box::new(std::iter::once(t.clone())),
        }
    }

    pub fn get_uinst(&self) -> UInst {
        match self {
            Self::Empty | Self::Base(..) => UInst::Ground,
            Self::Var(v) => UInst::Free(*v),
            Self::Record(fs) => {
                let mut vars = HashSet::new();
                for (_l, t) in fs.iter() {
                    match t.get_uinst() {
                        UInst::Free(var) => {
                            let _ = vars.insert(var);
                        }
                        UInst::Bound(vars0) => {
                            vars.extend(vars0.iter());
                        }
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
                        }
                        UInst::Bound(vars0) => {
                            vars.extend(vars0.iter());
                        }
                        UInst::Ground => {}
                    }
                }
                if vars.is_empty() {
                    UInst::Ground
                } else {
                    UInst::Bound(vars)
                }
            }
            Self::Seq(t) => match t.get_uinst() {
                UInst::Ground => UInst::Ground,
                UInst::Bound(vs) => UInst::Bound(vs),
                UInst::Free(v) => UInst::Bound(HashSet::from([v])),
            },
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
    varmaps: VarMapMap, // logically separate table of metacontext variant-maps for indirect aliasing
}

struct VarMapMap {
    store: HashMap<usize, VarMap>,
    next_id: usize,
}

impl VarMapMap {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn as_inner(&self) -> &HashMap<usize, VarMap> {
        &self.store
    }

    pub fn as_inner_mut(&mut self) -> &mut HashMap<usize, VarMap> {
        &mut self.store
    }

    pub fn get_new_id(&mut self) -> VMId {
        let ret = VMId(self.next_id);
        self.next_id += 1;
        ret
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub(crate) struct VMId(usize);

impl std::fmt::Display for VMId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) enum Constraints {
    #[default]
    Indefinite, // default value before union-type distinction is made
    Variant(VMId), // indirect index into typechecker metacontext 'varmap' hashmap
    Invariant(Constraint), // for all type metavariables, inferred non-variant constraint
}

impl Constraints {
    pub fn new() -> Self {
        Self::Indefinite
    }

    pub fn is_variant(&self) -> bool {
        matches!(self, Self::Variant(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Constraint {
    Equiv(Rc<UType>), // direct equivalence with a UType, which should not be a bare `UType::Var` (that is what TypeChecker.equivalences is for)
    Elem(BaseSet), // implicit restriction to a narrowed set of ground-types (e.g. from `Expr::AsU32`)
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constraint::Equiv(ut) => write!(f, "= {ut:?}"),
            Constraint::Elem(bs) => write!(f, "∈ {bs}"),
        }
    }
}

/// abstraction over explicit collections of BaseType values that could be in any order
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) enum BaseSet {
    Single(BaseType),
    UAny, // U8, U16, U32
}

impl BaseSet {
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (BaseSet::Single(b1), BaseSet::Single(b2)) => b1 == b2,
            (BaseSet::Single(b), BaseSet::UAny) | (BaseSet::UAny, BaseSet::Single(b)) => {
                b.is_numeric()
            }
            (BaseSet::UAny, BaseSet::UAny) => true,
        }
    }

    pub fn contains(&self, item: BaseType) -> bool {
        match self {
            BaseSet::Single(elem) => *elem == item,
            BaseSet::UAny => item.is_numeric(),
        }
    }

    pub fn union(&self, other: &Self) -> Result<Self, ConstraintError> {
        match (self, other) {
            (BaseSet::Single(b1), BaseSet::Single(b2)) => {
                if b1 == b2 {
                    Ok(*self)
                } else {
                    Err(ConstraintError::Unsatisfiable(
                        Constraint::Elem(*self),
                        Constraint::Elem(*other),
                    ))
                }
            }
            (BaseSet::UAny, BaseSet::Single(b)) | (BaseSet::Single(b), BaseSet::UAny) => {
                if b.is_numeric() {
                    Ok(BaseSet::Single(*b))
                } else {
                    Err(UnificationError::Unsatisfiable(
                        self.to_constraint(),
                        other.to_constraint(),
                    ))
                }
            }
            (BaseSet::UAny, BaseSet::UAny) => Ok(*self),
        }
    }

    /// Constructs the simplest-possible constraint from `self`, in particular substituting
    /// `Equiv(BaseType(b))` for `Elem(Single(b))`.
    pub fn to_constraint(&self) -> Constraint {
        match self {
            BaseSet::Single(b) => Constraint::Equiv(Rc::new(UType::Base(*b))),
            BaseSet::UAny => Constraint::Elem(*self),
        }
    }
}

impl std::fmt::Display for BaseSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseSet::UAny => write!(f, "{{ U8, U16, U32 }}"),
            BaseSet::Single(t) => write!(f, "{{ {t:?} }}"),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Solutions {
    substitutions: HashMap<usize, Rc<UType>>, // list of non-trivial (i.e. not `?n = ?n`) substitutions based on prior constraint-solving
}

impl Solutions {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
        }
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
            varmaps: VarMapMap::new(),
        }
    }

    /// Attempts to add a direct (non-Variant) constraint to an existing, possibly recently-created UVar
    ///
    /// Will panic if the UVar is pointing to a Variant Constraints structure and therefore cannot have
    /// any constraints added to it directly, but will not otherwise attempt to check for mutual satisfiability
    /// with other constraints on that variable, or any other it is aliased or equated to.
    fn unify_var_constraint(&mut self, uvar: UVar, constraint: Constraint) -> TCResult<Constraint> {
        let uix = uvar.0;
        let constraints = &mut self.constraints[uix];
        match constraints {
            Constraints::Indefinite => {
                let ret = constraint.clone();
                self.constraints[uix] = Constraints::Invariant(constraint);
                Ok(ret)
            }
            Constraints::Variant(vmid) => Err(TCError::VarianceMismatch(uvar, *vmid, constraint)),
            Constraints::Invariant(constraint) => match constraint {
                Constraint::Equiv(_) => todo!(),
                Constraint::Elem(_) => todo!(),
            },
        }
    }

    /// Much like [`add_constraint_nocheck`], only with best-effort mutual compatibility checks on the existing
    /// constraints, but not unbounded transitive equivalence checks since those can grow costly with quadratic
    /// performance or worse.
    fn add_constraint_check(&mut self, uvar: UVar, constraint: Constraint) -> TCResult<()> {
        let uix = uvar.0;
        let constraints = &mut self.constraints[uix];
        match constraints {
            Constraints::Indefinite => {
                let invariant = Constraints::Invariant(constraint);
                self.constraints[uix] = invariant;
            }
            Constraints::Variant(_) => {
                panic!("cannot add Constraint to a Constraints::Variant object (on {uvar})")
            }
            Constraints::Invariant(other) => {
                let new_constraint = self.unify_constraints(&constraint, &other)?;
                if &new_constraint != &*other {
                    *other = new_constraint;
                }
            }
        }
        Ok(())
    }

    fn init_varmap(&mut self) -> VMId {
        self.varmaps.get_new_id()
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
    pub fn add_constraints_variant(
        &mut self,
        uix: usize,
        cname: Label,
        inner: Rc<UType>,
    ) -> Result<(), TypeError> {
        let ref mut cnstrs = self.constraints[uix];
        match cnstrs {
            Constraints::Indefinite => {
                let id = self.init_varmap();
                let vm = self.varmaps.as_inner_mut().entry(id.0).or_default();
                vm.insert(cname, inner);
                *cnstrs = Constraints::Variant(id);
                Ok(())
            }
            Constraints::Variant(vmid) => {
                let vm = self
                    .varmaps
                    .as_inner_mut()
                    .get_mut(&vmid.0)
                    .expect("missing varmap for {vmid}");
                if let Some(prior) = vm.get(&cname) {
                    let updated = self.unify_type(prior.clone(), inner)?;
                    if updated.as_ref() != prior.as_ref() {
                        vm.insert(cname, updated);
                    }
                } else {
                    vm.insert(cname, inner);
                }
                Ok(())
            }
            Constraints::Invariant(_) => {
                panic!("Cannot add constraint to invariant constraints object (index: {uix})")
            }
        }
    }

    pub fn infer_utype_format_union(
        &mut self,
        branches: &[Format],
        module: &FormatModule,
    ) -> TCResult<UVar> {
        let newvar = UVar(self.constraints.len());
        // populate new structures for each relevant cross-indexed vector
        self.constraints.push(Constraints::new());
        self.equivalences.push(HashSet::new());

        for f in branches.into_iter() {
            match f {
                Format::Variant(lbl, inner) => {
                    let typ = self.infer_utype_format(inner.as_ref(), module)?;
                    self.add_constraints_variant(newvar.0, lbl.clone(), typ);
                }
                // FIXME - other is probably going to come up but it should be easy to fix later, this handles the hardest known case
                other => {
                    unreachable!("register_vars_format_union: found non-variant branch {other:?}")
                }
            }
        }
        Ok(newvar)
    }

    fn init_var_simple(&mut self, typ: UType) -> TCResult<Rc<UType>> {
        let newvar = self.get_new_uvar();
        let rc = Rc::new(typ);
        let constr = Constraint::Equiv(rc.clone());
        self.unify_var_constraint(newvar, constr)?;
        // FIMXE - not sure whether to return rc or newvar
        Ok(rc)
    }

    /// Assigns new metavariables and simple constraints for a format, and returns the most specific UType possible,
    /// which in many cases will be a Var pointing to a novel UVar.
    pub fn infer_utype_format(&mut self, f: &Format, module: &FormatModule) -> TCResult<Rc<UType>> {
        match f {
            Format::ItemVar(level, args) => {
                let newvar = self.get_new_uvar();
                if !args.is_empty() {
                    let scope = UScope::new();
                    for arg in args.iter() {
                        let _ = self.infer_utype_expr(arg, &scope)?;
                    }
                }
                let ret = self.infer_utype_format(module.get_format(*level), module)?;
                self.add_constraint_check(newvar, Constraint::Equiv(ret.clone()))?;
                Ok(ret)
            }
            Format::Fail | Format::EndOfInput | Format::Align(_) => {
                self.init_var_simple(UType::Empty)
            }
            Format::Byte(_n) => self.init_var_simple(UType::Base(BaseType::U8)),
            Format::Variant(cname, inner) => {
                todo!("register_vars_format: Variant case unhandled (not sure what to do)");
            }
            Format::Union(branches) => {
                let var = self.infer_utype_format_union(branches, module)?;
                Ok(Rc::new(UType::Var(var)))
            }
            Format::UnionNondet(lbl_branches) => {
                let var = self.infer_utype_format_union_nondet(lbl_branches, module)?;
                Ok(Rc::new(UType::Var(var)))
            }
            Format::Tuple(ts) => {
                let mut uts = Vec::with_capacity(ts.len());
                for t in ts {
                    uts.push(self.infer_utype_format(t, module)?);
                }
                Ok(Rc::new(UType::Tuple(uts)))
            }
            Format::Record(fs) => {
                let mut ufs = Vec::with_capacity(fs.len());
                for (lbl, f) in fs {
                    ufs.push((lbl.clone(), self.infer_utype_format(f, module)?));
                }
                Ok(Rc::new(UType::Record(ufs)))
            }
            // FIXME - logically these should be grouped together, but anything containing an expression has to be typed as a special-case
            Format::Repeat(inner) | Format::Repeat1(inner) => {
                let t = self.infer_utype_format(inner, module)?;
                Ok(Rc::new(UType::Seq(t)))
            }
            Format::RepeatCount(n, inner) => {
                let scope = UScope::new();
                let n_type = self.infer_utype_expr(n, &scope)?;
                // NOTE : we don't care about the constraint, only whether it was successfully computed
                let _constraint = self.unify_type_baseset(n_type, BaseSet::UAny)?;
                let inner_type = self.infer_utype_format(inner, module)?;
                Ok(UType::Seq(inner_type))
            }
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
    /// If a variable has no known solutions, this acts as the identity function.
    ///
    /// Any variables that cannot be expanded without invoking tautologies or infinite types
    /// are left unaltered in-place.
    fn reduce(&self, t: &UType, occurs: &mut HashSet<usize>) -> Rc<UType> {
        match t {
            UType::Var(v) => {
                if !occurs.contains(&v.0) {
                    if let Some(t0) = self.solutions.substitutions.get(&v.0) {
                        occurs.insert(v.0);
                        return self.reduce(t0, occurs);
                    }
                } else {
                }
                Rc::new(t.clone())
            }
            UType::Empty | UType::Base(..) => Rc::new(t.clone()),
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
            UType::Seq(t0) => Rc::new(UType::Seq(self.reduce(t0.as_ref(), occurs))),
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
            UType::Base(g) => Some(ValueType::Base(*g)),
            UType::Empty => Some(ValueType::Empty),
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
            UType::Seq(t0) => Some(ValueType::Seq(Box::new(self.reify(t0.as_ref())?))),
        }
    }

    fn get_new_uvar(&mut self) -> UVar {
        let ret = UVar(self.constraints.len());
        self.constraints.push(Constraints::new());
        self.equivalences.push(HashSet::new());
        ret
    }

    fn unify_type(&mut self, left: Rc<UType>, right: Rc<UType>) -> TCResult<Rc<UType>> {
        match (left.as_ref(), right.as_ref()) {
            (UType::Seq(e1), UType::Seq(e2)) => {
                if e1 == e2 {
                    Ok(left)
                } else {
                    let inner = self.unify_type(e1, e2)?;
                    Ok(Rc::new(UType::Seq(inner)))
                }
            }
            (UType::Base(b1), UType::Base(b2)) => {
                if b1 != b2 {
                    return Err(UnificationError::Unsatisfiable(left, right).into());
                }
                Ok(left)
            }
            (UType::Tuple(ts1), UType::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    return Err(UnificationError::Unsatisfiable(left, right).into());
                }
                if ts1 == ts2 {
                    return Ok(left);
                }
                let mut ts0 = Vec::with_capacity(ts1.len());
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    ts0.push(self.unify_type(t1.clone(), t2.clone())?);
                }
                Ok(Rc::new(UType::Tuple(ts0)))
            }
            (UType::Record(fs1), UType::Record(fs2)) => {
                if fs1.len() != fs2.len() {
                    return Err(UnificationError::Unsatisfiable(left.clone(), right.clone()));
                }
                if fs1 == fs2 {
                    return Ok(left);
                }
                let mut fs0 = Vec::with_capacity(fs1.len());
                for ((l1, f1), (l2, f2)) in Iterator::zip(fs1.iter(), fs2.iter()) {
                    if l1 != l2 {
                        return Err(
                            UnificationError::Unsatisfiable(left.clone(), right.clone()).into()
                        );
                    }
                    fs0.push((l1.clone(), self.unify_type(f1.clone(), f2.clone())?));
                }
                Ok(Rc::new(UType::Record(fs0)))
            }
            (UType::Var(v1), UType::Var(v2)) => {
                self.equate_uvars(v1, v2)?;
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

    /// Assigns a 'solution' (UType substitution and equivalence-constraint) to a UVar
    ///
    /// Does not perform transitive inference or checking, due to quadratic cost as linkages grow.
    fn assign_solution(&mut self, uvar: UVar, solution: Rc<UType>) -> TCResult<()> {
        let ref mut constraints = self.constraints[uvar.0];
        match constraints.as_ref() {
            Constraints::Indefinite => {
                *constraints = Constraint::Equiv(solution);
                Ok(())
            }
            Constraints::Variant(vmid) => Err(TCError::VarianceMismatch(
                uvar,
                vmid,
                Constraint::Equiv(solution),
            )),
            Constraints::Invariant(ref mut constraint) => match constraint {
                Constraint::Equiv(ut) => {
                    let unified = self.unify_type(ut.clone(), solution)?;
                    *constraint = Constraint::Equiv(unified);
                    Ok(())
                }
                Constraint::Elem(bs) => {
                    let new_constraint = self.unify_type_baseset(solution, *bs)?;
                    *constraint = new_constraint;
                    Ok(())
                }
            },
        }
    }

    fn equate_uvars(&self, v1: UVar, v2: UVar) -> Result<(), ConstraintError> {
        if v1 == v2 {
            return Ok(());
        }

        // short-circuit if already equated
        match (
            self.equivalences[v1.0].contains(&v2.0),
            self.equivalences[v2.0].contains(&v1.0),
        ) {
            (true, true) => return Ok(()),
            (false, false) => (),
            _ => unreachable!(
                "uvar equivalence disagreement: only one of {v1}, {v2} are equated to the other"
            ),
        }

        let c1 = &mut self.constraints[v1.0];
        let c2 = &mut self.constraints[v2.0];

        match (c1, c2) {
            (Constraints::Indefinite, Constraints::Indefinite) => {
                // nothing more can be known at this point
            }
            (Constraints::Indefinite, _) => {
                *c1 = *c2;
            }
            (_, Constraints::Indefinite) => {
                *c2 = *c1;
            }
            (Constraints::Variant(vmid1), Constraints::Variant(vmid2)) => {
                self.unify_varmaps(*v1, *vmid1, *v2, *vmid2)?;
            }
            (Constraints::Variant(vmid), Constraints::Invariant(c))
            | (Constraints::Invariant(c), Constraints::Variant(vmid)) => return Err(
                TCError::VarianceMismatch(Ord::min(*v1, *v2), *vmid, c.clone()),
            ),
            (Constraints::Invariant(c1), Constraints::Invariant(c2)) => {
                let c0 = self.unify_constraints(c1, c2)?;
                *c1 = c0.clone();
                *c2 = c0.clone();
            }
        }

        self.equivalences[v1.0].insert(v2.0);
        self.equivalences[v2.0].insert(v1.0);
        Ok(())
    }

    fn infer_utype_expr<'a>(&self, e: &Expr, scope: &'a UScope<'a>) -> TCResult<Rc<UType>> {
        match e {
            Expr::Var(lbl) => match scope.get_uvar_by_name(lbl) {
                Some(uv) => Ok(Rc::new(UType::Var(uv))),
                None => {
                    unreachable!("encountered unset variable: {lbl}");
                }
            },
            Expr::Bool(_) => Ok(Rc::new(UType::Base(BaseType::Bool))),
            Expr::U8(_) => Ok(Rc::new(UType::Base(BaseType::U8))),
            Expr::U16(_) => Ok(Rc::new(UType::Base(BaseType::U16))),
            Expr::U32(_) => Ok(Rc::new(UType::Base(BaseType::U32))),
            Expr::Tuple(ts) => {
                let newvar = self.get_new_uvar();
                let mut uts = Vec::with_capacity(ts.len());
                for t in ts {
                    uts.push(self.register_vars_expr(t, scope));
                }
                let ret = Rc::new(UType::Tuple(uts));
                self.assign_solution(newvar, ret.clone());
                Ok(ret)
            }
            Expr::TupleProj(_, _) => todo!(),
            Expr::Record(fs) => {}
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

    fn check_compatible(
        &self,
        uvar: UVar,
        constraint: &Constraint,
        other: &Constraint,
    ) -> Result<(), ConstraintError> {
        match (constraint, other) {
            (Constraint::Equiv(u1), Constraint::Equiv(u2)) => {
                self.unify_type(u1.clone(), u2.clone())?;
                Ok(())
            }
            (Constraint::Elem(bs1), Constraint::Elem(bs2)) => {
                if bs1.intersects(bs2) {
                    Ok(())
                } else {
                    return Err(ConstraintError::Incompatible(
                        uvar,
                        constraint.clone(),
                        other.clone(),
                    ));
                }
            }
            (Constraint::Elem(s), Constraint::Equiv(t))
            | (Constraint::Equiv(t), Constraint::Elem(s)) => {
                match t.as_ref() {
                    UType::Var(_) => Ok(()), // we aren't going to precheck every variable transitively this early
                    UType::Base(b) if s.contains(*b) => Ok(()),
                    // TODO : if BaseSet or Elem ever change, there may be valid cases behind this catch-all fail
                    _ => Err(UnificationError::Incompatible(
                        uvar,
                        constraint.clone(),
                        other.clone(),
                    )),
                }
            }
        }
    }

    fn unify_type_baseset(&self, ut: Rc<UType>, bs: BaseSet) -> TCResult<Constraint> {
        match ut.as_ref() {
            UType::Var(uv) => {
                let constraint = bs.to_constraint();
                let ret = self.unify_var_constraint(*uv, constraint)?;
                Ok(ret)
            }
            UType::Base(b) => {
                let ret = bs.union(&BaseSet::Single(*b))?.to_constraint();
                Ok(ret)
            }
            other => Err(UnificationError::Unsatisfiable(
                Constraint::Equiv(ut),
                bs.to_constraint(),
            )
            .into()),
        }
    }

    fn unify_constraints(&self, cs1: &Constraint, cs2: &Constraint) -> TCResult<Constraint> {
        match (cs1, cs2) {
            (Constraint::Equiv(t1), Constraint::Equiv(t2)) => {
                let t0 = self.unify_type(t1.clone(), t2.clone())?;
                Ok(Constraint::Equiv(t0))
            }
            (Constraint::Equiv(ut), Constraint::Elem(bs))
            | (Constraint::Elem(bs), Constraint::Equiv(ut)) => self.unify_type_baseset(ut, bs),
            (Constraint::Elem(bs1), Constraint::Elem(bs2)) => {
                if bs1 == bs2 {
                    Ok(Constraint::Equiv(*bs1))
                } else if bs1.intersects(bs2) {
                    bs1.union(bs2);
                }
            }
        }
    }

    fn alias_uvar_vmid(&mut self, uvar: UVar, vmid: VMId) -> TCResult<()> {
        let ref mut constrs = self.constraints[uvar.0];
        match constrs {
            Constraints::Variant(other) => {
                // FIXME - do we care about the value of other, or can we ignore it
                *other = vmid;
                Ok(())
            }
            Constraints::Invariant(orig) => {
                Err(TCError::VarianceMismatch(uvar, vmid, orig.clone()))
            }
            Constraints::Indefinite => {
                *constrs = Constraints::Variant(vmid);
                Ok(())
            }
        }
    }

    fn unify_varmaps(&self, v1: UVar, vmid1: VMId, v2: UVar, vmid2: VMId) -> TCResult<VMId> {
        if (vmid1 == vmid2) {
            return Ok(vmid1);
        }

        let (i, j) = if (vmid1 < vmid2) {
            (vmid1, vmid2)
        } else {
            (vmid2, vmid1)
        };

        let vm_i = self
            .varmaps
            .get_mut(&i.0)
            .unwrap_or_else(|| unreachable!("missing {i}"));
        let vm_j = self
            .varmaps
            .get_mut(&j.0)
            .unwrap_or_else(|| unreachable!("missing {j}"));

        // keep the earlier one, prune the later one, after computing the union
        for (vname, inner) in vm_j.drain() {
            if vm_i.contains_key(&vname) {
                let t_i = vm_i
                    .get(&vname)
                    .unwrap_or_else(|| unreachable!("cannot get key we already know is there"));
                let t_j = inner;
                let unified = self.unify_type(t_i, t_j)?;
                let _ = vm_i.insert(vname, unified);
            } else {
                vm_i.insert(vname, inner);
            }
        }

        // re-alias the two vars to vm_i (since we don't remember which is already pointing to it)

        // delete varmap j for potential re-use, even though we probably won't reuse it
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
            UnificationError::VarianceMismatch(_, _, _) => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
// Generic error in unification between two type-constraints, which are represented generically
pub enum UnificationError<T: std::fmt::Debug> {
    Incompatible(UVar, T, T), // two independent assertions about a UVar are incompatible
    Unsatisfiable(T, T),      // a single non-variable assertion is directly unsatisfiable
}

#[derive(Clone, Debug)]
pub enum TCError {
    VarianceMismatch(UVar, VMId, Constraint), // attempted unification of a variant and non-variant constraint
    Unification(ConstraintError),
}

impl From<TypeError> for TCError {
    fn from(value: TypeError) -> Self {
        Self::Unification(value.into())
    }
}

impl From<ConstraintError> for TCError {
    fn from(value: ConstraintError) -> Self {
        Self::Unification(value)
    }
}

impl std::fmt::Display for TCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TCError::VarianceMismatch(uv, vmid, constraint) => write!(
                f,
                "unable to proceed after attempted unification `{uv} {constraint} ∧ {uv} ⊇ {vmid}`"
            ),
            TCError::Unification(c_err) => write!(f, "{c_err}"),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for UnificationError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnificationError::Incompatible(var, lhs, rhs) => {
                write!(
                    f,
                    "incompatible equivalences `{var} = {lhs:?}` && `{var} = {rhs:?}`"
                )
            }
            UnificationError::Unsatisfiable(lhs, rhs) => {
                write!(f, "unsatisfiable equivalence  `{lhs:?} = {rhs:?}`")
            }
        }
    }
}

impl<T: std::fmt::Debug> std::error::Error for UnificationError<T> {}

pub(crate) fn typecheck(
    module: &FormatModule,
    f: &Format,
) -> Result<TypeChecker, UnificationError<UType>> {
    let mut tc = TypeChecker::new();
    let ut = tc.infer_utype_format(f, module);
    // FIXME - there should be a lot more that goes on under the covers here, especially since we want to detect errors
    Ok(tc)
}

pub(crate) type TCResult<T> = Result<T, TCError>;
