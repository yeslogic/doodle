use crate::output::Fragment;

/// Operator Precedence classes
///
/// A precedence class is used to determine the precedence of a given operator. This allows
/// for the most natural and readable rendering of operators in a given context, where
/// parentheses are only inserted where required to disambiguate the original expression
/// AST.
///
/// Rendering a term in the expression-algebra is always performed within a contextual
/// baseline precedence. By default, this baseline precedence is `Top`, which is inferior
/// to every other precedence class.
///
/// Any leaf term `T` without any operators or sub-expressions is considered to have the maximum
/// possible precedence, `Atomic`, which means that it is never parenthesized when rendered
/// as a term in any expression.
///
/// When rendering a term `O(T*)`, where `O` has inherent precedence `P`
/// and our current contextual precedence is `Q`, the rendering is determined by
/// the following rules:
///
///   - If `P .> Q`, then `O(T*)` is always rendered without parentheses.
///   - If `P .= Q`, then `O(T*)` is rendered with parentheses only if `O` is a non-associative operator.
///   - If `P .< Q` or `P >< Q`, then `O(T*)` is rendered with parentheses.
///
/// When rendering a term `O(T*)`, each sub-term `T` is rendered with a contextual baseline precedence
/// equal to the precedence `P` of the operator `O`.
#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum Precedence {
    /// Highest precedence, as if implicitly (if not actually) parenthesized
    Atomic,
    /// Post-fix projection such as method call, field access, or Try (`?`)
    Projection,
    /// Highest natural precedence - used for prefix operands such as borrow (&) and deref (*), as well as type-casts
    Mono(MonoLevel),
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum CalculusLevel {
    Invoke, // Highest calculus precedence
    Lambda,
    Match,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum CompareLevel {
    Comparison = 0, // Highest comparative precedence
    Equality,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum LogicalLevel {
    And = 0,
    Or = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum BitwiseLevel {
    Shift = 0, // Highest bitwise precedence
    And = 1,
    Or = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ArithLevel {
    DivRem = 0, // Highest arithmetic precedence
    Mul,
    AddSub,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum MonoLevel {
    // Any prefix operator (unary op, deref, borrow)
    Prefix = 0,
    // Standalone type-casts
    Postfix,
}


/// Intransitive partial relation over operator subclasses
///
/// The relation is not transitive, but it is anti-symmetric.
///
/// Given an operator `Op0` with inherent rank `R0` and an operator `Op1` with inherent rank `R1`,
/// we ascribe the following relations to `R0` and `R1`:
///
/// - If `X Op0 Y Op1 Z` is unambiguously parsed as `(X Op0 Y) Op1 Z`, then `R0 .> R1` (and `R1 .< R0`).
/// - If `X Op0 Y Op1 Z` is unambiguously parsed as `X Op0 (Y Op1 Z)`, then `R0 .< R1` (and `R1 .> R0`).
/// - If `X Op0 Y Op1 Z` has no natural interpretation or neither grouping is meaningful, then `R0 >< R1` (and `R1 >< R0`).
/// - If `(X Op0 Y) Op1 Z` and `X Op0 (Y Op1 Z)` are treated interchangeably, then `R0 .= R1` (and `R1 .= R0`).
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

    #[expect(dead_code)]
    fn congruent(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Congruent)
    }

    #[expect(dead_code)]
    fn inferior(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Inferior)
    }

    #[expect(dead_code)]
    fn superior(&self, other: &Self) -> bool {
        matches!(self.relate(other), Relation::Superior)
    }

    #[expect(dead_code)]
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
            (Self::AddSub, Self::DivRem | Self::Mul) => Relation::Inferior,
            (Self::DivRem | Self::Mul, Self::AddSub) => Relation::Superior,
        }
    }
}

impl IntransitiveOrd for MonoLevel {
    fn relate(&self, other: &Self) -> Relation {
        match (self, other) {
            (Self::Prefix, Self::Prefix) | (Self::Postfix, Self::Postfix) => {
                Relation::Congruent
            }
            (Self::Prefix, Self::Postfix) => Relation::Superior,
            (Self::Postfix, Self::Prefix) => Relation::Inferior,
        }
    }
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
            (Self::Top, Self::Top) => Relation::Congruent,

            // Descending relations
            (Self::Atomic, _) => Relation::Superior,
            (_, Self::Atomic) => Relation::Inferior,
            (Self::Projection, _) => Relation::Superior,
            (_, Self::Projection) => Relation::Inferior,

            // Ascending relations
            (Self::Top, _) => Relation::Inferior,
            (_, Self::Top) => Relation::Superior,

            // Implications
            (Self::Calculus(x), Self::Calculus(y)) => x.relate(y),
            (Self::ArithInfix(x), Self::ArithInfix(y)) => x.relate(y),
            (Self::BitwiseInfix(x), Self::BitwiseInfix(y)) => x.relate(y),
            (Self::LogicalInfix(x), Self::LogicalInfix(y)) => x.relate(y),
            (Self::Comparison(x), Self::Comparison(y)) => x.relate(y),
            (Self::Mono(x), Self::Mono(y)) => x.relate(y),

            // Ascending relations (continued)
            (Self::Calculus(_), _) => Relation::Inferior,
            (_, Self::Calculus(_)) => Relation::Superior,
            (Self::Comparison(_), _) => Relation::Inferior,
            (_, Self::Comparison(_)) => Relation::Superior,

            (Self::Mono(_), _) => Relation::Superior,
            (_, Self::Mono(_)) => Relation::Inferior,

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
    pub(crate) const TOP: Self = Precedence::Top;
    pub(crate) const ARROW: Self = Precedence::Calculus(CalculusLevel::Lambda);
    pub(crate) const MATCH: Self = Precedence::Calculus(CalculusLevel::Match);
    pub(crate) const INVOKE: Self = Precedence::Calculus(CalculusLevel::Invoke);
    pub(crate) const COMPARE: Self = Precedence::Comparison(CompareLevel::Comparison);
    pub(crate) const EQUALITY: Self = Precedence::Comparison(CompareLevel::Equality);
    pub(crate) const BITOR: Self = Precedence::BitwiseInfix(BitwiseLevel::Or);
    pub(crate) const ADD_SUB: Self = Precedence::ArithInfix(ArithLevel::AddSub);
    pub(crate) const DIV_REM: Self = Precedence::ArithInfix(ArithLevel::DivRem);
    pub(crate) const BITAND: Self = Precedence::BitwiseInfix(BitwiseLevel::And);
    pub(crate) const LOGICAL_AND: Self = Precedence::LogicalInfix(LogicalLevel::And);
    pub(crate) const LOGICAL_OR: Self = Precedence::LogicalInfix(LogicalLevel::Or);
    pub(crate) const LOGICAL_NEGATE: Self = Precedence::Mono(MonoLevel::Prefix);
    pub(crate) const NUMERIC_PREFIX: Self = Precedence::Mono(MonoLevel::Prefix);
    pub(crate) const MUL: Self = Precedence::ArithInfix(ArithLevel::Mul);
    pub(crate) const BIT_SHIFT: Self = Precedence::BitwiseInfix(BitwiseLevel::Shift);
    pub(crate) const FUN_APPLICATION: Self = Precedence::Mono(MonoLevel::Prefix);
    pub(crate) const CAST_INFIX: Self = Precedence::Calculus(CalculusLevel::Invoke);
    pub(crate) const CAST_PREFIX: Self = Precedence::Mono(MonoLevel::Prefix);
    pub(crate) const PROJ: Self = Precedence::Projection;
    pub(crate) const ATOM: Self = Precedence::Atomic;


    pub(crate) const PTR_PREFIX: Self = Precedence::Mono(MonoLevel::Prefix);
    // NOTE - ported from crate::numeric::printer
    pub(crate) const UNARY: Self = Precedence::Mono(MonoLevel::Prefix);
    pub(crate) const CAST: Self = Precedence::Mono(MonoLevel::Postfix);

    pub(crate) const FORMAT_COMPOUND: Self = Self::Top;

    // REVIEW - does list-append need its own precedence or is this good enough?
    pub(crate) const APPEND: Self = Precedence::ArithInfix(ArithLevel::AddSub);

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
