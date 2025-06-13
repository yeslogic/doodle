use crate::output::Fragment;

/// Operator Precedence classes
///
///
#[derive(Copy, Clone, Debug, Default)]
pub enum Precedence {
    /// Highest precedence, as if implicitly (if not actually) parenthesized
    Atomic,
    /// Post-fix projection such as method call, field access, or Try (`?`)
    Projection,
    /// Highest natural precedence - used for prefix operands such as borrow (&) and deref (*)
    Prefix,
    /// Infix arithmetic operation of the designated arithmetic sub-precedence
    ArithInfix(ArithLevel),
    /// Infix bitwise operation of the designated bitwise sub-precedence
    BitwiseInfix(BitwiseLevel),
    /// Infix logical operation of the designated logical sub-precedence
    LogicalInfix(LogicalLevel),
    /// Unchainable quantitative comparison, such as inequality and equality operations
    Comparison(CompareLevel),
    /// Functional abstractions such as `match` expressions, lambda abstractions, and invocations of anonymous functions
    Calculus(CalculusLevel),
    /// Lowest natural precedence - used for individual operands in the parameter list of a function call, or when no particular precedence is required or known
    #[default]
    Top,
}

#[derive(Copy, Clone, Debug)]
pub enum CalculusLevel {
    Invoke, // Highest calculus precedence
    Lambda,
    Match,
}

#[derive(Copy, Clone, Debug)]
pub enum CompareLevel {
    Comparison = 0, // Highest comparative precedence
    Equality,
}

#[derive(Copy, Clone, Debug)]
pub enum ArithLevel {
    DivRem = 0, // Highest arithmetic precedence
    Mul,
    AddSub,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum BitwiseLevel {
    Shift = 0, // Highest bitwise precedence
    And = 1,
    Or = 2,
}

#[derive(Copy, Clone, Debug)]
pub enum LogicalLevel {
    And = 0,
    Or = 1,
}

impl IntransitiveOrd for LogicalLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (LogicalLevel::And, LogicalLevel::And) => Relation::Congruent,
            (LogicalLevel::And, LogicalLevel::Or) => Relation::Superior,
            (LogicalLevel::Or, LogicalLevel::And) => Relation::Inferior,
            (LogicalLevel::Or, LogicalLevel::Or) => Relation::Congruent,
        }
    }
}

/// Intransitive partial relation over operator subclasses
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Relation {
    /// `.<`
    Inferior,
    /// `.=`
    Congruent,
    /// `.>`
    Superior,
    /// `><`
    Disjoint,
}

pub trait IntransitiveOrd {
    fn relate(&self, other: &Self) -> Relation;
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

impl IntransitiveOrd for CalculusLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (Self::Invoke, Self::Invoke)
            | (Self::Lambda, Self::Lambda)
            | (Self::Match, Self::Match) => Relation::Congruent,
            (Self::Lambda, Self::Invoke) => Relation::Inferior,
            (Self::Invoke, Self::Lambda) => Relation::Superior,
            (Self::Lambda, Self::Match) | (Self::Match, Self::Lambda) => Relation::Congruent,
            (Self::Invoke, Self::Match) => Relation::Superior,
            (Self::Match, Self::Invoke) => Relation::Inferior,
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
///   rel(x, y) = rel(Calculus(x), Calculus(y))
///   Bitwise(_) >< Arith(_)
impl IntransitiveOrd for Precedence {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            // Trivial Congruences
            (Self::Atomic, Self::Atomic) => Relation::Congruent,
            (Self::Projection, Self::Projection) => Relation::Congruent,
            (Self::Prefix, Self::Prefix) => Relation::Congruent,
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

            // Implications
            (Self::Calculus(x), Self::Calculus(y)) => x.relate(y),
            (Self::ArithInfix(x), Self::ArithInfix(y)) => x.relate(y),
            (Self::BitwiseInfix(x), Self::BitwiseInfix(y)) => x.relate(y),
            (Self::LogicalInfix(x), Self::LogicalInfix(y)) => x.relate(y),
            (Self::Comparison(x), Self::Comparison(y)) => x.relate(y),

            // Ascending relations (continued)
            (Self::Calculus(_), _) => Relation::Inferior,
            (_, Self::Calculus(_)) => Relation::Superior,
            (Self::Comparison(_), _) => Relation::Inferior,
            (_, Self::Comparison(_)) => Relation::Superior,

            // Disjunctions
            (Self::ArithInfix(_), Self::BitwiseInfix(_)) => Relation::Disjoint,
            (Self::BitwiseInfix(_), Self::ArithInfix(_)) => Relation::Disjoint,

            (Self::LogicalInfix(_), Self::ArithInfix(_)) => Relation::Disjoint,
            (Self::ArithInfix(_), Self::LogicalInfix(_)) => Relation::Disjoint,

            (Self::LogicalInfix(_), Self::BitwiseInfix(_)) => Relation::Disjoint,
            (Self::BitwiseInfix(_), Self::LogicalInfix(_)) => Relation::Disjoint,
        }
    }
}

impl Precedence {
    pub const TOP: Self = Precedence::Top;
    pub const ARROW: Self = Precedence::Calculus(CalculusLevel::Lambda);
    pub const MATCH: Self = Precedence::Calculus(CalculusLevel::Match);
    pub const INVOKE: Self = Precedence::Calculus(CalculusLevel::Invoke);
    pub const COMPARE: Self = Precedence::Comparison(CompareLevel::Comparison);
    pub const EQUALITY: Self = Precedence::Comparison(CompareLevel::Equality);
    pub const BITOR: Self = Precedence::BitwiseInfix(BitwiseLevel::Or);
    pub const ADD_SUB: Self = Precedence::ArithInfix(ArithLevel::AddSub);
    pub const DIV_REM: Self = Precedence::ArithInfix(ArithLevel::DivRem);
    pub const BITAND: Self = Precedence::BitwiseInfix(BitwiseLevel::And);
    pub const LOGICAL_AND: Self = Precedence::LogicalInfix(LogicalLevel::And);
    pub const LOGICAL_OR: Self = Precedence::LogicalInfix(LogicalLevel::Or);
    pub const LOGICAL_NEGATE: Self = Precedence::Prefix;
    pub const NUMERIC_PREFIX: Self = Precedence::Prefix;
    pub const MUL: Self = Precedence::ArithInfix(ArithLevel::Mul);
    pub const BIT_SHIFT: Self = Precedence::BitwiseInfix(BitwiseLevel::Shift);
    pub const FUN_APPLICATION: Self = Precedence::Prefix;
    pub const CAST_INFIX: Self = Precedence::Calculus(CalculusLevel::Invoke);
    pub const CAST_PREFIX: Self = Precedence::Prefix;
    pub const PROJ: Self = Precedence::Projection;
    pub const ATOM: Self = Precedence::Atomic;

    pub const FORMAT_COMPOUND: Self = Self::Top;

    // REVIEW - does list-append need its own precedence or is this good enough?
    pub const APPEND: Self = Precedence::ArithInfix(ArithLevel::AddSub);

    pub fn bump_format(&self) -> Self {
        match self {
            Precedence::Top => Precedence::Atomic,
            Precedence::Atomic => Precedence::Atomic,
            _ => unreachable!("Unexpected non-format precedence level {self:?}"),
        }
    }
}

pub fn cond_paren(frag: Fragment, current: Precedence, cutoff: Precedence) -> Fragment {
    match current.relate(&cutoff) {
        Relation::Disjoint | Relation::Superior => {
            Fragment::Char('(').cat(frag).cat(Fragment::Char(')'))
        }
        Relation::Congruent | Relation::Inferior => frag,
    }
}
