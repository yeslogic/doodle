use std::{collections::BTreeSet, ops::Index};

#[derive(Clone)]
pub struct Wec<T> {
    _store: Vec<T>,
    width: usize,
}

impl<T> std::fmt::Debug for Wec<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter_rows()).finish()
    }
}

impl<T> Index<usize> for Wec<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        assert!(
            self.width * (index + 1) <= self._store.len(),
            "Index<usize> called with out-of-range row index"
        );
        let row_start = self.width * index;
        let row_end = self.width * (index + 1);
        &self._store[row_start..row_end]
    }
}

impl<T> Index<(usize, usize)> for Wec<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        let (major, minor) = index;
        assert!(
            minor <= self.width,
            "Index<(usize, usize)> called with out-of-range column index"
        );
        assert!(
            major * (self.width + 1) <= self._store.len(),
            "Index<(usize, usize)> called with out-of-range row index"
        );
        &self._store[self.width * major + minor]
    }
}

impl<T> Wec<T> {
    pub fn new(width: usize) -> Self {
        Self {
            _store: Vec::new(),
            width,
        }
    }

    pub fn rows(&self) -> usize {
        self._store.len() / self.width
    }

    pub fn height(&self) -> usize {
        self._store.len() / self.width
    }

    pub fn cols(&self) -> usize {
        self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self._store.len()
    }

    pub fn is_empty(&self) -> bool {
        self._store.is_empty()
    }

    pub fn dims(&self) -> (usize, usize) {
        (self._store.len() / self.width, self.width)
    }

    pub fn push_row(&mut self, row: &mut Vec<T>) {
        assert_eq!(row.len(), self.width);
        self._store.append(row)
    }

    pub fn extend_full_row(&mut self, row: &[T])
    where
        T: Clone,
    {
        assert_eq!(row.len(), self.width);
        self._store.extend_from_slice(row)
    }

    pub fn from_vec(store: Vec<T>, width: usize) -> Self {
        {
            let _len = store.len();
            let _rem = _len % width;
            assert_eq!(_rem, 0, "{_len}-element vector would have {_rem} extra elements during conversion to {width}-column matrix");
        }
        Wec {
            _store: store,
            width,
        }
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.rows()).map(|ix| &self[ix])
    }
}

/// Abstraction over a set of `u16`-valued elements.
#[derive(Clone)]
pub struct U16Set {
    _store: BTreeSet<u16>,
}

impl PartialEq<U16Set> for U16Set {
    fn eq(&self, other: &U16Set) -> bool {
        self._store == other._store
    }
}

impl Eq for U16Set {}

impl std::fmt::Debug for U16Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl U16Set {
    pub const fn new() -> Self {
        U16Set {
            _store: BTreeSet::new(),
        }
    }

    /// Inserts an element into the `U16Set`, returning a boolean indicating if it was a novel
    /// element and the set grew as a result.
    ///
    /// A return value of `true` means the element did not previously exist in the collection and
    /// was updated accordingly.
    ///
    /// A return value of `false` means that a prior element with the same value already existed
    /// in the collection, and the set was not updated.
    pub fn insert(&mut self, elem: u16) -> bool {
        self._store.insert(elem)
    }

    pub fn len(&self) -> usize {
        self._store.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &u16> {
        self._store.iter()
    }
}

impl FromIterator<u16> for U16Set {
    fn from_iter<I: IntoIterator<Item = u16>>(iter: I) -> Self {
        U16Set {
            _store: BTreeSet::<u16>::from_iter(iter),
        }
    }
}
