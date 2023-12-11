use crate::output::Fragment;

/// Operator Precedence classes
///
///
#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum Precedence {
    Atomic, // Highest precedence
    Projection,
    Prefix, // Highest natural precedence
    ArithInfix(ArithLevel),
    BitwiseInfix(BitwiseLevel),
    Comparison(CompareLevel), // Unsound when chained
    Calculus,                 // Arrow and Match
    #[default]
    Top,        // Entry level for neutral context
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum CompareLevel {
    Comparison = 0, // Highest comparative precedence
    Equality,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ArithLevel {
    DivRem = 0, // Highest arithmetic precedence
    Mul,
    AddSub,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub(crate) enum BitwiseLevel {
    Shift = 0, // Highest bitwise precedence
    And = 1,
    Or = 2,
}

/// Intransitive partial relation over operator subclasses
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Relation {
    /// `.<`
    Inferior,
    /// `.=`
    Congruent,
    /// `.>`
    Superior,
    /// `><`
    Disjoint,
}

pub(crate) trait IntransitiveOrd {
    fn relate(&self, other: &Self) -> Relation;

    fn inferior(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Inferior)
    }

    fn superior(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Superior)
    }

    fn congruent(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Congruent)
    }

    fn disjoint(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Disjoint)
    }
}

impl IntransitiveOrd for CompareLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (Self::Comparison, Self::Comparison) | (Self::Equality, Self::Equality) => {
                Relation::Congruent
            }
            (Self::Comparison, Self::Equality) => Relation::Disjoint,
            (Self::Equality, Self::Comparison) => Relation::Disjoint,
        }
    }
}

impl IntransitiveOrd for ArithLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (Self::DivRem, Self::DivRem)
            | (Self::Mul, Self::Mul)
            | (Self::AddSub, Self::AddSub) => Relation::Congruent,
            (Self::DivRem, Self::Mul) | (Self::Mul, Self::DivRem) => Relation::Disjoint,
            (Self::AddSub, _) => Relation::Inferior,
            (_, Self::AddSub) => Relation::Superior,
        }
    }
}

impl IntransitiveOrd for BitwiseLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (BitwiseLevel::Shift, BitwiseLevel::Shift) => Relation::Congruent,
            (BitwiseLevel::Shift, _) => Relation::Superior,
            (_, BitwiseLevel::Shift) => Relation::Inferior,
            (BitwiseLevel::And, BitwiseLevel::And) => Relation::Congruent,
            (BitwiseLevel::And, BitwiseLevel::Or) => Relation::Superior,
            (BitwiseLevel::Or, BitwiseLevel::And) => Relation::Inferior,
            (BitwiseLevel::Or, BitwiseLevel::Or) => Relation::Congruent,
        }
    }
}

/// Rules:
///   x .= x
///   Atomic .> Proj .> Prefix .> *Infix .> Comparison .> Calculus .> Top
///   rel(x, y) = rel(ArithInfix(x), ArithInfix(y))
///   rel(x, y) = rel(BitwiseInfix(x), BitwiseInfix(y))
///   Bitwise(_) >< Arith(_)
impl IntransitiveOrd for Precedence {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            // Trivial Congruences
            (Self::Atomic, Self::Atomic) => Relation::Congruent,
            (Self::Projection, Self::Projection) => Relation::Congruent,
            (Self::Prefix, Self::Prefix) => Relation::Congruent,
            (Self::Calculus, Self::Calculus) => Relation::Congruent,
            (Self::Top, Self::Top) => Relation::Congruent,

            // Descending relations
            (Self::Atomic, _) => Relation::Superior,
            (_, Self::Atomic) => Relation::Superior,
            (Self::Projection, _) => Relation::Superior,
            (_, Self::Projection) => Relation::Inferior,
            (Self::Prefix, _) => Relation::Superior,
            (_, Self::Prefix) => Relation::Inferior,

            // Ascending relations
            (Self::Top, _) => Relation::Inferior,
            (_, Self::Top) => Relation::Superior,
            (Self::Calculus, _) => Relation::Inferior,
            (_, Self::Calculus) => Relation::Superior,

            // Implications
            (Self::ArithInfix(x), Self::ArithInfix(y)) => x.relate(y),
            (Self::BitwiseInfix(x), Self::BitwiseInfix(y)) => x.relate(y),
            (Self::Comparison(x), Self::Comparison(y)) => x.relate(y),

            // Ascending relations (continued)
            (Self::Comparison(_), _) => Relation::Inferior,
            (_, Self::Comparison(_)) => Relation::Superior,

            // Disjunctions
            (Self::ArithInfix(_), Self::BitwiseInfix(_)) => Relation::Disjoint,
            (Self::BitwiseInfix(_), Self::ArithInfix(_)) => Relation::Disjoint,
        }
    }
}

impl Precedence {
    #![allow(dead_code)]
    pub(crate) const TOP: Self = Precedence::Top;
    pub(crate) const ARROW: Self = Precedence::Calculus;
    pub(crate) const MATCH: Self = Precedence::Calculus;
    pub(crate) const COMPARE: Self = Precedence::Comparison(CompareLevel::Comparison);
    pub(crate) const EQUALITY: Self = Precedence::Comparison(CompareLevel::Equality);
    pub(crate) const BITOR: Self = Precedence::BitwiseInfix(BitwiseLevel::Or);
    pub(crate) const ADDSUB: Self = Precedence::ArithInfix(ArithLevel::AddSub);
    pub(crate) const BITAND: Self = Precedence::BitwiseInfix(BitwiseLevel::And);
    pub(crate) const DIVREM: Self = Precedence::ArithInfix(ArithLevel::DivRem);
    pub(crate) const MUL: Self = Precedence::ArithInfix(ArithLevel::Mul);
    pub(crate) const BITSHIFT: Self = Precedence::BitwiseInfix(BitwiseLevel::Shift);
    pub(crate) const FUNAPP: Self = Precedence::Prefix;
    pub(crate) const CAST_INFIX: Self = Precedence::Calculus;
    pub(crate) const CAST_PREFIX: Self = Precedence::Prefix;
    pub(crate) const PROJ: Self = Precedence::Projection;
    pub(crate) const ATOM: Self = Precedence::Atomic;

    pub(crate) const FORMAT_COMPOUND: Self = Self::Top;
    pub(crate) const FORMAT_ATOM: Self = Self::Atomic;

    pub(crate) fn bump_format(&self) -> Self {
        match self {
            Precedence::Top => Precedence::Atomic,
            Precedence::Atomic => Precedence::Atomic,
            _ => unreachable!("Unexpected non-format precedence level {self:?}"),
        }
    }
}

pub(crate) fn cond_paren(frag: Fragment, current: Precedence, cutoff: Precedence) -> Fragment {
    match current.relate(&cutoff) {
        Relation::Disjoint | Relation::Superior => {
            Fragment::Char('(').cat(frag).cat(Fragment::Char(')'))
        }
        Relation::Congruent | Relation::Inferior => frag,
    }
}
