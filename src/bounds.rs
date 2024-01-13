use serde::Serialize;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub struct Bounds {
    min: usize,
    max: Option<usize>,
}

impl Bounds {
    pub fn new(min: usize, max: Option<usize>) -> Bounds {
        Bounds { min, max }
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> Option<usize> {
        self.max
    }

    pub fn exact(n: usize) -> Bounds {
        Bounds {
            min: n,
            max: Some(n),
        }
    }

    pub fn is_exact(&self) -> Option<usize> {
        match self.max {
            Some(n) if n == self.min => Some(n),
            _ => None,
        }
    }

    pub fn contains(&self, n: usize) -> bool {
        n >= self.min
            && match self.max {
                Some(m) => n <= m,
                _ => true,
            }
    }

    pub fn union(lhs: Bounds, rhs: Bounds) -> Bounds {
        Bounds {
            min: usize::min(lhs.min, rhs.min),
            max: match (lhs.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(usize::max(m1, m2)),
                _ => None,
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
