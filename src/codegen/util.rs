use num_bigint::BigInt;
use num_traits::Signed;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::RangeInclusive,
};

use crate::{
    bounds::Bounds,
    codegen::{
        rust_ast::NumType,
        typed_format::{GenType, TypedPattern},
    },
};

pub trait Selector {
    type Map<K, V>: Default;
    type Set<K>: Default;
}

pub struct BTree;
pub struct FxHash;

impl Selector for BTree {
    type Map<K, V> = BTreeMap<K, V>;
    type Set<K> = BTreeSet<K>;
}

impl Selector for FxHash {
    type Map<K, V> = FxHashMap<K, V>;
    type Set<K> = FxHashSet<K>;
}

pub type StableMap<K, V, S> = <S as Selector>::Map<K, V>;
#[expect(unused)]
pub type StableSet<K, S> = <S as Selector>::Set<K>;

pub trait MapLike<K, V> {
    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Eq + std::hash::Hash;

    fn index<Q>(&self, k: &Q) -> &V
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Eq + std::hash::Hash;
}

impl<K: Eq + std::hash::Hash, V> MapLike<K, V> for std::collections::HashMap<K, V> {
    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Eq + std::hash::Hash,
    {
        self.contains_key(k)
    }

    fn index<Q>(&self, k: &Q) -> &V
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Eq + std::hash::Hash,
    {
        <Self as std::ops::Index<&Q>>::index(self, k)
    }
}

impl<K: Eq + std::hash::Hash, V> MapLike<K, V> for FxHashMap<K, V> {
    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Eq + std::hash::Hash,
    {
        self.contains_key(k)
    }

    fn index<Q>(&self, k: &Q) -> &V
    where
        K: std::borrow::Borrow<Q>,
        Q: ?Sized + Eq + std::hash::Hash,
    {
        <Self as std::ops::Index<&Q>>::index(self, k)
    }
}

/// Internal constant that determines the maximum number of Ranges stored in IntCoverage before SmallVec heap-allocation kicks in.
///
/// This should be a conservative estimate of the maximum number of non-overlapping, non-contiguous ranges that a set of Patterns in a given Match would typically contain
const COVERAGE_RANGES: usize = 4;

#[derive(Debug)]
pub(crate) struct IntCoverage {
    covered: range_set::RangeSet<[RangeInclusive<usize>; COVERAGE_RANGES]>,
}

impl IntCoverage {
    pub fn new() -> Self {
        Self {
            covered: range_set::RangeSet::new(),
        }
    }

    pub(crate) fn add(&mut self, pat: &TypedPattern<GenType>) {
        match pat {
            TypedPattern::Wildcard(..) | TypedPattern::Binding(..) => unreachable!(
                "contains_irrefutable_pattern failed to short-circuit for pattern: {pat:?}"
            ),
            &TypedPattern::U8(i) => {
                self.covered.insert(i as usize);
            }
            &TypedPattern::U16(i) => {
                self.covered.insert(i as usize);
            }
            &TypedPattern::U32(i) => {
                self.covered.insert(i as usize);
            }
            &TypedPattern::U64(i) => {
                self.covered.insert(i as usize);
            }
            &TypedPattern::Int(ref rep, Bounds { min, max }) => {
                if let Some(max) = max {
                    self.covered.insert_range(min..=max);
                } else {
                    let max = match rep.try_to_num_type() {
                        Some(NumType::U(uint)) => uint.upper_bound(),
                        Some(NumType::I(..)) => {
                            unreachable!("TypedPattern::Int type-rep should not be signed: {rep:?}")
                        }
                        None => unreachable!(
                            "TypedPattern::Int type-rep is not a numeric type: {rep:?}"
                        ),
                    };
                    self.covered.insert_range(min..=max);
                }
            }
            TypedPattern::ZConst(_, n) => self.insert_z_range(n, n),
            TypedPattern::ZRange(_, range) => self.insert_z_range(&range.min, &range.max),
            _ => unreachable!("unexpected pattern for IntCoverage: {pat:?}"),
        }
    }

    /// Marks the values of `[min, max]` as covered, ignoring any part of the range outside `[0, usize::MAX]`;
    /// `IntCoverage` is only used for unsigned scrutinees, which cannot take negative values.
    fn insert_z_range(&mut self, min: &BigInt, max: &BigInt) {
        if max.is_negative() {
            return;
        }
        let zero = BigInt::from(0u8);
        let Ok(lo) = usize::try_from(std::cmp::max(min, &zero)) else {
            // `min` exceeds `usize::MAX`
            return;
        };
        let hi = usize::try_from(max).unwrap_or(usize::MAX);
        self.covered.insert_range(lo..=hi);
    }

    pub fn covers_all(&self, range: std::ops::RangeInclusive<usize>) -> bool {
        self.covered.contains_range(range)
    }
}

impl<'a> FromIterator<&'a TypedPattern<GenType>> for IntCoverage {
    fn from_iter<T: IntoIterator<Item = &'a TypedPattern<GenType>>>(iter: T) -> Self {
        let mut coverage = IntCoverage::new();
        for pat in iter {
            coverage.add(&pat);
        }
        coverage
    }
}
