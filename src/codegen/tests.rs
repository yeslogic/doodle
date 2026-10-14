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

/// Unit-tests related to the `Elaborator` implementation, mostly concerned with ensuring metavariable traversal
/// is plausibly drift-free.
mod elaboration {
    use super::*;

    /// Runs full-module inference on `module` starting from top-level format `f` using `TypeChecker`,
    /// as well as full-module elaboration using the finalized `TypeChecker` artifact, and makes the
    /// assertion that the two have the exact same 'population count' (largest-index UVar in TypeChecker should
    /// equal the final traversal-index in Elaboration).
    ///
    /// Also ensures that the Elaborator returned a TypedFormat that roundtrips back to an equivalent format
    /// to `f`.
    ///
    /// Optionally takes a string `label` to include in the panic message if the `assert_eq!` for
    /// population-count fails.
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

    /// Batcher that takes an array of labelled formats and loops over them,
    /// registering and running `population_check` over each in turn.
    fn run_headcount(fs: &[(&'static str, Format)]) {
        let mut module = FormatModule::new();
        for (name, f) in fs.iter() {
            module.define_format(*name, f.clone());
            population_check(&module, f, Some(*name));
        }
    }

    /// Phase 4's codegen-path counterpart to `lib.rs`'s
    /// `define_format_rec_batch_mutual_recursion_ping_pong_record_variant_decodes`: the same
    /// record-shaped mutual recursion, but run through the real production codegen path
    /// (`generate_code`/`GTCompiler`) rather than the interpreter, since Phase 4's "Finding B"
    /// (see `experiments/doodle-rec/PLAN.md`) found the identically-shaped `LetFormat`/`MonadSeq`
    /// bug in `codegen::typed_decoder::GTCompiler::compile_gt_format` compiled `pong`'s reference
    /// back to `ping` into a dead, always-failing decoder rather than reusing `ping`'s real one.
    #[test]
    fn recursive_format_through_record_field_generates_no_dead_decoder() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let ping_body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Stop"), Box::new(is_byte(b'X'))),
            Format::Variant(
                Label::Borrowed("ToPong"),
                Box::new(record([
                    ("tag", is_byte(b'p')),
                    ("next", Format::RecVar(1)),
                ])),
            ),
        ]);
        let pong_body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Stop"), Box::new(is_byte(b'Y'))),
            Format::Variant(
                Label::Borrowed("ToPing"),
                Box::new(record([
                    ("tag", is_byte(b'q')),
                    ("next", Format::RecVar(0)),
                ])),
            ),
        ]);
        let refs = module.define_format_rec_batch(vec![
            ("test.ping_rec_cg", ping_body),
            ("test.pong_rec_cg", pong_body),
        ]);
        let ping_ref = refs[0];

        let src = produce_string_gencode(&module, &ping_ref.call());
        assert!(
            !src.contains("FailToken"),
            "generated decoder for a self-referential record field must not contain a dead \
             always-failing decoder:\n{src}"
        );
    }

    /// Base-case headcount test for an assortment of leaf-node formats
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
    fn phase3_final_check_peano_and_ping_pong() {
        let peano = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Z"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::RecVar(0),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let frefs = module.define_format_rec_batch(vec![(Label::Borrowed("peano"), peano)]);
        let top = frefs[0].call();
        let src = produce_string_gencode(&module, &top);
        assert!(src.contains("Box<peano>"), "source was:\n{src}");

        let ping = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Done"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("More"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'A'])),
                    Format::RecVar(1),
                ])),
            ),
        ]);
        let pong = Format::Tuple(vec![Format::Byte(ByteSet::from([b'B'])), Format::RecVar(0)]);
        let mut module2 = FormatModule::new();
        let frefs2 = module2.define_format_rec_batch(vec![
            (Label::Borrowed("ping"), ping),
            (Label::Borrowed("pong"), pong),
        ]);
        let top2 = frefs2[0].call();
        let src2 = produce_string_gencode(&module2, &top2);
        assert!(src2.contains("Box<ping>"), "source was:\n{src2}");
    }

    /// All direct child [`TypedFormat`]s of `tf`, for a generic (variant-agnostic) traversal.
    ///
    /// Returns owned `Rc`s rather than borrowed references because `FormatCall`'s child now lives
    /// behind a `LevelCell` (`Rc<RefCell<Rc<GTFormat>>>`) - `def.borrow().clone()` is a cheap `Rc`
    /// clone, not a deep copy, and every other variant's `Box`ed child is likewise cheaply re-homed
    /// in an `Rc` just for this (test-only) traversal.
    fn typed_format_children(tf: &GTFormat) -> Vec<Rc<GTFormat>> {
        use TypedFormat::*;
        let boxed = |f: &Box<GTFormat>| Rc::new((**f).clone());
        match tf {
            FormatCall(_, _, _, _, def) => vec![def.borrow().clone()],
            ForEach(_, _, _, f) => vec![boxed(f)],
            Fail | EndOfInput | Align(_) | Byte(_) | Pos(_) | SkipRemainder | WithView(..) => {
                vec![]
            }
            Variant(_, _, f) => vec![boxed(f)],
            Union(_, fs) | UnionNondet(_, fs) | Tuple(_, fs) | Sequence(_, fs) => {
                fs.iter().map(|f| Rc::new(f.clone())).collect()
            }
            Repeat(_, f) | Repeat1(_, f) => vec![boxed(f)],
            RepeatCount(_, _, f) => vec![boxed(f)],
            RepeatBetween(_, _, f) => vec![boxed(f)],
            RepeatUntilLast(_, _, f) | RepeatUntilSeq(_, _, f) => vec![boxed(f)],
            Maybe(_, _, f) => vec![boxed(f)],
            Peek(_, f) | PeekNot(_, f) => vec![boxed(f)],
            Slice(_, _, f) => vec![boxed(f)],
            Bits(_, f) => vec![boxed(f)],
            WithRelativeOffset(_, _, _, f) => vec![boxed(f)],
            Map(_, f, _) => vec![boxed(f)],
            Where(_, f, _) => vec![boxed(f)],
            Compute(_, _) => vec![],
            Let(_, _, _, f) => vec![boxed(f)],
            Match(_, _, branches) => branches.iter().map(|(_, f)| Rc::new(f.clone())).collect(),
            Dynamic(_, _, _, f) => vec![boxed(f)],
            Apply(_, _, _) => vec![],
            DecodeBytes(_, _, f) => vec![boxed(f)],
            ParseFromView(_, _, f) => vec![boxed(f)],
            LetFormat(_, f0, _, f1) => vec![boxed(f0), boxed(f1)],
            MonadSeq(_, f0, f1) => vec![boxed(f0), boxed(f1)],
            Hint(_, _, f) => vec![boxed(f)],
            AccumUntil(_, _, _, _, _, f) => vec![boxed(f)],
            LiftedOption(_, f) => f.as_ref().map(boxed).into_iter().collect(),
            LetView(_, _, f) => vec![boxed(f)],
            Phantom(_, f) => vec![boxed(f)],
            #[cfg(feature = "format_enforce")]
            Enforce(_, f) => vec![boxed(f)],
            Permit(_, f, _) => vec![boxed(f)],
        }
    }

    /// Recursively checks that [`TypedFormat::is_nonproductive`] agrees with the
    /// outcome of calling [`Format::is_nonproductive`] on `Format::from(tf.clone())`,
    /// on every distinct (structurally unique) `TypedFormat` node reachable from `tf`.
    ///
    /// `seen` is used to dedup nodes that are reached more than once (e.g. Rc-shared
    /// `def` of multiple FormatCalls pointing to the same level) so each unique format
    /// is only tested once.
    fn check_is_nonproductive_agrees(
        tf: &GTFormat,
        module: &FormatModule,
        seen: &mut std::collections::HashSet<GTFormat>,
    ) {
        if !seen.insert(tf.clone()) {
            return;
        }
        let erased: Format = tf.clone().into();
        assert_eq!(
            tf.is_nonproductive(),
            erased.is_nonproductive(module),
            "TypedFormat::is_nonproductive disagreed with erase-then-Format::is_nonproductive for {tf:?}"
        );
        for child in typed_format_children(tf) {
            check_is_nonproductive_agrees(&child, module, seen);
        }
    }

    /// Builds a JSON-like, 4-member mutually-recursive format (value/member/array/object -
    /// deliberately a bit more elaborate than `doodle-rec`'s own `json_lite` sandbox format, though
    /// still far short of full JSON: no escapes, no floats/negatives, unquoted-alpha-only keys),
    /// elaborates it exactly as the real `Generator::compile` pipeline does, and checks that
    /// `TypedFormat::is_nonproductive` agrees with the clone-erase-`Format::is_nonproductive` path
    /// at every distinct node of the elaborated tree - including, critically, nested `FormatCall`
    /// occurrences reached while their own level was still open during elaboration (whose `def` is
    /// only the elaborator's placeholder `TypedFormat::Fail`, per its own doc comment in
    /// `Elaborator::elaborate_format`'s `Format::ItemVar` arm - see `src/codegen/mod.rs`).
    #[test]
    fn is_nonproductive_agrees() {
        use crate::helper::{alts, byte_seq, is_byte, optional, record, repeat, repeat1, tuple};

        fn digit() -> Format {
            Format::Byte(ByteSet::from(b'0'..=b'9'))
        }
        fn alpha() -> Format {
            Format::Byte(ByteSet::from(b'a'..=b'z').union(&ByteSet::from(b'A'..=b'Z')))
        }
        fn string_lit() -> Format {
            tuple([is_byte(b'"'), repeat(alpha()), is_byte(b'"')])
        }

        // Batch layout: 0 = value, 1 = member, 2 = array, 3 = object
        let value = alts([
            ("Null", byte_seq(b"null")),
            ("True", byte_seq(b"true")),
            ("False", byte_seq(b"false")),
            ("Number", repeat1(digit())),
            ("String", string_lit()),
            ("Array", Format::RecVar(2)),
            ("Object", Format::RecVar(3)),
        ]);
        let member = record([
            ("key", string_lit()),
            ("colon", is_byte(b':')),
            ("value", Format::RecVar(0)),
        ]);
        let array = tuple([
            is_byte(b'['),
            optional(tuple([
                Format::RecVar(0),
                repeat(tuple([is_byte(b','), Format::RecVar(0)])),
            ])),
            is_byte(b']'),
        ]);
        let object = tuple([
            is_byte(b'{'),
            optional(tuple([
                Format::RecVar(1),
                repeat(tuple([is_byte(b','), Format::RecVar(1)])),
            ])),
            is_byte(b'}'),
        ]);

        let mut module = FormatModule::new();
        let refs = module.define_format_rec_batch(vec![
            (Label::Borrowed("json.value"), value),
            (Label::Borrowed("json.member"), member),
            (Label::Borrowed("json.array"), array),
            (Label::Borrowed("json.object"), object),
        ]);
        let top = refs[0].call();

        let tc = TypeChecker::infer_module(&module, &top).unwrap();
        let mut elaborator = Elaborator::new(&module, tc, CodeGen::new());
        let (top_typed, extra_typed) = elaborator.elaborate_module(&module, &top);

        let mut seen = std::collections::HashSet::new();
        check_is_nonproductive_agrees(&top_typed, &module, &mut seen);
        for extra in &extra_typed {
            check_is_nonproductive_agrees(extra, &module, &mut seen);
        }
        assert!(
            seen.len() > 10,
            "expected to have walked a nontrivial number of distinct TypedFormat nodes, only saw {}",
            seen.len()
        );
    }

    /// Generates small, non-recursive `Format` trees biased toward the variants
    /// `is_nonproductive_agrees` above doesn't exercise (that test's JSON-like format never
    /// constructs `Peek`/`PeekNot`/`Slice`/`RepeatCount`/`RepeatBetween`/`Match`/`UnionNondet`/
    /// `Map`/`Where`/`WithRelativeOffset`) - a fresh, arm-by-arm check on the mechanical port of
    /// `TypedFormat::match_bounds_recursive`/`lookahead_bounds_recursive`, independent of the
    /// recursion/placeholder concern the other test targets. Still not exhaustive: `Let`, `Dynamic`,
    /// `Apply`, `DecodeBytes`, `ParseFromView`, `AccumUntil`, `ForEach`, `LetView`, `WithView`,
    /// `Phantom`, `Permit`, `Bits` are left uncovered (each needs either dynamic-format registration
    /// or view machinery to construct validly, and are lower-value here since they're rare in real
    /// `doodle-formats` definitions).
    fn arb_gap_filling_format() -> impl Strategy<Value = Format> {
        use crate::helper::{
            f_id, fmt_match, map, record, repeat_between, repeat_count, repeat1, slice,
            union_nondet, where_lambda, with_relative_offset,
        };

        let leaf = prop_oneof![
            Just(Format::Byte(ByteSet::full())),
            Just(Format::EndOfInput),
            Just(Format::Fail),
            Just(Format::align(4)),
            Just(Format::Compute(Box::new(Expr::U8(0)))),
        ];

        leaf.prop_recursive(3, 12, 2, |inner| {
            prop_oneof![
                inner.clone().prop_map(|f| Format::Peek(Box::new(f))),
                inner.clone().prop_map(|f| Format::PeekNot(Box::new(f))),
                inner.clone().prop_map(repeat1),
                inner.clone().prop_map(|f| repeat_count(Expr::U8(2), f)),
                inner.clone().prop_map(|f| repeat_between(0, 2, f)),
                inner.clone().prop_map(|f| slice(Expr::U8(1), f)),
                inner.clone().prop_map(|f| map(f, f_id())),
                inner
                    .clone()
                    .prop_map(|f| where_lambda(f, "x", Expr::Bool(true))),
                inner
                    .clone()
                    .prop_map(|f| fmt_match(Expr::U8(0), [(Pattern::Wildcard, f)])),
                inner
                    .clone()
                    .prop_map(|f| with_relative_offset(None, Expr::U8(0), f)),
                inner.clone().prop_map(|f| record([("x", f)])),
                proptest::collection::vec(inner.clone(), 1..3).prop_map(|fs| union_nondet(fs)),
            ]
        })
    }

    proptest! {
        /// Property-test counterpart to `is_nonproductive_agrees`: over many small, arbitrary,
        /// non-recursive `Format` trees drawn from `arb_gap_filling_format`, checks the same
        /// `TypedFormat::is_nonproductive`-agrees-with-erasure property at every distinct node.
        /// A generated tree that fails to typecheck is discarded (not a property violation - just
        /// an input `arb_gap_filling_format` shouldn't have produced, e.g. from an unforeseen
        /// interaction between two combined combinators); genuine mismatches still panic through
        /// `check_is_nonproductive_agrees`'s `assert_eq!` and fail the test normally.
        #[test]
        fn is_nonproductive_agrees_over_arbitrary_formats(f in arb_gap_filling_format()) {
            let module = FormatModule::new();
            let Ok(tc) = TypeChecker::infer_module(&module, &f) else {
                return Ok(());
            };
            let mut elaborator = Elaborator::new(&module, tc, CodeGen::new());
            let (top_typed, extra_typed) = elaborator.elaborate_module(&module, &f);

            let mut seen = std::collections::HashSet::new();
            check_is_nonproductive_agrees(&top_typed, &module, &mut seen);
            for extra in &extra_typed {
                check_is_nonproductive_agrees(extra, &module, &mut seen);
            }
        }
    }

    /// `pair_a`/`pair_b` is a directly self-referential bare `Tuple` cycle with no `Record`/`Union`
    /// boundary anywhere on it - originally written to confirm this anti-pattern is rejected
    /// cleanly by codegen's box-placement `unreachable!("Expansion::Tuple(UVar) is directly
    /// self-referential ... not yet supported by codegen")` rather than hanging or mis-generating.
    ///
    /// A bare-`Tuple`-only cycle like this is, by construction, *also* always a cycle with no
    /// reachable base case (a `Tuple`'s own sequencing has no analog of `Union`'s "pick a different
    /// branch" escape, and nothing here wraps the recursive field in a `Repeat` to give it one) - so
    /// `FormatModule::check_batch_has_base_case` now rejects this exact shape earlier, at
    /// `define_format_rec_batch` registration time, before a `FormatRef` to it can ever reach
    /// `produce_string_gencode` (or the interpreter) to hit the older codegen-specific guard at all.
    /// The codegen `unreachable!` is kept as a defensive backstop - see `doc/RECURSION.md`'s
    /// Anti-patterns section - since `define_format_rec_batch`/`define_format_recursive` are
    /// documented as the only sanctioned entry points for real recursion, so nothing in the public
    /// API can currently construct a batch member that reaches it.
    #[test]
    #[should_panic(expected = "no reachable base case for: pair_a, pair_b")]
    fn phase3_final_check_pure_tuple_cycle_rejects_cleanly() {
        let pair_a = Format::Tuple(vec![Format::Byte(ByteSet::from([b'A'])), Format::RecVar(1)]);
        let pair_b = Format::Tuple(vec![Format::Byte(ByteSet::from([b'B'])), Format::RecVar(0)]);
        let mut module = FormatModule::new();
        let frefs = module.define_format_rec_batch(vec![
            (Label::Borrowed("pair_a"), pair_a),
            (Label::Borrowed("pair_b"), pair_b),
        ]);
        let top = frefs[0].call();
        let _src = produce_string_gencode(&module, &top);
    }

    /// Headcount test for a record-format consisting of only leaf-node field-formats
    #[test]
    fn test_headcount_record_simple() {
        let f = Format::record(vec![
            ("any_byte", Format::Byte(ByteSet::full())),
            ("align64", Format::align(64)),
            ("eoi", Format::EndOfInput),
        ]);

        run_headcount(&[("record_simple", f)]);
    }

    /// Headcount test for a simple tagged-union definition.
    #[test]
    fn test_headcount_adt_simple() {
        let f = Format::Union(vec![
            Format::Variant("some".into(), Box::new(Format::Byte(ByteSet::full()))),
            Format::Variant("none".into(), Box::new(Format::EMPTY)),
        ]);

        run_headcount(&[("adt_simple", f)]);
    }

    /// Headcount test for a top-level format that uses `Format::ItemVar` (without any arguments)
    #[test]
    fn test_headcount_item_var() {
        let sub_f = Format::Byte(ByteSet::full());
        let mut module = FormatModule::new();
        let sub_ref = module.define_format("test.any_byte", sub_f);
        let f = sub_ref.call();
        module.define_format("test.call_any_byte", f.clone());
        population_check(&module, &f, None);
    }

    /// Headcount test for Expr embedded using `Format::Compute`
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

    /// Headcount test for a format with complex embedded `Expr`s, including lambdas
    /// (`Format::RepeatUntilLast`, `Expr::FlatMapAccum`)
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

/// Regression tests for formats defined via `FormatModule::define_format_phantom_rec`, mostly concerned with
/// cycle-safe caching across the pipeline and lifetime inference for self-referential types.
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

/// Codegen tests for `Expr::Numeric` trees, including property-based tests over arbitrary numeric expressions.
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

    /// Watermark heuristic designed to check that generated-code looks 'sensible'
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

/// Codegen tests for `ReadArray` over fixed-shape record formats.
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

/// Smoke tests running small formats through the full codegen pipeline, with minimal assertions on the output.
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

    #[test]
    fn test_expr_cast_noop_codegen() {
        use crate::helper::{as_u32, compute, record, repeat, seq_length, var};

        let mut module = FormatModule::new();
        let format = record([
            ("x", repeat(Format::ANY_BYTE)),
            ("len", compute(as_u32(seq_length(var("x"))))),
        ]);
        let fref = module.define_format("test", format);
        let output = produce_string_gencode(&module, &fref.call());
        let anti_pattern = "as u32) as u32";
        let hits: Vec<String> = output
            .lines()
            .enumerate()
            .filter(|(_, line)| line.contains(anti_pattern))
            .map(|(ix, line)| format!("{:>5}| {}", ix + 1, line.trim()))
            .collect();
        if !hits.is_empty() {
            panic!(
                "redundant cast `{anti_pattern}` emitted for AsU32 over a u32 operand ({} hit(s)):\n{}",
                hits.len(),
                hits.join("\n")
            );
        } else {
            println!("{}", output);
        }
    }
}

/// Generators for the standalone codegen fixtures under `tests/` (`runtime_repeat`, `permit_state_error`).
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

    /// Regenerates `tests/recursion/mod.rs` (Phase 5's capstone fixture, see
    /// `experiments/doodle-rec/PLAN.md` and `doc/RECURSION.md`) from the real production codegen
    /// path, for provenance: that file should always be exactly what the last run of this test
    /// produced, never hand-edited. `peano` and `ping` are the same Union-based shapes as
    /// `elaboration::phase3_final_check_peano_and_ping_pong`, but `pong` is a named
    /// `Format::record` here, not a raw `Tuple` - a raw-`Tuple` `pong` hits Finding A's still-open
    /// recompute inconsistency (`Decoder_pong`'s own return-type signature disagreeing with
    /// `ping::More`'s field type on whether `pong`'s back-reference to `ping` needs `Box`ing) the
    /// moment the generated source is actually compiled, which that test itself never does
    /// (string-level check only). Both batches are registered in one shared
    /// `FormatModule`, with a synthetic `Tuple` of both entry points as the `generate_code` top
    /// format - never actually decoded as such by `tests/recursion/codegen_tests.rs` (which calls
    /// `Decoder_peano`/`Decoder_ping` directly), only used to make both batches reachable from one
    /// codegen invocation so the fixture is one coherent, consistently-numbered artifact rather
    /// than two separately-generated snippets pasted together.
    ///
    /// After changing the format definitions below, run:
    /// ```sh
    /// cargo test --lib -p doodle codegen::tests::fixture_gen::regenerate_recursion_fixture -- --ignored
    /// ```
    /// then re-run `cargo test --test recursion` to confirm the frozen decoders still behave as
    /// `codegen_tests.rs` expects.
    #[test]
    #[ignore = "regenerates tests/recursion/mod.rs from the real codegen path; run explicitly after changing the format definitions in this test"]
    fn regenerate_recursion_fixture() {
        let peano = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Z"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'S'])),
                    Format::RecVar(0),
                ])),
            ),
        ]);
        let ping = Format::Union(vec![
            Format::Variant(
                Label::Borrowed("Done"),
                Box::new(Format::Byte(ByteSet::from([b'Z']))),
            ),
            Format::Variant(
                Label::Borrowed("More"),
                Box::new(Format::Tuple(vec![
                    Format::Byte(ByteSet::from([b'A'])),
                    Format::RecVar(1),
                ])),
            ),
        ]);
        let pong = record([
            ("tag", Format::Byte(ByteSet::from([b'B']))),
            ("next", Format::RecVar(0)),
        ]);

        let mut module = FormatModule::new();
        let peano_refs = module.define_format_rec_batch(vec![(Label::Borrowed("peano"), peano)]);
        let pingpong_refs = module.define_format_rec_batch(vec![
            (Label::Borrowed("ping"), ping),
            (Label::Borrowed("pong"), pong),
        ]);
        let top = Format::Tuple(vec![peano_refs[0].call(), pingpong_refs[0].call()]);

        let src = produce_string_gencode(&module, &top);
        let dest = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/recursion/mod.rs");
        std::fs::write(dest, src).expect("failed to regenerate tests/recursion/mod.rs");
    }
}
