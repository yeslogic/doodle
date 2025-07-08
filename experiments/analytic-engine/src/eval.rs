use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub, Signed, Zero};
use std::any::type_name;
use std::cell::LazyCell;
use std::ops::Neg;
use std::rc::Rc;

// SECTION - Evaluation model for arbitrary operations over machine integer types
#[derive(Clone)]
pub struct IndirectEval {
    value: Rc<LazyCell<BigInt, Box<dyn FnOnce() -> BigInt>>>,
}

impl std::fmt::Debug for IndirectEval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndirectEval")
            .field("value", &self.value)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub enum Eval<T> {
    NaN,
    Direct(T),
    Indirect(IndirectEval),
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

pub enum EvalError {
    Indirect(BigInt),
    NotANumber,
}

impl<T: Copy> Eval<T> {
    pub fn eval(&self) -> Result<T, EvalError> {
        match self {
            Eval::NaN => Err(EvalError::NotANumber),
            Eval::Direct(x) => Ok(*x),
            Eval::Indirect(IndirectEval { value, .. }) => {
                Err(EvalError::Indirect((&***value).clone()))
            }
        }
    }
}

// !SECTION

// SECTION - Binary Operation Backend

/// Macro for bulk definition of homogenously typed binary operations with a checked version provided by a trait (`num_traits`), where
/// `None`` indicates underflow or overflow, falling back on available saturating and wrapping variants of the same operation for indirect computations
macro_rules! homogenous {
    ( $( $tr:ident, $meth:ident => $( ( $fname:ident , $t:ty ) ),+ $(,)? );+ $(;)? ) => {
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

/// Macro for bulk definition of homogenous source-type typed binary operations where the output type can represent every possible input type, but the computation might not succeed
macro_rules! widening {
    ( $( $tr:ident, $meth:ident => $( ( $in_t:ty => $( ( $fname:ident, $out_t:ty ) ),+ $(,)? ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                $(
                    pub fn $fname(lhs: $in_t, rhs: $in_t) -> Eval<$out_t> {
                        match <$out_t as $tr>::$meth(&(lhs as $out_t), &(rhs as $out_t)) {
                            Some(res) => Eval::Direct(res),
                            None => Eval::Indirect(IndirectEval { value: Rc::new(LazyCell::new(Box::new(move || BigInt::$meth(&BigInt::from(lhs), &BigInt::from(rhs)).unwrap()))) }),
                        }
                    }
                )*
            )*
        )*
    };
}

macro_rules! widening_quotient {
    ( $( $tr:ident, $meth:ident => $( ( $in_t:ty => $( ( $fname:ident, $out_t:ty ) ),+ $(,)? ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                $(
                    pub fn $fname(lhs: $in_t, rhs: $in_t) -> Eval<$out_t> {
                        match <$out_t as $tr>::$meth(&(lhs as $out_t), &(rhs as $out_t)) {
                            Some(res) => Eval::Direct(res),
                            None => Eval::NaN,
                        }
                    }
                )*
            )*
        )*
    };
}

/// Macro for bulk definition of heterogenous source-type typed binary operations where the output type can represent every possible input type, but the computation might not succeed
macro_rules! mixed_widening {
    ( $( $tr:ident, $meth:ident => $( ( $left_t:ty, $right_t:ty => $( ( $fname:ident, $out_t:ty ) ),+ $(,)? ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                $(
                    pub fn $fname(lhs: $left_t, rhs: $right_t) -> Eval<$out_t> {
                        match <$out_t as $tr>::$meth(&(lhs as $out_t), &(rhs as $out_t)) {
                            Some(res) => Eval::Direct(res),
                            None => Eval::Indirect(IndirectEval { value: Rc::new(LazyCell::new(Box::new(move || BigInt::$meth(&BigInt::from(lhs), &BigInt::from(rhs)).unwrap()))) }),
                        }
                    }
                )*
            )*
        )*
    };
}

macro_rules! mixed_widening_quotient {
    ( $( $tr:ident, $meth:ident => $( ( $left_t:ty, $right_t:ty => $( ( $fname:ident, $out_t:ty ) ),+ $(,)? ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                $(
                    pub fn $fname(lhs: $left_t, rhs: $right_t) -> Eval<$out_t> {
                        match <$out_t as $tr>::$meth(&(lhs as $out_t), &(rhs as $out_t)) {
                            Some(res) => Eval::Direct(res),
                            None => Eval::NaN,
                        }
                    }
                )*
            )*
        )*
    };
}

// SECTION - strictly homogenous (T -> T -> T) binary operations (40 total)

// Responsible for 24 function definitions
homogenous! {
    CheckedAdd, checked_add =>
        (add_u8, u8),
        (add_u16, u16),
        (add_u32, u32),
        (add_u64, u64),
        (add_i8, i8),
        (add_i16, i16),
        (add_i32, i32),
        (add_i64, i64);
    CheckedSub, checked_sub =>
        (sub_u8, u8),
        (sub_u16, u16),
        (sub_u32, u32),
        (sub_u64, u64),
        (sub_i8, i8),
        (sub_i16, i16),
        (sub_i32, i32),
        (sub_i64, i64);
    CheckedMul, checked_mul =>
        (mul_u8, u8),
        (mul_u16, u16),
        (mul_u32, u32),
        (mul_u64, u64),
        (mul_i8, i8),
        (mul_i16, i16),
        (mul_i32, i32),
        (mul_i64, i64);
}

// (T, T) -> T binary operations involving quotients (failure is NaN)
// Responsible for 16 function definitions
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

// SECTION - heterogenous-type binary operations ( `(T0, T0) -> T1` or `(T0, T1) -> T` )
// (T0, T0) -> T1 binary operations where the values of T0 are a subset of T1 and failure indicates overflow/underflow

// responsible for 42 function definitions
widening! {
    CheckedAdd, checked_add =>
        (u8 => (add_u8_u16, u16), (add_u8_u32, u32), (add_u8_u64, u64), (add_u8_i16, i16), (add_u8_i32, i32), (add_u8_i64, i64)),
        (u16 => (add_u16_u32, u32), (add_u16_u64, u64), (add_u16_i32, i32), (add_u16_i64, i64)),
        (u32 => (add_u32_u64, u64), (add_u32_i64, i64)),
        (i8 => (add_i8_i16, i16), (add_i8_i32, i32), (add_i8_i64, i64)),
        (i16 => (add_i16_i32, i32), (add_i16_i64, i64)),
        (i32 => (add_i32_i64, i64));
    CheckedSub, checked_sub =>
        (u8 => (sub_u8_u16, u16), (sub_u8_u32, u32), (sub_u8_u64, u64), (sub_u8_i16, i16), (sub_u8_i32, i32), (sub_u8_i64, i64)),
        (u16 => (sub_u16_u32, u32), (sub_u16_u64, u64), (sub_u16_i32, i32), (sub_u16_i64, i64)),
        (u32 => (sub_u32_u64, u64), (sub_u32_i64, i64)),
        (i8 => (sub_i8_i16, i16), (sub_i8_i32, i32), (sub_i8_i64, i64)),
        (i16 => (sub_i16_i32, i32), (sub_i16_i64, i64)),
        (i32 => (sub_i32_i64, i64));
    CheckedMul, checked_mul =>
        (u8 => (mul_u8_u16, u16), (mul_u8_u32, u32), (mul_u8_u64, u64), (mul_u8_i16, i16), (mul_u8_i32, i32), (mul_u8_i64, i64)),
        (u16 => (mul_u16_u32, u32), (mul_u16_u64, u64), (mul_u16_i32, i32), (mul_u16_i64, i64)),
        (u32 => (mul_u32_u64, u64), (mul_u32_i64, i64)),
        (i8 => (mul_i8_i16, i16), (mul_i8_i32, i32), (mul_i8_i64, i64)),
        (i16 => (mul_i16_i32, i32), (mul_i16_i64, i64)),
        (i32 => (mul_i32_i64, i64));
}

// (T0, T0) -> T1 binary operations where the values of T0 are a subset of T1 and failure indicates NaN
// Responsible for 28 function definitions
widening_quotient! {
    CheckedDiv, checked_div =>
        // 14 functions
        (u8 => (div_u8_u16, u16), (div_u8_u32, u32), (div_u8_u64, u64), (div_u8_i16, i16), (div_u8_i32, i32), (div_u8_i64, i64)),
        (u16 => (div_u16_u32, u32), (div_u16_u64, u64), (div_u16_i32, i32), (div_u16_i64, i64)),
        (u32 => (div_u32_u64, u64), (div_u32_i64, i64)),
        (i8 => (div_i8_i16, i16), (div_i8_i32, i32), (div_i8_i64, i64)),
        (i16 => (div_i16_i32, i32), (div_i16_i64, i64)),
        (i32 => (div_i32_i64, i64));
    CheckedRem, checked_rem =>
        // 14 functions
        (u8 => (rem_u8_u16, u16), (rem_u8_u32, u32), (rem_u8_u64, u64), (rem_u8_i16, i16), (rem_u8_i32, i32), (rem_u8_i64, i64)), // 6
        (u16 => (rem_u16_u32, u32), (rem_u16_u64, u64), (rem_u16_i32, i32), (rem_u16_i64, i64)), // 4
        (u32 => (rem_u32_u64, u64), (rem_u32_i64, i64)), // 2
        (i8 => (rem_i8_i16, i16), (rem_i8_i32, i32), (rem_i8_i64, i64)), // 3
        (i16 => (rem_i16_i32, i32), (rem_i16_i64, i64)), // 2
        (i32 => (rem_i32_i64, i64)); // 1
}

// (T0, T1) -> T binary operations
// Responsible for 210 functions
mixed_widening! {
    CheckedAdd, checked_add =>
        // <= Bits8 (6 functions)
        (u8, i8 => (add_u8_i8_i16, i16), (add_u8_i8_i32, i32), (add_u8_i8_i64, i64)),
        (i8, u8 => (add_i8_u8_i16, i16), (add_i8_u8_i32, i32), (add_i8_u8_i64, i64)),

        // >Bits8, <= Bits16 (30 functions)
        (u16, u8 => (add_u16_u8_u16, u16), (add_u16_u8_u32, u32), (add_u16_u8_u64, u64), (add_u16_u8_i32, i32), (add_u16_u8_i64, i64)),
        (u8, u16 => (add_u8_u16_u16, u16), (add_u8_u16_u32, u32), (add_u8_u16_u64, u64), (add_u8_u16_i32, i32), (add_u8_u16_i64, i64)),
        (u16, i8 => (add_u16_i8_i32, i32), (add_u16_i8_i64, i64)),
        (i8, u16 => (add_i8_u16_i32, i32), (add_i8_u16_i64, i64)),

        (i16, u8 => (add_i16_u8_i16, i16), (add_i16_u8_i32, i32), (add_i16_u8_i64, i64)),
        (u8, i16 => (add_u8_i16_i16, i16), (add_u8_i16_i32, i32), (add_u8_i16_i64, i64)),
        (i16, i8 => (add_i16_i8_i16, i16), (add_i16_i8_i32, i32), (add_i16_i8_i64, i64)),
        (i8, i16 => (add_i8_i16_i16, i16), (add_i8_i16_i32, i32), (add_i8_i16_i64, i64)),

        (u16, i16 => (add_u16_i16_i32, i32), (add_u16_i16_i64, i64)),
        (i16, u16 => (add_i16_u16_i32, i32), (add_i16_u16_i64, i64)),

        // >Bits16, <=Bits32 (34 functions)
        (u32, u8 => (add_u32_u8_u32, u32), (add_u32_u8_u64, u64), (add_u32_u8_i64, i64)),
        (u8, u32 => (add_u8_u32_u32, u32), (add_u8_u32_u64, u64), (add_u8_u32_i64, i64)),
        (u32, i8 => (add_u32_i8_i64, i64)),
        (i8, u32 => (add_i8_u32_i64, i64)),

        (u32, u16 => (add_u32_u16_u32, u32), (add_u32_u16_u64, u64), (add_u32_u16_i64, i64)),
        (u16, u32 => (add_u16_u32_u32, u32), (add_u16_u32_u64, u64), (add_u16_u32_i64, i64)),
        (u32, i16 => (add_u32_i16_i64, i64)),
        (i16, u32 => (add_i16_u32_i64, i64)),

        (i32, u8 => (add_i32_u8_i32, i32), (add_i32_u8_i64, i64)),
        (u8, i32 => (add_u8_i32_i32, i32), (add_u8_i32_i64, i64)),
        (i32, i8 => (add_i32_i8_i32, i32), (add_i32_i8_i64, i64)),
        (i8, i32 => (add_i8_i32_i32, i32), (add_i8_i32_i64, i64)),

        (i32, u16 => (add_i32_u16_i32, i32), (add_i32_u16_i64, i64)),
        (u16, i32 => (add_u16_i32_i32, i32), (add_u16_i32_i64, i64)),
        (i32, i16 => (add_i32_i16_i32, i32), (add_i32_i16_i64, i64)),
        (i16, i32 => (add_i16_i32_i32, i32), (add_i16_i32_i64, i64)),

        (u32, i32 => (add_u32_i32_i64, i64)),
        (i32, u32 => (add_i32_u32_i64, i64)),
        ;
    CheckedSub, checked_sub =>
        // <= Bits8 (6 functions)
        (u8, i8 => (sub_u8_i8_i16, i16), (sub_u8_i8_i32, i32), (sub_u8_i8_i64, i64)),
        (i8, u8 => (sub_i8_u8_i16, i16), (sub_i8_u8_i32, i32), (sub_i8_u8_i64, i64)),

        // >Bits8, <= Bits16 (30 functions)
        (u16, u8 => (sub_u16_u8_u16, u16), (sub_u16_u8_u32, u32), (sub_u16_u8_u64, u64), (sub_u16_u8_i32, i32), (sub_u16_u8_i64, i64)),
        (u8, u16 => (sub_u8_u16_u16, u16), (sub_u8_u16_u32, u32), (sub_u8_u16_u64, u64), (sub_u8_u16_i32, i32), (sub_u8_u16_i64, i64)),
        (u16, i8 => (sub_u16_i8_i32, i32), (sub_u16_i8_i64, i64)),
        (i8, u16 => (sub_i8_u16_i32, i32), (sub_i8_u16_i64, i64)),

        (u8, i16 => (sub_u8_i16_i16, i16), (sub_u8_i16_i32, i32), (sub_u8_i16_i64, i64)),
        (i16, u8 => (sub_i16_u8_i16, i16), (sub_i16_u8_i32, i32), (sub_i16_u8_i64, i64)),
        (i8, i16 => (sub_i8_i16_i16, i16), (sub_i8_i16_i32, i32), (sub_i8_i16_i64, i64)),
        (i16, i8 => (sub_i16_i8_i16, i16), (sub_i16_i8_i32, i32), (sub_i16_i8_i64, i64)),

        (u16, i16 => (sub_u16_i16_i32, i32), (sub_u16_i16_i64, i64)),
        (i16, u16 => (sub_i16_u16_i32, i32), (sub_i16_u16_i64, i64)),

        // >Bits16, <= Bits32 (34 functions)
        (u32, u8 => (sub_u32_u8_u32, u32), (sub_u32_u8_u64, u64), (sub_u32_u8_i64, i64)),
        (u8, u32 => (sub_u8_u32_u32, u32), (sub_u8_u32_u64, u64), (sub_u8_u32_i64, i64)),
        (u32, i8 => (sub_u32_i8_i64, i64)),
        (i8, u32 => (sub_i8_u32_i64, i64)),

        (u32, u16 => (sub_u32_u16_u32, u32), (sub_u32_u16_u64, u64), (sub_u32_u16_i64, i64)),
        (u16, u32 => (sub_u16_u32_u32, u32), (sub_u16_u32_u64, u64), (sub_u16_u32_i64, i64)),
        (u32, i16 => (sub_u32_i16_i64, i64)),
        (i16, u32 => (sub_i16_u32_i64, i64)),

        (i32, u8 => (sub_i32_u8_i32, i32), (sub_i32_u8_i64, i64)),
        (u8, i32 => (sub_u8_i32_i32, i32), (sub_u8_i32_i64, i64)),
        (i32, i8 => (sub_i32_i8_i32, i32), (sub_i32_i8_i64, i64)),
        (i8, i32 => (sub_i8_i32_i32, i32), (sub_i8_i32_i64, i64)),

        (i32, u16 => (sub_i32_u16_i32, i32), (sub_i32_u16_i64, i64)),
        (u16, i32 => (sub_u16_i32_i32, i32), (sub_u16_i32_i64, i64)),
        (i32, i16 => (sub_i32_i16_i32, i32), (sub_i32_i16_i64, i64)),
        (i16, i32 => (sub_i16_i32_i32, i32), (sub_i16_i32_i64, i64)),

        (u32, i32 => (sub_u32_i32_i64, i64)),
        (i32, u32 => (sub_i32_u32_i64, i64));
    CheckedMul, checked_mul =>
        // <= Bits8 (6 functions)
        (u8, i8 => (mul_u8_i8_i16, i16), (mul_u8_i8_i32, i32), (mul_u8_i8_i64, i64)),
        (i8, u8 => (mul_i8_u8_i16, i16), (mul_i8_u8_i32, i32), (mul_i8_u8_i64, i64)),

        // >Bits8, <= Bits16 (30 functions)
        (u16, u8 => (mul_u16_u8_u16, u16), (mul_u16_u8_u32, u32), (mul_u16_u8_u64, u64), (mul_u16_u8_i32, i32), (mul_u16_u8_i64, i64)),
        (u8, u16 => (mul_u8_u16_u16, u16), (mul_u8_u16_u32, u32), (mul_u8_u16_u64, u64), (mul_u8_u16_i32, i32), (mul_u8_u16_i64, i64)),
        (u16, i8 => (mul_u16_i8_i32, i32), (mul_u16_i8_i64, i64)),
        (i8, u16 => (mul_i8_u16_i32, i32), (mul_i8_u16_i64, i64)),

        (u8, i16 => (mul_u8_i16_i16, i16), (mul_u8_i16_i32, i32), (mul_u8_i16_i64, i64)),
        (i16, u8 => (mul_i16_u8_i16, i16), (mul_i16_u8_i32, i32), (mul_i16_u8_i64, i64)),
        (i8, i16 => (mul_i8_i16_i16, i16), (mul_i8_i16_i32, i32), (mul_i8_i16_i64, i64)),
        (i16, i8 => (mul_i16_i8_i16, i16), (mul_i16_i8_i32, i32), (mul_i16_i8_i64, i64)),

        (u16, i16 => (mul_u16_i16_i32, i32), (mul_u16_i16_i64, i64)),
        (i16, u16 => (mul_i16_u16_i32, i32), (mul_i16_u16_i64, i64)),

        // >Bits16, <= Bits32 (34 functions)
        (u32, u8 => (mul_u32_u8_u32, u32), (mul_u32_u8_u64, u64), (mul_u32_u8_i64, i64)),
        (u8, u32 => (mul_u8_u32_u32, u32), (mul_u8_u32_u64, u64), (mul_u8_u32_i64, i64)),
        (u32, i8 => (mul_u32_i8_i64, i64)),
        (i8, u32 => (mul_i8_u32_i64, i64)),

        (u32, u16 => (mul_u32_u16_u32, u32), (mul_u32_u16_u64, u64), (mul_u32_u16_i64, i64)),
        (u16, u32 => (mul_u16_u32_u32, u32), (mul_u16_u32_u64, u64), (mul_u16_u32_i64, i64)),
        (u32, i16 => (mul_u32_i16_i64, i64)),
        (i16, u32 => (mul_i16_u32_i64, i64)),

        (i32, u8 => (mul_i32_u8_i32, i32), (mul_i32_u8_i64, i64)),
        (u8, i32 => (mul_u8_i32_i32, i32), (mul_u8_i32_i64, i64)),
        (i32, i8 => (mul_i32_i8_i32, i32), (mul_i32_i8_i64, i64)),
        (i8, i32 => (mul_i8_i32_i32, i32), (mul_i8_i32_i64, i64)),

        (i32, u16 => (mul_i32_u16_i32, i32), (mul_i32_u16_i64, i64)),
        (u16, i32 => (mul_u16_i32_i32, i32), (mul_u16_i32_i64, i64)),
        (i32, i16 => (mul_i32_i16_i32, i32), (mul_i32_i16_i64, i64)),
        (i16, i32 => (mul_i16_i32_i32, i32), (mul_i16_i32_i64, i64)),

        (u32, i32 => (mul_u32_i32_i64, i64)),
        (i32, u32 => (mul_i32_u32_i64, i64));
}

// Responsible for 140 functions
mixed_widening_quotient! {
    CheckedDiv, checked_div =>
        // <= Bits8 (6 functions)
        (u8, i8 => (div_u8_i8_i16, i16), (div_u8_i8_i32, i32), (div_u8_i8_i64, i64)),
        (i8, u8 => (div_i8_u8_i16, i16), (div_i8_u8_i32, i32), (div_i8_u8_i64, i64)),

        // >Bits8, <= Bits16 (30 functions)
        (u16, u8 => (div_u16_u8_u16, u16), (div_u16_u8_u32, u32), (div_u16_u8_u64, u64), (div_u16_u8_i32, i32), (div_u16_u8_i64, i64)),
        (u8, u16 => (div_u8_u16_u16, u16), (div_u8_u16_u32, u32), (div_u8_u16_u64, u64), (div_u8_u16_i32, i32), (div_u8_u16_i64, i64)),
        (u16, i8 => (div_u16_i8_i32, i32), (div_u16_i8_i64, i64)),
        (i8, u16 => (div_i8_u16_i32, i32), (div_i8_u16_i64, i64)),

        (i16, u8 => (div_i16_u8_i16, i16), (div_i16_u8_i32, i32), (div_i16_u8_i64, i64)),
        (u8, i16 => (div_u8_i16_i16, i16), (div_u8_i16_i32, i32), (div_u8_i16_i64, i64)),
        (i16, i8 => (div_i16_i8_i16, i16), (div_i16_i8_i32, i32), (div_i16_i8_i64, i64)),
        (i8, i16 => (div_i8_i16_i16, i16), (div_i8_i16_i32, i32), (div_i8_i16_i64, i64)),

        (u16, i16 => (div_u16_i16_i32, i32), (div_u16_i16_i64, i64)),
        (i16, u16 => (div_i16_u16_i32, i32), (div_i16_u16_i64, i64)),

        // >Bits16, <=Bits32 (34 functions)
        (u32, u8 => (div_u32_u8_u32, u32), (div_u32_u8_u64, u64), (div_u32_u8_i64, i64)),
        (u8, u32 => (div_u8_u32_u32, u32), (div_u8_u32_u64, u64), (div_u8_u32_i64, i64)),
        (u32, i8 => (div_u32_i8_i64, i64)),
        (i8, u32 => (div_i8_u32_i64, i64)),

        (u32, u16 => (div_u32_u16_u32, u32), (div_u32_u16_u64, u64), (div_u32_u16_i64, i64)),
        (u16, u32 => (div_u16_u32_u32, u32), (div_u16_u32_u64, u64), (div_u16_u32_i64, i64)),
        (u32, i16 => (div_u32_i16_i64, i64)),
        (i16, u32 => (div_i16_u32_i64, i64)),

        (i32, u8 => (div_i32_u8_i32, i32), (div_i32_u8_i64, i64)),
        (u8, i32 => (div_u8_i32_i32, i32), (div_u8_i32_i64, i64)),
        (i32, i8 => (div_i32_i8_i32, i32), (div_i32_i8_i64, i64)),
        (i8, i32 => (div_i8_i32_i32, i32), (div_i8_i32_i64, i64)),

        (i32, u16 => (div_i32_u16_i32, i32), (div_i32_u16_i64, i64)),
        (u16, i32 => (div_u16_i32_i32, i32), (div_u16_i32_i64, i64)),
        (i32, i16 => (div_i32_i16_i32, i32), (div_i32_i16_i64, i64)),
        (i16, i32 => (div_i16_i32_i32, i32), (div_i16_i32_i64, i64)),


        (u32, i32 => (div_u32_i32_i64, i64)),
        (i32, u32 => (div_i32_u32_i64, i64)),
        ;
    CheckedRem, checked_rem =>
        // <= Bits8 (6 functions)
        (u8, i8 => (rem_u8_i8_i16, i16), (rem_u8_i8_i32, i32), (rem_u8_i8_i64, i64)),
        (i8, u8 => (rem_i8_u8_i16, i16), (rem_i8_u8_i32, i32), (rem_i8_u8_i64, i64)),

        // >Bits8, <= Bits16 (30 functions)
        (u16, u8 => (rem_u16_u8_u16, u16), (rem_u16_u8_u32, u32), (rem_u16_u8_u64, u64), (rem_u16_u8_i32, i32), (rem_u16_u8_i64, i64)),
        (u8, u16 => (rem_u8_u16_u16, u16), (rem_u8_u16_u32, u32), (rem_u8_u16_u64, u64), (rem_u8_u16_i32, i32), (rem_u8_u16_i64, i64)),
        (u16, i8 => (rem_u16_i8_i32, i32), (rem_u16_i8_i64, i64)),
        (i8, u16 => (rem_i8_u16_i32, i32), (rem_i8_u16_i64, i64)),

        (u8, i16 => (rem_u8_i16_i16, i16), (rem_u8_i16_i32, i32), (rem_u8_i16_i64, i64)),
        (i16, u8 => (rem_i16_u8_i16, i16), (rem_i16_u8_i32, i32), (rem_i16_u8_i64, i64)),
        (i8, i16 => (rem_i8_i16_i16, i16), (rem_i8_i16_i32, i32), (rem_i8_i16_i64, i64)),
        (i16, i8 => (rem_i16_i8_i16, i16), (rem_i16_i8_i32, i32), (rem_i16_i8_i64, i64)),

        (u16, i16 => (rem_u16_i16_i32, i32), (rem_u16_i16_i64, i64)),
        (i16, u16 => (rem_i16_u16_i32, i32), (rem_i16_u16_i64, i64)),

        // >Bits16, <= Bits32 (34 functions)
        (u32, u8 => (rem_u32_u8_u32, u32), (rem_u32_u8_u64, u64), (rem_u32_u8_i64, i64)),
        (u8, u32 => (rem_u8_u32_u32, u32), (rem_u8_u32_u64, u64), (rem_u8_u32_i64, i64)),
        (u32, i8 => (rem_u32_i8_i64, i64)),
        (i8, u32 => (rem_i8_u32_i64, i64)),

        (u32, u16 => (rem_u32_u16_u32, u32), (rem_u32_u16_u64, u64), (rem_u32_u16_i64, i64)),
        (u16, u32 => (rem_u16_u32_u32, u32), (rem_u16_u32_u64, u64), (rem_u16_u32_i64, i64)),
        (u32, i16 => (rem_u32_i16_i64, i64)),
        (i16, u32 => (rem_i16_u32_i64, i64)),

        (i32, u8 => (rem_i32_u8_i32, i32), (rem_i32_u8_i64, i64)),
        (u8, i32 => (rem_u8_i32_i32, i32), (rem_u8_i32_i64, i64)),
        (i32, i8 => (rem_i32_i8_i32, i32), (rem_i32_i8_i64, i64)),
        (i8, i32 => (rem_i8_i32_i32, i32), (rem_i8_i32_i64, i64)),

        (i32, u16 => (rem_i32_u16_i32, i32), (rem_i32_u16_i64, i64)),
        (u16, i32 => (rem_u16_i32_i32, i32), (rem_u16_i32_i64, i64)),
        (i32, i16 => (rem_i32_i16_i32, i32), (rem_i32_i16_i64, i64)),
        (i16, i32 => (rem_i16_i32_i32, i32), (rem_i16_i32_i64, i64)),

        (u32, i32 => (rem_u32_i32_i64, i64)),
        (i32, u32 => (rem_i32_u32_i64, i64));
}
// !SECTION

// !SECTION

pub fn eval_fallback<Lhs, Rhs, Res>(
    lhs: Lhs,
    rhs: Rhs,
    _op_hint: &'static str,
    checked_op: impl FnOnce(&BigInt, &BigInt) -> Option<BigInt>,
) -> Eval<Res>
where
    BigInt: From<Lhs> + From<Rhs>,
    Res: TryFrom<BigInt, Error = TryFromBigIntError<BigInt>>,
{
    eprintln!("[INFO]: encountered fallback operation `{_op_hint} : (({}, {}) -> {})` that may benefit from standalone function",
         type_name::<Lhs>(), type_name::<Rhs>(), type_name::<Res>()
     );
    let big_l = BigInt::from(lhs);
    let big_r = BigInt::from(rhs);
    let o_big_res = checked_op(&big_l, &big_r);
    if let Some(big_res) = o_big_res {
        match <Res as TryFrom<BigInt>>::try_from(big_res) {
            Ok(res) => Eval::Direct(res),
            Err(e) => Eval::Indirect(IndirectEval {
                value: Rc::new(LazyCell::new(Box::new(move || e.into_original()))),
            }),
        }
    } else {
        Eval::NaN
    }
}

// !SECTION

// SECTION - Unary Operation Backend

/// Macro for negating via the `Neg` trait impl on a signed integer output type
macro_rules! negate_as_signed {
    ( $( $t:ty, $fname:ident );+ $(;)? ) => {
        $(
            pub fn $fname(x: $t) -> Eval<$t> {
                Eval::Direct(<$t as Neg>::neg(x))
            }
        )*
    };
    ( $( $t0:ty => $( ( $fname:ident, $t1:ty ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
               pub fn $fname(x: $t0) -> Eval<$t1> {
                    Eval::Direct(<$t1 as Neg>::neg(x as $t1))
               }
            )+
        )+
    };
}

/// Macro for negating when the output type is unsigned
///
/// Contains rules for unsigned inputs (i.e. Indirect if non-zero, zero otherwise)
/// as well as for arbitrary inputs via signed intermediate representations (i.e. Indirect if positive, negation otherwise)
macro_rules! negate_into_unsigned {
    // If positive, return Indirect immediately
    // Check if positive (and therefore has no representable result), otherwise take absolute value and cast
    ( $( $t:ty => $( ( $fname:ident, $out:ty ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(x: $t) -> Eval<$out> {
                    if <$t as ::num_traits::Signed>::is_positive(&x) {
                        Eval::Indirect(IndirectEval { value: Rc::new(LazyCell::new(Box::new(move || BigInt::from(x).neg()))) })
                    } else {
                        if x == <$t>::MIN {
                            // FIXME - this should work, but is there a more elegant way of doing this?
                            Eval::Direct(<$t>::MAX as $out + 1)
                        } else {
                            Eval::Direct(x.neg() as $out)
                        }
                    }
                }
            )*
        )*
    };
}

// NOTE - This is the only macro which can specify narrowing conversions since 0 is the only representable value regardless of width
macro_rules! negate_unsigned_unsigned {
    ( $( $t:ty => $( ( $fname:ident, $out:ty ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(x: $t) -> Eval<$out> {
                    if x.is_zero() {
                        Eval::Direct(<$out as Zero>::zero())
                    } else {
                        Eval::Indirect(IndirectEval { value: Rc::new(LazyCell::new(Box::new(move || ::num_bigint::BigInt::from(x).neg()))) })
                    }
                }
            )*
        )*
    };
}

/// Macro for abs via the `Signed` trait impl on a signed integer output type
macro_rules! abs_as_signed {
    ( $( $t:ty, $fname:ident );+ $(;)? ) => {
        $( pub fn $fname(x: $t) -> Eval<$t> {
            let tmp = <$t as ::num_traits::Signed>::abs(&x);
            // NOTE - Signed::abs(iX::MIN) == iX::MIN
            if tmp == <$t>::MIN {
                Eval::Indirect(IndirectEval { value: Rc::new(LazyCell::new(Box::new(move || ::num_bigint::BigInt::from(x).abs()))) })
            } else {
                Eval::Direct(tmp)
            }
        } )+
    };
    ( $( $t:ty => $( ( $fname:ident, $t1:ty ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(x: $t) -> Eval<$t1> {
                    let tmp = <$t1 as ::num_traits::Signed>::abs(&(x as $t1));
                    // We don't need to check this because the cast from $t cannot yield <$t1>::MIN
                    Eval::Direct(tmp)
                }
            )*
        )*
    };
}

/// Macro for taking absolute value via `Signed` impl on input followed by as-cast
macro_rules! abs_signed_unsigned {
    ( $( $t:ty =>  $( ( $fname:ident, $t1:ty) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(x: $t) -> Eval<$t1> {
                    let tmp = <$t as ::num_traits::Signed>::abs(&x);
                    if tmp == <$t>::MIN {
                        Eval::Direct(<$t>::MAX as $t1 + 1)
                    } else {
                        Eval::Direct(tmp as $t1)
                    }
                }
            )+
        )+
    };
}

/// Any 'abs' function that is operationally equivalent to a (safe) cast (i.e. unsigned input and wider output type)
macro_rules! abs_unsigned_unsigned {
    ( $( $t:ty, $fname:ident );+ $(;)? ) => {
        $(
            #[inline]
            pub const fn $fname(x: $t) -> Eval<$t> {
                Eval::Direct(x)
            }
        )+
    };
    ( $( $t:ty => $( ( $fname:ident, $t1:ty ) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                #[inline]
                pub fn $fname(x: $t) -> Eval<$t1> {
                    Eval::Direct(x as $t1)
                }
            )+
        )+
    };
}

// SECTION - Negation Functions

// Type-preserving negations on signed integers (4 functions)
negate_as_signed! {
    i8, neg_i8;
    i16, neg_i16;
    i32, neg_i32;
    i64, neg_i64;
}

// Negations into signed integers (12 functions)
negate_as_signed! {
    u8 => (neg_u8_i16, i16), (neg_u8_i32, i32), (neg_u8_i64, i64);
    u16 => (neg_u16_i32, i32), (neg_u16_i64, i64);
    u32 => (neg_u32_i64, i64);
    i8 => (neg_i8_i16, i16), (neg_i8_i32, i32), (neg_i8_i64, i64);
    i16 => (neg_i16_i32, i32), (neg_i16_i64, i64);
    i32 => (neg_i32_i64, i64);
}

// Sign-preserving negations between unsigned integers (16 functions)
negate_unsigned_unsigned! {
    u8 => (neg_u8, u8), (neg_u8_u16, u16), (neg_u8_u32, u32), (neg_u8_u64, u64);
    u16 => (neg_u16_u8, u8), (neg_u16, u16), (neg_u16_u32, u32), (neg_u16_u64, u64);
    u32 => (neg_u32_u8, u8), (neg_u32_u16, u16), (neg_u32, u32), (neg_u32_u64, u64);
    u64 => (neg_u64_u8, u8), (neg_u64_u16, u16), (neg_u64_u32, u32), (neg_u64, u64);
}

// Negation from signed integers into unsigned integers (10 functions)
negate_into_unsigned! {
    i8 => (neg_i8_u8, u8), (neg_i8_u16, u16), (neg_i8_u32, u32), (neg_i8_u64, u64);
    i16 => (neg_i16_u16, u16), (neg_i16_u32, u32), (neg_i16_u64, u64);
    i32 => (neg_i32_u32, u32), (neg_i32_u64, u64);
    i64 => (neg_i64_u64, u64);
}

/* 22 omitted negation functions involving lossy conversion
 *  - u64 -> T != uX         (4)
 *  - u32 -> T < i64, != uX  (3)
 *  - u16 -> T < i32, != uX  (2)
 *  - u8  -> i8              (1)
 *  - i64 -> T < ?64         (6)
 *  - i32 -> T < ?32         (4)
 *  - i16 -> T < ?16         (2)
 */

// !SECTION

// SECTION - Absolute-Value Functions

// Type-preserving abs on signed integers (4 functions)
abs_as_signed! {
    i8, abs_i8;
    i16, abs_i16;
    i32, abs_i32;
    i64, abs_i64;
}

// Abs into signed integers (12 functions)
abs_as_signed! {
    u8 => (abs_u8_i16, i16), (abs_u8_i32, i32), (abs_u8_i64, i64);
    u16 => (abs_u16_i32, i32), (abs_u16_i64, i64);
    u32 => (abs_u32_i64, i64);
    i8 => (abs_i8_i16, i16), (abs_i8_i32, i32), (abs_i8_i64, i64);
    i16 => (abs_i16_i32, i32), (abs_i16_i64, i64);
    i32 => (abs_i32_i64, i64);
}

// Abs from signed into unsigned integers (10 functions)
abs_signed_unsigned! {
    i8 => (abs_i8_u8, u8), (abs_i8_u16, u16), (abs_i8_u32, u32), (abs_i8_u64, u64);
    i16 => (abs_i16_u16, u16), (abs_i16_u32, u32), (abs_i16_u64, u64);
    i32 => (abs_i32_u32, u32), (abs_i32_u64, u64);
    i64 => (abs_i64_u64, u64);
}

// Type-preserving abs on unsigned integers (4 functions)
abs_unsigned_unsigned! {
    u8, abs_u8;
    u16, abs_u16;
    u32, abs_u32;
    u64, abs_u64;
}

// Sign-preserving abs on unsigned integers (6 functions)
abs_unsigned_unsigned! {
    u8 => (abs_u8_u16, u16), (abs_u8_u32, u32), (abs_u8_u64, u64);
    u16 => (abs_u16_u32, u32), (abs_u16_u64, u64);
    u32 => (abs_u32_u64, u64);
}

/* 28 omitted abs functions involving lossy conversion
 *  - u64 -> T != u64        (7)
 *  - u32 -> T < ?64, != u32 (5)
 *  - u16 -> T < ?32, != u16 (3)
 *  - u8  -> i8              (1)
 *  - i64 -> T < ?64         (6)
 *  - i32 -> T < ?32         (4)
 *  - i16 -> T < ?16         (2)
 */

// !SECTION

pub fn eval_unary_fallback<In, Out>(
    x: In,
    _op_hint: &'static str,
    op: impl FnOnce(&BigInt) -> BigInt,
) -> Eval<Out>
where
    BigInt: From<In>,
    Out: TryFrom<BigInt, Error = TryFromBigIntError<BigInt>>,
{
    eprintln!("[INFO]: encountered fallback operation `{_op_hint} : ({} -> {})` that may benefit from standalone function", type_name::<In>(), type_name::<Out>());
    let big_x = BigInt::from(x);
    let big_res = op(&big_x);
    match <Out as TryFrom<BigInt>>::try_from(big_res) {
        Ok(res) => Eval::Direct(res),
        Err(e) => Eval::Indirect(IndirectEval {
            value: Rc::new(LazyCell::new(Box::new(move || e.into_original()))),
        }),
    }
}
// !SECTION

// SECTION - Pure Casts

macro_rules! direct_cast {
    ($($from:ty => $( ($fname:ident, $to:ty) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(x: $from) -> Eval<$to> {
                    Eval::Direct(x as $to)
                }
            )+
        )+
    }
}

macro_rules! attempt_cast {
    ($($from:ty => $( ($fname:ident, $to:ty) ),+ $(,)? );+ $(;)? ) => {
        $(
            $(
                pub fn $fname(x: $from) -> Eval<$to> {
                    match <$to as ::num_traits::NumCast>::from(x) {
                        Some(res) => Eval::Direct(res),
                        None => Eval::Indirect(IndirectEval {
                            value: Rc::new(LazyCell::new(Box::new(move || BigInt::from(x)))),
                        })
                    }
                }
            )+
        )+
    };
}

// Identity casts [elided] (8 functions)

// Natural casts (excluding identity casts) (18 functions)
direct_cast! {
    u8 => (cast_u8_u16, u16), (cast_u8_u32, u32), (cast_u8_u64, u64), (cast_u8_i16, i16), (cast_u8_i32, i32), (cast_u8_i64, i64);
    u16 => (cast_u16_u32, u32), (cast_u16_u64, u64), (cast_u16_i32, i32), (cast_u16_i64, i64);
    u32 => (cast_u32_u64, u64), (cast_u32_i64, i64);
    i8 => (cast_i8_i16, i16), (cast_i8_i32, i32), (cast_i8_i64, i64);
    i16 => (cast_i16_i32, i32), (cast_i16_i64, i64);
    i32 => (cast_i32_i64, i64);
}

// Fallible casts (38 functions)
attempt_cast! {
    u8 => (cast_u8_i8, i8);
    u16 => (cast_u16_u8, u8), (cast_u16_i8, i8), (cast_u16_i16, i16);
    u32 => (cast_u32_u8, u8), (cast_u32_u16, u16), (cast_u32_i8, i8), (cast_u32_i16, i16), (cast_u32_i32, i32);
    u64 => (cast_u64_u8, u8), (cast_u64_u16, u16), (cast_u64_u32, u32), (cast_u64_i8, i8), (cast_u64_i16, i16), (cast_u64_i32, i32), (cast_u64_i64, i64);
    i8 => (cast_i8_u8, u8), (cast_i8_u16, u16), (cast_i8_u32, u32), (cast_i8_u64, u64);
    i16 => (cast_i16_u8, u8), (cast_i16_u16, u16), (cast_i16_u32, u32), (cast_i16_u64, u64), (cast_i16_i8, i8);
    i32 => (cast_i32_u8, u8), (cast_i32_u16, u16), (cast_i32_u32, u32), (cast_i32_u64, u64), (cast_i32_i8, i8), (cast_i32_i16, i16);
    i64 => (cast_i64_u8, u8), (cast_i64_u16, u16), (cast_i64_u32, u32), (cast_i64_u64, u64), (cast_i64_i8, i8), (cast_i64_i16, i16), (cast_i64_i32, i32);
}
// !SECTION
