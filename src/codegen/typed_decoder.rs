use crate::byte_set::ByteSet;
use crate::{FormatModule, Label, MatchTree, MaybeTyped, Next};
use anyhow::{anyhow, Result as AResult};
use std::collections::HashMap;
use std::rc::Rc;

use crate::codegen::typed_format::{GenType, TypedPattern};

use super::{
    typed_format::{TypedDynFormat, TypedExpr, TypedFormat},
    GTFormat,
};

/// Decoders with a fixed amount of lookahead
#[derive(Clone, Debug)]
pub(crate) enum TypedDecoder<TypeRep> {
    Call(TypeRep, usize, Vec<(Label, TypedExpr<TypeRep>)>),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Variant(TypeRep, Label, Box<TypedDecoder<TypeRep>>),
    Parallel(TypeRep, Vec<TypedDecoder<TypeRep>>),
    Branch(TypeRep, MatchTree, Vec<TypedDecoder<TypeRep>>),
    Tuple(TypeRep, Vec<TypedDecoder<TypeRep>>),
    Record(TypeRep, Vec<(Label, TypedDecoder<TypeRep>)>),
    While(TypeRep, MatchTree, Box<TypedDecoder<TypeRep>>),
    Until(TypeRep, MatchTree, Box<TypedDecoder<TypeRep>>),
    RepeatCount(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    RepeatUntilLast(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    RepeatUntilSeq(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    Peek(TypeRep, Box<TypedDecoder<TypeRep>>),
    PeekNot(TypeRep, Box<TypedDecoder<TypeRep>>),
    Slice(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    Bits(TypeRep, Box<TypedDecoder<TypeRep>>),
    WithRelativeOffset(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    Map(TypeRep, Box<TypedDecoder<TypeRep>>, TypedExpr<TypeRep>),
    Compute(TypeRep, TypedExpr<TypeRep>),
    Let(
        TypeRep,
        Label,
        TypedExpr<TypeRep>,
        Box<TypedDecoder<TypeRep>>,
    ),
    Match(
        TypeRep,
        TypedExpr<TypeRep>,
        Vec<(TypedPattern<TypeRep>, TypedDecoder<TypeRep>)>,
    ),
    Dynamic(
        TypeRep,
        Label,
        TypedDynFormat<TypeRep>,
        Box<TypedDecoder<TypeRep>>,
    ),
    Apply(TypeRep, Label),
}

#[derive(Clone, Debug)]
pub(crate) struct TypedProgram<TypeRep> {
    pub decoders: Vec<(TypedDecoder<TypeRep>, TypeRep)>,
}

impl TypedProgram<GenType> {
    fn new() -> Self {
        let decoders = Vec::new();
        TypedProgram { decoders }
    }

    // pub fn run<'input>(
    //     &self,
    //     input: ReadCtxt<'input>
    // ) -> ParseResult<(TypedValue<GenType>, ReadCtxt<'input>)> {
    //     self.decoders[0].0.parse(self, &TScope::Empty, input)
    // }
}

pub struct GTCompiler<'a> {
    module: &'a FormatModule,
    program: TypedProgram<GenType>,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<(&'a GTFormat, Rc<Next<'a>>, usize)>,
}

pub(crate) type GTDecoder = TypedDecoder<GenType>;

impl<'a> GTCompiler<'a> {
    fn new(module: &'a FormatModule) -> Self {
        let program = TypedProgram::new();
        let decoder_map = HashMap::new();
        let compile_queue = Vec::new();
        GTCompiler {
            module,
            program,
            decoder_map,
            compile_queue,
        }
    }

    pub(crate) fn compile_program(
        module: &FormatModule,
        format: &GTFormat,
    ) -> AResult<TypedProgram<GenType>> {
        let mut compiler = GTCompiler::new(module);
        // type
        let t = match format.get_type() {
            None => unreachable!("cannot compile program from Void top-level format-type"),
            Some(t) => t.into_owned(),
        };
        // decoder
        compiler.queue_compile(t, format, Rc::new(Next::Empty));
        while let Some((f, next, n)) = compiler.compile_queue.pop() {
            let d = compiler.compile_gt_format(f, next)?;
            compiler.program.decoders[n].0 = d;
        }
        Ok(compiler.program)
    }

    fn queue_compile(&mut self, t: GenType, f: &'a GTFormat, next: Rc<Next<'a>>) -> usize {
        let n = self.program.decoders.len();
        self.program.decoders.push((TypedDecoder::Fail, t));
        self.compile_queue.push((f, next, n));
        n
    }

    fn compile_gt_format(
        &mut self,
        format: &'a GTFormat,
        next: Rc<Next<'a>>,
    ) -> AResult<GTDecoder> {
        match format {
            GTFormat::FormatCall(gt, level, arg_exprs, deref) => {
                let _f = self.module.get_format(*level);
                let next = if _f.depends_on_next(self.module) {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = self.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let n = self.queue_compile(gt.clone(), deref, next.clone());
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                let mut args = Vec::new();
                for (label, expr) in arg_exprs.iter() {
                    args.push((label.clone(), expr.clone()));
                }
                Ok(TypedDecoder::Call(gt.clone(), n, args))
            }
            GTFormat::Fail => Ok(TypedDecoder::Fail),
            GTFormat::EndOfInput => Ok(TypedDecoder::EndOfInput),
            GTFormat::Align(n) => Ok(TypedDecoder::Align(*n)),
            GTFormat::Byte(bs) => Ok(TypedDecoder::Byte(*bs)),
            GTFormat::Variant(gt, label, f) => {
                let d = self.compile_gt_format(f, next.clone())?;
                Ok(TypedDecoder::Variant(
                    gt.clone(),
                    label.clone(),
                    Box::new(d),
                ))
            }
            GTFormat::Union(gt, branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_gt_format(f, next.clone())?);
                    fs.push(f.clone().into());
                }
                if let Some(tree) = MatchTree::build(self.module, &fs, next) {
                    Ok(TypedDecoder::Branch(gt.clone(), tree, ds))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::UnionNondet(gt, branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    let d = self.compile_gt_format(f, next.clone())?;
                    ds.push(d);
                }
                Ok(TypedDecoder::Parallel(gt.clone(), ds))
            }
            GTFormat::Tuple(gt, fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Tuple(
                        MaybeTyped::Typed(fields.as_slice()),
                        next.clone(),
                    ));
                    let df = self.compile_gt_format(&f, next)?;
                    dfields.push(df);
                }
                Ok(TypedDecoder::Tuple(gt.clone(), dfields))
            }
            GTFormat::Record(gt, fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some((name, f)) = fields.next() {
                    let next = Rc::new(Next::Record(
                        MaybeTyped::Typed(fields.as_slice()),
                        next.clone(),
                    ));
                    let df = self.compile_gt_format(f, next)?;
                    dfields.push((name.clone(), df));
                }
                Ok(TypedDecoder::Record(gt.clone(), dfields))
            }
            GTFormat::Repeat(gt, a) => {
                if a.as_ref().is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_gt_format(
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                )?;
                let astar = TypedFormat::Repeat(gt.clone(), a.clone());
                let fa = TypedFormat::tuple(vec![(**a).clone(), astar]);
                let fb = TypedFormat::EMPTY;
                if let Some(tree) = MatchTree::build(self.module, &[fa.into(), fb.into()], next) {
                    Ok(TypedDecoder::While(gt.clone(), tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::Repeat1(gt, a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_gt_format(
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                )?;
                let astar = TypedFormat::Repeat(gt.clone(), a.clone());
                let fa = TypedFormat::EMPTY;
                let fb = TypedFormat::tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build(self.module, &[fa.into(), fb.into()], next) {
                    Ok(TypedDecoder::Until(gt.clone(), tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::RepeatCount(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, next)?);
                Ok(TypedDecoder::RepeatCount(gt.clone(), expr.clone(), da))
            }
            GTFormat::RepeatUntilLast(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, next)?);
                Ok(TypedDecoder::RepeatUntilLast(gt.clone(), expr.clone(), da))
            }
            GTFormat::RepeatUntilSeq(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, next)?);
                Ok(TypedDecoder::RepeatUntilSeq(gt.clone(), expr.clone(), da))
            }
            GTFormat::Peek(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Peek(gt.clone(), da))
            }
            GTFormat::PeekNot(_t, a) => {
                const MAX_LOOKAHEAD: usize = 1024;
                match a.match_bounds(self.module).max {
                    None => {
                        return Err(anyhow!("PeekNot cannot require unbounded lookahead"));
                    }
                    Some(n) if n > MAX_LOOKAHEAD => {
                        return Err(anyhow!(
                            "PeekNot cannot require > {MAX_LOOKAHEAD} bytes lookahead"
                        ));
                    }
                    _ => {}
                }
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::PeekNot(_t.clone(), da))
            }
            GTFormat::Slice(gt, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Slice(gt.clone(), expr.clone(), da))
            }
            GTFormat::Bits(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Bits(gt.clone(), da))
            }
            GTFormat::WithRelativeOffset(gt, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::WithRelativeOffset(
                    gt.clone(),
                    expr.clone(),
                    da,
                ))
            }
            GTFormat::Map(gt, a, expr) => {
                let da = Box::new(self.compile_gt_format(a, next.clone())?);
                Ok(TypedDecoder::Map(gt.clone(), da, expr.clone()))
            }
            GTFormat::Compute(gt, expr) => Ok(TypedDecoder::Compute(gt.clone(), expr.clone())),
            GTFormat::Let(gt, name, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, next.clone())?);
                Ok(TypedDecoder::Let(
                    gt.clone(),
                    name.clone(),
                    expr.clone(),
                    da,
                ))
            }
            GTFormat::Match(gt, head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| {
                        Ok((pattern.clone(), self.compile_gt_format(f, next.clone())?))
                    })
                    .collect::<AResult<_>>()?;
                Ok(TypedDecoder::Match(gt.clone(), head.clone(), branches))
            }
            GTFormat::Dynamic(gt, name, dynformat, a) => {
                let da = Box::new(self.compile_gt_format(a, next.clone())?);
                Ok(TypedDecoder::Dynamic(
                    gt.clone(),
                    name.clone(),
                    dynformat.clone(),
                    da,
                ))
            }
            GTFormat::Apply(gt, name, _) => Ok(TypedDecoder::Apply(gt.clone(), name.clone())),
        }
    }
}
