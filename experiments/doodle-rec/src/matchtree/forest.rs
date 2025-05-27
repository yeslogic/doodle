use doodle::prelude::ByteSet;

use crate::{Format, FormatDecl, FormatId, FormatModule, RecId, RecurseCtx};

pub type FirstSet = ByteSet;

#[derive(Debug)]
pub enum GrammarError {
    EpsilonLoop { top: FormatId, reentrant: RecId },
    RepeatNullable { format: Format },
    MultiNullUnion { format: Format },
}

impl std::fmt::Display for GrammarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrammarError::EpsilonLoop { top, reentrant } => {
                write!(
                    f,
                    "epsilon-move cycle found in format: #{} -*-> ~{} -*-> ~{}",
                    top,
                    reentrant,
                    reentrant,
                )
            }
            GrammarError::RepeatNullable { format  } => {
                write!(f, "repeat of nullable format: {:?}", format)
            }
            GrammarError::MultiNullUnion { format } => {
                write!(f, "multiple nullable formats in union: {:?}", format)
            }
        }
    }
}

impl std::error::Error for GrammarError {}

impl FormatDecl {
    pub fn first_set(&self, module: &FormatModule) -> Result<FirstSet, GrammarError> {
        let mut traversal = Traversal::new(self.fmt_id);
        let ctx = module.get_ctx(self.fmt_id);
        Ok(self.format.solve_first_set(module, &mut traversal, ctx)?.0)
    }
}

impl Format {
    /// Returns the first-set, along with `true` if the format is nullable and `false` otherwise
    fn solve_first_set(
        &self,
        module: &FormatModule,
        visited: &mut Traversal,
        ctx: RecurseCtx<'_>,
    ) -> Result<(FirstSet, bool), GrammarError> {
        match self {
            Format::ItemVar(level) => {
                let level = *level;
                let ctx = module.get_ctx(level);
                let mut visited = Traversal::new(level);
                module
                    .get_format(level)
                    .solve_first_set(module, &mut visited, ctx)
            },
            Format::RecVar(rec_ix) => {
                let level = ctx.convert_rec_var(*rec_ix);
                if visited.insert(level) {
                    let ctx = ctx.enter(*rec_ix).0;
                    ctx.get_format().unwrap().solve_first_set(module, visited, ctx)
                } else {
                    Err(GrammarError::EpsilonLoop {
                        top: visited.orig_level,
                        reentrant: *rec_ix,
                    })
                }
            }
            Format::Byte(set) => Ok((*set, false)),
            Format::FailWith(..) => Ok((FirstSet::empty(), false)),
            Format::EndOfInput => Ok((FirstSet::empty(), true)),
            Format::Compute(..) => Ok((FirstSet::empty(), true)),
            Format::Variant(.., format) => format.solve_first_set(module, visited, ctx),
            Format::Union(formats) => {
                let mut ret = ByteSet::empty();
                let mut any_nullable = false;
                for format in formats {
                    let (first_set, is_nullable) = format.solve_first_set(module, visited, ctx)?;
                    ret = ret.union(&first_set);
                    if is_nullable {
                        if any_nullable {
                            return Err(GrammarError::MultiNullUnion { format: self.clone() });
                        }
                        any_nullable = true;
                    }
                }
                Ok((ret, any_nullable))
            }
            Format::Repeat(format) => {
                let (first_set, is_nullable) = format.solve_first_set(module, visited, ctx)?;
                if is_nullable {
                    return Err(GrammarError::RepeatNullable { format: format.as_ref().clone() });
                }
                Ok((first_set, true))
            }
            Format::Tuple(formats) | Format::Seq(formats) => {
                let mut ret = ByteSet::empty();
                let mut nullable = true;
                for format in formats {
                    if !ret.is_empty() { break; }
                    let (first_set, is_nullable) = format.solve_first_set(module, visited, ctx)?;
                    ret = ret.union(&first_set);
                    nullable &= is_nullable;
                    if !nullable {
                        break;
                    }
                }
                Ok((ret, nullable))
            }
            Format::Maybe(_cond, format) => {
                let (first_set, _) = format.solve_first_set(module, visited, ctx)?;
                Ok((first_set, true))
            }
        }
    }
}


use traversal::Traversal;
mod traversal {
    use std::collections::BTreeSet;

    pub(super) struct Traversal {
        pub(super) orig_level: usize,
        seen_levels: BTreeSet<usize>,
    }

    impl Traversal {
        pub(super) fn new(orig_level: usize) -> Self {
            Self {
                orig_level,
                seen_levels: BTreeSet::new(),
            }
        }

        /// Returns `true` if the level has not yet been seen (including the original level)
        pub(super) fn is_novel(&self, level: usize) -> bool {
            level != self.orig_level && !self.seen_levels.contains(&level)
        }

        /// Record a level as being seen, returning `true` if it is novel.
        pub(super) fn insert(&mut self, level: usize) -> bool {
            if level == self.orig_level {
                return false;
            }
            self.seen_levels.insert(level)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_set() {

    }
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
    /// Alternation over two partial-formats that shared a common prefix we already consumed
    Union(Rc<PartialFormat<'a>>, Rc<PartialFormat<'a>>),
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
