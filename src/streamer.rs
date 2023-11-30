use crate::byte_set::ByteSet;
use crate::decoder::{
    self, make_huffman_codes, value_to_vec_usize, Decoder, Scope, ScopeBinding, ScopeEntry,
    ScopeLookup, Value,
};
use crate::error::{ParseError, ParseResult};
use crate::read::ReadCtxt;
use crate::{DynFormat, Expr, Format, FormatModule, Label, MatchTree, Next, Pattern};
use std::collections::HashMap;
use std::rc::Rc;

struct StreamCtxt<'a> {
    parent: Option<&'a StreamCtxt<'a>>,
    frames: Vec<StreamFrame<'a>>,
    values: Vec<Value>,
}

enum StreamFrame<'a> {
    Call(CallScope),
    Let(LetScope<'a>),
    Record(RecordScope),
    Pattern(RecordScope),
    Decoder(DecoderScope<'a>),
}

struct CallScope {
    args: Vec<(Label, Value)>,
}

struct LetScope<'a> {
    name: &'a str,
    value: Value,
}

struct RecordScope {
    record: Vec<(Label, Value)>,
}

struct DecoderScope<'a> {
    name: &'a str,
    decoder: Decoder,
}

impl CallScope {
    fn new(args: Vec<(Label, Value)>) -> CallScope {
        CallScope { args }
    }
}

impl<'a> LetScope<'a> {
    fn new(name: &'a str, value: Value) -> LetScope<'a> {
        LetScope { name, value }
    }
}

impl RecordScope {
    fn new(num_fields: usize) -> RecordScope {
        let record = Vec::with_capacity(num_fields);
        RecordScope { record }
    }
}

impl ScopeBinding for RecordScope {
    fn push(&mut self, name: Label, v: Value) {
        self.record.push((name, v));
    }
}

impl<'a> DecoderScope<'a> {
    fn new(name: &'a str, decoder: Decoder) -> DecoderScope<'a> {
        DecoderScope { name, decoder }
    }
}

impl<'a> ScopeLookup for StreamCtxt<'a> {
    fn get_value_by_name(&self, name: &str) -> &Value {
        for frame in self.frames.iter().rev() {
            match frame {
                StreamFrame::Call(call_scope) => {
                    for (n, v) in call_scope.args.iter().rev() {
                        if n == name {
                            return v;
                        }
                    }
                }
                StreamFrame::Let(let_scope) => {
                    if let_scope.name == name {
                        return &let_scope.value;
                    }
                }
                StreamFrame::Record(record_scope) => {
                    for (n, v) in record_scope.record.iter().rev() {
                        if n == name {
                            return v;
                        }
                    }
                }
                StreamFrame::Pattern(pattern_scope) => {
                    for (n, v) in pattern_scope.record.iter().rev() {
                        if n == name {
                            return v;
                        }
                    }
                }
                StreamFrame::Decoder(_decoder_scope) => {}
            }
        }
        match self.parent {
            None => panic!("could not get value: {name}"),
            Some(parent) => parent.get_value_by_name(name),
        }
    }

    fn get_decoder_by_name(&self, name: &str) -> &Decoder {
        for frame in self.frames.iter().rev() {
            match frame {
                StreamFrame::Call(_call_scope) => {}
                StreamFrame::Let(_let_scope) => {}
                StreamFrame::Record(_record_scope) => {}
                StreamFrame::Pattern(_pattern_scope) => {}
                StreamFrame::Decoder(decoder_scope) => {
                    if decoder_scope.name == name {
                        return &decoder_scope.decoder;
                    }
                }
            }
        }
        match self.parent {
            None => panic!("could not get decoder: {name}"),
            Some(parent) => parent.get_decoder_by_name(name),
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry)>) {
        for frame in self.frames.iter().rev() {
            match frame {
                StreamFrame::Call(call_scope) => {
                    for (name, value) in call_scope.args.iter().rev() {
                        bindings.push((name.clone(), ScopeEntry::Value(value.clone())));
                    }
                }
                StreamFrame::Let(let_scope) => {
                    bindings.push((
                        let_scope.name.to_string().into(),
                        ScopeEntry::Value(let_scope.value.clone()),
                    ));
                }
                StreamFrame::Record(record_scope) => {
                    for (name, value) in record_scope.record.iter().rev() {
                        bindings.push((name.clone(), ScopeEntry::Value(value.clone())));
                    }
                }
                StreamFrame::Pattern(pattern_scope) => {
                    for (name, value) in pattern_scope.record.iter().rev() {
                        bindings.push((name.clone(), ScopeEntry::Value(value.clone())));
                    }
                }
                StreamFrame::Decoder(_decoder_scope) => {} // FIXME
            }
        }
        match self.parent {
            None => {}
            Some(parent) => parent.get_bindings(bindings),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Block {
    Ops(Box<[Op]>),
}

#[derive(Clone, Debug)]
pub enum Op {
    Value(Block),
    Call(usize, Vec<(Label, Expr)>),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Variant(Label),
    Branch(usize),
    Parallel(Vec<Block>),
    MatchTree(MatchTree, Vec<usize>),
    Match(Expr, Vec<(Pattern, usize)>),
    PushTuple(usize),
    TupleField,
    PushRecord(usize),
    RecordField(Label),
    PopRecord,
    PushSeq,
    SeqItem,
    PushPattern,
    PopPattern,
    While(MatchTree, Block),
    Until(MatchTree, Block),
    RepeatCount(Expr, Block),
    RepeatUntilLast(Expr, Block),
    RepeatUntilSeq(Expr, Block),
    Map(Expr),
    Compute(Expr),
    PushLet(Label, Expr),
    PopLet,
    PushDynamic(Label, DynFormat),
    PopDynamic,
    ApplyDynamic(Label),
    PushInput,
    PushInputSlice(Expr),
    PushInputOffset(Expr),
    PopInput,
    Bits(Block),
    Negated(Block),
}

pub struct Program {
    blocks: Vec<Block>,
}

impl Program {
    fn new() -> Self {
        let blocks = Vec::new();
        Program { blocks }
    }

    pub fn run<'input>(&self, input: ReadCtxt<'input>) -> ParseResult<(Value, ReadCtxt<'input>)> {
        self.blocks[0].eval_clean(self, None, input)
    }
}

pub struct Compiler<'a> {
    module: &'a FormatModule,
    program: Program,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<(&'a Format, Rc<Next<'a>>, usize)>,
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

    pub fn compile(module: &FormatModule, format: &Format) -> Result<Program, String> {
        let mut compiler = Compiler::new(module);
        compiler.queue_compile(format, Rc::new(Next::Empty));
        while let Some((f, next, n)) = compiler.compile_queue.pop() {
            let b = Block::compile_next(&mut compiler, f, next)?;
            compiler.program.blocks[n] = b;
        }
        Ok(compiler.program)
    }

    fn add_block(&mut self, b: Block) -> usize {
        let n = self.program.blocks.len();
        self.program.blocks.push(b);
        n
    }

    fn queue_compile(&mut self, f: &'a Format, next: Rc<Next<'a>>) -> usize {
        let n = self.program.blocks.len();
        self.program.blocks.push(Block::Ops(Box::new([])));
        self.compile_queue.push((f, next, n));
        n
    }
}

impl Block {
    pub fn compile_one(format: &Format) -> Result<Program, String> {
        let module = FormatModule::new();
        Compiler::compile(&module, format)
    }

    pub fn compile<'a>(compiler: &mut Compiler<'a>, format: &'a Format) -> Result<Block, String> {
        Block::compile_next(compiler, format, Rc::new(Next::Empty))
    }

    fn compile_next<'a>(
        compiler: &mut Compiler<'a>,
        format: &'a Format,
        next: Rc<Next<'a>>,
    ) -> Result<Block, String> {
        match format {
            Format::ItemVar(level, arg_exprs) => {
                let f = compiler.module.get_format(*level);
                let next = if f.depends_on_next(compiler.module) {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = compiler.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let n = compiler.queue_compile(f, next.clone());
                    compiler.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                let arg_names = compiler.module.get_args(*level);
                let mut args = Vec::new();
                for ((name, _type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    args.push((name.clone(), expr.clone()));
                }
                Ok(Block::op(Op::Call(n, args)))
            }
            Format::Fail => Ok(Block::op(Op::Fail)),
            Format::EndOfInput => Ok(Block::op(Op::EndOfInput)),
            Format::Align(n) => Ok(Block::op(Op::Align(*n))),
            Format::Byte(bs) => Ok(Block::op(Op::Byte(*bs))),
            Format::Variant(label, f) => {
                let s = Block::compile_next(compiler, f, next.clone())?;
                Ok(Block::ops(Box::new([
                    Op::Value(s),
                    Op::Variant(label.clone()),
                ])))
            }
            Format::Union(branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut bs = Vec::with_capacity(branches.len());
                for (index, f) in branches.iter().enumerate() {
                    let s = Block::compile_next(compiler, f, next.clone())?;
                    let b =
                        compiler.add_block(Block::ops(Box::new([Op::Value(s), Op::Branch(index)])));
                    bs.push(b);
                    fs.push(f.clone());
                }
                if let Some(tree) = MatchTree::build(compiler.module, &fs, next) {
                    Ok(Block::op(Op::MatchTree(tree, bs)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::UnionVariant(branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut bs = Vec::with_capacity(branches.len());
                for (index, (label, f)) in branches.iter().enumerate() {
                    let s = Block::compile_next(compiler, f, next.clone())?;
                    let b = compiler.add_block(Block::ops(Box::new([
                        Op::Value(s),
                        Op::Variant(label.clone()),
                        Op::Branch(index),
                    ])));
                    bs.push(b);
                    fs.push(f.clone());
                }
                if let Some(tree) = MatchTree::build(compiler.module, &fs, next) {
                    Ok(Block::op(Op::MatchTree(tree, bs)))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::UnionNondet(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for (label, f) in branches {
                    let s = Block::compile_next(compiler, f, next.clone())?;
                    ds.push(Block::ops(Box::new([
                        Op::Value(s),
                        Op::Variant(label.clone()),
                    ])));
                }
                Ok(Block::op(Op::Parallel(ds)))
            }
            Format::Tuple(fields) => {
                let mut ops = Vec::with_capacity(fields.len() * 2 + 1);
                ops.push(Op::PushTuple(fields.len()));
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Tuple(fields.as_slice(), next.clone()));
                    let s = Block::compile_next(compiler, f, next)?;
                    ops.push(Op::Value(s));
                    ops.push(Op::TupleField);
                }
                Ok(Block::ops(ops.into_boxed_slice()))
            }
            Format::Record(fields) => {
                let mut ops = Vec::with_capacity(fields.len() * 2 + 2);
                ops.push(Op::PushRecord(fields.len()));
                let mut fields = fields.iter();
                while let Some((name, f)) = fields.next() {
                    let next = Rc::new(Next::Record(fields.as_slice(), next.clone()));
                    let s = Block::compile_next(compiler, f, next)?;
                    ops.push(Op::Value(s));
                    ops.push(Op::RecordField(name.clone()));
                }
                ops.push(Op::PopRecord);
                Ok(Block::ops(ops.into_boxed_slice()))
            }
            Format::Repeat(a) => {
                if a.is_nullable(compiler.module) {
                    return Err(format!("cannot repeat nullable format: {a:?}"));
                }
                let s = Block::compile_next(compiler, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let b = Block::ops(Box::new([Op::Value(s), Op::SeqItem]));
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                if let Some(tree) = MatchTree::build(compiler.module, &[fa, fb], next) {
                    Ok(Block::ops(Box::new([Op::PushSeq, Op::While(tree, b)])))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::Repeat1(a) => {
                if a.is_nullable(compiler.module) {
                    return Err(format!("cannot repeat nullable format: {a:?}"));
                }
                let s = Block::compile_next(compiler, a, Rc::new(Next::Repeat(a, next.clone())))?;
                let b = Block::ops(Box::new([Op::Value(s), Op::SeqItem]));
                let astar = Format::Repeat(a.clone());
                let fa = Format::EMPTY;
                let fb = Format::Tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build(compiler.module, &[fa, fb], next) {
                    Ok(Block::ops(Box::new([Op::PushSeq, Op::Until(tree, b)])))
                } else {
                    Err(format!("cannot build match tree for {:?}", format))
                }
            }
            Format::RepeatCount(expr, a) => {
                // FIXME probably not right
                let s = Block::compile_next(compiler, a, next)?;
                Ok(Block::op(Op::RepeatCount(expr.clone(), s)))
            }
            Format::RepeatUntilLast(expr, a) => {
                // FIXME probably not right
                let s = Block::compile_next(compiler, a, next)?;
                Ok(Block::op(Op::RepeatUntilLast(expr.clone(), s)))
            }
            Format::RepeatUntilSeq(expr, a) => {
                // FIXME probably not right
                let s = Block::compile_next(compiler, a, next)?;
                Ok(Block::op(Op::RepeatUntilSeq(expr.clone(), s)))
            }
            Format::Peek(a) => {
                let s = Block::compile_next(compiler, a, Rc::new(Next::Empty))?;
                Ok(Block::ops(Box::new([
                    Op::PushInput,
                    Op::Value(s),
                    Op::PopInput,
                ])))
            }
            Format::PeekNot(a) => {
                const MAX_LOOKAHEAD: usize = 1024;
                match a.match_bounds(compiler.module).max {
                    None => return Err("PeekNot cannot require unbounded lookahead".to_string()),
                    Some(n) if n > MAX_LOOKAHEAD => {
                        return Err(format!(
                            "PeekNot cannot require > {MAX_LOOKAHEAD} bytes lookahead"
                        ))
                    }
                    _ => {}
                }
                let s = Block::compile_next(compiler, a, Rc::new(Next::Empty))?;
                Ok(Block::ops(Box::new([
                    Op::PushInput,
                    Op::Negated(s),
                    Op::PopInput,
                ])))
            }
            Format::Slice(expr, a) => {
                let s = Block::compile_next(compiler, a, Rc::new(Next::Empty))?;
                Ok(Block::ops(Box::new([
                    Op::PushInputSlice(expr.clone()),
                    Op::Value(s),
                    Op::PopInput,
                ])))
            }
            Format::Bits(a) => {
                let s = Block::compile_next(compiler, a, Rc::new(Next::Empty))?;
                Ok(Block::op(Op::Bits(s)))
            }
            Format::WithRelativeOffset(expr, a) => {
                let s = Block::compile_next(compiler, a, Rc::new(Next::Empty))?;
                Ok(Block::ops(Box::new([
                    Op::PushInputOffset(expr.clone()),
                    Op::Value(s),
                    Op::PopInput,
                ])))
            }
            Format::Map(a, expr) => {
                let s = Block::compile_next(compiler, a, next.clone())?;
                Ok(Block::ops(Box::new([Op::Value(s), Op::Map(expr.clone())])))
            }
            Format::Compute(expr) => Ok(Block::op(Op::Compute(expr.clone()))),
            Format::Let(name, expr, a) => {
                let s = Block::compile_next(compiler, a, next.clone())?;
                Ok(Block::ops(Box::new([
                    Op::PushLet(name.clone(), expr.clone()),
                    Op::Value(s),
                    Op::PopLet,
                ])))
            }
            Format::Match(head, branches) => {
                let branches = branches
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, f))| {
                        let s = Block::compile_next(compiler, f, next.clone())?;
                        let b = compiler.add_block(Block::ops(Box::new([
                            Op::Value(s),
                            Op::Branch(index),
                            Op::PopPattern,
                        ])));
                        Ok((pattern.clone(), b))
                    })
                    .collect::<Result<_, String>>()?;
                Ok(Block::ops(Box::new([Op::Match(head.clone(), branches)])))
            }
            Format::MatchVariant(head, branches) => {
                let branches = branches
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, label, f))| {
                        let s = Block::compile_next(compiler, f, next.clone())?;
                        let b = compiler.add_block(Block::ops(Box::new([
                            Op::Value(s),
                            Op::Variant(label.clone()),
                            Op::Branch(index),
                            Op::PopPattern,
                        ])));
                        Ok((pattern.clone(), b))
                    })
                    .collect::<Result<_, String>>()?;
                Ok(Block::ops(Box::new([Op::Match(head.clone(), branches)])))
            }
            Format::Dynamic(name, dynformat, a) => {
                let s = Block::compile_next(compiler, a, next.clone())?;
                Ok(Block::ops(Box::new([
                    Op::PushDynamic(name.clone(), dynformat.clone()),
                    Op::Value(s),
                    Op::PopDynamic,
                ])))
            }
            Format::Apply(name) => Ok(Block::op(Op::ApplyDynamic(name.clone()))),
        }
    }

    fn op(op: Op) -> Block {
        Block::Ops(Box::new([op]))
    }

    fn ops(ops: Box<[Op]>) -> Block {
        Block::Ops(ops)
    }

    fn eval_clean<'input>(
        &self,
        program: &Program,
        parent_scope: Option<&StreamCtxt<'_>>,
        input: ReadCtxt<'input>,
    ) -> ParseResult<(Value, ReadCtxt<'input>)> {
        let mut stack = StreamCtxt {
            parent: parent_scope,
            frames: Vec::new(),
            values: Vec::new(),
        };
        let mut input_stack = Vec::new();
        self.eval(program, &mut stack, &mut input_stack, input)
    }

    fn eval<'a, 'input>(
        &'a self,
        program: &'a Program,
        ctxt: &mut StreamCtxt<'a>,
        input_stack: &mut Vec<ReadCtxt<'input>>,
        mut input: ReadCtxt<'input>,
    ) -> ParseResult<(Value, ReadCtxt<'input>)> {
        let mut block = self;
        'blockloop: loop {
            match block {
                Block::Ops(ops) => {
                    match 'oploop: loop {
                        for op in ops.iter() {
                            match op {
                                Op::Value(s) => match s.eval(program, ctxt, input_stack, input) {
                                    Ok((v, new_input)) => {
                                        ctxt.values.push(v);
                                        input = new_input;
                                    }
                                    Err(err) => {
                                        break 'oploop Err(err);
                                    }
                                },
                                Op::Call(n, es) => {
                                    let scope = &Scope::Other(ctxt);
                                    let mut args = Vec::with_capacity(es.len());
                                    for (name, e) in es {
                                        let v = e.eval_value(scope);
                                        args.push((name.clone(), v));
                                    }
                                    let call_scope = CallScope::new(args);
                                    let mut new_ctxt = StreamCtxt {
                                        parent: None,
                                        frames: vec![StreamFrame::Call(call_scope)],
                                        values: Vec::new(),
                                    };
                                    match program.blocks[*n].eval(
                                        program,
                                        &mut new_ctxt,
                                        input_stack,
                                        input,
                                    ) {
                                        Ok((v, new_input)) => {
                                            ctxt.values.push(v);
                                            input = new_input;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::Fail => {
                                    let scope = &Scope::Other(ctxt);
                                    break 'oploop Err(ParseError::fail(scope, input));
                                }
                                Op::EndOfInput => match input.read_byte() {
                                    None => ctxt.values.push(Value::UNIT),
                                    Some((b, _)) => {
                                        break 'oploop Err(ParseError::trailing(b, input.offset))
                                    }
                                },
                                Op::Align(n) => {
                                    let skip = (n - (input.offset % n)) % n;
                                    match input
                                        .split_at(skip)
                                        .ok_or(ParseError::overrun(skip, input.offset))
                                    {
                                        Ok((_, new_input)) => {
                                            ctxt.values.push(Value::UNIT);
                                            input = new_input;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::Byte(bs) => {
                                    match input
                                        .read_byte()
                                        .ok_or(ParseError::overbyte(input.offset))
                                    {
                                        Ok((b, new_input)) => {
                                            if bs.contains(b) {
                                                let v = Value::U8(b);
                                                ctxt.values.push(v);
                                                input = new_input;
                                            } else {
                                                break 'oploop Err(ParseError::unexpected(
                                                    b,
                                                    *bs,
                                                    input.offset,
                                                ));
                                            }
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::Variant(label) => {
                                    let v = ctxt.values.pop().unwrap();
                                    let v = Value::Variant(label.clone(), Box::new(v));
                                    ctxt.values.push(v);
                                }
                                Op::Branch(index) => {
                                    let v = ctxt.values.pop().unwrap();
                                    let v = Value::Branch(*index, Box::new(v));
                                    ctxt.values.push(v);
                                }
                                Op::Parallel(branches) => {
                                    match (|| {
                                        for (index, d) in branches.iter().enumerate() {
                                            let res = d.eval_clean(program, Some(ctxt), input);
                                            if let Ok((v, input)) = res {
                                                return Ok((
                                                    Value::Branch(index, Box::new(v)),
                                                    input,
                                                ));
                                            }
                                        }
                                        Err(ParseError::fail(&Scope::Other(ctxt), input))
                                    })() {
                                        Ok((v, new_input)) => {
                                            input = new_input;
                                            ctxt.values.push(v);
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::MatchTree(tree, bs) => {
                                    match tree.matches(input).ok_or(ParseError::NoValidBranch {
                                        offset: input.offset,
                                    }) {
                                        Ok(index) => {
                                            let b = bs[index];
                                            block = &program.blocks[b];
                                            continue 'blockloop;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::Match(head, branches) => {
                                    let scope = &Scope::Other(ctxt);
                                    let head = head.eval(scope);
                                    let mut matched = false;
                                    for (pattern, b) in branches.iter() {
                                        let mut pattern_scope = RecordScope::new(0);
                                        if head
                                            .coerce_mapped_value()
                                            .matches_inner(&mut pattern_scope, pattern)
                                        {
                                            ctxt.frames.push(StreamFrame::Pattern(pattern_scope));
                                            match program.blocks[*b].eval(
                                                program,
                                                ctxt,
                                                input_stack,
                                                input,
                                            ) {
                                                Ok((v, new_input)) => {
                                                    input = new_input;
                                                    ctxt.values.push(v);
                                                    matched = true;
                                                    break;
                                                }
                                                Err(err) => break 'oploop Err(err),
                                            }
                                        }
                                    }
                                    if !matched {
                                        panic!("non-exhaustive patterns");
                                    }
                                }
                                Op::PushTuple(num_fields) => {
                                    let v = Value::Tuple(Vec::with_capacity(*num_fields));
                                    ctxt.values.push(v);
                                }
                                Op::TupleField => {
                                    let v = ctxt.values.pop().unwrap();
                                    if let Value::Tuple(ref mut vs) =
                                        ctxt.values.last_mut().unwrap()
                                    {
                                        vs.push(v);
                                    } else {
                                        panic!("expected tuple value");
                                    }
                                }
                                Op::PushRecord(num_fields) => {
                                    let record_scope = RecordScope::new(*num_fields);
                                    ctxt.frames.push(StreamFrame::Record(record_scope));
                                }
                                Op::RecordField(name) => {
                                    let v = ctxt.values.pop().unwrap();
                                    if let StreamFrame::Record(ref mut record_scope) =
                                        ctxt.frames.last_mut().unwrap()
                                    {
                                        record_scope.record.push((name.clone(), v));
                                    } else {
                                        panic!("expected record stack frame");
                                    }
                                }
                                Op::PopRecord => {
                                    if let StreamFrame::Record(record_scope) =
                                        ctxt.frames.pop().unwrap()
                                    {
                                        let v = Value::Record(record_scope.record);
                                        ctxt.values.push(v);
                                    } else {
                                        panic!("expected record stack frame");
                                    }
                                }
                                Op::PushSeq => {
                                    let v = Value::Seq(Vec::new());
                                    ctxt.values.push(v);
                                }
                                Op::SeqItem => {
                                    let v = ctxt.values.pop().unwrap();
                                    if let Value::Seq(ref mut vs) = ctxt.values.last_mut().unwrap()
                                    {
                                        vs.push(v);
                                    } else {
                                        panic!("expected seq value");
                                    }
                                }
                                Op::PushPattern => {
                                    let pattern_scope = RecordScope::new(0);
                                    ctxt.frames.push(StreamFrame::Pattern(pattern_scope));
                                }
                                Op::PopPattern => {
                                    if let StreamFrame::Pattern(_pattern_scope) =
                                        ctxt.frames.pop().unwrap()
                                    {
                                    } else {
                                        panic!("expected pattern stack frame");
                                    }
                                }
                                Op::While(tree, a) => {
                                    match (|| {
                                        while tree.matches(input).ok_or(
                                            ParseError::NoValidBranch {
                                                offset: input.offset,
                                            },
                                        )? == 0
                                        {
                                            let (va, next_input) =
                                                a.eval(program, ctxt, input_stack, input)?;
                                            ctxt.values.push(va);
                                            input = next_input;
                                        }
                                        Ok(input)
                                    })() {
                                        Ok(new_input) => {
                                            input = new_input;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::Until(tree, a) => {
                                    match (|| {
                                        loop {
                                            let (va, next_input) =
                                                a.eval(program, ctxt, input_stack, input)?;
                                            ctxt.values.push(va);
                                            input = next_input;
                                            if tree.matches(input).ok_or(
                                                ParseError::NoValidBranch {
                                                    offset: input.offset,
                                                },
                                            )? == 0
                                            {
                                                break;
                                            }
                                        }
                                        Ok(input)
                                    })() {
                                        Ok(new_input) => {
                                            input = new_input;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::RepeatCount(expr, s) => {
                                    let scope = &Scope::Other(ctxt);
                                    let count = expr.eval_value(scope).unwrap_usize();
                                    let mut vs = Vec::with_capacity(count);
                                    for _ in 0..count {
                                        match s.eval(program, ctxt, input_stack, input) {
                                            Ok((va, new_input)) => {
                                                vs.push(va);
                                                input = new_input;
                                            }
                                            Err(err) => break 'oploop Err(err),
                                        }
                                    }
                                    ctxt.values.push(Value::Seq(vs));
                                }
                                Op::RepeatUntilLast(expr, s) => {
                                    let mut vs = Vec::new();
                                    loop {
                                        match s.eval(program, ctxt, input_stack, input) {
                                            Ok((va, new_input)) => {
                                                let scope = &Scope::Other(ctxt);
                                                let done =
                                                    expr.eval_lambda(scope, &va).unwrap_bool();
                                                vs.push(va);
                                                input = new_input;
                                                if done {
                                                    break;
                                                }
                                            }
                                            Err(err) => break 'oploop Err(err),
                                        }
                                    }
                                    ctxt.values.push(Value::Seq(vs));
                                }
                                Op::RepeatUntilSeq(expr, s) => {
                                    let mut vs = Vec::new();
                                    loop {
                                        match s.eval(program, ctxt, input_stack, input) {
                                            Ok((va, new_input)) => {
                                                vs.push(va);
                                                input = new_input;
                                                let v = Value::Seq(vs);
                                                let scope = &Scope::Other(ctxt);
                                                let done =
                                                    expr.eval_lambda(scope, &v).unwrap_bool();
                                                vs = match v {
                                                    Value::Seq(vs) => vs,
                                                    _ => unreachable!(),
                                                };
                                                if done {
                                                    break;
                                                }
                                            }
                                            Err(err) => break 'oploop Err(err),
                                        }
                                    }
                                    ctxt.values.push(Value::Seq(vs));
                                }
                                Op::Map(expr) => {
                                    let old_v = ctxt.values.pop().unwrap();
                                    let scope = &Scope::Other(ctxt);
                                    let new_v = expr.eval_lambda(scope, &old_v);
                                    let v = Value::Mapped(Box::new(old_v), Box::new(new_v));
                                    ctxt.values.push(v);
                                }
                                Op::Compute(expr) => {
                                    let scope = &Scope::Other(ctxt);
                                    let v = expr.eval_value(scope);
                                    ctxt.values.push(v);
                                }
                                Op::PushLet(name, expr) => {
                                    let scope = &Scope::Other(ctxt);
                                    let v = expr.eval_value(scope);
                                    let let_scope = LetScope::new(name, v);
                                    ctxt.frames.push(StreamFrame::Let(let_scope));
                                }
                                Op::PopLet => {
                                    if let StreamFrame::Let(_let_scope) = ctxt.frames.pop().unwrap()
                                    {
                                    } else {
                                        panic!("expected let scope");
                                    }
                                }
                                Op::PushDynamic(
                                    name,
                                    DynFormat::Huffman(lengths_expr, opt_values_expr),
                                ) => {
                                    let scope = &Scope::Other(ctxt);
                                    let lengths_val = lengths_expr.eval(scope);
                                    let lengths = value_to_vec_usize(&lengths_val);
                                    let lengths = match opt_values_expr {
                                        None => lengths,
                                        Some(e) => {
                                            let values = value_to_vec_usize(&e.eval(scope));
                                            let mut new_lengths = [0].repeat(values.len());
                                            for i in 0..lengths.len() {
                                                new_lengths[values[i]] = lengths[i];
                                            }
                                            new_lengths
                                        }
                                    };
                                    let f = make_huffman_codes(&lengths);
                                    let dyn_d = Decoder::compile_one(&f).unwrap();
                                    let decoder_scope = DecoderScope::new(name, dyn_d);
                                    ctxt.frames.push(StreamFrame::Decoder(decoder_scope));
                                }
                                Op::PopDynamic => {
                                    if let StreamFrame::Decoder(_decoder_scope) =
                                        ctxt.frames.pop().unwrap()
                                    {
                                    } else {
                                        panic!("expected decoder scope");
                                    }
                                }
                                Op::ApplyDynamic(name) => {
                                    let scope = &Scope::Other(ctxt);
                                    let d = scope.get_decoder_by_name(&name);
                                    match d.parse(&decoder::Program::new(), scope, input) {
                                        Ok((v, new_input)) => {
                                            input = new_input;
                                            ctxt.values.push(v);
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::PushInput => {
                                    input_stack.push(input.clone());
                                }
                                Op::PushInputSlice(expr) => {
                                    let scope = &Scope::Other(ctxt);
                                    let size = expr.eval_value(scope).unwrap_usize();
                                    match input
                                        .split_at(size)
                                        .ok_or(ParseError::overrun(size, input.offset))
                                    {
                                        Ok((slice_input, next_input)) => {
                                            input_stack.push(next_input);
                                            input = slice_input;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::PushInputOffset(expr) => {
                                    let scope = &Scope::Other(ctxt);
                                    let offset = expr.eval_value(scope).unwrap_usize();
                                    match input
                                        .split_at(offset)
                                        .ok_or(ParseError::overrun(offset, input.offset))
                                    {
                                        Ok((_, offset_input)) => {
                                            input_stack.push(input);
                                            input = offset_input;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::PopInput => {
                                    input = input_stack.pop().unwrap();
                                }
                                Op::Bits(s) => {
                                    let mut bits = Vec::with_capacity(input.remaining().len() * 8);
                                    let mut new_input_stack: Vec<ReadCtxt<'_>> = Vec::new();
                                    for b in input.remaining() {
                                        for i in 0..8 {
                                            bits.push((b & (1 << i)) >> i);
                                        }
                                    }
                                    match s.eval(
                                        program,
                                        ctxt,
                                        &mut new_input_stack,
                                        ReadCtxt::new(&bits),
                                    ) {
                                        Ok((v, bits)) => {
                                            let bytes_remain = bits.remaining().len() >> 3;
                                            let bytes_read = input.remaining().len() - bytes_remain;
                                            let (_, new_input) =
                                                input.split_at(bytes_read).unwrap();
                                            ctxt.values.push(v);
                                            input = new_input;
                                        }
                                        Err(err) => break 'oploop Err(err),
                                    }
                                }
                                Op::Negated(s) => {
                                    if s.eval_clean(program, Some(ctxt), input).is_ok() {
                                        break 'oploop Err(ParseError::fail(
                                            &Scope::Other(ctxt),
                                            input,
                                        ));
                                    } else {
                                        ctxt.values.push(Value::UNIT);
                                    }
                                }
                            }
                        }
                        break 'oploop Ok(());
                    } {
                        Ok(()) => {
                            let v = ctxt.values.pop().unwrap();
                            return Ok((v, input));
                        }
                        Err(err) => return Err(err),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use crate::IntoLabel;

    use super::*;

    fn alts<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
        Format::UnionVariant(
            (fields.into_iter())
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }

    fn record<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
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

    fn accepts(program: &Program, input: &[u8], tail: &[u8], expect: Value) {
        let (val, remain) = program.run(ReadCtxt::new(input)).unwrap();
        assert_eq!(val, expect);
        assert_eq!(remain.remaining(), tail);
    }

    fn rejects(program: &Program, input: &[u8]) {
        assert!(program.run(ReadCtxt::new(input)).is_err());
    }

    #[test]
    fn compile_fail() {
        let f = Format::Fail;
        let d = Block::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_empty() {
        let f = Format::EMPTY;
        let d = Block::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[0x00], Value::UNIT);
    }

    #[test]
    fn compile_byte_is() {
        let f = is_byte(0x00);
        let d = Block::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0));
        accepts(&d, &[0x00, 0xFF], &[0xFF], Value::U8(0));
        rejects(&d, &[0xFF]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_byte_not() {
        let f = not_byte(0x00);
        let d = Block::compile_one(&f).unwrap();
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        accepts(&d, &[0xFF, 0x00], &[0x00], Value::U8(0xFF));
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt() {
        let f = alts::<&str>([]);
        let d = Block::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_alt_byte() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0xFF))]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0xFF)))),
        );
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_ambiguous() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0x00))]);
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_byte() {
        let slice_a = Format::Slice(Expr::U8(1), Box::new(is_byte(0x00)));
        let slice_b = Format::Slice(Expr::U8(1), Box::new(is_byte(0xFF)));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0xFF)))),
        );
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_slice_ambiguous1() {
        let slice_a = Format::Slice(Expr::U8(1), Box::new(is_byte(0x00)));
        let slice_b = Format::Slice(Expr::U8(1), Box::new(is_byte(0x00)));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_ambiguous2() {
        let tuple_a = Format::Tuple(vec![is_byte(0x00), is_byte(0x00)]);
        let tuple_b = Format::Tuple(vec![is_byte(0x00), is_byte(0xFF)]);
        let slice_a = Format::Slice(Expr::U8(1), Box::new(tuple_a));
        let slice_b = Format::Slice(Expr::U8(1), Box::new(tuple_b));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail() {
        let f = alts([("a", Format::Fail), ("b", Format::Fail)]);
        let d = Block::compile_one(&f).unwrap();
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_end_of_input() {
        let f = alts([("a", Format::EndOfInput), ("b", Format::EndOfInput)]);
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_empty() {
        let f = alts([("a", Format::EMPTY), ("b", Format::EMPTY)]);
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail_end_of_input() {
        let f = alts([("a", Format::Fail), ("b", Format::EndOfInput)]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::UNIT))),
        );
    }

    #[test]
    fn compile_alt_end_of_input_or_byte() {
        let f = alts([("a", Format::EndOfInput), ("b", is_byte(0x00))]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::UNIT))),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[0x00, 0x00],
            &[0x00],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0x00)))),
        );
        rejects(&d, &[0x11]);
    }

    #[test]
    fn compile_alt_opt() {
        let f = alts([("a", Format::EMPTY), ("b", is_byte(0x00))]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::UNIT))),
        );
        accepts(
            &d,
            &[0xFF],
            &[0xFF],
            Value::Branch(0, Box::new(Value::variant("a", Value::UNIT))),
        );
    }

    #[test]
    fn compile_alt_opt_next() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), is_byte(0xFF)]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(Value::variant("some", Value::U8(0)))),
                Value::U8(0xFF),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                Value::U8(0xFF),
            ]),
        );
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_opt_opt() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0xFF))]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(Value::variant("some", Value::U8(0)))),
                Value::Branch(0, Box::new(Value::variant("some", Value::U8(0xFF)))),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(Value::variant("some", Value::U8(0)))),
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                Value::Branch(0, Box::new(Value::variant("some", Value::U8(0xFF)))),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
            ]),
        );
    }

    #[test]
    fn compile_alt_opt_ambiguous() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0x00))]);
        assert!(Block::compile_one(&f).is_err());
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
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_alt_repeat1_slow() {
        let f = repeat(alts([
            ("a", repeat1(is_byte(0x00))),
            ("b", is_byte(0x01)),
            ("c", is_byte(0x02)),
        ]));
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Block::compile_one(&f).unwrap();
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
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_cat_repeat() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0xFF))]);
        let d = Block::compile_one(&f).unwrap();
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
        let d = Block::compile_one(&f).unwrap();
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
        let d = Block::compile_one(&f).unwrap();
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
        assert!(Block::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x7F))),
        ]);
        assert!(Block::compile_one(&f).is_ok());
    }

    #[test]
    fn compile_repeat_fields_ambiguous() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x00))),
        ]);
        assert!(Block::compile_one(&f).is_err());
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
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::record([
                ("first", Value::Seq(vec![])),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)])),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                ),
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
                    Value::Branch(
                        0,
                        Box::new(Value::variant(
                            "some",
                            Value::record([
                                (
                                    "second",
                                    Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                                ),
                                ("third", Value::Seq(vec![])),
                            ]),
                        )),
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
                    Value::Branch(
                        0,
                        Box::new(Value::variant(
                            "some",
                            Value::record(vec![
                                (
                                    "second",
                                    Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![])]),
                                ),
                                ("third", Value::Seq(vec![Value::U8(0x00)])),
                            ]),
                        )),
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
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::variant("none", Value::UNIT))),
                ),
            ]),
        );
    }

    #[test]
    fn compile_repeat1() {
        let f = repeat1(is_byte(0x00));
        let d = Block::compile_one(&f).unwrap();
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

    #[test]
    fn compile_align1() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(1), is_byte(0xFF)]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_align2() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(2), is_byte(0xFF)]);
        let d = Block::compile_one(&f).unwrap();
        rejects(&d, &[0x00, 0xFF]);
        rejects(&d, &[0x00, 0x99, 0x99, 0xFF]);
        accepts(
            &d,
            &[0x00, 0x99, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_peek_not() {
        let any_byte = Format::Byte(ByteSet::full());
        let a = Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)]);
        let peek_not = Format::PeekNot(Box::new(a));
        let f = Format::Tuple(vec![peek_not, any_byte.clone(), any_byte.clone()]);
        let d = Block::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0xFF]);
        rejects(&d, &[0xFF, 0xFF]);
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::Tuple(vec![]), Value::U8(0x00), Value::U8(0xFF)]),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Tuple(vec![Value::Tuple(vec![]), Value::U8(0xFF), Value::U8(0x00)]),
        );
    }

    #[test]
    fn compile_peek_not_switch() {
        let any_byte = Format::Byte(ByteSet::full());
        let guard = Format::PeekNot(Box::new(Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)])));
        let a = Format::Tuple(vec![guard, Format::Repeat(Box::new(any_byte.clone()))]);
        let b = Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)]);
        let f = alts([("a", a), ("b", b)]);
        let d = Block::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![Value::Tuple(vec![]), Value::Seq(vec![])])),
                )),
            ),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![Value::U8(0xFF)]),
                    ])),
                )),
            ),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![Value::U8(0x00), Value::U8(0xFF)]),
                    ])),
                )),
            ),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![Value::U8(0xFF), Value::U8(0x00)]),
                    ])),
                )),
            ),
        );
        accepts(
            &d,
            &[0xFF, 0xFF],
            &[],
            Value::Branch(
                1,
                Box::new(Value::Variant(
                    "b".into(),
                    Box::new(Value::Tuple(vec![Value::U8(0xFF), Value::U8(0xFF)])),
                )),
            ),
        );
    }

    #[test]
    fn compile_peek_not_lookahead() {
        let peek_not = Format::PeekNot(Box::new(repeat1(is_byte(0x00))));
        let any_byte = Format::Byte(ByteSet::full());
        let f = Format::Tuple(vec![peek_not, repeat1(any_byte)]);
        assert!(Block::compile_one(&f).is_err());
    }
}
