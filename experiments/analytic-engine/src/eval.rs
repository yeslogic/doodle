use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub, SaturatingAdd, SaturatingMul,
    SaturatingSub, WrappingAdd, WrappingMul, WrappingSub,
};
use std::any::type_name;
use std::cell::LazyCell;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

// SECTION - Evaluation model for potentially heterogenous machine integer operations
#[derive(Clone, Debug)]
pub enum Eval<T> {
    NaN,
    Direct(T),
    Indirect(IndirectEval<T>),
}

impl<T> Eval<T> {
    pub const fn is_nan(&self) -> bool {
        matches!(self, &Eval::NaN)
    }
}

impl<T> PartialEq for Eval<T>
where
    T: PartialEq + Copy,
    BigInt: From<T>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NaN, _) | (_, Self::NaN) => false,
            (Self::Direct(l0), Self::Direct(r0)) => l0 == r0,
            (Self::Indirect(l0), Self::Indirect(r0)) => &**l0.value == &**r0.value,
            (Self::Direct(n), Self::Indirect(IndirectEval { ref value, .. }))
            | (Self::Indirect(IndirectEval { ref value, .. }), Self::Direct(n)) => {
                &***value == &BigInt::from(*n)
            }
        }
    }
}

#[derive(Clone)]
pub struct IndirectEval<T> {
    value: Rc<LazyCell<BigInt, Box<dyn FnOnce() -> BigInt>>>,
    saturated: T,
    wrapped: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for IndirectEval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndirectEval")
            .field("value", &self.value)
            .field("saturated", &self.saturated)
            .field("wrapped", &self.wrapped)
            .finish()
    }
}
// !SECTION

/// Macro for bulk definition of homogenously typed binary operations with a checked version provided by a trait (`num_traits`), where
/// `None`` indicates underflow or overflow, falling back on available saturating and wrapping variants of the same operation for indirect computations
macro_rules! homogenous {
    ( $( $tr:ident, $meth:ident, $sat_tr:ident, $sat_meth:ident, $wrap_tr:ident, $wrap_meth:ident => $( ( $fname:ident , $t:ty ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(lhs: $t, rhs: $t) -> Eval<$t> {
                    match <$t as $tr>::$meth(&lhs, &rhs) {
                        Some(res) => Eval::Direct(res),
                        None => {
                            Eval::Indirect(
                                IndirectEval {
                                    value: Rc::new(LazyCell::new(
                                        Box::new(move ||
                                            BigInt::from(lhs).$meth(&BigInt::from(rhs)).unwrap()
                                        )
                                    )),
                                    saturated: <$t as $sat_tr>::$sat_meth(&lhs, &rhs),
                                    wrapped: <$t as $wrap_tr>::$wrap_meth(&lhs, &rhs),
                                }
                            )
                        }
                    }
                }
            )*
        )*
    };
}

/// Macro for bulk definition of homogenously typed binary operations with a quotient version provided by a trait (`num_traits`), where None indicates NaN
macro_rules! homogenous_quotient {
    ( $( $tr:ident, $meth:ident => $( ( $fname:ident, $t:ty ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(lhs: $t, rhs: $t) -> Eval<$t> {
                    match <$t as $tr>::$meth(&lhs, &rhs) {
                        Some(res) => Eval::Direct(res),
                        None => Eval::NaN,
                    }
                }
            )*
        )*
    }
}

/// Macro for bulk definition of homogenous source-type typed binary operations where the output type can represent every possible input type, but the computation
macro_rules! widening {
    ( $( $tr:ident, $meth:ident => $( ( $in_t:ty => $( ( $fname:ident, $out_t:ty ) ),+ $(,)? ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                $(
                    pub fn $fname(lhs: $in_t, rhs: $in_t) -> Eval<$out_t> {
                        Eval::Direct(<$out_t as $tr>::$meth((lhs as $out_t), (rhs as $out_t)))
                    }
                )*
            )*
        )*
    };
}

/// Macro for bulk definition of homogenous source-type typed binary operations where the output type can represent every possible input type, but the nominal
macro_rules! widening {
    ( $( $tr:ident, $meth:ident => $( ( $in_t:ty => $( ( $fname:ident, $out_t:ty ) ),+ $(,)? ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                $(
                    pub fn $fname(lhs: $in_t, rhs: $in_t) -> Eval<$out_t> {
                        Eval::Direct(<$out_t as $tr>::$meth((lhs as $out_t), (rhs as $out_t)))
                    }
                )*
            )*
        )*
    };
}

// SECTION - Combinatorial Explosion

// SECTION - strictly homogenous (T -> T -> T) binary operations
homogenous! {
    CheckedAdd, checked_add, SaturatingAdd, saturating_add, WrappingAdd, wrapping_add =>
        (add_u8, u8),
        (add_u16, u16),
        (add_u32, u32),
        (add_u64, u64),
        (add_i8, i8),
        (add_i16, i16),
        (add_i32, i32),
        (add_i64, i64);
    CheckedSub, checked_sub, SaturatingSub, saturating_sub, WrappingSub, wrapping_sub =>
        (sub_u8, u8),
        (sub_u16, u16),
        (sub_u32, u32),
        (sub_u64, u64),
        (sub_i8, i8),
        (sub_i16, i16),
        (sub_i32, i32),
        (sub_i64, i64);
    CheckedMul, checked_mul, SaturatingMul, saturating_mul, WrappingMul, wrapping_mul =>
        (mul_u8, u8),
        (mul_u16, u16),
        (mul_u32, u32),
        (mul_u64, u64),
        (mul_i8, i8),
        (mul_i16, i16),
        (mul_i32, i32),
        (mul_i64, i64);
}

homogenous_quotient! {
    CheckedDiv, checked_div =>
        (div_u8, u8),
        (div_u16, u16),
        (div_u32, u32),
        (div_u64, u64),
        (div_i8, i8),
        (div_i16, i16),
        (div_i32, i32),
        (div_i64, i64);
    CheckedRem, checked_rem =>
        (rem_u8, u8),
        (rem_u16, u16),
        (rem_u32, u32),
        (rem_u64, u64),
        (rem_i8, i8),
        (rem_i16, i16),
        (rem_i32, i32),
        (rem_i64, i64);
}
// !SECTION

widening! {
    Add, add =>
        (u8 => (add_u8_u16, u16), (add_u8_u32, u32), (add_u8_u64, u64), (add_u8_i16, i16), (add_u8_i32, i32), (add_u8_i64, i64)),
        (u16 => (add_u16_u32, u32), (add_u16_u64, u64), (add_u16_i32, i32), (add_u16_i64, i64)),
        (u32 => (add_u32_u64, u64), (add_u32_i64, i64)),
        (i8 => (add_i8_i16, i16), (add_i8_i32, i32), (add_i8_i64, i64));
    Sub, sub =>
        (u8 => (sub_u8_i16, i16), (sub_u8_i32, i32), (sub_u8_i64, i64)),
        (u16 => (sub_u16_i32, i32), (sub_u16_i64, i64)),
        (u32 => (sub_u32_i64, i64)),
        (i8 => (sub_i8_i16, i16), (sub_i8_i32, i32), (sub_i8_i64, i64)),
        (i16 => (sub_i16_i32, i32), (sub_i16_i64, i64)),
        (i32 => (sub_i32_i64, i64));
    Mul, mul =>
        (u8 => (mul_u8_u16, u16), (mul_u8_u32, u32), (mul_u8_u64, u64), (mul_u8_i32, i32), (mul_u8_i64, i64)),
        (u16 => (mul_u16_u32, u32), (mul_u16_u64, u64), (mul_u16_i64, i64)),
        (u32 => (mul_u32_u64, u64)),
        (i8 => (mul_i8_i16, i16), (mul_i8_i32, i32), (mul_i8_i64, i64)),
        (i16 => (mul_i16_i32, i32), (mul_i16_i64, i64)),
        (i32 => (mul_i32_i64, i64));
}

// !SECTION

pub fn eval_fallback<Lhs, Rhs, Res, E>(
    lhs: Lhs,
    rhs: Rhs,
    _op_hint: &'static str,
    checked_op: impl FnOnce(&BigInt, &BigInt) -> Option<BigInt>,
    err_none: impl FnOnce() -> E,
) -> Result<Res, E>
where
    BigInt: From<Lhs> + From<Rhs>,
    Res: TryFrom<BigInt, Error = TryFromBigIntError<BigInt>>,
    E: From<TryFromBigIntError<BigInt>>,
{
    eprintln!("[INFO]: encountered fallback operation `{_op_hint} : (({}, {}) -> {})` that may benefit from standalone function",
         type_name::<Lhs>(), type_name::<Rhs>(), type_name::<Res>()
     );
    let big_l = BigInt::from(lhs);
    let big_r = BigInt::from(rhs);
    let o_big_res = checked_op(&big_l, &big_r);
    if let Some(big_res) = o_big_res {
        Ok(<Res as TryFrom<BigInt>>::try_from(big_res)?)
    } else {
        Err(err_none())
    }
}

#[derive(Debug)]
pub enum EvalError {
    Downcast(TryFromBigIntError<BigInt>),
    BadOperation,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::Downcast(e) => write!(f, "failed to cast from BigInt: {e}"),
            EvalError::BadOperation => write!(f, "bad operation (division or remainder by zero)"),
        }
    }
}

impl std::error::Error for EvalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EvalError::Downcast(e) => Some(e),
            EvalError::BadOperation => None,
        }
    }
}

impl From<TryFromBigIntError<BigInt>> for EvalError {
    fn from(value: TryFromBigIntError<BigInt>) -> EvalError {
        Self::Downcast(value)
    }
}
