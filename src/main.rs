use std::fs;

#[derive(Copy, Clone, Debug)]
enum ByteSet {
    Any,
    Is(u8),
    Not(u8),
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
    Array(usize, Box<Format>),
    Map(fn(&Value) -> Value, Box<Format>),
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
    Array(usize, Box<Decoder>),
    Map(fn(&Value) -> Value, Box<Decoder>),
}

impl Value {
    pub fn usize_or_panic(&self) -> usize {
        match *self {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::Unit | Value::Pair(_, _) | Value::Seq(_) | Value::Record(_) => {
                panic!("value is not number")
            }
        }
    }

    pub fn map_u16be_minus_two(&self) -> Self {
        if let Value::Pair(fst, snd) = self {
            if let Value::U8(hi) = **fst {
                if let Value::U8(lo) = **snd {
                    let n = (u16::from(hi) << 8) + u16::from(lo);
                    Value::U16(n - 2)
                } else {
                    panic!("second is not u8")
                }
            } else {
                panic!("first is not u8")
            }
        } else {
            panic!("value is not pair")
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
            (ByteSet::Is(m), ByteSet::Is(n)) => {
                if m == n {
                    ByteSet::Is(m)
                } else {
                    ByteSet::Any
                }
            }
            (ByteSet::Not(m), ByteSet::Not(n)) => {
                if m == n {
                    ByteSet::Not(m)
                } else {
                    ByteSet::Any
                }
            }
            (ByteSet::Is(m), ByteSet::Not(n)) => {
                if m != n {
                    ByteSet::Not(n)
                } else {
                    ByteSet::Any
                }
            }
            (ByteSet::Not(m), ByteSet::Is(n)) => {
                if m != n {
                    ByteSet::Not(m)
                } else {
                    ByteSet::Any
                }
            }
        }
    }
}

impl Format {
    pub fn might_match_lookahead(&self, input: &[ByteSet], next: Format) -> bool {
        match self {
            Format::Zero => false,
            Format::Unit => {
                if let Format::Unit = next {
                    true
                } else {
                    next.might_match_lookahead(input, Format::Unit)
                }
            }
            Format::Byte(bs) => {
                if input.len() > 0 {
                    if ByteSet::disjoint(bs, &input[0]) {
                        false
                    } else {
                        next.might_match_lookahead(&input[1..], Format::Unit)
                    }
                } else {
                    true
                }
            }
            Format::Alt(a, b) => {
                a.might_match_lookahead(input, next.clone()) || b.might_match_lookahead(input, next)
            }
            Format::Cat(a, b) => {
                a.might_match_lookahead(input, Format::Cat(b.clone(), Box::new(next)))
            }
            Format::Tuple(fields) => {
                if fields.is_empty() {
                    next.might_match_lookahead(input, Format::Unit)
                } else {
                    fields[0].might_match_lookahead(input, Format::Tuple(fields[1..].to_vec()))
                }
            }
            Format::Record(fields) => {
                if fields.is_empty() {
                    next.might_match_lookahead(input, Format::Unit)
                } else {
                    fields[0]
                        .1
                        .might_match_lookahead(input, Format::Record(fields[1..].to_vec()))
                }
            }
            Format::Star(_a) => {
                true // FIXME
            }
            Format::Array(_index, _a) => {
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
        if self.pattern.len() > input.len() {
            return false;
        }
        for i in 0..self.pattern.len() {
            if !self.pattern[i].contains(input[i]) {
                return false;
            }
        }
        return true;
    }

    pub fn from(f: &Format, len: usize, next: Format) -> Option<Lookahead> {
        match f {
            Format::Zero => None,
            Format::Unit => {
                if let Format::Unit = next {
                    Some(Lookahead::empty())
                } else {
                    Lookahead::from(&next, len, Format::Unit)
                }
            }
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
            Format::Tuple(fields) => {
                if fields.is_empty() {
                    Some(Lookahead::empty())
                } else {
                    Lookahead::from(&fields[0], len, Format::Tuple(fields[1..].to_vec()))
                }
            }
            Format::Record(fields) => {
                if fields.is_empty() {
                    Some(Lookahead::empty())
                } else {
                    Lookahead::from(&fields[0].1, len, Format::Record(fields[1..].to_vec()))
                }
            }
            Format::Star(_a) => {
                Some(Lookahead::empty()) // FIXME ?
            }
            Format::Array(_index, _a) => {
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
                let mut dfields = Vec::new();
                for i in 0..fields.len() {
                    let f = &fields[i];
                    let opt_next = if i + 1 < fields.len() {
                        Some(&fields[i + 1])
                    } else {
                        None
                    };
                    let df = Decoder::compile(f, opt_next)?;
                    dfields.push(df);
                }
                Ok(Decoder::Tuple(dfields))
            }
            Format::Record(fields) => {
                let mut dfields = Vec::new();
                for i in 0..fields.len() {
                    let (name, f) = &fields[i];
                    let opt_next = if i + 1 < fields.len() {
                        Some(&fields[i + 1].1)
                    } else {
                        None
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
                    Ok(Decoder::While(Lookahead::empty(), da))
                }
            }
            Format::Array(index, a) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                Ok(Decoder::Array(*index, da))
            }
            Format::Map(f, a) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                Ok(Decoder::Map(*f, da))
            }
        }
    }

    pub fn parse(&self, stack: &mut Vec<Value>, input: &[u8]) -> Option<(usize, Value)> {
        match self {
            Decoder::Zero => None,
            Decoder::Unit => Some((0, Value::Unit)),
            Decoder::Byte(bs) => {
                if input.len() > 0 {
                    if bs.contains(input[0]) {
                        Some((1, Value::U8(input[0])))
                    } else {
                        None
                    }
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
                if let Some((ca, va)) = a.parse(stack, input) {
                    stack.push(va);
                    if let Some((cb, vb)) = b.parse(stack, &input[ca..]) {
                        let va = stack.pop().unwrap();
                        Some((ca + cb, Value::Pair(Box::new(va), Box::new(vb))))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Decoder::Tuple(fields) => {
                let mut c = 0;
                let mut v = Vec::new();
                for f in fields {
                    let (cf, vf) = f.parse(stack, &input[c..])?;
                    c += cf;
                    v.push(vf.clone());
                    stack.push(vf);
                }
                for _ in fields {
                    stack.pop();
                }
                Some((c, Value::Seq(v)))
            }
            Decoder::Record(fields) => {
                let mut c = 0;
                let mut v = Vec::new();
                for (name, f) in fields {
                    let (cf, vf) = f.parse(stack, &input[c..])?;
                    c += cf;
                    v.push((name.clone(), vf.clone()));
                    stack.push(vf);
                }
                for _ in fields {
                    stack.pop();
                }
                Some((c, Value::Record(v)))
            }
            Decoder::While(look, a) => {
                let mut c = 0;
                let mut v = Vec::new();
                while look.matches(input) {
                    if let Some((ca, va)) = a.parse(stack, &input[c..]) {
                        c += ca;
                        v.push(va);
                    } else {
                        return None;
                    }
                }
                Some((c, Value::Seq(v)))
            }
            Decoder::Until(look, a) => {
                let mut c = 0;
                let mut v = Vec::new();
                while !look.matches(&input[c..]) {
                    if let Some((ca, va)) = a.parse(stack, &input[c..]) {
                        c += ca;
                        v.push(va);
                    } else {
                        return None;
                    }
                }
                Some((c, Value::Seq(v)))
            }
            Decoder::Array(index, a) => {
                let mut c = 0;
                let mut v = Vec::new();
                let count = stack[stack.len() - index - 1].usize_or_panic();
                for _i in 0..count {
                    if let Some((ca, va)) = a.parse(stack, &input[c..]) {
                        c += ca;
                        v.push(va);
                    } else {
                        return None;
                    }
                }
                Some((c, Value::Seq(v)))
            }
            Decoder::Map(f, a) => {
                if let Some((ca, va)) = a.parse(stack, input) {
                    Some((ca, f(&va)))
                } else {
                    None
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read("test.jpg")?;

    fn marker(b: u8) -> Format {
        Format::Cat(
            Box::new(Format::Byte(ByteSet::Is(0xFF))),
            Box::new(Format::Byte(ByteSet::Is(b))),
        )
    }

    let soi = marker(0xD8);
    let eoi = marker(0xD9);
    let length = Format::Map(
        Value::map_u16be_minus_two,
        Box::new(Format::Cat(
            Box::new(Format::Byte(ByteSet::Any)),
            Box::new(Format::Byte(ByteSet::Any)),
        )),
    );
    let var = Format::Record(vec![
        ("length".to_string(), length.clone()),
        (
            "data".to_string(),
            Format::Array(0, Box::new(Format::Byte(ByteSet::Any))),
        ),
    ]);
    let app0 = Format::Cat(Box::new(marker(0xE0)), Box::new(var.clone()));
    let dqt = Format::Cat(Box::new(marker(0xDB)), Box::new(var.clone()));
    let sof0 = Format::Cat(Box::new(marker(0xC0)), Box::new(var.clone()));
    let dht = Format::Cat(Box::new(marker(0xC4)), Box::new(var.clone()));
    let chunk = Format::Alt(
        Box::new(dqt.clone()),
        Box::new(Format::Alt(Box::new(sof0.clone()), Box::new(dht.clone()))),
    );
    let sos = Format::Cat(Box::new(marker(0xDA)), Box::new(var.clone()));
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
    let jpeg = Format::Record(vec![
        ("soi".to_string(), soi),
        ("app0".to_string(), app0),
        ("chunks".to_string(), Format::Star(Box::new(chunk.clone()))),
        ("sos".to_string(), sos),
        ("ecs".to_string(), ecs),
        ("eoi".to_string(), eoi),
    ]);
    let det_jpeg = Decoder::compile(&jpeg, None)?;
    let mut stack = Vec::new();
    let res = det_jpeg.parse(&mut stack, &input);
    println!("{:?}", res);
    Ok(())
}
