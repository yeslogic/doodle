#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use serde::Serialize;

use crate::byte_set::ByteSet;

pub mod byte_set;
pub mod output;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Pattern {
    Binding,
    Wildcard,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Pattern>),
    Variant(String, Box<Pattern>),
    Seq(Vec<Pattern>),
}

impl Pattern {
    pub const UNIT: Pattern = Pattern::Tuple(Vec::new());

    pub fn from_bytes(bs: &[u8]) -> Pattern {
        Pattern::Seq(bs.iter().copied().map(Pattern::U8).collect())
    }

    pub fn variant(label: impl Into<String>, value: impl Into<Box<Pattern>>) -> Pattern {
        Pattern::Variant(label.into(), value.into())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),
    Variant(String, Box<Value>),
    Seq(Vec<Value>),
}

impl Value {
    pub const UNIT: Value = Value::Tuple(Vec::new());

    fn record<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Value)>) -> Value {
        Value::Record(
            fields
                .into_iter()
                .map(|(label, value)| (label.into(), value))
                .collect(),
        )
    }

    fn variant(label: impl Into<String>, value: impl Into<Box<Value>>) -> Value {
        Value::Variant(label.into(), value.into())
    }

    fn record_proj(&self, label: &str) -> Value {
        match self {
            Value::Record(fields) => match fields.iter().find(|(l, _)| label == l) {
                Some((_, v)) => v.clone(),
                None => panic!("{label} not found in record"),
            },
            _ => panic!("expected record"),
        }
    }

    /// Returns `true` if the pattern successfully matches the value, pushing
    /// any values bound by the pattern onto the stack
    fn matches(&self, stack: &mut Stack, pattern: &Pattern) -> bool {
        match (pattern, &self.coerce_record_to_value()) {
            (Pattern::Binding, head) => {
                stack.push(head.clone());
                true
            }
            (Pattern::Wildcard, _) => true,
            (Pattern::Bool(b0), Value::Bool(b1)) => b0 == b1,
            (Pattern::U8(i0), Value::U8(i1)) => i0 == i1,
            (Pattern::U16(i0), Value::U16(i1)) => i0 == i1,
            (Pattern::U32(i0), Value::U32(i1)) => i0 == i1,
            (Pattern::Tuple(ps), Value::Tuple(vs)) | (Pattern::Seq(ps), Value::Seq(vs))
                if ps.len() == vs.len() =>
            {
                let initial_len = stack.len();
                for (p, v) in Iterator::zip(ps.iter(), vs.iter()) {
                    if !v.matches(stack, p) {
                        stack.truncate(initial_len);
                        return false;
                    }
                }
                true
            }
            (Pattern::Variant(label0, p), Value::Variant(label1, v)) if label0 == label1 => {
                v.matches(stack, p)
            }
            _ => false,
        }
    }

    fn coerce_record_to_value(&self) -> Value {
        match self {
            Value::Record(fields) => {
                if let Some((_l, v)) = fields.iter().find(|(l, _)| l == "@value") {
                    return v.clone();
                } else {
                    Value::Record(fields.clone())
                }
            }
            v => v.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Expr {
    Var(usize),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Expr>),
    TupleProj(Box<Expr>, usize),
    Record(Vec<(String, Expr)>),
    RecordProj(Box<Expr>, String),
    Variant(String, Box<Expr>),
    UnwrapVariant(Box<Expr>),
    Seq(Vec<Expr>),
    Match(Box<Expr>, Vec<(Pattern, Expr)>),

    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lte(Box<Expr>, Box<Expr>),
    Gte(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),

    AsU16(Box<Expr>),
    AsU32(Box<Expr>),

    U16Be(Box<Expr>),
    U16Le(Box<Expr>),
    U32Be(Box<Expr>),
    U32Le(Box<Expr>),

    SeqLength(Box<Expr>),
    SubSeq(Box<Expr>, Box<Expr>, Box<Expr>),
    FlatMap(Box<Expr>, Box<Expr>),
    FlatMapAccum(Box<Expr>, Box<Expr>, Box<Expr>),
    Dup(Box<Expr>, Box<Expr>),
    Inflate(Box<Expr>),
}

impl Expr {
    pub const UNIT: Expr = Expr::Tuple(Vec::new());

    pub fn record_proj(head: impl Into<Box<Expr>>, label: impl Into<String>) -> Expr {
        Expr::RecordProj(head.into(), label.into())
    }
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
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Format {
    /// Reference to a top-level item
    ItemVar(usize, Vec<Expr>),
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Skips bytes if necessary to align the current offset to a multiple of N
    Align(usize),
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Matches the union of the byte strings matched by all the formats
    Union(Vec<(String, Format)>),
    /// Matches a sequence of concatenated formats
    Tuple(Vec<Format>),
    /// Matches a sequence of named formats where later formats can depend on
    /// the decoded value of earlier formats
    Record(Vec<(String, Format)>),
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
    /// Restrict a format to a sub-stream of a given number of bytes
    Slice(Expr, Box<Format>),
    /// Parse bitstream
    Bits(Box<Format>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(Expr, Box<Format>),
    /// Compute a value
    Compute(Expr),
    /// Pattern match on an expression
    Match(Expr, Vec<(Pattern, Format)>),
    /// Format generated dynamically
    Dynamic(DynFormat),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum DynFormat {
    Huffman(Expr, Option<Expr>),
}

impl Format {
    pub const EMPTY: Format = Format::Tuple(Vec::new());

    pub fn alts<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
        Format::Union(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }

    pub fn record<Label: Into<String>>(
        fields: impl IntoIterator<Item = (Label, Format)>,
    ) -> Format {
        Format::Record(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }
}

#[derive(Copy, Clone)]
pub struct FormatRef(usize);

impl FormatRef {
    pub fn call(&self) -> Format {
        Format::ItemVar(self.0, vec![])
    }

    pub fn call_args(&self, args: Vec<Expr>) -> Format {
        Format::ItemVar(self.0, args)
    }
}

#[derive(Debug, Serialize)]
pub struct FormatModule {
    names: Vec<String>,
    formats: Vec<Format>,
}

impl FormatModule {
    pub fn new() -> FormatModule {
        FormatModule {
            names: Vec::new(),
            formats: Vec::new(),
        }
    }

    pub fn define_format(&mut self, name: impl Into<String>, format: Format) -> FormatRef {
        let level = self.names.len();
        self.names.push(name.into());
        self.formats.push(format);
        FormatRef(level)
    }

    fn get_name(&self, level: usize) -> &str {
        &self.names[level]
    }

    fn get_format(&self, level: usize) -> &Format {
        &self.formats[level]
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Next<'a> {
    Empty,
    Cat(&'a Format, Rc<Next<'a>>),
    Tuple(&'a [Format], Rc<Next<'a>>),
    Record(&'a [(String, Format)], Rc<Next<'a>>),
    Repeat(&'a Format, Rc<Next<'a>>),
}

#[derive(Clone, Debug)]
struct Nexts<'a> {
    set: HashSet<(usize, Rc<Next<'a>>)>,
}

#[derive(Clone, Debug)]
struct MatchTreeLevel<'a> {
    accept: Option<usize>,
    branches: Vec<(ByteSet, Nexts<'a>)>,
}

#[derive(Clone, Debug)]
pub struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
}

/// Decoders with a fixed amount of lookahead
enum Decoder {
    Call(usize, Vec<Expr>),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Branch(MatchTree, Vec<(String, Decoder)>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(MatchTree, Box<Decoder>),
    Until(MatchTree, Box<Decoder>),
    RepeatCount(Expr, Box<Decoder>),
    RepeatUntilLast(Expr, Box<Decoder>),
    RepeatUntilSeq(Expr, Box<Decoder>),
    Peek(Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    Bits(Box<Decoder>),
    WithRelativeOffset(Expr, Box<Decoder>),
    Compute(Expr),
    Match(Expr, Vec<(Pattern, Decoder)>),
    Dynamic(DynFormat),
}

impl Expr {
    fn eval(&self, stack: &mut Stack) -> Value {
        match self {
            Expr::Var(index) => stack.get(*index).clone(),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::U8(i) => Value::U8(*i),
            Expr::U16(i) => Value::U16(*i),
            Expr::U32(i) => Value::U32(*i),
            Expr::Tuple(exprs) => Value::Tuple(exprs.iter().map(|expr| expr.eval(stack)).collect()),
            Expr::TupleProj(head, index) => match head.eval_value(stack) {
                Value::Tuple(vs) => vs[*index].clone(),
                _ => panic!("expected tuple"),
            },
            Expr::Record(fields) => {
                Value::record(fields.iter().map(|(label, expr)| (label, expr.eval(stack))))
            }
            Expr::RecordProj(head, label) => head.eval(stack).record_proj(label),
            Expr::Variant(label, expr) => Value::variant(label, expr.eval(stack)),
            Expr::UnwrapVariant(expr) => match expr.eval_value(stack) {
                Value::Variant(_label, value) => *value.clone(),
                _ => panic!("expected variant"),
            },
            Expr::Seq(exprs) => Value::Seq(exprs.iter().map(|expr| expr.eval(stack)).collect()),
            Expr::Match(head, branches) => {
                let head = head.eval(stack);
                let initial_len = stack.len();
                let (_, expr) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(stack, pattern))
                    .expect("exhaustive patterns");
                let value = expr.eval(stack);
                stack.truncate(initial_len);
                value
            }

            Expr::BitAnd(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(x & y),
                (Value::U16(x), Value::U16(y)) => Value::U16(x & y),
                (Value::U32(x), Value::U32(y)) => Value::U32(x & y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::BitOr(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(x | y),
                (Value::U16(x), Value::U16(y)) => Value::U16(x | y),
                (Value::U32(x), Value::U32(y)) => Value::U32(x | y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Eq(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x == y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x == y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x == y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Ne(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x != y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x != y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x != y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Lt(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x < y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x < y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x < y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Gt(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x > y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x > y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x > y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Lte(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x <= y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x <= y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x <= y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Gte(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x >= y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x >= y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x >= y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Rem(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_rem(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_rem(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_rem(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            #[rustfmt::skip]
            Expr::Shl(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_shl(x, u32::from(y)).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_shl(x, u32::from(y)).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shl(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Add(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_add(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_add(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_add(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Sub(x, y) => match (x.eval_value(stack), y.eval_value(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_sub(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_sub(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_sub(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },

            Expr::AsU16(x) => match x.eval_value(stack) {
                Value::U8(x) => Value::U16(u16::from(x)),
                Value::U16(x) => Value::U16(x),
                x => panic!("cannot convert {x:?} to U16"),
            },
            Expr::AsU32(x) => match x.eval_value(stack) {
                Value::U8(x) => Value::U32(u32::from(x)),
                Value::U16(x) => Value::U32(u32::from(x)),
                Value::U32(x) => Value::U32(x),
                x => panic!("cannot convert {x:?} to U32"),
            },

            Expr::U16Be(bytes) => match bytes.eval_tuple(stack).as_slice() {
                [Value::U8(hi), Value::U8(lo)] => Value::U16(u16::from_be_bytes([*hi, *lo])),
                _ => panic!("U16Be: expected (U8, U8)"),
            },
            Expr::U16Le(bytes) => match bytes.eval_tuple(stack).as_slice() {
                [Value::U8(lo), Value::U8(hi)] => Value::U16(u16::from_le_bytes([*lo, *hi])),
                _ => panic!("U16Le: expected (U8, U8)"),
            },
            Expr::U32Be(bytes) => match bytes.eval_tuple(stack).as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                    Value::U32(u32::from_be_bytes([*a, *b, *c, *d]))
                }
                _ => panic!("U32Be: expected (U8, U8, U8, U8)"),
            },
            Expr::U32Le(bytes) => match bytes.eval_tuple(stack).as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                    Value::U32(u32::from_le_bytes([*a, *b, *c, *d]))
                }
                _ => panic!("U32Le: expected (U8, U8, U8, U8)"),
            },
            Expr::SeqLength(seq) => match seq.eval(stack) {
                Value::Seq(values) => {
                    let len = values.len();
                    if len < 65536 {
                        Value::U16(len as u16)
                    } else {
                        Value::U32(len as u32)
                    }
                }
                _ => panic!("SeqLength: expected Seq"),
            },
            Expr::SubSeq(seq, start, length) => match seq.eval(stack) {
                Value::Seq(values) => {
                    let start = start.eval_usize(stack);
                    let length = length.eval_usize(stack);
                    let values = &values[start..];
                    let values = &values[..length];
                    Value::Seq(values.to_vec())
                }
                _ => panic!("SubSeq: expected Seq"),
            },
            Expr::FlatMap(expr, seq) => match seq.eval(stack) {
                Value::Seq(values) => {
                    let mut vs = Vec::new();
                    for v in values {
                        stack.push(v);
                        if let Value::Seq(vn) = expr.eval(stack) {
                            vs.extend(vn);
                        } else {
                            panic!("FlatMap: expected Seq");
                        }
                        stack.pop();
                    }
                    Value::Seq(vs)
                }
                _ => panic!("FlatMap: expected Seq"),
            },
            Expr::FlatMapAccum(expr, accum, seq) => match seq.eval(stack) {
                Value::Seq(values) => {
                    let mut accum = accum.eval(stack);
                    let mut vs = Vec::new();
                    for v in values {
                        stack.push(Value::Tuple(vec![accum, v]));
                        accum = match expr.eval_tuple(stack).as_mut_slice() {
                            [accum, Value::Seq(vn)] => {
                                vs.extend_from_slice(&vn);
                                accum.clone()
                            }
                            _ => panic!("FlatMapAccum: expected two values"),
                        };
                        stack.pop();
                    }
                    Value::Seq(vs)
                }
                _ => panic!("FlatMapAccum: expected Seq"),
            },
            Expr::Dup(count, expr) => {
                let count = count.eval_usize(stack);
                let v = expr.eval(stack);
                let mut vs = Vec::new();
                for _ in 0..count {
                    vs.push(v.clone());
                }
                Value::Seq(vs)
            }
            Expr::Inflate(seq) => match seq.eval(stack) {
                Value::Seq(values) => {
                    let vs = inflate(&values);
                    Value::Seq(vs)
                }
                _ => panic!("Inflate: expected Seq"),
            },
        }
    }

    fn eval_value(&self, stack: &mut Stack) -> Value {
        self.eval(stack).coerce_record_to_value()
    }

    fn eval_bool(&self, stack: &mut Stack) -> bool {
        match self.eval_value(stack) {
            Value::Bool(b) => b,
            _ => panic!("value is not a bool"),
        }
    }

    fn eval_usize(&self, stack: &mut Stack) -> usize {
        match self.eval_value(stack) {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::U32(n) => usize::try_from(n).unwrap(),
            _ => panic!("value is not a number"),
        }
    }

    fn eval_tuple(&self, stack: &mut Stack) -> Vec<Value> {
        match self.eval_value(stack) {
            Value::Tuple(values) => values,
            _ => panic!("value is not a tuple"),
        }
    }
}

impl Format {
    /// Returns `true` if the format could match the empty byte string
    fn is_nullable(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).is_nullable(module),
            Format::Fail => false,
            Format::EndOfInput => true,
            Format::Align(_) => true,
            Format::Byte(_) => false,
            Format::Union(branches) => branches.iter().any(|(_, f)| f.is_nullable(module)),
            Format::Tuple(fields) => fields.iter().all(|f| f.is_nullable(module)),
            Format::Record(fields) => fields.iter().all(|(_, f)| f.is_nullable(module)),
            Format::Repeat(_) => true,
            Format::Repeat1(_) => false,
            Format::RepeatCount(_, _) => true,
            Format::RepeatUntilLast(_, f) => f.is_nullable(module),
            Format::RepeatUntilSeq(_, f) => f.is_nullable(module),
            Format::Peek(_) => true,
            Format::Slice(_, _) => true,
            Format::Bits(f) => f.is_nullable(module),
            Format::WithRelativeOffset(_, _) => true,
            Format::Compute(_) => true,
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.is_nullable(module)),
            Format::Dynamic(DynFormat::Huffman(_, _)) => false,
        }
    }

    /// True if the compilation of this format depends on the format that follows it
    fn depends_on_next(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).depends_on_next(module),
            Format::Fail => false,
            Format::EndOfInput => false,
            Format::Align(_) => false,
            Format::Byte(_) => false,
            Format::Union(branches) => Format::union_depends_on_next(&branches, module),
            Format::Tuple(fields) => fields.iter().any(|f| f.depends_on_next(module)),
            Format::Record(fields) => fields.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Repeat(_) => true,
            Format::Repeat1(_) => true,
            Format::RepeatCount(_, _f) => false,
            Format::RepeatUntilLast(_, _f) => false,
            Format::RepeatUntilSeq(_, _f) => false,
            Format::Peek(_) => false,
            Format::Slice(_, _) => false,
            Format::Bits(_) => false,
            Format::WithRelativeOffset(_, _) => false,
            Format::Compute(_) => false,
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Dynamic(_) => false,
        }
    }

    fn union_depends_on_next(branches: &[(String, Format)], module: &FormatModule) -> bool {
        let mut fs = Vec::with_capacity(branches.len());
        for (_label, f) in branches {
            if f.depends_on_next(module) {
                return true;
            }
            fs.push(f.clone());
        }
        MatchTree::build(module, &fs, Rc::new(Next::Empty)).is_none()
    }
}

impl<'a> Nexts<'a> {
    fn new() -> Self {
        Nexts {
            set: HashSet::new(),
        }
    }

    fn add(&mut self, index: usize, next: Rc<Next<'a>>) -> Result<(), ()> {
        self.set.insert((index, next));
        Ok(())
    }
}

impl<'a> MatchTreeLevel<'a> {
    fn reject() -> MatchTreeLevel<'a> {
        MatchTreeLevel {
            accept: None,
            branches: vec![],
        }
    }

    fn accept(&mut self, index: usize) -> Result<(), ()> {
        match self.accept {
            None => {
                self.accept = Some(index);
                Ok(())
            }
            Some(i) if i == index => Ok(()),
            Some(_) => Err(()),
        }
    }

    fn add_next(
        &mut self,
        module: &'a FormatModule,
        index: usize,
        next: Rc<Next<'a>>,
    ) -> Result<(), ()> {
        match next.as_ref() {
            Next::Empty => self.accept(index),
            Next::Cat(f, next) => self.add(module, index, f, next.clone()),
            Next::Tuple(fs, next) => match fs.split_first() {
                None => self.add_next(module, index, next.clone()),
                Some((f, fs)) => self.add(module, index, f, Rc::new(Next::Tuple(fs, next.clone()))),
            },
            Next::Record(fs, next) => match fs.split_first() {
                None => self.add_next(module, index, next.clone()),
                Some(((_n, f), fs)) => {
                    self.add(module, index, f, Rc::new(Next::Record(fs, next.clone())))
                }
            },
            Next::Repeat(a, next0) => {
                self.add_next(module, index, next0.clone())?;
                self.add(module, index, a, next)?;
                Ok(())
            }
        }
    }

    pub fn add(
        &mut self,
        module: &'a FormatModule,
        index: usize,
        f: &'a Format,
        next: Rc<Next<'a>>,
    ) -> Result<(), ()> {
        match f {
            Format::ItemVar(level, _args) => {
                self.add(module, index, module.get_format(*level), next)
            }
            Format::Fail => Ok(()),
            Format::EndOfInput => self.accept(index),
            Format::Align(_) => {
                self.accept(index) // FIXME
            }
            Format::Byte(bs) => {
                let mut bs = *bs;
                let mut new_branches = Vec::new();
                for (bs0, nexts) in self.branches.iter_mut() {
                    let common = bs0.intersection(&bs);
                    if !common.is_empty() {
                        let orig = bs0.difference(&bs);
                        if !orig.is_empty() {
                            new_branches.push((orig, nexts.clone()));
                        }
                        *bs0 = common;
                        nexts.add(index, next.clone())?;
                        bs = bs.difference(bs0);
                    }
                }
                if !bs.is_empty() {
                    let mut nexts = Nexts::new();
                    nexts.add(index, next.clone())?;
                    self.branches.push((bs, nexts));
                }
                self.branches.append(&mut new_branches);
                Ok(())
            }
            Format::Union(branches) => {
                for (_, f) in branches {
                    self.add(module, index, f, next.clone())?;
                }
                Ok(())
            }
            Format::Tuple(fields) => match fields.split_first() {
                None => self.add_next(module, index, next),
                Some((a, fields)) => {
                    self.add(module, index, a, Rc::new(Next::Tuple(fields, next.clone())))
                }
            },
            Format::Record(fields) => match fields.split_first() {
                None => self.add_next(module, index, next),
                Some(((_, a), fields)) => {
                    let next = Rc::new(Next::Record(fields, next.clone()));
                    self.add(module, index, a, next)
                }
            },
            Format::Repeat(a) => {
                self.add_next(module, index, next.clone())?;
                self.add(module, index, a, Rc::new(Next::Repeat(a, next.clone())))?;
                Ok(())
            }
            Format::Repeat1(a) => {
                self.add(module, index, a, Rc::new(Next::Repeat(a, next.clone())))?;
                Ok(())
            }
            Format::RepeatCount(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::RepeatUntilLast(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::RepeatUntilSeq(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::Peek(_a) => {
                self.accept(index) // FIXME
            }
            Format::Slice(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::Bits(_a) => {
                self.accept(index) // FIXME
            }
            Format::WithRelativeOffset(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::Compute(_expr) => self.add_next(module, index, next),
            Format::Match(_, branches) => {
                for (_, f) in branches {
                    self.add(module, index, f, next.clone())?;
                }
                Ok(())
            }
            Format::Dynamic(DynFormat::Huffman(_, _)) => {
                self.accept(index) // FIXME
            }
        }
    }

    fn accepts(nexts: &Nexts<'a>) -> Option<MatchTree> {
        let mut tree = MatchTreeLevel::reject();
        for (i, _next) in nexts.set.iter() {
            tree.accept(*i).ok()?;
        }
        Some(MatchTree {
            accept: tree.accept,
            branches: vec![],
        })
    }

    fn grow(module: &FormatModule, mut nexts: Nexts<'a>, depth: usize) -> Option<MatchTree> {
        if let Some(tree) = MatchTreeLevel::accepts(&nexts) {
            Some(tree)
        } else if depth > 0 {
            let mut tree = MatchTreeLevel::reject();
            for (i, next) in nexts.set.drain() {
                tree.add_next(module, i, next).ok()?;
            }
            let mut branches = Vec::new();
            for (bs, nexts) in tree.branches {
                let t = MatchTreeLevel::grow(module, nexts, depth - 1)?;
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
    fn matches<'a>(&self, input: ReadCtxt<'a>) -> Option<usize> {
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

    fn build(module: &FormatModule, branches: &[Format], next: Rc<Next<'_>>) -> Option<MatchTree> {
        let mut nexts = Nexts::new();
        for (i, f) in branches.iter().enumerate() {
            nexts.add(i, Rc::new(Next::Cat(f, next.clone()))).ok()?;
        }
        const MAX_DEPTH: usize = 32;
        MatchTreeLevel::grow(module, nexts, MAX_DEPTH)
    }
}

#[derive(Copy, Clone)]
pub struct ReadCtxt<'a> {
    input: &'a [u8],
    pub offset: usize,
}

impl<'a> ReadCtxt<'a> {
    pub fn new(input: &'a [u8]) -> ReadCtxt<'a> {
        let offset = 0;
        ReadCtxt { input, offset }
    }

    pub fn remaining(&self) -> &'a [u8] {
        &self.input[self.offset..]
    }

    pub fn read_byte(&self) -> Option<(u8, ReadCtxt<'a>)> {
        if self.offset < self.input.len() {
            let b = self.input[self.offset];
            Some((
                b,
                ReadCtxt {
                    input: self.input,
                    offset: self.offset + 1,
                },
            ))
        } else {
            None
        }
    }

    pub fn split_at(&self, n: usize) -> Option<(ReadCtxt<'a>, ReadCtxt<'a>)> {
        if self.offset + n <= self.input.len() {
            let fst = ReadCtxt {
                input: &self.input[..self.offset + n],
                offset: self.offset,
            };
            let snd = ReadCtxt {
                input: self.input,
                offset: self.offset + n,
            };
            Some((fst, snd))
        } else {
            None
        }
    }
}

pub struct Program {
    decoders: Vec<Decoder>,
}

impl Program {
    fn new() -> Self {
        let decoders = Vec::new();
        Program { decoders }
    }

    pub fn run<'input>(&self, input: ReadCtxt<'input>) -> Option<(Value, ReadCtxt<'input>)> {
        let mut stack = Stack::new();
        self.decoders[0].parse(self, &mut stack, input)
    }
}

pub struct Compiler<'a> {
    module: &'a FormatModule,
    program: Program,
    map: HashMap<(usize, Rc<Next<'a>>), usize>,
}

impl<'a> Compiler<'a> {
    fn new(module: &'a FormatModule) -> Self {
        let program = Program::new();
        let map = HashMap::new();
        Compiler {
            module,
            program,
            map,
        }
    }

    pub fn compile(module: &FormatModule, format: &Format) -> Result<Program, String> {
        let mut compiler = Compiler::new(module);
        let n = compiler.program.decoders.len();
        compiler.program.decoders.push(Decoder::Fail);
        let d = Decoder::compile(&mut compiler, format)?;
        compiler.program.decoders[n] = d;
        Ok(compiler.program)
    }
}

pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    fn new() -> Self {
        let values = Vec::new();
        Stack { values }
    }

    fn push(&mut self, v: Value) {
        self.values.push(v);
    }

    fn pop(&mut self) -> Value {
        self.values.pop().unwrap()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn truncate(&mut self, len: usize) {
        self.values.truncate(len);
    }

    fn get(&self, index: usize) -> &Value {
        &self.values[self.values.len() - index - 1]
    }
}

impl Decoder {
    pub fn compile_one(format: &Format) -> Result<Decoder, String> {
        let module = FormatModule::new();
        let mut compiler = Compiler::new(&module);
        Decoder::compile(&mut compiler, format)
    }

    pub fn compile<'a>(compiler: &mut Compiler<'a>, format: &'a Format) -> Result<Decoder, String> {
        Decoder::compile_next(compiler, format, Rc::new(Next::Empty))
    }

    fn compile_next<'a>(
        compiler: &mut Compiler<'a>,
        format: &'a Format,
        next: Rc<Next<'a>>,
    ) -> Result<Decoder, String> {
        match format {
            Format::ItemVar(level, args) => {
                let next = if compiler
                    .module
                    .get_format(*level)
                    .depends_on_next(compiler.module)
                {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = compiler.map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let d = Decoder::compile_next(
                        compiler,
                        compiler.module.get_format(*level),
                        next.clone(),
                    )?;
                    let n = compiler.program.decoders.len();
                    compiler.program.decoders.push(d);
                    compiler.map.insert((*level, next.clone()), n);
                    n
                };
                Ok(Decoder::Call(n, args.clone()))
            }
            Format::Fail => Ok(Decoder::Fail),
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Align(n) => Ok(Decoder::Align(*n)),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::Union(branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut ds = Vec::with_capacity(branches.len());
                for (label, f) in branches {
                    ds.push((
                        label.clone(),
                        Decoder::compile_next(compiler, f, next.clone())?,
                    ));
                    fs.push(f.clone());
                }
                if let Some(tree) = MatchTree::build(compiler.module, &fs, next) {
                    Ok(Decoder::Branch(tree, ds))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::Tuple(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Tuple(fields.as_slice(), next.clone()));
                    let df = Decoder::compile_next(compiler, f, next)?;
                    dfields.push(df);
                }
                Ok(Decoder::Tuple(dfields))
            }
            Format::Record(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some((name, f)) = fields.next() {
                    let next = Rc::new(Next::Record(fields.as_slice(), next.clone()));
                    let df = Decoder::compile_next(compiler, f, next)?;
                    dfields.push((name.clone(), df));
                }
                Ok(Decoder::Record(dfields))
            }
            Format::Repeat(a) => {
                if a.is_nullable(compiler.module) {
                    return Err("cannot repeat nullable format".to_string());
                }
                let da =
                    Decoder::compile_next(compiler, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                if let Some(tree) = MatchTree::build(compiler.module, &[fa, fb], next) {
                    Ok(Decoder::While(tree, Box::new(da)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::Repeat1(a) => {
                if a.is_nullable(compiler.module) {
                    return Err("cannot repeat nullable format".to_string());
                }
                let da =
                    Decoder::compile_next(compiler, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::EMPTY;
                let fb = Format::Tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build(compiler.module, &[fa, fb], next) {
                    Ok(Decoder::Until(tree, Box::new(da)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::RepeatCount(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(compiler, a, next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
            }
            Format::RepeatUntilLast(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(compiler, a, next)?);
                Ok(Decoder::RepeatUntilLast(expr.clone(), da))
            }
            Format::RepeatUntilSeq(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(compiler, a, next)?);
                Ok(Decoder::RepeatUntilSeq(expr.clone(), da))
            }
            Format::Peek(a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::Peek(da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::Bits(a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::Bits(da))
            }
            Format::WithRelativeOffset(expr, a) => {
                let da = Box::new(Decoder::compile_next(compiler, a, Rc::new(Next::Empty))?);
                Ok(Decoder::WithRelativeOffset(expr.clone(), da))
            }
            Format::Compute(expr) => Ok(Decoder::Compute(expr.clone())),
            Format::Match(head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| {
                        Ok((
                            pattern.clone(),
                            Decoder::compile_next(compiler, f, next.clone())?,
                        ))
                    })
                    .collect::<Result<_, String>>()?;
                Ok(Decoder::Match(head.clone(), branches))
            }
            Format::Dynamic(d) => Ok(Decoder::Dynamic(d.clone())),
        }
    }

    pub fn parse<'input>(
        &self,
        program: &Program,
        stack: &mut Stack,
        input: ReadCtxt<'input>,
    ) -> Option<(Value, ReadCtxt<'input>)> {
        match self {
            Decoder::Call(n, es) => {
                let mut new_stack = Stack::new();
                for e in es {
                    let v = e.eval(stack);
                    new_stack.push(v);
                }
                program.decoders[*n].parse(program, &mut new_stack, input)
            }
            Decoder::Fail => None,
            Decoder::EndOfInput => match input.read_byte() {
                None => Some((Value::UNIT, input)),
                Some(_) => None,
            },
            Decoder::Align(n) => {
                let skip = (n - input.offset % n) % n;
                let (_, input) = input.split_at(skip)?;
                Some((Value::UNIT, input))
            }
            Decoder::Byte(bs) => {
                let (b, input) = input.read_byte()?;
                if bs.contains(b) {
                    Some((Value::U8(b), input))
                } else {
                    None
                }
            }
            Decoder::Branch(tree, branches) => {
                let index = tree.matches(input)?;
                let (label, d) = &branches[index];
                let (v, input) = d.parse(program, stack, input)?;
                Some((Value::Variant(label.clone(), Box::new(v)), input))
            }
            Decoder::Tuple(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for f in fields {
                    let (vf, next_input) = f.parse(program, stack, input)?;
                    input = next_input;
                    v.push(vf.clone());
                }
                Some((Value::Tuple(v), input))
            }
            Decoder::Record(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for (name, f) in fields {
                    let (vf, next_input) = f.parse(program, stack, input)?;
                    input = next_input;
                    v.push((name.clone(), vf.clone()));
                    stack.push(vf);
                }
                for _ in fields {
                    stack.pop();
                }
                Some((Value::Record(v), input))
            }
            Decoder::While(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while tree.matches(input)? == 0 {
                    let (va, next_input) = a.parse(program, stack, input)?;
                    input = next_input;
                    v.push(va);
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Until(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, stack, input)?;
                    input = next_input;
                    v.push(va);
                    if tree.matches(input)? == 0 {
                        break;
                    }
                }
                Some((Value::Seq(v), input))
            }
            Decoder::RepeatCount(expr, a) => {
                let mut input = input;
                let count = expr.eval_usize(stack);
                let mut v = Vec::with_capacity(count);
                for _ in 0..count {
                    let (va, next_input) = a.parse(program, stack, input)?;
                    input = next_input;
                    v.push(va);
                }
                Some((Value::Seq(v), input))
            }
            Decoder::RepeatUntilLast(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, stack, input)?;
                    input = next_input;
                    stack.push(va);
                    let done = expr.eval_bool(stack);
                    let va = stack.pop();
                    v.push(va);
                    if done {
                        break;
                    }
                }
                Some((Value::Seq(v), input))
            }
            Decoder::RepeatUntilSeq(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, stack, input)?;
                    input = next_input;
                    v.push(va);
                    stack.push(Value::Seq(v));
                    let done = expr.eval_bool(stack);
                    v = if let Value::Seq(v) = stack.pop() {
                        v
                    } else {
                        panic!("expected Seq")
                    };
                    if done {
                        break;
                    }
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Peek(a) => {
                let (v, _next_input) = a.parse(program, stack, input)?;
                Some((v, input))
            }
            Decoder::Slice(expr, a) => {
                let size = expr.eval_usize(stack);
                let (slice, input) = input.split_at(size)?;
                let (v, _) = a.parse(program, stack, slice)?;
                Some((v, input))
            }
            Decoder::Bits(a) => {
                let mut bits = Vec::with_capacity(input.remaining().len() * 8);
                for b in input.remaining() {
                    for i in 0..8 {
                        bits.push((b & (1 << i)) >> i);
                    }
                }
                let (v, bits) = a.parse(program, stack, ReadCtxt::new(&bits))?;
                let bytes_remain = bits.remaining().len() >> 3;
                let bytes_read = input.remaining().len() - bytes_remain;
                let (_, input) = input.split_at(bytes_read)?;
                Some((v, input))
            }
            Decoder::WithRelativeOffset(expr, a) => {
                let offset = expr.eval_usize(stack);
                let (_, slice) = input.split_at(offset)?;
                let (v, _) = a.parse(program, stack, slice)?;
                Some((v, input))
            }
            Decoder::Compute(expr) => {
                let v = expr.eval(stack);
                Some((v, input))
            }
            Decoder::Match(head, branches) => {
                let head = head.eval(stack);
                let initial_len = stack.len();
                let (_, decoder) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(stack, pattern))
                    .expect("exhaustive patterns");
                let value = decoder.parse(program, stack, input);
                stack.truncate(initial_len);
                value
            }
            Decoder::Dynamic(DynFormat::Huffman(lengths_expr, opt_values_expr)) => {
                let lengths_val = lengths_expr.eval(stack);
                let lengths = value_to_vec_usize(&lengths_val);
                let lengths = match opt_values_expr {
                    None => lengths,
                    Some(e) => {
                        let values = value_to_vec_usize(&e.eval(stack));
                        let mut new_lengths = [0].repeat(values.len());
                        for i in 0..lengths.len() {
                            new_lengths[values[i]] = lengths[i];
                        }
                        new_lengths
                    }
                };
                let f = make_huffman_codes(&lengths);
                let d = Decoder::compile_one(&f).unwrap();
                d.parse(program, stack, input)
            }
        }
    }
}

fn value_to_vec_usize(v: &Value) -> Vec<usize> {
    let vs = match v {
        Value::Seq(vs) => vs,
        _ => panic!("expected Seq"),
    };
    vs.iter()
        .map(|v| match v.coerce_record_to_value() {
            Value::U8(n) => n as usize,
            Value::U16(n) => n as usize,
            _ => panic!("expected U8 or U16"),
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

    for n in 0..lengths.len() {
        let len = lengths[n];
        if len != 0 {
            codes.push((n.to_string(), bit_range(len, next_code[len], n)));
            //println!("{:?}", codes[codes.len()-1]);
            next_code[len] += 1;
        } else {
            //codes.push((n.to_string(), Format::Fail));
        }
    }

    Format::alts(codes)
}

fn bit_range(n: usize, bits: usize, val: usize) -> Format {
    let mut fs = Vec::with_capacity(n);
    for i in 0..n {
        let r = n - 1 - i;
        let b = (bits & (1 << r)) >> r != 0;
        fs.push(is_bit(b));
    }
    Format::record([
        ("bits", Format::Tuple(fs)),
        (
            "@value",
            Format::Compute(if val > 255 {
                Expr::U16(val.try_into().unwrap())
            } else {
                Expr::U8(val.try_into().unwrap())
            }),
        ),
    ])
}

fn is_bit(b: bool) -> Format {
    Format::Byte(ByteSet::from([if b { 1 } else { 0 }]))
}

fn inflate(codes: &[Value]) -> Vec<Value> {
    let mut vs = Vec::new();
    for code in codes {
        match code {
            Value::Variant(name, v) => match (name.as_str(), v.as_ref()) {
                ("literal", v) => match v.coerce_record_to_value() {
                    Value::U8(b) => vs.push(Value::U8(b)),
                    _ => panic!("inflate: expected U8"),
                },
                ("reference", Value::Record(fields)) => {
                    let length = &fields
                        .iter()
                        .find(|(label, _)| label == "length")
                        .unwrap()
                        .1;
                    let distance = &fields
                        .iter()
                        .find(|(label, _)| label == "distance")
                        .unwrap()
                        .1;
                    match (length, distance) {
                        (Value::U16(length), Value::U16(distance)) => {
                            let length = *length as usize;
                            let distance = *distance as usize;
                            if distance > vs.len() {
                                panic!("inflate: distance out of range");
                            }
                            let start = vs.len() - distance;
                            for i in 0..length {
                                vs.push(vs[start + i].clone());
                            }
                        }
                        _ => panic!(
                            "inflate: unexpected length/distance {:?} {:?}",
                            length, distance
                        ),
                    }
                }
                _ => panic!("inflate: unknown code"),
            },
            _ => panic!("inflate: expected variant"),
        }
    }
    vs
}

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use super::*;

    fn alts<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
        Format::Union(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }

    fn record<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
        Format::Record(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }

    fn optional(format: Format) -> Format {
        alts([("some", format), ("none", Format::EMPTY)])
    }

    fn repeat(format: Format) -> Format {
        Format::Repeat(Box::new(format))
    }

    fn repeat1(format: Format) -> Format {
        Format::Repeat1(Box::new(format))
    }

    fn is_byte(b: u8) -> Format {
        Format::Byte(ByteSet::from([b]))
    }

    fn not_byte(b: u8) -> Format {
        Format::Byte(!ByteSet::from([b]))
    }

    fn accepts(d: &Decoder, input: &[u8], tail: &[u8], expect: Value) {
        let program = Program::new();
        let mut stack = Stack::new();
        let (val, remain) = d.parse(&program, &mut stack, ReadCtxt::new(input)).unwrap();
        assert_eq!(val, expect);
        assert_eq!(remain.remaining(), tail);
    }

    fn rejects(d: &Decoder, input: &[u8]) {
        let program = Program::new();
        let mut stack = Stack::new();
        assert!(d
            .parse(&program, &mut stack, ReadCtxt::new(input))
            .is_none());
    }

    #[test]
    fn compile_fail() {
        let f = Format::Fail;
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_empty() {
        let f = Format::EMPTY;
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[0x00], Value::UNIT);
    }

    #[test]
    fn compile_byte_is() {
        let f = is_byte(0x00);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0));
        accepts(&d, &[0x00, 0xFF], &[0xFF], Value::U8(0));
        rejects(&d, &[0xFF]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_byte_not() {
        let f = not_byte(0x00);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        accepts(&d, &[0xFF, 0x00], &[0x00], Value::U8(0xFF));
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt() {
        let f = alts::<&str>([]);
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_alt_byte() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0xFF))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::variant("a", Value::U8(0x00)));
        accepts(&d, &[0xFF], &[], Value::variant("b", Value::U8(0xFF)));
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_ambiguous() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0x00))]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail() {
        let f = alts([("a", Format::Fail), ("b", Format::Fail)]);
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_end_of_input() {
        let f = alts([("a", Format::EndOfInput), ("b", Format::EndOfInput)]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_empty() {
        let f = alts([("a", Format::EMPTY), ("b", Format::EMPTY)]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail_end_of_input() {
        let f = alts([("a", Format::Fail), ("b", Format::EndOfInput)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::variant("b", Value::UNIT));
    }

    #[test]
    fn compile_alt_end_of_input_or_byte() {
        let f = alts([("a", Format::EndOfInput), ("b", is_byte(0x00))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::variant("a", Value::UNIT));
        accepts(&d, &[0x00], &[], Value::variant("b", Value::U8(0x00)));
        accepts(
            &d,
            &[0x00, 0x00],
            &[0x00],
            Value::variant("b", Value::U8(0x00)),
        );
        rejects(&d, &[0x11]);
    }

    #[test]
    fn compile_alt_opt() {
        let f = alts([("a", Format::EMPTY), ("b", is_byte(0x00))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::variant("b", Value::U8(0x00)));
        accepts(&d, &[], &[], Value::variant("a", Value::UNIT));
        accepts(&d, &[0xFF], &[0xFF], Value::variant("a", Value::UNIT));
    }

    #[test]
    fn compile_alt_opt_next() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), is_byte(0xFF)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::variant("some", Value::U8(0)), Value::U8(0xFF)]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![Value::variant("none", Value::UNIT), Value::U8(0xFF)]),
        );
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_opt_opt() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0xFF))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::variant("some", Value::U8(0)),
                Value::variant("some", Value::U8(0xFF)),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![
                Value::variant("some", Value::U8(0)),
                Value::variant("none", Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("some", Value::U8(0xFF)),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("none", Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("none", Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![
                Value::variant("none", Value::UNIT),
                Value::variant("none", Value::UNIT),
            ]),
        );
    }

    #[test]
    fn compile_alt_opt_ambiguous() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0x00))]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_opt_ambiguous_slow() {
        let alt = alts([
            ("0x00", is_byte(0x00)),
            ("0x01", is_byte(0x01)),
            ("0x02", is_byte(0x02)),
            ("0x03", is_byte(0x03)),
            ("0x04", is_byte(0x04)),
            ("0x05", is_byte(0x05)),
            ("0x06", is_byte(0x06)),
            ("0x07", is_byte(0x07)),
        ]);
        let rec = record([
            ("0", alt.clone()),
            ("1", alt.clone()),
            ("2", alt.clone()),
            ("3", alt.clone()),
            ("4", alt.clone()),
            ("5", alt.clone()),
            ("6", alt.clone()),
            ("7", alt.clone()),
        ]);
        let f = alts([("a", rec.clone()), ("b", rec.clone())]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_alt_repeat1_slow() {
        let f = repeat(alts([
            ("a", repeat1(is_byte(0x00))),
            ("b", is_byte(0x01)),
            ("c", is_byte(0x02)),
        ]));
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Decoder::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::Seq(vec![]));
        accepts(&d, &[0xFF], &[0xFF], Value::Seq(vec![]));
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)]));
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)]),
        );
    }

    #[test]
    fn compile_repeat_repeat() {
        let f = repeat(repeat(is_byte(0x00)));
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_cat_repeat() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0xFF))]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![]), Value::Seq(vec![])]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![Value::Seq(vec![Value::U8(0x00)]), Value::Seq(vec![])]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![Value::Seq(vec![]), Value::Seq(vec![Value::U8(0xFF)])]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)]),
                Value::Seq(vec![Value::U8(0xFF)]),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[0x00],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)]),
                Value::Seq(vec![Value::U8(0xFF)]),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![Value::Seq(vec![]), Value::Seq(vec![])]),
        );
    }

    #[test]
    fn compile_cat_end_of_input() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::EndOfInput]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT]),
        );
        rejects(&d, &[]);
        rejects(&d, &[0x00, 0x00]);
    }

    #[test]
    fn compile_cat_repeat_end_of_input() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), Format::EndOfInput]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![]), Value::UNIT]),
        );
        accepts(
            &d,
            &[0x00, 0x00, 0x00],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00), Value::U8(0x00), Value::U8(0x00)]),
                Value::UNIT,
            ]),
        );
        rejects(&d, &[0x00, 0x10]);
    }

    #[test]
    fn compile_cat_repeat_ambiguous() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0x00))]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x7F))),
        ]);
        assert!(Decoder::compile_one(&f).is_ok());
    }

    #[test]
    fn compile_repeat_fields_ambiguous() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x00))),
        ]);
        assert!(Decoder::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields_okay() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            (
                "second-and-third",
                optional(record([
                    (
                        "second",
                        Format::Tuple(vec![is_byte(0xFF), repeat(is_byte(0xFF))]),
                    ),
                    ("third", repeat(is_byte(0x00))),
                ])),
            ),
        ]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::record([
                ("first", Value::Seq(vec![])),
                ("second-and-third", Value::variant("none", Value::UNIT)),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                ("second-and-third", Value::variant("none", Value::UNIT)),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                (
                    "second-and-third",
                    Value::variant(
                        "some",
                        Value::record([
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                            ),
                            ("third", Value::Seq(vec![])),
                        ]),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                (
                    "second-and-third",
                    Value::variant(
                        "some",
                        Value::record(vec![
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                            ),
                            ("third", Value::Seq(vec![Value::U8(0x00)])),
                        ]),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0x7F],
            &[0x7F],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                ("second-and-third", Value::variant("none", Value::UNIT)),
            ]),
        );
    }

    #[test]
    fn compile_repeat1() {
        let f = repeat1(is_byte(0x00));
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0xFF]);
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)]));
        accepts(
            &d,
            &[0x00, 0xFF],
            &[0xFF],
            Value::Seq(vec![Value::U8(0x00)]),
        );
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)]),
        );
    }

    #[test]
    fn compile_align1() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(1), is_byte(0xFF)]);
        let d = Decoder::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_align2() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(2), is_byte(0xFF)]);
        let d = Decoder::compile_one(&f).unwrap();
        rejects(&d, &[0x00, 0xFF]);
        rejects(&d, &[0x00, 0x99, 0x99, 0xFF]);
        accepts(
            &d,
            &[0x00, 0x99, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }
}
