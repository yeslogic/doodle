use crate::byte_set::ByteSet;
use crate::{
    BaseKind, Endian, Format, FormatModule, Label, MatchTree, MaybeTyped, Next, StyleHint,
};
use anyhow::{Result as AResult, anyhow};
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::codegen::typed_format::{GenType, TypedPattern, TypedViewExpr};

use super::typed_format::TypedViewFormat;
use super::{
    GTFormat,
    typed_format::{TypedDynFormat, TypedExpr, TypedFormat},
};
use super::{PrimType, RustType};

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

impl TypedDecoder<GenType> {
    pub(crate) fn get_type(&self) -> Option<Cow<'_, GenType>> {
        match self {
            TypedDecoder::Fail => None,
            TypedDecoder::Align(_) | TypedDecoder::SkipRemainder | TypedDecoder::EndOfInput => {
                Some(Cow::Owned(GenType::Inline(RustType::from(PrimType::Unit))))
            }
            TypedDecoder::Byte(set) => {
                (!set.is_empty()).then_some(Cow::Owned(GenType::from(PrimType::U8)))
            }
            TypedDecoder::Pos => Some(Cow::Owned(GenType::from(PrimType::U64))),
            TypedDecoder::Call(t, ..)
            | TypedDecoder::Variant(t, ..)
            | TypedDecoder::Parallel(t, ..)
            | TypedDecoder::Branch(t, ..)
            | TypedDecoder::Tuple(t, ..)
            | TypedDecoder::Sequence(t, ..)
            | TypedDecoder::Repeat0While(t, ..)
            | TypedDecoder::Repeat1Until(t, ..)
            | TypedDecoder::RepeatCount(t, ..)
            | TypedDecoder::RepeatBetween(t, ..)
            | TypedDecoder::RepeatUntilLast(t, ..)
            | TypedDecoder::RepeatUntilSeq(t, ..)
            | TypedDecoder::Peek(t, ..)
            | TypedDecoder::PeekNot(t, ..)
            | TypedDecoder::Slice(t, ..)
            | TypedDecoder::Bits(t, ..)
            | TypedDecoder::WithRelativeOffset(t, ..)
            | TypedDecoder::Map(t, ..)
            | TypedDecoder::Where(t, ..)
            | TypedDecoder::Compute(t, ..)
            | TypedDecoder::Let(t, ..)
            | TypedDecoder::LetView(t, ..)
            | TypedDecoder::CaptureBytes(t, ..)
            | TypedDecoder::ReadArray(t, ..)
            | TypedDecoder::ParseFromView(t, ..)
            | TypedDecoder::ReifyView(t, ..)
            | TypedDecoder::Match(t, ..)
            | TypedDecoder::Dynamic(t, ..)
            | TypedDecoder::Apply(t, ..)
            | TypedDecoder::Maybe(t, ..)
            | TypedDecoder::ForEach(t, ..)
            | TypedDecoder::DecodeBytes(t, ..)
            | TypedDecoder::LetFormat(t, ..)
            | TypedDecoder::MonadSeq(t, ..)
            | TypedDecoder::Hint(t, ..)
            | TypedDecoder::LiftedOption(t, ..)
            | TypedDecoder::AccumUntil(t, ..) => Some(Cow::Borrowed(t)),
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
    Sequence(TypeRep, Vec<TypedDecoderExt<TypeRep>>),
    Repeat0While(TypeRep, MatchTree, Box<TypedDecoderExt<TypeRep>>),
    Repeat1Until(TypeRep, MatchTree, Box<TypedDecoderExt<TypeRep>>),
    RepeatCount(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    /// RepeatBetween: the MatchTree is an N-ary decision-tree where the matching index corresponds to the number of unparsed repetitions left in the limited LL(k) window.
    RepeatBetween(
        TypeRep,
        MatchTree,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    RepeatUntilLast(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    RepeatUntilSeq(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Peek(TypeRep, Box<TypedDecoderExt<TypeRep>>),
    PeekNot(TypeRep, Box<TypedDecoderExt<TypeRep>>),
    Slice(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Bits(TypeRep, Box<TypedDecoderExt<TypeRep>>),
    WithRelativeOffset(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Map(
        TypeRep,
        Box<TypedDecoderExt<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
    Where(
        TypeRep,
        Box<TypedDecoderExt<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
    Compute(TypeRep, Box<TypedExpr<TypeRep>>),
    Let(
        TypeRep,
        Label,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Match(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Vec<(TypedPattern<TypeRep>, TypedDecoderExt<TypeRep>)>,
    ),
    Dynamic(
        TypeRep,
        Label,
        TypedDynFormat<TypeRep>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Apply(TypeRep, Label),
    Maybe(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Pos,
    ForEach(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Label,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    SkipRemainder,
    DecodeBytes(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    LetFormat(
        TypeRep,
        Box<TypedDecoderExt<TypeRep>>,
        Label,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    MonadSeq(
        TypeRep,
        Box<TypedDecoderExt<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    AccumUntil(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    Hint(TypeRep, StyleHint, Box<TypedDecoderExt<TypeRep>>),
    LiftedOption(TypeRep, Option<Box<TypedDecoderExt<TypeRep>>>),
    LetView(TypeRep, Label, Box<TypedDecoderExt<TypeRep>>),
    CaptureBytes(TypeRep, TypedViewExpr<TypeRep>, Box<TypedExpr<TypeRep>>),
    ParseFromView(
        TypeRep,
        TypedViewExpr<TypeRep>,
        Box<TypedDecoderExt<TypeRep>>,
    ),
    ReadArray(
        TypeRep,
        TypedViewExpr<TypeRep>,
        Box<TypedExpr<TypeRep>>,
        BaseKind<Endian>,
    ),
    ReifyView(TypeRep, TypedViewExpr<TypeRep>),
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

type QueueItem<'a> = (
    &'a GTFormat,
    Option<Vec<(Label, GenType)>>,
    Rc<Next<'a>>,
    usize,
);

pub struct GTCompiler<'a> {
    module: &'a FormatModule,
    program: TypedProgram<GenType>,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<QueueItem<'a>>,
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
        extra: impl IntoIterator<Item = &'a GTFormat>,
    ) -> AResult<TypedProgram<GenType>> {
        let mut compiler = GTCompiler::new(module);

        compiler.compile_local_root(format, false)?;

        for extra_f in extra {
            compiler.compile_local_root(extra_f, true)?;
        }

        Ok(compiler.program)
    }

    fn compile_local_root(&mut self, format: &'a GTFormat, is_extra: bool) -> AResult<()> {
        let t = match format.get_type() {
            None => unreachable!("cannot compile program from local-root format with Void type"),
            Some(t) => t.into_owned(),
        };

        // skip extra formats that are not ad-hoc
        if is_extra && t.try_as_adhoc().is_none() {
            return Ok(());
        }

        self.queue_compile(t, format, None, Rc::new(Next::Empty));
        while let Some((f, args, next, n)) = self.compile_queue.pop() {
            let d = self.compile_gt_format(f, args, next)?;
            self.program.decoders[n].0 = d;
        }
        Ok(())
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
            TypedFormat::FormatCall(gt, level, arg_exprs, deref) => {
                let this_args = arg_exprs.to_vec();
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
            TypedFormat::DecodeBytes(gt, expr, f) => {
                let da = Box::new(self.compile_gt_format(f, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::DecodeBytes(gt.clone(), expr.clone(), da))
            }
            TypedFormat::ParseFromView(gt, view, f) => {
                let da = Box::new(self.compile_gt_format(f, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::ParseFromView(gt.clone(), view.clone(), da))
            }
            TypedFormat::ForEach(gt, expr, lbl, f) => {
                let da = Box::new(self.compile_gt_format(f, None, next)?);
                Ok(TypedDecoder::ForEach(
                    gt.clone(),
                    expr.clone(),
                    lbl.clone(),
                    da,
                ))
            }
            TypedFormat::Fail => Ok(TypedDecoder::Fail),
            TypedFormat::EndOfInput => Ok(TypedDecoder::EndOfInput),
            TypedFormat::SkipRemainder => Ok(TypedDecoder::SkipRemainder),
            TypedFormat::Align(n) => Ok(TypedDecoder::Align(*n)),
            TypedFormat::Byte(bs) => Ok(TypedDecoder::Byte(*bs)),
            TypedFormat::Variant(gt, label, f) => {
                let d = self.compile_gt_format(f, None, next.clone())?;
                // FIXME - this is a bit slipshod, maybe we want an inductive method for determining what formats are constructive vs void
                if matches!(&d.dec, TypedDecoder::Fail) {
                    Ok(TypedDecoder::Fail)
                } else {
                    Ok(TypedDecoder::Variant(
                        gt.clone(),
                        label.clone(),
                        Box::new(d),
                    ))
                }
            }
            TypedFormat::Union(gt, branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_gt_format(f, None, next.clone())?);
                    fs.push(f.clone().into());
                }
                if let Some(tree) = MatchTree::build(self.module, &fs, next) {
                    Ok(TypedDecoder::Branch(gt.clone(), tree, ds))
                } else {
                    Err(anyhow!("cannot build match tree for {}", serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()))
                }
            }
            TypedFormat::UnionNondet(gt, branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    let d = self.compile_gt_format(f, None, next.clone())?;
                    ds.push(d);
                }
                Ok(TypedDecoder::Parallel(gt.clone(), ds))
            }
            TypedFormat::Tuple(gt, fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(
                        MaybeTyped::Typed(fields.as_slice()),
                        next.clone(),
                    ));
                    let df = self.compile_gt_format(f, None, next)?;
                    dfields.push(df);
                }
                Ok(TypedDecoder::Tuple(gt.clone(), dfields))
            }
            TypedFormat::Sequence(gt, fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(
                        MaybeTyped::Typed(fields.as_slice()),
                        next.clone(),
                    ));
                    let df = self.compile_gt_format(f, None, next)?;
                    dfields.push(df);
                }
                Ok(TypedDecoder::Sequence(gt.clone(), dfields))
            }
            TypedFormat::Repeat(gt, a) => {
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
                    Err(anyhow!("cannot build match tree for {}", serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()))
                }
            }
            TypedFormat::Repeat1(gt, a) => {
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
                    Err(anyhow!("cannot build match tree for {}", serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()))
                }
            }
            TypedFormat::RepeatCount(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatCount(gt.clone(), expr.clone(), da))
            }
            TypedFormat::RepeatBetween(gt, min_expr, max_expr, a) => {
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

                let reps_left_tree = {
                    let mut branches: Vec<Format> = Vec::new();
                    // FIXME: this is inefficient but probably works
                    for count in 0..=max {
                        let f_count = TypedFormat::RepeatCount(
                            gt.clone(),
                            Box::new(TypedExpr::U32(count as u32)),
                            a.clone(),
                        );
                        branches.push(f_count.into());
                    }
                    let Some(tree) = MatchTree::build(self.module, &branches[..], next) else {
                        return Err(anyhow!("cannot build match tree for {}", serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()))
                    };
                    tree
                };
                Ok(TypedDecoder::RepeatBetween(
                    gt.clone(),
                    reps_left_tree,
                    min_expr.clone(),
                    max_expr.clone(),
                    Box::new(da),
                ))
            }
            TypedFormat::RepeatUntilLast(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatUntilLast(gt.clone(), expr.clone(), da))
            }
            TypedFormat::RepeatUntilSeq(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatUntilSeq(gt.clone(), expr.clone(), da))
            }
            TypedFormat::AccumUntil(gt, cond, update, init, _vt, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::AccumUntil(
                    gt.clone(),
                    cond.clone(),
                    update.clone(),
                    init.clone(),
                    da,
                ))
            }
            TypedFormat::Maybe(gt, cond, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::Maybe(gt.clone(), cond.clone(), da))
            }
            TypedFormat::Peek(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Peek(gt.clone(), da))
            }
            TypedFormat::PeekNot(_t, a) => {
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
            TypedFormat::Slice(gt, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Slice(gt.clone(), expr.clone(), da))
            }
            TypedFormat::Bits(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Bits(gt.clone(), da))
            }
            TypedFormat::WithRelativeOffset(gt, base_addr, offset, a) => {
                let da = Box::new(self.compile_gt_format(a, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::WithRelativeOffset(
                    gt.clone(),
                    base_addr.clone(),
                    offset.clone(),
                    da,
                ))
            }
            TypedFormat::Map(gt, a, expr) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Map(gt.clone(), da, expr.clone()))
            }
            TypedFormat::Where(gt, a, expr) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Where(gt.clone(), da, expr.clone()))
            }
            TypedFormat::Compute(gt, expr) => Ok(TypedDecoder::Compute(gt.clone(), expr.clone())),
            TypedFormat::Pos => Ok(TypedDecoder::Pos),
            TypedFormat::Let(gt, name, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Let(
                    gt.clone(),
                    name.clone(),
                    expr.clone(),
                    da,
                ))
            }
            TypedFormat::LetView(gt, name, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::LetView(gt.clone(), name.clone(), da))
            }
            TypedFormat::Match(gt, head, branches) => {
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
            TypedFormat::Dynamic(gt, name, dynformat, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Dynamic(
                    gt.clone(),
                    name.clone(),
                    dynformat.clone(),
                    da,
                ))
            }
            TypedFormat::Apply(gt, name, _) => Ok(TypedDecoder::Apply(gt.clone(), name.clone())),
            TypedFormat::LetFormat(gt, f0, name, f) => {
                let a_next = Next::Cat(MaybeTyped::Typed(f.as_ref()), next.clone());
                let d0 = Box::new(self.compile_gt_format(f0, None, Rc::new(a_next))?);
                let d = Box::new(self.compile_gt_format(f, None, next)?);
                Ok(TypedDecoder::LetFormat(gt.clone(), d0, name.clone(), d))
            }
            TypedFormat::MonadSeq(gt, f0, f) => {
                let a_next = Next::Cat(MaybeTyped::Typed(f.as_ref()), next.clone());
                let d0 = Box::new(self.compile_gt_format(f0, None, Rc::new(a_next))?);
                let d = Box::new(self.compile_gt_format(f, None, next)?);
                Ok(TypedDecoder::MonadSeq(gt.clone(), d0, d))
            }
            TypedFormat::Hint(gt, hint, a) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Hint(gt.clone(), hint.clone(), da))
            }
            TypedFormat::LiftedOption(gt, opt_f) => {
                let inner_dec = match opt_f {
                    None => None,
                    Some(a) => {
                        let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                        Some(da)
                    }
                };
                Ok(TypedDecoder::LiftedOption(gt.clone(), inner_dec))
            }
            TypedFormat::WithView(gt, view, vf) => match vf {
                TypedViewFormat::CaptureBytes(len) => Ok(TypedDecoder::CaptureBytes(
                    gt.clone(),
                    view.clone(),
                    len.clone(),
                )),
                TypedViewFormat::ReadArray(len, kind) => Ok(TypedDecoder::ReadArray(
                    gt.clone(),
                    view.clone(),
                    len.clone(),
                    *kind,
                )),
                TypedViewFormat::ReifyView => Ok(TypedDecoder::ReifyView(gt.clone(), view.clone())),
            },
        }?;
        Ok(TypedDecoderExt::new(dec, args))
    }
}
