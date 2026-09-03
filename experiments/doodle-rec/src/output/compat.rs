//! Local replacement for `doodle::precedence`, which `doodle-rec`'s pretty-printer
//! (`output/mod.rs`) used to reuse directly. Unlike the rest of `doodle::output`'s internals this
//! crate leaned on (now re-exposed as `pub` upstream specifically for `doodle-rec`'s benefit, see
//! the commit that added this file), `doodle::precedence::Precedence` wasn't just hidden - it was
//! redesigned into a hierarchy (`ArithInfix(ArithLevel)`, `Calculus(CalculusLevel)`, etc.) sized
//! for doodle's much larger `Expr`. `doodle-rec`'s own `Expr`/`Format` grammar is far smaller, so
//! rather than adopt that hierarchy here too, this is a flat total-order scheme purpose-built for
//! the vocabulary `output/mod.rs` already renders against. Diagnostic-only pretty-printing, so
//! exact precedence fidelity with doodle's scheme isn't load-bearing - only that nesting reads
//! unambiguously.

use doodle::output::Fragment;

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
