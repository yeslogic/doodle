use std::borrow::Cow;
use std::rc::Rc;

use crate::codegen::ToFragment;
use crate::codegen::typed_format::GenType;
use crate::output::Fragment;
use crate::precedence::{Precedence, cond_paren};

use crate::numeric::codegen::synthesize;
use crate::numeric::core::{BasicBinOp, BasicUnaryOp, BinOp, Expr, MachineRep, NumRep, TypedConst, UnaryOp};
use crate::typecheck::inference::InferenceEngine;
use crate::numeric::elaborator::{Elaborator, IntType, MapType, TypedBinOp, TypedCast, TypedExpr, TypedUnaryOp};

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
    let token = unary_op.inner.get_op().to_static_str();
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
    rep: MachineRep,
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
        BasicUnaryOp::IntSucc => "succ",
        BasicUnaryOp::IntPred => "pred",
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
pub(crate) fn compile_expr(expr: &Expr, prec: Precedence) -> Fragment {
    match expr {
        Expr::Const(typed_const) => Fragment::DisplayAtom(Rc::new(typed_const.clone())),
        Expr::BinOp(op, lhs, rhs) => match op.get_op() {
            BasicBinOp::Add | BasicBinOp::Sub => cond_paren(
                compile_binop(*op, lhs, rhs, Precedence::ADD_SUB, Precedence::ADD_SUB),
                prec,
                Precedence::ADD_SUB,
            ),
            BasicBinOp::Mul => cond_paren(
                compile_binop(*op, lhs, rhs, Precedence::MUL, Precedence::MUL),
                prec,
                Precedence::MUL,
            ),
            BasicBinOp::Div | BasicBinOp::Rem => cond_paren(
                compile_binop(*op, lhs, rhs, Precedence::DIV_REM, Precedence::DIV_REM),
                prec,
                Precedence::DIV_REM,
            ),
        },
        Expr::UnaryOp(unary_op, expr) => cond_paren(
            compile_prefix(unary_op, expr, Precedence::UNARY),
            prec,
            Precedence::UNARY,
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
    rep: MachineRep,
    inner: &TypedExpr<IntType>,
    inner_prec: Precedence,
) -> Fragment {
    if MachineRep::from(t) == rep {
        // If the int-type is directly analogous to the original rep, omit the type-annotation
        compile_typed_expr(inner, inner_prec)
            .cat(Fragment::String(Cow::Borrowed(" as ")))
            .cat(Fragment::String(Cow::Borrowed(rep.to_static_str())))
    } else {
        // NOTE - this would normally be a panic but we are content to observe it without failing
        eprintln!(
            "[WARNING]: Postfix operation (assumed to be Cast) has mismatched NumRep and IntType..."
        );
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
                    Precedence::ADD_SUB,
                    Precedence::ADD_SUB,
                ),
                prec,
                Precedence::ADD_SUB,
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
                    Precedence::DIV_REM,
                    Precedence::DIV_REM,
                ),
                prec,
                Precedence::DIV_REM,
            ),
        },
        TypedExpr::ElabUnaryOp(t, typed_unary_op, inner) => cond_paren(
            compile_elab_prefix(*t, typed_unary_op, inner, Precedence::UNARY),
            prec,
            Precedence::UNARY,
        ),
        TypedExpr::ElabCast(t, TypedCast { rep: _rep, .. }, inner) => cond_paren(
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
    let expr = expr.clone().map_type(&GenType::from);
    let ast = synthesize(&expr);
    format!("{}", ast.to_fragment())
}

pub fn print_conversion(expr: &Expr) {
    let mut ie = InferenceEngine::new();
    match ie.infer_var_expr(expr) {
        Ok(_) => {
            let mut elab = Elaborator::new(Box::new(ie));
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
    use num_bigint::BigInt;
    use crate::numeric::core::*;

    #[test]
    fn test_print_conversion() {
        let expr = {
            Expr::BinOp(
                BinOp::new(BasicBinOp::Add, None),
                Box::new(Expr::BinOp(
                    BinOp::new(BasicBinOp::Add, Some(MachineRep::U32)),
                    Box::new(Expr::Const(TypedConst(BigInt::from(10), NumRep::U32))),
                    Box::new(Expr::Const(TypedConst(BigInt::from(-1), NumRep::I32))),
                )),
                Box::new(Expr::Const(TypedConst(BigInt::from(5), NumRep::Auto))),
            )
        };
        print_conversion(&expr)
    }
}
