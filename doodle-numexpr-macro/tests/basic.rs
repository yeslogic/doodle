use std::borrow::Cow;

use doodle::numeric::core::{
    BasicBinOp, BasicUnaryOp, BinOp, CastOp, Expr, MachineRep, NumRep, TypedConst, UnaryOp,
};
use doodle_numexpr_macro::numexpr;

#[test]
fn const_literal_defaults_to_auto_rep() {
    let actual = numexpr!(5);
    let expected = Expr::Const(TypedConst::new(5i64, NumRep::AUTO));
    assert_eq!(actual, expected);
}

#[test]
fn const_literal_with_suffix() {
    let actual = numexpr!(5u16);
    let expected = Expr::Const(TypedConst::new(5i64, NumRep::U16));
    assert_eq!(actual, expected);
}

#[test]
fn variable_reference() {
    let actual = numexpr!("x");
    let expected = Expr::NumVar(Cow::Borrowed("x"));
    assert_eq!(actual, expected);
}

#[test]
fn binop_precedence_and_left_associativity() {
    // "a" + "b" * "c"  ==  "a" + ("b" * "c")
    let actual = numexpr!("a" + "b" * "c");
    let expected = Expr::BinOp(
        BinOp::new(BasicBinOp::Add, None),
        Box::new(Expr::NumVar(Cow::Borrowed("a"))),
        Box::new(Expr::BinOp(
            BinOp::new(BasicBinOp::Mul, None),
            Box::new(Expr::NumVar(Cow::Borrowed("b"))),
            Box::new(Expr::NumVar(Cow::Borrowed("c"))),
        )),
    );
    assert_eq!(actual, expected);

    // "a" - "b" - "c"  ==  ("a" - "b") - "c"
    let actual = numexpr!("a" - "b" - "c");
    let expected = Expr::BinOp(
        BinOp::new(BasicBinOp::Sub, None),
        Box::new(Expr::BinOp(
            BinOp::new(BasicBinOp::Sub, None),
            Box::new(Expr::NumVar(Cow::Borrowed("a"))),
            Box::new(Expr::NumVar(Cow::Borrowed("b"))),
        )),
        Box::new(Expr::NumVar(Cow::Borrowed("c"))),
    );
    assert_eq!(actual, expected);
}

#[test]
fn binop_forced_out_rep() {
    let actual = numexpr!("a" +u16 "b");
    let expected = Expr::BinOp(
        BinOp::new(BasicBinOp::Add, Some(MachineRep::U16)),
        Box::new(Expr::NumVar(Cow::Borrowed("a"))),
        Box::new(Expr::NumVar(Cow::Borrowed("b"))),
    );
    assert_eq!(actual, expected);
}

#[test]
fn parenthesized_overrides_precedence() {
    let actual = numexpr!(("a" + "b") * "c");
    let expected = Expr::BinOp(
        BinOp::new(BasicBinOp::Mul, None),
        Box::new(Expr::BinOp(
            BinOp::new(BasicBinOp::Add, None),
            Box::new(Expr::NumVar(Cow::Borrowed("a"))),
            Box::new(Expr::NumVar(Cow::Borrowed("b"))),
        )),
        Box::new(Expr::NumVar(Cow::Borrowed("c"))),
    );
    assert_eq!(actual, expected);
}

#[test]
fn negative_literal_folds_into_const_sign() {
    let actual = numexpr!(-5);
    let expected = Expr::Const(TypedConst::new(-5i64, NumRep::AUTO));
    assert_eq!(actual, expected);

    let actual = numexpr!(-5u16);
    let expected = Expr::Const(TypedConst::new(-5i64, NumRep::U16));
    assert_eq!(actual, expected);
}

#[test]
fn negate_operator_on_non_literal() {
    let actual = numexpr!(-"x");
    let expected = Expr::UnaryOp(
        UnaryOp::new(BasicUnaryOp::Negate, None),
        Box::new(Expr::NumVar(Cow::Borrowed("x"))),
    );
    assert_eq!(actual, expected);

    let actual = numexpr!(-u16 "x");
    let expected = Expr::UnaryOp(
        UnaryOp::new(BasicUnaryOp::Negate, Some(MachineRep::U16)),
        Box::new(Expr::NumVar(Cow::Borrowed("x"))),
    );
    assert_eq!(actual, expected);
}

#[test]
fn unary_keyword_ops() {
    let actual = numexpr!(abs "x");
    let expected = Expr::UnaryOp(
        UnaryOp::new(BasicUnaryOp::AbsVal, None),
        Box::new(Expr::NumVar(Cow::Borrowed("x"))),
    );
    assert_eq!(actual, expected);

    let actual = numexpr!(succ u8 "x");
    let expected = Expr::UnaryOp(
        UnaryOp::new(BasicUnaryOp::IntSucc, Some(MachineRep::U8)),
        Box::new(Expr::NumVar(Cow::Borrowed("x"))),
    );
    assert_eq!(actual, expected);

    // Unary ops stack (no parens needed) and bind tighter than binops.
    let actual = numexpr!(abs pred "x" + "y");
    let expected = Expr::BinOp(
        BinOp::new(BasicBinOp::Add, None),
        Box::new(Expr::UnaryOp(
            UnaryOp::new(BasicUnaryOp::AbsVal, None),
            Box::new(Expr::UnaryOp(
                UnaryOp::new(BasicUnaryOp::IntPred, None),
                Box::new(Expr::NumVar(Cow::Borrowed("x"))),
            )),
        )),
        Box::new(Expr::NumVar(Cow::Borrowed("y"))),
    );
    assert_eq!(actual, expected);
}

#[test]
fn unary_binds_tighter_than_binops() {
    // -"x" * "y"  ==  (-"x") * "y"
    let actual = numexpr!(-"x" * "y");
    let expected = Expr::BinOp(
        BinOp::new(BasicBinOp::Mul, None),
        Box::new(Expr::UnaryOp(
            UnaryOp::new(BasicUnaryOp::Negate, None),
            Box::new(Expr::NumVar(Cow::Borrowed("x"))),
        )),
        Box::new(Expr::NumVar(Cow::Borrowed("y"))),
    );
    assert_eq!(actual, expected);
}

#[test]
fn bitwise_and_arithmetic_casts() {
    let actual = numexpr!("x" as u8);
    let expected = Expr::Cast(
        CastOp::bitwise(MachineRep::U8),
        Box::new(Expr::NumVar(Cow::Borrowed("x"))),
    );
    assert_eq!(actual, expected);

    let actual = numexpr!("x" into u8);
    let expected = Expr::Cast(
        CastOp::arith(MachineRep::U8),
        Box::new(Expr::NumVar(Cow::Borrowed("x"))),
    );
    assert_eq!(actual, expected);
}

#[test]
fn cast_chains_left_associatively() {
    // "x" as u8 as i16  ==  ("x" as u8) as i16
    let actual = numexpr!("x" as u8 as i16);
    let expected = Expr::Cast(
        CastOp::bitwise(MachineRep::I16),
        Box::new(Expr::Cast(
            CastOp::bitwise(MachineRep::U8),
            Box::new(Expr::NumVar(Cow::Borrowed("x"))),
        )),
    );
    assert_eq!(actual, expected);
}

#[test]
fn cast_precedence() {
    // Cast binds tighter than binops...
    // "x" as u8 + "y"  ==  ("x" as u8) + "y"
    let actual = numexpr!("x" as u8 + "y");
    let expected = Expr::BinOp(
        BinOp::new(BasicBinOp::Add, None),
        Box::new(Expr::Cast(
            CastOp::bitwise(MachineRep::U8),
            Box::new(Expr::NumVar(Cow::Borrowed("x"))),
        )),
        Box::new(Expr::NumVar(Cow::Borrowed("y"))),
    );
    assert_eq!(actual, expected);

    // ...but looser than unary ops.
    // -"x" as u8  ==  (-"x") as u8
    let actual = numexpr!(-"x" as u8);
    let expected = Expr::Cast(
        CastOp::bitwise(MachineRep::U8),
        Box::new(Expr::UnaryOp(
            UnaryOp::new(BasicUnaryOp::Negate, None),
            Box::new(Expr::NumVar(Cow::Borrowed("x"))),
        )),
    );
    assert_eq!(actual, expected);
}
