use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use serde::Serialize;

use crate::byte_set::ByteSet;

mod byte_set;

#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum Value {
    Unit,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Pair(Box<Value>, Box<Value>),
    Seq(Vec<Value>),
    Record(Vec<(String, Value)>),
}

#[derive(Clone, Debug)]
enum Expr {
    Const(Value),
    Var(usize),
    Sub(Box<Expr>, Box<Expr>),
    Pair(Box<Expr>, Box<Expr>),
    Seq(Vec<Expr>),
    Record(Vec<(String, Expr)>),
}

#[derive(Clone, Debug)]
enum Func {
    Expr(Expr),
    Fst,
    Snd,
    U16Be,
    U16Le,
    U32Be,
    U32Le,
    Stream,
}

/// Binary format descriptions
///
/// A subset of binary formats can be modelled as [regular expressions]:
///
/// ```text
/// ⟦ Fail ⟧                    = ∅                     empty set
/// ⟦ Empty ⟧                   = ε                     empty byte string
/// ⟦ Byte(Any) ⟧               = .                     any byte
/// ⟦ Byte(Is(b)) ⟧             = b                     literal byte
/// ⟦ Alt(f0, f1) ⟧             = ⟦ f0 ⟧ | ⟦ f0 ⟧       alternation
/// ⟦ Cat(f0, f1) ⟧             = ⟦ f0 ⟧ ⟦ f0 ⟧         concatenation
/// ⟦ Tuple([]) ⟧               = ε                     empty byte string
/// ⟦ Tuple([f0, ..., fn]) ⟧    = ⟦ f0 ⟧ ... ⟦ fn ⟧     concatenation
/// ⟦ Repeat(f) ⟧               = ⟦ f0 ⟧*               Kleene star
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
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Matches the union of the byte strings matched by the two formats
    Alt(Box<Format>, Box<Format>),
    /// Matches the union of the byte strings matched by all the formats
    Switch(Vec<Format>),
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
    /// Repeat a format an exact number of times
    RepeatCount(Expr, Box<Format>),
    /// Restrict a format to a sub-stream of a given number of bytes
    Slice(Expr, Box<Format>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(Expr, Box<Format>),
    /// Transform a decoded value with a function
    Map(Func, Box<Format>),
    /// Conditional format
    If(Expr, Box<Format>, Box<Format>),
}

#[derive(Clone, Debug)]
struct ByteSwitch {
    branches: Vec<(ByteSet, Switch)>,
}

#[derive(Clone, Debug)]
struct Switch(Option<usize>, ByteSwitch);

enum Cond {
    Expr(Expr),
    Switch(Switch),
}

/// Decoders with a fixed amount of lookahead
enum Decoder {
    Fail,
    Empty,
    Byte(ByteSet),
    If(Cond, Box<Decoder>, Box<Decoder>),
    Switch(Switch, Vec<Decoder>),
    Cat(Box<Decoder>, Box<Decoder>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(Switch, Box<Decoder>),
    RepeatCount(Expr, Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    WithRelativeOffset(Expr, Box<Decoder>),
    Map(Func, Box<Decoder>),
}

impl Expr {
    fn eval(&self, stack: &[Value]) -> Value {
        match self {
            Expr::Const(v) => v.clone(),
            Expr::Var(index) => stack[stack.len() - index - 1].clone(),
            Expr::Sub(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_sub(x, y).unwrap()),
                (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_sub(x, y).unwrap()),
                (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_sub(x, y).unwrap()),
                (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
            },
            Expr::Pair(expr0, expr1) => {
                Value::Pair(Box::new(expr0.eval(stack)), Box::new(expr1.eval(stack)))
            }
            Expr::Seq(exprs) => Value::Seq(exprs.iter().map(|expr| expr.eval(stack)).collect()),
            Expr::Record(fields) => Value::Record(
                fields
                    .iter()
                    .map(|(label, expr)| (label.clone(), expr.eval(stack)))
                    .collect(),
            ),
        }
    }

    fn eval_bool(&self, stack: &[Value]) -> bool {
        match self.eval(stack) {
            Value::Bool(b) => b,
            _ => panic!("value is not bool"),
        }
    }

    fn eval_usize(&self, stack: &[Value]) -> usize {
        match self.eval(stack) {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::U32(n) => usize::try_from(n).unwrap(),
            Value::Unit | Value::Bool(_) | Value::Pair(_, _) | Value::Seq(_) | Value::Record(_) => {
                panic!("value is not number")
            }
        }
    }
}

impl Func {
    fn eval(&self, arg: Value) -> Value {
        match self {
            Func::Expr(e) => e.eval(&[]),
            Func::Fst => match arg {
                Value::Pair(fst, _snd) => *fst,
                _ => panic!("Fst: expected (_, _)"),
            },
            Func::Snd => match arg {
                Value::Pair(_fst, snd) => *snd,
                _ => panic!("Snd: expected (_, _)"),
            },
            Func::U16Be => match arg {
                Value::Pair(fst, snd) => match (fst.as_ref(), snd.as_ref()) {
                    (Value::U8(hi), Value::U8(lo)) => Value::U16(u16::from_be_bytes([*hi, *lo])),
                    (_, _) => panic!("expected (U8, U8)"),
                },
                _ => panic!("U16Be: expected (_, _)"),
            },
            Func::U16Le => match arg {
                Value::Pair(fst, snd) => match (fst.as_ref(), snd.as_ref()) {
                    (Value::U8(lo), Value::U8(hi)) => Value::U16(u16::from_le_bytes([*lo, *hi])),
                    (_, _) => panic!("expected (U8, U8)"),
                },
                _ => panic!("U16Be: expected (_, _)"),
            },
            Func::U32Be => match arg {
                Value::Seq(vs) => match vs.as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                        Value::U32(u32::from_be_bytes([*a, *b, *c, *d]))
                    }
                    _ => panic!("expected [U8, U8, U8, U8]"),
                },
                _ => panic!("U32Be: expected [_, _, _, _]"),
            },
            Func::U32Le => match arg {
                Value::Seq(vs) => match vs.as_slice() {
                    [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                        Value::U32(u32::from_le_bytes([*a, *b, *c, *d]))
                    }
                    _ => panic!("expected [U8, U8, U8, U8]"),
                },
                _ => panic!("U32Be: expected [_, _, _, _]"),
            },
            Func::Stream => match arg {
                Value::Seq(vs) => {
                    // FIXME could also condense nested sequences
                    Value::Seq(vs.into_iter().filter(|v| *v != Value::Unit).collect())
                }
                _ => panic!("Stream: expected Seq"),
            },
        }
    }
}

impl Format {
    fn nullable(&self) -> bool {
        match self {
            Format::Fail => false,
            Format::Empty => true,
            Format::Byte(_) => false,
            Format::Alt(a, b) => a.nullable() || b.nullable(),
            Format::Switch(branches) => branches.iter().any(|f| f.nullable()),
            Format::Cat(a, b) => a.nullable() && b.nullable(),
            Format::Tuple(fields) => fields.iter().all(|f| f.nullable()),
            Format::Record(fields) => fields.iter().all(|(_, f)| f.nullable()),
            Format::Repeat(_a) => true,
            Format::RepeatCount(_expr, _a) => true,
            Format::Slice(_expr, _a) => true,
            Format::Map(_f, a) => a.nullable(),
            Format::If(_expr, a, b) => a.nullable() || b.nullable(),
            Format::WithRelativeOffset(_, _) => true,
        }
    }
}

impl ByteSwitch {
    fn empty() -> ByteSwitch {
        let branches = Vec::new();
        ByteSwitch { branches }
    }

    fn single(bs: ByteSet, s: Switch) -> ByteSwitch {
        let branches = vec![(bs, s)];
        ByteSwitch { branches }
    }

    fn insert(self, mut bs: ByteSet, s: &Switch) -> Option<ByteSwitch> {
        let mut branches = Vec::new();
        for (bs0, s0) in self.branches {
            let bs_both = bs0.intersection(&bs);
            if !bs_both.is_empty() {
                let bs_old = bs0.difference(&bs);
                if !bs_old.is_empty() {
                    branches.push((bs_old, s0.clone()));
                }
                let v = Switch::union(s0, &s)?;
                branches.push((bs_both, v));
                bs = bs.difference(&bs0);
            } else {
                branches.push((bs0, s0));
            }
        }
        if !bs.is_empty() {
            branches.push((bs, s.clone()));
        }
        Some(ByteSwitch { branches })
    }

    fn union(a: ByteSwitch, b: &ByteSwitch) -> Option<ByteSwitch> {
        let mut c = a;
        for (bs, v) in &b.branches {
            c = c.insert(bs.clone(), v)?;
        }
        Some(c)
    }
}

#[derive(Debug)]
enum Next<'a> {
    Empty,
    Cat(&'a Format, &'a Next<'a>),
    Tuple(&'a [Format], &'a Next<'a>),
    Record(&'a [(String, Format)], &'a Next<'a>),
}

impl Switch {
    fn reject() -> Switch {
        Switch(None, ByteSwitch::empty())
    }

    fn accept(index: usize) -> Switch {
        Switch(Some(index), ByteSwitch::empty())
    }

    fn union(sa: Switch, sb: &Switch) -> Option<Switch> {
        match (sa, sb) {
            (Switch(a, sa), Switch(b, sb)) => {
                let c = match (a, b) {
                    (None, None) => None,
                    (Some(index), None) => Some(index),
                    (None, Some(index)) => Some(*index),
                    (Some(a), Some(b)) if a == *b => Some(a),
                    (Some(_), Some(_)) => return None,
                };
                Some(Switch(c, ByteSwitch::union(sa, &sb)?))
            }
        }
    }

    fn from_next(index: usize, depth: usize, next: &Next) -> Switch {
        match next {
            Next::Empty => Switch::accept(index),
            Next::Cat(f, next) => Switch::from(index, depth, f, next),
            Next::Tuple(fs, next) => match fs.split_first() {
                None => Switch::from_next(index, depth, next),
                Some((f, fs)) => Switch::from(index, depth, &f, &Next::Tuple(fs, next)),
            },
            Next::Record(fs, next) => match fs.split_first() {
                None => Switch::from_next(index, depth, next),
                Some(((_n, f), fs)) => Switch::from(index, depth, &f, &Next::Record(fs, next)),
            },
        }
    }

    pub fn from(index: usize, depth: usize, f: &Format, next: &Next) -> Switch {
        match f {
            Format::Fail => Switch::reject(),
            Format::Empty => Switch::from_next(index, depth, next),
            Format::Byte(bs) => Switch(
                None,
                ByteSwitch::single(
                    bs.clone(),
                    if depth > 0 {
                        Switch::from_next(index, depth - 1, next)
                    } else {
                        Switch::accept(index)
                    },
                ),
            ),
            Format::Alt(a, b) => {
                let sa = Switch::from(index, depth, a, next);
                let sb = Switch::from(index, depth, b, next);
                Switch::union(sa, &sb).unwrap()
            }
            Format::Switch(branches) => {
                let mut switch = Switch::reject();
                for f in branches {
                    let s = Switch::from(index, depth, f, next);
                    switch = Switch::union(switch, &s).unwrap();
                }
                switch
            }
            Format::Cat(a, b) => match **b {
                Format::Empty => Switch::from(index, depth, a, next),
                _ => Switch::from(index, depth, a, &Next::Cat(b, next)),
            },
            Format::Tuple(fields) => match fields.split_first() {
                None => Switch::from_next(index, depth, next),
                Some((a, fields)) => Switch::from(index, depth, a, &Next::Tuple(&fields, next)),
            },
            Format::Record(fields) => match fields.split_first() {
                None => Switch::from_next(index, depth, next),
                Some(((_, a), fields)) => {
                    Switch::from(index, depth, a, &Next::Record(&fields, next))
                }
            },
            Format::Repeat(a) => Switch::from(
                index,
                depth,
                &Format::Alt(
                    Box::new(Format::Empty),
                    Box::new(Format::Cat(a.clone(), Box::new(Format::Repeat(a.clone())))),
                ),
                next,
            ),
            Format::RepeatCount(_expr, _a) => {
                Switch::accept(index) // FIXME
            }
            Format::Slice(_expr, _a) => {
                Switch::accept(index) // FIXME
            }
            Format::WithRelativeOffset(_expr, _a) => {
                Switch::accept(index) // FIXME
            }
            Format::Map(_f, a) => Switch::from(index, depth, a, next),
            Format::If(_expr, a, b) => {
                Switch::from(index, depth, &Format::Alt(a.clone(), b.clone()), next)
            }
        }
    }

    fn matches(&self, input: &[u8]) -> Option<usize> {
        match self {
            Switch(accept, ByteSwitch { branches }) => match input.split_first() {
                None => *accept,
                Some((b, input)) => {
                    for (bs, s) in branches {
                        if bs.contains(*b) {
                            return s.matches(input);
                        }
                    }
                    *accept
                }
            },
        }
    }

    fn accepts(branches: &[(usize, Format)]) -> Option<usize> {
        match branches.split_first() {
            None => None,
            Some(((index, _), branches)) => {
                if branches.iter().all(|(i, _)| i == index) {
                    Some(*index)
                } else {
                    None
                }
            }
        }
    }

    fn build(branches: &[&Format], next: &Next) -> Option<Switch> {
        const MAX_DEPTH: usize = 2;
        let mut switch = Switch::reject();
        for i in 0..branches.len() {
            let res = Switch::from(i, MAX_DEPTH, &branches[i], next);
            switch = Switch::union(switch, &res)?;
        }
        Some(switch)
    }
}

impl Cond {
    fn eval(&self, stack: &[Value], input: &[u8]) -> bool {
        match self {
            Cond::Expr(expr) => expr.eval_bool(stack),
            Cond::Switch(switch) => switch.matches(input) == Some(0),
        }
    }
}

impl Decoder {
    pub fn compile(f: &Format, next: &Next) -> Result<Decoder, String> {
        match f {
            Format::Fail => Ok(Decoder::Fail),
            Format::Empty => Ok(Decoder::Empty),
            Format::Byte(bs) => Ok(Decoder::Byte(bs.clone())),
            Format::Alt(a, b) => {
                let da = Box::new(Decoder::compile(a, next)?);
                let db = Box::new(Decoder::compile(b, next)?);
                if let Some(switch) = Switch::build(&[&a, &b], next) {
                    Ok(Decoder::If(Cond::Switch(switch), da, db))
                } else {
                    Err(format!("cannot build switch for {:?}", f))
                }
            }
            Format::Switch(branches) => {
                let mut ds = Vec::new();
                let mut fs = Vec::new();
                for f in branches {
                    let d = Decoder::compile(f, next)?;
                    ds.push(d);
                    fs.push(f);
                }
                if let Some(switch) = Switch::build(&fs, next) {
                    Ok(Decoder::Switch(switch, ds))
                } else {
                    Err(format!("cannot build switch for {:?}", f))
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
                if a.nullable() {
                    return Err("cannot repeat nullable format".to_string());
                }
                let astar = Format::Repeat(a.clone());
                let da = Box::new(Decoder::compile(a, &Next::Cat(&astar, next))?);
                if let Next::Empty = next {
                    let switch = Switch(
                        None,
                        ByteSwitch::single(ByteSet::full(), Switch(Some(0), ByteSwitch::empty())),
                    );
                    Ok(Decoder::While(switch, da))
                } else {
                    let fa = &Format::Cat(a.clone(), Box::new(astar));
                    let fb = &Format::Empty;
                    if let Some(switch) = Switch::build(&[fa, fb], next) {
                        Ok(Decoder::While(switch, da))
                    } else {
                        Err(format!("cannot build switch for {:?}", f))
                    }
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
            Format::If(expr, a, b) => {
                let da = Box::new(Decoder::compile(a, next)?);
                let db = Box::new(Decoder::compile(b, next)?);
                Ok(Decoder::If(Cond::Expr(expr.clone()), da, db))
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
            Decoder::Empty => Some((Value::Unit, input)),
            Decoder::Byte(bs) => {
                let (&b, input) = input.split_first()?;
                if bs.contains(b) {
                    Some((Value::U8(b), input))
                } else {
                    None
                }
            }
            Decoder::If(cond, a, b) => {
                if cond.eval(stack, input) {
                    a.parse(stack, input)
                } else {
                    b.parse(stack, input)
                }
            }
            Decoder::Switch(switch, branches) => {
                let index = switch.matches(input)?;
                branches[index].parse(stack, input)
            }
            Decoder::Cat(a, b) => {
                let (va, input) = a.parse(stack, input)?;
                let (vb, input) = b.parse(stack, input)?;
                Some((Value::Pair(Box::new(va), Box::new(vb)), input))
            }
            Decoder::Tuple(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for f in fields {
                    let (vf, next_input) = f.parse(stack, input)?;
                    input = next_input;
                    v.push(vf.clone());
                }
                Some((Value::Seq(v), input))
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
            Decoder::While(switch, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while switch.matches(input) == Some(0) {
                    let (va, next_input) = a.parse(stack, input)?;
                    input = next_input;
                    v.push(va);
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
                Some((f.eval(va), input))
            }
        }
    }
}

mod render_tree {
    use std::fmt;

    use crate::Value;

    const SEQ_PREVIEW_LEN: usize = 10;

    fn is_atomic_value(value: &Value) -> bool {
        match value {
            Value::Unit => true,
            Value::Bool(_) => true,
            Value::U8(_) => true,
            Value::U16(_) => true,
            Value::U32(_) => true,
            Value::Pair(_, _) => false,
            Value::Seq(vals) => vals.is_empty(),
            Value::Record(fields) => fields.is_empty(),
        }
    }

    pub fn print_value(gutter: &mut Vec<bool>, value: &Value) {
        match value {
            Value::Unit => print!("()"),
            Value::Bool(true) => print!("true"),
            Value::Bool(false) => print!("false"),
            Value::U8(i) => print!("{i}"),
            Value::U16(i) => print!("{i}"),
            Value::U32(i) => print!("{i}"),
            Value::Pair(val0, val1) => {
                print_field_value_continue(gutter, 0, val0);
                print_field_value_last(gutter, 1, val1);
            }
            Value::Seq(vals) if vals.is_empty() => print!("[]"),
            Value::Seq(vals) => {
                if vals.len() > SEQ_PREVIEW_LEN && vals.iter().all(is_atomic_value) {
                    let last_index = vals.len() - 1;
                    for (index, val) in vals[0..SEQ_PREVIEW_LEN].iter().enumerate() {
                        print_field_value_continue(gutter, index, val);
                    }
                    if SEQ_PREVIEW_LEN != last_index {
                        print_field_skipped(gutter);
                    }
                    print_field_value_last(gutter, last_index, &vals[last_index]);
                } else {
                    let last_index = vals.len() - 1;
                    for (index, val) in vals[..last_index].iter().enumerate() {
                        print_field_value_continue(gutter, index, val);
                    }
                    print_field_value_last(gutter, last_index, &vals[last_index]);
                }
            }
            Value::Record(vals) if vals.is_empty() => print!("{{}}"),
            Value::Record(fields) => {
                let last_index = fields.len() - 1;
                for (label, val) in &fields[..last_index] {
                    print_field_value_continue(gutter, label, val);
                }
                let (label, val) = &fields[last_index];
                print_field_value_last(gutter, label, val);
            }
        }
    }

    fn print_gutter(gutter: &[bool]) {
        for is_continue in gutter {
            if *is_continue {
                print!("│   ");
            } else {
                print!("    ");
            }
        }
    }

    fn print_field_value_continue(gutter: &mut Vec<bool>, label: impl fmt::Display, value: &Value) {
        print_gutter(gutter);
        print!("├── {label} :=");
        gutter.push(true);
        if is_atomic_value(value) {
            print!(" ");
            print_value(gutter, value);
            println!();
        } else {
            println!();
            print_value(gutter, value);
        }
        gutter.pop();
    }

    fn print_field_value_last(gutter: &mut Vec<bool>, label: impl fmt::Display, value: &Value) {
        print_gutter(gutter);
        print!("└── {label} :=");
        gutter.push(false);
        if is_atomic_value(value) {
            print!(" ");
            print_value(gutter, value);
            println!();
        } else {
            println!();
            print_value(gutter, value);
        }
        gutter.pop();
    }

    fn print_field_skipped(gutter: &[bool]) {
        print_gutter(gutter);
        println!("~");
    }
}

fn alts(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Switch(formats.into_iter().collect())
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
        Box::new(Format::Cat(Box::new(any_byte()), Box::new(any_byte()))),
    )
}

fn u16le() -> Format {
    Format::Map(
        Func::U16Le,
        Box::new(Format::Cat(Box::new(any_byte()), Box::new(any_byte()))),
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

fn png_format() -> Format {
    let chunk = record([
        ("length", u32be()), // FIXME < 2^31
        (
            "type", // FIXME ASCII
            Format::Tuple(vec![any_byte(), any_byte(), any_byte(), any_byte()]),
        ),
        (
            "data",
            Format::RepeatCount(Expr::Var(1), Box::new(any_byte())),
        ),
        ("crc", u32be()), // FIXME check this
    ]);

    record([
        ("signature", is_bytes(b"\x89PNG\r\n\x1A\n")),
        ("chunks", Format::Repeat(Box::new(chunk))),
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
                Format::Map(
                    Func::Expr(Expr::Const(Value::Bool(false))),
                    Box::new(is_bytes(b"II")),
                ),
                Format::Map(
                    Func::Expr(Expr::Const(Value::Bool(true))),
                    Box::new(is_bytes(b"MM")),
                ),
            ]),
        ),
        (
            "magic",
            Format::If(Expr::Var(0), Box::new(u16be()), Box::new(u16le())), // 42
        ),
        (
            "offset",
            Format::If(Expr::Var(1), Box::new(u32be()), Box::new(u32le())),
        ),
        (
            "ifd",
            Format::WithRelativeOffset(
                // TODO: Offset from start of the TIFF header
                Expr::Sub(Box::new(Expr::Var(0)), Box::new(Expr::Const(Value::U32(8)))),
                Box::new(Format::If(
                    Expr::Var(2),
                    Box::new(ifd(true)),
                    Box::new(ifd(false)),
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
            Func::Snd,
            Box::new(Format::Cat(Box::new(is_byte(0xFF)), Box::new(is_byte(id)))),
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
                        Box::new(Expr::Const(Value::U16(2))),
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
        ("num-codes", repeat_count(Expr::Const(Value::U8(16)), u8())),
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
            Func::Expr(Expr::Const(Value::U8(0xFF))),
            Box::new(Format::Cat(
                Box::new(is_byte(0xFF)),
                Box::new(is_byte(0x00)),
            )),
        ),
    ]);

    // A series of entropy coded segments separated by restart markers
    let scan_data = Format::Map(
        Func::Stream,
        Box::new(repeat(alts([
            // FIXME: Extract into separate ECS repetition
            mcu, // TODO: repeat(mcu),
            // FIXME: Restart markers should cycle in order from rst0-rst7
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst0)),
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst1)),
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst2)),
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst3)),
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst4)),
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst5)),
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst6)),
            Format::Map(Func::Expr(Expr::Const(Value::Unit)), Box::new(rst7)),
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
    Debug,
    Json,
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

    let format = alts([jpeg_format(), png_format()]);
    let decoder = Decoder::compile(&format, &Next::Empty)?;

    let mut stack = Vec::new();
    let (val, _) = decoder.parse(&mut stack, &input).ok_or("parse failure")?;

    match args.output {
        OutputFormat::Debug => println!("{val:?}"),
        OutputFormat::Json => serde_json::to_writer(std::io::stdout(), &val).unwrap(),
        OutputFormat::Tree => render_tree::print_value(&mut Vec::new(), &val),
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
        accepts(&d, &[], &[], Value::Unit);
        accepts(&d, &[0x00], &[0x00], Value::Unit);
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
    fn compile_alt_empty() {
        let f = alts([Format::Empty, Format::Empty]);
        assert!(Decoder::compile(&f, &Next::Empty).is_err());
    }

    #[test]
    fn compile_alt_opt() {
        let f = alts([Format::Empty, is_byte(0x00)]);
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0x00));
        accepts(&d, &[], &[], Value::Unit);
        accepts(&d, &[0xFF], &[0xFF], Value::Unit);
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
            Value::Pair(Box::new(Value::U8(0)), Box::new(Value::U8(0xFF))),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Pair(Box::new(Value::Unit), Box::new(Value::U8(0xFF))),
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
            Value::Pair(Box::new(Value::U8(0)), Box::new(Value::U8(0xFF))),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Pair(Box::new(Value::U8(0)), Box::new(Value::Unit)),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Pair(Box::new(Value::Unit), Box::new(Value::U8(0xFF))),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Pair(Box::new(Value::Unit), Box::new(Value::Unit)),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Pair(Box::new(Value::Unit), Box::new(Value::Unit)),
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
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Decoder::compile(&f, &Next::Empty).unwrap();
        accepts(&d, &[], &[], Value::Seq(vec![]));
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)]));
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)]),
        );
        rejects(&d, &[0x00, 0xFF]);
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
            Value::Pair(Box::new(Value::Seq(vec![])), Box::new(Value::Seq(vec![]))),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Pair(
                Box::new(Value::Seq(vec![Value::U8(0x00)])),
                Box::new(Value::Seq(vec![])),
            ),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Pair(
                Box::new(Value::Seq(vec![])),
                Box::new(Value::Seq(vec![Value::U8(0xFF)])),
            ),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Pair(
                Box::new(Value::Seq(vec![Value::U8(0x00)])),
                Box::new(Value::Seq(vec![Value::U8(0xFF)])),
            ),
        );
        rejects(&d, &[0x7F]);
        rejects(&d, &[0xFF, 0x00]);
        rejects(&d, &[0x00, 0xFF, 0x00]);
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
                ("second-and-third".to_string(), Value::Unit),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Record(vec![
                ("first".to_string(), Value::Seq(vec![Value::U8(0x00)])),
                ("second-and-third".to_string(), Value::Unit),
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
                            Value::Pair(Box::new(Value::U8(0xFF)), Box::new(Value::Seq(vec![]))),
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
                            Value::Pair(Box::new(Value::U8(0xFF)), Box::new(Value::Seq(vec![]))),
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
                ("second-and-third".to_string(), Value::Unit),
            ]),
        );
    }
}
