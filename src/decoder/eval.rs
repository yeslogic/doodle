//! The generic evaluator shared by `Expr::eval` (`decoder.rs`, `V = Value`) and
//! `Expr::eval_with_loc` (`loc_decoder.rs`, `V = ParsedValue`).
//!
//! Both evaluators used to be hand-maintained mirrors of one another, differing only in the leaf
//! value representation. [`EvalValue`] captures the small set of operations `Expr::eval_generic`
//! needs from that representation; `impl EvalValue for Value` (`decoder/value.rs`) and
//! `impl EvalValue for ParsedValue` (`loc_decoder.rs`) provide it, and `eval`/`eval_with_loc` are
//! now thin wrappers around [`Expr::eval_generic`].
//!
//! See `decoder/eval_parity_tests.rs` for the differential test that backstops this.

use std::borrow::Cow;

use crate::error::EvalError;
use crate::numeric::core::{CoerceValueError, StrictValue};
use crate::scope::{GMultiScope, GScope, GSingleScope};
use crate::{Expr, Label, Pattern};

use super::seq_kind::{SeqBoundsOp, ValueSeq, sub_range};
use super::value::{Coerced, Value};
use super::{SeqKind, cow_map, cow_remap, extract_pair, search, sub_seq_inflate};

/// The operations `Expr::eval_generic` needs from a leaf value representation. Implemented by
/// [`Value`] (identity/no-op wrapping) and by [`crate::loc_decoder::ParsedValue`] (which also
/// threads parse-location info through).
///
/// NOTE - `Expr::Numeric` also needs `StrictValue: TryFrom<&Self>` (via `NumExpr::eval`'s own
/// `EvalScope`-based bound). It is *not* a supertrait bound here: moving it onto the trait itself
/// broke inference at call sites (the HRTB `for<'x>` combined with the `Error = CoerceValueError`
/// associated-type-equality bound doesn't elaborate cleanly through a supertrait in current
/// rustc), so it's repeated as a `where` clause on every function below that needs it instead.
pub(crate) trait EvalValue: Sized + Clone + std::fmt::Debug + From<usize> + 'static {
    /// Wraps a fully-evaluated, plain [`Value`] as `Self`.
    fn from_evaluated(v: Value) -> Self;

    /// Wraps a `Vec<Self>` (e.g. produced by `SubSeq`/`SubSeqInflate`/`Append`) as a `Self`-typed
    /// sequence.
    fn from_evaluated_seq(vs: Vec<Self>) -> Self;

    /// Converts (cloning as needed) to a plain [`Value`], discarding any location info.
    fn clone_into_value(&self) -> Value;

    /// Looks through `Mapped`/`Branch`/`Permit(Ok)`/`Permit(Err(Some))` wrappers, tracking
    /// whether a `Permit`-fallback value was found along the way (see [`Coerced`]).
    ///
    /// `ParsedValue`'s coercion never actually unwraps a `Permit(Err(Some))` (unlike `Value`'s),
    /// so its `Coerced` is always `is_fallback() == false` — that's an existing, separate
    /// asymmetry between the two, not something this trait method papers over.
    fn coerce_mapped_value(&self) -> Coerced<&Self>;

    /// Like [`Self::coerce_mapped_value`], but consumes `self` and converts straight to a plain
    /// [`Value`], letting each implementation avoid a redundant clone when it already owns the
    /// value (see `Value`'s impl, which is a no-op unwrap with no clone at all).
    fn extract_mapped_value(self) -> Value;

    /// Panics if `self` (after coercion) is not a tuple of at least `index + 1` elements.
    fn tuple_proj_raw(&self, index: usize) -> &Self;

    /// Panics if `self` (after coercion) is not a record with a `label` field.
    fn record_proj_raw(&self, label: &str) -> &Self;

    fn get_sequence(&self) -> Option<ValueSeq<'_, Self>>;

    fn collect_fields(fields: Vec<(Label, Self)>) -> Self;

    /// Wraps `opt` as an Option-typed `Self`, preserving `opt`'s own structure/location when
    /// `Some` rather than downgrading it to a plain `Value` first (used by `FindByKey`, so the
    /// matched element keeps its real parse-location instead of a synthesized one).
    fn lift_option(opt: Option<Self>) -> Self;

    /// Attempts to match `self` against `pattern`, coercing through `Mapped`/`Branch`/
    /// `Permit`-wrappers transparently first (see `Value::coerce_nominal_value`), returning the
    /// bindings introduced by the match on success.
    fn matches<'a>(
        &'a self,
        scope: &'a GScope<'a, Self>,
        pattern: &Pattern,
    ) -> Option<GMultiScope<'a, Self>>;
}

impl Expr {
    pub(crate) fn eval_generic<'a, V: EvalValue>(
        &'a self,
        scope: &'a GScope<'a, V>,
    ) -> Result<Cow<'a, V>, EvalError>
    where
        StrictValue: for<'x> TryFrom<&'x V, Error = CoerceValueError>,
    {
        Ok(match self {
            Expr::Var(name) => Cow::Borrowed(scope.get_value_by_name(name).unwrap()),
            Expr::Bool(b) => Cow::Owned(V::from_evaluated(Value::Bool(*b))),
            Expr::U8(i) => Cow::Owned(V::from_evaluated(Value::U8(*i))),
            Expr::U16(i) => Cow::Owned(V::from_evaluated(Value::U16(*i))),
            Expr::U32(i) => Cow::Owned(V::from_evaluated(Value::U32(*i))),
            Expr::U64(i) => Cow::Owned(V::from_evaluated(Value::U64(*i))),
            Expr::Numeric(n) => Cow::Owned(V::from_evaluated(Value::from(n.eval(scope)?))),
            Expr::Tuple(exprs) => Cow::Owned(V::from_evaluated(Value::Tuple(
                exprs
                    .iter()
                    .map(|expr| expr.eval_value_generic(scope))
                    .collect::<Result<_, EvalError>>()?,
            ))),
            Expr::TupleProj(head, index) => cow_map(head.eval_generic(scope)?, |v| {
                v.coerce_mapped_value()
                    .map(|v| v.tuple_proj_raw(*index))
                    .into_inner()
            }),
            Expr::Record(fields) => {
                let fields = fields
                    .iter()
                    .map(|(label, expr)| {
                        Ok((label.clone(), expr.eval_generic(scope)?.into_owned()))
                    })
                    .collect::<Result<Vec<_>, EvalError>>()?;
                Cow::Owned(V::collect_fields(fields))
            }
            Expr::RecordProj(head, label) => cow_map(head.eval_generic(scope)?, |v| {
                v.coerce_mapped_value()
                    .map(|v| v.record_proj_raw(label.as_ref()))
                    .into_inner()
            }),
            Expr::Variant(label, expr) => Cow::Owned(V::from_evaluated(Value::variant(
                label.clone(),
                expr.eval_value_generic(scope)?,
            ))),
            Expr::Seq(exprs) => Cow::Owned(V::from_evaluated(Value::Seq(
                exprs
                    .iter()
                    .map(|expr| expr.eval_value_generic(scope))
                    .collect::<Result<_, EvalError>>()?,
            ))),
            Expr::Match(head, branches) => {
                let head = head.eval_generic(scope)?;
                for (pattern, expr) in branches {
                    if let Some(pattern_scope) = head.matches(scope, pattern) {
                        let value = expr.eval_value_generic(&GScope::Multi(&pattern_scope))?;
                        return Ok(Cow::Owned(V::from_evaluated(value)));
                    }
                }
                return Err(EvalError::RefutedPattern {
                    cases: branches.iter().map(|(p, _)| p.clone()).collect(),
                    value: Box::new(head.clone_into_value()),
                });
            }
            Expr::Destructure(head, pattern, expr) => {
                let head = head.eval_generic(scope)?;
                if let Some(pattern_scope) = head.matches(scope, pattern) {
                    let value = expr.eval_value_generic(&GScope::Multi(&pattern_scope))?;
                    return Ok(Cow::Owned(V::from_evaluated(value)));
                } else {
                    return Err(EvalError::RefutedPattern {
                        cases: vec![pattern.clone()],
                        value: Box::new(head.clone_into_value()),
                    });
                }
            }
            Expr::Lambda(_, _) => panic!("cannot eval lambda"),

            Expr::IntRel(rel, x, y) => Cow::Owned(V::from_evaluated({
                let left = x.eval_value_generic(scope)?;
                let right = y.eval_value_generic(scope)?;
                Value::int_rel(*rel, left, right)?
            })),
            Expr::Arith(op, x, y) => Cow::Owned(V::from_evaluated({
                let left = x.eval_value_generic(scope)?;
                let right = y.eval_value_generic(scope)?;
                Value::arith(*op, left, right)?
            })),
            Expr::Unary(op, x) => Cow::Owned(V::from_evaluated({
                let value = x.eval_value_generic(scope)?;
                Value::unary(*op, value)?
            })),

            Expr::AsU8(x) => Cow::Owned(V::from_evaluated(
                x.eval_value_generic(scope)?.cast_to_u8()?,
            )),
            Expr::AsU16(x) => Cow::Owned(V::from_evaluated(
                x.eval_value_generic(scope)?.cast_to_u16()?,
            )),
            Expr::AsU32(x) => Cow::Owned(V::from_evaluated(
                x.eval_value_generic(scope)?.cast_to_u32()?,
            )),
            Expr::AsU64(x) => Cow::Owned(V::from_evaluated(
                x.eval_value_generic(scope)?.cast_to_u64()?,
            )),

            Expr::U16Be(bytes) => Cow::Owned(V::from_evaluated(Value::U16(u16::from_be_bytes(
                bytes
                    .eval_value_generic(scope)?
                    .unwrap_byte_array::<2>("U16Be"),
            )))),
            Expr::U16Le(bytes) => Cow::Owned(V::from_evaluated(Value::U16(u16::from_le_bytes(
                bytes
                    .eval_value_generic(scope)?
                    .unwrap_byte_array::<2>("U16Le"),
            )))),
            Expr::U32Be(bytes) => Cow::Owned(V::from_evaluated(Value::U32(u32::from_be_bytes(
                bytes
                    .eval_value_generic(scope)?
                    .unwrap_byte_array::<4>("U32Be"),
            )))),
            Expr::U32Le(bytes) => Cow::Owned(V::from_evaluated(Value::U32(u32::from_le_bytes(
                bytes
                    .eval_value_generic(scope)?
                    .unwrap_byte_array::<4>("U32Le"),
            )))),
            Expr::U64Be(bytes) => Cow::Owned(V::from_evaluated(Value::U64(u64::from_be_bytes(
                bytes
                    .eval_value_generic(scope)?
                    .unwrap_byte_array::<8>("U64Be"),
            )))),
            Expr::U64Le(bytes) => Cow::Owned(V::from_evaluated(Value::U64(u64::from_le_bytes(
                bytes
                    .eval_value_generic(scope)?
                    .unwrap_byte_array::<8>("U64Le"),
            )))),
            Expr::AsChar(x) => Cow::Owned(V::from_evaluated(
                x.eval_value_generic(scope)?.cast_to_char()?,
            )),
            Expr::SeqLength(seq) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let len = values.len();
                        Cow::Owned(V::from_evaluated(Value::U32(len as u32)))
                    }
                    _ => panic!("SeqLength: expected Seq"),
                }
            }
            Expr::SeqIx(seq, index) => {
                let index = index.eval_value_generic(scope)?.try_as_usize()?;
                let head = seq.eval_generic(scope)?;
                match head.coerce_mapped_value().get_sequence() {
                    Some(values) => values.check_index(SeqBoundsOp::SeqIx, index)?,
                    _ => panic!("SeqIx: expected Seq (or EnumFromTo)"),
                }
                cow_remap(head, |v| match v.coerce_mapped_value().get_sequence() {
                    Some(values) => match values {
                        ValueSeq::ValueSeq(values) => Cow::Borrowed(&values[index]),
                        ValueSeq::IntRange(mut range) => {
                            Cow::Owned(V::from(range.nth(index).unwrap()))
                        }
                    },
                    _ => unreachable!("bounds already checked above"),
                })
            }
            Expr::SubSeq(seq, start, length) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let start = start.eval_value_generic(scope)?.try_as_usize()?;
                        let length = length.eval_value_generic(scope)?.try_as_usize()?;
                        match values {
                            ValueSeq::ValueSeq(values) => Cow::Owned(V::from_evaluated_seq(
                                values
                                    .sub_seq(SeqBoundsOp::SubSeq, start, length)?
                                    .into_vec(),
                            )),
                            ValueSeq::IntRange(range) => {
                                Cow::Owned(V::from_evaluated(Value::EnumFromTo(sub_range(
                                    SeqBoundsOp::SubSeq,
                                    range,
                                    start,
                                    length,
                                )?)))
                            }
                        }
                    }
                    _ => panic!("SubSeq: expected Seq"),
                }
            }
            Expr::SubSeqInflate(seq, start, length) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let start = start.eval_value_generic(scope)?.try_as_usize()?;
                        let length = length.eval_value_generic(scope)?.try_as_usize()?;
                        Cow::Owned(V::from_evaluated_seq(sub_seq_inflate(
                            values, start, length,
                        )?))
                    }
                    _ => panic!("SubSeqInflate: expected Seq"),
                }
            }
            Expr::Append(seq0, seq1) => {
                let tmp0 = seq0.eval_generic(scope)?;
                let val0 = tmp0.coerce_mapped_value();

                let Some(val_seq0) = val0.get_sequence() else {
                    unreachable!("Append: expected Seq in (lhs)")
                };

                let tmp1 = seq1.eval_generic(scope)?;
                let val1 = tmp1.coerce_mapped_value();

                let Some(val_seq1) = val1.get_sequence() else {
                    unreachable!("Append: expected Seq in (rhs)")
                };

                if val_seq0.is_empty() {
                    return Ok(Cow::Owned(val1.into_inner().clone()));
                } else if val_seq1.is_empty() {
                    return Ok(Cow::Owned(val0.into_inner().clone()));
                }
                Cow::Owned(V::from_evaluated_seq(val_seq0.append(val_seq1).into_vec()))
            }
            Expr::FlatMap(expr, seq) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let mut vs = Vec::new();
                        for v in values {
                            match expr.eval_lambda_generic(scope, &v)? {
                                Value::Seq(vn) => {
                                    vs.extend(vn.into_vec());
                                }
                                Value::EnumFromTo(range) => {
                                    vs.extend(range.map(Value::from));
                                }
                                _ => {
                                    panic!("FlatMap: expected Seq (or EnumFromTo)");
                                }
                            }
                        }
                        Cow::Owned(V::from_evaluated(Value::Seq(SeqKind::Strict(vs))))
                    }
                    _ => panic!("FlatMap: expected Seq"),
                }
            }
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let mut accum = accum.eval_value_generic(scope)?;
                        let mut vs = Vec::new();
                        for v in values {
                            let ret = expr.eval_lambda_generic(
                                scope,
                                &V::from_evaluated(Value::Tuple(vec![accum, v.clone_into_value()])),
                            )?;
                            accum = match extract_pair(ret.unwrap_tuple()) {
                                (accum, Value::Seq(vn)) => {
                                    vs.extend(vn.into_vec());
                                    accum
                                }
                                other => panic!("FlatMapAccum: bad lambda output type {other:?}"),
                            };
                        }
                        Cow::Owned(V::from_evaluated(Value::Seq(SeqKind::Strict(vs))))
                    }
                    _ => panic!("FlatMapAccum: expected Seq"),
                }
            }
            Expr::LeftFold(expr, accum, _accum_type, seq) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => {
                        let mut accum = accum.eval_value_generic(scope)?;
                        for v in values {
                            accum = expr.eval_lambda_generic(
                                scope,
                                &V::from_evaluated(Value::Tuple(vec![accum, v.clone_into_value()])),
                            )?;
                        }
                        Cow::Owned(V::from_evaluated(accum))
                    }
                    _ => panic!("LeftFold: expected Seq"),
                }
            }
            Expr::FindByKey(is_sorted, f_get_key, query_key, seq) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(ValueSeq::ValueSeq(values)) => {
                        let query = query_key.eval_value_generic(scope)?;
                        // `search` is shared with `decoder`/`loc_decoder` and expects an infallible evaluator, so
                        // the first error is stashed here (the sentinel `query` key makes the search terminate
                        // immediately)
                        let eval_err = std::cell::Cell::new(None);
                        let eval =
                            |lambda: &Expr, arg: &V| match lambda.eval_lambda_generic(scope, arg) {
                                Ok(key) => key,
                                Err(err) => {
                                    eval_err.set(Some(err));
                                    query.clone()
                                }
                            };
                        let found = if *is_sorted {
                            search::find_index_by_key_sorted(f_get_key, &query, values, eval)
                        } else {
                            search::find_index_by_key_unsorted(f_get_key, &query, values, eval)
                        };
                        if let Some(err) = eval_err.take() {
                            return Err(err);
                        }
                        // Preserves the matched element's own location/structure (via
                        // `lift_option`) rather than discarding it by routing through a plain
                        // `Value::Option`.
                        Cow::Owned(V::lift_option(found.map(|ix| values[ix].clone())))
                    }
                    Some(ValueSeq::IntRange(_)) => {
                        unimplemented!("FindByKey: unimplemented on IntRange")
                    }
                    _ => panic!("FindByKey: expected Seq"),
                }
            }
            Expr::FlatMapList(expr, _ret_type, seq) => match seq.eval_value_generic(scope)? {
                Value::Seq(values) => {
                    let mut vs = Vec::new();
                    for v in values {
                        // Builds `arg` by moving (not cloning) `vs`/`v` into it, then reclaims
                        // `vs` afterward via `extract_mapped_value` (also a move, not a clone) -
                        // `eval_lambda_generic` only *borrows* `arg`, so it's still intact and
                        // owned once the call returns. Using `eval_lambda_value_generic` here
                        // instead would clone the *entire* accumulator (`vs`) on every iteration
                        // (it was embedded inside `arg`), turning this from O(n) into O(n²).
                        let arg = V::from_evaluated(Value::Tuple(vec![
                            Value::Seq(SeqKind::Strict(vs)),
                            v,
                        ]));
                        if let Value::Seq(vn) = expr.eval_lambda_generic(scope, &arg)? {
                            vs = match arg.extract_mapped_value() {
                                Value::Tuple(mut args) => match args.remove(0) {
                                    Value::Seq(vs) => vs.into_vec(),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            vs.extend(vn.into_vec());
                        } else {
                            panic!("FlatMapList: expected Seq");
                        }
                    }
                    Cow::Owned(V::from_evaluated(Value::Seq(SeqKind::Strict(vs))))
                }
                _ => panic!("FlatMapList: expected Seq"),
            },
            Expr::Dup(count, expr) => {
                let count = count.eval_value_generic(scope)?.try_as_usize()?;
                let v = expr.eval_value_generic(scope)?;
                Cow::Owned(V::from_evaluated(Value::Seq(SeqKind::Dup(
                    count,
                    Box::new(v),
                ))))
            }
            Expr::EnumFromTo(start, stop) => {
                let start = start.eval_value_generic(scope)?.try_as_usize()?;
                let stop = stop.eval_value_generic(scope)?.try_as_usize()?;
                Cow::Owned(V::from_evaluated(Value::EnumFromTo(start..stop)))
            }
            Expr::LiftOption(opt) => Cow::Owned(V::from_evaluated(Value::Option(
                opt.as_ref()
                    .map(|expr| expr.eval_value_generic(scope).map(Box::new))
                    .transpose()?,
            ))),
        })
    }

    pub(crate) fn eval_value_generic<'a, V: EvalValue>(
        &'a self,
        scope: &'a GScope<'a, V>,
    ) -> Result<Value, EvalError>
    where
        StrictValue: for<'x> TryFrom<&'x V, Error = CoerceValueError>,
    {
        Ok(match self.eval_generic(scope)? {
            // Already owned: let the impl avoid a redundant clone (see `extract_mapped_value`'s
            // doc comment — for `Value` this is a no-op unwrap, no clone at all).
            Cow::Owned(v) => v.extract_mapped_value(),
            // Only borrowed: coercing still only walks references, but converting to a plain
            // `Value` necessarily clones at the leaf either way.
            Cow::Borrowed(v) => v.coerce_mapped_value().into_inner().clone_into_value(),
        })
    }

    pub(crate) fn eval_lambda_generic<'a, V: EvalValue>(
        &self,
        scope: &'a GScope<'a, V>,
        arg: &'a V,
    ) -> Result<Value, EvalError>
    where
        StrictValue: for<'x> TryFrom<&'x V, Error = CoerceValueError>,
    {
        match self {
            Expr::Lambda(name, expr) => {
                let child_scope = GSingleScope::new(scope, name, arg);
                expr.eval_value_generic(&GScope::Single(child_scope))
            }
            _ => panic!("expected Lambda"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::{Scope, SeqBoundsError};

    fn eval_ok(expr: &Expr) -> Value {
        expr.eval_value_generic(&Scope::Empty).unwrap()
    }

    fn eval_err(expr: &Expr) -> EvalError {
        expr.eval_value_generic(&Scope::Empty).unwrap_err()
    }

    fn var(name: &'static str) -> Expr {
        Expr::Var(Label::from(name))
    }

    fn lam(name: &'static str, body: Expr) -> Expr {
        Expr::Lambda(Label::from(name), Box::new(body))
    }

    fn b(e: Expr) -> Box<Expr> {
        Box::new(e)
    }

    fn seq_u8(items: &[u8]) -> Expr {
        Expr::Seq(items.iter().copied().map(Expr::U8).collect())
    }

    fn range(start: u32, stop: u32) -> Expr {
        Expr::EnumFromTo(b(Expr::U32(start)), b(Expr::U32(stop)))
    }

    /// Direct (non-differential) correctness checks for Group 5's sequence-bounds fixes: these
    /// assert the actual `Ok`/`Err` outcome, not just that the two evaluators agree with each
    /// other (see `eval_parity_tests.rs` for that).
    #[test]
    fn seq_ix_out_of_bounds_errors() {
        let expr = Expr::SeqIx(b(seq_u8(&[1, 2, 3])), b(Expr::U32(3)));
        assert!(matches!(
            eval_err(&expr),
            EvalError::SeqBounds(SeqBoundsError {
                op: SeqBoundsOp::SeqIx,
                index: 3,
                len: 3,
            })
        ));
    }

    #[test]
    fn sub_seq_full_range_of_enum_from_to_succeeds() {
        // Regression test for an off-by-one in `sub_range`'s bounds check that used to reject
        // `start + length == range.len()`, the valid full-range case.
        let expr = Expr::SubSeq(b(range(0, 3)), b(Expr::U32(0)), b(Expr::U32(3)));
        assert_eq!(eval_ok(&expr), Value::EnumFromTo(0..3));
    }

    #[test]
    fn sub_seq_inflate_start_out_of_bounds_errors() {
        // start=5 is past the end of a 3-element source with nothing yet accumulated to
        // self-reference: this used to index an empty Vec and panic.
        let expr = Expr::SubSeqInflate(b(seq_u8(&[1, 2, 3])), b(Expr::U32(5)), b(Expr::U32(2)));
        assert!(matches!(
            eval_err(&expr),
            EvalError::SeqBounds(SeqBoundsError {
                op: SeqBoundsOp::SubSeqInflate,
                index: 5,
                len: 3,
            })
        ));
    }

    #[test]
    fn sub_seq_inflate_start_out_of_bounds_with_zero_length_succeeds() {
        // start is out of bounds, but length=0 means it's never dereferenced.
        let expr = Expr::SubSeqInflate(b(seq_u8(&[1, 2, 3])), b(Expr::U32(5)), b(Expr::U32(0)));
        assert_eq!(eval_ok(&expr), Value::Seq(SeqKind::Strict(vec![])));
    }

    /// Regression test for Group 6 (refuted-pattern panic -> `EvalError` conversion): an
    /// `Expr::Match` whose scrutinee matches none of the branch patterns.
    #[test]
    fn match_non_exhaustive_errors() {
        let expr = Expr::Match(b(Expr::U8(5)), vec![(Pattern::U8(1), Expr::U8(10))]);
        assert!(matches!(
            eval_err(&expr),
            EvalError::RefutedPattern { cases, value }
                if cases == vec![Pattern::U8(1)] && *value == Value::U8(5)
        ));
    }

    /// As above, but for `Expr::Destructure`'s single-pattern refutation.
    #[test]
    fn destructure_refuted_errors() {
        let expr = Expr::Destructure(b(Expr::U8(1)), Pattern::U8(2), b(Expr::U8(0)));
        assert!(matches!(
            eval_err(&expr),
            EvalError::RefutedPattern { cases, value }
                if cases == vec![Pattern::U8(2)] && *value == Value::U8(1)
        ));
    }

    #[test]
    fn sub_seq_inflate_wraps_correctly() {
        let expr = Expr::SubSeqInflate(b(seq_u8(&[1, 2, 3])), b(Expr::U32(1)), b(Expr::U32(6)));
        assert_eq!(
            eval_ok(&expr),
            Value::Seq(SeqKind::Strict(vec![
                Value::U8(2),
                Value::U8(3),
                Value::U8(2),
                Value::U8(3),
                Value::U8(2),
                Value::U8(3),
            ]))
        );
    }

    /// Regression test for the `FindByKey` rework (items 5/9): the matched element's *real*
    /// parse-location should survive, not get flattened to `Value` and re-synthesized.
    #[test]
    fn find_by_key_preserves_loc_for_parsed_value() {
        use crate::loc_decoder::{LocMultiScope, LocScope, ParseLoc, Parsed, ParsedValue};

        let elems = vec![
            ParsedValue::new_flat(Value::U8(1), 0, 1),
            ParsedValue::new_flat(Value::U8(2), 1, 1),
            ParsedValue::new_flat(Value::U8(3), 2, 1),
        ];
        let seq_value = ParsedValue::Seq(Parsed {
            loc: ParseLoc::Synthesized,
            inner: SeqKind::Strict(elems),
        });

        let root = LocScope::Empty;
        let mut multi = LocMultiScope::with_capacity(&root, 1);
        multi.push("seq", &seq_value);
        let scope = LocScope::Multi(&multi);

        let expr = Expr::FindByKey(true, b(lam("x", var("x"))), b(Expr::U8(2)), b(var("seq")));
        let result = expr.eval_generic(&scope).unwrap().into_owned();
        let ParsedValue::Option(Some(matched)) = result else {
            panic!("expected Option(Some(_)), got {result:?}");
        };
        assert_eq!(
            matched.get_loc(),
            ParseLoc::InBuffer {
                offset: 1,
                length: 1
            }
        );
    }
}
