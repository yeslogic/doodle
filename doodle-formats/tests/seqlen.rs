#![cfg(test)]
//! Regression tests pinning down the behavior of `SeqLength`, `SeqIx`, `SubSeq` and `Dup`
//! (see `doc/SEQLEN_PLAN.md`).
//!
//! Registration types `SeqLength` as `NumericHole` and accepts any unsigned type or Auto for the
//! index/start/length/count arguments of `SeqIx`, `SubSeq`, `SubSeqInflate` and `Dup`; the `TypeChecker`
//! accepts any unsigned type there (preferring `U32`); the interpreter produces an Auto Numeric for
//! `SeqLength` (`Value::from_seq_len`) and accepts any width for the arguments.
//!
//! `assert_codegen_ok` only checks that codegen accepts a format; it does not compile the emitted code.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use doodle::codegen::{ToFragment, generate_code};
use doodle::decoder::{Compiler, Value};
use doodle::helper::*;
use doodle::numeric::core::TypedConst;
use doodle::read::ReadCtxt;
use doodle::{Expr, Format, FormatModule};

/// Input for every test: the sequence-reading prefix consumes up to three of these bytes.
const INPUT: [u8; 8] = [0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11];

// the `Err`/`Panic` messages are only read through `Debug`, in assertion failures
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

/// An Auto Numeric with value `n`, as produced by `SeqLength`.
fn auto(n: u32) -> Value {
    Value::Numeric(Rc::new(TypedConst::new_auto(n)))
}

/// Reads `n` bytes into the sequence `s` (a `Seq(U8)`), then runs `body`.
fn with_seq(n: u8, body: Format) -> Format {
    chain(repeat_count(Expr::U8(n), u8()), "s", body)
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

/// Asserts that both interpreters return an error (not a panic) containing `expected`.
fn assert_interp_err(format: &Format, expected: &str) {
    for (which, outcome) in [
        ("interp", interp(format)),
        ("interp_loc", interp_loc(format)),
    ] {
        match outcome {
            Outcome::Err(msg) => {
                assert!(msg.contains(expected), "{which}: unexpected error: {msg}")
            }
            other => panic!("{which}: expected error containing {expected:?}, found {other:?}"),
        }
    }
}

/// Asserts that codegen (the `TypeChecker` plus code emission) accepts `format`.
///
/// The emitted code itself is not compiled.
fn assert_codegen_ok(format: &Format) {
    let code = generate_code(&FormatModule::new(), format);
    let _ = code.to_fragment().to_string();
}

// SECTION - SeqLength

#[test]
fn seq_length_is_auto_numeric() {
    let f = with_seq(3, compute(seq_length(var("s"))));
    assert_interp_ok(&f, auto(3));
    assert_codegen_ok(&f);
}

/// The `deflate` pattern: `seq_length(..) - as_u32(..)`.
#[test]
fn seq_length_minus_u32() {
    let f = with_seq(3, compute(sub(seq_length(var("s")), Expr::U32(1))));
    assert_interp_ok(&f, Value::U32(2));
    assert_codegen_ok(&f);
}

/// The `deflate` pattern: a comparison on `seq_length(..)`.
#[test]
fn seq_length_gte_u32() {
    let f = with_seq(3, compute(expr_gte(seq_length(var("s")), Expr::U32(2))));
    assert_interp_ok(&f, Value::Bool(true));
    assert_codegen_ok(&f);
}

/// Formerly rejected by registration, which typed `SeqLength` as `U32`.
#[test]
fn seq_length_eq_u8() {
    let f = with_seq(3, compute(expr_eq(seq_length(var("s")), Expr::U8(3))));
    assert_interp_ok(&f, Value::Bool(true));
    assert_codegen_ok(&f);
}

/// The `opentype::last_elem` / `seq_last_unchecked` pattern on a non-empty sequence.
#[test]
fn seq_last_unchecked_nonempty() {
    let f = with_seq(3, compute(seq_last_unchecked(var("s"))));
    assert_interp_ok(&f, Value::U8(0x0C));
    assert_codegen_ok(&f);
}

/// On an empty sequence, `pred(seq_length(..))` is `-1` (Auto), and the error is raised where it is used as
/// an index. (When `SeqLength` was a native `U32`, it was a `U32` underflow at the `pred`.)
#[test]
fn seq_last_unchecked_empty() {
    let f = with_seq(0, compute(seq_last_unchecked(var("s"))));
    assert_interp_err(
        &f,
        "NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto)",
    );
    assert_codegen_ok(&f);
}

/// The `opentype` pattern: `enum_from_to(U32(0), pred(seq_length(..)))`, on an empty sequence. As above, the
/// error is raised where `-1` is used as a bound.
#[test]
fn enum_from_to_pred_seq_length_empty() {
    let f = with_seq(
        0,
        compute(enum_from_to(Expr::U32(0), pred(seq_length(var("s"))))),
    );
    assert_interp_err(
        &f,
        "NumericConvert(TypedConst::as_usize: unable to convert typed-const TypedConst(-1, Auto)",
    );
    assert_codegen_ok(&f);
}

#[test]
fn seq_last_checked_nonempty_and_empty() {
    let f = with_seq(3, compute(seq_last_checked(var("s"))));
    assert_interp_ok(&f, Value::Option(Some(Box::new(Value::U8(0x0C)))));
    assert_codegen_ok(&f);

    let f = with_seq(0, compute(seq_last_checked(var("s"))));
    assert_interp_ok(&f, Value::Option(None));
    assert_codegen_ok(&f);
}

// SECTION - SeqIx, SubSeq and Dup arguments

#[test]
fn seq_ix_u32() {
    let f = with_seq(3, compute(index_unchecked(var("s"), Expr::U32(1))));
    assert_interp_ok(&f, Value::U8(0x0B));
    assert_codegen_ok(&f);
}

/// Formerly rejected by registration, which required exactly `U32`.
#[test]
fn seq_ix_u8() {
    let f = with_seq(3, compute(index_unchecked(var("s"), Expr::U8(1))));
    assert_interp_ok(&f, Value::U8(0x0B));
    assert_codegen_ok(&f);
}

#[test]
fn sub_seq_u32() {
    let f = with_seq(
        3,
        compute(seq_length(sub_seq(var("s"), Expr::U32(1), Expr::U32(2)))),
    );
    assert_interp_ok(&f, auto(2));
    assert_codegen_ok(&f);
}

/// Formerly rejected by registration, which required exactly `U32`.
#[test]
fn sub_seq_u16_start() {
    let f = with_seq(
        3,
        compute(index_unchecked(
            sub_seq(var("s"), Expr::U16(1), Expr::U32(2)),
            Expr::U32(0),
        )),
    );
    assert_interp_ok(&f, Value::U8(0x0B));
    assert_codegen_ok(&f);
}

#[test]
fn dup_u32() {
    let f = compute(seq_length(dup(Expr::U32(4), Expr::U8(0))));
    assert_interp_ok(&f, auto(4));
    assert_codegen_ok(&f);
}

/// Formerly rejected by registration, which required exactly `U32`.
#[test]
fn dup_u8() {
    let f = compute(seq_length(dup(Expr::U8(4), Expr::U8(0))));
    assert_interp_ok(&f, auto(4));
    assert_codegen_ok(&f);
}

/// The `deflate` `FIXME[epic=dup32]` pattern: a `U8`-derived count cast with `as_u32`.
#[test]
fn dup_as_u32_count() {
    let f = with_seq(
        1,
        compute(seq_length(dup(
            as_u32(add(index_unchecked(var("s"), Expr::U32(0)), Expr::U8(3))),
            Expr::U8(0),
        ))),
    );
    assert_interp_ok(&f, auto(13));
    assert_codegen_ok(&f);
}
