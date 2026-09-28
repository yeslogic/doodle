#![cfg(test)]

use doodle::{codegen::{ToFragment, generate_code}, decoder::{
    Compiler, Value,
    seq_kind::{SeqKind, ValueSeq},
}};
use doodle::helper::*;
use doodle::read::ReadCtxt;
use doodle::{Format, FormatModule, FormatRef};
use doodle_numexpr_macro::numexpr;

/// Setup for a format that converts an i8-read into char
fn setup_char() -> (FormatModule, FormatRef) {
    let mut module = FormatModule::new();
    let f = module.define_format(
        "test.signed_intops",
        chain(
            i8(),
            "x",
            compute(as_char(var("x"))),
        )
    );
    (module, f)
}

/// Regression test - AsChar(x : i8) works in interpreter
#[test]
fn test_interp_char() {
    let (module, f) = setup_char();
    let prog = Compiler::compile_program(&module, &f.call()).expect("compilation failed");
    let input = [0x00];
    let ctxt = ReadCtxt::new(&input);
    let (res, _) = prog.run(ctxt).expect("decoding failed on buf");
    let expected = Value::Char('\0');
    assert_eq!(res, expected);
}

/// Reproducibility test - AsChar(x : i8) fails in codegen (typechecker)
#[test]
// should-panic only documents the expected panic as a regression, we are not claiming this test panicking is the correct behavior
#[should_panic = "Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U8, U16, U32, U64 }`"]
fn test_gen_char() {
    let (module, f) = setup_char();
    let code = generate_code(&module, &f.call());
    println!("{}", code.to_fragment())
}

/// setup for a format that roundtrips an i8-read through succ and then pred
fn setup_unary() -> (FormatModule, FormatRef) {
    let mut module = FormatModule::new();
    let f = module.define_format(
        "test.signed_intops",
        chain(
            i8(),
            "x",
            compute(pred(succ(var("x")))),
        )
    );
    (module, f)
}

/// Reproducibility test to demonstrate Succ/Pred failing when applied to signed numerics (in this case, I8) in the interpreter
#[test]
// should-panic only documents the expected panic as a regression, we are not claiming this test panicking is the correct behavior
#[should_panic = "top-level unary operations should not be performed on raw-numeric with signed or auto representation"]
fn test_interp_unary() {
    let (module, f) = setup_unary();
    let prog = Compiler::compile_program(&module, &f.call()).expect("compilation failed");
    let input = [0x00];
    let ctxt = ReadCtxt::new(&input);
    let (res, _) = prog.run(ctxt).expect("decoding failed on buf");
    let expected = numeric(numexpr!(0i8)).eval_value(&doodle::scope::GScope::Empty).expect("eval failed");
    assert_eq!(res, expected);
}

/// Reproducibility test to demonstrate Succ/Pred failing when applied to signed numerics (in this case, I8) in the codegen layer during typechecking
#[test]
// should-panic only documents the expected panic as a regression, we are not claiming this test panicking is the correct behavior
#[should_panic = "Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U8, U16, U32, U64 }`"]
fn test_gen_unary() {
    let (module, f) = setup_unary();
    let code = generate_code(&module, &f.call());
    println!("{}", code.to_fragment())
}

/// setup for a format that adds 0 (auto) to an i8-read using core grammar addition
fn setup_binary() -> (FormatModule, FormatRef) {
    let mut module = FormatModule::new();
    let f = module.define_format(
        "test.signed_intops",
        chain(
            i8(),
            "x",
            compute(add(var("x"), poly_zero())),
        )
    );
    (module, f)
}

/// Reproducibility test to demonstrate Expr::Arith failing when applied to (signed) Numerics (in this case, I8) in the interpreter
#[test]
// should-panic only documents the expected panic as a regression, we are not claiming this test panicking is the correct behavior
#[should_panic = "raw arithmetic on numerics should be done in the numeric model, or with Expr-level casts beforehand"]
fn test_interp_binary() {
    let (module, f) = setup_binary();
    let prog = Compiler::compile_program(&module, &f.call()).expect("compilation failed");
    let input = [0x00];
    let ctxt = ReadCtxt::new(&input);
    let (res, _) = prog.run(ctxt).expect("decoding failed on buf");
    let expected = numeric(numexpr!(0i8)).eval_value(&doodle::scope::GScope::Empty).expect("eval failed");
    assert_eq!(res, expected);
}

/// Reproducibility test to demonstrate Expr::Arith failing when applied to (signed) Numerics (in this case, I8) in the codegen layer during typechecking
#[test]
// should-panic only documents the expected panic as a regression, we are not claiming this test panicking is the correct behavior
#[should_panic = "Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U8, U16, U32, U64 }`"]
fn test_gen_binary() {
    let (module, f) = setup_binary();
    let code = generate_code(&module, &f.call());
    println!("{}", code.to_fragment())
}
