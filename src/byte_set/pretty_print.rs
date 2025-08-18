use super::ByteSet;
use crate::{
    Label,
    helper::ascii::{
        ASCII_CHAR_STRICT, ASCII_DECIMAL_RANGE, ASCII_HEX_ANY, ASCII_HEX_LOWER, ASCII_HEX_UPPER,
        ASCII_OCTAL_RANGE, VALID_ASCII,
    },
};
use std::{collections::BTreeMap, ops::RangeInclusive};
use vec_collections::VecSet;

type RankSet<T> = BTreeMap<u8, VecSet<[T; 16]>>;

#[derive(Clone)]
pub enum PartitionedByteSet {
    Contiguous(ByteSet),
    Partitioned(RankSet<u8>),
}

impl ByteSet {
    /// Returns the number of elements between the first and last byte in the set (inclusive).
    pub fn bounds(&self) -> Option<(u8, u8)> {
        match (self.min_elem(), self.max_elem()) {
            (Some(min), Some(max)) => Some((min, max)),
            (None, None) => None,
            _ => unreachable!(),
        }
    }

    /// Returns the number of elements between the first and last byte in the set (inclusive).
    pub fn span(&self) -> u32 {
        match (self.min_elem(), self.max_elem()) {
            (Some(min), Some(max)) => max as u32 - min as u32 + 1,
            _ => 0,
        }
    }

    pub fn contains_range(&self, range: RangeInclusive<u8>) -> bool {
        let range_set = ByteSet::from(range);
        let intersection = self.intersection(&range_set);
        intersection.len() == range_set.len()
    }

    pub fn is_contiguous(&self) -> bool {
        self.span() == self.len()
    }

    pub fn find_enclosing_range(&self, byte: u8) -> Option<(u8, u8)> {
        if self.contains(byte) {
            let mut lo = byte;
            let mut hi = byte;

            while lo > 0 && self.contains(lo - 1) {
                lo -= 1;
            }

            while hi < 255 && self.contains(hi + 1) {
                hi += 1;
            }

            Some((lo, hi))
        } else {
            None
        }
    }

    /// Attempts to refactor the byte-set by removing outliers until it is easily
    /// expressed as a simple range
    pub fn partition_ranges(&self) -> PartitionedByteSet {
        if self.is_contiguous() {
            PartitionedByteSet::Contiguous(*self)
        } else {
            let mut this = *self;
            let mut ranked_ranges = RankSet::<u8>::new();

            while let Some(byte) = this.min_elem() {
                let (lo, hi) = this.find_enclosing_range(byte).unwrap();
                let rank = (hi - lo + 1) as u8;
                let set = lo..=hi;
                ranked_ranges.entry(rank).or_default().insert(lo);
                this = this.difference(&ByteSet::from(set));
            }

            PartitionedByteSet::Partitioned(ranked_ranges)
        }
    }

    pub fn is_equiv<Q>(&self, set: Q) -> bool
    where
        ByteSet: From<Q>,
    {
        *self == ByteSet::from(set)
    }

    pub fn as_token(&self) -> Option<&'static str> {
        match () {
            _ if self.is_full() => Some("U8"),
            _ if self.is_equiv(VALID_ASCII) => Some("ASCII"),
            _ if self.is_equiv(ASCII_CHAR_STRICT) => Some("ASCII.Strict"),
            _ if self.is_equiv(ASCII_DECIMAL_RANGE) => Some("ASCII.Decimal"),
            _ if self.is_equiv(ASCII_HEX_ANY) => Some("ASCII.Hex"),
            _ if self.is_equiv(ASCII_HEX_LOWER) => Some("ASCII.HexLower"),
            _ if self.is_equiv(ASCII_HEX_UPPER) => Some("ASCII.HexUpper"),
            _ if self.is_equiv(ASCII_OCTAL_RANGE) => Some("ASCII.Octal"),
            _ => None,
        }
    }

    pub fn to_best_string(&self) -> Label {
        if let Some(token) = self.as_token() {
            token.into()
        } else {
            if let Some(range) = self.as_range_string() {
                range
            } else {
                self.str_fallback()
            }
        }
    }

    pub fn as_range_string(&self) -> Option<Label> {
        if self.is_contiguous() {
            match self.len() {
                // Avoid edge-cases
                0 | 1 | 255 | 256 => Some(self.__str_raw()),
                _ => {
                    let (lo, hi) = self.bounds().unwrap();
                    Some(Label::Owned(format!("⟦{lo},{hi}⟧")))
                }
            }
        } else {
            None
        }
    }

    fn str_fallback(&self) -> Label {
        const THRESHOLD: u8 = 4;
        let PartitionedByteSet::Partitioned(parts) = self.partition_ranges() else {
            unreachable!("unexpected contiguous set in str_fallback");
        };
        let mut tokens = Vec::new();
        let mut overflow = ByteSet::new();
        parts.into_iter().for_each(|(rank, mins)| {
            if rank >= THRESHOLD {
                mins.into_iter().for_each(|lo| {
                    tokens.push(format!("⟦{lo},{}⟧", lo + rank - 1));
                });
            } else {
                for lo in mins {
                    let hi = lo + rank - 1;
                    for i in lo..=hi {
                        overflow.insert(i);
                    }
                }
            }
        });
        if !overflow.is_empty() {
            tokens.push(overflow.__str_raw().to_string());
        }
        Label::Owned(tokens.join(" ∪ "))
    }

    fn __str_raw(&self) -> Label {
        match self.len() {
            0 => Label::Borrowed("∅"),
            1 => Label::Owned(format!("[= {}]", self.min_elem().unwrap())),
            2..=127 => {
                let mut buf = String::new();
                buf.push_str("{");
                let tmp = self
                    .iter()
                    .map(|b| format!("{b}"))
                    .collect::<Vec<String>>()
                    .join(", ");
                buf.push_str(&tmp);
                buf.push_str("}");
                Label::Owned(buf)
            }
            128..=254 => {
                let mut buf = String::new();
                buf.push_str("∁({");
                let tmp = (!self)
                    .iter()
                    .map(|b| format!("{b}"))
                    .collect::<Vec<String>>()
                    .join(", ");
                buf.push_str(&tmp);
                buf.push_str("})");
                Label::Owned(buf)
            }
            255 => Label::Owned(format!("[!= {}]", (!self).min_elem().unwrap())),
            256 => Label::Borrowed("U8"),
            _n => unreachable!("impossible ByteSet length: {_n}"),
        }
    }
}

impl std::fmt::Display for ByteSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_best_string())
    }
}
