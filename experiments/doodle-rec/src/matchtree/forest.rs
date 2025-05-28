use std::{collections::HashSet, rc::Rc};

use doodle::{prelude::ByteSet, read::ReadCtxt};

use crate::{Format, FormatDecl, FormatId, FormatModule, RecId, RecurseCtx};

#[derive(Debug)]
pub enum GrammarError {
    LeftRecursion { top: FormatId, cycle: Vec<FormatId> },
    RepeatNullable { format: Format },
    AmbiguousFirst { left: ByteSet, right: ByteSet },
    MultiNullUnion,
    AmbiguousFollow { left: ByteSet, right: ByteSet },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Determinations {
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
            return Err(GrammarError::MultiNullUnion);
        }
        if !self.first_set.is_disjoint(&other.first_set) {
            return Err(GrammarError::AmbiguousFirst {
                left: self.first_set,
                right: other.first_set,
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

impl std::fmt::Display for GrammarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrammarError::LeftRecursion { top, cycle } => {
                write!(
                    f,
                    "left-recursion found in format: #{} -> {:#?}",
                    top, cycle,
                )
            }
            GrammarError::RepeatNullable { format } => {
                write!(f, "repeat of nullable format: {:?}", format)
            }
            GrammarError::MultiNullUnion => {
                write!(f, "multiple nullable formats in union")
            }
            GrammarError::AmbiguousFirst { left, right } => {
                write!(
                    f,
                    "ambiguity introduced by union of non-disjoint first sets: {:?} <|> {:?}",
                    left, right
                )
            }
            GrammarError::AmbiguousFollow { left, right } => {
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

impl FormatDecl {
    pub fn first_set(&self, module: &FormatModule) -> Result<ByteSet, GrammarError> {
        Ok(self.determinations(module)?.first_set)
    }

    pub fn is_nullable(&self, module: &FormatModule) -> Result<bool, GrammarError> {
        Ok(self.determinations(module)?.is_nullable)
    }

    pub(crate) fn determinations(
        &self,
        module: &FormatModule,
    ) -> Result<Determinations, GrammarError> {
        let ctx = module.get_ctx(self.fmt_id);
        Ok(self
            .format
            .solve_determinations(module, ctx)?)
    }
}

impl Format {
    fn solve_determinations_sequence(
        formats: &[Format],
        module: &FormatModule,
        ctx: RecurseCtx,
    ) -> Result<Determinations, GrammarError> {
        let mut det_seq = Determinations::zero();
        for format in formats {
            let det_format = format.solve_determinations(module, ctx)?;
            det_seq = det_seq.merge_seq(det_format)?;
        }
        Ok(det_seq)
    }

    /// Returns the first-set, along with `true` if the format is nullable and `false` otherwise
    fn solve_determinations(
        &self,
        module: &FormatModule,
        ctx: RecurseCtx<'_>,
    ) -> Result<Determinations, GrammarError> {
        match self {
            Format::ItemVar(level) => {
                let level = *level;
                let ctx = module.get_ctx(level);
                module
                    .get_format(level)
                    .solve_determinations(module,  ctx)
            }
            Format::RecVar(rec_ix) => {
                let ctx = ctx.enter(*rec_ix).0;
                let ret = ctx
                    .get_format()
                    .unwrap()
                    .solve_determinations(module, ctx)?;
                Ok(ret)
            }
            Format::Byte(set) => {
                Ok(Determinations {
                    first_set: *set,
                    is_productive: true,
                    is_nullable: false,
                    should_not_follow_set: ByteSet::empty(),
                })
            },
            Format::FailWith(..) => Ok(Determinations::one()),
            Format::Compute(..) => Ok(Determinations::zero()),
            // NOTE - EOI cannot be followed with other formats, but such cases are unlikely to occur...
            // REVIEW - if we add a should-not-follow or similar, this might need some thinking...
            Format::EndOfInput => Ok(Determinations::zero()),
            Format::Variant(.., format) => format.solve_determinations(module, ctx),
            Format::Union(formats) => {
                let mut det = Determinations::one();
                for format in formats {
                    let det_format = format.solve_determinations(module,  ctx)?;
                    det = det.union(det_format)?;
                }
                Ok(det)
            }
            Format::Repeat(format) => {
                let det_format = format.solve_determinations(module, ctx)?;
                if det_format.is_nullable {
                    return Err(GrammarError::RepeatNullable {
                        format: format.as_ref().clone(),
                    });
                }
                Ok(Determinations {
                    is_nullable: true,
                    should_not_follow_set: det_format.first_set,
                    ..det_format
                })
            }
            Format::Tuple(formats) | Format::Seq(formats) => {
                Format::solve_determinations_sequence(formats, module, ctx)
            }
            Format::Maybe(_cond, format) => {
                let det_format = format.solve_determinations(module, ctx)?;
                Ok(Determinations {
                    is_nullable: true,
                    ..det_format
                })
            }
        }
    }
}

use traversal::Traversal;
mod traversal {
    use linked_hash_set::LinkedHashSet;
    /// Semi-mutable traversal state for tracking which format-levels have been seen before and which are novel
    pub struct Traversal {
        pub(super) orig_level: usize,
        pub(super) seen_levels: LinkedHashSet<usize>,
    }

    impl Traversal {
        pub fn new(orig_level: usize) -> Self {
            Self {
                orig_level,
                seen_levels: LinkedHashSet::new(),
            }
        }

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
        pub fn escape(&mut self) {
            let Some(_) = self.seen_levels.pop_back() else {
                panic!("traversal stack is empty");
            };
        }

        pub fn reset(&mut self) {
            self.seen_levels.clear();
        }
    }
}

/// Representation of the right-justified remainder of a [`Format`] we have already
/// consumed some number (possibly 0) bytes of.
#[derive(PartialEq, Eq, Hash, Debug)]
enum PartialFormat<'a> {
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
    fn solve_determinations(
        &self,
        module: &'a FormatModule,
        ctx: RecurseCtx<'a>,
    ) -> Result<Determinations, GrammarError> {
        match self {
            PartialFormat::Empty => Ok(Determinations::zero()),
            PartialFormat::Cat(format, remnant) => {
                let det_format = format.solve_determinations(module, ctx)?;
                let det_remnant = remnant.solve_determinations(module, ctx)?;
                det_format.merge_seq(det_remnant)
            }
            PartialFormat::Sequence(formats, remnant) => {
                let det_formats =
                    Format::solve_determinations_sequence(formats, module, ctx)?;
                let det_remnant = remnant.solve_determinations(module, ctx)?;
                det_formats.merge_seq(det_remnant)
            }
            PartialFormat::Repeat(format, remnant) => {
                let det_format = format.solve_determinations(module, ctx)?;
                if det_format.is_nullable {
                    return Err(GrammarError::RepeatNullable {
                        format: (*format).clone(),
                    });
                }
                let det_remnant = remnant.solve_determinations(module, ctx)?;
                det_format.merge_seq(det_remnant)
            }
        }
    }
}

pub struct Interpreter<'a> {
    module: &'a FormatModule,
}

pub type PathTrace = Vec<Option<usize>>;

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
}

impl std::fmt::Display for InterpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpError::NoParse => write!(f, "no format to parse"),
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

impl<'a> Interpreter<'a> {
    pub fn new(module: &'a FormatModule) -> Self {
        Self { module }
    }

    pub fn run_level(
        &self,
        level: usize,
        input: ReadCtxt<'a>,
    ) -> (PathTrace, ReadCtxt<'a>, Option<InterpError>) {
        let ctx = self.module.get_ctx(level);
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
                    Ok(new_parse) => {
                        input = new_input;
                        parse = new_parse;
                        continue;
                    }
                }
            } else {
                match parse.solve_determinations(self.module, ctx) {
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

    pub fn parse_byte_from_format(
        &self,
        f: &'a Format,
        remnant: Rc<PartialFormat<'a>>,
        byte: u8,
        trace: &mut PathTrace,
        visited: &mut Traversal,
        ctx: RecurseCtx<'a>,
    ) -> Result<Rc<PartialFormat<'a>>, InterpError> {
        match f {
            Format::ItemVar(level) => {
                let new_ctx = self.module.get_ctx(*level);
                let format = &self.module.decls[*level].format;
                self.parse_byte_from_format(format, remnant, byte, trace, visited, new_ctx)
            }
            Format::RecVar(rec_ix) => {
                let level = ctx.convert_rec_var(*rec_ix);
                if visited.insert(level) {
                    let format = &self.module.decls[level].format;
                    let (new_ctx, _) = ctx.enter(*rec_ix);
                    let ret = self
                        .parse_byte_from_format(format, remnant, byte, trace, visited, new_ctx)?;
                    visited.reset();
                    Ok(ret)
                } else {
                    unreachable!("left-recursion")
                }
            }
            Format::FailWith(msg) => {
                return Err(InterpError::Fail {
                    message: msg.clone(),
                });
            }
            Format::EndOfInput => todo!(),
            Format::Byte(bs) => {
                if bs.contains(byte) {
                    Ok(remnant)
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
                        .solve_determinations(self.module, ctx)
                        .unwrap();
                    if dets.first_set.contains(byte) {
                        trace.push(Some(ix));
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
                    .solve_determinations(self.module, ctx)
                    .unwrap();
                if dets.first_set.contains(byte) {
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

    pub fn parse_byte_from_partial_format(
        &self,
        parse: Rc<PartialFormat<'a>>,
        byte: u8,
        trace: &mut PathTrace,
        visited: &mut Traversal,
        ctx: RecurseCtx<'a>,
    ) -> Result<Rc<PartialFormat<'a>>, InterpError> {
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
                    .solve_determinations(self.module, ctx)
                    .unwrap();
                if !dets.first_set.contains(byte) {
                    self.parse_byte_from_partial_format(remnant.clone(), byte, trace, visited, ctx)
                } else {
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
