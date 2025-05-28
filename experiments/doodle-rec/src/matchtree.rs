use std::{collections::HashSet, rc::Rc, vec};

use crate::{Format, FormatModule, RecurseCtx};
use doodle::{byte_set::ByteSet, read::ReadCtxt};

pub mod forest;
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

    /// Constructs a [MatchTreeStep] that accepts a given tuple of sequential formats, with a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_sequential(
        module: &'a FormatModule,
        fields: &'a [Format],
        next: Rc<Next<'a>>,
        ctx: RecurseCtx<'a>
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::from_next(module, next, ctx),
            Some((f, fs)) => Self::from_format(module, f, Rc::new(Next::Sequence(fs, next)), ctx),
        }
    }

    /// Constructs a [MatchTreeStep] from a [`Next`]
    fn from_next(module: &'a FormatModule, next: Rc<Next<'a>>, ctx: RecurseCtx<'a>) -> MatchTreeStep<'a> {
        match next.as_ref() {
            Next::Empty => Self::accept(),
            Next::Union(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone(), ctx);
                let tree2 = Self::from_next(module, next2.clone(), ctx);
                tree1.union(tree2)
            }
            Next::Cat(f, next) => MatchTreeStep::<'a>::from_format(module, *f, next.clone(), ctx),
            Next::Sequence(fields, next) => {
                let next = next.clone();
                match fields.split_first() {
                    None => Self::from_next(module, next, ctx),
                    Some((f, fs)) => {
                        Self::from_format(module, f, Rc::new(Next::Sequence(fs, next)), ctx)
                    }
                }
            }
            Next::Repeat(a, next0) => {
                let tree = MatchTreeStep::<'a>::from_next(module, next0.clone(), ctx);
                let next1 = next.clone();
                tree.union(MatchTreeStep::<'a>::from_format(module, *a, next1, ctx))
            }
            Next::DelayRef(_ix /*, next0 */) => {
                // FIXME - figure out how to resolve this
                Self::accept()
            }
        }
    }

    fn from_format(
        module: &'a FormatModule,
        f: &'a Format,
        next: Rc<Next<'a>>,
        ctx: RecurseCtx<'a>,
    ) -> MatchTreeStep<'a> {
        match f {
            Format::ItemVar(level) => {
                let ctx = module.get_ctx(*level);
                Self::from_format(module, module.get_format(*level), next, ctx)
            }
            Format::FailWith(_) => Self::reject(),
            Format::EndOfInput => Self::accept(),
            Format::Byte(bs) => Self::branch(*bs, next),
            Format::Variant(_label, f) => Self::from_format(module, f, next, ctx),
            Format::Union(branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone(), ctx));
                }
                tree
            }
            Format::Seq(fields) | Format::Tuple(fields) => {
                Self::from_sequential(module, fields, next, ctx)
            }
            Format::Repeat(a) => {
                let tree = Self::from_next(module, next.clone(), ctx);
                tree.union(Self::from_format(
                    module,
                    a,
                    Rc::new(Next::Repeat(a, next)),
                    ctx,
                ))
            }
            Format::Maybe(_expr, a) => {
                let tree_some = Self::from_format(module, a, next.clone(), ctx);
                let tree_none = Self::from_next(module, next, ctx);
                tree_some.union(tree_none)
            }
            Format::Compute(_expr) => Self::from_next(module, next, ctx),
            Format::RecVar(rec_ix) => {
                // FIXME - we discard the original `next` argument here
                let next = Rc::new(Next::DelayRef(ctx.convert_rec_var(*rec_ix).unwrap() /* , next.clone() */));
                Self::from_next(module, next, ctx)
            }
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
    fn grow(module: &'a FormatModule, nexts: LevelBranch<'a>, depth: usize, ctx: RecurseCtx<'a>) -> Option<MatchTree> {
        if let Some(tree) = Self::accepts(&nexts) {
            Some(tree)
        } else if depth > 0 {
            let mut tree = Self::reject();
            let mut tmp = Vec::from_iter(nexts);
            tmp.sort_by_key(|(ix, _)| *ix);
            for (i, next) in tmp.into_iter() {
                let subtree = MatchTreeStep::from_next(module, next, ctx);
                tree = tree.merge_step(i, subtree).ok()?;
            }
            let mut branches = Vec::new();
            for (bs, nexts) in tree.branches {
                let t = Self::grow(module, nexts, depth - 1, ctx)?;
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
    DelayRef(usize /* , Rc<Next<'a>> */),
    Union(Rc<Next<'a>>, Rc<Next<'a>>),
    Cat(&'a Format, Rc<Next<'a>>),
    Sequence(&'a [Format], Rc<Next<'a>>),
    Repeat(&'a Format, Rc<Next<'a>>),
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
            nexts.insert((i, Rc::new(Next::Cat(f, next.clone()))));
        }
        const MAX_DEPTH: usize = 80;
        MatchTreeLevel::grow(module, nexts, MAX_DEPTH, ctx)
    }
}

#[cfg(test)]
mod tests {
    use crate::Label;

    use super::*;

    #[test]
    fn construct_autorec_next() {
        let peano = Format::Union(vec![
            Format::Variant(Label::Borrowed("peanoZ"), Box::new(Format::Byte(ByteSet::from([b'Z'])))),
            Format::Variant(Label::Borrowed("peanoS"), Box::new(Format::Tuple(vec![Format::Byte(ByteSet::from([b'S'])), Format::RecVar(0)]))),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.declare_rec_formats(vec![
            (Label::Borrowed("test.peano"), peano),
        ]);
        let f = Format::Tuple(vec![Format::ItemVar(frefs[0].get_level()), Format::EndOfInput]);
        let ctx = RecurseCtx::NonRec;
        let tree = MatchTreeStep::from_format(&module, &f, Rc::new(Next::Empty), ctx);
        eprintln!("{tree:?}")
    }
}
