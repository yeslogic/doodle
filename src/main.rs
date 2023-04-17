use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use serde::Serialize;

use crate::byte_set::ByteSet;

mod byte_set;

#[derive(Clone, Debug)]
enum Pattern {
    Binding,
    Wildcard,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Pattern>),
    Seq(Vec<Pattern>),
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),
    Seq(Vec<Value>),
}

impl Value {
    const UNIT: Value = Value::Tuple(Vec::new());

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
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Var(usize),
    Sub(Box<Expr>, Box<Expr>),
    IsEven(Box<Expr>),
    Tuple(Vec<Expr>),
    Record(Vec<(String, Expr)>),
    Seq(Vec<Expr>),
}

impl Expr {
    const UNIT: Expr = Expr::Tuple(Vec::new());
}

#[derive(Clone, Debug)]
enum Func {
    Expr(Expr),
    TupleProj(usize),
    RecordProj(String),
    U16Be,
    U16Le,
    U32Be,
    U32Le,
    Stream,
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
/// ⟦ Fail ⟧                    = ∅
/// ⟦ Empty ⟧                   = ε
/// ⟦ Byte({}) ⟧                = ∅
/// ⟦ Byte(!{}) ⟧               = .
/// ⟦ Byte({b}) ⟧               = b
/// ⟦ Byte({b₀, ... bₙ}) ⟧      = b₀ | ... | bₙ
/// ⟦ Alt(f₀, f₁) ⟧             = ⟦ f₀ ⟧ | ⟦ f₁ ⟧
/// ⟦ Switch([]) ⟧              = ∅
/// ⟦ Switch([f₀, ..., fₙ]) ⟧   = ⟦ f₀ ⟧ | ... | ⟦ fₙ ⟧
/// ⟦ Cat(f₀, f₁) ⟧             = ⟦ f₀ ⟧ ⟦ f₁ ⟧
/// ⟦ Tuple([]) ⟧               = ε
/// ⟦ Tuple([f₀, ..., fₙ]) ⟧    = ⟦ f₀ ⟧ ... ⟦ fₙ ⟧
/// ⟦ Repeat(f) ⟧               = ⟦ f ⟧*
/// ⟦ Repeat1(f) ⟧              = ⟦ f ⟧ ⟦ f ⟧*
/// ⟦ RepeatCount(n, f) ⟧       = ⟦ f ⟧ ... ⟦ f ⟧
///                               ╰── n times ──╯
/// ```
///
/// Note that the data dependency present in record formats means that these
/// formats no longer describe regular languages.
///
/// [regular expressions]: https://en.wikipedia.org/wiki/Regular_expression#Formal_definition
#[derive(Clone, Debug)]
enum Format {
    /// A format that never matches
    Fail,
    /// A format that matches the empty byte string
    Empty,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Matches the union of the byte strings matched by the two formats
    Alt(Box<Format>, Box<Format>),
    /// Matches the union of the byte strings matched by all the formats
    Union(Vec<Format>),
    /// Matches the set of byte strings matched by the first format, followed by
    /// the second format
    Cat(Box<Format>, Box<Format>),
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
    /// Restrict a format to a sub-stream of a given number of bytes
    Slice(Expr, Box<Format>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(Expr, Box<Format>),
    /// Transform a decoded value with a function
    Map(Func, Box<Format>),
    /// Pattern match on an expression
    Match(Expr, Vec<(Pattern, Format)>),
}

#[derive(Clone, Debug)]
struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
}

/// Decoders with a fixed amount of lookahead
enum Decoder {
    Fail,
    Empty,
    EndOfInput,
    Byte(ByteSet),
    Branch(MatchTree, Vec<Decoder>),
    Cat(Box<Decoder>, Box<Decoder>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(MatchTree, Box<Decoder>),
    Until(MatchTree, Box<Decoder>),
    RepeatCount(Expr, Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    WithRelativeOffset(Expr, Box<Decoder>),
    Map(Func, Box<Decoder>),
    Match(Expr, Vec<(Pattern, Decoder)>),
}

impl Expr {
    fn eval(&self, stack: &[Value]) -> Value {
        match self {
            Expr::Bool(b) => Value::Bool(*b),
            Expr::U8(i) => Value::U8(*i),
            Expr::U16(i) => Value::U16(*i),
            Expr::U32(i) => Value::U32(*i),
            Expr::Var(index) => stack[stack.len() - index - 1].clone(),
            Expr::Sub(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_sub(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_sub(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_sub(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::IsEven(x) => match x.eval(stack) {
                Value::U8(x) => Value::Bool(x % 2 == 0),
                Value::U16(x) => Value::Bool(x % 2 == 0),
                Value::U32(x) => Value::Bool(x % 2 == 0),
                _ => panic!("IsEven expected number"),
            },
            Expr::Tuple(exprs) => Value::Tuple(exprs.iter().map(|expr| expr.eval(stack)).collect()),
            Expr::Record(fields) => Value::Record(
                fields
                    .iter()
                    .map(|(label, expr)| (label.clone(), expr.eval(stack)))
                    .collect(),
            ),
            Expr::Seq(exprs) => Value::Seq(exprs.iter().map(|expr| expr.eval(stack)).collect()),
        }
    }

    fn eval_usize(&self, stack: &[Value]) -> usize {
        match self.eval(stack) {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::U32(n) => usize::try_from(n).unwrap(),
            Value::Bool(_) | Value::Tuple(_) | Value::Record(_) | Value::Seq(_) => {
                panic!("value is not number")
            }
        }
    }
}

impl Func {
    fn eval(&self, stack: &mut Vec<Value>, arg: Value) -> Value {
        match self {
            Func::Expr(e) => e.eval(stack),
            Func::TupleProj(i) => match arg {
                Value::Tuple(vs) => vs[*i].clone(),
                _ => panic!("TupleProj: expected tuple"),
            },
            Func::RecordProj(label) => match arg {
                Value::Record(fields) => match fields.iter().find(|(l, _)| label == l) {
                    Some((_, v)) => v.clone(),
                    None => panic!("RecordProj: {label} not found in record"),
                },
                _ => panic!("RecordProj: expected record"),
            },
            Func::U16Be => match arg {
                Value::Tuple(vs) => match vs.as_slice() {
                    [Value::U8(hi), Value::U8(lo)] => Value::U16(u16::from_be_bytes([*hi, *lo])),
                    _ => panic!("U16Be: expected (U8, U8)"),
                },
                _ => panic!("U16Be: expected (_, _)"),
            },
            Func::U16Le => match arg {
                Value::Tuple(vs) => match vs.as_slice() {
                    [Value::U8(lo), Value::U8(hi)] => Value::U16(u16::from_le_bytes([*lo, *hi])),
                    _ => panic!("U16Le: expected (U8, U8)"),
                },
                _ => panic!("U16Le: expected (_, _)"),
            },
            Func::U32Be => match arg {
                Value::Tuple(vs) => match vs.as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                        Value::U32(u32::from_be_bytes([*a, *b, *c, *d]))
                    }
                    _ => panic!("U32Be: expected (U8, U8, U8, U8)"),
                },
                _ => panic!("U32Be: expected (_, _, _, _)"),
            },
            Func::U32Le => match arg {
                Value::Tuple(vs) => match vs.as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                        Value::U32(u32::from_le_bytes([*a, *b, *c, *d]))
                    }
                    _ => panic!("U32Le: expected (U8, U8, U8, U8)"),
                },
                _ => panic!("U32Le: expected (_, _, _, _)"),
            },
            Func::Stream => match arg {
                Value::Seq(vs) => {
                    // FIXME could also condense nested sequences
                    Value::Seq(vs.into_iter().filter(|v| *v != Value::UNIT).collect())
                }
                _ => panic!("Stream: expected Seq"),
            },
        }
    }
}

impl Format {
    /// Returns `true` if the format matches the empty byte string
    fn is_nullable(&self) -> bool {
        match self {
            Format::Fail => false,
            Format::Empty => true,
            Format::EndOfInput => true,
            Format::Byte(_) => false,
            Format::Alt(a, b) => a.is_nullable() || b.is_nullable(),
            Format::Union(branches) => branches.iter().any(|f| f.is_nullable()),
            Format::Cat(a, b) => a.is_nullable() && b.is_nullable(),
            Format::Tuple(fields) => fields.iter().all(|f| f.is_nullable()),
            Format::Record(fields) => fields.iter().all(|(_, f)| f.is_nullable()),
            Format::Repeat(_a) => true,
            Format::Repeat1(_a) => false,
            Format::RepeatCount(_expr, _a) => true,
            Format::Slice(_expr, _a) => true,
            Format::WithRelativeOffset(_, _) => true,
            Format::Map(_f, a) => a.is_nullable(),
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.is_nullable()),
        }
    }
}

#[derive(Debug)]
enum Next<'a> {
    Empty,
    Cat(&'a Format, &'a Next<'a>),
    Tuple(&'a [Format], &'a Next<'a>),
    Record(&'a [(String, Format)], &'a Next<'a>),
    Repeat(&'a Format, &'a Next<'a>),
}

impl MatchTree {
    fn reject() -> MatchTree {
        MatchTree {
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

    fn add_next(&mut self, index: usize, depth: usize, next: &Next) -> Result<(), ()> {
        match next {
            Next::Empty => self.accept(index),
            Next::Cat(f, next) => self.add(index, depth, f, next),
            Next::Tuple(fs, next) => match fs.split_first() {
                None => self.add_next(index, depth, next),
                Some((f, fs)) => self.add(index, depth, f, &Next::Tuple(fs, next)),
            },
            Next::Record(fs, next) => match fs.split_first() {
                None => self.add_next(index, depth, next),
                Some(((_n, f), fs)) => self.add(index, depth, f, &Next::Record(fs, next)),
            },
            Next::Repeat(a, next) => {
                self.add_next(index, depth, next)?;
                self.add(index, depth, a, &Next::Repeat(a, next))?;
                Ok(())
            }
        }
    }

    pub fn add(&mut self, index: usize, depth: usize, f: &Format, next: &Next) -> Result<(), ()> {
        match f {
            Format::Fail => Ok(()),
            Format::Empty => self.add_next(index, depth, next),
            Format::EndOfInput => self.accept(index),
            Format::Byte(bs) => {
                let mut bs = *bs;
                let mut new_branches = Vec::new();
                for (bs0, tree) in self.branches.iter_mut() {
                    let common = bs0.intersection(&bs);
                    if !common.is_empty() {
                        let orig = bs0.difference(&bs);
                        if !orig.is_empty() {
                            new_branches.push((orig, tree.clone()));
                        }
                        *bs0 = common;
                        if depth > 0 {
                            tree.add_next(index, depth - 1, next)?;
                        } else {
                            tree.accept(index)?;
                        }
                        bs = bs.difference(bs0);
                    }
                }
                if !bs.is_empty() {
                    let mut tree = MatchTree::reject();
                    if depth > 0 {
                        tree.add_next(index, depth - 1, next)?;
                    } else {
                        tree.accept(index)?;
                    }
                    self.branches.push((bs, tree));
                }
                self.branches.append(&mut new_branches);
                Ok(())
            }
            Format::Alt(a, b) => {
                self.add(index, depth, a, next)?;
                self.add(index, depth, b, next)?;
                Ok(())
            }
            Format::Union(branches) => {
                for f in branches {
                    self.add(index, depth, f, next)?;
                }
                Ok(())
            }
            Format::Cat(a, b) => self.add(index, depth, a, &Next::Cat(b, next)),
            Format::Tuple(fields) => match fields.split_first() {
                None => self.add_next(index, depth, next),
                Some((a, fields)) => self.add(index, depth, a, &Next::Tuple(fields, next)),
            },
            Format::Record(fields) => match fields.split_first() {
                None => self.add_next(index, depth, next),
                Some(((_, a), fields)) => self.add(index, depth, a, &Next::Record(fields, next)),
            },
            Format::Repeat(a) => {
                self.add_next(index, depth, next)?;
                self.add(index, depth, a, &Next::Repeat(a, next))?;
                Ok(())
            }
            Format::Repeat1(a) => {
                self.add(index, depth, a, &Next::Repeat(a, next))?;
                Ok(())
            }
            Format::RepeatCount(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::Slice(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::WithRelativeOffset(_expr, _a) => {
                self.accept(index) // FIXME
            }
            Format::Map(_f, a) => self.add(index, depth, a, next),
            Format::Match(_, branches) => {
                for (_, f) in branches {
                    self.add(index, depth, f, next)?;
                }
                Ok(())
            }
        }
    }

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

    fn build0(depth: usize, branches: &[&Format], next: &Next) -> Option<MatchTree> {
        let mut tree = MatchTree::reject();
        for (i, branch) in branches.iter().enumerate() {
            tree.add(i, depth, branch, next).ok()?;
        }
        Some(tree)
    }

    fn build(branches: &[&Format], next: &Next) -> Option<MatchTree> {
        const MAX_DEPTH: usize = 16;
        for depth in 0..MAX_DEPTH {
            if let Some(tree) = MatchTree::build0(depth, branches, next) {
                return Some(tree);
            }
        }
        None
    }
}

impl Decoder {
    pub fn compile(f: &Format, next: &Next) -> Result<Decoder, String> {
        match f {
            Format::Fail => Ok(Decoder::Fail),
            Format::Empty => Ok(Decoder::Empty),
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::Alt(a, b) => {
                let da = Decoder::compile(a, next)?;
                let db = Decoder::compile(b, next)?;
                if let Some(switch) = MatchTree::build(&[a, b], next) {
                    Ok(Decoder::Branch(switch, vec![da, db]))
                } else {
                    Err(format!("cannot build match tree for {:?}", f))
                }
            }
            Format::Union(branches) => {
                let mut ds = Vec::new();
                let mut fs = Vec::new();
                for f in branches {
                    let d = Decoder::compile(f, next)?;
                    ds.push(d);
                    fs.push(f);
                }
                if let Some(tree) = MatchTree::build(&fs, next) {
                    Ok(Decoder::Branch(tree, ds))
                } else {
                    Err(format!("cannot build match tree for {:?}", f))
                }
            }
            Format::Cat(a, b) => {
                let da = Box::new(Decoder::compile(a, &Next::Cat(b, next))?);
                let db = Box::new(Decoder::compile(b, next)?);
                Ok(Decoder::Cat(da, db))
            }
            Format::Tuple(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let df = Decoder::compile(f, &Next::Tuple(fields.as_slice(), next))?;
                    dfields.push(df);
                }
                Ok(Decoder::Tuple(dfields))
            }
            Format::Record(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some((name, f)) = fields.next() {
                    let df = Decoder::compile(f, &Next::Record(fields.as_slice(), next))?;
                    dfields.push((name.clone(), df));
                }
                Ok(Decoder::Record(dfields))
            }
            Format::Repeat(a) => {
                if a.is_nullable() {
                    return Err("cannot repeat nullable format".to_string());
                }
                let da = Box::new(Decoder::compile(a, &Next::Repeat(a, next))?);
                let astar = Format::Repeat(a.clone());
                let fa = &Format::Cat(a.clone(), Box::new(astar));
                let fb = &Format::Empty;
                if let Some(tree) = MatchTree::build(&[fa, fb], next) {
                    Ok(Decoder::While(tree, da))
                } else {
                    Err(format!("cannot build match tree for {:?}", f))
                }
            }
            Format::Repeat1(a) => {
                if a.is_nullable() {
                    return Err("cannot repeat nullable format".to_string());
                }
                let da = Box::new(Decoder::compile(a, &Next::Repeat(a, next))?);
                let astar = Format::Repeat(a.clone());
                let fa = &Format::Empty;
                let fb = &Format::Cat(a.clone(), Box::new(astar));
                if let Some(tree) = MatchTree::build(&[fa, fb], next) {
                    Ok(Decoder::Until(tree, da))
                } else {
                    Err(format!("cannot build match tree for {:?}", f))
                }
            }
            Format::RepeatCount(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile(a, next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(Decoder::compile(a, &Next::Empty)?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::WithRelativeOffset(expr, a) => {
                let da = Box::new(Decoder::compile(a, &Next::Empty)?);
                Ok(Decoder::WithRelativeOffset(expr.clone(), da))
            }
            Format::Map(f, a) => {
                let da = Box::new(Decoder::compile(a, next)?);
                Ok(Decoder::Map(f.clone(), da))
            }
            Format::Match(head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| Ok((pattern.clone(), Decoder::compile(f, next)?)))
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
            Decoder::Empty => Some((Value::UNIT, input)),
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
                branches[index].parse(stack, input)
            }
            Decoder::Cat(a, b) => {
                let (va, input) = a.parse(stack, input)?;
                let (vb, input) = b.parse(stack, input)?;
                Some((Value::Tuple(vec![va, vb]), input))
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
                while tree.matches(input) == Some(0) {
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
                    if tree.matches(input) == Some(0) {
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
            Decoder::Map(f, a) => {
                let (va, input) = a.parse(stack, input)?;
                Some((f.eval(stack, va), input))
            }
            Decoder::Match(head, branches) => {
                let head = head.eval(stack);
                let initial_len = stack.len();
                let (_, decoder) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(stack, &pattern))
                    .expect("exhaustive patterns");
                let value = decoder.parse(stack, input);
                stack.truncate(initial_len);
                value
            }
        }
    }
}

mod render_tree {
    use std::{fmt, io};

    use crate::Value;

    pub fn print_value(value: &Value) {
        Context::new(io::stdout()).write_value(value).unwrap()
    }

    fn is_atomic_value(value: &Value) -> bool {
        match value {
            Value::Bool(_) => true,
            Value::U8(_) => true,
            Value::U16(_) => true,
            Value::U32(_) => true,
            Value::Tuple(vals) => vals.is_empty(),
            Value::Record(fields) => fields.is_empty(),
            Value::Seq(vals) => vals.is_empty(),
        }
    }

    enum Column {
        Branch,
        Space,
    }

    pub struct Context<W: io::Write> {
        writer: W,
        gutter: Vec<Column>,
        preview_len: Option<usize>,
    }

    impl<W: io::Write> Context<W> {
        pub fn new(writer: W) -> Context<W> {
            Context {
                writer,
                gutter: Vec::new(),
                preview_len: Some(10),
            }
        }

        pub fn write_value(&mut self, value: &Value) -> io::Result<()> {
            match value {
                Value::Bool(true) => write!(&mut self.writer, "true"),
                Value::Bool(false) => write!(&mut self.writer, "false"),
                Value::U8(i) => write!(&mut self.writer, "{i}"),
                Value::U16(i) => write!(&mut self.writer, "{i}"),
                Value::U32(i) => write!(&mut self.writer, "{i}"),
                Value::Tuple(vals) if vals.is_empty() => write!(&mut self.writer, "()"),
                Value::Seq(vals) if vals.is_empty() => write!(&mut self.writer, "[]"),
                Value::Record(fields) if fields.is_empty() => write!(&mut self.writer, "{{}}"),
                Value::Tuple(vals) => self.write_elems(vals),
                Value::Seq(vals) => self.write_elems(vals),
                Value::Record(fields) => self.write_fields(fields),
            }
        }

        fn write_elems(&mut self, vals: &[Value]) -> Result<(), io::Error> {
            match self.preview_len {
                Some(max_len) if vals.len() > max_len && vals.iter().all(is_atomic_value) => {
                    let last_index = vals.len() - 1;
                    for (index, val) in vals[0..max_len].iter().enumerate() {
                        self.write_field_value_continue(index, val)?;
                    }
                    if max_len != last_index {
                        self.write_field_skipped()?;
                    }
                    self.write_field_value_last(last_index, &vals[last_index])
                }
                Some(_) | None => {
                    let last_index = vals.len() - 1;
                    for (index, val) in vals[..last_index].iter().enumerate() {
                        self.write_field_value_continue(index, val)?;
                    }
                    self.write_field_value_last(last_index, &vals[last_index])
                }
            }
        }

        fn write_fields(&mut self, fields: &[(String, Value)]) -> Result<(), io::Error> {
            let last_index = fields.len() - 1;
            for (label, val) in &fields[..last_index] {
                self.write_field_value_continue(label, val)?;
            }
            let (label, val) = &fields[last_index];
            self.write_field_value_last(label, val)
        }

        fn write_gutter(&mut self) -> io::Result<()> {
            for column in &self.gutter {
                match column {
                    Column::Branch => write!(&mut self.writer, "│   ")?,
                    Column::Space => write!(&mut self.writer, "    ")?,
                }
            }
            Ok(())
        }

        fn write_field_value_continue(
            &mut self,
            label: impl fmt::Display,
            value: &Value,
        ) -> io::Result<()> {
            self.write_gutter()?;
            write!(&mut self.writer, "├── {label} :=")?;
            self.gutter.push(Column::Branch);
            self.write_field_value(value)?;
            self.gutter.pop();
            Ok(())
        }

        fn write_field_value_last(
            &mut self,
            label: impl fmt::Display,
            value: &Value,
        ) -> io::Result<()> {
            self.write_gutter()?;
            write!(&mut self.writer, "└── {label} :=")?;
            self.gutter.push(Column::Space);
            self.write_field_value(value)?;
            self.gutter.pop();
            Ok(())
        }

        fn write_field_value(&mut self, value: &Value) -> io::Result<()> {
            if is_atomic_value(value) {
                write!(&mut self.writer, " ")?;
                self.write_value(value)?;
                writeln!(&mut self.writer)
            } else {
                writeln!(&mut self.writer)?;
                self.write_value(value)
            }
        }

        fn write_field_skipped(&mut self) -> io::Result<()> {
            self.write_gutter()?;
            writeln!(&mut self.writer, "~")
        }
    }
}

fn alts(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Union(formats.into_iter().collect())
}

fn record<Label: ToString>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
    Format::Record(
        (fields.into_iter())
            .map(|(label, format)| (label.to_string(), format))
            .collect(),
    )
}

fn optional(format: Format) -> Format {
    alts([format, Format::Empty])
}

fn repeat(format: Format) -> Format {
    Format::Repeat(Box::new(format))
}

fn repeat1(format: Format) -> Format {
    Format::Repeat1(Box::new(format))
}

fn repeat_count(len: Expr, format: Format) -> Format {
    Format::RepeatCount(len, Box::new(format))
}

fn is_byte(b: u8) -> Format {
    Format::Byte(ByteSet::from([b]))
}

fn not_byte(b: u8) -> Format {
    Format::Byte(!ByteSet::from([b]))
}

fn any_byte() -> Format {
    Format::Byte(ByteSet::full())
}

fn is_bytes(bytes: &[u8]) -> Format {
    Format::Tuple(bytes.iter().copied().map(is_byte).collect())
}

fn any_bytes() -> Format {
    repeat(any_byte())
}

fn u8() -> Format {
    any_byte()
}

fn u16be() -> Format {
    Format::Map(
        Func::U16Be,
        Box::new(Format::Tuple(vec![any_byte(), any_byte()])),
    )
}

fn u16le() -> Format {
    Format::Map(
        Func::U16Le,
        Box::new(Format::Tuple(vec![any_byte(), any_byte()])),
    )
}

fn u32be() -> Format {
    Format::Map(
        Func::U32Be,
        Box::new(Format::Tuple(vec![
            any_byte(),
            any_byte(),
            any_byte(),
            any_byte(),
        ])),
    )
}

fn u32le() -> Format {
    Format::Map(
        Func::U32Le,
        Box::new(Format::Tuple(vec![
            any_byte(),
            any_byte(),
            any_byte(),
            any_byte(),
        ])),
    )
}

fn riff_format() -> Format {
    fn chunk(tag: Format, data: Format) -> Format {
        record([
            ("tag", tag),
            ("length", u32le()),
            ("data", Format::Slice(Expr::Var(0), Box::new(data))),
            (
                "pad",
                Format::Match(
                    Expr::IsEven(Box::new(Expr::Var(1))),
                    vec![
                        (Pattern::Bool(true), Format::Empty),
                        (Pattern::Bool(false), is_byte(0x00)),
                    ],
                ),
            ),
        ])
    }

    let any_tag = Format::Tuple(vec![any_byte(), any_byte(), any_byte(), any_byte()]); // FIXME ASCII

    let subchunks = record([
        ("tag", any_tag.clone()),
        ("chunks", repeat(chunk(any_tag, any_bytes()))),
    ]);

    chunk(is_bytes(b"RIFF"), subchunks.clone())
}

fn png_format() -> Format {
    fn chunk(tag: Format, data: Format) -> Format {
        record([
            ("length", u32be()), // FIXME < 2^31
            ("tag", tag),
            ("data", Format::Slice(Expr::Var(1), Box::new(data))),
            ("crc", u32be()), // FIXME check this
        ])
    }

    //let any_tag = Format::Tuple(vec![any_byte(), any_byte(), any_byte(), any_byte()]); // FIXME ASCII

    let ihdr_tag = is_bytes(b"IHDR");
    let ihdr_data = record([
        ("width", u32be()),
        ("height", u32be()),
        ("bit-depth", u8()),
        ("color-type", u8()),
        ("compression-method", u8()),
        ("filter-method", u8()),
        ("interlace-method", u8()),
    ]);

    let idat_tag = is_bytes(b"IDAT");
    let idat_data = any_bytes();

    let iend_tag = is_bytes(b"IEND");
    let iend_data = Format::Empty; // FIXME ensure IEND length = 0

    let other_tag = alts([
        is_bytes(b"PLTE"),
        is_bytes(b"bKGD"),
        is_bytes(b"pHYs"),
        is_bytes(b"tIME"),
        is_bytes(b"tRNS"),
        // FIXME other tags excluding IHDR/IDAT/IEND
    ]);

    record([
        ("signature", is_bytes(b"\x89PNG\r\n\x1A\n")),
        ("ihdr", chunk(ihdr_tag, ihdr_data)),
        (
            "chunks",
            Format::Repeat(Box::new(chunk(other_tag.clone(), any_bytes()))),
        ),
        (
            "idat",
            Format::Repeat1(Box::new(chunk(idat_tag, idat_data))),
        ),
        (
            "more-chunks",
            Format::Repeat(Box::new(chunk(other_tag.clone(), any_bytes()))),
        ),
        ("iend", chunk(iend_tag, iend_data)),
    ])
}

/// TIFF Image file header
///
/// - [TIFF 6.0 Specification, Section 4.5](https://developer.adobe.com/content/dam/udp/en/open/standards/tiff/TIFF6.pdf#page=13)
/// - [Exif Version 2.32, Section 4.5.2](https://www.cipa.jp/std/documents/e/DC-X008-Translation-2019-E.pdf#page=23)
fn tiff_format() -> Format {
    // Image file directory field
    fn ifd_field(is_be: bool) -> Format {
        record([
            ("tag", if is_be { u16be() } else { u16le() }),
            ("type", if is_be { u16be() } else { u16le() }),
            ("length", if is_be { u32be() } else { u32le() }),
            ("offset-or-data", if is_be { u32be() } else { u32le() }),
            // TODO: Offset from start of the TIFF header for values longer than 4 bytes
        ])
    }

    // Image file directory
    fn ifd(is_be: bool) -> Format {
        record([
            ("num-fields", if is_be { u16be() } else { u16le() }),
            ("fields", repeat_count(Expr::Var(0), ifd_field(is_be))),
            ("next-ifd-offset", if is_be { u32be() } else { u32le() }),
            // TODO: Offset from start of the TIFF header (i.e. `offset + 2 + num-fields * 12`)
            // TODO: Recursive call to `ifd(is_be)`
            ("next-ifd", any_bytes()),
        ])
    }

    record([
        (
            "is-big-endian",
            alts([
                Format::Map(Func::Expr(Expr::Bool(false)), Box::new(is_bytes(b"II"))),
                Format::Map(Func::Expr(Expr::Bool(true)), Box::new(is_bytes(b"MM"))),
            ]),
        ),
        (
            "magic",
            Format::Match(
                Expr::Var(0),
                vec![
                    (Pattern::Bool(false), u16le()), // 42
                    (Pattern::Bool(true), u16be()),  // 42
                ],
            ),
        ),
        (
            "offset",
            Format::Match(
                Expr::Var(1),
                vec![
                    (Pattern::Bool(false), u32le()),
                    (Pattern::Bool(true), u32be()),
                ],
            ),
        ),
        (
            "ifd",
            Format::WithRelativeOffset(
                // TODO: Offset from start of the TIFF header
                Expr::Sub(Box::new(Expr::Var(0)), Box::new(Expr::U32(8))),
                Box::new(Format::Match(
                    Expr::Var(2),
                    vec![
                        (Pattern::Bool(false), ifd(false)),
                        (Pattern::Bool(true), ifd(true)),
                    ],
                )),
            ),
        ),
    ])
}

/// JPEG File Interchange Format
///
/// - [JPEG File Interchange Format Version 1.02](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
/// - [ITU T.81 | ISO IEC 10918-1](https://www.w3.org/Graphics/JPEG/itu-t81.pdf)
fn jpeg_format() -> Format {
    fn marker(id: u8) -> Format {
        Format::Map(
            Func::TupleProj(1),
            Box::new(Format::Tuple(vec![is_byte(0xFF), is_byte(id)])),
        )
    }

    fn marker_segment(id: u8, data: Format) -> Format {
        record([
            ("marker", marker(id)),
            ("length", u16be()),
            (
                "data",
                Format::Slice(
                    Expr::Sub(
                        Box::new(Expr::Var(0)), // length
                        Box::new(Expr::U16(2)),
                    ),
                    Box::new(data),
                ),
            ),
        ])
    }

    // SOF_n: Frame header (See ITU T.81 Section B.2.2)
    let sof_data = record([
        ("sample-precision", u8()),
        ("num-lines", u16be()),
        ("num-samples-per-line", u16be()),
        ("num-image-components", u8()),
        (
            "image-components",
            repeat_count(
                Expr::Var(0), // num-image-components
                record([
                    ("id", u8()),
                    ("sampling-factor", u8()), // { horizontal <- u4, vertical <- u4 }
                    ("quantization-table-id", u8()),
                ]),
            ),
        ),
    ]);

    // DHT: Define Huffman table (See ITU T.81 Section B.2.4.2)
    let dht_data = record([
        // class <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("class-table-id", u8()),
        ("num-codes", repeat_count(Expr::U8(16), u8())),
        ("values", any_bytes()), // List.map num-codes (\n => repeat-count n u8);
    ]);

    // DAC: Define arithmetic conditioning table (See ITU T.81 Section B.2.4.3)
    let dac_data = record([
        // class <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("class-table-id", u8()),
        ("value", u8()),
    ]);

    // SOS: Scan header (See ITU T.81 Section B.2.3)
    let sos_data = record([
        ("num-image-components", u8()), // 1 |..| 4
        (
            "image-components",
            repeat_count(
                Expr::Var(0), // num-image-components
                record([
                    ("component-selector", u8()), // ???
                    // dc-entropy-coding-table-id <- u4;
                    // ac-entropy-coding-table-id <- u4;
                    ("entropy-coding-table-ids", u8()),
                ]),
            ),
        ),
        ("start-spectral-selection", u8()),   // ???
        ("end-spectral-selection", u8()),     // ???
        ("approximation-bit-position", u8()), // { high <- u4, low <- u4 }
    ]);

    // DQT: Define quantization table (See ITU T.81 Section B.2.4.1)
    let dqt_data = record([
        // precision <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("precision-table-id", u8()),
        // elements <- match precision {
        //   0 => repeat-count 64 u8,
        //   1 => repeat-count 64 u16be,
        // };
        ("elements", any_bytes()),
    ]);

    // DNL: Define number of lines (See ITU T.81 Section B.2.5)
    let dnl_data = record([("num-lines", u16be())]);

    // DRI: Define restart interval (See ITU T.81 Section B.2.4.4)
    let dri_data = record([("restart-interval", u16be())]);

    // DHP: Define hierarchial progression (See ITU T.81 Section B.3.2)
    // NOTE: Same as SOF except for quantization-table-id
    let dhp_data = record([
        ("sample-precision", u8()),
        ("num-lines", u16be()),
        ("num-samples-per-line", u16be()),
        ("num-image-components", u8()),
        (
            "image-components",
            repeat_count(
                Expr::Var(0), // num-image-components
                record([
                    ("id", u8()),
                    ("sampling-factor", u8()), // { horizontal <- u4, vertical <- u4 }
                    ("quantization-table-id", is_byte(0)),
                ]),
            ),
        ),
    ]);

    // EXP: Expand reference components (See ITU T.81 Section B.3.3)
    let exp_data = record([
        // expand-horizontal <- u4 // 0 | 1;
        // expand-vertical <- u4 // 0 | 1;
        ("expand-horizontal-vertical", u8()),
    ]);

    // APP0: Application segment 0 (JFIF)
    let app0_jfif = record([
        ("identifier", is_bytes(b"JFIF\0")),
        ("version-major", u8()),
        ("version-minor", u8()),
        ("density-units", u8()), // 0 | 1 | 2
        ("density-x", u16be()),  // != 0
        ("density-y", u16be()),  // != 0
        ("thumbnail-width", u8()),
        ("thumbnail-height", u8()),
        (
            "thumbnail-pixels",
            repeat_count(
                Expr::Var(0), // thumbnail-height
                repeat_count(
                    Expr::Var(1), // thumbnail-width
                    record([("r", u8()), ("g", u8()), ("b", u8())]),
                ),
            ),
        ),
    ]);

    let app0_data = alts([
        app0_jfif,
        // FIXME there are other APP0 formats
        // see https://exiftool.org/TagNames/JPEG.html
    ]);

    // APP1: Application segment 1 (EXIF)
    //
    // - [Exif Version 2.32, Section 4.5.4](https://www.cipa.jp/std/documents/e/DC-X008-Translation-2019-E.pdf#page=24)
    let app1_exif = record([
        ("identifier", is_bytes(b"Exif\0")),
        ("padding", is_byte(0x00)),
        ("exif", tiff_format()),
    ]);

    // APP1: Application segment 1 (XMP)
    let app1_xmp = record([
        ("identifier", is_bytes(b"http://ns.adobe.com/xap/1.0/\0")),
        ("xmp", any_bytes()),
    ]);

    let app1_data = alts([
        app1_exif,
        app1_xmp,
        // FIXME there are other APP1 formats
        // see https://exiftool.org/TagNames/JPEG.html
    ]);

    let sof0 = marker_segment(0xC0, sof_data.clone()); // Start of frame (baseline jpeg)
    let sof1 = marker_segment(0xC1, sof_data.clone()); // Start of frame (extended sequential, huffman)
    let sof2 = marker_segment(0xC2, sof_data.clone()); // Start of frame (progressive, huffman)
    let sof3 = marker_segment(0xC3, sof_data.clone()); // Start of frame (lossless, huffman)
    let dht = marker_segment(0xC4, dht_data.clone()); // Define Huffman Table
    let sof5 = marker_segment(0xC5, sof_data.clone()); // Start of frame (differential sequential, huffman)
    let sof6 = marker_segment(0xC6, sof_data.clone()); // Start of frame (differential progressive, huffman)
    let sof7 = marker_segment(0xC7, sof_data.clone()); // Start of frame (differential lossless, huffman)
    let _jpeg = marker_segment(0xC8, any_bytes()); // Reserved for JPEG extension
    let sof9 = marker_segment(0xC9, sof_data.clone()); // Start of frame (extended sequential, arithmetic)
    let sof10 = marker_segment(0xCA, sof_data.clone()); // Start of frame (progressive, arithmetic)
    let sof11 = marker_segment(0xCB, sof_data.clone()); // Start of frame (lossless, arithmetic)
    let dac = marker_segment(0xCC, dac_data.clone()); // Define arithmetic coding conditioning
    let sof13 = marker_segment(0xCD, sof_data.clone()); // Start of frame (differential sequential, arithmetic)
    let sof14 = marker_segment(0xCE, sof_data.clone()); // Start of frame (differential progressive, arithmetic)
    let sof15 = marker_segment(0xCF, sof_data.clone()); // Start of frame (differential lossless, arithmetic)
    let rst0 = marker(0xD0); // Restart marker 0
    let rst1 = marker(0xD1); // Restart marker 1
    let rst2 = marker(0xD2); // Restart marker 2
    let rst3 = marker(0xD3); // Restart marker 3
    let rst4 = marker(0xD4); // Restart marker 4
    let rst5 = marker(0xD5); // Restart marker 5
    let rst6 = marker(0xD6); // Restart marker 6
    let rst7 = marker(0xD7); // Restart marker 7
    let soi = marker(0xD8); // Start of image
    let eoi = marker(0xD9); // End of of image
    let sos = marker_segment(0xDA, sos_data.clone()); // Start of scan
    let dqt = marker_segment(0xDB, dqt_data.clone()); // Define quantization table
    let dnl = marker_segment(0xDC, dnl_data.clone()); // Define number of lines
    let dri = marker_segment(0xDD, dri_data.clone()); // Define restart interval
    let _dhp = marker_segment(0xDE, dhp_data.clone()); // Define hierarchical progression
    let _exp = marker_segment(0xDF, exp_data.clone()); // Expand reference components
    let app0 = marker_segment(0xE0, app0_data.clone()); // Application segment 0 (JFIF/JFXX/AVI1/...)
    let app1 = marker_segment(0xE1, app1_data.clone()); // Application segment 1 (EXIF/XMP/XAP/...)
    let app2 = marker_segment(0xE2, any_bytes()); // Application segment 2 (FlashPix/ICC/...)
    let app3 = marker_segment(0xE3, any_bytes()); // Application segment 3 (Kodak/...)
    let app4 = marker_segment(0xE4, any_bytes()); // Application segment 4 (FlashPix/...)
    let app5 = marker_segment(0xE5, any_bytes()); // Application segment 5 (Ricoh/...)
    let app6 = marker_segment(0xE6, any_bytes()); // Application segment 6 (GoPro/...)
    let app7 = marker_segment(0xE7, any_bytes()); // Application segment 7 (Pentax/Qualcomm/...)
    let app8 = marker_segment(0xE8, any_bytes()); // Application segment 8 (Spiff/...)
    let app9 = marker_segment(0xE9, any_bytes()); // Application segment 9 (MediaJukebox/...)
    let app10 = marker_segment(0xEA, any_bytes()); // Application segment 10 (PhotoStudio)
    let app11 = marker_segment(0xEB, any_bytes()); // Application segment 11 (HDR)
    let app12 = marker_segment(0xEC, any_bytes()); // Application segment 12 (PictureInfo/Ducky)
    let app13 = marker_segment(0xED, any_bytes()); // Application segment 13 (PhotoShop/Adobe_CM)
    let app14 = marker_segment(0xEE, any_bytes()); // Application segment 14 (Adobe)
    let app15 = marker_segment(0xEF, any_bytes()); // Application segment 15 (GraphicConverter)
    let com = marker_segment(0xFE, any_bytes()); // Extension data (comment)

    let table_or_misc = alts([
        dqt.clone(), // Define quantization table
        dht.clone(), // Define Huffman Table
        dac.clone(), // Define arithmetic coding conditioning
        dri.clone(), // Define restart interval
        app0.clone(),
        app1.clone(),
        app2.clone(),
        app3.clone(),
        app4.clone(),
        app5.clone(),
        app6.clone(),
        app7.clone(),
        app8.clone(),
        app9.clone(),
        app10.clone(),
        app11.clone(),
        app12.clone(),
        app13.clone(),
        app14.clone(),
        app15.clone(),
        com.clone(), // Comment
    ]);

    let frame_header = alts([
        sof0.clone(),
        sof1.clone(),
        sof2.clone(),
        sof3.clone(),
        sof5.clone(),
        sof6.clone(),
        sof7.clone(),
        sof9.clone(),
        sof10.clone(),
        sof11.clone(),
        sof13.clone(),
        sof14.clone(),
        sof15.clone(),
    ]);

    // MCU: Minimum coded unit
    let mcu = alts([
        not_byte(0xFF),
        Format::Map(
            Func::Expr(Expr::U8(0xFF)),
            Box::new(Format::Tuple(vec![is_byte(0xFF), is_byte(0x00)])),
        ),
    ]);

    // A series of entropy coded segments separated by restart markers
    let scan_data = Format::Map(
        Func::Stream,
        Box::new(repeat(alts([
            // FIXME: Extract into separate ECS repetition
            mcu, // TODO: repeat(mcu),
            // FIXME: Restart markers should cycle in order from rst0-rst7
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst0)),
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst1)),
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst2)),
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst3)),
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst4)),
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst5)),
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst6)),
            Format::Map(Func::Expr(Expr::UNIT), Box::new(rst7)),
        ]))),
    );

    let scan = record([
        ("segments", repeat(table_or_misc.clone())),
        ("sos", sos.clone()),
        ("data", scan_data.clone()),
    ]);

    let frame = record([
        ("app0", alts([app0.clone(), app1.clone()])),
        ("segments", repeat(table_or_misc.clone())),
        ("header", frame_header.clone()),
        ("scan", scan.clone()),
        ("dnl", optional(dnl.clone())),
        ("scans", repeat(scan)),
    ]);

    let jpeg = record([
        ("soi", soi.clone()),
        ("frame", frame.clone()),
        ("eoi", eoi.clone()),
    ]);

    jpeg
}

#[derive(Copy, Clone, ValueEnum)]
enum OutputFormat {
    /// Use the debug formatter
    Debug,
    /// Serialize to JSON
    Json,
    /// Display as a human-readable tree
    Tree,
}

/// Decode a binary file
#[derive(Parser)]
struct Args {
    /// How decoded values are rendered
    #[arg(long, default_value = "tree")]
    output: OutputFormat,
    /// The binary file to decode
    #[arg(default_value = "test.jpg")]
    filename: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();
    let input = fs::read(args.filename)?;

    let format = Format::Map(
        Func::RecordProj("data".to_string()),
        Box::new(record([
            ("data", alts([jpeg_format(), png_format(), riff_format()])),
            ("end", Format::EndOfInput),
        ])),
    );
    let decoder = Decoder::compile(&format, &Next::Empty)?;

    let mut stack = Vec::new();
    let (val, _) = decoder.parse(&mut stack, &input).ok_or("parse failure")?;

    match args.output {
        OutputFormat::Debug => println!("{val:?}"),
        OutputFormat::Json => serde_json::to_writer(std::io::stdout(), &val).unwrap(),
        OutputFormat::Tree => render_tree::print_value(&val),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_empty() {
        let f = Format::Empty;
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[0x00], Value::UNIT);
    }

    #[test]
    fn compile_byte_is() {
        let f = is_byte(0x00);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0));
        accepts(&d, &[0x00, 0xFF], &[0xFF], Value::U8(0));
        rejects(&d, &[0xFF]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_byte_not() {
        let f = not_byte(0x00);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        accepts(&d, &[0xFF, 0x00], &[0x00], Value::U8(0xFF));
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt() {
        let f = alts([]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_alt_byte() {
        let f = alts([is_byte(0x00), is_byte(0xFF)]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0x00));
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_ambiguous() {
        let f = alts([is_byte(0x00), is_byte(0x00)]);
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_alt_fail() {
        let f = alts([Format::Fail, Format::Fail]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_end_of_input() {
        let f = alts([Format::EndOfInput, Format::EndOfInput]);
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_alt_empty() {
        let f = alts([Format::Empty, Format::Empty]);
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_alt_fail_end_of_input() {
        let f = alts([Format::Fail, Format::EndOfInput]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
    }

    #[test]
    fn compile_alt_end_of_input_or_byte() {
        let f = alts([Format::EndOfInput, is_byte(0x00)]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[], Value::U8(0x00));
        accepts(&d, &[0x00, 0x00], &[0x00], Value::U8(0x00));
        rejects(&d, &[0x11]);
    }

    #[test]
    fn compile_alt_opt() {
        let f = alts([Format::Empty, is_byte(0x00)]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0x00));
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0xFF], &[0xFF], Value::UNIT);
    }

    #[test]
    fn compile_alt_opt_next() {
        let f = Format::Cat(
            Box::new(alts([Format::Empty, is_byte(0x00)])),
            Box::new(is_byte(0xFF)),
        );
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0), Value::U8(0xFF)]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![Value::UNIT, Value::U8(0xFF)]),
        );
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_opt_opt() {
        let f = Format::Cat(
            Box::new(alts([Format::Empty, is_byte(0x00)])),
            Box::new(alts([Format::Empty, is_byte(0xFF)])),
        );
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0), Value::U8(0xFF)]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![Value::U8(0), Value::UNIT]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![Value::UNIT, Value::U8(0xFF)]),
        );
        accepts(&d, &[], &[], Value::Tuple(vec![Value::UNIT, Value::UNIT]));
        accepts(&d, &[], &[], Value::Tuple(vec![Value::UNIT, Value::UNIT]));
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![Value::UNIT, Value::UNIT]),
        );
    }

    #[test]
    fn compile_alt_opt_ambiguous() {
        let f = Format::Cat(
            Box::new(alts([Format::Empty, is_byte(0x00)])),
            Box::new(alts([Format::Empty, is_byte(0x00)])),
        );
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    #[ignore = "this test takes over a minute to run"]
    fn compile_alt_opt_ambiguous_slow() {
        let alt = alts([
            is_byte(0x00),
            is_byte(0x01),
            is_byte(0x02),
            is_byte(0x03),
            is_byte(0x04),
            is_byte(0x05),
            is_byte(0x06),
            is_byte(0x07),
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
        let f = alts([rec.clone(), rec.clone()]);
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    #[ignore = "this test takes over a minute to run"]
    fn compile_repeat_alt_repeat1_slow() {
        let f = repeat(alts([repeat1(is_byte(0x00)), is_byte(0x01), is_byte(0x02)]));
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
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
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_cat_repeat() {
        let f = Format::Cat(
            Box::new(repeat(is_byte(0x00))),
            Box::new(repeat(is_byte(0xFF))),
        );
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
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
        let f = Format::Cat(Box::new(is_byte(0x00)), Box::new(Format::EndOfInput));
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
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
        let f = Format::Cat(
            Box::new(repeat(is_byte(0x00))),
            Box::new(Format::EndOfInput),
        );
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
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
        let f = Format::Cat(
            Box::new(repeat(is_byte(0x00))),
            Box::new(repeat(is_byte(0x00))),
        );
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_repeat_fields() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x7F))),
        ]);
        assert!(Decoder::compile(&f, &Next::Empty).is_ok());
    }

    #[test]
    fn compile_repeat_fields_ambiguous() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x00))),
        ]);
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_repeat_fields_okay() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            (
                "second-and-third",
                alts([
                    Format::Empty,
                    record([
                        (
                            "second",
                            Format::Cat(Box::new(is_byte(0xFF)), Box::new(repeat(is_byte(0xFF)))),
                        ),
                        ("third", repeat(is_byte(0x00))),
                    ]),
                ]),
            ),
        ]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Record(vec![
                ("first".to_string(), Value::Seq(vec![])),
                ("second-and-third".to_string(), Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Record(vec![
                ("first".to_string(), Value::Seq(vec![Value::U8(0x00)])),
                ("second-and-third".to_string(), Value::UNIT),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Record(vec![
                ("first".to_string(), Value::Seq(vec![Value::U8(0x00)])),
                (
                    "second-and-third".to_string(),
                    Value::Record(vec![
                        (
                            "second".to_string(),
                            Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                        ),
                        ("third".to_string(), Value::Seq(vec![])),
                    ]),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[],
            Value::Record(vec![
                ("first".to_string(), Value::Seq(vec![Value::U8(0x00)])),
                (
                    "second-and-third".to_string(),
                    Value::Record(vec![
                        (
                            "second".to_string(),
                            Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                        ),
                        ("third".to_string(), Value::Seq(vec![Value::U8(0x00)])),
                    ]),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0x7F],
            &[0x7F],
            Value::Record(vec![
                ("first".to_string(), Value::Seq(vec![Value::U8(0x00)])),
                ("second-and-third".to_string(), Value::UNIT),
            ]),
        );
    }

    #[test]
    fn compile_repeat1() {
        let f = repeat1(is_byte(0x00));
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
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
