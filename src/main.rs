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
    /// A format that always fails
    Fail,
    /// A format that always succeeds, consuming no input
    Unit,
    /// A format that succeeds if it matches the given byte set
    Byte(ByteSet),
    /// A format that succeeds if either format succeeds
    Alt(Box<Format>, Box<Format>),
    /// The concatenation of two formats where the second format can depend on
    /// the decoded value of the first format
    Cat(Box<Format>, Box<Format>),
    /// A sequence of formats where later formats can depend on the
    /// decoded value of earlier formats
    Tuple(Vec<Format>),
    /// A sequence of named formats where later formats can depend on the
    /// decoded value of earlier formats
    Record(Vec<(String, Format)>),
    /// Repeat a format zero-or-more times
    Repeat(Box<Format>),
    /// Repeat a format an exact number of times
    RepeatCount(Expr, Box<Format>),
    /// Restrict a format to a sub-stream of a given number of bytes
    Slice(Expr, Box<Format>),
    /// Transform a decoded value with a function
    Map(fn(&Value) -> Value, Box<Format>), // TODO: Decouple from `Value`
}

#[derive(Debug)]
struct Lookahead {
    pattern: Vec<ByteSet>,
}

/// Decoders with a fixed amount of lookahead
enum Decoder {
    Fail,
    Unit,
    Byte(ByteSet),
    If(Lookahead, Box<Decoder>, Box<Decoder>),
    Cat(Box<Decoder>, Box<Decoder>),
    Tuple(Vec<Decoder>),
    Record(Vec<(String, Decoder)>),
    While(Lookahead, Box<Decoder>),
    Until(Lookahead, Box<Decoder>),
    RepeatCount(Expr, Box<Decoder>),
    Slice(Expr, Box<Decoder>),
    Map(fn(&Value) -> Value, Box<Decoder>), // TODO: Decouple from `Value`
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
            Format::Fail => false,
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
            Format::Repeat(_a) => {
                true // FIXME
            }
            Format::RepeatCount(_expr, _a) => {
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
        pattern.extend(b.pattern.iter());
        Lookahead { pattern }
    }

    pub fn new(a: &Format, b: &Format) -> Option<Lookahead> {
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
            Format::Fail => None,
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
                None => Lookahead::from(&next, len, Format::Unit),
                Some((a, fields)) => Lookahead::from(a, len, Format::Tuple(fields.to_vec())),
            },
            Format::Record(fields) => match fields.split_first() {
                None => Lookahead::from(&next, len, Format::Unit),
                Some(((_, a), fields)) => Lookahead::from(a, len, Format::Record(fields.to_vec())),
            },
            Format::Repeat(_a) => {
                Some(Lookahead::empty()) // FIXME ?
            }
            Format::RepeatCount(_expr, _a) => {
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
            Format::Fail => Ok(Decoder::Fail),
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
            Format::Repeat(a) => {
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
            Format::RepeatCount(expr, a) => {
                let da = Box::new(Decoder::compile(a, opt_next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
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
            Decoder::Fail => None,
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
                    let (va, next_input) = a.parse(stack, input).unwrap();
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
                Some((f(&va), input))
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
    repeat(Format::Byte(ByteSet::Any))
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

    // SOF: Frame header (See ITU T.81 Section B.2.2)
    let sof_data = Format::Record(vec![
        ("sample-precision".to_string(), u8()),
        ("num-lines".to_string(), u16be()),
        ("num-samples-per-line".to_string(), u16be()),
        ("num-image-components".to_string(), u8()),
        (
            "image-components".to_string(),
            repeat_count(
                Expr::Var(0), // num-image-components
                Format::Record(vec![
                    ("id".to_string(), u8()),
                    ("sampling-factor".to_string(), u8()), // { horizontal <- u4, vertical <- u4 }
                    ("quantization-table-id".to_string(), u8()),
                ]),
            ),
        ),
    ]);

    // DHT: Define Huffman table (See ITU T.81 Section B.2.4.2)
    let dht_data = repeat(Format::Record(vec![
        // class <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("class-table-id".to_string(), u8()),
        ("num-codes".to_string(), repeat_count(Expr::U8(16), u8())),
        ("values".to_string(), any_bytes()), // List.map num-codes (\n => repeat-count n u8);
    ]));

    // DAC: Define arithmetic conditioning table (See ITU T.81 Section B.2.4.3)
    let dac_data = repeat(Format::Record(vec![
        // class <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("class-table-id".to_string(), u8()),
        ("value".to_string(), u8()),
    ]));

    // SOS: Scan header (See ITU T.81 Section B.2.3)
    let sos_data = Format::Record(vec![
        ("num-image-components".to_string(), u8()), // 1 |..| 4
        (
            "image-components".to_string(),
            repeat_count(
                Expr::Var(0), // num-image-components
                Format::Record(vec![
                    ("component-selector".to_string(), u8()), // ???
                    // dc-entropy-coding-table-id <- u4;
                    // ac-entropy-coding-table-id <- u4;
                    ("entropy-coding-table-ids".to_string(), u8()),
                ]),
            ),
        ),
        ("start-spectral-selection".to_string(), u8()), // ???
        ("end-spectral-selection".to_string(), u8()),   // ???
        ("approximation-bit-position".to_string(), u8()), // { high <- u4, low <- u4 }
    ]);

    // DQT: Define quantization table  (See ITU T.81 Section B.2.4.1)
    let dqt_data = Format::Record(vec![
        // precision <- u4 //= 0 | 1;
        // table-id <- u4 //= 1 |..| 4;
        ("precision-table-id".to_string(), u8()),
        // elements <- match precision {
        //   0 => repeat-count 64 u8,
        //   1 => repeat-count 64 u16be,
        // };
        ("elements".to_string(), any_bytes()),
    ]);

    // APP0: Application segment 0
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
            repeat_count(
                Expr::Var(0), // thumbnail-height
                repeat_count(
                    Expr::Var(1), // thumbnail-width
                    Format::Record(vec![
                        ("r".to_string(), u8()),
                        ("g".to_string(), u8()),
                        ("b".to_string(), u8()),
                    ]),
                ),
            ),
        ),
    ]);

    let sof0 = marker_segment(0xC0, sof_data.clone()); // Start of frame (baseline jpeg)
    let _sof1 = marker_segment(0xC1, sof_data.clone()); // Start of frame (extended sequential, huffman)
    let _sof2 = marker_segment(0xC2, sof_data.clone()); // Start of frame (progressive, huffman)
    let _sof3 = marker_segment(0xC3, sof_data.clone()); // Start of frame (lossless, huffman)
    let dht = marker_segment(0xC4, dht_data.clone()); // Define Huffman Table
    let _sof5 = marker_segment(0xC5, sof_data.clone()); // Start of frame (differential sequential, huffman)
    let _sof6 = marker_segment(0xC6, sof_data.clone()); // Start of frame (differential progressive, huffman)
    let _sof7 = marker_segment(0xC7, sof_data.clone()); // Start of frame (differential lossless, huffman)
    let _jpeg = marker_segment(0xC8, any_bytes()); // Reserved for JPEG extension
    let _sof9 = marker_segment(0xC9, sof_data.clone()); // Start of frame (extended sequential, arithmetic)
    let _sof10 = marker_segment(0xCA, sof_data.clone()); // Start of frame (progressive, arithmetic)
    let _sof11 = marker_segment(0xCB, sof_data.clone()); // Start of frame (lossless, arithmetic)
    let dac = marker_segment(0xCC, dac_data.clone()); // Define arithmetic coding conditioning
    let _sof13 = marker_segment(0xCD, sof_data.clone()); // Start of frame (differential sequential, arithmetic)
    let _sof14 = marker_segment(0xCE, sof_data.clone()); // Start of frame (differential progressive, arithmetic)
    let _sof15 = marker_segment(0xCF, sof_data.clone()); // Start of frame (differential lossless, arithmetic)
    let soi = marker(0xD8); // Start of image
    let eoi = marker(0xD9); // End of of image
    let sos = marker_segment(0xDA, sos_data.clone()); // Start of scan
    let dqt = marker_segment(0xDB, dqt_data.clone()); // Define quantization table
    let _dnl = marker_segment(0xDC, any_bytes()); // Define number of lines
    let dri = marker_segment(0xDD, any_bytes()); // Define restart interval
    let app0 = marker_segment(0xE0, app0_data.clone()); // Application segment 0 (JFIF (len >=14) / JFXX (len >= 6) / AVI MJPEG)
    let app1 = marker_segment(0xE1, any_bytes()); // EXIF
    let app2 = marker_segment(0xE2, any_bytes()); // FlashPix / ICC
    let app13 = marker_segment(0xED, any_bytes()); // PhotoShop Save As
    let app14 = marker_segment(0xEE, any_bytes()); // Adobe
    let app15 = marker_segment(0xEF, any_bytes()); // GraphicConverter
    let com = marker_segment(0xFE, any_bytes()); // Extension data (comment)

    let table_or_misc = alts([
        dqt.clone(), // Define quantization table
        dht.clone(), // Define Huffman Table
        dac.clone(), // Define arithmetic coding conditioning
        dri.clone(), // Define restart interval
        app0.clone(),
        app1.clone(),
        app2.clone(),
        // TODO: app3..app12
        app13.clone(),
        app14.clone(),
        app15.clone(),
        com.clone(), // Comment
    ]);

    let frame_header = alts([
        sof0.clone(),
        // TODO: Error: "cannot find valid lookahead for star"
        // sof1.clone(),
        // sof2.clone(),
        // sof3.clone(),
        // sof5.clone(),
        // sof6.clone(),
        // sof7.clone(),
        // sof9.clone(),
        // sof10.clone(),
        // sof11.clone(),
        // sof13.clone(),
        // sof14.clone(),
        // sof15.clone(),
    ]);

    // TODO: Restart markers (rst0-rst7)
    let ecs = repeat(alts([
        Format::Byte(ByteSet::Not(0xFF)),
        Format::Map(
            |_| Value::U8(0xFF),
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0x00))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 0
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD0))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 1
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD1))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 2
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD2))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 3
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD3))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 4
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD4))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 5
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD5))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 6
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD6))),
            )),
        ),
        Format::Map(
            |_| Value::U8(0xFF), // FIXME reset marker 7
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0xD7))),
            )),
        ),
    ]));

    let scan = Format::Record(vec![
        ("segments".to_string(), repeat(table_or_misc.clone())),
        ("sos".to_string(), sos),
        ("ecs".to_string(), ecs),
    ]);

    let frame = Format::Record(vec![
        ("app".to_string(), alts([app0, app1])),
        ("segments".to_string(), repeat(table_or_misc)),
        ("header".to_string(), frame_header),
        ("scan".to_string(), scan.clone()),
        // TODO: ("dnl".to_string(), optional(dnl)), // Error: "cannot find valid lookahead for star"
        // TODO: ("scans".to_string(), repeat(scan)), // Error: "cannot find valid lookahead for star"
    ]);

    let jpeg = Format::Record(vec![
        ("soi".to_string(), soi),
        ("frame".to_string(), frame),
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
