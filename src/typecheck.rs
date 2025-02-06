use crate::{
    Arith, BaseType, DynFormat, Expr, Format, FormatModule, Label, Pattern, UnaryOp, ValueType,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    rc::Rc,
};

/// Perform a `?` operation but add additional trace-context to TCError values if encountered
///
/// # Syntax
///
/// ```ignore
/// try_with!( self.unify_var_pair(v1, v2) => ("unify_var_pair", v1, v2) );
/// try_with!( self.unify_var_pair(v1, v2) ); // equivalent to `?`
/// ```
#[allow(unused_macros)]
macro_rules! try_with {
    ($x:expr => $y:expr) => {
        match $x {
            Ok(val) => val,
            Err(e) => return Err(e.with_trace($y)),
        }
    };
    ($x:expr $(=> ())?) => {
        $x?
    };
}

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UVar(usize);

impl UVar {
    pub fn new(ix: usize) -> Self {
        Self(ix)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

/// Unification type, equivalent to ValueType up to abstraction
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum UType {
    /// Reserved case for Formats that fundamentally cannot be parsed successfully (Format::Fail and implied failure-cases)
    Empty,
    /// Anonymous type-hole for shape-only unifications (i.e. where we would want to use a meta-variable but don't have one available).
    Hole,
    /// Indexed type-hole acting as a unification metavariable
    Var(UVar),
    Base(BaseType),
    Tuple(Vec<Rc<UType>>),
    Record(Vec<(Label, Rc<UType>)>),
    Seq(Rc<UType>),
    /// For `std::option::Option<InnerType>`
    Option(Rc<UType>),
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

    pub fn tuple<T>(elems: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Rc<UType>>,
    {
        Self::Tuple(elems.into_iter().map(Into::into).collect())
    }

    /// Attempts to convert a `ValueType` to an `UType`, returning `Some(ut)` if the conversion was successful.
    ///
    /// Will return `None` if the conversion failed due to the presence of a Union-type at any layer.
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
            ValueType::Option(inner) => {
                let inner_t = Self::from_valuetype(inner)?;
                Some(UType::Option(Rc::new(inner_t)))
            }
            ValueType::Seq(inner) => Some(Self::Seq(Rc::new(Self::from_valuetype(inner)?))),
        }
    }
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

    pub fn push(&mut self, name: Label, v: UVar) {
        self.entries.push((name, v));
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
    pub(crate) module: &'a FormatModule,
    pub(crate) scope: &'a UScope<'a>,
    pub(crate) dyn_s: DynScope<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DynScope<'a> {
    Empty,
    Single(DynSingleScope<'a>),
}

impl<'a> DynScope<'a> {
    pub const fn new() -> Self {
        Self::Empty
    }

    pub(crate) fn get_dynf_var_by_name(&self, label: &str) -> Option<UVar> {
        match self {
            DynScope::Empty => None,
            DynScope::Single(single) => single.get_dynf_var_by_name(label),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DynSingleScope<'a> {
    parent: &'a DynScope<'a>,
    name: &'a str,
    dynf_var: UVar,
}

impl<'a> DynSingleScope<'a> {
    pub const fn new(parent: &'a DynScope<'a>, name: &'a str, dynf_var: UVar) -> Self {
        Self {
            parent,
            name,
            dynf_var,
        }
    }

    fn get_dynf_var_by_name(&self, label: &str) -> Option<UVar> {
        if label == self.name {
            Some(self.dynf_var)
        } else {
            self.parent.get_dynf_var_by_name(label)
        }
    }
}

impl<'a> Ctxt<'a> {
    /// Returns a copy of `self` with the given `UScope` instead of `self.scope`.
    pub(crate) fn with_scope(&'a self, scope: &'a UScope<'a>) -> Ctxt<'a> {
        Self {
            module: self.module,
            dyn_s: self.dyn_s,
            scope,
        }
    }

    pub(crate) fn with_dyn_binding(&'a self, name: &'a str, dynf_var: UVar) -> Ctxt<'a> {
        Self {
            module: self.module,
            dyn_s: DynScope::Single(DynSingleScope::new(&self.dyn_s, name, dynf_var)),
            scope: self.scope,
        }
    }

    pub const fn new(module: &'a FormatModule, scope: &'a UScope<'a>) -> Self {
        Self {
            module,
            scope,
            dyn_s: DynScope::new(),
        }
    }
}

impl<'a> USingleScope<'a> {
    pub const fn new(parent: &'a UScope<'a>, name: &'a str, uvar: UVar) -> USingleScope<'a> {
        Self { parent, name, uvar }
    }

    fn get_uvar_by_name(&self, name: &str) -> Option<UVar> {
        if self.name == name {
            return Some(self.uvar);
        }
        self.parent.get_uvar_by_name(name)
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
}

impl UType {
    /// Returns an iterator over any embedded UTypes during occurs-checks.
    ///
    /// This method presents a context-agnostic view that merely guarantees that the embedded UTypes of
    /// the receiver are all returned eventually, and in whatever order is most convenient.
    pub fn iter_embedded<'a>(&'a self) -> Box<dyn Iterator<Item = Rc<UType>> + 'a> {
        match self {
            UType::Empty | UType::Hole | UType::Var(..) | UType::Base(..) => {
                Box::new(std::iter::empty())
            }
            UType::Tuple(ts) => Box::new(ts.iter().cloned()),
            UType::Record(fs) => Box::new(fs.iter().map(|(_l, t)| t.clone())),
            UType::Seq(t) | UType::Option(t) => Box::new(std::iter::once(t.clone())),
        }
    }
}

/// Representation of an inferred type that is either fully-known or partly-known
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum VType {
    Base(BaseSet),
    Abstract(Rc<UType>),
    ImplicitTuple(Vec<Rc<UType>>),
    ImplicitRecord(Vec<(Label, Rc<UType>)>),
    IndefiniteUnion(VMId),
}

/// Mutably updated state-engine for performing complete type-inference on a top-level `Format`.
#[derive(Debug)]
pub struct TypeChecker {
    constraints: Vec<Constraints>,
    aliases: Vec<Alias>, // set of non-identity meta-variables that are aliased to ?ix
    varmaps: VarMapMap, // logically separate table of meta-context variant-maps for indirect aliasing
    level_vars: HashMap<usize, UVar>,
}

/// Association type that identifies the relationship between a metavariable and the possibly-empty set
/// of other meta-variables that must agree in order for the tree to be well-typed.
#[derive(Clone, Debug, Default)]
enum Alias {
    #[default]
    Ground, // no aliases anywhere
    BackRef(usize), // direct back-ref to earliest alias (which itself must be canonical)
    Canonical(HashSet<usize>), // list of forward-references to update if usurped by an earlier canonical alias
}

impl Alias {
    /// New, empty alias-set
    pub const fn new() -> Alias {
        Self::Ground
    }

    /// Returns `true` if `self` is the canonical alias of at least one other metavariable (i.e. [`Alias::Canonical`] over a non-empty set).
    pub fn is_canonical_nonempty(&self) -> bool {
        match self {
            Alias::Canonical(x) => !x.is_empty(),
            _ => false,
        }
    }

    /// Returns the index of the canonical back-reference if `self` is [`Alias::BackRef`], or `None` otherwise.
    pub fn as_backref(&self) -> Option<usize> {
        match self {
            Alias::Ground | Alias::Canonical(_) => None,
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
            Alias::Ground => {
                let _ = std::mem::replace(self, Alias::Canonical(HashSet::from([tgt])));
            }
            Alias::BackRef(_) => panic!("cannot add forward-ref to Alias::BackRef"),
            Alias::Canonical(fwd_refs) => {
                fwd_refs.insert(tgt);
            }
        }
    }

    /// Overwrites an Alias to be [`Alias::BackRef`] pointing to the specified index,
    /// returning its old value.
    fn set_backref(&mut self, tgt: usize) -> Alias {
        std::mem::replace(self, Alias::BackRef(tgt))
    }

    /// Returns an iterator over the set of forward-references if `self` is [`Alias::Canonical`], or an empty iterator otherwise.
    fn iter_fwd_refs<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        match self {
            Alias::Ground | Alias::BackRef(_) => Box::new(std::iter::empty()),
            Alias::Canonical(fwd_refs) => Box::new(fwd_refs.iter().copied()),
        }
    }

    /// Returns `true` iff `self` is [`Alias::Canonical`] and contains the specified forward-reference.
    fn contains_fwd_ref(&self, tgt: usize) -> bool {
        match self {
            Alias::Ground | Alias::BackRef(_) => false,
            Alias::Canonical(fwd_refs) => fwd_refs.contains(&tgt),
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
        self.store
            .get(&id.0)
            .unwrap_or_else(|| unreachable!("missing varmap for {id}"))
    }

    pub fn get_varmap_mut(&mut self, id: VMId) -> &mut VarMap {
        self.store
            .get_mut(&id.0)
            .unwrap_or_else(|| unreachable!("missing varmap for {id}"))
    }

    pub fn get_new_id(&mut self) -> VMId {
        let ret = VMId(self.next_id);
        self.next_id += 1;
        ret
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VMId(usize);

#[derive(Clone, Debug, Default)]
pub enum Constraints {
    #[default]
    Indefinite, // default value before union-type distinction is made
    Variant(VMId), // indirect index into typechecker meta-context 'varmap' hashmap
    Invariant(Constraint), // for all type meta-variables, inferred non-variant constraint
}

impl Constraints {
    pub fn new() -> Self {
        Self::Indefinite
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constraint {
    Equiv(Rc<UType>), // direct equivalence with a UType, which should not be a bare `UType::Var` (that is what TypeChecker.equivalences is for)
    Elem(BaseSet), // implicit restriction to a narrowed set of ground-types (e.g. from `Expr::AsU32`)
    Proj(ProjShape), // constraints implied by projections
}

impl Constraint {
    /// Returns true if the constraint is vacuous and therefore equivalent to `Constraints::Indefinite`
    ///
    /// Specifically checks for `Equiv` over `UType::Hole` or `UType::Empty`.
    pub fn is_vacuous(&self) -> bool {
        match self {
            Constraint::Equiv(ut) => matches!(ut.as_ref(), UType::Hole | UType::Empty),
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjShape {
    TupleWith(BTreeMap<usize, UVar>), // required associations of meta-variables at given indices of an uncertain tuple
    RecordWith(BTreeMap<Label, UVar>), // required associations of meta-variables at given fields of an uncertain record
    SeqOf(UVar),                       // simple sequence element-type projection
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BaseSet {
    /// Singleton set of any BaseType, even non-integral ones
    Single(BaseType),
    /// Some subset of U8, U16, U32, U64
    U(UintSet),
}

impl BaseSet {
    #[allow(non_upper_case_globals)]
    pub const UAny: Self = Self::U(UintSet::ANY);

    #[allow(non_upper_case_globals)]
    pub const USome: Self = Self::U(UintSet::ANY32);
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum IntWidth {
    Bits8 = 0,
    Bits16 = 1,
    Bits32 = 2,
    Bits64 = 3,
}

impl IntWidth {
    pub const MAX8: usize = u8::MAX as usize;
    pub const MAX16: usize = u16::MAX as usize;
    pub const MAX32: usize = u32::MAX as usize;
    pub const MAX64: usize = u64::MAX as usize;

    pub fn to_base_type(self) -> BaseType {
        match self {
            IntWidth::Bits8 => BaseType::U8,
            IntWidth::Bits16 => BaseType::U16,
            IntWidth::Bits32 => BaseType::U32,
            IntWidth::Bits64 => BaseType::U64,
        }
    }
}

impl crate::Bounds {
    pub fn min_required_width(&self) -> IntWidth {
        let max = self.max.unwrap_or(self.min);
        match () {
            _ if max <= IntWidth::MAX8 => IntWidth::Bits8,
            _ if max <= IntWidth::MAX16 => IntWidth::Bits16,
            _ if max <= IntWidth::MAX32 => IntWidth::Bits32,
            _ => IntWidth::Bits64,
        }
    }
}

/// Abstraction over rankings of candidate values in a set, where the least-value `Rank` is the default
/// unless it is tied with anything else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Excluded,
    At(u8),
}

impl Rank {
    pub const fn is_excluded(self) -> bool {
        matches!(self, Rank::Excluded)
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Rank::At(n), Rank::At(m)) => {
                // NOTE -  we call reverse this because we want lower numbers to be the maximum rank
                n.cmp(m).reverse()
            }
            (Rank::At(_), Rank::Excluded) => std::cmp::Ordering::Greater,
            (Rank::Excluded, Rank::At(_)) => std::cmp::Ordering::Less,
            (Rank::Excluded, Rank::Excluded) => std::cmp::Ordering::Equal,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct UintSet {
    // Array with ranks for U8, U16, U32, U64 in that order
    ranks: [Rank; 4],
}

impl std::fmt::Debug for UintSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RankedUintSet")
            .field("ranks", &self.ranks)
            .finish()
    }
}

impl std::fmt::Display for UintSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = self.normalize();
        if this.is_empty() {
            return write!(f, "{{}}");
        } else {
            write!(f, "{{ ")?;
            let labels = ["U8", "U16", "U32", "U64"];
            let mut ix_ranks = this
                .ranks
                .into_iter()
                .enumerate()
                .collect::<Vec<(usize, Rank)>>();
            ix_ranks.sort_by(|(_, r1), (_, r2)| r1.cmp(r2).reverse());
            let mut last_rank = None;
            for (ix, r) in ix_ranks {
                if r.is_excluded() {
                    break;
                }
                match last_rank {
                    None => {
                        write!(f, "{}", labels[ix])?;
                    }
                    Some(r0) => {
                        if r < r0 {
                            write!(f, " > {}", labels[ix])?;
                        } else {
                            write!(f, ", {}", labels[ix])?;
                        }
                    }
                }
                last_rank = Some(r);
            }
            write!(f, " }}")
        }
    }
}

impl UintSet {
    pub const ANY_DEFAULT_U32: Self = Self {
        ranks: [Rank::At(1), Rank::At(1), Rank::At(0), Rank::At(1)],
    };
    pub const ANY_DEFAULT_U64: Self = Self {
        ranks: [Rank::At(1), Rank::At(1), Rank::At(1), Rank::At(0)],
    };

    pub fn contains(&self, b: BaseType) -> bool {
        self.ranks[b.int_width() as usize] != Rank::Excluded
    }

    pub fn normalize(self) -> Self {
        let mut ranks = self.ranks;
        for ix in 0..4 {
            if self.ranks[ix] == Rank::Excluded {
                continue;
            }
            let orig_val = self.ranks[ix];
            let count_gte = (self.ranks.iter().filter(|r| **r >= orig_val).count() - 1) as u8;
            ranks[ix] = Rank::At(count_gte);
        }
        Self { ranks }
    }

    pub fn intersection(self, other: Self) -> Self {
        let mut ranks = [Rank::Excluded; 4];
        let this = self.normalize();
        let other = other.normalize();
        for (ix, (r1, r2)) in Iterator::zip(this.ranks.iter(), other.ranks.iter()).enumerate() {
            if matches!(r1, Rank::Excluded) || matches!(r2, Rank::Excluded) {
                continue;
            }
            ranks[ix] = Ord::max(*r1, *r2);
        }
        Self { ranks }.normalize()
    }

    pub fn is_empty(&self) -> bool {
        self.ranks == [Rank::Excluded; 4]
    }

    pub fn get_unique_solution(self) -> Option<BaseType> {
        let this = self.normalize();
        let mut candidate = None;
        let mut max_rank = Rank::Excluded;
        let mut is_unique = true;
        for (ix, r) in this.ranks.into_iter().enumerate() {
            match r {
                Rank::Excluded => continue,
                Rank::At(_n) => match r.cmp(&max_rank) {
                    std::cmp::Ordering::Greater => {
                        max_rank = r;
                        candidate = Some(ix);
                        is_unique = true;
                    }
                    std::cmp::Ordering::Less => continue,
                    std::cmp::Ordering::Equal => {
                        is_unique = false;
                    }
                },
            }
        }
        if let Some(ix) = candidate {
            if is_unique {
                Some(match ix {
                    0 => BaseType::U8,
                    1 => BaseType::U16,
                    2 => BaseType::U32,
                    3 => BaseType::U64,
                    _ => unreachable!(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl UintSet {
    // unrestricted and non-defaulting if more than one solution
    pub const ANY: UintSet = UintSet {
        ranks: [Rank::At(3); 4],
    };
    // U8 or U16, default solution is U8
    pub const SHORT8: UintSet = UintSet {
        ranks: [Rank::At(0), Rank::At(1), Rank::Excluded, Rank::Excluded],
    };

    // Some member of the restricted set of any whose width is no less than the given IntWidth
    pub const fn at_least(val: IntWidth) -> Self {
        let ranks = match val {
            IntWidth::Bits8 => [Rank::At(3), Rank::At(3), Rank::At(3), Rank::At(3)],
            IntWidth::Bits16 => [Rank::Excluded, Rank::At(2), Rank::At(2), Rank::At(2)],
            IntWidth::Bits32 => [Rank::Excluded, Rank::Excluded, Rank::At(1), Rank::At(1)],
            IntWidth::Bits64 => [Rank::Excluded, Rank::Excluded, Rank::Excluded, Rank::At(0)],
        };
        UintSet { ranks }
    }

    // unrestricted but resolves if ambiguous to the given IntWidth, unless precluded
    pub const fn any_default(val: IntWidth) -> Self {
        let ranks = match val {
            IntWidth::Bits8 => [Rank::At(0), Rank::At(3), Rank::At(3), Rank::At(3)],
            IntWidth::Bits16 => [Rank::At(3), Rank::At(0), Rank::At(3), Rank::At(3)],
            IntWidth::Bits32 => [Rank::At(3), Rank::At(3), Rank::At(0), Rank::At(3)],
            IntWidth::Bits64 => [Rank::At(3), Rank::At(3), Rank::At(3), Rank::At(0)],
        };
        UintSet { ranks }
    }
}

impl UintSet {
    pub const ANY32: Self = Self::any_default(IntWidth::Bits32);
}

impl BaseType {
    pub fn int_width(&self) -> IntWidth {
        match self {
            BaseType::U8 => IntWidth::Bits8,
            BaseType::U16 => IntWidth::Bits16,
            BaseType::U32 => IntWidth::Bits32,
            BaseType::U64 => IntWidth::Bits64,
            _ => unreachable!("cannot measure int-width of non-integral BaseType {self:?}"),
        }
    }
}

impl UintSet {
    // pub fn intersection(self, other: Self) -> Self {
    //     match (self, other) {
    //         (x, y) if x == y => x,
    //         (UintSet::ANY, x) | (x, UintSet::ANY) => x, // Any is the identity under intersection
    //         (UintSet::SHORT8, _) | (_, UintSet::SHORT8) => Self::SHORT8,
    //         (UintSet::at_least(w1), UintSet::at_least(w2)) => Self::at_least(Ord::max(w1, w2)),
    //         // Technically ambiguous cases
    //         (UintSet::any_default(w1), UintSet::any_default(w2)) => Self::any_default(Ord::max(w1, w2)),
    //         (UintSet::any_default(w_dft), UintSet::at_least(w_min)) | (UintSet::at_least(w_min), UintSet::any_default(w_dft)) => {
    //             panic!("unresolvable UintSet intersection: AnyDefault({w_dft:?}) & AtLeast({w_min:?})")
    //         }
    //     }
    // }

    // pub fn get_unique_solution(self) -> Option<BaseType> {
    //     match self {
    //         UintSet::Any => None,
    //         UintSet::AnyDefault(width) => Some(width.to_base_type()),
    //         UintSet::Short8 => Some(BaseType::U8),
    //         UintSet::AtLeast(IntWidth::Bits64) => Some(BaseType::U64),
    //         UintSet::AtLeast(_) => None,
    //     }
    // }
}

impl BaseSet {
    pub fn unify(&self, other: &Self) -> Result<Self, ConstraintError> {
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
            (BaseSet::U(u), BaseSet::Single(b)) | (BaseSet::Single(b), BaseSet::U(u)) => {
                if u.contains(*b) {
                    Ok(BaseSet::Single(*b))
                } else {
                    Err(UnificationError::Unsatisfiable(
                        self.to_constraint(),
                        other.to_constraint(),
                    ))
                }
            }
            (BaseSet::U(u1), BaseSet::U(u2)) => Ok(BaseSet::U(u1.intersection(*u2))),
        }
    }

    /// Constructs the simplest-possible constraint from `self`, in particular substituting
    /// `Equiv(BaseType(b))` in place of `Elem(Single(b))`.
    pub fn to_constraint(self) -> Constraint {
        match self {
            BaseSet::Single(b) => Constraint::Equiv(Rc::new(UType::Base(b))),
            _ => Constraint::Elem(self),
        }
    }

    fn get_unique_solution(&self, v: UVar) -> TCResult<Rc<UType>> {
        match self {
            BaseSet::Single(b) => Ok(Rc::new(UType::Base(*b))),
            BaseSet::U(u) => {
                if u.is_empty() {
                    return Err(TCErrorKind::NoSolution(v).into());
                }
                match u.get_unique_solution() {
                    Some(b) => Ok(Rc::new(UType::Base(b))),
                    None => Err(TCErrorKind::MultipleSolutions(v, *self).into()),
                }
            }
        }
    }
}

impl std::fmt::Display for BaseSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseSet::Single(t) => write!(f, "{{ {t:?} }}"),
            BaseSet::U(ranked_set) => ranked_set.fmt(f),
        }
    }
}

type VarMap = HashMap<Label, Rc<UType>>;

// SECTION - Construction and instantiation in the meta-context
impl TypeChecker {
    /// Constructs a typechecker with initially 0 meta-variables.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            aliases: Vec::new(),
            varmaps: VarMapMap::new(),
            level_vars: HashMap::new(),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Returns the number of individual `UVar`s in the meta-context of `self`.
    ///
    /// May panic in debug builds if, for any reason, the internal state has diverged
    /// (i.e. different numbers of aliases and constraints).
    pub fn size(&self) -> usize {
        if cfg!(debug_assertions) {
            self.check_uvar_sanity();
        }
        self.constraints.len()
    }

    /// Instantiates a new [`VarMap`] object in the typechecker meta-context and returns an identifier pointing to it.
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
        // FIXME - not sure whether to return rc or newvar
        Ok((newvar, rc))
    }

    /// Instantiates a new UVar with no known constraints or aliases, returning it by-value
    fn get_new_uvar(&mut self) -> UVar {
        let ret = UVar(self.constraints.len());
        self.constraints.push(Constraints::new());
        self.aliases.push(Alias::new());
        ret
    }

    fn infer_var_scope_pattern(
        &mut self,
        pat: &Pattern,
        scope: &mut UMultiScope<'_>,
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
            Pattern::U64(_) => {
                let var = self.init_var_simple(UType::Base(BaseType::U64))?.0;
                Ok(var)
            }
            Pattern::Int(bounds) => {
                let var = self.get_new_uvar();
                let width = bounds.min_required_width();
                self.unify_utype_baseset(var.into(), BaseSet::U(UintSet::at_least(width)))?;
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
                let newvar = self.get_new_uvar();
                let inner_var = self.infer_var_scope_pattern(inner.as_ref(), scope)?;
                self.add_uvar_variant(newvar, vname.clone(), Rc::new(UType::Var(inner_var)))?;
                Ok(newvar)
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
                    Rc::new(UType::Seq(Rc::new(UType::Var(elem_uvar)))),
                )?;
                Ok(seq_uvar)
            }
            Pattern::Option(opt) => {
                let outer_var = self.get_new_uvar();
                let inner_var = if let Some(inner) = opt.as_ref() {
                    self.infer_var_scope_pattern(inner, scope)?
                } else {
                    self.get_new_uvar()
                };
                self.unify_var_utype(
                    outer_var,
                    Rc::new(UType::Option(Rc::new(UType::Var(inner_var)))),
                )?;
                Ok(outer_var)
            }
        }
    }

    fn unify_utype_format_match_case(
        &mut self,
        head_t: Rc<UType>,
        pat: &Pattern,
        rhs_var: UVar,
        rhs_format: &Format,
        ctxt: Ctxt<'_>,
    ) -> TCResult<()> {
        let mut child = UMultiScope::new(ctxt.scope);
        let pvar = self.infer_var_scope_pattern(pat, &mut child)?;
        let tmp = child.clone();
        let new_scope = UScope::Multi(&tmp);
        let new_ctxt = ctxt.with_scope(&new_scope);
        let local_rhs_var = self.infer_var_format(rhs_format, new_ctxt)?;
        self.unify_var_utype(pvar, head_t)?;
        self.unify_var_pair(rhs_var, local_rhs_var)?;
        Ok(())
    }

    fn unify_utype_expr_match_case<'a>(
        &mut self,
        head_t: Rc<UType>,
        pat: &Pattern,
        rhs_var: UVar,
        rhs_expr: &Expr,
        scope: &'a UScope<'a>,
    ) -> TCResult<()> {
        let mut child = UMultiScope::new(scope);
        let pvar = self.infer_var_scope_pattern(pat, &mut child)?;
        let tmp = child.clone();
        let new_scope = UScope::Multi(&tmp);
        let local_rhs_var = self.infer_var_expr(rhs_expr, &new_scope)?;
        self.unify_var_utype(pvar, head_t)?;
        self.unify_var_pair(rhs_var, local_rhs_var)?;
        Ok(())
    }

    /// Converts a `UType` to Weak Head-Normal Form `VType` by unchaining as many meta-variables as required, after performing
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

    fn unify_var_valuetype_union<'a>(
        &mut self,
        var: UVar,
        branches: impl IntoIterator<Item = (&'a Label, &'a ValueType)> + 'a,
    ) -> TCResult<()> {
        for (lbl, branch_vt) in branches.into_iter() {
            let ut = if let Some(ut) = UType::from_valuetype(branch_vt) {
                Rc::new(ut)
            } else {
                let branch_var = self.get_new_uvar();
                self.unify_var_valuetype(branch_var, branch_vt)?;
                branch_var.into()
            };
            self.add_uvar_variant(var, lbl.clone(), ut)?;
        }
        Ok(())
    }

    fn infer_var_format_level(&mut self, level: usize, ctxt: Ctxt<'_>) -> TCResult<UVar> {
        if let Some(ret) = self.level_vars.get(&level) {
            Ok(*ret)
        } else {
            let ret = self.infer_var_format(ctxt.module.get_format(level), ctxt)?;
            self.level_vars.insert(level, ret);
            Ok(ret)
        }
    }

    fn infer_var_dyn_format(&mut self, dynf: &DynFormat, ctxt: Ctxt<'_>) -> TCResult<UVar> {
        match dynf {
            DynFormat::Huffman(code_lengths, opt_values_expr) => {
                let newvar = self.get_new_uvar();
                let codes_var = self.infer_var_expr(code_lengths, ctxt.scope)?;
                let code_var = self.get_new_uvar();

                // unify on expected type of Seq<u8> | Seq<u16>
                self.unify_var_proj_elem(codes_var, code_var)?;
                self.unify_var_constraint(code_var, Constraint::Elem(BaseSet::U(UintSet::SHORT8)))?;

                if let Some(values_expr) = opt_values_expr {
                    let values_var = self.infer_var_expr(values_expr, ctxt.scope)?;
                    let value_var = self.get_new_uvar();

                    self.unify_var_proj_elem(values_var, value_var)?;
                    self.unify_var_constraint(
                        value_var,
                        Constraint::Elem(BaseSet::U(UintSet::SHORT8)),
                    )?;
                }

                self.unify_var_constraint(
                    newvar,
                    Constraint::Elem(BaseSet::U(UintSet::any_default(IntWidth::Bits16))),
                )?;
                Ok(newvar)
            }
        }
    }
}
// !SECTION

// SECTION - checks and maintenance of invariants of the meta-context
impl TypeChecker {
    /// Performs a runtime assertion that the number of known UVars is agreed upon by all fields that track
    /// their expected properties.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn check_uvar_sanity(&self) {
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
            Constraints::Invariant(c) => match c {
                Constraint::Elem(_) => Ok(()),
                Constraint::Equiv(t) => self.occurs_in(v, t),
                Constraint::Proj(p) => match p {
                    ProjShape::TupleWith(ix_vars) => {
                        for (_ix, var) in ix_vars.iter() {
                            self.occurs_in(v, Rc::<UType>::from(*var))?;
                        }
                        Ok(())
                    }
                    ProjShape::RecordWith(fld_vars) => {
                        for (_lbl, var) in fld_vars.iter() {
                            self.occurs_in(v, Rc::<UType>::from(*var))?;
                        }
                        Ok(())
                    }
                    ProjShape::SeqOf(elem_v) => self.occurs_in(v, Rc::<UType>::from(*elem_v)),
                },
            },
        }
    }

    fn occurs_in(&self, v: UVar, t: impl AsRef<UType>) -> TCResult<()> {
        match t.as_ref() {
            UType::Hole | UType::Empty | UType::Base(_) => Ok(()),
            &UType::Var(v1) => {
                if self.is_aliased(v, v1) {
                    Err(TCErrorKind::InfiniteType(v, self.constraints[v.0].clone()).into())
                } else {
                    let c_ix = self.aliases[v1.0].as_backref().unwrap_or(v1.0);
                    self.occurs_in_constraints(v, &self.constraints[c_ix])
                }
            }
            UType::Tuple(ts) => {
                for t in ts.iter() {
                    self.occurs_in(v, t.clone())?;
                }
                Ok(())
            }
            UType::Record(fs) => {
                for (_lbl, t) in fs.iter() {
                    self.occurs_in(v, t.clone())?;
                }
                Ok(())
            }
            UType::Seq(inner) | UType::Option(inner) => {
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
        let constraints = &self.constraints[cv.0];
        match constraints {
            Constraints::Indefinite => {
                let id = self.init_varmap();
                let vm = self.varmaps.as_inner_mut().entry(id.0).or_default();
                vm.insert(cname, inner);
                self.set_uvar_vmid(cv, id)?;
                // update all forward-references to point to the same varmap
                let fwd_refs = self.aliases[cv.0].iter_fwd_refs().collect::<Vec<usize>>();
                for ix in fwd_refs.into_iter() {
                    self.set_uvar_vmid(UVar(ix), id)?;
                }
                Ok(())
            }
            Constraints::Variant(vmid) => {
                let id = *vmid;
                let vm = self.varmaps.get_varmap(id);
                if let Some(prior) = vm.get(&cname) {
                    let updated = self.unify_utype(prior.clone(), inner)?;
                    if updated.as_ref() != self.varmaps.get_varmap(id).get(&cname).unwrap().as_ref()
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
        let constraints = &mut self.constraints[uvar.0];
        match constraints {
            Constraints::Variant(other) => {
                let old = *other;

                // we only care about old if it is still an extant varmap
                if let Some(old_vm) = self.varmaps.as_inner().get(&old.0) {
                    let Some(new_vm) = self.varmaps.as_inner().get(&vmid.0) else {
                        unreachable!("HashMap::get returned None for {vmid} even though assertion on HashMap::contains_key succeeded")
                    };
                    for key in old_vm.keys() {
                        // NOTE: this check may be costly so we are gating it for non-release builds
                        // NOTE: this isn't necessarily enough to validate subset-equivalence of the old and new VarMaps, as we do not check that the values associated with the common keys can unify
                        debug_assert!(
                            new_vm.contains_key(key),
                            "previous varmap {other} of {uvar} has variant {key} but new varmap {vmid} does not"
                        );
                    }
                }

                // REVIEW - does the value of `old` matter here, or do we merely discard it?
                *other = vmid;
                Ok(())
            }
            Constraints::Invariant(orig) => Err(TCErrorKind::VarianceMismatch(
                uvar,
                vmid,
                self.varmaps.get_varmap(vmid).clone(),
                orig.clone(),
                Polarity::PriorVariant,
            )
            .into()),
            Constraints::Indefinite => {
                *constraints = Constraints::Variant(vmid);
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
            Constraints::Variant(vmid) => Err(TCErrorKind::VarianceMismatch(
                uvar,
                *vmid,
                self.varmaps.get_varmap(*vmid).clone(),
                constraint,
                Polarity::PriorInvariant,
            )
            .into()),
            Constraints::Invariant(prior) => {
                let c1 = prior.clone();
                if c1 == constraint {
                    return Ok(c1);
                }
                let _tmp = (
                    "unify_var_constraint@Invariant",
                    format!("{uvar} {prior}"),
                    format!("{uvar} {constraint}"),
                );
                let ret = self.unify_constraint_pair(c1, constraint)?;
                self.constraints[can_ix] = Constraints::Invariant(ret.clone());
                Ok(ret)
            }
        }
    }

    /// Unifies according to `?i ~ Tuple(..., ?j, ...?)`, with `tuple_var` on the lhs and `ix_var` on the rhs
    /// of the metavariable equation, with `ix_var` occurring at index `ix` of the tuple being projected.
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
            Constraints::Invariant(c) => match c {
                Constraint::Elem(_) => unreachable!("cannot solve tuple projection on base-set"),
                Constraint::Equiv(ut) => match ut.as_ref() {
                    UType::Tuple(ts) => {
                        assert!(ts.len() > ix);
                        let type_at_ix = ts[ix].clone();
                        self.unify_var_utype(ix_var, type_at_ix)?;
                        Ok(())
                    }
                    other => unreachable!("expected UType::Tuple, found {other:?}"),
                },
                Constraint::Proj(ps) => match ps {
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
                },
            },
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
            Constraints::Invariant(c) => match c {
                Constraint::Elem(_) => unreachable!("cannot solve record projection on base-set"),
                Constraint::Equiv(ut) => match ut.as_ref() {
                    UType::Record(fs) => {
                        let fld_type = fs
                            .iter()
                            .find(|(name, _)| name == &fname)
                            .unwrap()
                            .1
                            .clone();
                        self.unify_var_utype(fld_var, fld_type)?;
                        Ok(())
                    }
                    UType::Var(v_other) => {
                        let v = *v_other;
                        self.unify_var_proj_field(v, fname, fld_var)
                    }
                    other => unreachable!("expected UType::Record, found {other:?}"),
                },
                Constraint::Proj(ps) => match ps {
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
                },
            },
        }
    }

    /// Establishes `?i ~ Seq(?j)` through projective unification, where `seq_v` is the lhs
    /// and `elem_v` is the rhs metavariable.
    fn unify_var_proj_elem(&mut self, seq_v: UVar, elem_v: UVar) -> TCResult<()> {
        let can_v = self.get_canonical_uvar(seq_v);
        match &mut self.constraints[can_v.0] {
            Constraints::Indefinite => {
                let proj = ProjShape::seq_of(elem_v);
                self.unify_var_constraint(seq_v, Constraint::Proj(proj))?;
                Ok(())
            }
            Constraints::Variant(_) => unreachable!("cannot solve record projection on union"),
            Constraints::Invariant(c) => match c {
                Constraint::Elem(_) => unreachable!("cannot solve record projection on base-set"),
                Constraint::Equiv(ut) => match ut.as_ref() {
                    UType::Seq(inner) => {
                        let elem_t = inner.clone();
                        self.unify_var_utype(elem_v, elem_t)?;
                        Ok(())
                    }
                    other => unreachable!("expected UType::Seq, found {other:?}"),
                },
                Constraint::Proj(ps) => match ps {
                    ProjShape::SeqOf(other_var) => {
                        let tmp = *other_var;
                        self.unify_var_pair(elem_v, tmp)?;
                        Ok(())
                    }
                    _ => unreachable!("cannot unify on element type of non-seq projection"),
                },
            },
        }
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
            ProjShape::SeqOf(v) => VType::Abstract(Rc::new(UType::Seq((*v).into()))),
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

    fn unify_utype(&mut self, left: Rc<UType>, right: Rc<UType>) -> TCResult<Rc<UType>> {
        match (left.as_ref(), right.as_ref()) {
            (UType::Hole, UType::Hole) => {
                // FIXME - determine whether this is actually a proper case to see in practice
                unreachable!("Unexpected hole-hole unification may indicate bad logic path");
                // Ok(left)
            }
            (UType::Hole, _) => Ok(right),
            (_, UType::Hole) => Ok(left),
            (UType::Empty, _) => Ok(right),
            (_, UType::Empty) => Ok(left),
            (UType::Seq(e1), UType::Seq(e2)) => {
                if e1 == e2 {
                    Ok(left)
                } else {
                    let inner = self.unify_utype(e1.clone(), e2.clone())?;
                    Ok(Rc::new(UType::Seq(inner)))
                }
            }
            (UType::Option(o1), UType::Option(o2)) => {
                if o1 == o2 {
                    Ok(left)
                } else {
                    let inner = self.unify_utype(o1.clone(), o2.clone())?;
                    Ok(Rc::new(UType::Option(inner)))
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
                    Constraint::Elem(_) | Constraint::Proj(_) => {
                        unreachable!("equiv should erase proj and elem")
                    }
                }
            }
            (_, &UType::Var(v)) => {
                let constraint = Constraint::Equiv(left.clone());
                let after = self.unify_var_constraint(v, constraint)?;
                self.occurs(v)?;
                match after {
                    Constraint::Equiv(t) => Ok(t.clone()),
                    Constraint::Elem(_) | Constraint::Proj(_) => {
                        unreachable!("equiv should erase proj and elem")
                    }
                }
            }
            // all the remaining cases are mismatched UType constructors
            _ => Err(UnificationError::Unsatisfiable(left, right).into()),
        }
    }

    /// Assigns a 'solution' (destructively-updated invariant constraint) to a UVar
    fn unify_var_utype(&mut self, v1: UVar, solution: Rc<UType>) -> TCResult<()> {
        match solution.as_ref() {
            UType::Var(v2) => {
                self.unify_var_pair(v1, *v2)?;
                Ok(())
            }
            _ => {
                self.unify_var_constraint(v1, Constraint::Equiv(solution.clone()))?;
                Ok(())
            }
        }
    }

    /// Unifies a UType against a BaseSet, updating any variable constraints in the process.
    ///
    /// Returns a copy of the novel Constraint implied by the unification
    /// if `ut` is `Base`, `Var`, or `Hole` (in the case of `Hole`, no additional inference is performed
    /// and the constraint is directly returned without any further unification).
    ///
    /// Otherwise, returns an `Err` indicating that the unification was not possible.
    fn unify_utype_baseset(&mut self, ut: Rc<UType>, bs: BaseSet) -> TCResult<Constraint> {
        match ut.as_ref() {
            UType::Var(uv) => self.unify_var_baseset(*uv, bs),
            UType::Base(b) => {
                let ret = bs.unify(&BaseSet::Single(*b))?.to_constraint();
                Ok(ret)
            }
            UType::Hole => Ok(bs.to_constraint()),
            _other => Err(UnificationError::Unsatisfiable(
                Constraint::Equiv(ut),
                bs.to_constraint(),
            )
            .into()),
        }
    }

    /// Unifies a UVar against a BaseSet, updating any aliased-variable constraints in the process.
    ///
    /// Returns a copy of the novel Constraint implied by the unification if it was sound.
    /// Otherwise, returns an `Err` indicating that the unification was not possible.
    fn unify_var_baseset(&mut self, uv: UVar, bs: BaseSet) -> TCResult<Constraint> {
        let constraint = bs.to_constraint();
        self.unify_var_constraint(uv, constraint)
    }

    /// Attempt to unify a [`UVar`] with a [`ValueType`], primarily for use with `Expr::FlatMapAccum`.
    fn unify_var_valuetype(&mut self, uv: UVar, vt: &ValueType) -> TCResult<()> {
        match UType::from_valuetype(vt) {
            Some(ref ut) => self.unify_var_utype(uv, Rc::new(ut.clone()))?,
            _ => match vt {
                ValueType::Union(branches) => {
                    self.unify_var_valuetype_union(uv, branches)?;
                }
                ValueType::Record(fs) => {
                    for (lbl, fvt) in fs.iter() {
                        let fld_var = self.get_new_uvar();
                        self.unify_var_proj_field(uv, lbl.clone(), fld_var)?;
                        self.unify_var_valuetype(fld_var, fvt)?;
                    }
                }
                ValueType::Tuple(ts) => {
                    for (ix, t) in ts.iter().enumerate() {
                        let ix_var = self.get_new_uvar();
                        self.unify_var_proj_index(uv, ix, ix_var)?;
                        self.unify_var_valuetype(ix_var, t)?;
                    }
                }
                ValueType::Seq(inner) => {
                    let elem_v = self.get_new_uvar();
                    self.unify_var_proj_elem(uv, elem_v)?;
                    self.unify_var_valuetype(elem_v, inner)?;
                }
                other => unreachable!("unify_var_utype failed on non-nested ValueType {other:?}"),
            },
        }
        Ok(())
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
            (Constraint::Equiv(ut), Constraint::Elem(bs))
            | (Constraint::Elem(bs), Constraint::Equiv(ut)) => {
                Ok(self.unify_utype_baseset(ut.clone(), bs)?)
            }
            (Constraint::Elem(bs1), Constraint::Elem(bs2)) => {
                let bs0 = bs1.unify(&bs2)?;
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
                    match (t1.get(key), t2.get(key)) {
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
            (
                Constraint::Proj(ProjShape::SeqOf(elt_var1)),
                Constraint::Proj(ProjShape::SeqOf(elt_var2)),
            ) => {
                self.unify_var_pair(elt_var1, elt_var2)?;
                Ok(Constraint::Proj(ProjShape::SeqOf(elt_var1)))
            }
            (ref c1 @ Constraint::Proj(ref p), ref c2 @ Constraint::Equiv(ref ut))
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
                            Err(UnificationError::Unsatisfiable(c1.clone(), c2.clone()).into())
                        }
                    }
                    (ProjShape::SeqOf(elem_v), UType::Seq(elem_t)) => {
                        self.unify_var_utype(*elem_v, elem_t.clone())?;
                        Ok(Constraint::Equiv(ut.clone()))
                    }
                    (proj, UType::Var(var)) => {
                        self.unify_var_constraint(*var, Constraint::Proj(proj.clone()))
                    }
                    (ProjShape::RecordWith(flds), other) => {
                        unreachable!("could not match Record-Shape {flds:?} against {other:?}")
                    }
                    (ProjShape::TupleWith(elts), other) => {
                        unreachable!("could not match Tuple-Shape {elts:?} against {other:?}")
                    }
                    (ProjShape::SeqOf(_), other) => {
                        unreachable!("could not match Seq-Shape against {other:?}")
                    }
                }
            }
            (
                ref c1 @ Constraint::Proj(ProjShape::RecordWith(..)),
                ref c2 @ Constraint::Proj(ProjShape::SeqOf(..) | ProjShape::TupleWith(..)),
            )
            | (
                ref c1 @ Constraint::Proj(ProjShape::TupleWith(..)),
                ref c2 @ Constraint::Proj(ProjShape::SeqOf(..) | ProjShape::RecordWith(..)),
            )
            | (
                ref c1 @ Constraint::Proj(ProjShape::SeqOf(..)),
                ref c2 @ Constraint::Proj(ProjShape::TupleWith(..) | ProjShape::RecordWith(..)),
            )
            | (ref c1 @ Constraint::Elem(_), ref c2 @ Constraint::Proj(_))
            | (ref c1 @ Constraint::Proj(_), ref c2 @ Constraint::Elem(_)) => {
                Err(UnificationError::Unsatisfiable(c1.clone(), c2.clone()).into())
            }
        }
    }

    /// Takes two pairs UVar and VMId, which are implicitly equated within each pair (i.e. `v1 ~ vmid1` and `v2 ~ vmid2`)
    /// and performs VarMap unification, erasing the unused map after all updates are made, and returning the new VMId
    /// of the remaining VarMap that both variables should now be equated with.
    fn unify_varmaps(&mut self, v1: UVar, vmid1: VMId, v2: UVar, vmid2: VMId) -> TCResult<VMId> {
        if vmid1 == vmid2 {
            return Ok(vmid1);
        }

        let (lo, hi) = if vmid1 < vmid2 {
            (vmid1, vmid2)
        } else {
            (vmid2, vmid1)
        };

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
}
// !SECTION

// SECTION - mid-to-high-level model-type inference rules
impl TypeChecker {
    fn infer_var_expr_acc(
        &mut self,
        e: &Expr,
        vt: &ValueType,
        scope: &UScope<'_>,
    ) -> TCResult<UVar> {
        let uv = self.infer_var_expr(e, scope)?;
        self.unify_var_valuetype(uv, vt)?;
        Ok(uv)
    }

    fn infer_var_expr<'a>(&mut self, e: &Expr, scope: &'a UScope<'a>) -> TCResult<UVar> {
        Ok(match e {
            Expr::Var(lbl) => {
                let occ_var = self.get_new_uvar();
                match scope.get_uvar_by_name(lbl) {
                    Some(uv) => {
                        self.unify_var_pair(uv, occ_var)?;
                    }
                    None => unreachable!("variable {lbl} not in scope"),
                }
                occ_var
            }
            Expr::Bool(_) => self.init_var_simple(UType::Base(BaseType::Bool))?.0,
            Expr::U8(_) => self.init_var_simple(UType::Base(BaseType::U8))?.0,
            Expr::U16(_) => self.init_var_simple(UType::Base(BaseType::U16))?.0,
            Expr::U32(_) => self.init_var_simple(UType::Base(BaseType::U32))?.0,
            Expr::U64(_) => self.init_var_simple(UType::Base(BaseType::U64))?.0,
            Expr::Tuple(ts) => {
                let newvar = self.get_new_uvar();
                let mut uts = Vec::with_capacity(ts.len());
                for t in ts {
                    uts.push(self.infer_utype_expr(t, scope)?);
                }
                self.unify_var_utype(newvar, Rc::new(UType::Tuple(uts)))?;
                newvar
            }
            Expr::TupleProj(e_tup, ix) => {
                let newvar = self.get_new_uvar();
                let v_tup = self.infer_var_expr(e_tup, scope)?;
                self.unify_var_proj_index(v_tup, *ix, newvar)?;
                newvar
            }
            Expr::Record(fs) => {
                let newvar = self.get_new_uvar();
                let mut fields = Vec::with_capacity(fs.len());
                let mut child = UMultiScope::with_capacity(scope, fs.len());
                for (lbl, f) in fs {
                    let scope = UScope::Multi(&child);
                    let f_var = self.infer_var_expr(f, &scope)?;
                    child.push(lbl.clone(), f_var);
                    fields.push((lbl.clone(), f_var.into()));
                }
                self.unify_var_utype(newvar, Rc::new(UType::Record(fields)))?;
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
                    Rc::new(UType::Seq(Rc::new(UType::Var(elem_uvar)))),
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
            Expr::Arith(Arith::BoolAnd | Arith::BoolOr, x, y) => {
                let zvar = self.get_new_uvar();

                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::Single(BaseType::Bool))?;

                let yvar = self.infer_var_expr(y.as_ref(), scope)?;
                let _cy = self.unify_var_baseset(yvar, BaseSet::Single(BaseType::Bool))?;

                self.unify_var_constraint(zvar, BaseSet::Single(BaseType::Bool).to_constraint())?;
                zvar
            }
            Expr::Unary(UnaryOp::BoolNot, x) => {
                let newvar = self.get_new_uvar();
                let xvar = self.infer_var_expr(x.as_ref(), scope)?;

                self.unify_var_constraint(xvar, BaseSet::Single(BaseType::Bool).to_constraint())?;
                self.unify_var_constraint(newvar, BaseSet::Single(BaseType::Bool).to_constraint())?;
                newvar
            }
            Expr::Unary(UnaryOp::IntSucc | UnaryOp::IntPred, x) => {
                let newvar = self.get_new_uvar();

                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;

                self.unify_var_pair(newvar, xvar)?;
                newvar
            }
            Expr::Arith(_, x, y) => {
                let zvar = self.get_new_uvar();

                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;

                let yvar = self.infer_var_expr(y.as_ref(), scope)?;
                let _cy = self.unify_var_baseset(yvar, BaseSet::UAny)?;

                self.unify_var_pair(zvar, xvar)?;
                self.unify_var_pair(zvar, yvar)?;
                zvar
            }
            Expr::IntRel(_rel, x, y) => {
                let zvar = self.get_new_uvar();

                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;

                let yvar = self.infer_var_expr(y.as_ref(), scope)?;
                let _cy = self.unify_var_baseset(yvar, BaseSet::UAny)?;

                let _cxy = self.unify_var_pair(xvar, yvar)?;

                let cz = Constraint::Elem(BaseSet::Single(BaseType::Bool));
                self.unify_var_constraint(zvar, cz)?;

                zvar
            }
            Expr::AsU8(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U8))?.0;
                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;
                newvar
            }
            Expr::AsU16(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U16))?.0;
                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;
                newvar
            }
            Expr::AsU32(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U32))?.0;
                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;
                newvar
            }
            Expr::AsU64(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U64))?.0;
                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;
                newvar
            }
            Expr::AsChar(x) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::Char))?.0;
                let xvar = self.infer_var_expr(x.as_ref(), scope)?;
                let _cx = self.unify_var_baseset(xvar, BaseSet::UAny)?;
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
            Expr::U64Be(bytes) | Expr::U64Le(bytes) => {
                let newvar = self.init_var_simple(UType::Base(BaseType::U64))?.0;
                let ut = self.infer_utype_expr(bytes.as_ref(), scope)?;
                self.unify_utype(ut, Rc::new(UType::tuple([BaseType::U8; 8])))?;
                newvar
            }
            Expr::SeqLength(seq_expr) => {
                let newvar = self.get_new_uvar();
                // NOTE - we can't use `UintSet::any_default(Bits32)` because it causes a multiple-solution error when unified against Format::Pos.
                self.unify_var_baseset(newvar, BaseSet::Single(BaseType::U32))?;
                let seq_var = self.infer_var_expr(seq_expr.as_ref(), scope)?;
                let elem_var = self.get_new_uvar();
                self.unify_var_proj_elem(seq_var, elem_var)?;
                newvar
            }
            Expr::SeqIx(seq_expr, index_expr) => {
                let newvar = self.get_new_uvar();
                let seq_var = self.infer_var_expr(seq_expr.as_ref(), scope)?;

                let index_t = self.infer_utype_expr(index_expr.as_ref(), scope)?;
                self.unify_utype_baseset(index_t, BaseSet::USome)?;

                // directly project newvar as the element-type of seq_var
                self.unify_var_proj_elem(seq_var, newvar)?;

                newvar
            }
            Expr::SubSeq(seq_expr, start_expr, len_expr) => {
                let newvar = self.get_new_uvar();
                let seq_var = self.infer_var_expr(seq_expr.as_ref(), scope)?;

                let start_t = self.infer_utype_expr(start_expr.as_ref(), scope)?;
                let len_t = self.infer_utype_expr(len_expr.as_ref(), scope)?;

                self.unify_utype_baseset(start_t, BaseSet::USome)?;
                self.unify_utype_baseset(len_t, BaseSet::USome)?;

                // ensure that seq_t is a sequence type, and then equate seq_t to newvar
                let elem_var = self.get_new_uvar();
                self.unify_var_proj_elem(seq_var, elem_var)?;
                self.unify_var_pair(newvar, seq_var)?;

                newvar
            }
            Expr::SubSeqInflate(seq_expr, start_expr, len_expr) => {
                let newvar = self.get_new_uvar();
                let seq_var = self.infer_var_expr(seq_expr.as_ref(), scope)?;

                let start_t = self.infer_utype_expr(start_expr.as_ref(), scope)?;
                let len_t = self.infer_utype_expr(len_expr.as_ref(), scope)?;

                self.unify_utype_baseset(start_t, BaseSet::USome)?;
                self.unify_utype_baseset(len_t, BaseSet::USome)?;

                // ensure that seq_t is a sequence type, and then equate it to newvar
                let elem_var = self.get_new_uvar();
                self.unify_var_proj_elem(seq_var, elem_var)?;
                self.unify_var_pair(newvar, seq_var)?;

                newvar
            }
            Expr::FlatMap(f_expr, seq_expr) => {
                let newvar = self.get_new_uvar();

                let (in_v, out_v) = self.infer_vars_expr_lambda(f_expr.as_ref(), scope)?;
                let seq_v = self.infer_var_expr(seq_expr.as_ref(), scope)?;

                let out_elem_v = self.get_new_uvar();

                self.unify_var_proj_elem(seq_v, in_v)?;
                self.unify_var_proj_elem(out_v, out_elem_v)?;
                self.unify_var_pair(newvar, out_v)?;

                newvar
            }
            Expr::FlatMapAccum(f_expr, acc_expr, acc_vt, seq_expr) => {
                // NOTE - ((acc, x) -> (acc, [y])) -> acc -> Vt(acc) -> [x] -> [y]
                let ys_var = self.get_new_uvar();

                let (acc_x_var, acc_ys_var) = self.infer_vars_expr_lambda(f_expr, scope)?;
                let acc_var = self.infer_var_expr_acc(acc_expr, acc_vt.as_ref(), scope)?;
                let xs_var = self.infer_var_expr(seq_expr, scope)?;
                let x_var = self.get_new_uvar();
                let y_var = self.get_new_uvar();

                self.unify_var_proj_elem(ys_var, y_var)?;
                self.unify_var_proj_elem(xs_var, x_var)?;

                // constrain the shape to be exactly the tuple we expect
                self.unify_var_utype(
                    acc_x_var,
                    Rc::new(UType::Tuple(vec![acc_var.into(), x_var.into()])),
                )?;
                self.unify_var_utype(
                    acc_ys_var,
                    Rc::new(UType::Tuple(vec![acc_var.into(), ys_var.into()])),
                )?;

                ys_var
            }
            Expr::LeftFold(f_expr, acc_expr, acc_vt, seq_expr) => {
                // NOTE - ((acc, x) -> acc) -> acc -> Vt(acc) -> [x] -> acc
                let newvar = self.get_new_uvar();

                let (acc_x_var, ret_var) = self.infer_vars_expr_lambda(f_expr, scope)?;
                let acc_var = self.infer_var_expr_acc(acc_expr, acc_vt.as_ref(), scope)?;
                let xs_var = self.infer_var_expr(seq_expr, scope)?;
                let x_var = self.get_new_uvar();

                self.unify_var_proj_elem(xs_var, x_var)?;

                self.unify_var_utype(
                    acc_x_var,
                    Rc::new(UType::Tuple(vec![acc_var.into(), x_var.into()])),
                )?;
                self.unify_var_pair(acc_var, ret_var)?;
                self.unify_var_pair(newvar, acc_var)?;

                newvar
            }
            Expr::FindByKey(_is_sorted, f_get_key, query_key, seq_expr) => {
                // NOTE - (is-sorted) ~> (x -> key) -> key -> [x] -> Option(T)
                let newvar = self.get_new_uvar();

                let (elem_var, ret_var) = self.infer_vars_expr_lambda(f_get_key, scope)?;
                let key_var = self.infer_var_expr(query_key, scope)?;
                let xs_var = self.infer_var_expr(seq_expr, scope)?;

                let x_var = self.get_new_uvar();

                self.unify_var_proj_elem(xs_var, x_var)?;
                self.unify_var_pair(ret_var, key_var)?;
                self.unify_var_pair(elem_var, x_var)?;
                self.unify_var_utype(newvar, Rc::new(UType::Option(x_var.into())))?;

                newvar
            }
            Expr::FlatMapList(f_expr, ret_type, seq_expr) => {
                // NOTE - (([y], x) -> [y]) -> Vt(y) -> [x] -> [y]
                let ys_var = self.get_new_uvar();

                let (init_x_var, tail_var) = self.infer_vars_expr_lambda(f_expr, scope)?;
                let xs_var = self.infer_var_expr(seq_expr, scope)?;
                let x_var = self.get_new_uvar();
                let y_var = self.get_new_uvar();

                self.unify_var_proj_elem(ys_var, y_var)?;
                self.unify_var_proj_elem(xs_var, x_var)?;
                self.unify_var_valuetype(y_var, ret_type.as_ref())?;
                self.unify_var_pair(ys_var, tail_var)?;

                // constrain the shape to be exactly the tuple we expect
                self.unify_var_utype(
                    init_x_var,
                    Rc::new(UType::Tuple(vec![ys_var.into(), x_var.into()])),
                )?;

                ys_var
            }
            Expr::EnumFromTo(start, stop) => {
                let newvar = self.get_new_uvar();
                let start_var = self.infer_var_expr(start, scope)?;
                let stop_var = self.infer_var_expr(stop, scope)?;

                self.unify_var_baseset(start_var, BaseSet::USome)?;
                self.unify_var_pair(start_var, stop_var)?;

                self.unify_var_proj_elem(newvar, start_var)?;

                newvar
            }
            Expr::Dup(count, x) => {
                let newvar = self.get_new_uvar();
                let count_var = self.infer_var_expr(count, scope)?;
                let x_var = self.infer_var_expr(x, scope)?;

                self.unify_var_baseset(count_var, BaseSet::USome)?;
                self.unify_var_proj_elem(newvar, x_var)?;

                newvar
            }
            Expr::LiftOption(opt) => {
                let newvar = self.get_new_uvar();

                let inner_var = if let Some(expr) = opt.as_ref() {
                    self.infer_var_expr(expr, scope)?
                } else {
                    self.get_new_uvar()
                };

                self.unify_var_utype(
                    newvar,
                    Rc::new(UType::Option(Rc::new(UType::Var(inner_var)))),
                )?;

                newvar
            }
        })
    }

    fn infer_utype_expr(&mut self, e: &Expr, scope: &'_ UScope<'_>) -> TCResult<Rc<UType>> {
        let var = self.infer_var_expr(e, scope)?;
        Ok(Rc::new(UType::Var(var)))
    }

    fn infer_vars_expr_lambda<'a>(
        &mut self,
        expr: &Expr,
        scope: &'a UScope<'a>,
    ) -> TCResult<(UVar, UVar)> {
        match expr {
            Expr::Lambda(head, body) => {
                let head_var = self.get_new_uvar();
                let body_scope = USingleScope::new(scope, head, head_var);
                let body_var = self.infer_var_expr(body.as_ref(), &UScope::Single(body_scope))?;

                Ok((head_var, body_var))
            }
            _ => unreachable!("infer_utype_expr_lambda: unexpected non-lambda expr {expr:?}"),
        }
    }

    /// Attempt to substitute a variable for a VType with at least one more level of refinement
    ///
    /// If there is no possible direct substitution for a VType (i.e. no known constraints),
    /// or multiple possible non-base solutions that cannot be easily tie-broken, returns Ok(None).
    ///
    /// If the only possible refinement would be the identity transformation modulo aliasing, likewise returns
    /// Ok(None).
    ///
    /// If an occurs check fails, returns the corresponding `Err(_)` value.
    ///
    /// Otherwise, returns `Ok(Some(t))` where `t` is a UType other than UType::Var(_) on `v` or an alias thereof
    pub(crate) fn substitute_uvar_vtype(&self, v: UVar) -> Result<Option<VType>, TCError> {
        self.occurs(v)?;
        Ok(match &self.constraints[self.get_canonical_uvar(v).0] {
            Constraints::Indefinite => None,
            Constraints::Variant(vmid) => Some(VType::IndefiniteUnion(*vmid)),
            Constraints::Invariant(c) => match c {
                Constraint::Equiv(ut) => Some(self.to_whnf_vtype(ut.clone())),
                Constraint::Elem(bs) => Some(VType::Base(*bs)),
                Constraint::Proj(ps) => Some(self.proj_shape_to_vtype(ps)),
            },
        })
    }
}
// !SECTION

// SECTION - low-level methods dealing with UVar aliasing concerns
impl TypeChecker {
    /// Performs re-aliasing, recanonicalization, constraint propagation, and constraint unification
    /// to establish direct equality requirements between two meta-variables.
    fn unify_var_pair(&mut self, v1: UVar, v2: UVar) -> TCResult<&Constraints> {
        if v1 == v2 {
            return Ok(&self.constraints[v1.0]);
        }

        // short-circuit if already equated
        match (&self.aliases[v1.0], &self.aliases[v2.0]) {
            (Alias::Ground, Alias::Ground) => {
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
            (Alias::Ground, &Alias::BackRef(can_ix)) if v1.0 > can_ix => unsafe {
                self.repoint(can_ix, v1.0);
                self.transfer_constraints(can_ix, v1.0)
            },
            (Alias::Ground, &Alias::BackRef(can_ix)) if v1.0 < can_ix => {
                debug_assert!(
                    self.aliases[can_ix].is_canonical_nonempty(),
                    "half-alias ?{can_ix}-|<-{v2}"
                );
                debug_assert!(
                    !self.aliases[can_ix].contains_fwd_ref(v1.0),
                    "retrograde half-aliased 'forward' ref ?{can_ix}->|-{v1}"
                );
                unsafe { self.recanonicalize(v1.0, can_ix) }
            }
            (Alias::Ground, &Alias::BackRef(_can_ix)) => {
                unreachable!("unexpected half-alias {v1}-|<-{v2}");
            }
            (&Alias::BackRef(can_ix), Alias::Ground) if v2.0 > can_ix => unsafe {
                self.repoint(can_ix, v2.0);
                self.transfer_constraints(can_ix, v2.0)
            },
            (&Alias::BackRef(can_ix), Alias::Ground) if v2.0 < can_ix => {
                debug_assert!(
                    self.aliases[can_ix].is_canonical_nonempty(),
                    "half-alias ?{can_ix}-|<-{v1}"
                );
                debug_assert!(
                    !self.aliases[can_ix].contains_fwd_ref(v2.0),
                    "retrograde half-aliased 'forward' ref ?{can_ix}->|-{v2}"
                );
                unsafe { self.recanonicalize(v2.0, can_ix) }
            }
            (&Alias::BackRef(_can_ix), Alias::Ground) => {
                unreachable!("unexpected half-alias {v2}-|<-{v1}");
            }
            (Alias::Ground, Alias::Canonical(_)) => {
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
            (Alias::Canonical(_), Alias::Ground) => {
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
            (&Alias::BackRef(ix1), &Alias::BackRef(ix2)) if ix1 < ix2 => unsafe {
                self.recanonicalize(ix1, ix2)
            },
            (&Alias::BackRef(ix1), &Alias::BackRef(ix2)) if ix2 < ix1 => unsafe {
                self.recanonicalize(ix2, ix1)
            },
            (&Alias::BackRef(ix), &Alias::BackRef(_ix)) => {
                // the two are equal so nothing needs to be changed; we will check both are forward-aliased, however
                let common = &self.aliases[ix];
                debug_assert!(
                    common.contains_fwd_ref(v1.0),
                    "unexpected half-alias ?{ix}<-{v1}"
                );
                debug_assert!(
                    common.contains_fwd_ref(v2.0),
                    "unexpected half-alias ?{ix}<-{v2}"
                );
                Ok(&self.constraints[ix])
            }
            (a1 @ Alias::BackRef(tgt), a2 @ Alias::Canonical(fwd_refs)) => {
                let left = fwd_refs.contains(&v1.0);
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
            (a1 @ Alias::Canonical(fwd_refs), a2 @ Alias::BackRef(tgt)) => {
                let left = fwd_refs.contains(&v2.0);
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
    /// Preconditions:
    /// - `a1 < a2`
    /// - `a1` has no back-references, and is allowed (but not required) to have forward-references
    /// - `a2` has no back-references, and is allowed (but not required) to have forward-references
    unsafe fn recanonicalize(&mut self, a1: usize, a2: usize) -> TCResult<&Constraints> {
        let tmp = self.aliases[a2].set_backref(a1);
        let iter = tmp.iter_fwd_refs();
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
    /// and may be marked safe after code stabilizes. Otherwise it is not known to lead to any UB.
    /// guards are enforced in advance.
    unsafe fn repoint(&mut self, lo: usize, hi: usize) {
        self.aliases[hi].set_backref(lo);
        self.aliases[lo].add_forward_ref(hi);
    }

    /// Ensure that all constraints on `a2` are inherited by `a1`, with the reverse occurring as a side-effect.
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
            (Constraints::Variant(vmid), Constraints::Invariant(c)) => {
                if c.is_vacuous() {
                    Ok(&self.constraints[v1.0])
                } else {
                    Err(TCErrorKind::VarianceMismatch(
                        Ord::min(v1, v2),
                        *vmid,
                        self.varmaps.get_varmap(*vmid).clone(),
                        c.clone(),
                        Polarity::PriorVariant,
                    )
                    .into())
                }
            }
            (Constraints::Invariant(c), Constraints::Variant(vmid)) => {
                if c.is_vacuous() {
                    Ok(&self.constraints[v2.0])
                } else {
                    Err(TCErrorKind::VarianceMismatch(
                        Ord::min(v1, v2),
                        *vmid,
                        self.varmaps.get_varmap(*vmid).clone(),
                        c.clone(),
                        Polarity::PriorInvariant,
                    )
                    .into())
                }
            }
            (Constraints::Invariant(c1), Constraints::Invariant(c2)) => {
                let c0 = self.unify_constraint_pair(c1.clone(), c2.clone())?;
                let _ =
                    self.replace_constraints_with_value(v1.0, Constraints::Invariant(c0.clone()));
                let _ = self.replace_constraints_with_value(v2.0, Constraints::Invariant(c0));
                Ok(&self.constraints[v1.0])
            }
        }
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

    pub fn get_canonical_uvar(&self, v: UVar) -> UVar {
        match self.aliases[v.0] {
            Alias::Canonical(_) | Alias::Ground => v,
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
            (Alias::Ground, _) | (_, Alias::Ground) => false,
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
    pub(crate) fn infer_var_format_union(
        &mut self,
        branches: &[Format],
        ctxt: Ctxt<'_>,
    ) -> TCResult<UVar> {
        let newvar = self.get_new_uvar();

        for f in branches {
            match f {
                Format::Variant(lbl, inner) => {
                    let typ = self.infer_utype_format(inner.as_ref(), ctxt)?;
                    self.add_uvar_variant(newvar, lbl.clone(), typ)?;
                }
                other => {
                    let branch_type = self.infer_utype_format(other, ctxt)?;
                    self.unify_var_utype(newvar, branch_type)?;
                }
            }
        }
        Ok(newvar)
    }

    /// Assigns new meta-variables and simple constraints for a format, and returns the novel toplevel UVar
    pub(crate) fn infer_var_format(&mut self, f: &Format, ctxt: Ctxt<'_>) -> TCResult<UVar> {
        match f {
            Format::ItemVar(level, args) => {
                let newvar = self.get_new_uvar();
                let level_var = if !args.is_empty() {
                    let mut arg_scope = UMultiScope::new(ctxt.scope);
                    let expected = ctxt.module.get_args(*level);
                    for ((lbl, vt), arg) in Iterator::zip(expected.iter(), args.iter()) {
                        let v_arg = self.infer_var_expr(arg, ctxt.scope)?;
                        arg_scope.push(lbl.clone(), v_arg);
                        self.unify_var_valuetype(v_arg, vt)?;
                    }
                    let new_scope = UScope::Multi(&arg_scope);
                    let new_ctxt = ctxt.with_scope(&new_scope);
                    self.infer_var_format_level(*level, new_ctxt)?
                } else {
                    self.infer_var_format_level(*level, ctxt)?
                };
                self.unify_var_pair(newvar, level_var)?;
                Ok(newvar)
            }
            Format::ForEach(expr, lbl, inner) => {
                let newvar = self.get_new_uvar();
                let v_expr = self.infer_var_expr(expr, ctxt.scope)?;
                let v_elem = self.get_new_uvar();
                self.unify_var_proj_elem(v_expr, v_elem)?;
                let new_scope = UScope::Single(USingleScope::new(ctxt.scope, lbl, v_elem));
                let new_ctxt = ctxt.with_scope(&new_scope);
                let t_inner = self.infer_utype_format(inner.as_ref(), new_ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(t_inner)))?;
                Ok(newvar)
            }
            Format::Fail => Ok(self.init_var_simple(UType::Empty)?.0),
            Format::SkipRemainder | Format::EndOfInput | Format::Align(_) => {
                Ok(self.init_var_simple(UType::UNIT)?.0)
            }
            Format::DecodeBytes(expr, inner) => {
                let newvar = self.get_new_uvar();

                let v_expr = self.infer_var_expr(expr, ctxt.scope)?;
                let v_inner = self.infer_var_format(inner.as_ref(), ctxt)?;

                // we can only apply DecodeBytes to expressions of type `Seq(U8)`.
                self.unify_var_utype(
                    v_expr,
                    Rc::new(UType::Seq(Rc::new(UType::Base(BaseType::U8)))),
                )?;

                // provided the previous unification succeeded, our output type is equivalent to the inferred type of the inner format
                self.unify_var_pair(newvar, v_inner)?;

                Ok(newvar)
            }
            Format::Byte(_set) => {
                // REVIEW - is there a better approach when matching an empty set of bytes?
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
                Ok(newvar)
            }
            Format::Record(fs) => {
                let newvar = self.get_new_uvar();
                let mut child = UMultiScope::with_capacity(ctxt.scope, fs.len());
                let mut fields = Vec::with_capacity(fs.len());
                for (lbl, f) in fs {
                    let scope = UScope::Multi(&child);
                    let child_ctxt = ctxt.with_scope(&scope);
                    let fv = self.infer_var_format(f, child_ctxt)?;
                    child.push(lbl.clone(), fv);
                    fields.push((lbl.clone(), fv.into()));
                }
                self.unify_var_utype(newvar, Rc::new(UType::Record(fields)))?;
                Ok(newvar)
            }
            Format::Repeat(inner) | Format::Repeat1(inner) => {
                let newvar = self.get_new_uvar();
                let t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(t)))?;
                Ok(newvar)
            }
            Format::RepeatCount(n, inner) => {
                let newvar = self.get_new_uvar();
                let n_type = self.infer_utype_expr(n, ctxt.scope)?;
                // NOTE - we don't care about the constraint, only whether it was successfully computed
                let _constraint = self.unify_utype_baseset(n_type, BaseSet::UAny)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(inner_t)))?;
                Ok(newvar)
            }
            Format::RepeatBetween(min, max, inner) => {
                let newvar = self.get_new_uvar();
                let min_var = self.infer_var_expr(min, ctxt.scope)?;
                let max_var = self.infer_var_expr(max, ctxt.scope)?;
                let _constraint =
                    self.unify_utype_baseset(Rc::new(UType::Var(min_var)), BaseSet::UAny)?;
                self.unify_var_pair(min_var, max_var)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(inner_t)))?;
                Ok(newvar)
            }
            Format::RepeatUntilLast(f, inner) => {
                let newvar = self.get_new_uvar();
                let (in_var, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(in_var, inner_t.clone())?;
                self.unify_var_utype(out_var, Rc::new(UType::Base(BaseType::Bool)))?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(inner_t)))?;
                Ok(newvar)
            }
            Format::RepeatUntilSeq(f, inner) => {
                let newvar = self.get_new_uvar();
                let (in_var, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(in_var, Rc::new(UType::Seq(inner_t.clone())))?;
                self.unify_var_utype(out_var, Rc::new(UType::Base(BaseType::Bool)))?;
                self.unify_var_utype(newvar, Rc::new(UType::Seq(inner_t)))?;
                Ok(newvar)
            }
            Format::AccumUntil(lambda_acc_seq, lambda_acc_elt, init_acc, vt_acc, inner) => {
                // NOTE - ((acc, [x]) -> bool) -> ((acc. x) -> acc) -> acc -> Vt(acc) -> x -> (acc, [x])
                let newvar = self.get_new_uvar();
                let (acc_seq_var, done_var) =
                    self.infer_vars_expr_lambda(lambda_acc_seq, ctxt.scope)?;
                let (acc_elt_var, update_var) =
                    self.infer_vars_expr_lambda(lambda_acc_elt, ctxt.scope)?;
                let acc_var = self.infer_var_expr(init_acc, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;

                // establish known equivalences

                // Initial accumulator value has the declared type
                self.unify_var_valuetype(acc_var, vt_acc.as_ref())?;

                // terminal predicate is (acc, [x]) -> bool
                self.unify_var_utype(
                    acc_seq_var,
                    Rc::new(UType::tuple([
                        UType::Var(acc_var),
                        UType::Seq(inner_t.clone()),
                    ])),
                )?;
                self.unify_var_utype(done_var, Rc::new(UType::Base(BaseType::Bool)))?;

                // update function is (acc, x) -> acc
                self.unify_var_utype(
                    acc_elt_var,
                    Rc::new(UType::tuple([UType::Var(acc_var), (&*inner_t).clone()])),
                )?;
                self.unify_var_pair(update_var, acc_var)?;

                // assign correct type to newvar
                self.unify_var_pair(newvar, acc_seq_var)?;
                Ok(newvar)
            }
            Format::Maybe(cond, inner) => {
                let newvar = self.get_new_uvar();
                let cond_var = self.infer_var_expr(cond, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(cond_var, Rc::new(UType::Base(BaseType::Bool)))?;
                self.unify_var_utype(newvar, Rc::new(UType::Option(inner_t)))?;
                Ok(newvar)
            }
            Format::Peek(peek) => {
                let newvar = self.get_new_uvar();
                let peek_t = self.infer_utype_format(peek, ctxt)?;
                self.unify_var_utype(newvar, peek_t)?;
                Ok(newvar)
            }
            Format::PeekNot(peek) => {
                let newvar = self.init_var_simple(UType::UNIT)?.0;
                let _peek_t = self.infer_utype_format(peek, ctxt)?;
                Ok(newvar)
            }
            Format::Slice(sz, inner) => {
                let newvar = self.get_new_uvar();
                let sz_t = self.infer_utype_expr(sz, ctxt.scope)?;
                self.unify_utype_baseset(sz_t, BaseSet::USome)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar)
            }
            Format::Bits(inner) => {
                let newvar = self.get_new_uvar();
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar)
            }
            Format::WithRelativeOffset(addr, offs, inner) => {
                let newvar = self.get_new_uvar();
                let addr_var = self.infer_var_expr(addr, ctxt.scope)?;
                let offs_var = self.infer_var_expr(offs, ctxt.scope)?;
                self.unify_var_baseset(addr_var, BaseSet::USome)?;
                // REVIEW - addr_var and offs_var only need to be compatible, not identical, but in our current model it is hard to support heterogenous typings
                self.unify_var_pair(addr_var, offs_var)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar)
            }
            Format::Map(inner, f) => {
                let newvar = self.get_new_uvar();
                let inner_t = self.infer_utype_format(inner, ctxt)?;

                let (in_v, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                self.unify_var_utype(in_v, inner_t)?;
                self.unify_var_pair(newvar, out_var)?;
                Ok(newvar)
            }
            Format::Where(inner, f) => {
                let newvar = self.get_new_uvar();
                let inner_t = self.infer_utype_format(inner, ctxt)?;

                let (in_v, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                self.unify_var_pair(newvar, in_v)?;
                self.unify_var_utype(newvar, inner_t)?;
                self.unify_var_utype(out_var, Rc::new(UType::Base(BaseType::Bool)))?;
                Ok(newvar)
            }
            Format::Compute(x) => {
                let newvar = self.get_new_uvar();
                let xt = self.infer_utype_expr(x, ctxt.scope)?;
                self.unify_var_utype(newvar, xt)?;
                Ok(newvar)
            }
            Format::Let(lab, x, inner) => {
                let newvar = self.get_new_uvar();
                let xvar = self.infer_var_expr(x, ctxt.scope)?;
                let new_scope = UScope::Single(USingleScope::new(ctxt.scope, lab, xvar));
                let new_ctxt = ctxt.with_scope(&new_scope);
                let inner_t = self.infer_utype_format(inner, new_ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar)
            }
            Format::Match(x, branches) => {
                let newvar = self.get_new_uvar();
                let tx = self.infer_utype_expr(x, ctxt.scope)?;
                for (pat, rhs) in branches.iter() {
                    self.unify_utype_format_match_case(tx.clone(), pat, newvar, rhs, ctxt)?;
                }
                Ok(newvar)
            }
            Format::Dynamic(lbl, dynf, inner) => {
                let newvar = self.get_new_uvar();
                let uv_dynf = self.infer_var_dyn_format(dynf, ctxt)?;
                let new_ctxt = ctxt.with_dyn_binding(lbl, uv_dynf);
                let inner_t = self.infer_utype_format(inner, new_ctxt)?;
                self.unify_var_utype(newvar, inner_t)?;
                Ok(newvar)
            }
            Format::Apply(label) => {
                let newvar = self.get_new_uvar();
                let uv_dynf = ctxt
                    .dyn_s
                    .get_dynf_var_by_name(label)
                    .unwrap_or_else(|| panic!("missing dynformat {label}"));
                self.unify_var_pair(newvar, uv_dynf)?;
                Ok(newvar)
            }
            Format::Pos => {
                let newvar = self.get_new_uvar();
                self.unify_var_baseset(newvar, BaseSet::U(UintSet::any_default(IntWidth::Bits64)))?;
                Ok(newvar)
            }
            Format::LetFormat(f0, name, f) => {
                let newvar = self.get_new_uvar();
                let f0_v = self.infer_var_format(f0, ctxt)?;
                let scope = UScope::Single(USingleScope::new(ctxt.scope, name, f0_v));
                let new_ctxt = ctxt.with_scope(&scope);
                let f_v = self.infer_var_format(f, new_ctxt)?;
                self.unify_var_pair(newvar, f_v)?;
                Ok(newvar)
            }
        }
    }

    pub(crate) fn infer_utype_format(
        &mut self,
        format: &Format,
        ctxt: Ctxt<'_>,
    ) -> TCResult<Rc<UType>> {
        let uv = self.infer_var_format(format, ctxt)?;
        Ok(uv.into())
    }

    pub fn lookup_level_var(&self, level: usize) -> Option<UVar> {
        if level == 0 {
            Some(UVar(0))
        } else {
            Some(*self.level_vars.get(&level)?)
        }
    }

    /// Attempt to fully solve a `UType` until all free meta-variables are replaced with concrete type-assignments
    ///
    /// Returns None if at least one meta-variable cannot be reduced without more information, or if any unification
    /// is insoluble.
    ///
    /// # Panics
    ///
    /// Will panic if [`UType::Hole`] is encountered, or if any `UVar` has an unresolved record- or tuple- `ProjShape` constraint.
    pub fn reify(&self, t: Rc<UType>) -> Option<ValueType> {
        match t.as_ref() {
            UType::Hole => {
                // REVIEW - should this simply return None instead? or maybe ValueType::Any?
                unreachable!("reify: UType::Hole should be erased by any non-Hole unification!");
            }
            &UType::Var(uv) => {
                let v = self.get_canonical_uvar(uv);
                match self.substitute_uvar_vtype(v) {
                    Ok(Some(t0)) =>
                        match t0 {
                            VType::Base(bs) =>
                                match bs.get_unique_solution(uv).as_deref() {
                                    Ok(UType::Base(b)) => Some(ValueType::Base(*b)),
                                    Ok(other) => unreachable!("base-set {bs:?} yielded unexpected solution {other:?}"),
                                    Err(_e) => None,
                                }
                            VType::Abstract(ut) => self.reify(ut),
                            VType::IndefiniteUnion(vmid) => self.reify_union(vmid),
                            VType::ImplicitRecord(..) | VType::ImplicitTuple(..) =>
                                unreachable!(
                                    "Unsolved implicit Tuple or Record leftover from un-unified projection: {t0:?}"
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
            UType::Option(t0) => Some(ValueType::Option(Box::new(self.reify(t0.clone())?))),
        }
    }

    fn reify_union(&self, vmid: VMId) -> Option<ValueType> {
        let vm = self.varmaps.get_varmap(vmid);
        let mut branches = BTreeMap::new();
        for (label, ut) in vm.iter() {
            let variant_type = self.reify(ut.clone())?;
            // NOTE - only add a variant to the union-type if its inner type is inhabitable
            if !matches!(variant_type, ValueType::Empty) {
                branches.insert(label.clone(), variant_type);
            }
        }
        Some(ValueType::Union(branches))
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
pub enum UnificationError<T: std::fmt::Debug> {
    // Incompatible(UVar, T, T), // two independent assertions about a UVar are incompatible
    Unsatisfiable(T, T), // a single non-variable assertion is directly unsatisfiable
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Marker enum to track which of an invariant and variant constraints came first
pub enum Polarity {
    /// Attempting to add variants onto an invariant metavariable
    PriorInvariant,
    /// Attempting to enforce invariant constraints on a Variant metavariable
    PriorVariant,
}

#[derive(Debug)]
pub struct TCError {
    err: TCErrorKind,
    _trace: Vec<Box<dyn std::fmt::Debug + 'static + Send + Sync>>,
}

impl From<TCErrorKind> for TCError {
    fn from(value: TCErrorKind) -> Self {
        Self {
            err: value,
            _trace: Vec::new(),
        }
    }
}

impl TCError {
    #[allow(dead_code)]
    fn with_trace<T>(mut self, trace: T) -> Self
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

#[derive(Debug)]
pub enum TCErrorKind {
    VarianceMismatch(UVar, VMId, VarMap, Constraint, Polarity), // attempted unification of a variant and non-variant constraint
    Unification(ConstraintError),
    InfiniteType(UVar, Constraints),
    MultipleSolutions(UVar, BaseSet),
    NoSolution(UVar),
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
            err: TCErrorKind::Unification(value.0),
            _trace: vec![Box::new(value.1)],
        }
    }
}

impl From<ConstraintError> for TCErrorKind {
    fn from(value: ConstraintError) -> Self {
        Self::Unification(value)
    }
}

impl std::fmt::Display for TCErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VarianceMismatch(uv, vmid, vm, constraint, pol) =>
                match pol {
                    Polarity::PriorInvariant =>
                        write!(
                            f,
                            "prior constraint `{uv} {constraint}` precludes attempted unification `{uv} ⊇ {vmid} (:= {vm:?})`"
                        ),
                    Polarity::PriorVariant =>
                        write!(
                            f,
                            "attempted unification `{uv} {constraint}` precluded by prior constraint `{uv} ⊇ {vmid} (:= {vm:?})`"
                        ),
                }
            Self::Unification(c_err) => write!(f, "{c_err}"),
            Self::InfiniteType(v, constraints) =>
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
            Self::MultipleSolutions(uv, bs) =>
                write!(f, "no unique solution for `{uv} {}`", bs.to_constraint()),
            Self::NoSolution(uv) =>
                write!(f, "no valid solutions for `{uv}`"),
        }
    }
}

impl std::error::Error for TCErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unification(u_err) => Some(u_err),
            _ => None,
        }
    }
}

impl std::error::Error for TCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.err.source()
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

/// Perform a standalone type-inference on a format within a format-module, returning
/// the inferred value-type, if one could be inferred.
///
/// Will return `Ok(None)` if there were no typechecking errors, but a concrete ValueType
/// could not be inferred.
///
/// Will return `Err(_)` if any type-error is encountered.
///
/// Otherwise returns `Ok(Some(vt))` where `vt` is the inferred value-type of `f`
pub fn typecheck(module: &FormatModule, f: &Format) -> TCResult<Option<ValueType>> {
    let mut tc = TypeChecker::new();
    let scope = UScope::new();
    let ctxt = Ctxt::new(module, &scope);
    let _ut = tc.infer_utype_format(f, ctxt)?;
    // FIXME - there should be a lot more that goes on under the covers here, especially since we want to detect errors
    let ret = Ok(tc.reify(_ut));
    tc.check_uvar_sanity();
    ret
}

pub type TCResult<T> = Result<T, TCError>;

mod __impl {
    use crate::FormatModule;

    use super::{Constraint, Ctxt, ProjShape, UScope, UVar, VMId};
    use std::borrow::{Borrow, BorrowMut};

    impl std::fmt::Display for Constraint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Constraint::Equiv(ut) => write!(f, "= {ut:?}"),
                Constraint::Elem(bs) => write!(f, "∈ {bs}"),
                Constraint::Proj(ps) => match ps {
                    ProjShape::TupleWith(ts) => write!(
                        f,
                        "~ TupleGT{}(..)",
                        ts.last_key_value().map(|x| *x.0).unwrap_or(0)
                    ),
                    ProjShape::RecordWith(fs) => write!(f, "~ Record(..) (>={} fields)", fs.len()),
                    ProjShape::SeqOf(elv) => write!(f, "~ Seq({elv})"),
                },
            }
        }
    }

    impl std::fmt::Display for UVar {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "?{}", self.0)
        }
    }

    impl From<UVar> for usize {
        fn from(value: UVar) -> Self {
            value.0
        }
    }

    impl From<usize> for UVar {
        fn from(value: usize) -> Self {
            Self(value)
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

    impl<'a> From<&'a FormatModule> for Ctxt<'a> {
        fn from(value: &'a FormatModule) -> Self {
            let scope = &UScope::Empty;
            Self::new(value, scope)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::byte_set::ByteSet;
    use crate::helper::compute;
    use crate::{Arith, TypeHint};

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
        let output = tc
            .reify(ut)
            .unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Union(BTreeMap::from([
            ("A".into(), ValueType::Base(BaseType::U8)),
            ("B".into(), ValueType::Tuple(vec![])),
        ]));
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_non_union_complex() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::Record(vec![
            ("number".into(), Format::Byte(ByteSet::full())),
            (
                "isEven".into(),
                compute(Expr::Match(
                    Box::new(Expr::Arith(
                        Arith::Rem,
                        Box::new(Expr::Var("number".into())),
                        Box::new(Expr::U8(2)),
                    )),
                    vec![
                        (Pattern::U8(0), Expr::Bool(true)),
                        (Pattern::Wildcard, Expr::Bool(false)),
                    ],
                )),
            ),
        ]);
        let mut module = FormatModule::new();
        module.define_format("foo", format.clone());
        let scope = UScope::new();
        let ut = tc.infer_utype_format(&format, Ctxt::new(&module, &scope))?;
        println!("ut: {ut:?}");
        println!("tc: {tc:?}");
        let output = tc
            .reify(ut)
            .unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Record(vec![
            ("number".into(), ValueType::Base(BaseType::U8)),
            ("isEven".into(), ValueType::Base(BaseType::Bool)),
        ]);
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_union_complex() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::Record(vec![
            ("number".into(), Format::Byte(ByteSet::full())),
            (
                "parity".into(),
                compute(Expr::Match(
                    Box::new(Expr::Arith(
                        Arith::Rem,
                        Box::new(Expr::Var("number".into())),
                        Box::new(Expr::U8(2)),
                    )),
                    vec![
                        (
                            Pattern::U8(0),
                            Expr::Variant("Even".into(), Box::new(Expr::UNIT)),
                        ),
                        (
                            Pattern::Wildcard,
                            Expr::Variant("Odd".into(), Box::new(Expr::UNIT)),
                        ),
                    ],
                )),
            ),
        ]);
        let mut module = FormatModule::new();
        module.define_format("foo", format.clone());
        let scope = UScope::new();
        let ut = tc.infer_utype_format(&format, Ctxt::new(&module, &scope))?;
        println!("ut: {ut:?}");
        println!("tc: {tc:?}");
        let output = tc
            .reify(ut)
            .unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Record(vec![
            ("number".into(), ValueType::Base(BaseType::U8)),
            (
                "parity".into(),
                ValueType::Union(BTreeMap::from([
                    ("Even".into(), ValueType::UNIT),
                    ("Odd".into(), ValueType::UNIT),
                ])),
            ),
        ]);
        assert_eq!(output, expected);
        Ok(())
    }

    fn mk_format_u32() -> Format {
        Format::Map(
            Box::new(Format::Tuple(vec![Format::Byte(ByteSet::full()); 4])),
            Box::new(Expr::Lambda(
                "x".into(),
                Box::new(Expr::U32Be(Box::new(Expr::Var("x".into())))),
            )),
        )
    }

    #[test]
    fn test_lambda_accum() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::Map(
            Box::new(Format::Repeat(Box::new(mk_format_u32()))),
            Box::new(Expr::Lambda(
                "xs".into(),
                Box::new(Expr::FlatMapAccum(
                    Box::new(Expr::Lambda(
                        "acc_x".into(),
                        Box::new(Expr::Tuple(vec![
                            Expr::Arith(
                                Arith::Mul,
                                Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 0)),
                                Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 1)),
                            ),
                            Expr::Seq(vec![Expr::Arith(
                                Arith::Add,
                                Box::new(Expr::U32(1)),
                                Box::new(Expr::Arith(
                                    Arith::Mul,
                                    Box::new(Expr::TupleProj(
                                        Box::new(Expr::Var("acc_x".into())),
                                        0,
                                    )),
                                    Box::new(Expr::TupleProj(
                                        Box::new(Expr::Var("acc_x".into())),
                                        1,
                                    )),
                                )),
                            )]),
                        ])),
                    )),
                    Box::new(Expr::U32(1)),
                    TypeHint::from(ValueType::Base(BaseType::U32)),
                    Box::new(Expr::Var("xs".into())),
                )),
            )),
        );
        let module = FormatModule::new();
        // module.define_format("prod32", format.clone());
        let scope = UScope::new();
        let ut = tc.infer_utype_format(&format, Ctxt::new(&module, &scope))?;
        let _trace = format!("ut: {ut:?}\ntc: {tc:?}");
        let output = tc
            .reify(ut)
            .unwrap_or_else(|| panic!("reify returned None"));
        let expected = ValueType::Seq(Box::new(ValueType::Base(BaseType::U32)));
        assert_eq!(output, expected);
        Ok(())
    }
}
