use std::{borrow::Cow, fmt::Debug, ops::Index};

use serde::Serialize;

use crate::error::EvalError;

/// The `Expr` variant that failed in a [`SeqBoundsError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeqBoundsOp {
    SeqIx,
    SubSeq,
    SubSeqInflate,
}

impl std::fmt::Display for SeqBoundsOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeqBoundsOp::SeqIx => write!(f, "SeqIx"),
            SeqBoundsOp::SubSeq => write!(f, "SubSeq"),
            SeqBoundsOp::SubSeqInflate => write!(f, "SubSeqInflate"),
        }
    }
}

/// Error produced when `SeqIx`/`SubSeq`/`SubSeqInflate` indexes or slices past the end of a
/// sequence (or `EnumFromTo` range).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeqBoundsError {
    pub op: SeqBoundsOp,
    /// The (0-based) index, or sub-range start offset, that was out of bounds.
    pub index: usize,
    /// The length of the sequence (or range) that `index` was checked against.
    pub len: usize,
}

impl std::fmt::Display for SeqBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: index {} out of bounds for sequence of length {}",
            self.op, self.index, self.len
        )
    }
}

impl std::error::Error for SeqBoundsError {}

/// Represents a sequence of values, which is either explicitly constructed
/// or yielded through an array-generator term (i.e. [`crate::Expr::Dup`]).
#[derive(Clone, PartialEq, Debug, Serialize, Hash, Eq)]
#[serde(tag = "tag", content = "data")]
// NOTE - T must be clone in order for `Dup` to be well-founded, as non-Clone values cannot be duped
pub enum SeqKind<T: Clone> {
    Strict(Vec<T>),
    Dup(usize, Box<T>),
}

/// Abstraction over Value-like constructs that can be iterated over
/// to yield members of type `V`, where `V` is either `Value` or `ParsedValue`.
///
/// Used to allow for `IntRange` to be represented without allocating a `Vec` of `Value`/`ParsedValue`.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueSeq<'a, V: Clone = super::Value> {
    ValueSeq(&'a SeqKind<V>),
    IntRange(std::ops::Range<usize>),
}

impl<'a, V: Clone> ValueSeq<'a, V> {
    pub fn len(&self) -> usize {
        match self {
            ValueSeq::ValueSeq(vs) => vs.len(),
            ValueSeq::IntRange(r) => r.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ValueSeq::ValueSeq(sk) => sk.is_empty(),
            ValueSeq::IntRange(r) => r.is_empty(),
        }
    }

    pub fn append(self, other: Self) -> SeqKind<V> {
        if matches!(self, ValueSeq::IntRange(..)) | matches!(other, ValueSeq::IntRange(..)) {
            unimplemented!("appending int ranges");
        }
        match (self, other) {
            (ValueSeq::ValueSeq(v1), ValueSeq::ValueSeq(v2)) => (*v1).clone().append((*v2).clone()),
            _ => unreachable!(),
        }
    }

    /// Checks that `index` is in-bounds for `self` (used by `SeqIx`, and by `SubSeqInflate` to
    /// check its back-reference `start` — but not its `length`, which may legitimately
    /// self-reference past `self.len()` once `start` itself is valid).
    pub(crate) fn check_index(&self, op: SeqBoundsOp, index: usize) -> Result<(), EvalError> {
        let len = self.len();
        if index < len {
            Ok(())
        } else {
            Err(SeqBoundsError { op, index, len }.into())
        }
    }
}

/// Performs a virtual 'slice' operation on a [`Range<usize>`](std::ops::Range) as if it were an array/slice holding those values,
/// mirroring the intended behavior of [`Expr::SubSeq`] when applied to a sequence-value backed by `ValueSeq::IntRange`.
///
/// Checks its own bounds and returns `Err` rather than panicking: `start + len == range.len()`
/// is the valid full-range case (e.g. `start=0, len=range.len()`), so the check is `<=`, not `<`.
pub(crate) fn sub_range(
    op: SeqBoundsOp,
    range: std::ops::Range<usize>,
    start: usize,
    len: usize,
) -> Result<std::ops::Range<usize>, SeqBoundsError> {
    let range_len = range.len();
    match start.checked_add(len) {
        Some(end) if end <= range_len => Ok(range.start + start..range.start + start + len),
        _ => Err(SeqBoundsError {
            op,
            index: start,
            len: range_len,
        }),
    }
}

pub enum ValueIter<'a, V: Clone = super::Value> {
    ValueIter(Iter<'a, V>),
    IntRange(std::ops::Range<usize>),
}

impl<'a, V> Iterator for ValueIter<'a, V>
where
    V: Clone + From<usize>,
{
    type Item = Cow<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ValueIter::ValueIter(vs) => vs.next().map(Cow::Borrowed),
            ValueIter::IntRange(r) => r.next().map(|i| Cow::Owned(V::from(i))),
        }
    }
}

impl<'a, V: Clone> IntoIterator for ValueSeq<'a, V>
where
    V: From<usize>,
{
    type Item = Cow<'a, V>;

    type IntoIter = ValueIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueSeq::ValueSeq(vs) => ValueIter::ValueIter(vs.into_iter()),
            ValueSeq::IntRange(r) => ValueIter::IntRange(r),
        }
    }
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
    pub fn into_vec(self) -> Vec<T> {
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
            SeqKind::Dup(n, v) => (ix < *n).then_some(&**v),
        }
    }

    /// Specialized method for getting a sub-sequence starting at index `start` and with length
    /// `len`, that preserves laziness. Checks its own bounds and returns `Err` rather than
    /// panicking.
    pub fn sub_seq(
        &self,
        op: SeqBoundsOp,
        start: usize,
        len: usize,
    ) -> Result<Self, SeqBoundsError> {
        let seq_len = self.len();
        if start.checked_add(len).is_none_or(|end| end > seq_len) {
            return Err(SeqBoundsError {
                op,
                index: start,
                len: seq_len,
            });
        }
        Ok(match self {
            SeqKind::Strict(vs) => SeqKind::Strict(vs[start..start + len].to_vec()),
            SeqKind::Dup(_n, v) => SeqKind::Dup(len, v.clone()),
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            SeqKind::Strict(vs) => Iter::Strict(vs.iter()),
            SeqKind::Dup(n, v) => Iter::Dup(std::iter::repeat_n(Box::as_ref(v), *n)),
        }
    }

    pub fn append(self, other: SeqKind<T>) -> SeqKind<T> {
        if self.is_empty() {
            other
        } else if other.is_empty() {
            self
        } else {
            // REVIEW - there may be minor optimizations we could leverage, but they would be very marginal yield
            let mut seq0 = self.into_vec();
            let mut seq1 = other.into_vec();
            seq0.append(&mut seq1);
            SeqKind::Strict(seq0)
        }
    }
}

impl<T: Clone> Index<usize> for SeqKind<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("out of bounds indexing {index:?} (len: {})", self.len()))
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
