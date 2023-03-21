#[derive(Copy, Clone, Debug)]
enum ByteSet {
    Any,
    Is(u8),
    Not(u8),
}

#[derive(Debug)]
enum Format {
    Zero,
    Unit,
    Byte(ByteSet),
    Alt(Box<Format>, Box<Format>),
    Cat(Box<Format>, Box<Format>),
    Repeat(Box<Format>, Box<Format>),
}

#[derive(Debug)]
struct Lookahead {
    pattern: Vec<ByteSet>,
}

#[derive(Debug)]
enum DetFormat {
    Zero,
    Unit,
    Byte(ByteSet),
    If(Lookahead, Box<DetFormat>, Box<DetFormat>),
    Cat(Box<DetFormat>, Box<DetFormat>),
    While(Lookahead, Box<DetFormat>),
    Until(Lookahead, Box<DetFormat>),
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
    /*
    pub fn parse(&self, input: &[u8]) -> Option<usize> {
        match self {
            Format::Zero => None,
            Format::Unit => Some(0),
            Format::Byte(bs) => {
                if input.len() > 0 {
                    if bs.contains(input[0]) {
                        Some(1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Format::Alt(a, b) => a.parse(input).or(b.parse(input)),
            Format::Cat(a, b) => {
                if let Some(ca) = a.parse(input) {
                    if let Some(cb) = b.parse(&input[ca..]) {
                        Some(ca + cb)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Format::Repeat(a, b) => {
                let mut c = 0;
                while let Some(ca) = a.parse(&input[c..]) {
                    c += ca;
                }
                if let Some(cb) = b.parse(&input[c..]) {
                    Some(c + cb)
                } else {
                    None
                }
            }
        }
    }
    */

    pub fn can_match_lookahead(&self, input: &[ByteSet]) -> Option<usize> {
        match self {
            Format::Zero => None,
            Format::Unit => Some(0),
            Format::Byte(bs) => {
                if input.len() > 0 {
                    if ByteSet::disjoint(bs, &input[0]) {
                        None
                    } else {
                        Some(1)
                    }
                } else {
                    Some(1)
                }
            }
            Format::Alt(a, b) => a
                .can_match_lookahead(input)
                .or(b.can_match_lookahead(input)),
            Format::Cat(a, b) => {
                if let Some(ca) = a.can_match_lookahead(input) {
                    if let Some(cb) = b.can_match_lookahead(&input[ca..]) {
                        Some(ca + cb)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Format::Repeat(a, b) => {
                let mut c = 0;
                while let Some(ca) = a.can_match_lookahead(&input[c..]) {
                    c += ca;
                }
                if let Some(cb) = b.can_match_lookahead(&input[c..]) {
                    Some(c + cb)
                } else {
                    None
                }
            }
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
        let pa = Lookahead::from(a, LEN)?;
        if b.can_match_lookahead(&pa.pattern).is_none() {
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

    pub fn from(f: &Format, len: usize) -> Option<Lookahead> {
        match f {
            Format::Zero => None,
            Format::Unit => Some(Lookahead::empty()),
            Format::Byte(bs) => Some(Lookahead::single(bs.clone())),
            Format::Alt(a, b) | Format::Repeat(a, b) => {
                if let Some(pa) = Lookahead::from(a, len) {
                    if let Some(pb) = Lookahead::from(b, len) {
                        Some(Lookahead::alt(&pa, &pb))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Format::Cat(a, b) => {
                if let Some(pa) = Lookahead::from(a, len) {
                    if pa.pattern.len() < len {
                        if let Some(pb) = Lookahead::from(b, len - pa.pattern.len()) {
                            Some(Lookahead::cat(&pa, &pb))
                        } else {
                            None
                        }
                    } else {
                        Some(pa)
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl DetFormat {
    pub fn compile(f: &Format) -> Result<DetFormat, String> {
        match f {
            Format::Zero => Ok(DetFormat::Zero),
            Format::Unit => Ok(DetFormat::Unit),
            Format::Byte(bs) => Ok(DetFormat::Byte(bs.clone())),
            Format::Alt(a, b) => {
                let da = Box::new(DetFormat::compile(a)?);
                let db = Box::new(DetFormat::compile(b)?);
                if let Some(l) = Lookahead::new(a, b) {
                    Ok(DetFormat::If(l, da, db))
                } else if let Some(l) = Lookahead::new(b, a) {
                    Ok(DetFormat::If(l, db, da))
                } else {
                    Err("cannot find valid lookahead for alt".to_string())
                }
            }
            Format::Cat(a, b) => {
                let da = Box::new(DetFormat::compile(a)?);
                let db = Box::new(DetFormat::compile(b)?);
                Ok(DetFormat::Cat(da, db))
            }
            Format::Repeat(a, b) => {
                let da = Box::new(DetFormat::compile(a)?);
                let db = Box::new(DetFormat::compile(b)?);
                if let Some(l) = Lookahead::new(a, b) {
                    Ok(DetFormat::Cat(Box::new(DetFormat::While(l, da)), db))
                } else if let Some(l) = Lookahead::new(b, a) {
                    Ok(DetFormat::Cat(Box::new(DetFormat::Until(l, da)), db))
                } else {
                    Err("cannot find valid lookahead for repeat".to_string())
                }
            }
        }
    }
}

fn main() -> Result<(), String> {
    let jpeg = Format::Repeat(
        Box::new(Format::Alt(
            Box::new(Format::Byte(ByteSet::Not(0xFF))),
            Box::new(Format::Cat(
                Box::new(Format::Byte(ByteSet::Is(0xFF))),
                Box::new(Format::Byte(ByteSet::Is(0x00))),
            )),
        )),
        Box::new(Format::Cat(
            Box::new(Format::Byte(ByteSet::Is(0xFF))),
            Box::new(Format::Byte(ByteSet::Is(0xD9))),
        )),
    );
    let det_jpeg = DetFormat::compile(&jpeg)?;
    println!("{:?}", det_jpeg);
    Ok(())
}
