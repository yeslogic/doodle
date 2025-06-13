use std::rc::Rc;

use doodle::{prelude::ByteSet, read::ReadCtxt};

use crate::{Format, FormatDecl, FormatId, FormatModule, RecurseCtx};

#[derive(Debug)]
pub enum GrammarError<CtxValue: std::fmt::Debug + Sized = ()> {
    LeftRecursion {
        top: FormatId,
        cycle: Vec<FormatId>,
        context: CtxValue,
    },
    RepeatNullable {
        format: Format,
        context: CtxValue,
    },
    AmbiguousFirst {
        left: ByteSet,
        right: ByteSet,
        context: CtxValue,
    },
    MultiNullUnion {
        context: CtxValue,
    },
    AmbiguousFollow {
        left: ByteSet,
        right: ByteSet,
        context: CtxValue,
    },
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::Format {}
    impl<'a> Sealed for std::rc::Rc<super::PartialFormat<'a>> {}
    impl<'a> Sealed for super::FormatKind<'a> {}
}

trait IsFormat: private::Sealed + Clone + std::fmt::Debug + Sized {}

impl IsFormat for Format {}
impl<'a> IsFormat for Rc<PartialFormat<'a>> {}
impl<'a> IsFormat for FormatKind<'a> {}

#[derive(Clone)]
pub(crate) enum FormatKind<'a> {
    Total(Format),
    Partial(Rc<PartialFormat<'a>>),
}

impl<'a> std::fmt::Debug for FormatKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatKind::Total(format) => write!(f, "{:?}", format),
            FormatKind::Partial(partial_format) => write!(f, "{:?}", partial_format),
        }
    }
}

impl<'a> From<Format> for FormatKind<'a> {
    fn from(format: Format) -> Self {
        FormatKind::Total(format)
    }
}

impl<'a> From<Rc<PartialFormat<'a>>> for FormatKind<'a> {
    fn from(partial_format: Rc<PartialFormat<'a>>) -> Self {
        FormatKind::Partial(partial_format)
    }
}

impl<'a> From<GrammarError<Format>> for GrammarError<FormatKind<'a>> {
    fn from(err: GrammarError<Format>) -> Self {
        match err {
            GrammarError::LeftRecursion {
                top,
                cycle,
                context,
            } => GrammarError::LeftRecursion {
                top,
                cycle,
                context: context.into(),
            },
            GrammarError::RepeatNullable { format, context } => GrammarError::RepeatNullable {
                format,
                context: context.into(),
            },
            GrammarError::AmbiguousFirst {
                left,
                right,
                context,
            } => GrammarError::AmbiguousFirst {
                left,
                right,
                context: context.into(),
            },
            GrammarError::MultiNullUnion { context } => GrammarError::MultiNullUnion {
                context: context.into(),
            },
            GrammarError::AmbiguousFollow {
                left,
                right,
                context,
            } => GrammarError::AmbiguousFollow {
                left,
                right,
                context: context.into(),
            },
        }
    }
}

impl<'a> From<GrammarError<Rc<PartialFormat<'a>>>> for GrammarError<FormatKind<'a>> {
    fn from(err: GrammarError<Rc<PartialFormat<'a>>>) -> Self {
        match err {
            GrammarError::LeftRecursion {
                top,
                cycle,
                context,
            } => GrammarError::LeftRecursion {
                top,
                cycle,
                context: context.into(),
            },
            GrammarError::RepeatNullable { format, context } => GrammarError::RepeatNullable {
                format,
                context: context.into(),
            },
            GrammarError::AmbiguousFirst {
                left,
                right,
                context,
            } => GrammarError::AmbiguousFirst {
                left,
                right,
                context: context.into(),
            },
            GrammarError::MultiNullUnion { context } => GrammarError::MultiNullUnion {
                context: context.into(),
            },
            GrammarError::AmbiguousFollow {
                left,
                right,
                context,
            } => GrammarError::AmbiguousFollow {
                left,
                right,
                context: context.into(),
            },
        }
    }
}

impl<F: IsFormat> std::fmt::Display for GrammarError<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrammarError::LeftRecursion {
                top,
                cycle,
                context,
            } => {
                write!(
                    f,
                    "left-recursion found in format ({context:?}): #{} -> {:#?}",
                    top, cycle,
                )
            }
            GrammarError::RepeatNullable { format, context } => {
                write!(f, "repeat of nullable format ({context:?}): {:?}", format)
            }
            GrammarError::MultiNullUnion { context } => {
                write!(f, "multiple nullable formats in union ({context:?}")
            }
            GrammarError::AmbiguousFirst {
                left,
                right,
                context,
            } => {
                write!(
                    f,
                    "ambiguity introduced by union of non-disjoint first sets ({context:?}): {:?} <|> {:?}",
                    left, right
                )
            }
            GrammarError::AmbiguousFollow {
                left,
                right,
                context,
            } => {
                write!(
                    f,
                    "follow set and first set conflict ({context:?}): {:?} & {:?} ",
                    left, right
                )
            }
        }
    }
}

impl std::fmt::Display for GrammarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrammarError::LeftRecursion { top, cycle, .. } => {
                write!(
                    f,
                    "left-recursion found in format: #{} -> {:#?}",
                    top, cycle,
                )
            }
            GrammarError::RepeatNullable { format, .. } => {
                write!(f, "repeat of nullable format: {:?}", format)
            }
            GrammarError::MultiNullUnion { .. } => {
                write!(f, "multiple nullable formats in union")
            }
            GrammarError::AmbiguousFirst { left, right, .. } => {
                write!(
                    f,
                    "ambiguity introduced by union of non-disjoint first sets: {:?} <|> {:?}",
                    left, right
                )
            }
            GrammarError::AmbiguousFollow { left, right, .. } => {
                write!(
                    f,
                    "follow set and first set conflict: {:?} & {:?} ",
                    left, right
                )
            }
        }
    }
}

impl std::error::Error for GrammarError {}
impl<F: IsFormat> std::error::Error for GrammarError<F> {}

impl GrammarError<()> {
    fn add_context<F: IsFormat>(self, context: F) -> GrammarError<F> {
        match self {
            GrammarError::LeftRecursion { top, cycle, .. } => GrammarError::LeftRecursion {
                top,
                cycle,
                context,
            },
            GrammarError::RepeatNullable { format, .. } => {
                GrammarError::RepeatNullable { format, context }
            }
            GrammarError::AmbiguousFirst { left, right, .. } => GrammarError::AmbiguousFirst {
                left,
                right,
                context,
            },
            GrammarError::MultiNullUnion { .. } => GrammarError::MultiNullUnion { context },
            GrammarError::AmbiguousFollow { left, right, .. } => GrammarError::AmbiguousFollow {
                left,
                right,
                context,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Determinations {
    pub is_nullable: bool,
    pub is_productive: bool,
    pub first_set: ByteSet,
    pub should_not_follow_set: ByteSet,
}

impl Determinations {
    /// Additive (i.e. sequencing) identity element
    pub const fn zero() -> Self {
        Self {
            first_set: ByteSet::empty(),
            is_productive: true,
            is_nullable: true,
            should_not_follow_set: ByteSet::empty(),
        }
    }

    /// Multiplicative (i.e. disjunction) identity element
    pub const fn one() -> Self {
        Self {
            first_set: ByteSet::empty(),
            is_productive: false,
            is_nullable: false,
            should_not_follow_set: ByteSet::empty(),
        }
    }

    /// Solve the determination for `F >> G` (sequencing) given the determinations for F and G.
    pub fn merge_seq(self, other: Self) -> Result<Self, GrammarError> {
        if !self.should_not_follow_set.is_disjoint(&other.first_set) {
            return Err(GrammarError::AmbiguousFollow {
                left: self.should_not_follow_set,
                right: other.first_set,
                context: (),
            });
        }
        let is_nullable = self.is_nullable && other.is_nullable;
        let is_productive = self.is_productive && other.is_productive;
        let first_set = {
            let left = if self.is_nullable {
                other.first_set
            } else {
                ByteSet::empty()
            };
            let right = if other.is_productive {
                self.first_set
            } else {
                ByteSet::empty()
            };
            left.union(&right)
        };
        let should_not_follow_set = {
            let left = if other.is_nullable {
                self.should_not_follow_set
            } else {
                ByteSet::empty()
            };
            let right = if self.is_productive {
                other.should_not_follow_set
            } else {
                ByteSet::empty()
            };
            left.union(&right)
        };
        Ok(Self {
            first_set,
            is_nullable,
            is_productive,
            should_not_follow_set,
        })
    }

    pub fn union(self, other: Self) -> Result<Self, GrammarError> {
        if self.is_nullable && other.is_nullable {
            return Err(GrammarError::MultiNullUnion { context: () });
        }
        if !self.first_set.is_disjoint(&other.first_set) {
            return Err(GrammarError::AmbiguousFirst {
                left: self.first_set,
                right: other.first_set,
                context: (),
            });
        }
        let first_set = self.first_set.union(&other.first_set);
        let should_not_follow_set = {
            let left = self.should_not_follow_set;
            let right = other.should_not_follow_set;
            let mut ret = left.union(&right);
            if self.is_nullable {
                ret = ret.union(&other.first_set);
            } else if other.is_nullable {
                ret = ret.union(&self.first_set);
            }
            ret
        };
        Ok(Self {
            first_set,
            is_productive: self.is_productive || other.is_productive,
            is_nullable: self.is_nullable || other.is_nullable,
            should_not_follow_set,
        })
    }
}

impl FormatDecl {
    pub fn first_set(&self, module: &FormatModule) -> Result<ByteSet, GrammarError<Format>> {
        Ok(self.determinations(module)?.first_set)
    }

    pub fn is_nullable(&self, module: &FormatModule) -> Result<bool, GrammarError<Format>> {
        Ok(self.determinations(module)?.is_nullable)
    }

    pub fn determinations(
        &self,
        module: &FormatModule,
    ) -> Result<Determinations, GrammarError<Format>> {
        let mut traversal = Traversal::new(self.fmt_id);
        let ctx = module.get_ctx(self.fmt_id);
        Ok(self
            .format
            .solve_determinations(module, &mut traversal, ctx)?)
    }
}

impl Format {
    /// Returns the first-set, along with `true` if the format is nullable and `false` otherwise
    pub(crate) fn solve_determinations(
        &self,
        module: &FormatModule,
        visited: &mut Traversal,
        ctx: RecurseCtx<'_>,
    ) -> Result<Determinations, GrammarError<Format>> {
        match self {
            Format::ItemVar(level) => {
                let level = *level;
                let ctx = module.get_ctx(level);
                let mut visited = Traversal::new(level);
                module
                    .get_format(level)
                    .solve_determinations(module, &mut visited, ctx)
            }
            Format::RecVar(rec_ix) => {
                let level = ctx.convert_rec_var(*rec_ix).unwrap_or_else(|| {
                    unreachable!("solve_determinations: {ctx:?} has no recursive variable ~{rec_ix} (visited: {:#?})", visited.seen_levels.iter().copied().collect::<Vec<usize>>());
                });
                if visited.insert(level) {
                    let ctx = ctx.enter(*rec_ix);
                    let ret = ctx
                        .get_format()
                        .unwrap()
                        .solve_determinations(module, visited, ctx)?;
                    visited.escape();
                    Ok(ret)
                } else {
                    // REVIEW - loop-breaker fall-back
                    Ok(Determinations::one())
                }
            }
            Format::Byte(set) => Ok(Determinations {
                first_set: *set,
                is_productive: true,
                is_nullable: false,
                should_not_follow_set: ByteSet::empty(),
            }),
            Format::FailWith(..) => Ok(Determinations::one()),
            Format::Compute(..) => Ok(Determinations::zero()),
            // NOTE - EOI cannot be followed with other formats, but such cases are unlikely to occur...
            // REVIEW - if we add a should-not-follow or similar, this might need some thinking...
            Format::EndOfInput => Ok(Determinations::zero()),
            Format::Variant(.., format) => format.solve_determinations(module, visited, ctx),
            Format::Union(formats) => {
                let mut det = Determinations::one();
                for format in formats {
                    let det_format = format.solve_determinations(module, visited, ctx)?;
                    det = det
                        .union(det_format)
                        .map_err(|e| e.add_context(self.clone()))?;
                }
                Ok(det)
            }
            Format::Repeat(format) => {
                let det_format = format.solve_determinations(module, visited, ctx)?;
                if det_format.is_nullable {
                    return Err(GrammarError::RepeatNullable {
                        format: format.as_ref().clone(),
                        context: self.clone(),
                    });
                }
                Ok(Determinations {
                    is_nullable: true,
                    should_not_follow_set: det_format.first_set,
                    ..det_format
                })
            }
            Format::Tuple(formats) | Format::Seq(formats) => {
                let mut det_seq = Determinations::zero();
                for format in formats {
                    let det_format = format.solve_determinations(module, visited, ctx)?;
                    det_seq = det_seq
                        .merge_seq(det_format)
                        .map_err(|e| e.add_context(self.clone()))?;
                }
                Ok(det_seq)
            }
            Format::Maybe(_cond, format) => {
                let det_format = format.solve_determinations(module, visited, ctx)?;
                Ok(Determinations {
                    is_nullable: true,
                    ..det_format
                })
            }
        }
    }
}

pub(crate) use traversal::Traversal;
mod traversal {
    use linked_hash_set::LinkedHashSet;
    /// Semi-mutable traversal state for tracking which format-levels have been seen before and which are novel
    pub struct Traversal {
        pub(crate) orig_level: usize,
        pub(super) seen_levels: LinkedHashSet<usize>,
    }

    impl Traversal {
        pub fn new(orig_level: usize) -> Self {
            Self {
                orig_level,
                seen_levels: LinkedHashSet::new(),
            }
        }

        #[expect(dead_code)]
        /// Returns `true` if the level has not yet been seen (including the original level)
        pub fn is_novel(&self, level: usize) -> bool {
            level != self.orig_level && !self.seen_levels.contains(&level)
        }

        /// Immutable insert that returns the updated state and whether the level was novel
        pub fn insert(&mut self, level: usize) -> bool {
            if level == self.orig_level {
                return false;
            }
            self.seen_levels.insert(level)
        }

        /// Removes the most-recently inserted level, to avoid double-counting between branches rather than
        /// merely witnessing true cycles on a singular path.
        pub fn escape(&mut self) -> Option<usize> {
            self.seen_levels.pop_back()
        }

        pub fn reset(&mut self) {
            self.seen_levels.clear();
        }
    }
}

/// Representation of the right-justified remainder of a [`Format`] we have already
/// consumed some number (possibly 0) bytes of.
#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum PartialFormat<'a> {
    /// `ε`
    Empty,
    /// A full-format followed by a remnant
    Cat(&'a Format, Rc<PartialFormat<'a>>),
    /// A sequence of full-formats followed by a remnant
    Sequence(&'a [Format], Rc<PartialFormat<'a>>),
    /// Repeat the specified format zero or more times before processing a remnant
    Repeat(&'a Format, Rc<PartialFormat<'a>>),
}

impl<'a> PartialFormat<'a> {
    pub(crate) fn solve_determinations(
        self: Rc<Self>,
        module: &'a FormatModule,
        visited: &mut Traversal,
        ctx: RecurseCtx<'a>,
    ) -> Result<Determinations, GrammarError<FormatKind<'a>>> {
        match self.as_ref() {
            PartialFormat::Empty => Ok(Determinations::zero()),
            PartialFormat::Cat(format, remnant) => {
                let det_format = format.solve_determinations(module, visited, ctx)?;
                let det_remnant = remnant.clone().solve_determinations(module, visited, ctx)?;
                det_format
                    .merge_seq(det_remnant)
                    .map_err(|e| e.add_context(self.clone().into()))
            }
            PartialFormat::Sequence(formats, remnant) => {
                let det_formats = {
                    let mut det_seq = Determinations::zero();
                    for format in *formats {
                        let det_format = format.solve_determinations(module, visited, ctx)?;
                        det_seq = det_seq
                            .merge_seq(det_format)
                            .map_err(|e| e.add_context(self.clone()))?;
                    }
                    det_seq
                };
                let det_remnant = remnant.clone().solve_determinations(module, visited, ctx)?;
                det_formats
                    .merge_seq(det_remnant)
                    .map_err(|e| e.add_context(self.clone().into()))
            }
            PartialFormat::Repeat(format, remnant) => {
                let det_format = format.solve_determinations(module, visited, ctx)?;
                if det_format.is_nullable {
                    return Err(GrammarError::RepeatNullable {
                        format: (*format).clone(),
                        context: self.clone().into(),
                    });
                }
                let det_remnant = remnant.clone().solve_determinations(module, visited, ctx)?;
                det_format
                    .merge_seq(det_remnant)
                    .map_err(|e| e.add_context(self.clone().into()))
            }
        }
    }
}

pub struct Interpreter<'a> {
    module: &'a FormatModule,
}

pub type PathTrace = Vec<Choice>;

#[derive(Debug, Clone)]
pub enum Choice {
    UnionBranch(usize),
    RepeatYes,
    RepeatNo,
}

#[derive(Debug)]
pub enum InterpError {
    NoParse,
    DeadEnd {
        start: usize,
        trace: PathTrace,
        byte: u8,
        expects: ByteSet,
    },
    BadEpsilon {
        expects: ByteSet,
    },
    Fail {
        message: crate::Label,
    },
    ExpectsEnd,
}

impl std::fmt::Display for InterpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpError::NoParse => write!(f, "no format to parse"),
            InterpError::ExpectsEnd => write!(f, "EOI parse failed"),
            InterpError::DeadEnd {
                start,
                trace,
                byte,
                expects,
            } => {
                write!(
                    f,
                    "no valid path for byte `{byte:#02x}` ∉ {expects:?} (#{start}: {:?})",
                    trace
                )
            }
            InterpError::BadEpsilon { expects } => {
                write!(
                    f,
                    "no epsilon-move at end-of-input (allowed next-byte: {:?})",
                    expects
                )
            }
            InterpError::Fail { message } => {
                write!(f, "fail: {message}")
            }
        }
    }
}

impl std::error::Error for InterpError {}

impl<'a> Interpreter<'a> {
    pub fn new(module: &'a FormatModule) -> Self {
        Self { module }
    }

    pub fn run_level(
        &self,
        level: usize,
        input: ReadCtxt<'a>,
    ) -> (PathTrace, ReadCtxt<'a>, Option<InterpError>) {
        let mut ctx = self.module.get_ctx(level);
        let format = self.module.get_format(level);
        let mut parse = Rc::new(PartialFormat::Cat(format, Rc::new(PartialFormat::Empty)));
        let mut trace = Vec::new();
        let mut input = input;
        loop {
            if let Some((byte, new_input)) = input.read_byte() {
                let mut visited = Traversal::new(level);
                let result =
                    self.parse_byte_from_partial_format(parse, byte, &mut trace, &mut visited, ctx);
                match result {
                    Err(e) => return (trace, input, Some(e)),
                    Ok((new_ctx, new_parse)) => {
                        input = new_input;
                        parse = new_parse;
                        // NOTE - this is bugged but it should work for our JSON-lite
                        ctx = new_ctx;
                        continue;
                    }
                }
            } else {
                let mut visited = Traversal::new(level);
                match parse
                    .clone()
                    .solve_determinations(self.module, &mut visited, ctx)
                {
                    Err(_) => panic!("failed to solve determinations: {:?}", parse),
                    Ok(dets) => {
                        if dets.is_nullable {
                            return (trace, input, None);
                        } else {
                            return (
                                trace,
                                input,
                                Some(InterpError::BadEpsilon {
                                    expects: dets.first_set,
                                }),
                            );
                        }
                    }
                }
            }
        }
    }

    fn parse_byte_from_format(
        &self,
        f: &'a Format,
        remnant: Rc<PartialFormat<'a>>,
        byte: u8,
        trace: &mut PathTrace,
        visited: &mut Traversal,
        ctx: RecurseCtx<'a>,
    ) -> Result<(RecurseCtx<'a>, Rc<PartialFormat<'a>>), InterpError> {
        match f {
            Format::ItemVar(level) => {
                let new_ctx = self.module.get_ctx(*level);
                let format = &self.module.decls[*level].format;
                self.parse_byte_from_format(format, remnant, byte, trace, visited, new_ctx)
            }
            Format::RecVar(rec_ix) => {
                let level = ctx
                    .convert_rec_var(*rec_ix)
                    .unwrap_or_else(|| panic!("recursion variable not found in {ctx:?}: {rec_ix}"));
                if visited.insert(level) {
                    let format = &self.module.decls[level].format;
                    let new_ctx = ctx.enter(*rec_ix);
                    let (ctx, ret) = self
                        .parse_byte_from_format(format, remnant, byte, trace, visited, new_ctx)?;
                    visited.reset();
                    Ok((ctx, ret))
                } else {
                    unreachable!("left-recursion")
                }
            }
            Format::FailWith(msg) => {
                return Err(InterpError::Fail {
                    message: msg.clone(),
                });
            }
            Format::EndOfInput => return Err(InterpError::ExpectsEnd),
            Format::Byte(bs) => {
                if bs.contains(byte) {
                    Ok((ctx, remnant))
                } else {
                    Err(InterpError::DeadEnd {
                        start: visited.orig_level,
                        trace: trace.clone(),
                        byte,
                        expects: *bs,
                    })
                }
            }
            Format::Compute(_) => {
                self.parse_byte_from_partial_format(remnant, byte, trace, visited, ctx)
            }
            Format::Variant(_, format) => {
                self.parse_byte_from_format(format, remnant, byte, trace, visited, ctx)
            }
            Format::Union(branches) => {
                let mut accepts = ByteSet::empty();
                for (ix, branch) in branches.iter().enumerate() {
                    let dets = branch
                        .solve_determinations(self.module, visited, ctx)
                        .unwrap();
                    if dets.first_set.contains(byte) {
                        trace.push(Choice::UnionBranch(ix));
                        return self
                            .parse_byte_from_format(branch, remnant, byte, trace, visited, ctx);
                    } else {
                        accepts = accepts.union(&dets.first_set);
                    }
                }
                Err(InterpError::DeadEnd {
                    start: visited.orig_level,
                    trace: trace.clone(),
                    byte,
                    expects: accepts,
                })
            }
            Format::Repeat(format) => {
                let dets = format
                    .solve_determinations(self.module, visited, ctx)
                    .unwrap();
                if dets.first_set.contains(byte) {
                    trace.push(Choice::RepeatYes);
                    let new_remnant = self.parse_byte_from_format(
                        format,
                        Rc::new(PartialFormat::Repeat(format, remnant)),
                        byte,
                        trace,
                        visited,
                        ctx,
                    )?;
                    Ok(new_remnant)
                } else {
                    trace.push(Choice::RepeatNo);
                    self.parse_byte_from_partial_format(remnant, byte, trace, visited, ctx)
                }
            }
            Format::Seq(formats) | Format::Tuple(formats) => {
                let (format, rest) = formats.split_first().unwrap();
                self.parse_byte_from_format(
                    format,
                    Rc::new(PartialFormat::Sequence(rest, remnant)),
                    byte,
                    trace,
                    visited,
                    ctx,
                )
            }
            Format::Maybe(expr, format) => {
                let present = expr.eval().unwrap_bool();
                if present {
                    self.parse_byte_from_format(format, remnant, byte, trace, visited, ctx)
                } else {
                    self.parse_byte_from_partial_format(remnant, byte, trace, visited, ctx)
                }
            }
        }
    }

    fn parse_byte_from_partial_format(
        &self,
        parse: Rc<PartialFormat<'a>>,
        byte: u8,
        trace: &mut PathTrace,
        visited: &mut Traversal,
        ctx: RecurseCtx<'a>,
    ) -> Result<(RecurseCtx<'a>, Rc<PartialFormat<'a>>), InterpError> {
        match parse.as_ref() {
            PartialFormat::Empty => return Err(InterpError::NoParse),
            PartialFormat::Cat(f, remnant) => {
                self.parse_byte_from_format(f, remnant.clone(), byte, trace, visited, ctx)
            }
            PartialFormat::Sequence(formats, remnant) => {
                if formats.is_empty() {
                    self.parse_byte_from_partial_format(remnant.clone(), byte, trace, visited, ctx)
                } else {
                    let (format, rest) = formats.split_first().unwrap();
                    self.parse_byte_from_format(
                        format,
                        Rc::new(PartialFormat::Sequence(rest, remnant.clone())),
                        byte,
                        trace,
                        visited,
                        ctx,
                    )
                }
            }
            PartialFormat::Repeat(format, remnant) => {
                let dets = format
                    .solve_determinations(self.module, visited, ctx)
                    .unwrap();
                if !dets.first_set.contains(byte) {
                    trace.push(Choice::RepeatNo);
                    self.parse_byte_from_partial_format(remnant.clone(), byte, trace, visited, ctx)
                } else {
                    trace.push(Choice::RepeatYes);
                    self.parse_byte_from_format(
                        format,
                        Rc::new(PartialFormat::Repeat(format, remnant.clone())),
                        byte,
                        trace,
                        visited,
                        ctx,
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_set() {}
}

/*
/// A forest of [`FMatchTree`]s, one for each level of a [`FormatModule`] (with directly corresponding indices)
#[derive(Debug)]
pub struct MatchForest<'a> {
    module: &'a FormatModule,
    trees: Vec<Rc<OnceCell<FMatchTree>>>,
}

impl<'a> MatchForest<'a> {
    /// Initializes a new [`MatchForest`] from a [`FormatModule`].
    pub fn new(module: &'a FormatModule) -> Self {
        let trees = (0..module.decls.len())
            .map(|_| Rc::new(OnceCell::new()))
            .collect();
        Self { module, trees }
    }

    /// Returns a shared-pointer to the [`FMatchTree`] at `level`.
    #[must_use]
    pub(crate) fn get_level(&self, level: usize) -> Rc<OnceCell<FMatchTree>> {
        assert!(level < self.module.decls.len());
        self.trees[level].clone()
    }

    /// Populates (if it is not already populated) and returns a shared-pointer to the [`FMatchTree`] at `level`.
    fn populate_level(&self, level: usize) -> Rc<OnceCell<FMatchTree>> {
        assert!(level < self.module.decls.len());

        let ret = self.trees[level].clone();

        const LIMIT: usize = 40;

        if self.trees[level].get().is_none() {
            let ctx = self.module.get_ctx(level);
            let tree = FMatchTree::grow(&self, level, LIMIT, ctx);
            ret.set(tree);
        }

        ret
    }
}

#[derive(Debug, Clone)]
enum FollowSet<'a> {
    Delayed(usize, Rc<PartialFormat<'a>>),
    Branch(ByteSet, Rc<PartialFormat<'a>>),
}


#[derive(Debug)]
pub struct MatchBranch<'a> {
    accept: bool,
    branches: Vec<FollowSet<'a>>,
}

impl<'a> MatchBranch<'a> {
    pub fn reject() -> MatchBranch<'a> {
        Self {
            accept: false,
            branches: vec![],
        }
    }

    pub fn accept() -> MatchBranch<'a> {
        Self {
            accept: true,
            branches: vec![],
        }
    }

    fn branch(bs: ByteSet, remnant: Rc<PartialFormat<'a>>) -> MatchBranch<'a> {
        Self {
            accept: false,
            branches: vec![FollowSet::Branch(bs, remnant)],
        }
    }

    fn delayed(
        forest: &'a MatchForest<'a>,
        level: usize,
        remnant: Rc<PartialFormat<'a>>,
    ) -> MatchBranch<'a> {
        Self {
            accept: false,
            branches: vec![FollowSet::Delayed(level, remnant)],
        }
    }
    pub fn from_level(forest: &'a MatchForest<'a>, level: usize) -> MatchBranch<'a> {
        let decl = &forest.module.decls[level];
        let ctx = forest.module.get_ctx(level);
        MatchBranch::from_format(forest, &decl.format, Rc::new(PartialFormat::Empty), ctx)
    }

    fn from_partial_format(
        forest: &'a MatchForest<'a>,
        remnant: Rc<PartialFormat<'a>>,
        ctx: RecurseCtx<'a>,
    ) -> MatchBranch<'a> {
        match remnant.as_ref() {
            PartialFormat::Empty => Self::accept(),
            PartialFormat::Cat(format, remnant) => {
                Self::from_format(forest, *format, remnant.clone(), ctx)
            }
            PartialFormat::Sequence(formats, remnant) => {
                let remnant = remnant.clone();
                match formats.split_first() {
                    None => Self::from_partial_format(forest, remnant, ctx),
                    Some((f, fs)) => Self::from_format(
                        forest,
                        f,
                        Rc::new(PartialFormat::Sequence(fs, remnant)),
                        ctx,
                    ),
                }
            }
            PartialFormat::Repeat(format, remnant0) => {
                let tree = Self::from_partial_format(forest, remnant0.clone(), ctx);
                let remnant1 = remnant.clone();
                tree.union(Self::from_format(forest, *format, remnant1, ctx))
            }
            PartialFormat::Union(remnant1, remnant2) => {
                let tree1 = Self::from_partial_format(forest, remnant1.clone(), ctx);
                let tree2 = Self::from_partial_format(forest, remnant2.clone(), ctx);
                tree1.union(tree2)
            }
        }
    }

    fn union_branch(&mut self, mut bs: ByteSet, remnant: Rc<PartialFormat<'a>>) {
        let mut branches = Vec::new();
        for follow_set in self.branches.iter_mut() {
            match follow_set {
                FollowSet::Delayed(_level, _remnant) => {
                    // FIXME - how do we model union with delayed FollowSets?
                    todo!();
                }
                FollowSet::Branch(bs0, remnant0) => {
                    let common = bs0.intersection(&bs);
                    if !common.is_empty() {
                        let orig = bs0.difference(&bs);
                        if !orig.is_empty() {
                            branches.push(FollowSet::Branch(orig, remnant0.clone()));
                        }
                        *bs0 = common;
                        *remnant0 =
                            Rc::new(PartialFormat::Union(remnant0.clone(), remnant.clone()));
                        bs = bs.difference(bs0);
                    }
                }
            }
        }
        if !bs.is_empty() {
            self.branches.push(FollowSet::Branch(bs, remnant));
        }
        self.branches.append(&mut branches);
    }

    fn union_delayed(&mut self, level: usize, remnant: Rc<PartialFormat<'a>>) {
        let fset = FollowSet::Delayed(level, remnant);
        todo!()
    }

    fn union(mut self, other: MatchBranch<'a>) -> MatchBranch<'a> {
        self.accept = self.accept || other.accept;
        for follow_set in other.branches {
            match &follow_set {
                FollowSet::Delayed(level, remnant) => {
                    self.union_delayed(*level, remnant.clone());
                }
                FollowSet::Branch(bs, remnant) => {
                    self.union_branch(*bs, remnant.clone());
                }
            }
        }
        self
    }

    fn from_format(
        forest: &'a MatchForest<'a>,
        f: &'a Format,
        remnant: Rc<PartialFormat<'a>>,
        ctx: RecurseCtx<'a>,
    ) -> MatchBranch<'a> {
        match f {
            Format::ItemVar(level) => {
                let level = *level;
                let format = forest.module.get_format(level);
                let ctx = forest.module.get_ctx(level);
                Self::from_format(forest, format, remnant, ctx)
            }
            Format::RecVar(rec_ix) => {
                let RecurseCtx::Recurse { span, .. } = ctx else {
                    unreachable!(
                        "level {} is unexpectedly recursive (rec-var without ctx)",
                        rec_ix
                    )
                };
                let level = span.index(*rec_ix);
                Self::delayed(forest, level, remnant)
            }
            Format::FailWith(..) => Self::reject(),
            Format::EndOfInput => Self::accept(),
            Format::Byte(bs) => Self::branch(*bs, remnant),
            Format::Compute(..) => Self::from_partial_format(forest, remnant, ctx),
            Format::Variant(_, format) => Self::from_format(forest, format, remnant, ctx),
            Format::Union(branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_format(forest, f, remnant.clone(), ctx));
                }
                tree
            }
            Format::Repeat(format) => todo!(),
            Format::Seq(formats) => todo!(),
            Format::Tuple(formats) => todo!(),
            Format::Maybe(expr, format) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct FMatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, FMatchTree)>,
}

impl FMatchTree {
    fn grow<'a>(arg: &'a MatchForest<'a>, level: usize, limit: usize, ctx: RecurseCtx<'a>) -> Self {
        todo!()
    }
}
*/
