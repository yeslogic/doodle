use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    Arith, Expr, Format, FormatDecl, FormatId, FormatModule, FormatType, IntRel, Label, RecId,
    RecurseCtx, Span, Unary,
    matchtree::{MatchTree, Next},
};
use anyhow::{Result as AResult, anyhow};
use doodle::{byte_set::ByteSet, read::ReadCtxt};

/// Local stand-in for `doodle::IntWidth`, which no longer exists upstream (replaced by the
/// `src/numeric/` engine as part of refreshing this branch's copy of `doodle` from `main`).
/// `doodle-rec`'s own `Expr`/`FormatType` numeric model is still a fixed 4-width scheme, so this
/// is only needed locally to tag operand widths for the precision-match check in `Expr::eval`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

mod ll1;
pub use ll1::LL1Interpreter;

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

    Branch(usize, Box<Value>),
}

const MAX_SEQ_LEN: usize = 64;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::U8(i) => write!(f, "{}", i),
            Value::U16(i) => write!(f, "{}", i),
            Value::U32(i) => write!(f, "{}", i),
            Value::U64(i) => write!(f, "{}", i),
            Value::Char(c) => write!(f, "{:?}", c),

            Value::Option(v) => match v {
                None => write!(f, "None"),
                Some(v) => write!(f, "Some({})", v),
            },
            Value::Tuple(vs) => {
                write!(
                    f,
                    "({})",
                    vs.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Variant(label, value) => {
                write!(f, "`{}({})", label, value)
            }
            Value::Seq(vs) => {
                if vs.len() > MAX_SEQ_LEN {
                    write!(f, "[...; {}]", vs.len())
                } else {
                    write!(
                        f,
                        "[{}]",
                        vs.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            Value::Branch(_, v) => write!(f, "{}", v),
        }
    }
}

impl Value {
    pub fn coerce_value(&self) -> &Value {
        match self {
            Value::Branch(_, v) => v.coerce_value(),
            _ => self,
        }
    }

    fn get_usize_with_precision(&self) -> (usize, IntWidth) {
        match self {
            Value::U8(n) => (*n as usize, IntWidth::Bits8),
            Value::U16(n) => (*n as usize, IntWidth::Bits16),
            Value::U32(n) => (*n as usize, IntWidth::Bits32),
            Value::U64(n) => (*n as usize, IntWidth::Bits64),
            _ => panic!("value is not a number: {self:?}"),
        }
    }

    pub(crate) fn unwrap_bool(&self) -> bool {
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

pub(crate) type Batch = Option<Span<usize>>;

pub struct Compiler<'a> {
    module: &'a FormatModule,
    program: Program,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<(&'a Format, Rc<Next<'a>>, usize, Batch, Option<usize>)>,
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
        compiler.queue_compile(t, format, Rc::new(Next::Empty), batch, None);
        while let Some((f, next, n, batch, batch_slot_start)) = compiler.compile_queue.pop() {
            let f_ctx = match batch {
                Some(span) => RecurseCtx::Recurse {
                    span,
                    batch: &module.decls[span.start..=span.end],
                    entry_id: n - span.start,
                },
                None => RecurseCtx::NonRec,
            };
            let d = compiler.compile_format(f, next, f_ctx, batch_slot_start)?;
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
        batch_slot_start: Option<usize>,
    ) -> usize {
        let n = self.program.decoders.len();
        self.program.decoders.push((Decoder::FAIL, t));
        self.compile_queue
            .push((f, next, n, batch, batch_slot_start));
        n
    }

    /// Queues every member of a recursive batch for compilation, all sharing one contiguous run
    /// of `Program::decoders` slots starting at the returned instantiation's own `n` (queued as
    /// each member's `batch_slot_start`) - `Format::RecVar(batch_ix)` later resolves directly to
    /// `batch_slot_start + batch_ix` (see `compile_format`'s `RecVar` arm), rather than through a
    /// lookup table, specifically so that two independent instantiations of the same batch (under
    /// different `next` continuations - the same batch can be queued here more than once, since
    /// `Format::depends_on_next` can make two `ItemVar` call sites key `decoder_map` differently)
    /// never share, and can't clobber, each other's slot mapping.
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
            self.compile_queue
                .push((&d.format, next, n + ix, Some(span), Some(n)));
        }
        n + which_next
    }

    pub fn compile_one(format: &Format) -> AResult<Decoder> {
        let module = FormatModule::new();
        let mut compiler = Compiler::new(&module);
        let ctx = RecurseCtx::NonRec;
        compiler.compile_format(format, Rc::new(Next::Empty), ctx, None)
    }

    /// `batch_slot_start`, if `Some`, is the `Program::decoders` slot the *current recursive
    /// batch instantiation's* first member was queued at (see `queue_compile_batch`) - every
    /// recursive call within one instantiation's compilation passes it through unchanged; only
    /// `Format::ItemVar` (which starts a fresh instantiation, queued for later rather than
    /// compiled via direct recursion) doesn't need to thread it further.
    fn compile_format(
        &mut self,
        format: &'a Format,
        next: Rc<Next<'a>>,
        ctx: RecurseCtx<'a>,
        batch_slot_start: Option<usize>,
    ) -> AResult<Decoder> {
        match format {
            Format::ItemVar(level) => {
                let f = self.module.get_format(*level);
                // depends_on_next must be evaluated relative to `level`'s own batch context, not
                // the enclosing format's `ctx` - a RecVar inside `f` resolves against `level`'s
                // batch, which need not be (and often isn't) the batch `ctx` belongs to.
                let next = if f.depends_on_next(self.module, self.module.get_ctx(*level)) {
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
                        None => self.queue_compile(t, f, next.clone(), None, None),
                    };
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                Ok(Decoder::Call(n))
            }
            Format::RecVar(batch_ix) => {
                // Validate `batch_ix` against `ctx` the same way the pre-existing code did
                // (`enter` panics on an out-of-range index or a non-recursive ctx), but resolve
                // the actual `Program::decoders` slot via `batch_slot_start` - direct arithmetic
                // over the *current instantiation's* own slot range, not a lookup keyed only by
                // the abstract (instantiation-independent) module-level FormatId, which is what
                // let two instantiations of the same batch clobber each other's slots.
                let level = ctx.enter(*batch_ix).get_level().unwrap();
                let batch_slot_start = batch_slot_start.unwrap_or_else(|| {
                    panic!(
                        "RecVar({batch_ix}) (resolved to level {level}) used outside any batch instantiation"
                    )
                });
                Ok(Decoder::CallRec(batch_slot_start + batch_ix, *batch_ix))
            }
            Format::FailWith(msg) => Ok(Decoder::FailWith(msg.clone())),
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::Variant(label, f) => {
                let d = self.compile_format(f, next.clone(), ctx, batch_slot_start)?;
                Ok(Decoder::Variant(label.clone(), Box::new(d)))
            }
            Format::Compute(expr) => Ok(Decoder::Compute(expr.clone())),
            Format::Union(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_format(f, next.clone(), ctx, batch_slot_start)?);
                }
                if let Some(tree) = MatchTree::build(self.module, branches, next, ctx) {
                    Ok(Decoder::Branch(tree, ds))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            Format::UnionNondet(branches) => {
                // No MatchTree involved at all - each branch is compiled independently and tried
                // in order at decode time (Decoder::Parallel), backtracking on failure.
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_format(f, next.clone(), ctx, batch_slot_start)?);
                }
                Ok(Decoder::Parallel(ds))
            }
            Format::Tuple(elems) => {
                let mut decs = Vec::with_capacity(elems.len());
                let mut fields = elems.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(fields.as_slice(), ctx, next.clone()));
                    let df = self.compile_format(f, next, ctx, batch_slot_start)?;
                    decs.push(df);
                }
                Ok(Decoder::Tuple(decs))
            }
            Format::Seq(elems) => {
                let mut decs = Vec::with_capacity(elems.len());
                let mut fields = elems.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(fields.as_slice(), ctx, next.clone()));
                    let df = self.compile_format(f, next, ctx, batch_slot_start)?;
                    decs.push(df);
                }
                Ok(Decoder::Seq(decs))
            }
            Format::Repeat(a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_format(
                    a,
                    Rc::new(Next::Repeat(a, ctx, next.clone())),
                    ctx,
                    batch_slot_start,
                )?;
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
                let da = Box::new(self.compile_format(
                    a,
                    Rc::new(Next::Empty),
                    ctx,
                    batch_slot_start,
                )?);
                Ok(Decoder::Maybe(x.clone(), da))
            }
            Format::RepeatCount(count, a) => {
                let n = count.eval_usize();
                // The inner decoder is compiled once and looped `n` times at runtime (the loop
                // counter enforces the exact count, not the continuation), so - like Repeat - it
                // uses the generic self-referential `Next::Repeat` continuation rather than a
                // distinct `Next::RepeatCount(k, ..)` per remaining count (which would otherwise
                // force a separate compiled decoder per iteration for any ItemVar inside `a`
                // whose `depends_on_next` is true).
                let da = self.compile_format(
                    a,
                    Rc::new(Next::Repeat(a, ctx, next.clone())),
                    ctx,
                    batch_slot_start,
                )?;
                Ok(Decoder::RepeatCount(n, Box::new(da)))
            }
            Format::RepeatBetween(min_expr, max_expr, a) => {
                let (min, max) = (min_expr.eval_usize(), max_expr.eval_usize());
                assert!(
                    min <= max,
                    "incoherent RepeatBetween: min {min} > max {max}"
                );
                if min == max {
                    let da = self.compile_format(
                        a,
                        Rc::new(Next::Repeat(a, ctx, next.clone())),
                        ctx,
                        batch_slot_start,
                    )?;
                    return Ok(Decoder::RepeatCount(min, Box::new(da)));
                }
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_format(
                    a,
                    Rc::new(Next::Repeat(a, ctx, next.clone())),
                    ctx,
                    batch_slot_start,
                )?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                if let Some(tree) = MatchTree::build(self.module, &[fa, fb], next, ctx) {
                    Ok(Decoder::RepeatBetween(min, max - min, tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            Format::Peek(a) => {
                let da = Box::new(self.compile_format(
                    a,
                    Rc::new(Next::Empty),
                    ctx,
                    batch_slot_start,
                )?);
                Ok(Decoder::Peek(da))
            }
            Format::PeekNot(a) => {
                let da = Box::new(self.compile_format(
                    a,
                    Rc::new(Next::Empty),
                    ctx,
                    batch_slot_start,
                )?);
                Ok(Decoder::PeekNot(da))
            }
            Format::Slice(count, a) => {
                let n = count.eval_usize();
                let da = Box::new(self.compile_format(
                    a,
                    Rc::new(Next::Empty),
                    ctx,
                    batch_slot_start,
                )?);
                Ok(Decoder::Slice(n, da))
            }
            Format::WithRelativeOffset(base, offset, a) => {
                let abs_offset = base.eval_usize() + offset.eval_usize();
                let da = Box::new(self.compile_format(
                    a,
                    Rc::new(Next::Empty),
                    ctx,
                    batch_slot_start,
                )?);
                Ok(Decoder::WithRelativeOffset(abs_offset, da))
            }
        }
    }
}

impl Expr {
    /// Evaluates `self` and extracts a `usize`, panicking if the result isn't numeric. Since
    /// `Expr` has no `Var`, every `Expr` is compile-time-constant-foldable, so this is a total,
    /// pure function - used wherever `RepeatCount`/`RepeatBetween` need a concrete repeat bound
    /// (both in `MatchTree` construction and at decode time).
    pub(crate) fn eval_usize(&self) -> usize {
        self.eval().get_usize_with_precision().0
    }

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
    /// Tries each branch in order against the same starting position, backtracking on failure;
    /// the first to succeed wins. Mirrors `doodle::Decoder::Parallel` (the compiled form of
    /// `doodle::Format::UnionNondet`) exactly - `ReadCtxt` is `Copy`, so there's no explicit
    /// position save/restore to get right, only the control flow.
    Parallel(Vec<Decoder>),

    While(MatchTree, Box<Decoder>), // Repeat decoder while input matches

    Seq(Vec<Decoder>),
    Tuple(Vec<Decoder>),
    Maybe(Box<Expr>, Box<Decoder>),

    /// Runs the inner decoder exactly `usize` times.
    RepeatCount(usize, Box<Decoder>),
    /// Runs the inner decoder `usize` (min) times unconditionally, then up to `usize` (extra =
    /// max - min) more times, deciding "one more repetition, or stop" via the `MatchTree` at each
    /// of those additional positions (same 2-way decision `While` uses, just capped by `extra`).
    RepeatBetween(usize, usize, MatchTree, Box<Decoder>),
    /// Parses the inner decoder, then discards the position it advanced to, keeping the value.
    Peek(Box<Decoder>),
    /// Fails if the inner decoder successfully parses; otherwise succeeds with unit, at the
    /// original (unadvanced) position.
    PeekNot(Box<Decoder>),
    /// Restricts the inner decoder to exactly `usize` bytes, discarding any leftover; the outer
    /// position always advances by exactly that many bytes, regardless of how many the inner
    /// decoder actually consumed.
    Slice(usize, Box<Decoder>),
    /// Parses the inner decoder at the given absolute buffer offset, without advancing the outer
    /// position at all (the offset is already fully resolved at compile time - both `base` and
    /// `offset` are compile-time-constant since `Expr` has no `Var`).
    WithRelativeOffset(usize, Box<Decoder>),
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
            Decoder::Parallel(branches) => {
                for (index, d) in branches.iter().enumerate() {
                    if let Ok((v, new_input)) = d.parse(program, input) {
                        return Ok((Value::Branch(index, Box::new(v)), new_input));
                    }
                }
                Err(DecodeError::NoValidBranch {
                    offset: input.offset,
                })
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
            Decoder::RepeatCount(n, a) => {
                let mut input = input;
                let mut v = Vec::with_capacity(*n);
                for _ in 0..*n {
                    let (va, next_input) = a.parse(program, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::RepeatBetween(min, extra, tree, a) => {
                let mut input = input;
                let mut v = Vec::with_capacity(*min);
                for _ in 0..*min {
                    let (va, next_input) = a.parse(program, input)?;
                    input = next_input;
                    v.push(va);
                }
                for _ in 0..*extra {
                    match tree.matches(input) {
                        Some(0) => {
                            let (va, next_input) = a.parse(program, input)?;
                            input = next_input;
                            v.push(va);
                        }
                        Some(_) => break,
                        None => {
                            return Err(DecodeError::NoValidBranch {
                                offset: input.offset,
                            });
                        }
                    }
                }
                Ok((Value::Seq(v), input))
            }
            Decoder::Peek(a) => {
                let (v, _discarded) = a.parse(program, input)?;
                Ok((v, input))
            }
            Decoder::PeekNot(a) => match a.parse(program, input) {
                Ok(_) => Err(DecodeError::PeekNotMatched {
                    offset: input.offset,
                }),
                Err(_) => Ok((Value::Tuple(vec![]), input)),
            },
            Decoder::Slice(n, a) => {
                let (slice, rest) = input.split_at(*n).ok_or(DecodeError::SliceOverrun {
                    needed: *n,
                    offset: input.offset,
                })?;
                let (v, _leftover) = a.parse(program, slice)?;
                Ok((v, rest))
            }
            Decoder::WithRelativeOffset(abs_offset, a) => {
                let seek_input = input.seek_to(*abs_offset).ok_or(DecodeError::BadSeek {
                    target: *abs_offset,
                    len: input.input.len(),
                })?;
                let (v, _) = a.parse(program, seek_input)?;
                Ok((v, input))
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
            Format::Variant(
                Label::Borrowed("peanoZ"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("peanoS"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::RecVar(0),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![(Label::Borrowed("test.peano"), peano)]);
        let f = Format::Tuple(vec![frefs[0].call(), Format::EndOfInput]);
        let program = Compiler::compile_program(&module, &f, RecurseCtx::NonRec)?;
        let input = ReadCtxt::new(b"SSSSZ");
        let (value, _) = program.run(input)?;
        eprintln!("{}", to_string_pretty(&value).unwrap());
        Ok(())
    }

    fn seq_of_bytes(value: &Value) -> Vec<u8> {
        match value {
            Value::Seq(vals) => vals
                .iter()
                .map(|v| match v {
                    Value::U8(b) => *b,
                    other => panic!("expected U8, found {other:?}"),
                })
                .collect(),
            other => panic!("expected Seq, found {other:?}"),
        }
    }

    #[test]
    fn repeat_count_decodes_exact_n() -> AResult<()> {
        let f = Format::RepeatCount(
            Box::new(Expr::U8(3)),
            Box::new(Format::Byte(ByteSet::from(b'a'..=b'z'))),
        );
        let program = Compiler::compile_program(&FormatModule::new(), &f, RecurseCtx::NonRec)?;
        let (value, remaining) = program.run(ReadCtxt::new(b"abcdef"))?;
        assert_eq!(seq_of_bytes(&value), vec![b'a', b'b', b'c']);
        assert_eq!(remaining.offset, 3);
        Ok(())
    }

    #[test]
    fn repeat_between_stops_within_bounds() -> AResult<()> {
        // Between 1 and 3 'a's, followed by a mandatory 'b' - exercises the "one more, or stop"
        // MatchTree decision at each of the two optional positions.
        let f = Format::Tuple(vec![
            Format::RepeatBetween(
                Box::new(Expr::U8(1)),
                Box::new(Expr::U8(3)),
                Box::new(Format::Byte(ByteSet::from([b'a']))),
            ),
            Format::Byte(ByteSet::from([b'b'])),
        ]);
        let module = FormatModule::new();
        let program = Compiler::compile_program(&module, &f, RecurseCtx::NonRec)?;

        let (value, remaining) = program.run(ReadCtxt::new(b"ab"))?;
        let Value::Tuple(fields) = &value else {
            panic!("expected Tuple, found {value:?}")
        };
        assert_eq!(seq_of_bytes(&fields[0]), vec![b'a']);
        assert_eq!(remaining.offset, 2);

        let (value, remaining) = program.run(ReadCtxt::new(b"aaab"))?;
        let Value::Tuple(fields) = &value else {
            panic!("expected Tuple, found {value:?}")
        };
        assert_eq!(seq_of_bytes(&fields[0]), vec![b'a', b'a', b'a']);
        assert_eq!(remaining.offset, 4);

        // "aaaab" has 4 'a's, but max is 3 - the 4th 'a' is left for (and fails to match) the
        // mandatory trailing 'b' field.
        assert!(program.run(ReadCtxt::new(b"aaaab")).is_err());
        Ok(())
    }

    #[test]
    fn peek_does_not_consume() -> AResult<()> {
        let f = Format::Tuple(vec![
            Format::Peek(Box::new(Format::Byte(ByteSet::from([b'a'])))),
            Format::Byte(ByteSet::from([b'a'])),
        ]);
        let program = Compiler::compile_program(&FormatModule::new(), &f, RecurseCtx::NonRec)?;
        let (value, remaining) = program.run(ReadCtxt::new(b"a"))?;
        let Value::Tuple(fields) = &value else {
            panic!("expected Tuple, found {value:?}")
        };
        assert!(matches!(fields[0], Value::U8(b'a')));
        assert!(matches!(fields[1], Value::U8(b'a')));
        assert_eq!(remaining.offset, 1);
        Ok(())
    }

    #[test]
    fn peek_not_matched_fails() {
        let f = Format::PeekNot(Box::new(Format::Byte(ByteSet::from([b'a']))));
        let program = Compiler::compile_program(&FormatModule::new(), &f, RecurseCtx::NonRec)
            .expect("compiles");
        assert!(program.run(ReadCtxt::new(b"a")).is_err());
    }

    #[test]
    fn peek_not_unmatched_succeeds_without_consuming() -> AResult<()> {
        let f = Format::Tuple(vec![
            Format::PeekNot(Box::new(Format::Byte(ByteSet::from([b'a'])))),
            Format::Byte(ByteSet::from([b'b'])),
        ]);
        let program = Compiler::compile_program(&FormatModule::new(), &f, RecurseCtx::NonRec)?;
        let (value, remaining) = program.run(ReadCtxt::new(b"b"))?;
        let Value::Tuple(fields) = &value else {
            panic!("expected Tuple, found {value:?}")
        };
        assert!(matches!(&fields[0], Value::Tuple(v) if v.is_empty()));
        assert!(matches!(fields[1], Value::U8(b'b')));
        assert_eq!(remaining.offset, 1);
        Ok(())
    }

    /// Collects the slot numbers of every `Decoder::CallRec` reachable from `d` (without
    /// descending into `Decoder::Call`, which points at an independently-compiled slot rather
    /// than being part of `d`'s own body).
    fn collect_callrec_slots(d: &Decoder, out: &mut Vec<usize>) {
        match d {
            Decoder::CallRec(slot, _) => out.push(*slot),
            Decoder::Call(_)
            | Decoder::FailWith(_)
            | Decoder::EndOfInput
            | Decoder::Byte(_)
            | Decoder::Compute(_) => {}
            Decoder::Variant(_, d)
            | Decoder::Peek(d)
            | Decoder::PeekNot(d)
            | Decoder::Slice(_, d)
            | Decoder::WithRelativeOffset(_, d) => collect_callrec_slots(d, out),
            Decoder::Branch(_, ds)
            | Decoder::Seq(ds)
            | Decoder::Tuple(ds)
            | Decoder::Parallel(ds) => {
                for d in ds {
                    collect_callrec_slots(d, out);
                }
            }
            Decoder::While(_, d)
            | Decoder::Maybe(_, d)
            | Decoder::RepeatCount(_, d)
            | Decoder::RepeatBetween(_, _, _, d) => collect_callrec_slots(d, out),
        }
    }

    #[test]
    fn recvar_resolves_correctly_across_two_batch_instantiations() -> AResult<()> {
        // A self-recursive batch whose own body is directly byte-disjoint ('Z' vs 'S') is
        // decidable independent of `next`, so `depends_on_next` (via `union_depends_on_next`'s
        // `MatchTree::build(..., Next::Empty, ...)` fallback) would be `false` - `decoder_map`
        // would then key every `ItemVar` reference on the same `Next::Empty` regardless of call
        // site, so the batch would only ever be compiled once, and this test would exercise
        // nothing. Wrapping the recursive step in an unconditional `Maybe` forces
        // `depends_on_next` to be (hardcoded) `true`, so two call sites with different trailing
        // fields genuinely key `decoder_map` differently, forcing two separate
        // `queue_compile_batch` instantiations of the same batch.
        let peano = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Z"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::Maybe(Box::new(Expr::Bool(true)), Box::new(Format::RecVar(0))),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![(Label::Borrowed("test.peano"), peano)]);
        let peano_ref = frefs[0];

        // Two references to the same batch, each followed by a *different* trailing field.
        let f = Format::Tuple(vec![
            peano_ref.call(),
            Format::Byte(ByteSet::from([b'#'])),
            peano_ref.call(),
            Format::EndOfInput,
        ]);
        let program = Compiler::compile_program(&module, &f, RecurseCtx::NonRec)?;

        // Decoding still succeeds either way (see note below on why this alone doesn't prove
        // anything for this particular grammar), so the real assertion inspects the compiled
        // `Program`'s decoder graph directly: `Decoder::Tuple` at slot 0 must hold
        // `Decoder::Call(1)` then `Decoder::Call(2)` for the two `ItemVar` references (two
        // distinct slots proves two separate `queue_compile_batch` instantiations really did
        // happen, i.e. this test isn't vacuous), and each instantiation's own compiled body
        // (slots 1 and 2 respectively) must contain a `Decoder::CallRec` pointing back to
        // *itself* (batch_ix 0 of a 1-member batch always resolves to its own instantiation's
        // start) - not to the other instantiation's slot, which is exactly what `level_slot`
        // conflating two instantiations used to risk.
        //
        // (An end-to-end decode-behavior differential was tried first and rejected: for this
        // grammar shape, both instantiations compile to structurally identical decoders - 'Z'
        // vs 'S' disambiguation never actually depends on what `next` is - so even a genuinely
        // wrong slot number is silently harmless at runtime here. Direct structural inspection
        // is what actually pins the fix down.)
        let Decoder::Tuple(top_fields) = &program.decoders[0].0 else {
            panic!("expected top-level Tuple decoder");
        };
        let (n1, n2) = match (&top_fields[0], &top_fields[2]) {
            (Decoder::Call(n1), Decoder::Call(n2)) => (*n1, *n2),
            other => panic!("expected two Decoder::Call, found {other:?}"),
        };
        assert_ne!(
            n1, n2,
            "the two ItemVar references should have compiled to two separate instantiations"
        );

        let mut slots1 = Vec::new();
        collect_callrec_slots(&program.decoders[n1].0, &mut slots1);
        assert_eq!(
            slots1,
            vec![n1],
            "instantiation at slot {n1} should CallRec itself"
        );

        let mut slots2 = Vec::new();
        collect_callrec_slots(&program.decoders[n2].0, &mut slots2);
        assert_eq!(
            slots2,
            vec![n2],
            "instantiation at slot {n2} should CallRec itself"
        );

        Ok(())
    }

    #[test]
    fn union_nondet_tries_branches_in_order() -> AResult<()> {
        // Overlapping first bytes ('a' could start either branch) - not something a MatchTree
        // could safely disambiguate via lookahead alone without risking wrongly rejecting the
        // second branch, which is exactly why this needs backtracking rather than Format::Union.
        let f = Format::UnionNondet(vec![
            Format::Tuple(vec![
                Format::Byte(ByteSet::from([b'a'])),
                Format::Byte(ByteSet::from([b'b'])),
            ]),
            Format::Tuple(vec![
                Format::Byte(ByteSet::from([b'a'])),
                Format::Byte(ByteSet::from([b'c'])),
            ]),
        ]);
        let program = Compiler::compile_program(&FormatModule::new(), &f, RecurseCtx::NonRec)?;

        let (value, remaining) = program.run(ReadCtxt::new(b"ab"))?;
        assert!(matches!(value, Value::Branch(0, _)));
        assert_eq!(remaining.offset, 2);

        let (value, remaining) = program.run(ReadCtxt::new(b"ac"))?;
        assert!(matches!(value, Value::Branch(1, _)));
        assert_eq!(remaining.offset, 2);

        assert!(program.run(ReadCtxt::new(b"ad")).is_err());
        Ok(())
    }

    #[test]
    fn union_nondet_is_the_escape_valve_matchtree_cannot_provide() -> AResult<()> {
        // The exact scenario matchtree::tests::build_union_disambiguating_through_recursion
        // proved is *not* decidable via any fixed-depth MatchTree (both branches share an
        // unboundedly-long recursive peano prefix, only diverging on the byte immediately after
        // it) - the whole point of this step. Format::Union would fail to compile at all here
        // ("cannot build match tree"); Format::UnionNondet must still decode it correctly.
        let peano = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Z"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::RecVar(0),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![(Label::Borrowed("test.peano"), peano)]);
        let peano_ref = frefs[0];
        let f = Format::UnionNondet(vec![
            Format::Tuple(vec![peano_ref.call(), Format::Byte(ByteSet::from([b'A']))]),
            Format::Tuple(vec![peano_ref.call(), Format::Byte(ByteSet::from([b'B']))]),
        ]);
        let program = Compiler::compile_program(&module, &f, RecurseCtx::NonRec)?;

        let (value, remaining) = program.run(ReadCtxt::new(b"SSSZA"))?;
        assert!(matches!(value, Value::Branch(0, _)));
        assert_eq!(remaining.offset, 5);

        let (value, remaining) = program.run(ReadCtxt::new(b"SSSZB"))?;
        assert!(matches!(value, Value::Branch(1, _)));
        assert_eq!(remaining.offset, 5);

        Ok(())
    }

    #[test]
    fn slice_restricts_and_skips_leftover() -> AResult<()> {
        let f = Format::Tuple(vec![
            Format::Slice(
                Box::new(Expr::U8(3)),
                Box::new(Format::Byte(ByteSet::from([b'a']))),
            ),
            Format::Byte(ByteSet::from([b'X'])),
        ]);
        let program = Compiler::compile_program(&FormatModule::new(), &f, RecurseCtx::NonRec)?;
        // The slice is 3 bytes ("a??"), but the inner format only consumes the first ('a') -
        // the other two are leftover and skipped, so the outer stream picks up at offset 3 with
        // the mandatory 'X' field, not offset 1.
        let (value, remaining) = program.run(ReadCtxt::new(b"a??X"))?;
        let Value::Tuple(fields) = &value else {
            panic!("expected Tuple, found {value:?}")
        };
        assert!(matches!(fields[0], Value::U8(b'a')));
        assert!(matches!(fields[1], Value::U8(b'X')));
        assert_eq!(remaining.offset, 4);

        // Too few bytes left for the 3-byte slice.
        assert!(program.run(ReadCtxt::new(b"a?")).is_err());
        Ok(())
    }

    #[test]
    fn with_relative_offset_self_referential_recvar() -> AResult<()> {
        // A batch where one member's body jumps (via a compile-time-constant absolute offset,
        // since Expr has no Var - real OpenType-style data-dependent offsets aren't expressible
        // yet) to a fixed position holding a *different*, self-recursive batch member, reached
        // through RecVar (not ItemVar, per the established lesson: ItemVar resets its own ctx
        // via module.get_ctx regardless of what's ambient, which would mask a ctx bug here).
        // Proves the offset jump doesn't disturb ctx resolution (RecVar(1) still finds the right
        // sibling and its own further self-recursion still works) or the outer stream position
        // (unaffected by the jump entirely).
        let wrapper = Format::Tuple(vec![
            Format::Byte(ByteSet::from([b'#'])),
            Format::WithRelativeOffset(
                Box::new(Expr::U8(0)),
                Box::new(Expr::U8(3)),
                Box::new(Format::RecVar(1)),
            ),
        ]);
        let peano = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Z"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::RecVar(1),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![
            (Label::Borrowed("wrapper"), wrapper),
            (Label::Borrowed("peano"), peano),
        ]);
        let program = Compiler::compile_program(&module, &frefs[0].call(), RecurseCtx::NonRec)?;

        // byte 0 = '#' (the marker); bytes 1,2 = padding, never read directly; bytes 3.. = "SSZ"
        // (peano's own encoding) at the jump target.
        let (_value, remaining) = program.run(ReadCtxt::new(b"#--SSZ"))?;
        // The outer stream only ever consumed the marker byte - the offset jump doesn't move it,
        // regardless of how many bytes the jump target itself consumed.
        assert_eq!(remaining.offset, 1);
        Ok(())
    }
}
