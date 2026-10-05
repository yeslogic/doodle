#![cfg(test)]
//! Regression tests pinning down the current behavior of `Format::Pos` (see `doc/NUMERIC_PLAN.md`, Q2).
//!
//! Registration types `Pos` as `NumericHole` and the `TypeChecker` as any unsigned type (preferring `U64`),
//! while the interpreter produces a native `Value::U64`. These tests record what each layer currently does,
//! so that a change to `Pos`'s runtime value shows up here (and in the `test2.jpg`/`test.waldo` decode
//! snapshots, which cover the live uses of `Pos` in `tiff` and `waldo`).

use std::panic::{AssertUnwindSafe, catch_unwind};

use doodle::codegen::{ToFragment, generate_code};
use doodle::decoder::{Compiler, Value};
use doodle::helper::*;
use doodle::read::ReadCtxt;
use doodle::{Expr, Format, FormatModule};

/// Input for every test: two bytes are read before `Pos`, so it evaluates to 2, followed by padding.
const INPUT: [u8; 8] = [0xAA, 0xBB, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60];

// the `Err` message is only read through `Debug`, in assertion failures
#[allow(dead_code)]
#[derive(Debug)]
enum Outcome {
    Ok(Value),
    Err(String),
    Panic(String),
}

fn catch<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|payload| {
        if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else {
            String::from("<non-string panic payload>")
        }
    })
}

/// Reads two bytes, binds `Pos` to `p`, then runs `body`.
fn with_pos(body: Format) -> Format {
    chain(u16be(), "_skip", chain(Format::Pos, "p", body))
}

/// Runs `format` through registration and the main interpreter.
fn interp(format: &Format) -> Outcome {
    let prog =
        Compiler::compile_program(&FormatModule::new(), format).expect("registration failed");
    match catch(|| prog.run(ReadCtxt::new(&INPUT)).map(|(v, _)| v)) {
        Ok(Ok(v)) => Outcome::Ok(v),
        Ok(Err(e)) => Outcome::Err(format!("{e:?}")),
        Err(msg) => Outcome::Panic(msg),
    }
}

/// Runs `format` through registration and the location-tracking interpreter.
fn interp_loc(format: &Format) -> Outcome {
    let prog =
        Compiler::compile_program(&FormatModule::new(), format).expect("registration failed");
    match catch(|| {
        prog.run_with_loc(ReadCtxt::new(&INPUT))
            .map(|(v, _)| v.clone_into_value())
    }) {
        Ok(Ok(v)) => Outcome::Ok(v),
        Ok(Err(e)) => Outcome::Err(format!("{e:?}")),
        Err(msg) => Outcome::Panic(msg),
    }
}

/// Asserts that both interpreters produce `expected`.
fn assert_interp_ok(format: &Format, expected: Value) {
    for (which, outcome) in [
        ("interp", interp(format)),
        ("interp_loc", interp_loc(format)),
    ] {
        match outcome {
            Outcome::Ok(v) => assert_eq!(v, expected, "{which}"),
            other => panic!("{which}: expected Ok({expected:?}), found {other:?}"),
        }
    }
}

/// Asserts that both interpreters panic with a message containing `expected`.
fn assert_interp_panics(format: &Format, expected: &str) {
    for (which, outcome) in [
        ("interp", interp(format)),
        ("interp_loc", interp_loc(format)),
    ] {
        match outcome {
            Outcome::Panic(msg) => {
                assert!(msg.contains(expected), "{which}: unexpected panic: {msg}")
            }
            other => panic!("{which}: expected panic containing {expected:?}, found {other:?}"),
        }
    }
}

/// Asserts that codegen (the `TypeChecker` plus code emission) accepts `format`.
fn assert_codegen_ok(format: &Format) {
    let code = generate_code(&FormatModule::new(), format);
    let _ = code.to_fragment().to_string();
}

#[test]
fn pos_is_native_u64() {
    let f = with_pos(compute(var("p")));
    assert_interp_ok(&f, Value::U64(2));
    assert_codegen_ok(&f);
}

#[test]
fn pos_plus_u64() {
    let f = with_pos(compute(add(var("p"), Expr::U64(1))));
    assert_interp_ok(&f, Value::U64(3));
    assert_codegen_ok(&f);
}

/// The `waldo` pattern: a `U64` field minus `Pos`.
#[test]
fn u64_minus_pos() {
    let f = with_pos(compute(sub(Expr::U64(10), var("p"))));
    assert_interp_ok(&f, Value::U64(8));
    assert_codegen_ok(&f);
}

#[test]
fn pos_lt_u64() {
    let f = with_pos(compute(expr_lt(var("p"), Expr::U64(10))));
    assert_interp_ok(&f, Value::Bool(true));
    assert_codegen_ok(&f);
}

/// Known gap (Q2): registration and codegen accept `pos + U32(1)`, but the interpreter panics on the
/// native `U64`/`U32` mismatch.
#[test]
fn pos_plus_u32_panics_in_interp() {
    let f = with_pos(compute(add(var("p"), Expr::U32(1))));
    assert_interp_panics(&f, "cannot apply arith");
    assert_codegen_ok(&f);
}

/// Known gap (Q2): as above, for a comparison.
#[test]
fn pos_lt_u32_panics_in_interp() {
    let f = with_pos(compute(expr_lt(var("p"), Expr::U32(10))));
    assert_interp_panics(&f, "cannot apply int-rel");
    assert_codegen_ok(&f);
}

/// The `tiff` and `with_relative_offset(None, ..)` pattern: `Pos` as the base of a `WithRelativeOffset`,
/// with a `U32` offset.
#[test]
fn pos_as_relative_offset_base() {
    let f = with_pos(with_relative_offset(Some(var("p")), Expr::U32(1), u8()));
    assert_interp_ok(&f, Value::U8(0x20));
    assert_codegen_ok(&f);
}
