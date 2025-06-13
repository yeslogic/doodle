use crate::byte_set::ByteSet;
use crate::error::{DecodeError, DecodeResult};
use crate::read::ReadCtxt;
use crate::{
    pattern::Pattern, Arith, DynFormat, Expr, Format, FormatModule, IntRel, MatchTree, Next,
    TypeScope, ValueType,
};
use crate::{IntoLabel, Label, MaybeTyped, TypeHint, UnaryOp};
use anyhow::{anyhow, Result as AResult};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

pub mod seq_kind;
use seq_kind::sub_range;
pub use seq_kind::{SeqKind, ValueSeq};

pub(crate) fn extract_pair<T>(mut vec: Vec<T>) -> (T, T) {
    if vec.len() != 2 {
        panic!("expected pair");
    }
    unsafe {
        // Safe because we checked the length above
        let second = vec.pop().unwrap_unchecked();
        let first = vec.pop().unwrap_unchecked();
        (first, second)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Char(char),
    Usize(usize),
    EnumFromTo(std::ops::Range<usize>),
    Option(Option<Box<Value>>),
    Tuple(Vec<Value>),
    Record(Vec<(Label, Value)>),
    Variant(Label, Box<Value>),
    Seq(SeqKind<Value>),
    Mapped(Box<Value>, Box<Value>),
    Branch(usize, Box<Value>),
}

impl From<usize> for Value {
    fn from(value: usize) -> Value {
        Value::Usize(value)
    }
}

const MAX_SEQ_LEN: usize = 64;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::U8(i) => write!(f, "{}", i),
            Value::U16(i) => write!(f, "{}", i),
            Value::U32(i) => write!(f, "{}", i),
            Value::U64(i) => write!(f, "{}", i),
            Value::Char(c) => write!(f, "{:?}", c),
            Value::Usize(i) => write!(f, "{}", i),
            Value::EnumFromTo(r) => write!(f, "{:?}", r),
            Value::Option(v) => match v {
                None => write!(f, "None"),
                Some(v) => write!(f, "Some({})", v),
            },
            Value::Tuple(vs) => {
                write!(
                    f,
                    "({})",
                    vs.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Record(fields) => {
                write!(
                    f,
                    "{{ {} }}",
                    fields
                        .iter()
                        .map(|(f, v)| format!("{}: {}", f, v))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Variant(label, value) => {
                write!(f, "`{}({})", label, value)
            }
            Value::Seq(s_kind) => match s_kind {
                SeqKind::Dup(n, v) => write!(f, "[{v}; {n}]"),
                SeqKind::Strict(vs) => {
                    if vs.len() > MAX_SEQ_LEN {
                        write!(f, "[...; {}]", vs.len())
                    } else {
                        write!(
                            f,
                            "[{}]",
                            vs.iter()
                                .map(|v| v.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                }
            },
            Value::Mapped(orig, image) => {
                write!(f, "({} => {})", orig, image)
            }
            Value::Branch(n, value) => write!(f, "({n} :~ {})", value),
        }
    }
}

impl Value {
    fn tuple_proj(&self, index: usize) -> &Self {
        match self.coerce_mapped_value() {
            Value::Tuple(vs) => &vs[index],
            _ => panic!("expected tuple"),
        }
    }

    fn matches_inner<'a>(&'a self, scope: &mut MultiScope<'a>, pattern: &Pattern) -> bool {
        match (pattern, self) {
            (Pattern::Binding(name), head) => {
                scope.push(name.clone(), head);
                true
            }
            (Pattern::Wildcard, _) => true,
            (Pattern::Bool(b0), Value::Bool(b1)) => b0 == b1,
            (Pattern::U8(i0), Value::U8(i1)) => i0 == i1,
            (Pattern::U16(i0), Value::U16(i1)) => i0 == i1,
            (Pattern::U32(i0), Value::U32(i1)) => i0 == i1,
            (Pattern::U64(i0), Value::U64(i1)) => i0 == i1,
            (Pattern::Int(bounds), Value::U8(n)) => bounds.contains(usize::from(*n)),
            (Pattern::Int(bounds), Value::U16(n)) => bounds.contains(usize::from(*n)),
            (Pattern::Int(bounds), Value::U32(n)) => bounds.contains(usize::try_from(*n).unwrap()),
            (Pattern::Int(bounds), Value::U64(n)) => bounds.contains(usize::try_from(*n).unwrap()),
            (Pattern::Char(c0), Value::Char(c1)) => c0 == c1,
            (Pattern::Tuple(ps), Value::Tuple(vs)) if ps.len() == vs.len() => {
                for (p, v) in Iterator::zip(ps.iter(), vs.iter()) {
                    if !v.matches_inner(scope, p) {
                        return false;
                    }
                }
                true
            }
            (Pattern::Seq(ps), Value::Seq(vs)) if ps.len() == vs.len() => {
                for (p, v) in Iterator::zip(ps.iter(), vs.iter()) {
                    if !v.matches_inner(scope, p) {
                        return false;
                    }
                }
                true
            }

            (Pattern::Variant(label0, p), Value::Variant(label1, v)) if label0 == label1 => {
                v.matches_inner(scope, p)
            }
            (Pattern::Option(None), Value::Option(None)) => true,
            (Pattern::Option(Some(p)), Value::Option(Some(v))) => v.matches_inner(scope, p),
            _ => false,
        }
    }

    pub(crate) fn matches<'a>(
        &'a self,
        scope: &'a Scope<'a>,
        pattern: &Pattern,
    ) -> Option<MultiScope<'a>> {
        let mut pattern_scope = MultiScope::new(scope);
        self.coerce_mapped_value()
            .matches_inner(&mut pattern_scope, pattern)
            .then_some(pattern_scope)
    }

    pub fn coerce_mapped_value(&self) -> &Self {
        match self {
            Value::Mapped(_orig, v) => v.coerce_mapped_value(),
            Value::Branch(_n, v) => v.coerce_mapped_value(),
            v => v,
        }
    }

    pub fn extract_mapped_value(self) -> Self {
        match self {
            Value::Mapped(_orig, v) => v.extract_mapped_value(),
            Value::Branch(_n, v) => v.extract_mapped_value(),
            v => v,
        }
    }

    fn record_proj(&self, label: &str) -> &Self {
        match self {
            Value::Record(fields) => match fields.iter().find(|(l, _)| label == l) {
                Some((_, v)) => v,
                None => panic!("{label} not found in record"),
            },
            _ => panic!("expected record, found {self:?}"),
        }
    }

    pub(crate) fn get_sequence(&self) -> Option<ValueSeq<'_, Self>> {
        match self {
            Value::Seq(elts) => Some(ValueSeq::ValueSeq(elts)),
            Value::EnumFromTo(range) => Some(ValueSeq::IntRange(range.clone())),
            _ => None,
        }
    }

    pub(crate) fn is_boolean(&self) -> bool {
        match self.coerce_mapped_value() {
            Value::Bool(_) => true,
            _ => false,
        }
    }
}

impl Value {
    pub const UNIT: Value = Value::Tuple(Vec::new());

    pub fn record<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Value)>) -> Value {
        Value::Record(
            fields
                .into_iter()
                .map(|(label, value)| (label.into(), value))
                .collect(),
        )
    }

    pub fn variant(label: impl IntoLabel, value: impl Into<Box<Value>>) -> Value {
        Value::Variant(label.into(), value.into())
    }

    /// Unwraps any compatible numeric-typed `Value` and returns the contained number as a `usize`.
    ///
    /// # Panics
    ///
    /// Panics if the value is not numeric.
    pub(crate) fn unwrap_usize(self) -> usize {
        match self {
            Value::U8(n) => usize::from(n),
            Value::U16(n) => usize::from(n),
            Value::U32(n) => usize::try_from(n).unwrap(),
            Value::U64(n) => usize::try_from(n).unwrap(),
            Value::Usize(n) => n,
            _ => panic!("value is not a number"),
        }
    }

    /// Unwraps `Value::U8` and returns the contained value, or panics if the value is not `Value::U8`.
    pub(crate) fn get_as_u8(&self) -> u8 {
        match self {
            Value::U8(n) => *n,
            Value::U16(..) | Value::U32(..) | Value::U64(..) | Value::Usize(..) => panic!("value is numeric but not u8 (this may be a soft error, or even success, in future)"),
            _ => panic!("value is not a number"),
        }
    }

    pub(crate) fn unwrap_tuple(self) -> Vec<Value> {
        match self {
            Value::Tuple(values) => values,
            _ => panic!("value is not a tuple"),
        }
    }

    pub(crate) fn unwrap_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!("value is not a bool"),
        }
    }

    /// FIXME - do we really need this?
    #[allow(dead_code)]
    fn unwrap_char(self) -> char {
        match self {
            Value::Char(c) => c,
            _ => panic!("value is not a char"),
        }
    }
}

impl Expr {
    pub fn eval<'a>(&'a self, scope: &'a Scope<'a>) -> Cow<'a, Value> {
        match self {
            Expr::Var(name) => Cow::Borrowed(scope.get_value_by_name(name)),
            Expr::Bool(b) => Cow::Owned(Value::Bool(*b)),
            Expr::U8(i) => Cow::Owned(Value::U8(*i)),
            Expr::U16(i) => Cow::Owned(Value::U16(*i)),
            Expr::U32(i) => Cow::Owned(Value::U32(*i)),
            Expr::U64(i) => Cow::Owned(Value::U64(*i)),
            Expr::Tuple(exprs) => Cow::Owned(Value::Tuple(
                exprs.iter().map(|expr| expr.eval_value(scope)).collect(),
            )),
            Expr::TupleProj(head, index) => cow_map(head.eval(scope), |v| {
                v.coerce_mapped_value().tuple_proj(*index)
            }),
            Expr::Record(fields) => {
                Cow::Owned(Value::record(fields.iter().map(|(label, expr)| {
                    (label.clone(), expr.eval(scope).into_owned())
                })))
            }
            Expr::RecordProj(head, label) => cow_map(head.eval(scope), |v| {
                v.coerce_mapped_value().record_proj(label.as_ref())
            }),
            Expr::Variant(label, expr) => {
                Cow::Owned(Value::variant(label.clone(), expr.eval_value(scope)))
            }
            Expr::Seq(exprs) => Cow::Owned(Value::Seq(
                exprs.iter().map(|expr| expr.eval_value(scope)).collect(),
            )),
            Expr::Match(head, branches) => {
                let head = head.eval(scope);
                for (pattern, expr) in branches {
                    if let Some(pattern_scope) = head.matches(scope, pattern) {
                        let value = expr.eval_value(&Scope::Multi(&pattern_scope));
                        return Cow::Owned(value);
                    }
                }
                panic!("non-exhaustive patterns");
            }
            Expr::Destructure(head, pat, expr) => {
                let head = head.eval(scope);
                if let Some(pattern_scope) = head.matches(scope, pat) {
                    let value = expr.eval_value(&Scope::Multi(&pattern_scope));
                    return Cow::Owned(value);
                } else {
                    panic!("refutable pattern failed to match: {pat:?} :~ {head:?}");
                }
            }
            Expr::Lambda(_, _) => panic!("cannot eval lambda"),

            Expr::IntRel(IntRel::Eq, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x == y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x == y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x == y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x == y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::IntRel(IntRel::Ne, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x != y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x != y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x != y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x != y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::IntRel(IntRel::Lt, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x < y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x < y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x < y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x < y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::IntRel(IntRel::Gt, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x > y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x > y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x > y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x > y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::IntRel(IntRel::Lte, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x <= y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x <= y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x <= y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x <= y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::IntRel(IntRel::Gte, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::Bool(x >= y),
                    (Value::U16(x), Value::U16(y)) => Value::Bool(x >= y),
                    (Value::U32(x), Value::U32(y)) => Value::Bool(x >= y),
                    (Value::U64(x), Value::U64(y)) => Value::Bool(x >= y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::Add, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_add(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_add(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_add(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_add(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::Sub, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_sub(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_sub(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_sub(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_sub(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::Mul, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_mul(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_mul(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_mul(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_mul(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::Div, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_div(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_div(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_div(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_div(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::Rem, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(u8::checked_rem(x, y).unwrap()),
                    (Value::U16(x), Value::U16(y)) => Value::U16(u16::checked_rem(x, y).unwrap()),
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_rem(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => Value::U64(u64::checked_rem(x, y).unwrap()),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::BitAnd, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(x & y),
                    (Value::U16(x), Value::U16(y)) => Value::U16(x & y),
                    (Value::U32(x), Value::U32(y)) => Value::U32(x & y),
                    (Value::U64(x), Value::U64(y)) => Value::U64(x & y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::BitOr, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => Value::U8(x | y),
                    (Value::U16(x), Value::U16(y)) => Value::U16(x | y),
                    (Value::U32(x), Value::U32(y)) => Value::U32(x | y),
                    (Value::U64(x), Value::U64(y)) => Value::U64(x | y),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::BoolAnd, x, y) => {
                // REVIEW - do we want left-biased short-circuiting?
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::Bool(b0), Value::Bool(b1)) => Value::Bool(b0 && b1),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::BoolOr, x, y) => {
                // REVIEW - do we want left-biased short-circuiting?
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::Bool(b0), Value::Bool(b1)) => Value::Bool(b0 || b1),
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::Shl, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => {
                        Value::U8(u8::checked_shl(x, u32::from(y)).unwrap())
                    }
                    (Value::U16(x), Value::U16(y)) => {
                        Value::U16(u16::checked_shl(x, u32::from(y)).unwrap())
                    }
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shl(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => {
                        Value::U64(u64::checked_shl(x, u32::try_from(y).unwrap()).unwrap())
                    }
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Arith(Arith::Shr, x, y) => {
                Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                    (Value::U8(x), Value::U8(y)) => {
                        Value::U8(u8::checked_shr(x, u32::from(y)).unwrap())
                    }
                    (Value::U16(x), Value::U16(y)) => {
                        Value::U16(u16::checked_shr(x, u32::from(y)).unwrap())
                    }
                    (Value::U32(x), Value::U32(y)) => Value::U32(u32::checked_shr(x, y).unwrap()),
                    (Value::U64(x), Value::U64(y)) => {
                        Value::U64(u64::checked_shr(x, u32::try_from(y).unwrap()).unwrap())
                    }
                    (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                })
            }
            Expr::Unary(UnaryOp::BoolNot, x) => Cow::Owned(match x.eval_value(scope) {
                Value::Bool(x) => Value::Bool(!x),
                x => panic!("unexpected operand: expecting boolean, found `{x:?}`"),
            }),
            Expr::Unary(UnaryOp::IntSucc, x) => Cow::Owned(match x.eval_value(scope) {
                Value::U8(x) => Value::U8(
                    x.checked_add(1)
                        .unwrap_or_else(|| panic!("IntSucc(u8::MAX) overflowed")),
                ),
                Value::U16(x) => Value::U16(
                    x.checked_add(1)
                        .unwrap_or_else(|| panic!("IntSucc(u16::MAX) overflowed")),
                ),
                Value::U32(x) => Value::U32(
                    x.checked_add(1)
                        .unwrap_or_else(|| panic!("IntSucc(u32::MAX) overflowed")),
                ),
                Value::U64(x) => Value::U64(
                    x.checked_add(1)
                        .unwrap_or_else(|| panic!("IntSucc(u64::MAX) overflowed")),
                ),
                x => panic!("unexpected operand: expected integral value, found `{x:?}`"),
            }),
            Expr::Unary(UnaryOp::IntPred, x) => Cow::Owned(match x.eval_value(scope) {
                Value::U8(x) => Value::U8(
                    x.checked_sub(1)
                        .unwrap_or_else(|| panic!("IntPred(0u8) underflow")),
                ),
                Value::U16(x) => Value::U16(
                    x.checked_sub(1)
                        .unwrap_or_else(|| panic!("IntPred(0u16) underflow")),
                ),
                Value::U32(x) => Value::U32(
                    x.checked_sub(1)
                        .unwrap_or_else(|| panic!("IntPred(0u32) underflow")),
                ),
                Value::U64(x) => Value::U64(
                    x.checked_sub(1)
                        .unwrap_or_else(|| panic!("IntPred(0u64) underflow")),
                ),
                x => panic!("unexpected operand: expected integral value, found `{x:?}`"),
            }),

            Expr::AsU8(x) => {
                Cow::Owned(match x.eval_value(scope) {
                    Value::U8(x) => Value::U8(x),
                    Value::U16(x) => Value::U8(u8::try_from(x).unwrap_or_else(|err| {
                        panic!("cannot perform AsU8 cast on u16 {x}: {err}")
                    })),
                    Value::U32(x) => Value::U8(u8::try_from(x).unwrap_or_else(|err| {
                        panic!("cannot perform AsU8 cast on u32 {x}: {err}")
                    })),
                    Value::U64(x) => Value::U8(u8::try_from(x).unwrap_or_else(|err| {
                        panic!("cannot perform AsU8 cast on u64 {x}: {err}")
                    })),
                    x => panic!("cannot convert {x:?} to U8"),
                })
            }
            Expr::AsU16(x) => Cow::Owned(match x.eval_value(scope) {
                Value::U8(x) => Value::U16(u16::from(x)),
                Value::U16(x) => Value::U16(x),
                Value::U32(x) => Value::U16(u16::try_from(x).unwrap()),
                Value::U64(x) => Value::U16(u16::try_from(x).unwrap()),
                x => panic!("cannot convert {x:?} to U16"),
            }),
            Expr::AsU32(x) => Cow::Owned(match x.eval_value(scope) {
                Value::U8(x) => Value::U32(u32::from(x)),
                Value::U16(x) => Value::U32(u32::from(x)),
                Value::U32(x) => Value::U32(x),
                Value::U64(x) => Value::U32(u32::try_from(x).unwrap()),
                x => panic!("cannot convert {x:?} to U32"),
            }),
            Expr::AsU64(x) => Cow::Owned(match x.eval_value(scope) {
                Value::U8(x) => Value::U64(u64::from(x)),
                Value::U16(x) => Value::U64(u64::from(x)),
                Value::U32(x) => Value::U64(u64::from(x)),
                Value::U64(x) => Value::U64(x),
                x => panic!("cannot convert {x:?} to U64"),
            }),

            Expr::U16Be(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(hi), Value::U8(lo)] => {
                    Cow::Owned(Value::U16(u16::from_be_bytes([*hi, *lo])))
                }
                _ => panic!("U16Be: expected (U8, U8)"),
            },
            Expr::U16Le(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(lo), Value::U8(hi)] => {
                    Cow::Owned(Value::U16(u16::from_le_bytes([*lo, *hi])))
                }
                _ => panic!("U16Le: expected (U8, U8)"),
            },
            Expr::U32Be(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                    Cow::Owned(Value::U32(u32::from_be_bytes([*a, *b, *c, *d])))
                }
                _ => panic!("U32Be: expected (U8, U8, U8, U8)"),
            },
            Expr::U32Le(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d)] => {
                    Cow::Owned(Value::U32(u32::from_le_bytes([*a, *b, *c, *d])))
                }
                _ => panic!("U32Le: expected (U8, U8, U8, U8)"),
            },
            Expr::U64Be(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d), Value::U8(e), Value::U8(f), Value::U8(g), Value::U8(h)] => {
                    Cow::Owned(Value::U64(u64::from_be_bytes([
                        *a, *b, *c, *d, *e, *f, *g, *h,
                    ])))
                }
                _ => panic!("U32Be: expected (U8, U8, U8, U8, U8, U8, U8, U8)"),
            },
            Expr::U64Le(bytes) => match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                [Value::U8(a), Value::U8(b), Value::U8(c), Value::U8(d), Value::U8(e), Value::U8(f), Value::U8(g), Value::U8(h)] => {
                    Cow::Owned(Value::U64(u64::from_le_bytes([
                        *a, *b, *c, *d, *e, *f, *g, *h,
                    ])))
                }
                _ => panic!("U32Le: expected (U8, U8, U8, U8, U8, U8, U8, U8)"),
            },
            Expr::AsChar(bytes) => Cow::Owned(match bytes.eval_value(scope) {
                Value::U8(x) => Value::Char(char::from(x)),
                Value::U16(x) => {
                    Value::Char(char::from_u32(x as u32).unwrap_or(char::REPLACEMENT_CHARACTER))
                }
                Value::U32(x) => {
                    Value::Char(char::from_u32(x).unwrap_or(char::REPLACEMENT_CHARACTER))
                }
                Value::U64(x) => Value::Char(
                    char::from_u32(u32::try_from(x).unwrap())
                        .unwrap_or(char::REPLACEMENT_CHARACTER),
                ),
                _ => panic!("AsChar: expected U8, U16, U32, or U64"),
            }),
            Expr::SeqLength(seq) => match seq.eval(scope).coerce_mapped_value().get_sequence() {
                Some(values) => {
                    let len = values.len();
                    Cow::Owned(Value::U32(len as u32))
                }
                _ => panic!("SeqLength: expected Seq"),
            },
            Expr::SeqIx(seq, index) => cow_remap(seq.eval(scope), |v| {
                match v.coerce_mapped_value().get_sequence() {
                    Some(values) => match values {
                        ValueSeq::ValueSeq(values) => {
                            let index = index.eval_value(scope).unwrap_usize();
                            Cow::Borrowed(&values[index])
                        }
                        ValueSeq::IntRange(mut range) => {
                            let index = index.eval_value(scope).unwrap_usize();
                            Cow::Owned(Value::from(range.nth(index).unwrap()))
                        }
                    },
                    _ => panic!("SeqIx: expected Seq (or RangeFromTo)"),
                }
            }),
            Expr::SubSeq(seq, start, length) => {
                match seq.eval(scope).coerce_mapped_value().get_sequence() {
                    Some(values) => match values {
                        ValueSeq::ValueSeq(values) => {
                            let start = start.eval_value(scope).unwrap_usize();
                            let length = length.eval_value(scope).unwrap_usize();
                            Cow::Owned(Value::Seq(values.sub_seq(start, length)))
                        }
                        ValueSeq::IntRange(range) => {
                            let start = start.eval_value(scope).unwrap_usize();
                            let length = length.eval_value(scope).unwrap_usize();
                            Cow::Owned(Value::EnumFromTo(sub_range(range, start, length)))
                        }
                    },
                    _ => panic!("SubSeq: expected Seq"),
                }
            }
            Expr::SubSeqInflate(seq, start, length) => {
                match seq.eval(scope).coerce_mapped_value().get_sequence() {
                    Some(values) => {
                        let start = start.eval_value(scope).unwrap_usize();
                        let length = length.eval_value(scope).unwrap_usize();
                        let mut vs = Vec::new();
                        match values {
                            ValueSeq::ValueSeq(vs0) => {
                                for i in 0..length {
                                    if i + start < vs0.len() {
                                        vs.push(vs0[i + start].clone());
                                    } else {
                                        vs.push(vs[i + start - vs0.len()].clone());
                                    }
                                }
                            }
                            ValueSeq::IntRange(range) => {
                                // REVIEW - double-check this logic
                                let len = range.len();
                                let mut iter = range.skip(start);
                                for i in 0..length {
                                    if let Some(val) = iter.next() {
                                        vs.push(val.into());
                                    } else {
                                        vs.push(vs[i + start - len].clone());
                                    }
                                }
                            }
                        }
                        Cow::Owned(Value::Seq(vs.into()))
                    }
                    _ => panic!("SubSeqInflate: expected Seq"),
                }
            }
            Expr::Append(seq0, seq1) => {
                match seq0.eval(scope).coerce_mapped_value().get_sequence() {
                    Some(val_seq0) => match seq1.eval(scope).coerce_mapped_value().get_sequence() {
                        Some(val_seq1) => {
                            if val_seq0.is_empty() {
                                return Cow::Owned(seq1.eval(scope).coerce_mapped_value().clone());
                            } else if val_seq1.is_empty() {
                                return Cow::Owned(seq0.eval(scope).coerce_mapped_value().clone());
                            }
                            Cow::Owned(Value::Seq(val_seq0.append(val_seq1)))
                        }
                        _ => unreachable!("Append: expected Seq in (lhs)"),
                    },
                    _ => unreachable!("Append: expected Seq in (lhs)"),
                }
            }
            Expr::FlatMap(expr, seq) => {
                match seq.eval(scope).coerce_mapped_value().get_sequence() {
                    Some(values) => {
                        let mut vs = Vec::new();
                        for v in values {
                            match expr.eval_lambda(scope, &v) {
                                Value::Seq(vn) => {
                                    vs.extend(vn);
                                }
                                Value::EnumFromTo(range) => {
                                    vs.extend(range.map(Value::from));
                                }
                                _ => {
                                    panic!("FlatMap: expected Seq (or EnumFromTo)");
                                }
                            }
                        }
                        Cow::Owned(Value::Seq(vs.into()))
                    }
                    _ => panic!("FlatMap: expected Seq"),
                }
            }
            Expr::FlatMapAccum(expr, accum, _accum_type, seq) => match seq.eval_value(scope) {
                Value::Seq(values) => {
                    let mut accum = accum.eval_value(scope);
                    let mut vs = Vec::new();
                    for v in values {
                        let ret = expr.eval_lambda(scope, &Value::Tuple(vec![accum, v]));
                        accum = match extract_pair(ret.unwrap_tuple()) {
                            (accum, Value::Seq(vn)) => {
                                vs.extend(vn);
                                accum
                            }
                            _ => panic!("FlatMapAccum: expected two values"),
                        };
                    }
                    Cow::Owned(Value::Seq(vs.into()))
                }
                _ => panic!("FlatMapAccum: expected Seq"),
            },
            Expr::LeftFold(expr, accum, _accum_type, seq) => match seq.eval_value(scope) {
                Value::Seq(values) => {
                    let mut accum = accum.eval_value(scope);
                    for v in values {
                        let tmp = expr.eval_lambda(scope, &Value::Tuple(vec![accum, v]));
                        accum = tmp
                    }
                    Cow::Owned(accum)
                }
                _ => panic!("LeftFold: expected Seq"),
            },
            Expr::FindByKey(is_sorted, f_get_key, query_key, seq) => match seq.eval_value(scope) {
                Value::Seq(values) => {
                    let query = query_key.eval_value(scope);
                    let eval = |lambda: &Expr, arg: &Value| lambda.eval_lambda(scope, arg);
                    if *is_sorted {
                        match search::find_index_by_key_sorted(f_get_key, &query, &values, eval) {
                            Some(ix) => {
                                Cow::Owned(Value::Option(Some(Box::new(values[ix].clone()))))
                            }
                            None => Cow::Owned(Value::Option(None)),
                        }
                    } else {
                        match search::find_index_by_key_unsorted(f_get_key, &query, &values, eval) {
                            Some(ix) => {
                                Cow::Owned(Value::Option(Some(Box::new(values[ix].clone()))))
                            }
                            None => Cow::Owned(Value::Option(None)),
                        }
                    }
                }
                _ => panic!("FindByKey: expected Seq"),
            },
            Expr::FlatMapList(expr, _ret_type, seq) => match seq.eval_value(scope) {
                Value::Seq(values) => {
                    let mut vs = Vec::new();
                    for v in values {
                        let arg = Value::Tuple(vec![Value::Seq(SeqKind::Strict(vs)), v]);
                        if let Value::Seq(vn) = expr.eval_lambda(scope, &arg) {
                            vs = match arg {
                                Value::Tuple(mut args) => match args.remove(0) {
                                    Value::Seq(vs) => vs.into_vec(),
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            };
                            vs.extend(vn);
                        } else {
                            panic!("FlatMapList: expected Seq");
                        }
                    }
                    Cow::Owned(Value::Seq(vs.into()))
                }
                _ => panic!("FlatMapList: expected Seq"),
            },
            Expr::Dup(count, expr) => {
                let count = count.eval_value(scope).unwrap_usize();
                let v = expr.eval_value(scope);
                Cow::Owned(Value::Seq(SeqKind::Dup(count, Box::new(v))))
            }
            Expr::EnumFromTo(start, stop) => {
                let start = start.eval_value(scope).unwrap_usize();
                let stop = stop.eval_value(scope).unwrap_usize();
                Cow::Owned(Value::EnumFromTo(start..stop))
            }
            Expr::LiftOption(opt) => match opt {
                Some(expr) => Cow::Owned(Value::Option(Some(Box::new(expr.eval_value(scope))))),
                None => Cow::Owned(Value::Option(None)),
            },
        }
    }

    fn eval_value_ref<'a, 'b: 'a>(&'b self, scope: &'a Scope<'a>) -> Cow<'a, Value> {
        match self.eval(scope) {
            Cow::Borrowed(value) => Cow::Borrowed(value.coerce_mapped_value()),
            Cow::Owned(v) => Cow::Owned(v.extract_mapped_value()),
        }
    }

    pub fn eval_value<'a>(&self, scope: &'a Scope<'a>) -> Value {
        self.eval_value_ref(scope).into_owned()
    }

    fn eval_lambda<'a>(&self, scope: &'a Scope<'a>, arg: &Value) -> Value {
        match self {
            Expr::Lambda(name, expr) => {
                let child_scope = SingleScope::new(scope, name, arg);
                expr.eval_value(&Scope::Single(child_scope))
            }
            _ => panic!("expected Lambda"),
        }
    }
}

pub(crate) mod search;

/// Decoders with a fixed amount of lookahead
#[derive(Clone, Debug)]
pub enum Decoder {
    Call(usize, Vec<(Label, Expr)>),
    Pos,
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Variant(Label, Box<Decoder>),
    Parallel(Vec<Decoder>),
    Branch(MatchTree, Vec<Decoder>),
    Tuple(Vec<Decoder>),
    Sequence(Vec<Decoder>),
    While(MatchTree, Box<Decoder>),
    Until(MatchTree, Box<Decoder>),
    RepeatCount(Box<Expr>, Box<Decoder>),
    RepeatUntilLast(Box<Expr>, Box<Decoder>),
    RepeatUntilSeq(Box<Expr>, Box<Decoder>),
    Maybe(Box<Expr>, Box<Decoder>),
    Peek(Box<Decoder>),
    PeekNot(Box<Decoder>),
    Slice(Box<Expr>, Box<Decoder>),
    Bits(Box<Decoder>),
    WithRelativeOffset(Box<Expr>, Box<Expr>, Box<Decoder>),
    Map(Box<Decoder>, Box<Expr>),
    Where(Box<Decoder>, Box<Expr>),
    Compute(Box<Expr>),
    Let(Label, Box<Expr>, Box<Decoder>),
    Match(Box<Expr>, Vec<(Pattern, Decoder)>),
    Dynamic(Label, DynFormat, Box<Decoder>),
    Apply(Label),
    RepeatBetween(MatchTree, Box<Expr>, Box<Expr>, Box<Decoder>),
    ForEach(Box<Expr>, Label, Box<Decoder>),
    SkipRemainder,
    DecodeBytes(Box<Expr>, Box<Decoder>),
    LetFormat(Box<Decoder>, Label, Box<Decoder>),
    MonadSeq(Box<Decoder>, Box<Decoder>),
    AccumUntil(Box<Expr>, Box<Expr>, Box<Expr>, TypeHint, Box<Decoder>),
    LiftedOption(Option<Box<Decoder>>),
}

#[derive(Clone, Debug)]
pub struct Program {
    pub decoders: Vec<(Decoder, ValueType)>,
}

impl Program {
    fn new() -> Self {
        let decoders = Vec::new();
        Program { decoders }
    }

    pub fn run<'input>(&self, input: ReadCtxt<'input>) -> DecodeResult<(Value, ReadCtxt<'input>)> {
        self.decoders[0].0.parse(self, &Scope::Empty, input)
    }
}

pub struct Compiler<'a> {
    module: &'a FormatModule,
    program: Program,
    decoder_map: HashMap<(usize, Rc<Next<'a>>), usize>,
    compile_queue: Vec<(&'a Format, Rc<Next<'a>>, usize)>,
}

impl<'a> Compiler<'a> {
    fn new(module: &'a FormatModule) -> Self {
        let program = Program::new();
        let decoder_map = HashMap::new();
        let compile_queue = Vec::new();
        Compiler {
            module,
            program,
            decoder_map,
            compile_queue,
        }
    }

    pub fn compile_program(module: &FormatModule, format: &Format) -> AResult<Program> {
        let mut compiler = Compiler::new(module);
        // type
        let scope = TypeScope::new();
        let t = module.infer_format_type(&scope, format)?;
        // decoder
        compiler.queue_compile(t, format, Rc::new(Next::Empty));
        while let Some((f, next, n)) = compiler.compile_queue.pop() {
            let d = compiler.compile_format(f, next)?;
            compiler.program.decoders[n].0 = d;
        }
        Ok(compiler.program)
    }

    fn queue_compile(&mut self, t: ValueType, f: &'a Format, next: Rc<Next<'a>>) -> usize {
        let n = self.program.decoders.len();
        self.program.decoders.push((Decoder::Fail, t));
        self.compile_queue.push((f, next, n));
        n
    }

    pub fn compile_one(format: &Format) -> AResult<Decoder> {
        let module = FormatModule::new();
        let mut compiler = Compiler::new(&module);
        compiler.compile_format(format, Rc::new(Next::Empty))
    }

    fn compile_format(&mut self, format: &'a Format, next: Rc<Next<'a>>) -> AResult<Decoder> {
        match format {
            Format::ItemVar(level, arg_exprs) => {
                let f = self.module.get_format(*level);
                let next = if f.depends_on_next(self.module) {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = self.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let t = self.module.get_format_type(*level).clone();
                    let n = self.queue_compile(t, f, next.clone());
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                let arg_names = self.module.get_args(*level);
                let mut args = Vec::new();
                for ((name, _type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    args.push((name.clone(), expr.clone()));
                }
                Ok(Decoder::Call(n, args))
            }
            Format::Fail => Ok(Decoder::Fail),
            Format::DecodeBytes(expr, inner) => {
                let d = self.compile_format(inner, next.clone())?;
                Ok(Decoder::DecodeBytes(expr.clone(), Box::new(d)))
            }
            Format::Pos => Ok(Decoder::Pos),
            Format::EndOfInput => Ok(Decoder::EndOfInput),
            Format::Align(n) => Ok(Decoder::Align(*n)),
            Format::Byte(bs) => Ok(Decoder::Byte(*bs)),
            Format::SkipRemainder => Ok(Decoder::SkipRemainder),
            Format::Variant(label, f) => {
                let d = self.compile_format(f, next.clone())?;
                Ok(Decoder::Variant(label.clone(), Box::new(d)))
            }
            Format::Union(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_format(f, next.clone())?);
                }
                if let Some(tree) = MatchTree::build(self.module, branches, next) {
                    Ok(Decoder::Branch(tree, ds))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            Format::UnionNondet(branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    let d = self.compile_format(f, next.clone())?;
                    ds.push(d);
                }
                Ok(Decoder::Parallel(ds))
            }
            Format::Tuple(elems) => {
                let mut decs = Vec::with_capacity(elems.len());
                let mut fields = elems.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(
                        MaybeTyped::Untyped(fields.as_slice()),
                        next.clone(),
                    ));
                    let df = self.compile_format(f, next)?;
                    decs.push(df);
                }
                Ok(Decoder::Tuple(decs))
            }
            Format::Sequence(formats) => {
                let mut decs = Vec::with_capacity(formats.len());
                let mut fields = formats.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::Sequence(
                        MaybeTyped::Untyped(fields.as_slice()),
                        next.clone(),
                    ));
                    let df = self.compile_format(f, next)?;
                    decs.push(df);
                }
                Ok(Decoder::Sequence(decs))
            }
            Format::Repeat(a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_format(
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                )?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::Tuple(vec![(**a).clone(), astar]);
                let fb = Format::EMPTY;
                if let Some(tree) = MatchTree::build(self.module, &[fa, fb], next) {
                    Ok(Decoder::While(tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            Format::Repeat1(a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_format(
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                )?;
                let astar = Format::Repeat(a.clone());
                let fa = Format::EMPTY;
                let fb = Format::Tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build(self.module, &[fa, fb], next) {
                    Ok(Decoder::Until(tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            Format::RepeatCount(expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::RepeatCount(expr.clone(), da))
            }
            Format::RepeatBetween(xmin, xmax, a) => {
                // FIXME - preliminary support only for exact-bound limit values
                let Some(min) = xmin.bounds().is_exact() else {
                    unimplemented!("RepeatBetween on inexact bounds-expr")
                };
                let Some(max) = xmax.bounds().is_exact() else {
                    unimplemented!("RepeatBetween on inexact bounds-expr")
                };

                let da = self.compile_format(
                    a,
                    Rc::new(Next::RepeatBetween(
                        min.saturating_sub(1),
                        max.saturating_sub(1),
                        MaybeTyped::Untyped(a),
                        next.clone(),
                    )),
                )?;

                let tree = {
                    let mut branches: Vec<Format> = Vec::new();
                    // FIXME: this is inefficient but probably works
                    for count in 0..=max {
                        let f_count =
                            Format::RepeatCount(Box::new(Expr::U32(count as u32)), a.clone());
                        branches.push(f_count);
                    }
                    let Some(tree) = MatchTree::build(self.module, &branches[..], next) else {
                        panic!("cannot build match tree for {:?}", format)
                    };
                    tree
                };
                Ok(Decoder::RepeatBetween(
                    tree,
                    xmin.clone(),
                    xmax.clone(),
                    Box::new(da),
                ))
            }
            Format::RepeatUntilLast(expr, a) => {
                // FIXME - the `Next` value we pass in is probably not right
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::RepeatUntilLast(expr.clone(), da))
            }
            Format::ForEach(expr, lbl, a) => {
                // FIXME - the `Next` value we pass in is probably not right
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::ForEach(expr.clone(), lbl.clone(), da))
            }
            Format::RepeatUntilSeq(expr, a) => {
                // FIXME - the `Next` value we pass in is probably not right
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::RepeatUntilSeq(expr.clone(), da))
            }
            Format::AccumUntil(f_done, f_update, init, vt, a) => {
                // FIXME - the `Next` value we pass in is probably not right
                let da = Box::new(self.compile_format(a, next)?);
                Ok(Decoder::AccumUntil(
                    f_done.clone(),
                    f_update.clone(),
                    init.clone(),
                    vt.clone(),
                    da,
                ))
            }
            Format::Maybe(x, a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Maybe(x.clone(), da))
            }
            Format::Peek(a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Peek(da))
            }
            Format::PeekNot(a) => {
                const MAX_LOOKAHEAD: usize = 1024;
                match a.lookahead_bounds(self.module).max {
                    None => return Err(anyhow!("PeekNot cannot require unbounded lookahead")),
                    Some(n) if n > MAX_LOOKAHEAD => {
                        return Err(anyhow!(
                            "PeekNot cannot require > {MAX_LOOKAHEAD} bytes lookahead"
                        ))
                    }
                    _ => {}
                }
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::PeekNot(da))
            }
            Format::Slice(expr, a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Slice(expr.clone(), da))
            }
            Format::Bits(a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::Bits(da))
            }
            Format::WithRelativeOffset(addr, expr, a) => {
                let da = Box::new(self.compile_format(a, Rc::new(Next::Empty))?);
                Ok(Decoder::WithRelativeOffset(addr.clone(), expr.clone(), da))
            }
            Format::Map(a, expr) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Map(da, expr.clone()))
            }
            Format::Where(a, expr) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Where(da, expr.clone()))
            }
            Format::Compute(expr) => Ok(Decoder::Compute(expr.clone())),
            Format::Let(name, expr, a) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Let(name.clone(), expr.clone(), da))
            }
            Format::Match(head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| {
                        Ok((pattern.clone(), self.compile_format(f, next.clone())?))
                    })
                    .collect::<AResult<_>>()?;
                Ok(Decoder::Match(head.clone(), branches))
            }
            Format::Dynamic(name, dynformat, a) => {
                let da = Box::new(self.compile_format(a, next.clone())?);
                Ok(Decoder::Dynamic(name.clone(), dynformat.clone(), da))
            }
            Format::Apply(name) => Ok(Decoder::Apply(name.clone())),
            Format::LetFormat(first, name, second) => {
                let a_next = Next::Cat(MaybeTyped::Untyped(second), next.clone());
                let da = Box::new(self.compile_format(first, Rc::new(a_next))?);
                let db = Box::new(self.compile_format(second, next.clone())?);
                Ok(Decoder::LetFormat(da, name.clone(), db))
            }
            Format::MonadSeq(first, second) => {
                let a_next = Next::Cat(MaybeTyped::Untyped(second), next.clone());
                let da = Box::new(self.compile_format(first, Rc::new(a_next))?);
                let db = Box::new(self.compile_format(second, next.clone())?);
                Ok(Decoder::MonadSeq(da, db))
            }
            Format::Hint(_hint, a) => {
                // REVIEW - do we want to preserve any facet of the hinting within the Decoder?
                self.compile_format(a, next)
            }
            Format::LiftedOption(None) => Ok(Decoder::LiftedOption(None)),
            Format::LiftedOption(Some(a)) => Ok(Decoder::LiftedOption(Some(Box::new(
                self.compile_format(a, next)?,
            )))),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ScopeEntry<Value: Clone> {
    Value(Value),
    Decoder(Decoder),
}

pub enum Scope<'a> {
    Empty,
    Multi(&'a MultiScope<'a>),
    Single(SingleScope<'a>),
    Decoder(DecoderScope<'a>),
}

pub struct MultiScope<'a> {
    parent: &'a Scope<'a>,
    entries: Vec<(Label, Cow<'a, Value>)>,
}

pub struct SingleScope<'a> {
    parent: &'a Scope<'a>,
    name: &'a str,
    value: &'a Value,
}

pub struct DecoderScope<'a> {
    parent: &'a Scope<'a>,
    name: &'a str,
    decoder: Decoder,
}

impl<'a> Scope<'a> {
    fn get_value_by_name(&self, name: &str) -> &Value {
        match self {
            Scope::Empty => panic!("value not found: {name}"),
            Scope::Multi(multi) => multi.get_value_by_name(name),
            Scope::Single(single) => single.get_value_by_name(name),
            Scope::Decoder(decoder) => decoder.parent.get_value_by_name(name),
        }
    }

    fn get_decoder_by_name(&self, name: &str) -> &Decoder {
        match self {
            Scope::Empty => panic!("decoder not found: {name}"),
            Scope::Multi(multi) => multi.parent.get_decoder_by_name(name),
            Scope::Single(single) => single.parent.get_decoder_by_name(name),
            Scope::Decoder(decoder) => decoder.get_decoder_by_name(name),
        }
    }

    pub fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<Value>)>) {
        match self {
            Scope::Empty => {}
            Scope::Multi(multi) => multi.get_bindings(bindings),
            Scope::Single(single) => single.get_bindings(bindings),
            Scope::Decoder(decoder) => decoder.get_bindings(bindings),
        }
    }
}

impl<'a> MultiScope<'a> {
    fn new(parent: &'a Scope<'a>) -> MultiScope<'a> {
        let entries = Vec::new();
        MultiScope { parent, entries }
    }

    pub fn with_capacity(parent: &'a Scope<'a>, capacity: usize) -> MultiScope<'a> {
        let entries = Vec::with_capacity(capacity);
        MultiScope { parent, entries }
    }

    /// Pushes a new binding to the scope using a borrow that lives at least as long as the scope itself
    pub fn push(&mut self, name: impl Into<Label>, v: &'a Value) {
        self.entries.push((name.into(), Cow::Borrowed(v)));
    }

    /// Pushes a new binding to the scope using an owned [Value]
    pub fn push_owned(&mut self, name: impl Into<Label>, v: Value) {
        self.entries.push((name.into(), Cow::Owned(v)));
    }

    fn get_value_by_name(&self, name: &str) -> &Value {
        for (n, v) in self.entries.iter().rev() {
            if n == name {
                return v;
            }
        }
        self.parent.get_value_by_name(name)
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<Value>)>) {
        for (name, value) in self.entries.iter().rev() {
            bindings.push((name.clone(), ScopeEntry::Value(value.clone().into_owned())));
        }
        self.parent.get_bindings(bindings);
    }
}

impl<'a> SingleScope<'a> {
    pub fn new(parent: &'a Scope<'a>, name: &'a str, value: &'a Value) -> SingleScope<'a> {
        SingleScope {
            parent,
            name,
            value,
        }
    }

    fn get_value_by_name(&self, name: &str) -> &Value {
        if self.name == name {
            self.value
        } else {
            self.parent.get_value_by_name(name)
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<Value>)>) {
        bindings.push((
            self.name.to_string().into(),
            ScopeEntry::Value(self.value.clone()),
        ));
        self.parent.get_bindings(bindings);
    }
}

impl<'a> DecoderScope<'a> {
    fn new(parent: &'a Scope<'a>, name: &'a str, decoder: Decoder) -> DecoderScope<'a> {
        DecoderScope {
            parent,
            name,
            decoder,
        }
    }

    fn get_decoder_by_name(&self, name: &str) -> &Decoder {
        if self.name == name {
            &self.decoder
        } else {
            self.parent.get_decoder_by_name(name)
        }
    }

    fn get_bindings(&self, bindings: &mut Vec<(Label, ScopeEntry<Value>)>) {
        bindings.push((
            self.name.to_string().into(),
            ScopeEntry::Decoder(self.decoder.clone()),
        ));
        self.parent.get_bindings(bindings);
    }
}

impl Decoder {
    pub fn parse<'input>(
        &self,
        program: &Program,
        scope: &Scope<'_>,
        input: ReadCtxt<'input>,
    ) -> DecodeResult<(Value, ReadCtxt<'input>)> {
        match self {
            Decoder::Call(n, es) => {
                let mut new_scope = MultiScope::with_capacity(&Scope::Empty, es.len());
                for (name, e) in es {
                    let v = e.eval_value(scope);
                    new_scope.push_owned(name.clone(), v);
                }
                program.decoders[*n]
                    .0
                    .parse(program, &Scope::Multi(&new_scope), input)
            }
            Decoder::Fail => Err(DecodeError::<Value>::fail(scope, input)),
            Decoder::Pos => {
                let pos = input.offset as u64;
                Ok((Value::U64(pos), input))
            }
            Decoder::SkipRemainder => {
                let input = input.skip_remainder();
                Ok((Value::UNIT, input))
            }
            Decoder::EndOfInput => match input.read_byte() {
                None => Ok((Value::UNIT, input)),
                Some((b, _)) => Err(DecodeError::trailing(b, input.offset)),
            },
            Decoder::Align(n) => {
                let skip = (n - (input.offset % n)) % n;
                let (_, input) = input
                    .split_at(skip)
                    .ok_or(DecodeError::overrun(skip, input.offset))?;
                Ok((Value::UNIT, input))
            }
            Decoder::Byte(bs) => {
                let (b, input) = input
                    .read_byte()
                    .ok_or(DecodeError::overbyte(input.offset))?;
                if bs.contains(b) {
                    Ok((Value::U8(b), input))
                } else {
                    Err(DecodeError::unexpected(b, *bs, input.offset))
                }
            }
            Decoder::Variant(label, d) => {
                let (v, input) = d.parse(program, scope, input)?;
                Ok((Value::Variant(label.clone(), Box::new(v)), input))
            }
            Decoder::Branch(tree, branches) => {
                let index = tree.matches(input).ok_or(DecodeError::NoValidBranch {
                    offset: input.offset,
                })?;
                let d = &branches[index];
                let (v, input) = d.parse(program, scope, input)?;
                Ok((Value::Branch(index, Box::new(v)), input))
            }
            Decoder::Parallel(branches) => {
                for (index, d) in branches.iter().enumerate() {
                    let res = d.parse(program, scope, input);
                    if let Ok((v, input)) = res {
                        return Ok((Value::Branch(index, Box::new(v)), input));
                    }
                }
                Err(DecodeError::<Value>::fail(scope, input))
            }
            Decoder::Tuple(fields) => {
                let mut input = input;
                let mut v = Vec::with_capacity(fields.len());
                for f in fields {
                    let (vf, next_input) = f.parse(program, scope, input)?;
                    input = next_input;
                    v.push(vf);
                }
                Ok((Value::Tuple(v), input))
            }
            Decoder::Sequence(decs) => {
                let mut input = input;
                let mut v = Vec::with_capacity(decs.len());
                for d in decs {
                    let (vf, next_input) = d.parse(program, scope, input)?;
                    input = next_input;
                    v.push(vf);
                }
                Ok((Value::Seq(SeqKind::Strict(v)), input))
            }
            Decoder::While(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                while tree.matches(input).ok_or(DecodeError::NoValidBranch {
                    offset: input.offset,
                })? == 0
                {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v.into()), input))
            }
            Decoder::Until(tree, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                    if tree.matches(input).ok_or(DecodeError::NoValidBranch {
                        offset: input.offset,
                    })? == 0
                    {
                        break;
                    }
                }
                Ok((Value::Seq(v.into()), input))
            }
            Decoder::DecodeBytes(bytes, a) => {
                let bytes = {
                    let raw = bytes.eval_value(scope);
                    let seq_vals = raw.get_sequence().expect("bad type for DecodeBytes input");
                    seq_vals
                        .into_iter()
                        .map(|v| v.get_as_u8())
                        .collect::<Vec<u8>>()
                };
                let new_input = ReadCtxt::new(&bytes);
                let (va, rem_input) = a.parse(program, scope, new_input)?;
                // REVIEW - do we *actually* want to enforce full-consumption of the sub-buffer (i.e. no strictly partial reads)
                match rem_input.read_byte() {
                    Some((b, _)) => {
                        // FIXME - this error-value doesn't properly distinguish between offsets within the main input or the sub-buffer
                        Err(DecodeError::Trailing {
                            byte: b,
                            offset: rem_input.offset,
                        })
                    }
                    None => Ok((va, input)),
                }
            }
            Decoder::LetFormat(da, name, db) => {
                let (va, input) = da.parse(program, scope, input)?;
                let new_scope = Scope::Single(SingleScope::new(scope, name, &va));
                db.parse(program, &new_scope, input)
            }
            Decoder::MonadSeq(da, db) => {
                let (_, input) = da.parse(program, scope, input)?;
                db.parse(program, scope, input)
            }
            Decoder::ForEach(expr, lbl, a) => {
                let mut input = input;
                let val = expr.eval_value(scope);
                let seq = val.get_sequence().expect("bad type for ForEach input");
                let mut v = Vec::with_capacity(seq.len());
                for e in seq {
                    let new_scope = Scope::Single(SingleScope::new(scope, lbl, &e));
                    let (va, next_input) = a.parse(program, &new_scope, input)?;
                    v.push(va);
                    input = next_input;
                }
                Ok((Value::Seq(v.into()), input))
            }
            Decoder::RepeatCount(expr, a) => {
                let mut input = input;
                let count = expr.eval_value(scope).unwrap_usize();
                let mut v = Vec::with_capacity(count);
                for _ in 0..count {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v.into()), input))
            }
            Decoder::RepeatBetween(tree, min, max, a) => {
                let mut input = input;
                let min = min.eval_value(scope).unwrap_usize();
                let max = max.eval_value(scope).unwrap_usize();
                let mut v = Vec::new();
                loop {
                    if tree.matches(input).ok_or(DecodeError::NoValidBranch {
                        offset: input.offset,
                    })? == 0
                        || v.len() == max
                    {
                        if v.len() < min {
                            unreachable!("incoherent bounds for RepeatBetween(_, {min}, {max}, _)");
                        }
                        break;
                    }
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                }
                Ok((Value::Seq(v.into()), input))
            }
            Decoder::Maybe(expr, a) => {
                let is_present = expr.eval_value(scope).unwrap_bool();
                if is_present {
                    let (raw, next_input) = a.parse(program, scope, input)?;
                    Ok((Value::Option(Some(Box::new(raw))), next_input))
                } else {
                    Ok((Value::Option(None), input))
                }
            }
            Decoder::RepeatUntilLast(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    let done = expr.eval_lambda(scope, &va).unwrap_bool();
                    v.push(va);
                    if done {
                        break;
                    }
                }
                Ok((Value::Seq(v.into()), input))
            }
            Decoder::RepeatUntilSeq(expr, a) => {
                let mut input = input;
                let mut v = Vec::new();
                loop {
                    let (va, next_input) = a.parse(program, scope, input)?;
                    input = next_input;
                    v.push(va);
                    let vs = Value::Seq(v.into());
                    let done = expr.eval_lambda(scope, &vs).unwrap_bool();
                    v = match vs {
                        Value::Seq(v) => v.into_vec(),
                        _ => unreachable!(),
                    };
                    if done {
                        break;
                    }
                }
                Ok((Value::Seq(v.into()), input))
            }
            Decoder::AccumUntil(f_done, f_update, init, _vt, a) => {
                let mut input = input;
                let mut v = Vec::new();
                let mut accum = init.eval_value(scope);
                loop {
                    let done_arg = Value::Tuple(vec![accum.clone(), Value::Seq(v.clone().into())]);
                    let is_done = f_done.eval_lambda(&scope, &done_arg).unwrap_bool();
                    if is_done {
                        break;
                    }
                    let (next_elem, next_input) = a.parse(program, scope, input)?;
                    v.push(next_elem.clone());
                    let update_arg = Value::Tuple(vec![accum.clone(), next_elem]);
                    let next_accum = f_update.eval_lambda(scope, &update_arg);
                    accum = next_accum;
                    input = next_input;
                }
                Ok((Value::Tuple(vec![accum, Value::Seq(v.into())]), input))
            }
            Decoder::Peek(a) => {
                let (v, _next_input) = a.parse(program, scope, input)?;
                Ok((v, input))
            }
            Decoder::PeekNot(a) => {
                if a.parse(program, scope, input).is_ok() {
                    Err(DecodeError::<Value>::fail(scope, input))
                } else {
                    Ok((Value::Tuple(vec![]), input))
                }
            }
            Decoder::Slice(expr, a) => {
                let size = expr.eval_value(scope).unwrap_usize();
                let (slice, input) = input
                    .split_at(size)
                    .ok_or(DecodeError::overrun(size, input.offset))?;
                let (v, _) = a.parse(program, scope, slice)?;
                Ok((v, input))
            }
            Decoder::Bits(a) => {
                let mut bits = Vec::with_capacity(input.remaining().len() * 8);
                for b in input.remaining() {
                    for i in 0..8 {
                        bits.push((b & (1 << i)) >> i);
                    }
                }
                let (v, bits) = a.parse(program, scope, ReadCtxt::new(&bits))?;
                let bytes_remain = bits.remaining().len() >> 3;
                let bytes_read = input.remaining().len() - bytes_remain;
                let (_, input) = input
                    .split_at(bytes_read)
                    .ok_or(DecodeError::overrun(bytes_read, input.offset))?;
                Ok((v, input))
            }
            Decoder::WithRelativeOffset(base_addr, expr, a) => {
                let base = base_addr.eval_value(scope).unwrap_usize();
                let offset = expr.eval_value(scope).unwrap_usize();
                let abs_offset = base + offset;
                let seek_input = input
                    .seek_to(abs_offset)
                    .ok_or(DecodeError::bad_seek(abs_offset, input.input.len()))?;
                let (v, _) = a.parse(program, scope, seek_input)?;
                Ok((v, input))
            }
            Decoder::Map(d, expr) => {
                let (orig, input) = d.parse(program, scope, input)?;
                let v = expr.eval_lambda(scope, &orig);
                Ok((Value::Mapped(Box::new(orig), Box::new(v)), input))
            }
            Decoder::Where(d, expr) => {
                let (v, input) = d.parse(program, scope, input)?;
                match expr.eval_lambda(scope, &v).unwrap_bool() {
                    true => Ok((v, input)),
                    false => Err(DecodeError::bad_where(scope, *expr.clone(), v)),
                }
            }
            Decoder::Compute(expr) => {
                let v = expr.eval_value(scope);
                Ok((v, input))
            }
            Decoder::Let(name, expr, d) => {
                let v = expr.eval_value(scope);
                let let_scope = SingleScope::new(scope, name, &v);
                d.parse(program, &Scope::Single(let_scope), input)
            }
            Decoder::Match(head, branches) => {
                let head = head.eval(scope);
                for (index, (pattern, decoder)) in branches.iter().enumerate() {
                    if let Some(pattern_scope) = head.matches(scope, pattern) {
                        let (v, input) =
                            decoder.parse(program, &Scope::Multi(&pattern_scope), input)?;
                        return Ok((Value::Branch(index, Box::new(v)), input));
                    }
                }
                panic!(
                    "non-exhaustive patterns: {head:?} not in {:#?}",
                    branches.iter().map(|(p, _)| p).collect::<Vec<_>>()
                );
            }
            Decoder::Dynamic(name, DynFormat::Huffman(lengths_expr, opt_values_expr), d) => {
                let lengths_val = lengths_expr.eval(scope);
                let lengths = value_to_vec_usize(lengths_val.as_ref());
                let lengths = match opt_values_expr {
                    None => lengths,
                    Some(e) => {
                        let values = value_to_vec_usize(e.eval(scope).as_ref());
                        let mut new_lengths = [0].repeat(values.len());
                        for i in 0..lengths.len() {
                            new_lengths[values[i]] = lengths[i];
                        }
                        new_lengths
                    }
                };
                let f = make_huffman_codes(&lengths);
                let dyn_d = Compiler::compile_one(&f).unwrap();
                let child_scope = DecoderScope::new(scope, name, dyn_d);
                d.parse(program, &Scope::Decoder(child_scope), input)
            }
            Decoder::Apply(name) => {
                let d = scope.get_decoder_by_name(name);
                d.parse(program, scope, input)
            }
            Decoder::LiftedOption(None) => Ok((Value::Option(None), input)),
            Decoder::LiftedOption(Some(dec)) => {
                let (v, input) = dec.parse(program, scope, input)?;
                Ok((Value::Option(Some(Box::new(v))), input))
            }
        }
    }
}

fn value_to_vec_usize(v: &Value) -> Vec<usize> {
    let vs = match v {
        Value::Seq(vs) => vs,
        _ => panic!("expected Seq"),
    };
    vs.iter()
        .map(|v| match v.coerce_mapped_value() {
            Value::U8(n) => *n as usize,
            _ => panic!("expected U8"),
        })
        .collect::<Vec<usize>>()
}

fn make_huffman_codes(lengths: &[usize]) -> Format {
    let max_length = *lengths.iter().max().unwrap();
    let mut bl_count = [0].repeat(max_length + 1);

    for len in lengths {
        bl_count[*len] += 1;
    }

    let mut next_code = [0].repeat(max_length + 1);
    let mut code = 0;
    bl_count[0] = 0;

    for bits in 1..max_length + 1 {
        code = (code + bl_count[bits - 1]) << 1;
        next_code[bits] = code;
    }

    let mut codes = Vec::with_capacity(lengths.len());

    for (n, &len) in lengths.iter().enumerate() {
        if len != 0 {
            codes.push(Format::Map(
                Box::new(bit_range(len, next_code[len])),
                Box::new(Expr::Lambda(
                    "_".into(),
                    Box::new(Expr::U16(n.try_into().unwrap())),
                )),
            ));
            //println!("{:?}", codes[codes.len()-1]);
            next_code[len] += 1;
        } else {
            //codes.push((n.to_string(), Format::Fail));
        }
    }

    Format::Union(codes)
}

fn bit_range(n: usize, bits: usize) -> Format {
    let mut fs = Vec::with_capacity(n);
    for i in 0..n {
        let r = n - 1 - i;
        let b = (bits & (1 << r)) >> r != 0;
        fs.push(is_bit(b));
    }
    Format::Tuple(fs)
}

fn is_bit(b: bool) -> Format {
    Format::Byte(ByteSet::from([if b { 1 } else { 0 }]))
}

/// Applies a lifetime-preserving transformation to a reference held behind a [`std::borrow::Cow`],
/// referencing when possible and forcing ownership only when necessary.
pub(crate) fn cow_map<'a, T, U>(x: Cow<'a, T>, f: impl for<'i> Fn(&'i T) -> &'i U) -> Cow<'a, U>
where
    T: 'static + Clone,
    U: 'static + Clone + ToOwned,
{
    match x {
        Cow::Borrowed(x) => Cow::Borrowed(f(x)),
        Cow::Owned(x) => Cow::Owned(f(&x).to_owned()),
    }
}

/// Like [`cow_map`], but applies a closure argument `f` that itself returns a `Cow<'i, U>` value (instead of a `&'i U` value)
pub(crate) fn cow_remap<'a, T, U>(
    x: Cow<'a, T>,
    f: impl for<'i> Fn(&'i T) -> Cow<'i, U>,
) -> Cow<'a, U>
where
    T: 'static + Clone,
    U: 'static + Clone + ToOwned,
{
    match x {
        Cow::Borrowed(x) => f(x),
        Cow::Owned(x) => Cow::Owned(f(&x).into_owned()),
    }
}

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use super::*;
    use crate::helper::*;

    fn accepts(d: &Decoder, input: &[u8], tail: &[u8], expect: Value) {
        let program = Program::new();
        let (val, remain) = d
            .parse(&program, &Scope::Empty, ReadCtxt::new(input))
            .unwrap();
        assert_eq!(val, expect);
        assert_eq!(remain.remaining(), tail);
    }

    fn rejects(d: &Decoder, input: &[u8]) {
        let program = Program::new();
        assert!(d
            .parse(&program, &Scope::Empty, ReadCtxt::new(input))
            .is_err());
    }

    fn value_some(x: Value) -> Value {
        Value::Option(Some(Box::new(x)))
    }

    #[test]
    fn compile_fail() {
        let f = Format::Fail;
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_empty() {
        let f = Format::EMPTY;
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::UNIT);
        accepts(&d, &[0x00], &[0x00], Value::UNIT);
    }

    #[test]
    fn compile_byte_is() {
        let f = is_byte(0x00);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[0x00], &[], Value::U8(0));
        accepts(&d, &[0x00, 0xFF], &[0xFF], Value::U8(0));
        rejects(&d, &[0xFF]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_byte_not() {
        let f = not_byte(0x00);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[0xFF], &[], Value::U8(0xFF));
        accepts(&d, &[0xFF, 0x00], &[0x00], Value::U8(0xFF));
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt() {
        let f = alts::<&str>([]);
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
    }

    #[test]
    fn compile_alt_byte() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0xFF))]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0xFF)))),
        );
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_ambiguous() {
        let f = alts([("a", is_byte(0x00)), ("b", is_byte(0x00))]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_byte() {
        let slice_a = slice(Expr::U8(1), is_byte(0x00));
        let slice_b = slice(Expr::U8(1), is_byte(0xFF));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0xFF)))),
        );
        rejects(&d, &[0x11]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_slice_ambiguous1() {
        let slice_a = slice(Expr::U8(1), is_byte(0x00));
        let slice_b = slice(Expr::U8(1), is_byte(0x00));
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_slice_ambiguous2() {
        let tuple_a = Format::Tuple(vec![is_byte(0x00), is_byte(0x00)]);
        let tuple_b = Format::Tuple(vec![is_byte(0x00), is_byte(0xFF)]);
        let slice_a = slice(Expr::U8(1), tuple_a);
        let slice_b = slice(Expr::U8(1), tuple_b);
        let f = alts([("a", slice_a), ("b", slice_b)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail() {
        let f = alts([("a", Format::Fail), ("b", Format::Fail)]);
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_end_of_input() {
        let f = alts([("a", Format::EndOfInput), ("b", Format::EndOfInput)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_empty() {
        let f = alts([("a", Format::EMPTY), ("b", Format::EMPTY)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_fail_end_of_input() {
        let f = alts([("a", Format::Fail), ("b", Format::EndOfInput)]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::UNIT))),
        );
    }

    #[test]
    fn compile_alt_end_of_input_or_byte() {
        let f = alts([("a", Format::EndOfInput), ("b", is_byte(0x00))]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::UNIT))),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[0x00, 0x00],
            &[0x00],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0x00)))),
        );
        rejects(&d, &[0x11]);
    }

    #[test]
    fn compile_alt_opt() {
        let f = alts([("a", Format::EMPTY), ("b", is_byte(0x00))]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Branch(1, Box::new(Value::variant("b", Value::U8(0x00)))),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(0, Box::new(Value::variant("a", Value::UNIT))),
        );
        accepts(
            &d,
            &[0xFF],
            &[0xFF],
            Value::Branch(0, Box::new(Value::variant("a", Value::UNIT))),
        );
    }

    #[test]
    fn compile_alt_opt_next() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), is_byte(0xFF)]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(value_some(Value::U8(0)))),
                Value::U8(0xFF),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::U8(0xFF),
            ]),
        );
        rejects(&d, &[0x00]);
        rejects(&d, &[]);
    }

    #[test]
    fn compile_alt_opt_opt() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0xFF))]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(value_some(Value::U8(0)))),
                Value::Branch(0, Box::new(value_some(Value::U8(0xFF)))),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![
                Value::Branch(0, Box::new(value_some(Value::U8(0)))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(0, Box::new(value_some(Value::U8(0xFF)))),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![
                Value::Branch(1, Box::new(Value::Option(None))),
                Value::Branch(1, Box::new(Value::Option(None))),
            ]),
        );
    }

    #[test]
    fn compile_alt_opt_ambiguous() {
        let f = Format::Tuple(vec![optional(is_byte(0x00)), optional(is_byte(0x00))]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_alt_opt_ambiguous_slow() {
        let alt = alts([
            ("0x00", is_byte(0x00)),
            ("0x01", is_byte(0x01)),
            ("0x02", is_byte(0x02)),
            ("0x03", is_byte(0x03)),
            ("0x04", is_byte(0x04)),
            ("0x05", is_byte(0x05)),
            ("0x06", is_byte(0x06)),
            ("0x07", is_byte(0x07)),
        ]);
        let rec = record([
            ("0", alt.clone()),
            ("1", alt.clone()),
            ("2", alt.clone()),
            ("3", alt.clone()),
            ("4", alt.clone()),
            ("5", alt.clone()),
            ("6", alt.clone()),
            ("7", alt.clone()),
        ]);
        let f = alts([("a", rec.clone()), ("b", rec.clone())]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_alt_repeat1_slow() {
        let f = repeat(alts([
            ("a", repeat1(is_byte(0x00))),
            ("b", is_byte(0x01)),
            ("c", is_byte(0x02)),
        ]));
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat() {
        let f = repeat(is_byte(0x00));
        let d = Compiler::compile_one(&f).unwrap();
        accepts(&d, &[], &[], Value::Seq(vec![].into()));
        accepts(&d, &[0xFF], &[0xFF], Value::Seq(vec![].into()));
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)].into()));
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)].into()),
        );
    }

    #[test]
    fn compile_repeat_repeat() {
        let f = repeat(repeat(is_byte(0x00)));
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_cat_repeat() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0xFF))]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![].into()), Value::Seq(vec![].into())]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)].into()),
                Value::Seq(vec![].into()),
            ]),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![].into()),
                Value::Seq(vec![Value::U8(0xFF)].into()),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)].into()),
                Value::Seq(vec![Value::U8(0xFF)].into()),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[0x00],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00)].into()),
                Value::Seq(vec![Value::U8(0xFF)].into()),
            ]),
        );
        accepts(
            &d,
            &[0x7F],
            &[0x7F],
            Value::Tuple(vec![Value::Seq(vec![].into()), Value::Seq(vec![].into())]),
        );
    }

    #[test]
    fn compile_cat_end_of_input() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::EndOfInput]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT]),
        );
        rejects(&d, &[]);
        rejects(&d, &[0x00, 0x00]);
    }

    #[test]
    fn compile_cat_repeat_end_of_input() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), Format::EndOfInput]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Tuple(vec![Value::Seq(vec![].into()), Value::UNIT]),
        );
        accepts(
            &d,
            &[0x00, 0x00, 0x00],
            &[],
            Value::Tuple(vec![
                Value::Seq(vec![Value::U8(0x00), Value::U8(0x00), Value::U8(0x00)].into()),
                Value::UNIT,
            ]),
        );
        rejects(&d, &[0x00, 0x10]);
    }

    #[test]
    fn compile_cat_repeat_ambiguous() {
        let f = Format::Tuple(vec![repeat(is_byte(0x00)), repeat(is_byte(0x00))]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x7F))),
        ]);
        assert!(Compiler::compile_one(&f).is_ok());
    }

    #[test]
    fn compile_repeat_fields_ambiguous() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            ("second", repeat(is_byte(0xFF))),
            ("third", repeat(is_byte(0x00))),
        ]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_fields_okay() {
        let f = record([
            ("first", repeat(is_byte(0x00))),
            (
                "second-and-third",
                optional(record([
                    (
                        "second",
                        Format::Tuple(vec![is_byte(0xFF), repeat(is_byte(0xFF))]),
                    ),
                    ("third", repeat(is_byte(0x00))),
                ])),
            ),
        ]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::record([
                ("first", Value::Seq(vec![].into())),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::Option(None))),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::Option(None))),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::record([
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(
                        0,
                        Box::new(value_some(Value::record([
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![].into())]),
                            ),
                            ("third", Value::Seq(vec![].into())),
                        ]))),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0xFF, 0x00],
            &[],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(
                        0,
                        Box::new(value_some(Value::record(vec![
                            (
                                "second",
                                Value::Tuple(vec![Value::U8(0xFF), Value::Seq(vec![].into())]),
                            ),
                            ("third", Value::Seq(vec![Value::U8(0x00)].into())),
                        ]))),
                    ),
                ),
            ]),
        );
        accepts(
            &d,
            &[0x00, 0x7F],
            &[0x7F],
            Value::record(vec![
                ("first", Value::Seq(vec![Value::U8(0x00)].into())),
                (
                    "second-and-third",
                    Value::Branch(1, Box::new(Value::Option(None))),
                ),
            ]),
        );
    }

    #[test]
    fn compile_repeat1() {
        let f = repeat1(is_byte(0x00));
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0xFF]);
        accepts(&d, &[0x00], &[], Value::Seq(vec![Value::U8(0x00)].into()));
        accepts(
            &d,
            &[0x00, 0xFF],
            &[0xFF],
            Value::Seq(vec![Value::U8(0x00)].into()),
        );
        accepts(
            &d,
            &[0x00, 0x00],
            &[],
            Value::Seq(vec![Value::U8(0x00), Value::U8(0x00)].into()),
        );
    }

    #[test]
    fn compile_align1() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(1), is_byte(0xFF)]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_align2() {
        let f = Format::Tuple(vec![is_byte(0x00), Format::Align(2), is_byte(0xFF)]);
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[0x00, 0xFF]);
        rejects(&d, &[0x00, 0x99, 0x99, 0xFF]);
        accepts(
            &d,
            &[0x00, 0x99, 0xFF],
            &[],
            Value::Tuple(vec![Value::U8(0x00), Value::UNIT, Value::U8(0xFF)]),
        );
    }

    #[test]
    fn compile_peek_not() {
        let any_byte = Format::Byte(ByteSet::full());
        let a = Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)]);
        let peek_not = Format::PeekNot(Box::new(a));
        let f = Format::Tuple(vec![peek_not, any_byte.clone(), any_byte.clone()]);
        let d = Compiler::compile_one(&f).unwrap();
        rejects(&d, &[]);
        rejects(&d, &[0xFF]);
        rejects(&d, &[0xFF, 0xFF]);
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Tuple(vec![Value::Tuple(vec![]), Value::U8(0x00), Value::U8(0xFF)]),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Tuple(vec![Value::Tuple(vec![]), Value::U8(0xFF), Value::U8(0x00)]),
        );
    }

    #[test]
    fn compile_peek_not_switch() {
        let any_byte = Format::Byte(ByteSet::full());
        let guard = Format::PeekNot(Box::new(Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)])));
        let a = Format::Tuple(vec![guard, Format::Repeat(Box::new(any_byte.clone()))]);
        let b = Format::Tuple(vec![is_byte(0xFF), is_byte(0xFF)]);
        let f = alts([("a", a), ("b", b)]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![].into()),
                    ])),
                )),
            ),
        );
        accepts(
            &d,
            &[0xFF],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![Value::U8(0xFF)].into()),
                    ])),
                )),
            ),
        );
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![Value::U8(0x00), Value::U8(0xFF)].into()),
                    ])),
                )),
            ),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Branch(
                0,
                Box::new(Value::Variant(
                    "a".into(),
                    Box::new(Value::Tuple(vec![
                        Value::Tuple(vec![]),
                        Value::Seq(vec![Value::U8(0xFF), Value::U8(0x00)].into()),
                    ])),
                )),
            ),
        );
        accepts(
            &d,
            &[0xFF, 0xFF],
            &[],
            Value::Branch(
                1,
                Box::new(Value::Variant(
                    "b".into(),
                    Box::new(Value::Tuple(vec![Value::U8(0xFF), Value::U8(0xFF)])),
                )),
            ),
        );
    }

    #[test]
    fn compile_peek_not_lookahead() {
        let peek_not = Format::PeekNot(Box::new(repeat1(is_byte(0x00))));
        let any_byte = Format::Byte(ByteSet::full());
        let f = Format::Tuple(vec![peek_not, repeat1(any_byte)]);
        assert!(Compiler::compile_one(&f).is_err());
    }

    #[test]
    fn compile_repeat_between() {
        let repeat_between = Format::RepeatBetween(
            Box::new(Expr::U16(0u16)),
            Box::new(Expr::U16(2u16)),
            Box::new(is_byte(0)),
        );
        let trailer = is_byte(1);
        let f = Format::Tuple(vec![repeat_between, trailer]);
        assert!(Compiler::compile_one(&f).is_ok());
    }

    #[test]
    #[ignore] // TODO can we distinguish a Union based on disjoint Where clauses?
    fn compile_where_u16be_eq() {
        let u8 = Format::Byte(ByteSet::full());
        let u16be = map(
            tuple([u8.clone(), u8]),
            lambda("x", Expr::U16Be(Box::new(var("x")))),
        );
        let a = Format::Where(
            Box::new(u16be.clone()),
            Box::new(lambda("x", expr_eq(var("x"), Expr::U16(0x00FF)))),
        );
        let b = Format::Where(
            Box::new(u16be),
            Box::new(lambda("x", expr_eq(var("x"), Expr::U16(0xFF00)))),
        );
        let f = Format::Union(vec![a, b]);
        let d = Compiler::compile_one(&f).unwrap();
        accepts(
            &d,
            &[0x00, 0xFF],
            &[],
            Value::Branch(0, Box::new(Value::U16(0x00FF))),
        );
        accepts(
            &d,
            &[0xFF, 0x00],
            &[],
            Value::Branch(0, Box::new(Value::U16(0xFF00))),
        );
        rejects(&d, &[]);
        rejects(&d, &[0x00]);
        rejects(&d, &[0xFF]);
        rejects(&d, &[0x00, 0x00]);
        rejects(&d, &[0xFF, 0xFF]);
    }

    #[test]
    fn branch_value_record_revamp() {
        let f = Format::record([("inner", Format::Union(vec![is_byte(0), is_byte(1)]))]);
        let d = Compiler::compile_one(&f).unwrap();

        accepts(
            &d,
            &[0x00],
            &[],
            Value::Record(vec![(
                Label::Borrowed("inner"),
                Value::Branch(0, Box::new(Value::U8(0))),
            )]),
        );
    }
}
