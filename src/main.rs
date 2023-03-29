use std::fs;

#[derive(Copy, Clone, Debug)]
enum ByteSet {
    Any,
    Is(u8),
    Not(u8),
}

#[derive(Clone, Debug)]
enum Value {
    Unit,
    U8(u8),
    U16(u16),
    Pair(Box<Value>, Box<Value>),
    Seq(Vec<Value>),
    Record(Vec<(String, Value)>),
}

#[derive(Clone)]
enum Expr {
    Var(usize),
    Unit,
    U8(u8),
    U16(u16),
    Sub(Box<Expr>, Box<Expr>),
    Pair(Box<Expr>, Box<Expr>),
    Seq(Vec<Expr>),
    Record(Vec<(String, Expr)>),
}

#[derive(Clone)]
enum Format {
    Zero,
    Unit,
    Byte(ByteSet),
    Alt(Box<Format>, Box<Format>),
    Cat(Box<Format>, Box<Format>),
    Tuple(Vec<Format>),
    Record(Vec<(String, Format)>),
    Star(Box<Format>),
    Array(Expr, Box<Format>),
    Slice(Expr, Box<Format>),
    Map(fn(&Value) -> Value, Box<Format>),
}

#[derive(Debug)]
struct Lookahead {
    pattern: Vec<ByteSet>,
}

enum Decoder {
    Zero,
    Unit,
    Byte(ByteSet),
    If(Lookahead, Box<Decoder>, Box<Decoder>),
    Cat(Box<Decoder>, Box<Decoder>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(Lookahead, Box<Decoder>),
    Until(Lookahead, Box<Decoder>),
    Array(Expr, Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    Map(fn(&Value) -> Value, Box<Decoder>),
}

impl Expr {
    fn eval(&self, stack: &[Value]) -> Value {
        match self {
            Expr::Var(index) => stack[stack.len() - index - 1].clone(),
            Expr::Unit => Value::Unit,
            Expr::U8(x) => Value::U8(*x),
            Expr::U16(x) => Value::U16(*x),
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

    fn eval_usize(&self, stack: &[Value]) -> usize {
        match self.eval(stack) {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::Unit | Value::Pair(_, _) | Value::Seq(_) | Value::Record(_) => {
                panic!("value is not number")
            }
        }
    }
}

impl ByteSet {
    pub fn contains(&self, a: u8) -> bool {
        match *self {
            ByteSet::Any => true,
            ByteSet::Is(b) => a == b,
            ByteSet::Not(b) => a != b,
        }
    }

    pub fn disjoint(a: &Self, b: &Self) -> bool {
        match (*a, *b) {
            (ByteSet::Any, _) => false,
            (_, ByteSet::Any) => false,
            (ByteSet::Is(m), ByteSet::Is(n)) => m != n,
            (ByteSet::Not(m), ByteSet::Is(n)) => m == n,
            (ByteSet::Is(m), ByteSet::Not(n)) => m == n,
            (ByteSet::Not(_), ByteSet::Not(_)) => false,
        }
    }

    pub fn union(a: &Self, b: &Self) -> ByteSet {
        match (*a, *b) {
            (ByteSet::Any, _) => ByteSet::Any,
            (_, ByteSet::Any) => ByteSet::Any,
            (ByteSet::Is(m), ByteSet::Is(n)) if m == n => ByteSet::Is(m),
            (ByteSet::Is(_), ByteSet::Is(_)) => ByteSet::Any,
            (ByteSet::Not(m), ByteSet::Not(n)) if m == n => ByteSet::Not(m),
            (ByteSet::Not(_), ByteSet::Not(_)) => ByteSet::Any,
            (ByteSet::Is(m), ByteSet::Not(n)) if m != n => ByteSet::Not(n),
            (ByteSet::Is(_), ByteSet::Not(_)) => ByteSet::Any,
            (ByteSet::Not(m), ByteSet::Is(n)) if m != n => ByteSet::Not(m),
            (ByteSet::Not(_), ByteSet::Is(_)) => ByteSet::Any,
        }
    }
}

impl Format {
    fn from_bytes(bytes: &[u8]) -> Format {
        let v = bytes
            .iter()
            .map(|b| Format::Byte(ByteSet::Is(*b)))
            .collect();
        Format::Tuple(v)
    }

    pub fn might_match_lookahead(&self, input: &[ByteSet], next: Format) -> bool {
        match self {
            Format::Zero => false,
            Format::Unit => match next {
                Format::Unit => true,
                next => next.might_match_lookahead(input, Format::Unit),
            },
            Format::Byte(bs) => match input.split_first() {
                Some((b, _)) if ByteSet::disjoint(bs, b) => false,
                Some((_, input)) => next.might_match_lookahead(input, Format::Unit),
                None => true,
            },
            Format::Alt(a, b) => {
                a.might_match_lookahead(input, next.clone()) || b.might_match_lookahead(input, next)
            }
            Format::Cat(a, b) => {
                a.might_match_lookahead(input, Format::Cat(b.clone(), Box::new(next)))
            }
            Format::Tuple(fields) => match fields.split_first() {
                None => next.might_match_lookahead(input, Format::Unit),
                Some((a, fields)) => a.might_match_lookahead(input, Format::Tuple(fields.to_vec())),
            },
            Format::Record(fields) => match fields.split_first() {
                None => next.might_match_lookahead(input, Format::Unit),
                Some(((_, a), fields)) => {
                    a.might_match_lookahead(input, Format::Record(fields.to_vec()))
                }
            },
            Format::Star(_a) => {
                true // FIXME
            }
            Format::Array(_expr, _a) => {
                true // FIXME
            }
            Format::Slice(_expr, _a) => {
                true // FIXME
            }
            Format::Map(_f, a) => a.might_match_lookahead(input, next),
        }
    }
}

impl Lookahead {
    pub fn empty() -> Self {
        Lookahead { pattern: vec![] }
    }

    pub fn single(bs: ByteSet) -> Self {
        Lookahead { pattern: vec![bs] }
    }

    pub fn alt(a: &Self, b: &Self) -> Self {
        let mut pattern = Vec::new();
        for i in 0..std::cmp::min(a.pattern.len(), b.pattern.len()) {
            pattern.push(ByteSet::union(&a.pattern[i], &b.pattern[i]));
        }
        Lookahead { pattern }
    }

    pub fn cat(a: &Self, b: &Self) -> Self {
        let mut pattern = a.pattern.clone();
        pattern.extend(b.pattern.iter());
        Lookahead { pattern }
    }

    pub fn new(a: &Format, b: &Format) -> Option<Self> {
        const LEN: usize = 2;
        let pa = Lookahead::from(a, LEN, Format::Unit)?;
        if !b.might_match_lookahead(&pa.pattern, Format::Unit) {
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
            Format::Zero => None,
            Format::Unit => match next {
                Format::Unit => Some(Lookahead::empty()),
                next => Lookahead::from(&next, len, Format::Unit),
            },
            Format::Byte(bs) => {
                let pa = Lookahead::single(bs.clone());
                if len > 1 {
                    // FIXME do we still need to check for Format::Zero?
                    let pb = Lookahead::from(&next, len - 1, Format::Unit)?;
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
            Format::Cat(a, b) => {
                Lookahead::from(a, len, Format::Cat(Box::new(*b.clone()), Box::new(next)))
            }
            Format::Tuple(fields) => match fields.split_first() {
                None => Some(Lookahead::empty()),
                Some((a, fields)) => Lookahead::from(a, len, Format::Tuple(fields.to_vec())),
            },
            Format::Record(fields) => match fields.split_first() {
                None => Some(Lookahead::empty()),
                Some(((_, a), fields)) => Lookahead::from(a, len, Format::Record(fields.to_vec())),
            },
            Format::Star(_a) => {
                Some(Lookahead::empty()) // FIXME ?
            }
            Format::Array(_expr, _a) => {
                Some(Lookahead::empty()) // FIXME ?
            }
            Format::Slice(_expr, _a) => {
                Some(Lookahead::empty()) // FIXME ?
            }
            Format::Map(_f, a) => Lookahead::from(a, len, next),
        }
    }
}

impl Decoder {
    pub fn compile(f: &Format, opt_next: Option<&Format>) -> Result<Decoder, String> {
        match f {
            Format::Zero => Ok(Decoder::Zero),
            Format::Unit => Ok(Decoder::Unit),
            Format::Byte(bs) => Ok(Decoder::Byte(bs.clone())),
            Format::Alt(a, b) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                let db = Box::new(Decoder::compile(b, opt_next)?);
                if let Some(l) = Lookahead::new(a, b) {
                    Ok(Decoder::If(l, da, db))
                } else if let Some(l) = Lookahead::new(b, a) {
                    Ok(Decoder::If(l, db, da))
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
                let mut fields = fields.iter().peekable();

                while let Some(f) = fields.next() {
                    let opt_next = match fields.peek() {
                        Some(opt_next) => Some(*opt_next),
                        None => opt_next,
                    };
                    let df = Decoder::compile(f, opt_next)?;
                    dfields.push(df);
                }

                Ok(Decoder::Tuple(dfields))
            }
            Format::Record(fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter().peekable();

                while let Some((name, f)) = fields.next() {
                    let opt_next = match fields.peek() {
                        Some((_, opt_next)) => Some(opt_next),
                        None => opt_next,
                    };
                    let df = Decoder::compile(f, opt_next)?;
                    dfields.push((name.clone(), df));
                }

                Ok(Decoder::Record(dfields))
            }
            Format::Star(a) => {
                // FIXME next should be a|opt_next ?
                let da = Box::new(Decoder::compile(a, None)?);
                if let Some(next) = opt_next {
                    if let Some(l) = Lookahead::new(a, next) {
                        Ok(Decoder::While(l, da))
                    } else if let Some(l) = Lookahead::new(next, a) {
                        Ok(Decoder::Until(l, da))
                    } else {
                        Err("cannot find valid lookahead for star".to_string())
                    }
                } else {
                    Ok(Decoder::While(Lookahead::single(ByteSet::Any), da))
                }
            }
            Format::Array(expr, a) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                Ok(Decoder::Array(expr.clone(), da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(Decoder::compile(a, None)?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::Map(f, a) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                Ok(Decoder::Map(*f, da))
            }
        }
    }

    pub fn parse<'input>(
        &self,
        stack: &mut Vec<Value>,
        input: &'input [u8],
    ) -> Option<(Value, &'input [u8])> {
        match self {
            Decoder::Zero => None,
            Decoder::Unit => Some((Value::Unit, input)),
            Decoder::Byte(bs) => {
                let (&b, input) = input.split_first()?;
                if bs.contains(b) {
                    Some((Value::U8(b), input))
                } else {
                    None
                }
            }
            Decoder::If(look, a, b) => {
                if look.matches(input) {
                    a.parse(stack, input)
                } else {
                    b.parse(stack, input)
                }
            }
            Decoder::Cat(a, b) => {
                let (va, input) = a.parse(stack, input)?;
                stack.push(va);
                let (vb, input) = b.parse(stack, input)?;
                let va = stack.pop().unwrap();
                Some((Value::Pair(Box::new(va), Box::new(vb)), input))
            }
            Decoder::Tuple(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for f in fields {
                    let (vf, next_input) = f.parse(stack, input)?;
                    input = next_input;
                    v.push(vf.clone());
                    stack.push(vf);
                }
                for _ in fields {
                    stack.pop();
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Record(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for (name, f) in fields {
                    println!("field: {name}");
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
                println!("while: {look:?}");
                let mut input = input;
                let mut v = Vec::new();
                while look.matches(input) {
                    let (va, next_input) = a.parse(stack, input).unwrap();
                    input = next_input;
                    v.push(va);
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Until(look, a) => {
                println!("until: {look:?}");
                let mut input = input;
                let mut v = Vec::new();
                while !look.matches(input) {
                    let (va, next_input) = a.parse(stack, input)?;
                    input = next_input;
                    v.push(va);
                }
                Some((Value::Seq(v), input))
            }
            Decoder::Array(expr, a) => {
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
                Some((f(&va), input))
            }
        }
    }
}

fn any_bytes() -> Format {
    Format::Star(Box::new(Format::Byte(ByteSet::Any)))
}

fn alts(formats: impl IntoIterator<Item = Format>) -> Format {
    let mut formats = formats.into_iter();
    let format = formats.next().unwrap_or(Format::Zero);
    formats.fold(format, |acc, format| {
        Format::Alt(Box::new(acc), Box::new(format))
    })
}

fn u8() -> Format {
    Format::Byte(ByteSet::Any)
}

fn u16be() -> Format {
    Format::Map(
        |value| match value {
            Value::Pair(fst, snd) => match (fst.as_ref(), snd.as_ref()) {
                (Value::U8(hi), Value::U8(lo)) => Value::U16(u16::from_be_bytes([*hi, *lo])),
                (_, _) => panic!("expected (U8, U8)"),
            },
            _ => panic!("expected (_, _)"),
        },
        Box::new(Format::Cat(
            Box::new(Format::Byte(ByteSet::Any)),
            Box::new(Format::Byte(ByteSet::Any)),
        )),
    )
}

fn jpeg_format() -> Format {
    fn marker(id: u8) -> Format {
        Format::Map(
            |value| match value {
                Value::Pair(_, snd) => (**snd).clone(),
                _ => panic!("expected (_, _)"),
            },
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(id))),
            )),
        )
    }

    fn marker_segment(id: u8, data: Format) -> Format {
        Format::Record(vec![
            ("marker".to_string(), marker(id)),
            ("length".to_string(), u16be()),
            (
                "data".to_string(),
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

    let app0_data = Format::Record(vec![
        ("identifier".to_string(), Format::from_bytes(b"JFIF\0")),
        ("version-major".to_string(), u8()),
        ("version-minor".to_string(), u8()),
        ("density-units".to_string(), u8()), // 0 | 1 | 2
        ("density-x".to_string(), u16be()),  // != 0
        ("density-y".to_string(), u16be()),  // != 0
        ("thumbnail-width".to_string(), u8()),
        ("thumbnail-height".to_string(), u8()),
        (
            "thumbnail-pixels".to_string(),
            Format::Array(
                Expr::Var(0), // thumbnail-height
                Box::new(Format::Array(
                    Expr::Var(1), // thumbnail-width
                    Box::new(Format::Record(vec![
                        ("r".to_string(), u8()),
                        ("g".to_string(), u8()),
                        ("b".to_string(), u8()),
                    ])),
                )),
            ),
        ),
    ]);

    let ecs = Format::Star(Box::new(Format::Alt(
        Box::new(Format::Byte(ByteSet::Not(0xFF))),
        Box::new(Format::Map(
            |_| Value::U8(0xFF),
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0x00))),
            )),
        )),
    )));

    let sof0 = marker_segment(0xC0, any_bytes());
    let dht = marker_segment(0xC4, any_bytes());
    let sos = marker_segment(0xDA, any_bytes());
    let dqt = marker_segment(0xDB, any_bytes());
    let soi = marker(0xD8);
    let eoi = marker(0xD9);
    let app0 = marker_segment(0xE0, app0_data);

    let table_or_misc = alts([dqt, sof0, dht]);
    let scan = Format::Record(vec![("sos".to_string(), sos), ("ecs".to_string(), ecs)]);

    let jpeg = Format::Record(vec![
        ("soi".to_string(), soi),
        ("app0".to_string(), app0),
        (
            "segments".to_string(),
            Format::Star(Box::new(table_or_misc)),
        ),
        ("scan".to_string(), scan),
        ("eoi".to_string(), eoi),
    ]);

    jpeg
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read("test.jpg")?;
    let f = jpeg_format();
    let decoder = Decoder::compile(&f, None)?;
    let mut stack = Vec::new();
    let res = decoder.parse(&mut stack, &input);
    println!("{:?}", res);
    Ok(())
}
