use crate::bounds::Bounds;
use std::collections::HashSet;

/// Alias for clarifying the semantics in `match_bounds` and `lookahead_bounds` on `Format` and `TypedFormat`
pub type OpenSet = HashSet<usize>;

/// Analog to [`MatchTreeStep::guarded`](crate::MatchTreeStep::guarded) for the purposes
/// of Format bounds-prediction using [`OpenSet`].
pub(crate) fn guarded_bounds<F>(level: usize, open: &mut OpenSet, f: F) -> RecursiveBounds
where
    F: FnOnce(&mut OpenSet) -> RecursiveBounds,
{
    if !open.insert(level) {
        RecursiveBounds::unresolved()
    } else {
        let ret = f(open);
        open.remove(&level);
        ret
    }
}

/// Bounds computed by a recursion-aware traversal (see `Format::match_bounds`/`Format::lookahead_bounds`),
/// paired with whether `bounds.min` is still provisional because the traversal passed through a
/// currently-open recursive reference without (yet) finding a non-recursive resolution for it.
///
/// `bounds.max` needs no such flag: an unbounded max reported at an open recursive reference is
/// exact (a self-referential format can consume unboundedly many bytes), not a placeholder to be
/// refined later. Only `bounds.min` may be understated while `min_unresolved` is `true`, since a
/// recursive branch's own true minimum cannot be determined without first resolving the recursion.
#[derive(Copy, Clone, Debug)]
pub(crate) struct RecursiveBounds {
    bounds: Bounds,
    min_unresolved: bool,
}

impl RecursiveBounds {
    /// A fully-determined value: recursion (if any) has already bottomed out through a
    /// non-recursive branch, so `bounds` is trustworthy as-is.
    pub(crate) const fn resolved(bounds: Bounds) -> Self {
        Self {
            bounds,
            min_unresolved: false,
        }
    }

    /// The value to report at a currently-open recursive reference (i.e. `open.insert` returning
    /// `false` in the caller's traversal): unbounded, and with a minimum that is not yet
    /// trustworthy, since it depends on the recursion this very reference is part of.
    pub(crate) fn unresolved() -> Self {
        Self {
            bounds: Bounds::any(),
            min_unresolved: true,
        }
    }

    pub(crate) fn exact(n: usize) -> Self {
        Self::resolved(Bounds::exact(n))
    }

    pub(crate) fn any() -> Self {
        Self::resolved(Bounds::any())
    }

    pub(crate) fn at_least(min: usize) -> Self {
        Self::resolved(Bounds::at_least(min))
    }

    pub(crate) fn new(min: usize, max: usize) -> Self {
        Self::resolved(Bounds::new(min, max))
    }

    /// Combines two alternative branches (e.g. `Union`/`Match` arms, or `Maybe`'s implicit
    /// zero-length alternative).
    ///
    /// The maximum is the ordinary bounds-union (the highest of the two maxima, or unbounded if
    /// either is). The minimum ignores an operand that is still `min_unresolved`, as long as the
    /// other operand isn't - a resolved alternative's minimum is provably `<=` any still-open
    /// alternative's true minimum, since a recursive branch can only add a non-negative number of
    /// bytes around its own self-reference. Only when both operands are still unresolved does the
    /// combined result remain (conservatively) unresolved.
    pub(crate) fn union(self, other: Self) -> Self {
        let min = match (self.min_unresolved, other.min_unresolved) {
            (false, true) => self.bounds.min,
            (true, false) => other.bounds.min,
            _ => usize::min(self.bounds.min, other.bounds.min),
        };
        let max = match (self.bounds.max, other.bounds.max) {
            (Some(a), Some(b)) => Some(usize::max(a, b)),
            _ => None,
        };
        Self {
            bounds: Bounds { min, max },
            min_unresolved: self.min_unresolved && other.min_unresolved,
        }
    }

    pub(crate) fn bits_to_bytes(self) -> Self {
        Self {
            bounds: self.bounds.bits_to_bytes(),
            ..self
        }
    }

    /// Finalizes the traversal's result into a plain `Bounds`, dropping the unresolved flag. If
    /// the top-level format has no non-recursive resolution at all (a pathological, non-terminating
    /// format), this yields the same conservative fallback (`min` possibly understated) as before
    /// `RecursiveBounds` existed.
    pub(crate) fn into_bounds(self) -> Bounds {
        self.bounds
    }
}

impl std::ops::Add for RecursiveBounds {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            bounds: self.bounds + rhs.bounds,
            min_unresolved: self.min_unresolved || rhs.min_unresolved,
        }
    }
}

impl std::ops::Add<Bounds> for RecursiveBounds {
    type Output = Self;

    fn add(self, rhs: Bounds) -> Self {
        Self {
            bounds: self.bounds + rhs,
            min_unresolved: self.min_unresolved,
        }
    }
}

impl std::ops::Mul<Bounds> for RecursiveBounds {
    type Output = Self;

    fn mul(self, rhs: Bounds) -> Self {
        Self {
            bounds: self.bounds * rhs,
            min_unresolved: self.min_unresolved,
        }
    }
}
