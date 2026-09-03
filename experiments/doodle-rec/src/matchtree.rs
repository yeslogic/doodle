use std::{collections::HashSet, rc::Rc, vec};

use crate::{Format, FormatModule, RecurseCtx};
use doodle::{byte_set::ByteSet, read::ReadCtxt};

pub mod determinations;
use determinations::{Entry, Traversal};
// pub use forest::MatchForest;

#[derive(Clone, Debug)]
pub(crate) struct MatchTreeStep<'a> {
    accept: bool,
    branches: Vec<(ByteSet, Rc<Next<'a>>)>,
}

impl<'a> MatchTreeStep<'a> {
    pub fn accept() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: true,
            branches: vec![],
        }
    }

    pub fn reject() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![],
        }
    }

    fn branch(bs: ByteSet, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![(bs, next)],
        }
    }

    /// Modifies a `MatchTreeStep` in place, so that it will accept a new branch given by the argument values.
    fn union_branch(&mut self, mut bs: ByteSet, next: Rc<Next<'a>>) {
        let mut branches = Vec::new();
        for (bs0, next0) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    branches.push((orig, next0.clone()));
                }
                *bs0 = common;
                *next0 = Rc::new(Next::Union(next0.clone(), next.clone()));
                bs = bs.difference(bs0);
            }
        }
        if !bs.is_empty() {
            self.branches.push((bs, next));
        }
        self.branches.append(&mut branches);
    }

    /// Combines two `MatchTreeSteps` into their logical union
    fn union(mut self, other: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        self.accept = self.accept || other.accept;
        for (bs, next) in other.branches {
            self.union_branch(bs, next);
        }
        self
    }

    /// Returns a modified version of `self` that rejects any input that is not
    /// accepted by `peek`.
    fn peek(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        if peek.accept {
            // can ignore peek as it has already accepted
        } else if self.accept {
            // can ignore self as it has already accepted
            self.accept = peek.accept;
            self.branches = peek.branches;
        } else {
            // take the intersection of peek and self branches
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches {
                for (bs2, next2) in &peek.branches {
                    let bs = bs1.intersection(bs2);
                    if !bs.is_empty() {
                        let next = Rc::new(Next::Peek(next1.clone(), next2.clone()));
                        branches.push((bs, next));
                    }
                }
            }
            self.branches = branches;
        }
        self
    }

    /// Returns a modified version of `self` that rejects any input that is
    /// accepted by `peek`.
    fn peek_not(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        if peek.accept {
            self.accept = false;
            self.branches = Vec::new();
        } else {
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches.into_iter() {
                let mut diff = bs1;
                for (bs2, next2) in &peek.branches {
                    let common = bs1.intersection(bs2);
                    if !common.is_empty() {
                        let next = Rc::new(Next::PeekNot(next1.clone(), next2.clone()));
                        branches.push((common, next));
                    }
                    diff = diff.difference(bs2);
                }
                if !diff.is_empty() {
                    branches.push((diff, next1.clone()));
                }
            }
            self.branches = branches;
        }
        self
    }

    /// Constructs a [MatchTreeStep] that accepts a given tuple of sequential formats, with a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_sequential(
        module: &'a FormatModule,
        fields: &'a [Format],
        next: Rc<Next<'a>>,
        ctx: RecurseCtx<'a>,
        visited: &mut Traversal,
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::from_next(module, next, visited),
            Some((f, fs)) => Self::from_format(
                module,
                f,
                Rc::new(Next::Sequence(fs, ctx, next)),
                ctx,
                visited,
            ),
        }
    }

    /// Constructs a [MatchTreeStep] from a [`Next`].
    ///
    /// `visited` tracks which format-levels have already been entered via a [`Next::DelayRef`]
    /// without an intervening byte having been consumed, i.e. within a single top-level call to
    /// this function. Re-entering an already-visited level in that state means the underlying
    /// grammar is left-recursive; see the `Next::DelayRef` arm below.
    ///
    /// Unlike [`Self::from_format`], this takes no ambient `ctx`: every non-`Empty`/`Union`
    /// variant of [`Next`] carries (or, for `DelayRef`, can recover via `FormatModule::get_ctx`)
    /// the `RecurseCtx` that was actually in effect when it was constructed, which is what must
    /// be used here - a `Next` value routinely survives across a lookahead-depth boundary (see
    /// `MatchTreeLevel::grow`), by which point any single ambient ctx passed into this call would
    /// no longer necessarily match the context each stored `Next` was built under.
    fn from_next(
        module: &'a FormatModule,
        next: Rc<Next<'a>>,
        visited: &mut Traversal,
    ) -> MatchTreeStep<'a> {
        match next.as_ref() {
            Next::Empty => Self::accept(),
            Next::Union(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone(), visited);
                let tree2 = Self::from_next(module, next2.clone(), visited);
                tree1.union(tree2)
            }
            Next::Cat(f, ctx, next) => {
                MatchTreeStep::<'a>::from_format(module, *f, next.clone(), *ctx, visited)
            }
            Next::Sequence(fields, ctx, next) => {
                let next = next.clone();
                match fields.split_first() {
                    None => Self::from_next(module, next, visited),
                    Some((f, fs)) => Self::from_format(
                        module,
                        f,
                        Rc::new(Next::Sequence(fs, *ctx, next)),
                        *ctx,
                        visited,
                    ),
                }
            }
            Next::Repeat(a, ctx, next0) => {
                let tree = MatchTreeStep::<'a>::from_next(module, next0.clone(), visited);
                let next1 = next.clone();
                tree.union(MatchTreeStep::<'a>::from_format(
                    module, *a, next1, *ctx, visited,
                ))
            }
            Next::RepeatCount(n, a, ctx, next0) => {
                let n = *n;
                let next = next0.clone();
                if n > 0 {
                    Self::from_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatCount(n - 1, *a, *ctx, next)),
                        *ctx,
                        visited,
                    )
                } else {
                    Self::from_next(module, next, visited)
                }
            }
            Next::RepeatMax(n, a, ctx, next0) => {
                let n = *n;
                if n == 0 {
                    Self::from_next(module, next0.clone(), visited)
                } else {
                    let tree0 = MatchTreeStep::<'a>::from_next(module, next0.clone(), visited);
                    tree0.union(MatchTreeStep::<'a>::from_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatMax(n - 1, *a, *ctx, next0.clone())),
                        *ctx,
                        visited,
                    ))
                }
            }
            Next::RepeatBetween(min, max, a, ctx, next0) => {
                let (min, max) = (*min, *max);
                if min == max {
                    let next1 = Rc::new(Next::RepeatCount(min, *a, *ctx, next0.clone()));
                    Self::from_next(module, next1, visited)
                } else if min > 0 {
                    Self::from_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatBetween(
                            min - 1,
                            max - 1,
                            *a,
                            *ctx,
                            next0.clone(),
                        )),
                        *ctx,
                        visited,
                    )
                } else {
                    let next1 = Rc::new(Next::RepeatMax(max, *a, *ctx, next0.clone()));
                    Self::from_next(module, next1, visited)
                }
            }
            Next::Peek(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone(), visited);
                let tree2 = Self::from_next(module, next2.clone(), visited);
                tree1.peek(tree2)
            }
            Next::PeekNot(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone(), visited);
                let tree2 = Self::from_next(module, next2.clone(), visited);
                tree1.peek_not(tree2)
            }
            Next::Slice(count, inside, next0) => {
                Self::from_slice(module, *count, inside.clone(), next0.clone(), visited)
            }
            Next::DelayRef(level, next) => {
                if visited.insert(*level) == Entry::Novel {
                    let ctx = module.get_ctx(*level);
                    let format = module.get_format(*level);
                    let step = Self::from_format(module, format, next.clone(), ctx, visited);
                    visited.escape();
                    step
                } else {
                    // Left-recursive cycle: this format-level was already being expanded
                    // with zero bytes consumed in between. Bail out locally rather than
                    // looping/overflowing the stack; a dedicated static validation pass is
                    // responsible for surfacing this to the user as GrammarError::LeftRecursion.
                    Self::reject()
                }
            }
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a fixed-count repetition of a given format, with
    /// a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_repeat_count(
        module: &'a FormatModule,
        n: usize,
        format: &'a Format,
        ctx: RecurseCtx<'a>,
        next: Rc<Next<'a>>,
        visited: &mut Traversal,
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            Self::from_format(
                module,
                format,
                Rc::new(Next::RepeatCount(n - 1, format, ctx, next)),
                ctx,
                visited,
            )
        } else {
            Self::from_next(module, next, visited)
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a repetition whose count is bounded above and
    /// below, with a trailing sequence of partially-consumed formats ([`Next`]s).
    ///
    /// Presupposes that the invariant `max >= min` is upheld.
    fn from_repeat_between(
        module: &'a FormatModule,
        min_max: (usize, usize),
        format: &'a Format,
        ctx: RecurseCtx<'a>,
        next: Rc<Next<'a>>,
        visited: &mut Traversal,
    ) -> MatchTreeStep<'a> {
        let (min, max) = min_max;
        assert!(
            min <= max,
            "min-max pair ({min}, {max}) incoherent (min > max)"
        );
        if min == max {
            Self::from_repeat_count(module, min, format, ctx, next, visited)
        } else if min > 0 {
            Self::from_format(
                module,
                format,
                Rc::new(Next::RepeatBetween(min - 1, max - 1, format, ctx, next)),
                ctx,
                visited,
            )
        } else {
            Self::from_next(
                module,
                Rc::new(Next::RepeatMax(max, format, ctx, next)),
                visited,
            )
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a `count`-byte slice restricting `inner` (the
    /// slice's own not-yet-fully-consumed continuation, starting as `Next::Cat(f, ctx, Empty)` -
    /// `Empty`, not the outer `next`, since bytes left over within the slice are skipped, not
    /// passed on to whatever's inside), followed by `next` (what comes after the whole slice).
    fn from_slice(
        module: &'a FormatModule,
        count: usize,
        inner: Rc<Next<'a>>,
        next: Rc<Next<'a>>,
        visited: &mut Traversal,
    ) -> MatchTreeStep<'a> {
        if count > 0 {
            let mut tree = Self::from_next(module, inner, visited);
            tree.accept = false;
            if tree.branches.is_empty() {
                // `inner` doesn't itself branch on any byte (e.g. it's already fully satisfied,
                // or otherwise indifferent) - the slice's own byte-budget isn't used up yet
                // though, so any byte here is consumed by the slice regardless, with nothing
                // further required of it.
                let next = Rc::new(Next::Slice(count - 1, Rc::new(Next::Empty), next.clone()));
                tree.branches.push((ByteSet::full(), next));
            } else {
                for (_bs, inside) in tree.branches.iter_mut() {
                    *inside = Rc::new(Next::Slice(count - 1, inside.clone(), next.clone()));
                }
            }
            tree
        } else {
            Self::from_next(module, next, visited)
        }
    }

    fn from_format(
        module: &'a FormatModule,
        f: &'a Format,
        next: Rc<Next<'a>>,
        ctx: RecurseCtx<'a>,
        visited: &mut Traversal,
    ) -> MatchTreeStep<'a> {
        match f {
            Format::ItemVar(level) => {
                let ctx = module.get_ctx(*level);
                let mut visited = Traversal::new(*level);
                Self::from_format(module, module.get_format(*level), next, ctx, &mut visited)
            }
            Format::FailWith(_) => Self::reject(),
            Format::EndOfInput => Self::accept(),
            Format::Byte(bs) => Self::branch(*bs, next),
            Format::Variant(_label, f) => Self::from_format(module, f, next, ctx, visited),
            // `UnionNondet` never actually gets compiled via `MatchTree` (see `Compiler::
            // compile_format`'s arm) - this treats it identically to `Union` purely for the
            // benefit of *other* code that needs to know what bytes this format could start
            // with for its own lookahead purposes (e.g. a sibling field's `depends_on_next`
            // check, or an outer `Union` containing this as one of its own branches).
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone(), ctx, visited));
                }
                tree
            }
            Format::Seq(fields) | Format::Tuple(fields) => {
                Self::from_sequential(module, fields, next, ctx, visited)
            }
            Format::Repeat(a) => {
                let tree = Self::from_next(module, next.clone(), visited);
                tree.union(Self::from_format(
                    module,
                    a,
                    Rc::new(Next::Repeat(a, ctx, next)),
                    ctx,
                    visited,
                ))
            }
            Format::Maybe(_expr, a) => {
                let tree_some = Self::from_format(module, a, next.clone(), ctx, visited);
                let tree_none = Self::from_next(module, next, visited);
                tree_some.union(tree_none)
            }
            Format::Compute(_expr) => Self::from_next(module, next, visited),
            Format::RecVar(rec_ix) => {
                let level = ctx.convert_rec_var(*rec_ix).unwrap();
                let next = Rc::new(Next::DelayRef(level, next));
                Self::from_next(module, next, visited)
            }
            Format::RepeatCount(count, a) => {
                Self::from_repeat_count(module, count.eval_usize(), a, ctx, next, visited)
            }
            Format::RepeatBetween(min, max, a) => {
                let (min, max) = (min.eval_usize(), max.eval_usize());
                assert!(
                    min <= max,
                    "incoherent RepeatBetween: min {min} > max {max}"
                );
                Self::from_repeat_between(module, (min, max), a, ctx, next, visited)
            }
            // The peek target is examined via a fresh `Next::Empty` continuation, never the
            // ambient `next` - Peek/PeekNot are pure lookahead assertions that don't consume
            // input, so what comes after is irrelevant to whether the peek target matches here.
            // `visited` *is* shared with the ambient traversal (not reset), so a left-recursive
            // cycle reached only through a `Peek`/`PeekNot` target (zero bytes consumed either
            // way) is still caught by the same guard as ordinary left recursion.
            Format::Peek(a) => {
                let tree = Self::from_next(module, next.clone(), visited);
                let peek = Self::from_format(module, a, Rc::new(Next::Empty), ctx, visited);
                tree.peek(peek)
            }
            Format::PeekNot(a) => {
                let tree = Self::from_next(module, next.clone(), visited);
                let peek = Self::from_format(module, a, Rc::new(Next::Empty), ctx, visited);
                tree.peek_not(peek)
            }
            Format::Slice(count, a) => {
                let inner = Rc::new(Next::Cat(a, ctx, Rc::new(Next::Empty)));
                Self::from_slice(module, count.eval_usize(), inner, next, visited)
            }
            // Opaque to lookahead entirely, matching real doodle's own deliberately-inherited
            // limitation: an absolute jump doesn't consume any *outer* bytes (match_bounds is
            // exact(0)), so there's nothing here for lookahead to examine anyway - but this also
            // means a `WithRelativeOffset` branch inside a `Union` always looks like it
            // unconditionally matches, which can mask genuine ambiguity with its sibling
            // branches. Not attempted here, same as upstream.
            Format::WithRelativeOffset(..) => Self::accept(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MatchTreeLevel<'a> {
    accept: Option<usize>,
    branches: Vec<(ByteSet, LevelBranch<'a>)>,
}

impl<'a> MatchTreeLevel<'a> {
    /// Constructs a `MatchTreeLevel` that unconditionally rejects all inputs without branching.
    fn reject() -> MatchTreeLevel<'a> {
        MatchTreeLevel {
            accept: None,
            branches: vec![],
        }
    }

    /// Attempts to modify `self` such that `index` is marked as the unique index of the accepting format.
    ///
    /// Returns `Err(())` if a different index was already marked as accepting, and `Ok(())` otherwise.
    fn merge_accept(&mut self, index: usize) -> Result<(), ()> {
        match self.accept {
            None => {
                self.accept = Some(index);
                Ok(())
            }
            Some(i) if i == index => Ok(()),
            Some(_) => Err(()),
        }
    }

    /// Adds a new branch to `self` using a predicate byte-set and its associated follow-set,
    fn merge_branch(&mut self, index: usize, mut bs: ByteSet, next: Rc<Next<'a>>) {
        let mut new_branches = Vec::new();
        // For each bs0, nexts in the extant branches of `self`:
        for (bs0, nexts) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            // If bs and bs0 are not disjoint:
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    // 1. Enqueue a branch predicated on `bs0 - bs` with an inherited follow-set
                    new_branches.push((orig, nexts.clone()));
                }
                // 2. Leave behind a branch predicated on `bs0 & bs`
                *bs0 = common;
                // 2a. Add the `next` parameter to the follow-set of the existing branch we modified in-place
                nexts.insert((index, next.clone()));
                // 3. Remove all bytes from `bs` that are now covered by the branch we modified in-place
                bs = bs.difference(bs0);
            }
        }
        // If any bytes of bs were completely unique among all extant branches:
        if !bs.is_empty() {
            // 1. Create a novel branch with the follow-set implied by the `next` parameter
            let mut nexts = HashSet::new();
            nexts.insert((index, next.clone()));
            self.branches.push((bs, nexts));
        }
        // Append all enqueued branches from the iteration above
        self.branches.append(&mut new_branches);
    }

    /// Extends the set of choice-points and follow-sets of `self` with a provided [`MatchTreeStep`].
    fn merge_step(
        mut self,
        index: usize,
        step: MatchTreeStep<'a>,
    ) -> Result<MatchTreeLevel<'a>, ()> {
        if step.accept {
            self.merge_accept(index)?;
        }
        for (bs, next) in step.branches {
            self.merge_branch(index, bs, next);
        }
        Ok(self)
    }

    /// Attempt to construct and return a `MatchTree` that unconditionally accepts
    /// the same, common format-index as all elements of the set `nexts`.
    ///
    /// If `nexts` is empty, the `MatchTree` returned will instead reject all input
    ///
    /// If `nexts` contains multiple associated indices, returns `None`
    fn accepts(nexts: &LevelBranch<'a>) -> Option<MatchTree> {
        let mut tree = Self::reject();
        for (i, _next) in nexts.iter() {
            tree.merge_accept(*i).ok()?;
        }
        Some(MatchTree {
            accept: tree.accept,
            branches: vec![],
        })
    }

    /// Attempts to accumulate a `MatchTree` recursively up to an overall depth of `depth` layers,
    /// with the immediate layer constructed based on a bundle of indexed choice-points ([`LevelBranch`]).
    ///
    /// If the depth limit has been reached without a decisive choice of which index to accept, returns None.
    ///
    /// Otherwise, returns a `MatchTree` that is guaranteed to decide on a unique branch for
    /// all input within at most `depth` bytes of lookahead.
    fn grow(module: &'a FormatModule, nexts: LevelBranch<'a>, depth: usize) -> Option<MatchTree> {
        if let Some(tree) = Self::accepts(&nexts) {
            Some(tree)
        } else if depth > 0 {
            let mut tree = Self::reject();
            let mut tmp = Vec::from_iter(nexts);
            tmp.sort_by_key(|(ix, _)| *ix);
            for (i, next) in tmp.into_iter() {
                // Fresh per top-level step: no bytes are consumed within a single
                // `from_next` call, so a re-entered level here is genuine left recursion,
                // not just a level revisited from an independent sibling branch. No ambient
                // `ctx` is passed - every `next` here carries (or, for `DelayRef`, can recover)
                // its own correct context, since it may have been constructed under a different
                // one several depths/`grow` calls ago.
                let mut visited = Traversal::new_unscoped();
                let subtree = MatchTreeStep::from_next(module, next, &mut visited);
                tree = tree.merge_step(i, subtree).ok()?;
            }
            let mut branches = Vec::new();
            for (bs, nexts) in tree.branches {
                let t = Self::grow(module, nexts, depth - 1)?;
                branches.push((bs, t));
            }
            Some(MatchTree {
                accept: tree.accept,
                branches,
            })
        } else {
            None
        }
    }
}

type LevelBranch<'a> = HashSet<(usize, Rc<Next<'a>>)>;

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Next<'a> {
    Empty,
    /// `level` is an absolute `FormatId`, so unlike the other non-`Empty` variants below, no
    /// separate `RecurseCtx` needs to be carried alongside it - the correct ctx for expanding
    /// `level`'s format is always recoverable on demand via `FormatModule::get_ctx(level)`,
    /// exactly as `Format::ItemVar` resolves it.
    DelayRef(usize, Rc<Next<'a>>),
    Union(Rc<Next<'a>>, Rc<Next<'a>>),
    /// The `RecurseCtx` a variant carries is the context that was in effect when this `Next` was
    /// constructed, *not* whatever ctx happens to be ambient when it's later expanded - a `Next`
    /// can survive across a lookahead-depth boundary (i.e. past an actual consumed byte), by
    /// which point the ambient ctx at the resumption point may differ (see `MatchTreeLevel::grow`).
    Cat(&'a Format, RecurseCtx<'a>, Rc<Next<'a>>),
    Sequence(&'a [Format], RecurseCtx<'a>, Rc<Next<'a>>),
    Repeat(&'a Format, RecurseCtx<'a>, Rc<Next<'a>>),
    RepeatCount(usize, &'a Format, RecurseCtx<'a>, Rc<Next<'a>>),
    /// Dual to `RepeatCount`, for 0..=N repeats - only ever constructed internally while
    /// expanding a `RepeatBetween` (there's no corresponding `Format` variant).
    RepeatMax(usize, &'a Format, RecurseCtx<'a>, Rc<Next<'a>>),
    RepeatBetween(usize, usize, &'a Format, RecurseCtx<'a>, Rc<Next<'a>>),
    /// Neither `Peek` nor `PeekNot` needs its own `RecurseCtx`, unlike the raw-`&'a Format`-
    /// carrying variants above: both fields are already-built `Next` subtrees (the "what comes
    /// after" continuation and the lookahead-only "does the peek target match" continuation,
    /// respectively), each self-sufficient regarding its own ctx the same way `Next::Union`'s two
    /// fields are.
    Peek(Rc<Next<'a>>, Rc<Next<'a>>),
    PeekNot(Rc<Next<'a>>, Rc<Next<'a>>),
    /// `Slice`'s remaining-byte countdown, same shape/reasoning as `RepeatCount`'s, but neither
    /// of its two fields needs its own `RecurseCtx` either, for the same reason `Peek`/`PeekNot`
    /// above don't: both `inside` (the slice's own not-yet-fully-consumed inner continuation) and
    /// `outer` (what follows the whole slice) are already-built `Next` subtrees, not raw
    /// `&'a Format`, so whatever ctx they need was already stamped in when *they* were built.
    Slice(usize, Rc<Next<'a>>, Rc<Next<'a>>),
    // No `WithRelativeOffset` variant: `MatchTree` treats it as opaque to lookahead entirely
    // (see `Format::WithRelativeOffset`'s `from_format` arm), matching real doodle's own,
    // deliberately-inherited limitation - it never builds any `Next` continuation for it at all.
}

#[derive(Debug, Clone)]
pub struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
}

impl MatchTree {
    /// Returns the accepting index associated with the input-sequence starting from the current offset of `input`,
    /// looking ahead as many bytes as necessary until a definitive index is found or the lookahead limit is reached.
    ///
    /// Returns `None` if not enough lookahead remains to disambiguate multiple candidate indices.
    pub(crate) fn matches(&self, input: ReadCtxt<'_>) -> Option<usize> {
        match input.read_byte() {
            None => self.accept,
            Some((b, input)) => {
                for (bs, s) in &self.branches {
                    if bs.contains(b) {
                        return s.matches(input);
                    }
                }
                self.accept
            }
        }
    }

    /// Constructs a new `MatchTreeLevel` from an alternation of branches and a follow-set of partially decomposed formats,
    /// to within a fixed but externally opaque lookahead-depth.
    ///
    /// A `FormatModule` is also accepted to contextualize any contextually dependent formats, e.g. [`Format::ItemVar`]
    pub(crate) fn build<'a>(
        module: &'a FormatModule,
        branches: &'a [Format],
        next: Rc<Next<'a>>,
        ctx: RecurseCtx<'a>,
    ) -> Option<MatchTree> {
        let mut nexts = HashSet::new();
        for (i, f) in branches.iter().enumerate() {
            nexts.insert((i, Rc::new(Next::Cat(f, ctx, next.clone()))));
        }
        const MAX_DEPTH: usize = 80;
        MatchTreeLevel::grow(module, nexts, MAX_DEPTH)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Expr, Label};

    use super::*;

    #[test]
    fn construct_autorec_next() {
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
        let f = Format::Tuple(vec![
            Format::ItemVar(frefs[0].get_level()),
            Format::EndOfInput,
        ]);
        let ctx = RecurseCtx::NonRec;
        let mut visited = Traversal::new_unscoped();
        let tree = MatchTreeStep::from_format(&module, &f, Rc::new(Next::Empty), ctx, &mut visited);
        eprintln!("{tree:?}")
    }

    #[test]
    fn build_union_disambiguating_through_recursion() {
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
        let peano_ref = frefs[0];
        let branches = vec![
            Format::Tuple(vec![peano_ref.call(), Format::Byte(ByteSet::from([b'A']))]),
            Format::Tuple(vec![peano_ref.call(), Format::Byte(ByteSet::from([b'B']))]),
        ];
        // Both branches share an identical, unboundedly-long peano prefix and only diverge on
        // the byte immediately after it, so disambiguating them is *not* actually decidable
        // within any fixed lookahead depth - `None` (bounded lookahead exhausted) is the
        // correct, sound result here, same as it would be for two non-recursive branches with
        // an unboundedly long common prefix. What this regression-tests is that resolving the
        // `RecVar` inside peano's body - reached only after `MatchTreeLevel::grow` has crossed
        // at least one lookahead-depth boundary (past the first consumed 'S'/'Z' byte) - no
        // longer panics via `ctx.convert_rec_var(_).unwrap()` on a stale/mismatched ambient ctx.
        let tree = MatchTree::build(&module, &branches, Rc::new(Next::Empty), RecurseCtx::NonRec);
        assert!(tree.is_none());
    }

    #[test]
    fn repeat_count_ctx_across_depth_boundary() {
        // Unlike `build_union_disambiguating_through_recursion` (where the recursive reference
        // is reached via a fresh `ItemVar`, which always resets `ctx` itself via
        // `module.get_ctx`, masking any bug in an *ambient* ctx passed to it), the
        // `RepeatCount(2, RecVar(0))` here is embedded directly inside the batch's own
        // self-recursive body, referencing `RecVar(0)` (not `ItemVar`) - so resolving it depends
        // entirely on the ctx `Next::RepeatCount` itself carries once that node survives a
        // lookahead-depth boundary (which the second byte of input, "SZ", forces).
        let wrapper = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("stop"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("more"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::RepeatCount(Box::new(Expr::U8(2)), Box::new(Format::RecVar(0))),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![(Label::Borrowed("test.wrapper"), wrapper)]);
        let wrapper_ref = frefs[0];
        let branches = vec![
            Format::Tuple(vec![
                wrapper_ref.call(),
                Format::Byte(ByteSet::from([b'A'])),
            ]),
            Format::Tuple(vec![
                wrapper_ref.call(),
                Format::Byte(ByteSet::from([b'B'])),
            ]),
        ];
        // Same fundamental undecidability as the test above (unboundedly-long common prefix) -
        // `None` is correct, the point is no panic.
        let tree = MatchTree::build(&module, &branches, Rc::new(Next::Empty), RecurseCtx::NonRec);
        assert!(tree.is_none());
    }

    #[test]
    fn slice_ctx_across_depth_boundary() {
        // Same shape/reasoning as repeat_count_ctx_across_depth_boundary: `Slice(1, RecVar(0))`
        // embedded directly inside the batch's own body means resolving that `RecVar` depends on
        // the ctx captured when `Format::Slice`'s own `from_format` arm builds its initial
        // `Next::Cat(a, ctx, Empty)` surviving intact through `Next::Slice`'s countdown once that
        // survives a lookahead-depth boundary (forced by the second byte, "SZ").
        let wrapper = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("stop"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("more"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::Slice(Box::new(Expr::U8(1)), Box::new(Format::RecVar(0))),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![(Label::Borrowed("test.wrapper"), wrapper)]);
        let wrapper_ref = frefs[0];
        let branches = vec![
            Format::Tuple(vec![
                wrapper_ref.call(),
                Format::Byte(ByteSet::from([b'A'])),
            ]),
            Format::Tuple(vec![
                wrapper_ref.call(),
                Format::Byte(ByteSet::from([b'B'])),
            ]),
        ];
        // Unlike the RepeatCount analog above, this *is* decidable: `Slice(1, RecVar(0))` caps
        // the entire nested peano value to exactly 1 byte of lookahead (whatever comes after
        // that byte, inside the slice, is leftover and discarded at runtime, but MatchTree only
        // ever needs to look at that one byte here) - so the whole thing resolves in bounded
        // depth. `Some` is itself part of what this test demonstrates: proof the ctx survived
        // correctly enough to reach a real, decided tree, not just "didn't panic, gave up".
        let tree = MatchTree::build(&module, &branches, Rc::new(Next::Empty), RecurseCtx::NonRec);
        assert!(tree.is_some());
    }
}
