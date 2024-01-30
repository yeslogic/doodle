use std::{ collections::{HashMap, HashSet}, rc::Rc, borrow::Cow };
use crate::{precedence::IntransitiveOrd, Arith, IntRel, BaseType, Expr, Format, FormatModule, Label, ValueType, Pattern};

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct UVar(usize);



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
pub(crate) enum UScope<'a> {
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
pub(crate) struct UMultiScope<'a> {
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
        Constraint::Equiv(self.into_record_utype())
    }

    pub fn into_record_utype(self) -> Rc<UType> {
        let mut fields = Vec::with_capacity(self.entries.len());
        for (label, uv) in self.entries.into_iter() {
            let ut = UType::Var(uv);
            fields.push((label, Rc::new(ut)));
        }
        Rc::new(UType::Record(fields))
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
pub(crate) struct USingleScope<'a> {
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
        bindings.push((String::from(self.name).into(), self.uvar));
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
    /// Note that transitive inclusion is not checked, as this requires a typechecker context to evaluate
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
    Base(BaseSet),
    Concrete(Rc<ValueType>),
    Abstract(Rc<UType>),
    IndefiniteUnion(Rc<VarMap>),
}

pub(crate) struct TypeChecker {
    constraints: Vec<Constraints>,
    aliases: Vec<Alias>, // set of non-identity metavariables that are aliased to ?ix
    varmaps: VarMapMap, // logically separate table of metacontext variant-maps for indirect aliasing
}

#[derive(Clone, Debug, Default)]
enum Alias {
    #[default]
    NoAlias, // no aliases anywhere
    BackRef(usize), // direct back-ref to earliest alias (must be canonical)
    Canonical(HashSet<usize>) // list of forward-references to update if usurped by an earlier canonical alias
}

impl Alias {
    /// New, empty alias-set
    pub const fn new() -> Alias {
        Self::NoAlias
    }

    pub const fn is_canonical(&self) -> bool {
        matches!(self, Alias::Canonical(_) | Alias::NoAlias)
    }

    pub fn as_backref(&self) -> Option<usize> {
        match self {
            Alias::NoAlias | Alias::Canonical(_) => None,
            Alias::BackRef(ix) => Some(*ix),
        }
    }

    /// Adds a forward reference to a canonical-form Alias, forcing it to be [`Alias::Canonical`] if it is not already
    ///
    /// # Panics
    ///
    /// Will panic if `self` is [`Alias::BackRef`]
    fn add_forward_ref(&mut self, tgt: usize) {
        match self {
            Alias::NoAlias => {
                std::mem::replace(self, Alias::Canonical(HashSet::from([tgt])));
            }
            Alias::BackRef(_) => panic!("cannot add forward-ref to Alias::BackRef"),
            Alias::Canonical(fwds) => {
                fwds.insert(tgt);
            }
        }
    }

    /// Overwrites an Alias to be [`Alias::BackRef`] pointing to the specified index,
    /// returning its old value.
    fn set_backref(&mut self, tgt: usize) -> Alias {
        std::mem::replace(self, Alias::BackRef(tgt))
    }

    fn iter_fwd_refs<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        match self {
            Alias::NoAlias | Alias::BackRef(_) => Box::new(std::iter::empty()),
            Alias::Canonical(fwds) => {
                Box::new(fwds.iter().copied())
            }
        }
    }

    fn contains_fwd_ref(&self, tgt: usize) -> bool {
        match self {
            Alias::NoAlias | Alias::BackRef(_) => false,
            Alias::Canonical(fwds) => fwds.contains(&tgt),
        }
    }
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

    pub fn get_varmap(&self, id: VMId) -> &VarMap {
        self.store.get(&id.0).unwrap_or_else(|| unreachable!("missing varmap for {id}"))
    }

    pub fn get_varmap_mut(&mut self, id: VMId) -> &mut VarMap {
        self.store.get_mut(&id.0).unwrap_or_else(|| unreachable!("missing varmap for {id}"))
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

type VarMap = HashMap<Label, Rc<UType>>;

impl TypeChecker {
    #[cfg_attr(not(test), allow(dead_code))]
    fn uvar_sanity(&self) -> () {
        assert_eq!(self.constraints.len(), self.aliases.len());
    }

    /// Constructs a typechecker with initially 0 metavariables.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            aliases: Vec::new(),
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
        let constraints = &self.constraints[uix];
        match constraints {
            Constraints::Indefinite => {
                let ret = constraint.clone();
                self.constraints[uix] = Constraints::Invariant(constraint);
                Ok(ret)
            }
            Constraints::Variant(vmid) => Err(TCError::VarianceMismatch(uvar, *vmid, constraint)),
            Constraints::Invariant(prior) => {
                let ret = self.unify_constraints(prior, &constraint)?;
                self.constraints[uix] = Constraints::Invariant(ret.clone());
                Ok(ret)
            }
        }
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
    ) -> TCResult<()> {
        let ref cnstrs = self.constraints[uix];
        match cnstrs {
            Constraints::Indefinite => {
                let id = self.init_varmap();
                let vm = self.varmaps.as_inner_mut().entry(id.0).or_default();
                vm.insert(cname, inner);
                self.constraints[uix] = Constraints::Variant(id);
                Ok(())
            }
            Constraints::Variant(vmid) => {
                let vm = self.varmaps.get_varmap(vmid);
                if let Some(prior) = vm.get(&cname) {
                    let updated = self.unify_utype(prior.clone(), inner)?;
                    if updated.as_ref() != prior.as_ref() {
                        self.varmaps.get_varmap_mut(vmid).insert(cname, updated);
                    }
                } else {
                    self.varmaps.get_varmap_mut(vmid).insert(cname, inner);
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
        self.aliases.push(Alias::default());

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

    fn init_var_simple(&mut self, typ: UType) -> TCResult<(UVar, Rc<UType>)> {
        let newvar = self.get_new_uvar();
        let rc = Rc::new(typ);
        let constr = Constraint::Equiv(rc.clone());
        self.unify_var_constraint(newvar, constr)?;
        // FIMXE - not sure whether to return rc or newvar
        Ok((newvar, rc))
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
                self.unify_var_constraint(newvar, Constraint::Equiv(ret.clone()))?;
                Ok(ret)
            }
            Format::Fail | Format::EndOfInput | Format::Align(_) => {
                Ok(self.init_var_simple(UType::Empty)?.1)
            }
            Format::Byte(_n) => Ok(self.init_var_simple(UType::Base(BaseType::U8))?.1),
            Format::Variant(cname, inner) => {
                todo!("register_vars_format: Variant case unhandled (not sure what to do)");
            }
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let var = self.infer_utype_format_union(branches, module)?;
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
                let _constraint = self.unify_utype_baseset(n_type, BaseSet::UAny)?;
                let inner_type = self.infer_utype_format(inner, module)?;
                Ok(Rc::new(UType::Seq(inner_type)))
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

    /// Attempt to fully solve a `UType` until all free metavariables are eliminated
    ///
    /// Returns None if at least one metavariable is irreducible
    fn reify(&self, t: Rc<UType>) -> Option<ValueType> {
        match t.as_ref() {
            &UType::Var(v) => {
                // FIXME -   is this enough?
                match self.substitute_uvar_utype(v) {
                    Ok(Some(t0)) => self.reify(t0),
                    _ => None,
                }
            }
            UType::Base(g) => Some(ValueType::Base(*g)),
            UType::Empty => Some(ValueType::Empty),
            UType::Tuple(ts) => {
                let mut vts = Vec::with_capacity(ts.len());
                for elt in ts.iter() {
                    vts.push(self.reify(elt.clone())?);
                }
                Some(ValueType::Tuple(vts))
            }
            UType::Record(fs) => {
                let mut vfs = Vec::with_capacity(fs.len());
                for (lab, ft) in fs.iter() {
                    vfs.push((lab.clone(), self.reify(ft.clone())?));
                }
                Some(ValueType::Record(vfs))
            }
            UType::Seq(t0) => Some(ValueType::Seq(Box::new(self.reify(t0.clone())?))),
        }
    }

    fn get_new_uvar(&mut self) -> UVar {
        let ret = UVar(self.constraints.len());
        self.constraints.push(Constraints::new());
        self.aliases.push(Alias::new());
        ret
    }

    fn unify_utype(&mut self, left: Rc<UType>, right: Rc<UType>) -> TCResult<Rc<UType>> {
        match (left.as_ref(), right.as_ref()) {
            (UType::Seq(e1), UType::Seq(e2)) => {
                if e1 == e2 {
                    Ok(left)
                } else {
                    let inner = self.unify_utype(e1.clone(), e2.clone())?;
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
                    ts0.push(self.unify_utype(t1.clone(), t2.clone())?);
                }
                Ok(Rc::new(UType::Tuple(ts0)))
            }
            (UType::Record(fs1), UType::Record(fs2)) => {
                if fs1.len() != fs2.len() {
                    return Err(UnificationError::Unsatisfiable(left.clone(), right.clone()).into());
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
                    fs0.push((l1.clone(), self.unify_utype(f1.clone(), f2.clone())?));
                }
                Ok(Rc::new(UType::Record(fs0)))
            }
            (&UType::Var(v1), &UType::Var(v2)) => {
                self.equate_uvars(v1, v2)?;
                Ok(Rc::new(UType::Var(Ord::min(v1, v2))))
            }
            (&UType::Var(v), _) => {
                let constraint = Constraint::Equiv(right.clone());
                let after = self.unify_var_constraint(v, constraint)?;
                self.occurs(v)?;
                match after {
                    Constraint::Equiv(t) => Ok(t.clone()),
                    Constraint::Elem(_) => Ok(left.clone()),
                }
            }
            (_, &UType::Var(v)) => {
                let constraint = Constraint::Equiv(right.clone());
                let after = self.unify_var_constraint(v, constraint)?;
                self.occurs(v)?;
                match after {
                    Constraint::Equiv(t) => Ok(t.clone()),
                    Constraint::Elem(_) => Ok(right.clone()),
                }
            }
            // all the remaining cases are mismatched UType constructors
            _ => Err(UnificationError::Unsatisfiable(left, right).into()),
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
            &UType::Var(v0) => {
                self.occurs(v0).is_err()
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
    fn unify_var_utype(&mut self, uvar: UVar, solution: Rc<UType>) -> TCResult<()> {
        let ref mut constraints = self.constraints[uvar.0];
        match constraints {
            Constraints::Indefinite => {
                *constraints = Constraints::Invariant(Constraint::Equiv(solution));
                Ok(())
            }
            &mut Constraints::Variant(vmid) => Err(TCError::VarianceMismatch(
                uvar,
                vmid,
                Constraint::Equiv(solution),
            )),
            Constraints::Invariant(ref mut constraint) => match constraint {
                Constraint::Equiv(ut) => {
                    let unified = self.unify_utype(ut.clone(), solution)?;
                    *constraint = Constraint::Equiv(unified);
                    Ok(())
                }
                Constraint::Elem(bs) => {
                    let new_constraint = self.unify_utype_baseset(solution, *bs)?;
                    *constraint = new_constraint;
                    Ok(())
                }
            },
        }
    }

    fn unify_vars_constraints(&mut self, v1: UVar, v2: UVar) -> TCResult<&Constraints> {
        if v1 == v2 {
            return Ok(&self.constraints[v1.0]);
        }

        let c1 = &mut self.constraints[v1.0];
        let c2 = &mut self.constraints[v2.0];

        match (c1, c2) {
            (Constraints::Indefinite, Constraints::Indefinite) => {
                Ok(c1)
            }
            (Constraints::Indefinite, _) => {
                *c1 = *c2;
                Ok(c2)
            }
            (_, Constraints::Indefinite) => {
                *c2 = *c1;
                Ok(c1)
            }
            (Constraints::Variant(vmid1), Constraints::Variant(vmid2)) => {
                let _ = self.unify_varmaps(v1, vmid1, v2, vmid2)?;
                Ok(c1)
            }
            (Constraints::Variant(vmid), Constraints::Invariant(c))
            | (Constraints::Invariant(c), Constraints::Variant(vmid)) => return Err(
                TCError::VarianceMismatch(Ord::min(v1, v2), *vmid, c.clone()),
            ),
            (Constraints::Invariant(c1), Constraints::Invariant(c2)) => {
                let c0 = self.unify_constraints(c1, c2)?;
                *c1 = c0.clone();
                *c2 = c0.clone();
                Ok(&self.constraints[v1.0])
            }
        }
    }

    fn equate_uvars(&self, v1: UVar, v2: UVar) -> TCResult<()> {
        if v1 == v2 {
            return Ok(());
        }

        let ref mut a1 = self.aliases[v1.0];
        let ref mut a2 = self.aliases[v2.0];

        // short-circuit if already equated
        match (a1, a2) {
            (Alias::NoAlias, Alias::NoAlias) => {
                let c0 = self.unify_vars_constraints(v1, v2);
                if v1 < v2 {
                    a1.add_forward_ref(v2.0);
                    let _ = a2.set_backref(v1.0);
                } else {
                    a2.add_forward_ref(v1.0);
                    let _ = a1.set_backref(v2.0);
                }
                Ok(())
            }
            (Alias::NoAlias, Alias::BackRef(tgt)) => {
                let tmp = *tgt;
                if v1.0 > tmp {
                    a1.set_backref(tmp);
                } else if v1.0 < tmp {
                    // find the old canonical pointee
                    let mid = &mut self.aliases[tmp];
                    // it must be Canonical or else we have a problem
                    assert!(matches!(mid, Alias::Canonical(..)), "backref to non-canonical alias ?{tmp}: {mid:?}");
                    // it must not contain v1.0 already, or else something is *very* wrong
                    assert!(!mid.contains_fwd_ref(v1.0));
                    // iteratively update all backreferences to v1
                    for a in mid.iter_fwd_refs() {
                        self.aliases[a].set_backref(v1.0);
                    }
                    *a1 = mid.set_backref(v1.0);
                } else {
                    panic!("BackRef ({v1}<-{v2}) conflicts with NoAlias ({v1}-|)");
                }
                Ok(())
            }
            (Alias::BackRef(tgt), Alias::NoAlias) => {
                let tmp = *tgt;
                if v2.0 > tmp {
                    a1.set_backref(tmp);
                } else if v2.0 < tmp {
                    // find the old canonical pointee
                    let mid = &mut self.aliases[tmp];
                    // it must be Canonical or else we have a problem
                    assert!(matches!(mid, Alias::Canonical(..)), "backref to non-canonical alias ?{tmp}: {mid:?}");
                    // it must not contain v1.0 already, or else something is *very* wrong
                    assert!(!mid.contains_fwd_ref(v2.0));
                    // iteratively update all backreferences to v1
                    for a in mid.iter_fwd_refs() {
                        self.aliases[a].set_backref(v2.0);
                    }
                    *a1 = mid.set_backref(v2.0);
                } else {
                    panic!("BackRef ({v2}<-{v1}) conflicts with NoAlias ({v2}-|)");
                }
                Ok(())
            }
            (Alias::NoAlias, Alias::Canonical(fwds)) => {
                if v1.0 < v2.0 {
                    assert!(!a2.contains_fwd_ref(v1.0));
                    // iteratively update all backreferences to v1
                    let mid = a2;
                    for a in mid.iter_fwd_refs() {
                        self.aliases[a].set_backref(v1.0);
                    }
                    *a1 = a2.set_backref(v1.0);
                    a1.add_forward_ref(v2.0);
                } else {
                    a1.set_backref(v2.0);
                    a2.add_forward_ref(v1.0);
                }
                Ok(())
            }
            (Alias::Canonical(fwds), Alias::NoAlias) => {
                if v1.0 < v2.0 {
                     assert!(!a1.contains_fwd_ref(v2.0));
                    // iteratively update all backreferences to v1
                    let mid = a1;
                    for a in mid.iter_fwd_refs() {
                        self.aliases[a].set_backref(v2.0);
                    }
                    *a2 = a1.set_backref(v2.0);
                    a2.add_forward_ref(v1.0);
                } else {
                    a2.set_backref(v1.0);
                    a1.add_forward_ref(v2.0);
                }
                Ok(())
            }
            (Alias::BackRef(tgt1), Alias::BackRef(tgt2)) => {
                let ix1 = *tgt1;
                let ix2 = *tgt2;
                let fwd1 = &mut self.aliases[ix1];
                let fwd2 = &mut self.aliases[ix2];

                if ix1 < ix2 {
                    let mid = fwd2.set_backref(ix1);
                    for a in mid.iter_fwd_refs() {
                        assert!(!fwd1.contains_fwd_ref(a), "canonical alias of {v1} (?{ix1}) shares forward-reference ?{a} with canonical alias of {v2} (?{ix2})");
                        self.aliases[a].set_backref(ix1);
                        fwd1.add_forward_ref(a);
                    }
                    fwd1.add_forward_ref(ix2);
                } else if ix1 > ix2 {
                    let mid = fwd1.set_backref(ix2);
                    for a in mid.iter_fwd_refs() {
                        assert!(!fwd2.contains_fwd_ref(a), "canonical alias of {v2} (?{ix2}) shares forward-reference ?{a} with canonical alias of {v1} (?{ix1})");
                        self.aliases[a].set_backref(ix2);
                        fwd2.add_forward_ref(a);
                    }
                    fwd2.add_forward_ref(ix1);
                }
                Ok(())
            }
            (Alias::BackRef(tgt), Alias::Canonical(fwds)) => {
                let left = fwds.contains(tgt);
                let right = *tgt == v2.0;

                match (left, right) {
                    (true, true) => return Ok(()),
                    (true, false) | (false, true) => unreachable!("mismatched back- and forward-references for {v1} ({a1:?}) and {v2} ({a2:?})"),
                    (false, false) => ()
                }

                let ix1 = *tgt;
                let ix2 = v2.0;
                let fwd1 = &mut self.aliases[ix1];
                let fwd2 = a2;

                // check not the actual indices, but the canonical indices for tie-breaking
                if ix1 < ix2 {
                    let mid = fwd2.set_backref(ix1);
                    for a in mid.iter_fwd_refs() {
                        assert!(!fwd1.contains_fwd_ref(a), "canonical alias of {v1} (?{ix1}) shares forward-reference ?{a} with canonical alias {v2}");
                        self.aliases[a].set_backref(ix1);
                        fwd1.add_forward_ref(a);
                    }
                    fwd1.add_forward_ref(ix2);
                } else {
                    // ix1 > ix2 , because we already checked for equality
                    let mid = fwd1.set_backref(ix2);
                    for a in mid.iter_fwd_refs() {
                        assert!(!fwd2.contains_fwd_ref(a), "canonical alias {v2} shares forward-reference ?{a} with canonical alias of {v1} (?{ix1})");
                        self.aliases[a].set_backref(ix2);
                        fwd2.add_forward_ref(a);
                    }
                    fwd2.add_forward_ref(ix1);
                }
                Ok(())
            }
            (Alias::Canonical(fwds), Alias::BackRef(tgt)) => {
                let left = fwds.contains(tgt);
                let right = *tgt == v1.0;

                match (left, right) {
                    (true, true) => return Ok(()),
                    (true, false) | (false, true) => unreachable!("mismatched forward- and back-references for {v1} ({a1:?}) and {v2} ({a2:?})"),
                    (false, false) => ()
                }

                let ix1 = v1.0;
                let ix2 = *tgt;
                let fwd1 = a1;
                let fwd2 = &mut self.aliases[ix2];

                // check not the actual indices, but the canonical indices for tie-breaking
                if ix1 < ix2 {
                    let mid = fwd2.set_backref(ix1);
                    for a in mid.iter_fwd_refs() {
                        assert!(!fwd1.contains_fwd_ref(a), "canonical alias {v1} (?{ix1}) shares forward-reference ?{a} with canonical alias of {v2} (?{ix2})");
                        self.aliases[a].set_backref(ix1);
                        fwd1.add_forward_ref(a);
                    }
                    fwd1.add_forward_ref(ix2);
                } else {
                    // ix1 > ix2 , because we already checked for equality
                    let mid = fwd1.set_backref(ix2);
                    for a in mid.iter_fwd_refs() {
                        assert!(!fwd2.contains_fwd_ref(a), "canonical alias of {v2} (?{ix2}) shares forward-reference ?{a} with canonical alias {v1}");
                        self.aliases[a].set_backref(ix2);
                        fwd2.add_forward_ref(a);
                    }
                    fwd2.add_forward_ref(ix1);
                }
                Ok(())
            }
            (Alias::Canonical(_), Alias::Canonical(_)) => {
                if v1 < v2 {
                    let mid = a2.set_backref(v1.0);
                    for a in mid.iter_fwd_refs() {
                        assert!(!a1.contains_fwd_ref(a), "canonical alias {v1} shares forward-reference ?{a} with canonical alias {v2}");
                        self.aliases[a].set_backref(v1.0);
                        a1.add_forward_ref(a);
                    }
                    a1.add_forward_ref(v2.0);
                } else {
                    // v1 > v2, because we already checked for equality
                    let mid = a1.set_backref(v2.0);
                    for a in mid.iter_fwd_refs() {
                        assert!(!a2.contains_fwd_ref(a), "canonical alias {v2} shares forward-reference ?{a} with canonical alias {v1}");
                        self.aliases[a].set_backref(v2.0);
                        a2.add_forward_ref(a);
                    }
                    a2.add_forward_ref(v1.0);
                }
                Ok(())
            }
        }
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
                    uts.push(self.infer_utype_expr(t, scope)?);
                }
                let ret = Rc::new(UType::Tuple(uts));
                self.unify_var_utype(newvar, ret.clone());
                Ok(ret)
            }
            &Expr::TupleProj(ref e_tup, ix) => {
                let newvar = self.get_new_uvar();
                let ret = match self.infer_utype_expr(e_tup, scope)?.as_ref() {
                    UType::Tuple(elts) => {
                        if ix < elts.len() {
                            elts[ix].clone()
                        } else {
                            panic!("tuple projection `*.{ix}` out-of-bounds on tuple {elts:#?}");
                        }
                    }
                    UType::Var(_) => todo!("handle indirection for tupleproj on UVar-shaped tuple"),
                    other => unreachable!("expected tuple, found {other:?}"),
                };
                self.unify_var_utype(newvar, ret.clone())?;
                Ok(ret)
            }
            Expr::Record(fs) => {
                let newvar = self.get_new_uvar();
                let mut child = UMultiScope::new(scope);
                let mut ufs : Vec<(Label, Rc<UType>)> = Vec::with_capacity(fs.len());
                for (lbl, f) in fs {
                    self.infer_utype_expr_field(lbl, f, &mut child)?;
                }
                let ret = child.into_record_utype();
                self.unify_var_utype(newvar, ret.clone());
                Ok(ret)
            }
            Expr::RecordProj(e_rec, fname) => {
                let newvar = self.get_new_uvar();
                let ret = match self.infer_utype_expr(e_rec, scope)?.as_ref() {
                    UType::Record(flds) => {
                        if let Some((_, ft)) = flds.iter().find(|(lbl, _)| lbl == fname) {
                            ft.clone()
                        } else {
                            panic!("record projection `*.{fname}` is a non-existent field for record {flds:#?}");
                        }
                    }
                    UType::Var(_) => todo!("handle indirection for recordproj on UVar-shape record"),
                    other => unreachable!("expected record, found {other:?}"),
                };
                self.unify_var_utype(newvar, ret.clone())?;
                Ok(ret)
            }
            Expr::Variant(_, _) => todo!(),
            Expr::Seq(_) => todo!(),
            Expr::Match(_, _) => todo!(),
            Expr::Lambda(_, _) => todo!(),
            Expr::AsU8(_) => todo!(),
            Expr::AsU16(_) => todo!(),
            Expr::AsU32(_) => todo!(),
            Expr::AsChar(_) => todo!(),

            Expr::U16Be(_) => todo!(),
            Expr::U16Le(_) => todo!(),
            Expr::U32Be(_) => todo!(),
            Expr::U32Le(_) => todo!(),
            Expr::SeqLength(_) => todo!(),
            Expr::SubSeq(_, _, _) => todo!(),
            Expr::FlatMap(_, _) => todo!(),
            Expr::FlatMapAccum(_, _, _, _) => todo!(),
            Expr::Dup(_, _) => todo!(),
            Expr::Inflate(_) => todo!(),
        }
    }

    fn unify_utype_baseset(&self, ut: Rc<UType>, bs: BaseSet) -> TCResult<Constraint> {
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
                let t0 = self.unify_utype(t1.clone(), t2.clone())?;
                Ok(Constraint::Equiv(t0))
            }
            (Constraint::Equiv(ut), Constraint::Elem(bs))
            | (Constraint::Elem(bs), Constraint::Equiv(ut)) => Ok(self.unify_utype_baseset(ut.clone(), *bs)?),
            (Constraint::Elem(bs1), Constraint::Elem(bs2)) => {
                let bs0 = bs1.union(bs2)?;
                Ok(bs0.to_constraint())
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

    fn unify_varmaps(&self, v1: UVar, vmid1: &mut VMId, v2: UVar, vmid2: &mut VMId) -> TCResult<VMId> {
        if (*vmid1 == *vmid2) {
            return Ok(*vmid1);
        }

        let (lo, hi) = if (*vmid1 < *vmid2) {
            (vmid1, vmid2)
        } else {
            (vmid2, vmid1)
        };

        let vm_lo = self
            .varmaps
            .as_inner_mut()
            .get_mut(&lo.0)
            .unwrap_or_else(|| unreachable!("missing {lo}"));
        let vm_hi = self
            .varmaps
            .as_inner_mut()
            .get_mut(&hi.0)
            .unwrap_or_else(|| unreachable!("missing {hi}"));

        // keep the earlier one, prune the later one, after computing the union
        for (vname, inner) in vm_hi.drain() {
            if vm_lo.contains_key(&vname) {
                let t_lo = vm_lo
                    .get(&vname)
                    .unwrap_or_else(|| unreachable!("cannot get key we already know is there"));
                let t_hi = inner;
                let unified = self.unify_utype(t_lo.clone(), t_hi.clone())?;
                let _ = vm_lo.insert(vname, unified);
            } else {
                vm_lo.insert(vname, inner);
            }
        }

        // delete varmap at *hi to end up in a clean-ish state
        self.varmaps.as_inner_mut().remove(&hi.0);

        // overwrite the value pointed to by hi, to match that of lo
        *hi = *lo;

        Ok(*lo)
    }

    /// Performs an occurs-check for early detection of infinite types
    fn occurs(&self, v: UVar) -> TCResult<()> {
        self.occurs_in_constraints(v, &self.constraints[v.0])

    }

    fn occurs_in_constraints(&self, v: UVar, cs: &Constraints) -> TCResult<()> {
        match cs {
            Constraints::Indefinite => {
                Ok(())
            }
            Constraints::Variant(vmid) => {
                let Some(vm) = self.varmaps.as_inner().get(&v.0) else { unreachable!("missing vmap for {vmid}") };
                for (_label, inner) in vm.iter() {
                    self.occurs_in(v, inner.clone())?;
                }
                Ok(())
            }
            Constraints::Invariant(c) => match c {
                Constraint::Elem(_) => Ok(()),
                Constraint::Equiv(t) => self.occurs_in(v, t),
            }
        }
    }


    fn occurs_in(&self, v: UVar, t: impl AsRef<UType>) -> TCResult<()> {
        match t.as_ref() {
            UType::Empty | UType::Base(_) => Ok(()),
            &UType::Var(v1) => {
                if self.is_aliased(v, v1) {
                    Err(TCError::InfiniteType(v, self.constraints[v.0].clone()))
                } else {
                    let c_ix = self.aliases[v1.0].as_backref().unwrap_or(v1.0);
                    self.occurs_in_constraints(v, &self.constraints[c_ix])
                }
            }
            UType::Tuple(ts) => {
                for t in ts.iter() {
                    let _ = self.occurs_in(v, t.clone())?;
                }
                Ok(())
            }
            UType::Record(fs) => {
                for (_lbl, t) in fs.iter() {
                    let _ = self.occurs_in(v, t.clone())?;
                }
                Ok(())
            }
            UType::Seq(inner) => {
                self.occurs_in(v, inner.clone())?;
                Ok(())
            }
        }
    }

    fn infer_utype_expr_field<'a>(&mut self, lbl: &Label, expr: &Expr, scope: &'a mut UMultiScope<'a>) -> TCResult<Rc<UType>> {
        let newvar = self.get_new_uvar();
        let t = self.infer_utype_expr(expr, &UScope::Multi(scope))?;
        self.unify_var_utype(newvar, t.clone());
        scope.push(lbl.clone(), newvar);
        Ok(t)
    }

    /// Checks whether two UVars are equated via aliasing.
    ///
    /// Returns true for implicit reflexive aliasing, as well as direct referential aliasing.
    fn is_aliased(&self, v: UVar, v1: UVar) -> bool {
        if v == v1 {
            return true;
        }

        let a = &self.aliases[v.0];
        let a1 = &self.aliases[v1.0];

        match (a, a1) {
            (Alias::NoAlias, _) | (_ , Alias::NoAlias) => false,
            (Alias::BackRef(tgt1), Alias::BackRef(tgt2)) => tgt1 == tgt2,
            (Alias::BackRef(tgt), Alias::Canonical(..)) => *tgt == v1.0,
            (Alias::Canonical(..), Alias::BackRef(tgt)) => *tgt == v.0,
            (Alias::Canonical(_), Alias::Canonical(_)) => false,
        }
    }

    /// Attempt to substitute a variable for a shapeful UType with at least one more level of refinement
    ///
    /// If there is no possible direct substitution for a UType (i.e. no known constriants, or a variant type),
    /// returns Ok(None).
    ///
    /// If the only possible refinement would be the identity transformation modulo aliasing, likewise returns
    /// Ok(None).
    ///
    /// If an occurs check fails, returns Err of the appropriate error value back to the caller.
    ///
    /// Otherwise, returns Some(t) where t is a UType other than UType::Var(v) or UType::Var(v1) where
    /// v1 is an isomorphic alias to v.
    fn substitute_uvar_utype(&self, v: UVar) -> Result<Option<Rc<UType>>, TCError> {
        self.occurs(v)?;
        match self.constraints[v.0] {
            Constraints::Indefinite => Ok(None),
            Constraints::Variant(_) => Ok(None),
            Constraints::Invariant(_) => todo!(),
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
    Incompatible(UVar, T, T), // two independent assertions about a UVar are incompatible
    Unsatisfiable(T, T),      // a single non-variable assertion is directly unsatisfiable
}

#[derive(Clone, Debug)]
pub enum TCError {
    VarianceMismatch(UVar, VMId, Constraint), // attempted unification of a variant and non-variant constraint
    Unification(ConstraintError),
    InfiniteType(UVar, Constraints),
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
            TCError::InfiniteType(v, constraints)  => match constraints {
                Constraints::Indefinite => unreachable!("indefinite constraint `{v} = ??` is not infinite"),
                Constraints::Variant(vmid) => write!(f, "`{v} ⊇ {vmid}` constitutes an infinite type ({v} or alias occurs within {vmid})"),
                Constraints::Invariant(inv) => match inv {
                    Constraint::Equiv(t) => write!(f, "`{v} = {t:?}` is an infinite type ({v} or alias occurs within the rhs utype)"),
                    Constraint::Elem(_) => unreachable!("`{v} {inv}` is not infinite, but we thought it was"),
                }
            }
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

// #[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_aliasing() {
        let mut tc = TypeChecker::new();
        return;
    }
}

mod __impls {
    use std::borrow::{Borrow, BorrowMut};
    use super::{UVar, VMId};

    impl std::fmt::Display for UVar {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "?{}", self.0)
        }
    }

    impl AsRef<usize> for UVar {
        fn as_ref(&self) -> &usize {
            &self.0
        }
    }

    impl AsMut<usize> for UVar {
        fn as_mut(&mut self) -> &mut usize {
            &mut self.0
        }
    }

    impl Borrow<usize> for UVar {
        fn borrow(&self) -> &usize {
            &self.0
        }
    }

    impl BorrowMut<usize> for UVar {
        fn borrow_mut(&mut self) -> &mut usize {
            &mut self.0
        }
    }
    impl std::fmt::Display for VMId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "#{}", self.0)
        }
    }

    impl AsRef<usize> for VMId {
        fn as_ref(&self) -> &usize {
            &self.0
        }
    }
}
