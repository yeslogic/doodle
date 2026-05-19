use num_traits::{CheckedAdd, CheckedDiv, CheckedMul};
use serde::Serialize;
use std::{
    num::TryFromIntError,
    ops::{Add, BitAnd, BitOr, Div, Mul, Shl, Shr, Sub},
};

/// Returns the number of significant bits in `x`
///
/// If `x` is `0`, returns `0`.
fn sig_bits(x: usize) -> usize {
    match x {
        0 => 0,
        _ => (x.ilog2() + 1) as usize,
    }
}

macro_rules! into_bounds {
    ( $($t:ty),+ ) => {
        $(
            impl From<$t> for Bounds {
                fn from(x: $t) -> Self {
                    Bounds {
                        min: x as usize,
                        max: Some(x as usize),
                    }
                }
            }

            impl From<std::ops::RangeInclusive<$t>> for Bounds {
                fn from(x: std::ops::RangeInclusive<$t>) -> Self {
                    let (min, max) = x.into_inner();
                    Bounds {
                        min: min as usize,
                        max: Some(max as usize),
                    }
                }
            }

            impl From<std::ops::RangeFrom<$t>> for Bounds {
                fn from(x: std::ops::RangeFrom<$t>) -> Self {
                    let min = x.start as usize;
                    Bounds { min, max: None }
                }
            }
        )+
    };
}

into_bounds!(u8, u16, u32, u64, usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub struct Bounds {
    pub(crate) min: usize,
    pub(crate) max: Option<usize>,
}

impl std::fmt::Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(max) = self.max {
            write!(f, "({}..={})", self.min, max)
        } else {
            write!(f, "({}..)", self.min)
        }
    }
}

impl Bounds {
    #[inline]
    #[must_use]
    /// Return the lowest value of the given `Bounds`.
    pub const fn min(&self) -> usize {
        self.min
    }

    #[inline]
    #[must_use]
    /// Return the highest value of the given `Bounds`, which will be `None` for unbounded ranges.
    pub const fn max(&self) -> Option<usize> {
        self.max
    }

    /// Partial implementation of `Bounds % Bounds`, which is only guaranteed to yield
    /// the expected answer when both arguments are exact-bounds.
    ///
    /// Because remainder operations over ranges do not always yield contiguous ranges
    /// in either direction (e.g. `3..=5 % 4 -> [3, 0, 1]`; `110 % 11..=12 -> [0, 2]`),
    /// this method returns `Bounds::any()` if either range is inexact.
    ///
    /// Provided that `lhs = Bounds::exact(x)`, and `rhs = Bounds::exact(y)`, the
    /// result should always be `Bounds::exact(x % y)`.
    ///
    /// # Panics
    ///
    /// Will panic if both ranges are exact but `rhs == 0`.
    ///
    /// Even if `rhs` contains `0` as its minimum, will return `Bounds::any`
    /// rather than panicking provided `rhs` is not `Bounds::exact(0)`.
    // REVIEW - should this be a `std::ops::Rem` impl?
    pub(crate) fn rem(lhs: Bounds, rhs: Bounds) -> Bounds {
        if let Some(y) = rhs.as_exact() {
            if let Some(x) = lhs.as_exact() {
                return Bounds::exact(x.checked_rem(y).unwrap());
            } else {
                // NOTE - some edge-cases may have general solutions but we won't attempt to solve them, for now
                log::debug!(
                    "cannot generally solve potentially unsound operation Bounds % usize: {lhs} % {y}"
                );
            }
        } else {
            // NOTE - some edge-cases may have general solutions but we won't attempt to solve them, for now
            log::debug!(
                "cannot generally solve potentially unsound operation Bounds % Bounds: {lhs} % {rhs}"
            );
        }
        Bounds::any()
    }
}

impl Bounds {
    pub const fn new(min: usize, max: usize) -> Bounds {
        Bounds {
            min,
            max: Some(max),
        }
    }

    /// Given a range `(x, x+N)`, returns the `(k, (y, y+N))` for the largest
    /// value of `k` such that:
    ///   * `k & (y + i) == 0 && k | (y + i) == x + i` for `i` in `0..=N`
    ///   * `k == 0 || k > y + N`
    ///
    /// In other words, `k` is the longest common bit-prefix of `x` and `x + N`, which is
    /// `0` if `x + N` has strictly more significant bits than `x`. Correspondingly,
    /// `y` is a bit-masked version of `x` that fills in the complementary bits of `x` not covered
    /// by `k`.
    ///
    /// If `k == 0`, the returned `Bounds` will be a duplicate of `self`.
    pub(crate) fn bitwise_bounds(&self) -> (usize, Bounds) {
        // Unbounded Bounds have no longest common prefix
        let Some(max) = self.max else {
            return (0, *self);
        };

        // Exact bounds give `k = x`, `y = 0`
        if self.min == max {
            return (self.min, Bounds::exact(0));
        }

        // Highest significant bit of the common prefix
        let Some(nbits) = self.nbits_exact() else {
            return (0, *self);
        };

        // Highest significant bit not in the common prefix
        let mbits = sig_bits(max ^ self.min);

        // length, in bits, of the common prefix
        let set_bits = nbits
            .checked_sub(mbits)
            .expect("mbits should never exceed nbits");

        // a zero-length bit-prefix will always be 0, so we can skip the math
        if set_bits == 0 {
            return (0, *self);
        }

        // bit-mask over x that constitutes k
        let hi_mask = ((1 << set_bits) - 1) << mbits;
        // bit-mask over x that constitutes y
        let lo_mask = (1 << mbits) - 1;

        let k = hi_mask & max;
        let y0 = self.min & lo_mask;
        let y1 = max & lo_mask;

        (k, Bounds::new(y0, y1))
    }

    /// Returns a new `Bounds` value that indicates the range of the number of significant bits in
    /// `self` over the range `min..=max`.
    pub(crate) fn significant_bits(&self) -> Bounds {
        Self {
            min: sig_bits(self.min),
            max: self.max.map(sig_bits),
        }
    }

    /// Returns the exact number of significant bits in both `self.min` and `self.max` if they match,
    /// `None` otherwise.
    pub(crate) fn nbits_exact(&self) -> Option<usize> {
        self.significant_bits().as_exact()
    }

    /// Returns a `Bounds` whose minimum and maximum are both `n`.
    pub const fn exact(n: usize) -> Bounds {
        Bounds {
            min: n,
            max: Some(n),
        }
    }

    // Returns a `Bounds` value covering all values from `0` to `max_val`.
    pub const fn at_most(max_val: usize) -> Bounds {
        let min = 0;
        let max = Some(max_val);
        Bounds { min, max }
    }

    /// Returns a `Bounds` whose minimum is the given `min` and with no upper limit.
    pub const fn at_least(min: usize) -> Bounds {
        Bounds { min, max: None }
    }

    /// Returns a `Bounds` spanning the entire range of `usize`.
    pub const fn any() -> Bounds {
        Bounds { min: 0, max: None }
    }

    /// Returns `Some(n)` if `self` describes only a single exact value `n`, `None` otherwise.
    pub fn as_exact(&self) -> Option<usize> {
        match self.max {
            Some(n) if n == self.min => Some(n),
            _ => None,
        }
    }

    /// Returns `true` if the value `n` falls within the implicit range of `self`.
    pub fn contains(&self, n: usize) -> bool {
        n >= self.min
            && match self.max {
                Some(m) => n <= m,
                _ => true,
            }
    }

    /// Takes a conservative union over two `Bounds` objects, returning a new `Bounds`
    /// that has the lower of the two lower bounds and the higher of the two upper bounds.
    pub fn union(lhs: Bounds, rhs: Bounds) -> Bounds {
        Bounds {
            min: usize::min(lhs.min, rhs.min),
            max: match (lhs.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(usize::max(m1, m2)),
                _ => None,
            },
        }
    }

    /// Clamps `self` to the range `min..=max`.
    ///
    /// If the clamped minimum ends up greater than the clamped maximum, returns `None`.
    ///
    /// Otherwise, returns `Some(x)` where `x` is a `Bounds` that covers the intersection
    /// of the ranges described by `self`, and `min..=max`.
    ///
    /// # Panics
    ///
    /// Will panic if the provided `min` is greater than the provided `max`.
    pub fn clamp(self, min: usize, max: usize) -> Option<Bounds> {
        assert!(min <= max, "Bounds::clamp: min ({min}) > max ({max})");
        let min = usize::max(min, self.min);
        let max = self.max.map_or(max, |m| usize::min(m, max));
        if min > max {
            return None;
        }
        Some(Bounds {
            min,
            max: Some(max),
        })
    }

    /// Converts a `Bounds` that measures in increments of individual bits into the equivalent range
    /// of byte-precision values.
    pub fn bits_to_bytes(&self) -> Bounds {
        Bounds {
            min: self.min.div_ceil(8),
            max: self.max.map(|n| n.div_ceil(8)),
        }
    }

    /// Finds the pair `(x, m)` that yields the maximal `x & m` for `x` in the bounds of `self`.
    ///
    /// May fudge the value of `x` to an out-of-range value to shortcut the computation of `x & m`
    /// over all in-range values, with the guarantee that the resulting bitwise-and will be no less
    /// than the true maximum of `y & m` for actually-in-range `y`.
    fn best_mask(&self, m: usize) -> (usize, usize) {
        if self.contains(m) {
            (m, m)
        } else {
            match self.max {
                Some(n) => (mask(n), m),
                _ => (m, m),
            }
        }
    }
}

macro_rules! try_from_bounds {
    ( $( $t:ident ),+ ) => {
        $(
            impl TryFrom<Bounds> for ($t, Option<$t>) {
                type Error = TryFromIntError;

                fn try_from(value: Bounds) -> Result<Self, Self::Error> {
                    Ok((value.min.try_into()?, value.max.map(|n| n.try_into()).transpose()?))
                }
            }
        )+
    };
}

try_from_bounds!(u8, u16, u32, u64);

impl CheckedAdd for Bounds {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(Bounds {
            min: self.min.checked_add(v.min)?,
            max: match (self.max, v.max) {
                (Some(m1), Some(m2)) => m1.checked_add(m2),
                _ => None,
            },
        })
    }
}

impl Add for Bounds {
    type Output = Self;

    fn add(self, rhs: Bounds) -> Bounds {
        self.checked_add(&rhs).unwrap_or(Bounds::any())
    }
}

impl Sub for Bounds {
    type Output = Self;

    fn sub(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: match rhs.max {
                Some(m2) => self.min.saturating_sub(m2),
                None => 0,
            },
            max: self.max.map(|m1| m1.saturating_sub(rhs.min)),
        }
    }
}

impl CheckedMul for Bounds {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(Bounds {
            min: self.min.checked_mul(v.min)?,
            max: match (self.max, v.max) {
                (Some(m1), Some(m2)) => m1.checked_mul(m2),
                _ => None,
            },
        })
    }
}

impl Mul for Bounds {
    type Output = Self;

    fn mul(self, rhs: Bounds) -> Bounds {
        self.checked_mul(&rhs).unwrap_or(Bounds::any())
    }
}

impl CheckedDiv for Bounds {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        Some(Bounds {
            min: match v.max {
                Some(m2) => self.min.checked_div(m2)?,
                None => 0,
            },
            max: self.max.map(|m1| m1 / usize::max(v.min, 1)),
        })
    }
}

impl Div<Bounds> for Bounds {
    type Output = Self;

    fn div(self, rhs: Bounds) -> Bounds {
        self.checked_div(&rhs).unwrap_or(Bounds::any())
    }
}

impl Bounds {
    pub fn checked_shl(self, rhs: Self) -> Option<Self> {
        let min2 = u32::try_from(rhs.min).ok()?;
        let min = self.min.checked_shl(min2)?;
        let max = match (self.max, rhs.max) {
            (Some(m1), Some(m2)) => {
                let max2 = u32::try_from(m2).ok()?;
                // REVIEW - there may be edge-cases here where max2 < 64 but m1 overflows, yielding unexpectedly small values
                m1.checked_shl(max2)
            }
            _ => None,
        };
        Some(Bounds { min, max })
    }

    pub fn checked_shr(self, rhs: Self) -> Option<Self> {
        let min = match rhs.max {
            Some(m2) => {
                let m = u32::try_from(m2).ok()?;
                self.min.checked_shr(m)?
            }
            None => 0,
        };
        let min0 = u32::try_from(rhs.min).ok()?;
        let max = self.max.and_then(|m1| m1.checked_shr(min0));
        Some(Bounds { min, max })
    }
}

impl Shl<Bounds> for Bounds {
    type Output = Self;

    fn shl(self, rhs: Bounds) -> Bounds {
        self.checked_shl(rhs).unwrap_or(Bounds::any())
    }
}

impl Shr<Bounds> for Bounds {
    type Output = Self;

    fn shr(self, rhs: Bounds) -> Bounds {
        self.checked_shr(rhs).unwrap_or(Bounds::any())
    }
}

impl BitOr<Bounds> for Bounds {
    type Output = Self;

    fn bitor(self, rhs: Bounds) -> Bounds {
        let (k1, lo_self) = self.bitwise_bounds();
        let (k2, lo_rhs) = rhs.bitwise_bounds();
        Bounds {
            min: usize::max(self.min, rhs.min),
            max: match (lo_self.max, lo_rhs.max) {
                (Some(m1), Some(m2)) => Some((k1 | k2) | (mask(m1) | mask(m2))),
                _ => None,
            },
        }
    }
}

impl BitAnd<Bounds> for Bounds {
    type Output = Self;

    fn bitand(self, rhs: Bounds) -> Bounds {
        let (k1, lo_self) = self.bitwise_bounds();
        let (k2, lo_rhs) = rhs.bitwise_bounds();
        Bounds {
            min: (k1 & k2),
            max: match (lo_self.max, lo_rhs.max) {
                (Some(m1), Some(m2)) => Some(match (lo_self.min == m1, lo_rhs.min == m2) {
                    (true, true) => blend_bits(k1, k2, m1, m2),
                    (true, false) => {
                        // NOTE - the order of x2 and m in blend_bits matters
                        let (x2, m) = lo_rhs.best_mask(m1);
                        blend_bits(k1, k2, m, x2)
                    }
                    (false, true) => {
                        // NOTE - the order of x1 and m in blend_bits matters
                        let (x1, m) = lo_self.best_mask(m2);
                        blend_bits(k1, k2, x1, m)
                    }
                    (false, false) => blend_bits(k1, k2, mask(m1), mask(m2)),
                }),
                _ => None,
            },
        }
    }
}

/// Blends the bits of two bit-prefixes along with the remaining bits of the sub-ranges they correspond to.
///
/// The argument order matters as the presumed-zero terms `k1 & m1` and `k2 & m2` are omitted from the computation
fn blend_bits(k1: usize, k2: usize, m1: usize, m2: usize) -> usize {
    (k1 & k2) | (m1 & m2) | (k1 & m2) | (m1 & k2)
}

fn mask(x: usize) -> usize {
    if x != 0 {
        1_usize.checked_shl(x.ilog2() + 1).unwrap() - 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn precise_bounds() -> prop::strategy::Union<BoxedStrategy<Bounds>> {
        Strategy::prop_union(
            any::<u8>().prop_map(|n| Bounds::exact(n as usize)).boxed(),
            any::<(u8, u8)>()
                .prop_map(|(min, length)| {
                    Bounds::new(min as usize, (min as usize) + (length as usize))
                })
                .boxed(),
        )
    }

    fn any_bounds() -> impl Strategy<Value = Bounds> {
        precise_bounds().or(any::<u8>()
            .prop_map(|n| Bounds::at_least(n as usize))
            .boxed())
    }

    proptest! {
        #[test]
        fn test_add(a in any::<u8>(), b in any::<u8>(), x in any_bounds(), y in any_bounds()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                !x.contains(a) ||
                !y.contains(b) ||
                (x + y).contains(a + b));
        }
    }

    proptest! {
        #[test]
        fn test_sub(a in any::<u8>(), b in any::<u8>(), x in any_bounds(), y in any_bounds()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                !x.contains(a) ||
                !y.contains(b) ||
                (x - y).contains(a.saturating_sub(b)));
        }
    }

    proptest! {
        #[test]
        fn test_mul(a in any::<u8>(), b in any::<u8>(), x in any_bounds(), y in any_bounds()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                !x.contains(a) ||
                !y.contains(b) ||
                (x * y).contains(a * b));
        }
    }

    proptest! {
        #[test]
        fn test_div(a in any::<u8>(), b in any::<u8>(), x in any_bounds(), y in any_bounds()) {
            let a = a as usize;
            let b = b as usize + 1;
            prop_assert!(
                !x.contains(a) ||
                !y.contains(b) ||
                (x / y).contains(a / b));
        }
    }

    proptest! {
        #[test]
        fn test_bitor(a in any::<u8>(), b in any::<u8>(), x in any_bounds(), y in any_bounds()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                !x.contains(a) ||
                !y.contains(b) ||
                (x | y).contains(a | b));
        }
    }

    proptest! {
        #[test]
        fn test_bitand(a in any::<u8>(), b in any::<u8>(), x in any_bounds(), y in any_bounds()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                !x.contains(a) ||
                !y.contains(b) ||
                (x & y).contains(a & b));
        }
    }

    proptest! {
        #[test]
        fn test_add_exact(a in any::<u8>(), b in any::<u8>()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) + Bounds::exact(b)).as_exact().unwrap() == a + b)
        }
    }

    proptest! {
        #[test]
        fn test_sub_exact(a in any::<u8>(), b in any::<u8>()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) - Bounds::exact(b)).as_exact().unwrap() == a.saturating_sub(b))
        }
    }

    proptest! {
        #[test]
        fn test_mul_exact(a in any::<u8>(), b in any::<u8>()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) * Bounds::exact(b)).as_exact().unwrap() == a * b)
        }
    }

    proptest! {
        #[test]
        fn test_div_exact(a in any::<u8>(), b in any::<u8>()) {
            let a = a as usize;
            let b = b as usize + 1;
            prop_assert!(
                (Bounds::exact(a) / Bounds::exact(b)).as_exact().unwrap() == a / b)
        }
    }

    proptest! {
        #[test]
        fn test_shl_exact(a in any::<u8>(), b in 0..8) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) << Bounds::exact(b)).as_exact().unwrap() == a << b)
        }
    }

    proptest! {
        #[test]
        fn test_shr_exact(a in any::<u8>(), b in 0..8) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) >> Bounds::exact(b)).as_exact().unwrap() == a >> b)
        }
    }

    proptest! {
        #[test]
        fn test_bitwise_bounds(bounds in precise_bounds()) {
            let a = bounds.min;
            let b = bounds.max.unwrap();
            let (k, sub_bounds) = bounds.bitwise_bounds();
            let c = sub_bounds.min;
            let d = sub_bounds.max.unwrap();
            prop_assert!(
                ((k == 0 || k > d) && (k & c == 0 && k | c == a) && (k & d == 0 && k | d == b))
            )
        }
    }

    /*
        proptest! {
            #[test]
            fn test_bitor_exact(a in any::<u8>(), b in any::<u8>()) {
                let a = a as usize;
                let b = b as usize;
                prop_assert!(
                    (Bounds::exact(a) | Bounds::exact(b)).is_exact().unwrap() == a | b)
            }
        }

        proptest! {
            #[test]
            fn test_bitand_exact(a in any::<u8>(), b in any::<u8>()) {
                let a = a as usize;
                let b = b as usize;
                prop_assert!(
                    (Bounds::exact(a) & Bounds::exact(b)).is_exact().unwrap() == a & b)
            }
        }
    */
}
