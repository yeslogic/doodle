use std::borrow::Cow;
use std::rc::Rc;

use fragment::Fragment;
use precedence::{cond_paren, Precedence};

use crate::core::{BasicBinOp, BasicUnaryOp, BinOp, Expr, NumRep, TypedConst, UnaryOp};
use crate::elaborator::inference::InferenceEngine;
use crate::elaborator::{Elaborator, IntType, TypedBinOp, TypedCast, TypedExpr, TypedUnaryOp};
use crate::gen::{synthesize, ToFragment};

pub(crate) mod fragment {
    use std::borrow::Cow;
    use std::fmt::Write as _;
    use std::rc::Rc;

    #[derive(Clone, Default)]
    pub enum Fragment {
        #[default]
        Empty,
        Char(char),
        String(Cow<'static, str>),
        DisplayAtom(Rc<dyn std::fmt::Display>),
        Cat(Box<Fragment>, Box<Fragment>),
        Sequence {
            sep: Option<Box<Fragment>>,
            items: Vec<Fragment>,
        },
    }

    impl std::fmt::Debug for Fragment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "Empty"),
                Self::Char(c) => f.debug_tuple("Char").field(c).finish(),
                Self::String(s) => f.debug_tuple("String").field(s).finish(),
                Self::DisplayAtom(at) => f
                    .debug_tuple("DisplayAtom")
                    .field(&format!("{}", at))
                    .finish(),
                Self::Cat(x, y) => f.debug_tuple("Cat").field(x).field(y).finish(),
                Self::Sequence { sep, items } => f
                    .debug_struct("Sequence")
                    .field("sep", sep)
                    .field("items", items)
                    .finish(),
            }
        }
    }

    impl Fragment {
        pub fn cat(self, frag1: Self) -> Self {
            Self::Cat(Box::new(self), Box::new(frag1))
        }

        pub fn delimit(self, before: Self, after: Self) -> Self {
            Self::cat(before, self).cat(after)
        }

        pub fn seq(items: impl IntoIterator<Item = Fragment>, sep: Option<Fragment>) -> Self {
            Self::Sequence {
                items: items.into_iter().collect(),
                sep: sep.map(Box::new),
            }
        }
    }

    impl std::fmt::Display for Fragment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Fragment::Empty => Ok(()),
                Fragment::Char(c) => f.write_char(*c),
                Fragment::String(s) => f.write_str(s.as_ref()),
                Fragment::DisplayAtom(atom) => std::fmt::Display::fmt(&atom, f),
                Fragment::Cat(frag0, frag1) => {
                    frag0.fmt(f)?;
                    frag1.fmt(f)
                }
                Fragment::Sequence { sep, items } => {
                    let mut iter = items.iter();
                    if let Some(head) = iter.next() {
                        head.fmt(f)?;
                    } else {
                        return Ok(());
                    }
                    let f_sep: Box<dyn Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result> =
                        if let Some(frag) = sep.as_deref() {
                            Box::new(|f| frag.fmt(f))
                        } else {
                            Box::new(|_| Ok(()))
                        };
                    for item in iter {
                        f_sep(f)?;
                        item.fmt(f)?;
                    }
                    Ok(())
                }
            }
        }
    }
}

pub(crate) mod precedence {
    use super::fragment::Fragment;

    #[derive(Copy, Clone, Debug, Default)]
    pub(crate) enum Precedence {
        /// Highest precedence, as if implicitly (if not actually) parenthesized
        #[allow(dead_code)]
        Atomic,
        /// Highest natural precedence - used for unary operations and type-casts
        Mono(MonoLevel),
        /// Infix arithmetic operation of the designated arithmetic sub-precedence
        Arith(ArithLevel),
        /// Lowest natural precedence - used when no particular precedence is required or known        #[default]
        #[default]
        Top,
    }

    #[derive(Copy, Clone, Debug)]
    pub(crate) enum MonoLevel {
        // AbsVal and Negate
        Prefix = 0,
        // Standalone type-casts
        Postfix,
    }

    #[derive(Copy, Clone, Debug)]
    pub(crate) enum ArithLevel {
        DivRem = 0, // Highest arithmetic Precedence
        Mul,
        AddSub,
    }

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

    /// Rules:
    ///   x .= x
    ///   Atomic .> Mono .> *Infix .> Top
    ///   rel(x, y) = rel(ArithInfix(x), ArithInfix(y))
    ///   rel(x, y) = rel(BitwiseInfix(x), BitwiseInfix(y))
    ///   rel(x, y) = rel(Calculus(x), Calculus(y))
    ///   Bitwise(_) >< Arith(_)
    impl IntransitiveOrd for Precedence {
        fn relate(&self, other: &Self) -> Relation {
            match (self, other) {
                // Trivial Congruences
                (Precedence::Atomic, Precedence::Atomic) => Relation::Congruent,
                (Precedence::Top, Precedence::Top) => Relation::Congruent,

                // Descending relations
                (Precedence::Atomic, _) => Relation::Superior,
                (_, Precedence::Atomic) => Relation::Superior,

                // Ascending relations
                (Precedence::Top, _) => Relation::Inferior,
                (_, Precedence::Top) => Relation::Superior,

                // Implications
                (Precedence::Mono(x), Precedence::Mono(y)) => x.relate(y),
                (Precedence::Arith(x), Precedence::Arith(y)) => x.relate(y),

                // Mixed Relations
                (Precedence::Mono(_), Precedence::Arith(_)) => Relation::Superior,
                (Precedence::Arith(_), Precedence::Mono(_)) => Relation::Inferior,
            }
        }
    }

    impl Precedence {
        pub(crate) const TOP: Self = Precedence::Top;
        pub(crate) const DIVREM: Self = Precedence::Arith(ArithLevel::DivRem);
        pub(crate) const MUL: Self = Precedence::Arith(ArithLevel::Mul);
        pub(crate) const ADDSUB: Self = Precedence::Arith(ArithLevel::AddSub);
        pub(crate) const ABSNEG: Self = Precedence::Mono(MonoLevel::Prefix);
        pub(crate) const CAST: Self = Precedence::Mono(MonoLevel::Postfix);

        #[allow(dead_code)]
        pub(crate) const ATOM: Self = Precedence::Atomic;
    }

    pub(crate) fn cond_paren(frag: Fragment, current: Precedence, cutoff: Precedence) -> Fragment {
        match current.relate(&cutoff) {
            Relation::Disjoint | Relation::Superior => {
                frag.delimit(Fragment::Char('('), Fragment::Char(')'))
            }
            Relation::Congruent | Relation::Inferior => frag,
        }
    }
}

fn compile_bin_op(bin_op: &BinOp) -> Fragment {
    let token = match bin_op.get_op() {
        BasicBinOp::Add => "+",
        BasicBinOp::Sub => "-",
        BasicBinOp::Mul => "*",
        BasicBinOp::Div => "/",
        BasicBinOp::Rem => "%",
    };
    if let Some(rep) = bin_op.cast_rep() {
        Fragment::cat(
            Fragment::String(Cow::Borrowed(token)),
            Fragment::String(Cow::Borrowed(rep.to_static_str())),
        )
    } else {
        Fragment::String(Cow::Borrowed(token))
    }
    .delimit(Fragment::Char(' '), Fragment::Char(' '))
}

fn compile_typed_bin_op(bin_op: &TypedBinOp<IntType>) -> Fragment {
    let token = match bin_op.inner.get_op() {
        BasicBinOp::Add => "+",
        BasicBinOp::Sub => "-",
        BasicBinOp::Mul => "*",
        BasicBinOp::Div => "/",
        BasicBinOp::Rem => "%",
    };
    let ((t_in_l, t_in_r), t_out) = bin_op.sig;
    if t_in_l == t_in_r && t_in_l == t_out {
        // If the input types and output type all agree, render as operator only
        Fragment::String(Cow::Borrowed(token))
    } else {
        Fragment::cat(
            Fragment::String(Cow::Borrowed(token)),
            Fragment::String(Cow::Owned(format!("@({},{})->{}", t_in_l, t_in_r, t_out))),
        )
    }
    .delimit(Fragment::Char(' '), Fragment::Char(' '))
}

fn compile_typed_unary_op(unary_op: &'_ TypedUnaryOp<IntType>) -> Fragment {
    let token = match unary_op.inner.get_op() {
        BasicUnaryOp::Negate => "~",
        BasicUnaryOp::AbsVal => "abs",
    };
    let (t_in, t_out) = unary_op.sig;
    if t_in == t_out {
        Fragment::String(Cow::Borrowed(token))
    } else {
        Fragment::String(Cow::Borrowed(token)).cat(Fragment::String(Cow::Owned(format!(
            "@{}->{}",
            t_in, t_out
        ))))
    }
    .cat(Fragment::Char(' '))
}

fn compile_prefix(op: &UnaryOp, inner: &Expr, inner_prec: Precedence) -> Fragment {
    Fragment::cat(compile_unary_op(op), compile_expr(inner, inner_prec))
}

fn compile_postfix<'a>(
    token: &'static str,
    rep: NumRep,
    inner: &Expr,
    inner_prec: Precedence,
) -> Fragment {
    Fragment::cat(
        compile_expr(inner, inner_prec),
        Fragment::cat(
            Fragment::String(Cow::Borrowed(token)),
            Fragment::String(Cow::Borrowed(rep.to_static_str())),
        ),
    )
}

fn compile_unary_op<'a>(unary_op: &UnaryOp) -> Fragment {
    let token = match unary_op.get_op() {
        BasicUnaryOp::Negate => "~",
        BasicUnaryOp::AbsVal => "abs",
    };
    if let Some(rep) = unary_op.cast_rep() {
        Fragment::cat(
            Fragment::String(Cow::Borrowed(token)),
            Fragment::String(Cow::Borrowed(rep.to_static_str())),
        )
    } else {
        Fragment::String(Cow::Borrowed(token))
    }
}

fn compile_binop(
    op: BinOp,
    lhs: &Expr,
    rhs: &Expr,
    lhs_prec: Precedence,
    rhs_prec: Precedence,
) -> Fragment {
    Fragment::delimit(
        compile_bin_op(&op),
        compile_expr(lhs, lhs_prec),
        compile_expr(rhs, rhs_prec),
    )
}

// FIXME - adopt pretty-printing engine with precedence rules
fn compile_expr(expr: &Expr, prec: Precedence) -> Fragment {
    match expr {
        Expr::Const(typed_const) => Fragment::DisplayAtom(Rc::new(typed_const.clone())),
        Expr::BinOp(op, lhs, rhs) => match op.get_op() {
            BasicBinOp::Add | BasicBinOp::Sub => cond_paren(
                compile_binop(*op, lhs, rhs, Precedence::ADDSUB, Precedence::ADDSUB),
                prec,
                Precedence::ADDSUB,
            ),
            BasicBinOp::Mul => cond_paren(
                compile_binop(*op, lhs, rhs, Precedence::MUL, Precedence::MUL),
                prec,
                Precedence::MUL,
            ),
            BasicBinOp::Div | BasicBinOp::Rem => cond_paren(
                compile_binop(*op, lhs, rhs, Precedence::DIVREM, Precedence::DIVREM),
                prec,
                Precedence::DIVREM,
            ),
        },
        Expr::UnaryOp(unary_op, expr) => cond_paren(
            compile_prefix(unary_op, expr, Precedence::ABSNEG),
            prec,
            Precedence::ABSNEG,
        ),
        Expr::Cast(num_rep, expr) => cond_paren(
            compile_postfix(" as ", *num_rep, expr, Precedence::CAST),
            prec,
            Precedence::CAST,
        ),
    }
}

fn show_expr(expr: &Expr) -> String {
    format!("{}", compile_expr(expr, Precedence::TOP))
}

fn compile_elab_const(t: IntType, typed_const: &TypedConst) -> Fragment {
    if NumRep::from(t) == typed_const.get_rep() {
        // If the int-type is directly analogous to the original rep, omit the type-annotation
        Fragment::DisplayAtom(Rc::new(typed_const.clone()))
    } else {
        Fragment::cat(
            Fragment::DisplayAtom(Rc::new(typed_const.clone())),
            Fragment::cat(
                Fragment::Char('@'),
                Fragment::String(Cow::Borrowed(t.to_static_str())),
            ),
        )
    }
}

fn compile_elab_binop(
    t: IntType,
    op: &TypedBinOp<IntType>,
    lhs: &TypedExpr<IntType>,
    rhs: &TypedExpr<IntType>,
    lhs_prec: Precedence,
    rhs_prec: Precedence,
) -> Fragment {
    compile_typed_expr(lhs, lhs_prec)
        .cat(compile_typed_bin_op(op))
        .cat(compile_typed_expr(rhs, rhs_prec))
        .cat(Fragment::String(Cow::Borrowed(" :: ")))
        .cat(Fragment::String(Cow::Borrowed(t.to_static_str())))
}

fn compile_elab_prefix(
    t: IntType,
    op: &TypedUnaryOp<IntType>,
    inner: &TypedExpr<IntType>,
    inner_prec: Precedence,
) -> Fragment {
    if t == op.sig.0 && op.sig.0 == op.sig.1 {
        compile_typed_unary_op(op).cat(compile_typed_expr(inner, inner_prec))
    } else {
        compile_typed_unary_op(op)
            .cat(compile_typed_expr(inner, inner_prec))
            .cat(Fragment::String(Cow::Borrowed(" :: ")))
            .cat(Fragment::String(Cow::Borrowed(t.to_static_str())))
    }
}

fn compile_elab_postfix<'a>(
    t: IntType,
    rep: NumRep,
    inner: &TypedExpr<IntType>,
    inner_prec: Precedence,
) -> Fragment {
    if NumRep::from(t) == rep {
        // If the int-type is directly analogous to the original rep, omit the type-annotation
        compile_typed_expr(inner, inner_prec)
            .cat(Fragment::String(Cow::Borrowed(" as ")))
            .cat(Fragment::String(Cow::Borrowed(rep.to_static_str())))
    } else {
        // NOTE - this would normally be a panic but we are content to observe it without failing
        eprintln!("[WARNING]: Postfix operation (assumed to be Cast) has mismatched NumRep and IntType...");
        compile_typed_expr(inner, inner_prec)
            .cat(Fragment::String(Cow::Borrowed(" as ")))
            .cat(Fragment::String(Cow::Borrowed(rep.to_static_str())))
            .cat(Fragment::String(Cow::Borrowed(" :: ")))
            .cat(Fragment::String(Cow::Borrowed(t.to_static_str())))
    }
}

// FIXME - adopt pretty-printing engine with precedence rules
fn compile_typed_expr(t_expr: &TypedExpr<IntType>, prec: Precedence) -> Fragment {
    match t_expr {
        TypedExpr::ElabConst(t, typed_const) => compile_elab_const(*t, typed_const),
        TypedExpr::ElabBinOp(t, typed_bin_op, lhs, rhs) => match typed_bin_op.inner.get_op() {
            BasicBinOp::Add | BasicBinOp::Sub => cond_paren(
                compile_elab_binop(
                    *t,
                    typed_bin_op,
                    lhs,
                    rhs,
                    Precedence::ADDSUB,
                    Precedence::ADDSUB,
                ),
                prec,
                Precedence::ADDSUB,
            ),
            BasicBinOp::Mul => cond_paren(
                compile_elab_binop(*t, typed_bin_op, lhs, rhs, Precedence::MUL, Precedence::MUL),
                prec,
                Precedence::MUL,
            ),
            BasicBinOp::Div | BasicBinOp::Rem => cond_paren(
                compile_elab_binop(
                    *t,
                    typed_bin_op,
                    lhs,
                    rhs,
                    Precedence::DIVREM,
                    Precedence::DIVREM,
                ),
                prec,
                Precedence::DIVREM,
            ),
        },
        TypedExpr::ElabUnaryOp(t, typed_unary_op, inner) => cond_paren(
            compile_elab_prefix(*t, typed_unary_op, inner, Precedence::ABSNEG),
            prec,
            Precedence::ABSNEG,
        ),
        TypedExpr::ElabCast(t, TypedCast { _rep, .. }, inner) => cond_paren(
            compile_elab_postfix(*t, *_rep, inner, Precedence::CAST),
            prec,
            Precedence::CAST,
        ),
    }
}

fn show_typed_expr(expr: &TypedExpr<IntType>) -> String {
    format!("{}", compile_typed_expr(expr, Precedence::TOP))
}

fn show_code(expr: &TypedExpr<IntType>) -> String {
    let ast = synthesize(expr);
    format!("{}", ast.to_fragment())
}

pub fn print_conversion(expr: &Expr) {
    let mut ie = InferenceEngine::new();
    match ie.infer_var_expr(expr) {
        Ok(_) => {
            let mut elab = Elaborator::new(ie);
            match elab.elaborate_expr(expr) {
                Ok(t_expr) => {
                    println!("Raw: {}", show_expr(expr));
                    println!("Elaborated: {}", show_typed_expr(&t_expr));
                    println!("Transcribed: {}", show_code(&t_expr));
                }
                Err(elab_err) => {
                    eprintln!(
                        "Error encountered during elaboration of `{}`: {}",
                        show_expr(expr),
                        elab_err
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Inference failed ({}) on {:?}", e, expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::TypedConst;

    #[test]
    fn test_print_conversion() {
        let expr = {
            Expr::BinOp(
                BinOp::new(BasicBinOp::Add, None),
                Box::new(Expr::BinOp(
                    BinOp::new(BasicBinOp::Add, Some(NumRep::U32)),
                    Box::new(Expr::Const(TypedConst(BigInt::from(10), NumRep::U32))),
                    Box::new(Expr::Const(TypedConst(BigInt::from(-1), NumRep::I32))),
                )),
                Box::new(Expr::Const(TypedConst(BigInt::from(5), NumRep::Auto))),
            )
        };
        print_conversion(&expr)
    }
}
