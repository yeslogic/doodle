#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::collections::HashSet;
use std::ops::Add;
use std::rc::Rc;

use serde::Serialize;
use anyhow::{anyhow, Result as AResult};

use crate::bounds::Bounds;
use crate::byte_set::ByteSet;
use crate::read::ReadCtxt;

pub mod bounds;
pub mod byte_set;
pub mod codegen;
pub mod decoder;
pub mod error;

pub mod output;
mod precedence;
pub mod prelude;
pub mod read;

mod extension;
// use extension::{UD, TC, Extension, TotalExtension, HOInfo, MCInfo, NLInfo, MaybeCast};

mod typecheck;
use typecheck::UnificationError;

pub type Label = std::borrow::Cow<'static, str>;

pub trait IntoLabel: Into<Label> {}

impl<T> IntoLabel for T where T: Into<Label> {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Pattern {
    Binding(Label),
    Wildcard,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Char(char),
    Tuple(Vec<Pattern>),
    Variant(Label, Box<Pattern>),
    Seq(Vec<Pattern>),
}

impl Pattern {
    pub const UNIT: Pattern = Pattern::Tuple(Vec::new());

    pub fn from_bytes(bs: &[u8]) -> Pattern {
        Pattern::Seq(bs.iter().copied().map(Pattern::U8).collect())
    }

    pub fn variant(label: impl IntoLabel, value: impl Into<Box<Self>>) -> Self {
        Pattern::Variant(label.into(), value.into())
    }

    pub fn binding(name: impl IntoLabel) -> Pattern {
        Pattern::Binding(name.into())
    }

    fn build_scope(&self, scope: &mut TypeScope<'_>, t: Rc<ValueType>) {
        match (self, t.as_ref()) {
            (Pattern::Binding(name), t) => {
                // FIXME - do we want to store an Rc<ValueType> in the scope instead, perhaps...?
                scope.push(name.clone(), t.clone());
            }
            (Pattern::Wildcard, _) => {}
            (Pattern::Bool(..), ValueType::Ground(GroundType::Bool)) => {}
            (Pattern::U8(..), ValueType::Ground(GroundType::U8)) => {}
            (Pattern::U16(..), ValueType::Ground(GroundType::U16)) => {}
            (Pattern::U32(..), ValueType::Ground(GroundType::U32)) => {}
            (Pattern::Tuple(ps), ValueType::Tuple(ts)) if ps.len() == ts.len() => {
                for (p, t) in Iterator::zip(ps.iter(), ts.iter()) {
                    p.build_scope(scope, Rc::new(t.clone()));
                }
            }
            (Pattern::Seq(ps), ValueType::Seq(t)) => {
                for p in ps {
                    p.build_scope(scope, Rc::new((**t).clone()));
                }
            }
            (Pattern::Variant(label, p), ValueType::Union(branches)) => {
                if let Some((_l, t)) = branches.iter().find(|(l, _t)| label == l) {
                    // FIXME - this is pretty bad, but it is hard to do better without more destructive changes
                    let tmp = Rc::new(t.clone());
                    p.build_scope(scope, tmp);
                } else {
                    panic!("no {label} in {branches:?}");
                }
            }
            _ => panic!("pattern build_scope failed"),
        }
    }

    fn infer_expr_branch_type(
        &self,
        scope: &TypeScope<'_>,
        head_type: Rc<ValueType>,
        expr: &Expr
    ) -> AResult<ValueType> {
        let mut pattern_scope = TypeScope::child(scope);
        self.build_scope(&mut pattern_scope, head_type);
        expr.infer_type(&pattern_scope)
    }

    fn infer_format_branch_type(
        &self,
        scope: &TypeScope<'_>,
        head_type: Rc<ValueType>,
        module: &FormatModule,
        format: &Format
    ) -> AResult<ValueType> {
        let mut pattern_scope = TypeScope::child(scope);
        self.build_scope(&mut pattern_scope, head_type);
        module.infer_format_type(&pattern_scope, format)
    }
}

pub enum ValueKind {
    Value(ValueType),
    Format(ValueType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash)]
pub enum GroundType {
    Bool,
    U8,
    U16,
    U32,
    Char,
}
impl GroundType {
    fn is_numeric(&self) -> bool {
        matches!(self, Self::U8 | Self::U16 | Self::U32)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum ValueType {
    Any,
    Empty,
    Ground(GroundType),
    Tuple(Vec<ValueType>),
    Record(Vec<(Label, ValueType)>),
    Union(Vec<(Label, ValueType)>),
    Seq(Box<ValueType>),
}

impl ValueType {
    pub const UNIT: ValueType = ValueType::Tuple(Vec::new());

    fn record_proj(&self, label: &str) -> ValueType {
        match self {
            ValueType::Record(fields) =>
                match fields.iter().find(|(l, _)| label == l) {
                    Some((_, t)) => t.clone(),
                    None => panic!("{label} not found in record type"),
                }
            _ => panic!("expected record type"),
        }
    }

    fn unwrap_tuple_type(self) -> Vec<ValueType> {
        match self {
            ValueType::Tuple(ts) => ts,
            _ => panic!("type is not a tuple"),
        }
    }

    fn as_tuple_type(&self) -> &[ValueType] {
        match self {
            ValueType::Tuple(ts) => ts.as_slice(),
            other => panic!("type is not a tuple: {other:?}")
        }
    }

    fn is_numeric_type(&self) -> bool {
        match self {
            ValueType::Ground(g) => g.is_numeric(),
            _ => false,
        }
    }

    fn unify(&self, other: &ValueType) -> Result<ValueType, UnificationError<ValueType>> {
        match (self, other) {
            (ValueType::Any, rhs) => Ok(rhs.clone()),
            (lhs, ValueType::Any) => Ok(lhs.clone()),
            (ValueType::Empty, ValueType::Empty) => Ok(ValueType::Empty),
            (ValueType::Ground(g1), ValueType::Ground(g2)) => {
                if g1 == g2 {
                    Ok(ValueType::Ground(*g1))
                } else {
                    Err(UnificationError::Unsatisfiable(self.clone(), other.clone()))
                }
            }
            (ValueType::Tuple(ts1), ValueType::Tuple(ts2)) => {
                if ts1.len() != ts2.len() {
                    // tuple arity mismatch
                    return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                }
                let mut ts = Vec::new();
                for (t1, t2) in Iterator::zip(ts1.iter(), ts2.iter()) {
                    ts.push(t1.unify(t2)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            (ValueType::Record(fs1), ValueType::Record(fs2)) => {
                if fs1.len() != fs2.len() {
                    // field count mismatch
                    return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                }
                // NOTE - because fields are parsed in declared order, two records with conflicting field orders are not operationally equivalent
                let mut fs = Vec::new();
                for ((l1, t1), (l2, t2)) in Iterator::zip(fs1.iter(), fs2.iter()) {
                    if l1 != l2 {
                        // field label mismatch
                        return Err(UnificationError::Unsatisfiable(self.clone(), other.clone()));
                    }
                    fs.push((l1.clone(), t1.unify(t2)?));
                }
                Ok(ValueType::Record(fs))
            }
            (ValueType::Union(bs1), ValueType::Union(bs2)) => {
                let mut bs: Vec<(Label, ValueType)> = Vec::new();
                for (label, t2) in bs2 {
                    let t = if let Some((_l, t1)) = bs.iter().find(|(l, _)| label == l) {
                        t1.unify(t2)?
                    } else {
                        t2.clone()
                    };
                    bs.push((label.clone(), t));
                }
                for (label, t1) in bs1 {
                    if !bs.iter().any(|(l, _)| label == l) {
                        bs.push((label.clone(), t1.clone()));
                    }
                }
                Ok(ValueType::Union(bs))
            }
            (ValueType::Seq(t1), ValueType::Seq(t2)) => Ok(ValueType::Seq(Box::new(t1.unify(t2)?))),
            (t1, t2) => Err(UnificationError::Unsatisfiable(t1.clone(), t2.clone())),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum IntRel {
    Eq,
    Ne,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum Arith {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    Shl,
    Shr,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Expr {
    Var(Label),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Expr>),
    TupleProj(Box<Expr>, usize),
    Record(Vec<(Label, Expr)>),
    RecordProj(Box<Expr>, Label),
    Variant(Label, Box<Expr>),
    Seq(Vec<Expr>),
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
    Lambda(Label, Box<Expr>),

    IntRel(IntRel, Box<Expr>, Box<Expr>),
    Arith(Arith, Box<Expr>, Box<Expr>),

    AsU8(Box<Expr>),
    AsU16(Box<Expr>),
    AsU32(Box<Expr>),
    AsChar(Box<Expr>),

    U16Be(Box<Expr>),
    U16Le(Box<Expr>),
    U32Be(Box<Expr>),
    U32Le(Box<Expr>),

    SeqLength(Box<Expr>),
    SubSeq(Box<Expr>, Box<Expr>, Box<Expr>),
    FlatMap(Box<Expr>, Box<Expr>),
    FlatMapAccum(Box<Expr>, Box<Expr>, ValueType, Box<Expr>),
    Dup(Box<Expr>, Box<Expr>),
    Inflate(Box<Expr>),
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum HigherOrderType {
//     Point(ValueType),
//     Arrow(ValueType, ValueType),
// }

impl Expr {
    pub const UNIT: Self = Expr::Tuple(Vec::new());

    pub fn record_proj(head: impl Into<Box<Expr>>, label: impl IntoLabel) -> Expr {
        let head: Box<Expr> = head.into();
        let label: Label = label.into();

        Expr::RecordProj(head.into(), label.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjKind {
    Record,
    Tuple,
}

impl Expr {
    // FIXME: is this still an inherent method, or should we have a UD -> TC phase and use get_type_info instead?
    fn infer_type(&self, scope: &TypeScope<'_>) -> AResult<ValueType> {
        match self {
            Expr::Var(name) =>
                match scope.get_type_by_name(name) {
                    ValueKind::Value(t) => Ok(t.clone()),
                    ValueKind::Format(_t) => Err(anyhow!("expected ValueKind::Value, found ValueKind::Format for var {name}")),
                }
            Expr::Bool(_b) => Ok(ValueType::Ground(GroundType::Bool)),
            Expr::U8(_n) => Ok(ValueType::Ground(GroundType::U8)),
            Expr::U16(_n) => Ok(ValueType::Ground(GroundType::U16)),
            Expr::U32(_n) => Ok(ValueType::Ground(GroundType::U32)),
            Expr::Tuple(exprs) => {
                let mut ts = Vec::new();
                for expr in exprs {
                    ts.push(expr.infer_type(scope)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            Expr::TupleProj(head, index) =>
                match head.infer_type(scope)? {
                    ValueType::Tuple(vs) => Ok(vs[*index].clone()),
                    other => Err(anyhow!("tuple projection on non-tuple type {other:?}")),
                }
            Expr::Record(fields) => {
                let mut fs = Vec::new();
                for (label, expr) in fields {
                    fs.push((label.clone(), expr.infer_type(scope)?));
                }
                Ok(ValueType::Record(fs))
            }
            // FIXME - TupleProj
            Expr::RecordProj(head, label) => Ok(head.infer_type(scope)?.record_proj(label)),
            Expr::Variant(label, expr) =>
                Ok(ValueType::Union(vec![(label.clone(), expr.infer_type(scope)?)])),
            Expr::Seq(exprs) => {
                let mut t = ValueType::Any;
                for e in exprs {
                    t = t.unify(&e.infer_type(scope)?)?;
                }
                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::Match(head, branches) => {
                if branches.is_empty() {
                    return Err(anyhow!("cannot infer type of empty match expression"));
                }
                let head_type = Rc::new(head.infer_type(scope)?);
                let mut t = ValueType::Any;
                for (pattern, branch) in branches {
                    t = t.unify(&pattern.infer_expr_branch_type(scope, head_type.clone(), branch)?)?;
                }
                Ok(t)
            }
            Expr::Lambda(..) => Err(anyhow!("infer_type encountered unexpected lambda")),

            Expr::IntRel(_rel, x, y) => match (x.infer_type(scope)?, y.infer_type(scope)?) {
                (ValueType::Ground(g1), ValueType::Ground(g2)) if g1 == g2 && g1.is_numeric() => Ok(ValueType::Bool),
                (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
            },
            Expr::Arith(_arith, x, y) => match (x.infer_type(scope)?, y.infer_type(scope)?) {
                (ValueType::U8, ValueType::U8) => Ok(ValueType::U8),
                (ValueType::U16, ValueType::U16) => Ok(ValueType::U16),
                (ValueType::U32, ValueType::U32) => Ok(ValueType::U32),
                (x, y) => Err(format!("mismatched operands {x:?}, {y:?}")),
            },

            Expr::AsU8(x) =>
                match x.infer_type(scope)? {
                    ValueType::Ground(g) if g.is_numeric() => Ok(ValueType::Ground(GroundType::U8)),
                    x => Err(anyhow!("unsound type cast AsU8(_ : {x:?})")),
                }
            Expr::AsU16(x) =>
                match x.infer_type(scope)? {
                    ValueType::Ground(g) if g.is_numeric() =>
                        Ok(ValueType::Ground(GroundType::U16)),
                    x => Err(anyhow!("unsound type cast AsU16(_ : {x:?})")),
                }
            Expr::AsU32(x) =>
                match x.infer_type(scope)? {
                    ValueType::Ground(g) if g.is_numeric() =>
                        Ok(ValueType::Ground(GroundType::U32)),
                    x => Err(anyhow!("unsound type cast AsU32(_ : {x:?})")),
                }
            Expr::AsChar(x) =>
                match x.infer_type(scope)? {
                    ValueType::Ground(g) if g.is_numeric() =>
                        Ok(ValueType::Ground(GroundType::Char)),
                    x => Err(anyhow!("unsound type cast AsChar(_ : {x:?})")),
                }

            Expr::U16Be(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [ValueType::Ground(GroundType::U8), ValueType::Ground(GroundType::U8)] =>
                        Ok(ValueType::Ground(GroundType::U16)),
                    _ => Err(anyhow!("unsound byte-level type cast U16Be(_ : {_t:?})")),
                }
            }
            Expr::U16Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [ValueType::Ground(GroundType::U8), ValueType::Ground(GroundType::U8)] =>
                        Ok(ValueType::Ground(GroundType::U16)),
                    _ => Err(anyhow!("unsound byte-level type cast U16Le(_ : {_t:?})")),
                }
            }
            Expr::U32Be(bytes)  => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [
                        ValueType::Ground(GroundType::U8),
                        ValueType::Ground(GroundType::U8),
                        ValueType::Ground(GroundType::U8),
                        ValueType::Ground(GroundType::U8),
                    ] => Ok(ValueType::Ground(GroundType::U32)),
                    _ => Err(anyhow!("unsound byte-level type cast U32Be(_ : {_t:?})")),
                }
            }
            Expr::U32Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [
                        ValueType::Ground(GroundType::U8),
                        ValueType::Ground(GroundType::U8),
                        ValueType::Ground(GroundType::U8),
                        ValueType::Ground(GroundType::U8),
                    ] => Ok(ValueType::Ground(GroundType::U32)),
                    _ => Err(anyhow!("unsound byte-level type cast U32Le(_ : {_t:?})")),
                }
            }
            Expr::SeqLength(seq) =>
                match seq.infer_type(scope)? {
                    ValueType::Seq(_t) => Ok(ValueType::Ground(GroundType::U32)),
                    other => Err(anyhow!("seq-length called on non-sequence type: {other:?}")),
                }
            Expr::SubSeq(seq, start, length) =>
                match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        let start_type = start.infer_type(scope)?;
                        let length_type = length.infer_type(scope)?;
                        if !start_type.is_numeric_type() {
                            return Err(
                                anyhow!("subseq start value should have numeric type, found {start_type:?}")
                            );
                        }
                        if !length_type.is_numeric_type() {
                            return Err(
                                anyhow!("subseq length value should have numeric type, found {length_type:?}")
                            );
                        }
                        Ok(ValueType::Seq(t))
                    }
                    other => Err(anyhow!("subseq called on non-sequence type: {other:?}")),
                }
            Expr::FlatMap(expr, seq) =>
                match expr.as_ref() {
                    Expr::Lambda(name, expr) =>
                        match seq.infer_type(scope)? {
                            ValueType::Seq(t) => {
                                let mut child_scope = TypeScope::child(scope);
                                child_scope.push(name.clone(), *t);
                                match expr.infer_type(&child_scope)? {
                                    ValueType::Seq(t2) => Ok(ValueType::Seq(t2)),
                                    other => Err(anyhow!("flat-map lambda output is non-sequence type: {other:?}")),
                                }
                            }
                            other => Err(anyhow!("flat-map called on non-sequence type: {other:?}")),
                        }
                    other => Err(anyhow!("FlatMap: expected Lambda, found {other:?}")),
                }
            Expr::FlatMapAccum(expr, accum, accum_type, seq) =>
                match expr.as_ref() {
                    Expr::Lambda(name, expr) =>
                        match seq.infer_type(scope)? {
                            ValueType::Seq(t) => {
                                let accum_type = accum.infer_type(scope)?.unify(accum_type)?;
                                let mut child_scope = TypeScope::child(scope);
                                child_scope.push(
                                    name.clone(),
                                    ValueType::Tuple(vec![accum_type.clone(), *t])
                                );
                                match
                                    expr
                                        .infer_type(&child_scope)?
                                        .unwrap_tuple_type()
                                        .as_mut_slice()
                                {
                                    [accum_result, ValueType::Seq(t2)] => {
                                        accum_result.unify(&accum_type)?;
                                        Ok(ValueType::Seq(t2.clone()))
                                    }
                                    _ => Err(anyhow!("FlatMapAccum: expected two values")),
                                }
                            }
                            other => Err(anyhow!("FlatMapAccum: expected Seq, found {other:?}")),
                        }
                    other => Err(anyhow!("FlatMapAccum: expected Lambda, found {other:?}")),
                }
            Expr::Dup(count, expr) => {
                if !count.infer_type(scope)?.is_numeric_type() {
                    return Err(anyhow!("Dup: count is not numeric: {count:?}"));
                }
                let t = expr.infer_type(scope)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::Inflate(seq) =>
                match seq.infer_type(scope)? {
                    // FIXME should check values are appropriate variants
                    ValueType::Seq(_values) =>
                        Ok(ValueType::Seq(Box::new(ValueType::Ground(GroundType::U8)))),
                    other => Err(anyhow!("Inflate: expected Seq, found {other:?}")),
                }
        }
    }

    /// Conservative bounds for unsigned numeric expressions
    fn bounds(&self) -> Bounds {
        match self {
            Expr::U8(n) => Bounds::exact(usize::from(*n)),
            Expr::U16(n) => Bounds::exact(usize::from(*n)),
            Expr::U32(n) => Bounds::exact(*n as usize),
            Expr::Arith(Arith::Add, a, b) => a.bounds() + b.bounds(),
            Expr::Arith(Arith::Mul, a, b) => a.bounds() * b.bounds(),
            _ => Bounds::new(0, None),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum DynFormat {
    Huffman(Expr, Option<Expr>),
}

/// Binary format descriptions
///
/// # Binary formats as regular expressions
///
/// Given a language of [regular expressions]:
///
/// ```text
/// r ∈ Regexp ::=
///   | ∅           empty set
///   | ε           empty byte string
///   | .           any byte
///   | b           literal byte
///   | r|r         alternation
///   | r r         concatenation
///   | r*          Kleene star
/// ```
///
/// We can use these to model a subset of our binary format descriptions:
///
/// ```text
/// ⟦ _ ⟧ : Format ⇀ Regexp
/// ⟦ Fail ⟧                                = ∅
/// ⟦ Byte({}) ⟧                            = ∅
/// ⟦ Byte(!{}) ⟧                           = .
/// ⟦ Byte({b}) ⟧                           = b
/// ⟦ Byte({b₀, ... bₙ}) ⟧                  = b₀ | ... | bₙ
/// ⟦ Union([]) ⟧                           = ∅
/// ⟦ Union([(l₀, f₀), ..., (lₙ, fₙ)]) ⟧    = ⟦ f₀ ⟧ | ... | ⟦ fₙ ⟧
/// ⟦ Tuple([]) ⟧                           = ε
/// ⟦ Tuple([f₀, ..., fₙ]) ⟧                = ⟦ f₀ ⟧ ... ⟦ fₙ ⟧
/// ⟦ Repeat(f) ⟧                           = ⟦ f ⟧*
/// ⟦ Repeat1(f) ⟧                          = ⟦ f ⟧ ⟦ f ⟧*
/// ⟦ RepeatCount(n, f) ⟧                   = ⟦ f ⟧ ... ⟦ f ⟧
///                                           ╰── n times ──╯
/// ```
///
/// Note that the data dependency present in record formats means that these
/// formats no longer describe regular languages.
///
/// [regular expressions]: https://en.wikipedia.org/wiki/Regular_expression#Formal_definition
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[derive(Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Format {
    /// Reference to a top-level item
    ItemVar(usize, Vec<Expr>), // FIXME - do the exprs here need type(+) info?
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Skips bytes if necessary to align the current offset to a multiple of N
    Align(usize),
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Wraps the value from the inner format in a variant
    Variant(Label, Box<Format>),
    /// Matches the union of all the formats, which must have the same type
    Union(Vec<Format>),
    /// Nondeterministic unions, where the formats are not mutually exclusive
    UnionNondet(Vec<Format>),
    /// Matches a sequence of concatenated formats
    Tuple(Vec<Format>),
    /// Matches a sequence of named formats where later formats can depend on
    /// the decoded value of earlier formats
    Record(Vec<(Label, Format)>),
    /// Repeat a format zero-or-more times
    Repeat(Box<Format>),
    /// Repeat a format one-or-more times
    Repeat1(Box<Format>),
    /// Repeat a format an exact number of times
    RepeatCount(Expr, Box<Format>),
    /// Repeat a format until a condition is satisfied by its last item
    RepeatUntilLast(Expr, Box<Format>),
    /// Repeat a format until a condition is satisfied by the sequence
    RepeatUntilSeq(Expr, Box<Format>),
    /// Parse a format without advancing the stream position afterwards
    Peek(Box<Format>),
    /// Attempt to parse a format and fail if it succeeds
    PeekNot(Box<Format>),
    /// Restrict a format to a sub-stream of a given number of bytes (skips any leftover bytes in the sub-stream)
    Slice(Expr, Box<Format>),
    /// Parse bitstream
    Bits(Box<Format>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(Expr, Box<Format>),
    /// Map a value with a lambda expression
    Map(Box<Format>, Expr),
    /// Compute a value
    Compute(Expr),
    /// Let binding
    Let(Label, Expr, Box<Format>),
    /// Pattern match on an expression
    Match(Expr, Vec<(Pattern, Format)>),
    /// Format generated dynamically
    Dynamic(Label, DynFormat, Box<Format>),
    /// Apply a dynamic format from a named variable in the scope
    Apply(Label),
}

impl Format {
    pub const EMPTY: Format = Format::Tuple(Vec::new());

    pub fn alts<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
        Format::Union(
            fields
                .into_iter()
                .map(|(label, format)| Format::Variant(label.into(), Box::new(format)))
                .collect()
        )
    }

    pub fn record<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
        Format::Record(
            fields
                .into_iter()
                .map(|(label, format)| (label.into(), format))
                .collect()
        )
    }
}

impl Format {
    /// Conservative bounds for number of bytes matched by a format
    fn match_bounds(&self, module: &FormatModule) -> Bounds {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).match_bounds(module),
            Format::Fail => Bounds::exact(0),
            Format::EndOfInput => Bounds::exact(0),
            Format::Align(n) => Bounds::new(0, Some(n - 1)),
            Format::Byte(_) => Bounds::exact(1),
            Format::Variant(_label, f) => f.match_bounds(module),
            Format::Union(branches) | Format::UnionNondet(branches) => branches
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Tuple(fields) => fields
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Record(fields) => fields
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Repeat(_) => Bounds::new(0, None),
            Format::Repeat1(f) => f.match_bounds(module) * Bounds::new(1, None),
            Format::RepeatCount(expr, f) => f.match_bounds(module) * expr.bounds(),
            Format::RepeatUntilLast(_, f) => f.match_bounds(module) * Bounds::new(1, None),
            Format::RepeatUntilSeq(_, _f) => Bounds::new(0, None),
            Format::Peek(_) => Bounds::exact(0),
            Format::PeekNot(_) => Bounds::exact(0),
            Format::Slice(expr, _) => expr.bounds(),
            Format::Bits(f) => f.match_bounds(module).bits_to_bytes(),
            Format::WithRelativeOffset(_, _) => Bounds::exact(0),
            Format::Map(f, _expr) => f.match_bounds(module),
            Format::Compute(_) => Bounds::exact(0),
            Format::Let(_name, _expr, f) => f.match_bounds(module),
            Format::Match(_, branches) =>
                branches
                    .iter()
                    .map(|(_, f)| f.match_bounds(module))
                    .reduce(Bounds::union)
                    .unwrap(),
            Format::Dynamic(_name, _dynformat, f) => f.match_bounds(module),
            Format::Apply(_) => Bounds::new(1, None),
        }
    }

    /// Returns `true` if the format could match the empty byte string
    fn is_nullable(&self, module: &FormatModule) -> bool {
        self.match_bounds(module).min == 0
    }

    /// True if the compilation of this format depends on the format that follows it
    fn depends_on_next(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).depends_on_next(module),
            Format::Fail => false,
            Format::EndOfInput => false,
            Format::Align(..) => false,
            Format::Byte(..) => false,
            Format::Variant(_label, f) => f.depends_on_next(module),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                Format::union_depends_on_next(branches, module)
            }
            Format::Tuple(fields) => fields.iter().any(|f| f.depends_on_next(module)),
            Format::Record(fields) => fields.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Repeat(..) => true,
            Format::Repeat1(..) => true,
            Format::RepeatCount(..) => false,
            Format::RepeatUntilLast(..) => false,
            Format::RepeatUntilSeq(..) => false,
            Format::Peek(..) => false,
            Format::PeekNot(..) => false,
            Format::Slice(..) => false,
            Format::Bits(..) => false,
            Format::WithRelativeOffset(..) => false,
            Format::Map(f, _expr) => f.depends_on_next(module),
            Format::Compute(..) => false,
            Format::Let(_name, _expr, f) => f.depends_on_next(module),
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Dynamic(_name, _dynformat, f) => f.depends_on_next(module),
            Format::Apply(..) => false,
        }
    }

    fn union_depends_on_next(branches: &[Format], module: &FormatModule) -> bool {
        let mut fs = Vec::with_capacity(branches.len());
        for f in branches {
            if f.depends_on_next(module) {
                return true;
            }
            fs.push(f.clone());
        }
        MatchTree::build(module, &fs, Rc::new(Next::Empty)).is_none()
    }
}

impl Format {
    /// Returns `true` if values associated to this format should be handled as single ASCII characters
    pub fn is_ascii_char_format(&self, module: &FormatModule) -> bool {
        match self {
            // NOTE - currently only true for named formats matching 'base\.ascii-char.*'
            Format::ItemVar(level, _args) => module.get_name(*level).starts_with("base.ascii-char"),
            _ => false,
        }
    }

    /// Returns `true` if values associated to this format should be handled as multi-character ASCII strings
    pub fn is_ascii_string_format(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => {
                let fmt_name = module.get_name(*level);
                // REVIEW - consider different heuristic for short-circuit
                if fmt_name.contains("ascii-string") || fmt_name.contains("asciiz-string") {
                    return true;
                }
                module.get_format(*level).is_ascii_string_format(module)
            }
            Format::Tuple(formats) => {
                !formats.is_empty() && formats.iter().all(|f| f.is_ascii_char_format(module))
            }
            | Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => format.is_ascii_char_format(module),
            Format::Slice(_, format) => format.is_ascii_string_format(module),
            // NOTE there may be other cases we should consider ASCII
            _ => false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct FormatRef(usize);

impl FormatRef {
    pub fn get_level(&self) -> usize {
        self.0
    }

    pub fn call(&self) -> Format {
        Format::ItemVar(self.0, vec![])
    }

    pub fn call_args(&self, args: Vec<Expr>) -> Format {
        Format::ItemVar(self.0, args)
    }
}

#[derive(Debug, Serialize)]
pub struct FormatModule {
    names: Vec<Label>,
    args: Vec<Vec<(Label, ValueType)>>,
    formats: Vec<Format>,
    format_types: Vec<ValueType>,
}

impl FormatModule {
    pub fn new() -> FormatModule {
        FormatModule {
            names: Vec::new(),
            args: Vec::new(),
            formats: Vec::new(),
            format_types: Vec::new(),
        }
    }

    pub fn define_format(&mut self, name: impl IntoLabel, format: Format) -> FormatRef {
        self.define_format_args(name, vec![], format)
    }

    pub fn define_format_args(
        &mut self,
        name: impl IntoLabel,
        args: Vec<(Label, ValueType)>,
        format: Format
    ) -> FormatRef {
        let mut scope = TypeScope::new();
        for (arg_name, arg_type) in &args {
            scope.push(arg_name.clone(), arg_type.clone());
        }
        let format_type = match self.infer_format_type(&scope, &format) {
            Ok(t) => t,
            Err(msg) => panic!("{msg}"),
        };
        let level = self.names.len();
        self.names.push(name.into());
        self.args.push(args);
        self.formats.push(format);
        self.format_types.push(format_type);
        FormatRef(level)
    }

    fn get_name(&self, level: usize) -> &str {
        &self.names[level]
    }

    fn get_args(&self, level: usize) -> &[(Label, ValueType)] {
        &self.args[level]
    }

    fn get_format(&self, level: usize) -> &Format {
        &self.formats[level]
    }

    pub fn get_format_type(&self, level: usize) -> &ValueType {
        &self.format_types[level]
    }

    fn infer_format_type(&self, scope: &TypeScope<'_>, f: &Format) -> AResult<ValueType> {
        match f {
            Format::ItemVar(level, arg_exprs) => {
                let arg_names = self.get_args(*level);
                for ((_name, arg_type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    let t = expr.infer_type(scope)?;
                    let _t = arg_type.unify(&t)?;
                }
                Ok(self.get_format_type(*level).clone())
            }
            Format::Fail => Ok(ValueType::Empty),
            Format::EndOfInput => Ok(ValueType::Tuple(vec![])),
            Format::Align(_n) => Ok(ValueType::Tuple(vec![])),
            Format::Byte(_bs) => Ok(ValueType::Ground(GroundType::U8)),
            Format::Variant(label, f) =>
                Ok(ValueType::Union(vec![(label.clone(), self.infer_format_type(scope, f)?)])),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut t = ValueType::Any;
                for f in branches {
                    t = t.unify(&self.infer_format_type(scope, f)?)?;
                }
                Ok(t)
            }
            Format::Tuple(fields) => {
                let mut ts = Vec::with_capacity(fields.len());
                for f in fields {
                    ts.push(self.infer_format_type(scope, f)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            Format::Record(fields) => {
                let mut ts = Vec::with_capacity(fields.len());
                let mut record_scope = TypeScope::child(scope);
                for (label, f) in fields {
                    let t = self.infer_format_type(&record_scope, f)?;
                    ts.push((label.clone(), t.clone()));
                    record_scope.push(label.clone(), t);
                }
                Ok(ValueType::Record(ts))
            }
            Format::Repeat(a) | Format::Repeat1(a) => {
                let t = self.infer_format_type(scope, a)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            | Format::RepeatCount(_expr, a)
            | Format::RepeatUntilLast(_expr, a)
            | Format::RepeatUntilSeq(_expr, a) => {
                let t = self.infer_format_type(scope, a)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Format::Peek(a) => self.infer_format_type(scope, a),
            Format::PeekNot(_a) => Ok(ValueType::Tuple(vec![])),
            Format::Slice(_expr, a) => self.infer_format_type(scope, a),
            Format::Bits(a) => self.infer_format_type(scope, a),
            Format::WithRelativeOffset(_expr, a) => self.infer_format_type(scope, a),
            Format::Map(a, expr) => {
                let arg_type = self.infer_format_type(scope, a)?;
                match expr {
                    Expr::Lambda(name, body) => {
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(name.clone(), arg_type);
                        body.infer_type(&child_scope)
                    }
                    other => Err(anyhow!("Map: expected lambda, found {other:?}")),
                }
            }
            Format::Compute(expr) => expr.infer_type(scope),
            Format::Let(name, expr, format) => {
                let t = expr.infer_type(scope)?;
                let mut child_scope = TypeScope::child(scope);
                child_scope.push(name.clone(), t);
                self.infer_format_type(&child_scope, format)
            }
            Format::Match(head, branches) => {
                if branches.is_empty() {
                    return Err(anyhow!("infer_format_type: empty Match"));
                }
                let head_type = Rc::new(head.infer_type(scope)?);
                let mut t = ValueType::Any;
                for (pattern, branch) in branches {
                    t = t.unify(
                        &pattern.infer_format_branch_type(scope, head_type.clone(), self, branch)?
                    )?;
                }
                Ok(t)
            }
            Format::Dynamic(name, dynformat, format) => {
                match dynformat {
                    DynFormat::Huffman(lengths_expr, _opt_values_expr) => {
                        match lengths_expr.infer_type(scope)? {
                            ValueType::Seq(t) =>
                                match &*t {
                                    | ValueType::Ground(GroundType::U8)
                                    | ValueType::Ground(GroundType::U16) => {}
                                    other => {
                                        return Err(
                                            anyhow!("Huffman: expected U8 or U16, found {other:?}")
                                        );
                                    }
                                }
                            other => {
                                return Err(anyhow!("Huffman: expected Seq, found {other:?}"));
                            }
                        }
                        // FIXME check opt_values_expr type
                    }
                }
                let mut child_scope = TypeScope::child(scope);
                child_scope.push_format(name.clone(), ValueType::Ground(GroundType::U16));
                self.infer_format_type(&child_scope, format)
            }
            Format::Apply(name) =>
                match scope.get_type_by_name(name) {
                    ValueKind::Format(t) => Ok(t.clone()),
                    ValueKind::Value(t) => Err(anyhow!("Apply: expected format, found {t:?}")),
                }
        }
    }
}

/// Incremental decomposition of a Format into a partially consumed head
/// sub-format, and a possibly-empty tail of remaining sub-formats.
///
/// All variants other than [`Next::Empty`] and [`Next::Union`] implicitly have a tail-recursive
/// element, which is invariably the final positional argument for that variant. In the case of
/// [`Next::Union`], the recursive descent is symmetric and may be balanced arbitrarily.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Next<'a> {
    Empty,
    Union(Rc<Next<'a>>, Rc<Next<'a>>),
    Cat(&'a Format, Rc<Next<'a>>),
    Tuple(&'a [Format], Rc<Next<'a>>),
    Record(&'a [(Label, Format)], Rc<Next<'a>>),
    Repeat(&'a Format, Rc<Next<'a>>),
    RepeatCount(usize, &'a Format, Rc<Next<'a>>),
    Slice(usize, Rc<Next<'a>>, Rc<Next<'a>>),
    Peek(Rc<Next<'a>>, Rc<Next<'a>>),
    PeekNot(Rc<Next<'a>>, Rc<Next<'a>>),
}

/// A single choice-point in a conceptual [MatchTree] structure.
#[derive(Clone, Debug)]
struct MatchTreeStep<'a> {
    accept: bool,
    branches: Vec<(ByteSet, Rc<Next<'a>>)>,
}

/// The superposition of choice-points at a common descent-depth into a conceptual [MatchTree] structure.
#[derive(Clone, Debug)]
struct MatchTreeLevel<'a> {
    accept: Option<usize>,
    branches: Vec<(ByteSet, LevelBranch<'a>)>,
}

/// A bundle of follow-sets with an externally significant index, e.g. into an array of decoders
type LevelBranch<'a> = HashSet<(usize, Rc<Next<'a>>)>;

/// A byte-level prefix-tree evaluated to a fixed depth.
#[derive(Clone, Debug)]
pub struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
}

impl<'a> MatchTreeStep<'a> {
    /// Returns a `MatchTreeStep` that rejects all inputs without branching.
    fn reject() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![],
        }
    }

    /// Returns a `MatchTreeStep` that accepts all inputs without branching.
    fn accept() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: true,
            branches: vec![],
        }
    }

    /// Constructs a `MatchTreeStep` consisting of a single branch, defined by the argument values.
    fn branch(bs: ByteSet, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![(bs, next)],
        }
    }

    /// Modifies a `MatchTreeStep` in place, so that it will accept a new branch given by the argument values.
    fn union_branch(&mut self, mut bs: ByteSet, next: Rc<Next<'a>>) {
        let mut branches = Vec::new();
        for (bs0, next0) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    branches.push((orig, next0.clone()));
                }
                *bs0 = common;
                *next0 = Rc::new(Next::Union(next0.clone(), next.clone()));
                bs = bs.difference(bs0);
            }
        }
        if !bs.is_empty() {
            self.branches.push((bs, next));
        }
        self.branches.append(&mut branches);
    }

    /// Combines two `MatchTreeSteps` into their logical union
    fn union(mut self, other: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        self.accept = self.accept || other.accept;
        for (bs, next) in other.branches {
            self.union_branch(bs, next);
        }
        self
    }

    /// Returns a modified version of `self` that rejects any input that is not
    /// accepted by `peek`.
    fn peek(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        if peek.accept {
            // can ignore peek as it has already accepted
        } else if self.accept {
            // can ignore self as it has already accepted
            self.accept = peek.accept;
            self.branches = peek.branches;
        } else {
            // take the intersection of peek and self branches
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches {
                for (bs2, next2) in &peek.branches {
                    let bs = bs1.intersection(bs2);
                    if !bs.is_empty() {
                        let next = Rc::new(Next::Peek(next1.clone(), next2.clone()));
                        branches.push((bs, next));
                    }
                }
            }
            self.branches = branches;
        }
        self
    }

    /// Returns a modified version of `self` that rejects any input that is
    /// accepted by `peek`.
    fn peek_not(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        if peek.accept {
            self.accept = false;
            self.branches = Vec::new();
        } else {
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches.into_iter() {
                for (bs2, next2) in &peek.branches {
                    let common = bs1.intersection(bs2);
                    let diff = bs1.difference(bs2);
                    if !common.is_empty() {
                        let next = Rc::new(Next::PeekNot(next1.clone(), next2.clone()));
                        branches.push((common, next));
                    }
                    if !diff.is_empty() {
                        branches.push((diff, next1.clone()));
                    }
                }
            }
            self.branches = branches;
        }
        self
    }

    /// Constructs a [MatchTreeStep] that accepts a given tuple of sequential formats, with a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_tuple(
        module: &'a FormatModule,
        fields: &'a [Format],
        next: Rc<Next<'a>>
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::from_next(module, next),
            Some((f, fs)) => Self::from_format(module, f, Rc::new(Next::Tuple(fs, next))),
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a given record of sequential formats, with a trailing sequence of partially-consumed formats ([`Next`]s).
    ///
    /// This is mostly equivalent to `from_tuple`, as the name of a given field does not have implications on the prefix tree of the overall format.
    fn from_record(
        module: &'a FormatModule,
        fields: &'a [(Label, Format)],
        next: Rc<Next<'a>>
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::from_next(module, next),
            Some(((_label, f), fs)) => {
                Self::from_format(module, f, Rc::new(Next::Record(fs, next)))
            }
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a fixed-count repetition of a given format, with a trailing sequence of partially-consumed formats ([`Next`s]).
    fn from_repeat_count(
        module: &'a FormatModule,
        n: usize,
        format: &'a Format,
        next: Rc<Next<'a>>
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            Self::from_format(module, format, Rc::new(Next::RepeatCount(n - 1, format, next)))
        } else {
            Self::from_next(module, next)
        }
    }

    /// Constructs a [MatchTreeStep] from a given (partial) format `inner` and a slice-length `n`
    pub fn from_slice(
        module: &'a FormatModule,
        n: usize,
        inner: Rc<Next<'a>>,
        next: Rc<Next<'a>>
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            let mut tree = Self::from_next(module, inner);
            tree.accept = false;
            if tree.branches.is_empty() {
                let next = Rc::new(Next::Slice(n - 1, Rc::new(Next::Empty), next.clone()));
                tree.branches.push((ByteSet::full(), next));
            } else {
                for (_bs, ref mut inside) in tree.branches.iter_mut() {
                    *inside = Rc::new(Next::Slice(n - 1, inside.clone(), next.clone()));
                }
            }
            tree
        } else {
            Self::from_next(module, next.clone())
        }
    }

    /// Constructs a [MatchTreeStep] from a [`Next`]
    fn from_next(module: &'a FormatModule, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        match next.as_ref() {
            Next::Empty => Self::accept(),
            Next::Union(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone());
                let tree2 = Self::from_next(module, next2.clone());
                tree1.union(tree2)
            }
            Next::Cat(f, next) => Self::from_format(module, f, next.clone()),
            Next::Tuple(fields, next) => Self::from_tuple(module, fields, next.clone()),
            Next::Record(fields, next) => Self::from_record(module, fields, next.clone()),
            Next::Repeat(a, next0) => {
                let tree = Self::from_next(module, next0.clone());
                tree.union(Self::from_format(module, a, next))
            }
            Next::RepeatCount(n, a, next0) => Self::from_repeat_count(module, *n, a, next0.clone()),
            Next::Slice(n, inside, next0) => {
                Self::from_slice(module, *n, inside.clone(), next0.clone())
            }
            Next::Peek(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone());
                let tree2 = Self::from_next(module, next2.clone());
                tree1.peek(tree2)
            }
            Next::PeekNot(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone());
                let tree2 = Self::from_next(module, next2.clone());
                tree1.peek_not(tree2)
            }
        }
    }

    /// Constructs a [MatchTreeStep] from an Format and a trailing [`Next`] value
    pub fn from_format(
        module: &'a FormatModule,
        f: &'a Format,
        next: Rc<Next<'a>>
    ) -> MatchTreeStep<'a> {
        match f {
            Format::ItemVar(level, _args) => {
                Self::from_format(module, module.get_format(*level), next)
            }
            Format::Fail => Self::reject(),
            Format::EndOfInput => Self::accept(),
            Format::Align(_) => {
                Self::accept() // FIXME
            }
            Format::Byte(bs) => Self::branch(*bs, next),
            Format::Variant(_label, f) => Self::from_format(module, f, next.clone()),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone()));
                }
                tree
            }
            Format::Tuple(fields) => Self::from_tuple(module, fields, next),
            Format::Record(fields) => Self::from_record(module, fields, next),
            Format::Repeat(a) => {
                let tree = Self::from_next(module, next.clone());
                tree.union(Self::from_format(module, a, Rc::new(Next::Repeat(a, next.clone()))))
            }
            Format::Repeat1(a) => {
                Self::from_format(module, a, Rc::new(Next::Repeat(a, next.clone())))
            }
            Format::RepeatCount(expr, a) => {
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    Self::from_repeat_count(module, n, a, next.clone())
                } else {
                    Self::from_repeat_count(module, bounds.min, a, Rc::new(Next::Empty))
                }
            }
            Format::RepeatUntilLast(_expr, _a) => {
                Self::accept() // FIXME
            }
            Format::RepeatUntilSeq(_expr, _a) => {
                Self::accept() // FIXME
            }
            Format::Peek(a) => {
                let tree = Self::from_next(module, next.clone());
                let peek = Self::from_format(module, a, Rc::new(Next::Empty));
                tree.peek(peek)
            }
            Format::PeekNot(a) => {
                let tree = Self::from_next(module, next.clone());
                let peek = Self::from_format(module, a, Rc::new(Next::Empty));
                tree.peek_not(peek)
            }
            Format::Slice(expr, f) => {
                let inside = Rc::new(Next::Cat(f, Rc::new(Next::Empty)));
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    Self::from_slice(module, n, inside, next)
                } else {
                    Self::from_slice(module, bounds.min, inside, Rc::new(Next::Empty))
                }
            }
            Format::Bits(_a) => {
                Self::accept() // FIXME
            }
            Format::WithRelativeOffset(expr, a) => {
                // REVIEW - this is a bit hackish but it is at least somewhat better than before
                let tree = Self::from_next(module, next.clone());
                let bounds = expr.bounds();
                match bounds.is_exact() {
                    None => tree, // if the lookahead is indeterminate, ignore it
                    Some(n) => {
                        let peek = match n {
                            0 => Self::from_format(module, a, Rc::new(Next::Empty)),
                            _ =>
                                Self::from_slice(
                                    module,
                                    n,
                                    Rc::new(Next::Empty),
                                    Rc::new(Next::Tuple(std::slice::from_ref(a.as_ref()), next))
                                ),
                        };
                        tree.peek(peek)
                    }
                }
            }
            Format::Map(f, _expr) => Self::from_format(module, f, next),
            Format::Compute(_expr) => Self::from_next(module, next),
            Format::Let(_name, _expr, f) => Self::from_format(module, f, next),
            Format::Match(_, branches) => {
                let mut tree = Self::reject();
                for (_, f) in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone()));
                }
                tree
            }
            Format::Dynamic(_name, _expr, f) => Self::from_format(module, f, next),
            Format::Apply(_name) => Self::accept(),
        }
    }
}

impl<'a> MatchTreeLevel<'a> {
    /// Constructs a `MatchTreeLevel` that unconditionally rejects all inputs without branching.
    fn reject() -> MatchTreeLevel<'a> {
        MatchTreeLevel {
            accept: None,
            branches: vec![],
        }
    }

    /// Attempts to modify `self` such that `index` is marked as the unique index of the accepting format.
    ///
    /// Returns `Err(())` if a different index was already marked as accepting, and `Ok(())` otherwise.
    fn merge_accept(&mut self, index: usize) -> Result<(), ()> {
        match self.accept {
            None => {
                self.accept = Some(index);
                Ok(())
            }
            Some(i) if i == index => Ok(()),
            Some(_) => Err(()),
        }
    }

    /// Adds a new branch to `self` using a predicate byte-set and its associated follow-set,
    fn merge_branch(&mut self, index: usize, mut bs: ByteSet, next: Rc<Next<'a>>) {
        let mut new_branches = Vec::new();
        // For each bs0, nexts in the extant branches of `self`:
        for (bs0, nexts) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            // If bs and bs0 are not disjoint:
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    // 1. Enqueue a branch predicated on `bs0 - bs` with an inherited follow-set
                    new_branches.push((orig, nexts.clone()));
                }
                // 2. Leave behind a branch predicated on `bs0 & bs`
                *bs0 = common;
                // 2a. Add the `next` parameter to the follow-set of the existing branch we modified in-place
                nexts.insert((index, next.clone()));
                // 3. Remove all bytes from `bs` that are now covered by the branch we modified in-place
                bs = bs.difference(bs0);
            }
        }
        // If any bytes of bs were completely unique among all extant branches:
        if !bs.is_empty() {
            // 1. Create a novel branch with the follow-set implied by the `next` parameter
            let mut nexts = HashSet::new();
            nexts.insert((index, next.clone()));
            self.branches.push((bs, nexts));
        }
        // Append all enqueued branches from the iteration above
        self.branches.append(&mut new_branches);
    }

    /// Extends the set of choice-points and follow-sets of `self` with a provided [`MatchTreeStep`].
    fn merge_step(
        mut self,
        index: usize,
        step: MatchTreeStep<'a>
    ) -> Result<MatchTreeLevel<'a>, ()> {
        if step.accept {
            self.merge_accept(index)?;
        }
        for (bs, next) in step.branches {
            self.merge_branch(index, bs, next);
        }
        Ok(self)
    }

    /// Attempt to construct and return a `MatchTree` that unconditionally accepts
    /// the same, common format-index as all elements of the set `nexts`.
    ///
    /// If `nexts` is empty, the `MatchTree` returned will instead reject all input
    ///
    /// If `nexts` contains multiple associated indices, returns `None`
    fn accepts(nexts: &LevelBranch<'a>) -> Option<MatchTree> {
        let mut tree = Self::reject();
        for (i, _next) in nexts.iter() {
            tree.merge_accept(*i).ok()?;
        }
        Some(MatchTree {
            accept: tree.accept,
            branches: vec![],
        })
    }

    /// Attempts to accumulate a `MatchTree` recursively up to an overall depth of `depth` layers,
    /// with the immediate layer constructed based on a bundle of indexed choice-points ([`LevelBranch`]).
    ///
    /// If the depth limit has been reached without a decisive choice of which index to accept, returns None.
    ///
    /// Otherwise, returns a `MatchTree` that is guaranteed to decide on a unique branch for
    /// all input within at most `depth` bytes of lookahead.
    fn grow(module: &'a FormatModule, nexts: LevelBranch<'a>, depth: usize) -> Option<MatchTree> {
        if let Some(tree) = Self::accepts(&nexts) {
            Some(tree)
        } else if depth > 0 {
            let mut tree = Self::reject();
            let mut tmp = Vec::from_iter(nexts.into_iter());
            tmp.sort_by_key(|(ix, _)| *ix);
            for (i, next) in tmp.into_iter() {
                let subtree = MatchTreeStep::from_next(module, next);
                tree = tree.merge_step(i, subtree).ok()?;
            }
            let mut branches = Vec::new();
            for (bs, nexts) in tree.branches {
                let t = Self::grow(module, nexts, depth - 1)?;
                branches.push((bs, t));
            }
            Some(MatchTree {
                accept: tree.accept,
                branches,
            })
        } else {
            None
        }
    }
}

impl MatchTree {
    /// Returns the accepting index associated with the input-sequence starting from the current offset of `input`,
    /// looking ahead as many bytes as necessary until a definitive index is found or the lookahead limit is reached.
    ///
    /// Returns `None` if not enough lookahead remains to disambiguate multiple candidate indices.
    fn matches(&self, input: ReadCtxt<'_>) -> Option<usize> {
        match input.read_byte() {
            None => self.accept,
            Some((b, input)) => {
                for (bs, s) in &self.branches {
                    if bs.contains(b) {
                        return s.matches(input);
                    }
                }
                self.accept
            }
        }
    }

    /// Constructs a new `MatchTreeLevel` from an alternation of branches and a follow-set of partially decomposed formats,
    /// to within a fixed but externally opaque lookahead-depth.
    ///
    /// A `FormatModule` is also accepted to contextualize any contextually dependent formats, e.g. [`Format::ItemVar`]
    fn build(module: &FormatModule, branches: &[Format], next: Rc<Next<'_>>) -> Option<MatchTree> {
        let mut nexts = HashSet::new();
        for (i, f) in branches.iter().enumerate() {
            nexts.insert((i, Rc::new(Next::Cat(f, next.clone()))));
        }
        const MAX_DEPTH: usize = 32;
        MatchTreeLevel::grow(module, nexts, MAX_DEPTH)
    }
}

pub struct TypeScope<'a> {
    parent: Option<&'a TypeScope<'a>>,
    names: Vec<Label>,
    types: Vec<ValueKind>,
}

impl<'a> TypeScope<'a> {
    fn new() -> Self {
        let parent = None;
        let names = Vec::new();
        let types = Vec::new();
        TypeScope {
            parent,
            names,
            types,
        }
    }

    fn child(parent: &'a TypeScope<'a>) -> Self {
        let parent = Some(parent);
        let names = Vec::new();
        let types = Vec::new();
        TypeScope {
            parent,
            names,
            types,
        }
    }

    fn push(&mut self, name: Label, t: ValueType) {
        self.names.push(name);
        self.types.push(ValueKind::Value(t));
    }

    fn push_format(&mut self, name: Label, t: ValueType) {
        self.names.push(name);
        self.types.push(ValueKind::Format(t));
    }

    fn get_type_by_name(&self, name: &str) -> &ValueKind {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return &self.types[i];
            }
        }
        if let Some(scope) = self.parent {
            scope.get_type_by_name(name)
        } else {
            panic!("variable not found: {name}");
        }
    }
}
