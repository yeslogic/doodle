use num_bigint::BigInt;
use num_traits::{One as _, Signed, Zero};
use serde::Serialize;
use std::borrow::Cow;

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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash,)]
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

    pub const fn is_signed(self) -> bool {
        self.is_signed
    }

    pub fn compare_width(self, other: Self) -> std::cmp::Ordering {
        self.bit_width.cmp(&other.bit_width)
    }

    pub fn encompasses(self, other: Self) -> bool {
        self.as_bounds().encompasses(&other.as_bounds())
    }
}

impl NumRep {
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
    where BigInt: From<N> {
        Self(BigInt::from(n), rep)
    }

    /// Returns `true` if the stored `NumRep` is abstract (i.e. `Auto`).
    pub fn is_abstract(self) -> bool {
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
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Const(TypedConst),
    // Opt(Option<Box<Value>>),
}

impl Value {
    /// Returns `true` if `self` is representable:
    ///   - If `self` is a constant value, it must itself be representable
    ///   - If `self` is Some(x), `x` must be representable
    ///
    /// `None` is always representable.
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
    op: BasicBinOp,
    // If None: op(T, T | auto) -> T, op(T0, T1) { T0 != T1 } -> ambiguous; otherwise, forces rep for `Some(rep)``
    #[serde(serialize_with = "ser_opt_machine_rep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    out_rep: Option<MachineRep>,
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

#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Expr {
    Const(TypedConst),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Cast(#[serde(serialize_with = "ser_machine_rep")] MachineRep, Box<Expr>),
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(arg0) => write!(f, "{}", arg0),
            Self::BinOp(op, lhs, rhs) => write!(f, "({:?} {} {:?})", lhs, op, rhs),
            Self::UnaryOp(op, inner) => write!(f, "{}({:?})", op, inner),
            Self::Cast(rep, inner) => write!(f, "Cast({:?}, {:?})", inner, rep),
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    DivideByZero,
    RemainderNonPositive,
    Ambiguous(NumRep, NumRep),
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
        }
    }
}

impl std::error::Error for EvalError {}

impl Expr {
    pub fn eval(&self) -> Result<Value, EvalError> {
        match self {
            Expr::Const(typed_const) => Ok(Value::Const(typed_const.clone())),
            Expr::BinOp(bin_op, lhs, rhs) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
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
                let expr = expr.eval()?;
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
            Expr::Cast(mach_rep, expr) => {
                let val = expr.eval()?;
                match val {
                    Value::Const(TypedConst(num, _rep)) => {
                        Ok(Value::Const(TypedConst(num, NumRep::Concrete(*mach_rep))))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    use num_traits::One;
    use proptest::prelude::*;

    fn abstract_strategy() -> BoxedStrategy<NumRep> {
        prop_oneof![Just(NumRep::AUTO)].boxed()
    }

    fn concrete_strategy() -> BoxedStrategy<NumRep> {
        prop_oneof![
            Just(NumRep::U8),
            Just(NumRep::U16),
            Just(NumRep::U32),
            Just(NumRep::U64),
            Just(NumRep::I8),
            Just(NumRep::I16),
            Just(NumRep::I32),
            Just(NumRep::I64),
        ]
        .boxed()
    }

    fn numrep_strategy() -> BoxedStrategy<NumRep> {
        prop_oneof![abstract_strategy(), concrete_strategy(),].boxed()
    }

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
                .eval()?
                .as_const()
                .unwrap()
                .eq_num(&BigInt::from(2))
        );
        Ok(())
    }

    proptest! {
        #[test]
        fn cast_works(orig in numrep_strategy(), tgt in numrep_strategy()) {
            let one = TypedConst(BigInt::one(), orig);
            let casted_one = Expr::Cast(tgt.into(), Box::new(Expr::Const(one)));
            let val = casted_one.eval().unwrap();
            let rep = val.as_const().unwrap().get_rep();
            prop_assert_eq!(rep, tgt);
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
            let actual = two_should_be_rep.eval().unwrap().as_const().unwrap().get_rep();
            prop_assert_eq!(actual, rep);
        }
    }
}
