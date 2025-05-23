use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use serde::Serialize;

use crate::{
    matchtree::{MatchTree, Next}, Arith, Expr, Format, FormatDecl, FormatId, FormatModule, FormatType, IntRel, Label, RecId, RecurseCtx, Span, Unary
};
use anyhow::{Result as AResult, anyhow};
use doodle::{IntWidth, byte_set::ByteSet, read::ReadCtxt};

#[derive(Debug, Clone, Serialize)]
pub enum Value {
    // Primitive values
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Bool(bool),
    Char(char),

    // Shape-based values
    Tuple(Vec<Value>),
    Seq(Vec<Value>),
    Option(Option<Box<Value>>),
    Variant(Label, Box<Value>),
}

impl Value {
    fn get_usize_with_precision(&self) -> (usize, IntWidth) {
        match self {
            Value::U8(n) => (*n as usize, IntWidth::Bits8),
            Value::U16(n) => (*n as usize, IntWidth::Bits16),
            Value::U32(n) => (*n as usize, IntWidth::Bits32),
            Value::U64(n) => (*n as usize, IntWidth::Bits64),
            _ => panic!("value is not a number: {self:?}"),
        }
    }

    fn unwrap_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!("value is not a bool"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub decoders: Vec<(Decoder, FormatType)>,
}

impl Program {
    fn new() -> Self {
        let decoders = Vec::new();
        Program { decoders }
    }

    pub fn run<'input>(&self, input: ReadCtxt<'input>) -> DecodeResult<(Value, ReadCtxt<'input>)> {
        self.decoders[0].0.parse(self, input)
    }
}

type Batch = Option<Span<usize>>;

pub struct Compiler<'a> {
    module: &'a FormatModule,
    program: Program,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<(&'a Format, Rc<Next<'a>>, usize, Batch)>,
}

impl<'a> Compiler<'a> {
    fn new(module: &'a FormatModule) -> Self {
        let program = Program::new();
        let decoder_map = HashMap::new();
        let compile_queue = Vec::new();
        Compiler {
            module,
            program,
            decoder_map,
            compile_queue,
        }
    }

    pub fn compile_program(
        module: &FormatModule,
        format: &Format,
        ctx: RecurseCtx,
    ) -> AResult<Program> {
        let mut compiler = Compiler::new(module);

        let mut visited = HashSet::new();

        let batch = ctx.as_span();

        let t = format.infer_type(&mut visited, module, batch)?;
        compiler.queue_compile(t, format, Rc::new(Next::Empty), batch);
        while let Some((f, next, n, batch)) = compiler.compile_queue.pop() {
            let f_ctx = match batch {
                Some(span) => {
                    RecurseCtx::Recurse {
                        span,
                        batch: &module.decls[span.start..=span.end],
                        entry_id: n - span.start,
                    }
                }
                None => RecurseCtx::NonRec,
            };
            let d = compiler.compile_format(f, next, f_ctx)?;
            compiler.program.decoders[n].0 = d;
        }
        Ok(compiler.program)
    }

    fn queue_compile(
        &mut self,
        t: FormatType,
        f: &'a Format,
        next: Rc<Next<'a>>,
        batch: Option<Span<usize>>,
    ) -> usize {
        let n = self.program.decoders.len();
        self.program.decoders.push((Decoder::FAIL, t));
        self.compile_queue.push((f, next, n, batch));
        n
    }

    fn queue_compile_batch(
        &mut self,
        decls: &'a [FormatDecl],
        which_next: RecId,
        next: Rc<Next<'a>>,
        span: Span<FormatId>,
    ) -> usize {
        let n = self.program.decoders.len();
        for (ix, d) in decls.into_iter().enumerate() {
            let t = d.solve_type(self.module).unwrap().clone();
            self.program.decoders.push((Decoder::FAIL, t));
            let next = if ix == which_next {
                next.clone()
            } else {
                Rc::new(Next::Empty)
            };
            self.compile_queue.push((&d.format, next, n + ix, Some(span)));
        }
        n + which_next
    }

    pub fn compile_one(format: &Format) -> AResult<Decoder> {
        let module = FormatModule::new();
        let mut compiler = Compiler::new(&module);
        let ctx = RecurseCtx::NonRec;
        compiler.compile_format(format, Rc::new(Next::Empty), ctx)
    }

    fn compile_format(
        &mut self,
        format: &'a Format,
        next: Rc<Next<'a>>,
        ctx: RecurseCtx<'a>,
    ) -> AResult<Decoder> {
        match format {
            Format::ItemVar(level) => {
                let f = self.module.get_format(*level);
                let next = if f.depends_on_next(self.module, ctx) {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = self.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let t = self.module.get_format_type(*level).clone();
                    let n = match self.module.get_batch(*level) {
                        Some(span) => {
                            let batch = &self.module.decls[span.start..=span.end];
                            self.queue_compile_batch(batch, level - span.start, next.clone(), span)
                        }
                        None => {
                            self.queue_compile(t, f, next.clone(), None)
                        }
                    };
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                Ok(Decoder::Call(n))
            }
            Format::RecVar(batch_ix) => {
                let (new_ctx, _) = ctx.enter(*batch_ix);
                let level = new_ctx.get_level().unwrap();
                // REVIEW - do we need to do any work here?
                Ok(Decoder::CallRec(level, *batch_ix))
            }
            Format::FailWith(msg) => Ok(Decoder::FailWith(msg.clone())),
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::Variant(label, f) => {
                let d = self.compile_format(f, next.clone(), ctx)?;
                Ok(Decoder::Variant(label.clone(), Box::new(d)))
            }
            Format::Compute(expr) => Ok(Decoder::Compute(expr.clone())),
            Format::Union(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_format(f, next.clone(), ctx)?);
                }
                if let Some(tree) = MatchTree::build(self.module, branches, next, ctx) {
                    Ok(Decoder::Branch(tree, ds))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            Format::Tuple(elems) => {
                let mut decs = Vec::with_capacity(elems.len());
                let mut fields = elems.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(fields.as_slice(), next.clone()));
                    let df = self.compile_format(f, next, ctx)?;
                    decs.push(df);
                }
                Ok(Decoder::Tuple(decs))
            }
            Format::Seq(elems) => {
                let mut decs = Vec::with_capacity(elems.len());
                let mut fields = elems.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(fields.as_slice(), next.clone()));
                    let df = self.compile_format(f, next, ctx)?;
                    decs.push(df);
                }
                Ok(Decoder::Seq(decs))
            }
            Format::Repeat(a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_format(a, Rc::new(Next::Repeat(a, next.clone())), ctx)?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                if let Some(tree) = MatchTree::build(self.module, &[fa, fb], next, ctx) {
                    Ok(Decoder::While(tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            Format::Maybe(x, a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty), ctx)?);
                Ok(Decoder::Maybe(x.clone(), da))
            }
        }
    }
}

impl Expr {
    pub fn eval(&self) -> Value {
        match self {
            Expr::U8(i) => Value::U8(*i),
            Expr::U16(i) => Value::U16(*i),
            Expr::U32(i) => Value::U32(*i),
            Expr::U64(i) => Value::U64(*i),
            Expr::Bool(b) => Value::Bool(*b),

            Expr::AsChar(expr) => match expr.eval() {
                Value::U8(x) => Value::Char(char::from(x)),
                Value::U16(x) => {
                    Value::Char(char::from_u32(x as u32).unwrap_or(char::REPLACEMENT_CHARACTER))
                }
                Value::U32(x) => {
                    Value::Char(char::from_u32(x).unwrap_or(char::REPLACEMENT_CHARACTER))
                }
                Value::U64(x) => Value::Char(
                    char::from_u32(u32::try_from(x).unwrap())
                        .unwrap_or(char::REPLACEMENT_CHARACTER),
                ),
                _ => panic!("AsChar: expected U8, U16, U32, or U64"),
            },
            Expr::AsU8(x) => {
                match x.eval() {
                    Value::U8(x) => Value::U8(x),
                    Value::U16(x) => Value::U8(u8::try_from(x).unwrap_or_else(|err| {
                        panic!("cannot perform AsU8 cast on u16 {x}: {err}")
                    })),
                    Value::U32(x) => Value::U8(u8::try_from(x).unwrap_or_else(|err| {
                        panic!("cannot perform AsU8 cast on u32 {x}: {err}")
                    })),
                    Value::U64(x) => Value::U8(u8::try_from(x).unwrap_or_else(|err| {
                        panic!("cannot perform AsU8 cast on u64 {x}: {err}")
                    })),
                    x => panic!("cannot convert {x:?} to U8"),
                }
            }

            Expr::AsU16(x) => match x.eval() {
                Value::U8(x) => Value::U16(u16::from(x)),
                Value::U16(x) => Value::U16(x),
                Value::U32(x) => Value::U16(u16::try_from(x).unwrap()),
                Value::U64(x) => Value::U16(u16::try_from(x).unwrap()),
                x => panic!("cannot convert {x:?} to U16"),
            },
            Expr::AsU32(x) => match x.eval() {
                Value::U8(x) => Value::U32(u32::from(x)),
                Value::U16(x) => Value::U32(u32::from(x)),
                Value::U32(x) => Value::U32(x),
                Value::U64(x) => Value::U32(u32::try_from(x).unwrap()),
                x => panic!("cannot convert {x:?} to U32"),
            },
            Expr::AsU64(x) => match x.eval() {
                Value::U8(x) => Value::U64(u64::from(x)),
                Value::U16(x) => Value::U64(u64::from(x)),
                Value::U32(x) => Value::U64(u64::from(x)),
                Value::U64(x) => Value::U64(x),
                x => panic!("cannot convert {x:?} to U64"),
            },
            Expr::Seq(exprs) => Value::Seq(exprs.iter().map(Expr::eval).collect()),
            Expr::Tuple(exprs) => Value::Tuple(exprs.iter().map(Expr::eval).collect()),
            Expr::LiftOption(None) => Value::Option(None),
            Expr::LiftOption(Some(expr)) => Value::Option(Some(Box::new(expr.eval()))),
            Expr::Variant(lab, expr) => Value::Variant(lab.clone(), Box::new(expr.eval())),
            Expr::IntRel(rel, lhs, rhs) => {
                let lhs = lhs.eval();
                let rhs = rhs.eval();
                let (l, _lw) = lhs.get_usize_with_precision();
                let (r, _rw) = rhs.get_usize_with_precision();
                if _lw != _rw {
                    panic!("cannot compare {lhs:?} with {rhs:?}");
                }
                match rel {
                    IntRel::Eq => Value::Bool(l == r),
                    IntRel::Lt => Value::Bool(l < r),
                    IntRel::Gt => Value::Bool(l > r),
                    IntRel::Neq => Value::Bool(l != r),
                    IntRel::Lte => Value::Bool(l <= r),
                    IntRel::Gte => Value::Bool(l >= r),
                }
            }
            Expr::Arith(Arith::Add, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l.checked_add(r).unwrap()),
                (Value::U16(l), Value::U16(r)) => Value::U16(l.checked_add(r).unwrap()),
                (Value::U32(l), Value::U32(r)) => Value::U32(l.checked_add(r).unwrap()),
                (Value::U64(l), Value::U64(r)) => Value::U64(l.checked_add(r).unwrap()),
                (l, r) => panic!("cannot add {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::Sub, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l.checked_sub(r).unwrap()),
                (Value::U16(l), Value::U16(r)) => Value::U16(l.checked_sub(r).unwrap()),
                (Value::U32(l), Value::U32(r)) => Value::U32(l.checked_sub(r).unwrap()),
                (Value::U64(l), Value::U64(r)) => Value::U64(l.checked_sub(r).unwrap()),
                (l, r) => panic!("cannot subtract {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::Mul, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l.checked_mul(r).unwrap()),
                (Value::U16(l), Value::U16(r)) => Value::U16(l.checked_mul(r).unwrap()),
                (Value::U32(l), Value::U32(r)) => Value::U32(l.checked_mul(r).unwrap()),
                (Value::U64(l), Value::U64(r)) => Value::U64(l.checked_mul(r).unwrap()),
                (l, r) => panic!("cannot multiply {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::Div, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l.checked_div(r).unwrap()),
                (Value::U16(l), Value::U16(r)) => Value::U16(l.checked_div(r).unwrap()),
                (Value::U32(l), Value::U32(r)) => Value::U32(l.checked_div(r).unwrap()),
                (Value::U64(l), Value::U64(r)) => Value::U64(l.checked_div(r).unwrap()),
                (l, r) => panic!("cannot divide {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::Rem, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l.checked_rem(r).unwrap()),
                (Value::U16(l), Value::U16(r)) => Value::U16(l.checked_rem(r).unwrap()),
                (Value::U32(l), Value::U32(r)) => Value::U32(l.checked_rem(r).unwrap()),
                (Value::U64(l), Value::U64(r)) => Value::U64(l.checked_rem(r).unwrap()),
                (l, r) => panic!("cannot compute remainder {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::BitAnd, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l & r),
                (Value::U16(l), Value::U16(r)) => Value::U16(l & r),
                (Value::U32(l), Value::U32(r)) => Value::U32(l & r),
                (Value::U64(l), Value::U64(r)) => Value::U64(l & r),
                (l, r) => panic!("cannot bitwise and {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::BitOr, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l | r),
                (Value::U16(l), Value::U16(r)) => Value::U16(l | r),
                (Value::U32(l), Value::U32(r)) => Value::U32(l | r),
                (Value::U64(l), Value::U64(r)) => Value::U64(l | r),
                (l, r) => panic!("cannot bitwise or {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::Shl, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l.checked_shl(r as u32).unwrap()),
                (Value::U16(l), Value::U16(r)) => Value::U16(l.checked_shl(r as u32).unwrap()),
                (Value::U32(l), Value::U32(r)) => Value::U32(l.checked_shl(r).unwrap()),
                (Value::U64(l), Value::U64(r)) => {
                    Value::U64(l.checked_shl(u32::try_from(r).unwrap()).unwrap())
                }
                (l, r) => panic!("cannot shift left {l:?} and {r:?}"),
            },
            Expr::Arith(Arith::Shr, lhs, rhs) => match (lhs.eval(), rhs.eval()) {
                (Value::U8(l), Value::U8(r)) => Value::U8(l.checked_shr(r as u32).unwrap()),
                (Value::U16(l), Value::U16(r)) => Value::U16(l.checked_shr(r as u32).unwrap()),
                (Value::U32(l), Value::U32(r)) => Value::U32(l.checked_shr(r).unwrap()),
                (Value::U64(l), Value::U64(r)) => {
                    Value::U64(l.checked_shr(u32::try_from(r).unwrap()).unwrap())
                }
                (l, r) => panic!("cannot shift right {l:?} and {r:?}"),
            },
            Expr::Unary(Unary::BoolNot, expr) => match expr.eval() {
                Value::Bool(x) => Value::Bool(!x),
                x => panic!("cannot negate {x:?}"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Decoder {
    Call(FormatId),
    CallRec(FormatId, RecId),

    FailWith(Label),
    EndOfInput,
    Byte(ByteSet),
    Compute(Box<Expr>),

    Variant(Label, Box<Decoder>),
    Branch(MatchTree, Vec<Decoder>),

    While(MatchTree, Box<Decoder>), // Repeat decoder while input matches

    Seq(Vec<Decoder>),
    Tuple(Vec<Decoder>),
    Maybe(Box<Expr>, Box<Decoder>),
}

pub(crate) mod error;
use error::DecodeError;

pub type DecodeResult<T> = Result<T, DecodeError>;

impl Decoder {
    pub(crate) const FAIL: Self = Decoder::FailWith(Label::Borrowed("FAIL_CONST"));

    pub fn parse<'input>(
        &self,
        program: &Program,
        input: ReadCtxt<'input>,
    ) -> DecodeResult<(Value, ReadCtxt<'input>)> {
        match self {
            Decoder::FailWith(msg) => Err(DecodeError::fail(msg.clone(), input)),
            Decoder::EndOfInput => match input.read_byte() {
                None => Ok((Value::Tuple(vec![]), input)),
                Some((b, _)) => Err(DecodeError::Trailing {
                    byte: b,
                    offset: input.offset,
                }),
            },
            Decoder::Byte(bs) => {
                let (b, input) = input.read_byte().ok_or(DecodeError::Overbyte {
                    offset: input.offset,
                })?;
                if bs.contains(b) {
                    Ok((Value::U8(b), input))
                } else {
                    Err(DecodeError::Unexpected {
                        found: b,
                        expected: *bs,
                        offset: input.offset,
                    })
                }
            }
            Decoder::Call(ix) => program.decoders[*ix].0.parse(program, input),
            Decoder::CallRec(level, _) => program.decoders[*level].0.parse(program, input),
            Decoder::Compute(expr) => {
                let v = expr.eval();
                Ok((v, input))
            }
            Decoder::Variant(lab, da) => {
                let (v, input) = da.parse(program, input)?;
                Ok((Value::Variant(lab.clone(), Box::new(v)), input))
            }
            Decoder::Branch(tree, branches) => {
                let index = tree.matches(input).ok_or(DecodeError::NoValidBranch {
                    offset: input.offset,
                })?;
                let d = &branches[index];
                // let (v, input) = d.parse(program, input)?;
                // Ok(Value::Branch(index, Box::new(v)), input))
                d.parse(program, input)
            }
            Decoder::Seq(decs) => {
                let mut input = input;
                let mut v = Vec::with_capacity(decs.len());
                for d in decs {
                    let (va, next_input) = d.parse(program, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::Tuple(decs) => {
                let mut input = input;
                let mut v = Vec::with_capacity(decs.len());
                for d in decs {
                    let (va, next_input) = d.parse(program, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Tuple(v), input))
            }
            Decoder::While(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while tree.matches(input).ok_or(DecodeError::NoValidBranch {
                    offset: input.offset,
                })? == 0
                {
                    let (va, next_input) = a.parse(program, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::Maybe(expr, a) => {
                let is_present = expr.eval().unwrap_bool();
                if is_present {
                    let (v, input) = a.parse(program, input)?;
                    Ok((Value::Option(Some(Box::new(v))), input))
                } else {
                    Ok((Value::Option(None), input))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::ser::to_string_pretty;

    #[test]
    fn not_actually_recursive() -> AResult<()> {
        let dead_end = Format::Byte(ByteSet::from_bits([1, 0, 0, 0]));
        let text = Format::Tuple(vec![
            Format::Repeat(Box::new(Format::Byte(ByteSet::from(0x01..=0x7f)))),
            Format::RecVar(0),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![
            (Label::Borrowed("text.null"), dead_end),
            (Label::Borrowed("text.cstring"), text),
        ]);
        let f = frefs[1].call();
        let program = Compiler::compile_program(&module, &f, RecurseCtx::NonRec)?;
        let input = ReadCtxt::new(b"hello world\x00");
        let (value, _) = program.run(input)?;
        eprintln!("{value:?}");
        Ok(())
    }

    #[test]
    fn auto_recursive() -> AResult<()> {
        let peano = Format::Union(vec![
            Format::Variant(Label::Borrowed("peanoZ"), Box::new(Format::Byte(ByteSet::from([b'Z'])))),
            Format::Variant(Label::Borrowed("peanoS"), Box::new(Format::Tuple(vec![Format::Byte(ByteSet::from([b'S'])), Format::RecVar(0)]))),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![
            (Label::Borrowed("test.peano"), peano),
        ]);
        let f = Format::Tuple(vec![frefs[0].call(), Format::EndOfInput]);
        let program = Compiler::compile_program(&module, &f, RecurseCtx::NonRec)?;
        let input = ReadCtxt::new(b"SSSSZ");
        let (value, _) = program.run(input)?;
        eprintln!("{}", to_string_pretty(&value).unwrap());
        Ok(())
    }
}
