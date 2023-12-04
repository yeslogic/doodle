use std::ops::{Range, RangeInclusive};
use std::{fmt, ops};

use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

/// Compact, allocation-free set of `u8`s.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ByteSet {
    /// Each bit of the array represents a number in the range `0..=255`
    bits: [u64; 4],
}

const fn min_set_bit(x: u64) -> Option<u8> {
    match x.trailing_zeros() {
        64 => None,
        i => Some(i as u8),
    }
}

const fn bit_to_quad(bit: u8, offset: u8) -> u64 {
    if offset > bit {
        0
    } else {
        match 1u64.overflowing_shl((bit - offset) as u32) {
            (n, false) => n,
            (_, true) => 0,
        }
    }
}

impl ByteSet {
    pub const fn new() -> ByteSet {
        ByteSet::empty()
    }

    /// Construct a ByteSet that consists of all bytes `i` such that bit `i % 64` is set
    /// in the binary representation of the 64-bit value at index `i / 4` of the argument array `bits`.
    pub const fn from_bits(bits: [u64; 4]) -> ByteSet {
        ByteSet { bits }
    }

    pub const fn to_bits(&self) -> [u64; 4] {
        self.bits
    }

    pub const fn empty() -> ByteSet {
        ByteSet::from_bits([0; 4])
    }

    pub const fn singleton(bit: u8) -> ByteSet {
        let bits = [
            bit_to_quad(bit, 0),
            bit_to_quad(bit, 0x40),
            bit_to_quad(bit, 0x80),
            bit_to_quad(bit, 0xc0),
        ];
        ByteSet { bits }
    }

    pub const fn full() -> ByteSet {
        ByteSet::from_bits([u64::MAX; 4])
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = u8> {
        (0..=255).filter(|b| self.contains(*b))
    }

    pub const fn min_elem(&self) -> Option<u8> {
        if let Some(i) = min_set_bit(self.bits[0]) {
            return Some(i);
        }
        if let Some(i) = min_set_bit(self.bits[1]) {
            return Some(0x40 + i);
        }
        if let Some(i) = min_set_bit(self.bits[2]) {
            return Some(0x80 + i);
        }
        if let Some(i) = min_set_bit(self.bits[3]) {
            return Some(0xc0 + i);
        }
        None
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
        *self == ByteSet::empty()
    }

    pub fn is_full(&self) -> bool {
        *self == ByteSet::full()
    }

    pub fn insert(&mut self, b: u8) {
        self.set_bit_with(b, |bits, i| {
            *bits |= 1 << i;
        });
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, b: u8) {
        self.set_bit_with(b, |byte, i| {
            *byte &= !(1 << i);
        });
    }

    pub fn contains(&self, b: u8) -> bool {
        self.get_bit_with(b, |bits, i| (bits & (1 << i)) != 0)
    }

    pub fn complement(&self) -> ByteSet {
        self.map_bits(|bits| !bits)
    }

    pub fn union(&self, other: &ByteSet) -> ByteSet {
        ByteSet::zip_bits_with(self, other, |bits0, bits1| bits0 | bits1)
    }

    pub fn difference(&self, other: &ByteSet) -> ByteSet {
        ByteSet::zip_bits_with(self, other, |b0, b1| b0 & !b1)
    }

    pub fn intersection(&self, other: &ByteSet) -> ByteSet {
        ByteSet::zip_bits_with(self, other, |bits0, bits1| bits0 & bits1)
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

impl<'a> From<&'a [u8]> for ByteSet {
    fn from(bytes: &'a [u8]) -> ByteSet {
        let mut bs = ByteSet::new();
        for b in bytes {
            bs.insert(*b);
        }
        bs
    }
}

impl From<Range<u8>> for ByteSet {
    fn from(value: Range<u8>) -> Self {
        if value.end <= value.start {
            ByteSet::empty()
        } else {
            ByteSet::from(value.start..=value.end - 1)
        }
    }
}

impl From<RangeInclusive<u8>> for ByteSet {
    fn from(value: RangeInclusive<u8>) -> Self {
        // because the values are adjacent, we can optimize if they are within the same quadrant
        let lo = value.start();
        let hi = value.end();
        match lo.cmp(hi) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => return ByteSet::from([*lo]),
            std::cmp::Ordering::Greater => return ByteSet::empty(),
        }
        let start_ix = *lo >> 6;
        let end_ix = *hi >> 6;
        if start_ix == end_ix {
            let mut mask = 0u64;
            for i in value {
                mask |= 1 << (i & 63);
            }
            let bits = match start_ix {
                0 => [mask, 0, 0, 0],
                1 => [0, mask, 0, 0],
                2 => [0, 0, mask, 0],
                3 => [0, 0, 0, mask],
                _ => unreachable!("u8 / 64 not in range 0..=3 ??"),
            };
            ByteSet { bits }
        } else {
            // Fall-back to FromIter implementation, which is less efficient in general
            value.collect::<ByteSet>()
        }
    }
}

// REVIEW - there might be an optimization we can perform here
impl FromIterator<u8> for ByteSet {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut bs = ByteSet::empty();
        for byte in iter {
            bs.insert(byte);
        }
        bs
    }
}

impl fmt::Debug for ByteSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.len() < 128 {
            f.debug_set().entries(self.iter()).finish()
        } else {
            f.write_str("!")?;
            f.debug_set().entries((!self).iter()).finish()
        }
    }
}

impl ops::Not for &ByteSet {
    type Output = ByteSet;

    fn not(self) -> ByteSet {
        self.complement()
    }
}

impl ops::Not for ByteSet {
    type Output = ByteSet;

    fn not(self) -> ByteSet {
        !&self
    }
}

impl Serialize for ByteSet {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        struct All(ByteSet);

        impl Serialize for All {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let len = usize::try_from(self.0.len()).unwrap();
                let mut seq = serializer.serialize_seq(Some(len))?;
                for b in self.0.iter() {
                    seq.serialize_element(&b)?;
                }
                seq.end()
            }
        }

        let mut byte_set = serializer.serialize_struct("ByteSet", 2)?;
        if self.len() < 128 {
            byte_set.serialize_field("tag", "includes")?;
            byte_set.serialize_field("data", &All(*self))?;
        } else {
            byte_set.serialize_field("tag", "excludes")?;
            byte_set.serialize_field("data", &All(!*self))?;
        }
        byte_set.end()
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

    mod is_disjoint {
        use super::*;

        proptest! {
            #[test]
            fn test_complement(bs in any_byte_set()) {
                assert!(ByteSet::is_disjoint(&bs, &bs.complement()));
            }

            #[test]
            fn test_difference_left(bs0 in any_byte_set(), bs1 in any_byte_set()) {
                assert!(ByteSet::is_disjoint(&ByteSet::difference(&bs0, &bs1), &bs1));
            }

            #[test]
            fn test_difference_right(bs0 in any_byte_set(), bs1 in any_byte_set()) {
                assert!(ByteSet::is_disjoint(&bs0, &ByteSet::difference(&bs1, &bs0)));
            }
        }
    }

    #[test]
    fn test_debug_below_128() {
        assert_eq!(format!("{:?}", ByteSet::from([32, 1])), "{1, 32}");
    }

    #[test]
    fn test_debug_above_128() {
        assert_eq!(format!("{:?}", !ByteSet::from([32, 1])), "!{1, 32}");
    }

    mod same_result {
        use super::*;

        prop_compose! {
        fn range_in_quadrant(q: u8)
                            (start in (q*64)..=(q*64)+63)
                            (start in Just(start), end in start..=(q*64)+63) -> (u8, u8) {
                                (start, end)
                            }
                        }
        prop_compose! {
            fn range_any()(start in 0u8..=255)(start in Just(start), end in start..=255) -> (u8, u8) {
                (start, end)
            }
        }

        fn range_bounds() -> impl Strategy<Value = (u8, u8)> {
            prop_oneof![(0u8..=3).prop_flat_map(range_in_quadrant), range_any(),]
        }

        proptest! {
            #[test]
            fn single_quadrant((start, end) in range_bounds()) {
                let bs_range_inc = ByteSet::from(start..=end);
                // NOTE - hack to avoid overflowing integer bounds when end == 255
                let bs_range_exc = if end < 255 {
                    ByteSet::from(start..end+1)
                } else {
                    bs_range_inc.clone()
                };
                let bs_iter = (start..=end).collect::<ByteSet>();
                let bs_manual = {
                    let mut tmp = ByteSet::empty();
                    for i in start..=end {
                        tmp.insert(i);
                    }
                    tmp
                };
                prop_assert_eq!(bs_range_inc, bs_range_exc);
                prop_assert_eq!(bs_range_inc, bs_iter);
                prop_assert_eq!(bs_range_inc, bs_manual);
            }
        }
    }
}
