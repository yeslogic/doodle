//! Small self-contained replacements for `doodle::output`/`doodle::precedence` internals that
//! `doodle-rec`'s pretty-printer (`output/mod.rs`) used to reuse directly, back when they were
//! `pub`. They have since been scoped down to `pub(crate)` in `doodle` proper (or, in the case of
//! `Precedence`, redesigned into a richer hierarchy this crate's much smaller `Expr`/`Format`
//! grammar has no need of) as part of refreshing this branch's stale copy of `doodle`'s `src/` and
//! `doodle-formats/` from `main`. `doodle-rec`'s own pretty-printer is diagnostic-only (used by
//! `main.rs`'s `PPrinter` demo), so exact precedence fidelity with `doodle`'s original scheme isn't
//! load-bearing here - only that nesting is unambiguous.
//!
//! `Fragment` itself (and its variants) is still `pub`, so everything below is expressed purely in
//! terms of that public surface.

use doodle::output::Fragment;

/// Extension trait vendoring the [`Fragment`] methods that became `pub(crate)` upstream. Bodies
/// are copied verbatim from `doodle::output`'s originals.
pub(crate) trait FragmentExt {
    fn group(self) -> Self;
    fn enclose(&mut self) -> &mut Self;
    fn is_single_line(&self, is_final: bool) -> bool;
    fn fits_inline(&self) -> bool;
    fn join_with_wsp(self, other: Self) -> Self;
}

impl FragmentExt for Fragment {
    fn group(self) -> Self {
        Fragment::Group(Box::new(self))
    }

    fn enclose(&mut self) -> &mut Self {
        let this = Box::new(std::mem::take(self));
        *self = Fragment::Group(this);
        self
    }

    fn is_single_line(&self, is_final: bool) -> bool {
        match self {
            Fragment::Empty => true,
            Fragment::Char('\n') => is_final,
            Fragment::Char(_) => true,
            Fragment::String(s) => {
                let ix_nl = s.find('\n');
                match ix_nl {
                    Some(n) if n == s.len() - 1 => is_final,
                    None => true,
                    _ => false,
                }
            }
            Fragment::Symbol(_) => true,
            Fragment::DisplayAtom(_) | Fragment::DebugAtom(_) => true,
            Fragment::Group(frag) => frag.is_single_line(is_final),
            Fragment::Cat(lhs, rhs) => lhs.is_single_line(false) && rhs.is_single_line(is_final),
            Fragment::Sequence { sep, items } => {
                match sep {
                    None => (),
                    Some(join) => {
                        if !items.is_empty() && !join.is_single_line(false) {
                            return false;
                        }
                    }
                }
                let l = items.len();
                items
                    .iter()
                    .enumerate()
                    .all(|(ix, frag)| frag.is_single_line(is_final && (ix == l - 1)))
            }
        }
    }

    fn fits_inline(&self) -> bool {
        match self {
            Fragment::Empty => true,
            Fragment::Char(c) => *c != '\n',
            Fragment::String(s) => !s.contains('\n'),
            Fragment::Symbol(_) => false,
            Fragment::DisplayAtom(_) | Fragment::DebugAtom(_) => true,
            Fragment::Group(frag) => frag.fits_inline(),
            Fragment::Cat(lhs, rhs) => lhs.fits_inline() && rhs.fits_inline(),
            Fragment::Sequence { sep, items } => {
                match sep {
                    None => (),
                    Some(join) => {
                        if !items.is_empty() && !join.fits_inline() {
                            return false;
                        }
                    }
                }
                items.iter().all(Self::fits_inline)
            }
        }
    }

    fn join_with_wsp(self, other: Self) -> Self {
        if other.fits_inline() {
            self.cat(Self::Char(' ')).cat(other).cat_break()
        } else {
            self.cat_break().cat(other)
        }
    }
}

fn frag_is_empty(frag: &Fragment) -> bool {
    matches!(frag, Fragment::Empty)
}

/// Local reimplementation of `doodle::output::FragmentBuilder` (now `pub(crate)` upstream),
/// unchanged in behavior from the original.
pub(crate) struct FragmentBuilder {
    frozen: Vec<Fragment>,
    active: Fragment,
}

impl FragmentBuilder {
    pub(crate) fn new() -> Self {
        Self {
            frozen: Vec::new(),
            active: Fragment::Empty,
        }
    }

    pub(crate) fn active_mut(&mut self) -> &mut Fragment {
        &mut self.active
    }

    fn renew(&mut self) -> &mut Fragment {
        let frag = std::mem::take(&mut self.active);
        if !frag_is_empty(&frag) {
            self.frozen.push(frag);
        }
        &mut self.active
    }

    pub(crate) fn push(&mut self, frag: Fragment) {
        let old = std::mem::take(&mut self.active);

        if frag_is_empty(&old) {
            self.frozen.push(frag);
        } else {
            self.frozen.push(old);
            if !frag_is_empty(&frag) {
                self.frozen.push(frag);
            }
            self.active = Fragment::Empty;
        }
    }

    pub(crate) fn finalize(mut self) -> Fragment {
        let _ = self.renew();
        Fragment::seq(self.frozen, None)
    }

    pub(crate) fn finalize_with_sep(mut self, sep: Fragment) -> Fragment {
        let _ = self.renew();
        Fragment::seq(self.frozen, Some(sep))
    }
}

/// Local, flat total-order precedence scheme for `doodle-rec`'s own (much smaller) `Expr`/`Format`
/// grammar - not a reuse of `doodle::precedence::Precedence`, which was redesigned upstream (as
/// part of the same `src/numeric/` overhaul that dropped `IntWidth`, see `decoder.rs`) into a
/// hierarchy keyed to a much larger operator set this crate doesn't have. Named constants mirror
/// the vocabulary `output/mod.rs` already renders against; only relative order matters, since this
/// is diagnostic-only pretty-printing, not a parser front-end that must round-trip exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Precedence(u16);

impl Precedence {
    pub(crate) const TOP: Self = Self(0);
    pub(crate) const FUN_APPLICATION: Self = Self(10);
    pub(crate) const LOGICAL_NEGATE: Self = Self(20);
    pub(crate) const EQUALITY: Self = Self(30);
    pub(crate) const COMPARE: Self = Self(40);
    pub(crate) const BITOR: Self = Self(50);
    pub(crate) const BITAND: Self = Self(60);
    pub(crate) const BIT_SHIFT: Self = Self(70);
    pub(crate) const ADD_SUB: Self = Self(80);
    pub(crate) const MUL: Self = Self(90);
    pub(crate) const DIV_REM: Self = Self(90);
    pub(crate) const CAST_PREFIX: Self = Self(100);
    pub(crate) const FORMAT_COMPOUND: Self = Self(110);
    pub(crate) const ATOM: Self = Self(u16::MAX);

    /// Raises `self` to at least [`Precedence::FORMAT_COMPOUND`], so that a nested compound
    /// format (`variant`/`maybe`/`repeat`/`compute`) gets parenthesized when it appears as a
    /// direct child of another one.
    pub(crate) fn bump_format(self) -> Self {
        self.max(Self::FORMAT_COMPOUND)
    }
}

impl Default for Precedence {
    fn default() -> Self {
        Self::TOP
    }
}

/// Parenthesizes `frag` iff the ambient/required precedence `current` is stricter than `frag`'s
/// own operator precedence `cutoff`.
pub(crate) fn cond_paren(frag: Fragment, current: Precedence, cutoff: Precedence) -> Fragment {
    if current > cutoff {
        Fragment::Char('(').cat(frag).cat(Fragment::Char(')'))
    } else {
        frag
    }
}
