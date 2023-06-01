#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::collections::HashSet;
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
    fn matches(&self, stack: &mut Vec<Value>, pattern: &Pattern) -> bool {
        match (pattern, self) {
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
    Seq(Vec<Expr>),
    Match(Box<Expr>, Vec<(Pattern, Expr)>),

    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
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

    FlatMap(Box<Expr>, Box<Expr>),
    FlatMapAccum(Box<Expr>, Box<Expr>, Box<Expr>),
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
    ItemVar(usize),
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
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
    RepeatUntil(Expr, Box<Format>),
    /// Parse a format without advancing the stream position afterwards
    Peek(Box<Format>),
    /// Restrict a format to a sub-stream of a given number of bytes
    Slice(Expr, Box<Format>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(Expr, Box<Format>),
    /// Transform a decoded value with an expr
    Map(Expr, Box<Format>),
    /// Pattern match on an expression
    Match(Expr, Vec<(Pattern, Format)>),
}

impl Format {
    pub const EMPTY: Format = Format::Tuple(Vec::new());
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

    pub fn define_format(&mut self, name: impl Into<String>, format: Format) -> Format {
        let level = self.names.len();
        self.names.push(name.into());
        self.formats.push(format);
        Format::ItemVar(level)
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
pub enum Decoder {
    Fail,
    EndOfInput,
    Byte(ByteSet),
    Branch(MatchTree, Vec<(String, Decoder)>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(MatchTree, Box<Decoder>),
    Until(MatchTree, Box<Decoder>),
    RepeatCount(Expr, Box<Decoder>),
    RepeatUntil(Expr, Box<Decoder>),
    Peek(Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    WithRelativeOffset(Expr, Box<Decoder>),
    Map(Expr, Box<Decoder>),
    Match(Expr, Vec<(Pattern, Decoder)>),
}

impl Expr {
    fn eval(&self, stack: &mut Vec<Value>) -> Value {
        match self {
            Expr::Var(index) => stack[stack.len() - index - 1].clone(),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::U8(i) => Value::U8(*i),
            Expr::U16(i) => Value::U16(*i),
            Expr::U32(i) => Value::U32(*i),
            Expr::Tuple(exprs) => Value::Tuple(exprs.iter().map(|expr| expr.eval(stack)).collect()),
            Expr::TupleProj(head, index) => match head.eval(stack) {
                Value::Tuple(vs) => vs[*index].clone(),
                _ => panic!("expected tuple"),
            },
            Expr::Record(fields) => {
                Value::record(fields.iter().map(|(label, expr)| (label, expr.eval(stack))))
            }
            Expr::RecordProj(head, label) => head.eval(stack).record_proj(label),
            Expr::Variant(label, expr) => Value::variant(label, expr.eval(stack)),
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

            Expr::BitAnd(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(x & y),
                (Value::U16(x), Value::U16(y)) => Value::U16(x & y),
                (Value::U32(x), Value::U32(y)) => Value::U32(x & y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::BitOr(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(x | y),
                (Value::U16(x), Value::U16(y)) => Value::U16(x | y),
                (Value::U32(x), Value::U32(y)) => Value::U32(x | y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Eq(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x == y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x == y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x == y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Ne(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::Bool(x != y),
                (Value::U16(x), Value::U16(y)) => Value::Bool(x != y),
                (Value::U32(x), Value::U32(y)) => Value::Bool(x != y),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Rem(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_rem(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_rem(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_rem(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            #[rustfmt::skip]
            Expr::Shl(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_shl(x, u32::from(y)).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_shl(x, u32::from(y)).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shl(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Add(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_add(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_add(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_add(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Sub(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_sub(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_sub(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_sub(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },

            Expr::AsU16(x) => match x.eval(stack) {
                Value::U8(x) => Value::U16(u16::from(x)),
                Value::U16(x) => Value::U16(x),
                x => panic!("cannot convert {x:?} to U16"),
            },
            Expr::AsU32(x) => match x.eval(stack) {
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
        }
    }

    fn eval_bool(&self, stack: &mut Vec<Value>) -> bool {
        match self.eval(stack) {
            Value::Bool(b) => b,
            _ => panic!("value is not a bool"),
        }
    }

    fn eval_usize(&self, stack: &mut Vec<Value>) -> usize {
        match self.eval(stack) {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::U32(n) => usize::try_from(n).unwrap(),
            _ => panic!("value is not a number"),
        }
    }

    fn eval_tuple(&self, stack: &mut Vec<Value>) -> Vec<Value> {
        match self.eval(stack) {
            Value::Tuple(values) => values,
            _ => panic!("value is not a tuple"),
        }
    }
}

impl Format {
    /// Returns `true` if the format matches the empty byte string
    fn is_nullable(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level) => module.get_format(*level).is_nullable(module),
            Format::Fail => false,
            Format::EndOfInput => true,
            Format::Byte(_) => false,
            Format::Union(branches) => branches.iter().any(|(_, f)| f.is_nullable(module)),
            Format::Tuple(fields) => fields.iter().all(|f| f.is_nullable(module)),
            Format::Record(fields) => fields.iter().all(|(_, f)| f.is_nullable(module)),
            Format::Repeat(_) => true,
            Format::Repeat1(_) => false,
            Format::RepeatCount(_, _) => true,
            Format::RepeatUntil(_, f) => f.is_nullable(module),
            Format::Peek(_) => true,
            Format::Slice(_, _) => true,
            Format::WithRelativeOffset(_, _) => true,
            Format::Map(_, f) => f.is_nullable(module),
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.is_nullable(module)),
        }
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
            Format::ItemVar(level) => self.add(module, index, module.get_format(*level), next),
            Format::Fail => Ok(()),
            Format::EndOfInput => self.accept(index),
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
                None => self.add_next(module, index, next.clone()),
                Some((a, fields)) => {
                    self.add(module, index, a, Rc::new(Next::Tuple(fields, next.clone())))
                }
            },
            Format::Record(fields) => match fields.split_first() {
                None => self.add_next(module, index, next.clone()),
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
            Format::RepeatUntil(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::Peek(_a) => {
                self.accept(index) // FIXME
            }
            Format::Slice(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::WithRelativeOffset(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::Map(_expr, a) => self.add(module, index, a, next),
            Format::Match(_, branches) => {
                for (_, f) in branches {
                    self.add(module, index, f, next.clone())?;
                }
                Ok(())
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
    fn matches(&self, input: &[u8]) -> Option<usize> {
        match input.split_first() {
            None => self.accept,
            Some((b, input)) => {
                for (bs, s) in &self.branches {
                    if bs.contains(*b) {
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

impl Decoder {
    pub fn compile(module: &FormatModule, format: &Format) -> Result<Decoder, String> {
        Decoder::compile_next(module, format, Rc::new(Next::Empty))
    }

    fn compile_next(
        module: &FormatModule,
        format: &Format,
        next: Rc<Next<'_>>,
    ) -> Result<Decoder, String> {
        match format {
            Format::ItemVar(level) => {
                Decoder::compile_next(module, module.get_format(*level), next)
            }
            Format::Fail => Ok(Decoder::Fail),
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::Union(branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut ds = Vec::with_capacity(branches.len());
                for (label, f) in branches {
                    ds.push((
                        label.clone(),
                        Decoder::compile_next(module, f, next.clone())?,
                    ));
                    fs.push(f.clone());
                }
                if let Some(tree) = MatchTree::build(module, &fs, next) {
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
                    let df = Decoder::compile_next(module, f, next)?;
                    dfields.push(df);
                }
                Ok(Decoder::Tuple(dfields))
            }
            Format::Record(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some((name, f)) = fields.next() {
                    let next = Rc::new(Next::Record(fields.as_slice(), next.clone()));
                    let df = Decoder::compile_next(module, f, next)?;
                    dfields.push((name.clone(), df));
                }
                Ok(Decoder::Record(dfields))
            }
            Format::Repeat(a) => {
                if a.is_nullable(module) {
                    return Err("cannot repeat nullable format".to_string());
                }
                let da = Decoder::compile_next(module, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                if let Some(tree) = MatchTree::build(module, &[fa, fb], next) {
                    Ok(Decoder::While(tree, Box::new(da)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::Repeat1(a) => {
                if a.is_nullable(module) {
                    return Err("cannot repeat nullable format".to_string());
                }
                let da = Decoder::compile_next(module, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::EMPTY;
                let fb = Format::Tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build(module, &[fa, fb], next) {
                    Ok(Decoder::Until(tree, Box::new(da)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::RepeatCount(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(module, a, next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
            }
            Format::RepeatUntil(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile_next(module, a, next)?);
                Ok(Decoder::RepeatUntil(expr.clone(), da))
            }
            Format::Peek(a) => {
                let da = Box::new(Decoder::compile_next(module, a, next)?);
                Ok(Decoder::Peek(da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(Decoder::compile_next(module, a, Rc::new(Next::Empty))?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::WithRelativeOffset(expr, a) => {
                let da = Box::new(Decoder::compile_next(module, a, Rc::new(Next::Empty))?);
                Ok(Decoder::WithRelativeOffset(expr.clone(), da))
            }
            Format::Map(expr, a) => {
                let da = Box::new(Decoder::compile_next(module, a, next)?);
                Ok(Decoder::Map(expr.clone(), da))
            }
            Format::Match(head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| {
                        Ok((
                            pattern.clone(),
                            Decoder::compile_next(module, f, next.clone())?,
                        ))
                    })
                    .collect::<Result<_, String>>()?;
                Ok(Decoder::Match(head.clone(), branches))
            }
        }
    }

    pub fn parse<'input>(
        &self,
        stack: &mut Vec<Value>,
        input: &'input [u8],
    ) -> Option<(Value, &'input [u8])> {
        match self {
            Decoder::Fail => None,
            Decoder::EndOfInput => match input {
                [] => Some((Value::UNIT, &[])),
                _ => None,
            },
            Decoder::Byte(bs) => {
                let (&b, input) = input.split_first()?;
                if bs.contains(b) {
                    Some((Value::U8(b), input))
                } else {
                    None
                }
            }
            Decoder::Branch(tree, branches) => {
                let index = tree.matches(input)?;
                let (label, d) = &branches[index];
                let (v, input) = d.parse(stack, input)?;
                Some((Value::Variant(label.clone(), Box::new(v)), input))
            }
            Decoder::Tuple(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for f in fields {
                    let (vf, next_input) = f.parse(stack, input)?;
                    input = next_input;
                    v.push(vf.clone());
                }
                Some((Value::Tuple(v), input))
            }
            Decoder::Record(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for (name, f) in fields {
                    let (vf, next_input) = f.parse(stack, input)?;
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
                    let (va, next_input) = a.parse(stack, input)?;
                    input = next_input;
                    v.push(va);
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Until(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(stack, input)?;
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
                    let (va, next_input) = a.parse(stack, input)?;
                    input = next_input;
                    v.push(va);
                }
                Some((Value::Seq(v), input))
            }
            Decoder::RepeatUntil(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(stack, input)?;
                    input = next_input;
                    stack.push(va);
                    let done = expr.eval_bool(stack);
                    let va = stack.pop()?;
                    v.push(va);
                    if done {
                        break;
                    }
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Peek(a) => {
                let (v, _next_input) = a.parse(stack, input)?;
                Some((v, input))
            }
            Decoder::Slice(expr, a) => {
                let size = expr.eval_usize(stack);
                if size <= input.len() {
                    let (slice, input) = input.split_at(size);
                    let (v, _) = a.parse(stack, slice)?;
                    Some((v, input))
                } else {
                    None
                }
            }
            Decoder::WithRelativeOffset(expr, a) => {
                let offset = expr.eval_usize(stack);
                if offset <= input.len() {
                    let (_, slice) = input.split_at(offset);
                    let (v, _) = a.parse(stack, slice)?;
                    Some((v, input))
                } else {
                    None
                }
            }
            Decoder::Map(expr, a) => {
                let (va, input) = a.parse(stack, input)?;
                stack.push(va);
                let v = expr.eval(stack);
                stack.pop();
                Some((v, input))
            }
            Decoder::Match(head, branches) => {
                let head = head.eval(stack);
                let initial_len = stack.len();
                let (_, decoder) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(stack, pattern))
                    .expect("exhaustive patterns");
                let value = decoder.parse(stack, input);
                stack.truncate(initial_len);
                value
            }
        }
    }
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
        let mut stack = Vec::new();
        let (val, remain) = d.parse(&mut stack, input).unwrap();
        assert_eq!(val, expect);
        assert_eq!(remain, tail);
    }

    fn rejects(d: &Decoder, input: &[u8]) {
        let mut stack = Vec::new();
        assert!(d.parse(&mut stack, input).is_none());
    }

    #[test]
    fn compile_fail() {
        let f = Format::Fail;
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_empty() {
        let f = Format::EMPTY;
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[0x00], Value::UNIT);
    }

    #[test]
    fn compile_byte_is() {
        let f = is_byte(0x00);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0));
        accepts(&d, &[0x00, 0xFF], &[0xFF], Value::U8(0));
        rejects(&d, &[0xFF]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_byte_not() {
        let f = not_byte(0x00);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        accepts(&d, &[0xFF, 0x00], &[0x00], Value::U8(0xFF));
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt() {
        let f = alts::<&str>([]);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_alt_byte() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0xFF))]);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        accepts(&d, &[0x00], &[], Value::variant("a", Value::U8(0x00)));
        accepts(&d, &[0xFF], &[], Value::variant("b", Value::U8(0xFF)));
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_ambiguous() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0x00))]);
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
    }

    #[test]
    fn compile_alt_fail() {
        let f = alts([("a", Format::Fail), ("b", Format::Fail)]);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_end_of_input() {
        let f = alts([("a", Format::EndOfInput), ("b", Format::EndOfInput)]);
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
    }

    #[test]
    fn compile_alt_empty() {
        let f = alts([("a", Format::EMPTY), ("b", Format::EMPTY)]);
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
    }

    #[test]
    fn compile_alt_fail_end_of_input() {
        let f = alts([("a", Format::Fail), ("b", Format::EndOfInput)]);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        accepts(&d, &[], &[], Value::variant("b", Value::UNIT));
    }

    #[test]
    fn compile_alt_end_of_input_or_byte() {
        let f = alts([("a", Format::EndOfInput), ("b", is_byte(0x00))]);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
        accepts(&d, &[0x00], &[], Value::variant("b", Value::U8(0x00)));
        accepts(&d, &[], &[], Value::variant("a", Value::UNIT));
        accepts(&d, &[0xFF], &[0xFF], Value::variant("a", Value::UNIT));
    }

    #[test]
    fn compile_alt_opt_next() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), is_byte(0xFF)]);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
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
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
    }

    #[test]
    fn compile_repeat_alt_repeat1_slow() {
        let f = repeat(alts([
            ("a", repeat1(is_byte(0x00))),
            ("b", is_byte(0x01)),
            ("c", is_byte(0x02)),
        ]));
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
    }

    #[test]
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
    }

    #[test]
    fn compile_cat_repeat() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0xFF))]);
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
    }

    #[test]
    fn compile_repeat_fields() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x7F))),
        ]);
        assert!(Decoder::compile(&FormatModule::new(), &f).is_ok());
    }

    #[test]
    fn compile_repeat_fields_ambiguous() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x00))),
        ]);
        assert!(Decoder::compile(&FormatModule::new(), &f).is_err());
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
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
        let d = Decoder::compile(&FormatModule::new(), &f).unwrap();
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
}
