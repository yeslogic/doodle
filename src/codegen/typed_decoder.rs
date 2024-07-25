use crate::byte_set::ByteSet;
use crate::{Format, FormatModule, Label, MatchTree, MaybeTyped, Next};
use anyhow::{anyhow, Result as AResult};
use std::collections::HashMap;
use std::rc::Rc;

use crate::codegen::typed_format::{GenType, TypedPattern};

use super::{
    typed_format::{TypedDynFormat, TypedExpr, TypedFormat},
    GTFormat,
};

#[derive(Clone, Debug)]
pub(crate) struct TypedDecoderExt<TypeRep> {
    dec: TypedDecoder<TypeRep>,
    args: Option<Vec<(Label, TypeRep)>>,
}

impl<TypeRep> TypedDecoderExt<TypeRep> {
    pub fn new(dec: TypedDecoder<TypeRep>, args: Option<Vec<(Label, TypeRep)>>) -> Self {
        Self { dec, args }
    }

    pub fn get_args(&self) -> &Option<Vec<(Label, TypeRep)>> {
        &self.args
    }

    pub fn get_dec(&self) -> &TypedDecoder<TypeRep> {
        &self.dec
    }
}

impl<TypeRep> AsRef<TypedDecoder<TypeRep>> for TypedDecoderExt<TypeRep> {
    fn as_ref(&self) -> &TypedDecoder<TypeRep> {
        self.get_dec()
    }
}

impl<TypeRep> From<TypedDecoder<TypeRep>> for TypedDecoderExt<TypeRep> {
    fn from(value: TypedDecoder<TypeRep>) -> Self {
        Self {
            dec: value,
            args: None,
        }
    }
}

/// Decoders with a fixed amount of lookahead
#[derive(Clone, Debug)]
pub(crate) enum TypedDecoder<TypeRep> {
    Call(TypeRep, usize, Vec<(Label, TypedExpr<TypeRep>)>),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Variant(TypeRep, Label, Box<TypedDecoderExt<TypeRep>>),
    Parallel(TypeRep, Vec<TypedDecoderExt<TypeRep>>),
    Branch(TypeRep, MatchTree, Vec<TypedDecoderExt<TypeRep>>),
    Tuple(TypeRep, Vec<TypedDecoderExt<TypeRep>>),
    Record(TypeRep, Vec<(Label, TypedDecoderExt<TypeRep>)>),
    Repeat0While(TypeRep, MatchTree, Box<TypedDecoderExt<TypeRep>>),
    Repeat1Until(TypeRep, MatchTree, Box<TypedDecoderExt<TypeRep>>),
    RepeatCount(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoderExt<TypeRep>>),
    RepeatBetween(
        TypeRep,
        MatchTree,
        TypedExpr<TypeRep>,
        TypedExpr<TypeRep>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    RepeatUntilLast(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoderExt<TypeRep>>),
    RepeatUntilSeq(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoderExt<TypeRep>>),
    Peek(TypeRep, Box<TypedDecoderExt<TypeRep>>),
    PeekNot(TypeRep, Box<TypedDecoderExt<TypeRep>>),
    Slice(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoderExt<TypeRep>>),
    Bits(TypeRep, Box<TypedDecoderExt<TypeRep>>),
    WithRelativeOffset(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoderExt<TypeRep>>),
    Map(TypeRep, Box<TypedDecoderExt<TypeRep>>, TypedExpr<TypeRep>),
    Where(TypeRep, Box<TypedDecoderExt<TypeRep>>, TypedExpr<TypeRep>),
    Compute(TypeRep, TypedExpr<TypeRep>),
    Let(
        TypeRep,
        Label,
        TypedExpr<TypeRep>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Match(
        TypeRep,
        TypedExpr<TypeRep>,
        Vec<(TypedPattern<TypeRep>, TypedDecoderExt<TypeRep>)>,
    ),
    Dynamic(
        TypeRep,
        Label,
        TypedDynFormat<TypeRep>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Apply(TypeRep, Label),
    Maybe(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoderExt<TypeRep>>),
    Pos,
}

#[derive(Clone, Debug)]
pub(crate) struct TypedProgram<TypeRep> {
    pub decoders: Vec<(TypedDecoderExt<TypeRep>, TypeRep)>,
}

impl TypedProgram<GenType> {
    fn new() -> Self {
        let decoders = Vec::new();
        TypedProgram { decoders }
    }
}

pub struct GTCompiler<'a> {
    module: &'a FormatModule,
    program: TypedProgram<GenType>,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<(
        &'a GTFormat,
        Option<Vec<(Label, GenType)>>,
        Rc<Next<'a>>,
        usize,
    )>,
}

pub(crate) type GTDecoder = TypedDecoder<GenType>;
pub(crate) type GTDecoderExt = TypedDecoderExt<GenType>;

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
        compiler.queue_compile(t, format, None, Rc::new(Next::Empty));
        while let Some((f, args, next, n)) = compiler.compile_queue.pop() {
            let d = compiler.compile_gt_format(f, args, next)?;
            compiler.program.decoders[n].0 = d;
        }
        Ok(compiler.program)
    }

    fn queue_compile(
        &mut self,
        t: GenType,
        f: &'a GTFormat,
        args: Option<Vec<(Label, GenType)>>,
        next: Rc<Next<'a>>,
    ) -> usize {
        let n = self.program.decoders.len();
        self.program.decoders.push((TypedDecoder::Fail.into(), t));
        self.compile_queue.push((f, args, next, n));
        n
    }

    fn compile_gt_format(
        &mut self,
        format: &'a GTFormat,
        args: Option<Vec<(Label, GenType)>>,
        next: Rc<Next<'a>>,
    ) -> AResult<GTDecoderExt> {
        let dec = match format {
            GTFormat::FormatCall(gt, level, arg_exprs, deref) => {
                let this_args = arg_exprs.iter().cloned().collect();
                let sig_args = if arg_exprs.is_empty() {
                    None
                } else {
                    Some(
                        arg_exprs
                            .iter()
                            .map(|(lab, gtx)| {
                                (
                                    lab.clone(),
                                    gtx.get_type()
                                        .expect("found lambda in format args")
                                        .into_owned(),
                                )
                            })
                            .collect(),
                    )
                };

                let _f = self.module.get_format(*level);
                let next = if _f.depends_on_next(self.module) {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = self.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let n = self.queue_compile(gt.clone(), deref, sig_args, next.clone());
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };

                Ok(TypedDecoder::Call(gt.clone(), n, this_args))
            }
            GTFormat::Fail => Ok(TypedDecoder::Fail),
            GTFormat::EndOfInput => Ok(TypedDecoder::EndOfInput),
            GTFormat::Align(n) => Ok(TypedDecoder::Align(*n)),
            GTFormat::Byte(bs) => Ok(TypedDecoder::Byte(*bs)),
            GTFormat::Variant(gt, label, f) => {
                let d = self.compile_gt_format(f, None, next.clone())?;
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
                    ds.push(self.compile_gt_format(f, None, next.clone())?);
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
                    let d = self.compile_gt_format(f, None, next.clone())?;
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
                    let df = self.compile_gt_format(&f, None, next)?;
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
                    let df = self.compile_gt_format(f, None, next)?;
                    dfields.push((name.clone(), df));
                }
                Ok(TypedDecoder::Record(gt.clone(), dfields))
            }
            GTFormat::Repeat(gt, a) => {
                if a.as_ref().is_nullable() {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_gt_format(
                    a,
                    None,
                    Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                )?;
                let astar = TypedFormat::Repeat(gt.clone(), a.clone());
                let fa = TypedFormat::tuple(vec![(**a).clone(), astar]);
                let fb = TypedFormat::EMPTY;
                if let Some(tree) = MatchTree::build(self.module, &[fa.into(), fb.into()], next) {
                    Ok(TypedDecoder::Repeat0While(gt.clone(), tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::Repeat1(gt, a) => {
                if a.is_nullable() {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_gt_format(
                    a,
                    None,
                    Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                )?;
                let astar = TypedFormat::Repeat(gt.clone(), a.clone());
                let fa = TypedFormat::EMPTY;
                let fb = TypedFormat::tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build(self.module, &[fa.into(), fb.into()], next) {
                    Ok(TypedDecoder::Repeat1Until(gt.clone(), tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::RepeatCount(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatCount(gt.clone(), expr.clone(), da))
            }
            GTFormat::RepeatBetween(gt, min_expr, max_expr, a) => {
                // FIXME - preliminary support only for exact-bound limit values
                let Some(min) = min_expr.bounds().is_exact() else {
                    unimplemented!("RepeatBetween on inexact bounds-expr")
                };
                let Some(max) = max_expr.bounds().is_exact() else {
                    unimplemented!("RepeatBetween on inexact bounds-expr")
                };

                let da = self.compile_gt_format(
                    a,
                    None,
                    Rc::new(Next::RepeatBetween(
                        min.saturating_sub(1),
                        max.saturating_sub(1),
                        MaybeTyped::Typed(a),
                        next.clone(),
                    )),
                )?;

                let tree = {
                    let mut branches: Vec<Format> = Vec::new();
                    // FIXME: this is inefficient but probably works
                    for count in 0..=max {
                        let f_count = TypedFormat::RepeatCount(
                            gt.clone(),
                            TypedExpr::U32(count as u32),
                            a.clone(),
                        );
                        branches.push(f_count.into());
                    }
                    let Some(tree) = MatchTree::build(self.module, &branches[..], next) else {
                        panic!("cannot build match tree for {:?}", format)
                    };
                    tree
                };
                Ok(TypedDecoder::RepeatBetween(
                    gt.clone(),
                    tree,
                    min_expr.clone(),
                    max_expr.clone(),
                    Box::new(da),
                ))
            }
            GTFormat::RepeatUntilLast(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatUntilLast(gt.clone(), expr.clone(), da))
            }
            GTFormat::RepeatUntilSeq(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatUntilSeq(gt.clone(), expr.clone(), da))
            }
            GTFormat::Maybe(gt, cond, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::Maybe(gt.clone(), cond.clone(), da))
            }
            GTFormat::Peek(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Peek(gt.clone(), da))
            }
            GTFormat::PeekNot(_t, a) => {
                const MAX_LOOKAHEAD: usize = 1024;
                match a.lookahead_bounds().max {
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
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::PeekNot(_t.clone(), da))
            }
            GTFormat::Slice(gt, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Slice(gt.clone(), expr.clone(), da))
            }
            GTFormat::Bits(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Bits(gt.clone(), da))
            }
            GTFormat::WithRelativeOffset(gt, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::WithRelativeOffset(
                    gt.clone(),
                    expr.clone(),
                    da,
                ))
            }
            GTFormat::Map(gt, a, expr) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Map(gt.clone(), da, expr.clone()))
            }
            GTFormat::Where(gt, a, expr) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Where(gt.clone(), da, expr.clone()))
            }
            GTFormat::Compute(gt, expr) => Ok(TypedDecoder::Compute(gt.clone(), expr.clone())),
            GTFormat::Pos => Ok(TypedDecoder::Pos),
            GTFormat::Let(gt, name, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
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
                        Ok((
                            pattern.clone(),
                            self.compile_gt_format(f, None, next.clone())?,
                        ))
                    })
                    .collect::<AResult<_>>()?;
                Ok(TypedDecoder::Match(gt.clone(), head.clone(), branches))
            }
            GTFormat::Dynamic(gt, name, dynformat, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Dynamic(
                    gt.clone(),
                    name.clone(),
                    dynformat.clone(),
                    da,
                ))
            }
            GTFormat::Apply(gt, name, _) => Ok(TypedDecoder::Apply(gt.clone(), name.clone())),
        }?;
        Ok(TypedDecoderExt::new(dec, args))
    }
}
