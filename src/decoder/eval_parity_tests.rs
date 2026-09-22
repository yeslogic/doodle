//! Differential tests asserting that `Expr::eval_value` (see `decoder.rs`) and `Expr::eval_value_with_loc`
//! (see `loc_decoder.rs`) agree on every input.
//!
//! The two evaluators are hand-maintained mirrors of one another. These tests exist as a safety net for
//! unifying them: each [`Case`] is run through both, and the observable outcomes (result `Value`, error
//! message, or "panicked") are compared.
//!
//! Known, not-yet-fixed divergences are recorded as [`Expect::Diverges`] (none at present, now that
//! `Expr::eval`/`Expr::eval_with_loc` both delegate to the shared `Expr::eval_generic`). Such a case
//! *fails* once the evaluators start to agree, at which point it should be switched to
//! [`Expect::Parity`].

use std::panic::{AssertUnwindSafe, catch_unwind};

use super::{MultiScope, Scope, SeqKind, Value};
use crate::loc_decoder::{LocMultiScope, LocScope, ParsedValue};
use crate::{Arith, Expr, IntRel, Label, Pattern, TypeHint, ValueType};

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Value(String),
    Err(String),
    Panic,
}

/// Rewrites every `SeqKind::Dup` into `SeqKind::Strict`, since `ParsedValue::from_evaluated` materializes
/// `Dup` sequences and the two evaluators would otherwise differ in representation only.
fn normalize(v: Value) -> Value {
    match v {
        Value::Seq(elts) => Value::Seq(SeqKind::Strict(elts.into_iter().map(normalize).collect())),
        Value::Tuple(vs) => Value::Tuple(vs.into_iter().map(normalize).collect()),
        Value::Record(fs) => {
            Value::Record(fs.into_iter().map(|(l, v)| (l, normalize(v))).collect())
        }
        Value::Variant(l, inner) => Value::Variant(l, Box::new(normalize(*inner))),
        Value::Option(opt) => Value::Option(opt.map(|inner| Box::new(normalize(*inner)))),
        other => other,
    }
}

fn observe(f: impl FnOnce() -> Result<Value, crate::error::EvalError>) -> Outcome {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(v)) => Outcome::Value(format!("{:?}", normalize(v))),
        Ok(Err(e)) => Outcome::Err(e.to_string()),
        Err(_) => Outcome::Panic,
    }
}

fn eval_main(expr: &Expr, vars: &[(&'static str, Value)]) -> Outcome {
    observe(|| {
        let root = Scope::Empty;
        let mut multi = MultiScope::with_capacity(&root, vars.len());
        for (name, value) in vars {
            multi.push(*name, value);
        }
        let scope = Scope::Multi(&multi);
        expr.eval_value(&scope)
    })
}

fn eval_loc(expr: &Expr, vars: &[(&'static str, Value)]) -> Outcome {
    observe(|| {
        let root = LocScope::Empty;
        let mut multi = LocMultiScope::with_capacity(&root, vars.len());
        for (name, value) in vars {
            multi.push_owned(*name, ParsedValue::from_evaluated(value.clone()));
        }
        let scope = LocScope::Multi(&multi);
        expr.eval_value_with_loc(&scope)
    })
}

enum Expect {
    Parity,
    /// Currently unused (no known divergences), but kept available: a future change that
    /// reintroduces a divergence should be caught here and documented via [`diverges`]/
    /// [`diverges_with`] rather than silently breaking [`Expect::Parity`] cases.
    #[allow(dead_code)]
    Diverges(&'static str),
}

struct Case {
    name: &'static str,
    expr: Expr,
    vars: Vec<(&'static str, Value)>,
    expect: Expect,
}

fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

/// The evaluators ignore type hints, so any `ValueType` will do
fn hint() -> TypeHint {
    ValueType::NumericHole.into()
}

fn var(name: &'static str) -> Expr {
    Expr::Var(Label::from(name))
}

fn lam(name: &'static str, body: Expr) -> Expr {
    Expr::Lambda(Label::from(name), b(body))
}

fn seq_u8(items: &[u8]) -> Expr {
    Expr::Seq(items.iter().copied().map(Expr::U8).collect())
}

fn bytes(items: &[u8]) -> Expr {
    Expr::Tuple(items.iter().copied().map(Expr::U8).collect())
}

fn parity(name: &'static str, expr: Expr) -> Case {
    Case {
        name,
        expr,
        vars: Vec::new(),
        expect: Expect::Parity,
    }
}

fn parity_with(name: &'static str, expr: Expr, vars: Vec<(&'static str, Value)>) -> Case {
    Case {
        name,
        expr,
        vars,
        expect: Expect::Parity,
    }
}

#[allow(dead_code)]
fn diverges(name: &'static str, why: &'static str, expr: Expr) -> Case {
    diverges_with(name, why, expr, Vec::new())
}

#[allow(dead_code)]
fn diverges_with(
    name: &'static str,
    why: &'static str,
    expr: Expr,
    vars: Vec<(&'static str, Value)>,
) -> Case {
    Case {
        name,
        expr,
        vars,
        expect: Expect::Diverges(why),
    }
}

/// `\p -> (p.0, [p.1])`, the lambda shape used by `FlatMapAccum`
fn accum_pass_through() -> Expr {
    lam(
        "p",
        Expr::Tuple(vec![
            Expr::TupleProj(b(var("p")), 0),
            Expr::Seq(vec![Expr::TupleProj(b(var("p")), 1)]),
        ]),
    )
}

/// `\p -> p.1`, the lambda shape used by `LeftFold` (keeps the last element seen)
fn fold_last() -> Expr {
    lam("p", Expr::TupleProj(b(var("p")), 1))
}

fn range(start: u32, stop: u32) -> Expr {
    Expr::EnumFromTo(b(Expr::U32(start)), b(Expr::U32(stop)))
}

fn cases() -> Vec<Case> {
    let mut cases = vec![
        // -- literals and structural constructors
        parity("bool", Expr::Bool(true)),
        parity("u8", Expr::U8(7)),
        parity("u64", Expr::U64(u64::MAX)),
        parity("tuple", Expr::Tuple(vec![Expr::U8(1), Expr::Bool(false)])),
        parity(
            "tuple_proj",
            Expr::TupleProj(b(Expr::Tuple(vec![Expr::U8(1), Expr::U16(2)])), 1),
        ),
        parity(
            "record_proj",
            Expr::RecordProj(
                b(Expr::Record(vec![
                    ("a".into(), Expr::U8(1)),
                    ("b".into(), Expr::U16(2)),
                ])),
                "b".into(),
            ),
        ),
        parity("variant", Expr::Variant("v".into(), b(Expr::U8(3)))),
        parity("seq", seq_u8(&[1, 2, 3])),
        parity("lift_option_some", Expr::LiftOption(Some(b(Expr::U8(1))))),
        parity("lift_option_none", Expr::LiftOption(None)),
        // -- variables
        parity_with(
            "var_tuple_proj",
            Expr::TupleProj(b(var("t")), 0),
            vec![("t", Value::Tuple(vec![Value::U8(9), Value::Bool(true)]))],
        ),
        parity_with(
            "var_record_proj",
            Expr::RecordProj(b(var("r")), "x".into()),
            vec![("r", Value::record([("x", Value::U32(5))]))],
        ),
        // -- arithmetic and relations
        parity(
            "add_ok",
            Expr::Arith(Arith::Add, b(Expr::U8(1)), b(Expr::U8(2))),
        ),
        parity(
            "add_overflow",
            Expr::Arith(Arith::Add, b(Expr::U8(255)), b(Expr::U8(1))),
        ),
        parity(
            "sub_underflow",
            Expr::Arith(Arith::Sub, b(Expr::U16(0)), b(Expr::U16(1))),
        ),
        parity(
            "div_zero",
            Expr::Arith(Arith::Div, b(Expr::U32(4)), b(Expr::U32(0))),
        ),
        parity(
            "rel_lt",
            Expr::IntRel(IntRel::Lt, b(Expr::U8(1)), b(Expr::U8(2))),
        ),
        parity(
            "rel_eq_false",
            Expr::IntRel(IntRel::Eq, b(Expr::U64(1)), b(Expr::U64(2))),
        ),
        // -- narrowing / widening casts
        parity("as_u8_ok", Expr::AsU8(b(Expr::U32(200)))),
        parity("as_u8_overflow", Expr::AsU8(b(Expr::U32(300)))),
        parity("as_u16_widen", Expr::AsU16(b(Expr::U8(9)))),
        parity("as_u32_overflow", Expr::AsU32(b(Expr::U64(u64::MAX)))),
        parity("as_u64_widen", Expr::AsU64(b(Expr::U32(9)))),
        parity(
            "as_u8_from_usize",
            Expr::AsU8(b(Expr::SeqLength(b(seq_u8(&[1, 2]))))),
        ),
        parity("as_char_u8", Expr::AsChar(b(Expr::U8(65)))),
        parity("as_char_u32_invalid", Expr::AsChar(b(Expr::U32(0xD800)))),
        parity("as_char_u64_overflow", Expr::AsChar(b(Expr::U64(u64::MAX)))),
        // -- byte-tuple decoders
        parity("u16be", Expr::U16Be(b(bytes(&[1, 2])))),
        parity("u16le", Expr::U16Le(b(bytes(&[1, 2])))),
        parity("u32be", Expr::U32Be(b(bytes(&[1, 2, 3, 4])))),
        parity("u32le", Expr::U32Le(b(bytes(&[1, 2, 3, 4])))),
        parity("u64be", Expr::U64Be(b(bytes(&[1, 2, 3, 4, 5, 6, 7, 8])))),
        parity("u64le", Expr::U64Le(b(bytes(&[1, 2, 3, 4, 5, 6, 7, 8])))),
        parity("u16be_bad_shape", Expr::U16Be(b(bytes(&[1, 2, 3])))),
        // -- sequences
        parity("seq_length", Expr::SeqLength(b(seq_u8(&[1, 2, 3])))),
        parity("seq_length_range", Expr::SeqLength(b(range(2, 7)))),
        parity(
            "seq_ix",
            Expr::SeqIx(b(seq_u8(&[10, 20, 30])), b(Expr::U32(1))),
        ),
        parity(
            "seq_ix_oob",
            Expr::SeqIx(b(seq_u8(&[10, 20, 30])), b(Expr::U32(3))),
        ),
        parity("seq_ix_range", Expr::SeqIx(b(range(5, 9)), b(Expr::U32(2)))),
        parity(
            "seq_ix_range_oob",
            Expr::SeqIx(b(range(5, 9)), b(Expr::U32(4))),
        ),
        parity(
            "sub_seq",
            Expr::SubSeq(b(seq_u8(&[1, 2, 3, 4])), b(Expr::U32(1)), b(Expr::U32(2))),
        ),
        parity(
            "sub_seq_oob",
            Expr::SubSeq(b(seq_u8(&[1, 2, 3, 4])), b(Expr::U32(3)), b(Expr::U32(5))),
        ),
        parity(
            "sub_seq_range",
            Expr::SubSeq(b(range(10, 20)), b(Expr::U32(2)), b(Expr::U32(3))),
        ),
        parity(
            "sub_seq_inflate_wrap",
            Expr::SubSeqInflate(b(seq_u8(&[1, 2, 3])), b(Expr::U32(1)), b(Expr::U32(6))),
        ),
        parity(
            "sub_seq_inflate_range",
            Expr::SubSeqInflate(b(range(0, 3)), b(Expr::U32(1)), b(Expr::U32(6))),
        ),
        // Regression cases for Group 5 (sequence-bounds panic -> EvalError conversion).
        parity(
            "sub_seq_range_full",
            // start=0, length=3 exactly spans a 3-element range: the boundary case a previous
            // off-by-one in `sub_range`'s bounds check used to reject.
            Expr::SubSeq(b(range(0, 3)), b(Expr::U32(0)), b(Expr::U32(3))),
        ),
        parity(
            "sub_seq_inflate_start_oob",
            // start=5 is past the end of a 3-element source with nothing yet accumulated to
            // self-reference: used to index an empty Vec and panic.
            Expr::SubSeqInflate(b(seq_u8(&[1, 2, 3])), b(Expr::U32(5)), b(Expr::U32(2))),
        ),
        parity(
            "sub_seq_inflate_range_start_oob",
            Expr::SubSeqInflate(b(range(0, 3)), b(Expr::U32(5)), b(Expr::U32(2))),
        ),
        parity(
            "sub_seq_inflate_start_oob_zero_length",
            // start is out of bounds, but length=0 means it's never dereferenced, so this should
            // still succeed (an empty Seq), not error.
            Expr::SubSeqInflate(b(seq_u8(&[1, 2, 3])), b(Expr::U32(5)), b(Expr::U32(0))),
        ),
        parity("append", Expr::Append(b(seq_u8(&[1])), b(seq_u8(&[2, 3])))),
        parity(
            "append_lhs_empty",
            Expr::Append(b(seq_u8(&[])), b(seq_u8(&[2, 3]))),
        ),
        parity(
            "append_rhs_empty",
            Expr::Append(b(seq_u8(&[1])), b(seq_u8(&[]))),
        ),
        parity("append_range", Expr::Append(b(range(0, 2)), b(range(5, 7)))),
        parity("dup", Expr::Dup(b(Expr::U32(3)), b(Expr::U8(7)))),
        parity("dup_zero", Expr::Dup(b(Expr::U32(0)), b(Expr::U8(7)))),
        parity("enum_from_to", range(3, 6)),
        parity("enum_from_to_empty", range(3, 3)),
        // -- higher-order sequence operations
        parity(
            "flat_map_seq",
            Expr::FlatMap(
                b(lam("x", Expr::Seq(vec![var("x"), var("x")]))),
                b(seq_u8(&[1, 2, 3])),
            ),
        ),
        parity(
            "find_by_key_unsorted",
            Expr::FindByKey(
                false,
                b(lam("x", var("x"))),
                b(Expr::U8(2)),
                b(seq_u8(&[3, 2, 1])),
            ),
        ),
        parity(
            "find_by_key_sorted_hit",
            Expr::FindByKey(
                true,
                b(lam("x", var("x"))),
                b(Expr::U8(2)),
                b(seq_u8(&[1, 2, 3])),
            ),
        ),
        parity(
            "find_by_key_missing",
            Expr::FindByKey(
                true,
                b(lam("x", var("x"))),
                b(Expr::U8(9)),
                b(seq_u8(&[1, 2, 3])),
            ),
        ),
        // -- pattern-matching
        parity(
            "match_first_branch",
            Expr::Match(
                b(Expr::U8(1)),
                vec![
                    (Pattern::U8(1), Expr::U8(10)),
                    (Pattern::Wildcard, Expr::U8(20)),
                ],
            ),
        ),
        parity(
            "match_fallthrough_binding",
            Expr::Match(
                b(Expr::U8(5)),
                vec![
                    (Pattern::U8(1), Expr::U8(10)),
                    (Pattern::Binding("n".into()), var("n")),
                ],
            ),
        ),
        parity(
            "match_non_exhaustive",
            Expr::Match(b(Expr::U8(5)), vec![(Pattern::U8(1), Expr::U8(10))]),
        ),
        parity(
            "destructure_tuple",
            Expr::Destructure(
                b(Expr::Tuple(vec![Expr::U8(1), Expr::U8(2)])),
                Pattern::Tuple(vec![
                    Pattern::Binding("a".into()),
                    Pattern::Binding("b".into()),
                ]),
                b(Expr::Arith(Arith::Add, b(var("a")), b(var("b")))),
            ),
        ),
        parity(
            "destructure_refuted",
            Expr::Destructure(b(Expr::U8(1)), Pattern::U8(2), b(Expr::U8(0))),
        ),
    ];

    // -- accumulating / folding operations (seq input)
    cases.push(parity(
        "flat_map_accum_seq",
        Expr::FlatMapAccum(
            b(accum_pass_through()),
            b(Expr::U8(0)),
            hint(),
            b(seq_u8(&[1, 2, 3])),
        ),
    ));
    cases.push(parity(
        "left_fold_seq",
        Expr::LeftFold(
            b(fold_last()),
            b(Expr::U8(0)),
            hint(),
            b(seq_u8(&[1, 2, 3])),
        ),
    ));
    cases.push(parity(
        "flat_map_list_seq",
        Expr::FlatMapList(
            b(lam("p", Expr::Seq(vec![Expr::TupleProj(b(var("p")), 1)]))),
            hint(),
            b(seq_u8(&[1, 2, 3])),
        ),
    ));
    cases.push(parity(
        "flat_map_list_range",
        Expr::FlatMapList(
            b(lam("p", Expr::Seq(vec![Expr::TupleProj(b(var("p")), 1)]))),
            hint(),
            b(range(0, 3)),
        ),
    ));

    // -- formerly-divergent cases: both evaluators now share `Expr::eval_generic` (Stage 3 of the
    // eval-unification project), so these are unconditional parity cases rather than a live
    // divergence list. Kept as named regression cases for the specific behaviors each one pins
    // down (see the eval-unification plan memory for the resolution each one records).
    cases.push(parity(
        "as_char_usize",
        Expr::AsChar(b(Expr::SeqIx(b(range(2, 7)), b(Expr::U32(0))))),
    ));
    cases.push(parity(
        "flat_map_accum_range",
        Expr::FlatMapAccum(
            b(accum_pass_through()),
            b(Expr::U8(0)),
            hint(),
            b(range(0, 3)),
        ),
    ));
    cases.push(parity(
        "left_fold_range",
        Expr::LeftFold(b(fold_last()), b(Expr::U8(0)), hint(), b(range(0, 3))),
    ));
    cases.push(parity_with(
        "match_permit_err_some",
        Expr::Match(
            b(var("p")),
            vec![
                (Pattern::U8(1), Expr::U8(10)),
                (Pattern::Wildcard, Expr::U8(20)),
            ],
        ),
        vec![("p", Value::Permit(Err(Some(Box::new(Value::U8(1))))))],
    ));
    cases.push(parity(
        "flat_map_returns_range",
        Expr::FlatMap(b(lam("x", range(0, 2))), b(seq_u8(&[1, 2]))),
    ));

    cases
}

#[test]
fn eval_and_eval_with_loc_agree() {
    let mut failures = Vec::new();
    for Case {
        name,
        expr,
        vars,
        expect,
    } in cases()
    {
        let main = eval_main(&expr, &vars);
        let loc = eval_loc(&expr, &vars);
        match expect {
            Expect::Parity if main != loc => failures.push(format!(
                "{name}: DIVERGED\n    main: {main:?}\n    loc:  {loc:?}"
            )),
            Expect::Diverges(why) if main == loc => failures.push(format!(
                "{name}: now agrees ({main:?}); drift fixed? switch to `Parity` (was: {why})"
            )),
            _ => {}
        }
    }
    assert!(
        failures.is_empty(),
        "{} case(s) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
