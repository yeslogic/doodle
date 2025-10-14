use std::collections::BTreeMap as Map;

/// Helper trait for marking container-types that can be considered "null"
pub trait IsNull {
    /// Returns `true` if the container is empty.
    fn is_null(&self) -> bool;
}

impl<T> IsNull for Vec<T> {
    fn is_null(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsNull for Option<T> {
    fn is_null(&self) -> bool {
        self.is_none()
    }
}

/// Abstraction of a linear array of `Vec<T>` optimized for the vast majority of entries
/// being empty-vectors.
#[derive(Clone, Debug)]
pub(crate) struct IxHeap<T: IsNull> {
    // FIXME - implement a more efficient storage container
    _store: Map<usize, T>,
    _len: usize,
}

impl<T: IsNull + serde::Serialize> serde::Serialize for IxHeap<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self._store.serialize(serializer)
    }
}

impl<T: IsNull> IxHeap<T> {
    /// Constructs a new, empty `IxHeap`.
    pub const fn new() -> Self {
        Self {
            _store: Map::new(),
            _len: 0,
        }
    }

    /// Returns the logical length of an `IxHeap`, which includes empty entries.
    pub const fn len(&self) -> usize {
        self._len
    }

    /// Pushes an element into the `IxHeap`.
    ///
    /// If the element is null, only the length is changed, and no element is stored.
    pub fn push(&mut self, elem: T) {
        if elem.is_null() {
            self.push_empty()
        } else {
            self.push_nonempty(elem)
        }
    }

    fn push_empty(&mut self) {
        self._len += 1;
    }

    fn push_nonempty(&mut self, elem: T) {
        self._store.insert(self._len, elem);
        self._len += 1;
    }
}

impl<T> IxHeap<Vec<T>> {
    /// Retrieves the element at index `ix`, or an empty slice if it does not exist.
    ///
    /// # Notes
    ///
    /// Does not necessarily check whether `ix` is in-bounds, and therefore should not be relied upon
    /// as an indicator of validity.
    pub fn get(&self, ix: usize) -> &[T] {
        debug_assert!(ix < self._len);
        if self._store.contains_key(&ix) {
            &self._store[&ix]
        } else {
            &[]
        }
    }
}

impl<T> std::ops::Index<usize> for IxHeap<Vec<T>> {
    type Output = [T];

    fn index(&self, ix: usize) -> &[T] {
        self.get(ix)
    }
}
