use std::{collections::BTreeSet, ops::Index};

/// Simulated two-dimensional array using a single vector allocation
///
/// All filled 'rows' must have the same width, but the final row may be under-populated (during construction at least)
///
/// (`Wec` because 'W' is visually similar to 'VV', for "V[ec]Vec")
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

    pub fn with_capacity(width: usize, capacity: usize) -> Self {
        assert_eq!(
            capacity % width,
            0,
            "capacity must be a multiple of width: {capacity} % {width} != 0"
        );
        Self {
            _store: Vec::with_capacity(capacity),
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

    /// Returns dimensions of array, in `(rows, cols)` order.
    pub fn dims(&self) -> (usize, usize) {
        (self._store.len() / self.width, self.width)
    }

    /// Pushes a single row to the end of the Wec.
    pub fn push_row(&mut self, row: &mut Vec<T>) {
        assert_eq!(row.len(), self.width);
        self._store.append(row)
    }

    pub fn extend_full_row(&mut self, row: &[T])
    where
        T: Clone,
    {
        debug_assert_eq!(
            self.width,
            row.len(),
            "mismatched column count for Wec::extend_full_row: width={} but row has length {}",
            self.width,
            row.len()
        );
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

    /// Attempts to index into a Wec using a pre-computed `i * width + j` index, as if the `Wec`
    /// were a one-dimensional `Vec` with the same elements.
    pub fn index_raw(&self, raw_index: usize) -> Option<&T> {
        self._store.get(raw_index)
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.rows()).map(|ix| &self[ix])
    }
}

impl<T> Extend<Vec<T>> for Wec<T> {
    fn extend<I: IntoIterator<Item = Vec<T>>>(&mut self, iter: I) {
        for mut row in iter {
            debug_assert_eq!(row.len(), self.width);
            self._store.append(&mut row);
        }
    }
}

impl<T> Extend<T> for Wec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut tmp = iter.into_iter().collect::<Vec<T>>();
        debug_assert_eq!(tmp.len() % self.width, 0);
        self._store.append(&mut tmp);
    }
}

/// Abstraction over a set of `u16`-valued elements.
///
/// Might be replaced with a raw bit-array of 1024 64-bit words, but not a bottleneck so even if this is an upgrade, it isn't very high-priority.
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

/// Splits a slice into a prefix, middle, and suffix.
///
/// # Safety
///
/// `slice` must have length at least `prefix_len + suffix_len`. If this is not upheld,
/// calling this function will lead to undefined behavior.
pub unsafe fn trisect_unchecked<T>(
    slice: &[T],
    prefix_len: usize,
    suffix_len: usize,
) -> (&[T], &[T], &[T]) {
    let len = slice.len();
    let (lead, suffix) = slice.split_at_unchecked(len - suffix_len);
    let (prefix, middle) = lead.split_at_unchecked(prefix_len);
    (prefix, middle, suffix)
}

/// Specialized helper Iterator object for cross-indexing an array with an iterator,
/// where we want an index for every item (as with `enumerate`) but want to know afterwards
/// whether we stopped early because the iterator ran out, or if we reached the final index
/// of the array successfully.
pub struct PerIndex<'a, T, R: Iterator<Item = usize>, I: Iterator<Item = T>,> {
    index_iter: &'a mut R,
    value_iter: &'a mut I,
}

impl<T, R, I> Iterator for PerIndex<'_, T, R, I>
where
    R: Iterator<Item = usize>,
    I: Iterator<Item = T>,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        // we take the value element first to avoid incrementing the index if no value is yielded
        let ret = self.value_iter.next()?;
        let ix = self.index_iter.next()?;
        Some((ix, ret))
    }
}

pub struct EnumLen<T, I: Iterator<Item = T>> {
    iter: I,
    range: std::ops::Range<usize>,
    len: usize,
}


#[derive(Debug)]
pub enum EnumLenError<T> {
    TooShort { len: usize, yielded: usize },
    TooLong { len: usize, next: T },
}

impl<T: std::fmt::Display> std::fmt::Display for EnumLenError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumLenError::TooShort { len, yielded } => write!(f, "value iteration stopped early (after {yielded} of {len} elements)"),
            EnumLenError::TooLong { len, next } => write!(f, "value iterator not fully-consumed (next value: {next}) after yielding all {len} elements requested"),
        }
    }
}

impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for EnumLenError<T> {}

impl<T, I: Iterator<Item = T>> EnumLen<T, I> {
    pub fn new(iter: I, len: usize) -> Self {
        EnumLen { iter, len, range: 0..len }
    }

    pub fn iter_with(&mut self) -> impl '_ + Iterator<Item = (usize, T)> {
        PerIndex {
            index_iter: &mut self.range,
            value_iter: &mut self.iter,
        }
    }

    pub fn finish(mut self, strict: bool) -> Result<(), EnumLenError<T>> {
        match self.range.next() {
            Some(ix) => Err(EnumLenError::TooShort { len: self.len, yielded: ix }),
            None => {
                if strict {
                    match self.iter.next() {
                        Some(next) => Err(EnumLenError::TooLong { len: self.len, next }),
                        None => Ok(()),
                    }
                } else {
                    Ok(())
                }
            }
        }
    }
}
