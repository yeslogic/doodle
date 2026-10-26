use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::rc::Rc;

use anyhow::{Result as AResult, anyhow};

use crate::byte_set::ByteSet;
use crate::error::{DecodeError, DecodeErrorKind, EvalResultExt as _};
use crate::fixed::{SpineElem, analyze_fixed_shape};
use crate::read::{BufferKind, ReadCtxt};
use crate::util::{EResult, WithErr};
use crate::util::{ErrTrace as _, try_downgrade_error_with};
use crate::validation::Condition;
use crate::{
    BaseKind, DynFormat, Endian, Expr, Format, FormatModule, Label, MATCH_TREE_MAX_DEPTH,
    MatchTree, MaybeTyped, Next, NextDependence, Pattern, RepeatRange, TypeHint, TypeScope,
    ValueType, ViewExpr, ViewFormat,
};
use crate::{FixedReadKind, try_with};

pub(crate) mod eval;
#[cfg(test)]
mod eval_parity_tests;
pub mod seq_kind;
pub use seq_kind::{SeqBoundsError, SeqBoundsOp, SeqKind, ValueSeq};

/// Helper macro for discarding identifiers but keeping their repetition-group
macro_rules! wildcard {
    ( $x:ident ) => {
        _
    };
}
pub(crate) use wildcard;

/// Helper macro for breaking out of a loop if a "done" flag is set in a `WithErr` value.
///
/// Syntax:
///
/// Takes the `WithErr` value as the first argument,
/// with a comma-separated and parenthesized list of identifiers (up to but **excluding** the "done" flag),
/// separated by `=>`.
///
/// ```ignore
/// break_if_done!(expr => (ident1, ident2, ..., identN))
/// ```
macro_rules! break_if_done {
    ( $expr:expr => ( $($x:ident),+ ) ) => {
        let ($($crate::decoder::wildcard!($x)),* , done) = $expr.as_ref();
        if *done {
            break;
        }
    };
}
pub(crate) use break_if_done;

pub(crate) fn extract_pair<T>(mut vec: Vec<T>) -> (T, T) {
    assert_eq!(vec.len(), 2, "expected pair");
    unsafe {
        // Safe because we checked the length above
        let second = vec.pop().unwrap_unchecked();
        let first = vec.pop().unwrap_unchecked();
        (first, second)
    }
}

/// Implements `Expr::SubSeqInflate`: yields `length` items starting at `start`, where an index that runs past
/// the end of `values` refers back into the output built so far (i.e. the sequence repeats, LZ77-style).
///
/// Shared by the `Value` and `ParsedValue` evaluators.
///
/// `length` is typically derived from parsed data (unlike `start`, which is bounds-checked against
/// the source sequence up front, `length` has no natural upper bound - the output buffer is grown
/// to fit it). The output `Vec` is allocated via `try_reserve_exact` rather than the infallible
/// allocation APIs, so a `length` large enough to exhaust available memory surfaces as
/// `EvalError::Alloc` instead of aborting the process.
pub(crate) fn sub_seq_inflate<V: Clone + From<Value>>(
    values: ValueSeq<'_, V>,
    start: usize,
    length: usize,
) -> Result<Vec<V>, EvalError> {
    let len = values.len();
    if start >= len {
        if length == 0 {
            // `start` is never dereferenced when `length == 0` (the loops below don't run), so
            // this isn't a hard error - but it's still a suspicious `Expr` to have produced, so
            // flag it in debug builds without changing behavior.
            #[cfg(debug_assertions)]
            log::error!(
                "SubSeqInflate: start={start} is out of bounds for source length {len}, \
                 but length=0 means it will not be dereferenced; permitting, but this is \
                 likely a bug in the calling Expr"
            );
        } else {
            return Err(SeqBoundsError {
                op: SeqBoundsOp::SubSeqInflate,
                index: start,
                len,
            }
            .into());
        }
    }
    let mut vs = Vec::new();
    vs.try_reserve_exact(length)?;
    match values {
        ValueSeq::ValueSeq(vs0) => {
            for i in 0..length {
                if i + start < vs0.len() {
                    vs.push(vs0[i + start].clone());
                } else {
                    vs.push(vs[i + start - vs0.len()].clone());
                }
            }
        }
        ValueSeq::IntRange(range, tag) => {
            // REVIEW - double-check this logic
            let len = range.len();
            let mut iter = range.skip(start);
            for i in 0..length {
                if let Some(val) = iter.next() {
                    vs.push(tag.mk(val).into());
                } else {
                    vs.push(vs[i + start - len].clone());
                }
            }
        }
    }
    Ok(vs)
}

pub mod value;
pub use crate::error::EvalError;
pub use value::{ArithError, ArithOp, IntTag, Value};

pub type DecodeResult<T> = Result<T, DecodeError>;
pub type EDecodeResult<T> = EResult<T, DecodeError>;

impl Expr {
    pub fn eval<'a>(&'a self, scope: &'a Scope<'a>) -> Result<Cow<'a, Value>, EvalError> {
        self.eval_generic(scope)
    }

    pub fn eval_value<'a>(&self, scope: &'a Scope<'a>) -> Result<Value, EvalError> {
        self.eval_value_generic(scope)
    }

    fn eval_lambda<'a>(&self, scope: &'a Scope<'a>, arg: &'a Value) -> Result<Value, EvalError> {
        self.eval_lambda_generic(scope, arg)
    }
}

pub(crate) mod search;

/// Compiled (interpreter-ready) form of [`FixedReadKind`], used by [`Decoder::ReadArray`].
///
/// Unlike `FixedReadKind::FixedFormat`, which only carries a `FormatRef`, this carries the
/// flattened field-layout precomputed from it (see `record_fmt::analyze_fixed_shape`) -- a
/// compiled `Decoder` tree is fully self-contained and is never given `&FormatModule` access
/// again once built, so the layout has to be resolved once, at compile-time
/// (`Compiler::compile_format`), rather than looked up per-parse.
#[derive(Clone, Debug, PartialEq)]
pub enum ReadArrayKind {
    Base(BaseKind<Endian>),
    /// A single `SpineElem` (see `crate::fixed::FixedShape::Single`), read directly as the
    /// element value with no record-wrapping.
    Single {
        elem: SpineDecoder,
        stride: usize,
    },
    /// A struct-shaped element made up purely of primitive fields. Persisted (named) fields
    /// become entries of the resulting `Value::Record`; anonymous/ephemeral fields (`None`)
    /// are still read -- and so still contribute to `stride` -- but are not persisted.
    ///
    /// NOTE - switching struct `ReadUnchecked`/codegen derivation from on-demand to blanket
    /// (see codegen) requires no change here: this variant already carries the complete field
    /// layout regardless of which structs end up with a generated Rust-side impl.
    FixedFormat {
        fields: Vec<(Option<Label>, SpineDecoder)>,
        stride: usize,
    },
}

/// Compiled (interpreter-ready) form of `crate::fixed::SpineElem`: either a direct base-kind
/// read, or a self-contained sub-`Decoder` compiled from the target of a `SpineElem::Indirect`'s
/// `FormatRef` (which is restricted to a single `CommonOp` read, possibly `Variant`-wrapped, so
/// the compiled `Decoder` never contains a `Decoder::Call` of its own and can be parsed standalone).
#[derive(Debug, Clone, PartialEq)]
pub enum SpineDecoder {
    Raw(BaseKind<Endian>),
    Indirect { dec: Rc<Decoder> },
}

impl ReadArrayKind {
    pub(crate) fn stride(&self) -> usize {
        match self {
            ReadArrayKind::Base(kind) => kind.size(),
            ReadArrayKind::Single { stride, .. } => *stride,
            ReadArrayKind::FixedFormat { stride, .. } => *stride,
        }
    }
}

/// Decoders with a fixed amount of lookahead
#[derive(Clone, Debug, PartialEq)]
pub enum Decoder {
    Call(usize, Vec<(Label, Expr)>, Vec<(Label, ViewExpr)>),
    Pos,
    Fail,
    EndOfInput,
    Align(NonZeroUsize),
    Byte(ByteSet),
    Variant(Label, Box<Decoder>),
    Parallel(Vec<Decoder>),
    Branch(MatchTree, Vec<Decoder>),
    Tuple(Vec<Decoder>),
    Sequence(Vec<Decoder>),
    While(MatchTree, Box<Decoder>),
    Until(MatchTree, Box<Decoder>),
    RepeatCount(Box<Expr>, Box<Decoder>),
    RepeatUntilLast(Box<Expr>, Box<Decoder>),
    RepeatUntilSeq(Box<Expr>, Box<Decoder>),
    Maybe(Box<Expr>, Box<Decoder>),
    Peek(Box<Decoder>),
    PeekNot(Box<Decoder>),
    Slice(Box<Expr>, Box<Decoder>),
    Bits(Box<Decoder>),
    WithRelativeOffset(Box<Expr>, Box<Expr>, Box<Decoder>),
    Map(Box<Decoder>, Box<Expr>),
    Where(Box<Decoder>, Condition),
    Compute(Box<Expr>),
    Let(Label, Box<Expr>, Box<Decoder>),
    Match(Box<Expr>, Vec<(Pattern, Decoder)>),
    Dynamic(Label, DynFormat, Box<Decoder>),
    Apply(Label),
    /// RepeatBetween: the MatchTree is an N-ary decision-tree where the matching index corresponds to the number of unparsed repetitions left in the limited LL(k) window.
    RepeatBetween(MatchTree, RepeatRange, Box<Decoder>),
    ForEach(Box<Expr>, Label, Box<Decoder>),
    SkipRemainder,
    DecodeBytes(Box<Expr>, Box<Decoder>),
    ParseFromView(ViewExpr, Box<Decoder>),
    LetFormat(Box<Decoder>, Label, Box<Decoder>),
    MonadSeq(Box<Decoder>, Box<Decoder>),
    AccumUntil(Box<Expr>, Box<Expr>, Box<Expr>, TypeHint, Box<Decoder>),
    LiftedOption(Option<Box<Decoder>>),
    LetView(Label, Box<Decoder>),
    CaptureBytes(ViewExpr, Box<Expr>),
    ReadArray(ViewExpr, Box<Expr>, ReadArrayKind),
    ReifyView(ViewExpr),
    Phantom,
    #[cfg(feature = "format_enforce")]
    Enforce(Box<Decoder>),
    Permit(Box<Decoder>, Box<Expr>),
    /// The decoder of a level in a recursive batch (named by the `Label`), which counts towards
    /// the program's recursion limit while it runs (see [`Program::set_recursion_limit`]).
    Recursive(Label, Box<Decoder>),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub decoders: Vec<(Decoder, ValueType)>,
    /// How deeply decoders of recursive levels may nest; see [`Self::set_recursion_limit`].
    recursion_limit: usize,
}

std::thread_local! {
    /// How many `Decoder::Recursive` decoders are running on this thread.
    static RECURSION_DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// Counts one running `Decoder::Recursive` decoder in [`RECURSION_DEPTH`] for as long as it lives,
/// so the count is restored however the decoder returns.
pub(crate) struct RecursionGuard;

impl RecursionGuard {
    /// Counts one more running recursive decoder, or returns `None` if `limit` are already running.
    pub(crate) fn enter(limit: usize) -> Option<Self> {
        RECURSION_DEPTH.with(|depth| {
            let current = depth.get();
            (current < limit).then(|| {
                depth.set(current + 1);
                RecursionGuard
            })
        })
    }
}

/// How many `Decoder::Recursive` decoders are running on this thread.
pub(crate) fn recursion_depth() -> usize {
    RECURSION_DEPTH.with(Cell::get)
}

impl Drop for RecursionGuard {
    fn drop(&mut self) {
        RECURSION_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

/// A compiled decoder that may call other decoders of the same program by index.
pub(crate) trait CallIndices {
    /// Applies `f` to the index of every call this decoder makes.
    fn for_each_call_mut(&mut self, f: &mut dyn FnMut(&mut usize));
}

/// Merges decoders of a compiled program that are identical, along with their types, into the
/// first of them, and renumbers every call accordingly; repeats until no two remain identical,
/// since merging the decoders two others call can make those two identical as well. Index 0,
/// the program's entry point, keeps its index.
///
/// The compilers produce such duplicates for a level whose decoder can depend on the
/// continuation it is compiled against (see `NextDependence`) but happens not to for some of
/// the continuations it is called with.
pub(crate) fn merge_identical_decoders<D, T>(decoders: &mut Vec<(D, T)>)
where
    D: CallIndices + PartialEq,
    T: PartialEq,
{
    loop {
        let n = decoders.len();
        let mut canonical: Vec<usize> = (0..n).collect();
        let mut merged = false;
        for i in 1..n {
            if let Some(j) = (0..i).find(|&j| canonical[j] == j && decoders[j] == decoders[i]) {
                canonical[i] = j;
                merged = true;
            }
        }
        if !merged {
            return;
        }
        let mut new_index = vec![usize::MAX; n];
        let mut kept = 0;
        for i in 0..n {
            if canonical[i] == i {
                new_index[i] = kept;
                kept += 1;
            }
        }
        let remap: Vec<usize> = canonical.iter().map(|&c| new_index[c]).collect();
        let old = std::mem::take(decoders);
        for (i, (mut dec, t)) in old.into_iter().enumerate() {
            if canonical[i] == i {
                dec.for_each_call_mut(&mut |ix| *ix = remap[*ix]);
                decoders.push((dec, t));
            }
        }
    }
}

impl CallIndices for Decoder {
    fn for_each_call_mut(&mut self, f: &mut dyn FnMut(&mut usize)) {
        match self {
            Decoder::Call(ix, _, _) => f(ix),
            Decoder::Pos
            | Decoder::Fail
            | Decoder::EndOfInput
            | Decoder::Align(_)
            | Decoder::Byte(_)
            | Decoder::Compute(_)
            | Decoder::Apply(_)
            | Decoder::SkipRemainder
            | Decoder::CaptureBytes(..)
            // `SpineDecoder`s never contain a `Call`
            | Decoder::ReadArray(..)
            | Decoder::ReifyView(_)
            | Decoder::Phantom
            | Decoder::LiftedOption(None) => {}
            Decoder::Parallel(ds)
            | Decoder::Branch(_, ds)
            | Decoder::Tuple(ds)
            | Decoder::Sequence(ds) => {
                for d in ds {
                    d.for_each_call_mut(f);
                }
            }
            Decoder::Match(_, branches) => {
                for (_, d) in branches {
                    d.for_each_call_mut(f);
                }
            }
            Decoder::LetFormat(d0, _, d1) | Decoder::MonadSeq(d0, d1) => {
                d0.for_each_call_mut(f);
                d1.for_each_call_mut(f);
            }
            Decoder::Variant(_, d)
            | Decoder::While(_, d)
            | Decoder::Until(_, d)
            | Decoder::RepeatCount(_, d)
            | Decoder::RepeatUntilLast(_, d)
            | Decoder::RepeatUntilSeq(_, d)
            | Decoder::Maybe(_, d)
            | Decoder::Peek(d)
            | Decoder::PeekNot(d)
            | Decoder::Slice(_, d)
            | Decoder::Bits(d)
            | Decoder::WithRelativeOffset(_, _, d)
            | Decoder::Map(d, _)
            | Decoder::Where(d, _)
            | Decoder::Let(_, _, d)
            | Decoder::Dynamic(_, _, d)
            | Decoder::RepeatBetween(_, _, d)
            | Decoder::ForEach(_, _, d)
            | Decoder::DecodeBytes(_, d)
            | Decoder::ParseFromView(_, d)
            | Decoder::AccumUntil(_, _, _, _, d)
            | Decoder::LiftedOption(Some(d))
            | Decoder::LetView(_, d)
            | Decoder::Permit(d, _)
            | Decoder::Recursive(_, d) => d.for_each_call_mut(f),
            #[cfg(feature = "format_enforce")]
            Decoder::Enforce(d) => d.for_each_call_mut(f),
        }
    }
}

impl Program {
    /// The default for [`Self::set_recursion_limit`].
    pub const DEFAULT_RECURSION_LIMIT: usize = 256;

    pub(crate) fn new() -> Self {
        let decoders = Vec::new();
        Program {
            decoders,
            recursion_limit: Self::DEFAULT_RECURSION_LIMIT,
        }
    }

    /// Sets how many decoders of recursive levels (levels registered by
    /// [`FormatModule::define_format_rec_batch`] or [`FormatModule::define_format_recursive`]) may
    /// be running at once, nested within each other; a call that would exceed it fails with
    /// [`DecodeErrorKind::RecursionLimit`]. Defaults to [`Self::DEFAULT_RECURSION_LIMIT`].
    ///
    /// The limit counts nested calls, not stack space, so it prevents a stack overflow only if
    /// the stack can hold that many: how much each call takes depends on the format, the build
    /// profile and what the thread's stack size is. Nested BSON documents, for instance, count two
    /// calls per level of nesting. The count is per thread, and is shared by any programs running
    /// nested within each other on that thread.
    pub fn set_recursion_limit(&mut self, limit: usize) {
        self.recursion_limit = limit;
    }

    /// See [`Self::set_recursion_limit`].
    pub fn recursion_limit(&self) -> usize {
        self.recursion_limit
    }

    pub fn run<'input>(&self, input: ReadCtxt<'input>) -> DecodeResult<(Value, ReadCtxt<'input>)> {
        Ok(self.decoders[0]
            .0
            .parse(self, &Scope::Empty, input)?
            .extract_warn())
    }
}

pub struct Compiler<'a> {
    module: &'a FormatModule,
    program: Program,
    /// The decoder compiled for each call to a level, keyed on the level and the continuation the
    /// call is compiled against: that continuation, truncated to `MATCH_TREE_MAX_DEPTH` bytes, if
    /// the level's decoder can depend on it (see `NextDependence`), and `Next::Empty` otherwise.
    /// Continuations are compared structurally, so a dependent level called with continuations
    /// that differ, but not in anything its decisions consult, gets identical decoders; those are
    /// merged once compilation finishes (see [`merge_identical_decoders`]).
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    /// Formats awaiting compilation, with the continuation to compile each against, the index of
    /// its decoder, and the name of its level if that level is recursive (see
    /// [`FormatModule::is_recursive_level`]), in which case its decoder is a `Decoder::Recursive`.
    compile_queue: Vec<(&'a Format, Rc<Next<'a>>, usize, Option<Label>)>,
    next_dependence: NextDependence,
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
            next_dependence: NextDependence::default(),
        }
    }

    /// Compiles a top-level entry-point `Format` into a `Program` using the provided `FormatModule`.
    pub fn compile_program(module: &FormatModule, format: &Format) -> AResult<Program> {
        let mut compiler = Compiler::new(module);
        // type
        let scope = TypeScope::with_rec_types(&module.format_types);
        let t = module.infer_format_type(&scope, format)?;
        // decoder
        compiler.queue_compile(t, format, Rc::new(Next::Empty), None);
        while let Some((f, next, n, recursive)) = compiler.compile_queue.pop() {
            let d = compiler.compile_format(f, next)?;
            compiler.program.decoders[n].0 = match recursive {
                Some(level) => Decoder::Recursive(level, Box::new(d)),
                None => d,
            };
        }
        merge_identical_decoders(&mut compiler.program.decoders);
        Ok(compiler.program)
    }

    fn queue_compile(
        &mut self,
        t: ValueType,
        f: &'a Format,
        next: Rc<Next<'a>>,
        recursive: Option<Label>,
    ) -> usize {
        let n = self.program.decoders.len();
        self.program.decoders.push((Decoder::Fail, t));
        self.compile_queue.push((f, next, n, recursive));
        n
    }

    /// Single-entrypoint for compiling a standalone `Format` that does not call into any `FormatRef`s.
    ///
    /// Constructs an empty `FormatModule` to use for registration.
    pub fn compile_one(format: &Format) -> AResult<Decoder> {
        let module = FormatModule::new();
        let mut compiler = Compiler::new(&module);
        compiler.compile_format(format, Rc::new(Next::Empty))
    }

    /// Compiles a [`crate::fixed::SpineElem`] into its interpreter-ready [`SpineDecoder`]
    /// counterpart. `Raw` needs no compilation; `Indirect` compiles the (validated, single-hop)
    /// target of the `FormatRef` into a self-contained `Decoder`, which never emits its own
    /// `Decoder::Call` since `SpineElem::Indirect` only ever refers to a target that performs
    /// exactly one `CommonOp` read (optionally `Variant`-wrapped) with no further indirection.
    fn compile_spine_elem(&mut self, elem: &SpineElem) -> AResult<SpineDecoder> {
        match elem {
            SpineElem::Raw(kind) => Ok(SpineDecoder::Raw(*kind)),
            SpineElem::Indirect(format_ref, _kind) => {
                let target = self.module.get_format(format_ref.get_level());
                let dec = self.compile_format(target, Rc::new(Next::Empty))?;
                Ok(SpineDecoder::Indirect { dec: Rc::new(dec) })
            }
        }
    }

    /// Extracted helper to construct the inner `next` value to be used for
    /// compiling the inner format of `RepeatCount` and similar repeat-style
    /// formats that terminate on a non-MatchTree condition.
    fn __repeat_next(format: &'a Format, next: Rc<Next<'a>>) -> Rc<Next<'a>> {
        // NOTE - stop condition is determined by value at runtime, but structurally the matchtree looks identical to a standard Repeat
        Rc::new(Next::Repeat(MaybeTyped::Untyped(format), next))
    }

    fn compile_format(&mut self, format: &'a Format, next: Rc<Next<'a>>) -> AResult<Decoder> {
        match format {
            Format::ItemVar(level, arg_exprs, arg_views) => {
                let f = self.module.get_format(*level);
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
                    let t = self.module.get_format_type(*level).clone();
                    let recursive = self
                        .module
                        .is_recursive_level(*level)
                        .then(|| Label::from(self.module.get_name(*level).to_string()));
                    let n = self.queue_compile(t, f, next.clone(), recursive);
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                let arg_names = self.module.get_args(*level);
                let mut args = Vec::with_capacity(arg_names.len());
                for ((name, _type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    args.push((name.clone(), expr.clone()));
                }
                let view_names = self.module.get_view_args(*level);
                let mut views = Vec::with_capacity(view_names.len());
                for (name, view) in Iterator::zip(view_names.iter(), arg_views.iter()) {
                    views.push((name.clone(), view.clone()));
                }
                Ok(Decoder::Call(n, args, views))
            }
            Format::RecVar(_) => unreachable!(
                "Format::RecVar is rewritten to ItemVar at batch registration; never appears in a stored Format"
            ),
            Format::Phantom(_inner) => Ok(Decoder::Phantom),
            Format::Fail => Ok(Decoder::Fail),
            Format::DecodeBytes(expr, inner) => {
                let d = self.compile_format(inner, Rc::new(Next::Empty))?;
                Ok(Decoder::DecodeBytes(expr.clone(), Box::new(d)))
            }
            Format::ParseFromView(v_expr, inner) => {
                let d = self.compile_format(inner, Rc::new(Next::Empty))?;
                Ok(Decoder::ParseFromView(v_expr.clone(), Box::new(d)))
            }
            Format::Pos => Ok(Decoder::Pos),
            Format::LetView(ident, a) => {
                let d = self.compile_format(a, next.clone())?;
                Ok(Decoder::LetView(ident.clone(), Box::new(d)))
            }
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Align(n) => Ok(Decoder::Align(*n)),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::SkipRemainder => Ok(Decoder::SkipRemainder),
            Format::Variant(label, f) => {
                let d = self.compile_format(f, next.clone())?;
                Ok(Decoder::Variant(label.clone(), Box::new(d)))
            }
            Format::Union(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_format(f, next.clone())?);
                }
                let tree =
                    MatchTree::build_for(self.module, branches, next, || format!("{format:?}"))?;
                Ok(Decoder::Branch(tree, ds))
            }
            Format::UnionNondet(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    let d = self.compile_format(f, next.clone())?;
                    ds.push(d);
                }
                Ok(Decoder::Parallel(ds))
            }
            Format::Tuple(elems) => {
                let mut decs = Vec::with_capacity(elems.len());
                let mut fields = elems.iter();
                while let Some(f) = fields.next() {
                    let field_next =
                        Next::sequence(MaybeTyped::Untyped(fields.as_slice()), next.clone());
                    let df = self.compile_format(f, field_next)?;
                    decs.push(df);
                }
                Ok(Decoder::Tuple(decs))
            }
            Format::Sequence(formats) => {
                let mut decs = Vec::with_capacity(formats.len());
                let mut fields = formats.iter();
                while let Some(f) = fields.next() {
                    let field_next =
                        Next::sequence(MaybeTyped::Untyped(fields.as_slice()), next.clone());
                    let df = self.compile_format(f, field_next)?;
                    decs.push(df);
                }
                Ok(Decoder::Sequence(decs))
            }
            Format::Repeat(a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_format(
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                )?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                let tree =
                    MatchTree::build_for(self.module, &[fa, fb], next, || format!("{format:?}"))?;
                Ok(Decoder::While(tree, Box::new(da)))
            }
            Format::Repeat1(a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_format(
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                )?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::EMPTY;
                let fb = Format::Tuple(vec![(**a).clone(), astar]);
                let tree =
                    MatchTree::build_for(self.module, &[fa, fb], next, || format!("{format:?}"))?;
                Ok(Decoder::Until(tree, Box::new(da)))
            }
            Format::RepeatCount(expr, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
            }
            Format::RepeatBetween(range, a) => {
                let (min, max) = (range.min(), range.max());
                let da = self.compile_format(
                    a,
                    Rc::new(Next::RepeatBetween(
                        min.saturating_sub(1),
                        max.saturating_sub(1),
                        MaybeTyped::Untyped(a),
                        next.clone(),
                    )),
                )?;

                let tree = {
                    let mut branches: Vec<Format> = Vec::new();
                    // FIXME: this is inefficient but probably works
                    for count in 0..=max {
                        let f_count =
                            Format::RepeatCount(Box::new(Expr::U32(count as u32)), a.clone());
                        branches.push(f_count);
                    }
                    MatchTree::build_for(self.module, &branches[..], next, || {
                        format!("{format:?}")
                    })?
                };
                Ok(Decoder::RepeatBetween(tree, *range, Box::new(da)))
            }
            Format::RepeatUntilLast(expr, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::RepeatUntilLast(expr.clone(), da))
            }
            Format::ForEach(expr, lbl, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::ForEach(expr.clone(), lbl.clone(), da))
            }
            Format::RepeatUntilSeq(expr, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::RepeatUntilSeq(expr.clone(), da))
            }
            Format::AccumUntil(f_done, f_update, init, vt, a) => {
                let next = Self::__repeat_next(a, next);
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::AccumUntil(
                    f_done.clone(),
                    f_update.clone(),
                    init.clone(),
                    vt.clone(),
                    da,
                ))
            }
            Format::Maybe(x, a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Maybe(x.clone(), da))
            }
            Format::Peek(a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Peek(da))
            }
            Format::PeekNot(a) => {
                const MAX_LOOKAHEAD: usize = 1024;
                match a.lookahead_bounds(self.module).max {
                    None => return Err(anyhow!("PeekNot cannot require unbounded lookahead")),
                    Some(n) if n > MAX_LOOKAHEAD => {
                        return Err(anyhow!(
                            "PeekNot cannot require > {MAX_LOOKAHEAD} bytes lookahead"
                        ));
                    }
                    _ => {}
                }
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::PeekNot(da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::Bits(a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Bits(da))
            }
            Format::WithRelativeOffset(addr, expr, a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::WithRelativeOffset(addr.clone(), expr.clone(), da))
            }
            Format::Map(a, expr) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Map(da, expr.clone()))
            }
            Format::Where(a, expr) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Where(da, expr.clone()))
            }
            #[cfg(feature = "format_enforce")]
            Format::Enforce(a) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Enforce(da))
            }
            Format::Permit(a, expr) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Permit(da, expr.clone()))
            }
            Format::Compute(expr) => Ok(Decoder::Compute(expr.clone())),
            Format::Let(name, expr, a) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Let(name.clone(), expr.clone(), da))
            }
            Format::Match(head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| {
                        Ok((pattern.clone(), self.compile_format(f, next.clone())?))
                    })
                    .collect::<AResult<_>>()?;
                Ok(Decoder::Match(head.clone(), branches))
            }
            Format::Dynamic(name, dynformat, a) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Dynamic(name.clone(), dynformat.clone(), da))
            }
            Format::Apply(name) => Ok(Decoder::Apply(name.clone())),
            Format::LetFormat(first, name, second) => {
                // When `second` is a non-productive format (e.g. `Format::Compute`), we compile against `next` directly and avoid cache-misses in `decoder_map` for self-referential `first` formats
                let a_next = Next::cat(MaybeTyped::Untyped(second), next.clone());
                let da = Box::new(self.compile_format(first, a_next)?);
                let db = Box::new(self.compile_format(second, next.clone())?);
                Ok(Decoder::LetFormat(da, name.clone(), db))
            }
            Format::MonadSeq(first, second) => {
                // When `second` is a non-productive format (e.g. `Format::Compute`), we compile against `next` directly and avoid cache-misses in `decoder_map` for self-referential `first` formats
                let a_next = Next::cat(MaybeTyped::Untyped(second), next.clone());
                let da = Box::new(self.compile_format(first, a_next)?);
                let db = Box::new(self.compile_format(second, next.clone())?);
                Ok(Decoder::MonadSeq(da, db))
            }
            Format::Hint(_hint, a) => {
                // REVIEW - do we want to preserve any facet of the hinting within the Decoder?
                self.compile_format(a, next)
            }
            Format::LiftedOption(None) => Ok(Decoder::LiftedOption(None)),
            Format::LiftedOption(Some(a)) => Ok(Decoder::LiftedOption(Some(Box::new(
                self.compile_format(a, next)?,
            )))),
            Format::WithView(v_expr, vf) => match vf {
                ViewFormat::CaptureBytes(len) => {
                    // REVIEW - do we want to fuse WithView and ViewFormat, or keep them separate?
                    Ok(Decoder::CaptureBytes(v_expr.clone(), len.clone()))
                }
                ViewFormat::ReadArray(len, FixedReadKind::Base(k)) => Ok(Decoder::ReadArray(
                    v_expr.clone(),
                    len.clone(),
                    ReadArrayKind::Base(*k),
                )),
                ViewFormat::ReadArray(len, FixedReadKind::FixedFormat(format_ref)) => {
                    let level = format_ref.get_level();
                    // Re-derives the same `FixedShape` already validated by
                    // `typecheck::infer_var_view_format` -- cheap and pure over `&Format`, so
                    // recomputing here (rather than threading it through the AST) keeps this
                    // as the single source of truth for the field layout.
                    let shape = analyze_fixed_shape(self.module, *format_ref).unwrap_or_else(|e| {
                        panic!(
                            "format `{}` is not eligible for FixedFormat ReadArray: {e}",
                            self.module.get_name(level),
                        )
                    });
                    let read_array_kind = match shape {
                        crate::fixed::FixedShape::Single { format, stride } => {
                            let elem = self.compile_spine_elem(&format)?;
                            ReadArrayKind::Single { elem, stride }
                        }
                        crate::fixed::FixedShape::Record { fields, stride } => {
                            let mut accum = Vec::with_capacity(fields.len());
                            for (name, field) in fields.iter() {
                                let dec = self.compile_spine_elem(field)?;
                                accum.push((name.clone(), dec));
                            }
                            ReadArrayKind::FixedFormat {
                                fields: accum,
                                stride,
                            }
                        }
                    };
                    Ok(Decoder::ReadArray(
                        v_expr.clone(),
                        len.clone(),
                        read_array_kind,
                    ))
                }
                ViewFormat::ReifyView => Ok(Decoder::ReifyView(v_expr.clone())),
            },
        }
    }
}

// `Scope`/`MultiScope`/`SingleScope`/`DecoderScope`/`ViewScope`/`ScopeEntry`/`View` are generic
// over the leaf value representation (`Value` here, `ParsedValue` for `LocScope` et al. in
// `loc_decoder.rs`) and defined once in `crate::scope` as `GScope<'a, V>` and friends.
pub use crate::scope::{DecoderScope, MultiScope, Scope, ScopeEntry, SingleScope, View, ViewScope};

/// The signature shared by `Decoder::parse` and the per-variant functions it dispatches to.
type ParseFn = for<'input> fn(
    &Decoder,
    &Program,
    &Scope<'_>,
    ReadCtxt<'input>,
) -> EDecodeResult<(Value, ReadCtxt<'input>)>;

impl Decoder {
    pub fn parse<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        // Selecting the arm's function and calling it once keeps this frame small: every
        // recursive decode passes through it, and a `match` that called each function
        // directly would reserve a separate result slot per arm in unoptimized builds.
        let parse_arm: ParseFn = match self {
            Decoder::Call(..) => Self::parse_call,
            Decoder::Phantom => Self::parse_phantom,
            Decoder::Fail => Self::parse_fail,
            Decoder::Pos => Self::parse_pos,
            Decoder::SkipRemainder => Self::parse_skip_remainder,
            Decoder::EndOfInput => Self::parse_end_of_input,
            Decoder::Align(..) => Self::parse_align,
            Decoder::Byte(..) => Self::parse_byte,
            Decoder::Variant(..) => Self::parse_variant,
            Decoder::Branch(..) => Self::parse_branch,
            Decoder::Parallel(..) => Self::parse_parallel,
            Decoder::Tuple(..) => Self::parse_tuple,
            Decoder::Sequence(..) => Self::parse_sequence,
            Decoder::While(..) => Self::parse_while,
            Decoder::Until(..) => Self::parse_until,
            Decoder::DecodeBytes(..) => Self::parse_decode_bytes,
            Decoder::ParseFromView(..) => Self::parse_from_view,
            Decoder::LetFormat(..) => Self::parse_let_format,
            Decoder::MonadSeq(..) => Self::parse_monad_seq,
            Decoder::ForEach(..) => Self::parse_for_each,
            Decoder::RepeatCount(..) => Self::parse_repeat_count,
            Decoder::RepeatBetween(..) => Self::parse_repeat_between,
            Decoder::Maybe(..) => Self::parse_maybe,
            Decoder::RepeatUntilLast(..) => Self::parse_repeat_until_last,
            Decoder::RepeatUntilSeq(..) => Self::parse_repeat_until_seq,
            Decoder::AccumUntil(..) => Self::parse_accum_until,
            Decoder::Peek(..) => Self::parse_peek,
            Decoder::PeekNot(..) => Self::parse_peek_not,
            Decoder::Slice(..) => Self::parse_slice,
            Decoder::Bits(..) => Self::parse_bits,
            Decoder::WithRelativeOffset(..) => Self::parse_with_relative_offset,
            Decoder::Map(..) => Self::parse_map,
            Decoder::Where(..) => Self::parse_where,
            Decoder::Compute(..) => Self::parse_compute,
            Decoder::Let(..) => Self::parse_let,
            Decoder::LetView(..) => Self::parse_let_view,
            Decoder::Match(..) => Self::parse_match,
            Decoder::Dynamic(..) => Self::parse_dynamic_huffman,
            Decoder::Apply(..) => Self::parse_apply,
            Decoder::LiftedOption(None) => Self::parse_lifted_option_none,
            Decoder::LiftedOption(Some(_)) => Self::parse_lifted_option_some,
            Decoder::CaptureBytes(..) => Self::parse_capture_bytes,
            Decoder::ReadArray(..) => Self::parse_read_array,
            Decoder::ReifyView(..) => Self::parse_reify_view,
            #[cfg(feature = "format_enforce")]
            Decoder::Enforce(..) => Self::parse_enforce,
            Decoder::Permit(..) => Self::parse_permit,
            Decoder::Recursive(..) => Self::parse_recursive,
        };
        parse_arm(self, program, scope, input)
    }

    #[inline(never)]
    fn parse_call<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Call(n, es, vs) = self else {
            unreachable!()
        };
        let mut new_scope = MultiScope::with_capacity(&Scope::Empty, es.len());
        for (name, e) in es {
            let v = e
                .eval_value(scope)
                .trace_eval(|| ("Call(e)", format!("{e:?}")))?;
            new_scope.push_owned(name.clone(), v);
        }
        for (name, v) in vs {
            let vv = Self::eval_view_expr(scope, v)?;
            new_scope.push_view(name.clone(), vv);
        }
        Ok(try_with! (
        program.decoders[*n]
            .0
            .parse(program, &Scope::Multi(&new_scope), input)
        => ("parse@Call", *n)
        ))
    }

    #[inline(never)]
    fn parse_phantom<'input>(
        &self,
        _program: &Program,
        _scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        Ok(WithErr::new((Value::PhantomData, input)))
    }

    #[inline(never)]
    fn parse_fail<'input>(
        &self,
        _program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        Err(DecodeErrorKind::<Value>::fail(scope, input).into())
    }

    #[inline(never)]
    fn parse_pos<'input>(
        &self,
        _program: &Program,
        _scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        Ok(WithErr::new((Value::from_pos(input.offset), input)))
    }

    #[inline(never)]
    fn parse_skip_remainder<'input>(
        &self,
        _program: &Program,
        _scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let input = input.skip_remainder();
        Ok(WithErr::new((Value::UNIT, input)))
    }

    #[inline(never)]
    fn parse_end_of_input<'input>(
        &self,
        _program: &Program,
        _scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        match input.read_byte() {
            None => Ok(WithErr::new((Value::UNIT, input))),
            Some((b, _)) => Err(input.kind.trailing(b, input.offset).into()),
        }
    }

    #[inline(never)]
    fn parse_align<'input>(
        &self,
        _program: &Program,
        _scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Align(n) = self else {
            unreachable!()
        };
        let n = n.get();
        let skip = (n - (input.offset % n)) % n;
        let (_, input) = input
            .split_at(skip)
            .ok_or(input.kind.overrun(skip, input.offset))?;
        Ok(WithErr::new((Value::UNIT, input)))
    }

    #[inline(never)]
    fn parse_byte<'input>(
        &self,
        _program: &Program,
        _scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Byte(bs) = self else {
            unreachable!()
        };
        let (b, input) = input.read_byte().ok_or(input.kind.overbyte(input.offset))?;
        if bs.contains(b) {
            Ok(WithErr::new((Value::U8(b), input)))
        } else {
            Err(DecodeErrorKind::unexpected(b, *bs, input.offset).into())
        }
    }

    #[inline(never)]
    fn parse_variant<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Variant(label, d) = self else {
            unreachable!()
        };
        Ok(d.parse(program, scope, input)?
            .map(move |(v, input)| (Value::Variant(label.clone(), Box::new(v)), input)))
    }

    #[inline(never)]
    fn parse_branch<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Branch(tree, branches) = self else {
            unreachable!()
        };
        let index = tree.matches(input).ok_or(DecodeErrorKind::NoValidBranch {
            offset: input.offset,
        })?;
        let d = &branches[index];
        Ok(d.parse(program, scope, input)?
            .map(|(v, input)| (Value::Branch(index, Box::new(v)), input)))
    }

    #[inline(never)]
    fn parse_parallel<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Parallel(branches) = self else {
            unreachable!()
        };
        // Inside a recursive decoder, hitting the recursion limit rejects the input; outside one
        // it only rules out the branch, e.g. one format among the alternatives a file is tried as.
        let limit_is_fatal = recursion_depth() > 0;
        for (index, d) in branches.iter().enumerate() {
            match d.parse(program, scope, input) {
                Ok(p) => return Ok(p.map(|(v, input)| (Value::Branch(index, Box::new(v)), input))),
                Err(err) if limit_is_fatal && err.is_recursion_limit() => return Err(err),
                Err(_) => {}
            }
        }
        Err(DecodeErrorKind::<Value>::fail(scope, input).with_trace("no valid branch"))
    }

    #[inline(never)]
    fn parse_tuple<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Tuple(fields) = self else {
            unreachable!()
        };
        Ok(WithErr::fold(
            (Vec::with_capacity(fields.len()), input),
            fields.iter().enumerate(),
            |(mut v, input), (ix, f)| {
                Ok(
                    try_with!(f.parse(program, scope, input) => ("Tuple", ix)).map(
                        move |(vf, new_input)| {
                            v.push(vf);
                            (v, new_input)
                        },
                    ),
                )
            },
        )?
        .map(|(v, input)| (Value::Tuple(v), input)))
    }

    #[inline(never)]
    fn parse_sequence<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Sequence(decs) = self else {
            unreachable!()
        };
        Ok(WithErr::fold(
            (Vec::with_capacity(decs.len()), input),
            decs.iter(),
            |(mut v, input), f| {
                Ok(
                    try_with!(f.parse(program, scope, input) => ("Sequence", v.len())).map(
                        move |(vf, new_input)| {
                            v.push(vf);
                            (v, new_input)
                        },
                    ),
                )
            },
        )?
        .map(|(v, input)| (Value::Seq(SeqKind::Strict(v)), input)))
    }

    #[inline(never)]
    fn parse_while<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::While(tree, a) = self else {
            unreachable!()
        };
        let mut input = input;
        let mut res = WithErr::new((Vec::new(), input));
        while try_with!(tree.matches(input).ok_or(DecodeErrorKind::NoValidBranch {
            offset: input.offset,
        }) => ("While(matchtree)", res.as_ref().0.len()))
            == 0
        {
            res = res.join(|(mut v, input)| {
                Ok(
                    try_with!(a.parse(program, scope, input) => ("While(parse)", v.len())).map(
                        |(va, next_input)| {
                            v.push(va);
                            (v, next_input)
                        },
                    ),
                )
            })?;
            input = res.as_ref().1;
        }
        Ok(res.map(|(v, input)| (Value::Seq(v.into()), input)))
    }

    #[inline(never)]
    fn parse_until<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Until(tree, a) = self else {
            unreachable!()
        };
        let mut res = WithErr::new((Vec::new(), input));
        loop {
            res = res.join(|(mut v, input)| {
                Ok(
                    try_with!(a.parse(program, scope, input) => ("Until(parse)", v.len())).map(
                        |(va, next_input)| {
                            v.push(va);
                            (v, next_input)
                        },
                    ),
                )
            })?;
            let input = res.as_ref().1;
            if try_with!(tree.matches(input).ok_or(DecodeErrorKind::NoValidBranch {
                offset: input.offset,
            }) => ("Until(matchtree)", res.as_ref().0.len()))
                == 0
            {
                break;
            }
        }
        Ok(res.map(|(v, input)| (Value::Seq(v.into()), input)))
    }

    #[inline(never)]
    fn parse_decode_bytes<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::DecodeBytes(bytes, a) = self else {
            unreachable!()
        };
        let bytes = {
            let raw = bytes
                .eval_value(scope)
                .trace_eval(|| ("DecodeBytes(bytes)", format!("{bytes:?}")))?;
            let seq_vals = raw.get_sequence().expect("bad type for DecodeBytes input");
            seq_vals
                .into_iter()
                .map(|v| v.get_as_u8())
                .collect::<Vec<u8>>()
        };
        let new_input = ReadCtxt::from_value(&bytes);
        try_with!(a.parse(program, scope, new_input) => ("DecodeBytes", bytes.len())).join(
            |(va, rem_input)| {
                // FIXME: trailing bytes here are reported as a warning embedded in an
                // `Ok(WithErr)`, not a hard `Err`. `Decoder::Permit` only branches on
                // `Err` from its first sub-decoder, so it can never see this warning and
                // therefore can't fall back to its second branch when `DecodeBytes`'s
                // held format under-consumes the sub-buffer. `permit(decode_bytes(...))`
                // used as a strict "fully valid or not" gate is silently unsound as a
                // result. Tracked in a separate issue (see git issue tracker).
                Ok(match rem_input.read_byte() {
                    Some((b, _)) => {
                        let err = rem_input.kind.trailing(b, rem_input.offset).into();
                        WithErr::with_err((va, input), err)
                    }
                    None => WithErr::new((va, input)),
                })
            },
        )
    }

    #[inline(never)]
    fn parse_from_view<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::ParseFromView(v_expr, a) = self else {
            unreachable!()
        };
        let view_window = Self::eval_view_expr(scope, v_expr)?;
        Ok(try_with!(a.parse(program, scope, view_window) => ("ParseFromView", format!("{:?}", v_expr)))
            .map(|(va, _)| (va, input)))
    }

    #[inline(never)]
    fn parse_let_format<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::LetFormat(da, name, db) = self else {
            unreachable!()
        };
        try_with!(da.parse(program, scope, input) => ("LetFormat(lhs)", name.to_string())).join(
            |(va, input)| {
                let new_scope = Scope::Single(SingleScope::new(scope, name, &va));
                db.parse(program, &new_scope, input)
            },
        )
    }

    #[inline(never)]
    fn parse_monad_seq<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::MonadSeq(da, db) = self else {
            unreachable!()
        };
        try_with!(da.parse(program, scope, input) => "MonadSeq(lhs)")
            .join(|(_, input)| Ok(try_with!(db.parse(program, scope, input) => "MonadSeq(rhs)")))
    }

    #[inline(never)]
    fn parse_for_each<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::ForEach(expr, lbl, a) = self else {
            unreachable!()
        };
        // we need val because it would otherwise be a dropped temporary binding
        let val = expr
            .eval_value(scope)
            .trace_eval(|| ("ForEach(expr)", format!("{expr:?}")))?;
        let seq = val.get_sequence().expect("bad type for ForEach input");
        Ok(WithErr::fold(
            (Vec::with_capacity(seq.len()), input),
            seq,
            |(mut v, input), e| {
                let new_scope = Scope::Single(SingleScope::new(scope, lbl, &e));
                Ok(try_with!(a.parse(program, &new_scope, input) => ("ForEach", v.len(), format!("{e:?}")))
                    .map(|(va, next_input)| {
                        v.push(va);
                        (v, next_input)
                    }))
            },
        )?
        .map(|(v, input)| (Value::Seq(v.into()), input)))
    }

    #[inline(never)]
    fn parse_repeat_count<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::RepeatCount(expr, a) = self else {
            unreachable!()
        };
        let count = expr
            .eval_value(scope)
            .and_then(|v| v.try_as_usize())
            .trace_eval(|| ("RepeatCount(expr)", format!("{expr:?}")))?;
        Ok(WithErr::fold(
            (Vec::with_capacity(count), input),
            0..count,
            |(mut v, input), _| {
                Ok(
                    try_with!(a.parse(program, scope, input) => ("RepeatCount", v.len())).map(
                        |(va, next_input)| {
                            v.push(va);
                            (v, next_input)
                        },
                    ),
                )
            },
        )?
        .map(|(v, input)| (Value::Seq(v.into()), input)))
    }

    #[inline(never)]
    fn parse_repeat_between<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::RepeatBetween(reps_left_tree, range, a) = self else {
            unreachable!()
        };
        let (min, max) = (range.min(), range.max());
        let mut res = WithErr::new((Vec::new(), input));
        loop {
            // NOTE - lookahead must be performed from the current position, not the start of the repetition
            let (v, cur_input) = res.as_ref();
            let reps_left = reps_left_tree.matches(*cur_input).ok_or(
                DecodeErrorKind::NoValidBranch {
                    offset: cur_input.offset,
                }
                .with_trace(v.len()),
            )?;
            if reps_left == 0 || v.len() == max {
                if v.len() < min {
                    return Err(DecodeErrorKind::InsufficientRepeats {
                        min,
                        found: v.len(),
                        offset: cur_input.offset,
                    }
                    .with_trace("RepeatBetween"));
                }
                break;
            }
            res = res.join(|(mut v, input)| {
                Ok(
                    try_with!(a.parse(program, scope, input) => ("RepeatBetween", v.len())).map(
                        |(va, next_input)| {
                            v.push(va);
                            (v, next_input)
                        },
                    ),
                )
            })?;
        }
        Ok(res.map(|(v, input)| (Value::Seq(v.into()), input)))
    }

    #[inline(never)]
    fn parse_maybe<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Maybe(expr, a) = self else {
            unreachable!()
        };
        let is_present = expr
            .eval_value(scope)
            .trace_eval(|| ("Maybe(expr)", format!("{expr:?}")))?
            .unwrap_bool();
        if is_present {
            Ok(try_with!(a.parse(program, scope, input) => "Maybe")
                .map(|(val, input)| (Value::Option(Some(Box::new(val))), input)))
        } else {
            Ok(WithErr::new((Value::Option(None), input)))
        }
    }

    #[inline(never)]
    fn parse_repeat_until_last<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::RepeatUntilLast(expr, a) = self else {
            unreachable!()
        };
        // third value is "done" flag
        let mut res = WithErr::new((Vec::new(), input, false));
        loop {
            res = res.join(|(mut v, input, _done)| {
                try_with!(a.parse(program, scope, input) => ("RepeatUntilLast", v.len())).join(
                    |(va, next_input)| {
                        let done = expr
                            .eval_lambda(scope, &va)
                            .trace_eval(|| ("RepeatUntilLast(expr)", format!("{expr:?}")))?
                            .unwrap_bool();
                        v.push(va);
                        Ok(WithErr::new((v, next_input, done)))
                    },
                )
            })?;
            break_if_done!(res => (v, input));
        }
        Ok(res.map(|(v, input, _)| (Value::Seq(v.into()), input)))
    }

    #[inline(never)]
    fn parse_repeat_until_seq<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::RepeatUntilSeq(expr, a) = self else {
            unreachable!()
        };
        // third value is "done" flag
        let mut res = WithErr::new((Vec::new(), input, false));
        loop {
            res = res.join(|(mut v, input, _done)| {
                try_with!(a.parse(program, scope, input) => ("RepeatUntilSeq", format!("len={}", v.len()), format!("{a:?}")))
                    .join(|(va, next_input)| {
                        v.push(va);
                        let vs = Value::Seq(v.into());
                        let done = expr.eval_lambda(scope, &vs).trace_eval(|| ("RepeatUntilSeq(expr)", format!("{expr:?}")))?.unwrap_bool();
                        let v = match vs {
                            Value::Seq(v) => v.into_vec(),
                            _ => unreachable!(),
                        };
                        Ok(WithErr::new((v, next_input, done)))
                    })
            })?;
            break_if_done!(res => (v, input));
        }
        Ok(res.map(|(v, input, _)| (Value::Seq(v.into()), input)))
    }

    #[inline(never)]
    fn parse_accum_until<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::AccumUntil(f_done, f_update, init, _vt, a) = self else {
            unreachable!()
        };
        let accum = init
            .eval_value(scope)
            .trace_eval(|| ("AccumUntil(init)", format!("{init:?}")))?;
        let mut res = WithErr::new((Vec::new(), accum, input, false));
        loop {
            res = res.join(|(mut v, accum, input, _done)| {
                let done_arg =
                    Value::Tuple(vec![accum.clone(), Value::Seq(v.clone().into())]);
                let is_done = f_done.eval_lambda(scope, &done_arg).trace_eval(|| ("AccumUntil(f_done)", format!("{f_done:?}")))?.unwrap_bool();
                if is_done {
                    return Ok(WithErr::new((v, accum, input, true)));
                }
                try_with!(a.parse(program, scope, input) => ("AccumUntil", format!("len={}", v.len()), format!("accum={:?}", accum)))
                    .join(|(next_elem, next_input)| {
                        v.push(next_elem.clone());
                        let update_arg = Value::Tuple(vec![accum.clone(), next_elem]);
                        let next_accum = f_update.eval_lambda(scope, &update_arg).trace_eval(|| ("AccumUntil(f_update)", format!("{f_update:?}")))?;
                        Ok(WithErr::new((v, next_accum, next_input, false)))
                    })
            })?;
            break_if_done!(res => (v, accum, input));
        }
        Ok(
            res.map(|(v, accum, input, _)| {
                (Value::Tuple(vec![accum, Value::Seq(v.into())]), input)
            }),
        )
    }

    #[inline(never)]
    fn parse_peek<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Peek(a) = self else {
            unreachable!()
        };
        Ok(a.parse(program, scope, input)?.map(|(v, _)| (v, input)))
    }

    #[inline(never)]
    fn parse_peek_not<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::PeekNot(a) = self else {
            unreachable!()
        };
        match a.parse(program, scope, input) {
            Ok(_) => Err(DecodeErrorKind::<Value>::fail(scope, input)
                .with_trace(("PeekNot", format!("{:?}", a)))),
            Err(err) if err.is_recursion_limit() => Err(err),
            Err(_) => Ok(WithErr::new((Value::Tuple(vec![]), input))),
        }
    }

    #[inline(never)]
    fn parse_slice<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Slice(expr, a) = self else {
            unreachable!()
        };
        let size = expr
            .eval_value(scope)
            .and_then(|v| v.try_as_usize())
            .trace_eval(|| ("Slice(expr)", format!("{expr:?}")))?;
        let (mut slice, input) = input.split_at(size).ok_or(
            input
                .kind
                .overrun(size, input.offset)
                .with_trace(("Slice(create)", format!("{:?}->{size}", expr))),
        )?;
        slice.kind &= BufferKind::Slice;
        Ok(
            try_with!(a.parse(program, scope, slice) => ("Slice(parse)", format!("{:?}", a)))
                .map(|(v, _)| (v, input)),
        )
    }

    #[inline(never)]
    fn parse_bits<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Bits(a) = self else {
            unreachable!()
        };
        // FIXME - copying the entire buffer as bits feels inefficient, we should measure performance and see if there is a better alternative
        let mut bits = Vec::with_capacity(input.remaining().len() * 8);
        for b in input.remaining() {
            for i in 0..8 {
                bits.push((b & (1 << i)) >> i);
            }
        }
        try_with!(a.parse(program, scope, ReadCtxt::new(&bits)) => ("Bits(parse)", format!("{a:?}")))
            .join(|(v, bits)| {
                let bytes_remain = bits.remaining().len() >> 3;
                let bytes_read = input.remaining().len() - bytes_remain;
                let (_, input) = input
                    .split_at(bytes_read)
                    .ok_or(input.kind.overrun(bytes_read, input.offset))?;
                Ok(WithErr::new((v, input)))
            })
    }

    #[inline(never)]
    fn parse_with_relative_offset<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::WithRelativeOffset(base_addr, expr, a) = self else {
            unreachable!()
        };
        let base = base_addr
            .eval_value(scope)
            .and_then(|v| v.try_as_usize())
            .trace_eval(|| ("WithRelativeOffset(base_addr)", format!("{base_addr:?}")))?;
        let offset = expr
            .eval_value(scope)
            .and_then(|v| v.try_as_usize())
            .trace_eval(|| ("WithRelativeOffset(expr)", format!("{expr:?}")))?;
        // `checked_add` rather than `+`: an unchecked add here would panic on overflow in
        // debug builds and silently wrap in release builds, potentially seeking to a
        // small, wrapped-around (and spuriously valid-looking) offset instead of failing.
        let abs_offset = base.checked_add(offset).ok_or_else(|| {
            input
                .kind
                .offset_overflow(base, offset)
                .with_trace("WithRelativeOffset(seek)")
        })?;
        let seek_input = input.seek_to(abs_offset).ok_or(
            input
                .kind
                .bad_seek(abs_offset, input.input.len())
                .with_trace("WithRelativeOffset(seek)"),
        )?;
        Ok(try_with!(a.parse(program, scope, seek_input) => ("WithRelativeOffset(parse)", format!("{a:?}")))
            .map(|(v, _)| (v, input)))
    }

    #[inline(never)]
    fn parse_map<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Map(d, expr) = self else {
            unreachable!()
        };
        try_with!(d.parse(program, scope, input) => ("Map(parse)", format!("{d:?}"))).join(
            |(orig, input)| {
                let v = expr
                    .eval_lambda(scope, &orig)
                    .trace_eval(|| ("Map(expr)", format!("{expr:?}")))?;
                Ok(WithErr::new((
                    Value::Mapped(Box::new(orig), Box::new(v)),
                    input,
                )))
            },
        )
    }

    #[inline(never)]
    fn parse_where<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Where(d, cond) = self else {
            unreachable!()
        };
        try_with!(d.parse(program, scope, input) => ("Where(parse)", format!("{d:?}"))).join(
            |(v, input)| {
                let Condition { expr, severity } = cond;
                match expr
                    .eval_lambda(scope, &v)
                    .trace_eval(|| ("Where(expr)", format!("{expr:?}")))?
                    .unwrap_bool()
                {
                    true => Ok(WithErr::new((v, input))),
                    false => {
                        let err =
                            DecodeErrorKind::bad_where(scope, expr.clone(), Box::new(v.clone()))
                                .into();
                        if severity.is_strict() {
                            Err(err)
                        } else {
                            Ok(WithErr::with_err((v, input), err))
                        }
                    }
                }
            },
        )
    }

    #[inline(never)]
    fn parse_compute<'input>(
        &self,
        _program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Compute(expr) = self else {
            unreachable!()
        };
        let v = expr
            .eval_value(scope)
            .trace_eval(|| ("Compute(expr)", format!("{expr:?}")))?;
        Ok(WithErr::new((v, input)))
    }

    #[inline(never)]
    fn parse_let<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Let(name, expr, d) = self else {
            unreachable!()
        };
        let v = expr
            .eval_value(scope)
            .trace_eval(|| ("Let(expr)", format!("{expr:?}")))?;
        let let_scope = SingleScope::new(scope, name, &v);
        Ok(
            try_with!(d.parse(program, &Scope::Single(let_scope), input) => ("Let(parse)", format!("{} := {:?} <- {:?}", name, v, expr), format!("{d:?}"))),
        )
    }

    #[inline(never)]
    fn parse_let_view<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::LetView(name, d) = self else {
            unreachable!()
        };
        let mut view = input;
        view.kind &= BufferKind::View;
        let let_scope = ViewScope::new(scope, name, view);
        Ok(
            try_with!(d.parse(program, &Scope::View(let_scope), input) => ("LetView(parse)", name.to_string(), format!("{d:?}"))),
        )
    }

    #[inline(never)]
    fn parse_match<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Match(head, branches) = self else {
            unreachable!()
        };
        let head = head
            .eval(scope)
            .trace_eval(|| ("Match(head)", format!("{head:?}")))?;
        for (index, (pattern, decoder)) in branches.iter().enumerate() {
            if let Some(pattern_scope) = head.matches(scope, pattern) {
                return Ok(
                    try_with!(decoder.parse(program, &Scope::Multi(&pattern_scope), input)
                    => ("Match(parse)", format!("[{}]: {:?} => {:?}", index, pattern, decoder))
                    )
                    .map(|(v, input)| (Value::Branch(index, Box::new(v)), input)),
                );
            }
        }
        // NOTE - if we reach this point, it means that none of the patterns matched the head value, and we therefore have to return an error
        let cases = branches.iter().map(|(p, _)| p.clone()).collect::<Vec<_>>();
        Err(DecodeError::from(DecodeErrorKind::RefutedPatternMatch {
            cases,
            value: Box::new(head.into_owned()),
        }))
    }

    #[inline(never)]
    fn parse_dynamic_huffman<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Dynamic(name, DynFormat::Huffman(lengths_expr, opt_values_expr), d) = self
        else {
            unreachable!()
        };
        let lengths_val = lengths_expr
            .eval(scope)
            .trace_eval(|| ("Dynamic(lengths_expr)", format!("{lengths_expr:?}")))?;
        let lengths = value_to_vec_usize(lengths_val.as_ref());
        let lengths = match opt_values_expr {
            None => lengths,
            Some(e) => {
                let values = value_to_vec_usize(
                    e.eval(scope)
                        .trace_eval(|| ("Dynamic(e)", format!("{e:?}")))?
                        .as_ref(),
                );
                let mut new_lengths = [0].repeat(values.len());
                for i in 0..lengths.len() {
                    new_lengths[values[i]] = lengths[i];
                }
                new_lengths
            }
        };
        let f = make_huffman_codes(&lengths);
        let dyn_d = Compiler::compile_one(&f).unwrap();
        let child_scope = DecoderScope::new(scope, name, dyn_d);
        Ok(
            try_with!(d.parse(program, &Scope::Decoder(child_scope), input) => ("Dynamic(parse)", name.to_string(), format!("{d:?}"))),
        )
    }

    #[inline(never)]
    fn parse_apply<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Apply(name) = self else {
            unreachable!()
        };
        let d = scope.get_decoder_by_name(name);
        Ok(
            try_with!(d.parse(program, scope, input) => ("Apply(parse)", name.to_string(), format!("{d:?}"))),
        )
    }

    #[inline(never)]
    fn parse_lifted_option_none<'input>(
        &self,
        _program: &Program,
        _scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::LiftedOption(None) = self else {
            unreachable!()
        };
        Ok(WithErr::new((Value::Option(None), input)))
    }

    #[inline(never)]
    fn parse_lifted_option_some<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::LiftedOption(Some(dec)) = self else {
            unreachable!()
        };
        Ok(try_with!(dec
            .parse(program, scope, input) => ("LiftedOption(parse)", format!("{dec:?}")))
        .map(|(v, input)| (Value::Option(Some(Box::new(v))), input)))
    }

    #[inline(never)]
    fn parse_capture_bytes<'input>(
        &self,
        _program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::CaptureBytes(v_expr, len) = self else {
            unreachable!()
        };
        let len = len
            .eval_value(scope)
            .and_then(|v| v.try_as_usize())
            .trace_eval(|| ("CaptureBytes(len)", format!("{len:?}")))?;

        let view_window = Self::eval_view_expr(scope, v_expr)?;

        // accumulate `len` bytes into a Vec<Value>
        let mut accum = Vec::with_capacity(len);
        let mut buf = view_window;
        for _ in 0..len {
            let Some((byte, new_buf)) = buf.read_byte() else {
                return Err(input.kind.overbyte(buf.offset).with_trace((
                    "CaptureBytes",
                    format!("len: {len}"),
                    format!("view: {v_expr:?}"),
                )));
            };
            accum.push(Value::U8(byte));
            buf = new_buf;
        }

        // return the accumulated bytes, along with the original input
        Ok(WithErr::new((Value::Seq(SeqKind::Strict(accum)), input)))
    }

    #[inline(never)]
    fn parse_read_array<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::ReadArray(v_expr, len, kind) = self else {
            unreachable!()
        };
        let len = len
            .eval_value(scope)
            .and_then(|v| v.try_as_usize())
            .trace_eval(|| ("ReadArray(len)", format!("{len:?}")))?;
        let view_window = Self::eval_view_expr(scope, v_expr)?;

        let folded = WithErr::fold(
            (Vec::with_capacity(len), view_window),
            0..len,
            |(mut accum, buf), ix| {
                Ok(try_with!(read_array_elem(program, buf, kind) => ("ReadArray", format!("index: {ix}"), format!("kind: {kind:?}"), format!("view: {v_expr:?}"))).map(|(val, new_buf)| {
                    accum.push(val);
                    (accum, new_buf)
                }))
            },
        )?;
        Ok(folded.map(|(accum, _buf)| (Value::Seq(SeqKind::Strict(accum)), input)))
    }

    #[inline(never)]
    fn parse_reify_view<'input>(
        &self,
        _program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::ReifyView(v_expr) = self else {
            unreachable!()
        };
        let view = try_with!(Self::eval_view_expr(scope, v_expr) => ("ReifyView", format!("view: {v_expr:?}")));
        Ok(WithErr::new((
            Value::View {
                offset: view.offset,
            },
            input,
        )))
    }

    #[cfg(feature = "format_enforce")]
    #[inline(never)]
    fn parse_enforce<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Enforce(a) = self else {
            unreachable!()
        };
        let res = a.parse(program, scope, input)?;
        match res.into_strict() {
            Ok((v, input)) => Ok(WithErr::new((v, input))),
            Err(mut errs) => {
                let e = errs.swap_remove(0);
                return Err(e);
            }
        }
    }

    #[inline(never)]
    fn parse_permit<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Permit(a, expr) = self else {
            unreachable!()
        };
        let res = a.parse(program, scope, input);
        if matches!(&res, Err(err) if err.is_recursion_limit()) {
            return res;
        }
        Ok(try_downgrade_error_with(
            res.map(|ok| ok.map(|(v, input)| (Value::Permit(Ok(Box::new(v))), input))),
            || {
                Ok::<_, EvalError>((
                    Value::Permit(Err(Some(Box::new(expr.eval_value(scope)?)))),
                    input,
                ))
            },
        )
        .trace_eval(|| ("Permit(default)", format!("{expr:?}")))?)
    }

    #[inline(never)]
    fn parse_recursive<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> EDecodeResult<(Value, ReadCtxt<'input>)> {
        let Decoder::Recursive(level, d) = self else {
            unreachable!()
        };
        let Some(_guard) = RecursionGuard::enter(program.recursion_limit) else {
            return Err(DecodeErrorKind::RecursionLimit {
                level: level.clone(),
                limit: program.recursion_limit,
                offset: input.offset,
            }
            .into());
        };
        d.parse(program, scope, input)
    }
    /// Given a `ViewExpr` and a `Scope` to evaluate named views under, returns the appropriate `View`
    /// (`:= ReadCtxt`).
    ///
    /// Returns an error if the `ViewExpr` is defined via an offset that overruns the base view.
    ///
    /// # Panics
    ///
    /// Will panic if the `ViewExpr` depends on a view-variable that is not in `scope`.
    fn eval_view_expr<'a>(
        scope: &Scope<'a>,
        v_expr: &ViewExpr,
    ) -> Result<View<'a>, DecodeErrorKind> {
        match v_expr {
            ViewExpr::Var(ident) => {
                let view = scope.get_view_by_name(ident);
                Ok(view)
            }
            ViewExpr::Offset(base, offset) => {
                let offset = offset.eval_value(scope)?.try_as_usize()?;
                let base_view = Self::eval_view_expr(scope, base)?;
                let Some((_, mut view_window)) = base_view.split_at(offset) else {
                    return Err(base_view.kind.overrun(offset, base_view.offset).into());
                };
                view_window.kind &= BufferKind::View;
                Ok(view_window)
            }
        }
    }
}

fn read_array_elem<'a>(
    program: &'a Program,
    buf: ReadCtxt<'a>,
    kind: &ReadArrayKind,
) -> EDecodeResult<(Value, ReadCtxt<'a>)> {
    match kind {
        ReadArrayKind::Base(kind) => Ok(WithErr::new(read_base(buf, *kind)?)),
        ReadArrayKind::Single { elem, .. } => read_spine_elem(program, buf, elem),
        ReadArrayKind::FixedFormat { fields, .. } => read_fixed_record(program, buf, fields),
    }
}

/// Reads a single `SpineDecoder`-kinded parse-directive: either a direct base-kind read, or a
/// recursive invocation of the compiled sub-`Decoder` for a `SpineElem::Indirect`'s `FormatRef`.
fn read_spine_elem<'a>(
    program: &'a Program,
    buf: ReadCtxt<'a>,
    elem: &SpineDecoder,
) -> EDecodeResult<(Value, ReadCtxt<'a>)> {
    match elem {
        SpineDecoder::Raw(kind) => Ok(WithErr::new(read_base(buf, *kind)?)),
        SpineDecoder::Indirect { dec } => dec.parse(program, &Scope::Empty, buf),
    }
}

/// Reads one element of a `FixedFormat`-kinded `ReadArray`: sequentially reads each field of
/// `fields` (in order), and constructs a `Value::Record` out of the persisted (named) ones.
/// Anonymous/ephemeral fields (`None`) are read -- consuming their share of bytes -- but
/// discarded rather than included in the resulting record, matching `record_fmt::analyze_fixed_shape`.
fn read_fixed_record<'a>(
    program: &'a Program,
    buf: ReadCtxt<'a>,
    fields: &[(Option<Label>, SpineDecoder)],
) -> EDecodeResult<(Value, ReadCtxt<'a>)> {
    let folded = WithErr::fold(
        (Vec::with_capacity(fields.len()), buf),
        fields,
        |(mut captured, input), (name, elem)| {
            Ok(try_with!(read_spine_elem(program, input, elem) => ("FixedFormat field", format!("name: {name:?}"))).map(|(v, new_buf)| {
                if let Some(name) = name {
                    captured.push((name.clone(), v));
                }
                (captured, new_buf)
            }))
        },
    )?;
    Ok(folded.map(|(captured, buf)| (Value::Record(captured), buf)))
}

fn read_base(buf: ReadCtxt<'_>, kind: BaseKind<Endian>) -> DecodeResult<(Value, ReadCtxt<'_>)> {
    buf.read_base(kind)
        .map_err(|e| DecodeErrorKind::from(e).into())
}

fn value_to_vec_usize(v: &Value) -> Vec<usize> {
    let vs = match v {
        Value::Seq(vs) => vs,
        _ => panic!("expected Seq"),
    };
    vs.iter()
        .map(|v| match v.coerce_mapped_value().into_inner() {
            Value::U8(n) => *n as usize,
            _ => panic!("expected U8"),
        })
        .collect::<Vec<usize>>()
}

fn make_huffman_codes(lengths: &[usize]) -> Format {
    let max_length = *lengths.iter().max().unwrap();
    let mut bl_count = [0].repeat(max_length + 1);

    for len in lengths {
        bl_count[*len] += 1;
    }

    let mut next_code = [0].repeat(max_length + 1);
    let mut code = 0;
    bl_count[0] = 0;

    for bits in 1..max_length + 1 {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }

    let mut codes = Vec::with_capacity(lengths.len());

    for (n, &len) in lengths.iter().enumerate() {
        if len != 0 {
            codes.push(Format::Map(
                Box::new(bit_range(len, next_code[len])),
                Box::new(Expr::Lambda(
                    "_".into(),
                    Box::new(Expr::U16(n.try_into().unwrap())),
                )),
            ));
            //println!("{:?}", codes[codes.len()-1]);
            next_code[len] += 1;
        } else {
            //codes.push((n.to_string(), Format::Fail));
        }
    }

    Format::Union(codes)
}

fn bit_range(n: usize, bits: usize) -> Format {
    let mut fs = Vec::with_capacity(n);
    for i in 0..n {
        let r = n - 1 - i;
        let b = (bits & (1 << r)) >> r != 0;
        fs.push(is_bit(b));
    }
    Format::Tuple(fs)
}

fn is_bit(b: bool) -> Format {
    Format::Byte(ByteSet::from([if b { 1 } else { 0 }]))
}

/// Applies a lifetime-preserving transformation to a reference held behind a [`std::borrow::Cow`],
/// referencing when possible and forcing ownership only when necessary.
pub(crate) fn cow_map<'a, T, U>(x: Cow<'a, T>, f: impl for<'i> Fn(&'i T) -> &'i U) -> Cow<'a, U>
where
    T: 'static + Clone,
    U: 'static + Clone + ToOwned,
{
    match x {
        Cow::Borrowed(x) => Cow::Borrowed(f(x)),
        Cow::Owned(x) => Cow::Owned(f(&x).to_owned()),
    }
}

/// Like [`cow_map`], but applies a closure argument `f` that itself returns a `Cow<'i, U>` value (instead of a `&'i U` value)
pub(crate) fn cow_remap<'a, T, U>(
    x: Cow<'a, T>,
    f: impl for<'i> Fn(&'i T) -> Cow<'i, U>,
) -> Cow<'a, U>
where
    T: 'static + Clone,
    U: 'static + Clone + ToOwned,
{
    match x {
        Cow::Borrowed(x) => f(x),
        Cow::Owned(x) => Cow::Owned(f(&x).into_owned()),
    }
}

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use super::*;
    use crate::helper::*;

    mod recursion_limit {
        use super::*;
        use crate::FormatRef;

        /// Registers `nat := 'Z' | 'S' nat`, whose every call to itself counts towards the limit.
        fn nat(module: &mut FormatModule) -> FormatRef {
            module.define_format_rec_batch(vec![(
                "test.nat",
                alts([
                    ("Z", is_byte(b'Z')),
                    ("S", tuple([is_byte(b'S'), Format::RecVar(0)])),
                ]),
            )])[0]
        }

        /// Decodes `input` with both interpreters and returns their outcomes, which must agree:
        /// `Ok(())`, or `Err(true)` for a `RecursionLimit` error and `Err(false)` for any other.
        /// Also checks that no decoder is left counted as running afterwards.
        fn run(
            module: &FormatModule,
            top: &Format,
            limit: usize,
            input: &[u8],
        ) -> Result<(), bool> {
            let mut program = Compiler::compile_program(module, top).unwrap();
            program.set_recursion_limit(limit);
            let plain = program
                .run(ReadCtxt::new(input))
                .map(|_| ())
                .map_err(|err| err.is_recursion_limit());
            assert_eq!(recursion_depth(), 0, "run left recursive decoders counted");
            let with_loc = program
                .run_with_loc(ReadCtxt::new(input))
                .map(|_| ())
                .map_err(|err| err.is_recursion_limit());
            assert_eq!(
                recursion_depth(),
                0,
                "run_with_loc left recursive decoders counted"
            );
            assert_eq!(plain, with_loc, "interpreters disagree");
            plain
        }

        fn s_then_z(n: usize) -> Vec<u8> {
            let mut input = vec![b'S'; n];
            input.push(b'Z');
            input
        }

        /// Each nested call to a recursive level counts once: `S^n Z` makes `n + 1` calls.
        #[test]
        fn counts_nested_calls_to_recursive_levels() {
            let mut module = FormatModule::new();
            let top = nat(&mut module).call();
            assert_eq!(run(&module, &top, 8, &s_then_z(7)), Ok(()));
            assert_eq!(run(&module, &top, 8, &s_then_z(8)), Err(true));
        }

        #[test]
        fn default_limit_is_256() {
            let mut module = FormatModule::new();
            let top = nat(&mut module).call();
            let program = Compiler::compile_program(&module, &top).unwrap();
            assert_eq!(program.recursion_limit(), 256);
            // only `run`: `run_with_loc` overflows a debug-build stack well short of 256 levels
            let run = |input: &[u8]| {
                let res = program.run(ReadCtxt::new(input));
                assert_eq!(recursion_depth(), 0, "run left recursive decoders counted");
                res.map(|_| ()).map_err(|err| err.is_recursion_limit())
            };
            assert_eq!(run(&s_then_z(255)), Ok(()));
            assert_eq!(run(&s_then_z(256)), Err(true));
        }

        /// `Permit` would otherwise substitute its default.
        #[test]
        fn permit_propagates_it() {
            let mut module = FormatModule::new();
            let nat = nat(&mut module);
            let top = permit(chain(nat.call(), "n", compute(Expr::U8(0))), Expr::U8(1));
            assert_eq!(run(&module, &top, 8, &s_then_z(7)), Ok(()));
            assert_eq!(run(&module, &top, 8, &s_then_z(8)), Err(true));
        }

        /// `PeekNot` would otherwise succeed. Its format is a `Slice` so that its lookahead is
        /// bounded, and fails at the first `'X'` unless the limit is reached first.
        #[test]
        fn peek_not_propagates_it() {
            let mut module = FormatModule::new();
            let nat = nat(&mut module);
            let top = tuple([
                Format::PeekNot(Box::new(slice(Expr::U8(16), nat.call()))),
                repeat(ANY_BYTE),
            ]);
            let s_then_x = |n: usize| {
                let mut input = vec![b'S'; n];
                input.resize(20, b'X');
                input
            };
            assert_eq!(run(&module, &top, 8, &s_then_x(7)), Ok(()));
            assert_eq!(run(&module, &top, 8, &s_then_x(8)), Err(true));
        }

        /// Inside a recursive decoder, a `UnionNondet` propagates the error rather than trying its
        /// next branch, here a `Shallow` branch that would accept the rest of the input.
        #[test]
        fn union_nondet_inside_recursion_propagates_it() {
            let mut module = FormatModule::new();
            let deep = module.define_format_rec_batch(vec![(
                "test.deep",
                alts_nondet([
                    ("Deep", tuple([is_byte(b'S'), Format::RecVar(0)])),
                    ("Shallow", repeat(ANY_BYTE)),
                ]),
            )])[0];
            let top = deep.call();
            assert_eq!(run(&module, &top, 8, &[b'S'; 7]), Ok(()));
            assert_eq!(run(&module, &top, 8, &[b'S'; 20]), Err(true));
        }

        /// Outside any recursive decoder, a `UnionNondet` treats the error like any other and tries
        /// its next branch.
        #[test]
        fn union_nondet_outside_recursion_tries_next_branch() {
            let mut module = FormatModule::new();
            let nat = nat(&mut module);
            let top = alts_nondet([("Nat", nat.call()), ("Rest", repeat(ANY_BYTE))]);
            assert_eq!(run(&module, &top, 8, &s_then_z(8)), Ok(()));
            let program = {
                let mut program = Compiler::compile_program(&module, &top).unwrap();
                program.set_recursion_limit(8);
                program
            };
            let (value, _) = program.run(ReadCtxt::new(&s_then_z(8))).unwrap();
            assert!(
                matches!(value, Value::Branch(1, _)),
                "expected the `Rest` branch, got {value:?}"
            );
        }
    }

    /// Identical decoders merge into the first of them, and calls are renumbered to match; two
    /// decoders that differ only in calling decoders that themselves merge (`3` and `4` below)
    /// merge in a later round. The entry point keeps index 0.
    #[test]
    fn merge_identical_decoders_merges_transitively_and_renumbers_calls() {
        let call = |ix| Decoder::Call(ix, Vec::new(), Vec::new());
        let byte = |b| Decoder::Byte(ByteSet::from([b]));
        let mut decoders = vec![
            (
                Decoder::Tuple(vec![call(3), call(4), call(5)]),
                ValueType::Any,
            ),
            (byte(b'a'), ValueType::U8),
            (byte(b'a'), ValueType::U8),
            (Decoder::Tuple(vec![call(1)]), ValueType::Any),
            (Decoder::Tuple(vec![call(2)]), ValueType::Any),
            (byte(b'b'), ValueType::U8),
        ];
        merge_identical_decoders(&mut decoders);
        assert_eq!(
            decoders,
            vec![
                (
                    Decoder::Tuple(vec![call(2), call(2), call(3)]),
                    ValueType::Any
                ),
                (byte(b'a'), ValueType::U8),
                (Decoder::Tuple(vec![call(1)]), ValueType::Any),
                (byte(b'b'), ValueType::U8),
            ]
        );
    }

    /// Test-helper function for decoder tests.
    ///
    /// Given a decoder `d` and input `input`, checks that `d.parse(input)` returns the value `expect`,
    /// and that the remaining input after parsing matches `tail`.
    fn accepts(d: &Decoder, input: &[u8], tail: &[u8], expect: Value) {
        let program = Program::new();
        let (val, remain) = d
            .parse(&program, &Scope::Empty, ReadCtxt::new(input))
            .unwrap()
            .into_inner();
        assert_eq!(val, expect);
        assert_eq!(remain.remaining(), tail);
    }

    fn rejects(d: &Decoder, input: &[u8]) {
        let program = Program::new();
        assert!(
            d.parse(&program, &Scope::Empty, ReadCtxt::new(input))
                .is_err()
        );
    }

    fn value_some(x: Value) -> Value {
        Value::Option(Some(Box::new(x)))
    }

    #[test]
    fn compile_fail() {
        let f = Format::Fail;
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_empty() {
        let f = Format::EMPTY;
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[0x00], Value::UNIT);
    }

    #[test]
    fn compile_byte_is() {
        let f = is_byte(0x00);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0));
        accepts(&d, &[0x00, 0xFF], &[0xFF], Value::U8(0));
        rejects(&d, &[0xFF]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_byte_not() {
        let f = not_byte(0x00);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        accepts(&d, &[0xFF, 0x00], &[0x00], Value::U8(0xFF));
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt() {
        let f = alts::<&str>([]);
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_alt_byte() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0xFF))]);
        let d = Compiler::compile_one(&f).unwrap();
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
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_byte() {
        let slice_a = slice(Expr::U8(1), is_byte(0x00));
        let slice_b = slice(Expr::U8(1), is_byte(0xFF));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        let d = Compiler::compile_one(&f).unwrap();
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
        let slice_a = slice(Expr::U8(1), is_byte(0x00));
        let slice_b = slice(Expr::U8(1), is_byte(0x00));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_ambiguous2() {
        let tuple_a = Format::Tuple(vec![is_byte(0x00), is_byte(0x00)]);
        let tuple_b = Format::Tuple(vec![is_byte(0x00), is_byte(0xFF)]);
        let slice_a = slice(Expr::U8(1), tuple_a);
        let slice_b = slice(Expr::U8(1), tuple_b);
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail() {
        let f = alts([("a", Format::Fail), ("b", Format::Fail)]);
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_end_of_input() {
        let f = alts([("a", Format::EndOfInput), ("b", Format::EndOfInput)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_empty() {
        let f = alts([("a", Format::EMPTY), ("b", Format::EMPTY)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail_end_of_input() {
        let f = alts([("a", Format::Fail), ("b", Format::EndOfInput)]);
        let d = Compiler::compile_one(&f).unwrap();
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
        let d = Compiler::compile_one(&f).unwrap();
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
        let d = Compiler::compile_one(&f).unwrap();
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
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(value_some(Value::U8(0)))),
                Value::U8(0xFF),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::U8(0xFF),
            ]),
        );
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_opt_opt() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0xFF))]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(value_some(Value::U8(0)))),
                Value::Branch(0, Box::new(value_some(Value::U8(0xFF)))),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(value_some(Value::U8(0)))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(0, Box::new(value_some(Value::U8(0xFF)))),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
    }

    #[test]
    fn compile_alt_opt_ambiguous() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0x00))]);
        assert!(Compiler::compile_one(&f).is_err());
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
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_alt_repeat1_slow() {
        let f = repeat(alts([
            ("a", repeat1(is_byte(0x00))),
            ("b", is_byte(0x01)),
            ("c", is_byte(0x02)),
        ]));
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::Seq(vec![].into()));
        accepts(&d, &[0xFF], &[0xFF], Value::Seq(vec![].into()));
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)].into()));
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)].into()),
        );
    }

    #[test]
    fn compile_repeat_repeat() {
        let f = repeat(repeat(is_byte(0x00)));
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_cat_repeat() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0xFF))]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![].into()), Value::Seq(vec![].into())]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)].into()),
                Value::Seq(vec![].into()),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![].into()),
                Value::Seq(vec![Value::U8(0xFF)].into()),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)].into()),
                Value::Seq(vec![Value::U8(0xFF)].into()),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[0x00],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)].into()),
                Value::Seq(vec![Value::U8(0xFF)].into()),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![Value::Seq(vec![].into()), Value::Seq(vec![].into())]),
        );
    }

    #[test]
    fn compile_cat_end_of_input() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::EndOfInput]);
        let d = Compiler::compile_one(&f).unwrap();
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
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![].into()), Value::UNIT]),
        );
        accepts(
            &d,
            &[0x00, 0x00, 0x00],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00), Value::U8(0x00), Value::U8(0x00)].into()),
                Value::UNIT,
            ]),
        );
        rejects(&d, &[0x00, 0x10]);
    }

    #[test]
    fn compile_cat_repeat_ambiguous() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0x00))]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x7F))),
        ]);
        assert!(Compiler::compile_one(&f).is_ok());
    }

    #[test]
    fn compile_repeat_fields_ambiguous() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x00))),
        ]);
        assert!(Compiler::compile_one(&f).is_err());
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
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::record([
                ("first", Value::Seq(vec![].into())),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::Option(None))),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::Option(None))),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(
                        0,
                        Box::new(value_some(Value::record([
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![].into())]),
                            ),
                            ("third", Value::Seq(vec![].into())),
                        ]))),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(
                        0,
                        Box::new(value_some(Value::record(vec![
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![].into())]),
                            ),
                            ("third", Value::Seq(vec![Value::U8(0x00)].into())),
                        ]))),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0x7F],
            &[0x7F],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::Option(None))),
                ),
            ]),
        );
    }

    #[test]
    fn compile_repeat1() {
        let f = repeat1(is_byte(0x00));
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0xFF]);
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)].into()));
        accepts(
            &d,
            &[0x00, 0xFF],
            &[0xFF],
            Value::Seq(vec![Value::U8(0x00)].into()),
        );
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)].into()),
        );
    }

    #[test]
    fn compile_align1() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::align(1), is_byte(0xFF)]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_align2() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::align(2), is_byte(0xFF)]);
        let d = Compiler::compile_one(&f).unwrap();
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
    #[should_panic(expected = "alignment modulus must be nonzero")]
    fn align_zero_panics() {
        // `n` in `Format::Align(n)`/`Decoder::Align(n)` is always a static, format-author-supplied
        // constant (never derived from parsed data), so `Align(0)` is a spec bug rather than a
        // data-reachable failure. `Format::Align`'s inner type is `NonZeroUsize`, so this is now
        // caught at format-construction time (the earliest possible point, before any compile or
        // decode pass even runs), not at decode time - `Format::Align` itself can't be built with
        // a zero modulus except via the validating `Format::align` constructor.
        let _ = Format::align(0);
    }

    #[test]
    fn compile_peek_not() {
        let any_byte = Format::Byte(ByteSet::full());
        let a = Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)]);
        let peek_not = Format::PeekNot(Box::new(a));
        let f = Format::Tuple(vec![peek_not, any_byte.clone(), any_byte.clone()]);
        let d = Compiler::compile_one(&f).unwrap();
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
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![].into()),
                    ])),
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
                        Value::Seq(vec![Value::U8(0xFF)].into()),
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
                        Value::Seq(vec![Value::U8(0x00), Value::U8(0xFF)].into()),
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
                        Value::Seq(vec![Value::U8(0xFF), Value::U8(0x00)].into()),
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
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_between() {
        let repeat_between = repeat_between(0, 2, is_byte(0));
        let trailer = is_byte(1);
        let f = Format::Tuple(vec![repeat_between, trailer]);
        assert!(Compiler::compile_one(&f).is_ok());
    }

    /// Regression test: lookahead for `RepeatBetween` must be evaluated at the current position on each
    /// iteration (not the start of the repetition), and short repetitions must be errors rather than panics.
    #[test]
    fn repeat_between_counts() {
        let f = Format::Tuple(vec![repeat_between(4, 8, is_byte(b'a')), is_byte(b'b')]);
        let d = Compiler::compile_one(&f).unwrap();
        for n in 4..=8 {
            let mut input = vec![b'a'; n];
            input.push(b'b');
            input.push(b'c');
            let expect = Value::Tuple(vec![
                Value::Seq(vec![Value::U8(b'a'); n].into()),
                Value::U8(b'b'),
            ]);
            accepts(&d, &input, b"c", expect);
        }
        for n in [0, 1, 3, 9] {
            let mut input = vec![b'a'; n];
            input.push(b'b');
            rejects(&d, &input);
        }
    }

    #[test]
    #[ignore = "matchtree cannot unify based on disjoint where clauses, only byte-patterns"]
    fn compile_where_u16be_eq() {
        let a = where_lambda(u16be(), "x", expr_eq(var("x"), Expr::U16(0x00FF)));
        let b = where_lambda(u16be(), "x", expr_eq(var("x"), Expr::U16(0xFF00)));
        let f = Format::Union(vec![a, b]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Branch(0, Box::new(Value::U16(0x00FF))),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Branch(0, Box::new(Value::U16(0xFF00))),
        );
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
        rejects(&d, &[0xFF]);
        rejects(&d, &[0x00, 0x00]);
        rejects(&d, &[0xFF, 0xFF]);
    }

    #[test]
    fn branch_value_record_revamp() {
        let f = Format::record([("inner", Format::Union(vec![is_byte(0), is_byte(1)]))]);
        let d = Compiler::compile_one(&f).unwrap();

        accepts(
            &d,
            &[0x00],
            &[],
            Value::Record(vec![(
                Label::Borrowed("inner"),
                Value::Branch(0, Box::new(Value::U8(0))),
            )]),
        );
    }

    #[test]
    fn permit_valid_parse_expected_value() {
        let f = fmt_some(Format::record([(
            "foo",
            mk_ascii_string(byte_seq(b"hello world")),
        )]));
        let g = permit(f, expr_none());
        let d = Compiler::compile_one(&g).unwrap();

        const DATA: &[u8] = b"hello world";

        let expect = Value::Permit(Ok(Box::new(Value::Option(Some(Box::new(Value::Record(
            vec![(
                Label::Borrowed("foo"),
                Value::Seq(SeqKind::Strict(
                    (b"hello world")
                        .into_iter()
                        .map(|b| Value::U8(*b))
                        .collect::<Vec<_>>(),
                )),
            )],
        )))))));
        assert!(!expect.is_fallback());
        accepts(&d, DATA, &[], expect)
    }

    #[test]
    fn permit_invalid_parse_expected_value() {
        let f = fmt_some(Format::record([(
            "foo",
            mk_ascii_string(byte_seq(b"hello world")),
        )]));
        let df = Compiler::compile_one(&f).unwrap();
        let g = permit(f, expr_none());
        let dg = Compiler::compile_one(&g).unwrap();

        let expect = Value::Permit(Err(Some(Box::new(Value::Option(None)))));

        const DATA: &[u8] = b"hello";

        assert!(expect.is_fallback());
        rejects(&df, DATA);
        accepts(&dg, DATA, DATA, expect)
    }

    #[test]
    fn matches_inner_on_permit_err() {
        let f = permit(
            fmt_some(Format::record([("foo", Format::ANY_BYTE)])),
            expr_none(),
        );
        let g = record([
            ("x", f),
            (
                "is_good",
                fmt_match(
                    var("x"),
                    [
                        (pat_some(bind("y")), compute(Expr::Bool(true))),
                        (pat_none(), compute(Expr::Bool(false))),
                    ],
                ),
            ),
        ]);
        let d = Compiler::compile_one(&g).unwrap();

        const DATA: &[u8] = &[];

        accepts(
            &d,
            DATA,
            DATA,
            Value::Record(vec![
                (
                    Label::Borrowed("x"),
                    Value::Permit(Err(Some(Box::new(Value::Option(None))))),
                ),
                (
                    Label::Borrowed("is_good"),
                    Value::Branch(1, Box::new(Value::Bool(false))),
                ),
            ]),
        );
    }

    #[test]
    fn test_negative_number_match() {
        let f = i8();
        let g = chain(
            f,
            "x",
            fmt_match(
                var("x"),
                [
                    (
                        Pattern::ZConst(num_bigint::BigInt::from(-1)),
                        compute(Expr::Bool(true)),
                    ),
                    (Pattern::Wildcard, compute(Expr::Bool(false))),
                ],
            ),
        );
        let d = Compiler::compile_one(&g).unwrap();

        const DATA: &[u8] = &[0xFF];

        accepts(&d, DATA, &[], Value::Branch(0, Box::new(Value::Bool(true))));
    }

    #[test]
    fn test_bits() {
        let byte = 0b1101_0110;
        // pattern is the sequence of bits of `byte`, read from lsb to msb
        let pattern = [0, 1, 1, 0, 1, 0, 1, 1];

        let f = Format::Bits(Box::new(byte_seq(&pattern)));
        let d = Compiler::compile_one(&f).unwrap();
        let input = &[byte];

        accepts(
            &d,
            input,
            &[],
            Value::Seq(SeqKind::Strict(
                pattern.into_iter().map(Value::U8).collect(),
            )),
        );
    }

    /// Regression test for the `Next`-context bug in `compile_format` for `RepeatCount`
    /// (and analogous repeat-like formats).
    ///
    /// The format `(0xAA (0xBB)*)×2  0xCC` is a well-defined, unambiguous language:
    /// two "runs" of zero-or-more 0xBB bytes each preceded by 0xAA, then a 0xCC sentinel.
    ///
    /// The inner `Repeat(Byte(0xBB))` must stop when it sees 0xAA (the start of the next
    /// outer iteration) as well as when it sees 0xCC (the outer sentinel).  The fix
    /// passes `Next::Repeat(a, outer_next)` as the inner `next`, so its MatchTree gains a
    /// stop-edge for 0xAA in addition to 0xCC.
    #[test]
    fn repeat_count_next_context_correct_behaviour() {
        let inner = record([("a", is_byte(0xAA)), ("bs", repeat(is_byte(0xBB)))]);
        let outer = record([
            ("pairs", repeat_count(Expr::U32(2), inner)),
            ("end", is_byte(0xCC)),
        ]);
        let d = Compiler::compile_one(&outer).unwrap();

        let data: &[u8] = &[0xAA, 0xBB, 0xBB, 0xAA, 0xBB, 0xCC];

        let mk_inner = |bs: Vec<Value>| {
            Value::Record(vec![
                (Label::Borrowed("a"), Value::U8(0xAA)),
                (Label::Borrowed("bs"), Value::Seq(SeqKind::Strict(bs))),
            ])
        };
        let expected = Value::Record(vec![
            (
                Label::Borrowed("pairs"),
                Value::Seq(SeqKind::Strict(vec![
                    mk_inner(vec![Value::U8(0xBB), Value::U8(0xBB)]),
                    mk_inner(vec![Value::U8(0xBB)]),
                ])),
            ),
            (Label::Borrowed("end"), Value::U8(0xCC)),
        ]);

        accepts(&d, data, &[], expected);
    }

    /// Exercises `FixedReadKind::FixedFormat`: a `ReadArray` whose element-type is a
    /// defined format (`point`, a record of two `u16be` fields) rather than a bare `BaseKind`.
    /// Covers eligibility validation (`record_fmt::analyze_fixed_shape`, run both by
    /// `FormatModule::infer_format_type` and by `Compiler::compile_format`) and the interpreter's
    /// inline flattened decode (`decoder::read_fixed_record`).
    #[test]
    fn read_array_fixed_format() {
        let mut module = FormatModule::new();
        let point = module.define_format("point", record([("x", u16be()), ("y", u16be())]));
        let outer = from_here(read_array(Expr::U8(2), point));
        let d = Compiler::new(&module)
            .compile_format(&outer, Rc::new(Next::Empty))
            .unwrap();

        let mk_point = |x: u16, y: u16| {
            Value::Record(vec![
                (Label::Borrowed("x"), Value::U16(x)),
                (Label::Borrowed("y"), Value::U16(y)),
            ])
        };
        let expected = Value::Seq(SeqKind::Strict(vec![mk_point(1, 2), mk_point(3, 4)]));

        // View-based reads (like `CaptureBytes`) read from a separate `View` established by
        // `from_here`/`LetView`, and never advance the main `input` cursor -- so `tail` is the
        // full, unconsumed original input, not the post-read remainder of the view.
        let data: &[u8] = &[0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04];
        accepts(&d, data, data, expected);
        // only 3 bytes available for the second point's 4-byte record -> should fail
        rejects(&d, &[0x00, 0x01, 0x00, 0x02, 0x00, 0x03]);
    }

    /// An ineligible `FixedFormat` reference (a field with a data-dependent length) must be
    /// rejected by `record_fmt::analyze_fixed_shape`, surfacing as a panic at compile-time
    /// (mirroring the existing `Format::to_record_format`/`infer_format_type` panic-on-invariant-
    /// violation convention) rather than silently producing an incorrect stride.
    #[test]
    #[should_panic(expected = "is not eligible for FixedFormat ReadArray")]
    fn read_array_fixed_format_rejects_dependent_length() {
        let mut module = FormatModule::new();
        let variable = module.define_format(
            "variable",
            record([("len", u8()), ("data", repeat_count(var("len"), u8()))]),
        );
        let outer = from_here(read_array(Expr::U8(1), variable));
        let _ = Compiler::new(&module).compile_format(&outer, Rc::new(Next::Empty));
    }
}
