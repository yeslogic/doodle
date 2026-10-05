#![cfg(test)]

use doodle::helper::*;
use doodle::read::ReadCtxt;
use doodle::{Format, FormatModule};
use doodle::{
    codegen::{ToFragment, generate_code},
    decoder::{
        Compiler, Value,
        seq_kind::{SeqKind, ValueSeq},
    },
};
use doodle_numexpr_macro::numexpr;

// NOTE - signed operands to native `AsChar`, `IntSucc`/`IntPred` and `Arith` are rejected by both registration
// and the TypeChecker (see doc/NUMERIC_PLAN.md, items 1 and 3), so neither the interpreter nor codegen has to
// support them. Signed arithmetic belongs in `Expr::Numeric`.

/// Asserts that registration (the first step of `Compiler::compile_program`) rejects `format` with an error
/// containing `expected`.
fn assert_registration_rejects(format: &Format, expected: &str) {
    match Compiler::compile_program(&FormatModule::new(), format) {
        Ok(_) => panic!("registration unexpectedly accepted the format"),
        Err(e) => assert!(e.to_string().contains(expected), "unexpected error: {e}"),
    }
}

/// Format that converts an i8-read into char
fn char_format() -> Format {
    chain(i8(), "x", compute(as_char(var("x"))))
}

#[test]
fn test_registration_char() {
    assert_registration_rejects(&char_format(), "unsound type cast AsChar(_ : Signed(I8))");
}

#[test]
#[should_panic = "Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U8, U16, U32, U64 }`"]
fn test_gen_char() {
    let code = generate_code(&FormatModule::new(), &char_format());
    println!("{}", code.to_fragment())
}

/// The supported idiom for converting a signed value to char is `AsChar(AsU32(x))`.
#[test]
fn test_interp_char_via_u32() {
    let format = chain(i8(), "x", compute(as_char(as_u32(var("x")))));
    let prog =
        Compiler::compile_program(&FormatModule::new(), &format).expect("compilation failed");
    let input = [0x00];
    let (res, _) = prog
        .run(ReadCtxt::new(&input))
        .expect("decoding failed on buf");
    assert_eq!(res, Value::Char('\0'));
}

/// Format that roundtrips an i8-read through succ and then pred
fn unary_format() -> Format {
    chain(i8(), "x", compute(pred(succ(var("x")))))
}

#[test]
fn test_registration_unary() {
    assert_registration_rejects(
        &unary_format(),
        "unexpected operand type for IntSucc: Signed(I8)",
    );
}

#[test]
#[should_panic = "Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U8, U16, U32, U64 }`"]
fn test_gen_unary() {
    let code = generate_code(&FormatModule::new(), &unary_format());
    println!("{}", code.to_fragment())
}

/// Format that adds 0 (auto) to an i8-read using core grammar addition
fn binary_format() -> Format {
    chain(i8(), "x", compute(add(var("x"), poly_zero())))
}

#[test]
fn test_registration_binary() {
    assert_registration_rejects(
        &binary_format(),
        "mismatched operand types for Add: Signed(I8), NumericHole",
    );
}

#[test]
#[should_panic = "Failed to infer module-wide type annotations: cross-layer numeric error: PrimInt not in BaseSet: `i8` ∉ `{ U8, U16, U32, U64 }`"]
fn test_gen_binary() {
    let code = generate_code(&FormatModule::new(), &binary_format());
    println!("{}", code.to_fragment())
}

// SECTION - Numeric-handling survey across all three callers
//
// Diagnostic only: prints a markdown acceptance matrix, never asserts on outcomes. Run with
//
//     cargo test -p doodle-formats --test signed_intops -- --ignored --nocapture numeric_survey
//
// For each (construction, operands) case, the same `Format` is fed to:
//
// - **registration**: `Compiler::compile_program` on an empty module, whose first step is the legacy
//   `infer_format_type`/`Expr::infer_type` pass that `define_format` also runs. Note that a failure
//   here may also come from the decoder-compilation step that follows inference.
// - **codegen**: `generate_code` + `to_fragment` on an empty module, which runs the `TypeChecker`
//   (the emitted code itself is *not* compiled or run)
// - **interpreter**: `Program::run` on the program compiled for registration
//
// Each cell is `✓` (accepted), `✗ #n` (rejected with an error) or `! #n` (panicked), where `#n`
// refers to the full message listed after the table; `–` means the interpreter was not reached.
// Only inconsistent cases are tabulated: cases every layer accepts, or every reached layer rejects
// with an error (no panics), are instead listed by category after the table.
// `generate_code` reports every `TypeChecker` rejection by panicking, so panics carrying its
// type-inference prefix ([`CODEGEN_REJECT_PREFIX`]) count as `✗` rather than `!`.
mod survey {
    use super::*;
    use doodle::bounds::Bounds;
    use doodle::{Expr, Pattern};
    use std::panic::{AssertUnwindSafe, catch_unwind};

    /// Where an operand comes from, and what representation it has.
    #[derive(Clone, Copy, Debug)]
    enum Src {
        NatU8,
        NatU16,
        NumU8,
        NumU16,
        NumI8,
        NumI8Neg,
        Auto,
        AutoNeg,
        /// `i8()` read of `0x05`, bound to a variable
        ReadI8,
        /// `i8()` read of `0xFF` (i.e. `-1`), bound to a variable
        ReadI8Neg,
        /// `u8()` read of `0x05`, bound to a variable
        ReadU8,
    }

    impl Src {
        fn label(self) -> &'static str {
            match self {
                Src::NatU8 => "U8(5)",
                Src::NatU16 => "U16(5)",
                Src::NumU8 => "5u8",
                Src::NumU16 => "5u16",
                Src::NumI8 => "5i8",
                Src::NumI8Neg => "-1i8",
                Src::Auto => "5auto",
                Src::AutoNeg => "-1auto",
                Src::ReadI8 => "i8()=5",
                Src::ReadI8Neg => "i8()=-1",
                Src::ReadU8 => "u8()=5",
            }
        }

        /// Returns the operand expression, plus the format (and input byte) that binds `name`, if any.
        fn build(self, name: &'static str) -> (Expr, Option<(Format, u8)>) {
            match self {
                Src::NatU8 => (Expr::U8(5), None),
                Src::NatU16 => (Expr::U16(5), None),
                Src::NumU8 => (numeric(numexpr!(5u8)), None),
                Src::NumU16 => (numeric(numexpr!(5u16)), None),
                Src::NumI8 => (numeric(numexpr!(5i8)), None),
                Src::NumI8Neg => (numeric(numexpr!(-1i8)), None),
                Src::Auto => (poly_const(5u8), None),
                Src::AutoNeg => (poly_const(-1i8), None),
                Src::ReadI8 => (var(name), Some((i8(), 0x05))),
                Src::ReadI8Neg => (var(name), Some((i8(), 0xFF))),
                Src::ReadU8 => (var(name), Some((u8(), 0x05))),
            }
        }
    }

    const UNARY_SRCS: &[Src] = &[
        Src::NatU8,
        Src::NumU8,
        Src::NumU16,
        Src::NumI8,
        Src::NumI8Neg,
        Src::Auto,
        Src::AutoNeg,
        Src::ReadI8,
        Src::ReadI8Neg,
        Src::ReadU8,
    ];

    const BINARY_SRCS: &[(Src, Src)] = &[
        (Src::NatU8, Src::NatU8),
        (Src::NatU8, Src::NatU16),
        (Src::NumU8, Src::NatU8),
        (Src::NatU8, Src::NumU8),
        (Src::NumU8, Src::NumU8),
        (Src::NumU8, Src::NumU16),
        (Src::NumI8, Src::NumI8),
        (Src::NumI8, Src::NumU8),
        (Src::Auto, Src::NatU8),
        (Src::Auto, Src::NumI8),
        (Src::Auto, Src::Auto),
        (Src::NumI8Neg, Src::Auto),
        (Src::ReadI8, Src::Auto),
        (Src::ReadI8, Src::NumI8),
        (Src::ReadU8, Src::NumU8),
    ];

    /// Trailing input after any operand-binding bytes, for contexts that read (`RepeatCount` etc.)
    const PAD: [u8; 16] = [0; 16];

    fn seq8() -> Expr {
        Expr::Seq((0..8).map(Expr::U8).collect())
    }

    fn match_bool(e: Expr, pat: Pattern) -> Format {
        compute(expr_match(
            e,
            [
                (pat, Expr::Bool(true)),
                (Pattern::Wildcard, Expr::Bool(false)),
            ],
        ))
    }

    /// Case-label templates use `{a}`/`{b}` as placeholders for the operand labels.
    fn unary_contexts() -> Vec<(&'static str, fn(Expr) -> Format)> {
        vec![
            ("IntSucc({a})", |e| compute(succ(e))),
            ("IntPred({a})", |e| compute(pred(e))),
            ("AsU8({a})", |e| compute(as_u8(e))),
            ("AsU32({a})", |e| compute(as_u32(e))),
            ("AsChar({a})", |e| compute(as_char(e))),
            ("SeqIx(seq, {a})", |e| compute(index_unchecked(seq8(), e))),
            ("SubSeq(seq, {a}, 1)", |e| {
                compute(sub_seq(seq8(), e, Expr::U32(1)))
            }),
            ("Dup({a}, U8(0))", |e| compute(dup(e, Expr::U8(0)))),
            ("FindByKey(id, {a}, [U8(5)])", |e| {
                compute(find_by_key(false, |x| x, e, Expr::Seq(vec![Expr::U8(5)])))
            }),
            ("RepeatCount({a}, u8)", |e| repeat_count(e, u8())),
            ("CaptureBytes({a})", capture_bytes_from_here),
            ("ReadArray({a}, U8)", |e| {
                from_here(read_array(e, BaseKind::U8))
            }),
            ("Offset(v, {a})", |e| {
                let_view(
                    "v",
                    with_view(vvar("v").offset(e), capture_bytes(Expr::U32(1))),
                )
            }),
            ("{a} ~ U8(5)", |e| match_bool(e, Pattern::U8(5))),
            ("{a} ~ Int(0..=10)", |e| {
                match_bool(e, Pattern::Int(Bounds::new(0, 10)))
            }),
            ("{a} ~ ZConst(5)", |e| {
                match_bool(e, Pattern::ZConst(5.into()))
            }),
            ("{a} ~ ZConst(-1)", |e| {
                match_bool(e, Pattern::ZConst((-1).into()))
            }),
        ]
    }

    fn binary_contexts() -> Vec<(&'static str, fn(Expr, Expr) -> Format)> {
        vec![
            ("{a} == {b}", |a, b| compute(expr_eq(a, b))),
            ("{a} < {b}", |a, b| compute(expr_lt(a, b))),
            ("{a} + {b}", |a, b| compute(add(a, b))),
            ("EnumFromTo({a}, {b})", |a, b| compute(enum_from_to(a, b))),
            ("[{a}, {b}]", |a, b| compute(Expr::Seq(vec![a, b]))),
            ("if _ then {a} else {b}", |a, b| {
                compute(expr_if_else(Expr::Bool(true), a, b))
            }),
            ("RepeatBetween({a}, {b}, u8)", |a, b| {
                repeat_between(a, b, u8())
            }),
        ]
    }

    /// Binds each operand (if it needs a variable), then builds the context around the operand exprs.
    fn assemble(srcs: &[Src], body: impl FnOnce(Vec<Expr>) -> Format) -> (Format, Vec<u8>) {
        const NAMES: [&str; 2] = ["x", "y"];
        let mut exprs = Vec::new();
        let mut binds = Vec::new();
        let mut input = Vec::new();
        for (src, name) in srcs.iter().zip(NAMES) {
            let (e, bind) = src.build(name);
            exprs.push(e);
            if let Some((f, byte)) = bind {
                binds.push((name, f));
                input.push(byte);
            }
        }
        input.extend(PAD);
        let mut format = body(exprs);
        for (name, f) in binds.into_iter().rev() {
            format = chain(f, name, format);
        }
        (format, input)
    }

    enum Outcome {
        Ok,
        Err(String),
        Panic(String),
        Skip,
    }

    impl Outcome {
        fn is_ok(&self) -> bool {
            matches!(self, Outcome::Ok)
        }
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

    /// Prefix of the panic by which `generate_code` reports a `TypeChecker` rejection.
    const CODEGEN_REJECT_PREFIX: &str = "Failed to infer module-wide type annotations:";

    /// Broad grouping of a case-label template, for summarizing consistent cases.
    fn category(template: &str) -> &'static str {
        match template.split('(').next().unwrap_or("") {
            "IntSucc" | "IntPred" => "increment/decrement",
            "AsU8" | "AsU32" | "AsChar" => "casts",
            "SeqIx" | "SubSeq" | "Dup" => "sequence index/length arguments",
            "FindByKey" => "FindByKey keys",
            "RepeatCount" | "RepeatBetween" => "repetition counts",
            "CaptureBytes" | "ReadArray" | "Offset" => "view lengths/offsets",
            "EnumFromTo" => "EnumFromTo bounds",
            _ if template.contains(" ~ ") => "pattern matching",
            _ if template.contains(" == ") || template.contains(" < ") => "comparison",
            _ if template.contains(" + ") => "arithmetic",
            _ => "sequence literals/if-else branches",
        }
    }

    struct Row {
        category: &'static str,
        case: String,
        legacy: Outcome,
        interp: Outcome,
        codegen: Outcome,
    }

    fn run_case(template: &'static str, srcs: &[Src], format: Format, input: Vec<u8>) -> Row {
        let mut case = template.replace("{a}", srcs[0].label());
        if let Some(b) = srcs.get(1) {
            case = case.replace("{b}", b.label());
        }
        let module = FormatModule::new();

        let (legacy, interp) = match catch(|| Compiler::compile_program(&module, &format)) {
            Ok(Ok(prog)) => {
                let legacy = Outcome::Ok;
                let interp = match catch(|| prog.run(ReadCtxt::new(&input)).map(|(v, _)| v)) {
                    Ok(Ok(_)) => Outcome::Ok,
                    Ok(Err(e)) => Outcome::Err(format!("{e:?}")),
                    Err(msg) => Outcome::Panic(msg),
                };
                (legacy, interp)
            }
            Ok(Err(e)) => (Outcome::Err(format!("{e}")), Outcome::Skip),
            Err(msg) => (Outcome::Panic(msg), Outcome::Skip),
        };

        let codegen = match catch(|| generate_code(&module, &format).to_fragment().to_string()) {
            Ok(_) => Outcome::Ok,
            Err(msg) if msg.starts_with(CODEGEN_REJECT_PREFIX) => Outcome::Err(msg),
            Err(msg) => Outcome::Panic(msg),
        };

        Row {
            category: category(template),
            case,
            legacy,
            interp,
            codegen,
        }
    }

    /// Interns full failure messages so table cells can refer to them by number.
    #[derive(Default)]
    struct Messages(Vec<String>);

    impl Messages {
        fn id(&mut self, msg: &str) -> usize {
            // strip any trailing backtrace that anyhow may attach
            let msg = msg.lines().next().unwrap_or("").to_string();
            match self.0.iter().position(|m| *m == msg) {
                Some(ix) => ix + 1,
                None => {
                    self.0.push(msg);
                    self.0.len()
                }
            }
        }

        fn cell(&mut self, outcome: &Outcome) -> String {
            match outcome {
                Outcome::Ok => String::from("✓"),
                Outcome::Err(m) => format!("✗ #{}", self.id(m)),
                Outcome::Panic(m) => format!("! #{}", self.id(m)),
                Outcome::Skip => String::from("–"),
            }
        }
    }

    impl Row {
        fn outcomes(&self) -> impl Iterator<Item = &Outcome> {
            [&self.legacy, &self.codegen, &self.interp]
                .into_iter()
                .filter(|o| !matches!(o, Outcome::Skip))
        }

        /// `Some(true)` if every layer accepts, `Some(false)` if every reached layer rejects with an
        /// error (not a panic), and `None` otherwise.
        fn consistent(&self) -> Option<bool> {
            if self.outcomes().all(Outcome::is_ok) {
                Some(true)
            } else if self.outcomes().all(|o| matches!(o, Outcome::Err(_))) {
                Some(false)
            } else {
                None
            }
        }
    }

    const INTERP_PANICS: &str = "interp panics after registration accepts";

    /// Highlights disagreements between callers. A runtime `ERR` after legacy acceptance is not
    /// flagged, as value-dependent failures (e.g. `AsU8(-1i8)`) are expected to surface there.
    fn flags(row: &Row) -> &'static str {
        match (&row.legacy, &row.interp, row.codegen.is_ok()) {
            (Outcome::Ok, Outcome::Panic(_), _) => INTERP_PANICS,
            (_, _, _) if matches!(row.codegen, Outcome::Panic(_)) => "codegen panics",
            (Outcome::Ok, _, false) => "codegen stricter",
            (Outcome::Err(_) | Outcome::Panic(_), _, true) => "codegen looser",
            _ => "",
        }
    }

    #[test]
    #[ignore = "diagnostic survey; prints a matrix and asserts nothing"]
    fn numeric_survey() {
        let mut rows = Vec::new();

        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        for (context, build) in unary_contexts() {
            for &src in UNARY_SRCS {
                let (format, input) = assemble(&[src], |es| build(es[0].clone()));
                rows.push(run_case(context, &[src], format, input));
            }
        }
        for (context, build) in binary_contexts() {
            for &(a, b) in BINARY_SRCS {
                let (format, input) = assemble(&[a, b], |es| build(es[0].clone(), es[1].clone()));
                rows.push(run_case(context, &[a, b], format, input));
            }
        }
        std::panic::set_hook(prev_hook);

        let mut messages = Messages::default();
        println!("| case | registration | codegen | interpreter | note |");
        println!("|---|:-:|:-:|:-:|---|");
        for row in rows.iter().filter(|r| r.consistent().is_none()) {
            let (r, c, i) = (
                messages.cell(&row.legacy),
                messages.cell(&row.codegen),
                messages.cell(&row.interp),
            );
            println!("| `{}` | {r} | {c} | {i} | {} |", row.case, flags(row));
        }

        println!("\nConsistent cases (omitted from the table):\n");
        let mut categories: Vec<&str> = Vec::new();
        for row in &rows {
            if !categories.contains(&row.category) {
                categories.push(row.category);
            }
        }
        for cat in categories {
            for (accepted, verdict) in [(true, "all accept"), (false, "all reject")] {
                let cases: Vec<String> = rows
                    .iter()
                    .filter(|r| r.category == cat && r.consistent() == Some(accepted))
                    .map(|r| format!("`{}`", r.case))
                    .collect();
                if !cases.is_empty() {
                    println!("- **{cat}**, {verdict}: {}", cases.join(", "));
                }
            }
        }

        println!("\nMessages:\n");
        for (ix, msg) in messages.0.iter().enumerate() {
            println!("{}. {msg}", ix + 1);
        }

        let count = |f: fn(&Row) -> bool| rows.iter().filter(|r| f(r)).count();
        println!(
            "\n{} cases ({} inconsistent): registration ok {}, codegen ok {}, interpreter ok {}; interp panics after registration accepts {}, codegen stricter {}, codegen looser {}",
            rows.len(),
            count(|r| r.consistent().is_none()),
            count(|r| r.legacy.is_ok()),
            count(|r| r.codegen.is_ok()),
            count(|r| r.interp.is_ok()),
            count(|r| flags(r) == INTERP_PANICS),
            count(|r| flags(r) == "codegen stricter"),
            count(|r| flags(r) == "codegen looser"),
        );
    }
}
// !SECTION
