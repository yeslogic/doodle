use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

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
    Abstract,
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
}

impl NumRep {
    pub const fn is_abstract(&self) -> bool {
        matches!(self, NumRep::Abstract)
    }
}

/// Representative min and max bounds for a numeric type
pub struct Bounds {
    min: Number,
    max: Number,
}

macro_rules! bounds_of {
    ( $t:ty ) => {
        (Number::from(<$t>::MIN), Number::from(<$t>::MAX))
    };
}

impl NumRep {
    fn as_bounds(&self) -> Option<Bounds> {
        let (min, max) = match self {
            NumRep::Abstract => return None,
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
            NumRep::Abstract => write!(f, "{}??", n),
        }
    }
}

impl TypedConst {
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
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Const(TypedConst),
    Opt(Option<Box<Value>>),
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
            Value::Opt(value) => value.as_deref().map_or(true, Value::is_representable),
        }
    }

    pub fn as_const(&self) -> Option<&TypedConst> {
        match self {
            Value::Const(c) => Some(c),
            Value::Opt(value) => value.as_deref().and_then(Value::as_const),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Const(num) => write!(f, "{num}"),
            Value::Opt(None) => write!(f, "None"),
            Value::Opt(Some(x)) => write!(f, "Some({})", x),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BinOp {
    op: BasicBinOp,
    // If None, picks either the common rep of the two arguments or Abstract if they disagree
    out_rep: Option<NumRep>,
}

#[derive(Clone, Copy, Debug)]
pub struct UnaryOp {
    op: BasicUnaryOp,
    // If None, will pick the same type as the input (even if this produces a temporary unrepresentable)
    out_rep: Option<NumRep>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Const(TypedConst),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Cast(NumRep, Box<Expr>),
    TryUnwrap(Box<Expr>),
}

#[derive(Debug)]
pub enum EvalError {
    DivideByZero,
    RemainderNonPositive,
    Unrepresentable(Value),
    ArithOrCastOption,
    TryUnwrapNone,
    TryUnwrapConst,
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
            EvalError::TryUnwrapNone => {
                write!(f, "TryUnwrap called over expr evaluating to Opt(None)")
            }
            EvalError::TryUnwrapConst => write!(
                f,
                "TryUnwrap called over expr evaluating to Const (and not Opt)"
            ),
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
                    (_, Value::Opt(..), _) | (_, _, Value::Opt(..)) => {
                        return Err(EvalError::ArithOrCastOption)
                    }
                };
                let rep_out = match out_rep {
                    Some(rep) => *rep,
                    None => {
                        if rep0 == rep1 {
                            rep0
                        } else {
                            NumRep::Abstract
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
                    (_, Value::Opt(_)) => return Err(EvalError::ArithOrCastOption),
                }
            }
            Expr::Cast(num_rep, expr) => {
                let val = expr.eval()?;
                match val {
                    Value::Const(TypedConst(num, _rep)) => {
                        Ok(Value::Const(TypedConst(num, *num_rep)))
                    }
                    Value::Opt(_) => return Err(EvalError::ArithOrCastOption),
                }
            }
            Expr::TryUnwrap(expr) => match expr.eval()? {
                Value::Const(_) => return Err(EvalError::TryUnwrapConst),
                Value::Opt(None) => return Err(EvalError::TryUnwrapNone),
                Value::Opt(Some(x)) => Ok(*x),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_is_two() -> Result<(), EvalError> {
        let one = TypedConst(BigInt::one(), NumRep::Abstract);
        let should_be_two = Expr::BinOp(
            BinOp {
                op: BasicBinOp::Add,
                out_rep: None,
            },
            Box::new(Expr::Const(one.clone())),
            Box::new(Expr::Const(one)),
        );
        assert_eq!(
            should_be_two.eval()?.as_const().unwrap().as_raw_value(),
            &BigInt::from(2)
        );
        Ok(())
    }
}
