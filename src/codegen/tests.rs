#![cfg(test)]
//! End-to-end tests of the codegen pipeline, grouped by shared setup.

use super::*;
use crate::BaseKind;
use crate::TypeHint;
use crate::helper::{ANY_BYTE, compute, record, succ, var};
use crate::numeric::MachineRep;
use proptest::prelude::*;

/// Returns a string containing the generated code for the given format and any registerd
/// subformats thereof within `module`.
fn produce_string_gencode(module: &FormatModule, f: &Format) -> String {
    generate_code(module, f).to_fragment().to_string()
}

mod elaboration {
    use super::*;

    fn population_check(module: &FormatModule, f: &Format, label: Option<&'static str>) {
        let tc = TypeChecker::infer_module(module, f).unwrap();
        let tc_pop = tc.size();

        // println!("{tc:?}");

        let cg = CodeGen::new();
        let mut tv = Elaborator::new(module, tc, cg);
        let (dec_f, _extra) = tv.elaborate_module(module, f);
        let re_f = Format::from(dec_f.clone());
        assert_eq!(
            &re_f,
            f,
            "post-elaboration format mismatch: {} != {}",
            serde_json::ser::to_string(&re_f).unwrap(),
            serde_json::ser::to_string(&f).unwrap()
        );
        let tv_pop = tv.next_index;

        // println!("{f:?} => {dec_f:?}");
        assert_eq!(
            tv_pop,
            tc_pop,
            "failed population check {} ({} TC vs {} TV)", // on {:?}\n{}\n{}",
            label.unwrap_or_default(),
            tc_pop,
            tv_pop,
            // dec_f,
            // serde_json::ser::to_string(&re_f).unwrap(),
            // serde_json::ser::to_string(&f).unwrap()
        );
    }

    fn run_headcount(fs: &[(&'static str, Format)]) {
        let mut module = FormatModule::new();
        for (name, f) in fs.iter() {
            module.define_format(*name, f.clone());
            population_check(&module, f, None);
        }
    }

    #[test]
    fn test_headcount_simple() {
        let formats = vec![
            ("test.fail", Format::Fail),
            ("test.eoi", Format::EndOfInput),
            ("test.align64", Format::align(64)),
            ("test.any_byte", Format::Byte(ByteSet::full())),
        ];
        run_headcount(&formats);
    }

    #[test]
    fn test_headcount_record_simple() {
        let f = Format::record(vec![
            ("any_byte", Format::Byte(ByteSet::full())),
            ("align64", Format::align(64)),
            ("eoi", Format::EndOfInput),
        ]);

        run_headcount(&[("record_simple", f)]);
    }

    #[test]
    fn test_headcount_adt_simple() {
        let f = Format::Union(vec![
            Format::Variant("some".into(), Box::new(Format::Byte(ByteSet::full()))),
            Format::Variant("none".into(), Box::new(Format::EMPTY)),
        ]);

        run_headcount(&[("adt_simple", f)]);
    }

    #[test]
    fn test_headcount_item_var() {
        let sub_f = Format::Byte(ByteSet::full());
        let mut module = FormatModule::new();
        let sub_ref = module.define_format("test.any_byte", sub_f);
        let f = sub_ref.call();
        module.define_format("test.call_any_byte", f.clone());
        population_check(&module, &f, None);
    }

    #[test]
    fn test_headcount_compute_simple() {
        let x = Format::Byte(ByteSet::full());
        let fx = compute(Expr::Var("x".into()));
        let gx = compute(Expr::Arith(
            Arith::Add,
            Box::new(Expr::Var("x".into())),
            Box::new(Expr::Var("x".into())),
        ));

        let f = Format::record(vec![("x", x), ("fx", fx), ("gx", gx)]);
        run_headcount(&[("test.compute_simple", f)]);
    }

    #[test]
    fn test_headcount_compute_complex() {
        let is_null = Expr::Lambda(
            "x".into(),
            Box::new(Expr::IntRel(
                IntRel::Eq,
                Box::new(Expr::U8(0)),
                Box::new(Expr::Var("x".into())),
            )),
        );
        let ix_dup = Expr::Lambda(
            "acc_x".into(),
            Box::new(Expr::Tuple(vec![
                Expr::Arith(
                    Arith::Add,
                    Box::new(Expr::U32(1)),
                    Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 0)),
                ),
                Expr::Dup(
                    Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 0)),
                    Box::new(Expr::TupleProj(Box::new(Expr::Var("acc_x".into())), 1)),
                ),
            ])),
        );

        let xs =
            Format::RepeatUntilLast(Box::new(is_null), Box::new(Format::Byte(ByteSet::full())));
        let fxs = compute(Expr::FlatMapAccum(
            Box::new(ix_dup),
            Box::new(Expr::U32(1)),
            TypeHint::from(crate::ValueType::from(
                crate::valuetype::augmented::AugValueType::from(BaseType::U32),
            )),
            Box::new(Expr::Var("xs".into())),
        ));

        let f = Format::record(vec![("xs", xs), ("fxs", fxs)]);
        run_headcount(&[("test.compute_complex", f)]);
    }

    /// Lockstep regression test for levels unreachable from the top-level format, where one such level is
    /// referenced (via `ItemVar`) by a later one.
    ///
    /// `TypeChecker::infer_module` and `Elaborator::elaborate_module` must agree on whether that reference re-walks
    /// the earlier level's body (allocating `UVar`s / incrementing `next_index`) or hits their level-keyed caches;
    /// if only one side records the earlier level, every subsequent index is offset and elaboration reads the wrong
    /// types.
    #[test]
    fn unreached_level_referenced_by_later_unreached_level() {
        use crate::helper::tuple;
        let mut module = FormatModule::new();
        let inner = module.define_format(
            "test.inner",
            tuple([Format::ANY_BYTE, Format::Compute(Box::new(Expr::U32(1)))]),
        );
        let _outer = module.define_format(
            "test.outer",
            tuple([inner.call(), Format::Compute(Box::new(Expr::U64(2)))]),
        );
        let top = module.define_format("test.top", Format::ANY_BYTE);
        let output = produce_string_gencode(&module, &top.call());
        assert!(!output.is_empty());
    }
}

mod phantom_rec {
    use super::*;

    /// Builds the smallest module that exercises `FormatModule::define_format_phantom_rec`'s
    /// intended use-case: a format that refers to itself solely underneath a `Format::Phantom`.
    ///
    /// Registration itself (via [`FormatModule::define_format_phantom_rec`]) succeeds - see
    /// `crate::test::format_phantom_rec_registers_and_decodes` - and so does decoding via the
    /// plain (untyped) `decoder::Compiler`. The blockers watermarked below are specific to the
    /// *codegen* pipeline (`codegen::TypeChecker`/`codegen::Elaborator`), not registration or
    /// plain decoding.
    fn make_phantom_rec_module() -> (FormatModule, Format) {
        let mut module = FormatModule::new();
        let self_ref = module.define_format_phantom_rec(
            "test.phantom_rec",
            |phantom_field| {
                Format::record(vec![
                    ("tag", Format::Byte(ByteSet::full())),
                    ("child", phantom_field),
                ])
            },
            |rec_ref| rec_ref.call(),
        );
        let f = self_ref.call();
        (module, f)
    }

    /// Like `make_phantom_rec_module`, but with an extra field that genuinely needs a borrowed
    /// lifetime (`ReadArray`) alongside the self-referencing one - used to confirm that the
    /// self-reference itself never *causes* a lifetime parameter to be added (see
    /// `phantom_rec_full_pipeline_repro`'s generated `test_phantom_rec`, which has none), while a
    /// genuine need elsewhere in the same recursive type is still correctly detected and
    /// consistently threaded through both the recursive reference and the type's own definition.
    fn make_phantom_rec_module_with_borrow() -> (FormatModule, Format) {
        let mut module = FormatModule::new();
        let self_ref = module.define_format_phantom_rec(
            "test.phantom_rec_borrow",
            |phantom_field| {
                Format::record(vec![
                    ("tag", Format::Byte(ByteSet::full())),
                    (
                        "arr",
                        crate::helper::from_here(crate::helper::read_array(
                            Expr::U8(3),
                            BaseKind::U8,
                        )),
                    ),
                    ("child", phantom_field),
                ])
            },
            |rec_ref| rec_ref.call(),
        );
        let f = self_ref.call();
        (module, f)
    }

    #[test]
    fn phantom_rec_with_genuine_borrow_gets_lifetime() {
        let (module, f) = make_phantom_rec_module_with_borrow();
        let output = produce_string_gencode(&module, &f);
        assert!(
            output.contains("'input"),
            "expected a lifetime parameter threaded through the recursive type, found none:\n{output}"
        );
    }

    /// Like `make_phantom_rec_module`, but a self-referential `Union` (the shape relevant to the
    /// real motivating case, COLRv1's `PaintTable`, a multi-branch alternation) rather than a
    /// `Record`.
    fn make_phantom_rec_union_module() -> (FormatModule, Format) {
        let mut module = FormatModule::new();
        let self_ref = module.define_format_phantom_rec(
            "test.phantom_rec_union",
            |phantom_field| {
                Format::Union(vec![
                    Format::Variant(
                        Label::Borrowed("Leaf"),
                        Box::new(Format::Byte(ByteSet::full())),
                    ),
                    Format::Variant(Label::Borrowed("Node"), Box::new(phantom_field)),
                ])
            },
            |rec_ref| rec_ref.call(),
        );
        let f = self_ref.call();
        (module, f)
    }

    #[test]
    fn phantom_rec_union_full_pipeline() {
        let (module, f) = make_phantom_rec_union_module();
        let output = produce_string_gencode(&module, &f);
        assert!(!output.is_empty());
        assert!(
            !output.contains("'input"),
            "no field here genuinely needs a lifetime - the self-reference alone shouldn't add one:\n{output}"
        );
    }

    /// Regression test for the type-checking blocker against `define_format_phantom_rec`-defined
    /// formats: `TypeChecker::infer_var_format_level` used to populate its `level_vars` cache only
    /// *after* recursively inferring a level's own body, so a self-reference reached during that
    /// recursion (necessarily confined within a `Format::Phantom`) found nothing cached and
    /// recursed again without bound - this overflowed the stack and aborted the process outright
    /// (not a catchable `panic!`/`Result::Err`). Fixed by seeding the cache with a placeholder
    /// `UVar` *before* recursing (see `infer_var_format_level`'s doc comment).
    ///
    /// This test used to require `--ignored` and a subprocess-based watermark to observe its
    /// failure safely (a stack overflow can't be asserted against directly); now that the blocker
    /// is fixed, it runs as an ordinary test.
    #[test]
    fn phantom_rec_typecheck_resolves() {
        let (module, f) = make_phantom_rec_module();
        let tc = TypeChecker::infer_module(&module, &f).expect("infer_module failed");
        assert!(tc.size() > 0);
    }

    /// Regression test for the full codegen pipeline (type-checking, then elaboration, then
    /// codegen proper) against `define_format_phantom_rec`-defined formats.
    ///
    /// This exercised three distinct instances of the same bug class in turn, each fixed the same
    /// way (reserve a placeholder in the relevant cache *before* recursing into a level's own
    /// body, rather than inserting the real value only after):
    /// - `TypeChecker::infer_var_format_level`'s `level_vars` cache (see
    ///   `phantom_rec_typecheck_resolves` above).
    /// - `Elaborator::elaborate_format`'s `t_formats` cache.
    /// - `CodeGen::lift_uvar`'s `self.metavariables` cache (`Expansion::Record`/`Expansion::Union`
    ///   handling) - empirically confirmed via depth-limited instrumentation to cycle
    ///   `UVar(0) -> UVar(3) -> UVar(5) -> UVar(0) -> ...`. This last one needed a different
    ///   shape of fix than the other two: `NameGen`'s naming cache (`rev_map`) is keyed by the
    ///   *structural content* of a `RustTypeDecl`, not by `UVar`, so a naive placeholder-decl
    ///   insert would get assigned a different name/index than the real decl once it's complete
    ///   (different content -> different hash) - silently wrong codegen, not just a crash. The
    ///   fix instead reserves a name/index from the active naming-context path up front (see
    ///   `NameGen::reserve_name`/`commit_reservation`), before the real decl is known.
    ///
    /// A further subtlety specific to this last fix: a bare self-reference must never by itself
    /// justify giving the type a lifetime parameter - forcing one unconditionally produces a
    /// struct/enum whose lifetime parameter is otherwise unused (rustc E0392) whenever nothing
    /// else in the type borrows anything. See `var_needs_lifetime` and the dedicated regression
    /// tests `phantom_rec_with_genuine_borrow_gets_lifetime` and `phantom_rec_union_full_pipeline`
    /// below, which check both directions (a genuine borrow elsewhere still gets a lifetime;
    /// recursion alone does not).
    #[test]
    fn phantom_rec_full_pipeline_repro() {
        let (module, f) = make_phantom_rec_module();
        let output = produce_string_gencode(&module, &f);
        assert!(!output.is_empty());
    }
}

mod numeric {
    use super::*;

    #[test]
    fn test_numtree_codegen() {
        use crate::numeric::core;
        let mut module = FormatModule::new();
        let f = module.define_format(
            "test.math",
            compute(Expr::Numeric(Box::new(core::Expr::Const(
                core::TypedConst(8u8.into(), core::NumRep::Concrete(core::MachineRep::U8)),
            )))),
        );
        let output = produce_string_gencode(&module, &f.call());
        println!("{}", output);
    }

    #[test]
    fn test_math_codegen() {
        let tree = {
            use crate::numeric::core::*;
            let eightu8 = Expr::Const(TypedConst(8u8.into(), NumRep::Concrete(MachineRep::U8)));
            let oneu16 = Expr::Const(TypedConst(1u16.into(), NumRep::Concrete(MachineRep::U16)));
            let sumu32 = Expr::BinOp(
                BinOp {
                    op: BasicBinOp::Add,
                    out_rep: Some(MachineRep::U32),
                },
                Box::new(eightu8),
                Box::new(oneu16),
            );
            sumu32
        };
        let mut module = FormatModule::new();
        let f = module.define_format("test.math", compute(Expr::Numeric(Box::new(tree))));
        let output = produce_string_gencode(&module, &f.call());
        println!("{}", output);
    }

    fn is_valid_output(output: &str) -> bool {
        // FIXME - write a more sophisticated check
        output.len() > 0
    }

    #[test]
    fn test_numtree_codegen_proptest() {
        let log = std::fs::File::create("pbt.log").unwrap();
        let handle = std::cell::RefCell::new(log);
        proptest!(|(tree in crate::numeric::core::strategy::any_expr())| {
            let mut log = handle.borrow_mut();
            use std::io::Write;
            writeln!(&mut log, "## {}", crate::numeric::printer::show_expr(&tree)).unwrap();
            let mut module = FormatModule::new();
            let f = module.define_format("test.arb", compute(Expr::Numeric(Box::new(tree))));
            let output = produce_string_gencode(&module, &f.call());
            writeln!(&mut log, "{}", output).unwrap();
            writeln!(&mut log, "\n\n").unwrap();
            prop_assert!(is_valid_output(&output))
        });
        proptest!(|(tree in crate::numeric::core::strategy::unsigned_expr())| {
            let mut log = handle.borrow_mut();
            use std::io::Write;
            writeln!(&mut log, "## {}", crate::numeric::printer::show_expr(&tree)).unwrap();
            let mut module = FormatModule::new();
            let f = module.define_format("test.arb", compute(Expr::Numeric(Box::new(tree))));
            let g = module.define_format("test.arb2", record([
                ("a", f.call()),
                ("b", compute(succ(var("a")))),
            ]));
            let output = produce_string_gencode(&module, &g.call());
            writeln!(&mut log, "{}", output).unwrap();
            writeln!(&mut log, "\n\n").unwrap();
            prop_assert!(is_valid_output(&output))
        });
        let strat = crate::numeric::core::strategy::any_expr_with_vars([("x", MachineRep::U8)]);
        proptest!(|(tree in strat)| {
            let mut log = handle.borrow_mut();
            use std::io::Write;
            writeln!(&mut log, "## {}", crate::numeric::printer::show_expr(&tree)).unwrap();
            let mut module = FormatModule::new();
            let f = module.define_format("test.arb_var", record([
                ("x", Format::Byte(ByteSet::full())),
                ("tree", compute(Expr::Numeric(Box::new(tree)))),
            ]));
            let output = produce_string_gencode(&module, &f.call());
            writeln!(&mut log, "{}", output).unwrap();
            writeln!(&mut log, "\n\n").unwrap();
            prop_assert!(is_valid_output(&output))
        })
    }
}

mod read_array {
    use super::*;

    #[test]
    fn test_fixed_format_read_array_codegen() {
        use crate::helper::{base::u16be, from_here, read_array, record};

        let mut module = FormatModule::new();
        let point = module.define_format("point", record([("x", u16be()), ("y", u16be())]));
        let f = from_here(read_array(Expr::U8(2), point));
        module.define_format("test.points", f.clone());
        let output = produce_string_gencode(&module, &f);
        println!("{output}");
        assert!(
            output.contains("ReadArray<'input, point>"),
            "expected a `ReadArray<'input, point>` type referencing the adhoc `point` struct:\n{output}"
        );
        assert!(
            output.contains("impl ReadUnchecked for point"),
            "expected an `impl ReadUnchecked for point` block to have been generated:\n{output}"
        );
        assert!(
            output.contains("as_read_array::<point>"),
            "expected the turbofish call-site to reference the adhoc `point` type:\n{output}"
        );
    }
}

mod smoke {
    use super::*;

    #[test]
    fn test_codegen_output() {
        let f = Format::Compute(Box::new(Expr::Unary(
            UnaryOp::IntPred,
            Box::new(Expr::U32(43)),
        )));
        let mut module = FormatModule::new();
        module.define_format("test.output", f.clone());
        let output = produce_string_gencode(&module, &f);
        println!("{}", output);
    }

    #[test]
    fn test_embed_expr_unit_variant() {
        let f = Format::Variant(Label::Borrowed("Empty"), Box::new(Format::EMPTY));
        let mut module = FormatModule::new();
        module.define_format("test.empty", f.clone());
        let output = produce_string_gencode(&module, &f);
        println!("{}", output);
    }

    #[test]
    fn test_simple_closure_codegen() {
        let mut module = FormatModule::new();
        let inner = module.define_format("test.inner", ANY_BYTE);
        let outer = module.define_format("test.outer", record([("inner", inner.call())]));
        let output = produce_string_gencode(&module, &outer.call());
        println!("{}", output);
    }
}

mod fixture_gen {
    use std::{fs::File, io::Write};

    use super::*;
    use crate::helper::*;

    #[test]
    fn runtime_repeat_behavior_test() {
        let mut module = FormatModule::new();
        let inner = module.define_format(
            "test.inner",
            record([("a", is_byte(0xAA)), ("bs", repeat(is_byte(0xBB)))]),
        );
        let outer = module.define_format(
            "test.outer",
            record([
                ("pairs", repeat_count(Expr::U32(2), inner.call())),
                ("end", is_byte(0xCC)),
            ]),
        );
        let top = outer.call();
        let frag = generate_code(&module, &top).to_fragment();
        const TEST_PATH: &str = "tests/runtime_repeat/mod.rs";
        let mut f = File::create(TEST_PATH).unwrap();
        write!(f, "{}", frag).unwrap();
    }

    #[test]
    #[ignore = "running this test clobbers the manually-edited test file for permit_state_error"]
    /// Test to pre-generate a Permit-parse of a slice we will peek within and manually remove the 'close_peek' within to cause a state-error we want to see
    /// how permit handles.
    fn permit_slice_no_close_peek_base_codegen() {
        let mut module = FormatModule::new();
        let in_permit = module.define_format(
            "test.in_permit",
            monad_seq(
                slice(Expr::U8(6), Format::Peek(Box::new(byte_seq(b"abcd")))),
                compute(Expr::Bool(true)),
            ),
        );
        let fallback = module.define_format("test.fallback", compute(Expr::UNIT));
        let outer = module.define_format(
            "test.main",
            alts_nondet([
                ("InPermit", permit(in_permit.call(), Expr::Bool(false))),
                ("Fallback", fallback.call()),
            ]),
        );
        let top = outer.call();
        let frag = generate_code(&module, &top).to_fragment();
        const TEST_PATH: &str = "tests/permit_state_error/mod.rs";
        let mut f = File::create(TEST_PATH).unwrap();
        write!(f, "{}", frag).unwrap();
    }
}
