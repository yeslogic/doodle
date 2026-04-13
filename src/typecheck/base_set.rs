use std::cmp::Ordering;

use super::*;
use crate::BaseType;
use crate::numeric::MachineRep;
use crate::numeric::elaborator::PRIM_INTS;
use crate::numeric::elaborator::PrimInt;
use crate::typecheck::error::{ConstraintError, TCErrorKind, UnificationError};

use crate::numeric::core::{BitWidth, Bounds as ZBounds};

/// Abstraction over explicit collections of BaseType values that could be in any order
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseSet {
    /// Singleton set of any BaseType, even non-integral ones
    Single(BaseType),
    /// Some subset of U8, U16, U32, U64
    U(UintSet),
}

impl BaseSet {
    /// Returns `true` if `self` contains the given `BaseType`, and `false` otherwise.
    pub fn contains(self, base: BaseType) -> bool {
        match self {
            BaseSet::Single(b) => b == base,
            BaseSet::U(us) => us.contains(base),
        }
    }

    /// Returns `true` if the set contains no members, i.e. if it is empty.
    pub const fn is_empty(&self) -> bool {
        match self {
            BaseSet::Single(_) => false,
            BaseSet::U(us) => us.is_empty(),
        }
    }
}

impl BaseSet {
    /// Unordered universal set of all integral BaseTypes, i.e. `{U8, U16, U32, U64}`.
    #[allow(non_upper_case_globals)]
    pub const UAny: Self = Self::U(UintSet::ANY);

    /// Universal set of all integral BaseTypes, with a default solution of U32, i.e. `{U32 > U8, U16, U64}`.
    #[allow(non_upper_case_globals)]
    pub const USome: Self = Self::U(UintSet::ANY32);
}

impl BaseSet {
    pub fn unify(self, other: Self) -> Result<Self, ConstraintError> {
        match (self, other) {
            (BaseSet::Single(b1), BaseSet::Single(b2)) => {
                if b1 == b2 {
                    Ok(self)
                } else {
                    Err(ConstraintError::Unsatisfiable(
                        Constraint::Elem(self),
                        Constraint::Elem(other),
                    ))
                }
            }
            (BaseSet::U(u), BaseSet::Single(b)) | (BaseSet::Single(b), BaseSet::U(u)) => {
                if u.contains(b) {
                    Ok(BaseSet::Single(b))
                } else {
                    Err(UnificationError::Unsatisfiable(
                        self.to_constraint(),
                        other.to_constraint(),
                    ))
                }
            }
            (BaseSet::U(u1), BaseSet::U(u2)) => Ok(BaseSet::U(u1.intersection(u2))),
        }
    }

    /// Constructs the simplest-possible constraint from `self`, in particular substituting
    /// `Equiv(Base(b))` in place of `Elem(Single(b))` or singleton `Elem(U(_))`.
    pub fn to_constraint(self) -> Constraint {
        match self {
            BaseSet::Single(b) => Constraint::Equiv(Rc::new(UType::Base(b))),
            BaseSet::U(set) => match set.as_singleton() {
                Some(b) => Constraint::Equiv(Rc::new(UType::Base(b))),
                None => Constraint::Elem(self),
            },
        }
    }

    /// Returns `Some(UType)` if `self` is a singleton `BaseSet`, and `None` otherwise.
    ///
    /// Will not attempt to solve the set if it is not an explicit singleton, i.e. `BaseSet::U` regardless
    /// of whether it has a unique solution.
    pub fn try_to_utype(self) -> Option<Rc<UType>> {
        match self {
            BaseSet::Single(b) => Some(Rc::new(UType::Base(b))),
            BaseSet::U(_) => None,
        }
    }

    pub(crate) fn get_unique_solution(self, v: UVar) -> TCResult<BaseType> {
        match self {
            BaseSet::Single(b) => Ok(b),
            BaseSet::U(u) => {
                if u.is_empty() {
                    return Err(TCErrorKind::NoSolution(v).into());
                }
                match u.get_unique_solution() {
                    Some(b) => Ok(b),
                    None => Err(TCErrorKind::MultipleSolutions(v, self).into()),
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntSet {
    /// Singleton set of any PrimInt
    Single(PrimInt),
    Z(PrimIntSet),
}

impl IntSet {
    #[allow(non_upper_case_globals)]
    pub const ZAny: Self = Self::Z(PrimIntSet::ANY);
}

impl IntSet {
    /// Returns `true` if `self` contains the given `PrimInt`, and `false` otherwise.
    pub fn contains(self, prim: PrimInt) -> bool {
        match self {
            IntSet::Single(p) => p == prim,
            IntSet::Z(ps) => ps.contains(prim),
        }
    }

    /// Given the known bounds of an untyped value-set, returns the set of valid type-assignments
    /// that can be used to represent the range of values in question.
    ///
    /// If only one type is valid, returns `IntSet::Single`.
    ///
    /// Otherwise, returns `IntSet::Z`.
    pub(crate) fn from_bounds(bounds: &ZBounds) -> Self {
        let mut set = PrimIntSet::ANY;

        // FIXME - there might be a more efficient way to do this than brute-force
        for p in PRIM_INTS.into_iter() {
            let r = MachineRep::from(p);
            let b = r.as_bounds();
            if !bounds.is_encompassed_by(&b) {
                set.remove(p);
            }
        }
        let ret = set.normalize();

        if let Some(p) = ret.as_singleton() {
            IntSet::Single(p)
        } else {
            IntSet::Z(ret)
        }
    }

    /// Converts an IntSet to a Constraint.
    ///
    /// If the IntSet is a single PrimInt, returns Constraint::Equiv with a UType::Int containing the PrimInt.
    ///
    /// If the IntSet is an arbitrary set of PrimInts, returns Constraint::NumTree with the IntSet.
    pub fn to_constraint(self) -> Constraint {
        match self {
            IntSet::Single(p) => Constraint::Equiv(Rc::new(UType::Int(p.into()))),
            IntSet::Z(ps) => match ps.as_singleton() {
                Some(p) => Constraint::Equiv(Rc::new(UType::Int(p.into()))),
                _ => Constraint::NumTree(IntSet::Z(ps)),
            },
        }
    }

    /// Constructs an IntSet from a BaseSet, failing if the BaseSet is a non-numeric singleton.
    pub(crate) fn try_from_base_set(bs: BaseSet) -> Result<Self, TryFromBaseTypeError> {
        match bs {
            BaseSet::Single(bt) => {
                let prim = PrimInt::try_from(bt)?;
                Ok(IntSet::Single(prim))
            }
            BaseSet::U(ui_set) => {
                let pi_set = ui_set.to_prim_int_set();
                Ok(IntSet::Z(pi_set))
            }
        }
    }

    pub(crate) fn to_base_set(self) -> Result<BaseSet, TryFromPrimIntError> {
        match self {
            IntSet::Single(prim) => Ok(BaseSet::Single(BaseType::try_from(prim)?)),
            IntSet::Z(set) => {
                let u_set = set.to_uint_set();
                Ok(BaseSet::U(u_set))
            }
        }
    }

    /// Returns the intersection of `self` and `other`.
    pub(crate) fn unify(self, other: IntSet) -> Result<IntSet, ConstraintError> {
        match self {
            IntSet::Single(prim) => {
                if other.contains(prim) {
                    Ok(IntSet::Single(prim))
                } else {
                    Err(ConstraintError::Unsatisfiable(
                        self.to_constraint(),
                        other.to_constraint(),
                    ))
                }
            }
            IntSet::Z(set0) => match other {
                IntSet::Single(prim) => {
                    if set0.contains(prim) {
                        Ok(IntSet::Single(prim))
                    } else {
                        Err(ConstraintError::Unsatisfiable(
                            self.to_constraint(),
                            other.to_constraint(),
                        ))
                    }
                }
                IntSet::Z(set1) => Ok(IntSet::Z(set0.intersection(set1))),
            },
        }
    }

    /// Returns `true` if the set contains no members, i.e. if it is empty.
    pub const fn is_empty(self) -> bool {
        match self {
            IntSet::Single(_) => false,
            IntSet::Z(set) => set.is_empty(),
        }
    }

    /// Attempts to obtain a single unique solution from the set, returning an error if there are zero solutions,
    /// or if there are multiple solutions that cannot be uniquely tie-broken.
    pub(crate) fn get_unique_solution(self, v: UVar) -> TCResult<PrimInt> {
        match self {
            IntSet::Single(p) => Ok(p),
            IntSet::Z(ps) => {
                if ps.is_empty() {
                    return Err(TCErrorKind::NoSolution(v).into());
                }
                match ps.get_unique_solution() {
                    Some(p) => Ok(p),
                    None => Err(TCErrorKind::MultipleIntSolutions(v, self).into()),
                }
            }
        }
    }
}

/// Abstraction over rankings of candidate values in a set, where the least-value `Rank` is the default
/// unless it is tied with anything else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    /// The value is not in the set
    Excluded,
    /// The value is in the set and has priority rank `n` (lower numbers are higher priority)
    At(u8),
}

impl Rank {
    pub const fn is_excluded(self) -> bool {
        matches!(self, Rank::Excluded)
    }
}

mod __impl_rank {
    use super::Rank;

    impl PartialOrd for Rank {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Rank {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match (self, other) {
                (Rank::At(n), Rank::At(m)) => {
                    // NOTE -  we call reverse this because we want lower numbers to be the maximum rank
                    n.cmp(m).reverse()
                }
                (Rank::At(_), Rank::Excluded) => std::cmp::Ordering::Greater,
                (Rank::Excluded, Rank::At(_)) => std::cmp::Ordering::Less,
                (Rank::Excluded, Rank::Excluded) => std::cmp::Ordering::Equal,
            }
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UintSet {
    // Array with ranks for U8, U16, U32, U64 in that order
    pub(crate) ranks: [Rank; 4],
}

// SECTION - UintSet global consts and constructors
impl UintSet {
    pub(crate) const IX_ORDER: [BaseType; 4] =
        [BaseType::U8, BaseType::U16, BaseType::U32, BaseType::U64];

    // unrestricted and non-defaulting if more than one solution
    pub const ANY: Self = Self {
        ranks: [Rank::At(3); 4],
    };

    /// UintSet with no members
    pub(crate) const EMPTY: Self = Self {
        ranks: [Rank::Excluded; 4],
    };

    // U8 or U16, default solution is U8
    pub const SHORT8: Self = Self {
        ranks: [Rank::At(0), Rank::At(1), Rank::Excluded, Rank::Excluded],
    };

    pub const ANY32: Self = Self::any_default(BitWidth::Bits32);

    pub const ANY_DEFAULT_U32: Self = Self {
        ranks: [Rank::At(1), Rank::At(1), Rank::At(0), Rank::At(1)],
    };
    pub const ANY_DEFAULT_U64: Self = Self {
        ranks: [Rank::At(1), Rank::At(1), Rank::At(1), Rank::At(0)],
    };

    // unrestricted but resolves if ambiguous to the given BitWidth, unless precluded
    pub const fn any_default(val: BitWidth) -> Self {
        let ranks = match val {
            BitWidth::Bits8 => [Rank::At(0), Rank::At(3), Rank::At(3), Rank::At(3)],
            BitWidth::Bits16 => [Rank::At(3), Rank::At(0), Rank::At(3), Rank::At(3)],
            BitWidth::Bits32 => [Rank::At(3), Rank::At(3), Rank::At(0), Rank::At(3)],
            BitWidth::Bits64 => [Rank::At(3), Rank::At(3), Rank::At(3), Rank::At(0)],
        };
        UintSet { ranks }
    }
    // Some member of the restricted set of any whose width is no less than the given BitWidth
    pub const fn at_least(val: BitWidth) -> Self {
        let ranks = match val {
            BitWidth::Bits8 => [Rank::At(3), Rank::At(3), Rank::At(3), Rank::At(3)],
            BitWidth::Bits16 => [Rank::Excluded, Rank::At(2), Rank::At(2), Rank::At(2)],
            BitWidth::Bits32 => [Rank::Excluded, Rank::Excluded, Rank::At(1), Rank::At(1)],
            BitWidth::Bits64 => [Rank::Excluded, Rank::Excluded, Rank::Excluded, Rank::At(0)],
        };
        UintSet { ranks }
    }

    /// Upcasts a `UintSet` into the equivalent `PrimIntSet`.
    pub(crate) const fn to_prim_int_set(self) -> PrimIntSet {
        PrimIntSet {
            ranks: [
                self.ranks[0],
                self.ranks[1],
                self.ranks[2],
                self.ranks[3],
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
            ],
        }
    }
}
// !SECTION

// SECTION - UintSet queries and mutators
impl UintSet {
    pub fn contains(self, b: BaseType) -> bool {
        self.ranks[b.int_width() as usize] != Rank::Excluded
    }

    /// Returns `true` if the set is
    pub const fn is_empty(self) -> bool {
        matches!(
            self.ranks,
            [
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded
            ]
        )
    }

    /// Attempts to convert the `BaseType` into a [`BaseType`], if it is a singleton.
    ///
    /// Returns `None` if the `BaseType` is not a singleton, whether it is empty or it
    /// has more than one element.
    ///
    /// Does not tie-break based on [Rank], and only considers 'inclusion' for the purposes
    /// of determining whether the set is a singleton.
    pub fn as_singleton(self) -> Option<BaseType> {
        let this = self.normalize();
        let mut arr = this.ranks.into_iter().enumerate().collect::<Vec<_>>();
        arr.retain(|(_ix, r)| !r.is_excluded());
        if arr.len() == 1 {
            Some(Self::IX_ORDER[arr[0].0])
        } else {
            None
        }
    }

    #[cfg(test)]
    pub fn get_ranks(self) -> [Rank; 4] {
        self.ranks
    }

    /// Renormalizes the internal ranks such that the rank-value of any member is equal to
    /// the number of other members of equal or greater priority (i.e. whose original rank-value
    /// was less than or equal to its own original rank-value).
    pub fn normalize(self) -> Self {
        let mut ranks = self.ranks;
        for rank in ranks.iter_mut() {
            let orig_val = *rank;
            if orig_val == Rank::Excluded {
                continue;
            }
            let count_gte = (self.ranks.iter().filter(|r| **r >= orig_val).count() - 1) as u8;
            *rank = Rank::At(count_gte);
        }
        Self { ranks }
    }

    /// Returns a `UintSet` whose solution-set is the intersection of `self` and `other`,
    /// using the higher-priority rank for each member-element that neither one excludes.
    pub fn intersection(self, other: Self) -> Self {
        let mut ret = Self::EMPTY;
        let this = self.normalize();
        let other = other.normalize();
        for (ix, (r1, r2)) in Iterator::zip(this.ranks.iter(), other.ranks.iter()).enumerate() {
            let (lo, hi) = super::min_max(*r1, *r2);
            if !lo.is_excluded() {
                ret.ranks[ix] = hi;
            }
        }
        ret.normalize()
    }

    /// Given a `UintSet`, determines the unique solution it has, if any.
    ///
    /// If multiple solutions exist, but there is one solution with a higher-priority
    /// rank than all others, that solution is returned.
    ///
    /// If no solutions exist (i.e. the set is empty), or if there is more than one
    /// solution ascribed with the highest-priority rank, then `None` is returned.
    pub fn get_unique_solution(self) -> Option<BaseType> {
        let this = self.normalize();
        let mut candidate = None;
        let mut max_rank = Rank::Excluded;
        let mut is_unique = true;
        for (ix, r) in this.ranks.into_iter().enumerate() {
            match r {
                Rank::Excluded => continue,
                Rank::At(_n) => match r.cmp(&max_rank) {
                    Ordering::Greater => {
                        max_rank = r;
                        candidate = Some(ix);
                        is_unique = true;
                    }
                    Ordering::Less => continue,
                    Ordering::Equal => {
                        is_unique = false;
                    }
                },
            }
        }
        if let Some(ix) = candidate
            && is_unique
        {
            Some(Self::IX_ORDER[ix])
        } else {
            None
        }
    }
}
// !SECTION

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrimIntSet {
    // Array with ranks for PrimInt in the same order as PRIM_INTS
    pub(crate) ranks: [Rank; PRIM_INTS.len()],
}

macro_rules! rank8 {
    ( + : $( $r:literal ),* $(,)? ) => {
        [
            $( Rank::At($r), )*
            Rank::Excluded,
            Rank::Excluded,
            Rank::Excluded,
            Rank::Excluded
        ]
    };
    ( - : $( $r:literal ),* $(,)? ) => {
        [
            Rank::Excluded,
            Rank::Excluded,
            Rank::Excluded,
            Rank::Excluded,
            $( Rank::At($r) ),*
        ]
    };
    ( +, - : $( $r:literal ),* $(,)? ) => {
        [
            $( Rank::At($r) ),*
        ]
    };
}

// SECTION - PrimIntSet constants and constructors
impl PrimIntSet {
    const IX_ORDER: [PrimInt; 8] = PRIM_INTS;

    // unrestricted and non-defaulting if more than one solution
    pub const ANY: Self = Self {
        // ranks are chosen to be identity under intersection
        ranks: [Rank::At(7); 8],
    };

    pub const ANY_UNSIGNED: Self = Self {
        ranks: rank8!(+ : 7, 7, 7, 7),
    };

    pub const ANY_SIGNED: Self = Self {
        ranks: rank8!(- : 7, 7, 7, 7),
    };

    pub const EMPTY: Self = Self {
        ranks: [Rank::Excluded; 8],
    };
}
// !SECTION

// SECTION - PrimIntSet mutators and query methods
impl PrimIntSet {
    pub const fn is_empty(&self) -> bool {
        matches!(
            self.ranks,
            [
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded,
                Rank::Excluded
            ]
        )
    }

    /// Returns `true` if the set contains the given [`PrimInt`], at any rank.
    pub const fn contains(self, prim: PrimInt) -> bool {
        matches!(self.ranks[Self::prim_as_index(prim)], Rank::At(_))
    }

    /// Attempts to convert the `PrimIntSet` into a [`PrimInt`], if it is a singleton.
    ///
    /// Returns `None` if the set is not a singleton, whether it is empty or it
    /// has more than one element.
    ///
    /// Does not tie-break based on [Rank], and only considers 'inclusion' for the purposes
    /// of determining whether the set is a singleton.
    pub fn as_singleton(self) -> Option<PrimInt> {
        let this = self.normalize();
        let mut arr = this.ranks.into_iter().enumerate().collect::<Vec<_>>();
        arr.retain(|(_ix, r)| !r.is_excluded());
        if arr.len() == 1 {
            Some(Self::IX_ORDER[arr[0].0])
        } else {
            None
        }
    }

    /// Converts the `PrimIntSet` into a [`UintSet`], excluding all unsupported members.
    ///
    /// The UintSet returned by this method will be normalized, and may be empty.
    pub fn to_uint_set(self) -> UintSet {
        let ranks: [Rank; 4] = self.ranks[0..4].try_into().unwrap();
        UintSet { ranks }.normalize()
    }

    /// Given a `PrimIntSet` and a [`PrimInt`] `p`, returns the set-difference
    /// of `self ∖ {p}`
    pub fn excluding(mut self, p: PrimInt) -> Self {
        self.ranks[Self::prim_as_index(p)] = Rank::Excluded;
        self
    }

    /// Mutates a `PrimIntSet` in-place to exclude the given `PrimInt`.
    ///
    /// Does not check whether the `PrimInt` is already excluded.
    pub fn remove(&mut self, p: PrimInt) {
        self.ranks[Self::prim_as_index(p)] = Rank::Excluded;
    }

    /// Renormalizes the internal ranks such that the rank-value of any member is equal to
    /// the number of other members of equal or greater priority (i.e. whose original rank-value
    /// was less than or equal to its own original rank-value).
    pub fn normalize(self) -> Self {
        let mut ranks = self.ranks;
        for rank in ranks.iter_mut() {
            let orig_val = *rank;
            if orig_val == Rank::Excluded {
                continue;
            }
            let count_gte = (self.ranks.iter().filter(|r| **r >= orig_val).count() - 1) as u8;
            *rank = Rank::At(count_gte);
        }
        Self { ranks }
    }

    /// Returns a `PrimIntSet` whose solution-set is the intersection of `self` and `other`,
    /// using the higher-priority rank for each member-element that neither one excludes.
    pub fn intersection(self, other: Self) -> Self {
        let mut ret = Self::EMPTY;
        let this = self.normalize();
        let other = other.normalize();
        for (ix, (r1, r2)) in Iterator::zip(this.ranks.iter(), other.ranks.iter()).enumerate() {
            let (lo, hi) = super::min_max(*r1, *r2);
            if !lo.is_excluded() {
                ret.ranks[ix] = hi;
            }
        }
        ret.normalize()
    }

    /// Given a `PrimIntSet`, determines the unique solution it has, if any.
    ///
    /// If multiple solutions exist, but there is one solution with a higher-priority
    /// rank than all others, that solution is returned.
    ///
    /// If no solutions exist (i.e. the set is empty), or if there is more than one
    /// solution ascribed with the highest-priority rank, then `None` is returned.
    pub fn get_unique_solution(self) -> Option<PrimInt> {
        let this = self.normalize();
        let mut candidate = None;
        let mut max_rank = Rank::Excluded;
        let mut is_unique = true;
        for (ix, r) in this.ranks.into_iter().enumerate() {
            match r {
                Rank::Excluded => continue,
                Rank::At(_n) => match r.cmp(&max_rank) {
                    Ordering::Greater => {
                        max_rank = r;
                        candidate = Some(ix);
                        is_unique = true;
                    }
                    Ordering::Less => continue,
                    Ordering::Equal => {
                        is_unique = false;
                    }
                },
            }
        }
        if let Some(ix) = candidate
            && is_unique
        {
            Some(PRIM_INTS[ix])
        } else {
            None
        }
    }
}
// !SECTION

impl PrimIntSet {
    /// Returns the index of the given `PrimInt` in PRIM_INTS, corresponding to the
    /// index of the `Rank` in the `ranks` array-order.
    ///
    /// # Example
    ///
    /// ```
    /// # use doodle::base_set::PrimIntSet;
    /// use doodle::numeric::elaborator::PRIM_INTS;
    /// for (ix, prim) in PRIM_INTS.iter().enumerate() {
    ///     assert_eq!(ix, PrimIntSet::prim_as_index(*prim));
    /// }
    /// ```
    pub const fn prim_as_index(prim: PrimInt) -> usize {
        match prim {
            PrimInt::U8 => 0,
            PrimInt::U16 => 1,
            PrimInt::U32 => 2,
            PrimInt::U64 => 3,
            PrimInt::I8 => 4,
            PrimInt::I16 => 5,
            PrimInt::I32 => 6,
            PrimInt::I64 => 7,
        }
    }
}

impl BaseType {
    pub fn int_width(&self) -> BitWidth {
        match self {
            BaseType::U8 => BitWidth::Bits8,
            BaseType::U16 => BitWidth::Bits16,
            BaseType::U32 => BitWidth::Bits32,
            BaseType::U64 => BitWidth::Bits64,
            _ => unreachable!("cannot measure int-width of non-integral BaseType {self:?}"),
        }
    }
}

mod __impl {
    use super::*;
    use crate::BaseType;
    use crate::numeric::elaborator::PrimInt;

    impl std::fmt::Display for BaseSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                BaseSet::Single(t) => write!(f, "{{ {t:?} }}"),
                BaseSet::U(ranked_set) => ranked_set.fmt(f),
            }
        }
    }

    impl std::fmt::Display for IntSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                IntSet::Single(t) => write!(f, "ℤ {{ {} }}", t.to_static_str()),
                IntSet::Z(ranked_set) => write!(f, "ℤ {}", ranked_set),
            }
        }
    }

    #[derive(Debug)]
    pub struct TryFromPrimIntError(PrimInt);

    impl std::fmt::Display for TryFromPrimIntError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "unable to convert prim-int to base-type {}", self.0)
        }
    }

    impl std::error::Error for TryFromPrimIntError {}

    impl TryFrom<PrimInt> for BaseType {
        type Error = TryFromPrimIntError;

        fn try_from(value: PrimInt) -> Result<Self, Self::Error> {
            match value {
                PrimInt::U8 => Ok(BaseType::U8),
                PrimInt::U16 => Ok(BaseType::U16),
                PrimInt::U32 => Ok(BaseType::U32),
                PrimInt::U64 => Ok(BaseType::U64),
                _ => Err(TryFromPrimIntError(value)),
            }
        }
    }

    #[derive(Debug)]
    pub struct TryFromBaseTypeError(BaseType);

    impl std::fmt::Display for TryFromBaseTypeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "unable to convert non-numeric base-type {}", self.0)
        }
    }

    impl std::error::Error for TryFromBaseTypeError {}

    impl TryFrom<BaseType> for PrimInt {
        type Error = TryFromBaseTypeError;

        fn try_from(value: BaseType) -> Result<Self, Self::Error> {
            match value {
                BaseType::U8 => Ok(PrimInt::U8),
                BaseType::U16 => Ok(PrimInt::U16),
                BaseType::U32 => Ok(PrimInt::U32),
                BaseType::U64 => Ok(PrimInt::U64),
                BaseType::Char | BaseType::Bool => Err(TryFromBaseTypeError(value)),
            }
        }
    }

    impl std::fmt::Debug for UintSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RankedUintSet")
                .field("ranks", &self.ranks)
                .finish()
        }
    }

    impl std::fmt::Display for UintSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let this = self.normalize();
            if this.is_empty() {
                write!(f, "{{}}")
            } else {
                write!(f, "{{ ")?;
                let labels = ["U8", "U16", "U32", "U64"];
                let mut ix_ranks = this
                    .ranks
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<(usize, Rank)>>();
                ix_ranks.sort_by(|(_, r1), (_, r2)| r1.cmp(r2).reverse());
                let mut last_rank = None;
                for (ix, r) in ix_ranks {
                    if r.is_excluded() {
                        break;
                    }
                    match last_rank {
                        None => {
                            write!(f, "{}", labels[ix])?;
                        }
                        Some(r0) => {
                            if r < r0 {
                                write!(f, " > {}", labels[ix])?;
                            } else {
                                write!(f, ", {}", labels[ix])?;
                            }
                        }
                    }
                    last_rank = Some(r);
                }
                write!(f, " }}")
            }
        }
    }

    impl std::fmt::Debug for PrimIntSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut dbg = f.debug_struct("PrimIntSet");
            for (ix, p) in PRIM_INTS.iter().enumerate() {
                if self.ranks[ix] == Rank::Excluded {
                    continue;
                }
                dbg.field(p.to_static_str(), &self.ranks[ix]);
            }
            dbg.finish()
        }
    }

    impl std::fmt::Display for PrimIntSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let this = self.normalize();
            if this.is_empty() {
                write!(f, "{{}}")
            } else {
                write!(f, "{{ ")?;
                let labels = ["U8", "U16", "U32", "U64", "I8", "I16", "I32", "I64"];
                let mut ix_ranks = this
                    .ranks
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<(usize, Rank)>>();
                ix_ranks.sort_by(|(_, r1), (_, r2)| r1.cmp(r2).reverse());
                let mut last_rank = None;
                for (ix, r) in ix_ranks {
                    if r.is_excluded() {
                        break;
                    }
                    match last_rank {
                        None => {
                            write!(f, "{}", labels[ix])?;
                        }
                        Some(r0) => {
                            if r < r0 {
                                write!(f, " > {}", labels[ix])?;
                            } else {
                                write!(f, ", {}", labels[ix])?;
                            }
                        }
                    }
                    last_rank = Some(r);
                }
                write!(f, " }}")
            }
        }
    }

    impl BitWidth {
        pub const MAX8: usize = u8::MAX as usize;
        pub const MAX16: usize = u16::MAX as usize;
        pub const MAX32: usize = u32::MAX as usize;
        pub const MAX64: usize = u64::MAX as usize;

        pub fn reify_as_base(self) -> BaseType {
            match self {
                BitWidth::Bits8 => BaseType::U8,
                BitWidth::Bits16 => BaseType::U16,
                BitWidth::Bits32 => BaseType::U32,
                BitWidth::Bits64 => BaseType::U64,
            }
        }
    }

    impl crate::Bounds {
        pub fn min_required_width(&self) -> BitWidth {
            let max = self.max.unwrap_or(self.min);
            match () {
                _ if max <= BitWidth::MAX8 => BitWidth::Bits8,
                _ if max <= BitWidth::MAX16 => BitWidth::Bits16,
                _ if max <= BitWidth::MAX32 => BitWidth::Bits32,
                _ => BitWidth::Bits64,
            }
        }
    }
}
pub(crate) use __impl::{TryFromBaseTypeError, TryFromPrimIntError};
