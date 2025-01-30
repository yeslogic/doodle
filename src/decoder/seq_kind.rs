use std::{fmt::Debug, ops::Index};

use serde::Serialize;

#[derive(Clone, PartialEq, Debug, Serialize, Hash, Eq)]
// NOTE - T must be clone in order for `Dup` to be well-founded, as non-Clone values cannot be duped
pub enum SeqKind<T: Clone> {
    Strict(Vec<T>),
    Dup(usize, Box<T>),
}

impl<T: Clone> SeqKind<T> {
    /// Constructs an empty (strict) `SeqKind` value.
    pub const fn new() -> Self {
        SeqKind::Strict(Vec::new())
    }

    pub fn len(&self) -> usize {
        match self {
            SeqKind::Strict(vs) => vs.len(),
            SeqKind::Dup(n, _) => *n,
        }
    }

    pub const fn is_strict(&self) -> bool {
        matches!(self, SeqKind::Strict(_))
    }

    /// Returns `true` if the sequence contains no elements.
    pub fn is_empty(&self) -> bool {
        match self {
            SeqKind::Strict(vs) => vs.is_empty(),
            SeqKind::Dup(n, _) => *n == 0,
        }
    }

    /// Forcibly convert and return a strict vector, erasing any laziness that may be present.
    pub fn into_vec(self) -> Vec<T>
    {
        match self {
            SeqKind::Strict(vs) => vs,
            SeqKind::Dup(n, v) => vec![*v; n],
        }
    }

    /// Return a reference to the value at index `ix` in the sequence, if it is in-bounds, or
    /// `None` if it is out-of-bounds.
    pub fn get(&self, ix: usize) -> Option<&T> {
        match self {
            SeqKind::Strict(vs) => vs.get(ix),
            SeqKind::Dup(n, v) => {
                if ix >= *n {
                    None
                } else {
                    Some(v)
                }
            }
        }
    }

    /// Specialized method for getting a sub-sequence starting at index `start` and with length `len`,
    /// that preserves laziness.
    pub fn sub_seq(&self, start: usize, len: usize) -> Self {
        match self {
            SeqKind::Strict(vs) => {
                let tmp = &vs[start..];
                let tmp = &tmp[..len];
                SeqKind::Strict(tmp.to_vec())
            }
            SeqKind::Dup(n, v) => {
                if start + len <= *n {
                    SeqKind::Dup(len, v.clone())
                } else {
                    // REVIEW - we can either enforce `T: Debug` above, to add in the T-param, or keep it abstract
                    panic!("sub-seq out of bounds: {start}, {len} on SeqKind::Dup({n}, _)")
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            SeqKind::Strict(vs) => Iter::Strict(vs.iter()),
            SeqKind::Dup(n, v) => Iter::Dup(std::iter::repeat_n(Box::as_ref(v), *n)),
        }
    }
}

impl<T> Index<usize> for SeqKind<T>
where
    T: Clone + Debug,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .expect(format!("out of bounds indexing {index:?} on {self:?}").as_str())
    }
}

pub enum Iter<'a, T> {
    Strict(std::slice::Iter<'a, T>),
    Dup(std::iter::RepeatN<&'a T>),
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Strict(it) => it.next(),
            Iter::Dup(it) => it.next(),
        }
    }
}

/// Iterator type for [`SeqKind::T`](SeqKind)
pub enum IntoIter<T: Clone> {
    Strict(std::vec::IntoIter<T>),
    Dup(std::iter::RepeatN<T>),
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Strict(it) => it.next(),
            IntoIter::Dup(it) => it.next(),
        }
    }
}

impl<T: Clone> IntoIterator for SeqKind<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SeqKind::Strict(vs) => IntoIter::Strict(vs.into_iter()),
            SeqKind::Dup(n, v) => IntoIter::Dup(std::iter::repeat_n(*v, n)),
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a SeqKind<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SeqKind::Strict(vs) => Iter::Strict(vs.iter()),
            SeqKind::Dup(n, v) => Iter::Dup(std::iter::repeat_n(&**v, *n)),
        }
    }
}

impl<T: Clone> FromIterator<T> for SeqKind<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        SeqKind::Strict(Vec::from_iter(iter))
    }
}

impl<T: Clone> From<Vec<T>> for SeqKind<T> {
    fn from(v: Vec<T>) -> Self {
        SeqKind::Strict(v)
    }
}
