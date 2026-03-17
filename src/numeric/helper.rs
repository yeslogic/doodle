//! Convenience functions for constructing numeric expressions
use crate::numeric::core::{BasicBinOp, BasicUnaryOp, BinOp, Expr, MachineRep, TypedConst, UnaryOp};

/// Helper for `Expr::Const`
pub fn expr_const(c: TypedConst) -> Expr {
    Expr::Const(c)
}

pub fn lift_const<Num: Into<TypedConst>>(c: Num) -> Expr {
    Expr::Const(c.into())
}

pub fn binop_auto(op: BasicBinOp, lhs: Expr, rhs: Expr) -> Expr {
    binop_with_rep(op, None, lhs, rhs)
}

pub fn binop_with_rep(op: BasicBinOp, out_rep: Option<MachineRep>, lhs: Expr, rhs: Expr) -> Expr {
    let binop = BinOp { op, out_rep };
    Expr::BinOp(binop, Box::new(lhs), Box::new(rhs))
}

pub fn unary_auto(op: BasicUnaryOp, expr: Expr) -> Expr {
    unary_with_rep(op, None, expr)
}

pub fn unary_with_rep(op: BasicUnaryOp, out_rep: Option<MachineRep>, expr: Expr) -> Expr {
    let unop = UnaryOp { op, out_rep };
    Expr::UnaryOp(unop, Box::new(expr))
}

pub fn cast(rep: MachineRep, expr: Expr) -> Expr {
    Expr::Cast(rep, Box::new(expr))
}
