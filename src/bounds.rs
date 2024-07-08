use serde::Serialize;
use std::{
    num::TryFromIntError,
    ops::{Add, BitAnd, BitOr, Div, Mul, Shl, Shr, Sub},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub struct Bounds {
    pub min: usize,
    pub max: Option<usize>,
}

impl Bounds {
    pub const fn new(min: usize, max: usize) -> Bounds {
        Bounds {
            min,
            max: Some(max),
        }
    }

    pub const fn exact(n: usize) -> Bounds {
        Bounds {
            min: n,
            max: Some(n),
        }
    }

    pub const fn at_least(min: usize) -> Bounds {
        Bounds { min, max: None }
    }

    pub const fn any() -> Bounds {
        Bounds { min: 0, max: None }
    }

    /// Returns `Some(n)` if `self` describes only a single exact value `n`, `None` otherwise.
    pub fn is_exact(&self) -> Option<usize> {
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

    /// Dual method to `union`, which keeps the most restrictive upper-bound of the
    /// two Bounds values rather than the least.
    pub fn intersection(lhs: Bounds, rhs: Bounds) -> Bounds {
        Bounds {
            min: usize::min(lhs.min, rhs.min),
            max: match (lhs.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(usize::min(m1, m2)),
                (Some(m1), None) => Some(m1),
                _ => rhs.max,
            },
        }
    }

    pub fn bits_to_bytes(&self) -> Bounds {
        Bounds {
            min: (self.min + 7) / 8,
            max: self.max.map(|n| (n + 7) / 8),
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

impl Add for Bounds {
    type Output = Self;

    fn add(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: self.min.checked_add(rhs.min).unwrap(),
            max: match (self.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(m1.checked_add(m2).unwrap()),
                _ => None,
            },
        }
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
            max: match self.max {
                Some(m1) => Some(m1.saturating_sub(rhs.min)),
                None => None,
            },
        }
    }
}

impl Mul<Bounds> for Bounds {
    type Output = Self;

    fn mul(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: self.min.checked_mul(rhs.min).unwrap(),
            max: match (self.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(m1.checked_mul(m2).unwrap()),
                _ => None,
            },
        }
    }
}

impl Div<Bounds> for Bounds {
    type Output = Self;

    fn div(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: match rhs.max {
                Some(m2) => self.min.checked_div(m2).unwrap(),
                None => 0,
            },
            max: match self.max {
                Some(m1) => Some(m1 / usize::max(rhs.min, 1)),
                _ => None,
            },
        }
    }
}

impl Shl<Bounds> for Bounds {
    type Output = Self;

    fn shl(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: self
                .min
                .checked_shl(u32::try_from(rhs.min).unwrap())
                .unwrap(),
            max: match (self.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(m1.checked_shl(u32::try_from(m2).unwrap()).unwrap()),
                _ => None,
            },
        }
    }
}

impl Shr<Bounds> for Bounds {
    type Output = Self;

    fn shr(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: match rhs.max {
                Some(m2) => self.min.checked_shr(u32::try_from(m2).unwrap()).unwrap(),
                None => 0,
            },
            max: match self.max {
                Some(m1) => Some(m1.checked_shr(u32::try_from(rhs.min).unwrap()).unwrap()),
                _ => None,
            },
        }
    }
}

impl BitOr<Bounds> for Bounds {
    type Output = Self;

    fn bitor(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: usize::max(self.min, rhs.min),
            max: match (self.max, rhs.max) {
                (Some(m1), Some(m2)) => {
                    if self.min == m1 || rhs.min == m2 {
                        Some(m1 | m2)
                    } else {
                        Some(mask(m1) | mask(m2))
                    }
                }
                _ => None,
            },
        }
    }
}

impl BitAnd<Bounds> for Bounds {
    type Output = Self;

    fn bitand(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: 0,
            max: match (self.max, rhs.max) {
                (Some(m1), Some(m2)) => {
                    if self.min == m1 || rhs.min == m2 {
                        Some(m1 & m2)
                    } else {
                        Some(mask(m1) & mask(m2))
                    }
                }
                _ => None,
            },
        }
    }
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

    fn any_bounds() -> impl Strategy<Value = Bounds> {
        Strategy::prop_union(
            any::<u8>().prop_map(|n| Bounds::exact(n as usize)).boxed(),
            any::<(u8, u8)>()
                .prop_map(|(min, length)| {
                    Bounds::new(min as usize, (min as usize) + (length as usize))
                })
                .boxed(),
        )
        .or(any::<u8>()
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
                (Bounds::exact(a) + Bounds::exact(b)).is_exact().unwrap() == a + b)
        }
    }

    proptest! {
        #[test]
        fn test_sub_exact(a in any::<u8>(), b in any::<u8>()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) - Bounds::exact(b)).is_exact().unwrap() == a.saturating_sub(b))
        }
    }

    proptest! {
        #[test]
        fn test_mul_exact(a in any::<u8>(), b in any::<u8>()) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) * Bounds::exact(b)).is_exact().unwrap() == a * b)
        }
    }

    proptest! {
        #[test]
        fn test_div_exact(a in any::<u8>(), b in any::<u8>()) {
            let a = a as usize;
            let b = b as usize + 1;
            prop_assert!(
                (Bounds::exact(a) / Bounds::exact(b)).is_exact().unwrap() == a / b)
        }
    }

    proptest! {
        #[test]
        fn test_shl_exact(a in any::<u8>(), b in 0..8) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) << Bounds::exact(b)).is_exact().unwrap() == a << b)
        }
    }

    proptest! {
        #[test]
        fn test_shr_exact(a in any::<u8>(), b in 0..8) {
            let a = a as usize;
            let b = b as usize;
            prop_assert!(
                (Bounds::exact(a) >> Bounds::exact(b)).is_exact().unwrap() == a >> b)
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
