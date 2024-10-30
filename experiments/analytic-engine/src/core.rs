use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BasicUnaryOp {
    Negate,
    AbsVal,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum NumRep {
    Abstract {
        auto: bool,
    },
    Concrete {
        is_signed: bool,
        bit_width: BitWidth,
    },
}

impl NumRep {
    pub const I8: NumRep = NumRep::Concrete {
        is_signed: true,
        bit_width: BitWidth::Bits8,
    };
    pub const I16: NumRep = NumRep::Concrete {
        is_signed: true,
        bit_width: BitWidth::Bits16,
    };
    pub const I32: NumRep = NumRep::Concrete {
        is_signed: true,
        bit_width: BitWidth::Bits32,
    };
    pub const I64: NumRep = NumRep::Concrete {
        is_signed: true,
        bit_width: BitWidth::Bits64,
    };

    pub const U8: NumRep = NumRep::Concrete {
        is_signed: false,
        bit_width: BitWidth::Bits8,
    };
    pub const U16: NumRep = NumRep::Concrete {
        is_signed: false,
        bit_width: BitWidth::Bits16,
    };
    pub const U32: NumRep = NumRep::Concrete {
        is_signed: false,
        bit_width: BitWidth::Bits32,
    };
    pub const U64: NumRep = NumRep::Concrete {
        is_signed: false,
        bit_width: BitWidth::Bits64,
    };

    pub const AUTO: NumRep = NumRep::Abstract { auto: true };
    pub const AMBIGUOUS: NumRep = NumRep::Abstract { auto: false };
}

impl NumRep {
    pub const fn is_abstract(&self) -> bool {
        matches!(self, NumRep::Abstract { .. })
    }
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

impl NumRep {
    pub(crate) fn as_bounds(&self) -> Option<Bounds> {
        let (min, max) = match self {
            NumRep::Abstract { .. } => return None,
            &NumRep::U8 => bounds_of!(u8),
            &NumRep::U16 => bounds_of!(u16),
            &NumRep::U32 => bounds_of!(u32),
            &NumRep::U64 => bounds_of!(u64),
            &NumRep::I8 => bounds_of!(i8),
            &NumRep::I16 => bounds_of!(i16),
            &NumRep::I32 => bounds_of!(i32),
            &NumRep::I64 => bounds_of!(i64),
        };
        Some(Bounds { min, max })
    }

    pub const fn is_auto(&self) -> bool {
        matches!(self, NumRep::Abstract { auto: true })
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum BitWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TypedConst(BigInt, NumRep);

impl std::fmt::Display for TypedConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = &self.0;
        match self.1 {
            NumRep::U8 => write!(f, "{}u8", n),
            NumRep::U16 => write!(f, "{}u16", n),
            NumRep::U32 => write!(f, "{}u32", n),
            NumRep::U64 => write!(f, "{}u64", n),
            NumRep::I8 => write!(f, "{}i8", n),
            NumRep::I16 => write!(f, "{}i16", n),
            NumRep::I32 => write!(f, "{}i32", n),
            NumRep::I64 => write!(f, "{}i64", n),
            NumRep::AUTO => write!(f, "{}?", n),
            NumRep::AMBIGUOUS => write!(f, "{}??", n),
        }
    }
}

impl TypedConst {
    /// Returns `true` if the stored `NumRep` is abstract (either auto or ambiguous).
    pub fn is_abstract(&self) -> bool {
        self.1.is_abstract()
    }

    /// Returns `true` if `self` is representable, which is true if either:
    ///   - The `NumRep` is `Abstract`
    ///   - The `NumRep` is concrete and `n` is in the bounds of the `NumRep`
    pub fn is_representable(&self) -> bool {
        let TypedConst(ref n, rep) = self;
        if let Some(bounds) = rep.as_bounds() {
            n >= &bounds.min && n <= &bounds.max
        } else {
            debug_assert!(rep.is_abstract());
            true
        }
    }

    pub fn as_raw_value(&self) -> &BigInt {
        &self.0
    }

    /// Type-agnostic equality on a pure mathematical level.
    ///
    /// Does not check for representablity of either value, nor even whether either representative is some flavor of `Abstract`.
    pub fn eq_val(&self, other: &TypedConst) -> bool {
        &self.0 == &other.0
    }

    /// Numeric equality test on `self`, that the value it holds is equal to `other` regardless of type.
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

#[derive(Clone, Copy, Debug)]
pub struct BinOp {
    op: BasicBinOp,
    // If None: op(T, T | auto) -> T, op(T0, T1) { T0 != T1 } -> ambiguous; otherwise, forces rep for `Some(rep)``
    out_rep: Option<NumRep>,
}

impl BinOp {
    pub fn output_type(&self, left: NumRep, right: NumRep) -> NumRep {
        if let Some(rep) = self.out_rep {
            rep
        } else if left == right || right.is_auto() {
            left
        } else if left.is_auto() {
            right
        } else {
            NumRep::AMBIGUOUS
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UnaryOp {
    op: BasicUnaryOp,
    // If None, will pick the same type as the input (even if this produces a temporary unrepresentable)
    out_rep: Option<NumRep>,
}
impl UnaryOp {
    fn output_type(&self, in_rep: NumRep) -> NumRep {
        if let Some(rep) =  self.out_rep {
            rep
        } else {
            in_rep
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Const(TypedConst),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Cast(NumRep, Box<Expr>),
    // TryUnwrap(Box<Expr>),
}

impl Expr {
    pub(crate) fn get_rep(&self) -> NumRep {
        match self {
            Expr::Const(tc) => tc.get_rep(),
            Expr::Cast(rep, _) => *rep,
            Expr::BinOp(bin_op, expr, expr1) => {
                bin_op.output_type(expr.get_rep(), expr1.get_rep())
            }
            Expr::UnaryOp(unary_op, expr) => {
                unary_op.output_type(expr.get_rep())
            }
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    DivideByZero,
    RemainderNonPositive,
    Unrepresentable(Value),
    ArithOrCastOption,
    // TryUnwrapNone,
    // TryUnwrapConst,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::DivideByZero => write!(f, "attempted division by zero"),
            EvalError::RemainderNonPositive => write!(f, "remainder rhs must be positive"),
            EvalError::Unrepresentable(value) => write!(f, "value `{value}` is unrepresentable"),
            EvalError::ArithOrCastOption => {
                write!(f, "arithmetic and casts on Value::Opt not supported")
            }
            // EvalError::TryUnwrapNone => {
            //     write!(f, "TryUnwrap called over expr evaluating to Opt(None)")
            // }
            // EvalError::TryUnwrapConst => write!(
            //     f,
            //     "TryUnwrap called over expr evaluating to Const (and not Opt)"
            // ),
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
                let BinOp { op, out_rep } = bin_op;
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
                    }
                    // (_, Value::Opt(..), _) | (_, _, Value::Opt(..)) => {
                    //     return Err(EvalError::ArithOrCastOption)
                    // }
                };
                let rep_out = match out_rep {
                    Some(rep) => *rep,
                    None => {
                        if rep0 == rep1 || rep1.is_auto() {
                            rep0
                        } else if rep0.is_auto() {
                            rep1
                        } else {
                            NumRep::AMBIGUOUS
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
                            Some(rep) => rep,
                            None => rep,
                        };
                        Ok(Value::Const(TypedConst(-n, rep_out)))
                    }
                    (BasicUnaryOp::AbsVal, Value::Const(TypedConst(n, rep))) => {
                        let rep_out = match unary_op.out_rep {
                            Some(rep) => rep,
                            None => rep,
                        };
                        Ok(Value::Const(TypedConst(n.abs(), rep_out)))
                    }
                    // (_, Value::Opt(_)) => return Err(EvalError::ArithOrCastOption),
                }
            }
            Expr::Cast(num_rep, expr) => {
                let val = expr.eval()?;
                match val {
                    Value::Const(TypedConst(num, _rep)) => {
                        Ok(Value::Const(TypedConst(num, *num_rep)))
                    }
                    // Value::Opt(_) => return Err(EvalError::ArithOrCastOption),
                }
            }
            // Expr::TryUnwrap(expr) => match expr.eval()? {
                // Value::Const(_) => return Err(EvalError::TryUnwrapConst),
                // Value::Opt(None) => return Err(EvalError::TryUnwrapNone),
                // Value::Opt(Some(x)) => Ok(*x),
            // },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    use proptest::prelude::*;

    fn abstract_strategy() -> BoxedStrategy<NumRep> {
        prop_oneof![Just(NumRep::AUTO), Just(NumRep::AMBIGUOUS)].boxed()
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
        assert!(should_be_two
            .eval()?
            .as_const()
            .unwrap()
            .eq_num(&BigInt::from(2)));
        Ok(())
    }

    proptest! {
        #[test]
        fn cast_works(orig in numrep_strategy(), tgt in numrep_strategy()) {
            let one = TypedConst(BigInt::one(), orig);
            let casted_one = Expr::Cast(tgt, Box::new(Expr::Const(one)));
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
