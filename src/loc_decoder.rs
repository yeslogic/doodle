use crate::byte_set::ByteSet;
use crate::error::{DecodeError, LocDecodeError};
use crate::read::ReadCtxt;
use crate::decoder::{cow_map, cow_remap, extract_pair, search::{find_index_by_key_sorted, find_index_by_key_unsorted}, seq_kind::sub_range, Compiler, Decoder, Program, ScopeEntry, SeqKind, Value, ValueSeq};
use crate::{
    Pattern,
    Arith, UnaryOp, IntRel, Label, DynFormat, Expr, Format,
};
use std::borrow::Cow;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ParseLoc {
    InBuffer {
        offset: usize,
        length: usize,
    },
    #[default]
    Synthesized,
}

impl PartialOrd for ParseLoc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ParseLoc {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                &ParseLoc::InBuffer {
                    offset: o0,
                    length: l0,
                },
                &ParseLoc::InBuffer {
                    offset: o1,
                    length: l1,
                },
            ) => match o0.cmp(&o1) {
                Ordering::Equal => l0.cmp(&l1),
                other => other,
            },
            // NOTE - this ensures that Iterator::min will naturally settle on the 'earliest' in-buffer location even if Synthesized locations are found along the way
            (&ParseLoc::InBuffer { .. }, ParseLoc::Synthesized) => Ordering::Less,
            (ParseLoc::Synthesized, ParseLoc::InBuffer { .. }) => Ordering::Greater,
            // NOTE - because synthesized locations have no logical provenance, they are technically equable even though it would be equally plausible to say they are incomparable
            (ParseLoc::Synthesized, ParseLoc::Synthesized) => Ordering::Equal,
        }
    }
}

impl ParseLoc {
    /// Returns the length, in bytes (or bits, when parsing within a [`Format::Bits`] context) of the buffer-slice that a [`Value`] was directly interpreted from.
    ///
    /// Will return `0` for the associated location of any zero-width or synthesized `Value` productions. In the latter case, Synthesized values are
    /// implicitly zero-length as they do not have a simple correspondence with any given sequence of in-buffer bytes (either being entirely constructed,
    /// or having a complex correspondence that cannot be directly represented).
    pub fn get_length(&self) -> usize {
        match self {
            ParseLoc::Synthesized => 0,
            ParseLoc::InBuffer { length, .. } => *length,
        }
    }

    /// Returns the offset from the start of either the entire buffer (by default) or the sub-buffer (for bits-mode parses)
    /// where a [`Value`]'s corresponding buffer-slice began.
    ///
    /// Will return `None` if and only if `self` happens to be `ParseLoc::Synthesized`
    pub fn get_offset(&self) -> Option<usize> {
        match self {
            ParseLoc::Synthesized => None,
            ParseLoc::InBuffer { offset, .. } => Some(*offset),
        }
    }

    /// Naive unification that joins two `ParseLoc` values into a single `ParseLoc` under the assumption that
    /// they are adjacent.
    ///
    /// As long as the set of individual `ParseLoc` values that are called on this method in a chain or iterative
    /// fold/reduce are ultimately contiguous, the order in which they are `joined` does not matter. In fact, the
    /// overall order does not matter as this operation is commutative, but will variously misrepresent any sparse
    /// collection of `ParseLoc`s, or any mostly-contiguous set with even one outlier.
    ///
    /// In particular, will preferentially use a concrete `ParseLoc::InBuffer` and shadow any `Synthesized` locations
    /// that are seen along the way.
    pub fn join(self, other: Self) -> Self {
        match other {
            ParseLoc::Synthesized => self,
            ParseLoc::InBuffer { offset, length } => match self {
                ParseLoc::Synthesized => other,
                ParseLoc::InBuffer {
                    offset: offset0,
                    length: length0,
                } => ParseLoc::InBuffer {
                    offset: Ord::min(offset0, offset),
                    length: length0 + length,
                },
            },
        }
    }

    /// Helper function that performs a [`ParseLoc::join`] operation in a manual fold over a provided iterator
    /// whose value type is `ParseLoc`.
    ///
    /// Will return [`ParseLoc::Synthesized`] when the iterator yields no [`ParseLoc::InBuffer`] values (including when it is zero-length).
    ///
    /// If the iterator in question traverses a set of contiguous slices in arbitrary order, the result will be the exact spanning location
    /// from the earliest in-buffer location and of the cumulative length of each item in the iteration.
    ///
    /// If there are non-trivial holes or outlying elements, the returned `ParseLoc` may be misleading to varying degrees.
    pub fn accum(iter: impl Iterator<Item = Self>) -> Self {
        let mut ret = ParseLoc::Synthesized;
        for item in iter {
            ret = ret.join(item);
        }
        ret
    }
}

/// Helper type for associating a [`ParseLoc`] with a value of a generic type.
#[derive(Clone, Copy, Debug)]
pub struct Parsed<T: Clone> {
    pub(crate) loc: ParseLoc,
    pub(crate) inner: T,
}

impl<T: Clone> Parsed<T> {
    /// Returns a reference to the stored value, without the accompanying [`ParseLoc`].
    pub(crate) fn get_inner(&self) -> &T {
        &self.inner
    }
}

impl<T: Clone> AsRef<T> for Parsed<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

#[derive(Clone, Debug)]
pub enum ParsedValue {
    /// Flat parses of the sub-set of `Value` variants that do not contain any embedded `Value` terms
    Flat(Parsed<Value>),
    Tuple(Parsed<Vec<ParsedValue>>),
    Record(Parsed<Vec<(Label, ParsedValue)>>),
    Variant(Label, Box<ParsedValue>),
    Seq(Parsed<SeqKind<ParsedValue>>),
    Mapped(Box<ParsedValue>, Box<ParsedValue>),
    Branch(usize, Box<ParsedValue>),
    Option(Option<Box<ParsedValue>>),
}

impl From<usize> for ParsedValue {
    fn from(v: usize) -> Self {
        ParsedValue::Flat(Parsed {
            inner: Value::Usize(v),
            loc: ParseLoc::Synthesized,
        })
    }
}

impl ParsedValue {
    /// Returns the [`ParseLoc`] directly associated with this `ParsedValue`.
    pub fn get_loc(&self) -> ParseLoc {
        match self {
            ParsedValue::Flat(Parsed { loc, .. }) => *loc,
            ParsedValue::Tuple(p_ts) => p_ts.loc,
            ParsedValue::Record(p_fs) => p_fs.loc,
            ParsedValue::Seq(p_xs) => p_xs.loc,
            ParsedValue::Variant(_lab, inner) => inner.get_loc(),
            ParsedValue::Mapped(orig, _) => orig.get_loc(),
            ParsedValue::Branch(_ix, inner) => inner.get_loc(),
            ParsedValue::Option(Some(inner)) => inner.get_loc(),
            ParsedValue::Option(None) => ParseLoc::Synthesized,
        }
    }

    /// Helper function to construct a `Value::UNIT` with a zero-length spanning slice at a given starting offset.
    pub(crate) const fn unit_at(offset: usize) -> ParsedValue {
        Self::unit_spanning(offset, 0)
    }

    /// Helper function to construct a `Value::UNIT` of specific length starting at a given offset.
    const fn unit_spanning(offset: usize, length: usize) -> ParsedValue {
        ParsedValue::Tuple(Parsed {
            loc: ParseLoc::InBuffer { offset, length },
            inner: Vec::new(),
        })
    }

    /// Constructs a new `ParsedValue` from a `Value` with a provided offset and length.
    ///
    /// While this method will never fail and does not enforce any invariants,
    /// there is an implicit expectation that the `Value` being passed in is 'flat', i.e. contains no embedded `Value`s.
    /// If this expectation is violated, there may be complications down the line.
    pub const fn new_flat(inner: Value, offset: usize, length: usize) -> ParsedValue {
        ParsedValue::Flat(Parsed {
            loc: ParseLoc::InBuffer { offset, length },
            inner,
        })
    }

    pub fn wrap_variant(lab: Label, v: ParsedValue) -> ParsedValue {
        ParsedValue::Variant(lab, Box::new(v))
    }

    fn new_tuple(v: Vec<ParsedValue>, offset: usize, length: usize) -> ParsedValue {
        ParsedValue::Tuple(Parsed {
            loc: ParseLoc::InBuffer { offset, length },
            inner: v,
        })
    }

    fn new_seq(v: Vec<ParsedValue>, offset: usize, length: usize) -> ParsedValue {
        ParsedValue::Seq(Parsed {
            loc: ParseLoc::InBuffer { offset, length },
            inner: SeqKind::Strict(v),
        })
    }

    /// Helper function that constructs a Synthesized `ParsedValue` as appropriate and immediately
    /// ascribes it the same location as an original `ParsedValue`.
    ///
    /// Mostly useful for handling `Format::Map`.
    fn inherit(orig: &ParsedValue, v: Value) -> ParsedValue {
        let mut tmp = Self::from_evaluated(v);
        tmp.transpose(orig.get_loc());
        tmp
    }

    /// Overwrites a `ParsedValue`'s associated `ParseLoc` using the provided argument, discarding its previous value.
    pub fn transpose(&mut self, new_loc: ParseLoc) {
        match self {
            ParsedValue::Flat(p) => p.loc = new_loc,
            ParsedValue::Tuple(p) => p.loc = new_loc,
            ParsedValue::Record(p) => p.loc = new_loc,
            ParsedValue::Seq(p) => p.loc = new_loc,
            ParsedValue::Variant(_, inner) => inner.transpose(new_loc),
            ParsedValue::Branch(_, inner) => inner.transpose(new_loc),
            ParsedValue::Mapped(_, image) => image.transpose(new_loc),
            ParsedValue::Option(Some(inner)) => inner.transpose(new_loc),
            ParsedValue::Option(None) => {}
        }
    }

    pub(crate) fn is_boolean(&self) -> bool {
        match self.coerce_mapped_value() {
            ParsedValue::Flat(v) => v.inner.is_boolean(),
            _ => false,
        }
    }
}

impl From<ParsedValue> for Value {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::Flat(v) => v.inner,
            ParsedValue::Tuple(ts) => {
                Value::Tuple(Vec::from_iter(ts.inner.into_iter().map(Value::from)))
            }
            ParsedValue::Seq(elts) => {
                Value::Seq(SeqKind::from_iter(elts.inner.into_iter().map(Value::from)))
            }
            ParsedValue::Record(fs) => Value::Record(Vec::from_iter(
                fs.inner.into_iter().map(|(lab, f)| (lab, f.into())),
            )),
            ParsedValue::Variant(lab, inner) => Value::Variant(lab, Box::new((*inner).into())),
            ParsedValue::Mapped(orig, image) => {
                Value::Mapped(Box::new((*orig).into()), Box::new((*image).into()))
            }
            ParsedValue::Branch(ix, inner) => Value::Branch(ix, Box::new((*inner).into())),
            ParsedValue::Option(opt) => Value::Option(opt.map(|v| Box::new((*v).into()))),
        }
    }
}

impl ParsedValue {
    pub fn into_cow_value(&self) -> Cow<'_, Value> {
        match self {
            ParsedValue::Flat(Parsed { inner, .. }) => Cow::Borrowed(inner),
            _ => Cow::Owned(self.clone().into()),
        }
    }

    pub fn record_proj(&self, label: &str) -> &Self {
        match self {
            ParsedValue::Record(Parsed { inner: fields, .. }) => {
                match fields.iter().find(|(l, _)| label == l) {
                    Some((_, v)) => v,
                    None => panic!("{label} not found in record"),
                }
            }
            _ => panic!("expected record, found {self:?}"),
        }
    }

    pub fn clone_into_value(&self) -> Value {
        match self {
            ParsedValue::Flat(Parsed { inner, .. }) => inner.clone(),
            ParsedValue::Tuple(ts) => {
                Value::Tuple(Vec::from_iter(ts.inner.iter().cloned().map(Value::from)))
            }
            ParsedValue::Seq(seq) => {
                match &seq.inner {
                    SeqKind::Strict(elts) => Value::Seq(SeqKind::Strict(Vec::from_iter(
                        elts.iter().cloned().map(Value::from),
                    ))),
                    SeqKind::Dup(n, v) => Value::Seq(SeqKind::Dup(*n, Box::new(v.clone_into_value()))),
                }
            },
            ParsedValue::Record(fs) => Value::Record(Vec::from_iter(
                fs.inner.iter().cloned().map(|(lab, f)| (lab, f.into())),
            )),
            ParsedValue::Variant(lab, inner) => {
                Value::Variant(lab.clone(), Box::new((**inner).clone().into()))
            }
            ParsedValue::Mapped(orig, image) => Value::Mapped(
                Box::new((**orig).clone().into()),
                Box::new((**image).clone().into()),
            ),
            ParsedValue::Branch(ix, inner) => {
                Value::Branch(*ix, Box::new((**inner).clone().into()))
            }
            ParsedValue::Option(opt) => {
                Value::Option(opt.as_ref().map(|v| Box::new((**v).clone().into())))
            }
        }
    }

    pub fn matches_inner(&self, scope: &mut LocMultiScope<'_>, pattern: &Pattern) -> bool {
        match (pattern, self) {
            (Pattern::Binding(name), head) => {
                scope.push(name.clone(), head.clone());
                true
            }
            (Pattern::Wildcard, _) => true,
            (
                Pattern::Bool(b0),
                ParsedValue::Flat(Parsed {
                    inner: Value::Bool(b1),
                    ..
                }),
            ) => b0 == b1,
            (
                Pattern::U8(i0),
                ParsedValue::Flat(Parsed {
                    inner: Value::U8(i1),
                    ..
                }),
            ) => i0 == i1,
            (
                Pattern::U16(i0),
                ParsedValue::Flat(Parsed {
                    inner: Value::U16(i1),
                    ..
                }),
            ) => i0 == i1,
            (
                Pattern::U32(i0),
                ParsedValue::Flat(Parsed {
                    inner: Value::U32(i1),
                    ..
                }),
            ) => i0 == i1,
            (
                Pattern::U64(i0),
                ParsedValue::Flat(Parsed {
                    inner: Value::U64(i1),
                    ..
                }),
            ) => i0 == i1,
            (
                Pattern::Char(c0),
                ParsedValue::Flat(Parsed {
                    inner: Value::Char(c1),
                    ..
                }),
            ) => c0 == c1,
            (Pattern::Tuple(ps), ParsedValue::Tuple(vs)) if ps.len() == vs.inner.len() => {
                for (p, v) in Iterator::zip(ps.iter(), vs.inner.iter()) {
                    if !v.matches_inner(scope, p) {
                        return false;
                    }
                }
                true
            }
            (Pattern::Seq(ps), ParsedValue::Seq(vs)) if ps.len() == vs.inner.len() => {
                for (p, v) in Iterator::zip(ps.iter(), vs.inner.iter()) {
                    if !v.matches_inner(scope, p) {
                        return false;
                    }
                }
                true
            }
            (Pattern::Variant(label0, p), ParsedValue::Variant(label1, v)) if label0 == label1 => {
                v.matches_inner(scope, p)
            }
            (Pattern::Option(None), ParsedValue::Option(None)) => true,
            (Pattern::Option(Some(p)), ParsedValue::Option(Some(v))) => v.matches_inner(scope, p),
            _ => false,
        }
    }

    fn tuple_proj(&self, index: usize) -> &Self {
        match self.coerce_mapped_value() {
            ParsedValue::Tuple(Parsed { inner, .. }) => &inner[index],
            _ => panic!("expected tuple"),
        }
    }

    fn collect_fields(fields: Vec<(Label, Self)>) -> Self {
        assert!(
            !fields.is_empty(),
            "ParsedValue::record_from_entries found empty vector"
        );
        let loc = ParseLoc::accum(fields.iter().map(|(_lab, fld)| fld.get_loc()));
        ParsedValue::Record(Parsed { loc, inner: fields })
    }

    pub(crate) fn coerce_mapped_value(&self) -> &Self {
        match self {
            ParsedValue::Mapped(_orig, v) => v.coerce_mapped_value(),
            ParsedValue::Branch(_n, v) => v.coerce_mapped_value(),
            v => v,
        }
    }

    pub(crate) fn from_evaluated(expr_value: Value) -> Self {
        match expr_value {
            Value::Bool(_)
            | Value::U8(_)
            | Value::U16(_)
            | Value::U32(_)
            | Value::U64(_)
            | Value::Usize(_)
            | Value::EnumFromTo(_)
            | Value::Char(_) => ParsedValue::Flat(Parsed {
                loc: ParseLoc::Synthesized,
                inner: expr_value,
            }),
            Value::Tuple(vs) => {
                let mut p_vs = Vec::with_capacity(vs.len());
                for v in vs.into_iter() {
                    p_vs.push(ParsedValue::from_evaluated(v));
                }
                ParsedValue::Tuple(Parsed {
                    loc: ParseLoc::Synthesized,
                    inner: p_vs,
                })
            }
            Value::Record(fs) => {
                let mut p_fs = Vec::with_capacity(fs.len());
                for (lab, v) in fs.into_iter() {
                    p_fs.push((lab, ParsedValue::from_evaluated(v)));
                }
                ParsedValue::Record(Parsed {
                    loc: ParseLoc::Synthesized,
                    inner: p_fs,
                })
            }
            Value::Seq(elts) => {
                let mut p_elts = Vec::with_capacity(elts.len());
                for elt in elts.into_iter() {
                    p_elts.push(ParsedValue::from_evaluated(elt));
                }
                ParsedValue::Seq(Parsed {
                    loc: ParseLoc::Synthesized,
                    inner: p_elts.into(),
                })
            }
            Value::Variant(lab, inner) => {
                ParsedValue::Variant(lab, Box::new(ParsedValue::from_evaluated(*inner)))
            }
            Value::Mapped(orig, image) => {
                let orig = Box::new(ParsedValue::from_evaluated(*orig));
                let image = Box::new(ParsedValue::from_evaluated(*image));
                ParsedValue::Mapped(orig, image)
            }
            Value::Branch(ix, inner) => {
                let inner = Box::new(ParsedValue::from_evaluated(*inner));
                ParsedValue::Branch(ix, inner)
            }
            Value::Option(opt) => {
                ParsedValue::Option(opt.map(|inner| Box::new(ParsedValue::from_evaluated(*inner))))
            }
        }
    }

    fn from_evaluated_seq<S: Into<SeqKind<Self>>>(elts: S) -> Self {
        ParsedValue::Seq(Parsed {
            loc: ParseLoc::Synthesized,
            inner: elts.into(),
        })
    }

    fn get_sequence(&self) -> Option<ValueSeq<'_, Self>> {
        match self {
            ParsedValue::Seq(parsed) => Some(ValueSeq::ValueSeq(&parsed.inner)),
            ParsedValue::Flat(Parsed {
                inner: Value::EnumFromTo(range),
                ..
            }) => Some(ValueSeq::IntRange(range.clone())),
            _ => None,
        }
    }

    fn matches<'a>(&self, scope: &'a LocScope<'a>, pattern: &Pattern) -> Option<LocMultiScope<'a>> {
        let mut pattern_scope = LocMultiScope::new(scope);
        self.coerce_mapped_value()
            .matches_inner(&mut pattern_scope, pattern)
            .then_some(pattern_scope)
    }
}

impl Expr {
    pub fn eval_with_loc<'a>(&'a self, scope: &'a LocScope<'a>) -> Cow<'a, ParsedValue> {
        match self {
            Expr::Var(name) => Cow::Borrowed(scope.get_value_by_name(name)),
            Expr::Bool(b) => Cow::Owned(ParsedValue::from_evaluated(Value::Bool(*b))),
            Expr::U8(i) => Cow::Owned(ParsedValue::from_evaluated(Value::U8(*i))),
            Expr::U16(i) => Cow::Owned(ParsedValue::from_evaluated(Value::U16(*i))),
            Expr::U32(i) => Cow::Owned(ParsedValue::from_evaluated(Value::U32(*i))),
            Expr::U64(i) => Cow::Owned(ParsedValue::from_evaluated(Value::U64(*i))),
            Expr::Tuple(exprs) => Cow::Owned(ParsedValue::from_evaluated(Value::Tuple(
                exprs
                    .iter()
                    .map(|expr| expr.eval_value_with_loc(scope))
                    .collect(),
            ))),
            Expr::TupleProj(head, index) => cow_map(head.eval_with_loc(scope), |v| {
                v.coerce_mapped_value().tuple_proj(*index)
            }),
            Expr::Record(fields) => Cow::Owned(ParsedValue::from_evaluated(Value::record(
                fields
                    .iter()
                    .map(|(label, expr)| (label.clone(), expr.eval_value_with_loc(scope))),
            ))),
            Expr::RecordProj(head, label) => cow_map(head.eval_with_loc(scope), |v| {
                v.coerce_mapped_value().record_proj(label.as_ref())
            }),
            Expr::Variant(label, expr) => Cow::Owned(ParsedValue::from_evaluated(Value::variant(
                label.clone(),
                expr.eval_value_with_loc(scope),
            ))),
            Expr::Seq(exprs) => Cow::Owned(ParsedValue::from_evaluated(Value::Seq(
                exprs
                    .iter()
                    .map(|expr| expr.eval_value_with_loc(scope))
                    .collect(),
            ))),
            Expr::Match(head, branches) => {
                let head = head.eval_with_loc(scope);
                for (pattern, expr) in branches {
                    if let Some(pattern_scope) = head.matches(scope, pattern) {
                        let value = expr.eval_value_with_loc(&LocScope::Multi(&pattern_scope));
                        return Cow::Owned(ParsedValue::from_evaluated(value));
                    }
                }
                panic!("non-exhaustive patterns");
            }
            Expr::Lambda(_, _) => panic!("cannot eval lambda"),

            Expr::IntRel(IntRel::Eq, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x == y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x == y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x == y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x == y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::IntRel(IntRel::Ne, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x != y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x != y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x != y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x != y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::IntRel(IntRel::Lt, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x < y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x < y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x < y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x < y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::IntRel(IntRel::Gt, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x > y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x > y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x > y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x > y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::IntRel(IntRel::Lte, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x <= y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x <= y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x <= y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x <= y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::IntRel(IntRel::Gte, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x >= y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x >= y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x >= y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x >= y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::Add, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_add(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_add(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_add(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_add(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::Sub, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_sub(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_sub(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_sub(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_sub(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::Mul, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_mul(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_mul(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_mul(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_mul(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::Div, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_div(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_div(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_div(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_div(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::Rem, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_rem(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_rem(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_rem(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_rem(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::BitAnd, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(x & y),
                    (Value::U16(x), Value::U16(y)) => Value::U16(x & y),
                    (Value::U32(x), Value::U32(y)) => Value::U32(x & y),
                    (Value::U64(x), Value::U64(y)) => Value::U64(x & y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::BitOr, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(x | y),
                    (Value::U16(x), Value::U16(y)) => Value::U16(x | y),
                    (Value::U32(x), Value::U32(y)) => Value::U32(x | y),
                    (Value::U64(x), Value::U64(y)) => Value::U64(x | y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::BoolAnd, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::Bool(b0), Value::Bool(b1)) => Value::Bool(b0 && b1),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::BoolOr, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::Bool(b0), Value::Bool(b1)) => Value::Bool(b0 || b1),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::Shl, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => {
                        Value::U8(u8::checked_shl(x, u32::from(y)).unwrap())
                    }
                    (Value::U16(x), Value::U16(y)) => {
                        Value::U16(u16::checked_shl(x, u32::from(y)).unwrap())
                    }
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shl(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => {
                        Value::U64(u64::checked_shl(x, u32::try_from(y).unwrap()).unwrap())
                    }
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Arith(Arith::Shr, x, y) => Cow::Owned(ParsedValue::from_evaluated(
                match (x.eval_value_with_loc(scope), y.eval_value_with_loc(scope)) {
                    (Value::U8(x), Value::U8(y)) => {
                        Value::U8(u8::checked_shr(x, u32::from(y)).unwrap())
                    }
                    (Value::U16(x), Value::U16(y)) => {
                        Value::U16(u16::checked_shr(x, u32::from(y)).unwrap())
                    }
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shr(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => {
                        Value::U64(u64::checked_shr(x, u32::try_from(y).unwrap()).unwrap())
                    }
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                },
            )),
            Expr::Unary(UnaryOp::BoolNot, x) => Cow::Owned(ParsedValue::from_evaluated(
                match x.eval_value_with_loc(scope) {
                    Value::Bool(x) => Value::Bool(!x),
                    x => panic!("unexpected operand: expecting boolean, found `{x:?}`"),
                },
            )),
            Expr::Unary(UnaryOp::IntSucc, x) => Cow::Owned(ParsedValue::from_evaluated(
                match x.eval_value_with_loc(scope) {
                    Value::U8(x) => Value::U8(
                        x.checked_add(1)
                            .unwrap_or_else(|| panic!("IntSucc(u8::MAX) overflow")),
                    ),
                    Value::U16(x) => Value::U16(
                        x.checked_add(1)
                            .unwrap_or_else(|| panic!("IntSucc(u16::MAX) overflow")),
                    ),
                    Value::U32(x) => Value::U32(
                        x.checked_add(1)
                            .unwrap_or_else(|| panic!("IntSucc(u32::MAX) overflow")),
                    ),
                    Value::U64(x) => Value::U64(
                        x.checked_add(1)
                            .unwrap_or_else(|| panic!("IntSucc(u64::MAX) overflow")),
                    ),
                    x => panic!("unexpected operand: expected integral value, found `{x:?}`"),
                },
            )),
            Expr::Unary(UnaryOp::IntPred, x) => Cow::Owned(ParsedValue::from_evaluated(
                match x.eval_value_with_loc(scope) {
                    Value::U8(x) => Value::U8(
                        x.checked_sub(1)
                            .unwrap_or_else(|| panic!("IntPred(0u8) underflow")),
                    ),
                    Value::U16(x) => Value::U16(
                        x.checked_sub(1)
                            .unwrap_or_else(|| panic!("IntPred(0u16) underflow")),
                    ),
                    Value::U32(x) => Value::U32(
                        x.checked_sub(1)
                            .unwrap_or_else(|| panic!("IntPred(0u32) underflow")),
                    ),
                    Value::U64(x) => Value::U64(
                        x.checked_sub(1)
                            .unwrap_or_else(|| panic!("IntPred(0u64) underflow")),
                    ),
                    x => panic!("unexpected operand: expected integral value, found `{x:?}`"),
                },
            )),
            Expr::AsU8(x) => Cow::Owned(ParsedValue::from_evaluated(
                match x.eval_value_with_loc(scope) {
                    Value::U8(x) => Value::U8(x),
                    Value::U16(x) => Value::U8(u8::try_from(x).unwrap()),
                    Value::U32(x) => Value::U8(u8::try_from(x).unwrap()),
                    Value::U64(x) => Value::U8(u8::try_from(x).unwrap()),
                    x => panic!("cannot convert {x:?} to U8"),
                },
            )),
            Expr::AsU16(x) => Cow::Owned(ParsedValue::from_evaluated(
                match x.eval_value_with_loc(scope) {
                    Value::U8(x) => Value::U16(u16::from(x)),
                    Value::U16(x) => Value::U16(x),
                    Value::U32(x) => Value::U16(u16::try_from(x).unwrap()),
                    Value::U64(x) => Value::U16(u16::try_from(x).unwrap()),
                    x => panic!("cannot convert {x:?} to U16"),
                },
            )),
            Expr::AsU32(x) => Cow::Owned(ParsedValue::from_evaluated(
                match x.eval_value_with_loc(scope) {
                    Value::U8(x) => Value::U32(u32::from(x)),
                    Value::U16(x) => Value::U32(u32::from(x)),
                    Value::U32(x) => Value::U32(x),
                    Value::U64(x) => Value::U32(u32::try_from(x).unwrap()),
                    x => panic!("cannot convert {x:?} to U32"),
                },
            )),
            Expr::AsU64(x) => Cow::Owned(ParsedValue::from_evaluated(
                match x.eval_value_with_loc(scope) {
                    Value::U8(x) => Value::U64(u64::from(x)),
                    Value::U16(x) => Value::U64(u64::from(x)),
                    Value::U32(x) => Value::U64(u64::from(x)),
                    Value::U64(x) => Value::U64(x),
                    x => panic!("cannot convert {x:?} to U64"),
                },
            )),

            Expr::U16Be(bytes) => {
                match bytes.eval_value_with_loc(scope).unwrap_tuple().as_slice() {
                    [Value::U8(hi), Value::U8(lo)] => Cow::Owned(ParsedValue::from_evaluated(
                        Value::U16(u16::from_be_bytes([*hi, *lo])),
                    )),
                    _ => panic!("U16Be: expected (U8, U8)"),
                }
            }
            Expr::U16Le(bytes) => {
                match bytes.eval_value_with_loc(scope).unwrap_tuple().as_slice() {
                    [Value::U8(lo), Value::U8(hi)] => Cow::Owned(ParsedValue::from_evaluated(
                        Value::U16(u16::from_le_bytes([*lo, *hi])),
                    )),
                    _ => panic!("U16Le: expected (U8, U8)"),
                }
            }
            Expr::U32Be(bytes) => {
                match bytes.eval_value_with_loc(scope).unwrap_tuple().as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                        Cow::Owned(ParsedValue::from_evaluated(Value::U32(u32::from_be_bytes(
                            [*a, *b, *c, *d],
                        ))))
                    }
                    _ => panic!("U32Be: expected (U8, U8, U8, U8)"),
                }
            }
            Expr::U32Le(bytes) => {
                match bytes.eval_value_with_loc(scope).unwrap_tuple().as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                        Cow::Owned(ParsedValue::from_evaluated(Value::U32(u32::from_le_bytes(
                            [*a, *b, *c, *d],
                        ))))
                    }
                    _ => panic!("U32Le: expected (U8, U8, U8, U8)"),
                }
            }
            Expr::U64Be(bytes) => {
                match bytes.eval_value_with_loc(scope).unwrap_tuple().as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d), Value::U8(e), Value::U8(f), Value::U8(g), Value::U8(h)] => {
                        Cow::Owned(ParsedValue::from_evaluated(Value::U64(u64::from_be_bytes(
                            [*a, *b, *c, *d, *e, *f, *g, *h],
                        ))))
                    }
                    _ => panic!("U32Be: expected (U8, U8, U8, U8, U8, U8, U8, U8)"),
                }
            }
            Expr::U64Le(bytes) => {
                match bytes.eval_value_with_loc(scope).unwrap_tuple().as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d), Value::U8(e), Value::U8(f), Value::U8(g), Value::U8(h)] => {
                        Cow::Owned(ParsedValue::from_evaluated(Value::U64(u64::from_le_bytes(
                            [*a, *b, *c, *d, *e, *f, *g, *h],
                        ))))
                    }
                    _ => panic!("U32Le: expected (U8, U8, U8, U8, U8, U8, U8, U8)"),
                }
            }
            Expr::AsChar(bytes) => Cow::Owned(ParsedValue::from_evaluated(
                match bytes.eval_value_with_loc(scope) {
                    Value::U8(x) => Value::Char(char::from(x)),
                    Value::U16(x) => {
                        Value::Char(char::from_u32(x as u32).unwrap_or(char::REPLACEMENT_CHARACTER))
                    }
                    Value::U32(x) => {
                        Value::Char(char::from_u32(x).unwrap_or(char::REPLACEMENT_CHARACTER))
                    }
                    Value::U64(x) => Value::Char(
                        char::from_u32(u32::try_from(x).unwrap())
                            .unwrap_or(char::REPLACEMENT_CHARACTER),
                    ),
                    _ => panic!("AsChar: expected U8, U16, U32, or U64"),
                },
            )),
            Expr::SeqLength(seq) => match seq
                .eval_with_loc(scope)
                .coerce_mapped_value()
                .get_sequence()
            {
                Some(values) => {
                    let len = values.len();
                    Cow::Owned(ParsedValue::from_evaluated(Value::U32(len as u32)))
                }
                _ => panic!("SeqLength: expected Seq (or EnumFromTo)"),
            },
            Expr::SeqIx(seq, index) => cow_remap(seq.eval_with_loc(scope), |v| {
                match v.coerce_mapped_value().get_sequence() {
                    Some(values) => {
                        let index = index.eval_value_with_loc(scope).unwrap_usize();
                        match values {
                            ValueSeq::ValueSeq(values) => Cow::Borrowed(&values[index]),
                            ValueSeq::IntRange(mut range) => {
                                Cow::Owned(ParsedValue::from_evaluated(Value::Usize(
                                    range.nth(index).unwrap(),
                                )))
                            }
                        }
                    }
                    _ => panic!("SeqIx: expected Seq (or EnumFromTo)"),
                }
            }),
            Expr::SubSeq(seq, start, length) => {
                match seq
                    .eval_with_loc(scope)
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let start = start.eval_value_with_loc(scope).unwrap_usize();
                        let length = length.eval_value_with_loc(scope).unwrap_usize();
                        match values {
                            ValueSeq::ValueSeq(values) => Cow::Owned(
                                ParsedValue::from_evaluated_seq(values.sub_seq(start, length)),
                            ),
                            ValueSeq::IntRange(range) => Cow::Owned(ParsedValue::from_evaluated(
                                Value::EnumFromTo(sub_range(range, start, length)),
                            )),
                        }
                    }
                    _ => panic!("SubSeq: expected Seq"),
                }
            }
            Expr::SubSeqInflate(seq, start, length) => {
                match seq
                    .eval_with_loc(scope)
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let start = start.eval_value_with_loc(scope).unwrap_usize();
                        let length = length.eval_value_with_loc(scope).unwrap_usize();
                        let mut vs = Vec::new();
                        match values {
                            ValueSeq::ValueSeq(vs0) => {
                                for i in 0..length {
                                    if i + start < vs0.len() {
                                        vs.push(vs0[i + start].clone());
                                    } else {
                                        vs.push(vs[i + start - vs0.len()].clone());
                                    }
                                }
                            }
                            ValueSeq::IntRange(range) => {
                                // REVIEW - double-check this logic
                                let len = range.len();
                                let mut iter = range.skip(start);
                                for i in 0..length {
                                    if let Some(val) = iter.next() {
                                        vs.push(val.into());
                                    } else {
                                        vs.push(vs[i + start - len].clone());
                                    }
                                }
                            }
                        }
                        Cow::Owned(ParsedValue::from_evaluated_seq(vs))
                    }
                    _ => panic!("SubSeqInflate: expected Seq"),
                }
            }
            Expr::FlatMap(expr, seq) => {
                match seq
                    .eval_with_loc(scope)
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let mut vs: Vec<Value> = Vec::new();
                        for v in values {
                            if let Value::Seq(vn) = expr.eval_lambda_with_loc(scope, &v) {
                                vs.extend(vn);
                            } else {
                                panic!("FlatMap: expected Seq");
                            }
                        }
                        Cow::Owned(ParsedValue::from_evaluated(Value::Seq(vs.into())))
                    }
                    _ => panic!("FlatMap: expected Seq"),
                }
            }
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => match seq.eval_with_loc(scope).coerce_mapped_value().get_sequence() {
                Some(values) => {
                    let mut accum = accum.eval_value_with_loc(scope);
                    let mut vs = Vec::new();
                    for v in values {
                        let ret = expr.eval_lambda_with_loc(
                            scope,
                            &ParsedValue::from_evaluated(Value::Tuple(vec![accum, v.clone_into_value()])),
                        );
                        accum = match extract_pair(ret.unwrap_tuple()) {
                            (accum, Value::Seq(vn)) => {
                                vs.extend(vn);
                                accum
                            }
                            other => panic!("FlatMapAccum: bad lambda output type {other:?}"),
                        };
                    }
                    Cow::Owned(ParsedValue::from_evaluated(Value::Seq(vs.into())))
                }
                None => panic!("FlatMapAccum: expected Seq"),
            }
            Expr::LeftFold(expr, accum, _accum_type, seq) => match seq.eval_with_loc(scope).coerce_mapped_value().get_sequence() {
                Some(values) => {
                    let mut accum = accum.eval_value_with_loc(scope);
                    for v in values {
                        let new_accum = expr.eval_lambda_with_loc(
                            scope,
                            &ParsedValue::from_evaluated(Value::Tuple(vec![accum, v.clone_into_value()])),
                        );
                        accum = new_accum;
                    }
                    Cow::Owned(ParsedValue::from_evaluated(accum))
                }
                None => panic!("LeftFold: expected Seq"),
            }
            Expr::FindByKey(is_sorted, f_get_key, query_key, seq) => {
                match seq.eval_with_loc(scope).coerce_mapped_value().get_sequence() {
                    Some(ValueSeq::ValueSeq(values)) => {
                        let eval = |lambda: &Expr, v: &ParsedValue| {
                            lambda.eval_lambda_with_loc(scope, v)
                        };
                        let query = query_key.eval_value_with_loc(scope);
                        if *is_sorted {
                            match find_index_by_key_sorted(f_get_key, &query, &values, eval) {
                                Some(ix) => Cow::Owned(ParsedValue::Option(Some(Box::new(values[ix].clone())))),
                                None => Cow::Owned(ParsedValue::from_evaluated(Value::Option(None))),
                            }
                        } else {
                            match find_index_by_key_unsorted(f_get_key, &query, &values, eval) {
                                Some(ix) => Cow::Owned(ParsedValue::Option(Some(Box::new(values[ix].clone())))),
                                None => Cow::Owned(ParsedValue::from_evaluated(Value::Option(None))),
                            }
                        }
                    }
                    Some(ValueSeq::IntRange(_)) => unimplemented!("FindByKey: unimplemented on IntRange"),
                    None => panic!("FindByKey: expected Seq"),
                }
            }
            Expr::FlatMapList(expr, _ret_type, seq) => match seq.eval_value_with_loc(scope) {
                Value::Seq(values) => {
                    let mut vs = Vec::new();
                    for v in values {
                        let arg = Value::Tuple(vec![Value::Seq(vs.into()), v]);
                        // TODO can we avoid cloning arg here?
                        if let Value::Seq(vn) = expr
                            .eval_lambda_with_loc(scope, &ParsedValue::from_evaluated(arg.clone()))
                        {
                            vs = match arg {
                                Value::Tuple(mut args) => match args.remove(0) {
                                    Value::Seq(vs) => vs.into_vec(),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            vs.extend(vn);
                        } else {
                            panic!("FlatMapList: expected Seq");
                        }
                    }
                    Cow::Owned(ParsedValue::from_evaluated(Value::Seq(vs.into())))
                }
                _ => panic!("FlatMapList: expected Seq"),
            },
            Expr::Dup(count, expr) => {
                let count = count.eval_value_with_loc(scope).unwrap_usize();
                let v = expr.eval_value_with_loc(scope);
                Cow::Owned(ParsedValue::from_evaluated(Value::Seq(SeqKind::Dup(
                    count,
                    Box::new(v),
                ))))
            }
            Expr::EnumFromTo(start, stop) => {
                let start = start.eval_value_with_loc(scope).unwrap_usize();
                let stop = stop.eval_value_with_loc(scope).unwrap_usize();
                Cow::Owned(ParsedValue::from_evaluated(Value::EnumFromTo(start..stop)))
            }
            Expr::LiftOption(opt) => Cow::Owned(ParsedValue::from_evaluated(Value::Option(
                opt.as_ref()
                    .map(|expr| Box::new(expr.eval_value_with_loc(scope))),
            ))),
        }
    }

    pub fn eval_value_with_loc<'a>(&self, scope: &'a LocScope<'a>) -> Value {
        self.eval_with_loc(scope)
            .coerce_mapped_value()
            .clone_into_value()
    }

    fn eval_lambda_with_loc<'a>(&self, scope: &'a LocScope<'a>, arg: &ParsedValue) -> Value {
        match self {
            Expr::Lambda(name, expr) => {
                let child_scope = LocSingleScope::new(scope, name, arg);
                expr.eval_value_with_loc(&LocScope::Single(child_scope))
            }
            _ => panic!("expected Lambda"),
        }
    }
}

impl Program {
    pub fn run_with_loc<'input>(
        &self,
        input: ReadCtxt<'input>,
    ) -> LocDecodeError<(ParsedValue, ReadCtxt<'input>)> {
        self.decoders[0]
            .0
            .parse_with_loc(self, &LocScope::Empty, input)
    }
}

pub type LocScopeEntry = ScopeEntry<ParsedValue>;

pub enum LocScope<'a> {
    Empty,
    Multi(&'a LocMultiScope<'a>),
    Single(LocSingleScope<'a>),
    Decoder(LocDecoderScope<'a>),
}

pub struct LocMultiScope<'a> {
    parent: &'a LocScope<'a>,
    entries: Vec<(Label, ParsedValue)>,
}

pub struct LocSingleScope<'a> {
    parent: &'a LocScope<'a>,
    name: &'a str,
    value: &'a ParsedValue,
}

pub struct LocDecoderScope<'a> {
    parent: &'a LocScope<'a>,
    name: &'a str,
    decoder: Decoder,
}

impl<'a> LocScope<'a> {
    fn get_value_by_name(&self, name: &str) -> &ParsedValue {
        match self {
            LocScope::Empty => panic!("value not found: {name}"),
            LocScope::Multi(multi) => multi.get_value_by_name(name),
            LocScope::Single(single) => single.get_value_by_name(name),
            LocScope::Decoder(decoder) => decoder.parent.get_value_by_name(name),
        }
    }

    fn get_decoder_by_name(&self, name: &str) -> &Decoder {
        match self {
            LocScope::Empty => panic!("decoder not found: {name}"),
            LocScope::Multi(multi) => multi.parent.get_decoder_by_name(name),
            LocScope::Single(single) => single.parent.get_decoder_by_name(name),
            LocScope::Decoder(decoder) => decoder.get_decoder_by_name(name),
        }
    }

    pub fn get_bindings(&self, bindings: &mut Vec<(Label, LocScopeEntry)>) {
        match self {
            LocScope::Empty => {}
            LocScope::Multi(multi) => multi.get_bindings(bindings),
            LocScope::Single(single) => single.get_bindings(bindings),
            LocScope::Decoder(decoder) => decoder.get_bindings(bindings),
        }
    }
}

impl<'a> LocMultiScope<'a> {
    fn new(parent: &'a LocScope<'a>) -> LocMultiScope<'a> {
        let entries = Vec::new();
        LocMultiScope { parent, entries }
    }

    pub fn with_capacity(parent: &'a LocScope<'a>, capacity: usize) -> LocMultiScope<'a> {
        let entries = Vec::with_capacity(capacity);
        LocMultiScope { parent, entries }
    }

    pub fn push(&mut self, name: impl Into<Label>, v: ParsedValue) {
        self.entries.push((name.into(), v));
    }

    fn get_value_by_name(&self, name: &str) -> &ParsedValue {
        for (n, v) in self.entries.iter().rev() {
            if n == name {
                return v;
            }
        }
        self.parent.get_value_by_name(name)
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, LocScopeEntry)>) {
        for (name, value) in self.entries.iter().rev() {
            bindings.push((name.clone(), LocScopeEntry::Value(value.clone())));
        }
        self.parent.get_bindings(bindings);
    }

    fn into_record(self) -> ParsedValue {
        ParsedValue::collect_fields(self.entries)
    }
}

impl<'a> LocSingleScope<'a> {
    pub fn new(
        parent: &'a LocScope<'a>,
        name: &'a str,
        value: &'a ParsedValue,
    ) -> LocSingleScope<'a> {
        LocSingleScope {
            parent,
            name,
            value,
        }
    }

    fn get_value_by_name(&self, name: &str) -> &ParsedValue {
        if self.name == name {
            self.value
        } else {
            self.parent.get_value_by_name(name)
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, LocScopeEntry)>) {
        bindings.push((
            self.name.to_string().into(),
            LocScopeEntry::Value(self.value.clone()),
        ));
        self.parent.get_bindings(bindings);
    }
}

impl<'a> LocDecoderScope<'a> {
    fn new(parent: &'a LocScope<'a>, name: &'a str, decoder: Decoder) -> LocDecoderScope<'a> {
        LocDecoderScope {
            parent,
            name,
            decoder,
        }
    }

    fn get_decoder_by_name(&self, name: &str) -> &Decoder {
        if self.name == name {
            &self.decoder
        } else {
            self.parent.get_decoder_by_name(name)
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, LocScopeEntry)>) {
        bindings.push((
            self.name.to_string().into(),
            LocScopeEntry::Decoder(self.decoder.clone()),
        ));
        self.parent.get_bindings(bindings);
    }
}

impl Decoder {
    pub fn parse_with_loc<'input>(
        &self,
        program: &Program,
        scope: &LocScope<'_>,
        input: ReadCtxt<'input>,
    ) -> LocDecodeError<(ParsedValue, ReadCtxt<'input>)> {
        let start_offset = input.offset;
        match self {
            Decoder::Call(n, es) => {
                let mut new_scope = LocMultiScope::with_capacity(&LocScope::Empty, es.len());
                for (name, e) in es {
                    let v = e.eval_with_loc(scope).as_ref().clone();
                    new_scope.push(name.clone(), v);
                }
                program.decoders[*n]
                    .0
                    .parse_with_loc(program, &LocScope::Multi(&new_scope), input)
            }
            Decoder::SkipRemainder => {
                let start = input.offset;
                let input = input.skip_remainder();
                let ret = ParsedValue::unit_spanning(start, input.offset - start);
                Ok((ret, input))
            }
            Decoder::Fail => Err(DecodeError::<ParsedValue>::loc_fail(scope, input)),
            Decoder::EndOfInput => match input.read_byte() {
                None => Ok((ParsedValue::unit_at(start_offset), input)),
                Some((b, _)) => Err(DecodeError::<ParsedValue>::trailing(b, input.offset)),
            },
            Decoder::Align(n) => {
                let skip = (n - (input.offset % n)) % n;
                let (_, input) = input
                    .split_at(skip)
                    .ok_or(DecodeError::overrun(skip, input.offset))?;
                Ok((ParsedValue::unit_spanning(start_offset, skip), input))
            }
            Decoder::Pos => {
                let pos = input.offset as u64;
                Ok((ParsedValue::from_evaluated(Value::U64(pos)), input))
            }
            Decoder::ForEach(expr, lbl, a) => {
                let mut input = input;
                let val = expr.eval_with_loc(scope);
                let seq = val.get_sequence().expect("bad type for ForEach input");
                let mut v = Vec::with_capacity(seq.len());
                for e in seq {
                    let new_scope = LocScope::Single(LocSingleScope::new(scope, lbl, &e));
                    let (va, next_input) = a.parse_with_loc(program, &new_scope, input)?;
                    v.push(va);
                    input = next_input;
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_seq(v, start_offset, total_len), input))
            }
            Decoder::DecodeBytes(bytes, a) => {
                let bytes = {
                    let raw = bytes.eval_value_with_loc(scope);
                    let seq_vals = raw.get_sequence().expect("bad type for DecodeBytes input");
                    seq_vals
                        .into_iter()
                        .map(|v| v.get_as_u8())
                        .collect::<Vec<u8>>()
                };
                let new_input = ReadCtxt::new(&bytes);
                let (va, rem_input) = a.parse_with_loc(program, scope, new_input)?;
                match rem_input.read_byte() {
                    Some((b, _)) => {
                        // FIXME - this error-value doesn't properly distinguish between offsets within the main input or the sub-buffer
                        Err(DecodeError::Trailing {
                            byte: b,
                            offset: rem_input.offset,
                        })
                    }
                    None => Ok((va, input)),
                }
            }
            Decoder::Byte(bs) => {
                let (b, input) = input
                    .read_byte()
                    .ok_or(DecodeError::overbyte(input.offset))?;
                if bs.contains(b) {
                    Ok((ParsedValue::new_flat(Value::U8(b), start_offset, 1), input))
                } else {
                    Err(DecodeError::unexpected(b, *bs, input.offset))
                }
            }
            Decoder::Variant(label, d) => {
                let (v, input) = d.parse_with_loc(program, scope, input)?;
                Ok((ParsedValue::Variant(label.clone(), Box::new(v)), input))
            }
            Decoder::Branch(tree, branches) => {
                let index = tree.matches(input).ok_or(DecodeError::NoValidBranch {
                    offset: input.offset,
                })?;
                let d = &branches[index];
                let (v, input) = d.parse_with_loc(program, scope, input)?;
                Ok((ParsedValue::Branch(index, Box::new(v)), input))
            }
            Decoder::Parallel(branches) => {
                for (index, d) in branches.iter().enumerate() {
                    let res = d.parse_with_loc(program, scope, input);
                    if let Ok((v, input)) = res {
                        return Ok((ParsedValue::Branch(index, Box::new(v)), input));
                    }
                }
                Err(DecodeError::loc_fail(scope, input))
            }
            Decoder::Tuple(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for f in fields {
                    let (vf, next_input) = f.parse_with_loc(program, scope, input)?;
                    input = next_input;
                    v.push(vf.clone());
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_tuple(v, start_offset, total_len), input))
            }
            Decoder::Record(fields) => {
                let mut input = input;
                let mut record_scope = LocMultiScope::with_capacity(scope, fields.len());
                for (name, f) in fields {
                    let (vf, next_input) =
                        f.parse_with_loc(program, &LocScope::Multi(&record_scope), input)?;
                    record_scope.push(name.clone(), vf);
                    input = next_input;
                }
                Ok((record_scope.into_record(), input))
            }
            Decoder::While(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while tree.matches(input).ok_or(DecodeError::NoValidBranch {
                    offset: input.offset,
                })? == 0
                {
                    let (va, next_input) = a.parse_with_loc(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_seq(v, start_offset, total_len), input))
            }
            Decoder::Until(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse_with_loc(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                    if tree.matches(input).ok_or(DecodeError::NoValidBranch {
                        offset: input.offset,
                    })? == 0
                    {
                        break;
                    }
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_seq(v, start_offset, total_len), input))
            }
            Decoder::RepeatCount(expr, a) => {
                let mut input = input;
                let count = expr.eval_value_with_loc(scope).unwrap_usize();
                let mut v = Vec::with_capacity(count);
                for _ in 0..count {
                    let (va, next_input) = a.parse_with_loc(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_seq(v, start_offset, total_len), input))
            }
            Decoder::RepeatBetween(tree, min, max, a) => {
                let mut input = input;
                let min = min.eval_value_with_loc(scope).unwrap_usize();
                let max = max.eval_value_with_loc(scope).unwrap_usize();
                let mut v = Vec::new();
                loop {
                    if tree.matches(input).ok_or(DecodeError::NoValidBranch {
                        offset: input.offset,
                    })? == 0
                        || v.len() == max
                    {
                        if v.len() < min {
                            unreachable!("incoherent bounds for RepeatBetween(_, {min}, {max}, _)");
                        }
                        break;
                    }
                    let (va, next_input) = a.parse_with_loc(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_seq(v, start_offset, total_len), input))
            }
            Decoder::RepeatUntilLast(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse_with_loc(program, scope, input)?;
                    input = next_input;
                    let done = expr.eval_lambda_with_loc(scope, &va).unwrap_bool();
                    v.push(va);
                    if done {
                        break;
                    }
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_seq(v, start_offset, total_len), input))
            }
            Decoder::RepeatUntilSeq(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse_with_loc(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                    let vs = ParsedValue::from_evaluated_seq(v);
                    let done = expr.eval_lambda_with_loc(scope, &vs).unwrap_bool();
                    v = match vs {
                        ParsedValue::Seq(v) => v.inner.into_vec(),
                        _ => unreachable!(),
                    };
                    if done {
                        break;
                    }
                }
                let total_len = input.offset - start_offset;
                Ok((ParsedValue::new_seq(v, start_offset, total_len), input))
            }
            Decoder::AccumUntil(f_done, f_update, init, _vt, a) => {
                let mut input = input;
                let mut v = Vec::new();
                let mut accum = init.eval_value_with_loc(scope);
                loop {
                    let done_arg = ParsedValue::from_evaluated(Value::Tuple(vec![
                        accum.clone(),
                        ParsedValue::new_seq(v.clone(), start_offset, input.offset - start_offset)
                            .clone_into_value(),
                    ]));
                    let is_done = f_done.eval_lambda_with_loc(&scope, &done_arg).unwrap_bool();
                    if is_done {
                        break;
                    }
                    let (next_elem, next_input) = a.parse_with_loc(program, scope, input)?;
                    v.push(next_elem.clone());
                    let update_arg = ParsedValue::from_evaluated(Value::Tuple(vec![
                        accum.clone(),
                        next_elem.clone_into_value(),
                    ]));
                    let next_accum = f_update.eval_lambda_with_loc(scope, &update_arg);
                    accum = next_accum;
                    input = next_input;
                }
                let total_len = input.offset - start_offset;
                Ok((
                    ParsedValue::new_tuple(
                        vec![
                            ParsedValue::from_evaluated(accum),
                            ParsedValue::new_seq(v, start_offset, total_len),
                        ],
                        start_offset,
                        total_len,
                    ),
                    input,
                ))
            }
            Decoder::Maybe(expr, a) => {
                let is_present = expr.eval_value_with_loc(scope).unwrap_bool();
                if is_present {
                    let (raw, next_input) = a.parse_with_loc(program, scope, input)?;
                    Ok((ParsedValue::Option(Some(Box::new(raw))), next_input))
                } else {
                    Ok((ParsedValue::Option(None), input))
                }
            }
            Decoder::Peek(a) => {
                let (v, _next_input) = a.parse_with_loc(program, scope, input)?;
                Ok((v, input))
            }
            Decoder::LetFormat(da, name, db) => {
                let (va, input) = da.parse_with_loc(program, scope, input)?;
                let new_scope = LocScope::Single(LocSingleScope::new(scope, name, &va));
                db.parse_with_loc(program, &new_scope, input)
            }
            Decoder::PeekNot(a) => {
                if a.parse_with_loc(program, scope, input).is_ok() {
                    Err(DecodeError::loc_fail(scope, input))
                } else {
                    Ok((ParsedValue::unit_at(start_offset), input))
                }
            }
            Decoder::Slice(expr, a) => {
                let size = expr.eval_value_with_loc(scope).unwrap_usize();
                let (slice, input) = input
                    .split_at(size)
                    .ok_or(DecodeError::overrun(size, input.offset))?;
                let (v, _) = a.parse_with_loc(program, scope, slice)?;
                Ok((v, input))
            }
            Decoder::Bits(a) => {
                let mut bits = Vec::with_capacity(input.remaining().len() * 8);
                for b in input.remaining() {
                    for i in 0..8 {
                        bits.push((b & (1 << i)) >> i);
                    }
                }
                let (v, bits) = a.parse_with_loc(program, scope, ReadCtxt::new(&bits))?;
                let bytes_remain = bits.remaining().len() >> 3;
                let bytes_read = input.remaining().len() - bytes_remain;
                let (_, input) = input
                    .split_at(bytes_read)
                    .ok_or(DecodeError::overrun(bytes_read, input.offset))?;
                Ok((v, input))
            }
            Decoder::WithRelativeOffset(base_addr, expr, a) => {
                let base_addr = base_addr.eval_value_with_loc(scope).unwrap_usize();
                let offset = expr.eval_value_with_loc(scope).unwrap_usize();
                let abs_offset = base_addr + offset;
                let seek_input = input
                    .seek_to(abs_offset)
                    .ok_or(DecodeError::bad_seek(abs_offset, input.input.len()))?;
                let (v, _) = a.parse_with_loc(program, scope, seek_input)?;
                Ok((v, input))
            }
            Decoder::Map(d, expr) => {
                let (orig, input) = d.parse_with_loc(program, scope, input)?;
                let v = expr.eval_lambda_with_loc(scope, &orig);
                let image = ParsedValue::inherit(&orig, v);
                Ok((ParsedValue::Mapped(Box::new(orig), Box::new(image)), input))
            }
            Decoder::Where(d, expr) => {
                let (v, input) = d.parse_with_loc(program, scope, input)?;
                match expr.eval_lambda_with_loc(scope, &v).unwrap_bool() {
                    true => Ok((v, input)),
                    false => Err(DecodeError::loc_bad_where(scope, *expr.clone(), v)),
                }
            }
            Decoder::Compute(expr) => {
                let v = expr.eval_with_loc(scope);
                Ok((v.as_ref().clone(), input))
            }
            Decoder::Let(name, expr, d) => {
                let v = expr.eval_with_loc(scope).as_ref().clone();
                let let_scope = LocSingleScope::new(scope, name, &v);
                d.parse_with_loc(program, &LocScope::Single(let_scope), input)
            }
            Decoder::Match(head, branches) => {
                let head = head.eval_with_loc(scope);
                for (index, (pattern, decoder)) in branches.iter().enumerate() {
                    if let Some(pattern_scope) = head.matches(scope, pattern) {
                        let (v, input) = decoder.parse_with_loc(
                            program,
                            &LocScope::Multi(&pattern_scope),
                            input,
                        )?;
                        return Ok((ParsedValue::Branch(index, Box::new(v)), input));
                    }
                }
                panic!("non-exhaustive patterns");
            }
            Decoder::Dynamic(name, DynFormat::Huffman(lengths_expr, opt_values_expr), d) => {
                let lengths_val = lengths_expr.eval_with_loc(scope);
                let lengths = value_to_vec_usize(lengths_val.as_ref());
                let lengths = match opt_values_expr {
                    None => lengths,
                    Some(e) => {
                        let values = value_to_vec_usize(e.eval_with_loc(scope).as_ref());
                        let mut new_lengths = [0].repeat(values.len());
                        for i in 0..lengths.len() {
                            new_lengths[values[i]] = lengths[i];
                        }
                        new_lengths
                    }
                };
                let f = make_huffman_codes(&lengths);
                let dyn_d = Compiler::compile_one(&f).unwrap();
                let child_scope = LocDecoderScope::new(scope, name, dyn_d);
                d.parse_with_loc(program, &LocScope::Decoder(child_scope), input)
            }
            Decoder::Apply(name) => {
                let d = scope.get_decoder_by_name(name);
                d.parse_with_loc(program, scope, input)
            }
        }
    }
}

fn value_to_vec_usize(v: &ParsedValue) -> Vec<usize> {
    let vs = match v.clone_into_value() {
        Value::Seq(vs) => vs,
        _ => panic!("expected Seq"),
    };
    vs.iter()
        .map(|v| match v.coerce_mapped_value() {
            Value::U8(n) => *n as usize,
            _ => panic!("expected U8"),
        })
        .collect::<Vec<usize>>()
}

fn make_huffman_codes(lengths: &[usize]) -> Format {
    let max_length = *lengths.iter().max().unwrap();
    let mut bl_count = [0].repeat(max_length + 1);

    for len in lengths {
        bl_count[*len] += 1;
    }

    let mut next_code = [0].repeat(max_length + 1);
    let mut code = 0;
    bl_count[0] = 0;

    for bits in 1..max_length + 1 {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }

    let mut codes = Vec::with_capacity(lengths.len());

    for (n, &len) in lengths.iter().enumerate() {
        if len != 0 {
            codes.push(Format::Map(
                Box::new(bit_range(len, next_code[len])),
                Box::new(Expr::Lambda(
                    "_".into(),
                    Box::new(Expr::U16(n.try_into().unwrap())),
                )),
            ));
            //println!("{:?}", codes[codes.len()-1]);
            next_code[len] += 1;
        } else {
            //codes.push((n.to_string(), Format::Fail));
        }
    }

    Format::Union(codes)
}

fn bit_range(n: usize, bits: usize) -> Format {
    let mut fs = Vec::with_capacity(n);
    for i in 0..n {
        let r = n - 1 - i;
        let b = (bits & (1 << r)) >> r != 0;
        fs.push(is_bit(b));
    }
    Format::Tuple(fs)
}

fn is_bit(b: bool) -> Format {
    Format::Byte(ByteSet::from([if b { 1 } else { 0 }]))
}
