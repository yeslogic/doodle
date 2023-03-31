use std::{fmt, ops};

/// Compact, allocation-free set of `u8`s.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ByteSet {
    /// Each bit of the array represents a number in the range `0..=255`
    bits: [u64; 4],
}

impl ByteSet {
    pub fn new() -> ByteSet {
        ByteSet::empty()
    }

    pub fn from_bits(bits: [u64; 4]) -> ByteSet {
        ByteSet { bits }
    }

    pub fn empty() -> ByteSet {
        ByteSet::from_bits([0; 4])
    }

    pub fn full() -> ByteSet {
        ByteSet::from_bits([u64::MAX; 4])
    }

    pub fn iter<'this>(&'this self) -> impl 'this + Iterator<Item = u8> {
        (0..=255).filter(|b| self.contains(*b))
    }

    fn get_bit_with<T>(&self, b: u8, f: impl FnOnce(u64, u8) -> T) -> T {
        match b {
            0..=63 => f(self.bits[0], b),
            64..=127 => f(self.bits[1], b - 64),
            128..=191 => f(self.bits[2], b - 128),
            192..=255 => f(self.bits[3], b - 192),
        }
    }

    fn set_bit_with(&mut self, b: u8, f: impl FnOnce(&mut u64, u8)) {
        match b {
            0..=63 => f(&mut self.bits[0], b),
            64..=127 => f(&mut self.bits[1], b - 64),
            128..=191 => f(&mut self.bits[2], b - 128),
            192..=255 => f(&mut self.bits[3], b - 192),
        }
    }

    fn map_bits(&self, f: impl Fn(u64) -> u64) -> ByteSet {
        ByteSet::from_bits([
            f(self.bits[0]),
            f(self.bits[1]),
            f(self.bits[2]),
            f(self.bits[3]),
        ])
    }

    fn zip_bits_with(&self, other: &ByteSet, f: impl Fn(u64, u64) -> u64) -> ByteSet {
        ByteSet::from_bits([
            f(self.bits[0], other.bits[0]),
            f(self.bits[1], other.bits[1]),
            f(self.bits[2], other.bits[2]),
            f(self.bits[3], other.bits[3]),
        ])
    }

    pub fn len(&self) -> u32 {
        self.bits[0].count_ones()
            + self.bits[1].count_ones()
            + self.bits[2].count_ones()
            + self.bits[3].count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, b: u8) {
        self.set_bit_with(b, |bits, i| *bits |= 1 << i);
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, b: u8) {
        self.set_bit_with(b, |byte, i| *byte &= !(1 << i));
    }

    pub fn contains(&self, b: u8) -> bool {
        self.get_bit_with(b, |bits, i| bits & (1 << i) != 0)
    }

    pub fn complement(&self) -> ByteSet {
        self.map_bits(|bits| !bits)
    }

    pub fn union(&self, other: &ByteSet) -> ByteSet {
        ByteSet::zip_bits_with(&self, other, |bits0, bits1| bits0 | bits1)
    }

    pub fn intersection(&self, other: &ByteSet) -> ByteSet {
        ByteSet::zip_bits_with(&self, other, |bits0, bits1| bits0 & bits1)
    }

    pub fn is_disjoint(&self, other: &ByteSet) -> bool {
        ByteSet::intersection(self, other).is_empty()
    }
}

impl<const LEN: usize> From<[u8; LEN]> for ByteSet {
    fn from(bytes: [u8; LEN]) -> ByteSet {
        let mut bs = ByteSet::new();
        for b in bytes {
            bs.insert(b);
        }
        bs
    }
}

impl fmt::Debug for ByteSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl ops::Not for ByteSet {
    type Output = ByteSet;

    fn not(self) -> ByteSet {
        self.complement()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest::sample;

    use super::*;

    fn any_byte_set() -> impl Strategy<Value = ByteSet> {
        Strategy::prop_union(
            sample::select(vec![ByteSet::empty(), ByteSet::full()]).boxed(),
            any::<[u64; 4]>().prop_map(ByteSet::from_bits).boxed(),
        )
    }

    mod is_empty {
        use super::*;

        #[test]
        fn test_empty() {
            assert!(ByteSet::empty().is_empty());
        }

        #[test]
        fn test_full() {
            assert!(!ByteSet::full().is_empty());
        }

        proptest! {
            #[test]
            fn test_any(b in any::<u8>()) {
                assert!(!ByteSet::from([b]).is_empty())
            }
        }
    }

    mod contains {
        use super::*;

        proptest! {
            #[test]
            fn test_any(b in any::<u8>()) {
                assert!(ByteSet::from([b]).contains(b));
            }

            #[test]
            fn test_insert(b in any::<u8>(), mut bs in any_byte_set()) {
                bs.insert(b);
                assert!(bs.contains(b));
            }

            #[test]
            fn test_remove(b in any::<u8>(), mut bs in any_byte_set()) {
                bs.remove(b);
                assert!(!bs.contains(b));
            }
        }
    }

    #[test]
    fn is_disjoint() {
        let not_00 = !ByteSet::from([0x00]);
        let not_255 = !ByteSet::from([0xFF]);
        let is_00 = ByteSet::from([0x00]);
        let is_255 = ByteSet::from([0xFF]);
        assert!(!ByteSet::is_disjoint(&not_00, &not_00));
        assert!(!ByteSet::is_disjoint(&not_00, &not_255));
        assert!(ByteSet::is_disjoint(&not_00, &is_00));
        assert!(!ByteSet::is_disjoint(&not_00, &is_255));
        assert!(ByteSet::is_disjoint(&is_00, &is_255));
    }
}
