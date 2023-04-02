use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
enum ByteSet {
    Includes(HashSet<u8>),
    Excludes(HashSet<u8>),
}

impl ByteSet {
    pub fn any() -> ByteSet {
        ByteSet::Excludes(HashSet::new())
    }

    pub fn is(b: u8) -> ByteSet {
        ByteSet::Includes(HashSet::from([b]))
    }

    pub fn not(b: u8) -> ByteSet {
        ByteSet::Excludes(HashSet::from([b]))
    }

    pub fn contains(&self, b: u8) -> bool {
        match self {
            ByteSet::Includes(included) => included.contains(&b),
            ByteSet::Excludes(excluded) => !excluded.contains(&b),
        }
    }

    pub fn is_disjoint(bs0: &ByteSet, bs1: &ByteSet) -> bool {
        match (bs0, bs1) {
            // Easy: check that the sets of included bytes are disjoint
            (ByteSet::Includes(included0), ByteSet::Includes(included1)) => {
                HashSet::is_disjoint(included0, included1)
            }
            // If the set of included bytes are a subset of the excluded bytes,
            // then the byte sets are disjoint
            (ByteSet::Includes(included), ByteSet::Excludes(excluded))
            | (ByteSet::Excludes(excluded), ByteSet::Includes(included)) => {
                HashSet::is_subset(included, excluded)
            }
            // Hard: enumerate these by brute force - they are disjoint if all
            // bytes are contained in one byte set or the other set
            (ByteSet::Excludes(excluded0), ByteSet::Excludes(excluded1)) => {
                (0..=u8::MAX).all(|b| !excluded0.contains(&b) || !excluded1.contains(&b))
            }
        }
    }

    pub fn union(bs0: &ByteSet, bs1: &ByteSet) -> ByteSet {
        match (bs0, bs1) {
            (ByteSet::Includes(included0), ByteSet::Includes(included1)) => {
                ByteSet::Includes(HashSet::union(included0, included1).copied().collect())
            }
            // Remove the included bytes from the excluded bytes
            (ByteSet::Includes(included), ByteSet::Excludes(excluded))
            | (ByteSet::Excludes(excluded), ByteSet::Includes(included)) => {
                ByteSet::Excludes(HashSet::difference(excluded, included).copied().collect())
            }
            (ByteSet::Excludes(excluded0), ByteSet::Excludes(excluded1)) => ByteSet::Excludes(
                HashSet::intersection(excluded0, excluded1)
                    .copied()
                    .collect(),
            ),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    Unit,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    Pair(Box<Value>, Box<Value>),
    Seq(Vec<Value>),
    Record(Vec<(String, Value)>),
}

#[derive(Clone)]
enum Expr {
    Const(Value),
    Var(usize),
    Sub(Box<Expr>, Box<Expr>),
    Pair(Box<Expr>, Box<Expr>),
    Seq(Vec<Expr>),
    Record(Vec<(String, Expr)>),
}

#[derive(Clone)]
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
#[derive(Clone)]
enum Format {
    /// A format that never matches
    Fail,
    /// A format that matches the empty byte string
    Empty,
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Matches the union of the byte strings matched by the two formats
    Alt(Box<Format>, Box<Format>),
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
    /// Transform a decoded value with a function
    Map(Func, Box<Format>),
    /// Conditional format
    If(Expr, Box<Format>, Box<Format>),
}

#[derive(Debug)]
struct Lookahead {
    pattern: Vec<ByteSet>,
}

enum Cond {
    Expr(Expr),
    Peek(Lookahead),
}

/// Decoders with a fixed amount of lookahead
enum Decoder {
    Fail,
    Empty,
    Byte(ByteSet),
    If(Cond, Box<Decoder>, Box<Decoder>),
    Cat(Box<Decoder>, Box<Decoder>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(Lookahead, Box<Decoder>),
    Until(Lookahead, Box<Decoder>),
    RepeatCount(Expr, Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    Map(Func, Box<Decoder>),
}

impl Expr {
    fn eval(&self, stack: &[Value]) -> Value {
        match self {
            Expr::Const(v) => v.clone(),
            Expr::Var(index) => stack[stack.len() - index - 1].clone(),
            Expr::Sub(x, y) => match (x.eval(stack), y.eval(stack)) {
                (Value::U8(x), Value::U8(y)) => Value::U8(x - y),
                (Value::U16(x), Value::U16(y)) => Value::U16(x - y),
                (_, _) => panic!("mismatched operands"),
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
            Value::U32(n) => n as usize, // FIXME
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
    fn from_bytes(bytes: &[u8]) -> Format {
        let v = bytes
            .iter()
            .map(|b| Format::Byte(ByteSet::is(*b)))
            .collect();
        Format::Tuple(v)
    }

    fn nullable(&self) -> bool {
        match self {
            Format::Fail => false,
            Format::Empty => true,
            Format::Byte(_) => false,
            Format::Alt(a, b) => a.nullable() || b.nullable(),
            Format::Cat(a, b) => a.nullable() && b.nullable(),
            Format::Tuple(fields) => fields.iter().all(|f| f.nullable()),
            Format::Record(fields) => fields.iter().all(|(_, f)| f.nullable()),
            Format::Repeat(_a) => true,
            Format::RepeatCount(_expr, _a) => true,
            Format::Slice(_expr, _a) => true,
            Format::Map(_f, a) => a.nullable(),
            Format::If(_expr, a, b) => a.nullable() || b.nullable(),
        }
    }

    pub fn might_match_lookahead(&self, input: &[ByteSet], next: Format) -> bool {
        match self {
            Format::Fail => false,
            Format::Empty => match next {
                Format::Empty => input.is_empty(),
                next => next.might_match_lookahead(input, Format::Empty),
            },
            Format::Byte(bs) => match input.split_first() {
                Some((b, _)) if ByteSet::is_disjoint(bs, b) => false,
                Some((_, input)) => next.might_match_lookahead(input, Format::Empty),
                None => true,
            },
            Format::Alt(a, b) => {
                a.might_match_lookahead(input, next.clone()) || b.might_match_lookahead(input, next)
            }
            Format::Cat(a, b) => match **b {
                Format::Empty => a.might_match_lookahead(input, next),
                _ => a.might_match_lookahead(input, Format::Cat(b.clone(), Box::new(next))),
            },
            Format::Tuple(fields) => match fields.split_first() {
                None => next.might_match_lookahead(input, Format::Empty),
                Some((a, fields)) => a.might_match_lookahead(
                    input,
                    Format::Cat(Box::new(Format::Tuple(fields.to_vec())), Box::new(next)),
                ),
            },
            Format::Record(fields) => match fields.split_first() {
                None => next.might_match_lookahead(input, Format::Empty),
                Some(((_, a), fields)) => a.might_match_lookahead(
                    input,
                    Format::Cat(Box::new(Format::Record(fields.to_vec())), Box::new(next)),
                ),
            },
            Format::Repeat(a) => Format::Alt(
                Box::new(Format::Empty),
                Box::new(Format::Cat(a.clone(), Box::new(Format::Repeat(a.clone())))),
            )
            .might_match_lookahead(input, next),
            Format::RepeatCount(_expr, _a) => {
                true // FIXME
            }
            Format::Slice(_expr, _a) => {
                true // FIXME
            }
            Format::Map(_f, a) => a.might_match_lookahead(input, next),
            Format::If(_expr, a, b) => {
                a.might_match_lookahead(input, next.clone()) || b.might_match_lookahead(input, next)
            }
        }
    }
}

impl Lookahead {
    pub fn empty() -> Lookahead {
        Lookahead { pattern: vec![] }
    }

    pub fn single(bs: ByteSet) -> Lookahead {
        Lookahead { pattern: vec![bs] }
    }

    pub fn alt(a: &Lookahead, b: &Lookahead) -> Lookahead {
        Lookahead {
            pattern: Iterator::zip(a.pattern.iter(), b.pattern.iter())
                .map(|(ba, bb)| ByteSet::union(&ba, bb))
                .collect(),
        }
    }

    pub fn cat(a: &Lookahead, b: &Lookahead) -> Lookahead {
        let mut pattern = a.pattern.clone();
        pattern.extend(b.pattern.iter().cloned());
        Lookahead { pattern }
    }

    /// Find a lookahead that only the first format matches
    pub fn new(a: &Format, b: &Format, opt_next: Option<&Format>) -> Option<Lookahead> {
        const LEN: usize = 2;
        let next = match opt_next {
            None => Format::Empty,
            Some(next) => next.clone(),
        };
        let pa = Lookahead::from(a, LEN, next.clone())?;
        if !b.might_match_lookahead(&pa.pattern, next) {
            Some(pa)
        } else {
            None
        }
    }
    /*
        pub fn disjoint(a: &Self, b: &Self) -> bool {
            for i in 0..std::cmp::min(a.pattern.len(), b.pattern.len()) {
                if ByteSet::disjoint(&a.pattern[i], &b.pattern[i]) {
                    return true;
                }
            }
            false
        }
    */
    pub fn matches(&self, input: &[u8]) -> bool {
        self.pattern.len() <= input.len()
            && Iterator::zip(self.pattern.iter(), input.iter()).all(|(p, b)| p.contains(*b))
    }

    pub fn from(f: &Format, len: usize, next: Format) -> Option<Lookahead> {
        match f {
            Format::Fail => None,
            Format::Empty => match next {
                Format::Empty => Some(Lookahead::empty()),
                next => Lookahead::from(&next, len, Format::Empty),
            },
            Format::Byte(bs) => {
                let pa = Lookahead::single(bs.clone());
                if len > 1 {
                    // FIXME do we still need to check for Format::Zero?
                    let pb = Lookahead::from(&next, len - 1, Format::Empty)?;
                    Some(Lookahead::cat(&pa, &pb))
                } else {
                    Some(pa)
                }
            }
            Format::Alt(a, b) => {
                let ra = Lookahead::from(a, len, next.clone());
                let rb = Lookahead::from(b, len, next);
                match (ra, rb) {
                    (None, None) => None,
                    (Some(pa), None) => Some(pa),
                    (None, Some(pb)) => Some(pb),
                    (Some(pa), Some(pb)) => Some(Lookahead::alt(&pa, &pb)),
                }
            }
            Format::Cat(a, b) => match **b {
                Format::Empty => Lookahead::from(a, len, next),
                _ => Lookahead::from(a, len, Format::Cat(Box::new(*b.clone()), Box::new(next))),
            },
            Format::Tuple(fields) => match fields.split_first() {
                None => Lookahead::from(&next, len, Format::Empty),
                Some((a, fields)) => Lookahead::from(
                    a,
                    len,
                    Format::Cat(Box::new(Format::Tuple(fields.to_vec())), Box::new(next)),
                ),
            },
            Format::Record(fields) => match fields.split_first() {
                None => Lookahead::from(&next, len, Format::Empty),
                Some(((_, a), fields)) => Lookahead::from(
                    a,
                    len,
                    Format::Cat(Box::new(Format::Record(fields.to_vec())), Box::new(next)),
                ),
            },
            Format::Repeat(a) => Lookahead::from(
                &Format::Alt(
                    Box::new(Format::Empty),
                    Box::new(Format::Cat(a.clone(), Box::new(Format::Repeat(a.clone())))),
                ),
                len,
                next,
            ),
            Format::RepeatCount(_expr, _a) => {
                Some(Lookahead::empty()) // FIXME ?
            }
            Format::Slice(_expr, _a) => {
                Some(Lookahead::empty()) // FIXME ?
            }
            Format::Map(_f, a) => Lookahead::from(a, len, next),
            Format::If(_expr, a, b) => {
                Lookahead::from(&Format::Alt(a.clone(), b.clone()), len, next)
            }
        }
    }
}

impl Cond {
    fn eval(&self, stack: &[Value], input: &[u8]) -> bool {
        match self {
            Cond::Expr(expr) => expr.eval_bool(stack),
            Cond::Peek(look) => look.matches(input),
        }
    }
}

impl Decoder {
    pub fn compile(f: &Format, opt_next: Option<&Format>) -> Result<Decoder, String> {
        match f {
            Format::Fail => Ok(Decoder::Fail),
            Format::Empty => Ok(Decoder::Empty),
            Format::Byte(bs) => Ok(Decoder::Byte(bs.clone())),
            Format::Alt(a, b) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                let db = Box::new(Decoder::compile(b, opt_next)?);
                if let Some(look) = Lookahead::new(a, b, opt_next) {
                    Ok(Decoder::If(Cond::Peek(look), da, db))
                } else if let Some(look) = Lookahead::new(b, a, opt_next) {
                    Ok(Decoder::If(Cond::Peek(look), db, da))
                } else {
                    Err("cannot find valid lookahead for alt".to_string())
                }
            }
            Format::Cat(a, b) => {
                let da = Box::new(Decoder::compile(a, Some(&b))?);
                let db = Box::new(Decoder::compile(b, opt_next)?);
                Ok(Decoder::Cat(da, db))
            }
            Format::Tuple(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.as_slice();

                while let Some((f, remain)) = fields.split_first() {
                    fields = remain;
                    let df = match (remain.is_empty(), opt_next) {
                        (true, None) => Decoder::compile(f, None)?,
                        (true, Some(next)) => Decoder::compile(f, Some(next))?,
                        (false, None) => {
                            Decoder::compile(f, Some(&Format::Tuple(remain.to_vec())))?
                        }
                        (false, Some(next)) => Decoder::compile(
                            f,
                            Some(&Format::Cat(
                                Box::new(Format::Tuple(remain.to_vec())),
                                Box::new(next.clone()),
                            )),
                        )?,
                    };
                    dfields.push(df);
                }

                Ok(Decoder::Tuple(dfields))
            }
            Format::Record(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.as_slice();

                while let Some(((name, f), remain)) = fields.split_first() {
                    fields = remain;
                    let df = match (remain.is_empty(), opt_next) {
                        (true, None) => Decoder::compile(f, None)?,
                        (true, Some(next)) => Decoder::compile(f, Some(next))?,
                        (false, None) => {
                            Decoder::compile(f, Some(&Format::Record(remain.to_vec())))?
                        }
                        (false, Some(next)) => Decoder::compile(
                            f,
                            Some(&Format::Cat(
                                Box::new(Format::Record(remain.to_vec())),
                                Box::new(next.clone()),
                            )),
                        )?,
                    };
                    dfields.push((name.clone(), df));
                }

                Ok(Decoder::Record(dfields))
            }
            Format::Repeat(a) => {
                if a.nullable() {
                    return Err("cannot repeat nullable format".to_string());
                }
                let da = Box::new(Decoder::compile(a, None)?);
                if opt_next.is_some() {
                    let aplus = Format::Cat(a.clone(), Box::new(Format::Repeat(a.clone())));
                    if let Some(look) = Lookahead::new(&aplus, &Format::Empty, opt_next) {
                        Ok(Decoder::While(look, da))
                    } else if let Some(look) = Lookahead::new(&Format::Empty, &aplus, opt_next) {
                        Ok(Decoder::Until(look, da))
                    } else {
                        Err("cannot find valid lookahead for repeat".to_string())
                    }
                } else {
                    Ok(Decoder::While(Lookahead::single(ByteSet::any()), da))
                }
            }
            Format::RepeatCount(expr, a) => {
                // FIXME probably not right
                let da = Box::new(Decoder::compile(a, opt_next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(Decoder::compile(a, None)?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::Map(f, a) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                Ok(Decoder::Map(f.clone(), da))
            }
            Format::If(expr, a, b) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                let db = Box::new(Decoder::compile(b, opt_next)?);
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
            Decoder::While(look, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while look.matches(input) {
                    let (va, next_input) = a.parse(stack, input)?;
                    input = next_input;
                    v.push(va);
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Until(look, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while !look.matches(input) {
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
            Decoder::Map(f, a) => {
                let (va, input) = a.parse(stack, input)?;
                Some((f.eval(va), input))
            }
        }
    }
}

fn alts(formats: impl IntoIterator<Item = Format>) -> Format {
    let mut formats = formats.into_iter();
    let format = formats.next().unwrap_or(Format::Fail);
    formats.fold(format, |acc, format| {
        Format::Alt(Box::new(acc), Box::new(format))
    })
}

fn record<Label: ToString>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
    Format::Record(
        (fields.into_iter())
            .map(|(label, format)| (label.to_string(), format))
            .collect(),
    )
}

// fn optional(format: Format) -> Format {
//     alts([format, Format::Unit])
// }

fn repeat(format: Format) -> Format {
    Format::Repeat(Box::new(format))
}

fn repeat_count(len: Expr, format: Format) -> Format {
    Format::RepeatCount(len, Box::new(format))
}

fn any_bytes() -> Format {
    repeat(Format::Byte(ByteSet::any()))
}

fn u8() -> Format {
    Format::Byte(ByteSet::any())
}

fn u16be() -> Format {
    Format::Map(
        Func::U16Be,
        Box::new(Format::Cat(
            Box::new(Format::Byte(ByteSet::any())),
            Box::new(Format::Byte(ByteSet::any())),
        )),
    )
}

fn u16le() -> Format {
    Format::Map(
        Func::U16Le,
        Box::new(Format::Cat(
            Box::new(Format::Byte(ByteSet::any())),
            Box::new(Format::Byte(ByteSet::any())),
        )),
    )
}

fn u32be() -> Format {
    Format::Map(
        Func::U32Be,
        Box::new(Format::Tuple(vec![
            Format::Byte(ByteSet::any()),
            Format::Byte(ByteSet::any()),
            Format::Byte(ByteSet::any()),
            Format::Byte(ByteSet::any()),
        ])),
    )
}

fn u32le() -> Format {
    Format::Map(
        Func::U32Le,
        Box::new(Format::Tuple(vec![
            Format::Byte(ByteSet::any()),
            Format::Byte(ByteSet::any()),
            Format::Byte(ByteSet::any()),
            Format::Byte(ByteSet::any()),
        ])),
    )
}

fn png_format() -> Format {
    let chunk = record([
        ("length", u32be()), // FIXME < 2^31
        (
            "type", // FIXME ASCII
            Format::Tuple(vec![
                Format::Byte(ByteSet::any()),
                Format::Byte(ByteSet::any()),
                Format::Byte(ByteSet::any()),
                Format::Byte(ByteSet::any()),
            ]),
        ),
        (
            "data",
            Format::RepeatCount(Expr::Var(1), Box::new(Format::Byte(ByteSet::any()))),
        ),
        ("crc", u32be()), // FIXME check this
    ]);

    record([
        ("signature", Format::from_bytes(b"\x89PNG\r\n\x1A\n")),
        ("chunks", Format::Repeat(Box::new(chunk))),
    ])
}

fn jpeg_format() -> Format {
    fn marker(id: u8) -> Format {
        Format::Map(
            Func::Snd,
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::is(0xFF))),
                Box::new(Format::Byte(ByteSet::is(id))),
            )),
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

    // SOF: Frame header (See ITU T.81 Section B.2.2)
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
    let dht_data = repeat(record([
        // class <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("class-table-id", u8()),
        ("num-codes", repeat_count(Expr::Const(Value::U8(16)), u8())),
        ("values", any_bytes()), // List.map num-codes (\n => repeat-count n u8);
    ]));

    // DAC: Define arithmetic conditioning table (See ITU T.81 Section B.2.4.3)
    let dac_data = repeat(record([
        // class <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("class-table-id", u8()),
        ("value", u8()),
    ]));

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

    // DRI: Define restart interval (See ITU T.81 Section B.2.4.4)
    let dri_data = record([("restart-interval", u16be())]);

    // APP0: Application segment 0
    let app0_data = record([
        ("identifier", Format::from_bytes(b"JFIF\0")),
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

    let app1_exif = record([
        ("identifier", Format::from_bytes(b"Exif\0\0")),
        (
            "big-endian",
            alts([
                Format::Map(
                    Func::Expr(Expr::Const(Value::Bool(false))),
                    Box::new(Format::from_bytes(b"II*\0")),
                ),
                Format::Map(
                    Func::Expr(Expr::Const(Value::Bool(true))),
                    Box::new(Format::from_bytes(b"MM\0*")),
                ),
            ]),
        ),
        (
            "offset",
            Format::If(Expr::Var(0), Box::new(u32be()), Box::new(u32le())),
        ),
        ("exif", any_bytes()),
    ]);

    let app1_xmp = record([
        (
            "identifier",
            Format::from_bytes(b"http://ns.adobe.com/xap/1.0/\0"),
        ),
        ("xmp", any_bytes()),
        // FIXME there are other APP1 formats
        // see https://exiftool.org/TagNames/JPEG.html
    ]);

    let app1_data = alts([app1_exif, app1_xmp]);

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
    let _dnl = marker_segment(0xDC, any_bytes()); // Define number of lines
    let dri = marker_segment(0xDD, dri_data.clone()); // Define restart interval
    let _dhp = marker_segment(0xDE, any_bytes()); // Define hierarchical progression
    let _exp = marker_segment(0xDF, any_bytes()); // Expand reference components
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
        Format::Byte(ByteSet::not(0xFF)),
        Format::Map(
            Func::Expr(Expr::Const(Value::U8(0xFF))),
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::is(0xFF))),
                Box::new(Format::Byte(ByteSet::is(0x00))),
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
        // TODO: ("dnl", optional(dnl)), // Error: "cannot find valid lookahead for star"
        // TODO: ("scans", repeat(scan)), // Error: "cannot find valid lookahead for star"
    ]);

    let jpeg = record([
        ("soi", soi.clone()),
        ("frame", frame.clone()),
        ("eoi", eoi.clone()),
    ]);

    jpeg
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() < 2 { "test.jpg" } else { &args[1] };
    let input = fs::read(filename)?;

    let format = alts([jpeg_format(), png_format()]);
    let decoder = Decoder::compile(&format, None)?;

    let mut stack = Vec::new();
    let (val, _) = decoder.parse(&mut stack, &input).ok_or("parse failure")?;

    println!("{:?}", val);

    Ok(())
}
