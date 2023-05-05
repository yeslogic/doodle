use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BitSet {
    zero: bool,
    one: bool,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet::empty()
    }

    pub fn empty() -> BitSet {
        BitSet {
            zero: false,
            one: false,
        }
    }

    pub fn full() -> BitSet {
        BitSet {
            zero: true,
            one: true,
        }
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = bool> {
        [false, true].into_iter().filter(|b| self.contains(*b))
    }

    fn map_bits(&self, f: impl Fn(bool) -> bool) -> BitSet {
        BitSet {
            zero: f(self.zero),
            one: f(self.one),
        }
    }

    fn zip_bits_with(&self, other: &BitSet, f: impl Fn(bool, bool) -> bool) -> BitSet {
        BitSet {
            zero: f(self.zero, other.zero),
            one: f(self.one, other.one),
        }
    }

    pub fn len(&self) -> u32 {
        if self.zero && self.one {
            2
        } else if self.zero || self.one {
            1
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == BitSet::empty()
    }

    pub fn is_full(&self) -> bool {
        *self == BitSet::full()
    }

    pub fn insert(&mut self, b: bool) {
        match b {
            false => self.zero = true,
            true => self.one = true,
        }
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, b: bool) {
        match b {
            false => self.zero = false,
            true => self.one = false,
        }
    }

    pub fn contains(&self, b: bool) -> bool {
        match b {
            false => self.zero,
            true => self.one,
        }
    }

    pub fn complement(&self) -> BitSet {
        self.map_bits(|b| !b)
    }

    pub fn union(&self, other: &BitSet) -> BitSet {
        BitSet::zip_bits_with(self, other, |b0, b1| b0 || b1)
    }

    pub fn difference(&self, other: &BitSet) -> BitSet {
        BitSet::zip_bits_with(self, other, |b0, b1| b0 && !b1)
    }

    pub fn intersection(&self, other: &BitSet) -> BitSet {
        BitSet::zip_bits_with(self, other, |b0, b1| b0 && b1)
    }

    pub fn is_disjoint(&self, other: &BitSet) -> bool {
        BitSet::intersection(self, other).is_empty()
    }
}

impl<const LEN: usize> From<[bool; LEN]> for BitSet {
    fn from(bits: [bool; LEN]) -> BitSet {
        let mut bs = BitSet::new();
        for b in bits {
            bs.insert(b);
        }
        bs
    }
}

impl ops::Not for &BitSet {
    type Output = BitSet;

    fn not(self) -> BitSet {
        self.complement()
    }
}

impl ops::Not for BitSet {
    type Output = BitSet;

    fn not(self) -> BitSet {
        !&self
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest::sample;

    use super::*;

    fn any_byte_set() -> impl Strategy<Value = BitSet> {
        Strategy::prop_union(
            sample::select(vec![BitSet::empty(), BitSet::full()]).boxed(),
            any::<[bool; 1]>().prop_map(BitSet::from).boxed(),
        )
    }

    mod is_empty {
        use super::*;

        #[test]
        fn test_empty() {
            assert!(BitSet::empty().is_empty());
        }

        #[test]
        fn test_full() {
            assert!(!BitSet::full().is_empty());
        }

        proptest! {
            #[test]
            fn test_any(b in any::<bool>()) {
                assert!(!BitSet::from([b]).is_empty())
            }
        }
    }

    mod contains {
        use super::*;

        proptest! {
            #[test]
            fn test_any(b in any::<bool>()) {
                assert!(BitSet::from([b]).contains(b));
            }

            #[test]
            fn test_insert(b in any::<bool>(), mut bs in any_byte_set()) {
                bs.insert(b);
                assert!(bs.contains(b));
            }

            #[test]
            fn test_remove(b in any::<bool>(), mut bs in any_byte_set()) {
                bs.remove(b);
                assert!(!bs.contains(b));
            }
        }
    }

    mod is_disjoint {
        use super::*;

        proptest! {
            #[test]
            fn test_complement(bs in any_byte_set()) {
                assert!(BitSet::is_disjoint(&bs, &bs.complement()));
            }

            #[test]
            fn test_difference_left(bs0 in any_byte_set(), bs1 in any_byte_set()) {
                assert!(BitSet::is_disjoint(&BitSet::difference(&bs0, &bs1), &bs1));
            }

            #[test]
            fn test_difference_right(bs0 in any_byte_set(), bs1 in any_byte_set()) {
                assert!(BitSet::is_disjoint(&bs0, &BitSet::difference(&bs1, &bs0)));
            }
        }
    }
}
