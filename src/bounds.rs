use std::ops::{Add, Mul};

pub struct Bounds {
    pub min: usize,
    pub max: Option<usize>,
}

impl Bounds {
    pub fn new(min: usize, max: Option<usize>) -> Bounds {
        Bounds { min, max }
    }

    pub fn exact(n: usize) -> Bounds {
        Bounds {
            min: n,
            max: Some(n),
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
            min: self.min + rhs.min,
            max: match (self.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(m1 + m2),
                _ => None,
            },
        }
    }
}

impl Mul<Bounds> for Bounds {
    type Output = Self;

    fn mul(self, rhs: Bounds) -> Bounds {
        Bounds {
            min: self.min * rhs.min,
            max: match (self.max, rhs.max) {
                (Some(m1), Some(m2)) => Some(m1 * m2),
                _ => None,
            },
        }
    }
}
