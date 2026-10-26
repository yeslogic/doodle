#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::iter::repeat_n;
use std::num::NonZeroUsize;
use std::rc::Rc;

use anyhow::{Result as AResult, anyhow};
use codegen::typed_format::{GenType, TypedFormat};
use serde::Serialize;

use crate::bounds::Bounds;
use crate::byte_set::ByteSet;
use crate::numeric::core::{Expr as NumExpr, VOID};
use crate::read::ReadCtxt;
use crate::valuetype::BaseNumType;

pub mod alt;
pub mod bounds;
pub mod byte_set;
pub mod codegen;
pub mod decoder;

pub mod dep_ref;
pub(crate) mod fixed;
pub use dep_ref::DepFormat;

pub mod error;
pub mod helper;
pub mod loc_decoder;
pub mod marker;
pub use marker::{BaseKind, Endian, FixedReadKind};
pub mod numeric;
pub mod output;
pub mod parser;
mod precedence;
pub mod prelude;
pub mod read;
pub(crate) mod recursion;

pub mod scope;

mod typecheck;
pub use typecheck::{TCResult, base_set, error::TCError, typecheck};
mod util;
pub(crate) use util::IxHeap;

pub(crate) mod validation;

pub type Label = std::borrow::Cow<'static, str>;

pub trait IntoLabel: Into<Label> {}

impl<T> IntoLabel for T where T: Into<Label> {}

pub(crate) mod pattern;
pub use pattern::Pattern;

#[derive(Debug, Clone)]
pub enum ValueKind<T> {
    /// A local 'view' of an input buffer (e.g. `ReadScope`).
    View,
    Value(T),
    Format(T),
}

pub(crate) mod valuetype;
pub use valuetype::{BaseType, TypeHint, ValueType};

fn mk_value_expr(vt: &ValueType) -> Option<Expr> {
    match vt {
        ValueType::Any | ValueType::Empty => None,
        ValueType::ViewObj | ValueType::PhantomData(..) => None,
        ValueType::Base(b) => Some(match b {
            BaseType::Bool => Expr::Bool(false),
            BaseType::U8 => Expr::U8(0),
            BaseType::U16 => Expr::U16(0),
            BaseType::U32 => Expr::U32(0),
            BaseType::U64 => Expr::U64(0),
            BaseType::Char => Expr::AsChar(Box::new(Expr::U32(0))),
        }),
        ValueType::NumericHole => Some(Expr::Numeric(Box::new(NumExpr::Const(
            numeric::TypedConst::from_u8(0),
        )))),
        ValueType::Signed(t) => Some(Expr::Numeric(Box::new(NumExpr::Const(
            numeric::TypedConst::new(-1, (*t).into()),
        )))),
        ValueType::Tuple(ts) => {
            let mut xs = Vec::with_capacity(ts.len());
            for t in ts {
                xs.push(mk_value_expr(t)?);
            }
            Some(Expr::Tuple(xs))
        }
        ValueType::Record(fs) => {
            let mut xs = Vec::with_capacity(fs.len());
            for (lbl, t) in fs {
                xs.push((lbl.clone(), mk_value_expr(t)?));
            }
            Some(Expr::Record(xs))
        }
        ValueType::Union(branches) => {
            let (lbl, branch) = branches.first_key_value()?;
            Some(Expr::Variant(lbl.clone(), Box::new(mk_value_expr(branch)?)))
        }
        ValueType::Seq(t) => Some(Expr::Seq(vec![mk_value_expr(t.as_ref())?])),
        ValueType::Option(t) => Some(Expr::LiftOption(Some(Box::new(mk_value_expr(t)?)))),
        ValueType::Rec(_) => None,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum IntRel {
    Eq,
    Ne,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum Arith {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    Shl,
    Shr,
    BoolOr,
    BoolAnd,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum UnaryOp {
    /// BoolNot models the Rust-native operation `!`
    BoolNot,
    /// The successor function over any integral type (behavior unspecified when applied to max_val)
    IntSucc,
    /// The predecessor function over any integral type (behavior unspecified when applied to min_val)
    IntPred,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Expr {
    Var(Label),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Tuple(Vec<Expr>),
    TupleProj(Box<Expr>, usize),
    Record(Vec<(Label, Expr)>),
    RecordProj(Box<Expr>, Label),
    Variant(Label, Box<Expr>),
    Seq(Vec<Expr>),

    Numeric(Box<NumExpr>),

    Match(Box<Expr>, Vec<(Pattern, Expr)>),
    Destructure(Box<Expr>, Pattern, Box<Expr>),

    Lambda(Label, Box<Expr>),

    IntRel(IntRel, Box<Expr>, Box<Expr>),
    Arith(Arith, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),

    AsU8(Box<Expr>),
    AsU16(Box<Expr>),
    AsU32(Box<Expr>),
    AsU64(Box<Expr>),
    AsChar(Box<Expr>),

    U16Be(Box<Expr>),
    U16Le(Box<Expr>),
    U32Be(Box<Expr>),
    U32Le(Box<Expr>),
    U64Be(Box<Expr>),
    U64Le(Box<Expr>),

    SeqLength(Box<Expr>),
    /// SubSeq :: \[T\] -> start:U32 -> length:U32 -> \[T\] (start >= 0, length >= 0, start + length <= length of sequence)
    SubSeq(Box<Expr>, Box<Expr>, Box<Expr>),
    /// SeqIx :: \[T\] -> ix:U32 -> T (panic on unguarded OOB index)
    SeqIx(Box<Expr>, Box<Expr>),
    /// SubSeqInflate :: \[T\] -> start:U32 -> length:U32 -> \[T\] (start >= 0, length >= 0)
    SubSeqInflate(Box<Expr>, Box<Expr>, Box<Expr>),
    /// FlatMap :: (T -> \[U\]) -> \[T\] -> \[U\]
    FlatMap(Box<Expr>, Box<Expr>),
    /// FlatMapAccum :: ((Acc, T) -> (Acc, \[U\])) -> Acc -> TypeRep Acc -> \[T\] -> \[U\]
    FlatMapAccum(Box<Expr>, Box<Expr>, TypeHint, Box<Expr>),
    /// FlatMapList :: ((\[U\], T) -> \[U\]) -> TypeRep U -> \[T\] -> \[U\] (lambda call yields a run of values to append to the final list)
    FlatMapList(Box<Expr>, TypeHint, Box<Expr>),
    /// FindByKey :: (is-sorted) -> (T -> K) -> K -> \[T\] ->  Option(T) (is-sorted indicates whether the values are sorted w.r.t. the keying function in question; only simple types can be keys for now)
    FindByKey(bool, Box<Expr>, Box<Expr>, Box<Expr>),

    /// EnumFromTo :: (Num, Num) -> \[Num\]
    EnumFromTo(Box<Expr>, Box<Expr>),
    /// LeftFold :: ((Acc, T) -> Acc) -> Acc -> \[T\] -> Acc
    LeftFold(Box<Expr>, Box<Expr>, TypeHint, Box<Expr>),

    /// `Dup :: U32 -> T -> [T]`
    Dup(Box<Expr>, Box<Expr>),

    LiftOption(Option<Box<Expr>>),
    /// `[T] -> [T] -> [T]` (append operation)
    Append(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub const UNIT: Self = Expr::Tuple(Vec::new());
    pub const NIL: Self = Expr::Seq(Vec::new());

    pub fn record_proj(head: impl Into<Box<Expr>>, label: impl IntoLabel) -> Expr {
        let head: Box<Expr> = head.into();
        let label: Label = label.into();

        Expr::RecordProj(head, label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjKind {
    Record,
    Tuple,
}

impl Expr {
    // FIXME: is this still an inherent method, or should we have a UD -> TC phase and use get_type_info instead?
    fn infer_type(&self, scope: &TypeScope<'_, ValueType>) -> AResult<ValueType> {
        match self {
            Expr::Var(name) => match scope.get_type_by_name(name) {
                ValueKind::Value(t) => Ok(scope.unfold(t.clone())),
                ValueKind::View => Err(anyhow!(
                    "expected ValueKind::Value, found ValueKind::View for var {name}"
                )),
                ValueKind::Format(_t) => Err(anyhow!(
                    "expected ValueKind::Value, found ValueKind::Format for var {name}"
                )),
            },
            Expr::Bool(_b) => Ok(ValueType::Base(BaseType::Bool)),
            Expr::U8(_n) => Ok(ValueType::U8),
            Expr::U16(_n) => Ok(ValueType::U16),
            Expr::U32(_n) => Ok(ValueType::U32),
            Expr::U64(_n) => Ok(ValueType::U64),
            Expr::Numeric(n_tree) => n_tree.infer_type(scope),
            Expr::Tuple(exprs) => {
                let mut ts = Vec::new();
                for expr in exprs {
                    ts.push(expr.infer_type(scope)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            Expr::TupleProj(head, index) => match head.infer_type(scope)? {
                ValueType::Tuple(vs) => Ok(scope.unfold(vs[*index].clone())),
                other => Err(anyhow!("tuple projection on non-tuple type {other:?}")),
            },
            Expr::Record(fields) => {
                let mut fs = Vec::new();
                for (label, expr) in fields {
                    fs.push((label.clone(), expr.infer_type(scope)?));
                }
                Ok(ValueType::Record(fs))
            }
            Expr::RecordProj(head, label) => {
                Ok(scope.unfold(head.infer_type(scope)?.record_proj(label)))
            }
            Expr::Variant(label, expr) => Ok(ValueType::Union(BTreeMap::from([(
                label.clone(),
                expr.infer_type(scope)?,
            )]))),
            Expr::Seq(exprs) => {
                let mut t = ValueType::Any;
                for e in exprs {
                    t = scope.unify(&t, &e.infer_type(scope)?)?;
                }
                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::Match(head, branches) => {
                if branches.is_empty() {
                    return Err(anyhow!("cannot infer type of empty match expression"));
                }
                let head_type = Rc::new(head.infer_type(scope)?);
                let mut t = ValueType::Any;
                for (pattern, branch) in branches {
                    t = scope.unify(
                        &t,
                        &pattern.infer_expr_branch_type(scope, head_type.clone(), branch)?,
                    )?;
                }
                Ok(t)
            }
            Expr::Destructure(head, pattern, body) => {
                let head_type = Rc::new(head.infer_type(scope)?);
                pattern.infer_expr_branch_type(scope, head_type, body)
            }
            Expr::Lambda(..) => Err(anyhow!("infer_type encountered unexpected lambda")),

            Expr::IntRel(_rel, x, y) => {
                let (t0, t1) = (x.infer_type(scope)?, y.infer_type(scope)?);
                if scope
                    .unify(&t0, &t1)
                    .as_ref()
                    .is_ok_and(ValueType::is_numeric)
                {
                    Ok(ValueType::Base(BaseType::Bool))
                } else {
                    Err(anyhow!(
                        "mismatched operand types for {_rel:?}: {t0:?}, {t1:?}"
                    ))
                }
            }
            Expr::Arith(_arith @ (Arith::BoolAnd | Arith::BoolOr), x, y) => {
                match (x.infer_type(scope)?, y.infer_type(scope)?) {
                    (ValueType::Base(BaseType::Bool), ValueType::Base(BaseType::Bool)) => {
                        Ok(ValueType::Base(BaseType::Bool))
                    }
                    (x, y) => Err(anyhow!(
                        "mismatched operand types for {_arith:?}: {x:?}, {y:?}"
                    )),
                }
            }

            Expr::Arith(_arith, x, y) => {
                let (t1, t2) = (x.infer_type(scope)?, y.infer_type(scope)?);
                scope
                    .unify(&t1, &t2)
                    .map_err(|e| anyhow!("{e}"))
                    .and_then(|t0| {
                        // NOTE - signed arithmetic is only supported within `Expr::Numeric`
                        if t0.is_unsigned_or_auto() {
                            Ok(t0)
                        } else {
                            Err(anyhow!(
                                "mismatched operand types for {_arith:?}: {t1:?}, {t2:?}"
                            ))
                        }
                    })
            }
            Expr::Unary(_op @ UnaryOp::BoolNot, x) => match x.infer_type(scope)? {
                ValueType::Base(BaseType::Bool) => Ok(ValueType::Base(BaseType::Bool)),
                x => Err(anyhow!("unexpected operand type for {_op:?}: {x:?}")),
            },
            Expr::Unary(_op @ (UnaryOp::IntSucc | UnaryOp::IntPred), x) => {
                let t = x.infer_type(scope)?;
                if t.is_unsigned_or_auto() {
                    Ok(t)
                } else {
                    Err(anyhow!("unexpected operand type for {_op:?}: {t:?}"))
                }
            }
            Expr::AsU8(x) => match x.infer_type(scope)? {
                t if t.is_numeric() => Ok(ValueType::U8),
                x => Err(anyhow!("unsound type cast AsU8(_ : {x:?})")),
            },
            Expr::AsU16(x) => match x.infer_type(scope)? {
                t if t.is_numeric() => Ok(ValueType::U16),
                x => Err(anyhow!("unsound type cast AsU16(_ : {x:?})")),
            },
            Expr::AsU32(x) => match x.infer_type(scope)? {
                t if t.is_numeric() => Ok(ValueType::U32),
                x => Err(anyhow!("unsound type cast AsU32(_ : {x:?})")),
            },
            Expr::AsU64(x) => match x.infer_type(scope)? {
                t if t.is_numeric() => Ok(ValueType::U64),
                x => Err(anyhow!("cannot convert {x:?} to U64")),
            },
            // NOTE - unlike AsU8..AsU64, signed operands are rejected; use `AsChar(AsU32(x))` instead
            Expr::AsChar(x) => match x.infer_type(scope)? {
                t if t.is_unsigned_or_auto() => Ok(ValueType::Base(BaseType::Char)),
                x => Err(anyhow!("unsound type cast AsChar(_ : {x:?})")),
            },
            Expr::U16Be(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.try_as_tuple_type() {
                    Ok([ValueType::U8, ValueType::U8]) => Ok(ValueType::U16),
                    _ => Err(anyhow!("unsound byte-level type cast U16Be(_ : {_t:?})")),
                }
            }
            Expr::U16Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.try_as_tuple_type() {
                    Ok([ValueType::U8, ValueType::U8]) => Ok(ValueType::U16),
                    _ => Err(anyhow!("unsound byte-level type cast U16Le(_ : {_t:?})")),
                }
            }
            Expr::U32Be(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.try_as_tuple_type() {
                    Ok([ValueType::U8, ValueType::U8, ValueType::U8, ValueType::U8]) => {
                        Ok(ValueType::U32)
                    }
                    _ => Err(anyhow!("unsound byte-level type cast U32Be(_ : {_t:?})")),
                }
            }
            Expr::U32Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.try_as_tuple_type() {
                    Ok([ValueType::U8, ValueType::U8, ValueType::U8, ValueType::U8]) => {
                        Ok(ValueType::U32)
                    }
                    _ => Err(anyhow!("unsound byte-level type cast U32Le(_ : {_t:?})")),
                }
            }
            Expr::U64Be(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.try_as_tuple_type() {
                    Ok(
                        [
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                        ],
                    ) => Ok(ValueType::U64),
                    _ => Err(anyhow!("unsound byte-level type cast U64Be(_ : {_t:?})")),
                }
            }
            Expr::U64Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.try_as_tuple_type() {
                    Ok(
                        [
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                            ValueType::U8,
                        ],
                    ) => Ok(ValueType::U64),
                    _ => Err(anyhow!("unsound byte-level type cast U64Le(_ : {_t:?})")),
                }
            }
            // NOTE - typed like `Format::Pos`: the interpreter yields an Auto Numeric, which takes the type of
            // whatever native operand it meets
            Expr::SeqLength(seq) => match seq.infer_type(scope)? {
                ValueType::Seq(_t) => Ok(ValueType::NumericHole),
                other => Err(anyhow!("seq-length called on non-sequence type: {other:?}")),
            },
            Expr::SeqIx(seq, index) => match seq.infer_type(scope)? {
                ValueType::Seq(t) => {
                    let index_type = index.infer_type(scope)?;
                    if !index_type.is_unsigned_or_auto() {
                        return Err(anyhow!(
                            "SeqIx `index` param: expected unsigned or auto, found {index_type:?}"
                        ));
                    }
                    Ok(scope.unfold(ValueType::clone(&t)))
                }
                other => Err(anyhow!("SeqIx: expected Seq, found {other:?}")),
            },
            Expr::SubSeq(seq, start, length) | Expr::SubSeqInflate(seq, start, length) => {
                let op = if matches!(self, Expr::SubSeq(..)) {
                    "SubSeq"
                } else {
                    "SubSeqInflate"
                };
                match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        for (what, expr) in [("start", start), ("length", length)] {
                            let t = expr.infer_type(scope)?;
                            if !t.is_unsigned_or_auto() {
                                return Err(anyhow!(
                                    "{op} `{what}` param: expected unsigned or auto, found {t:?}"
                                ));
                            }
                        }
                        Ok(ValueType::Seq(t))
                    }
                    other => Err(anyhow!("{op}: expected Seq, found {other:?}")),
                }
            }
            Expr::FlatMap(expr, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(name.clone(), *t);
                        match expr.infer_type(&child_scope)? {
                            ValueType::Seq(t2) => Ok(ValueType::Seq(t2)),
                            other => Err(anyhow!("FlatMap: expected Seq, found {other:?}")),
                        }
                    }
                    other => Err(anyhow!("FlatMap: expected Seq, found {other:?}")),
                },
                other => Err(anyhow!("FlatMap: expected Lambda, found {other:?}")),
            },
            Expr::FlatMapAccum(expr, accum, accum_type, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        let accum_type =
                            scope.unify(&accum.infer_type(scope)?, accum_type.as_ref())?;
                        let mut child_scope = TypeScope::child(scope);
                        child_scope
                            .push(name.clone(), ValueType::Tuple(vec![accum_type.clone(), *t]));
                        match expr
                            .infer_type(&child_scope)?
                            .try_into_tuple_type()?
                            .as_mut_slice()
                        {
                            [accum_result, ValueType::Seq(t2)] => {
                                scope.unify(accum_result, &accum_type)?;
                                Ok(ValueType::Seq(t2.clone()))
                            }
                            _ => Err(anyhow!("FlatMapAccum: expected two values")),
                        }
                    }
                    other => Err(anyhow!("FlatMapAccum: expected Seq, found {other:?}")),
                },
                other => Err(anyhow!("FlatMapAccum: expected Lambda, found {other:?}")),
            },
            Expr::LeftFold(expr, accum, accum_type, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        let accum_type =
                            scope.unify(&accum.infer_type(scope)?, accum_type.as_ref())?;
                        let mut child_scope = TypeScope::child(scope);
                        child_scope
                            .push(name.clone(), ValueType::Tuple(vec![accum_type.clone(), *t]));
                        Ok(scope.unify(&expr.infer_type(&child_scope)?, &accum_type)?)
                    }
                    other => Err(anyhow!("LeftFold: expected Seq, found {other:?}")),
                },
                other => Err(anyhow!("LeftFold: expected Lambda, found {other:?}")),
            },
            Expr::FindByKey(_is_sorted, get_key, query_key, seq) => match get_key.as_ref() {
                Expr::Lambda(name, expr) => match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        let key_type = query_key.infer_type(scope)?;
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(name.clone(), t.as_ref().clone());
                        match scope.unify(&expr.infer_type(&child_scope)?, &key_type)? {
                            ValueType::Base(b) if b.is_numeric() => Ok(ValueType::Option(t)),
                            other => Err(anyhow!(
                                "FindByKey: expected numeric key type, found {other:?}"
                            )),
                        }
                    }
                    other => Err(anyhow!("FindByKey: expected Seq, found {other:?}")),
                },
                other => Err(anyhow!("FindByKey: Expected Lambda, found {other:?}")),
            },
            Expr::FlatMapList(expr, ret_type, seq) => match expr.as_ref() {
                Expr::Lambda(name, expr) => match seq.infer_type(scope)? {
                    ValueType::Seq(t) => {
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(
                            name.clone(),
                            ValueType::Tuple(vec![
                                ValueType::Seq(Box::new(ret_type.as_ref().clone())),
                                *t,
                            ]),
                        );
                        match expr.infer_type(&child_scope)? {
                            ValueType::Seq(t2) => Ok(ValueType::Seq(t2)),
                            other => Err(anyhow!("FlatMapList: expected Seq, found {other:?}")),
                        }
                    }
                    other => Err(anyhow!("FlatMapList: expected Seq, found {other:?}")),
                },
                other => Err(anyhow!("FlatMapList: expected Lambda, found {other:?}")),
            },
            Expr::EnumFromTo(start, end) => {
                let start_type = start.infer_type(scope)?;
                let end_type = end.infer_type(scope)?;

                let Ok(t) = scope.unify(&start_type, &end_type) else {
                    return Err(anyhow!(
                        "EnumFromTo: start and end do not agree: {start_type:?} ∩ {end_type:?} = ∅"
                    ));
                };

                // NOTE - checked after unification so that a signed bound cannot hide behind an Auto one
                if !t.is_unsigned_or_auto() {
                    return Err(anyhow!(
                        "EnumFromTo: bounds must be unsigned or auto, found {t:?}"
                    ));
                }

                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::Dup(count, expr) => {
                let count_type = count.infer_type(scope)?;
                if !count_type.is_unsigned_or_auto() {
                    return Err(anyhow!(
                        "Dup: count should be unsigned or auto, found {count_type:?}"
                    ));
                }
                let t = expr.infer_type(scope)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::LiftOption(expr) => match expr {
                Some(expr) => Ok(ValueType::Option(Box::new(expr.infer_type(scope)?))),
                None => Ok(ValueType::Option(Box::new(ValueType::Any))),
            },
            Expr::Append(lhs, rhs) => {
                let lhs_type = lhs.infer_type(scope)?;
                let rhs_type = rhs.infer_type(scope)?;
                match (&lhs_type, &rhs_type) {
                    (ValueType::Seq(t1), ValueType::Seq(t2)) => {
                        let elem_t = scope.unify(t1, t2)?;
                        Ok(ValueType::Seq(Box::new(elem_t)))
                    }
                    (ValueType::Seq(..), other) => {
                        Err(anyhow!("Append: rhs is not Seq: {other:?}"))
                    }
                    (other, ValueType::Seq(..)) => {
                        Err(anyhow!("Append: lhs is not Seq: {other:?}"))
                    }
                    (lhs_type, rhs_type) => Err(anyhow!(
                        "Append: lhs and rhs must be Seq: {lhs_type:?}, {rhs_type:?} !~ Seq(_)"
                    )),
                }
            }
        }
    }

    /// Returns `true` if the evaluation of `self` contains any references to an external variable with a given identifier
    /// of `name`. This occurs when the expression contains `Expr::Var(name)` that it does not itself provide a local binding for
    /// (e.g. via a capture in a pattern-match or lambda head).
    pub fn is_shadowed_by(&self, name: &str) -> bool {
        match self {
            Expr::Var(vname) => vname == name,
            Expr::Lambda(head_var, body) => {
                if head_var == name {
                    false
                } else {
                    body.is_shadowed_by(name)
                }
            }
            Expr::Arith(_, x, y) | Expr::IntRel(_, x, y) | Expr::Append(x, y) => {
                x.is_shadowed_by(name) || y.is_shadowed_by(name)
            }
            Expr::Unary(_, x) => x.is_shadowed_by(name),
            Expr::Dup(x, y) => x.is_shadowed_by(name) || y.is_shadowed_by(name),
            Expr::Bool(_) | Expr::U8(_) | Expr::U16(_) | Expr::U32(_) | Expr::U64(_) => false,
            Expr::Numeric(num_tree) => num_tree.iter_vars().any(|v| v == name),
            Expr::Tuple(ts) => ts.iter().any(|x| x.is_shadowed_by(name)),
            Expr::TupleProj(tup, _) => tup.is_shadowed_by(name),
            Expr::Record(fs) => {
                for (fname, fld) in fs.iter() {
                    if fld.is_shadowed_by(name) {
                        // NOTE - the first field-expr that is shadowed by `name` wins
                        return true;
                    } else if fname == name {
                        // NOTE - if a field-name matches `name` exactly, then it itself will shadow any binding of `name` external to the record
                        return false;
                    }
                }
                false
            }
            Expr::RecordProj(rec, _) => rec.is_shadowed_by(name),
            Expr::Variant(_, inner) => inner.is_shadowed_by(name),
            Expr::Seq(elts) => elts.iter().any(|x| x.is_shadowed_by(name)),
            Expr::Match(head, arms) => {
                head.is_shadowed_by(name)
                    || arms
                        .iter()
                        .any(|(pat, x)| !pat.shadows(name) && x.is_shadowed_by(name))
            }
            Expr::Destructure(head, pattern, body) => {
                head.is_shadowed_by(name) || (!pattern.shadows(name) && body.is_shadowed_by(name))
            }
            Expr::AsU8(x)
            | Expr::AsU16(x)
            | Expr::AsU32(x)
            | Expr::AsU64(x)
            | Expr::AsChar(x)
            | Expr::U16Be(x)
            | Expr::U16Le(x)
            | Expr::U32Be(x)
            | Expr::U32Le(x)
            | Expr::U64Be(x)
            | Expr::U64Le(x)
            | Expr::SeqLength(x) => x.is_shadowed_by(name),
            Expr::EnumFromTo(s, e) => s.is_shadowed_by(name) || e.is_shadowed_by(name),
            Expr::SubSeq(x, s, l) | Expr::SubSeqInflate(x, s, l) => {
                x.is_shadowed_by(name) || s.is_shadowed_by(name) || l.is_shadowed_by(name)
            }
            Expr::SeqIx(x, i) => x.is_shadowed_by(name) || i.is_shadowed_by(name),
            Expr::FlatMap(f, x) | Expr::FlatMapList(f, _, x) => {
                f.is_shadowed_by(name) || x.is_shadowed_by(name)
            }
            Expr::FlatMapAccum(f, z, _, x) | Expr::LeftFold(f, z, _, x) => {
                f.is_shadowed_by(name) || z.is_shadowed_by(name) || x.is_shadowed_by(name)
            }
            Expr::FindByKey(_, k, v, s) => {
                k.is_shadowed_by(name) || v.is_shadowed_by(name) || s.is_shadowed_by(name)
            }
            Expr::LiftOption(opt_x) => opt_x.as_ref().is_some_and(|x| x.is_shadowed_by(name)),
        }
    }

    /// Conservative bounds for unsigned numeric expressions
    fn bounds(&self) -> Bounds {
        match self {
            Expr::U8(n) => Bounds::exact(usize::from(*n)),
            Expr::U16(n) => Bounds::exact(usize::from(*n)),
            Expr::U32(n) => Bounds::exact(*n as usize),
            Expr::U64(n) => Bounds::exact(*n as usize),

            Expr::AsU8(x) => x
                .bounds()
                .clamp(0, u8::MAX as usize)
                .unwrap_or(Bounds::any()),
            Expr::AsU16(x) => x
                .bounds()
                .clamp(0, u16::MAX as usize)
                .unwrap_or(Bounds::any()),
            Expr::AsU32(x) => x
                .bounds()
                .clamp(0, u32::MAX as usize)
                .unwrap_or(Bounds::any()),
            Expr::AsU64(x) => x
                .bounds()
                .clamp(0, u64::MAX as usize)
                .unwrap_or(Bounds::any()),

            Expr::Arith(arith, a, b) => {
                let a_bounds = a.bounds();
                let b_bounds = b.bounds();
                Bounds::arith(*arith, a_bounds, b_bounds)
            }

            Expr::Unary(op, a) => match op {
                UnaryOp::BoolNot => Bounds::any(),
                UnaryOp::IntPred => a.bounds() - Bounds::exact(1),
                UnaryOp::IntSucc => a.bounds() + Bounds::exact(1),
            },

            Expr::Numeric(n) => {
                let tmp = n.eval_strict(VOID);
                if let Ok(sv) = tmp
                    && sv.is_valid()
                    && let Some(n) = sv.value.as_usize()
                {
                    Bounds::exact(n)
                } else {
                    // NOTE - this case doesn't distinguish between variable-dependent num-exprs, temporary overflow/underflow, negative results, and positive results larger than usize::MAX
                    Bounds::any()
                }
            }

            // TODO - consider the remaining cases in this catch-all and determine, of those remaining, which have statically solvable bounds
            _ => Bounds::any(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum DynFormat {
    /// Huffman(lengths, values)
    Huffman(Box<Expr>, Option<Box<Expr>>),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum ViewFormat {
    /// CaptureBytes(N): captures a slice of N bytes from the start of the View
    CaptureBytes(Box<Expr>),
    /// ReadArray(M, Kind): captures an array of M elements of the indicated Kind
    ReadArray(Box<Expr>, FixedReadKind),
    /// ReifyView: produces a value-element that encapsulates the View-object
    ReifyView,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum ViewExpr {
    Var(Label),
    /// Offset(BaseView, OffsetExpr): advances the start of BaseView by a number of byte-positions equal to the numeric value of OffsetExpr (which can be any numeric type)
    Offset(Box<ViewExpr>, Box<Expr>),
}

impl ViewExpr {
    pub fn get_base(&self) -> &Label {
        match self {
            ViewExpr::Var(name) => name,
            ViewExpr::Offset(base, _) => base.get_base(),
        }
    }

    pub fn var(name: impl IntoLabel) -> Self {
        ViewExpr::Var(name.into())
    }

    pub fn offset(self, offset: Expr) -> Self {
        ViewExpr::Offset(Box::new(self), Box::new(offset))
    }

    pub fn check_type(&self, scope: &TypeScope<'_, ValueType>) -> AResult<()> {
        match self {
            ViewExpr::Var(ident) => match scope.get_type_by_name(ident) {
                ValueKind::View => Ok(()),
                ValueKind::Value(_t) => Err(anyhow!("expected View, found Value({_t:?}")),
                ValueKind::Format(_t) => Err(anyhow!("expected View, found FOrmat({_t:?}")),
            },
            ViewExpr::Offset(view_expr, expr) => {
                let t = expr.infer_type(scope)?;
                if t.is_unsigned_or_auto() {
                    view_expr.check_type(scope)
                } else {
                    Err(anyhow!(
                        "offset-expr in ViewExpr must be unsigned or auto, found {t:?}"
                    ))
                }
            }
        }
    }

    pub fn check_type_ext(&self, scope: &TypeScope<'_, alt::ValueTypeExt>) -> AResult<()> {
        match self {
            ViewExpr::Var(ident) => match scope.get_type_by_name(ident) {
                ValueKind::View => Ok(()),
                ValueKind::Value(_t) => Err(anyhow!("expected View, found Value({_t:?}")),
                ValueKind::Format(_t) => Err(anyhow!("expected View, found FOrmat({_t:?}")),
            },
            ViewExpr::Offset(view_expr, expr) => {
                let t = expr.infer_type_ext(scope)?;
                if t.is_unsigned_or_auto() {
                    view_expr.check_type_ext(scope)
                } else {
                    Err(anyhow!(
                        "offset-expr in ViewExpr must be unsigned or auto, found {t:?}"
                    ))
                }
            }
        }
    }
}

/// Operations we want to treat as semi-first-class in downstream processing,
/// without forcing us to add new primitives into the Format layer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "tag", content = "args")]
pub enum CommonOp {
    // FIXME[epic=signed-parse] - add in expressivity for signed-integer parsing as commonop
    EndianParse(BaseKind<Endian>),
}

/// The input is a UTF-8 encoded string, and the output is a UTF-8 encoded string
// NOTE - as currently defined, StyleHint could easily be Copy, but it would be a breaking change if we later had to remove that trait
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "tag", content = "args")]
pub enum StyleHint {
    /// Old-style: all field-parses are named and persisted in the original order
    /// New-style: not all fields are persisted, and names can be changed as needed
    Record {
        old_style: bool,
    },
    AsciiStr,
    AsciiChar,
    // REVIEW - consider implementing Utf8Char
    UTF8Str,
    Common(CommonOp),
}

pub(crate) mod record_fmt;
pub(crate) use record_fmt::{OwnedRecordFormat, RecordBuilder};

pub mod format;
use format::LookaheadEffect;
pub use format::{Format, RepeatRange};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct FormatRef(usize);

impl FormatRef {
    pub fn get_level(&self) -> usize {
        self.0
    }

    pub fn call(&self) -> Format {
        Format::ItemVar(self.0, Vec::new(), Vec::new())
    }

    // REVIEW - do we need it to be `Vec` or is `impl IntoIterator<Item = Expr>` better?
    pub fn call_args(&self, args: Vec<Expr>) -> Format {
        Format::ItemVar(self.0, args, Vec::new())
    }

    pub fn call_views(&self, views: Vec<ViewExpr>) -> Format {
        Format::ItemVar(self.0, Vec::new(), views)
    }

    pub fn call_args_views(&self, args: Vec<Expr>, views: Vec<ViewExpr>) -> Format {
        Format::ItemVar(self.0, args, views)
    }

    pub fn call_view(&self, view: ViewExpr) -> Format {
        Format::ItemVar(self.0, vec![], vec![view])
    }
}

#[derive(Debug, Serialize)]
pub struct FormatModule {
    names: Vec<Label>,
    args: Vec<Vec<(Label, ValueType)>>,
    views: IxHeap<Vec<Label>>,
    formats: Vec<Format>,
    format_types: Vec<ValueType>,
}

impl FormatModule {
    pub fn new() -> FormatModule {
        FormatModule {
            names: Vec::new(),
            args: Vec::new(),
            views: IxHeap::new(),
            formats: Vec::new(),
            format_types: Vec::new(),
        }
    }

    pub fn define_format(&mut self, name: impl IntoLabel, format: Format) -> FormatRef {
        self.define_format_args(name, vec![], format)
    }

    pub fn define_format_args(
        &mut self,
        name: impl IntoLabel,
        args: Vec<(Label, ValueType)>,
        format: Format,
    ) -> FormatRef {
        self.define_format_args_views(name, args, vec![], format)
    }

    pub fn define_format_views(
        &mut self,
        name: impl IntoLabel,
        views: Vec<Label>,
        format: Format,
    ) -> FormatRef {
        self.define_format_args_views(name, vec![], views, format)
    }

    pub fn define_format_args_views(
        &mut self,
        name: impl IntoLabel,
        args: Vec<(Label, ValueType)>,
        views: Vec<Label>,
        format: Format,
    ) -> FormatRef {
        let mut scope = TypeScope::with_rec_types(&self.format_types);
        for (arg_name, arg_type) in &args {
            scope.push(arg_name.clone(), arg_type.clone());
        }
        for view_name in &views {
            scope.push_view(view_name.clone());
        }
        let format_type = match self.infer_format_type(&scope, &format) {
            Ok(t) => t,
            Err(msg) => panic!("{msg}"),
        };
        let level = self.names.len();
        self.names.push(name.into());
        self.args.push(args);
        self.views.push(views);
        self.formats.push(format);
        self.format_types.push(format_type);
        FormatRef(level)
    }

    /// Registers a self-referential ("phantom-recursive") format under `name`.
    ///
    /// This is the no-args, no-views convenience form of [`Self::define_format_phantom_rec_args_views`].
    pub fn define_format_phantom_rec(
        &mut self,
        name: impl IntoLabel,
        outer: impl FnOnce(Format) -> Format,
        inner: impl FnOnce(FormatRef) -> Format,
    ) -> FormatRef {
        self.define_format_phantom_rec_args(name, vec![], outer, inner)
    }

    /// Registers a self-referential ("phantom-recursive") format under `name`, with arguments but no views.
    ///
    /// See [`Self::define_format_phantom_rec_args_views`] for the full behavior.
    pub fn define_format_phantom_rec_args(
        &mut self,
        name: impl IntoLabel,
        args: Vec<(Label, ValueType)>,
        outer: impl FnOnce(Format) -> Format,
        inner: impl FnOnce(FormatRef) -> Format,
    ) -> FormatRef {
        self.define_format_phantom_rec_args_views(name, args, vec![], outer, inner)
    }

    /// Registers a self-referential ("phantom-recursive") format under `name`, with views but no arguments.
    ///
    /// See [`Self::define_format_phantom_rec_args_views`] for the full behavior.
    pub fn define_format_phantom_rec_views(
        &mut self,
        name: impl IntoLabel,
        views: Vec<Label>,
        outer: impl FnOnce(Format) -> Format,
        inner: impl FnOnce(FormatRef) -> Format,
    ) -> FormatRef {
        self.define_format_phantom_rec_args_views(name, vec![], views, outer, inner)
    }

    /// Registers a self-referential ("phantom-recursive") format under `name`.
    ///
    /// Ordinary [`Self::define_format_args_views`] cannot define a format that refers to its own
    /// [`FormatRef`], because the level a format will occupy is only known (and only valid to look
    /// up) once registration has completed, whereas type-checking the format happens beforehand.
    /// This method allows exactly one, deliberately constrained form of self-reference: a reference
    /// to the format's own (about-to-be-assigned) level that occurs *only* underneath a
    /// [`Format::Phantom`] wrapper.
    ///
    /// `inner` is given a [`FormatRef`] for the level this call will register, and must produce the
    /// `Format` that ends up wrapped in `Format::Phantom`; it may invoke that `FormatRef` (via
    /// [`FormatRef::call`] or its sibling methods) any number of times. `outer` then receives the
    /// resulting `Format::Phantom(..)` node and weaves it into the format to be registered, e.g. by
    /// embedding (and, since `Format` is `Clone`, possibly duplicating) it into one or more
    /// branches of a record or alternation.
    ///
    /// The recursion is safe to register because every processing pathway that descends into a
    /// `Format` for purposes other than naming/typing treats `Format::Phantom`'s contents as opaque
    /// (no bytes are read on its account), so confining the self-reference there guarantees it can
    /// never be the cause of unbounded data-driven recursion. It does *not*, on its own, guarantee
    /// that every later stage that walks into `Phantom` for type- or name-resolution purposes
    /// already tolerates a self-reference found there; some such stages may need adjustment of
    /// their own before a format defined this way can be successfully elaborated/compiled/generated.
    ///
    /// # Panics
    ///
    /// Panics if the constructed format fails to type-check.
    pub fn define_format_phantom_rec_args_views(
        &mut self,
        name: impl IntoLabel,
        args: Vec<(Label, ValueType)>,
        views: Vec<Label>,
        outer: impl FnOnce(Format) -> Format,
        inner: impl FnOnce(FormatRef) -> Format,
    ) -> FormatRef {
        let level = self.names.len();
        let self_ref = FormatRef(level);

        // Reserve the slot before `inner`/`outer` run, so that the self-reference they embed
        // (which can only ever be reached through the `Phantom` wrapper applied below) resolves
        // to a real registration rather than indexing past the end of the module's tables.
        //
        // `args`/`views` are real and final already (the caller supplies them up front), but the
        // format and its inferred type are necessarily provisional at this point - the placeholder
        // type `ValueType::Any` is never actually inspected: the only thing that consults
        // `format_types[level]` before it is overwritten below is the `ItemVar` case of
        // `infer_format_type`, when it reaches the self-reference, and the enclosing
        // `Format::Phantom` arm only wraps that result in `ValueType::PhantomData` without
        // examining its shape.
        self.names.push(name.into());
        self.args.push(args.clone());
        self.views.push(views.clone());
        self.formats.push(Format::EMPTY);
        self.format_types.push(ValueType::Any);

        let phantom_param = inner(self_ref);
        let format = outer(Format::Phantom(Box::new(phantom_param)));

        let mut scope = TypeScope::with_rec_types(&self.format_types);
        for (arg_name, arg_type) in &args {
            scope.push(arg_name.clone(), arg_type.clone());
        }
        for view_name in &views {
            scope.push_view(view_name.clone());
        }
        let format_type = match self.infer_format_type(&scope, &format) {
            Ok(t) => t,
            Err(msg) => panic!("{msg}"),
        };

        self.formats[level] = format;
        self.format_types[level] = format_type;

        self_ref
    }

    /// Registers a batch of mutually-referencing formats under the given names.
    ///
    /// Unlike [`Self::define_format_phantom_rec_args_views`] (which permits exactly one format to
    /// reference only its own, [`Format::Phantom`]-confined level), this reserves
    /// `formats.len()` levels up front, then rewrites every [`Format::RecVar`] in each supplied
    /// body to the real, absolute [`Format::ItemVar`] for its batch position (`RecVar(0)` is the
    /// first name in `formats`, `RecVar(1)` the second, and so on - "self" for a singly-recursive
    /// member is just `RecVar` of its own batch position). Every occurrence is rewritten before
    /// the format is ever installed into the module, so no other pass over `Format` ever
    /// encounters a `RecVar` - see that variant's own doc comment.
    ///
    /// No `args`/`views` support (matching the shape of a plain [`Self::define_format`] call per
    /// batch member); add a `_views`/`_args`-suffixed sibling if a real recursive format ever
    /// needs them.
    ///
    /// # Panics
    ///
    /// Panics if any constructed format fails to type-check, or if any batch member has no
    /// reachable, non-recursive base case (see `check_batch_has_base_case`) - e.g. `F := 'a' F`,
    /// unconditionally self-referential with no non-recursive alternative anywhere on its cycle,
    /// which would otherwise recurse without any depth bound at decode time.
    pub fn define_format_rec_batch<L: IntoLabel>(
        &mut self,
        formats: Vec<(L, Format)>,
    ) -> Vec<FormatRef> {
        let n = formats.len();
        let mut names = Vec::with_capacity(n);
        let mut bodies = Vec::with_capacity(n);
        for (name, format) in formats {
            names.push(name.into());
            bodies.push(format);
        }

        let start = self.reserve_batch_levels(names);
        for (ix, format) in bodies.into_iter().enumerate() {
            self.formats[start + ix] = format.substitute_rec_var(start, n);
        }
        self.check_batch_has_base_case(start, n);
        self.infer_batch_types(start, n);

        (start..start + n).map(FormatRef).collect()
    }

    /// Registers a batch of `N` co-referential formats specified via delayed-construction closures.
    ///
    /// Reserves all `N` levels up front, exactly like [`Self::define_format_rec_batch`], but hands
    /// each member-builder (a reference to) the resulting `[FormatRef; N]`, rather than requiring the
    /// caller to pre-build a whole `Format` tree using raw `RecVar` indices. Since [`FormatRef`] is
    /// `Copy` (unlike [`Format`]), a builder can call `.call()`/`.call_args()`/etc. on any of `refs`
    /// as many times as it needs, without having to thread a fresh `Format::RecVar(i)`-valued
    /// parameter through its own signature for each additional use site of the same recursive
    /// reference - the exact friction that motivated this constructor (see `doc/RECURSION.md`'s
    /// "mental model" section for the underlying `ItemVar`/level model this still builds on).
    ///
    /// Builders do *not* receive `&mut FormatModule` - unlike every other `define_format_*`
    /// helper, a batch member's own body is built *after* all `N` levels are reserved, so any
    /// auxiliary sub-format a builder registered mid-build would land at a level *above* the
    /// batch it logically belongs under, the one place in the module where the usual "a format
    /// can only be referenced by a later one" ordering would silently fail to hold. A builder
    /// that needs its own named auxiliary sub-formats (e.g. one that dispatches on a tagged union
    /// of many sub-formats worth naming independently) registers them *before* this call instead,
    /// in an ordinary function taking `&mut FormatModule` that returns the residual
    /// `Box<dyn FnOnce(&[FormatRef; N]) -> Format>` - e.g. `bson.rs`'s `element`, which takes
    /// `&mut FormatModule` and two sibling `FormatRef`s, registers its sixteen
    /// `bson.element.*` tag-dispatch sub-formats, and returns the boxed closure that finishes the
    /// job once `refs` exists. The caller applies it eagerly (`element(module, cstring, string)`)
    /// and passes the resulting `Box` into `formats` like any other member.
    ///
    /// Builders are invoked in `formats` order, and the same order-dependent type-inference
    /// behavior documented on [`Self::define_format_rec_batch`] applies here too: a member
    /// referencing an *earlier* sibling sees that sibling's real, already-solved type, while one
    /// referencing itself or a *later* sibling sees the still-provisional placeholder.
    ///
    /// This is presently a thin, additive convenience layer over the same underlying level/`ItemVar`
    /// machinery `define_format_rec_batch` uses (a builder's returned `Format` never contains a
    /// `RecVar` to begin with, so no substitution pass is needed) - it does not remove or replace
    /// `Format::RecVar`/`define_format_rec_batch`, which remain the lower-level, `Vec`-based entry
    /// point.
    ///
    /// Builders are boxed (`Box<dyn FnOnce(..) -> Format>`) rather than a single generic `F`,
    /// because distinct batch members typically close over distinct free variables (e.g. one member
    /// needing a couple of sibling `FormatRef`s a neighboring member doesn't) - which makes them
    /// distinct concrete closure types that a single monomorphic `[F; N]` could not hold together.
    ///
    /// # Panics
    ///
    /// Panics if any constructed format fails to type-check, or if any batch member has no
    /// reachable, non-recursive base case - see [`Self::define_format_rec_batch`]'s own `# Panics`
    /// section.
    pub fn define_format_recursive<L: IntoLabel, const N: usize>(
        &mut self,
        names: [L; N],
        formats: [Box<dyn FnOnce(&[FormatRef; N]) -> Format>; N],
    ) -> [FormatRef; N] {
        let start = self.reserve_batch_levels(names.into_iter().map(Into::into));
        let refs: [FormatRef; N] = std::array::from_fn(|ix| FormatRef(start + ix));

        for (ix, build) in formats.into_iter().enumerate() {
            // A batch length of 0 rejects any `RecVar` a builder returned, before the base-case
            // check below can trip over it.
            let body = build(&refs).substitute_rec_var(start, 0);
            self.formats[start + ix] = body;
        }
        self.check_batch_has_base_case(start, N);
        self.infer_batch_types(start, N);

        refs
    }

    /// Reserves `names.len()` contiguous levels, starting at the module's current length, each with
    /// a placeholder (`Format::EMPTY`/`ValueType::Any`) body - so that a self- or forward-reference
    /// to any reserved level (including one still awaiting its real body) resolves to a real, if
    /// still type-provisional, slot rather than indexing out of bounds. Shared by
    /// [`Self::define_format_rec_batch`] and [`Self::define_format_recursive`]; returns the starting
    /// level.
    fn reserve_batch_levels(&mut self, names: impl IntoIterator<Item = Label>) -> usize {
        let start = self.names.len();
        for name in names {
            self.names.push(name);
            self.args.push(Vec::new());
            self.views.push(Vec::new());
            self.formats.push(Format::EMPTY);
            self.format_types.push(ValueType::Any);
        }
        start
    }

    /// Rejects a just-installed recursive batch if any of its `n` members (starting at `start`) has
    /// no reachable, non-recursive base case - i.e. every derivation through its body eventually
    /// depends on re-entering a level that's still open (see [`Format::has_reachable_base_case`]).
    /// `F := 'a' F`, with no non-recursive alternative anywhere on its cycle, is the canonical
    /// example: it makes byte progress on every step (so the [`MatchTree`] cycle guard treats it as
    /// fine, and it isn't left-recursive), yet can never actually complete on any finite input - a
    /// decoder for it would only ever stop because the input ran out, after recursing once per
    /// remaining byte with no depth bound of its own. Left unchecked, this shape reaches
    /// `MatchTree`/codegen and recurses at decode time via a plain, uncounted call stack (both the
    /// interpreter's `Decoder::Call` and generated code's per-level functions), so an
    /// attacker-controlled input to a format with this shape can exhaust the stack. Catching it here
    /// - at the single, shared point both the interpreter and codegen compile from - closes that gap
    /// for both backends at once, rather than needing a runtime depth counter threaded through
    /// either one.
    ///
    /// This is a distinct, narrower check than genuine left recursion (`bad := bad | 'Z'`, still
    /// only caught later, at `MatchTree`-build time - see [`Self::define_format_rec_batch`]'s own
    /// doc comment): a format can have a perfectly reachable base case and still be left-recursive
    /// (progress is not reachability), and a format can make byte progress on every single step and
    /// still have no base case at all (reachability is not progress). Neither check subsumes the
    /// other, so both remain necessary.
    ///
    /// Panics naming every offending member, mirroring the type-check panic just below it - a
    /// malformed recursive batch is a genuine grammar defect, not a recoverable `Result`, exactly
    /// like an ill-typed one.
    fn check_batch_has_base_case(&self, start: usize, n: usize) {
        let bad_names: Vec<&str> = (start..start + n)
            .filter(|&ix| !self.formats[ix].has_reachable_base_case(self))
            .map(|ix| self.names[ix].as_ref())
            .collect();
        if !bad_names.is_empty() {
            panic!(
                "recursive format batch has no reachable base case for: {} - every derivation \
                 requires revisiting a still-open recursive reference; add a non-recursive \
                 alternative (e.g. a `Union` branch, or `Repeat`'s implicit zero-length case) \
                 somewhere on the cycle",
                bad_names.join(", ")
            );
        }
    }

    /// Solves and installs the real type for each of `n` levels starting at `start`.
    ///
    /// Each member is first inferred in ascending order by registration's own inference, with
    /// the same checks as any other registration. `infer_format_type`'s `Format::ItemVar` arm
    /// reads `format_types[level]` directly rather than recursing into the level's body, so a
    /// member referencing an *earlier* sibling sees that sibling's type from this pass, while one
    /// referencing a *later* sibling (or itself) sees the `ValueType::Any` placeholder, which
    /// `ValueType::unify` accepts unconditionally. An ill-typed use of such a reference (e.g.
    /// comparing a recursive field against a `u8`) therefore passes this pass.
    ///
    /// The batch is then solved by [`TypeChecker::solve_batch_types`](typecheck::TypeChecker::solve_batch_types), whose equirecursive
    /// inference sees through the cycle and rejects such a use. Its solution, in which a reference
    /// back to a batch level is [`ValueType::Rec`] instead of a placeholder, is what gets installed,
    /// so a later registration that reaches into a member's recursive field sees its real type.
    /// Shared by [`Self::define_format_rec_batch`] and [`Self::define_format_recursive`].
    fn infer_batch_types(&mut self, start: usize, n: usize) {
        for ix in start..start + n {
            let scope = TypeScope::with_rec_types(&self.format_types);
            let format_type = match self.infer_format_type(&scope, &self.formats[ix]) {
                Ok(t) => t,
                Err(msg) => panic!("Failed to solve type for {}: {msg}", self.names[ix]),
            };
            self.format_types[ix] = format_type;
        }
        match typecheck::TypeChecker::solve_batch_types(self, start..start + n) {
            Ok(types) => {
                for (ix, format_type) in (start..start + n).zip(types) {
                    self.format_types[ix] = format_type;
                }
            }
            Err(err) => panic!(
                "recursive format batch failed to type-check ({}): {err}",
                self.names[start..start + n].join(", ")
            ),
        }
    }

    pub fn get_name(&self, level: usize) -> &str {
        &self.names[level]
    }

    /// Iterates through every format defined in this module, constructing an invocation for each
    /// with an appropriate array of arguments with the expected `ValueType`s.
    pub fn iter_formats(&self) -> impl Iterator<Item = (usize, Format)> + '_ {
        (0..self.formats.len()).filter_map(|ix| {
            let mut x_args = Vec::with_capacity(self.args[ix].len());
            for (_, vt) in self.args[ix].iter() {
                x_args.push(mk_value_expr(vt)?);
            }
            match self.views[ix].len() {
                0 => Some((ix, Format::ItemVar(ix, x_args, Vec::new()))),
                n => {
                    let x_views = repeat_n(ViewExpr::var("dummy"), n).collect();
                    Some((
                        ix,
                        Format::LetView(
                            Label::Borrowed("dummy"),
                            Box::new(Format::ItemVar(ix, x_args, x_views)),
                        ),
                    ))
                }
            }
        })
    }

    fn get_args(&self, level: usize) -> &[(Label, ValueType)] {
        &self.args[level]
    }

    fn get_view_args(&self, level: usize) -> &[Label] {
        &self.views[level]
    }

    pub fn get_format(&self, level: usize) -> &Format {
        &self.formats[level]
    }

    pub fn get_format_type(&self, level: usize) -> &ValueType {
        &self.format_types[level]
    }

    fn infer_format_type(&self, scope: &TypeScope<'_>, f: &Format) -> AResult<ValueType> {
        match f {
            Format::ItemVar(level, arg_exprs, arg_views) => {
                let arg_names = self.get_args(*level);
                if arg_names.len() != arg_exprs.len() {
                    return Err(anyhow!(
                        "Expected {} arguments, found {}",
                        arg_names.len(),
                        arg_exprs.len()
                    ));
                }
                let view_names = self.get_view_args(*level);
                if view_names.len() != arg_views.len() {
                    return Err(anyhow!(
                        "Expected {} views, found {}",
                        view_names.len(),
                        arg_views.len(),
                    ));
                }
                for ((_name, arg_type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    let t = expr.infer_type(scope)?;
                    let _t = scope.unify(arg_type, &t)?;
                }
                Ok(self.get_format_type(*level).clone())
            }
            // `define_format_rec_batch` rewrites every `RecVar` before inferring types, so one
            // reaching here was passed to some other registration path (e.g. `define_format`, or
            // returned from a `define_format_recursive` builder).
            Format::RecVar(ix) => Err(anyhow!(
                "Format::RecVar({ix}) is only valid in the body of a `define_format_rec_batch` member \
                 (in a `define_format_recursive` builder, use the provided `FormatRef`s instead)"
            )),
            Format::DecodeBytes(bytes, f) => {
                let bytes_type = bytes.infer_type(scope)?;
                match bytes_type {
                    ValueType::Seq(bt) if matches!(*bt, ValueType::U8) => {
                        self.infer_format_type(scope, f)
                    }
                    other => Err(anyhow!(
                        "DecodeBytes first argument type should be Seq(U8), found {other:?} instead"
                    )),
                }
            }
            Format::ParseFromView(view, f) => {
                view.check_type(scope)?;
                self.infer_format_type(scope, f)
            }
            Format::Fail => Ok(ValueType::Empty),
            Format::SkipRemainder | Format::EndOfInput => Ok(ValueType::Tuple(vec![])),
            Format::Align(_n) => Ok(ValueType::Tuple(vec![])),
            Format::Byte(_bs) => Ok(ValueType::U8),
            Format::Variant(label, f) => Ok(ValueType::Union(BTreeMap::from([(
                label.clone(),
                self.infer_format_type(scope, f)?,
            )]))),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut t = ValueType::Any;
                for f in branches {
                    t = scope.unify(&t, &self.infer_format_type(scope, f)?)?;
                }
                Ok(t)
            }
            Format::Tuple(fields) => {
                let mut ts = Vec::with_capacity(fields.len());
                for f in fields {
                    ts.push(self.infer_format_type(scope, f)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            Format::Repeat(a) | Format::Repeat1(a) => {
                let t = self.infer_format_type(scope, a)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Format::RepeatCount(count, a) => match count.infer_type(scope)? {
                t if t.is_unsigned_or_auto() => {
                    let t = self.infer_format_type(scope, a)?;
                    Ok(ValueType::Seq(Box::new(t)))
                }
                other => Err(anyhow!(
                    "RepeatCount first argument type should be unsigned or auto, found {other:?} instead"
                )),
            },
            Format::Sequence(formats) => {
                let mut elem_t = ValueType::Any;
                for f in formats {
                    elem_t = scope.unify(&elem_t, &self.infer_format_type(scope, f)?)?;
                }
                Ok(ValueType::Seq(Box::new(elem_t)))
            }
            Format::RepeatBetween(_range, a) => {
                let t = self.infer_format_type(scope, a)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Format::RepeatUntilLast(lambda_elem, a) => match lambda_elem.as_ref() {
                Expr::Lambda(head, expr) => {
                    let t = self.infer_format_type(scope, a)?;
                    let mut child_scope = TypeScope::child(scope);
                    child_scope.push(head.clone(), t.clone());
                    let ret_type = expr.infer_type(&child_scope)?;
                    match ret_type {
                        ValueType::Base(BaseType::Bool) => Ok(ValueType::Seq(Box::new(t))),
                        other => Err(anyhow!(
                            "RepeatUntilLast first argument (lambda) return type should be Bool, found {other:?} instead"
                        )),
                    }
                }
                other => Err(anyhow!(
                    "RepeatUntilLast first argument type should be lambda, found {other:?} instead"
                )),
            },
            Format::RepeatUntilSeq(lambda_seq, a) => match lambda_seq.as_ref() {
                Expr::Lambda(head, expr) => {
                    let t = self.infer_format_type(scope, a)?;
                    let mut child_scope = TypeScope::child(scope);
                    child_scope.push(head.clone(), ValueType::Seq(Box::new(t.clone())));
                    let ret_type = expr.infer_type(&child_scope)?;
                    match ret_type {
                        ValueType::Base(BaseType::Bool) => Ok(ValueType::Seq(Box::new(t))),
                        other => Err(anyhow!(
                            "RepeatUntilSeq first argument (lambda) return type should be Bool, found {other:?} instead"
                        )),
                    }
                }
                other => Err(anyhow!(
                    "RepeatUntilSeq first argument type should be lambda, found {other:?} instead"
                )),
            },
            Format::AccumUntil(lambda_acc_seq, lambda_acc_val, init, vt, a) => {
                match lambda_acc_seq.as_ref() {
                    Expr::Lambda(head, expr) => {
                        let t = self.infer_format_type(scope, a)?;
                        // Check that the initial accumulator value's type unifies with the type-claim
                        let _acc_type = scope.unify(&init.infer_type(scope)?, vt.as_ref())?;
                        let mut child_scope = TypeScope::child(scope);
                        let t_seq = ValueType::Seq(Box::new(t.clone()));
                        let vt_acc_seq = ValueType::Tuple(vec![vt.as_ref().clone(), t_seq.clone()]);
                        child_scope.push(head.clone(), vt_acc_seq.clone());
                        let ret_type = expr.infer_type(&child_scope)?;
                        match ret_type {
                            ValueType::Base(BaseType::Bool) => {
                                match lambda_acc_val.as_ref() {
                                    Expr::Lambda(head, expr) => {
                                        let mut child_scope = TypeScope::child(&child_scope);
                                        let vt_acc_elem =
                                            ValueType::Tuple(vec![vt.as_ref().clone(), t.clone()]);
                                        child_scope.push(head.clone(), vt_acc_elem);
                                        // we just need to check that these types unify, the value is unimportant
                                        let _ret_type = scope
                                            .unify(&expr.infer_type(&child_scope)?, vt.as_ref())?;
                                        Ok(vt_acc_seq)
                                    }
                                    other => Err(anyhow!(
                                        "AccumUntil second argument type should be lambda, found {other:?} instead"
                                    )),
                                }
                            }
                            other => Err(anyhow!(
                                "AccumUntil first argument (lambda) return type should be Bool, found {other:?} instead"
                            )),
                        }
                    }
                    other => Err(anyhow!(
                        "AccumUntil first argument type should be lambda, found {other:?} instead"
                    )),
                }
            }
            Format::Maybe(x, a) => match x.infer_type(scope)? {
                ValueType::Base(BaseType::Bool) => {
                    let t = self.infer_format_type(scope, a)?;
                    Ok(ValueType::Option(Box::new(t)))
                }
                other => Err(anyhow!(
                    "Maybe-predicate is not a bool-type: {x:?} ~ {other:?}"
                )),
            },
            Format::Peek(a) => self.infer_format_type(scope, a),
            Format::PeekNot(_a) => Ok(ValueType::Tuple(vec![])),
            Format::Slice(len, a) => match len.infer_type(scope)? {
                t if t.is_unsigned_or_auto() => self.infer_format_type(scope, a),
                other => Err(anyhow!(
                    "Slice length should be unsigned or auto, found {other:?} instead"
                )),
            },
            Format::Bits(a) => self.infer_format_type(scope, a),
            Format::WithRelativeOffset(base_addr, offset, a) => {
                for (what, expr) in [("base address", base_addr), ("offset", offset)] {
                    let t = expr.infer_type(scope)?;
                    if !t.is_unsigned_or_auto() {
                        return Err(anyhow!(
                            "WithRelativeOffset {what} should be unsigned or auto, found {t:?} instead"
                        ));
                    }
                }
                self.infer_format_type(scope, a)
            }
            Format::Map(a, expr) => {
                let arg_type = self.infer_format_type(scope, a)?;
                match expr.as_ref() {
                    Expr::Lambda(name, body) => {
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(name.clone(), arg_type);
                        body.infer_type(&child_scope)
                    }
                    other => Err(anyhow!("Map: expected lambda, found {other:?}")),
                }
            }
            Format::Where(a, expr) => {
                let arg_type = self.infer_format_type(scope, a)?;
                match expr.as_ref() {
                    Expr::Lambda(name, body) => {
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(name.clone(), arg_type.clone());
                        let t = body.infer_type(&child_scope)?;
                        if t != ValueType::Base(BaseType::Bool) {
                            return Err(anyhow!("Where: expected bool lambda, found {t:?}"));
                        }
                        Ok(arg_type)
                    }
                    other => Err(anyhow!("Where: expected lambda, found {other:?}")),
                }
            }
            Format::Compute(expr) => expr.infer_type(scope),
            Format::Let(name, expr, format) => {
                let t = expr.infer_type(scope)?;
                let mut child_scope = TypeScope::child(scope);
                child_scope.push(name.clone(), t);
                self.infer_format_type(&child_scope, format)
            }
            Format::LetView(name, format) => {
                let mut child_scope = TypeScope::child(scope);
                child_scope.push_view(name.clone());
                self.infer_format_type(&child_scope, format)
            }
            Format::LetFormat(f0, name, f) => {
                let t0 = self.infer_format_type(scope, f0)?;
                let mut new_scope = TypeScope::child(scope);
                new_scope.push(name.clone(), t0);
                self.infer_format_type(&new_scope, f)
            }
            Format::MonadSeq(f0, f) => {
                // We want to check that the type of f0 is inferable, but it is used anywhere in the type of self
                self.infer_format_type(scope, f0)?;
                self.infer_format_type(scope, f)
            }
            Format::Hint(_hint, f) => self.infer_format_type(scope, f),
            #[cfg(feature = "format_enforce")]
            Format::Enforce(f) => self.infer_format_type(scope, f),
            Format::Permit(f, expr) => {
                let t0 = expr.infer_type(scope)?;
                let t = self.infer_format_type(scope, f)?;
                Ok(scope.unify(&t0, &t)?)
            }
            Format::Match(head, branches) => {
                if branches.is_empty() {
                    return Err(anyhow!("infer_format_type: empty Match"));
                }
                let head_type = Rc::new(head.infer_type(scope)?);
                let mut t = ValueType::Any;
                for (pattern, branch) in branches {
                    t = scope.unify(
                        &t,
                        &pattern.infer_format_branch_type(
                            scope,
                            head_type.clone(),
                            self,
                            branch,
                        )?,
                    )?;
                }
                Ok(t)
            }
            Format::Dynamic(name, dynformat, format) => {
                match dynformat {
                    DynFormat::Huffman(lengths_expr, opt_values_expr) => {
                        match lengths_expr.infer_type(scope)? {
                            ValueType::Seq(t) => match &*t {
                                &ValueType::U8 | &ValueType::U16 => {}
                                other => {
                                    return Err(anyhow!(
                                        "Huffman: expected U8 or U16, found {other:?}"
                                    ));
                                }
                            },
                            other => {
                                return Err(anyhow!("Huffman: expected Seq, found {other:?}"));
                            }
                        }
                        if let Some(values_expr) = opt_values_expr {
                            match values_expr.infer_type(scope)? {
                                ValueType::Seq(t) => match &*t {
                                    &ValueType::U8 | &ValueType::U16 => {}
                                    other => {
                                        return Err(anyhow!(
                                            "Huffman: expected U8 or U16, found {other:?}"
                                        ));
                                    }
                                },
                                other => {
                                    return Err(anyhow!("Huffman: expected Seq, found {other:?}"));
                                }
                            }
                        }
                    }
                }
                let mut child_scope = TypeScope::child(scope);
                child_scope.push_format(name.clone(), ValueType::U16);
                self.infer_format_type(&child_scope, format)
            }
            Format::Apply(name) => match scope.get_type_by_name(name) {
                ValueKind::Format(t) => Ok(t.clone()),
                ValueKind::View => Err(anyhow!("Apply: expected format, found View")),
                ValueKind::Value(t) => Err(anyhow!("Apply: expected format, found {t:?}")),
            },
            Format::Pos => Ok(ValueType::NumericHole),
            Format::ForEach(expr, lbl, format) => {
                let expr_t = expr.infer_type(scope)?;
                let elem_t = match expr_t {
                    ValueType::Seq(elem_t) => (*elem_t).clone(),
                    _ => return Err(anyhow!("ForEach: expected Seq, found {expr_t:?}")),
                };
                let mut child_scope = TypeScope::child(scope);
                child_scope.push(lbl.clone(), elem_t);
                let inner_t = self.infer_format_type(&child_scope, format)?;
                Ok(ValueType::Seq(Box::new(inner_t)))
            }
            Format::LiftedOption(opt_f) => {
                let inner_type = match opt_f {
                    None => ValueType::Any,
                    Some(inner_f) => self.infer_format_type(scope, inner_f)?,
                };
                Ok(ValueType::Option(Box::new(inner_type)))
            }
            Format::WithView(view, v_format) => match v_format {
                ViewFormat::CaptureBytes(len) => {
                    view.check_type(scope)?;
                    match len.infer_type(scope)? {
                        t if t.is_unsigned_or_auto() => {}
                        other => {
                            return Err(anyhow!(
                                "CaptureBytes@0: expected unsigned or auto, found {other:?}"
                            ));
                        }
                    }
                    // NOTE[epic=view-format] - in the current base-model design and implementation, CaptureBytes captures a `Seq<U8>`
                    Ok(ValueType::Seq(Box::new(ValueType::U8)))
                }
                ViewFormat::ReadArray(len, kind) => {
                    view.check_type(scope)?;
                    match len.infer_type(scope)? {
                        t if t.is_unsigned_or_auto() => {}
                        other => {
                            return Err(anyhow!(
                                "ReadArray@0: expected unsigned or auto, found {other:?}"
                            ));
                        }
                    }
                    // NOTE[epic=view-format] - in the current base-model design and implementation, ReadArray captures a `Seq<K>` where K is informed by `kind`
                    let ty = match kind {
                        FixedReadKind::Base(bk) => ValueType::from(BaseNumType::from(*bk)),
                        FixedReadKind::FixedFormat(format_ref) => {
                            let level = format_ref.get_level();
                            // As in `typecheck::infer_var_view_format`, a `FixedFormat`
                            // reference is only valid if the referenced format is fixed-size
                            // and composed entirely of primitive fields (see
                            // `fixed::analyze_fixed_shape`); this is a precondition on
                            // construction, not a recoverable error.
                            fixed::analyze_fixed_shape(self, *format_ref).unwrap_or_else(|e| {
                                panic!(
                                    "format `{}` is not eligible for FixedFormat ReadArray: {e}",
                                    self.get_name(level),
                                )
                            });
                            self.get_format_type(level).clone()
                        }
                    };
                    Ok(ValueType::Seq(Box::new(ty)))
                }
                ViewFormat::ReifyView => {
                    view.check_type(scope)?;
                    Ok(ValueType::ViewObj)
                }
            },
            Format::Phantom(inner) => {
                let inner_t = self.infer_format_type(scope, &**inner)?;
                Ok(ValueType::PhantomData(Box::new(inner_t)))
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MaybeTyped<'a, U: ?Sized, T: ?Sized> {
    Untyped(&'a U),
    Typed(&'a T),
}

impl<'a, U: ?Sized + 'a, T: ?Sized + 'a> Clone for MaybeTyped<'a, U, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, U: ?Sized + 'a, T: ?Sized + 'a> Copy for MaybeTyped<'a, U, T> {}

type MTFormatRef<'a> = MaybeTyped<'a, Format, TypedFormat<GenType>>;
type MTFormatSlice<'a> = MaybeTyped<'a, [Format], [TypedFormat<GenType>]>;

/// Incremental decomposition of a Format into a partially consumed head
/// sub-format, and a possibly-empty tail of remaining sub-formats.
///
/// All variants other than [`Next::Empty`] and [`Next::Union`] implicitly have a tail-recursive
/// element, which is invariably the final positional argument for that variant. In the case of
/// [`Next::Union`], the recursive descent is symmetric and may be balanced arbitrarily.
#[derive(PartialEq, Eq, Hash, Debug)]
enum Next<'a> {
    Empty,
    Union(Rc<Next<'a>>, Rc<Next<'a>>),
    Cat(MTFormatRef<'a>, Rc<Next<'a>>),
    Sequence(MTFormatSlice<'a>, Rc<Next<'a>>),
    Repeat(MTFormatRef<'a>, Rc<Next<'a>>),
    RepeatCount(usize, MTFormatRef<'a>, Rc<Next<'a>>),
    RepeatMax(usize, MTFormatRef<'a>, Rc<Next<'a>>), // dual to [RepeatCount] for 0..=N repeats
    RepeatBetween(usize, usize, MTFormatRef<'a>, Rc<Next<'a>>), // extension of RepeatMax/RepeatCount for N..=M repeats
    Slice(usize, Rc<Next<'a>>, Rc<Next<'a>>),
    Peek(Rc<Next<'a>>, Rc<Next<'a>>),
    PeekNot(Rc<Next<'a>>, Rc<Next<'a>>),
    /// Stands in for an unknown continuation while working out whether a format's compiled
    /// decoder depends on its continuation (see [`NextDependence`]). Behaves exactly like
    /// [`Next::Empty`] for lookahead, but records in the [`CycleGuard`] that it was consulted.
    /// Never part of a continuation that a decoder is compiled against.
    Probe,
}

impl<'a> Next<'a> {
    /// Smart constructor for [`Next::Cat`]. When `content` is transparent to lookahead (it neither
    /// consumes bytes nor constrains what may follow, e.g. a trailing `Format::Compute`), wrapping
    /// it around `next` adds no real information but *does* add a structurally distinct `Next`
    /// layer - which, for a self-referential reference compiled under it, can miss `decoder_map`'s
    /// `(level, next)` cache and queue a doomed duplicate compile (see `doc/RECURSION.md`'s
    /// mechanism 4). Returns `next` directly in that case, and `Next::Empty` when `content` accepts
    /// without consulting `next` at all (see [`LookaheadEffect`]).
    ///
    /// Merely matching zero bytes is not enough: a zero-width `Peek`/`PeekNot`/`Fail` still
    /// constrains the lookahead of whatever precedes it.
    fn cat(content: MTFormatRef<'a>, next: Rc<Next<'a>>) -> Rc<Next<'a>> {
        let effect = match content {
            MaybeTyped::Untyped(f) => f.lookahead_effect(),
            MaybeTyped::Typed(f) => f.lookahead_effect(),
        };
        match effect {
            LookaheadEffect::Transparent => next,
            LookaheadEffect::Terminal => Rc::new(Next::Empty),
            LookaheadEffect::Opaque => Rc::new(Next::Cat(content, next)),
        }
    }

    /// Smart constructor for [`Next::Sequence`]: analogous to [`Self::cat`], but for a `Tuple`/
    /// `Sequence`'s trailing field-suffix as a whole, whose effect is that of its first element
    /// that isn't transparent; an empty or all-transparent suffix is transparent (the previously
    /// special-cased `remaining == []` case falls out of this for free).
    fn sequence(content: MTFormatSlice<'a>, next: Rc<Next<'a>>) -> Rc<Next<'a>> {
        let effect = match content {
            MaybeTyped::Untyped(fs) => {
                LookaheadEffect::of_sequence(fs.iter().map(Format::lookahead_effect))
            }
            MaybeTyped::Typed(fs) => {
                LookaheadEffect::of_sequence(fs.iter().map(TypedFormat::lookahead_effect))
            }
        };
        match effect {
            LookaheadEffect::Transparent => next,
            LookaheadEffect::Terminal => Rc::new(Next::Empty),
            LookaheadEffect::Opaque => Rc::new(Next::Sequence(content, next)),
        }
    }

    /// Returns `next` with whatever follows its first `budget` bytes (by minimum match length)
    /// replaced by [`Next::Empty`].
    ///
    /// A `MatchTree` never looks more than [`MATCH_TREE_MAX_DEPTH`] bytes past its decision
    /// point, and a level's decision points all precede the continuation it is compiled against,
    /// so truncating that continuation at that many bytes cannot change the compiled decoder. It
    /// does bound the continuations a recursive level is compiled against, each of which would
    /// otherwise be a new `(level, next)` decoder-cache key, with no end to the keys when every
    /// level of nesting extends the continuation.
    fn truncated(next: &Rc<Next<'a>>, module: &FormatModule, budget: usize) -> Rc<Next<'a>> {
        Self::truncate(next, module, budget).unwrap_or_else(|| next.clone())
    }

    /// Low-level helper for [`Self::truncated`]: returns `None` if nothing needs truncating.
    fn truncate(next: &Rc<Next<'a>>, module: &FormatModule, budget: usize) -> Option<Rc<Next<'a>>> {
        let min_len = |f: MTFormatRef<'a>| match f {
            MaybeTyped::Untyped(f) => f.match_bounds(module).min,
            MaybeTyped::Typed(f) => f.match_bounds().min,
        };
        if budget == 0 {
            return match next.as_ref() {
                Next::Empty => None,
                _ => Some(Rc::new(Next::Empty)),
            };
        }
        match next.as_ref() {
            Next::Empty | Next::Probe => None,
            // never part of a continuation that a level is compiled against
            Next::Slice(..) | Next::Peek(..) | Next::PeekNot(..) => None,
            Next::Union(n1, n2) => {
                let t1 = Self::truncate(n1, module, budget);
                let t2 = Self::truncate(n2, module, budget);
                if t1.is_none() && t2.is_none() {
                    return None;
                }
                Some(Rc::new(Next::Union(
                    t1.unwrap_or_else(|| n1.clone()),
                    t2.unwrap_or_else(|| n2.clone()),
                )))
            }
            Next::Cat(f, n) => {
                let rest = Self::truncate(n, module, budget.saturating_sub(min_len(*f)))?;
                Some(Rc::new(Next::Cat(*f, rest)))
            }
            Next::Sequence(fs, n) => {
                let mut used = 0;
                match fs {
                    MaybeTyped::Untyped(fs) => {
                        for f in fs.iter() {
                            used += f.match_bounds(module).min;
                            if used >= budget {
                                break;
                            }
                        }
                    }
                    MaybeTyped::Typed(fs) => {
                        for f in fs.iter() {
                            used += f.match_bounds().min;
                            if used >= budget {
                                break;
                            }
                        }
                    }
                }
                let rest = Self::truncate(n, module, budget.saturating_sub(used))?;
                Some(Rc::new(Next::Sequence(*fs, rest)))
            }
            Next::Repeat(a, n) => {
                let rest = Self::truncate(n, module, budget)?;
                Some(Rc::new(Next::Repeat(*a, rest)))
            }
            Next::RepeatMax(k, a, n) => {
                let rest = Self::truncate(n, module, budget)?;
                Some(Rc::new(Next::RepeatMax(*k, *a, rest)))
            }
            Next::RepeatCount(k, a, n) => {
                let used = k.saturating_mul(min_len(*a));
                let rest = Self::truncate(n, module, budget.saturating_sub(used))?;
                Some(Rc::new(Next::RepeatCount(*k, *a, rest)))
            }
            Next::RepeatBetween(lo, hi, a, n) => {
                let used = lo.saturating_mul(min_len(*a));
                let rest = Self::truncate(n, module, budget.saturating_sub(used))?;
                Some(Rc::new(Next::RepeatBetween(*lo, *hi, *a, rest)))
            }
        }
    }
}

/// Per-level answers to whether a level's compiled decoder depends on the continuation it is
/// compiled against. Both compilers (`decoder::Compiler` and codegen's `GTCompiler`) consult it
/// to decide whether a call's `(level, next)` decoder-cache key keeps `next` or uses
/// [`Next::Empty`].
///
/// A level depends on its continuation if some `MatchTree` built while compiling it can consult
/// that continuation (see [`MatchTree::reaches_probe`]), directly or within a level it calls
/// with a continuation that extends its own. This follows how both compilers thread
/// continuations through each `Format`; where they differ (`Maybe`, whose inner format codegen
/// compiles against the outer continuation and the interpreter against `Next::Empty`), the
/// answer covers both.
///
/// Recursive levels are resolved as a least fixpoint: a level reached again while its own answer
/// is being worked out is assumed independent. A `true` answer is always final; a `false` answer
/// that relied on that assumption for a level further out is not recorded, and is worked out
/// again if asked for later.
#[derive(Default)]
pub(crate) struct NextDependence {
    known: HashMap<usize, bool>,
    in_progress: Vec<usize>,
    /// The lowest position in `in_progress` assumed independent while working out the answer
    /// for the innermost level in progress.
    lowest_assumed: Option<usize>,
}

impl NextDependence {
    pub(crate) fn level_depends_on_next(&mut self, module: &FormatModule, level: usize) -> bool {
        if let Some(&known) = self.known.get(&level) {
            return known;
        }
        if let Some(pos) = self.in_progress.iter().position(|&l| l == level) {
            self.lowest_assumed = Some(self.lowest_assumed.map_or(pos, |p| p.min(pos)));
            return false;
        }
        let depth = self.in_progress.len();
        self.in_progress.push(level);
        let outer_assumed = self.lowest_assumed.take();
        let depends =
            self.format_depends_on_next(module, module.get_format(level), Rc::new(Next::Probe));
        self.in_progress.pop();
        let assumed_further_out = self.lowest_assumed.filter(|&pos| pos < depth);
        if depends || assumed_further_out.is_none() {
            self.known.insert(level, depends);
        }
        let carried = if depends { None } else { assumed_further_out };
        self.lowest_assumed = match (outer_assumed, carried) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        depends
    }

    /// Whether compiling `format` against `next` can consult a [`Next::Probe`] within `next`.
    fn format_depends_on_next<'a>(
        &mut self,
        module: &'a FormatModule,
        format: &'a Format,
        next: Rc<Next<'a>>,
    ) -> bool {
        if matches!(next.as_ref(), Next::Empty) {
            return false;
        }
        match format {
            Format::ItemVar(level, _, _) => self.level_depends_on_next(module, *level),
            Format::RecVar(_) => unreachable!(
                "Format::RecVar is rewritten to ItemVar at batch registration; never appears in a stored Format"
            ),
            // compiled without a continuation, or against `Next::Empty`
            Format::Phantom(_)
            | Format::Fail
            | Format::Pos
            | Format::EndOfInput
            | Format::Align(_)
            | Format::Byte(_)
            | Format::SkipRemainder
            | Format::Compute(_)
            | Format::Apply(_)
            | Format::WithView(..)
            | Format::DecodeBytes(..)
            | Format::ParseFromView(..)
            | Format::Peek(_)
            | Format::PeekNot(_)
            | Format::Slice(..)
            | Format::Bits(_)
            | Format::WithRelativeOffset(..)
            | Format::LiftedOption(None) => false,
            Format::Variant(_, a)
            | Format::LetView(_, a)
            | Format::Map(a, _)
            | Format::Where(a, _)
            | Format::Permit(a, _)
            | Format::Let(_, _, a)
            | Format::Dynamic(_, _, a)
            | Format::Hint(_, a)
            | Format::LiftedOption(Some(a))
            | Format::Maybe(_, a) => self.format_depends_on_next(module, a, next),
            #[cfg(feature = "format_enforce")]
            Format::Enforce(a) => self.format_depends_on_next(module, a, next),
            Format::Union(branches) => {
                branches
                    .iter()
                    .any(|f| self.format_depends_on_next(module, f, next.clone()))
                    || MatchTree::reaches_probe(module, branches, next)
            }
            Format::UnionNondet(branches) => branches
                .iter()
                .any(|f| self.format_depends_on_next(module, f, next.clone())),
            Format::Match(_, branches) => branches
                .iter()
                .any(|(_, f)| self.format_depends_on_next(module, f, next.clone())),
            Format::Tuple(fields) | Format::Sequence(fields) => {
                let mut rest = fields.iter();
                while let Some(f) = rest.next() {
                    let field_next =
                        Next::sequence(MaybeTyped::Untyped(rest.as_slice()), next.clone());
                    if self.format_depends_on_next(module, f, field_next) {
                        return true;
                    }
                }
                false
            }
            Format::Repeat(a) | Format::Repeat1(a) => {
                if a.is_nullable(module) {
                    // fails to compile whatever the continuation
                    return true;
                }
                let inner_next = Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone()));
                let fa = Format::Tuple(vec![(**a).clone(), Format::Repeat(a.clone())]);
                self.format_depends_on_next(module, a, inner_next)
                    || MatchTree::reaches_probe(module, &[fa, Format::EMPTY], next)
            }
            Format::RepeatCount(_, a)
            | Format::RepeatUntilLast(_, a)
            | Format::RepeatUntilSeq(_, a)
            | Format::ForEach(_, _, a)
            | Format::AccumUntil(_, _, _, _, a) => {
                let inner_next = Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next));
                self.format_depends_on_next(module, a, inner_next)
            }
            Format::RepeatBetween(range, a) => {
                let (min, max) = (range.min(), range.max());
                let inner_next = Rc::new(Next::RepeatBetween(
                    min.saturating_sub(1),
                    max.saturating_sub(1),
                    MaybeTyped::Untyped(a),
                    next.clone(),
                ));
                let branches: Vec<Format> = (0..=max)
                    .map(|count| Format::RepeatCount(Box::new(Expr::U32(count as u32)), a.clone()))
                    .collect();
                self.format_depends_on_next(module, a, inner_next)
                    || MatchTree::reaches_probe(module, &branches, next)
            }
            Format::LetFormat(first, _, second) | Format::MonadSeq(first, second) => {
                let first_next = Next::cat(MaybeTyped::Untyped(second), next.clone());
                self.format_depends_on_next(module, first, first_next)
                    || self.format_depends_on_next(module, second, next)
            }
        }
    }
}

/// A single choice-point in a conceptual [MatchTree] structure.
#[derive(Clone, Debug)]
struct MatchTreeStep<'a> {
    accept: bool,
    branches: Vec<(ByteSet, Rc<Next<'a>>)>,
}

/// The superposition of choice-points at a common descent-depth into a conceptual [MatchTree] structure.
#[derive(Clone, Debug)]
struct MatchTreeLevel<'a> {
    accept: Option<usize>,
    branches: Vec<(ByteSet, LevelBranch<'a>)>,
}

/// A bundle of follow-sets with an externally significant index, e.g. into an array of decoders
type LevelBranch<'a> = HashSet<(usize, Rc<Next<'a>>)>;

thread_local! {
    // `Format` is not `Sync`, so a `'static` instance is leaked once per thread instead.
    static ANY_BYTE: &'static Format = Box::leak(Box::new(Format::Byte(ByteSet::full())));
}

/// Any single byte, for over-approximating continuations (see [`MatchTreeStep::any_bytes`]).
fn any_byte() -> &'static Format {
    ANY_BYTE.with(|f| *f)
}

/// Maximum number of bytes of lookahead a [`MatchTree`] examines past its decision point.
const MATCH_TREE_MAX_DEPTH: usize = 80;

/// A byte-level prefix-tree evaluated to a fixed depth.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
}

/// Alias used for semantic transparency of the `usize` value used to identify `Rc<Next>`-values within [`CycleGuard`]
type NextPtrAddr = usize;

/// Tracks which `Format::ItemVar`/`TypedFormat::FormatCall` levels are currently being eagerly
/// expanded within a single top-level [`MatchTreeStep`] construction (one iteration of
/// [`MatchTreeLevel::try_grow`]'s per-index loop), so that construction terminates instead of
/// recursing forever on a self-referential format.
///
/// A level is added to `open_levels` immediately before recursing into [`MatchTreeStep::from_format`]
/// on `module.get_format(level)`, and is removed just after that call returns.
///
/// The walk of a level's body only ever reaches the continuation it was opened with (`exit`) once
/// the body itself has been fully matched, so whenever [`MatchTreeStep::from_next`] is called on
/// exactly that continuation, the level is closed for the duration of that call (see
/// [`Self::leave`]). A later reference to the same level from within that continuation - e.g. the
/// second `B` in `B ~ B ~ 'x'` with a nullable `B` - is then a sibling, not a re-entry. Exits are
/// matched by `Rc` pointer identity: the continuation stays alive for as long as its level is open,
/// so no other live `Next` can share its address, and a structurally equal but separately
/// allocated continuation is simply not recognized as an exit (conservative, never unsound).
///
/// If a level would be added to `open_levels` for the second time, then there must be a zero-progress cycle
/// (provided that a fresh `CycleGuard` is instantiated for each separate pass within [`MatchTreeLevel::try_grow`]).
/// When this happens, `has_cycle` is set to true (and `cycle_level` to the re-entered level), and the
/// return value can be arbitrarily chosen as `MatchTreeStep::reject()`; the calling
/// `MatchTreeLevel::try_grow` is responsible for checking `has_cycle`, and if it is set, then
/// [`MatchTreeError::Cycle`] is returned.
#[derive(Default)]
struct CycleGuard {
    /// Each open level, mapped to the address of the continuation (`exit`) it was opened with.
    open_levels: HashMap<usize, NextPtrAddr>,
    has_cycle: bool,
    /// The first level found to be re-entered, if any.
    cycle_level: Option<usize>,
    /// Whether [`Next::Probe`] was consulted.
    reached_probe: bool,
}

impl CycleGuard {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// The address identifying `next` as some open level's `exit`.
    fn exit_addr(next: &Rc<Next<'_>>) -> NextPtrAddr {
        Rc::as_ptr(next) as *const () as usize
    }

    /// Inserts `level` into the open-set, with `exit` the continuation its body is walked under,
    /// updating `has_cycle` based on whether the insertion was novel (i.e. setting it to `true`
    /// if the `level` was already open).
    ///
    /// Returns the value of `self.has_cycle`.
    fn open(&mut self, level: usize, exit: &Rc<Next<'_>>) -> bool {
        if self.open_levels.contains_key(&level) {
            self.has_cycle = true;
            self.cycle_level.get_or_insert(level);
        } else {
            self.open_levels.insert(level, Self::exit_addr(exit));
        }
        self.has_cycle
    }

    /// Closes, and returns, every open level whose `exit` is `next`: the walk is leaving those
    /// levels' bodies. [`Self::reenter`] must be called with the result once the walk of `next`
    /// returns, since other paths through those bodies may still be explored afterwards.
    fn leave(&mut self, next: &Rc<Next<'_>>) -> Vec<(usize, NextPtrAddr)> {
        if self.open_levels.is_empty() {
            return Vec::new();
        }
        let addr = Self::exit_addr(next);
        let left: Vec<(usize, NextPtrAddr)> = self
            .open_levels
            .iter()
            .filter(|(_, exit)| **exit == addr)
            .map(|(level, exit)| (*level, *exit))
            .collect();
        for (level, _) in &left {
            self.open_levels.remove(level);
        }
        left
    }

    /// Reopens the levels returned by [`Self::leave`].
    fn reenter(&mut self, left: Vec<(usize, NextPtrAddr)>) {
        self.open_levels.extend(left);
    }

    /// Marks `level` as closed, removing it from the open-set.
    ///
    /// This should be called after returning from a recursive call to [`MatchTreeStep::from_format`] on `module.get_format(level)`.
    fn close(&mut self, level: usize) {
        self.open_levels.remove(&level);
    }
}

impl<'a> MatchTreeStep<'a> {
    /// Returns a `MatchTreeStep` that rejects all inputs without branching.
    fn reject() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![],
        }
    }

    /// Returns a `MatchTreeStep` that accepts all inputs without branching.
    fn accept() -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: true,
            branches: vec![],
        }
    }

    /// Constructs a `MatchTreeStep` consisting of a single branch, defined by the argument values.
    fn branch(bs: ByteSet, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        MatchTreeStep {
            accept: false,
            branches: vec![(bs, next)],
        }
    }

    /// Modifies a `MatchTreeStep` in place, so that it will accept a new branch given by the argument values.
    fn union_branch(&mut self, mut bs: ByteSet, next: Rc<Next<'a>>) {
        let mut branches = Vec::new();
        for (bs0, next0) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    branches.push((orig, next0.clone()));
                }
                *bs0 = common;
                *next0 = Rc::new(Next::Union(next0.clone(), next.clone()));
                bs = bs.difference(bs0);
            }
        }
        if !bs.is_empty() {
            self.branches.push((bs, next));
        }
        self.branches.append(&mut branches);
    }

    /// Combines two `MatchTreeSteps` into their logical union
    fn union(mut self, other: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        self.accept = self.accept || other.accept;
        for (bs, next) in other.branches {
            self.union_branch(bs, next);
        }
        self
    }

    /// Returns a modified version of `self` that rejects any input that is not
    /// accepted by `peek`.
    fn peek(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        if peek.accept {
            // can ignore peek as it has already accepted
        } else if self.accept {
            // can ignore self as it has already accepted
            self.accept = peek.accept;
            self.branches = peek.branches;
        } else {
            // take the intersection of peek and self branches
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches {
                for (bs2, next2) in &peek.branches {
                    let bs = bs1.intersection(bs2);
                    if !bs.is_empty() {
                        let next = Rc::new(Next::Peek(next1.clone(), next2.clone()));
                        branches.push((bs, next));
                    }
                }
            }
            self.branches = branches;
        }
        self
    }

    /// Returns a modified version of `self` that rejects any input that is
    /// accepted by `peek`.
    fn peek_not(mut self, peek: MatchTreeStep<'a>) -> MatchTreeStep<'a> {
        if peek.accept {
            self.accept = false;
            self.branches = Vec::new();
        } else {
            let mut branches = Vec::new();
            for (bs1, next1) in self.branches.into_iter() {
                let mut diff = bs1;
                for (bs2, next2) in &peek.branches {
                    let common = bs1.intersection(bs2);
                    if !common.is_empty() {
                        let next = Rc::new(Next::PeekNot(next1.clone(), next2.clone()));
                        branches.push((common, next));
                    }
                    diff = diff.difference(bs2);
                }
                if !diff.is_empty() {
                    branches.push((diff, next1.clone()));
                }
            }
            self.branches = branches;
        }
        self
    }

    /// Constructs a [MatchTreeStep] that accepts a given series of sequential formats, with a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_sequential(
        module: &'a FormatModule,
        fields: &'a [Format],
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::from_next(module, next, guard),
            Some((f, fs)) => Self::from_format(
                module,
                f,
                Rc::new(Next::Sequence(MaybeTyped::Untyped(fs), next)),
                guard,
            ),
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a fixed-count repetition of a given format, with a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_repeat_count(
        module: &'a FormatModule,
        n: usize,
        format: &'a Format,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            Self::from_format(
                module,
                format,
                Rc::new(Next::RepeatCount(n - 1, MaybeTyped::Untyped(format), next)),
                guard,
            )
        } else {
            Self::from_next(module, next, guard)
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a repetition whose count is bounded above and below, with a trailing sequence of partially-consumed formats ([`Next`]s)
    ///
    /// The format in question will repeat an arbitrary number of times between `min` and `max`, where `min_max ::= (min, max)`
    ///
    /// Presupposes that the invariant `max >= min` is upheld.
    fn from_repeat_between(
        module: &'a FormatModule,
        min_max: (usize, usize),
        format: &'a Format,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        let (min, max) = min_max;
        assert!(
            min <= max,
            "min-max pair ({min}, {max}) incoherent (min > max)",
        );
        if min == max {
            Self::from_repeat_count(module, min, format, next, guard)
        } else if min > 0 {
            Self::from_format(
                module,
                format,
                Rc::new(Next::RepeatBetween(
                    min - 1,
                    max - 1,
                    MaybeTyped::Untyped(format),
                    next,
                )),
                guard,
            )
        } else {
            Self::from_next(
                module,
                Rc::new(Next::RepeatMax(
                    max,
                    MaybeTyped::Untyped(format),
                    next.clone(),
                )),
                guard,
            )
        }
    }

    /// Constructs a [MatchTreeStep] from a given (partial) format `inner` and a slice-length `n`
    pub fn from_slice(
        module: &'a FormatModule,
        n: usize,
        inner: Rc<Next<'a>>,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            let mut tree = Self::from_next(module, inner, guard);
            tree.accept = false;
            if tree.branches.is_empty() {
                let next = Rc::new(Next::Slice(n - 1, Rc::new(Next::Empty), next.clone()));
                tree.branches.push((ByteSet::full(), next));
            } else {
                for (_bs, inside) in tree.branches.iter_mut() {
                    *inside = Rc::new(Next::Slice(n - 1, inside.clone(), next.clone()));
                }
            }
            tree
        } else {
            Self::from_next(module, next.clone(), guard)
        }
    }

    /// Constructs a [MatchTreeStep] from a [`Next`]
    fn from_next(
        module: &'a FormatModule,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        let left = guard.leave(&next);
        let step = Self::from_next_inner(module, next, guard);
        guard.reenter(left);
        step
    }

    /// The body of [`Self::from_next`], once any levels exited by `next` have been closed.
    fn from_next_inner(
        module: &'a FormatModule,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        match next.as_ref() {
            Next::Empty => Self::accept(),
            Next::Probe => {
                guard.reached_probe = true;
                Self::accept()
            }
            Next::Union(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone(), guard);
                let tree2 = Self::from_next(module, next2.clone(), guard);
                tree1.union(tree2)
            }
            Next::Cat(f, next) => {
                let next0: Rc<Next<'a>> = next.clone();
                MatchTreeStep::<'a>::from_mt_format(module, *f, next0, guard)
            }
            Next::Sequence(fields, next) => {
                let next = next.clone();
                match fields {
                    MaybeTyped::Untyped(fields) => match fields.split_first() {
                        None => Self::from_next(module, next, guard),
                        Some((f, fs)) => Self::from_format(
                            module,
                            f,
                            Rc::new(Next::Sequence(MaybeTyped::Untyped(fs), next)),
                            guard,
                        ),
                    },
                    MaybeTyped::Typed(fields) => match fields.split_first() {
                        None => Self::from_next(module, next, guard),
                        Some((f, fs)) => Self::from_gt_format(
                            module,
                            f,
                            Rc::new(Next::Sequence(MaybeTyped::Typed(fs), next)),
                            guard,
                        ),
                    },
                }
            }
            Next::Repeat(a, next0) => {
                let tree = MatchTreeStep::<'a>::from_next(module, next0.clone(), guard);
                let next1 = next.clone();
                tree.union(MatchTreeStep::<'a>::from_mt_format(
                    module, *a, next1, guard,
                ))
            }
            Next::RepeatBetween(n, m, a, next0) => {
                let min = *n;
                let max = *m;
                if min == max {
                    log::warn!("RepeatBetween({min}, {max}) converted to RepeatCount({min})");
                    let next1 = Next::RepeatCount(min, *a, next0.clone());
                    return Self::from_next(module, Rc::new(next1), guard);
                }
                if min > 0 {
                    Self::from_mt_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatBetween(min - 1, max - 1, *a, next0.clone())),
                        guard,
                    )
                } else {
                    Self::from_next(
                        module,
                        Rc::new(Next::RepeatMax(max, *a, next0.clone())),
                        guard,
                    )
                }
            }
            Next::RepeatMax(n, a, next0) => {
                let n = *n;
                if n == 0 {
                    Self::from_next(module, next0.clone(), guard)
                } else {
                    let tree0 = MatchTreeStep::<'a>::from_next(module, next0.clone(), guard);
                    tree0.union(MatchTreeStep::<'a>::from_mt_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatMax(n - 1, *a, next0.clone())),
                        guard,
                    ))
                }
            }
            Next::RepeatCount(n, a, next0) => {
                let n = *n;
                let next = next0.clone();
                if n > 0 {
                    Self::from_mt_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatCount(n - 1, *a, next)),
                        guard,
                    )
                } else {
                    Self::from_next(module, next, guard)
                }
            }
            Next::Slice(n, inside, next0) => {
                Self::from_slice(module, *n, inside.clone(), next0.clone(), guard)
            }
            Next::Peek(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone(), guard);
                let tree2 = Self::from_next(module, next2.clone(), guard);
                tree1.peek(tree2)
            }
            Next::PeekNot(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone(), guard);
                let tree2 = Self::from_next(module, next2.clone(), guard);
                tree1.peek_not(tree2)
            }
        }
    }

    pub fn from_mt_format(
        module: &'a FormatModule,
        f: MTFormatRef<'a>,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        match f {
            MaybeTyped::Untyped(f) => Self::from_format(module, f, next, guard),
            MaybeTyped::Typed(tf) => Self::from_gt_format(module, tf, next, guard),
        }
    }

    /// Internal helper to dedup and simplify the common ItemVar/FormatCall handling between `from_format` and `from_gt_format`
    ///
    /// Opens `level` in `guard`, returning `Self::reject` if `level` was already open (a zero-progress cycle; see
    /// [`CycleGuard`]). Otherwise, computes `f(guard)`, yielding its value after marking `level` as closed.
    ///
    /// # Notes
    ///
    /// `exit` is the Rc-wrapped [`Next`] continuation immediately following the `ItemVar`/`FormatCall` whose level we are
    /// guarding descent into. It is recorded (as a [`NextPtrAddr`]) so that [`Self::from_next`] can close `level` once the
    /// walk leaves its body and continues into `exit`. It plays no part in detecting a cycle, which depends on `level`
    /// alone.
    ///
    /// While `level` is open, no byte has been consumed since entering it (the walk stops at the first byte), so reaching
    /// `level` again means re-entering it from within its own body without progress, i.e. left recursion. A sibling
    /// reference to the same, possibly zero-width, level (the second `B` in `B ~ B ~ 'x'`) is only reached through
    /// `exit`, by which point `level` has been closed.
    ///
    /// This guard serves only to prevent left recursion; formats it accepts may still fail elsewhere (e.g. as ambiguous).
    fn guarded<F>(level: usize, exit: &Rc<Next<'a>>, guard: &mut CycleGuard, f: F) -> Self
    where
        F: FnOnce(&mut CycleGuard) -> Self,
    {
        if guard.open(level, exit) {
            return Self::reject();
        }
        let step = f(guard);
        guard.close(level);
        step
    }

    pub fn from_gt_format(
        module: &'a FormatModule,
        f: &'a TypedFormat<GenType>,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        match f {
            TypedFormat::FormatCall(_, level, ..) => {
                Self::guarded(*level, &next.clone(), guard, move |guard| {
                    Self::from_format(module, module.get_format(*level), next, guard)
                })
            }
            TypedFormat::Fail => Self::reject(),
            TypedFormat::EndOfInput => Self::accept(),
            TypedFormat::Align(n) => Self::from_align(module, next, *n, guard),
            TypedFormat::Phantom(..) => Self::from_next(module, next, guard),
            TypedFormat::SkipRemainder => Self::any_bytes(module, Rc::new(Next::Empty), guard),
            TypedFormat::Byte(bs) => Self::branch(*bs, next),
            TypedFormat::Variant(_, _label, f) => {
                Self::from_gt_format(module, f, next.clone(), guard)
            }
            TypedFormat::Union(_, branches) | TypedFormat::UnionNondet(_, branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_gt_format(module, f, next.clone(), guard));
                }
                tree
            }
            TypedFormat::Sequence(_, fields) | TypedFormat::Tuple(_, fields) => {
                match fields.split_first() {
                    None => Self::from_next(module, next, guard),
                    Some((f, fs)) => Self::from_gt_format(
                        module,
                        f,
                        Rc::new(Next::Sequence(MaybeTyped::Typed(fs), next)),
                        guard,
                    ),
                }
            }
            TypedFormat::Repeat(_, a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                tree.union(Self::from_gt_format(
                    module,
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                    guard,
                ))
            }
            TypedFormat::ForEach(_, _expr, _lbl, a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                // FIXME - we might want a more robust solution to nullable formats in ForEach
                let bounds = a.match_bounds();
                if bounds.min() == 0 {
                    match bounds.max() {
                        Some(0) => tree,
                        _ => Self::any_bytes(module, next, guard),
                    }
                } else {
                    tree.union(Self::from_gt_format(
                        module,
                        a,
                        Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                        guard,
                    ))
                }
            }
            TypedFormat::Repeat1(_, a) => Self::from_gt_format(
                module,
                a,
                Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                guard,
            ),
            TypedFormat::RepeatCount(_, expr, a) => {
                let bounds = expr.bounds();
                if let Some(n) = bounds.as_exact() {
                    {
                        let next = next.clone();
                        if n > 0 {
                            Self::from_gt_format(
                                module,
                                a,
                                Rc::new(Next::RepeatCount(n - 1, MaybeTyped::Typed(a), next)),
                                guard,
                            )
                        } else {
                            Self::from_next(module, next, guard)
                        }
                    }
                } else {
                    {
                        let n = bounds.min;
                        let next = Rc::new(Next::Empty);
                        if n > 0 {
                            Self::from_gt_format(
                                module,
                                a,
                                Rc::new(Next::RepeatCount(n - 1, MaybeTyped::Typed(a), next)),
                                guard,
                            )
                        } else {
                            Self::from_next(module, next, guard)
                        }
                    }
                }
            }
            TypedFormat::RepeatBetween(_, range, a) => {
                let (min, max) = (range.min(), range.max());
                if min == max {
                    let next = next.clone();
                    if min > 0 {
                        Self::from_gt_format(
                            module,
                            a,
                            Rc::new(Next::RepeatCount(min - 1, MaybeTyped::Typed(&**a), next)),
                            guard,
                        )
                    } else {
                        Self::from_next(module, next, guard)
                    }
                } else if min > 0 {
                    Self::from_gt_format(
                        module,
                        a,
                        Rc::new(Next::RepeatBetween(
                            min - 1,
                            max - 1,
                            MaybeTyped::Typed(&**a),
                            next,
                        )),
                        guard,
                    )
                } else {
                    Self::from_next(
                        module,
                        Rc::new(Next::RepeatMax(max, MaybeTyped::Typed(&**a), next.clone())),
                        guard,
                    )
                }
            }
            // as for `Format::RepeatUntilLast`/`RepeatUntilSeq`/`AccumUntil` in `from_format`
            TypedFormat::RepeatUntilLast(_, _expr, a) => {
                if a.is_nullable() {
                    Self::any_bytes(module, next, guard)
                } else {
                    Self::from_gt_format(
                        module,
                        a,
                        Rc::new(Next::Repeat(MaybeTyped::Typed(a), next)),
                        guard,
                    )
                }
            }
            TypedFormat::RepeatUntilSeq(_, _, a) | TypedFormat::AccumUntil(.., a) => {
                if a.is_nullable() {
                    Self::any_bytes(module, next, guard)
                } else {
                    Self::from_next(
                        module,
                        Rc::new(Next::Repeat(MaybeTyped::Typed(a), next)),
                        guard,
                    )
                }
            }
            TypedFormat::Maybe(_, _cond, a) => {
                let tree_some = Self::from_gt_format(module, a, next.clone(), guard);
                let tree_none = Self::from_next(module, next, guard);
                tree_none.union(tree_some)
            }
            TypedFormat::LiftedOption(_, None) => Self::from_next(module, next, guard),
            TypedFormat::LiftedOption(_, Some(a)) => Self::from_gt_format(module, a, next, guard),
            TypedFormat::Peek(_, a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                let peek = Self::from_gt_format(module, a, Rc::new(Next::Empty), guard);
                tree.peek(peek)
            }
            TypedFormat::PeekNot(_, a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                let peek = Self::from_gt_format(module, a, Rc::new(Next::Empty), guard);
                tree.peek_not(peek)
            }
            TypedFormat::Slice(_, expr, f) => {
                let inside = Rc::new(Next::Cat(
                    MaybeTyped::Typed(f.as_ref()),
                    Rc::new(Next::Empty),
                ));
                let bounds = expr.bounds();
                if let Some(n) = bounds.as_exact() {
                    Self::from_slice(module, n, inside, next, guard)
                } else {
                    Self::from_slice(module, bounds.min, inside, Rc::new(Next::Empty), guard)
                }
            }
            TypedFormat::Bits(_, _a) => Self::any_bytes(module, next, guard),
            TypedFormat::WithRelativeOffset(_, _base_addr, _offset, _a) => {
                Self::from_next(module, next, guard)
            }
            TypedFormat::Map(_, f, _) | TypedFormat::Where(_, f, _) => {
                Self::from_gt_format(module, f, next, guard)
            }

            TypedFormat::DecodeBytes(..)
            | TypedFormat::ParseFromView(..)
            | TypedFormat::Compute(..) => Self::from_next(module, next, guard),

            TypedFormat::Pos(_) => Self::from_next(module, next, guard),
            TypedFormat::Let(_, _name, _expr, f) => Self::from_gt_format(module, f, next, guard),
            TypedFormat::LetView(_, _name, f) => Self::from_gt_format(module, f, next, guard),
            TypedFormat::Match(_, _, branches) => {
                let mut tree = Self::reject();
                for (_, f) in branches {
                    tree = tree.union(Self::from_gt_format(module, f, next.clone(), guard));
                }
                tree
            }
            TypedFormat::Dynamic(_, _name, _expr, f) => {
                Self::from_gt_format(module, f, next, guard)
            }
            TypedFormat::Apply(..) => Self::any_bytes(module, next, guard),
            TypedFormat::MonadSeq(_, f0, f) | TypedFormat::LetFormat(_, f0, _, f) => {
                let next0 = Rc::new(Next::Cat(MaybeTyped::Typed(f), next));
                Self::from_gt_format(module, f0, next0, guard)
            }
            TypedFormat::Hint(_, _hint, f) => Self::from_gt_format(module, f, next, guard),
            TypedFormat::Permit(_, f, _expr) => Self::from_gt_format(module, f, next, guard),
            #[cfg(feature = "format_enforce")]
            TypedFormat::Enforce(_, f) => Self::from_gt_format(module, f, next, guard),
            TypedFormat::WithView(_, _ident, _vf) => Self::from_next(module, next, guard),
        }
    }

    /// Constructs a [MatchTreeStep] from an Format and a trailing [`Next`] value
    pub fn from_format(
        module: &'a FormatModule,
        f: &'a Format,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        match f {
            Format::ItemVar(level, _args, _views) => {
                let level = *level;
                Self::guarded(level, &next.clone(), guard, move |guard| {
                    Self::from_format(module, module.get_format(level), next, guard)
                })
            }
            Format::RecVar(_) => unreachable!(
                "Format::RecVar is rewritten to ItemVar at batch registration; never appears in a stored Format"
            ),
            // never decoded, so consumes nothing
            Format::Phantom(..) => Self::from_next(module, next, guard),
            Format::Fail => Self::reject(),
            Format::EndOfInput => Self::accept(),
            // consumes whatever remains, which only the empty continuation can follow
            Format::SkipRemainder => Self::any_bytes(module, Rc::new(Next::Empty), guard),
            Format::Align(n) => Self::from_align(module, next, *n, guard),
            Format::DecodeBytes(_bytes, _f) => Self::from_next(module, next, guard),
            Format::ParseFromView(_view, _f) => Self::from_next(module, next, guard),
            Format::Byte(bs) => Self::branch(*bs, next),
            Format::Variant(_label, f) => Self::from_format(module, f, next, guard),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone(), guard));
                }
                tree
            }
            Format::Sequence(fields) | Format::Tuple(fields) => {
                Self::from_sequential(module, fields, next, guard)
            }
            Format::Repeat(a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                tree.union(Self::from_format(
                    module,
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                    guard,
                ))
            }
            Format::ForEach(_expr, _lbl, a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                // FIXME - we might want a more robust solution to nullable formats in ForEach
                let bounds = a.match_bounds(module);
                if bounds.min() == 0 {
                    match bounds.max() {
                        Some(0) => tree,
                        // `a*` cannot be walked when `a` is nullable
                        _ => Self::any_bytes(module, next, guard),
                    }
                } else {
                    tree.union(Self::from_format(
                        module,
                        a,
                        Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                        guard,
                    ))
                }
            }
            Format::Repeat1(a) => Self::from_format(
                module,
                a,
                Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                guard,
            ),
            Format::RepeatCount(expr, a) => {
                let bounds = expr.bounds();
                if let Some(n) = bounds.as_exact() {
                    Self::from_repeat_count(module, n, a, next.clone(), guard)
                } else {
                    Self::from_repeat_count(module, bounds.min, a, Rc::new(Next::Empty), guard)
                }
            }
            Format::RepeatBetween(range, a) => Self::from_repeat_between(
                module,
                (range.min(), range.max()),
                a,
                next.clone(),
                guard,
            ),
            // when to stop is decided by the values read, so these are modelled by the repetitions
            // they could make: at least one `a` for `RepeatUntilLast`, any number for the others
            Format::RepeatUntilLast(_expr, a) => {
                if a.is_nullable(module) {
                    Self::any_bytes(module, next, guard)
                } else {
                    Self::from_format(
                        module,
                        a,
                        Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next)),
                        guard,
                    )
                }
            }
            Format::RepeatUntilSeq(_, a) | Format::AccumUntil(.., a) => {
                if a.is_nullable(module) {
                    Self::any_bytes(module, next, guard)
                } else {
                    Self::from_next(
                        module,
                        Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next)),
                        guard,
                    )
                }
            }
            Format::Maybe(_expr, a) => {
                let tree_some = Self::from_format(module, a, next.clone(), guard);
                let tree_none = Self::from_next(module, next, guard);
                tree_some.union(tree_none)
            }
            Format::Peek(a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                let peek = Self::from_format(module, a, Rc::new(Next::Empty), guard);
                tree.peek(peek)
            }
            Format::PeekNot(a) => {
                let tree = Self::from_next(module, next.clone(), guard);
                let peek = Self::from_format(module, a, Rc::new(Next::Empty), guard);
                tree.peek_not(peek)
            }
            Format::Slice(expr, f) => {
                let inside = Rc::new(Next::Cat(
                    MaybeTyped::Untyped(f.as_ref()),
                    Rc::new(Next::Empty),
                ));
                let bounds = expr.bounds();
                if let Some(n) = bounds.as_exact() {
                    Self::from_slice(module, n, inside, next, guard)
                } else {
                    Self::from_slice(module, bounds.min, inside, Rc::new(Next::Empty), guard)
                }
            }
            // bit-level, so has no byte-level language to model
            Format::Bits(_a) => Self::any_bytes(module, next, guard),
            // reads elsewhere, consuming nothing here
            Format::WithRelativeOffset(_addr, _offset, _a) => Self::from_next(module, next, guard),
            Format::Map(f, _expr) => Self::from_format(module, f, next, guard),
            Format::Where(f, _expr) => Self::from_format(module, f, next, guard),
            Format::Pos => Self::from_next(module, next, guard),
            Format::Compute(_expr) => Self::from_next(module, next, guard),
            Format::Let(_name, _expr, f) => Self::from_format(module, f, next, guard),
            Format::LetView(_name, f) => {
                // FIXME - does the construction of a view-binding affect our matchtree?
                Self::from_format(module, f, next, guard)
            }
            // REVIEW - it isn't fully clear whether Permit should affect the matchtree in some way...
            Format::Permit(f, _) => Self::from_format(module, f, next, guard),
            #[cfg(feature = "format_enforce")]
            Format::Enforce(f) => Self::from_format(module, f, next, guard),
            Format::Match(_, branches) => {
                let mut tree = Self::reject();
                for (_, f) in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone(), guard));
                }
                tree
            }
            Format::Dynamic(_name, _expr, f) => Self::from_format(module, f, next, guard),
            // a dynamically built format, whose language is unknown here
            Format::Apply(_name) => Self::any_bytes(module, next, guard),
            Format::MonadSeq(f0, f) | Format::LetFormat(f0, _, f) => {
                let next0 = Rc::new(Next::Cat(MaybeTyped::Untyped(f), next));
                Self::from_format(module, f0, next0, guard)
            }
            Format::Hint(_hint, f) => Self::from_format(module, f, next, guard),
            Format::LiftedOption(None) => Self::from_next(module, next, guard),
            Format::LiftedOption(Some(f)) => Self::from_format(module, f, next, guard),
            // REVIEW - is this a sound implementation?
            Format::WithView(_ident, _vf) => Self::from_next(module, next, guard),
        }
    }

    /// Constructs a [MatchTreeStep] that matches the various possible align-offset versions of `next`, for small enough `n`,
    /// and otherwise fudges the return value with a universal-acceptance.
    ///
    /// NOTE - currently 'small enough' just means that `n` is `1` (irrefutable as an alignment modulus). `n == 0` has no
    /// valid interpretation, but is unrepresentable (`n: NonZeroUsize`) rather than an illegal value to check for here.
    fn from_align(
        module: &'a FormatModule,
        next: Rc<Next<'a>>,
        n: NonZeroUsize,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        match n.get() {
            1 => Self::from_next(module, next, guard), // guaranteed to already be in alignment
            // skips up to `n - 1` bytes, depending on the position
            n => Self::from_next(
                module,
                Rc::new(Next::RepeatMax(
                    n - 1,
                    MaybeTyped::Untyped(any_byte()),
                    next,
                )),
                guard,
            ),
        }
    }

    /// Over-approximates a format whose byte-level language cannot be modelled here as `Σ*`,
    /// followed by `next`. A `MatchTree` can then never tell it apart from an alternative by what
    /// the alternative reads, so such a choice is rejected (as ambiguous or needing too much
    /// lookahead) rather than settled by treating the format as matching nothing.
    fn any_bytes(
        module: &'a FormatModule,
        next: Rc<Next<'a>>,
        guard: &mut CycleGuard,
    ) -> MatchTreeStep<'a> {
        Self::from_next(
            module,
            Rc::new(Next::Repeat(MaybeTyped::Untyped(any_byte()), next)),
            guard,
        )
    }
}

/// Reason why [`MatchTreeLevel::try_grow`] (and hence [`MatchTree::try_build`]) failed.
#[derive(Clone, Debug, PartialEq, Eq)]
enum MatchTreeError {
    /// A zero-progress cycle through `level` was detected by the [`CycleGuard`].
    Cycle { level: usize },
    /// Two distinct indices both accept at the same point, so no amount of lookahead can decide
    /// between them.
    Ambiguous { accepted: usize, conflicting: usize },
    /// The lookahead-depth limit was reached without a decisive choice of index.
    DepthExhausted,
}

impl MatchTreeError {
    /// Describes why construction failed, naming the level of a cycle as registered in `module`.
    fn describe(&self, module: &FormatModule) -> String {
        match self {
            MatchTreeError::Cycle { level } => format!(
                "zero-progress recursive cycle through `{}`",
                module.get_name(*level)
            ),
            MatchTreeError::Ambiguous {
                accepted,
                conflicting,
            } => format!(
                "alternatives {accepted} and {conflicting} both accept the same input, so no \
                 amount of lookahead can decide between them"
            ),
            MatchTreeError::DepthExhausted => format!(
                "alternatives cannot be told apart within {MATCH_TREE_MAX_DEPTH} bytes of lookahead"
            ),
        }
    }
}

impl<'a> MatchTreeLevel<'a> {
    /// Constructs a `MatchTreeLevel` that unconditionally rejects all inputs without branching.
    fn reject() -> MatchTreeLevel<'a> {
        MatchTreeLevel {
            accept: None,
            branches: vec![],
        }
    }

    /// Attempts to modify `self` such that `index` is marked as the unique index of the accepting format.
    ///
    /// Returns `Err(i)` if a different index `i` was already marked as accepting, and `Ok(())` otherwise.
    fn merge_accept(&mut self, index: usize) -> Result<(), usize> {
        match self.accept {
            None => {
                self.accept = Some(index);
                Ok(())
            }
            Some(i) if i == index => Ok(()),
            Some(i) => Err(i),
        }
    }

    /// Adds a new branch to `self` using a predicate byte-set and its associated follow-set,
    fn merge_branch(&mut self, index: usize, mut bs: ByteSet, next: Rc<Next<'a>>) {
        let mut new_branches = Vec::new();
        // For each bs0, nexts in the extant branches of `self`:
        for (bs0, nexts) in self.branches.iter_mut() {
            let common = bs0.intersection(&bs);
            // If bs and bs0 are not disjoint:
            if !common.is_empty() {
                let orig = bs0.difference(&bs);
                if !orig.is_empty() {
                    // 1. Enqueue a branch predicated on `bs0 - bs` with an inherited follow-set
                    new_branches.push((orig, nexts.clone()));
                }
                // 2. Leave behind a branch predicated on `bs0 & bs`
                *bs0 = common;
                // 2a. Add the `next` parameter to the follow-set of the existing branch we modified in-place
                nexts.insert((index, next.clone()));
                // 3. Remove all bytes from `bs` that are now covered by the branch we modified in-place
                bs = bs.difference(bs0);
            }
        }
        // If any bytes of bs were completely unique among all extant branches:
        if !bs.is_empty() {
            // 1. Create a novel branch with the follow-set implied by the `next` parameter
            let mut nexts = HashSet::new();
            nexts.insert((index, next.clone()));
            self.branches.push((bs, nexts));
        }
        // Append all enqueued branches from the iteration above
        self.branches.append(&mut new_branches);
    }

    /// Extends the set of choice-points and follow-sets of `self` with a provided [`MatchTreeStep`].
    fn merge_step(
        mut self,
        index: usize,
        step: MatchTreeStep<'a>,
    ) -> Result<MatchTreeLevel<'a>, MatchTreeError> {
        if step.accept {
            self.merge_accept(index)
                .map_err(|accepted| MatchTreeError::Ambiguous {
                    accepted,
                    conflicting: index,
                })?;
        }
        for (bs, next) in step.branches {
            self.merge_branch(index, bs, next);
        }
        Ok(self)
    }

    /// Attempt to construct and return a `MatchTree` that unconditionally accepts
    /// the same, common format-index as all elements of the set `nexts`.
    ///
    /// If `nexts` is empty, the `MatchTree` returned will instead reject all input
    ///
    /// If `nexts` contains multiple associated indices, returns `None`
    fn accepts(nexts: &LevelBranch<'a>) -> Option<MatchTree> {
        let mut tree = Self::reject();
        for (i, _next) in nexts.iter() {
            tree.merge_accept(*i).ok()?;
        }
        Some(MatchTree {
            accept: tree.accept,
            branches: vec![],
        })
    }

    /// Attempts to accumulate a `MatchTree` recursively up to an overall depth of `depth` layers,
    /// with the immediate layer constructed based on a bundle of indexed choice-points ([`LevelBranch`]).
    ///
    /// If the depth limit has been reached without a decisive choice of which index to accept,
    /// returns [`MatchTreeError::DepthExhausted`]; if two indices accept at the same point,
    /// [`MatchTreeError::Ambiguous`]; and if a zero-progress cycle is detected,
    /// [`MatchTreeError::Cycle`].
    ///
    /// Otherwise, returns a `MatchTree` that is guaranteed to decide on a unique branch for
    /// all input within at most `depth` bytes of lookahead.
    ///
    /// Sets `reached_probe` if any step consults [`Next::Probe`].
    fn try_grow(
        module: &'a FormatModule,
        nexts: LevelBranch<'a>,
        depth: usize,
        reached_probe: &mut bool,
    ) -> Result<MatchTree, MatchTreeError> {
        if let Some(tree) = Self::accepts(&nexts) {
            Ok(tree)
        } else if depth > 0 {
            let mut tree = Self::reject();
            let mut tmp = Vec::from_iter(nexts);
            tmp.sort_by_key(|(ix, _)| *ix);
            for (i, next) in tmp.into_iter() {
                let _next = next.clone();
                // instantiate a fresh CycleGuard to detect zero-progress ItemVar cycles in `from_next` call-stack
                let mut guard = CycleGuard::new();
                let subtree = MatchTreeStep::from_next(module, next, &mut guard);
                *reached_probe |= guard.reached_probe;
                if guard.has_cycle {
                    // Upon detecting a zero-progress cycle, we know the format is not well-formed, and we return early to surface the error to the caller.
                    log::error!(
                        "zero-progress recursive cycle detected in format; cannot construct a valid match tree ({next:?})",
                        next = _next.as_ref()
                    );
                    let level = guard
                        .cycle_level
                        .expect("has_cycle is only set alongside cycle_level");
                    return Err(MatchTreeError::Cycle { level });
                }
                tree = tree.merge_step(i, subtree)?;
            }
            let mut branches = Vec::new();
            for (bs, nexts) in tree.branches {
                let t = Self::try_grow(module, nexts, depth - 1, reached_probe)?;
                branches.push((bs, t));
            }
            Ok(MatchTree {
                accept: tree.accept,
                branches,
            })
        } else {
            Err(MatchTreeError::DepthExhausted)
        }
    }
}

impl MatchTree {
    /// Returns the accepting index associated with the input-sequence starting from the current offset of `input`,
    /// looking ahead as many bytes as necessary until a definitive index is found or the lookahead limit is reached.
    ///
    /// Returns `None` if not enough lookahead remains to disambiguate multiple candidate indices.
    fn matches(&self, input: ReadCtxt<'_>) -> Option<usize> {
        match input.read_byte() {
            None => self.accept,
            Some((b, input)) => {
                for (bs, s) in &self.branches {
                    if bs.contains(b) {
                        return s.matches(input);
                    }
                }
                self.accept
            }
        }
    }

    /// Constructs a new `MatchTreeLevel` from an alternation of branches and a follow-set of partially decomposed formats,
    /// to within a fixed but externally opaque lookahead-depth.
    ///
    /// A `FormatModule` is also accepted to contextualize any contextually dependent formats, e.g. [`Format::ItemVar`]
    ///
    /// The compilers use [`Self::build_for`], which reports why construction failed.
    #[cfg(test)]
    fn build(module: &FormatModule, branches: &[Format], next: Rc<Next<'_>>) -> Option<MatchTree> {
        Self::try_build(module, branches, next).ok()
    }

    /// Builds the `MatchTree` for `branches` followed by `next`, or reports why it cannot be built.
    fn try_build(
        module: &FormatModule,
        branches: &[Format],
        next: Rc<Next<'_>>,
    ) -> Result<MatchTree, MatchTreeError> {
        Self::try_build_probed(module, branches, next, &mut false)
    }

    /// Like [`Self::try_build`], but as an error naming `what` (the format whose choice the tree
    /// would decide) along with why construction failed.
    fn build_for(
        module: &FormatModule,
        branches: &[Format],
        next: Rc<Next<'_>>,
        what: impl FnOnce() -> String,
    ) -> AResult<MatchTree> {
        Self::try_build(module, branches, next).map_err(|err| {
            anyhow!(
                "cannot build match tree for {}: {}",
                what(),
                err.describe(module)
            )
        })
    }

    /// Returns `true` if building the `MatchTree` for `branches` followed by `next` could consult
    /// a [`Next::Probe`] within `next`, i.e. if the tree could differ depending on what that probe
    /// stands for. A failed build also counts, since whether it fails can depend on the probe.
    fn reaches_probe(module: &FormatModule, branches: &[Format], next: Rc<Next<'_>>) -> bool {
        let mut reached = false;
        let res = Self::try_build_probed(module, branches, next, &mut reached);
        reached || res.is_err()
    }

    fn try_build_probed(
        module: &FormatModule,
        branches: &[Format],
        next: Rc<Next<'_>>,
        reached_probe: &mut bool,
    ) -> Result<MatchTree, MatchTreeError> {
        let mut nexts = HashSet::new();
        for (i, f) in branches.iter().enumerate() {
            nexts.insert((i, Rc::new(Next::Cat(MaybeTyped::Untyped(f), next.clone()))));
        }
        MatchTreeLevel::try_grow(module, nexts, MATCH_TREE_MAX_DEPTH, reached_probe)
    }
}

pub struct TypeScope<'a, T = ValueType> {
    parent: Option<&'a TypeScope<'a, T>>,
    names: Vec<Label>,
    types: Vec<ValueKind<T>>,
    /// The types each [`ValueType::Rec`] refers to, indexed by level (a `FormatModule`'s
    /// registered format types); shared by every child of the root scope.
    rec_types: &'a [ValueType],
}

impl<'a, T> TypeScope<'a, T> {
    fn new() -> Self {
        Self::with_rec_types(&[])
    }

    /// Constructs a root scope that resolves [`ValueType::Rec`] against `rec_types`.
    fn with_rec_types(rec_types: &'a [ValueType]) -> Self {
        let parent = None;
        let names = Vec::new();
        let types = Vec::new();
        TypeScope {
            parent,
            names,
            types,
            rec_types,
        }
    }

    fn child(parent: &'a TypeScope<'a, T>) -> Self {
        let rec_types = parent.rec_types;
        let parent = Some(parent);
        let names = Vec::new();
        let types = Vec::new();
        TypeScope {
            parent,
            names,
            types,
            rec_types,
        }
    }

    /// Replaces a head [`ValueType::Rec`] in `t` with the type it refers to, so that `t` can be
    /// destructured. Applied wherever a component type is extracted from a larger one (variable
    /// lookup, projections, pattern destructuring), which is the only way a `Rec` can reach the
    /// head of a type: a registered format type itself always has a structural head.
    fn unfold(&self, t: ValueType) -> ValueType {
        match t {
            ValueType::Rec(level) if level < self.rec_types.len() => self.rec_types[level].clone(),
            other => other,
        }
    }

    /// Unifies `a` and `b`, resolving any [`ValueType::Rec`] they contain.
    fn unify(
        &self,
        a: &ValueType,
        b: &ValueType,
    ) -> Result<ValueType, typecheck::error::UnificationError<ValueType>> {
        a.unify_with(b, self.rec_types)
    }

    fn push(&mut self, name: Label, t: T) {
        self.names.push(name);
        self.types.push(ValueKind::Value(t));
    }

    fn push_format(&mut self, name: Label, t: T) {
        self.names.push(name);
        self.types.push(ValueKind::Format(t));
    }

    fn push_view(&mut self, name: Label) {
        self.names.push(name);
        self.types.push(ValueKind::View);
    }

    fn get_type_by_name(&self, name: &str) -> &ValueKind<T> {
        for (i, n) in self.names.iter().enumerate().rev() {
            if n == name {
                return &self.types[i];
            }
        }
        if let Some(scope) = self.parent {
            scope.get_type_by_name(name)
        } else {
            panic!("variable not found: {name}");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::helper::compute;
    use decoder::Value;

    use super::*;

    fn registration(f: &Format) -> AResult<ValueType> {
        FormatModule::new().infer_format_type(&TypeScope::new(), f)
    }

    /// Registration rejects signed operands to native arithmetic/casts/counts/lengths but accepts Auto (`NumericHole`);
    /// `RepeatBetween` bounds are well-formed; and pattern/type mismatches are reported as errors rather than panics.
    #[test]
    fn registration_numeric_acceptance() {
        use crate::byte_set::ByteSet;
        use crate::helper::{add, poly_zero};
        use crate::numeric::core::TypedConst;
        use crate::numeric::helper as num;

        let i16_one = || Expr::Numeric(Box::new(num::expr_const(TypedConst::from_i16(1))));
        let byte = || Box::new(Format::Byte(ByteSet::full()));

        // R1
        assert!(registration(&compute(add(i16_one(), i16_one()))).is_err());
        assert!(registration(&compute(add(poly_zero(), Expr::U8(1)))).is_ok());
        assert!(registration(&compute(Expr::AsChar(Box::new(i16_one())))).is_err());
        assert!(
            registration(&compute(Expr::AsChar(Box::new(Expr::AsU32(Box::new(
                i16_one()
            ))))))
            .is_ok()
        );
        assert!(registration(&Format::RepeatCount(Box::new(poly_zero()), byte())).is_ok());
        assert!(registration(&Format::RepeatCount(Box::new(i16_one()), byte())).is_err());

        // R4 - non-constant bounds are unrepresentable, and `min > max` is rejected by `RepeatRange::new`
        assert!(RepeatRange::new(2, 1).is_none());
        assert!(
            registration(&Format::RepeatBetween(
                RepeatRange::new(1, 2).unwrap(),
                byte()
            ))
            .is_ok()
        );

        // R5
        assert!(registration(&Format::Slice(Box::new(Expr::U32(1)), byte())).is_ok());
        assert!(registration(&Format::Slice(Box::new(poly_zero()), byte())).is_ok());
        assert!(registration(&Format::Slice(Box::new(Expr::U64(1)), byte())).is_ok());
        assert!(registration(&Format::Slice(Box::new(Expr::Bool(true)), byte())).is_err());

        // R3
        let match_u8 = |head| {
            Format::Match(
                Box::new(head),
                vec![
                    (Pattern::U8(0), (*byte()).clone()),
                    (Pattern::Wildcard, (*byte()).clone()),
                ],
            )
        };
        assert!(registration(&match_u8(poly_zero())).is_ok());
        assert!(registration(&match_u8(Expr::Bool(true))).is_err());
    }

    #[test]
    fn format_let_eval_precedence() {
        let fmt = Format::Let(
            Label::Borrowed("y"),
            Box::new(Expr::U8(10)),
            Box::new(Format::Let(
                Label::Borrowed("x"),
                Box::new(Expr::Var(Label::Borrowed("y"))),
                Box::new(Format::record(vec![
                    (Label::Borrowed("y"), compute(Expr::U8(5))),
                    (
                        Label::Borrowed("z"),
                        compute(Expr::Var(Label::Borrowed("x"))),
                    ),
                ])),
            )),
        );
        let mut module = FormatModule::new();
        let ref_fmt = module.define_format("test", fmt);
        let prog = super::decoder::Compiler::compile_program(&module, &ref_fmt.call()).unwrap();
        let buf = ReadCtxt::new(&[]);
        let (ret, _) = prog.run(buf).unwrap();
        let expected = Value::Record(vec![
            (Label::Borrowed("y"), Value::U8(5)),
            (Label::Borrowed("z"), Value::U8(10)),
        ]);
        assert_eq!(expected, ret);
    }

    #[test]
    fn format_phantom_rec_registers_and_decodes() {
        use crate::helper::*;

        let mut module = FormatModule::new();
        let self_ref = module.define_format_phantom_rec(
            "test.recursive",
            |phantom_field| {
                Format::record(vec![
                    (Label::Borrowed("tag"), u8()),
                    (Label::Borrowed("child"), phantom_field),
                ])
            },
            |rec_ref| rec_ref.call(),
        );

        // Registration must have replaced the `ValueType::Any` placeholder with a real type.
        let level = self_ref.get_level();
        assert!(!matches!(module.get_format_type(level), ValueType::Any));

        let prog = super::decoder::Compiler::compile_program(&module, &self_ref.call()).unwrap();
        let buf = ReadCtxt::new(&[42u8]);
        let (ret, _) = prog.run(buf).unwrap();
        match ret {
            Value::Record(fields) => {
                assert_eq!(fields[0], (Label::Borrowed("tag"), Value::U8(42)));
            }
            other => panic!("expected a record, found {other:?}"),
        }
    }

    fn assert_bounds<B>(e: Expr, expected: B)
    where
        Bounds: From<B>,
    {
        let bounds = e.bounds();
        assert_eq!(Bounds::from(expected), bounds);
    }

    #[test]
    fn test_expr_bounds() {
        use crate::helper::*;
        use crate::numeric::helper as n;

        // monic const expression
        assert_bounds(Expr::U8(42), 42u8);

        // arithmetic operations in doodle grammar
        assert_bounds(add(Expr::U8(1), Expr::U8(2)), 3u8);
        assert_bounds(mul(Expr::U8(1), Expr::U8(2)), 2u8);
        assert_bounds(div(Expr::U8(8), Expr::U8(2)), 4u8);
        assert_bounds(sub(Expr::U8(8), Expr::U8(2)), 6u8);

        // rem works only when both bounds are exact, but const expressions should always have exact-bounds
        assert_bounds(rem(Expr::U8(5), Expr::U8(2)), 1u8);

        // embedded numeric subtrees
        assert_bounds(numeric(n::lift_const(5000u32)), 5000u32);
        assert_bounds(
            numeric(n::binop_auto(
                numeric::BasicBinOp::Add,
                n::lift_const(1u32),
                n::auto_const(2u32),
            )),
            3u32,
        )

        // STUB - add more cases to test
    }

    /// Reserves a slot for a self-referential (non-`Phantom`) format at a known level, without
    /// going through `define_format`'s type-checked registration - which cannot yet express such
    /// a self-reference at all (see `define_format_phantom_rec_args_views`'s own doc comment: the
    /// only self-reference it permits is one confined under `Format::Phantom`). These tests only
    /// exercise `MatchTreeStep`/`MatchTree` construction directly, not the full
    /// typecheck/codegen pipeline, so bypassing that registration path here is sufficient and
    /// keeps the test focused on the cycle guard itself.
    fn reserve_recursive_level(module: &mut FormatModule) -> usize {
        let level = module.names.len();
        module.names.push(Label::Borrowed("test.recursive"));
        module.args.push(Vec::new());
        module.views.push(Vec::new());
        module.formats.push(Format::EMPTY);
        module.format_types.push(ValueType::Any);
        level
    }

    /// Guarded (byte-first) self-recursion - `peano := 'Z' | 'S' ~ peano` - must terminate
    /// `MatchTreeStep::from_format` and disambiguate correctly, without ever tripping the
    /// left-recursion guard: the `ItemVar` self-reference is reached only after the `'S'` byte
    /// has already been consumed, which defers it into a `Next::Sequence` continuation rather
    /// than re-entering `Format::ItemVar` within this same eager descent.
    #[test]
    fn matchtree_guarded_self_reference_terminates_and_disambiguates() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let level = reserve_recursive_level(&mut module);
        let peano = Format::Union(vec![
            Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'Z'))),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    is_byte(b'S'),
                    Format::ItemVar(level, Vec::new(), Vec::new()),
                ])),
            ),
        ]);
        module.formats[level] = peano.clone();

        let mut guard = CycleGuard::new();
        let step = MatchTreeStep::from_format(&module, &peano, Rc::new(Next::Empty), &mut guard);
        assert!(
            !guard.has_cycle,
            "guarded (byte-first) recursion must not be misclassified as left recursion"
        );
        assert!(!step.accept, "peano itself never accepts zero bytes");
        assert_eq!(
            step.branches.len(),
            2,
            "expected exactly one branch each for the 'Z' and 'S' prefixes, got {:?}",
            step.branches
        );
    }

    /// A genuinely left-recursive format - `bad := bad | 'Z'` - recurses into its own `ItemVar`
    /// with zero bytes consumed in between, which `MatchTreeStep::from_format`'s guard must
    /// detect (`guard.has_cycle`) instead of looping/overflowing the stack.
    #[test]
    fn matchtree_zero_progress_self_reference_is_detected_as_left_recursion() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let level = reserve_recursive_level(&mut module);
        let bad = Format::Union(vec![
            Format::ItemVar(level, Vec::new(), Vec::new()),
            is_byte(b'Z'),
        ]);
        module.formats[level] = bad.clone();

        let mut guard = CycleGuard::new();
        let _ = MatchTreeStep::from_format(&module, &bad, Rc::new(Next::Empty), &mut guard);
        assert!(
            guard.has_cycle,
            "an unconditional (zero-progress) self-reference must be flagged as a cycle"
        );
    }

    /// The same genuine left recursion as above, but reached through `MatchTree::build` (the
    /// actual entry point every real `Union`/`RepeatBetween` compile site uses): the whole build
    /// must fail outright (`None`, matching the existing "cannot build match tree" convention
    /// used for undisambiguable unions) rather than silently treating the cyclic branch as though
    /// it simply contributed no further disambiguating information.
    #[test]
    fn matchtree_build_fails_outright_on_genuine_left_recursion() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let level = reserve_recursive_level(&mut module);
        let bad = Format::Union(vec![
            Format::ItemVar(level, Vec::new(), Vec::new()),
            is_byte(b'Z'),
        ]);
        module.formats[level] = bad;

        let branches = vec![
            Format::Tuple(vec![
                Format::ItemVar(level, Vec::new(), Vec::new()),
                is_byte(b'A'),
            ]),
            is_byte(b'B'),
        ];
        let tree = MatchTree::build(&module, &branches, Rc::new(Next::Empty));
        assert!(
            tree.is_none(),
            "MatchTree::build must fail outright (None) on a genuinely left-recursive format, \
             not silently drop the offending branch"
        );
    }

    /// `MatchTree::try_build` reports which level closed a genuine zero-progress cycle.
    #[test]
    fn matchtree_try_build_reports_cycle_level() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let level = reserve_recursive_level(&mut module);
        module.formats[level] = Format::Union(vec![
            Format::ItemVar(level, Vec::new(), Vec::new()),
            is_byte(b'Z'),
        ]);
        let branches = vec![
            Format::ItemVar(level, Vec::new(), Vec::new()),
            is_byte(b'B'),
        ];
        let res = MatchTree::try_build(&module, &branches, Rc::new(Next::Empty));
        assert_eq!(res.err(), Some(MatchTreeError::Cycle { level }));
    }

    /// Two branches that both accept after the same input can never be told apart.
    #[test]
    fn matchtree_try_build_reports_ambiguous_accept() {
        use crate::helper::is_byte;

        let module = FormatModule::new();
        let branches = vec![is_byte(b'a'), is_byte(b'a')];
        let res = MatchTree::try_build(&module, &branches, Rc::new(Next::Empty));
        assert_eq!(
            res.err(),
            Some(MatchTreeError::Ambiguous {
                accepted: 0,
                conflicting: 1
            })
        );
    }

    /// `'a'* ~ 'b'` versus `'a'* ~ 'c'` can only be decided after an unbounded run of `'a'`s,
    /// which exceeds any fixed lookahead depth.
    #[test]
    fn matchtree_try_build_reports_depth_exhausted() {
        use crate::helper::is_byte;

        let module = FormatModule::new();
        let a_star = || Format::Repeat(Box::new(is_byte(b'a')));
        let branches = vec![
            Format::Tuple(vec![a_star(), is_byte(b'b')]),
            Format::Tuple(vec![a_star(), is_byte(b'c')]),
        ];
        let res = MatchTree::try_build(&module, &branches, Rc::new(Next::Empty));
        assert_eq!(res.err(), Some(MatchTreeError::DepthExhausted));
    }

    /// Closing a level on leaving its body must not hide real left recursion through a nullable
    /// prefix: `A := B ~ A | 'z'` with zero-width `B` is `A := A | 'z'` in disguise. Leaving `B`
    /// closes only `B`; `A`'s continuation is different, so `A` is still open when it is reached.
    #[test]
    fn matchtree_left_recursion_through_nullable_prefix_is_a_cycle() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let b = module.define_format("test.zero", Format::Compute(Box::new(Expr::U8(0))));
        let a = reserve_recursive_level(&mut module);
        module.formats[a] = Format::Union(vec![
            Format::Tuple(vec![b.call(), Format::ItemVar(a, Vec::new(), Vec::new())]),
            is_byte(b'z'),
        ]);
        let branches = vec![Format::ItemVar(a, Vec::new(), Vec::new()), is_byte(b'y')];
        let res = MatchTree::try_build(&module, &branches, Rc::new(Next::Empty));
        assert_eq!(res.err(), Some(MatchTreeError::Cycle { level: a }));
    }

    /// A failure must be attributed to the right cause.
    ///
    /// `branch0 := B ~ B ~ 'x'` with `B := ' '*` is not recursive, but it is ambiguous: an input
    /// with `N` spaces before the `'x'` has `N + 1` ways to split them between the two `B`s. So
    /// compilation failing is correct, but it must fail at `B`'s own `Repeat`, whose match tree
    /// cannot decide when to stop. The union `[branch0, 'y']` itself is decidable (`' '`/`'x'`
    /// versus `'y'`), as it is when `B` is inlined: the first `B` can match zero bytes, but the
    /// walk only reaches the second `B` through the first one's exit, where the `CycleGuard` has
    /// already closed it, so it is not mistaken for a zero-progress cycle.
    #[test]
    fn matchtree_ambiguous_repeated_nullable_item_not_reported_as_cycle() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let spaces = module.define_format("test.spaces", Format::Repeat(Box::new(is_byte(b' '))));
        let branch0 = Format::Tuple(vec![spaces.call(), spaces.call(), is_byte(b'x')]);
        let branches = vec![branch0.clone(), is_byte(b'y')];
        let res = MatchTree::try_build(&module, &branches, Rc::new(Next::Empty));
        assert!(
            res.is_ok(),
            "the union itself is decidable and must not be reported as a cycle: {:?}",
            res.err()
        );

        let union = module.define_format(
            "test.union",
            Format::Union(vec![
                Format::Variant(Label::Borrowed("X"), Box::new(branch0)),
                Format::Variant(Label::Borrowed("Y"), Box::new(is_byte(b'y'))),
            ]),
        );
        let err = super::decoder::Compiler::compile_program(&module, &union.call())
            .err()
            .expect("an ambiguous format must not compile")
            .to_string();
        assert!(
            err.starts_with("cannot build match tree for Repeat("),
            "compilation must fail at `B`'s ambiguous `Repeat`, got: {err}"
        );
    }

    /// Builds `Union([F: top, Z: 'z'])` as a named format and checks that it compiles, and that each
    /// `(input, ok)` pair decodes (or fails to) as expected. Building it through
    /// `MatchTree::build` (via `compile_program`) puts the `ItemVar`s inside `top` on the
    /// `CycleGuard`'s path, which a single top-level `Tuple` never would.
    fn check_union_with_z(module: &mut FormatModule, top: Format, cases: &[(&[u8], bool)]) {
        use crate::helper::is_byte;

        let union = module.define_format(
            "test.union_with_z",
            Format::Union(vec![
                Format::Variant(Label::Borrowed("F"), Box::new(top)),
                Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'z'))),
            ]),
        );
        let prog = super::decoder::Compiler::compile_program(module, &union.call())
            .expect("well-formed non-recursive format must compile");
        for (input, ok) in cases {
            assert_eq!(
                prog.run(ReadCtxt::new(input)).is_ok(),
                *ok,
                "decoding {:?}",
                String::from_utf8_lossy(input)
            );
        }
    }

    /// A named zero-width format referenced twice in a row - `F := B ~ B ~ 'x'` with
    /// `B := compute(0)` - is unambiguous, so it must compile named as well as inlined. The walk
    /// reaches the second `B` with zero bytes consumed, but only through the first `B`'s exit,
    /// where the `CycleGuard` closes it, so the second reference is not a re-entry.
    #[test]
    fn matchtree_repeated_zero_width_item_is_not_a_cycle() {
        use crate::helper::is_byte;

        let zero = || Format::Compute(Box::new(Expr::U8(0)));
        let cases: &[(&[u8], bool)] = &[(b"x", true), (b"z", true), (b"y", false)];

        let mut module = FormatModule::new();
        check_union_with_z(
            &mut module,
            Format::Tuple(vec![zero(), zero(), is_byte(b'x')]),
            cases,
        );

        let mut module = FormatModule::new();
        let b = module.define_format("test.zero", zero());
        check_union_with_z(
            &mut module,
            Format::Tuple(vec![b.call(), b.call(), is_byte(b'x')]),
            cases,
        );
    }

    /// A nullable named format that can also match bytes, referenced twice with a separator whose
    /// empty case is guarded by lookahead: `F := B ~ C ~ B ~ ('x' | 'c')`, with `B := 'a'?` and
    /// `C := 'b' | Peek('c')`. `C` can only match empty right before a `'c'`, which neither `B`
    /// can consume, so every input splits only one way, and it must compile named as well as
    /// inlined. As in the zero-width case above, the zero-byte route `B ~ C ~ B` reaches the
    /// second `B` only after leaving the first.
    #[test]
    fn matchtree_lookahead_separated_nullable_item_is_not_a_cycle() {
        use crate::helper::is_byte;

        let opt_a = || {
            Format::Union(vec![
                Format::Variant(Label::Borrowed("Some"), Box::new(is_byte(b'a'))),
                Format::Variant(Label::Borrowed("None"), Box::new(Format::EMPTY)),
            ])
        };
        let sep = || {
            Format::Union(vec![
                Format::Variant(Label::Borrowed("B"), Box::new(is_byte(b'b'))),
                Format::Variant(
                    Label::Borrowed("P"),
                    Box::new(Format::Peek(Box::new(is_byte(b'c')))),
                ),
            ])
        };
        let tail = || Format::Byte(ByteSet::from([b'x', b'c']));
        let cases: &[(&[u8], bool)] = &[
            (b"ac", true),
            (b"abax", true),
            (b"bx", true),
            (b"c", true),
            (b"z", true),
            (b"x", false),
        ];

        let mut module = FormatModule::new();
        check_union_with_z(
            &mut module,
            Format::Tuple(vec![opt_a(), sep(), opt_a(), tail()]),
            cases,
        );

        let mut module = FormatModule::new();
        let b = module.define_format("test.opt_a", opt_a());
        check_union_with_z(
            &mut module,
            Format::Tuple(vec![b.call(), sep(), b.call(), tail()]),
            cases,
        );
    }

    /// In `A | B` with `A := RepeatUntilLast(until 'x', any byte)` and `B := "ab"`, where `A`
    /// stops is decided by the bytes' values, which a `MatchTree` cannot model. `"ax"` matches
    /// only `A`, so it must either decode (as `A`), or the format must be rejected up front as
    /// undecidable; choosing `B` and then failing to decode is wrong either way.
    #[test]
    fn matchtree_accept_stub_does_not_lose_to_byte_reading_branch() {
        use crate::helper::{alts, expr_eq, is_byte, lambda, tuple, var};
        use decoder::Value;

        let until_x = Format::RepeatUntilLast(
            Box::new(lambda("b", expr_eq(var("b"), Expr::U8(b'x')))),
            Box::new(Format::Byte(ByteSet::full())),
        );
        let mut module = FormatModule::new();
        let top = module.define_format(
            "test.accept_stub",
            alts([("A", until_x), ("B", tuple([is_byte(b'a'), is_byte(b'b')]))]),
        );
        let Ok(prog) = super::decoder::Compiler::compile_program(&module, &top.call()) else {
            return; // rejected up front
        };
        let res = prog.run(ReadCtxt::new(b"ax"));
        assert!(
            matches!(res, Ok((Value::Branch(0, _), _))),
            "\"ax\" matches only the `RepeatUntilLast` branch, got {res:?}"
        );
    }

    /// A format that consumes nothing where it is parsed (`Phantom`, `WithRelativeOffset`) must not
    /// hide what follows it from a `MatchTree`: in `A | B` with `A := (z, 'a', 'x')` and
    /// `B := ('a', 'b')`, the second byte decides.
    #[test]
    fn zero_width_format_leading_a_branch_does_not_hide_what_follows() {
        use crate::helper::{alts, is_byte, tuple};
        use decoder::Value;

        let any_byte = || Box::new(Format::Byte(ByteSet::full()));
        for (name, zero_width) in [
            ("Phantom", Format::Phantom(any_byte())),
            (
                "WithRelativeOffset",
                Format::WithRelativeOffset(
                    Box::new(Expr::U32(0)),
                    Box::new(Expr::U32(0)),
                    any_byte(),
                ),
            ),
        ] {
            let mut module = FormatModule::new();
            let top = module.define_format(
                "test.zero_width_lead",
                alts([
                    ("A", tuple([zero_width, is_byte(b'a'), is_byte(b'x')])),
                    ("B", tuple([is_byte(b'a'), is_byte(b'b')])),
                ]),
            );
            let prog = super::decoder::Compiler::compile_program(&module, &top.call())
                .unwrap_or_else(|e| panic!("{name}: {e}"));
            for (input, branch) in [(&b"ax"[..], 0), (b"ab", 1)] {
                let res = prog.run(ReadCtxt::new(input));
                assert!(
                    matches!(res, Ok((Value::Branch(b, _), _)) if b == branch),
                    "{name}: {input:?} should decode as branch {branch}, got {res:?}"
                );
            }
        }
    }

    /// `Next::cat` must not drop a zero-width continuation that still *constrains* lookahead.
    ///
    /// `MonadSeq(('a' | 'a' ~ 'c'), Peek('c'))` on input `"ac"`: only branch 0 (consume `'a'`, then
    /// peek the `'c'`) can succeed, and the union's `MatchTree` can only see that if it is built
    /// against the `Peek('c')` continuation. If the `Peek` is skipped as "nonproductive" (its
    /// `match_bounds` is exactly 0), the tree prefers branch 1 on seeing `'c'`, which consumes the
    /// whole input and then fails the `Peek`.
    #[test]
    fn zero_width_peek_continuation_constrains_union() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let top = module.define_format(
            "test.peek_after_union",
            Format::MonadSeq(
                Box::new(Format::Union(vec![
                    Format::Variant(Label::Borrowed("A"), Box::new(is_byte(b'a'))),
                    Format::Variant(
                        Label::Borrowed("AC"),
                        Box::new(Format::Tuple(vec![is_byte(b'a'), is_byte(b'c')])),
                    ),
                ])),
                Box::new(Format::Peek(Box::new(is_byte(b'c')))),
            ),
        );
        let prog = super::decoder::Compiler::compile_program(&module, &top.call()).unwrap();
        assert!(
            prog.run(ReadCtxt::new(b"acc")).is_ok(),
            "branch 1 followed by a peeked 'c' must decode"
        );
        assert!(
            prog.run(ReadCtxt::new(b"ac")).is_ok(),
            "the union must pick branch 0 so that the trailing Peek('c') can succeed"
        );
    }

    /// End-to-end test of `FormatModule::define_format_rec_batch` for a self-recursive format,
    /// through the *real* public registration API (unlike the hand-poked-field tests above) and
    /// the actual interpreter, at real recursion depth >= 2 - proving `Format::RecVar`'s
    /// construction-time rewrite, `infer_format_type`'s registration-time type inference, and the
    /// Phase 1 `MatchTree` cycle guard all compose correctly for a format nobody could have built
    /// through the public API before this.
    #[test]
    fn define_format_rec_batch_self_recursive_peano_decodes() {
        use crate::helper::is_byte;
        use decoder::Value;

        let mut module = FormatModule::new();
        let peano_body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'Z'))),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![is_byte(b'S'), Format::RecVar(0)])),
            ),
        ]);
        let refs = module.define_format_rec_batch(vec![("test.peano", peano_body)]);
        let peano_ref = refs[0];

        let prog = super::decoder::Compiler::compile_program(&module, &peano_ref.call()).unwrap();
        let buf = ReadCtxt::new(b"SSZ");
        let (ret, _) = prog.run(buf).unwrap();

        // Each layer is wrapped in `Value::Branch(index, ..)` recording which `Union` arm (Z=0,
        // S=1) matched, on top of the `Value::Variant` labeling it - both added by the decoder,
        // not spelled out in `peano_body` itself.
        let expected = Value::Branch(
            1,
            Box::new(Value::Variant(
                Label::Borrowed("S"),
                Box::new(Value::Tuple(vec![
                    Value::U8(b'S'),
                    Value::Branch(
                        1,
                        Box::new(Value::Variant(
                            Label::Borrowed("S"),
                            Box::new(Value::Tuple(vec![
                                Value::U8(b'S'),
                                Value::Branch(
                                    0,
                                    Box::new(Value::Variant(
                                        Label::Borrowed("Z"),
                                        Box::new(Value::U8(b'Z')),
                                    )),
                                ),
                            ])),
                        )),
                    ),
                ])),
            )),
        );
        assert_eq!(
            expected, ret,
            "peano-shaped self-recursion decoded incorrectly"
        );
    }

    /// Phase 5's capstone (see `experiments/doodle-rec/PLAN.md`) explicitly calls for a
    /// malformed-input rejection case through the interpreter, not just the build-time
    /// left-recursion rejection `define_format_rec_batch_left_recursion_still_rejected_at_compile_time`
    /// already covers - this proves a *well-formed* recursive grammar's `MatchTree`/decoder still
    /// correctly rejects bad bytes deep inside a real recursive descent, not just at the top level.
    #[test]
    fn define_format_rec_batch_self_recursive_peano_rejects_malformed_input() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let peano_body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'Z'))),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![is_byte(b'S'), Format::RecVar(0)])),
            ),
        ]);
        let refs = module.define_format_rec_batch(vec![("test.peano", peano_body)]);
        let peano_ref = refs[0];

        let prog = super::decoder::Compiler::compile_program(&module, &peano_ref.call()).unwrap();
        // Two valid 'S' layers deep, then a byte that is neither 'S' nor 'Z' - must be rejected
        // from *within* the recursive descent, not just at the top-level call.
        let buf = ReadCtxt::new(b"SSX");
        assert!(
            prog.run(buf).is_err(),
            "a byte that is neither 'Z' nor 'S', reached mid-recursion, must be rejected"
        );
    }

    /// Same as above, but for genuine *mutual* recursion (two distinct batch members referencing
    /// each other, not just themselves) - the shape doodle-rec's own port-planning found exercises
    /// different code paths than self-recursion alone (e.g. Box-placement ordering across distinct
    /// types, relevant to a later phase). Decodes real input alternating between both members at
    /// depth >= 2.
    #[test]
    fn define_format_rec_batch_mutual_recursion_ping_pong_decodes() {
        use crate::helper::is_byte;
        use decoder::Value;

        let mut module = FormatModule::new();
        // ping (batch index 0) refers to pong (index 1); pong refers back to ping (index 0).
        let ping_body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Stop"), Box::new(is_byte(b'X'))),
            Format::Variant(
                Label::Borrowed("ToPong"),
                Box::new(Format::Tuple(vec![is_byte(b'p'), Format::RecVar(1)])),
            ),
        ]);
        let pong_body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Stop"), Box::new(is_byte(b'Y'))),
            Format::Variant(
                Label::Borrowed("ToPing"),
                Box::new(Format::Tuple(vec![is_byte(b'q'), Format::RecVar(0)])),
            ),
        ]);
        let refs = module
            .define_format_rec_batch(vec![("test.ping", ping_body), ("test.pong", pong_body)]);
        let ping_ref = refs[0];

        let prog = super::decoder::Compiler::compile_program(&module, &ping_ref.call()).unwrap();
        let buf = ReadCtxt::new(b"pqX");
        let (ret, _) = prog.run(buf).unwrap();

        // See the peano test above for why each layer is wrapped in `Value::Branch(index, ..)`
        // (Stop=0, ToPong/ToPing=1 in both ping's and pong's own Union) on top of `Value::Variant`.
        let expected = Value::Branch(
            1,
            Box::new(Value::Variant(
                Label::Borrowed("ToPong"),
                Box::new(Value::Tuple(vec![
                    Value::U8(b'p'),
                    Value::Branch(
                        1,
                        Box::new(Value::Variant(
                            Label::Borrowed("ToPing"),
                            Box::new(Value::Tuple(vec![
                                Value::U8(b'q'),
                                Value::Branch(
                                    0,
                                    Box::new(Value::Variant(
                                        Label::Borrowed("Stop"),
                                        Box::new(Value::U8(b'X')),
                                    )),
                                ),
                            ])),
                        )),
                    ),
                ])),
            )),
        );
        assert_eq!(
            expected, ret,
            "ping/pong mutual recursion decoded incorrectly"
        );
    }

    /// Same mutual recursion as above, but with each variant's payload built via
    /// [`crate::helper::record`] (the "1-tuple variant convention real doodle productions end up
    /// with") rather than a raw `Tuple`. This is the exact shape that surfaced Phase 4's "Finding
    /// B" (see `experiments/doodle-rec/PLAN.md`): `Format::record`'s desugaring to nested
    /// `LetFormat`s ending in a zero-width `Compute(Record(...))` step wrapped `next` in a
    /// `Next::Cat` for the recursive last field, even though that wrapper adds no real lookahead
    /// information - giving the self-referential field a structurally different `next` than its
    /// own top-level entry used, missing `decoder_map`'s `(level, next)` cache and (in
    /// `codegen::typed_decoder`) compiling into a dead always-failing decoder. Here we exercise
    /// `decoder::Compiler`'s identically-shaped bug in the plain interpreter path: before the
    /// `LetFormat`/`MonadSeq` fix, `pong`'s `next` field failed to build a `MatchTree` for its
    /// duplicate compile of `ping`, surfacing as an `anyhow!` error from `compile_program` rather
    /// than a decode.
    #[test]
    fn define_format_rec_batch_mutual_recursion_ping_pong_record_variant_decodes() {
        use crate::helper::{is_byte, record};
        use decoder::Value;

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
            ("test.ping_rec", ping_body),
            ("test.pong_rec", pong_body),
        ]);
        let ping_ref = refs[0];

        let prog = super::decoder::Compiler::compile_program(&module, &ping_ref.call()).unwrap();
        let buf = ReadCtxt::new(b"pqX");
        let (ret, _) = prog.run(buf).unwrap();

        // Same `Value::Branch`/`Value::Variant` wrapping as the raw-`Tuple` version above; the
        // variant payload itself is now a `Value::Record` (tag, next) rather than a `Value::Tuple`.
        let expected = Value::Branch(
            1,
            Box::new(Value::Variant(
                Label::Borrowed("ToPong"),
                Box::new(Value::Record(vec![
                    (Label::Borrowed("tag"), Value::U8(b'p')),
                    (
                        Label::Borrowed("next"),
                        Value::Branch(
                            1,
                            Box::new(Value::Variant(
                                Label::Borrowed("ToPing"),
                                Box::new(Value::Record(vec![
                                    (Label::Borrowed("tag"), Value::U8(b'q')),
                                    (
                                        Label::Borrowed("next"),
                                        Value::Branch(
                                            0,
                                            Box::new(Value::Variant(
                                                Label::Borrowed("Stop"),
                                                Box::new(Value::U8(b'X')),
                                            )),
                                        ),
                                    ),
                                ])),
                            )),
                        ),
                    ),
                ])),
            )),
        );
        assert_eq!(
            expected, ret,
            "ping/pong mutual recursion through Format::record decoded incorrectly"
        );
    }

    /// A batch member registered via the real public API can still be genuinely left-recursive
    /// (`define_format_rec_batch` itself never checks for progress - `infer_format_type` has no
    /// occurs-check at all, see its own doc comment on `Format::RecVar`) - proving Phase 1's guard
    /// still catches it at `MatchTree`-build time, reached this time via the ordinary
    /// `decoder::Compiler::compile_program` entry point rather than calling `MatchTree::build`
    /// directly, and that registration itself does not hang or panic.
    #[test]
    fn define_format_rec_batch_left_recursion_still_rejected_at_compile_time() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        // bad := bad | 'Z' -- zero-progress self-reference in the first branch.
        let bad_body = Format::Union(vec![Format::RecVar(0), is_byte(b'Z')]);
        let refs = module.define_format_rec_batch(vec![("test.bad", bad_body)]);
        let bad_ref = refs[0];

        let result = super::decoder::Compiler::compile_program(&module, &bad_ref.call());
        assert!(
            result.is_err(),
            "compiling a genuinely left-recursive format must fail cleanly, not panic or hang"
        );
    }

    /// A compile error for an undecidable choice says why the `MatchTree` could not be built.
    #[test]
    fn match_tree_compile_errors_report_the_reason() {
        use crate::helper::{alts, is_byte, tuple};

        let error_for = |module: &FormatModule, f: &Format| {
            super::decoder::Compiler::compile_program(module, f)
                .err()
                .map(|e| e.to_string())
                .unwrap_or_default()
        };
        let a_star = || Format::Repeat(Box::new(is_byte(b'a')));

        let module = FormatModule::new();
        let ambiguous = alts([("A", is_byte(b'a')), ("B", is_byte(b'a'))]);
        assert!(error_for(&module, &ambiguous).contains("both accept the same input"));
        let too_deep = alts([
            ("B", tuple([a_star(), is_byte(b'b')])),
            ("C", tuple([a_star(), is_byte(b'c')])),
        ]);
        assert!(error_for(&module, &too_deep).contains("bytes of lookahead"));

        let mut module = FormatModule::new();
        let bad_body = Format::Union(vec![Format::RecVar(0), is_byte(b'Z')]);
        let bad = module.define_format_rec_batch(vec![("test.bad", bad_body)])[0];
        assert!(
            error_for(&module, &bad.call())
                .contains("zero-progress recursive cycle through `test.bad`")
        );
    }

    /// A `RecVar` index past the end of its batch must be rejected at registration, rather than
    /// becoming an `ItemVar` that indexes out of bounds or silently aliases whatever format is
    /// registered at that level later.
    #[test]
    #[should_panic(
        expected = "Format::RecVar(1) is out of range for a recursive batch of 1 member(s)"
    )]
    fn define_format_rec_batch_rejects_out_of_range_rec_var() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'Z'))),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![is_byte(b'S'), Format::RecVar(1)])),
            ),
        ]);
        let _ = module.define_format_rec_batch(vec![("test.typo", body)]);
    }

    /// A `RecVar` passed to `define_format` (outside any batch) is a usage error, and the panic
    /// must say so.
    #[test]
    #[should_panic(expected = "is only valid in the body of a `define_format_rec_batch` member")]
    fn define_format_rejects_rec_var() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let _ = module.define_format(
            "test.stray",
            Format::Union(vec![is_byte(b'Z'), Format::RecVar(0)]),
        );
    }

    /// Same as above, for a `RecVar` returned from a `define_format_recursive` builder, which must
    /// use its `FormatRef`s instead.
    #[test]
    #[should_panic(expected = "is only valid in the body of a `define_format_rec_batch` member")]
    fn define_format_recursive_rejects_rec_var() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let _ = module.define_format_recursive(
            ["test.stray"],
            [Box::new(|_refs: &[FormatRef; 1]| {
                Format::Union(vec![
                    Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'Z'))),
                    Format::Variant(
                        Label::Borrowed("S"),
                        Box::new(Format::Tuple(vec![is_byte(b'S'), Format::RecVar(0)])),
                    ),
                ])
            })],
        );
    }

    /// Batch registration's own inference types a member's references to itself (and to later
    /// siblings) as the `ValueType::Any` placeholder, which `ValueType::unify` accepts, so it lets
    /// a comparison of peano's recursive field against a `u8` through; the interpreter panics on
    /// that comparison while decoding (`cannot apply int-rel Eq`). Either registration must reject
    /// the format, or decoding must fail without panicking.
    #[test]
    fn ill_typed_use_of_recursive_field_is_rejected_or_decodes_without_panic() {
        use crate::helper::{expr_eq, is_byte, var, where_lambda};
        use std::panic::{AssertUnwindSafe, catch_unwind};

        let body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'Z'))),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![
                    is_byte(b'S'),
                    where_lambda(Format::RecVar(0), "x", expr_eq(var("x"), Expr::U8(0))),
                ])),
            ),
        ]);
        let mut module = FormatModule::new();
        let Ok(refs) = catch_unwind(AssertUnwindSafe(|| {
            module.define_format_rec_batch(vec![("test.peano_ill_typed", body)])
        })) else {
            return; // rejected at registration
        };
        let Ok(prog) = super::decoder::Compiler::compile_program(&module, &refs[0].call()) else {
            return; // rejected at compile time
        };
        let decoded = catch_unwind(AssertUnwindSafe(|| {
            let _ = prog.run(ReadCtxt::new(b"SZ"));
        }));
        assert!(
            decoded.is_ok(),
            "an ill-typed use of a recursive field passed registration and panicked while decoding"
        );
    }

    /// `tree := 'L' | 'N' ~ tree* ~ 'E'`, registered as a batch.
    fn register_tree(module: &mut FormatModule) -> FormatRef {
        use crate::helper::{is_byte, repeat};

        let tree = Format::Union(vec![
            Format::Variant(Label::Borrowed("Leaf"), Box::new(is_byte(b'L'))),
            Format::Variant(
                Label::Borrowed("Node"),
                Box::new(Format::Tuple(vec![
                    is_byte(b'N'),
                    repeat(Format::RecVar(0)),
                    is_byte(b'E'),
                ])),
            ),
        ]);
        module.define_format_rec_batch(vec![("test.tree", tree)])[0]
    }

    /// `t := 'a' | 'a' ~ 'b' | 'N' ~ t* ~ 'E'`, registered as a batch: whether an `'a'` is
    /// followed by a `'b'` that belongs to it can only be told from the byte after it, which for a
    /// nested `t` lies in that call's continuation.
    fn register_continuation_dependent_tree(module: &mut FormatModule) -> FormatRef {
        use crate::helper::{is_byte, repeat};

        let t = Format::Union(vec![
            Format::Variant(Label::Borrowed("A"), Box::new(is_byte(b'a'))),
            Format::Variant(
                Label::Borrowed("AB"),
                Box::new(Format::Tuple(vec![is_byte(b'a'), is_byte(b'b')])),
            ),
            Format::Variant(
                Label::Borrowed("Node"),
                Box::new(Format::Tuple(vec![
                    is_byte(b'N'),
                    repeat(Format::RecVar(0)),
                    is_byte(b'E'),
                ])),
            ),
        ]);
        module.define_format_rec_batch(vec![("test.cont_tree", t)])[0]
    }

    /// `tree` settles every choice before reaching its own continuation (`'L'`/`'N'` against `'E'`),
    /// even though it contains a `Repeat`; `'a' | 'a' ~ 'b'` cannot settle whether to read the
    /// `'b'` without looking past itself.
    #[test]
    fn next_dependence_is_decided_by_reachable_lookahead() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let tree = register_tree(&mut module);
        let a_or_ab = module.define_format(
            "test.a_or_ab",
            Format::Union(vec![
                Format::Variant(Label::Borrowed("A"), Box::new(is_byte(b'a'))),
                Format::Variant(
                    Label::Borrowed("AB"),
                    Box::new(Format::Tuple(vec![is_byte(b'a'), is_byte(b'b')])),
                ),
            ]),
        );
        let cont_tree = register_continuation_dependent_tree(&mut module);
        let mut dependence = NextDependence::default();
        assert!(!dependence.level_depends_on_next(&module, tree.get_level()));
        assert!(dependence.level_depends_on_next(&module, a_or_ab.get_level()));
        assert!(dependence.level_depends_on_next(&module, cont_tree.get_level()));
    }

    /// Compiling a recursive level once per distinct continuation of its calls never ends when
    /// each nested call extends that continuation (as `tree*` inside `tree` does), unless the
    /// level is found not to depend on its continuation.
    #[test]
    fn interpreter_compiles_and_decodes_repeat_of_recursive_item() {
        let mut module = FormatModule::new();
        let tree = register_tree(&mut module);
        let prog = super::decoder::Compiler::compile_program(&module, &tree.call()).unwrap();
        assert!(prog.run(ReadCtxt::new(b"NLNLEE")).is_ok());
        assert!(prog.run(ReadCtxt::new(b"NLNLE")).is_err());
    }

    /// A recursive level that does depend on its continuation is compiled once per distinct
    /// continuation, truncated to `MATCH_TREE_MAX_DEPTH` bytes, which bounds how many there are.
    #[test]
    fn continuation_dependent_recursion_compiles_to_bounded_decoders() {
        let mut module = FormatModule::new();
        let t = register_continuation_dependent_tree(&mut module);
        let prog = super::decoder::Compiler::compile_program(&module, &t.call()).unwrap();
        assert!(prog.decoders.len() <= 2 * MATCH_TREE_MAX_DEPTH);
        for (input, inner) in [(&b"NaNabEE"[..], "\"AB\""), (b"NabNaEE", "\"A\"")] {
            let (value, _) = prog.run(ReadCtxt::new(input)).unwrap();
            assert!(format!("{value:?}").contains(inner), "{value:?}");
        }
    }

    /// `peano := 'Z' | 'S' ~ peano`, registered as a batch, for the tests below.
    fn register_peano(module: &mut FormatModule) -> FormatRef {
        use crate::helper::is_byte;

        let body = Format::Union(vec![
            Format::Variant(Label::Borrowed("Z"), Box::new(is_byte(b'Z'))),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![is_byte(b'S'), Format::RecVar(0)])),
            ),
        ]);
        module.define_format_rec_batch(vec![("test.peano", body)])[0]
    }

    /// A format registered after a batch, rather than as part of it, can reach a member's
    /// recursive field (`S(_, inner)`) and compare it against a `u8`; the interpreter panics on
    /// that comparison while decoding (`cannot apply int-rel Eq`). Either registration must reject
    /// the format, or decoding must fail without panicking.
    #[test]
    fn ill_typed_use_of_recursive_field_from_later_registration_is_rejected_or_decodes_without_panic()
     {
        use crate::helper::{expr_eq, expr_match, var, where_lambda};
        use std::panic::{AssertUnwindSafe, catch_unwind};

        let mut module = FormatModule::new();
        let peano = register_peano(&mut module);
        let user = where_lambda(
            peano.call(),
            "x",
            expr_match(
                var("x"),
                [
                    (
                        Pattern::variant(
                            "S",
                            Pattern::Tuple(vec![Pattern::Wildcard, Pattern::binding("inner")]),
                        ),
                        expr_eq(var("inner"), Expr::U8(0)),
                    ),
                    (Pattern::Wildcard, Expr::Bool(true)),
                ],
            ),
        );
        let Ok(user_ref) = catch_unwind(AssertUnwindSafe(|| {
            module.define_format("test.peano_user", user)
        })) else {
            return; // rejected at registration
        };
        let Ok(prog) = super::decoder::Compiler::compile_program(&module, &user_ref.call()) else {
            return; // rejected at compile time
        };
        let decoded = catch_unwind(AssertUnwindSafe(|| {
            let _ = prog.run(ReadCtxt::new(b"SZ"));
        }));
        assert!(
            decoded.is_ok(),
            "an ill-typed use of a recursive field passed registration and panicked while decoding"
        );
    }

    /// A batch member's registered type refers back to itself through `ValueType::Rec`, rather
    /// than through a placeholder that every later use would unify with.
    #[test]
    fn rec_batch_member_type_refers_to_itself_through_rec() {
        let mut module = FormatModule::new();
        let peano = register_peano(&mut module);
        let level = peano.get_level();
        assert_eq!(
            module.get_format_type(level),
            &ValueType::Union(BTreeMap::from([
                (Label::Borrowed("Z"), ValueType::U8),
                (
                    Label::Borrowed("S"),
                    ValueType::Tuple(vec![ValueType::U8, ValueType::Rec(level)]),
                ),
            ]))
        );
    }

    /// A batch member's registered type follows registration's own conventions rather than the
    /// `TypeChecker`'s: an auto-width numeric stays `NumericHole` although the `TypeChecker` has a
    /// default width for it, and a union branch that can never be decoded is kept.
    #[test]
    fn rec_batch_member_type_keeps_registration_conventions() {
        use crate::helper::{compute, is_byte, poly_zero};

        let body = Format::Union(vec![
            Format::Variant(Label::Borrowed("N"), Box::new(compute(poly_zero()))),
            Format::Variant(Label::Borrowed("Dead"), Box::new(Format::Fail)),
            Format::Variant(
                Label::Borrowed("S"),
                Box::new(Format::Tuple(vec![is_byte(b'S'), Format::RecVar(0)])),
            ),
        ]);
        let mut module = FormatModule::new();
        let refs = module.define_format_rec_batch(vec![("test.conventions", body)]);
        let level = refs[0].get_level();
        assert_eq!(
            module.get_format_type(level),
            &ValueType::Union(BTreeMap::from([
                (Label::Borrowed("N"), ValueType::NumericHole),
                (Label::Borrowed("Dead"), ValueType::Empty),
                (
                    Label::Borrowed("S"),
                    ValueType::Tuple(vec![ValueType::U8, ValueType::Rec(level)]),
                ),
            ]))
        );
    }

    /// The specific gap `check_batch_has_base_case` closes: `F := 'a' F`, with no non-recursive
    /// alternative anywhere on its cycle, makes byte progress on every step (unlike
    /// `define_format_rec_batch_left_recursion_still_rejected_at_compile_time`'s `bad` above), so
    /// neither the `MatchTree` cycle guard nor `infer_format_type`'s occurs-check has any reason to
    /// reject it - it would otherwise reach `MatchTree`/codegen and recurse at decode time with no
    /// depth bound, bounded only by how much input an attacker supplies. Must now be rejected at
    /// registration time instead, before a `FormatRef` to it ever escapes into `compile_program` or
    /// codegen.
    #[test]
    #[should_panic(expected = "no reachable base case for: test.no_base_case")]
    fn define_format_rec_batch_no_base_case_rejected_at_registration() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        // F := 'a' F -- unconditionally, mandatorily self-referential; no branch ever escapes.
        let no_base_case = Format::Tuple(vec![is_byte(b'a'), Format::RecVar(0)]);
        let _ = module.define_format_rec_batch(vec![("test.no_base_case", no_base_case)]);
    }

    /// Mutual-recursion counterpart of the above: two batch members that unconditionally call each
    /// other (`a := 'a' b`, `b := 'b' a`) with no non-recursive alternative anywhere on the cycle -
    /// confirms the check follows `ItemVar` references across batch members, not just within a
    /// single member's own body.
    #[test]
    #[should_panic(expected = "no reachable base case for: test.mutual_a, test.mutual_b")]
    fn define_format_rec_batch_mutual_no_base_case_rejected_at_registration() {
        use crate::helper::is_byte;

        let mut module = FormatModule::new();
        let a_body = Format::Tuple(vec![is_byte(b'a'), Format::RecVar(1)]);
        let b_body = Format::Tuple(vec![is_byte(b'b'), Format::RecVar(0)]);
        let _ = module
            .define_format_rec_batch(vec![("test.mutual_a", a_body), ("test.mutual_b", b_body)]);
    }

    /// Guards a subtle correctness boundary inside `Format::has_reachable_base_case` itself, not
    /// just the two `no_base_case` cases above: `Format::Repeat` (zero-or-more) always has an
    /// implicit, unconditional base case (choosing zero repetitions), *regardless* of what its
    /// inner format is - `match_bounds_recursive`'s `Repeat` arm reports `RecursiveBounds::any()`
    /// without even inspecting `inner` at all. Deliberately calls `has_reachable_base_case` directly
    /// on a hand-built `Format` against an otherwise-empty `FormatModule` (level `0` is never
    /// actually installed) rather than going through `define_format_rec_batch` end-to-end: this
    /// isolates the exact algorithmic claim (`Repeat` never needs to resolve what's inside it) from
    /// `MatchTree`'s own, separate and unrelated inability to disambiguate repetition of a directly
    /// self-referential item at decode time - a real but orthogonal limitation, not a regression in
    /// this check. If a future change to the `Repeat` arm ever started threading `inner`'s own
    /// resolution through instead of unconditionally escaping, this would start failing here (and
    /// real recursive-list shapes like a BSON array/document, whose elements can themselves recurse
    /// through a `Repeat`, would start spuriously failing to register).
    #[test]
    fn has_reachable_base_case_repeat_escape_ignores_inner_entirely() {
        use crate::helper::is_byte;

        let module = FormatModule::new();
        let body = Format::Tuple(vec![
            is_byte(b'a'),
            Format::Repeat(Box::new(Format::ItemVar(0, vec![], vec![]))),
        ]);
        assert!(
            body.has_reachable_base_case(&module),
            "a self-reference reached only inside `Repeat` must have a base case, regardless of \
             whether the referenced level itself does"
        );
    }

    /// The mirror-image check: `Format::Repeat1` (one-or-more) does *not* get `Repeat`'s free pass -
    /// it unconditionally requires at least one resolution of its inner format, so wrapping the
    /// *only* self-reference in `Repeat1` alone (with nothing else on the cycle) is exactly as
    /// mandatorily-infinite as `F := 'a' F` and must still be rejected. This is the case a naive
    /// "any `Repeat*` variant has a free escape" simplification would wrongly let through.
    #[test]
    #[should_panic(expected = "no reachable base case for: test.repeat1_self")]
    fn define_format_rec_batch_self_recursive_via_repeat1_still_rejected() {
        let mut module = FormatModule::new();
        // F := Repeat1(F) -- one-or-more copies of itself, with no non-recursive alternative.
        let body = Format::Repeat1(Box::new(Format::RecVar(0)));
        let _ = module.define_format_rec_batch(vec![("test.repeat1_self", body)]);
    }
}
