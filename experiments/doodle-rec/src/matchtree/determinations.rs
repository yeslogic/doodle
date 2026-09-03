use std::rc::Rc;

use doodle::prelude::ByteSet;

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
    ///
    /// Namely, `Self::merge_seq(x, Self::zero()) == Self::merge_seq(Self::zero(), x) == x` should hold for all `x`.
    pub const fn zero() -> Self {
        Self {
            first_set: ByteSet::empty(),
            is_productive: true,
            is_nullable: true,
            should_not_follow_set: ByteSet::empty(),
        }
    }

    /// Multiplicative (i.e. disjunction) identity element
    ///
    /// Namely, `Self::union(x, Self::one()) == x` should hold for all `x`.
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

    /// Combines the determinations for two branches of a [`Format::UnionNondet`] - the same
    /// arithmetic as [`Self::union`], but infallible: overlapping first-sets (`AmbiguousFirst`)
    /// and multiple nullable branches (`MultiNullUnion`) are exactly what `UnionNondet` exists to
    /// permit (resolved at parse time via backtracking, not statically proven safe here), so
    /// neither check applies.
    pub fn union_nondet(self, other: Self) -> Self {
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
        Self {
            first_set,
            is_productive: self.is_productive || other.is_productive,
            is_nullable: self.is_nullable || other.is_nullable,
            should_not_follow_set,
        }
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
                    unreachable!(
                        "solve_determinations: {ctx:?} has no recursive variable ~{rec_ix} (open: {:#?})",
                        visited.open_levels().collect::<Vec<usize>>()
                    );
                });
                match visited.insert(level) {
                    Entry::Novel => {
                        let ctx = ctx.enter(*rec_ix);
                        let ret = ctx
                            .get_format()
                            .unwrap()
                            .solve_determinations(module, visited, ctx)?;
                        visited.escape();
                        Ok(ret)
                    }
                    Entry::Guarded => {
                        // Already open, but guarded (progress made) since it was opened: this is
                        // ordinary, terminating recursion. Short-circuit rather than re-expand -
                        // `level`'s own recursive structure is already being (or will be)
                        // checked in full on its own terms elsewhere.
                        Ok(Determinations::one())
                    }
                    Entry::LeftRecursive => {
                        // Still unguarded since it was opened - reaching it again would derive
                        // itself with zero progress. `visited.open_levels()` (outermost first)
                        // plus this repeated `level` traces the actual cycle.
                        let top = visited.orig_level.unwrap_or(level);
                        let cycle = visited
                            .open_levels()
                            .chain(std::iter::once(level))
                            .collect();
                        Err(GrammarError::LeftRecursion {
                            top,
                            cycle,
                            context: self.clone(),
                        })
                    }
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
                    // Branches are alternatives, not a sequence: a guard reached inside one
                    // branch must not make its unrelated sibling look guarded too. Cloning
                    // (rather than `fork`) still carries in whatever this Union's own ancestors
                    // already opened/guarded, since those remain relevant to every branch.
                    let mut branch_visited = visited.clone();
                    let det_format =
                        format.solve_determinations(module, &mut branch_visited, ctx)?;
                    det = det
                        .union(det_format)
                        .map_err(|e| e.add_context(self.clone()))?;
                }
                Ok(det)
            }
            Format::UnionNondet(formats) => {
                let mut det = Determinations::one();
                for format in formats {
                    // Same branch-isolation reasoning as `Union` above; only the ambiguity
                    // *checks* differ (`union_nondet` has none - overlapping first-sets and
                    // multiple nullable branches are exactly what `UnionNondet` permits), but a
                    // genuine left-recursion cycle reached through any branch is still an error.
                    let mut branch_visited = visited.clone();
                    let det_format =
                        format.solve_determinations(module, &mut branch_visited, ctx)?;
                    det = det.union_nondet(det_format);
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
                    if !det_format.is_nullable {
                        // Definitely consumes something: every level still open at this point
                        // (however it was reached) has now had progress made since it was
                        // opened, so re-entering any of them later in this sequence is no longer
                        // left recursion.
                        visited.guard();
                    }
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
            Format::RepeatCount(count, format) => {
                let n = count.eval_usize();
                let mut det_seq = Determinations::zero();
                for _ in 0..n {
                    let det_format = format.solve_determinations(module, visited, ctx)?;
                    if !det_format.is_nullable {
                        visited.guard();
                    }
                    det_seq = det_seq
                        .merge_seq(det_format)
                        .map_err(|e| e.add_context(self.clone()))?;
                }
                Ok(det_seq)
            }
            Format::RepeatBetween(min, max, format) => {
                let (min, max) = (min.eval_usize(), max.eval_usize());
                let mut det_seq = Determinations::zero();
                // Mandatory prefix: `min` copies, exactly like `RepeatCount`.
                for _ in 0..min {
                    let det_format = format.solve_determinations(module, visited, ctx)?;
                    if !det_format.is_nullable {
                        visited.guard();
                    }
                    det_seq = det_seq
                        .merge_seq(det_format)
                        .map_err(|e| e.add_context(self.clone()))?;
                }
                // Optional tail: up to `max - min` more. Modeled as a *single* unit, exactly like
                // `Format::Repeat`'s own body (one `solve_determinations` call, one `merge_seq`
                // into `det_seq`) - not `max - min` separate `merge_seq` calls chained against
                // each other. The latter would ask `merge_seq`'s static first/follow-set check to
                // prove "one more repetition, or stop" is unambiguous against *another optional
                // copy of the same format*, which it can't (both look identical - that's exactly
                // the "no batteries left" case `should_not_follow_set` exists to guard against, so
                // it fires as a false-positive `AmbiguousFollow` on an otherwise perfectly
                // decidable-by-lookahead construct). The real decision procedure for "continue or
                // stop" at each of the optional positions belongs to `MatchTree`/`Next::RepeatMax`
                // at build/decode time, same as unbounded `Repeat`; this static pass only needs to
                // check the boundary between the optional tail *as a whole* and whatever follows.
                if min < max {
                    let det_format = format.solve_determinations(module, visited, ctx)?;
                    if det_format.is_nullable {
                        return Err(GrammarError::RepeatNullable {
                            format: format.as_ref().clone(),
                            context: self.clone(),
                        });
                    }
                    let det_optional = Determinations {
                        is_nullable: true,
                        should_not_follow_set: det_format.first_set,
                        ..det_format
                    };
                    det_seq = det_seq
                        .merge_seq(det_optional)
                        .map_err(|e| e.add_context(self.clone()))?;
                }
                Ok(det_seq)
            }
            // Peek/PeekNot don't advance the real stream, so - unlike every other position -
            // their target's first-set/nullability isn't folded into the surrounding sequence at
            // all (modeled as the sequencing identity, same as `Compute`); precisely representing
            // "conditional on the next byte(s), but consumes none of them" isn't attempted here.
            // The target *is* still traversed, to catch left-recursion reached only through a
            // peek - using a clone of `visited` (same as `Format::Union`'s branches), not the
            // shared reference: this still tracks every currently-open level (so a genuine cycle
            // through the peek target is still caught), while any `guard()` calls the target
            // triggers - reflecting only hypothetical, lookahead-only progress - stay local to
            // the clone rather than incorrectly marking real, unconsumed ancestor cycles guarded.
            Format::Peek(format) | Format::PeekNot(format) => {
                let mut peek_visited = visited.clone();
                format.solve_determinations(module, &mut peek_visited, ctx)?;
                Ok(Determinations::zero())
            }
        }
    }
}

pub(crate) use traversal::{Entry, Traversal};
mod traversal {
    /// Outcome of [`Traversal::insert`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Entry {
        /// `level` is genuinely new along the current path: proceed to recurse into it.
        Novel,
        /// `level` is already open, but has been *guarded* (see [`Traversal::guard`]) since it
        /// was opened — some definitely-non-nullable step has been taken since then, so this is
        /// ordinary, terminating recursion, not left recursion. The caller should stop here
        /// (short-circuit) rather than recurse again, but this is not an error.
        Guarded,
        /// `level` is already open and still *unguarded* since it was opened: reaching it again
        /// would happen with zero progress made — this is left recursion, and the caller should
        /// report it rather than recurse further.
        LeftRecursive,
    }

    /// Semi-mutable traversal state for tracking which format-levels are currently open (being
    /// expanded) and whether each has been *guarded* yet, used to detect left recursion while
    /// walking a recursive [`Format`](crate::Format)/[`Next`](crate::matchtree::Next).
    ///
    /// A level is *unguarded* from the moment it's opened until some definitely-non-nullable step
    /// (see [`guard`](Traversal::guard)) is taken somewhere along the path since it was opened —
    /// re-entering it while still unguarded is left recursion (it would derive itself with zero
    /// progress); re-entering it once guarded is ordinary, terminating recursion, since *some*
    /// progress is guaranteed to have been made every time it recurses.
    ///
    /// A `Traversal` is *not* meant to be reused across a byte actually being consumed at
    /// runtime — once input has actually advanced, any cycle has been legitimately broken, and a
    /// fresh (or [`reset`](Traversal::reset)/[`fork`](Traversal::fork)ed) `Traversal` should be
    /// used going forward. Its job is only to catch unguarded recursion within a single
    /// standalone traversal.
    #[derive(Clone)]
    pub struct Traversal {
        /// The format-level this `Traversal` is anchored to, if any, tracked (and guarded)
        /// independently of `open` rather than being inserted into it up front, so that
        /// [`reset`](Traversal::reset) — which only clears `open` — can't accidentally forget it.
        ///
        /// - In static-analysis contexts with a well-defined "format currently being analyzed"
        ///   (e.g. [`FormatDecl::determinations`](super::super::FormatDecl::determinations), or
        ///   the `ItemVar` case of [`solve_determinations`](super::Format::solve_determinations)),
        ///   this is `Some(level)`: the level whose determinations/first-set are being solved.
        /// - In contexts with no single canonical "current level" — notably while growing a
        ///   [`MatchTree`](crate::matchtree::MatchTree) (see `MatchTreeLevel::grow`), where one
        ///   `Traversal` is used per top-level [`MatchTreeStep`](crate::matchtree::MatchTreeStep)
        ///   construction and may synthesize a step from several unrelated `Next` chains — this
        ///   is `None` (see [`new_unscoped`](Traversal::new_unscoped)), and cycle detection relies
        ///   purely on `open`. Note this caller never calls `guard`, so every open level simply
        ///   stays unguarded for the life of the traversal — equivalent to plain cycle rejection.
        pub(crate) orig_level: Option<usize>,
        /// Whether `orig_level` (if any) is still unguarded. See the field's docs and `guard`.
        orig_unguarded: bool,
        /// Currently-open `(level, still_unguarded)` frames, innermost/most-recently-entered
        /// last — entered via `insert`, removed via `escape`. A freshly-opened level always
        /// starts unguarded, regardless of any ancestor's guardedness: whether *it* is
        /// left-recursive is a question about progress made since *it* was opened, not about
        /// progress made before it was reached.
        open: Vec<(usize, bool)>,
    }

    impl Traversal {
        /// Constructs a `Traversal` anchored to `orig_level`. Use this whenever there is a
        /// well-defined single format-level the traversal is being performed on behalf of.
        pub fn new(orig_level: usize) -> Self {
            Self {
                orig_level: Some(orig_level),
                orig_unguarded: true,
                open: Vec::new(),
            }
        }

        /// Constructs a `Traversal` with no anchoring format-level. Use this where cycle-tracking
        /// is needed but there is no single "current level" to guard up front (e.g. across the
        /// several `Next` chains considered while growing one `MatchTree` step) — every level
        /// that needs protecting must instead be explicitly `insert`ed as it's entered.
        pub fn new_unscoped() -> Self {
            Self {
                orig_level: None,
                orig_unguarded: true,
                open: Vec::new(),
            }
        }

        /// Constructs a fresh `Traversal` anchored to the same `orig_level` as `self` (which may
        /// be `None`), with empty/unguarded state otherwise. Use this to start an independent
        /// nested traversal that should still treat `self`'s originating level as off-limits, but
        /// must not leak its own progress back into `self` — most importantly, sibling branches
        /// of a `Union` (alternatives, not a sequence: one branch reaching a guard must not make
        /// the *next*, unrelated branch look guarded too).
        pub fn fork(&self) -> Self {
            Self {
                orig_level: self.orig_level,
                orig_unguarded: true,
                open: Vec::new(),
            }
        }

        /// Records `level` as entered. See [`Entry`] for what the caller should do with each
        /// outcome. A `Novel` result opens `level` (unguarded) for the caller to `escape` later.
        pub fn insert(&mut self, level: usize) -> Entry {
            if self.orig_level == Some(level) {
                return if self.orig_unguarded {
                    Entry::LeftRecursive
                } else {
                    Entry::Guarded
                };
            }
            if let Some(&(_, unguarded)) = self.open.iter().find(|(l, _)| *l == level) {
                return if unguarded {
                    Entry::LeftRecursive
                } else {
                    Entry::Guarded
                };
            }
            self.open.push((level, true));
            Entry::Novel
        }

        /// Removes the most-recently inserted level (never `orig_level`, which is tracked
        /// separately), to avoid double-counting between branches rather than merely witnessing
        /// true cycles on a singular path.
        pub fn escape(&mut self) -> Option<usize> {
            self.open.pop().map(|(level, _)| level)
        }

        /// Iterates the currently-open levels, oldest (outermost) first — for building a
        /// human-readable cycle path once a [`Entry::LeftRecursive`] result has been reported.
        pub fn open_levels(&self) -> impl Iterator<Item = usize> + '_ {
            self.open.iter().map(|(level, _)| *level)
        }

        /// Marks every currently-open level, and `orig_level` if anchored, as guarded: call this
        /// once a definitely-non-nullable step has been taken along the current path (e.g. after
        /// a non-nullable field of a `Tuple`/`Seq`). Every level open at that moment has now had
        /// progress made since *its* opening too, transitively, however it was reached — so
        /// re-entering any of them from this point on is no longer left recursion. Levels opened
        /// later still start fresh-unguarded: whether *they* are left-recursive is independent of
        /// how much progress preceded the point they were reached from.
        pub fn guard(&mut self) {
            self.orig_unguarded = false;
            for (_, unguarded) in self.open.iter_mut() {
                *unguarded = false;
            }
        }

        /// Clears all currently-open levels, without disturbing `orig_level`'s guardedness.
        /// Intended for callers that track progress differently from the insert/escape push-pop
        /// discipline (e.g. after a byte has been consumed and any levels opened on the way to it
        /// are no longer relevant to what follows).
        pub fn reset(&mut self) {
            self.open.clear();
        }
    }
}

/// Representation of the right-justified remainder of a [`Format`] we have already
/// consumed some number (possibly 0) bytes of.
#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum PartialFormat<'a> {
    /// `ε`
    Empty,
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
    PeekNotMatched,
    /// Every branch of a `Format::UnionNondet` failed to parse at this position.
    NoValidBranch,
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
            InterpError::PeekNotMatched => {
                write!(f, "peek-not target unexpectedly matched")
            }
            InterpError::NoValidBranch => {
                write!(f, "no branch of a nondeterministic union matched")
            }
        }
    }
}

impl std::error::Error for InterpError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_set() {}
}
