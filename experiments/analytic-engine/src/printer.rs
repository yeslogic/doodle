use crate::core::{BasicBinOp, BasicUnaryOp, BinOp, Expr, UnaryOp};
use crate::elaborator::inference::InferenceEngine;
use crate::elaborator::{Elaborator, IntType, TypedBinOp, TypedExpr, TypedUnaryOp};

fn show_bin_op(bin_op: &BinOp) -> String {
    let token = match bin_op.get_op() {
        BasicBinOp::Add => "+",
        BasicBinOp::Sub => "-",
        BasicBinOp::Mul => "*",
        BasicBinOp::Div => "/",
        BasicBinOp::Rem => "%",
    };
    if let Some(rep) = bin_op.cast_rep() {
        format!("{}{}", token, rep)
    } else {
        format!("{}", token)
    }
}

fn show_typed_bin_op(bin_op: &TypedBinOp<IntType>) -> String {
    let token = match bin_op.inner.get_op() {
        BasicBinOp::Add => "+",
        BasicBinOp::Sub => "-",
        BasicBinOp::Mul => "*",
        BasicBinOp::Div => "/",
        BasicBinOp::Rem => "%",
    };
    format!(
        "({} : ({},{}) -> {})",
        token, bin_op.sig.0 .0, bin_op.sig.0 .1, bin_op.sig.1
    )
}

fn show_typed_unary_op(unary_op: &TypedUnaryOp<IntType>) -> String {
    let token = match unary_op.inner.get_op() {
        BasicUnaryOp::Negate => "~",
        BasicUnaryOp::AbsVal => "abs",
    };
    format!("({} : {} -> {})", token, unary_op.sig.0, unary_op.sig.1)
}

fn show_unary_op(unary_op: &UnaryOp) -> String {
    let token = match unary_op.get_op() {
        BasicUnaryOp::Negate => "~",
        BasicUnaryOp::AbsVal => "abs",
    };
    if let Some(rep) = unary_op.cast_rep() {
        format!("{}{}", token, rep)
    } else {
        format!("{}", token)
    }
}

// FIXME - adopt pretty-printing engine with precedence rules
pub fn show_expr(expr: &Expr) -> String {
    match expr {
        Expr::Const(typed_const) => format!("{}", typed_const),
        Expr::BinOp(bin_op, expr, expr1) => format!(
            "({} {} {})",
            show_expr(expr),
            show_bin_op(bin_op),
            show_expr(expr1)
        ),
        Expr::UnaryOp(unary_op, expr) => {
            format!("{}({})", show_unary_op(unary_op), show_expr(expr))
        }
        Expr::Cast(num_rep, expr) => format!("{} as {}", show_expr(expr), num_rep),
    }
}

// FIXME - adopt pretty-printing engine with precedence rules
fn show_typed_expr(t_expr: &TypedExpr<IntType>) -> String {
    match t_expr {
        TypedExpr::ElabConst(t, typed_const) => {
            format!("({}: {})", typed_const, t)
        }
        TypedExpr::ElabBinOp(t, typed_bin_op, typed_expr, typed_expr1) => {
            format!(
                "({} {} {} : {})",
                show_typed_expr(typed_expr),
                show_typed_bin_op(typed_bin_op),
                show_typed_expr(typed_expr1),
                t
            )
        }
        TypedExpr::ElabUnaryOp(t, typed_unary_op, typed_expr) => {
            format!(
                "({}({}) : {})",
                show_typed_unary_op(typed_unary_op),
                show_typed_expr(typed_expr),
                t
            )
        }
        TypedExpr::ElabCast(t, num_rep, typed_expr) => {
            format!("({} as {} : {})", show_typed_expr(typed_expr), num_rep, t)
        }
    }
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
