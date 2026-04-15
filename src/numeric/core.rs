use num_bigint::BigInt;
use num_traits::{One as _, Signed, Zero};
use serde::Serialize;
use std::{borrow::Cow, cmp::Ordering};

pub const VOID: &'static VoidScope = &VoidScope;

use crate::decoder::UnknownVarError;
use crate::scope::{EvalScope, VoidScope};
use crate::{Label, bounds::Bounds as UBounds};

pub type Number = BigInt;

/// Standalone ground operations on two numeric arguments
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BasicBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl BasicBinOp {
    pub const fn to_static_str(self) -> &'static str {
        match self {
            BasicBinOp::Add => "+",
            BasicBinOp::Sub => "-",
            BasicBinOp::Mul => "*",
            BasicBinOp::Div => "/",
            BasicBinOp::Rem => "%",
        }
    }
}

impl std::fmt::Display for BasicBinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_static_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BasicUnaryOp {
    /// Arithmetic negation
    Negate,
    /// Absolute value
    AbsVal,
    /// The successor function over any integral type (behavior unspecified when applied to max_val)
    IntSucc,
    /// The predecessor function over any integral type (behavior unspecified when applied to min_val)
    IntPred,
}

impl BasicUnaryOp {
    pub const fn to_static_str(self) -> &'static str {
        match self {
            BasicUnaryOp::Negate => "~",
            BasicUnaryOp::AbsVal => "abs",
            BasicUnaryOp::IntSucc => "succ",
            BasicUnaryOp::IntPred => "pred",
        }
    }
}

impl std::fmt::Display for BasicUnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_static_str())
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum BitWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

/// Machine-representation of a numeric value, consisting of its signedness
/// and its bit-width.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct MachineRep {
    pub is_signed: bool,
    pub bit_width: BitWidth,
}

impl MachineRep {
    pub const I8: Self = MachineRep {
        is_signed: true,
        bit_width: BitWidth::Bits8,
    };
    pub const I16: Self = MachineRep {
        is_signed: true,
        bit_width: BitWidth::Bits16,
    };
    pub const I32: Self = MachineRep {
        is_signed: true,
        bit_width: BitWidth::Bits32,
    };
    pub const I64: Self = MachineRep {
        is_signed: true,
        bit_width: BitWidth::Bits64,
    };

    pub const U8: Self = MachineRep {
        is_signed: false,
        bit_width: BitWidth::Bits8,
    };
    pub const U16: Self = MachineRep {
        is_signed: false,
        bit_width: BitWidth::Bits16,
    };
    pub const U32: Self = MachineRep {
        is_signed: false,
        bit_width: BitWidth::Bits32,
    };
    pub const U64: Self = MachineRep {
        is_signed: false,
        bit_width: BitWidth::Bits64,
    };
}

impl From<MachineRep> for NumRep {
    fn from(value: MachineRep) -> Self {
        NumRep::Concrete(value)
    }
}

/// Marker-type for the intended representation fo a numeric value,
/// which an either be an explicit, concrete `MachineRep` or `Auto`.
///
/// Using `Auto` may not always work, but whenever there is a single
/// intuitive choice for what representation is natural, it should
/// yield the same result as using the concrete representation of that expected
/// interpretation.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum NumRep {
    Auto,
    Concrete(MachineRep),
}

impl MachineRep {
    /// Outputs a string representation of this machine-representation,
    /// suitable for value suffixing of numeric consts in Rust.
    pub const fn to_static_str(self) -> &'static str {
        if self.is_signed {
            match self.bit_width {
                BitWidth::Bits8 => "i8",
                BitWidth::Bits16 => "i16",
                BitWidth::Bits32 => "i32",
                BitWidth::Bits64 => "i64",
            }
        } else {
            match self.bit_width {
                BitWidth::Bits8 => "u8",
                BitWidth::Bits16 => "u16",
                BitWidth::Bits32 => "u32",
                BitWidth::Bits64 => "u64",
            }
        }
    }

    /// Returns `true` if every value of `other` is a valid value of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use doodle::numeric::core::MachineRep;
    ///
    /// assert!(MachineRep::U32.is_superset(MachineRep::U32));
    /// assert!(MachineRep::U32.is_superset(MachineRep::U16));
    /// assert!(MachineRep::I32.is_superset(MachineRep::U16));
    /// assert!(!MachineRep::I32.is_superset(MachineRep::U32));
    /// assert!(MachineRep::I32.is_superset(MachineRep::I8));
    /// assert!(!MachineRep::U32.is_superset(MachineRep::I8));
    /// ```
    pub fn is_superset(self, other: MachineRep) -> bool {
        match (
            self.is_signed,
            other.is_signed,
            self.bit_width.cmp(&other.bit_width),
        ) {
            (true, false, Ordering::Greater) => true,
            (a, b, Ordering::Equal | Ordering::Greater) if a == b => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for MachineRep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_static_str())
    }
}

impl NumRep {
    /// Outputs a string representation of this numeric representation.
    ///
    /// When concrete, this is the same as [`MachineRep::to_static_str`].
    ///
    /// For `Auto`, outputs the notation chosen for auto-value suffixing
    /// (and auto-output-type for arithmetic operations), namely `?`.
    pub const fn to_static_str(self) -> &'static str {
        match self {
            NumRep::Auto => "?",
            NumRep::Concrete(machine) => machine.to_static_str(),
        }
    }
}

impl std::fmt::Display for NumRep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

impl NumRep {
    pub const I8: NumRep = NumRep::Concrete(MachineRep::I8);
    pub const I16: NumRep = NumRep::Concrete(MachineRep::I16);
    pub const I32: NumRep = NumRep::Concrete(MachineRep::I32);
    pub const I64: NumRep = NumRep::Concrete(MachineRep::I64);

    pub const U8: NumRep = NumRep::Concrete(MachineRep::U8);
    pub const U16: NumRep = NumRep::Concrete(MachineRep::U16);
    pub const U32: NumRep = NumRep::Concrete(MachineRep::U32);
    pub const U64: NumRep = NumRep::Concrete(MachineRep::U64);

    pub const AUTO: NumRep = NumRep::Auto;
}

/// Representative min and max bounds for a numeric type
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Bounds {
    min: Number,
    max: Number,
}

impl std::fmt::Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", &self.min, &self.max)
    }
}

impl Bounds {
    pub fn new(min: Number, max: Number) -> Self {
        Self { min, max }
    }

    pub fn singleton(n: Number) -> Self {
        Self {
            min: n.clone(),
            max: n,
        }
    }

    /// Returns `true` if every value in `sub_range` is also within `self`.
    ///
    /// If `inferior` has inverted bounds, will panic.
    pub fn encompasses(&self, inferior: &Self) -> bool {
        assert!(inferior.min <= inferior.max);
        self.min <= inferior.min && self.max >= inferior.max
    }

    /// Dual to [`encompasses`].
    ///
    /// Returns `true` if every value in `self` is also within `superior`.
    ///
    /// If `superior` has inverted bounds, will panic.
    pub fn is_encompassed_by(&self, superior: &Self) -> bool {
        assert!(superior.min <= superior.max);
        self.min >= superior.min && self.max <= superior.max
    }

    pub(crate) fn unify<'a>(&'a self, bs2: &'a Bounds) -> Cow<'a, Bounds> {
        if self.is_encompassed_by(bs2) {
            Cow::Borrowed(bs2)
        } else if self.encompasses(bs2) {
            Cow::Borrowed(self)
        } else {
            Cow::Owned(Bounds {
                min: Ord::min(&self.min, &bs2.min).clone(),
                max: Ord::max(&self.max, &bs2.max).clone(),
            })
        }
    }
}

/// Macro for constructing the (min, max) bounds-pair of any numeric type `T` for which:
///   - Associated `MIN` and `MAX` consts are defined
///   - `BigInt` has a `From<T>` impl
macro_rules! bounds_of {
    ( $t:ty ) => {
        (Number::from(<$t>::MIN), Number::from(<$t>::MAX))
    };
}

impl MachineRep {
    pub fn as_bounds(self) -> Bounds {
        let (min, max) = match self {
            Self::U8 => bounds_of!(u8),
            Self::U16 => bounds_of!(u16),
            Self::U32 => bounds_of!(u32),
            Self::U64 => bounds_of!(u64),
            Self::I8 => bounds_of!(i8),
            Self::I16 => bounds_of!(i16),
            Self::I32 => bounds_of!(i32),
            Self::I64 => bounds_of!(i64),
        };
        Bounds { min, max }
    }

    /// Returns `true` if `self` represents a signed type.
    pub const fn is_signed(self) -> bool {
        self.is_signed
    }

    /// Returns a comparison between `self.bit_width` and `other.bit_width`.
    pub fn compare_width(self, other: Self) -> std::cmp::Ordering {
        self.bit_width.cmp(&other.bit_width)
    }

    /// Returns `true` if every value that is representable within `other` is also representable
    /// as a value within `self`.
    pub fn encompasses(self, other: Self) -> bool {
        self.as_bounds().encompasses(&other.as_bounds())
    }
}

impl NumRep {
    /// Returns `Some(bounds)` if `self` is a concrete machine-rep, or `None` if `self` is `Auto`.
    pub fn as_bounds(self) -> Option<Bounds> {
        match self {
            NumRep::Auto => return None,
            NumRep::Concrete(mr) => Some(mr.as_bounds()),
        }
    }

    pub const fn is_auto(self) -> bool {
        matches!(self, NumRep::Auto)
    }
}

/// Value-level numeric constant with an accompanying representation.
///
/// If the representation is `Auto`, the exact choice of concrete representation
/// that will be assumed is determined contextually based on the operation the value is
/// involved in. Top-level TypedConst values (i.e. those that where no further arithmetic is to
/// be performed on) should not be `Auto`-representation.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct TypedConst(
    #[serde(serialize_with = "ser_bigint")] pub BigInt,
    #[serde(serialize_with = "ser_num_rep")] pub NumRep,
);

fn ser_bigint<S>(value: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn ser_num_rep<S>(value: &NumRep, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(value.to_static_str())
}

impl std::fmt::Display for TypedConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = &self.0;
        let r = &self.1;
        write!(f, "{n}{r}")
    }
}

impl TypedConst {
    pub fn new<N>(n: N, rep: NumRep) -> Self
    where
        BigInt: From<N>,
    {
        Self(BigInt::from(n), rep)
    }

    /// Returns `true` if `self` has an abstracted representation, i.e. if its rep is [`NumRep::Auto`].
    pub fn is_abstract(&self) -> bool {
        self.1.is_auto()
    }

    /// Returns `true` if `self` is 'representable', i.e. if its value is representable within the
    /// implicit bounds of the `NumRep` it is associated with. This always true when for `NumRep::Auto`,
    /// and otherwise is true when the value is in the bounds of the concrete `NumRep`.
    pub fn is_representable(&self) -> bool {
        let TypedConst(n, rep) = self;
        if let Some(bounds) = rep.as_bounds() {
            n >= &bounds.min && n <= &bounds.max
        } else {
            debug_assert!(rep.is_auto());
            true
        }
    }

    /// Returns the inner `BigInt` of a `TypedConst`.
    pub fn as_raw_value(&self) -> &BigInt {
        &self.0
    }

    /// Type-agnostic equality on a pure mathematical level.
    ///
    /// This check will return `true` if `self` and `other` have the same value,
    /// regardless of their individual representations, even in cases where either
    /// value is not representable (see [`TypedConst::is_representable`]).
    pub fn eq_val(&self, other: &TypedConst) -> bool {
        &self.0 == &other.0
    }

    /// Numeric equality test on `self`, that the value it holds is equal to `other` regardless of the
    /// `NumRep` chosen.
    ///
    /// Saves the construction of a new TypedConst compared to [`eq_val`] if the query is made starting with a BigInt in mind.
    pub fn eq_num(&self, other: &BigInt) -> bool {
        &self.0 == other
    }

    /// Returns `true` if and only if `self` is notionally equivalent to the `M`-value `other` and
    /// has a nominal representation that is compatible with `rep`.
    ///
    /// # Notes
    ///
    /// If the feature-flag `"pattern_matches_auto_rep"` is set, then `self._rep() == NumRep::Auto`
    /// is treated as a wildcard and will match for any choice of `rep`.
    ///
    /// If that same feature-flag is not enabled, then `self.get_rep() == NumRep::Auto` is treated
    /// as a non-match and will always return `false`.
    pub fn pat_matches<M>(&self, other: M, rep: MachineRep) -> bool
    where
        BigInt: From<M>,
    {
        ((cfg!(feature = "pattern_matches_auto_rep") && self.is_abstract())
            || self.1 == NumRep::Concrete(rep))
            && &self.0 == &BigInt::from(other)
    }

    /// Returns true if `self` can be considered to match `Pattern::U8(other)`.
    pub fn matches_u8(&self, other: u8) -> bool {
        self.pat_matches(other, MachineRep::U8)
    }

    /// Returns true if `self` can be considered to match `Pattern::U16(other)`.
    pub fn matches_u16(&self, other: u16) -> bool {
        self.pat_matches(other, MachineRep::U16)
    }

    /// Returns true if `self` can be considered to match `Pattern::U32(other)`.
    pub fn matches_u32(&self, other: u32) -> bool {
        self.pat_matches(other, MachineRep::U32)
    }

    /// Returns true if `self` can be considered to match `Pattern::U64(other)`.
    pub fn matches_u64(&self, other: u64) -> bool {
        self.pat_matches(other, MachineRep::U64)
    }

    /// Returns `true` if the raw value of `self` falls within the bounds of `bounds`.
    fn falls_within(&self, bounds: UBounds) -> bool {
        use num_bigint::BigUint;

        if self.0.is_negative() {
            return false;
        }
        let this = self.0.magnitude();

        this >= &BigUint::from(bounds.min())
            && bounds
                .max
                .map(|max| this <= &BigUint::from(max))
                .unwrap_or(true)
    }

    /// Returns `true` if `self` should be considered a match for pattern `Pattern::Int(bounds)`
    pub fn matches_int_range(&self, bounds: UBounds) -> bool {
        // REVIEW - if self.0 exceeds the maximum of the nominal type of the pattern, but bounds.max is None, what behavior is correct?
        self.falls_within(bounds)
    }

    /// Returns the NumRep of a `TypedConst`.
    pub fn get_rep(&self) -> NumRep {
        self.1
    }
}

impl TypedConst {
    pub fn from_u8(value: u8) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::U8))
    }

    pub fn from_u16(value: u16) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::U16))
    }

    pub fn from_u32(value: u32) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::U32))
    }

    pub fn from_u64(value: u64) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::U64))
    }

    pub fn from_i8(value: i8) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::I8))
    }

    pub fn from_i16(value: i16) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::I16))
    }

    pub fn from_i32(value: i32) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::I32))
    }

    pub fn from_i64(value: i64) -> TypedConst {
        TypedConst(BigInt::from(value), NumRep::Concrete(MachineRep::I64))
    }

    #[cfg(test)]
    /// Replaces the rep of a TypedConst.
    fn replace_rep(self, rep: MachineRep) -> TypedConst {
        TypedConst(self.0, NumRep::Concrete(rep))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Const(TypedConst),
    // Opt(Option<Box<Value>>),
}

impl Value {
    /// Returns `true` if `self` is representable:
    ///   - If `self` is a constant value, it must itself be representable
    pub fn is_representable(&self) -> bool {
        match self {
            Value::Const(c) => c.is_representable(),
            // Value::Opt(value) => value.as_deref().map_or(true, Value::is_representable),
        }
    }

    /// Extracts a reference to the `TypedConst` held within a Value, irrespective of its numeric representative.
    pub fn as_const(&self) -> Option<&TypedConst> {
        match self {
            Value::Const(c) => Some(c),
            // Value::Opt(value) => value.as_deref().and_then(Value::as_const),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Const(num) => write!(f, "{num}"),
            // Value::Opt(None) => write!(f, "None"),
            // Value::Opt(Some(x)) => write!(f, "Some({})", x),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, Hash)]
pub struct BinOp {
    #[serde(serialize_with = "ser_basic_binop")]
    pub op: BasicBinOp,
    // If None: op(T, T | auto) -> T, op(T0, T1) { T0 != T1 } -> ambiguous; otherwise, forces rep for `Some(rep)``
    #[serde(serialize_with = "ser_opt_machine_rep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_rep: Option<MachineRep>,
}

impl BinOp {
    #[cfg(test)]
    fn replace_rep(self, rep: MachineRep) -> Self {
        BinOp {
            op: self.op,
            out_rep: Some(rep),
        }
    }
}

fn ser_basic_binop<S>(op: &BasicBinOp, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(op.to_static_str())
}

fn ser_opt_machine_rep<S>(rep: &Option<MachineRep>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match rep {
        None => s.serialize_none(),
        Some(rep) => ser_machine_rep(rep, s),
    }
}

fn ser_machine_rep<S>(rep: &MachineRep, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(rep.to_static_str())
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.out_rep {
            None => write!(f, "{}", self.op),
            Some(rep) => write!(f, "{}{}", self.op, rep),
        }
    }
}

impl BinOp {
    pub const fn new(op: BasicBinOp, out_rep: Option<MachineRep>) -> Self {
        Self { op, out_rep }
    }

    pub fn cast_rep(&self) -> Option<MachineRep> {
        self.out_rep
    }

    pub fn is_cast_and(&self, predicate: impl Fn(MachineRep) -> bool) -> bool {
        self.out_rep.is_some_and(predicate)
    }

    pub fn get_op(&self) -> BasicBinOp {
        self.op
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct UnaryOp {
    #[serde(serialize_with = "ser_basic_unaryop")]
    pub op: BasicUnaryOp,
    // If None, will pick the same type as the input (even if this produces a temporary unrepresentable)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "ser_opt_machine_rep")]
    pub out_rep: Option<MachineRep>,
}

fn ser_basic_unaryop<S>(op: &BasicUnaryOp, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(op.to_static_str())
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.out_rep {
            None => write!(f, "{}", self.op),
            Some(rep) => write!(f, "{}{}", self.op, rep),
        }
    }
}

impl UnaryOp {
    #[cfg(test)]
    fn replace_rep(self, rep: MachineRep) -> Self {
        UnaryOp {
            op: self.op,
            out_rep: Some(rep),
        }
    }
}

impl UnaryOp {
    pub const fn new(op: BasicUnaryOp, out_rep: Option<MachineRep>) -> Self {
        Self { op, out_rep }
    }

    pub fn cast_rep(&self) -> Option<MachineRep> {
        self.out_rep
    }

    pub fn is_cast_and(&self, predicate: fn(MachineRep) -> bool) -> bool {
        self.out_rep.is_some_and(predicate)
    }

    pub fn get_op(&self) -> BasicUnaryOp {
        self.op
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct CastOp {
    #[serde(serialize_with = "ser_machine_rep")]
    pub out_rep: MachineRep,
    #[serde(skip_serializing_if = "CastSemantics::is_arithmetic")]
    pub cast_semantics: CastSemantics,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default, Serialize)]
pub enum CastSemantics {
    #[default]
    Arithmetic,
    Bitwise,
}

impl CastSemantics {
    pub const fn is_arithmetic(&self) -> bool {
        matches!(self, CastSemantics::Arithmetic)
    }
}

impl CastOp {
    #[cfg(test)]
    fn replace_rep(self, rep: MachineRep) -> Self {
        CastOp {
            out_rep: rep,
            cast_semantics: self.cast_semantics,
        }
    }

    pub fn arith(out_rep: MachineRep) -> Self {
        CastOp {
            out_rep,
            cast_semantics: CastSemantics::Arithmetic,
        }
    }

    pub fn bitwise(out_rep: MachineRep) -> Self {
        CastOp {
            out_rep,
            cast_semantics: CastSemantics::Bitwise,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Expr {
    Const(TypedConst),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Cast(CastOp, Box<Expr>),
    /// Numerically-typed variable reference
    NumVar(Label),
}

impl Expr {
    #[cfg(test)]
    fn replace_rep(self, rep: MachineRep) -> Self {
        match self {
            Self::Cast(c, inner) => Self::Cast(c.replace_rep(rep), inner),
            Self::BinOp(op, lhs, rhs) => Self::BinOp(op.replace_rep(rep), lhs, rhs),
            Self::UnaryOp(op, inner) => Self::UnaryOp(op.replace_rep(rep), inner),
            Self::Const(c) => Self::Const(c.replace_rep(rep)),
            Self::NumVar(_) => self,
        }
    }

    pub(crate) fn iter_vars(&self) -> impl '_ + Iterator<Item = &'_ str> {
        match self {
            Self::NumVar(label) => {
                Box::new(std::iter::once(label.as_ref())) as Box<dyn Iterator<Item = &'_ str>>
            }
            Self::Const(_) => Box::new(std::iter::empty()),
            Self::BinOp(_, lhs, rhs) => Box::new(lhs.iter_vars().chain(rhs.iter_vars())),
            Self::UnaryOp(_, inner) => inner.iter_vars(),
            Self::Cast(_, inner) => inner.iter_vars(),
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(arg0) => write!(f, "{}", arg0),
            Self::BinOp(op, lhs, rhs) => write!(f, "({:?} {} {:?})", lhs, op, rhs),
            Self::UnaryOp(op, inner) => write!(f, "{}({:?})", op, inner),
            Self::Cast(rep, inner) => write!(f, "Cast({:?}, {:?})", inner, rep),
            Self::NumVar(lab) => write!(f, "NumVar({})", lab),
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    DivideByZero,
    RemainderNonPositive,
    Ambiguous(NumRep, NumRep),
    UnknownVar(UnknownVarError),
    BadVariable(CoerceValueError),
}

impl From<UnknownVarError> for EvalError {
    fn from(value: UnknownVarError) -> Self {
        EvalError::UnknownVar(value)
    }
}

impl From<CoerceValueError> for EvalError {
    fn from(value: CoerceValueError) -> Self {
        EvalError::BadVariable(value)
    }
}

#[derive(Debug)]
pub struct CoerceValueError {
    bad_value: crate::decoder::Value,
}

impl std::fmt::Display for CoerceValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "coerce-to-numeric failed for non-numeric value: {}",
            self.bad_value
        )
    }
}

impl<'a> TryFrom<&'a crate::decoder::Value> for StrictValue {
    type Error = CoerceValueError;

    fn try_from(value: &'a crate::decoder::Value) -> Result<Self, Self::Error> {
        use crate::decoder::Value as Raw;
        match value {
            Raw::Mapped(_, v) | Raw::Branch(_, v) => Self::try_from(&**v),
            Raw::Numeric(typed_const) => {
                Ok(StrictValue::new(Value::Const((&**typed_const).clone())))
            }
            Raw::U8(i) => Ok(StrictValue::from_u8(*i)),
            Raw::U16(i) => Ok(StrictValue::from_u16(*i)),
            Raw::U32(i) => Ok(StrictValue::from_u32(*i)),
            Raw::U64(i) => Ok(StrictValue::from_u64(*i)),
            Raw::Usize(i) => {
                log::warn!(
                    "StrictValue::try_from: Value::Usize coerced as auto-rep, inference may fail..."
                );
                Ok(StrictValue::new(Value::Const(TypedConst::new(
                    *i,
                    NumRep::Auto,
                ))))
            }
            Raw::Bool(..)
            | Raw::Char(..)
            | Raw::View { .. }
            | Raw::PhantomData
            | Raw::EnumFromTo(..)
            | Raw::Option(..)
            | Raw::Tuple(..)
            | Raw::Record(..)
            | Raw::Variant(..)
            | Raw::Seq(..) => Err(CoerceValueError {
                bad_value: value.clone(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a crate::loc_decoder::ParsedValue> for StrictValue {
    type Error = CoerceValueError;

    fn try_from(value: &'a crate::loc_decoder::ParsedValue) -> Result<Self, Self::Error> {
        use crate::loc_decoder::{Parsed, ParsedValue as Raw};

        match value {
            Raw::Flat(Parsed { inner, .. }) => Self::try_from(inner),
            Raw::Mapped(_, v) | Raw::Branch(_, v) => Self::try_from(&**v),
            Raw::Tuple(..)
            | Raw::Record(..)
            | Raw::Variant(..)
            | Raw::Seq(..)
            | Raw::Option(..) => Err(CoerceValueError {
                bad_value: value.clone_into_value(),
            }),
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::DivideByZero => write!(f, "attempted division by zero"),
            EvalError::RemainderNonPositive => write!(f, "remainder rhs must be positive"),
            EvalError::Ambiguous(rep0, rep1) => {
                write!(
                    f,
                    "operation over {rep0} and {rep1} must have an explicit output representation to be evaluated"
                )
            }
            EvalError::UnknownVar(var_err) => write!(f, "{var_err}"),
            EvalError::BadVariable(val_err) => write!(f, "{val_err}"),
        }
    }
}

impl std::error::Error for EvalError {}

#[derive(Debug, Clone)]
pub struct Strict<T> {
    pub value: T,
    pub is_valid: bool,
}

pub type StrictValue = Strict<Value>;

impl<'a> TryFrom<&'a std::convert::Infallible> for StrictValue {
    type Error = CoerceValueError;

    fn try_from(_value: &'a std::convert::Infallible) -> Result<Self, Self::Error> {
        unreachable!("StrictValue cannot be constructed from Infallible")
    }
}

impl StrictValue {
    pub fn new(value: Value) -> Self {
        let is_valid = value.is_representable();
        Self { value, is_valid }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Maps a function over the inner `Value` of `self`, returning a new `StrictValue` with the mapped value and an updated validity flag.
    ///
    /// If self already had an invalid value, the resulting `StrictValue` will also be invalid regardless of the output of `f`.
    /// Otherwise, the validity of the resulting `StrictValue` will depend on whether the output of `f` is representable.
    pub fn map<E>(self, f: impl FnOnce(Value) -> Result<Value, E>) -> Result<Self, E> {
        let value = f(self.value)?;
        let is_valid = self.is_valid && value.is_representable();
        Ok(Strict { value, is_valid })
    }

    pub fn map2<E>(
        self,
        other: Self,
        f: impl FnOnce(Value, Value) -> Result<Value, E>,
    ) -> Result<Self, E> {
        let value = f(self.value, other.value)?;
        let is_valid = self.is_valid && other.is_valid && value.is_representable();
        Ok(Strict { value, is_valid })
    }
}

impl StrictValue {
    pub fn from_u8(i: u8) -> Self {
        let value = Value::Const(TypedConst::from_u8(i));
        Self {
            value,
            is_valid: true,
        }
    }

    pub fn from_u16(i: u16) -> Self {
        let value = Value::Const(TypedConst::from_u16(i));
        Self {
            value,
            is_valid: true,
        }
    }

    pub fn from_u32(i: u32) -> Self {
        let value = Value::Const(TypedConst::from_u32(i));
        Self {
            value,
            is_valid: true,
        }
    }

    pub fn from_u64(i: u64) -> Self {
        let value = Value::Const(TypedConst::from_u64(i));
        Self {
            value,
            is_valid: true,
        }
    }

    // STUB - from_i* are not yet implemented, but they wouldn't have an immediate use-case
}

#[expect(dead_code)]
pub(crate) type EvalResult<T> = std::result::Result<T, EvalError>;

fn bitwise_cast(num: BigInt, rep_in: NumRep, rep_out: MachineRep) -> BigInt {
    match (rep_in, rep_out) {
        (NumRep::Auto, _) => {
            log::warn!(
                "Bitwise cast from auto-rep, assuming output rep for representability check..."
            );
            num
        }
        // FIXME - implement support for the non-trivial cases here, and also consider the implications for the representability checks of the output value
        (NumRep::Concrete(r0), r1) => match (r0, r1) {
            // SECTION - Same-size casts - C.f. https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.int-same-size
            (MachineRep::U8, MachineRep::I8) => todo!("same-size cast from u8 to i8"),
            (MachineRep::I8, MachineRep::U8) => todo!("same-size cast from i8 to u8"),
            (MachineRep::U16, MachineRep::I16) => todo!("same-size cast from u16 to i16"),
            (MachineRep::I16, MachineRep::U16) => todo!("same-size cast from i16 to u16"),
            (MachineRep::U32, MachineRep::I32) => todo!("same-size cast from u32 to i32"),
            (MachineRep::I32, MachineRep::U32) => todo!("same-size cast from i32 to u32"),
            (MachineRep::U64, MachineRep::I64) => todo!("same-size cast from u64 to i64"),
            (MachineRep::I64, MachineRep::U64) => todo!("same-size cast from i64 to u64"),
            // !SECTION
            // SECTION - Truncating casts - C.f https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.int-truncation
            (MachineRep::U16, MachineRep::U8) => todo!("truncating cast from u16 to u8"),
            (MachineRep::U32, MachineRep::U8) => todo!("truncating cast from u32 to u8"),
            (MachineRep::U32, MachineRep::U16) => todo!("truncating cast from u32 to u16"),
            (MachineRep::U64, MachineRep::U8) => todo!("truncating cast from u64 to u8"),
            (MachineRep::U64, MachineRep::U16) => todo!("truncating cast from u64 to u16"),
            (MachineRep::U64, MachineRep::U32) => todo!("truncating cast from u64 to u32"),
            (MachineRep::I16, MachineRep::I8) => todo!("truncating cast from i16 to i8"),
            (MachineRep::I32, MachineRep::I8) => todo!("truncating cast from i32 to i8"),
            (MachineRep::I32, MachineRep::I16) => todo!("truncating cast from i32 to i16"),
            (MachineRep::I64, MachineRep::I8) => todo!("truncating cast from i64 to i8"),
            (MachineRep::I64, MachineRep::I16) => todo!("truncating cast from i64 to i16"),
            (MachineRep::I64, MachineRep::I32) => todo!("truncating cast from i64 to i32"),

            (MachineRep::U16, MachineRep::I8) => todo!("truncating cast from u16 to i8"),
            (MachineRep::U32, MachineRep::I8) => todo!("truncating cast from u32 to i8"),
            (MachineRep::U32, MachineRep::I16) => todo!("truncating cast from u32 to i16"),
            (MachineRep::U64, MachineRep::I8) => todo!("truncating cast from u64 to i8"),
            (MachineRep::U64, MachineRep::I16) => todo!("truncating cast from u64 to i16"),
            (MachineRep::U64, MachineRep::I32) => todo!("truncating cast from u64 to i32"),

            (MachineRep::I16, MachineRep::U8) => todo!("truncating cast from i16 to u8"),
            (MachineRep::I32, MachineRep::U8) => todo!("truncating cast from i32 to u8"),
            (MachineRep::I32, MachineRep::U16) => todo!("truncating cast from i32 to u16"),
            (MachineRep::I64, MachineRep::U8) => todo!("truncating cast from i64 to u8"),
            (MachineRep::I64, MachineRep::U16) => todo!("truncating cast from i64 to u16"),
            (MachineRep::I64, MachineRep::U32) => todo!("truncating cast from i64 to u32"),
            // !SECTION
            // SECTION - Sign-extending casts - C.f. https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.int-extension
            (MachineRep::I8, MachineRep::U16) => todo!("sign-extending cast from i8 to u16"),
            (MachineRep::I8, MachineRep::U32) => todo!("sign-extending cast from i8 to u32"),
            (MachineRep::I8, MachineRep::U64) => todo!("sign-extending cast from i8 to u64"),
            (MachineRep::I16, MachineRep::U32) => todo!("sign-extending cast from i16 to u32"),
            (MachineRep::I16, MachineRep::U64) => todo!("sign-extending cast from i16 to u64"),
            (MachineRep::I32, MachineRep::U64) => todo!("sign-extending cast from i32 to u64"),
            // !SECTION
            // SECTION - No-op casts, for identity and zero-extension
            | (MachineRep::U8, _) // ~I8
            | (MachineRep::I8, _) // ~(U8 | U16 | U32 | U64)
            | (MachineRep::U16, _) // ~(U8 | I8 | I16)
            | (MachineRep::I16, _) // ~(U8 | I8 | U16 | U32 | U64)
            | (MachineRep::U32, _) // ~(U8 | I8 | U16 | I16 | I32)
            | (MachineRep::I32, _) // ~(U8 | I8 | U16 | I16 | U32 | I64)
            | (MachineRep::U64, MachineRep::U64)
            | (MachineRep::I64, MachineRep::I64)
            => num,
            // !SECTION

        },
    }
}

impl Expr {
    /// Like `eval`, except that the representability of every individual sub-term is also checked,
    /// and if any term is unrepresentable, the validity flag of the return-value will be `false`.
    pub fn eval_strict<'a, S, V>(&self, scope: &'a S) -> Result<Strict<Value>, EvalError>
    where
        S: 'a + EvalScope<'a, Output = &'a V, Error = UnknownVarError>,
        V: 'static,
        StrictValue: for<'x> TryFrom<&'x V, Error = CoerceValueError>,
    {
        match self {
            Expr::Const(typed_const) => Ok(Strict::new(Value::Const(typed_const.clone()))),
            Expr::BinOp(bin_op, lhs, rhs) => {
                let lhs = lhs.eval_strict(scope)?;
                let rhs = rhs.eval_strict(scope)?;
                let BinOp { op, out_rep } = *bin_op;
                lhs.map2(rhs, |lhs: Value, rhs: Value| {
                    let (raw, rep0, rep1) = match (op, lhs, rhs) {
                        (BasicBinOp::Add, Value::Const(lhs), Value::Const(rhs)) => {
                            (lhs.0 + rhs.0, lhs.1, rhs.1)
                        }
                        (BasicBinOp::Sub, Value::Const(lhs), Value::Const(rhs)) => {
                            (lhs.0 - rhs.0, lhs.1, rhs.1)
                        }
                        (BasicBinOp::Mul, Value::Const(lhs), Value::Const(rhs)) => {
                            (lhs.0 * rhs.0, lhs.1, rhs.1)
                        }
                        (BasicBinOp::Div, Value::Const(lhs), Value::Const(rhs)) => {
                            if rhs.0.is_zero() {
                                return Err(EvalError::DivideByZero);
                            }
                            (lhs.0 / rhs.0, lhs.1, rhs.1)
                        }
                        (BasicBinOp::Rem, Value::Const(lhs), Value::Const(rhs)) => {
                            if rhs.0.is_positive() {
                                (lhs.0 % rhs.0, lhs.1, rhs.1)
                            } else {
                                return Err(EvalError::RemainderNonPositive);
                            }
                        } // (_, Value::Opt(..), _) | (_, _, Value::Opt(..)) => {
                          //     return Err(EvalError::ArithOrCastOption)
                          // }
                    };
                    let rep_out = match out_rep {
                        Some(rep) => NumRep::Concrete(rep),
                        None => {
                            if rep0 == rep1 || rep1.is_auto() {
                                rep0
                            } else if rep0.is_auto() {
                                rep1
                            } else {
                                return Err(EvalError::Ambiguous(rep0, rep1));
                            }
                        }
                    };
                    Ok(Value::Const(TypedConst(raw, rep_out)))
                })
            }
            Expr::UnaryOp(unary_op, expr) => {
                expr.eval_strict(scope)?
                    .map(|expr| match (unary_op.op, expr) {
                        (BasicUnaryOp::Negate, Value::Const(TypedConst(n, rep))) => {
                            let rep_out = match unary_op.out_rep {
                                Some(rep) => NumRep::Concrete(rep),
                                None => rep,
                            };
                            Ok(Value::Const(TypedConst(-n, rep_out)))
                        }
                        (BasicUnaryOp::AbsVal, Value::Const(TypedConst(n, rep))) => {
                            let rep_out = match unary_op.out_rep {
                                Some(rep) => NumRep::Concrete(rep),
                                None => rep,
                            };
                            Ok(Value::Const(TypedConst(n.abs(), rep_out)))
                        }
                        (BasicUnaryOp::IntSucc, Value::Const(TypedConst(n, rep))) => {
                            let rep_out = match unary_op.out_rep {
                                Some(rep) => NumRep::Concrete(rep),
                                None => rep,
                            };
                            Ok(Value::Const(TypedConst(n + BigInt::one(), rep_out)))
                        }
                        (BasicUnaryOp::IntPred, Value::Const(TypedConst(n, rep))) => {
                            let rep_out = match unary_op.out_rep {
                                Some(rep) => NumRep::Concrete(rep),
                                None => rep,
                            };
                            Ok(Value::Const(TypedConst(n - BigInt::one(), rep_out)))
                        }
                    })
            }
            Expr::Cast(cast_op, expr) => expr.eval_strict(scope)?.map(|val| match val {
                Value::Const(TypedConst(num, rep)) => {
                    let num0 = if cast_op.cast_semantics.is_arithmetic() {
                        num
                    } else {
                        bitwise_cast(num, rep, cast_op.out_rep)
                    };
                    Ok(Value::Const(TypedConst(
                        num0,
                        NumRep::Concrete(cast_op.out_rep),
                    )))
                }
            }),
            Expr::NumVar(lbl) => {
                let raw = scope.lookup_var(lbl)?;
                let val = raw.try_into()?;
                Ok(val)
            }
        }
    }

    pub fn eval<'a, S, V>(&self, scope: &'a S) -> Result<Value, EvalError>
    where
        S: 'a + EvalScope<'a, Output = &'a V, Error = UnknownVarError>,
        V: 'static,
        StrictValue: for<'x> TryFrom<&'x V, Error = CoerceValueError>,
    {
        match self {
            Expr::Const(typed_const) => Ok(Value::Const(typed_const.clone())),
            Expr::BinOp(bin_op, lhs, rhs) => {
                let lhs = lhs.eval(scope)?;
                let rhs = rhs.eval(scope)?;
                let BinOp { op, out_rep } = *bin_op;
                let (raw, rep0, rep1) = match (op, lhs, rhs) {
                    (BasicBinOp::Add, Value::Const(lhs), Value::Const(rhs)) => {
                        (lhs.0 + rhs.0, lhs.1, rhs.1)
                    }
                    (BasicBinOp::Sub, Value::Const(lhs), Value::Const(rhs)) => {
                        (lhs.0 - rhs.0, lhs.1, rhs.1)
                    }
                    (BasicBinOp::Mul, Value::Const(lhs), Value::Const(rhs)) => {
                        (lhs.0 * rhs.0, lhs.1, rhs.1)
                    }
                    (BasicBinOp::Div, Value::Const(lhs), Value::Const(rhs)) => {
                        if rhs.0.is_zero() {
                            return Err(EvalError::DivideByZero);
                        }
                        (lhs.0 / rhs.0, lhs.1, rhs.1)
                    }
                    (BasicBinOp::Rem, Value::Const(lhs), Value::Const(rhs)) => {
                        if rhs.0.is_positive() {
                            (lhs.0 % rhs.0, lhs.1, rhs.1)
                        } else {
                            return Err(EvalError::RemainderNonPositive);
                        }
                    } // (_, Value::Opt(..), _) | (_, _, Value::Opt(..)) => {
                      //     return Err(EvalError::ArithOrCastOption)
                      // }
                };
                let rep_out = match out_rep {
                    Some(rep) => NumRep::Concrete(rep),
                    None => {
                        if rep0 == rep1 || rep1.is_auto() {
                            rep0
                        } else if rep0.is_auto() {
                            rep1
                        } else {
                            return Err(EvalError::Ambiguous(rep0, rep1));
                        }
                    }
                };
                Ok(Value::Const(TypedConst(raw, rep_out)))
            }
            Expr::UnaryOp(unary_op, expr) => {
                let expr = expr.eval(scope)?;
                match (unary_op.op, expr) {
                    (BasicUnaryOp::Negate, Value::Const(TypedConst(n, rep))) => {
                        let rep_out = match unary_op.out_rep {
                            Some(rep) => NumRep::Concrete(rep),
                            None => rep,
                        };
                        Ok(Value::Const(TypedConst(-n, rep_out)))
                    }
                    (BasicUnaryOp::AbsVal, Value::Const(TypedConst(n, rep))) => {
                        let rep_out = match unary_op.out_rep {
                            Some(rep) => NumRep::Concrete(rep),
                            None => rep,
                        };
                        Ok(Value::Const(TypedConst(n.abs(), rep_out)))
                    }
                    (BasicUnaryOp::IntSucc, Value::Const(TypedConst(n, rep))) => {
                        let rep_out = match unary_op.out_rep {
                            Some(rep) => NumRep::Concrete(rep),
                            None => rep,
                        };
                        Ok(Value::Const(TypedConst(n + BigInt::one(), rep_out)))
                    }
                    (BasicUnaryOp::IntPred, Value::Const(TypedConst(n, rep))) => {
                        let rep_out = match unary_op.out_rep {
                            Some(rep) => NumRep::Concrete(rep),
                            None => rep,
                        };
                        Ok(Value::Const(TypedConst(n - BigInt::one(), rep_out)))
                    }
                }
            }
            Expr::Cast(cast_op, expr) => {
                let val = expr.eval(scope)?;
                let num0 = match val {
                    Value::Const(TypedConst(num, rep)) => {
                        if cast_op.cast_semantics.is_arithmetic() {
                            num
                        } else {
                            bitwise_cast(num, rep, cast_op.out_rep)
                        }
                    }
                };
                Ok(Value::Const(TypedConst(
                    num0,
                    NumRep::Concrete(cast_op.out_rep),
                )))
            }
            Expr::NumVar(lbl) => {
                let raw = scope.lookup_var(lbl)?;
                let val: StrictValue = raw.try_into()?;
                Ok(val.value)
            }
        }
    }
}

#[cfg(test)]
pub mod strategy {
    use super::*;
    use proptest::prelude::*;

    pub fn unsigned_rep_strategy() -> impl Strategy<Value = NumRep> {
        machine_uint_strategy().prop_map(NumRep::Concrete)
    }

    pub fn signed_rep_strategy() -> impl Strategy<Value = NumRep> {
        machine_sint_strategy().prop_map(NumRep::Concrete)
    }

    fn concrete_rep_strategy() -> impl Strategy<Value = NumRep> {
        machine_strategy().prop_map(NumRep::Concrete)
    }

    pub fn numrep_strategy() -> BoxedStrategy<NumRep> {
        prop_oneof![Just(NumRep::Auto), concrete_rep_strategy()].boxed()
    }

    fn machine_uint_strategy() -> impl Strategy<Value = MachineRep> {
        prop_oneof![
            Just(MachineRep::U8),
            Just(MachineRep::U16),
            Just(MachineRep::U32),
            Just(MachineRep::U64),
        ]
    }

    fn machine_sint_strategy() -> impl Strategy<Value = MachineRep> {
        prop_oneof![
            Just(MachineRep::I8),
            Just(MachineRep::I16),
            Just(MachineRep::I32),
            Just(MachineRep::I64),
        ]
    }

    pub fn machine_strategy() -> impl Strategy<Value = MachineRep> {
        prop_oneof![machine_uint_strategy(), machine_sint_strategy(),]
    }

    pub fn small_positive() -> impl Strategy<Value = TypedConst> {
        (1..=64u8).prop_map(|val| TypedConst::from_u8(val))
    }

    pub fn small_negative() -> impl Strategy<Value = TypedConst> {
        (-64..=-1i8).prop_map(|val| TypedConst::from_i8(val))
    }

    pub fn arb_const_from_rep(rep: MachineRep) -> BoxedStrategy<TypedConst> {
        match rep {
            MachineRep::U8 => any::<u8>().prop_map(TypedConst::from_u8).boxed(),
            MachineRep::U16 => any::<u16>().prop_map(TypedConst::from_u16).boxed(),
            MachineRep::U32 => any::<u32>().prop_map(TypedConst::from_u32).boxed(),
            MachineRep::U64 => any::<u64>().prop_map(TypedConst::from_u64).boxed(),
            MachineRep::I8 => any::<i8>().prop_map(TypedConst::from_i8).boxed(),
            MachineRep::I16 => any::<i16>().prop_map(TypedConst::from_i16).boxed(),
            MachineRep::I32 => any::<i32>().prop_map(TypedConst::from_i32).boxed(),
            MachineRep::I64 => any::<i64>().prop_map(TypedConst::from_i64).boxed(),
        }
    }

    pub fn unary_with_rep(rep: MachineRep, term: &BoxedStrategy<Expr>) -> BoxedStrategy<Expr> {
        fn expr_pred(x: Expr, rep: MachineRep) -> Expr {
            Expr::UnaryOp(
                UnaryOp {
                    op: BasicUnaryOp::IntPred,
                    out_rep: Some(rep),
                },
                Box::new(x),
            )
        }
        fn expr_succ(x: Expr, rep: MachineRep) -> Expr {
            Expr::UnaryOp(
                UnaryOp {
                    op: BasicUnaryOp::IntSucc,
                    out_rep: Some(rep),
                },
                Box::new(x),
            )
        }
        fn expr_negate(x: Expr, rep: MachineRep) -> Expr {
            Expr::UnaryOp(
                UnaryOp {
                    op: BasicUnaryOp::Negate,
                    out_rep: Some(rep),
                },
                Box::new(x),
            )
        }
        fn expr_abs(x: Expr, rep: MachineRep) -> Expr {
            Expr::UnaryOp(
                UnaryOp {
                    op: BasicUnaryOp::AbsVal,
                    out_rep: Some(rep),
                },
                Box::new(x),
            )
        }

        // TODO - swap in arb_expr_with_rep once all shallow constructions are tested
        let strat = Strategy::prop_union(
            term.clone().prop_map(move |x| expr_pred(x, rep)).boxed(),
            term.clone().prop_map(move |x| expr_succ(x, rep)).boxed(),
        );

        if rep.is_signed() {
            prop_oneof![
                strat,
                term.clone().prop_map(move |x| expr_negate(x, rep)),
                term.clone().prop_map(move |x| expr_abs(x, rep)),
            ]
            .boxed()
        } else {
            prop_oneof![strat, term.clone().prop_map(move |x| expr_abs(x, rep)),].boxed()
        }
    }

    fn subset_rep(rep: MachineRep) -> BoxedStrategy<MachineRep> {
        machine_strategy()
            .prop_filter(
                "sub-expression reps must fit into output rep",
                move |rep1| rep.is_superset(*rep1),
            )
            .boxed()
    }

    pub fn binary_with_rep(rep: MachineRep, term: &BoxedStrategy<Expr>) -> BoxedStrategy<Expr> {
        fn expr_add(x: Expr, y: Expr, rep: MachineRep) -> Expr {
            Expr::BinOp(
                BinOp {
                    op: BasicBinOp::Add,
                    out_rep: Some(rep),
                },
                Box::new(x),
                Box::new(y),
            )
        }
        fn expr_sub(x: Expr, y: Expr, rep: MachineRep) -> Expr {
            Expr::BinOp(
                BinOp {
                    op: BasicBinOp::Sub,
                    out_rep: Some(rep),
                },
                Box::new(x),
                Box::new(y),
            )
        }
        fn expr_mul(x: Expr, y: Expr, rep: MachineRep) -> Expr {
            Expr::BinOp(
                BinOp {
                    op: BasicBinOp::Mul,
                    out_rep: Some(rep),
                },
                Box::new(x),
                Box::new(y),
            )
        }
        fn expr_div(x: Expr, y: Expr, rep: MachineRep) -> Expr {
            Expr::BinOp(
                BinOp {
                    op: BasicBinOp::Div,
                    out_rep: Some(rep),
                },
                Box::new(x),
                Box::new(y),
            )
        }
        fn expr_rem(x: Expr, y: Expr, rep: MachineRep) -> Expr {
            Expr::BinOp(
                BinOp {
                    op: BasicBinOp::Rem,
                    out_rep: Some(rep),
                },
                Box::new(x),
                Box::new(y),
            )
        }

        prop_compose! {
            fn term_pair(rep: MachineRep, term1: BoxedStrategy<Expr>, term2: BoxedStrategy<Expr>)
                        (rep1 in subset_rep(rep), rep2 in subset_rep(rep), tc1 in term1, tc2 in term2) -> (Expr, Expr) {
                            (tc1.replace_rep(rep1), tc2.replace_rep(rep2))
            }
        }

        prop_oneof![
            term_pair(rep, term.clone(), term.clone()).prop_map(move |(l, r)| expr_add(l, r, rep)),
            term_pair(rep, term.clone(), term.clone()).prop_map(move |(l, r)| expr_sub(l, r, rep)),
            term_pair(rep, term.clone(), term.clone()).prop_map(move |(l, r)| expr_mul(l, r, rep)),
            term_pair(rep, term.clone(), term.clone()).prop_map(move |(l, r)| expr_div(l, r, rep)),
            term_pair(rep, term.clone(), term.clone()).prop_map(move |(l, r)| expr_rem(l, r, rep)),
        ]
        .boxed()
    }

    prop_compose! {
        fn cast_to_rep(rep: MachineRep, term: &BoxedStrategy<Expr>)
                       (x in term.clone(), r in subset_rep(rep)) -> Expr {
            if r == rep {
                return x.replace_rep(r);
            }
            Expr::Cast(CastOp::arith(rep), Box::new(x.replace_rep(r)))
        }
    }

    pub fn arb_expr_with_rep<const N: usize>(
        rep: MachineRep,
        vars: [(&'static str, MachineRep); N],
    ) -> BoxedStrategy<Expr> {
        let depth = 8;
        let max_nodes = 64;

        let leaf = if vars.is_empty() {
            arb_const_from_rep(rep).prop_map(Expr::Const).boxed()
        } else {
            prop_oneof![
                arb_const_from_rep(rep).prop_map(Expr::Const),
                prop::sample::select(vars.to_vec()).prop_map(move |(s, r)| {
                    if r == rep {
                        Expr::NumVar((*s).into())
                    } else {
                        Expr::Cast(CastOp::arith(rep), Box::new(Expr::NumVar((*s).into())))
                    }
                }),
            ]
            .boxed()
        };

        leaf.prop_recursive(depth, max_nodes, 2, move |inner| {
            let term = inner.boxed();
            prop_oneof![
                unary_with_rep(rep, &term),
                binary_with_rep(rep, &term),
                cast_to_rep(rep, &term),
            ]
        })
        .boxed()
    }

    pub fn any_expr() -> impl Strategy<Value = Expr> {
        machine_strategy()
            .prop_flat_map(|rep| arb_expr_with_rep(rep, []))
            .prop_filter("exprs must be well-typed", |x| {
                let mut ie = crate::typecheck::inference::InferenceEngine::new();
                let Ok((v, _)) = ie.infer_var_expr(x) else {
                    return false;
                };
                ie.reify(v.into()).is_some() && x.eval_strict(VOID).is_ok_and(|x| x.is_valid())
            })
    }

    pub fn any_expr_with_vars<const N: usize>(
        vars: [(&'static str, MachineRep); N],
    ) -> impl Strategy<Value = Expr> {
        machine_strategy()
            .prop_flat_map(move |rep| arb_expr_with_rep(rep, vars))
            .prop_filter("expr must contain a var if any", move |x| {
                if vars.is_empty() {
                    return true;
                }
                let mut it = x.iter_vars();
                it.next().is_some()
            })
    }

    pub fn unsigned_expr() -> impl Strategy<Value = Expr> {
        machine_uint_strategy()
            .prop_flat_map(|rep| arb_expr_with_rep(rep, []))
            .prop_filter("exprs must be well-typed", |x| {
                let mut ie = crate::typecheck::inference::InferenceEngine::new();
                let Ok((v, _)) = ie.infer_var_expr(x) else {
                    return false;
                };
                ie.reify(v.into()).is_some_and(|t| !t.to_prim().is_signed())
                    && x.eval_strict(VOID).is_ok_and(|x| x.is_valid())
            })
    }

    pub fn unsigned_expr_with_vars<const N: usize>(
        vars: [(&'static str, MachineRep); N],
    ) -> impl Strategy<Value = Expr> {
        machine_uint_strategy()
            .prop_flat_map(move |rep| arb_expr_with_rep(rep, vars))
            .prop_filter("expr must contain a var if any", move |x| {
                if vars.is_empty() {
                    return true;
                }
                let mut it = x.iter_vars();
                it.next().is_some()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::VOID;
    use super::strategy::*;
    use crate::numeric::core::*;
    use num_traits::One;
    use proptest::prelude::*;

    #[test]
    fn one_plus_one_is_two() -> Result<(), EvalError> {
        let one = TypedConst(BigInt::one(), NumRep::AUTO);
        let should_be_two = Expr::BinOp(
            BinOp {
                op: BasicBinOp::Add,
                out_rep: None,
            },
            Box::new(Expr::Const(one.clone())),
            Box::new(Expr::Const(one)),
        );
        assert!(
            should_be_two
                .eval(VOID)?
                .as_const()
                .unwrap()
                .eq_num(&BigInt::from(2))
        );
        Ok(())
    }

    proptest! {
        #[test]
        fn cast_works(orig in numrep_strategy(), tgt in machine_strategy()) {
            let one = TypedConst(BigInt::one(), orig);
            let casted_one = Expr::Cast(CastOp::arith(tgt), Box::new(Expr::Const(one)));
            let val = casted_one.eval(VOID).unwrap();
            let rep = val.as_const().unwrap().get_rep();
            prop_assert_eq!(rep, NumRep::Concrete(tgt));
        }

        #[test]
        fn auto_is_eagerly_erased(rep in numrep_strategy()) {
            let one = TypedConst(BigInt::one(), NumRep::AUTO);
            let rep_one = TypedConst(BigInt::one(), rep);
            let two_should_be_rep = Expr::BinOp(
                BinOp {
                    op: BasicBinOp::Add,
                    out_rep: None,
                },
                Box::new(Expr::Const(one)),
                Box::new(Expr::Const(rep_one)),
            );
            let actual = two_should_be_rep.eval(VOID).unwrap().as_const().unwrap().get_rep();
            prop_assert_eq!(actual, rep);
        }
    }
}
