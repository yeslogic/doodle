use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{BTreeMap, BTreeSet};

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
