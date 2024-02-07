use crate::{ BaseType, Expr, Format, FormatModule, Label, Pattern, ValueType };
use std::{ collections::{ BTreeMap, HashMap, HashSet }, rc::Rc };

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct UVar(usize);

/// Unification type, equivalent to ValueType up to abstraction
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) enum UType {
    Empty, // Reserved for value-free Formats (Format::Fail only)
    Hole, // ground type-hole for shape-only unifications
    Var(UVar), // type-hole
    Base(BaseType),
    Tuple(Vec<Rc<UType>>),
    Record(Vec<(Label, Rc<UType>)>),
    Seq(Rc<UType>),
}

impl From<BaseType> for UType {
    fn from(value: BaseType) -> Self {
        Self::Base(value)
    }
}

impl From<BaseType> for Rc<UType> {
    fn from(value: BaseType) -> Self {
        Rc::new(UType::from(value))
    }
}

impl From<UVar> for UType {
    fn from(value: UVar) -> Self {
        Self::Var(value)
    }
}

impl From<UVar> for Rc<UType> {
    fn from(value: UVar) -> Self {
        Rc::new(UType::Var(value))
    }
}

impl UType {
    pub const UNIT: Self = UType::Tuple(Vec::new());

    pub fn tuple<T>(elems: impl IntoIterator<Item = T>) -> Self where T: Into<Rc<UType>> {
        Self::Tuple(elems.into_iter().map(Into::into).collect())
    }

    pub(crate) fn from_valuetype(vt: &ValueType) -> Option<UType> {
        match vt {
            ValueType::Any => Some(Self::Hole),
            ValueType::Empty => Some(Self::Empty),
            ValueType::Base(b) => Some(Self::Base(*b)),
            ValueType::Tuple(vts) => {
                let mut uts = Vec::with_capacity(vts.len());
                for vt in vts.iter() {
                    uts.push(Rc::new(Self::from_valuetype(vt)?));
                }
                Some(Self::Tuple(uts))
            }
            ValueType::Record(vfs) => {
                let mut ufs = Vec::with_capacity(vfs.len());
                for (lab, vf) in vfs.iter() {
                    ufs.push((lab.clone(), Rc::new(Self::from_valuetype(vf)?)));
                }
                Some(Self::Record(ufs))
            }
            ValueType::Union(..) => None,
            ValueType::Seq(inner) => Some(Self::Seq(Rc::new(Self::from_valuetype(inner)?))),
        }
    }
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

#[derive(Clone, Copy)]
pub(crate) struct Ctxt<'a> {
    module: &'a FormatModule,
    scope: &'a UScope<'a>,
}

impl<'a> Ctxt<'a> {
    fn with_scope<'b>(self, newscope: &'b UScope<'b>) -> Ctxt<'b> where 'a: 'b, 'b: 'a {
        Self {
            module: self.module,
            scope: newscope,
        }
    }

    fn new(module: &'a FormatModule, scope: &'a UScope<'a>) -> Self {
        Self { module, scope }
    }
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
            UType::Empty | UType::Hole | UType::Var(..) | UType::Base(..) => {
                Box::new(std::iter::empty())
            }
            UType::Tuple(ts) => Box::new(ts.iter().cloned()),
            UType::Record(fs) => Box::new(fs.iter().map(|(_l, t)| t.clone())),
            UType::Seq(t) => Box::new(std::iter::once(t.clone())),
        }
    }

    pub fn get_uinst(&self) -> UInst {
        match self {
            Self::Hole | Self::Empty | Self::Base(..) => UInst::Ground,
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
            Self::Seq(t) =>
                match t.get_uinst() {
                    UInst::Ground => UInst::Ground,
                    UInst::Bound(vs) => UInst::Bound(vs),
                    UInst::Free(v) => UInst::Bound(HashSet::from([v])),
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
    /// Note that transitive inclusion is not checked, as this requires a typechecker context to evaluate
    /// indirect equivalences, so a return value of `None` does not necessarily rule out infinite types
    /// or tautologies.
    pub fn find_var_depth(&self, var: &UVar) -> Option<bool> {
        match self.get_uinst() {
            UInst::Ground => None,
            UInst::Bound(vs) => {
                if vs.contains(var) { Some(false) } else { None }
            }
            UInst::Free(v0) => {
                if v0 == *var { Some(true) } else { None }
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
    ImplicitTuple(Vec<Rc<UType>>),
    ImplicitRecord(Vec<(Label, Rc<UType>)>),
    IndefiniteUnion(VMId),
}

#[derive(Debug)]
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
    Canonical(HashSet<usize>), // list of forward-references to update if usurped by an earlier canonical alias
}

impl Alias {
    /// New, empty alias-set
    pub const fn new() -> Alias {
        Self::NoAlias
    }

    pub const fn is_canonical(&self) -> bool {
        matches!(self, Alias::Canonical(_) | Alias::NoAlias)
    }

    pub fn is_canonical_nonempty(&self) -> bool {
        match self {
            Alias::Canonical(x) => !x.is_empty(),
            _ => false,
        }
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
                let _ = std::mem::replace(self, Alias::Canonical(HashSet::from([tgt])));
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
            Alias::Canonical(fwds) => Box::new(fwds.iter().copied()),
        }
    }

    fn contains_fwd_ref(&self, tgt: usize) -> bool {
        match self {
            Alias::NoAlias | Alias::BackRef(_) => false,
            Alias::Canonical(fwds) => fwds.contains(&tgt),
        }
    }
}

#[derive(Debug)]
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
    Proj(ProjShape), // constraints implied by projections
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ProjShape {
    TupleWith(BTreeMap<usize, UVar>), // required associations of metavars at given indices of an uncertain tuple
    RecordWith(BTreeMap<Label, UVar>), // required associations of metavars at given fields of an uncertain record
    SeqOf(UVar), // simple sequence element-type projection
}

impl ProjShape {
    pub fn new_tuple() -> Self {
        Self::TupleWith(BTreeMap::new())
    }

    pub fn new_record() -> Self {
        Self::RecordWith(BTreeMap::new())
    }

    pub fn seq_of(elem: UVar) -> Self {
        Self::SeqOf(elem)
    }

    fn as_tuple_mut(&mut self) -> &mut BTreeMap<usize, UVar> {
        match self {
            ProjShape::TupleWith(map) => map,
            other => panic!("called as_tuple_mut on non-tuple {other:?}"),
        }
    }

    fn as_record_mut(&mut self) -> &mut BTreeMap<Label, UVar> {
        match self {
            ProjShape::RecordWith(map) => map,
            other => panic!("called as_record_mut on non-record {other:?}"),
        }
    }
}

/// Abstraction over explicit collections of BaseType values that could be in any order
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) enum BaseSet {
    /// singleton set of any basetype, even non-integral ones
    Single(BaseType),
    /// U8, U16, U32 (or any future types like U64)
    UAny,
    /// Unifies as if UAny, solves as if Single(BaseSet::U32)
    USome,
}

impl BaseSet {
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (BaseSet::Single(b1), BaseSet::Single(b2)) => b1 == b2,
            | (BaseSet::Single(b), BaseSet::UAny | BaseSet::USome)
            | (BaseSet::UAny | BaseSet::USome, BaseSet::Single(b)) => b.is_numeric(),
            (BaseSet::UAny | BaseSet::USome, BaseSet::UAny | BaseSet::USome) => true,
        }
    }

    pub fn contains(&self, item: BaseType) -> bool {
        match self {
            BaseSet::Single(elem) => *elem == item,
            BaseSet::UAny | BaseSet::USome => item.is_numeric(),
        }
    }

    pub fn union(&self, other: &Self) -> Result<Self, ConstraintError> {
        match (self, other) {
            (BaseSet::Single(b1), BaseSet::Single(b2)) => {
                if b1 == b2 {
                    Ok(*self)
                } else {
                    Err(
                        ConstraintError::Unsatisfiable(
                            Constraint::Elem(*self),
                            Constraint::Elem(*other)
                        )
                    )
                }
            }
            | (BaseSet::UAny | BaseSet::USome, BaseSet::Single(b))
            | (BaseSet::Single(b), BaseSet::UAny | BaseSet::USome) => {
                if b.is_numeric() {
                    Ok(BaseSet::Single(*b))
                } else {
                    Err(
                        UnificationError::Unsatisfiable(self.to_constraint(), other.to_constraint())
                    )
                }
            }
            (BaseSet::UAny, BaseSet::UAny) => Ok(*self),
            (BaseSet::UAny | BaseSet::USome, BaseSet::USome) | (BaseSet::USome, BaseSet::UAny) => {
                Ok(BaseSet::USome)
            }
        }
    }

    /// Constructs the simplest-possible constraint from `self`, in particular substituting
    /// `Equiv(BaseType(b))` in place of `Elem(Single(b))`.
    pub fn to_constraint(&self) -> Constraint {
        match self {
            BaseSet::Single(b) => Constraint::Equiv(Rc::new(UType::Base(*b))),
            BaseSet::UAny | BaseSet::USome => Constraint::Elem(*self),
        }
    }

    fn get_unique_solution(&self, v: UVar) -> TCResult<Rc<UType>> {
        match self {
            BaseSet::Single(b) => Ok(Rc::new(UType::Base(*b))),
            BaseSet::USome => Ok(Rc::new(UType::Base(BaseType::U32))),
            BaseSet::UAny => Err(TCError::MultipleSolutions(v, *self)),
        }
    }
}

impl std::fmt::Display for BaseSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseSet::UAny => write!(f, "{{ U8, U16, U32 }}"),
            BaseSet::USome => write!(f, "{{ U8, U16, [U32] }}"),
            BaseSet::Single(t) => write!(f, "{{ {t:?} }}"),
        }
    }
}

type VarMap = HashMap<Label, Rc<UType>>;

// SECTION - Construction and instantiation in the metacontext
impl TypeChecker {
    /// Constructs a typechecker with initially 0 metavariables.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            aliases: Vec::new(),
            varmaps: VarMapMap::new(),
        }
    }

    /// Instantiates a new [`VarMap`] object in the typechecker metacontext and returns an identifier pointing to it.
    fn init_varmap(&mut self) -> VMId {
        self.varmaps.get_new_id()
    }

    /// Instantiates a new UVar and immediately equates it with a specified UType
    ///
    /// Useful primarily for preserving traversal-order requirements for down-the-line association with
    /// otherwise untyped Format-tree nodes
    ///
    /// This should never return `Err(_)` unless something catastrophic has happened, or it was called
    /// with an improper `UTYpe` (e.g. one that references an out-of-range UVar at the time it was constructed)
    fn init_var_simple(&mut self, typ: UType) -> TCResult<(UVar, Rc<UType>)> {
        let newvar = self.get_new_uvar();
        let rc = Rc::new(typ);
        let constr = Constraint::Equiv(rc.clone());
        self.unify_var_constraint(newvar, constr)?;
        // FIMXE - not sure whether to return rc or newvar
        Ok((newvar, rc))
    }

    /// Instantiates a new UVar with no known constraints or aliases, returning it by-value
    fn get_new_uvar(&mut self) -> UVar {
        let ret = UVar(self.constraints.len());
        self.constraints.push(Constraints::new());
        self.aliases.push(Alias::new());
        ret
    }

    fn reify_union(&self, vmid: VMId) -> Option<ValueType> {
        let vm = self.varmaps.get_varmap(vmid);
        let mut branches = Vec::with_capacity(vm.len());
        for (label, ut) in vm.iter() {
            branches.push((label.clone(), self.reify(ut.clone())?));
        }
        // ensure branches are sorted lexically by label
        branches.sort_by_key(|x| x.0.to_owned());
        Some(ValueType::Union(branches))
    }

    fn infer_var_scope_pattern<'a, 'b>(
        &mut self,
        pat: &Pattern,
        scope: &'a mut UMultiScope<'b>
    ) -> TCResult<UVar> {
        match pat {
            Pattern::Binding(name) => {
                let var = self.get_new_uvar();
                scope.push(name.clone(), var);
                Ok(var)
            }
            Pattern::Wildcard => {
                let var = self.get_new_uvar();
                Ok(var)
            }
            Pattern::Bool(_) => {
                let var = self.init_var_simple(UType::Base(BaseType::Bool))?.0;
                Ok(var)
            }
            Pattern::U8(_) => {
                let var = self.init_var_simple(UType::Base(BaseType::U8))?.0;
                Ok(var)
            }
            Pattern::U16(_) => {
                let var = self.init_var_simple(UType::Base(BaseType::U16))?.0;
                Ok(var)
            }
            Pattern::U32(_) => {
                let var = self.init_var_simple(UType::Base(BaseType::U32))?.0;
                Ok(var)
            }
            Pattern::Char(_) => {
                let var = self.init_var_simple(UType::Base(BaseType::Char))?.0;
                Ok(var)
            }
            Pattern::Tuple(pats) => {
                let tuple_var = self.get_new_uvar();
                let mut elem_vars = Vec::with_capacity(pats.len());
                for p in pats.iter() {
                    let pvar = self.infer_var_scope_pattern(p, scope)?;
                    elem_vars.push(Rc::new(UType::Var(pvar)));
                }
                self.unify_var_utype(tuple_var, Rc::new(UType::Tuple(elem_vars)))?;
                Ok(tuple_var)
            }
            Pattern::Variant(vname, inner) => {
                let topvar = self.get_new_uvar();
                let inner_var = self.infer_var_scope_pattern(inner.as_ref(), scope)?;
                self.add_uvar_variant(topvar, vname.clone(), Rc::new(UType::Var(inner_var)))?;
                Ok(topvar)
            }
            Pattern::Seq(elts) => {
                let seq_uvar = self.get_new_uvar();
                let elem_uvar = self.get_new_uvar();
                for elt in elts.iter() {
                    let elt_uvar = self.infer_var_scope_pattern(elt, scope)?;
                    self.unify_var_pair(elem_uvar, elt_uvar)?;
                }
                self.unify_var_utype(
                    seq_uvar,
                    Rc::new(UType::Seq(Rc::new(UType::Var(elem_uvar))))
                )?;
                Ok(seq_uvar)
            }
        }
    }

    fn unify_utype_format_match_case<'a>(
        &mut self,
        head_t: Rc<UType>,
        pat: &Pattern,
        rhs_var: UVar,
        rhs_format: &Format,
        ctxt: Ctxt<'a>
    ) -> TCResult<()> {
        let mut child = UMultiScope::new(ctxt.scope);
        let pvar = self.infer_var_scope_pattern(pat, &mut child)?;
        let tmp = child.clone();
        let newscope = UScope::Multi(&tmp);
        let newctxt = ctxt.with_scope(&newscope);
        let rhs_type = self.infer_utype_format(rhs_format, newctxt)?;
        self.unify_var_utype(pvar, head_t)?;
        self.unify_var_utype(rhs_var, rhs_type)?;
        Ok(())
    }

    fn unify_utype_expr_match_case<'a>(
        &mut self,
        head_t: Rc<UType>,
        pat: &Pattern,
        rhs_var: UVar,
        rhs_expr: &Expr,
        scope: &'a UScope<'a>
    ) -> TCResult<()> {
        let mut child = UMultiScope::new(scope);
        let pvar = self.infer_var_scope_pattern(pat, &mut child)?;
        let tmp = child.clone();
        let newscope = UScope::Multi(&tmp);
        let rhs_type = self.infer_utype_expr(rhs_expr, &newscope)?;
        self.unify_var_utype(pvar, head_t)?;
        self.unify_var_utype(rhs_var, rhs_type)?;
        Ok(())
    }

    /// Converts a `UType` to Weak Head-Normal Form `VType` by unchaining as many metavariables as required, after performing
    /// a sanity check to eliminate potential infinite types from consideration.
    ///
    /// Will avoid panicking at all costs, even if it requires returning a non-WHNF variable.
    fn to_whnf_vtype(&self, t: Rc<UType>) -> VType {
        assert!(!self.is_infinite_type(t.clone()));
        match t.as_ref() {
            UType::Var(v) => {
                let v0 = self.get_canonical_uvar(*v);
                match &self.constraints[v0.0] {
                    Constraints::Invariant(Constraint::Equiv(ut)) => self.to_whnf_vtype(ut.clone()),
                    Constraints::Invariant(Constraint::Elem(bs)) => VType::Base(*bs),
                    Constraints::Invariant(Constraint::Proj(ps)) => self.proj_shape_to_vtype(ps),
                    Constraints::Variant(vmid) => VType::IndefiniteUnion(*vmid),
                    Constraints::Indefinite => VType::Abstract(v0.into()),
                }
            }
            _ => VType::Abstract(t),
        }
    }
}
// !SECTION

// SECTION - checks and maintenance of invariants of the metacontext
impl TypeChecker {
    /// Performs a runtime assertion that the number of known UVars is agreed upon by all fields that track
    /// their expected properties.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn check_uvar_sanity(&self) -> () {
        assert_eq!(self.constraints.len(), self.aliases.len());
    }

    /// Returns `true` if `t` describes an infinite type, considering
    /// tautologies only in recursive calls
    ///
    /// Does not bother to check any variables other than v while traversing
    /// `t`, as those should be ruled out by a theoretical inductive hypothesis
    pub fn is_infinite_type(&self, t: Rc<UType>) -> bool {
        match t.as_ref() {
            UType::Empty | UType::Base(..) => false,
            UType::Var(v) => self.occurs(*v).is_err(),
            _ => t.iter_embedded().any(|sub_t| self.is_infinite_type(sub_t)),
        }
    }

    /// Performs an occurs-check for early detection of infinite types
    pub fn occurs(&self, v: UVar) -> TCResult<()> {
        self.occurs_in_constraints(v, &self.constraints[v.0])
    }

    /// Low-level helper for [`TypeChecker::occurs`] for recursive, possibly iterative occurs-checks within a
    /// [`Constraints`] object that was encountered during the expansion of the original UVar association
    fn occurs_in_constraints(&self, v: UVar, cs: &Constraints) -> TCResult<()> {
        match cs {
            Constraints::Indefinite => Ok(()),
            Constraints::Variant(vmid) => {
                let vm = self.varmaps.get_varmap(*vmid);
                for (_label, inner) in vm.iter() {
                    self.occurs_in(v, inner.clone())?;
                }
                Ok(())
            }
            Constraints::Invariant(c) =>
                match c {
                    Constraint::Elem(_) => Ok(()),
                    Constraint::Equiv(t) => self.occurs_in(v, t),
                    Constraint::Proj(p) =>
                        match p {
                            ProjShape::TupleWith(ixvars) => {
                                for (_ix, var) in ixvars.iter() {
                                    self.occurs_in(v, Rc::<UType>::from(*var))?;
                                }
                                Ok(())
                            }
                            ProjShape::RecordWith(fldvars) => {
                                for (_lbl, var) in fldvars.iter() {
                                    self.occurs_in(v, Rc::<UType>::from(*var))?;
                                }
                                Ok(())
                            }
                            ProjShape::SeqOf(elem_v) =>
                                self.occurs_in(v, Rc::<UType>::from(*elem_v)),
                        }
                }
        }
    }

    fn occurs_in(&self, v: UVar, t: impl AsRef<UType>) -> TCResult<()> {
        match t.as_ref() {
            UType::Hole | UType::Empty | UType::Base(_) => Ok(()),
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

    /// Attempts to add a new variant to the implied partial union-type of an existing metavariable.
    ///
    /// Returns `Ok(())` if successful.
    /// Returns `Err(e)` if unification with an identically named, pre-existing variant returned the unification error `e`.
    ///
    /// # Panics
    ///
    /// Will panic if called on a uix value (index stored within a UVar) corresponding to [`Constraints::Invariant`]
    /// constraints-object.
    fn add_uvar_variant(&mut self, v: UVar, cname: Label, inner: Rc<UType>) -> TCResult<()> {
        // find the canonical uvar
        let cv = self.get_canonical_uvar(v);

        // update the canonical uvar constraints
        let ref cnstrs = self.constraints[cv.0];
        match cnstrs {
            Constraints::Indefinite => {
                let id = self.init_varmap();
                let vm = self.varmaps.as_inner_mut().entry(id.0).or_default();
                vm.insert(cname, inner);
                self.set_uvar_vmid(cv, id)?;
                // update all forward-references to point to the same varmap
                let fwds = self.aliases[cv.0].iter_fwd_refs().collect::<Vec<usize>>();
                for ix in fwds.into_iter() {
                    self.set_uvar_vmid(UVar(ix), id)?;
                }
                Ok(())
            }
            Constraints::Variant(vmid) => {
                let id = *vmid;
                let vm = self.varmaps.get_varmap(id);
                if let Some(prior) = vm.get(&cname) {
                    let updated = self.unify_utype(prior.clone(), inner)?;
                    if
                        updated.as_ref() !=
                        self.varmaps.get_varmap(id).get(&cname).unwrap().as_ref()
                    {
                        self.varmaps.get_varmap_mut(id).insert(cname, updated);
                    }
                } else {
                    self.varmaps.get_varmap_mut(*vmid).insert(cname, inner);
                }
                Ok(())
            }
            Constraints::Invariant(_) => {
                panic!("Cannot add constraint to invariant constraints object (index: {v})")
            }
        }
    }

    fn set_uvar_vmid(&mut self, uvar: UVar, vmid: VMId) -> TCResult<()> {
        assert!(
            self.varmaps.as_inner().contains_key(&vmid.0),
            "set_uvar_vmid called on missing VMId {vmid}"
        );
        let ref mut constrs = self.constraints[uvar.0];
        match constrs {
            Constraints::Variant(other) => {
                let old = *other;

                // we only care about old if it is still an extant varmap
                if let Some(old_vm) = self.varmaps.as_inner().get(&old.0) {
                    let Some(new_vm) = self.varmaps.as_inner().get(&vmid.0) else {
                        unreachable!("impossible failure after previous assertion")
                    };
                    for key in old_vm.keys() {
                        // this check may be costly so we are gating it for non-release builds
                        debug_assert!(
                            new_vm.contains_key(key),
                            "previous varmap {other} of {uvar} has variant {key} but new varmap {vmid} does not"
                        );
                    }
                }

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
}
// !SECTION

// SECTION - Pairwise unification steps
impl TypeChecker {
    /// Attempts to add a direct (non-Variant) constraint to an existing, possibly recently-created UVar
    ///
    /// Will panic if the UVar is pointing to a Variant Constraints structure and therefore cannot have
    /// any constraints added to it directly, but will not otherwise attempt to check for mutual satisfiability
    /// with other constraints on that variable, or any other it is aliased or equated to.
    fn unify_var_constraint(&mut self, uvar: UVar, constraint: Constraint) -> TCResult<Constraint> {
        let can_ix = self.get_canonical_uvar(uvar).0;
        match &self.constraints[can_ix] {
            Constraints::Indefinite => {
                let ret = constraint.clone();
                self.constraints[can_ix] = Constraints::Invariant(constraint);
                Ok(ret)
            }
            Constraints::Variant(vmid) => Err(TCError::VarianceMismatch(uvar, *vmid, constraint)),
            Constraints::Invariant(prior) => {
                let c1 = prior.clone();
                if c1 == constraint {
                    return Ok(c1);
                }
                let ret = self.unify_constraint_pair(c1, constraint)?;
                self.constraints[can_ix] = Constraints::Invariant(ret.clone());
                Ok(ret)
            }
        }
    }

    /// Unifies according to `?i ~ Tuple(..., ?j, ...?)`, with `tuple_var` on the lhs and `ix_var` on the rhs
    /// of the metavariable equation, with `ix_var` occuring at index `ix` of the tuple being projected.
    fn unify_var_proj_index(&mut self, tuple_var: UVar, ix: usize, ix_var: UVar) -> TCResult<()> {
        let can_v = self.get_canonical_uvar(tuple_var);
        match &mut self.constraints[can_v.0] {
            Constraints::Indefinite => {
                let mut proj = ProjShape::new_tuple();
                proj.as_tuple_mut().insert(ix, ix_var);
                self.unify_var_constraint(tuple_var, Constraint::Proj(proj))?;
                Ok(())
            }
            Constraints::Variant(_) => unreachable!("cannot solve tuple projection on union"),
            Constraints::Invariant(c) =>
                match c {
                    Constraint::Elem(_) =>
                        unreachable!("cannot solve tuple projection on base-set"),
                    Constraint::Equiv(ut) => {
                        match ut.as_ref() {
                            UType::Tuple(ts) => {
                                assert!(ts.len() > ix);
                                let type_at_ix = ts[ix].clone();
                                self.unify_var_utype(ix_var, type_at_ix)?;
                                Ok(())
                            }
                            other => unreachable!("expected UType::Tuple, found {other:?}"),
                        }
                    }
                    Constraint::Proj(ps) =>
                        match ps {
                            ProjShape::TupleWith(map) => {
                                if let Some(&other_var) = map.get(&ix) {
                                    self.unify_var_pair(ix_var, other_var)?;
                                    Ok(())
                                } else {
                                    map.insert(ix, ix_var);
                                    Ok(())
                                }
                            }
                            _ => unreachable!("cannot unify on index of non-tuple projection"),
                        }
                }
        }
    }

    /// Unifies according to `?i ~ Record(name: ?j, ...?)`, with `rec_var` on the lhs and `fld_var` on the rhs
    /// of the metavariable equation, with `fld_var` being the field with label `fname`.
    fn unify_var_proj_field(&mut self, rec_var: UVar, fname: Label, fld_var: UVar) -> TCResult<()> {
        let can_v = self.get_canonical_uvar(rec_var);
        match &mut self.constraints[can_v.0] {
            Constraints::Indefinite => {
                let mut proj = ProjShape::new_record();
                proj.as_record_mut().insert(fname, fld_var);
                self.unify_var_constraint(rec_var, Constraint::Proj(proj))?;
                Ok(())
            }
            Constraints::Variant(_) => unreachable!("cannot solve record projection on union"),
            Constraints::Invariant(c) =>
                match c {
                    Constraint::Elem(_) =>
                        unreachable!("cannot solve record projection on base-set"),
                    Constraint::Equiv(ut) => {
                        match ut.as_ref() {
                            UType::Record(fs) => {
                                let fld_type = fs
                                    .iter()
                                    .find(|(name, _)| name == &fname)
                                    .unwrap()
                                    .1.clone();
                                self.unify_var_utype(fld_var, fld_type)?;
                                Ok(())
                            }
                            other => unreachable!("expected UType::Record, found {other:?}"),
                        }
                    }
                    Constraint::Proj(ps) =>
                        match ps {
                            ProjShape::RecordWith(map) => {
                                if let Some(&other_var) = map.get(&fname) {
                                    self.unify_var_pair(fld_var, other_var)?;
                                    Ok(())
                                } else {
                                    map.insert(fname, fld_var);
                                    Ok(())
                                }
                            }
                            _ => unreachable!("cannot unify on field of non-record projection"),
                        }
                }
        }
    }

    /// Establishes `?i ~ Seq(?j)` through projective unification, where `seq_v` is the lhs
    /// and `elem_v` is the rhs metavariable.
    fn unify_var_proj_elem(&mut self, seq_v: UVar, elem_v: UVar) -> TCResult<()> {
        let can_v = self.get_canonical_uvar(seq_v);
        match &mut self.constraints[can_v.0] {
            Constraints::Indefinite => {
                let mut proj = ProjShape::seq_of(elem_v);
                self.unify_var_constraint(seq_v, Constraint::Proj(proj))?;
                Ok(())
            }
            Constraints::Variant(_) => unreachable!("cannot solve record projection on union"),
            Constraints::Invariant(c) =>
                match c {
                    Constraint::Elem(_) =>
                        unreachable!("cannot solve record projection on base-set"),
                    Constraint::Equiv(ut) => {
                        match ut.as_ref() {
                            UType::Seq(inner) => {
                                let elem_t = inner.clone();
                                self.unify_var_utype(elem_v, elem_t)?;
                                Ok(())
                            }
                            other => unreachable!("expected UType::Seq, found {other:?}"),
                        }
                    }
                    Constraint::Proj(ps) =>
                        match ps {
                            ProjShape::SeqOf(other_var) => {
                                let tmp = *other_var;
                                self.unify_var_pair(elem_v, tmp)?;
                                Ok(())
                            }
                            _ =>
                                unreachable!("cannot unify on element typen of non-seq projection"),
                        }
                }
        }
    }

    fn unify_utype(&mut self, left: Rc<UType>, right: Rc<UType>) -> TCResult<Rc<UType>> {
        match (left.as_ref(), right.as_ref()) {
            (UType::Hole, UType::Hole) => {
                // FIXME - determine whether this is actually a proper case to see in practice
                unreachable!("Unexpected hole-hole unification may indicate bad logic path");
                // Ok(left)
            }
            (UType::Hole, _) => Ok(right),
            (_, UType::Hole) => Ok(left),
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
                    return Err(UnificationError::Unsatisfiable(left, right).into());
                }
                if fs1 == fs2 {
                    return Ok(left);
                }
                let mut fs0 = Vec::with_capacity(fs1.len());
                for ((l1, f1), (l2, f2)) in Iterator::zip(fs1.iter(), fs2.iter()) {
                    if l1 != l2 {
                        return Err(UnificationError::Unsatisfiable(left, right).into());
                    }
                    fs0.push((l1.clone(), self.unify_utype(f1.clone(), f2.clone())?));
                }
                Ok(Rc::new(UType::Record(fs0)))
            }
            (&UType::Var(v1), &UType::Var(v2)) => {
                self.unify_var_pair(v1, v2)?;
                Ok(Rc::new(UType::Var(Ord::min(v1, v2))))
            }
            (&UType::Var(v), _) => {
                let constraint = Constraint::Equiv(right.clone());
                let after = self.unify_var_constraint(v, constraint)?;
                self.occurs(v)?;
                match after {
                    Constraint::Equiv(t) => Ok(t.clone()),
                    Constraint::Elem(_) | Constraint::Proj(_) =>
                        unreachable!("equiv should erase proj and elem"),
                }
            }
            (_, &UType::Var(v)) => {
                let constraint = Constraint::Equiv(left.clone());
                let after = self.unify_var_constraint(v, constraint)?;
                self.occurs(v)?;
                match after {
                    Constraint::Equiv(t) => Ok(t.clone()),
                    Constraint::Elem(_) | Constraint::Proj(_) =>
                        unreachable!("equiv should erase proj and elem"),
                }
            }
            (UType::Empty, UType::Empty) => Ok(left),
            // all the remaining cases are mismatched UType constructors
            _ => Err(UnificationError::Unsatisfiable(left, right).into()),
        }
    }

    /// Assigns a 'solution' (destructively-updated invariant constraint) to a UVar
    fn unify_var_utype(&mut self, uvar: UVar, solution: Rc<UType>) -> TCResult<()> {
        self.unify_var_constraint(uvar, Constraint::Equiv(solution))?;
        Ok(())
    }

    /// Overwrites the [`Constraints`] at the specified index `tgt_ix` with the immediate value at the specified index `src_ix`, returning
    /// an up-to-date reference to the value at the rewritten index.
    #[must_use]
    fn replace_constraints_from_index(&mut self, tgt_ix: usize, src_ix: usize) -> &Constraints {
        assert_ne!(tgt_ix, src_ix);
        let val = self.constraints[src_ix].clone();
        self.replace_constraints_with_value(tgt_ix, val)
    }

    /// Overwrites the constraints at the specified index with the provided value, returning
    /// an up-to-date reference to the target-indexed [`Constraints`]` element.
    #[must_use]
    fn replace_constraints_with_value(&mut self, ix: usize, val: Constraints) -> &Constraints {
        self.constraints[ix] = val;
        &self.constraints[ix]
    }

    fn infer_var_expr_acc(
        &mut self,
        e: &Expr,
        vt: &ValueType,
        scope: &UScope<'_>
    ) -> TCResult<UVar> {
        let uv = self.infer_var_expr(e, scope)?;
        self.unify_var_valuetype(uv, vt)?;
        Ok(uv)
    }

    fn infer_var_expr<'a>(&mut self, e: &Expr, scope: &'a UScope<'a>) -> TCResult<UVar> {
        let topvar = match e {
            Expr::Var(lbl) =>
                match scope.get_uvar_by_name(lbl) {
                    Some(uv) => {
                        let occ_var = self.get_new_uvar();
                        self.unify_var_pair(uv, occ_var)?;
                        occ_var
                    }
                    None => {
                        unreachable!("encountered unset variable: {lbl}");
                    }
                }
            Expr::Bool(_) => self.init_var_simple(UType::Base(BaseType::Bool))?.0,
            Expr::U8(_) => self.init_var_simple(UType::Base(BaseType::U8))?.0,
            Expr::U16(_) => self.init_var_simple(UType::Base(BaseType::U16))?.0,
            Expr::U32(_) => self.init_var_simple(UType::Base(BaseType::U32))?.0,
            Expr::Tuple(ts) => {
                let newvar = self.get_new_uvar();
                let mut uts = Vec::with_capacity(ts.len());
                for t in ts {
                    uts.push(self.infer_utype_expr(t, scope)?);
                }
                self.unify_var_utype(newvar, Rc::new(UType::Tuple(uts)))?;
                newvar
            }
            &Expr::TupleProj(ref e_tup, ix) => {
                let newvar = self.get_new_uvar();
                let v_tup = self.infer_var_expr(e_tup, scope)?;
                self.unify_var_proj_index(v_tup, ix, newvar)?;
                newvar
            }
            Expr::Record(fs) => {
                let newvar = self.get_new_uvar();
                let mut child = UMultiScope::new(scope);
                for (lbl, f) in fs {
                    let scope = UScope::Multi(&child);
                    let fvar = self.infer_var_expr(f, &scope)?;
                    child.push(lbl.clone(), fvar);
                }
                self.unify_var_utype(newvar, child.into_record_utype())?;
                newvar
            }
            Expr::RecordProj(e_rec, fname) => {
                let newvar = self.get_new_uvar();
                let rec_var = self.infer_var_expr(e_rec, scope)?;
                self.unify_var_proj_field(rec_var, fname.clone(), newvar)?;
                newvar
            }
            Expr::Seq(elems) => {
                let seq_uvar = self.get_new_uvar();
                let elem_uvar = self.get_new_uvar();
                for elem in elems.iter() {
                    let elem_t = self.infer_utype_expr(elem, scope)?;
                    self.unify_var_utype(elem_uvar, elem_t)?;
                }
                self.unify_var_utype(
                    seq_uvar,
                    Rc::new(UType::Seq(Rc::new(UType::Var(elem_uvar))))
                )?;
                seq_uvar
            }
            Expr::Match(head, branches) => {
                let newvar = self.get_new_uvar();
                let head_t = self.infer_utype_expr(head, scope)?;
                for (pat, rhs_expr) in branches.iter() {
                    self.unify_utype_expr_match_case(head_t.clone(), pat, newvar, rhs_expr, scope)?;
                }
                newvar
            }
            Expr::Lambda(_, _) => {
                unreachable!("infer_utype_expr: cannot directly infer utype of lambda expression")
            }

            Expr::Variant(vname, inner) => {
                let newvar = self.get_new_uvar();
                let inner_t = self.infer_utype_expr(inner, scope)?;
                self.add_uvar_variant(newvar, vname.clone(), inner_t)?;
                newvar
            }

            Expr::Arith(_arith, x, y) => {
                let zvar = self.get_new_uvar();

                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let tx = Rc::new(UType::Var(xvar));
                let _cx = self.unify_utype_baseset(tx, BaseSet::UAny)?;

                let yvar = self.infer_var_expr(y.as_ref(), scope)?;
                let ty = Rc::new(UType::Var(yvar));
                let _cy = self.unify_utype_baseset(ty, BaseSet::UAny)?;

                self.unify_var_pair(zvar, xvar)?;
                self.unify_var_pair(zvar, yvar)?;
                zvar
            }
            Expr::IntRel(_rel, x, y) => {
                let zvar = self.get_new_uvar();

                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let tx = Rc::new(UType::Var(xvar));
                let _cx = self.unify_utype_baseset(tx, BaseSet::UAny)?;

                let yvar = self.infer_var_expr(y.as_ref(), scope)?;
                let ty = Rc::new(UType::Var(yvar));
                let _cy = self.unify_utype_baseset(ty, BaseSet::UAny)?;

                self.unify_var_pair(xvar, yvar)?;

                let cz = Constraint::Elem(BaseSet::Single(BaseType::Bool));
                self.unify_var_constraint(zvar, cz)?;

                zvar
            }

            Expr::AsU8(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U8))?.0;
                let ut = self.infer_utype_expr(x.as_ref(), scope)?;
                self.unify_utype_baseset(ut, BaseSet::UAny)?;
                newvar
            }
            Expr::AsU16(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U16))?.0;
                let ut = self.infer_utype_expr(x.as_ref(), scope)?;
                self.unify_utype_baseset(ut, BaseSet::UAny)?;
                newvar
            }
            Expr::AsU32(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U32))?.0;
                let ut = self.infer_utype_expr(x.as_ref(), scope)?;
                self.unify_utype_baseset(ut, BaseSet::UAny)?;
                newvar
            }
            Expr::AsChar(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::Char))?.0;
                let ut = self.infer_utype_expr(x.as_ref(), scope)?;
                self.unify_utype_baseset(ut, BaseSet::UAny)?;
                newvar
            }

            Expr::U16Be(bytes) | Expr::U16Le(bytes) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U16))?.0;
                let ut = self.infer_utype_expr(bytes.as_ref(), scope)?;
                self.unify_utype(ut, Rc::new(UType::tuple([BaseType::U8; 2])))?;
                newvar
            }
            Expr::U32Be(bytes) | Expr::U32Le(bytes) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U32))?.0;
                let ut = self.infer_utype_expr(bytes.as_ref(), scope)?;
                self.unify_utype(ut, Rc::new(UType::tuple([BaseType::U8; 4])))?;
                newvar
            }
            Expr::SeqLength(seq_expr) => {
                // REVIEW - does this always have to be U32?
                let newvar = self.init_var_simple(UType::Base(BaseType::U32))?.0;
                let seq_t = self.infer_utype_expr(seq_expr.as_ref(), scope)?;
                self.unify_utype(seq_t, Rc::new(UType::Seq(Rc::new(UType::Hole))))?;
                newvar
            }
            Expr::SubSeq(seq_expr, start_expr, len_expr) => {
                let newvar = self.get_new_uvar();
                let seq_t = self.infer_utype_expr(seq_expr.as_ref(), scope)?;

                let start_t = self.infer_utype_expr(start_expr.as_ref(), scope)?;
                let len_t = self.infer_utype_expr(len_expr.as_ref(), scope)?;

                // FIXME - this may break on some formats, but it is based on a change of model we want to impose upstream
                self.unify_utype_baseset(start_t, BaseSet::USome)?;
                self.unify_utype_baseset(len_t, BaseSet::USome)?;

                // ensure that seq_t is a sequence type, and then equate it to newvar
                self.unify_utype(seq_t.clone(), Rc::new(UType::Seq(Rc::new(UType::Hole))))?;
                self.unify_var_utype(newvar, seq_t)?;

                newvar
            }
            Expr::FlatMap(f_expr, seq_expr) => {
                let newvar = self.get_new_uvar();

                let (in_var, out_var) = self.infer_vars_expr_lambda(f_expr.as_ref(), scope)?;
                let seq_t = self.infer_utype_expr(seq_expr.as_ref(), scope)?;

                self.unify_utype(seq_t, Rc::new(UType::Seq(Rc::new(UType::Var(in_var)))))?;
                self.unify_var_utype(out_var, Rc::new(UType::Seq(Rc::new(UType::Hole))))?;

                self.unify_var_pair(newvar, out_var)?;

                newvar
            }
            Expr::FlatMapAccum(f_expr, acc_expr, acc_vt, seq_expr) => {
                // NOTE - ((acc, x) -> (acc, y)) -> acc -> Vt(acc) -> [x] -> [y]
                let ys_var = self.get_new_uvar();

                let (acc_x_var, acc_y_var) = self.infer_vars_expr_lambda(f_expr, scope)?;
                let acc_var = self.infer_var_expr_acc(&acc_expr, acc_vt, scope)?;
                let xs_var = self.infer_var_expr(&seq_expr, scope)?;
                let x_var = self.get_new_uvar();
                let y_var = self.get_new_uvar();

                self.unify_var_proj_elem(ys_var, y_var)?;
                self.unify_var_proj_elem(xs_var, x_var)?;
                self.unify_var_proj_index(acc_x_var, 0, acc_var)?;
                self.unify_var_proj_index(acc_y_var, 0, acc_var)?;
                self.unify_var_proj_index(acc_x_var, 1, x_var)?;
                self.unify_var_proj_index(acc_y_var, 1, y_var)?;

                ys_var
            }
            Expr::Dup(count, x) => {
                let newvar = self.get_new_uvar();
                let count_t = self.infer_utype_expr(&count, scope)?;
                let x_t = self.infer_utype_expr(&x, scope)?;

                // REVIEW - this choice may have effects down the road if forced to unify against an incompatible numeric type
                self.unify_utype_baseset(count_t, BaseSet::USome)?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(x_t)))?;

                newvar
            }
            Expr::Inflate(seq_expr) => {
                let newvar = self.get_new_uvar();
                let seq_t = self.infer_utype_expr(&seq_expr, scope)?;
                let Some(elem_t) = self.substitute_hole(
                    Rc::new(UType::Seq(Rc::new(UType::Hole))),
                    seq_t
                )? else {
                    unreachable!("substitute_hole failure")
                };
                // FIXME - not sure what checks to apply to elem_t
                debug_assert!(matches!(elem_t, _));

                self.unify_var_utype(
                    newvar,
                    Rc::new(UType::Seq(Rc::new(UType::Base(BaseType::U8))))
                )?;
                newvar
            }
        };
        Ok(topvar)
    }

    fn infer_utype_expr(&mut self, e: &Expr, scope: &'_ UScope<'_>) -> TCResult<Rc<UType>> {
        let var = self.infer_var_expr(e, scope)?;
        Ok(Rc::new(UType::Var(var)))
    }

    /// Unifies a UType against a BaseSet, updating any variable constraints in the process.
    ///
    /// Fails (with `Err`) for any shapeful UType other than `Base` or `Var`.
    ///
    /// Otherwise returns a copy of the novel constraint implied by the unification.
    fn unify_utype_baseset(&mut self, ut: Rc<UType>, bs: BaseSet) -> TCResult<Constraint> {
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
            other =>
                Err(
                    UnificationError::Unsatisfiable(
                        Constraint::Equiv(ut),
                        bs.to_constraint()
                    ).into()
                ),
        }
    }

    /// Attempts to replace a single `UType::Hole` (either the only one, or the first encountered in DFS order)
    /// in `pattern` with the non-Hole UType its equivalent-position value in `tree`.
    ///
    /// If no hole is found in `pattern`, returns `Ok(None)`.
    /// If unification is impossible, returns `Err` with the corresponding error.
    ///
    /// Note that mutability is not technically required, except we have no version of a non-destructive check
    /// for potential unification, and so we perform a mutable unification internally to propogate the
    /// relevant errors.
    ///
    /// # Panics
    ///
    /// Will panic if any hole is found in `tree` (but only before a hole is found in `pattern`).
    fn substitute_hole(&self, pattern: Rc<UType>, tree: Rc<UType>) -> TCResult<Option<Rc<UType>>> {
        let VType::Abstract(tree) = self.to_whnf_vtype(tree.clone()) else {
            unreachable!("bad tree: {tree:?}");
        };
        match (pattern.as_ref(), tree.as_ref()) {
            (_, UType::Hole) =>
                unreachable!(
                    "subsitute_hole: only pattern-argument should contain holes, but found one in tree-argument position"
                ),
            (UType::Hole, _) => Ok(Some(tree)),
            (UType::Var(_) | UType::Base(_) | UType::Empty, _) => {
                // These can never be Hole or contain Hole, so skip even if they might not unify
                Ok(None)
            }
            (UType::Tuple(pats), UType::Tuple(trees)) => {
                for (pat0, tree0) in Iterator::zip(pats.iter(), trees.iter()) {
                    match self.substitute_hole(pat0.clone(), tree0.clone())? {
                        None => {
                            continue;
                        }
                        Some(ret) => {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            (UType::Record(lpats), UType::Record(ltrees)) => {
                for ((p_lbl, pat0), (t_lbl, tree0)) in Iterator::zip(lpats.iter(), ltrees.iter()) {
                    if p_lbl != t_lbl {
                        unreachable!(
                            "mismatched fields `{p_lbl}` and `{t_lbl}` of `{pattern:?}` and `{tree:?}`"
                        );
                    }
                    match self.substitute_hole(pat0.clone(), tree0.clone())? {
                        None => {
                            continue;
                        }
                        Some(ret) => {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            (UType::Seq(pat0), UType::Seq(tree0)) => {
                self.substitute_hole(pat0.clone(), tree0.clone())
            }
            _ => Err(TypeError::Unsatisfiable(pattern, tree).into()),
        }
    }

    /// Returns the number of variants already associated with a possibly-variant [`UVar`].
    ///
    /// Returns None if there is no acceptable answer, i.e. the metavariable has an invariant
    /// constraint or if it is pointing to a deleted or otherwise out-of-index [`VarMap`].
    ///
    /// Returns Some(n) where `n` is the size of the `VarMap` pointed to by `uv`, or
    /// in the case of an unconstrained metavariable, `Some(0)`.
    fn len_varmap(&self, uv: UVar) -> Option<usize> {
        let var = self.get_canonical_uvar(uv);
        match &self.constraints[var.0] {
            Constraints::Indefinite => Some(0),
            Constraints::Invariant(_) => None,
            Constraints::Variant(vmid) => {
                if let Some(vm) = self.varmaps.as_inner().get(&vmid.0) {
                    Some(vm.len())
                } else {
                    None
                }
            }
        }
    }

    /// Attempt to unify a [`UVar`] with a [`ValueType`], primarily for use with `Expr::FlatMapAccum`.
    fn unify_var_valuetype(&mut self, uv: UVar, vt: &ValueType) -> TCResult<()> {
        // FIXME - only top-level unions are handled property, any embedded union will break down
        match vt {
            ValueType::Union(branches) => {
                // record how many branches we expect uv to have after adding all variants
                let n_branches = branches.len();
                for (label, branch) in branches.iter() {
                    let Some(ut) = UType::from_valuetype(branch) else {
                        todo!("unify_var_vtype: handle unions properly")
                    };
                    self.add_uvar_variant(uv, label.clone(), Rc::new(ut))?;
                }
                // check that we have as many branches as we just added (i.e. all known variants are from this ValueType)
                match self.len_varmap(uv) {
                    None => unreachable!("bad varmap (or incompatible constraint) for {uv}"),
                    Some(n) =>
                        assert_eq!(
                            n,
                            n_branches,
                            "post-unification {uv} has {} branches, but its supposed value-type has {}",
                            n,
                            n_branches
                        ),
                }
                Ok(())
            }
            _ => {
                let Some(ut) = UType::from_valuetype(vt) else {
                    todo!("unify_var_vtype: handle unions properly")
                };
                self.unify_var_utype(uv, Rc::new(ut))?;
                Ok(())
            }
        }
    }

    /// Takes two standalone `Constraint` objects and attempts to unify them, unifying any intermediate
    /// constraints that may occur in the same position in their expansion.
    ///
    /// Returns the value of the constraint that represents their unified form.
    ///
    /// If any subordinate unification results in an error, short-circuits and returns this error to the caller instead.
    fn unify_constraint_pair(&mut self, c1: Constraint, c2: Constraint) -> TCResult<Constraint> {
        match (c1, c2) {
            (Constraint::Equiv(t1), Constraint::Equiv(t2)) => {
                if t1 == t2 {
                    Ok(Constraint::Equiv(t1.clone()))
                } else {
                    let t0 = self.unify_utype(t1.clone(), t2.clone())?;
                    Ok(Constraint::Equiv(t0))
                }
            }
            | (Constraint::Equiv(ut), Constraint::Elem(bs))
            | (Constraint::Elem(bs), Constraint::Equiv(ut)) => {
                Ok(self.unify_utype_baseset(ut.clone(), bs)?)
            }
            (Constraint::Elem(bs1), Constraint::Elem(bs2)) => {
                let bs0 = bs1.union(&bs2)?;
                Ok(bs0.to_constraint())
            }
            (
                Constraint::Proj(ProjShape::TupleWith(t1)),
                Constraint::Proj(ProjShape::TupleWith(t2)),
            ) => {
                if t1 == t2 {
                    return Ok(Constraint::Proj(ProjShape::TupleWith(t1)));
                }

                let mut t0 = BTreeMap::new();

                let keys_t1 = t1.keys().copied().collect::<HashSet<_>>();
                let keys_t2 = t2.keys().copied().collect::<HashSet<_>>();

                let keys_t0 = HashSet::union(&keys_t1, &keys_t2);

                for key in keys_t0.into_iter() {
                    match (t1.get(&key), t2.get(&key)) {
                        (Some(var1), Some(var2)) => {
                            self.unify_var_pair(*var1, *var2)?;
                            t0.insert(*key, Ord::min(*var1, *var2));
                        }
                        (Some(var), None) | (None, Some(var)) => {
                            t0.insert(*key, *var);
                        }
                        _ => unreachable!("key must be in at least one of t1, t2"),
                    }
                }

                Ok(Constraint::Proj(ProjShape::TupleWith(t0)))
            }
            (
                Constraint::Proj(ProjShape::RecordWith(r1)),
                Constraint::Proj(ProjShape::RecordWith(r2)),
            ) => {
                if r1 == r2 {
                    return Ok(Constraint::Proj(ProjShape::RecordWith(r1)));
                }

                let mut r0 = BTreeMap::new();

                let keys_r1 = r1.keys().cloned().collect::<HashSet<_>>();
                let keys_r2 = r2.keys().cloned().collect::<HashSet<_>>();

                let keys_r0 = HashSet::union(&keys_r1, &keys_r2);

                for key in keys_r0.into_iter() {
                    match (r1.get(key), r2.get(key)) {
                        (Some(var1), Some(var2)) => {
                            self.unify_var_pair(*var1, *var2)?;
                            r0.insert(key.clone(), Ord::min(*var1, *var2));
                        }
                        (Some(var), None) | (None, Some(var)) => {
                            r0.insert(key.clone(), *var);
                        }
                        _ => unreachable!("key must be in at least one of r1, r2"),
                    }
                }

                Ok(Constraint::Proj(ProjShape::RecordWith(r0)))
            }
            | (ref c1 @ Constraint::Proj(ref p), ref c2 @ Constraint::Equiv(ref ut))
            | (ref c1 @ Constraint::Equiv(ref ut), ref c2 @ Constraint::Proj(ref p)) => {
                match (p, ut.as_ref()) {
                    (ProjShape::RecordWith(fld_p), UType::Record(fld_ut)) => {
                        let keys_p = fld_p.keys().cloned().collect::<HashSet<_>>();
                        let mut keys_ut = HashSet::new();

                        for (fld, ut) in fld_ut.iter() {
                            keys_ut.insert(fld.clone());
                            if let Some(var) = fld_p.get(fld) {
                                self.unify_var_utype(*var, ut.clone())?;
                            }
                        }

                        if keys_ut.is_superset(&keys_p) {
                            Ok(Constraint::Equiv(ut.clone()))
                        } else {
                            Err(UnificationError::Unsatisfiable(c1.clone(), c2.clone()).into())
                        }
                    }
                    (ProjShape::RecordWith(flds), other) =>
                        unreachable!("could not match Record-Shape {flds:?} against {other:?}"),
                    (ProjShape::TupleWith(elt_p), UType::Tuple(elt_ut)) => {
                        let keys_p = elt_p.keys().copied().collect::<HashSet<_>>();
                        let mut keys_ut = HashSet::new();

                        for (ix, ut) in elt_ut.iter().enumerate() {
                            keys_ut.insert(ix);
                            if let Some(var) = elt_p.get(&ix) {
                                self.unify_var_utype(*var, ut.clone())?;
                            }
                        }

                        if keys_ut.is_superset(&keys_p) {
                            Ok(Constraint::Equiv(ut.clone()))
                        } else {
                            Err(
                                TCError::Unification(
                                    UnificationError::Unsatisfiable(c1.clone(), c2.clone())
                                )
                            )
                        }
                    }
                    (ProjShape::TupleWith(elts), other) =>
                        unreachable!("could not match Tuple-Shape {elts:?} against {other:?}"),
                    (ProjShape::SeqOf(elem_v), UType::Seq(elem_t)) => {
                        self.unify_var_utype(*elem_v, elem_t.clone())?;
                        Ok(Constraint::Equiv(ut.clone()))
                    }
                    (ProjShape::SeqOf(elem_v), other) =>
                        unreachable!("could not match Seq-Shape against {other:?}"),
                }
            }
            (c1, c2) => Err(UnificationError::Unsatisfiable(c1, c2).into()),
        }
    }

    /// Takes two pairs UVar and VMId, which are implicitly equated within each pair (i.e. `v1 ~ vmid1` and `v2 ~ vmid2`)
    /// and performs VarMap unification, erasing the unused map after all updates are made, and returning the new VMId
    /// of the remaining VarMap that both variables should now be equated with.
    fn unify_varmaps(&mut self, v1: UVar, vmid1: VMId, v2: UVar, vmid2: VMId) -> TCResult<VMId> {
        if vmid1 == vmid2 {
            return Ok(vmid1);
        }

        let (lo, hi) = if vmid1 < vmid2 { (vmid1, vmid2) } else { (vmid2, vmid1) };

        let vm_hi = self.varmaps.get_varmap_mut(hi);
        let hi_entries = vm_hi.drain().collect::<Vec<_>>();

        for (vname, inner) in hi_entries.into_iter() {
            if let Some(t_lo) = self.varmaps.get_varmap(lo).get(&vname) {
                let t_hi = inner;
                let unified = self.unify_utype(t_lo.clone(), t_hi.clone())?;
                let _ = self.varmaps.get_varmap_mut(lo).insert(vname, unified);
            } else {
                self.varmaps.get_varmap_mut(lo).insert(vname, inner);
            }
        }

        // delete varmap at hi to end up in a clean-ish state
        let Some(_hi) = self.varmaps.as_inner_mut().remove(&hi.0) else {
            unreachable!("missing varmap {hi} could not be removed")
        };

        // ensure both variables now point to lo
        self.repoint_vmid(v1, lo);
        self.repoint_vmid(v2, lo);

        // return the de-facto vmid for both variables
        Ok(lo)
    }

    fn infer_vars_expr_lambda<'a>(
        &mut self,
        expr: &Expr,
        scope: &'a UScope<'a>
    ) -> TCResult<(UVar, UVar)> {
        match expr {
            Expr::Lambda(head, body) => {
                let head_var = self.get_new_uvar();
                let body_scope = USingleScope::new(scope, &head, head_var);
                let body_var = self.infer_var_expr(body.as_ref(), &UScope::Single(body_scope))?;

                Ok((head_var, body_var))
            }
            _ => unreachable!("infer_utype_expr_lambda: unexpected non-lambda expr {expr:?}"),
        }
    }

    /// Attempt to substitute a variable for a VType with at least one more level of refinement
    ///
    /// If there is no possible direct substitution for a VType (i.e. no known constriants),
    /// or multiple possible non-base solutions that cannot be easily tiebroken, returns Ok(None).
    ///
    /// If the only possible refinement would be the identity transformation modulo aliasing, likewise returns
    /// Ok(None).
    ///
    /// If an occurs check fails, returns the corresponding `Err(_)` value.
    ///
    /// Otherwise, returns Some(t) where t is a UType other than UType::Var(v) or UType::Var(v1) where
    /// v1 is an isomorphic alias to v.
    fn substitute_uvar_vtype(&self, v: UVar) -> Result<Option<VType>, TCError> {
        self.occurs(v)?;
        Ok(match &self.constraints[self.get_canonical_uvar(v).0] {
            Constraints::Indefinite => None,
            Constraints::Variant(vmid) => Some(VType::IndefiniteUnion(*vmid)),
            Constraints::Invariant(c) => {
                match c {
                    Constraint::Equiv(ut) => Some(self.to_whnf_vtype(ut.clone())),
                    Constraint::Elem(bs) => Some(VType::Base(*bs)),
                    Constraint::Proj(ps) => Some(self.proj_shape_to_vtype(ps)),
                }
            }
        })
    }

    fn proj_shape_to_vtype(&self, shape: &ProjShape) -> VType {
        match shape {
            ProjShape::TupleWith(ix_vars) => {
                let mut flat = Vec::new();
                for (ix, var) in ix_vars.iter() {
                    while *ix > flat.len() {
                        flat.push(Rc::new(UType::Hole));
                    }
                    flat.push((*var).into());
                }
                VType::ImplicitTuple(flat)
            }
            ProjShape::RecordWith(fld_vars) => {
                let mut flat = Vec::new();
                for (lbl, var) in fld_vars.iter() {
                    flat.push((lbl.clone(), (*var).into()));
                }
                VType::ImplicitRecord(flat)
            }
            ProjShape::SeqOf(v) => VType::Abstract((*v).into()),
        }
    }

    fn repoint_vmid(&mut self, v: UVar, id: VMId) {
        match &mut self.constraints[v.0] {
            Constraints::Variant(placeholder) => {
                *placeholder = id;
            }
            other => unreachable!("repoint_vmid expects Constraints::Variant, found {other:?}"),
        }
    }
}

// SECTION - low-level methods dealing with UVar aliasing concerns
impl TypeChecker {
    /// Performs re-aliasing, recanonicalization, constraint propogation, and constraint unification
    /// to establish direct equality requirements between two metavariables.
    fn unify_var_pair(&mut self, v1: UVar, v2: UVar) -> TCResult<&Constraints> {
        if v1 == v2 {
            return Ok(&self.constraints[v1.0]);
        }

        // short-circuit if already equated
        match (&self.aliases[v1.0], &self.aliases[v2.0]) {
            (Alias::NoAlias, Alias::NoAlias) => {
                if v1 < v2 {
                    unsafe {
                        self.repoint(v1.0, v2.0);
                        self.transfer_constraints(v1.0, v2.0)
                    }
                } else {
                    unsafe {
                        self.repoint(v2.0, v1.0);
                        self.transfer_constraints(v2.0, v1.0)
                    }
                }
            }
            (Alias::NoAlias, Alias::BackRef(tgt)) => {
                let can_ix = *tgt;

                if v1.0 > can_ix {
                    unsafe {
                        self.repoint(can_ix, v1.0);
                        self.transfer_constraints(can_ix, v1.0)
                    }
                } else if v1.0 < can_ix {
                    debug_assert!(
                        self.aliases[can_ix].is_canonical_nonempty(),
                        "half-alias ?{can_ix}-|<-{v2}"
                    );
                    debug_assert!(
                        !self.aliases[can_ix].contains_fwd_ref(v1.0),
                        "retrograde half-aliased 'forward' ref ?{can_ix}->|-{v1}"
                    );
                    unsafe { self.recanonicalize(v1.0, can_ix) }
                } else {
                    unreachable!("unexpected half-alias {v1}-|<-{v2}");
                }
            }
            (Alias::BackRef(tgt), Alias::NoAlias) => {
                let can_ix = *tgt;
                if v2.0 > can_ix {
                    unsafe {
                        self.repoint(can_ix, v2.0);
                        self.transfer_constraints(can_ix, v2.0)
                    }
                } else if v2.0 < can_ix {
                    debug_assert!(
                        self.aliases[can_ix].is_canonical_nonempty(),
                        "half-alias ?{can_ix}-|<-{v1}"
                    );
                    debug_assert!(
                        !self.aliases[can_ix].contains_fwd_ref(v2.0),
                        "retrograde half-aliased 'forward' ref ?{can_ix}->|-{v2}"
                    );
                    unsafe { self.recanonicalize(v2.0, can_ix) }
                } else {
                    unreachable!("unexpected half-alias {v2}-|<-{v1}");
                }
            }
            (Alias::NoAlias, Alias::Canonical(_)) => {
                if v1.0 < v2.0 {
                    debug_assert!(
                        !self.aliases[v2.0].contains_fwd_ref(v1.0),
                        "retrograde half-aliased 'forward' ref {v2}->|-{v1}"
                    );
                    unsafe { self.recanonicalize(v1.0, v2.0) }
                } else {
                    unsafe {
                        self.repoint(v2.0, v1.0);
                        self.transfer_constraints(v2.0, v1.0)
                    }
                }
            }
            (Alias::Canonical(_), Alias::NoAlias) => {
                if v2.0 < v1.0 {
                    debug_assert!(
                        !self.aliases[v1.0].contains_fwd_ref(v2.0),
                        "retrograde half-aliased 'forward' ref {v1}->|-{v2}"
                    );
                    unsafe { self.recanonicalize(v2.0, v1.0) }
                } else {
                    unsafe {
                        self.repoint(v1.0, v2.0);
                        self.transfer_constraints(v1.0, v2.0)
                    }
                }
            }
            (Alias::BackRef(tgt1), Alias::BackRef(tgt2)) => {
                let ix1 = *tgt1;
                let ix2 = *tgt2;

                if ix1 < ix2 {
                    unsafe { self.recanonicalize(ix1, ix2) }
                } else if ix2 < ix1 {
                    unsafe { self.recanonicalize(ix2, ix1) }
                } else {
                    // the two are equal so nothing needs to be changed; we will check both are forward-aliased, however
                    let common = &self.aliases[ix1];
                    debug_assert!(
                        common.contains_fwd_ref(v1.0),
                        "unexpected half-alias ?{ix1}<-{v1}"
                    );
                    debug_assert!(
                        common.contains_fwd_ref(v2.0),
                        "unexpected half-alias ?{ix1}<-{v2}"
                    );
                    Ok(&self.constraints[ix1])
                }
            }
            (a1 @ Alias::BackRef(tgt), a2 @ Alias::Canonical(fwds)) => {
                let left = fwds.contains(tgt);
                let right = *tgt == v2.0;

                match (left, right) {
                    (true, true) => {
                        return Ok(&self.constraints[v2.0]);
                    }
                    (true, false) | (false, true) =>
                        unreachable!(
                            "mismatched back- and forward-references for {v1} ({a1:?}) and {v2} ({a2:?})"
                        ),
                    (false, false) => (),
                }

                let ix1 = *tgt;
                let ix2 = v2.0;

                // check not the actual indices, but the canonical indices for tie-breaking
                if ix1 < ix2 {
                    unsafe { self.recanonicalize(ix1, ix2) }
                } else {
                    unsafe { self.recanonicalize(ix2, ix1) }
                }
            }
            (a1 @ Alias::Canonical(fwds), a2 @ Alias::BackRef(tgt)) => {
                let left = fwds.contains(tgt);
                let right = *tgt == v1.0;

                match (left, right) {
                    (true, true) => {
                        return Ok(&self.constraints[v1.0]);
                    }
                    (true, false) | (false, true) =>
                        unreachable!(
                            "mismatched forward- and back-references for {v1} ({a1:?}) and {v2} ({a2:?})"
                        ),
                    (false, false) => (),
                }

                let ix1 = v1.0;
                let ix2 = *tgt;

                // check not the actual indices, but the canonical indices for tie-breaking
                if ix1 < ix2 {
                    unsafe { self.recanonicalize(ix1, ix2) }
                } else {
                    unsafe { self.recanonicalize(ix2, ix1) }
                }
            }
            (Alias::Canonical(_), Alias::Canonical(_)) => {
                if v1 < v2 {
                    unsafe { self.recanonicalize(v1.0, v2.0) }
                } else {
                    unsafe { self.recanonicalize(v2.0, v1.0) }
                }
            }
        }
    }

    /// Modifies the aliasing table of `self` so that the alias stored at `a1` is canonical and takes over
    /// the old status of canonical `a2`, modifying its back-references and unifying with its constraints
    ///
    /// # Safety
    ///
    /// As this method is designed ot be internal with a specific singular use-case, there are a number of preconditions that must either be
    /// assumed or asserted, in order to ensure that the call is sound and valid. These preconditions are numerous enough to merit unsafe status for
    /// the call to this method, at least as a temporary linting-helper, to avoid unguarded calls from neutral contexts.
    ///
    /// - `a1 < a2`
    /// - `a1` has no back-references but may have some forward-references
    /// - `a2` has no back-references
    unsafe fn recanonicalize(&mut self, a1: usize, a2: usize) -> TCResult<&Constraints> {
        let tmp = self.aliases[a2].set_backref(a1);
        let mut iter = tmp.iter_fwd_refs();
        for a in iter {
            assert!(
                !self.aliases[a1].contains_fwd_ref(a),
                "forward ref of ?{a2} is also a forward ref of ?{a1}, somehow"
            );
            self.repoint(a1, a);
        }
        self.aliases[a1].add_forward_ref(a2);
        self.transfer_constraints(a1, a2)
    }

    /// Rewrites the aliasing of `self` so that `lo<->hi` is enforced, without any other changes.
    ///
    /// Assumes that this aliasing does not exist already, and may cause unexpected but not undefined behavior
    /// if called in a context that does not check certain preconditions.
    ///
    /// # Safety
    ///
    /// Like [`Self::recanonicalize`], this method is niche enough to not be suitable for calls in a neutral context,
    /// and may be marked safe after code stablizes. Otherwise it is not known to lead to any UB.
    /// guards are enforced in advance.
    unsafe fn repoint(&mut self, lo: usize, hi: usize) {
        self.aliases[hi].set_backref(lo);
        self.aliases[lo].add_forward_ref(hi);
    }

    /// Ensure that all constraints on `a2` are inherited by `a1`, with the reverse occuring as a side-effect.
    unsafe fn transfer_constraints(&mut self, a1: usize, a2: usize) -> TCResult<&Constraints> {
        let v1 = UVar(a1);
        let v2 = UVar(a2);
        if v1 == v2 {
            return Ok(&self.constraints[v1.0]);
        }

        match (&self.constraints[v1.0], &self.constraints[v2.0]) {
            (Constraints::Indefinite, Constraints::Indefinite) => Ok(&Constraints::Indefinite),
            (Constraints::Indefinite, _) => Ok(self.replace_constraints_from_index(v1.0, v2.0)),
            (_, Constraints::Indefinite) => Ok(self.replace_constraints_from_index(v2.0, v1.0)),
            (Constraints::Variant(vmid1), Constraints::Variant(vmid2)) => {
                let _ = self.unify_varmaps(v1, *vmid1, v2, *vmid2)?;
                Ok(&self.constraints[v1.0])
            }
            | (Constraints::Variant(vmid), Constraints::Invariant(c))
            | (Constraints::Invariant(c), Constraints::Variant(vmid)) => {
                return Err(TCError::VarianceMismatch(Ord::min(v1, v2), *vmid, c.clone()));
            }
            (Constraints::Invariant(c1), Constraints::Invariant(c2)) => {
                let c0 = self.unify_constraint_pair(c1.clone(), c2.clone())?;
                let _ = self.replace_constraints_with_value(
                    v1.0,
                    Constraints::Invariant(c0.clone())
                );
                let _ = self.replace_constraints_with_value(v2.0, Constraints::Invariant(c0));
                Ok(&self.constraints[v1.0])
            }
        }
    }

    fn get_canonical_uvar(&self, v: UVar) -> UVar {
        match self.aliases[v.0] {
            Alias::Canonical(_) | Alias::NoAlias => v,
            Alias::BackRef(ix) => UVar(ix),
        }
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
            (Alias::NoAlias, _) | (_, Alias::NoAlias) => false,
            (Alias::BackRef(tgt1), Alias::BackRef(tgt2)) => tgt1 == tgt2,
            (Alias::BackRef(tgt), Alias::Canonical(..)) => *tgt == v1.0,
            (Alias::Canonical(..), Alias::BackRef(tgt)) => *tgt == v.0,
            (Alias::Canonical(_), Alias::Canonical(_)) => false,
        }
    }
}
// !SECTION

// SECTION - interface between the typechecker and the rest of the crate
impl TypeChecker {
    pub fn infer_var_format_union<'a>(
        &mut self,
        branches: &[Format],
        ctxt: Ctxt<'a>
    ) -> TCResult<UVar> {
        let newvar = UVar(self.constraints.len());
        // populate new structures for each relevant cross-indexed vector
        self.constraints.push(Constraints::new());
        self.aliases.push(Alias::default());

        for f in branches.into_iter() {
            match f {
                Format::Variant(lbl, inner) => {
                    let typ = self.infer_utype_format(inner.as_ref(), ctxt)?;
                    self.add_uvar_variant(newvar, lbl.clone(), typ)?;
                }
                // FIXME - other is probably going to come up but it should be easy to fix later, this handles the hardest known case
                other => {
                    unreachable!("register_vars_format_union: found non-variant branch {other:?}");
                }
            }
        }
        Ok(newvar)
    }

    /// Assigns new metavariables and simple constraints for a format, and returns the most specific UType possible,
    /// which in many cases will be a Var pointing to a novel UVar.
    pub fn infer_var_format<'a>(&mut self, f: &Format, ctxt: Ctxt<'a>) -> TCResult<UVar> {
        match f {
            Format::ItemVar(level, args) => {
                let newvar = self.get_new_uvar();
                if !args.is_empty() {
                    for arg in args.iter() {
                        let _ = self.infer_var_expr(arg, ctxt.scope)?;
                    }
                }
                let ut = self.infer_utype_format(ctxt.module.get_format(*level), ctxt)?;
                self.unify_var_utype(newvar, ut)?;
                Ok(newvar.into())
            }
            Format::Fail => Ok(self.init_var_simple(UType::Empty)?.0),
            Format::EndOfInput | Format::Align(_) => Ok(self.init_var_simple(UType::UNIT)?.0),
            Format::Byte(_set) => {
                // FIXME - this may be a bit overly pedantic
                if _set.is_empty() {
                    Ok(self.init_var_simple(UType::Empty)?.0)
                } else {
                    Ok(self.init_var_simple(UType::Base(BaseType::U8))?.0)
                }
            }
            Format::Variant(cname, inner) => {
                let newvar = self.get_new_uvar();
                let t_inner = self.infer_utype_format(inner.as_ref(), ctxt)?;
                self.add_uvar_variant(newvar, cname.clone(), t_inner)?;
                Ok(newvar)
            }
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let newvar = self.infer_var_format_union(branches, ctxt)?;
                Ok(newvar)
            }
            Format::Tuple(ts) => {
                let newvar = self.get_new_uvar();
                let mut uts = Vec::with_capacity(ts.len());
                for t in ts {
                    uts.push(self.infer_utype_format(t, ctxt)?);
                }
                self.unify_var_utype(newvar, Rc::new(UType::Tuple(uts)))?;
                Ok(newvar.into())
            }
            Format::Record(fs) => {
                let newvar = self.get_new_uvar();
                let mut child = UMultiScope::new(ctxt.scope);
                for (lbl, f) in fs {
                    let scope = UScope::Multi(&child);
                    let child_ctxt = ctxt.with_scope(&scope);
                    let fv = self.infer_var_format(f, child_ctxt)?;
                    child.push(lbl.clone(), fv);
                }
                let record = child.into_record_constraint();
                self.unify_var_constraint(newvar, record)?;
                Ok(newvar.into())
            }
            // FIXME - logically these should be grouped together, but anything containing an expression has to be typed as a special-case
            Format::Repeat(inner) | Format::Repeat1(inner) => {
                let newvar = self.get_new_uvar();
                let t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(t)))?;
                Ok(newvar.into())
            }
            Format::RepeatCount(n, inner) => {
                let newvar = self.get_new_uvar();
                let n_type = self.infer_utype_expr(n, ctxt.scope)?;
                // NOTE - we don't care about the constraint, only whether it was successfully computed
                let _constraint = self.unify_utype_baseset(n_type, BaseSet::UAny)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(inner_t)))?;
                Ok(newvar.into())
            }
            Format::RepeatUntilLast(f, inner) => {
                let newvar = self.get_new_uvar();
                let (in_var, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(in_var, inner_t.clone())?;
                self.unify_var_utype(out_var, BaseType::Bool.into())?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(inner_t)))?;
                Ok(newvar.into())
            }
            Format::RepeatUntilSeq(f, inner) => {
                let newvar = self.get_new_uvar();
                let (in_var, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(in_var, Rc::new(UType::Seq(inner_t.clone())))?;
                self.unify_var_utype(out_var, BaseType::Bool.into())?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(inner_t)))?;
                Ok(newvar.into())
            }
            Format::Peek(peek) => {
                let newvar = self.get_new_uvar();
                let peek_t = self.infer_utype_format(&peek, ctxt)?;
                self.unify_var_utype(newvar, peek_t)?;
                Ok(newvar.into())
            }
            Format::PeekNot(peek) => {
                let newvar = self.init_var_simple(UType::UNIT)?.0;
                // FIXME - not sure if this is neccessary for any reason other than generating a variable tree...
                let _peek_t = self.infer_utype_format(&peek, ctxt)?;
                Ok(newvar.into())
            }
            Format::Slice(sz, inner) => {
                let newvar = self.get_new_uvar();
                let sz_t = self.infer_utype_expr(sz, ctxt.scope)?;
                self.unify_utype_baseset(sz_t, BaseSet::USome)?;
                let inner_t = self.infer_utype_format(&inner, ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar.into())
            }
            Format::Bits(inner) => {
                let newvar = self.get_new_uvar();
                let inner_t = self.infer_utype_format(&inner, ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar.into())
            }
            Format::WithRelativeOffset(ofs, inner) => {
                let newvar = self.get_new_uvar();
                let sz_t = self.infer_utype_expr(ofs, ctxt.scope)?;
                self.unify_utype_baseset(sz_t, BaseSet::USome)?;
                let inner_t = self.infer_utype_format(&inner, ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar.into())
            }
            Format::Map(inner, f) => {
                let newvar = self.get_new_uvar();
                let inner_t = self.infer_utype_format(&inner, ctxt)?;

                let (in_v, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                self.unify_var_utype(in_v, inner_t)?;
                self.unify_var_pair(newvar, out_var)?;
                Ok(newvar)
            }
            Format::Compute(x) => {
                let newvar = self.get_new_uvar();
                let xt = self.infer_utype_expr(&x, ctxt.scope)?;
                self.unify_var_utype(newvar, xt)?;
                Ok(newvar)
            }
            Format::Let(lab, x, inner) => {
                let newvar = self.get_new_uvar();
                let xvar = self.infer_var_expr(x, ctxt.scope)?;
                let newscope = UScope::Single(USingleScope::new(ctxt.scope, lab, xvar));
                let new_ctxt = ctxt.with_scope(&newscope);
                let t_inner = self.infer_utype_format(&inner, new_ctxt)?;
                self.unify_var_utype(newvar, t_inner)?;
                Ok(newvar)
            }
            Format::Match(x, branches) => {
                let newvar = self.get_new_uvar();
                let tx = self.infer_utype_expr(&x, ctxt.scope)?;
                for (pat, rhs) in branches.iter() {
                    self.unify_utype_format_match_case(tx.clone(), &pat, newvar, &rhs, ctxt)?;
                }
                Ok(newvar)
            }
            Format::Dynamic(_, _, _) => todo!(),
            Format::Apply(_) => todo!(),
        }
    }

    pub fn infer_utype_format<'a>(
        &mut self,
        format: &Format,
        ctxt: Ctxt<'a>
    ) -> TCResult<Rc<UType>> {
        let uv = self.infer_var_format(format, ctxt)?;
        Ok(uv.into())
    }

    /// Attempt to fully solve a `UType` until all free metavariables are replaced with concrete type-assignments
    ///
    /// Returns None if at least one metavariable canoot be reduced without more information, or if any unification
    /// is insoluble.
    pub fn reify(&self, t: Rc<UType>) -> Option<ValueType> {
        match t.as_ref() {
            UType::Hole => {
                // FIXME - should this simply return None instead?
                unreachable!("reify: UType::Hole should be erased by any non-Hole unification!");
            }
            &UType::Var(uv) => {
                let v = self.get_canonical_uvar(uv);
                match self.substitute_uvar_vtype(v) {
                    Ok(Some(t0)) =>
                        match t0 {
                            VType::Base(bs) => {
                                match bs.get_unique_solution(uv).as_deref() {
                                    Ok(UType::Base(b)) => Some(ValueType::Base(*b)),
                                    Ok(other) => unreachable!("unexpected solution {other:?}"),
                                    Err(_) => None,
                                }
                            }
                            VType::Concrete(vt) => Some(vt.as_ref().clone()),
                            VType::Abstract(ut) => self.reify(ut),
                            VType::IndefiniteUnion(vmid) => self.reify_union(vmid),
                            _ =>
                                unreachable!(
                                    "Unsolved implicit Tuple or Record leftover from un-unified projection"
                                ),
                        }
                    Err(_) => None,
                    Ok(None) => {
                        // substitute_uvar_utype returns none for assumed-partial union types, so handle that case proactively
                        match &self.constraints[v.0] {
                            Constraints::Variant(vmid) => self.reify_union(*vmid),
                            _ => None,
                        }
                    }
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
}
// !SECTION

pub(crate) type TypeError = UnificationError<Rc<UType>>;
pub(crate) type ConstraintError = UnificationError<Constraint>;

impl From<TypeError> for ConstraintError {
    fn from(value: TypeError) -> Self {
        match value {
            // UnificationError::Incompatible(ix, lt, rt) => {
            //     let lc = Constraint::Equiv(lt);
            //     let rc = Constraint::Equiv(rt);
            //     UnificationError::Incompatible(ix, lc, rc)
            // }
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
pub(crate) enum UnificationError<T: std::fmt::Debug> {
    // Incompatible(UVar, T, T), // two independent assertions about a UVar are incompatible
    Unsatisfiable(T, T), // a single non-variable assertion is directly unsatisfiable
}

#[derive(Clone, Debug)]
pub(crate) enum TCError {
    VarianceMismatch(UVar, VMId, Constraint), // attempted unification of a variant and non-variant constraint
    Unification(ConstraintError),
    InfiniteType(UVar, Constraints),
    MultipleSolutions(UVar, BaseSet),
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
            TCError::VarianceMismatch(uv, vmid, constraint) =>
                write!(
                    f,
                    "unable to proceed after attempted unification `{uv} {constraint} ∧ {uv} ⊇ {vmid}`"
                ),
            TCError::Unification(c_err) => write!(f, "{c_err}"),
            TCError::InfiniteType(v, constraints) =>
                match constraints {
                    Constraints::Indefinite =>
                        unreachable!("indefinite constraint `{v} = ??` is not infinite"),
                    Constraints::Variant(vmid) =>
                        write!(
                            f,
                            "`{v} ⊇ {vmid}` constitutes an infinite type ({v} or alias occurs within {vmid})"
                        ),
                    Constraints::Invariant(inv) =>
                        match inv {
                            Constraint::Equiv(t) =>
                                write!(
                                    f,
                                    "`{v} = {t:?}` is an infinite type ({v} or alias occurs within the rhs utype)"
                                ),
                            Constraint::Elem(_) =>
                                unreachable!("`{v} {inv}` is not infinite, but we thought it was"),
                            Constraint::Proj(ps) => {
                                write!(f, "`{v} ~ {ps:?}` constitutes an infinite type")
                            }
                        }
                }
            TCError::MultipleSolutions(uv, bs) =>
                write!(f, "no unique solution for `{uv} {}`", bs.to_constraint()),
        }
    }
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

pub(crate) fn typecheck(module: &FormatModule, f: &Format) -> TCResult<TypeChecker> {
    let mut tc = TypeChecker::new();
    let scope = UScope::new();
    let ctxt = Ctxt::new(module, &scope);
    let _ut = tc.infer_utype_format(f, ctxt)?;
    // FIXME - there should be a lot more that goes on under the covers here, especially since we want to detect errors
    Ok(tc)
}

pub(crate) type TCResult<T> = Result<T, TCError>;

mod __impls {
    use super::{ Constraint, ProjShape, UVar, VMId };
    use std::borrow::{ Borrow, BorrowMut };

    impl std::fmt::Display for Constraint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Constraint::Equiv(ut) => write!(f, "= {ut:?}"),
                Constraint::Elem(bs) => write!(f, "∈ {bs}"),
                Constraint::Proj(ps) =>
                    match ps {
                        ProjShape::TupleWith(ts) =>
                            write!(
                                f,
                                "~ TupleGT{}(..)",
                                ts
                                    .last_key_value()
                                    .map(|x| *x.0)
                                    .unwrap_or(0)
                            ),
                        ProjShape::RecordWith(fs) =>
                            write!(f, "~ Record(..) (>={} fields)", fs.len()),
                        ProjShape::SeqOf(elv) => write!(f, "~ Seq({elv})"),
                    }
            }
        }
    }

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

#[cfg(test)]
mod tests {
    use crate::byte_set::ByteSet;
    use crate::Arith;

    use super::*;

    fn adhoc_union(iter: impl IntoIterator<Item = (&'static str, Format)>) -> Format {
        let tmp = iter
            .into_iter()
            .map(|(str, fmt)| Format::Variant(Label::from(str), Box::new(fmt)))
            .collect();
        Format::Union(tmp)
    }

    #[test]
    fn test_union() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = adhoc_union([
            ("A", Format::Byte(ByteSet::full())),
            ("B", Format::EndOfInput),
        ]);
        let mut module = FormatModule::new();
        module.define_format("foo", format.clone());
        let scope = UScope::new();
        let ut = tc.infer_utype_format(&format, Ctxt::new(&module, &scope))?;
        println!("ut: {ut:?}");
        println!("tc: {tc:?}");
        let oput = tc.reify(ut).unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Union(
            vec![
                ("A".into(), ValueType::Base(BaseType::U8)),
                ("B".into(), ValueType::Tuple(vec![]))
            ]
        );
        assert_eq!(oput, expected);
        return Ok(());
    }

    #[test]
    fn test_non_union_complex() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::Record(
            vec![
                ("number".into(), Format::Byte(ByteSet::full())),
                (
                    "isEven".into(),
                    Format::Compute(
                        Expr::Match(
                            Box::new(
                                Expr::Arith(
                                    Arith::Rem,
                                    Box::new(Expr::Var("number".into())),
                                    Box::new(Expr::U8(2))
                                )
                            ),
                            vec![
                                (Pattern::U8(0), Expr::Bool(true)),
                                (Pattern::Wildcard, Expr::Bool(false))
                            ]
                        )
                    ),
                )
            ]
        );
        let mut module = FormatModule::new();
        module.define_format("foo", format.clone());
        let scope = UScope::new();
        let ut = tc.infer_utype_format(&format, Ctxt::new(&module, &scope))?;
        println!("ut: {ut:?}");
        println!("tc: {tc:?}");
        let oput = tc.reify(ut).unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Record(
            vec![
                ("number".into(), ValueType::Base(BaseType::U8)),
                ("isEven".into(), ValueType::Base(BaseType::Bool))
            ]
        );
        assert_eq!(oput, expected);
        return Ok(());
    }

    #[test]
    fn test_union_complex() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::Record(
            vec![
                ("number".into(), Format::Byte(ByteSet::full())),
                (
                    "parity".into(),
                    Format::Compute(
                        Expr::Match(
                            Box::new(
                                Expr::Arith(
                                    Arith::Rem,
                                    Box::new(Expr::Var("number".into())),
                                    Box::new(Expr::U8(2))
                                )
                            ),
                            vec![
                                (
                                    Pattern::U8(0),
                                    Expr::Variant("Even".into(), Box::new(Expr::UNIT)),
                                ),
                                (
                                    Pattern::Wildcard,
                                    Expr::Variant("Odd".into(), Box::new(Expr::UNIT)),
                                )
                            ]
                        )
                    ),
                )
            ]
        );
        let mut module = FormatModule::new();
        module.define_format("foo", format.clone());
        let scope = UScope::new();
        let ut = tc.infer_utype_format(&format, Ctxt::new(&module, &scope))?;
        println!("ut: {ut:?}");
        println!("tc: {tc:?}");
        let oput = tc.reify(ut).unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Record(
            vec![
                ("number".into(), ValueType::Base(BaseType::U8)),
                (
                    "parity".into(),
                    ValueType::Union(
                        vec![("Even".into(), ValueType::UNIT), ("Odd".into(), ValueType::UNIT)]
                    ),
                )
            ]
        );
        assert_eq!(oput, expected);
        return Ok(());
    }

    fn mk_format_u32() -> Format {
        Format::Map(
            Box::new(Format::Tuple(vec![Format::Byte(ByteSet::full()); 4])),
            Expr::Lambda("x".into(), Box::new(Expr::U32Be(Box::new(Expr::Var("x".into())))))
        )
    }

    #[test]
    fn test_lambda_accum() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::Map(
            Box::new(Format::Repeat(Box::new(mk_format_u32()))),
            Expr::Lambda(
                "xs".into(),
                Box::new(
                    Expr::FlatMapAccum(
                        Box::new(
                            Expr::Lambda(
                                "acc_x".into(),
                                Box::new(
                                    Expr::Tuple(
                                        vec![
                                            Expr::Arith(
                                                Arith::Mul,
                                                Box::new(
                                                    Expr::TupleProj(
                                                        Box::new(Expr::Var("acc_x".into())),
                                                        0
                                                    )
                                                ),
                                                Box::new(
                                                    Expr::TupleProj(
                                                        Box::new(Expr::Var("acc_x".into())),
                                                        1
                                                    )
                                                )
                                            ),
                                            Expr::Arith(
                                                Arith::Add,
                                                Box::new(Expr::U32(1)),
                                                Box::new(
                                                    Expr::Arith(
                                                        Arith::Mul,
                                                        Box::new(
                                                            Expr::TupleProj(
                                                                Box::new(Expr::Var("acc_x".into())),
                                                                0
                                                            )
                                                        ),
                                                        Box::new(
                                                            Expr::TupleProj(
                                                                Box::new(Expr::Var("acc_x".into())),
                                                                1
                                                            )
                                                        )
                                                    )
                                                )
                                            )
                                        ]
                                    )
                                )
                            )
                        ),
                        Box::new(Expr::U32(1)),
                        ValueType::Base(BaseType::U32),
                        Box::new(Expr::Var("xs".into()))
                    )
                )
            )
        );
        let module = FormatModule::new();
        // module.define_format("prod32", format.clone());
        let scope = UScope::new();
        let ut = tc.infer_utype_format(&format, Ctxt::new(&module, &scope))?;
        println!("ut: {ut:?}");
        println!("tc: {tc:?}");
        let oput = tc.reify(ut).unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Base(BaseType::U32);
        assert_eq!(oput, expected);
        return Ok(());
    }
}
