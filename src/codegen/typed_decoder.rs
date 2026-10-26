use crate::byte_set::ByteSet;
use crate::decoder::{CallIndices, merge_identical_decoders};
use crate::validation::TypedCondition;
use crate::{
    Format, FormatModule, Label, MATCH_TREE_MAX_DEPTH, MatchTree, MaybeTyped, Next, NextDependence,
    RepeatRange, StyleHint,
};
use anyhow::{Result as AResult, anyhow};
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::rc::Rc;

use crate::codegen::typed_format::{GenType, TypedFixedReadKind, TypedPattern, TypedViewExpr};

use super::rust_ast::NumType;
use super::typed_format::TypedViewFormat;
use super::{
    GTFormat,
    typed_format::{TypedDynFormat, TypedExpr, TypedFormat},
};
use super::{PrimType, RustLt, RustType};

impl<TypeRep> CallIndices for TypedDecoderExt<TypeRep> {
    fn for_each_call_mut(&mut self, f: &mut dyn FnMut(&mut usize)) {
        match &mut self.dec {
            TypedDecoder::Call(_, ix, _) => f(ix),
            TypedDecoder::Fail
            | TypedDecoder::EndOfInput
            | TypedDecoder::Align(_)
            | TypedDecoder::Byte(_)
            | TypedDecoder::Compute(..)
            | TypedDecoder::Apply(..)
            | TypedDecoder::Pos(_)
            | TypedDecoder::SkipRemainder
            | TypedDecoder::CaptureBytes(..)
            | TypedDecoder::ReadArray(..)
            | TypedDecoder::ReifyView(..)
            | TypedDecoder::Phantom
            | TypedDecoder::LiftedOption(_, None) => {}
            TypedDecoder::Parallel(_, ds)
            | TypedDecoder::Branch(_, _, ds)
            | TypedDecoder::Tuple(_, ds)
            | TypedDecoder::Sequence(_, ds) => {
                for d in ds {
                    d.for_each_call_mut(f);
                }
            }
            TypedDecoder::Match(_, _, branches) => {
                for (_, d) in branches {
                    d.for_each_call_mut(f);
                }
            }
            TypedDecoder::LetFormat(_, d0, _, d1) | TypedDecoder::MonadSeq(_, d0, d1) => {
                d0.for_each_call_mut(f);
                d1.for_each_call_mut(f);
            }
            TypedDecoder::Variant(_, _, d)
            | TypedDecoder::Repeat0While(_, _, d)
            | TypedDecoder::Repeat1Until(_, _, d)
            | TypedDecoder::RepeatCount(_, _, d)
            | TypedDecoder::RepeatBetween(_, _, _, d)
            | TypedDecoder::RepeatUntilLast(_, _, d)
            | TypedDecoder::RepeatUntilSeq(_, _, d)
            | TypedDecoder::Peek(_, d)
            | TypedDecoder::PeekNot(_, d)
            | TypedDecoder::Slice(_, _, d)
            | TypedDecoder::Bits(_, d)
            | TypedDecoder::WithRelativeOffset(_, _, _, d)
            | TypedDecoder::Map(_, d, _)
            | TypedDecoder::Where(_, d, _)
            | TypedDecoder::Let(_, _, _, d)
            | TypedDecoder::Dynamic(_, _, _, d)
            | TypedDecoder::Maybe(_, _, d)
            | TypedDecoder::ForEach(_, _, _, d)
            | TypedDecoder::DecodeBytes(_, _, d)
            | TypedDecoder::AccumUntil(_, _, _, _, d)
            | TypedDecoder::Hint(_, _, d)
            | TypedDecoder::LiftedOption(_, Some(d))
            | TypedDecoder::LetView(_, _, d)
            | TypedDecoder::ParseFromView(_, _, d)
            | TypedDecoder::Permit(_, d, _) => d.for_each_call_mut(f),
            #[cfg(feature = "format_enforce")]
            TypedDecoder::Enforce(_, d) => d.for_each_call_mut(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
            TypedDecoder::Align(_)
            | TypedDecoder::SkipRemainder
            | TypedDecoder::EndOfInput
            | TypedDecoder::Phantom => {
                Some(Cow::Owned(GenType::Inline(RustType::from(PrimType::Unit))))
            }
            TypedDecoder::Byte(set) => {
                (!set.is_empty()).then_some(Cow::Owned(GenType::from(PrimType::U8)))
            }
            TypedDecoder::Pos(nt) => match nt {
                NumType::U(m_uint) => Some(Cow::Owned(GenType::Inline(RustType::from(*m_uint)))),
                NumType::I(m_sint) => Some(Cow::Owned(GenType::Inline(RustType::from(*m_sint)))),
            },
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
            | TypedDecoder::Permit(t, ..)
            | TypedDecoder::LiftedOption(t, ..)
            | TypedDecoder::AccumUntil(t, ..) => Some(Cow::Borrowed(t)),
            #[cfg(feature = "format_enforce")]
            TypedDecoder::Enforce(t, ..) => Some(Cow::Borrowed(t)),
        }
    }
}

/// Decoders with a fixed amount of lookahead
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TypedDecoder<TypeRep> {
    Call(
        TypeRep,
        usize,
        (
            Vec<(Label, TypedExpr<TypeRep>)>,
            Vec<(Label, TypedViewExpr<TypeRep>)>,
        ),
    ),
    Fail,
    EndOfInput,
    Align(NonZeroUsize),
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
        RepeatRange,
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
        TypedCondition<TypeRep>,
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
    Pos(NumType),
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
        TypedFixedReadKind<TypeRep>,
    ),
    ReifyView(TypeRep, TypedViewExpr<TypeRep>),
    Phantom,
    #[cfg(feature = "format_enforce")]
    Enforce(TypeRep, Box<TypedDecoderExt<TypeRep>>),
    Permit(
        TypeRep,
        Box<TypedDecoderExt<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
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
    /// As `decoder::Compiler`'s `decoder_map`: the decoder compiled for each call to a level,
    /// keyed on the level and the (possibly truncated, possibly `Next::Empty`) continuation the
    /// call is compiled against.
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<QueueItem<'a>>,
    next_dependence: NextDependence,
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
            next_dependence: NextDependence::default(),
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

        merge_identical_decoders(&mut compiler.program.decoders);
        Ok(compiler.program)
    }

    /// Given a `TypedFormat` that is either an overall root or the local root of
    /// a disconnected tree within a Format-forest, performs compilation over the
    /// format in question as well as all of its directly reachable descendants.
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

    fn __repeat_next(format: &'a GTFormat, next: Rc<Next<'a>>) -> Rc<Next<'a>> {
        Rc::new(Next::Repeat(MaybeTyped::Typed(format), next))
    }

    fn compile_gt_format(
        &mut self,
        format: &'a GTFormat,
        args: Option<Vec<(Label, GenType)>>,
        next: Rc<Next<'a>>,
    ) -> AResult<GTDecoderExt> {
        let dec: AResult<GTDecoder> = match format {
            TypedFormat::FormatCall(gt, level, arg_exprs, arg_views, deref) => {
                let this_args = (arg_exprs.to_vec(), arg_views.to_vec());
                let sig_args = if arg_exprs.is_empty() && arg_views.is_empty() {
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
                            .chain(arg_views.into_iter().map(|(lab, _)| {
                                (
                                    lab.clone(),
                                    GenType::Inline(RustType::ViewObject(RustLt::Parametric(
                                        Label::Borrowed(super::model::DEFAULT_LT),
                                    ))),
                                )
                            }))
                            .collect(),
                    )
                };
                let next = if self
                    .next_dependence
                    .level_depends_on_next(self.module, *level)
                {
                    Next::truncated(&next, self.module, MATCH_TREE_MAX_DEPTH)
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = self.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let real = TypedFormat::leak_level_cell(deref);
                    let n = self.queue_compile(gt.clone(), real, sig_args, next.clone());
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };

                Ok(TypedDecoder::Call(gt.clone(), n, this_args))
            }
            TypedFormat::Phantom(_, inner) => {
                self.compile_gt_format(inner, None, Rc::new(Next::Empty))?;
                Ok(TypedDecoder::Phantom)
            }
            TypedFormat::DecodeBytes(gt, expr, f) => {
                let da = Box::new(self.compile_gt_format(f, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::DecodeBytes(gt.clone(), expr.clone(), da))
            }
            TypedFormat::ParseFromView(gt, view, f) => {
                let da = Box::new(self.compile_gt_format(f, None, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::ParseFromView(gt.clone(), view.clone(), da))
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
                let tree = MatchTree::build_for(self.module, &fs, next, || {
                    serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()
                })?;
                Ok(TypedDecoder::Branch(gt.clone(), tree, ds))
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
                    let field_next =
                        Next::sequence(MaybeTyped::Typed(fields.as_slice()), next.clone());
                    let df = self.compile_gt_format(f, None, field_next)?;
                    dfields.push(df);
                }
                Ok(TypedDecoder::Tuple(gt.clone(), dfields))
            }
            TypedFormat::Sequence(gt, fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let field_next =
                        Next::sequence(MaybeTyped::Typed(fields.as_slice()), next.clone());
                    let df = self.compile_gt_format(f, None, field_next)?;
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
                let tree =
                    MatchTree::build_for(self.module, &[fa.into(), fb.into()], next, || {
                        serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()
                    })?;
                Ok(TypedDecoder::Repeat0While(gt.clone(), tree, Box::new(da)))
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
                let tree =
                    MatchTree::build_for(self.module, &[fa.into(), fb.into()], next, || {
                        serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()
                    })?;
                Ok(TypedDecoder::Repeat1Until(gt.clone(), tree, Box::new(da)))
            }
            TypedFormat::RepeatCount(gt, expr, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatCount(gt.clone(), expr.clone(), da))
            }
            TypedFormat::RepeatBetween(gt, range, a) => {
                let (min, max) = (range.min(), range.max());

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
                    MatchTree::build_for(self.module, &branches[..], next, || {
                        serde_json::to_string_pretty(&Format::from(format.clone())).unwrap()
                    })?
                };
                Ok(TypedDecoder::RepeatBetween(
                    gt.clone(),
                    reps_left_tree,
                    *range,
                    Box::new(da),
                ))
            }
            TypedFormat::RepeatUntilLast(gt, expr, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatUntilLast(gt.clone(), expr.clone(), da))
            }
            TypedFormat::RepeatUntilSeq(gt, expr, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::RepeatUntilSeq(gt.clone(), expr.clone(), da))
            }
            TypedFormat::AccumUntil(gt, cond, update, init, _vt, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_gt_format(a, None, next)?);
                Ok(TypedDecoder::AccumUntil(
                    gt.clone(),
                    cond.clone(),
                    update.clone(),
                    init.clone(),
                    da,
                ))
            }
            TypedFormat::ForEach(gt, expr, lbl, f) => {
                let next = Self::__repeat_next(f, next);
                let da = Box::new(self.compile_gt_format(f, None, next)?);
                Ok(TypedDecoder::ForEach(
                    gt.clone(),
                    expr.clone(),
                    lbl.clone(),
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
            TypedFormat::Where(gt, a, cond) => {
                let da = Box::new(self.compile_gt_format(a, None, next.clone())?);
                Ok(TypedDecoder::Where(gt.clone(), da, cond.clone()))
            }
            TypedFormat::Compute(gt, expr) => Ok(TypedDecoder::Compute(gt.clone(), expr.clone())),
            TypedFormat::Pos(gt) => {
                let nt = match gt.try_to_num_type() {
                    Some(num_t) => match num_t {
                        NumType::U(_) => num_t,
                        NumType::I(m_sint) => {
                            log::debug!("signed-integer type inferred for Format::Pos: {m_sint}");
                            num_t
                        }
                    },
                    None => {
                        unreachable!("non-numeric type ascription for Format::Pos: {gt:?}")
                    }
                };
                Ok(TypedDecoder::Pos(nt))
            }
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
                // See the identical guard in `decoder::Compiler::compile_format`'s `LetFormat`
                // arm (Phase 4's "Finding B"): skip the `Next::Cat` wrapping when `f` can only
                // ever match zero bytes, so a self-referential `f0` sees the same `next` here as
                // it would compiled directly, letting `decoder_map` actually memoize it.
                let a_next = Next::cat(MaybeTyped::Typed(f.as_ref()), next.clone());
                let d0 = Box::new(self.compile_gt_format(f0, None, a_next)?);
                let d = Box::new(self.compile_gt_format(f, None, next)?);
                Ok(TypedDecoder::LetFormat(gt.clone(), d0, name.clone(), d))
            }
            TypedFormat::MonadSeq(gt, f0, f) => {
                let a_next = Next::cat(MaybeTyped::Typed(f.as_ref()), next.clone());
                let d0 = Box::new(self.compile_gt_format(f0, None, a_next)?);
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
                    kind.clone(),
                )),
                TypedViewFormat::ReifyView => Ok(TypedDecoder::ReifyView(gt.clone(), view.clone())),
            },
            #[cfg(feature = "format_enforce")]
            TypedFormat::Enforce(gt, inner) => {
                let da = Box::new(self.compile_gt_format(inner, None, next)?);
                Ok(TypedDecoder::Enforce(gt.clone(), da))
            }
            TypedFormat::Permit(gt, inner, expr) => {
                let da = Box::new(self.compile_gt_format(inner, None, next)?);
                Ok(TypedDecoder::Permit(gt.clone(), da, expr.clone()))
            }
        };
        let dec = dec?;
        Ok(TypedDecoderExt::new(dec, args))
    }
}
