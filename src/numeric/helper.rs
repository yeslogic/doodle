//! Convenience functions for constructing numeric expressions
use super::core::{BasicBinOp, BasicUnaryOp, BinOp, Expr, MachineRep, TypedConst, UnaryOp};
use crate::{IntoLabel, numeric::core::Number};

use super::core::CastOp;

/// Helper for `Expr::NumVar`.
pub fn num_var(name: impl IntoLabel) -> Expr {
    Expr::NumVar(name.into())
}

/// Helper for `Expr::Const`
pub fn expr_const(c: TypedConst) -> Expr {
    Expr::Const(c)
}

/// Constructs an `Expr::Const` over a given numeric value, using an `Auto` representation.
pub fn auto_const<N>(n: N) -> Expr
where
    Number: From<N>,
{
    Expr::Const(TypedConst::new_auto(n))
}

/// Constructs an `Expr::Const` over a given numeric value, using the representation corresponding to the type of the value.
pub fn lift_const<Num>(c: Num) -> Expr
where
    Num: Into<TypedConst>,
{
    Expr::Const(c.into())
}

/// Helper for [`binop_with_rep`] using an `Auto` output representation.
pub fn binop_auto(op: BasicBinOp, lhs: Expr, rhs: Expr) -> Expr {
    binop_with_rep(op, None, lhs, rhs)
}

/// Constructs an `Expr::BinOp` with the specified operation, output representation, and operands.
pub fn binop_with_rep(op: BasicBinOp, out_rep: Option<MachineRep>, lhs: Expr, rhs: Expr) -> Expr {
    let binop = BinOp { op, out_rep };
    Expr::BinOp(binop, Box::new(lhs), Box::new(rhs))
}

/// Helper for [`unary_with_rep`] using an `Auto` output representation.
pub fn unary_auto(op: BasicUnaryOp, expr: Expr) -> Expr {
    unary_with_rep(op, None, expr)
}

/// Constructs an `Expr::UnaryOp` with the specified operation, output representation, and operand.
pub fn unary_with_rep(op: BasicUnaryOp, out_rep: Option<MachineRep>, expr: Expr) -> Expr {
    let unop = UnaryOp { op, out_rep };
    Expr::UnaryOp(unop, Box::new(expr))
}

/// Constructs an `Expr::Cast` (arithmetic cast-semantics) of the given expression to the specified representation.
pub fn cast(rep: MachineRep, expr: Expr) -> Expr {
    let op = CastOp::arith(rep);
    Expr::Cast(op, Box::new(expr))
}

/// Constructs an `Expr::Cast` (bitwise cast-semantics) of the given expression to the specified representation.
pub fn cast_bitwise(rep: MachineRep, expr: Expr) -> Expr {
    let op = CastOp::bitwise(rep);
    Expr::Cast(op, Box::new(expr))
}
