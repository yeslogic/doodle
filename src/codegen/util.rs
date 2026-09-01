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
            &TypedPattern::ZConst(..) => {
                unreachable!("IntCoverage does not support ZConst yet");
            }
            &TypedPattern::ZRange(..) => {
                unreachable!("IntCoverage does not support ZRange yet");
            }
            _ => unreachable!("unexpected pattern for IntCoverage: {pat:?}"),
        }
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
