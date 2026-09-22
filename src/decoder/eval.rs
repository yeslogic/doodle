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

use super::seq_kind::{ValueSeq, sub_range};
use super::value::Value;
use super::{SeqKind, cow_map, cow_remap, extract_pair, search, sub_seq_inflate};

/// The operations `Expr::eval_generic` needs from a leaf value representation. Implemented by
/// [`Value`] (identity/no-op wrapping) and by [`crate::loc_decoder::ParsedValue`] (which also
/// threads parse-location info through).
pub(crate) trait EvalValue: Sized + Clone + std::fmt::Debug + From<usize> + 'static {
    /// Wraps a fully-evaluated, plain [`Value`] as `Self`.
    fn from_evaluated(v: Value) -> Self;

    /// Wraps a `Vec<Self>` (e.g. produced by `SubSeq`/`SubSeqInflate`/`Append`) as a `Self`-typed
    /// sequence.
    fn from_evaluated_seq(vs: Vec<Self>) -> Self;

    /// Converts (cloning as needed) to a plain [`Value`], discarding any location info.
    fn clone_into_value(&self) -> Value;

    /// Looks through `Mapped`/`Branch`/`Permit(Ok)`/`Permit(Err(Some))` wrappers.
    fn coerce_mapped_value(&self) -> &Self;

    /// Panics if `self` (after coercion) is not a tuple of at least `index + 1` elements.
    fn tuple_proj_raw(&self, index: usize) -> &Self;

    /// Panics if `self` (after coercion) is not a record with a `label` field.
    fn record_proj_raw(&self, label: &str) -> &Self;

    fn get_sequence(&self) -> Option<ValueSeq<'_, Self>>;

    fn collect_fields(fields: Vec<(Label, Self)>) -> Self;

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
                v.coerce_mapped_value().tuple_proj_raw(*index)
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
                v.coerce_mapped_value().record_proj_raw(label.as_ref())
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
                panic!("non-exhaustive patterns");
            }
            Expr::Destructure(head, pattern, expr) => {
                let head = head.eval_generic(scope)?;
                if let Some(pattern_scope) = head.matches(scope, pattern) {
                    let value = expr.eval_value_generic(&GScope::Multi(&pattern_scope))?;
                    return Ok(Cow::Owned(V::from_evaluated(value)));
                } else {
                    panic!("refuted pattern: {head:?} does not match {pattern:?}");
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
                cow_remap(seq.eval_generic(scope)?, |v| {
                    match v.coerce_mapped_value().get_sequence() {
                        Some(values) => match values {
                            ValueSeq::ValueSeq(values) => Cow::Borrowed(&values[index]),
                            ValueSeq::IntRange(mut range) => {
                                Cow::Owned(V::from(range.nth(index).unwrap()))
                            }
                        },
                        _ => panic!("SeqIx: expected Seq (or EnumFromTo)"),
                    }
                })
            }
            Expr::SubSeq(seq, start, length) => {
                match seq
                    .eval_generic(scope)?
                    .coerce_mapped_value()
                    .get_sequence()
                {
                    Some(values) => match values {
                        ValueSeq::ValueSeq(values) => {
                            let start = start.eval_value_generic(scope)?.try_as_usize()?;
                            let length = length.eval_value_generic(scope)?.try_as_usize()?;
                            Cow::Owned(V::from_evaluated_seq(
                                values.sub_seq(start, length).into_vec(),
                            ))
                        }
                        ValueSeq::IntRange(range) => {
                            let start = start.eval_value_generic(scope)?.try_as_usize()?;
                            let length = length.eval_value_generic(scope)?.try_as_usize()?;
                            Cow::Owned(V::from_evaluated(Value::EnumFromTo(sub_range(
                                range, start, length,
                            ))))
                        }
                    },
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
                        )))
                    }
                    _ => panic!("SubSeqInflate: expected Seq"),
                }
            }
            Expr::Append(seq0, seq1) => {
                let tmp0 = seq0.eval_generic(scope)?;
                let val0 = tmp0.coerce_mapped_value();
                match val0.get_sequence() {
                    Some(val_seq0) => {
                        let tmp1 = seq1.eval_generic(scope)?;
                        let val1 = tmp1.coerce_mapped_value();
                        match val1.get_sequence() {
                            Some(val_seq1) => {
                                if val_seq0.is_empty() {
                                    return Ok(Cow::Owned(val1.clone()));
                                } else if val_seq1.is_empty() {
                                    return Ok(Cow::Owned(val0.clone()));
                                }
                                Cow::Owned(V::from_evaluated_seq(
                                    val_seq0.append(val_seq1).into_vec(),
                                ))
                            }
                            _ => unreachable!("Append: expected Seq in (rhs)"),
                        }
                    }
                    _ => unreachable!("Append: expected Seq in (lhs)"),
                }
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
                match seq.eval_value_generic(scope)? {
                    Value::Seq(values) => {
                        let query = query_key.eval_value_generic(scope)?;
                        // `search` is shared with `decoder`/`loc_decoder` and expects an infallible evaluator, so
                        // the first error is stashed here (the sentinel `query` key makes the search terminate
                        // immediately)
                        let eval_err = std::cell::Cell::new(None);
                        let eval = |lambda: &Expr, arg: &Value| match lambda
                            .eval_lambda_value_generic(scope, arg)
                        {
                            Ok(key) => key,
                            Err(err) => {
                                eval_err.set(Some(err));
                                query.clone()
                            }
                        };
                        let found = if *is_sorted {
                            search::find_index_by_key_sorted(f_get_key, &query, &values, eval)
                        } else {
                            search::find_index_by_key_unsorted(f_get_key, &query, &values, eval)
                        };
                        if let Some(err) = eval_err.take() {
                            return Err(err);
                        }
                        match found {
                            Some(ix) => Cow::Owned(V::from_evaluated(Value::Option(Some(
                                Box::new(values[ix].clone()),
                            )))),
                            None => Cow::Owned(V::from_evaluated(Value::Option(None))),
                        }
                    }
                    _ => panic!("FindByKey: expected Seq"),
                }
            }
            Expr::FlatMapList(expr, _ret_type, seq) => match seq.eval_value_generic(scope)? {
                Value::Seq(values) => {
                    let mut vs = Vec::new();
                    for v in values {
                        let arg = Value::Tuple(vec![Value::Seq(SeqKind::Strict(vs)), v]);
                        if let Value::Seq(vn) = expr.eval_lambda_value_generic(scope, &arg)? {
                            vs = match arg {
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
        Ok(self
            .eval_generic(scope)?
            .coerce_mapped_value()
            .clone_into_value())
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

    /// Like [`Self::eval_lambda_generic`], but for an argument that is already a plain [`Value`]
    /// (used by `FindByKey`, which downgrades its sequence to `Value` up front).
    fn eval_lambda_value_generic<'a, V: EvalValue>(
        &self,
        scope: &'a GScope<'a, V>,
        arg: &Value,
    ) -> Result<Value, EvalError>
    where
        StrictValue: for<'x> TryFrom<&'x V, Error = CoerceValueError>,
    {
        self.eval_lambda_generic(scope, &V::from_evaluated(arg.clone()))
    }
}
