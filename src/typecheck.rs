use crate::typecheck::inference::InferenceError;
use crate::valuetype::{SeqBorrowHint, augmented::AugValueType};
use crate::{
    Arith, BaseType, DynFormat, Expr, Format, FormatModule, Label, Pattern, UnaryOp, ValueType,
    ViewExpr, ViewFormat,
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    rc::Rc,
};

pub(crate) mod inference;
use crate::numeric::elaborator::{IntType, PrimInt};

/// Helper function for constructing a tuple of the form `(min(a, b), max(a, b))`.
///
/// Used primarily for case-analysis in aliasing logic.
#[inline(always)]
fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}

mod scope {
    use std::collections::BTreeSet;

    use super::UVar;
    use crate::{FormatModule, Label};

    #[derive(Debug, Clone, Copy, Default)]
    pub(crate) enum UScope<'a> {
        #[default]
        Empty,
        Multi(&'a UMultiScope<'a>),
        Single(USingleScope<'a>),
    }

    impl<'a> UScope<'a> {
        pub const fn new() -> Self {
            Self::Empty
        }

        pub(crate) fn get_uvar_by_name(&self, name: &str) -> Option<UVar> {
            match self {
                UScope::Empty => None,
                UScope::Multi(multi) => multi.get_uvar_by_name(name),
                UScope::Single(single) => single.get_uvar_by_name(name),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct UMultiScope<'a> {
        pub(crate) parent: &'a UScope<'a>,
        pub(crate) entries: Vec<(Label, UVar)>,
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

        pub(crate) fn get_uvar_by_name(&self, name: &str) -> Option<UVar> {
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
        pub(crate) parent: &'a UScope<'a>,
        pub(crate) name: &'a str,
        pub(crate) uvar: UVar,
    }

    impl<'a> USingleScope<'a> {
        pub const fn new(parent: &'a UScope<'a>, name: &'a str, uvar: UVar) -> USingleScope<'a> {
            Self { parent, name, uvar }
        }

        pub(crate) fn get_uvar_by_name(&self, name: &str) -> Option<UVar> {
            if self.name == name {
                return Some(self.uvar);
            }
            self.parent.get_uvar_by_name(name)
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub(crate) enum ViewScope<'a> {
        #[default]
        Empty,
        Single(ViewSingleScope<'a>),
        Multi(&'a ViewMultiScope<'a>),
    }

    impl<'a> ViewScope<'a> {
        pub const fn new() -> Self {
            Self::Empty
        }

        pub(crate) fn includes_name(&self, name: &str) -> bool {
            match self {
                ViewScope::Empty => false,
                ViewScope::Single(s) => s.includes_name(name),
                ViewScope::Multi(m) => m.includes_name(name),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct ViewMultiScope<'a> {
        pub(crate) parent: &'a ViewScope<'a>,
        pub(crate) entries: BTreeSet<Label>,
    }

    impl<'a> ViewMultiScope<'a> {
        pub fn new(parent: &'a ViewScope<'a>) -> Self {
            Self {
                parent,
                entries: BTreeSet::new(),
            }
        }

        /// Records the presence of a view-kinded dep-format parameter in this scope.
        ///
        /// # Panics
        ///
        /// Will panic if the same identifier is added twice to the same local scope (but not if it is
        /// re-used across layers of scope).
        pub fn push_view(&mut self, name: Label) {
            // FIXME - the clone cost ought to be avoidable but
            let _name = name.clone();
            if !self.entries.insert(name) {
                unreachable!("duplicate parameter identifier in format view-params: {_name}");
            }
        }

        pub(crate) fn includes_name(&self, name: &str) -> bool
where {
            self.entries.contains(name) || self.parent.includes_name(name)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct ViewSingleScope<'a> {
        pub(crate) parent: &'a ViewScope<'a>,
        pub(crate) name: &'a str,
    }

    impl<'a> ViewSingleScope<'a> {
        pub const fn new(parent: &'a ViewScope<'a>, name: &'a str) -> ViewSingleScope<'a> {
            Self { parent, name }
        }

        pub(crate) fn includes_name(&self, name: &str) -> bool {
            self.name == name || self.parent.includes_name(name)
        }
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
        pub(crate) parent: &'a DynScope<'a>,
        pub(crate) name: &'a str,
        pub(crate) dynf_var: UVar,
    }

    impl<'a> DynSingleScope<'a> {
        pub const fn new(parent: &'a DynScope<'a>, name: &'a str, dynf_var: UVar) -> Self {
            Self {
                parent,
                name,
                dynf_var,
            }
        }

        pub(crate) fn get_dynf_var_by_name(&self, label: &str) -> Option<UVar> {
            if label == self.name {
                Some(self.dynf_var)
            } else {
                self.parent.get_dynf_var_by_name(label)
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub(crate) struct Ctxt<'a> {
        pub(crate) module: &'a FormatModule,
        pub(crate) scope: &'a UScope<'a>,
        pub(crate) dyn_s: DynScope<'a>,
        pub(crate) views: ViewScope<'a>,
    }

    impl<'a> Ctxt<'a> {
        pub const fn new(module: &'a FormatModule, scope: &'a UScope<'a>) -> Self {
            Self {
                module,
                scope,
                dyn_s: DynScope::new(),
                views: ViewScope::new(),
            }
        }
        /// Returns a copy of `self` with the given `UScope` instead of `self.scope`.
        pub(crate) fn with_scope(&'a self, scope: &'a UScope<'a>) -> Ctxt<'a> {
            Self {
                module: self.module,
                dyn_s: self.dyn_s,
                views: self.views,
                scope,
            }
        }

        pub(crate) fn with_view_binding(&'a self, name: &'a str) -> Ctxt<'a> {
            Self {
                module: self.module,
                dyn_s: self.dyn_s,
                scope: self.scope,
                views: ViewScope::Single(ViewSingleScope::new(&self.views, name)),
            }
        }

        pub(crate) fn with_view_bindings(&'a self, views: &'a ViewMultiScope<'a>) -> Ctxt<'a> {
            Self {
                module: self.module,
                dyn_s: self.dyn_s,
                views: ViewScope::Multi(views),
                scope: self.scope,
            }
        }

        pub(crate) fn with_dyn_binding(&'a self, name: &'a str, dynf_var: UVar) -> Ctxt<'a> {
            Self {
                module: self.module,
                dyn_s: DynScope::Single(DynSingleScope::new(&self.dyn_s, name, dynf_var)),
                scope: self.scope,
                views: self.views.clone(),
            }
        }
    }
}
use scope::{Ctxt, UMultiScope, UScope, USingleScope, ViewMultiScope};

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
    ($x:expr_2021 => $y:expr_2021) => {
        match $x {
            Ok(val) => val,
            Err(e) => return Err(e.with_trace($y)),
        }
    };
    ($x:expr_2021 $(=> ())?) => {
        $x?
    };
}

/// Unification variable for use in typechecking
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UVar(pub(crate) usize);

impl UVar {
    pub fn new(ix: usize) -> Self {
        Self(ix)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ExtVar(pub(crate) usize);

impl ExtVar {
    pub fn new(ix: usize) -> Self {
        Self(ix)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

/// Unification type, equivalent to ValueType up to abstraction
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UType {
    /// Reserved case for Formats that fundamentally cannot be parsed successfully (Format::Fail and implied failure-cases)
    Empty,
    /// Anonymous type-hole for shape-only unifications (i.e. where we would want to use a meta-variable but don't have one available).
    Hole,
    /// Reserved case for View-Objects manifest through ViewFormat::ReifyView
    ViewObj,
    /// Indexed type-hole acting as a unification metavariable
    Var(UVar),
    Base(BaseType),
    Tuple(Vec<Rc<UType>>),
    Record(Vec<(Label, Rc<UType>)>),
    Seq(Rc<UType>, SeqBorrowHint),
    /// For `std::option::Option<InnerType>`
    Option(Rc<UType>),
    PhantomData(Rc<UType>),
    /// Type equivalent to the ascribed type of the root node of an embedded numeric-inference-engine
    ExternVar(ExtVar),
}

impl UType {
    pub fn seq(self: Rc<Self>) -> Self {
        Self::Seq(self, SeqBorrowHint::Constructed)
    }

    pub fn opt(self: Rc<Self>) -> Self {
        Self::Option(self)
    }

    pub fn seq_view(self: Rc<Self>) -> Self {
        Self::Seq(self, SeqBorrowHint::BufferView)
    }

    pub fn seq_array(self: Rc<Self>) -> Self {
        Self::Seq(self, SeqBorrowHint::ReadArray)
    }
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

impl From<ExtVar> for Rc<UType> {
    fn from(value: ExtVar) -> Self {
        Rc::new(UType::ExternVar(value))
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
            ValueType::Any | ValueType::UnknownNumeric => Some(Self::Hole),
            ValueType::Empty => Some(Self::Empty),
            ValueType::ViewObj => Some(Self::ViewObj),
            ValueType::PhantomData(inner) => {
                let inner_t = Self::from_valuetype(inner)?;
                Some(UType::PhantomData(Rc::new(inner_t)))
            }
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
            ValueType::Seq(inner) => Some(Self::seq(Rc::new(Self::from_valuetype(inner)?))),
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
            UType::Empty
            | UType::ViewObj
            | UType::Hole
            | UType::Var(..)
            | UType::ExternVar(..)
            | UType::Base(..) => Box::new(std::iter::empty()),
            UType::Tuple(ts) => Box::new(ts.iter().cloned()),
            UType::Record(fs) => Box::new(fs.iter().map(|(_l, t)| t.clone())),
            UType::Seq(t, _) | UType::Option(t) | UType::PhantomData(t) => {
                Box::new(std::iter::once(t.clone()))
            }
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

/// Association table between the label for a branch-variant and the type of its contents
type VarMap = HashMap<Label, Rc<UType>>;

/// Bookkeeping structure that stores the association tables for each union-kinded metavariable constraint (as the underlying value of a VMId),
/// as well as keeping track of the next VMId to use if a novel union-kinded constraint is encountered
///
/// During type unification, co-aliased meta-variables that start out with distinct constraint VMIds will be merged, along with the
/// corresponding VarMaps in question (provided unification is non-conflicting), pruning the entry for one of the two key VMIds.
#[derive(Debug)]
struct VarMapMap {
    /// Mapping from VMId to VarMap
    store: HashMap<usize, VarMap>,
    /// Next available VMId value to use
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

/// Type representing the general constraints on a metavariable.
///
/// By default, any meta-variable that is fully unconstrained will have `Constraints::Indefinite` as its value.
///
/// Otherwise, a metavariable will have either `Constraints::Variant` or `Constraints::Invariant` as its value,
/// depending on whether it is observed to be a tagged union-member or an untagged value, respectively.
#[derive(Clone, Debug, Default)]
pub enum Constraints {
    #[default]
    /// Default value before any inference is possible. Erased by any non-trivial unification
    Indefinite,
    /// Indirection via a VarMap Identifier (VMId) to a partial set of observed variants.
    Variant(VMId),
    /// Inferred constraints on a metavariable that is not a tagged union-member. Unification against `Constraints::Variant` may yet be possible but only in select cases.
    Invariant(Constraint),
    // TODO: add `Numeric`/`Engine` for incremental integration of InferenceEngine
}

impl Constraints {
    pub const fn new() -> Self {
        Self::Indefinite
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Constraint {
    /// Direct equivalence with a UType, which should not be a bare `UType::Var` (as that is implied by the independently-tracked Alias value for a metavariable)
    Equiv(Rc<UType>),
    /// Member of a set of ground-types (e.g. in the case of the inner expression of  `Expr::AsU32(..)`)
    Elem(BaseSet),
    /// Constraints implied by projections into a parametric type (e.g. Option, Seq), a Tuple, or a Record
    Proj(ProjShape),
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

    /// Normalizes a constraint so that effectively-identical constraints are structurally equivalent.
    ///
    /// Specifically, `Constraint::Equiv(UType::Base(b))` is normalized to `Constraint::Elem(BaseSet::Single(b))`,
    /// and all other constraints are returned unchanged.
    pub fn normalize(self) -> Self {
        if let Constraint::Equiv(ut) = &self
            && let UType::Base(b) = ut.as_ref()
        {
            Constraint::Elem(BaseSet::Single(*b))
        } else {
            self
        }
    }
}

/// Type representing each possible shape of a projective constraint on a higher-order metavariable
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjShape {
    TupleWith(BTreeMap<usize, UVar>), // required associations of meta-variables at given indices of an uncertain tuple
    RecordWith(BTreeMap<Label, UVar>), // required associations of meta-variables at given fields of an uncertain record
    SeqOf(UVar),                       // simple sequence element-type projection
    OptOf(UVar),                       // simple Option param-type projection
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

    pub fn opt_of(inner: UVar) -> Self {
        Self::OptOf(inner)
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

pub mod base_set {
    use super::*;
    use crate::BaseType;
    use crate::numeric::elaborator::PrimInt;

    /// Abstraction over explicit collections of BaseType values that could be in any order
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum BaseSet {
        /// Singleton set of any BaseType, even non-integral ones
        Single(BaseType),
        /// Some subset of U8, U16, U32, U64
        U(UintSet),
    }

    impl BaseSet {
        pub fn contains(self, base: BaseType) -> bool {
            match self {
                BaseSet::Single(b) => b == base,
                BaseSet::U(us) => us.contains(base),
            }
        }
    }

    #[derive(Debug)]
    pub struct TryFromSignedPrimIntError(PrimInt);

    impl std::fmt::Display for TryFromSignedPrimIntError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "unable to convert signed-integer-type {}", self.0)
        }
    }

    impl TryFrom<PrimInt> for BaseType {
        type Error = TryFromSignedPrimIntError;

        fn try_from(value: PrimInt) -> Result<Self, Self::Error> {
            match value {
                PrimInt::U8 => Ok(BaseType::U8),
                PrimInt::U16 => Ok(BaseType::U16),
                PrimInt::U32 => Ok(BaseType::U32),
                PrimInt::U64 => Ok(BaseType::U64),
                _ => Err(TryFromSignedPrimIntError(value)),
            }
        }
    }

    impl BaseSet {
        #[allow(non_upper_case_globals)]
        pub const UAny: Self = Self::U(UintSet::ANY);

        #[allow(non_upper_case_globals)]
        pub const USome: Self = Self::U(UintSet::ANY32);
    }

    // REVIEW - merge candidate with crate::numeric::core::BitWidth
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
        /// The value is not in the set
        Excluded,
        /// The value is in the set and has priority rank `n` (lower numbers are higher priority)
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

    #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct UintSet {
        // Array with ranks for U8, U16, U32, U64 in that order
        pub(crate) ranks: [Rank; 4],
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
                write!(f, "{{}}")
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

        /// Normalizes all inhabited ranks such that the lowest rank is `Rank::At(0)`,
        /// and each successive rank-value is exactly one more than its predecessor.
        pub fn normalize(self) -> Self {
            let mut ranks = self.ranks;
            for rank in ranks.iter_mut() {
                let orig_val = *rank;
                if orig_val == Rank::Excluded {
                    continue;
                }
                let count_gte = (self.ranks.iter().filter(|r| **r >= orig_val).count() - 1) as u8;
                *rank = Rank::At(count_gte);
            }
            Self { ranks }
        }

        /// Returns a `UintSet` whose solution-set is the intersection of `self` and `other`,
        /// using the lower-priority rank for each member-element that neither one excludes.
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

        /// Returns `true` if the solution-set of `self` is empty.
        pub fn is_empty(&self) -> bool {
            self.ranks == [Rank::Excluded; 4]
        }

        /// Given a `UintSet`, determines the unique solution it has, if any.
        ///
        /// If multiple solutions exist, but there is one solution with a higher-priority
        /// rank than all others, that solution is returned.
        ///
        /// If no solutions exist (i.e. the set is empty), or if there is more than one
        /// solution ascribed with the highest-priority rank, then `None` is returned.
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

        /// Returns `Some(UType)` if `self` is a singleton `BaseSet`, and `None` otherwise.
        ///
        /// Will not attempt to solve the set if it is not an explicit singleton, i.e. `BaseSet::U` regardless
        /// of whether it has a unique solution.
        pub fn try_to_utype(self) -> Option<Rc<UType>> {
            match self {
                BaseSet::Single(b) => Some(Rc::new(UType::Base(b))),
                BaseSet::U(_) => None,
            }
        }

        pub(crate) fn get_unique_solution(&self, v: UVar) -> TCResult<BaseType> {
            match self {
                BaseSet::Single(b) => Ok(*b),
                BaseSet::U(u) => {
                    if u.is_empty() {
                        return Err(TCErrorKind::NoSolution(v).into());
                    }
                    match u.get_unique_solution() {
                        Some(b) => Ok(b),
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
}
use base_set::{BaseSet, IntWidth, UintSet};

/// Type for indicating the solution-state of an ExtVar (num-tree root-node type)
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum NumSolution {
    #[default]
    /// Solution has not yet been found
    Unsolved,
    /// Solution has been found but was elided due to aliasing; should not be stored for canonical ExtVars
    Elided,
    /// Canonical solution-state before elision is performed on non-canonical ExtVars
    Solved(crate::numeric::elaborator::IntType),
}

impl std::fmt::Display for NumSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumSolution::Unsolved => write!(f, "⁇"),
            NumSolution::Elided => write!(f, "…"),
            NumSolution::Solved(sol) => write!(f, "{sol}"),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct EmbeddedResolver {
    subtrees: Vec<Rc<inference::InferenceEngine>>,
    aliases: Vec<Alias>,
    outcomes: HashMap<ExtVar, NumSolution>,
}

// SECTION - EmbeddedResolver core functionality
impl EmbeddedResolver {
    /// Constructs a new, empty `EmbeddedResolver`.
    pub(crate) fn new() -> Self {
        Self {
            subtrees: Vec::new(),
            aliases: Vec::new(),
            outcomes: HashMap::new(),
        }
    }

    /// Initializes a new ExtVar with the given InferenceEngine, returning the ExtVar.
    fn init_new_extvar(&mut self, engine: inference::InferenceEngine) -> ExtVar {
        let ext_var = ExtVar(self.subtrees.len());
        self.subtrees.push(Rc::new(engine));
        self.aliases.push(Alias::new());
        // NOTE - we leave outcomes alone because it is keyed, not indexed linearly
        ext_var
    }
}
// !SECTION

// SECTION - methods related to querying outcomes on individual ExtVars
impl EmbeddedResolver {
    pub(crate) fn solve(&mut self, ext: ExtVar) -> TCResult<crate::numeric::elaborator::IntType> {
        match self.outcomes.get(&ext) {
            None | Some(NumSolution::Unsolved) => {
                match &self.aliases[ext.0] {
                    a @ (Alias::Canonical(_) | Alias::Ground) => {
                        let sol = self.solve_standalone(ext)?;
                        // if ground, this iteration will be a no-op
                        {
                            // we have to collect the indices beforehand to avoid borrow-conflicts
                            let other_vars = a.iter_fwd_refs().map(ExtVar).collect::<Vec<_>>();
                            for other_var in other_vars {
                                self.solve_and_elide(other_var, sol)?;
                            }
                        }
                        self.outcomes.insert(ext, NumSolution::Solved(sol));
                        Ok(sol)
                    }
                    Alias::BackRef(can_ix) => {
                        let can = ExtVar(*can_ix);
                        match self.peek_solution(can) {
                            NumSolution::Elided => {
                                unreachable!("invalid state: elided solution on canonical alias")
                            }
                            NumSolution::Unsolved => {
                                // if the canonical alias has not been solved yet, we solve it and update all its forward-refs to Elided (if they agree on the same solution)
                                let sol = self.solve_standalone(ext)?;
                                // if ground, this iteration will be a no-op
                                {
                                    // we have to collect the indices beforehand to avoid borrow-conflicts
                                    let other_vars = self.aliases[*can_ix]
                                        .iter_fwd_refs()
                                        .map(ExtVar)
                                        .collect::<Vec<_>>();
                                    for other_var in other_vars {
                                        self.solve_and_elide(other_var, sol)?;
                                    }
                                }
                                self.outcomes.insert(ext, NumSolution::Solved(sol));
                                Ok(sol)
                            }
                            NumSolution::Solved(sol) => {
                                // since sol already exists, we assume all other aliases agree
                                self.solve_and_elide(ext, sol)?;
                                Ok(sol)
                            }
                        }
                    }
                }
            }
            Some(NumSolution::Elided) => {
                let can = self.get_canonical_extvar(ext);
                assert!(
                    can < ext,
                    "canonical alias {can} must be less than non-canonical {ext} (based on Elided)"
                );
                self.solve(can)
            }
            Some(NumSolution::Solved(sol)) => Ok(*sol),
        }
    }

    /// Performs standalone inference to determine the solution for `ext`.
    ///
    /// Should only be called when no outcome is registered, either on `ext` or its canonical alias.
    ///
    /// Does not mutate the resolver (i.e. no alias-updates or outcome-updates).
    fn solve_standalone(&self, ext: ExtVar) -> TCResult<crate::numeric::elaborator::IntType> {
        let engine = self.subtrees[ext.0].clone();
        engine
            .reify_err(UVar(0).into())
            .map_err(|e| TCErrorKind::Inference(ext, e).into())
    }

    /// Given a non-canonical ExtVar `ext` and a canonical-alias solution `sol`, performs the necessary
    /// checks and updates according to the following:
    ///
    ///   - If the outcome `ext` is already `Elided`, does nothing and returns early.
    ///   - If the outcome `ext` is not Solved, solves it.
    ///   - Once a non elided-solution for `ext` is found, ensures it is equal to `sol`, and then replaces with `Elided`.
    fn solve_and_elide(
        &mut self,
        ext: ExtVar,
        sol: crate::numeric::elaborator::IntType,
    ) -> TCResult<()> {
        let sol1 = match self.outcomes.get(&ext) {
            Some(NumSolution::Elided) => return Ok(()),
            Some(NumSolution::Unsolved) | None => self.solve_standalone(ext)?,
            Some(NumSolution::Solved(sol1)) => *sol1,
        };
        if sol == sol1 {
            self.outcomes.insert(ext, NumSolution::Elided);
            Ok(())
        } else {
            Err(TCErrorKind::IrreconcilableNumSolutions(
                NumSolution::Solved(sol),
                NumSolution::Solved(sol1),
            )
            .into())
        }
    }

    /// Returns the current solution-state for `ext`, or that of its canonical alias if it is a non-canonical alias
    /// with an elided solution.
    ///
    /// Does not mutate the resolver, perform any new inference, or enforce any constraints or alias-unifications.
    fn peek_solution(&self, ext: ExtVar) -> NumSolution {
        match self.outcomes.get(&ext) {
            None | Some(NumSolution::Unsolved) => NumSolution::Unsolved,
            Some(NumSolution::Elided) => {
                let can = self.get_canonical_extvar(ext);
                assert!(
                    can < ext,
                    "canonical alias {can} must be less than non-canonical {ext} (based on Elided)"
                );
                self.peek_solution(can)
            }
            Some(NumSolution::Solved(sol)) => NumSolution::Solved(*sol),
        }
    }

    /// Given an `ExtVar`, either retrieves a pre-computed solution, or solves the subtree it is
    /// associated with, storing the novel answer and returning it.
    ///
    /// Additionally returns a boolean value that indicates whether the solution was novel (i.e. whether
    /// it was solved as a result of this method's invocation).
    ///  - If `true`, the caller should perform any alias-unifications required byh novel solutions.
    ///  - If `false`, the solution was already recorded, and so no updates should be necessary.
    ///
    /// This function does not perform any alias unifications or updates; however,
    /// in the case of unsolved subtrees, this function will update the `outcomes` table
    /// accordingly, for the single-variable in question.
    ///
    /// Recurses if `ext` is a non-canonical alias with an `Elided` solution.
    fn ensure_outcome(&mut self, ext: ExtVar) -> TCResult<(NumSolution, bool)> {
        // FIXME - there are various checks being skipped, e.g. elided => canonical recursion must be solved, etc.
        let sol = self.outcomes.get(&ext).copied().unwrap_or_default();
        match sol {
            NumSolution::Elided => {
                let can = self.get_canonical_extvar(ext);
                assert!(
                    can < ext,
                    "canonical alias {can} must be less than non-canonical {ext} (based on Elided)"
                );
                self.ensure_outcome(can)
            }
            NumSolution::Unsolved => {
                let ret = self.solve_standalone(ext)?;
                let sol = NumSolution::Solved(ret);
                self.outcomes.insert(ext, sol);
                Ok((sol, true))
            }
            NumSolution::Solved(_) => return Ok((sol, false)),
        }
    }

    /// Immediately after the subtree corresponding to an `ExtVar` has been solved, ensure that
    /// the novel solution is compatible with the imputed solutions of any and all other ext-vars
    /// to which the original was aliased.
    ///
    /// Assumes that the association in `outcomes` has already been updated.
    #[expect(dead_code)]
    fn enforce_aliasing(&mut self, solved_var: ExtVar, sol: NumSolution) -> TCResult<()> {
        // FIXME - change type signature to accept Option<IntType> to avoid needless casework
        assert!(matches!(sol, NumSolution::Solved(_)));

        match &self.aliases[solved_var.0] {
            Alias::Ground => Ok(()),
            alias @ Alias::Canonical(_) => {
                for ix in alias.iter_fwd_refs() {
                    let other_ext = ExtVar(ix);
                    let outcome = self.outcomes.entry(other_ext).or_default();
                    match *outcome {
                        NumSolution::Unsolved => {
                            // NOTE - avoid over-eager evaluation of unsolved non-canonical ext-vars
                            continue;
                        }
                        NumSolution::Elided => continue,
                        other_sol @ NumSolution::Solved(_) => {
                            let (sol0, sol1) =
                                Self::reconcile_solution_pair((sol, other_sol), false)?;
                            assert_eq!(
                                sol, sol0,
                                "original solution to {solved_var} changed from {sol} to {sol0} during reconciliation with {other_sol}"
                            );
                            if other_sol != sol1 {
                                *outcome = sol1;
                            }
                        }
                    }
                }
                Ok(())
            }
            &Alias::BackRef(ix) => {
                match sol {
                    NumSolution::Elided | NumSolution::Unsolved => Ok(()),
                    sol_hi @ NumSolution::Solved(_) => {
                        // force the canonical ext-var to be solved, and reconcile with it
                        self.ensure_outcome(ExtVar(ix))?;
                        let sol_lo = self.outcomes.entry(ExtVar(ix)).or_default();
                        let (sol0, sol1) = Self::reconcile_solution_pair((*sol_lo, sol_hi), false)?;
                        // REVIEW[epic=embedded-num] - do we need to check on the values of sol0 and sol1 beyond equality?
                        if *sol_lo != sol0 {
                            *sol_lo = sol0;
                        }
                        if sol_hi != sol1 {
                            self.outcomes.entry(ExtVar(ix)).and_modify(|s| *s = sol1);
                        }
                        Ok(())
                    }
                }
            }
        }
    }
}
// !SECTION

// SECTION - methods for low-level management of ext-var alias-groups
impl EmbeddedResolver {
    /// Returns `true` if the ExtVar `var` is canonical for its alias-group.
    pub fn is_canonical(&self, ix: usize) -> bool {
        matches!(&self.aliases[ix], Alias::Canonical(_) | Alias::Ground)
    }

    /// Returns the canonical-among-aliased ExtVar for `ext_var`.
    ///
    /// # Notes
    ///
    /// This method is only suitable for lookups of root-solutions for mutual-satisfiability
    /// of External and natural constraints. It should not be used for elaboration purposes, as even two
    /// aliased ext-vars are not interchangeable when crawling their respective sub-trees for recursive elaboration.
    pub(crate) fn get_canonical_extvar(&self, ext_v: ExtVar) -> ExtVar {
        match self.aliases[ext_v.0] {
            Alias::Canonical(_) | Alias::Ground => ext_v,
            Alias::BackRef(ix) => ExtVar(ix),
        }
    }

    /// Introduces a new alias between the subtree-root ext-variables `var0` and `var1`.
    ///
    /// Returns the solution for the canonical ext-var in the resulting alias-group.
    ///
    /// Returns an error if the implied equivalence cannot be satisfied.
    fn resolve_alias(&mut self, v1: ExtVar, v2: ExtVar) -> TCResult<NumSolution> {
        if v1 == v2 {
            let sol = *self.outcomes.entry(v1).or_default();
            return Ok(sol);
        }

        match (&self.aliases[v1.0], &self.aliases[v2.0]) {
            (Alias::Ground, Alias::Ground) => {
                /*
                 * Ground-Ground: simple repoint that makes the lower-indexed ext-var canonical,
                 * and ensures that their outcomes are synchronized.
                 */
                let (lo, hi) = min_max(v1.0, v2.0);
                unsafe { self.insert_ground(lo, hi) }
            }
            (Alias::Ground, &Alias::BackRef(can_ix)) => {
                /*
                 * x = Ground, y = BackRef(z):
                 *   if x > z, then insert x into the alias-group of z and synchronize outcomes of z and x
                 *   if x < z, then have x inherit canonical status of z using recanonicalize
                 *   (panic if x = z: impossible because x-|<-y would then be only half-aliased)
                 */
                if v1.0 > can_ix {
                    let lo = can_ix;
                    let hi = v1.0;
                    unsafe { self.insert_ground(lo, hi) }
                } else if v1.0 < can_ix {
                    let lo = v1.0;
                    let hi = can_ix;
                    debug_assert!(
                        self.aliases[hi].is_canonical_nonempty(),
                        "half-alias ?{hi}-|<-{v2}"
                    );
                    debug_assert!(
                        !self.aliases[hi].contains_fwd_ref(lo),
                        "retrograde half-aliased 'forward' ref ?{hi}->|-{v1}"
                    );
                    unsafe { self.recanonicalize(lo, hi) }
                } else {
                    // we can only get here if v1.0 == can_ix, but this is not valid as v1 should then be Canonical and not Ground
                    unreachable!("unexpected half-alias {v1}-|<-{v2}");
                }
            }
            (&Alias::BackRef(can_ix), Alias::Ground) => {
                /*
                 * x = BackRef(z), y = Ground:
                 *   if y > z, then insert y into the alias-group of z and synchronize outcomes of z and y
                 *   if y < z, then have y inherit canonical status of z using recanonicalize
                 *   (panic if y = z: impossible because y-|<-x would then be only half-aliased)
                 */
                if v2.0 > can_ix {
                    let lo = can_ix;
                    let hi = v2.0;
                    unsafe { self.insert_ground(lo, hi) }
                } else if v2.0 < can_ix {
                    let lo = v2.0;
                    let hi = can_ix;
                    debug_assert!(
                        self.aliases[hi].is_canonical_nonempty(),
                        "half-alias ?{hi}-|<-{v1}"
                    );
                    debug_assert!(
                        !self.aliases[hi].contains_fwd_ref(lo),
                        "retrograde half-aliased 'forward' ref ?{hi}->|-{v2}"
                    );
                    unsafe { self.recanonicalize(lo, hi) }
                } else {
                    unreachable!("unexpected half-alias {v2}-|<-{v1}");
                }
            }
            (Alias::Ground, Alias::Canonical(_)) => {
                /*
                 * x = Ground, y = Canonical:
                 *   if x < y, recanonicalize x for the alias-group of y and insert y into it, after synchronizing outcomes.
                 *   if x > y, simply insert x into the alias-group for y without any shifts, and synchronize outcomes
                 */
                if v1.0 < v2.0 {
                    let lo = v1.0;
                    let hi = v2.0;
                    debug_assert!(
                        !self.aliases[hi].contains_fwd_ref(lo),
                        "retrograde half-aliased 'forward' ref {v2}->|-{v1}"
                    );
                    unsafe { self.recanonicalize(lo, hi) }
                } else {
                    // v2 < v1
                    let lo = v2.0;
                    let hi = v1.0;
                    unsafe { self.insert_ground(lo, hi) }
                }
            }
            (Alias::Canonical(_), Alias::Ground) => {
                /*
                 * x = Canonical, y = Ground:
                 *   if x > y, recanonicalize y for the alias-group of x and insert x into it, after synchronizing outcomes.
                 *   if x < y, simply insert y into the alias-group for x without any shifts, and synchronize outcomes
                 */
                if v2.0 < v1.0 {
                    let lo = v2.0;
                    let hi = v1.0;
                    debug_assert!(
                        !self.aliases[v1.0].contains_fwd_ref(v2.0),
                        "retrograde half-aliased 'forward' ref {v1}->|-{v2}"
                    );
                    unsafe { self.recanonicalize(lo, hi) }
                } else {
                    let lo = v1.0;
                    let hi = v2.0;
                    unsafe { self.insert_ground(lo, hi) }
                }
            }
            (&Alias::BackRef(ix1), &Alias::BackRef(ix2)) => {
                /*
                 * x = BackRef(z), y = BackRef(w):
                 *   if z < w, recanonicalize z for the alias-group of w
                 *   if z > w, recanonicalize w for the alias-group of z
                 *   if z = w, check that the alias-group is well-formed and return the outcome for z
                 */
                if ix1 == ix2 {
                    // z = w, so x and y are in the same alias-group
                    let common = &self.aliases[ix1];
                    debug_assert!(
                        common.contains_fwd_ref(v1.0),
                        "unexpected half-alias ?{ix1}<-{v1}"
                    );
                    debug_assert!(
                        common.contains_fwd_ref(v2.0),
                        "unexpected half-alias ?{ix1}<-{v2}"
                    );
                    let v0 = ExtVar(ix1);
                    let Some(ret) = self.outcomes.get(&v0) else {
                        // FIXME - in this case, do we want to insert an Unsolved outcome instead?
                        unreachable!("canonical ext-var {v0} has no registered outcome");
                    };
                    Ok(*ret)
                } else {
                    let (lo, hi) = if ix1 < ix2 { (ix1, ix2) } else { (ix2, ix1) };
                    unsafe { self.recanonicalize(lo, hi) }
                }
            }
            (a1 @ Alias::BackRef(tgt), a2 @ Alias::Canonical(fwd_refs)) => {
                /*
                 * x = BackRef(z), y = Canonical(S):
                 *   - if x ∈ S and y = z, return the outcome for y
                 *   - if x ∉ s and y != z, recanonicalize the lesser of y and z for the alias-group of the greater
                 *   - otherwise, panic due to inconsistent back- and forward-references
                 */
                let is_forward_aliased = fwd_refs.contains(&v1.0);
                let is_backward_aliased = *tgt == v2.0;

                match (is_forward_aliased, is_backward_aliased) {
                    (true, true) => {
                        return Ok(*self.outcomes.get(&v2).unwrap());
                    }
                    (false, false) => {
                        let z = *tgt;
                        let y = v2.0;

                        let (lo, hi) = if z < y { (z, y) } else { (y, z) };
                        unsafe { self.recanonicalize(lo, hi) }
                    }
                    _ => unreachable!(
                        "mismatched back- and forward-references for {v1} ({a1:?}) and {v2} ({a2:?})"
                    ),
                }
            }
            (a1 @ Alias::Canonical(fwd_refs), a2 @ Alias::BackRef(tgt)) => {
                /*
                 * x = Canonical(S), y = BackRef(z):
                 *   - if y ∈ S and x = z, return the outcome for x
                 *   - if y ∉ s and x != z, recanonicalize the lesser of x and z for the alias-group of the greater
                 *   - otherwise, panic due to inconsistent back- and forward-references
                 */
                let is_forward_aliased = fwd_refs.contains(&v2.0);
                let is_backward_aliased = *tgt == v1.0;

                match (is_forward_aliased, is_backward_aliased) {
                    (true, true) => {
                        return Ok(*self.outcomes.get(&v1).unwrap());
                    }
                    (false, false) => {
                        let x = v1.0;
                        let z = *tgt;

                        let (lo, hi) = min_max(x, z);
                        unsafe { self.recanonicalize(lo, hi) }
                    }
                    _ => unreachable!(
                        "mismatched forward- and back-references for {v1} ({a1:?}) and {v2} ({a2:?})"
                    ),
                }
            }
            (Alias::Canonical(_), Alias::Canonical(_)) => {
                /*
                 * x = Canonical(S), y = Canonical(T): recanonicalize the lesser of x and y over the union of S and T
                 */
                let (lo, hi) = min_max(v1.0, v2.0);
                unsafe { self.recanonicalize(lo, hi) }
            }
        }
    }

    /// Rewrites the aliasing of `self` so that `lo<->hi` is enforced, where `lo` is canonical for its
    /// alias-group and `hi` is ground-aliased, and `lo < hi`.
    ///
    /// Additionally synchronizes the outcomes between `lo` and `hi`.
    ///
    /// # Safety
    ///
    /// This method is not known to cause UB, but it is an internal-facing call that may lead to
    /// corrupted alias-state if called in a context that does not check certain preconditions.
    ///
    /// In particular, these preconditions are: `lo < hi`, `lo` must be canonical, and `hi` must be ground-aliased.
    unsafe fn insert_ground(&mut self, lo: usize, hi: usize) -> TCResult<NumSolution> {
        unsafe {
            self.repoint(lo, hi);
            self.synchronize_outcomes(lo, hi)
        }
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
    /// Guards are enforced in advance.
    unsafe fn repoint(&mut self, lo: usize, hi: usize) {
        self.aliases[hi].set_backref(lo);
        self.aliases[lo].add_forward_ref(hi);
    }

    /// Modifies the aliasing table of `self` to merge the aliasing-groups given by two
    /// canonical indices `lo` and `hi`, where `lo` < `hi`; the new group's canonical element
    /// will be `lo`, whose forward references will include:
    ///     - All existing forward-references for the original alias-group of `lo`
    ///     - All existing forward-references for the original alias-group of `hi`
    ///     - The newly de-canonicalized `hi`
    ///
    /// Also ensures that all back-references of `hi` now point to `lo` instead, as well as that
    /// `hi` itself becomes a back-ref to `lo`.
    ///
    /// In addition to fixing the aliasing-table, also synchronizes outcomes between `lo` and `hi`,
    /// taken as representative members of the notionally-common solutions for the members of their
    /// respective former aliasing groups.
    ///
    /// # Panics
    ///
    /// Will panic if the aliasing-table is in an inconsistent state at the time of call
    /// (specifically, if the aliasing-groups of `lo` and `hi` have a non-empty intersection),
    /// or otherwise under the same conditions that [`Self::synchronize_outcomes`] panics:
    /// `lo >= hi` or `lo` is non-canonical.
    ///
    /// # Safety
    ///
    /// As this method is designed to be internal with a specific singular use-case, there are a number of preconditions that must either be
    /// assumed or asserted, in order to ensure that the call is sound and valid. These preconditions are numerous enough to merit unsafe status for
    /// the call to this method, at least as a temporary linting-helper, to avoid unguarded calls from neutral contexts.
    ///
    /// Preconditions:
    /// - `lo < hi`
    /// - `lo` has no back-references; it is allowed (but not required) to have forward-references
    /// - `hi` has no back-references; it is allowed (but not required) to have forward-references
    unsafe fn recanonicalize(&mut self, lo: usize, hi: usize) -> TCResult<NumSolution> {
        // tmp is the old aliasing-group of `hi` after overwriting `hi` to back-reference `lo`
        let tmp = self.aliases[hi].set_backref(lo);
        let iter = tmp.iter_fwd_refs();
        for a in iter {
            // double-checks that none of the members of the previous aliasing-group of `hi` are included in the aliasing-group of `lo`
            assert!(
                !self.aliases[lo].contains_fwd_ref(a),
                "forward ref of ?{hi} is also a forward ref of ?{lo}, somehow"
            );
            unsafe { self.repoint(lo, a) };
        }
        self.aliases[lo].add_forward_ref(hi);
        unsafe { self.synchronize_outcomes(lo, hi) }
    }

    /// Modifies internal state to establish all expected post-conditions of the novel aliasing `lo->|<-hi`.
    ///
    /// Specifically, will adjust the entries for `lo` and `hi` in `self.outcomes` such that, if both are solved,
    /// the two solutions are unified (and will return Err if their unification is unsatisfiable).
    ///
    /// If `lo` is unsolved and `hi` is solved, will force the solution of `lo` and peform the same unification, after
    /// which (if not Err) the entry for `hi` is elided and the entry for `lo` is updated (and returned).
    ///
    /// If `lo` is solved and `hi` is unsolved, will merely return the entry for `lo` and leave reconciliation of the
    /// deferred solution of `hi` to be handled when the solution is manifested at a later time.
    ///
    /// If `lo` is elided, or if `hi` is elided, will panic.
    ///
    /// The return-value is the outcome of the ExtVar with index `lo`, after enforcing its unification with `hi`.
    ///
    /// Preconditions:
    ///   - `lo < hi`
    ///   - Neither `lo` nor `hi` have elided solutions
    ///   - `lo` is canonical
    ///   - `hi` was formerly canonical but has been repointed to `lo`.
    ///   - All previous members of the alias-group of `lo` have compatible solution-states with that of `lo`
    unsafe fn synchronize_outcomes(&mut self, lo: usize, hi: usize) -> TCResult<NumSolution> {
        assert!(lo < hi);
        assert!(self.is_canonical(lo));

        // ensure that the canonical ExtVar `#lo` does not have an unsolved subtree
        let (_sol, _is_novel) = self.ensure_outcome(ExtVar(lo))?;

        // If no solution has been inserted for `hi` yet, create an entry for it storing `NumSolution::Unsolved`.
        {
            let _hi_entry = self.outcomes.entry(ExtVar(hi)).or_default();
        }
        let (lo_outcome, hi_outcome) =
            match self.outcomes.get_disjoint_mut([&ExtVar(lo), &ExtVar(hi)]) {
                [Some(lo_outcome), Some(hi_outcome)] => (lo_outcome, hi_outcome),
                [None, _] => unreachable!("[BUG]: force_outcome failed to create entry for #{lo}"),
                [_, None] => unreachable!("[BUG]: failed to create entry for #{hi}"),
            };

        let (new_sol_lo, new_sol_hi) =
            Self::reconcile_solution_pair((*lo_outcome, *hi_outcome), false)?;
        *lo_outcome = new_sol_lo;
        *hi_outcome = new_sol_hi;

        Ok(new_sol_lo)
    }
}
// !SECTION

// SECTION - low level solution-unification
impl EmbeddedResolver {
    /// Given a pair of `NumSolution`s that correspond to two aliased `ExtVar`s,
    /// returns a pair of mutually satisfiable `NumSolution`s to replace the original pair.
    ///
    /// It is implicitly assumed that the first `NumSolution` corresponds to the lower-indexed `ExtVar`
    /// of a co-aliased pair, which is the canonical `ExtVar` in its aliasing group.
    ///
    /// # Panics
    ///
    /// The boolean flag `permit_elision` controls how the function handles `NumSolution::Elided` in the second pair-element.
    /// - If `true`, will treat `NumSolution::Elided` as `NumSolution::Unsolved`
    /// - If `false`, this method will panic.
    ///
    /// This method will always panic if the first pair-element is `NumSolution::Elided`.
    ///
    /// It will also panic if the first-pair-element is `NumSolution::Unsolved` (unless the second element is also `NumSolution::Unsolved`), which should be precluded by the caller
    /// through [`EmbeddedResolver::force_outcome`].
    fn reconcile_solution_pair(
        sols: (NumSolution, NumSolution),
        permit_elision: bool,
    ) -> TCResult<(NumSolution, NumSolution)> {
        let (orig_lo, orig_hi) = match sols {
            (NumSolution::Solved(sol_lo), NumSolution::Solved(sol_hi)) => (sol_lo, sol_hi),
            (NumSolution::Unsolved, NumSolution::Unsolved) => {
                // REVIEW[epic=embedded-num] - is it proper to return the same solutions, or do we want to enforce the constraint that `sol.0` should at least be solved?
                return Ok(sols);
            }
            (NumSolution::Elided, _) => unreachable!(
                "reconcile_solution_pair: canonical ExtVar should not have elided solution"
            ),
            (_, NumSolution::Elided) if !permit_elision => {
                panic!(
                    "reconcile_solution_pair: unexpected elided solution for non-canonical ExtVar with permit_elision=false"
                );
            }
            (NumSolution::Solved(_), NumSolution::Unsolved | NumSolution::Elided) => {
                // NOTE[epic=embedded-num] - rather than solve `hi` here, we leave the alias-unification to be solved later once we request the solution for `hi`.
                return Ok(sols);
            }
            (NumSolution::Unsolved, sol_hi) => {
                panic!(
                    "reconcile_solution_pair: canonical ext-var is unsolved, but non-canonical ext-var is solved with {sol_hi}"
                );
            }
        };

        if orig_lo == orig_hi {
            Ok((NumSolution::Solved(orig_lo), NumSolution::Elided))
        } else {
            Err(TCErrorKind::IrreconcilableNumSolutions(
                NumSolution::Solved(orig_lo),
                NumSolution::Solved(orig_hi),
            )
            .into())
        }
    }
}
// !SECTION

/// Mutably updated state-engine for performing complete type-inference on a top-level `Format`.
#[derive(Debug)]
pub struct TypeChecker {
    // TODO - implement segmented store to keep dep-format solving from interfering with proper sequencing
    /// Stores, at each index `ix`, the incrementally refined constraints on the types that are valid assignments to meta-variable `?ix`
    constraints: Vec<Constraints>,
    /// Stores, at each index `ix`, the set of unique meta-variables that are aliased to meta-variable `?ix` (excluding `?ix` itself)
    aliases: Vec<Alias>,
    /// Associations between VMId in meta-variable constraints and the incrementally refined VarMap for the union-type it corresponds to
    varmaps: VarMapMap,
    /// Association between ItemVar/FormatRef levels in the FormatModule and the UVar they are mapped to
    level_vars: HashMap<usize, UVar>,
    /// Scaffolding for compartmentalized external type inference in the arithmetic extension grammar
    sub_extension: EmbeddedResolver,
}

// SECTION - Construction and instantiation in the meta-context
impl TypeChecker {
    /// Constructs a typechecker with initially 0 meta-variables.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            aliases: Vec::new(),
            varmaps: VarMapMap::new(),
            level_vars: HashMap::new(),
            sub_extension: EmbeddedResolver::new(),
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
                    Rc::new(UType::seq(Rc::new(UType::Var(elem_uvar)))),
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
    ///
    /// # Panics
    ///
    /// Will only ever panic if `t` is an infinite type.
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

    fn infer_var_view_format(
        &mut self,
        view_format: &ViewFormat,
        ctxt: Ctxt<'_>,
    ) -> TCResult<UVar> {
        match view_format {
            ViewFormat::CaptureBytes(len) => {
                let newvar = self.get_new_uvar();
                let len_var = self.infer_var_expr(len, ctxt.scope)?;
                self.unify_var_baseset(len_var, BaseSet::U(UintSet::ANY))?;
                // REVIEW - should we have a special UType for captured View-window reads?
                self.unify_var_utype(
                    newvar,
                    Rc::new(UType::seq_view(Rc::new(UType::Base(BaseType::U8)))),
                )?;
                Ok(newvar)
            }
            ViewFormat::ReadArray(len, kind) => {
                let newvar = self.get_new_uvar();
                let len_var = self.infer_var_expr(len, ctxt.scope)?;
                self.unify_var_baseset(len_var, BaseSet::U(UintSet::ANY))?;
                // REVIEW - should we have a special UType for captured View-window reads?
                self.unify_var_utype(
                    newvar,
                    // REVIEW - how do we distinguish CaptureBytes (seq_view ~> &'a [u8]) from ReadArray (seq_view ~> ReadArray<'a, K>)?
                    Rc::new(UType::seq_array(Rc::new(UType::Base(BaseType::from(
                        *kind,
                    ))))),
                )?;
                Ok(newvar)
            }
            ViewFormat::ReifyView => {
                let (newvar, _) = self.init_var_simple(UType::ViewObj)?;
                Ok(newvar)
            }
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

    fn traverse_view_expr(&mut self, view: &ViewExpr, ctxt: Ctxt<'_>) -> TCResult<()> {
        match view {
            ViewExpr::Var(ident) => {
                if ctxt.views.includes_name(ident) {
                    Ok(())
                } else {
                    Err(TCError::from(TCErrorKind::MissingView(ident.clone())))
                }
            }
            ViewExpr::Offset(base, offs) => {
                self.traverse_view_expr(base.as_ref(), ctxt)?;
                let v_offs = self.infer_var_expr(offs, ctxt.scope)?;
                self.unify_var_baseset(v_offs, BaseSet::U(UintSet::ANY))?;
                Ok(())
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
                    ProjShape::OptOf(param_v) => self.occurs_in(v, Rc::<UType>::from(*param_v)),
                },
            },
        }
    }

    /// Performs an 'occurs-check' that determines if a variable `v` occurs in a [`UType`] `t`, used
    /// for detecting infinite types.
    fn occurs_in(&self, v: UVar, t: impl AsRef<UType>) -> TCResult<()> {
        match t.as_ref() {
            UType::Hole | UType::Empty | UType::ViewObj | UType::ExternVar(..) | UType::Base(_) => {
                Ok(())
            }
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
            UType::Seq(inner, _) | UType::Option(inner) | UType::PhantomData(inner) => {
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
                        unreachable!(
                            "HashMap::get returned None for {vmid} even though assertion on HashMap::contains_key succeeded"
                        )
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

    /// Establishes `?i ~ Opt(?j)` through projective unification, where `opt_v` is the lhs
    /// and `param_v` is the rhs metavariable.
    fn unify_var_proj_param(&mut self, opt_v: UVar, param_v: UVar) -> TCResult<()> {
        let can_v = self.get_canonical_uvar(opt_v);
        match &mut self.constraints[can_v.0] {
            Constraints::Indefinite => {
                let proj = ProjShape::opt_of(param_v);
                self.unify_var_constraint(opt_v, Constraint::Proj(proj))?;
                Ok(())
            }
            Constraints::Variant(_) => unreachable!("cannot solve param projection on union"),
            Constraints::Invariant(c) => match c {
                Constraint::Elem(_) => unreachable!("cannot solve param projection on base-set"),
                Constraint::Equiv(ut) => match ut.as_ref() {
                    UType::Option(inner) => {
                        let param_t = inner.clone();
                        self.unify_var_utype(param_v, param_t)?;
                        Ok(())
                    }
                    other => unreachable!("expected UType::Option, found {other:?}"),
                },
                Constraint::Proj(ps) => match ps {
                    ProjShape::OptOf(other_var) => {
                        let tmp = *other_var;
                        self.unify_var_pair(param_v, tmp)?;
                        Ok(())
                    }
                    _ => unreachable!("cannot unify on parameter type of non-opt projection"),
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
            Constraints::Variant(_) => unreachable!("cannot solve elem projection on union"),
            Constraints::Invariant(c) => match c {
                Constraint::Elem(_) => unreachable!("cannot solve elem projection on base-set"),
                Constraint::Equiv(ut) => match ut.as_ref() {
                    UType::Seq(inner, _) => {
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
            ProjShape::SeqOf(v) => VType::Abstract(Rc::new(UType::seq((*v).into()))),
            ProjShape::OptOf(v) => VType::Abstract(Rc::new(UType::opt((*v).into()))),
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
                log::warn!("Hole-Hole unification");
                Ok(left)
            }
            (UType::Hole, _) => Ok(right),
            (_, UType::Hole) => Ok(left),
            (UType::Empty, _) => Ok(right),
            (_, UType::Empty) => Ok(left),
            (UType::ViewObj, UType::ViewObj) => Ok(left),
            (UType::Seq(e1, h1), UType::Seq(e2, h2)) => {
                if e1 == e2 {
                    if h1 <= h2 { Ok(left) } else { Ok(right) }
                } else {
                    let inner = self.unify_utype(e1.clone(), e2.clone())?;
                    Ok(Rc::new(UType::Seq(inner, Ord::min(*h1, *h2))))
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
            (UType::PhantomData(p1), UType::PhantomData(p2)) => {
                if p1 == p2 {
                    Ok(left)
                } else {
                    let inner = self.unify_utype(p1.clone(), p2.clone())?;
                    Ok(Rc::new(UType::PhantomData(inner)))
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
            (&UType::ExternVar(ext1), &UType::ExternVar(ext2)) => {
                self.unify_extvar_pair(ext1, ext2)?;
                Ok(Rc::new(UType::ExternVar(Ord::min(ext1, ext2))))
            }
            (&UType::Var(v), &UType::ExternVar(ext)) | (&UType::ExternVar(ext), &UType::Var(v)) => {
                self.unify_var_extvar(v, ext)
            }
            (&UType::ExternVar(ext), _) => self.unify_extvar_utype(ext, right.clone()),
            (_, &UType::ExternVar(ext)) => self.unify_extvar_utype(ext, left.clone()),
            (&UType::Var(v), _) => {
                let constraint = Constraint::Equiv(right.clone());
                let _ = self.unify_var_constraint(v, constraint)?;
                self.occurs(v)?;
                Ok(Rc::new(UType::Var(v)))
            }
            (_, &UType::Var(v)) => {
                let constraint = Constraint::Equiv(left.clone());
                let _ = self.unify_var_constraint(v, constraint)?;
                self.occurs(v)?;
                Ok(Rc::new(UType::Var(v)))
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
            UType::ExternVar(ext_v) => self.unify_extvar_baseset_as_constraint(*ext_v, bs),
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

    /// Handles case-logic for Constraint-level unification of `Equiv(ExternVar(ext_v))` and `Elem(bs)`.
    ///
    /// Returns a `Constraint` if the unification is sound, or an `Err` otherwise.
    fn unify_extvar_baseset_as_constraint(
        &mut self,
        ext_v: ExtVar,
        bs: BaseSet,
    ) -> TCResult<Constraint> {
        let IntType::Prim(pt) = self.resolve_extvar(ext_v)?;
        if let Ok(bt) = pt.try_into()
            && bs.contains(bt)
        {
            Ok(Constraint::Equiv(Rc::new(UType::Base(bt))))
        } else {
            Err(TCErrorKind::UnexpectedSolution(ext_v, pt, bs).into())
        }
    }

    /// Attempts to unify a [`PrimInt`] with a [`BaseType`], returning the unified [`UType`] if successful.
    ///
    /// Returns an `Err` if the unification is not possible.
    ///
    /// # Notes
    ///
    /// For error-tracking purposes, the caller may wish to attach extra context using
    /// [`TCError::with_trace`] to indicate the `ExtVar` or `UVar` whose unification
    /// is being attempted.
    fn unify_primint_basetype(prim: PrimInt, base: BaseType) -> TCResult<Rc<UType>> {
        let reason = match base {
            BaseType::Char | BaseType::Bool => CrossLayerBadnessReason::NonNumeric,
            BaseType::U8 => match prim {
                PrimInt::U8 => return Ok(Rc::new(UType::Base(BaseType::U8))),
                _ => {
                    if prim.is_signed() {
                        CrossLayerBadnessReason::Unsupported
                    } else {
                        CrossLayerBadnessReason::Mismatched
                    }
                }
            },
            BaseType::U16 => match prim {
                PrimInt::U16 => return Ok(Rc::new(UType::Base(BaseType::U16))),
                _ => {
                    if prim.is_signed() {
                        CrossLayerBadnessReason::Unsupported
                    } else {
                        CrossLayerBadnessReason::Mismatched
                    }
                }
            },
            BaseType::U32 => match prim {
                PrimInt::U32 => return Ok(Rc::new(UType::Base(BaseType::U32))),
                _ => {
                    if prim.is_signed() {
                        CrossLayerBadnessReason::Unsupported
                    } else {
                        CrossLayerBadnessReason::Mismatched
                    }
                }
            },
            BaseType::U64 => match prim {
                PrimInt::U64 => return Ok(Rc::new(UType::Base(BaseType::U64))),
                _ => {
                    if prim.is_signed() {
                        CrossLayerBadnessReason::Unsupported
                    } else {
                        CrossLayerBadnessReason::Mismatched
                    }
                }
            },
        };
        Err(TCErrorKind::BadEquivalence { prim, base, reason }.into())
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
                ValueType::Option(inner) => {
                    let param_v = self.get_new_uvar();
                    self.unify_var_proj_param(uv, param_v)?;
                    self.unify_var_valuetype(param_v, inner)?;
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
            (
                Constraint::Proj(ProjShape::OptOf(param_var1)),
                Constraint::Proj(ProjShape::OptOf(param_var2)),
            ) => {
                self.unify_var_pair(param_var1, param_var2)?;
                Ok(Constraint::Proj(ProjShape::OptOf(param_var1)))
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
                    (ProjShape::SeqOf(elem_v), UType::Seq(elem_t, _)) => {
                        self.unify_var_utype(*elem_v, elem_t.clone())?;
                        Ok(Constraint::Equiv(ut.clone()))
                    }
                    (ProjShape::OptOf(param_v), UType::Option(param_t)) => {
                        self.unify_var_utype(*param_v, param_t.clone())?;
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
                    (ProjShape::OptOf(_), other) => {
                        unreachable!("could not match Opt-Shape against {other:?}")
                    }
                }
            }
            (
                ref c1 @ Constraint::Proj(ProjShape::RecordWith(..)),
                ref c2 @ Constraint::Proj(
                    ProjShape::SeqOf(..) | ProjShape::TupleWith(..) | ProjShape::OptOf(..),
                ),
            )
            | (
                ref c1 @ Constraint::Proj(ProjShape::TupleWith(..)),
                ref c2 @ Constraint::Proj(
                    ProjShape::SeqOf(..) | ProjShape::RecordWith(..) | ProjShape::OptOf(..),
                ),
            )
            | (
                ref c1 @ Constraint::Proj(ProjShape::SeqOf(..)),
                ref c2 @ Constraint::Proj(
                    ProjShape::TupleWith(..) | ProjShape::RecordWith(..) | ProjShape::OptOf(..),
                ),
            )
            | (
                ref c1 @ Constraint::Proj(ProjShape::OptOf(..)),
                ref c2 @ Constraint::Proj(
                    ProjShape::TupleWith(..) | ProjShape::RecordWith(..) | ProjShape::SeqOf(..),
                ),
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

    /// Computes or retrieves a stored solution for an `ExtVar`, as an `IntType`.
    fn resolve_extvar(&mut self, ext_v: ExtVar) -> TCResult<IntType> {
        self.sub_extension.solve(ext_v)
    }

    /// Computes or retrieves a stored solution for an `ExtVar`, as an `UType`.
    ///
    /// In addition to standard error-conditions from ExtVar resolution, this method
    /// will also fail if the resulting solution cannot be directly modelled as a UType
    /// (i.e. it is a signed PrimInt).
    fn resolve_extvar_as_utype(&mut self, ext_v: ExtVar) -> TCResult<Rc<UType>> {
        let sub = &mut self.sub_extension;
        let IntType::Prim(prim) = sub.solve(ext_v)?;
        let bt = match prim {
            PrimInt::U8 => BaseType::U8,
            PrimInt::U16 => BaseType::U16,
            PrimInt::U32 => BaseType::U32,
            PrimInt::U64 => BaseType::U64,
            PrimInt::I8 | PrimInt::I16 | PrimInt::I32 | PrimInt::I64 => {
                return Err(TCErrorKind::UTypeFromSigned(ext_v, prim).into());
            }
        };
        Ok(Rc::new(UType::Base(bt)))
    }

    /// If `v` has a direct-equality constraint against an `ExtVar`, return that `ExtVar`.
    /// Returns `None` in all other cases.
    fn expand_var_to_extvar(&self, v: UVar) -> Option<ExtVar> {
        let v0 = self.get_canonical_uvar(v);

        if let Constraints::Invariant(c) = &self.constraints[v0.0]
            && let Constraint::Equiv(t) = c
            && let UType::ExternVar(ext) = t.as_ref()
        {
            Some(*ext)
        } else {
            None
        }
    }

    /// Performs unification of a standard-model metavariable `v` against a numeric-extension metavariable `ext`.
    ///
    /// If `v` already points to a numeric-extension metavariable, performs numeric-layer metavariable unification.
    /// Otherwise, forces a solution to `ext` (and coerces it to be a UType), and unifies `v` against that solution.
    ///
    /// In either case, if successful, returns the original metavariable `v` as a `UType`.
    fn unify_var_extvar(&mut self, v: UVar, ext: ExtVar) -> TCResult<Rc<UType>> {
        if let Some(ext0) = self.expand_var_to_extvar(v) {
            self.unify_extvar_pair(ext0, ext)?;
        } else {
            let typ = self.resolve_extvar_as_utype(ext)?;
            self.unify_var_utype(v, typ)?;
        }
        Ok(Rc::new(UType::ExternVar(ext)))
    }

    /// Internal function for unifying non-variable UTypes against extvars.
    ///
    /// To maintain elaborator state, the value eturned will always be the original `extvar`, presuming
    /// that unification succeeds.
    ///
    /// # Panics
    ///
    /// Will panic if `utype` is either `Var` or `ExternVar`.
    fn unify_extvar_utype(&mut self, ext: ExtVar, utype: Rc<UType>) -> TCResult<Rc<UType>> {
        match utype.as_ref() {
            UType::ExternVar(..) => unreachable!("unify_extvar_utype called with UType::ExternVar"),
            UType::Var(..) => unreachable!("unify_extvar_utype called with UType::Var"),
            UType::Empty | UType::Hole => Ok(Rc::new(UType::ExternVar(ext))),

            // REVIEW - Bool and Char are also handled by `unify_primint_basetype`, but the error-variant will differ in that case
            UType::Base(BaseType::Bool | BaseType::Char)
            | UType::ViewObj
            | UType::Tuple(..)
            | UType::Record(..)
            | UType::Seq(..)
            | UType::Option(..)
            | UType::PhantomData(..) => Err(TCError::from(
                TCErrorKind::NonNumericExtVarUnification(ext, utype),
            )),

            UType::Base(btype) => {
                let IntType::Prim(ptype) = self.resolve_extvar(ext)?;
                let _ret = try_with!(Self::unify_primint_basetype(ptype, *btype) => ext);
                Ok(Rc::new(UType::ExternVar(ext)))
            }
        }
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
            Expr::Numeric(expr) => {
                let newvar = self.get_new_uvar();

                let mut engine = inference::InferenceEngine::new();

                // create a dummy extvar for error reporting, which we don't end up using elsewhere
                let _ext_var = ExtVar(self.sub_extension.subtrees.len());

                // WIP - figure out what to do with Rep
                let (sub_var, _rep) = try_with!(engine
                    .infer_var_expr(expr)
                    .map_err(|e| TCErrorKind::Inference(_ext_var, e)) => format!("failed inference on {newvar} @ Numeric({expr:?})"));
                assert_eq!(
                    sub_var,
                    UVar(0),
                    "expected sub-var ({newvar} = {_ext_var}) to be ?0, found {sub_var}"
                );

                let ext_var = self.sub_extension.init_new_extvar(engine);
                self.unify_var_utype(newvar, Rc::new(UType::ExternVar(ext_var)))?;

                newvar
            }
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
                // FIXME - to allow empty-seq to not clobber views, we use a slight hack here
                if elems.is_empty() {
                    self.unify_var_utype(
                        seq_uvar,
                        Rc::new(UType::seq_view(Rc::new(UType::Var(elem_uvar)))),
                    )?;
                } else {
                    self.unify_var_utype(
                        seq_uvar,
                        Rc::new(UType::seq(Rc::new(UType::Var(elem_uvar)))),
                    )?;
                }
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
            Expr::Destructure(head, pattern, rhs_expr) => {
                let newvar = self.get_new_uvar();
                let head_t = self.infer_utype_expr(head, scope)?;
                self.unify_utype_expr_match_case(head_t, pattern, newvar, rhs_expr, scope)?;
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
            Expr::Append(seq0, seq1) => {
                let newvar = self.get_new_uvar();
                let seq0_var = self.infer_var_expr(seq0.as_ref(), scope)?;
                let seq1_var = self.infer_var_expr(seq1.as_ref(), scope)?;

                let elem_var = self.get_new_uvar();

                self.unify_var_pair(seq0_var, seq1_var)?;
                self.unify_var_proj_elem(seq0_var, elem_var)?;
                self.unify_var_proj_elem(newvar, elem_var)?;

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
    /// Performs re-aliasing, recanonicalization, and any other constraint propagation as necessary,
    /// to establish direct equality requirements between two external variables
    ///
    /// Returns the canonical external variable that both `ext1` and `ext2` should now be aliased
    /// with.
    fn unify_extvar_pair(&mut self, ext1: ExtVar, ext2: ExtVar) -> TCResult<ExtVar> {
        self.sub_extension.resolve_alias(ext1, ext2)?;
        Ok(self.sub_extension.get_canonical_extvar(ext1))
    }

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
                    (true, false) | (false, true) => unreachable!(
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
                    (true, false) | (false, true) => unreachable!(
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
    /// As this method is designed to be internal with a specific singular use-case, there are a number of preconditions that must either be
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
            unsafe { self.repoint(a1, a) };
        }
        self.aliases[a1].add_forward_ref(a2);
        unsafe { self.transfer_constraints(a1, a2) }
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

    /// Ensures that all constraints on `a2` are inherited by `a1`, with the reverse occurring as a side-effect.
    ///
    /// # Safety
    ///
    /// While no inherently unsafe functions are called, the caller must ensure that certain preconditions are met,
    ///
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

    /// Public interface for [`SubExtension::get_canonical_extvar`].
    pub fn get_canonical_extvar(&self, ext_v: ExtVar) -> ExtVar {
        self.sub_extension.get_canonical_extvar(ext_v)
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
            Format::Phantom(inner) => {
                let newvar = self.get_new_uvar();
                let inner_var = self.infer_var_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::PhantomData(inner_var.into())))?;
                Ok(newvar)
            }
            Format::ItemVar(level, args, views) => {
                let newvar = self.get_new_uvar();
                let level_var = if !args.is_empty() || !views.is_empty() {
                    let mut arg_scope = UMultiScope::new(ctxt.scope);
                    let mut view_scope = ViewMultiScope::new(&ctxt.views);
                    let expected = ctxt.module.get_args(*level);
                    for ((lbl, vt), arg) in Iterator::zip(expected.iter(), args.iter()) {
                        let v_arg = self.infer_var_expr(arg, ctxt.scope)?;
                        arg_scope.push(lbl.clone(), v_arg);
                        self.unify_var_valuetype(v_arg, vt)?;
                    }
                    let expected = ctxt.module.get_view_args(*level);
                    for (lbl, view_x) in Iterator::zip(expected.iter(), views.iter()) {
                        self.traverse_view_expr(view_x, ctxt)?;
                        view_scope.push_view(lbl.clone());
                    }
                    let new_scope = UScope::Multi(&arg_scope);
                    let tmp = ctxt.with_scope(&new_scope);
                    let new_ctxt = tmp.with_view_bindings(&view_scope);
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
                self.unify_var_utype(newvar, Rc::new(UType::seq(t_inner)))?;
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
                    Rc::new(UType::seq(Rc::new(UType::Base(BaseType::U8)))),
                )?;

                // provided the previous unification succeeded, our output type is equivalent to the inferred type of the inner format
                self.unify_var_pair(newvar, v_inner)?;

                Ok(newvar)
            }
            Format::ParseFromView(view, inner) => {
                let newvar = self.get_new_uvar();

                // view requires discovery but will always have View-kind
                self.traverse_view_expr(view, ctxt)?;

                // infer inner-format type and equate it with newvar
                let v_inner = self.infer_var_format(inner.as_ref(), ctxt)?;
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
            Format::Sequence(ts) => {
                let newvar = self.get_new_uvar();
                let elem_v = self.get_new_uvar();
                for t in ts {
                    let v = self.infer_var_format(t, ctxt)?;
                    self.unify_var_pair(elem_v, v)?;
                }
                self.unify_var_utype(newvar, Rc::new(UType::seq(Rc::new(UType::Var(elem_v)))))?;
                Ok(newvar)
            }
            Format::Repeat(inner) | Format::Repeat1(inner) => {
                let newvar = self.get_new_uvar();
                let t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::seq(t)))?;
                Ok(newvar)
            }
            Format::RepeatCount(n, inner) => {
                let newvar = self.get_new_uvar();
                let n_type = self.infer_utype_expr(n, ctxt.scope)?;
                // NOTE - we don't care about the constraint, only whether it was successfully computed
                let _constraint = self.unify_utype_baseset(n_type, BaseSet::UAny)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(newvar, Rc::new(UType::seq(inner_t)))?;
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
                self.unify_var_utype(newvar, Rc::new(UType::seq(inner_t)))?;
                Ok(newvar)
            }
            Format::RepeatUntilLast(f, inner) => {
                let newvar = self.get_new_uvar();
                let (in_var, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(in_var, inner_t.clone())?;
                self.unify_var_utype(out_var, Rc::new(UType::Base(BaseType::Bool)))?;
                self.unify_var_utype(newvar, Rc::new(UType::seq(inner_t)))?;
                Ok(newvar)
            }
            Format::RepeatUntilSeq(f, inner) => {
                let newvar = self.get_new_uvar();
                let (in_var, out_var) = self.infer_vars_expr_lambda(f, ctxt.scope)?;
                let inner_t = self.infer_utype_format(inner, ctxt)?;
                self.unify_var_utype(in_var, Rc::new(UType::seq(inner_t.clone())))?;
                self.unify_var_utype(out_var, Rc::new(UType::Base(BaseType::Bool)))?;
                self.unify_var_utype(newvar, Rc::new(UType::seq(inner_t)))?;
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
                        UType::seq(inner_t.clone()),
                    ])),
                )?;
                self.unify_var_utype(done_var, Rc::new(UType::Base(BaseType::Bool)))?;

                // update function is (acc, x) -> acc
                self.unify_var_utype(
                    acc_elt_var,
                    Rc::new(UType::tuple([UType::Var(acc_var), (*inner_t).clone()])),
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
            Format::LetView(lab, inner) => {
                let newvar = self.get_new_uvar();
                let new_ctxt = ctxt.with_view_binding(lab);
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
            Format::MonadSeq(f0, f) => {
                let newvar = self.get_new_uvar();
                let _f0_v = self.infer_var_format(f0, ctxt)?;
                let f_v = self.infer_var_format(f, ctxt)?;
                self.unify_var_pair(newvar, f_v)?;
                Ok(newvar)
            }
            Format::Hint(.., inner) => {
                let newvar = self.get_new_uvar();
                let inner_v = self.infer_var_format(inner, ctxt)?;
                self.unify_var_pair(newvar, inner_v)?;
                Ok(newvar)
            }
            Format::LiftedOption(opt_f) => {
                let newvar = self.get_new_uvar();
                let inner_var = match opt_f {
                    None => self.get_new_uvar(),
                    Some(inner_f) => self.infer_var_format(inner_f, ctxt)?,
                };
                self.unify_var_utype(newvar, Rc::new(UType::Option(inner_var.into())))?;
                Ok(newvar)
            }
            Format::WithView(view, vf) => {
                let newvar = self.get_new_uvar();

                // fully explore the ViewExpr
                self.traverse_view_expr(view, ctxt)?;

                let vf_var = self.infer_var_view_format(vf, ctxt)?;
                self.unify_var_pair(newvar, vf_var)?;
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

    pub(crate) fn infer_module(module: &FormatModule, top_format: &Format) -> TCResult<Self> {
        let mut this = Self::new();
        let scope = UScope::Empty;
        let ctxt = Ctxt::new(module, &scope);

        let mut unexplored = BTreeSet::from_iter(0..module.formats.len());

        let _ = this.infer_var_format(top_format, ctxt)?;
        let mut seen_levels = this.level_vars.keys().copied().collect::<BTreeSet<usize>>();
        for already_seen in seen_levels.iter() {
            unexplored.remove(already_seen);
        }

        loop {
            let Some(next_level) = unexplored.pop_first() else {
                break;
            };
            let _ = this.infer_var_format(module.get_format(next_level), ctxt)?;
            let all_seen_levels = this.level_vars.keys().copied().collect::<BTreeSet<usize>>();
            for just_seen in all_seen_levels.difference(&seen_levels) {
                unexplored.remove(just_seen);
            }
            seen_levels = all_seen_levels;
        }
        Ok(this)
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
    pub(crate) fn reify(&self, t: Rc<UType>) -> Option<AugValueType> {
        match t.as_ref() {
            UType::Hole => {
                // REVIEW - should this simply return None instead? or maybe ValueType::Any?
                unreachable!("reify: UType::Hole should be erased by any non-Hole unification!");
            }
            UType::ViewObj => Some(AugValueType::ViewObj),
            UType::ExternVar(_ext_var) => None,
            &UType::Var(uv) => {
                let v = self.get_canonical_uvar(uv);
                match self.substitute_uvar_vtype(v) {
                    Ok(Some(t0)) => match t0 {
                        VType::Base(bs) => match bs.get_unique_solution(uv) {
                            Ok(b) => Some(AugValueType::Base(b)),
                            Err(_e) => None,
                        },
                        VType::Abstract(ut) => self.reify(ut),
                        VType::IndefiniteUnion(vmid) => self.reify_union(vmid),
                        VType::ImplicitRecord(..) | VType::ImplicitTuple(..) => unreachable!(
                            "Unsolved implicit Tuple or Record leftover from un-unified projection: {t0:?}"
                        ),
                    },
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
            UType::Base(g) => Some(AugValueType::Base(*g)),
            UType::Empty => Some(AugValueType::Empty),
            UType::Tuple(ts) => {
                let mut vts = Vec::with_capacity(ts.len());
                for elt in ts.iter() {
                    vts.push(self.reify(elt.clone())?);
                }
                Some(AugValueType::Tuple(vts))
            }
            UType::Record(fs) => {
                let mut vfs = Vec::with_capacity(fs.len());
                for (lab, ft) in fs.iter() {
                    vfs.push((lab.clone(), self.reify(ft.clone())?));
                }
                Some(AugValueType::Record(vfs))
            }
            UType::Seq(t0, h) => Some(AugValueType::Seq(Box::new(self.reify(t0.clone())?), *h)),
            UType::Option(t0) => Some(AugValueType::Option(Box::new(self.reify(t0.clone())?))),
            UType::PhantomData(t0) => {
                Some(AugValueType::PhantomData(Box::new(self.reify(t0.clone())?)))
            }
        }
    }

    fn reify_union(&self, vmid: VMId) -> Option<AugValueType> {
        let vm = self.varmaps.get_varmap(vmid);
        let mut branches = BTreeMap::new();
        for (label, ut) in vm.iter() {
            let variant_type = self.reify(ut.clone())?;
            // NOTE - only add a variant to the union-type if its inner type is inhabitable
            if !matches!(variant_type, AugValueType::Empty) {
                branches.insert(label.clone(), variant_type);
            }
        }
        Some(AugValueType::Union(branches))
    }
}
// !SECTION

#[derive(Debug, Clone, Copy)]
pub(crate) enum WHNFSolution {
    Var(UVar),
    Base(BaseType),
}

impl WHNFSolution {
    pub fn coerce(ty: &UType) -> Self {
        match ty {
            UType::Var(v) => Self::Var(*v),
            UType::Base(b) => Self::Base(*b),
            _ => panic!("non-whnf utype encountered during coercion: {ty:?}"),
        }
    }
}

/// Output type for step-by-step expansion.
///
/// Hybrid between UType and VType specialized for stepwise reification (expansion).
#[derive(Debug, Clone)]
pub(crate) enum Expansion {
    Empty,
    Base(BaseType),
    Outcome(ExtVar),
    Record(Vec<(Label, WHNFSolution)>),
    Union(BTreeMap<Label, WHNFSolution>),
    Seq(WHNFSolution, SeqBorrowHint),
    Option(WHNFSolution),
    Tuple(Vec<WHNFSolution>),
    ViewObj,
    PhantomData(WHNFSolution),
}

// SECTION - specialized methods for elaboration and codegen purposes
impl TypeChecker {
    /// Retrieves the associated InferenceEngine for a given ExtVar.
    pub(crate) fn get_extern(&self, ext_var: ExtVar) -> Rc<inference::InferenceEngine> {
        let ix = ext_var.0;
        if ix >= self.sub_extension.subtrees.len() {
            unreachable!(
                "invalid external var {ext_var} (out-of-range): {ix} >= {}",
                self.sub_extension.subtrees.len()
            );
        }
        self.sub_extension.subtrees[ix].clone()
    }

    pub(crate) fn expand_var(&self, var: UVar) -> Expansion {
        let v = self.get_canonical_uvar(var);
        match &self.constraints[v.0] {
            Constraints::Indefinite => panic!("expand_var: indefinite constraint on {v}"),
            Constraints::Variant(vmid) => self.expand_union(*vmid),
            Constraints::Invariant(constraint) => match constraint {
                Constraint::Equiv(utype) => self.expand_type(utype.clone()),
                Constraint::Elem(bs) => match bs.get_unique_solution(v) {
                    Ok(b) => Expansion::Base(b),
                    Err(e) => panic!("{e}"),
                },
                Constraint::Proj(proj_shape) => match proj_shape {
                    ProjShape::TupleWith(ix_vars) => {
                        let mut flat = Vec::new();
                        for (ix, var) in ix_vars.iter() {
                            if *ix > flat.len() {
                                panic!("missing index {ix} in {v}")
                            }
                            flat.push(WHNFSolution::Var(*var));
                        }
                        Expansion::Tuple(flat)
                    }
                    ProjShape::RecordWith(fld_vars) => {
                        let mut flat = Vec::new();
                        for (lbl, var) in fld_vars.iter() {
                            flat.push((lbl.clone(), WHNFSolution::Var(*var)));
                        }
                        Expansion::Record(flat)
                    }
                    ProjShape::SeqOf(v) => {
                        Expansion::Seq(WHNFSolution::Var(*v), SeqBorrowHint::Constructed)
                    }
                    ProjShape::OptOf(v) => Expansion::Option(WHNFSolution::Var(*v)),
                },
            },
        }
    }

    // NOTE - the implementation here is not the most robust as it relies on every UVar expanding to a WHNF all of whose UTypes are Var.
    fn expand_type(&self, ty: Rc<UType>) -> Expansion {
        match ty.as_ref() {
            UType::Hole => {
                // REVIEW - should this simply return None instead? or maybe ValueType::Any?
                unreachable!(
                    "expand_type: UType::Hole should be erased by any non-Hole unification!"
                );
            }
            UType::ViewObj => Expansion::ViewObj,
            UType::Var(uv) => self.expand_var(*uv),
            UType::ExternVar(ext_var) => Expansion::Outcome(*ext_var),
            UType::Base(g) => Expansion::Base(*g),
            UType::Empty => Expansion::Empty,
            UType::Tuple(ts) => {
                let mut vts = Vec::with_capacity(ts.len());
                for elt in ts.iter() {
                    let sol = WHNFSolution::coerce(&elt);
                    vts.push(sol);
                }
                Expansion::Tuple(vts)
            }
            UType::Record(fs) => {
                let mut vfs = Vec::with_capacity(fs.len());
                for (lab, ft) in fs.iter() {
                    let sol = WHNFSolution::coerce(&ft);
                    vfs.push((lab.clone(), sol));
                }
                Expansion::Record(vfs)
            }
            UType::Seq(t0, h) => {
                let v0 = WHNFSolution::coerce(&t0);
                Expansion::Seq(v0, *h)
            }
            UType::Option(t0) => {
                let v0 = WHNFSolution::coerce(&t0);
                Expansion::Option(v0)
            }
            UType::PhantomData(t0) => {
                let v0 = WHNFSolution::coerce(&t0);
                Expansion::PhantomData(v0)
            }
        }
    }

    fn is_empty_var(&self, var: WHNFSolution) -> bool {
        let WHNFSolution::Var(var) = var else {
            return false;
        };
        let var = self.get_canonical_uvar(var);
        matches!(&self.constraints[var.0], Constraints::Invariant(con) if matches!(con, Constraint::Equiv(ut) if matches!(**ut, UType::Empty)))
    }

    fn expand_union(&self, vmid: VMId) -> Expansion {
        let vm = self.varmaps.get_varmap(vmid);
        let mut branches = BTreeMap::new();
        for (label, branch_t) in vm.iter() {
            let var = WHNFSolution::coerce(&branch_t);
            // NOTE - only add a variant to the union-type if its inner type is inhabitable
            if !self.is_empty_var(var) {
                branches.insert(label.clone(), var);
            }
        }
        Expansion::Union(branches)
    }
}

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
    err: Box<TCErrorKind>,
    _trace: Vec<Box<dyn std::fmt::Debug + 'static + Send + Sync>>,
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

impl std::error::Error for TCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.err.source()
    }
}

#[derive(Debug)]
pub enum TCErrorKind {
    VarianceMismatch(UVar, VMId, VarMap, Constraint, Polarity), // attempted unification of a variant and non-variant constraint
    Unification(ConstraintError),
    InfiniteType(UVar, Constraints),
    MultipleSolutions(UVar, BaseSet),
    NoSolution(UVar),
    MissingView(Label),
    Inference(ExtVar, inference::InferenceError),
    // FIXME - trim down these variants a bit
    /// Two numeric solutions cannot be reconciled (across aliased ExtVar)
    IrreconcilableNumSolutions(NumSolution, NumSolution),
    /// ExtVar unified with an explicitly non-numeric UType
    NonNumericExtVarUnification(ExtVar, Rc<UType>),
    /// Equating a PrimInt with a non-numeric BaseType
    BadEquivalence {
        prim: PrimInt,
        base: BaseType,
        reason: CrossLayerBadnessReason,
    },
    UTypeFromSigned(ExtVar, PrimInt),
    /// ExtVar solution (PrimInt) cannot be reconciled with expectations of BaseSet constraint
    UnexpectedSolution(ExtVar, PrimInt, BaseSet),
}

impl TCErrorKind {
    fn with_trace<T>(self, trace: T) -> TCError
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
                    Constraint::Elem(_) => {
                        unreachable!("`{v} {inv}` is not infinite, but we thought it was")
                    }
                    Constraint::Proj(ps) => {
                        write!(f, "`{v} ~ {ps:?}` constitutes an infinite type")
                    }
                },
            },
            TCErrorKind::MultipleSolutions(uv, bs) => {
                write!(f, "no unique solution for `{uv} {}`", bs.to_constraint())
            }
            TCErrorKind::NoSolution(uv) => write!(f, "no valid solutions for `{uv}`"),
            TCErrorKind::MissingView(lbl) => {
                write!(f, "view-based parse depends on unbound identifier `{lbl}`")
            }
            TCErrorKind::Inference(ext_v, err) => {
                write!(f, "inference failed for extension-var `{ext_v}`: {err}")
            }
            TCErrorKind::NonNumericExtVarUnification(ext_var, utype) => {
                write!(
                    f,
                    "cannot reconcile numeric-extension `{ext_var}` with non-numeric unification-type `{utype:?}`"
                )
            }
            TCErrorKind::IrreconcilableNumSolutions(lhs, rhs) => {
                write!(f, "cannot reconcile numeric solutions `{lhs}` and `{rhs}`")
            }
            TCErrorKind::UnexpectedSolution(ext, pt, bs) => {
                write!(
                    f,
                    "found prim-int type `{pt}` for numeric extension `{ext}`, but unification requires `{bs}`"
                )
            }
            TCErrorKind::BadEquivalence { prim, base, reason } => {
                write!(
                    f,
                    "bad equivalence (unification on ExtVar) between prim-int type `{prim}` and base type `{base}`: {reason}",
                )
            }
            TCErrorKind::UTypeFromSigned(ext, prim) => {
                write!(
                    f,
                    "failed to convert signed prim-int type `{prim}` (solution for {ext}) to utype for unification purposes",
                )
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

/// Classification for error-condition of [`TCErrorKind::BadEquivalence`]
/// for improved clarity of error-reporting.
#[derive(Clone, Copy, Debug)]
pub enum CrossLayerBadnessReason {
    /// Supported PrimInt paired with non-matching BaseType
    Mismatched,
    /// PrimInt does not have any direct analogue in BaseType layer (i.e. it is Signed)
    Unsupported,
    /// BaseType is not numeric
    NonNumeric,
}

impl std::fmt::Display for CrossLayerBadnessReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrossLayerBadnessReason::Mismatched => write!(f, "mismatched precision"),
            CrossLayerBadnessReason::Unsupported => {
                write!(f, "signed-int unification not supported")
            }
            CrossLayerBadnessReason::NonNumeric => write!(f, "base-type is not numeric"),
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

impl From<(ExtVar, InferenceError)> for TCErrorKind {
    fn from((ext_var, err): (ExtVar, InferenceError)) -> Self {
        Self::Inference(ext_var, err)
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
    let ret = Ok(tc.reify(_ut).map(|aug_t| aug_t.into()));
    tc.check_uvar_sanity();
    ret
}

pub type TCResult<T> = Result<T, TCError>;

mod __impl {
    use crate::{FormatModule, typecheck::ExtVar};

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
                    ProjShape::OptOf(parv) => write!(f, "~ Opt({parv})"),
                },
            }
        }
    }

    impl std::fmt::Display for UVar {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "?{}", self.0)
        }
    }

    impl std::fmt::Display for ExtVar {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "#{}", self.0)
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
            write!(f, "${}", self.0)
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
        let expected = AugValueType::Union(BTreeMap::from([
            ("A".into(), AugValueType::Base(BaseType::U8)),
            ("B".into(), AugValueType::Tuple(vec![])),
        ]));
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_non_union_complex() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::record(vec![
            ("number", Format::Byte(ByteSet::full())),
            (
                "isEven",
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
        let expected = AugValueType::Record(vec![
            ("number".into(), AugValueType::Base(BaseType::U8)),
            ("isEven".into(), AugValueType::Base(BaseType::Bool)),
        ]);
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_union_complex() -> TCResult<()> {
        let mut tc = TypeChecker::new();
        let format = Format::record(vec![
            ("number", Format::Byte(ByteSet::full())),
            (
                "parity",
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
        let expected = AugValueType::Record(vec![
            ("number".into(), AugValueType::Base(BaseType::U8)),
            (
                "parity".into(),
                AugValueType::Union(BTreeMap::from([
                    ("Even".into(), AugValueType::UNIT),
                    ("Odd".into(), AugValueType::UNIT),
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
        let expected = AugValueType::Seq(
            Box::new(AugValueType::Base(BaseType::U32)),
            SeqBorrowHint::Constructed,
        );
        assert_eq!(output, expected);
        Ok(())
    }
}
