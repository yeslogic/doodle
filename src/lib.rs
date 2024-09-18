#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::ops::Add;
use std::rc::Rc;

use anyhow::{anyhow, Result as AResult};
use codegen::typed_format::{GenType, TypedFormat};
use serde::Serialize;

use crate::bounds::Bounds;
use crate::byte_set::ByteSet;
use crate::read::ReadCtxt;

pub mod bounds;
pub mod byte_set;
pub mod codegen;
pub mod decoder;
pub mod disjoint;
pub mod error;
pub mod helper;
pub mod loc_decoder;

pub mod output;
pub mod parser;
mod precedence;
pub mod prelude;
pub mod read;

mod typecheck;
pub use typecheck::{typecheck, TCError, TCResult};

pub type Label = std::borrow::Cow<'static, str>;

pub trait IntoLabel: Into<Label> {}

impl<T> IntoLabel for T where T: Into<Label> {}

pub(crate) mod pattern;
pub use pattern::Pattern;

pub enum ValueKind {
    Value(ValueType),
    Format(ValueType),
}

pub(crate) mod valuetype;
pub use valuetype::{BaseType, TypeHint, ValueType};

fn mk_value_expr(vt: &ValueType) -> Option<Expr> {
    match vt {
        ValueType::Any | ValueType::Empty => None,
        ValueType::Base(b) => Some(match b {
            BaseType::Bool => Expr::Bool(false),
            BaseType::U8 => Expr::U8(0),
            BaseType::U16 => Expr::U16(0),
            BaseType::U32 => Expr::U32(0),
            BaseType::U64 => Expr::U64(0),
            BaseType::Char => Expr::AsChar(Box::new(Expr::U32(0))),
        }),
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
        ValueType::Option(t) => Some(Expr::Variant(
            Label::from("Some"),
            Box::new(mk_value_expr(t)?),
        )),
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
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
    Lambda(Label, Box<Expr>),

    IntRel(IntRel, Box<Expr>, Box<Expr>),
    Arith(Arith, Box<Expr>, Box<Expr>),

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
    /// SubSeq :: [T] -> start:U32 -> length:U32 -> [T] (start >= 0, length >= 0, start + length <= length of sequence)
    SubSeq(Box<Expr>, Box<Expr>, Box<Expr>),
    /// SeqIx :: [T] -> ix:U32 -> T [panic on unguarded OOB index]
    SeqIx(Box<Expr>, Box<Expr>),
    /// SubSeqInflate :: [T] -> start:U32 -> length:U32 -> [T] (start >= 0, length >= 0)
    SubSeqInflate(Box<Expr>, Box<Expr>, Box<Expr>),
    /// FlatMap :: (T -> [U]) -> [T] -> [U]
    FlatMap(Box<Expr>, Box<Expr>),
    /// FlatMapAccum :: ((V, T) -> (V, [U])) -> V -> TypeRep V -> [T] -> [U]
    FlatMapAccum(Box<Expr>, Box<Expr>, TypeHint, Box<Expr>),
    /// FlatMapList :: (([U], T) -> [U]) -> TypeRep U -> [T] -> [U]
    FlatMapList(Box<Expr>, TypeHint, Box<Expr>),

    /// LeftFold :: ((U, T) -> U) -> U -> [T} -> U
    LeftFold(Box<Expr>, Box<Expr>, TypeHint, Box<Expr>),

    /// Dup :: U32 -> T -> [T]
    Dup(Box<Expr>, Box<Expr>),

    LiftOption(Option<Box<Expr>>),
}

impl Expr {
    pub const UNIT: Self = Expr::Tuple(Vec::new());

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
    fn infer_type(&self, scope: &TypeScope<'_>) -> AResult<ValueType> {
        match self {
            Expr::Var(name) => match scope.get_type_by_name(name) {
                ValueKind::Value(t) => Ok(t.clone()),
                ValueKind::Format(_t) => Err(anyhow!(
                    "expected ValueKind::Value, found ValueKind::Format for var {name}"
                )),
            },
            Expr::Bool(_b) => Ok(ValueType::Base(BaseType::Bool)),
            Expr::U8(_n) => Ok(ValueType::Base(BaseType::U8)),
            Expr::U16(_n) => Ok(ValueType::Base(BaseType::U16)),
            Expr::U32(_n) => Ok(ValueType::Base(BaseType::U32)),
            Expr::U64(_n) => Ok(ValueType::Base(BaseType::U64)),
            Expr::Tuple(exprs) => {
                let mut ts = Vec::new();
                for expr in exprs {
                    ts.push(expr.infer_type(scope)?);
                }
                Ok(ValueType::Tuple(ts))
            }
            Expr::TupleProj(head, index) => match head.infer_type(scope)? {
                ValueType::Tuple(vs) => Ok(vs[*index].clone()),
                other => Err(anyhow!("tuple projection on non-tuple type {other:?}")),
            },
            Expr::Record(fields) => {
                let mut fs = Vec::new();
                for (label, expr) in fields {
                    fs.push((label.clone(), expr.infer_type(scope)?));
                }
                Ok(ValueType::Record(fs))
            }
            Expr::RecordProj(head, label) => Ok(head.infer_type(scope)?.record_proj(label)),
            Expr::Variant(label, expr) => Ok(ValueType::Union(BTreeMap::from([(
                label.clone(),
                expr.infer_type(scope)?,
            )]))),
            Expr::Seq(exprs) => {
                let mut t = ValueType::Any;
                for e in exprs {
                    t = t.unify(&e.infer_type(scope)?)?;
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
                    t = t.unify(&pattern.infer_expr_branch_type(
                        scope,
                        head_type.clone(),
                        branch,
                    )?)?;
                }
                Ok(t)
            }
            Expr::Lambda(..) => Err(anyhow!("infer_type encountered unexpected lambda")),

            Expr::IntRel(_rel, x, y) => match (x.infer_type(scope)?, y.infer_type(scope)?) {
                (ValueType::Base(b1), ValueType::Base(b2)) if b1 == b2 && b1.is_numeric() => {
                    Ok(ValueType::Base(BaseType::Bool))
                }
                (x, y) => Err(anyhow!(
                    "mismatched operand types for {_rel:?}: {x:?}, {y:?}"
                )),
            },
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

            Expr::Arith(_arith, x, y) => match (x.infer_type(scope)?, y.infer_type(scope)?) {
                (ValueType::Base(b1), ValueType::Base(b2)) if b1 == b2 && b1.is_numeric() => {
                    Ok(ValueType::Base(b1))
                }
                (x, y) => Err(anyhow!(
                    "mismatched operand types for {_arith:?}: {x:?}, {y:?}"
                )),
            },

            Expr::AsU8(x) => match x.infer_type(scope)? {
                ValueType::Base(b) if b.is_numeric() => Ok(ValueType::Base(BaseType::U8)),
                x => Err(anyhow!("unsound type cast AsU8(_ : {x:?})")),
            },
            Expr::AsU16(x) => match x.infer_type(scope)? {
                ValueType::Base(b) if b.is_numeric() => Ok(ValueType::Base(BaseType::U16)),
                x => Err(anyhow!("unsound type cast AsU16(_ : {x:?})")),
            },
            Expr::AsU32(x) => match x.infer_type(scope)? {
                ValueType::Base(b) if b.is_numeric() => Ok(ValueType::Base(BaseType::U32)),
                x => Err(anyhow!("unsound type cast AsU32(_ : {x:?})")),
            },
            Expr::AsU64(x) => match x.infer_type(scope)? {
                ValueType::Base(b) if b.is_numeric() => Ok(ValueType::Base(BaseType::U64)),
                x => Err(anyhow!("cannot convert {x:?} to U64")),
            },
            Expr::AsChar(x) => match x.infer_type(scope)? {
                ValueType::Base(b) if b.is_numeric() => Ok(ValueType::Base(BaseType::Char)),
                x => Err(anyhow!("unsound type cast AsChar(_ : {x:?})")),
            },
            Expr::U16Be(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8)] => {
                        Ok(ValueType::Base(BaseType::U16))
                    }
                    _ => Err(anyhow!("unsound byte-level type cast U16Be(_ : {_t:?})")),
                }
            }
            Expr::U16Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8)] => {
                        Ok(ValueType::Base(BaseType::U16))
                    }
                    _ => Err(anyhow!("unsound byte-level type cast U16Le(_ : {_t:?})")),
                }
            }
            Expr::U32Be(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8)] => {
                        Ok(ValueType::Base(BaseType::U32))
                    }
                    _ => Err(anyhow!("unsound byte-level type cast U32Be(_ : {_t:?})")),
                }
            }
            Expr::U32Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8)] => {
                        Ok(ValueType::Base(BaseType::U32))
                    }
                    _ => Err(anyhow!("unsound byte-level type cast U32Le(_ : {_t:?})")),
                }
            }
            Expr::U64Be(bytes) | Expr::U64Le(bytes) => {
                let _t = bytes.infer_type(scope)?;
                match _t.as_tuple_type() {
                    [ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8), ValueType::Base(BaseType::U8)] => {
                        Ok(ValueType::Base(BaseType::U64))
                    }
                    other => Err(anyhow!(
                        "U64Be/Le: expected (U8, U8, U8, U8, U8, U8, U8, U8), found {other:#?}"
                    )),
                }
            }
            Expr::SeqLength(seq) => match seq.infer_type(scope)? {
                ValueType::Seq(_t) => Ok(ValueType::Base(BaseType::U32)),
                other => Err(anyhow!("seq-length called on non-sequence type: {other:?}")),
            },
            Expr::SeqIx(seq, index) => match seq.infer_type(scope)? {
                ValueType::Seq(t) => {
                    let index_type = index.infer_type(scope)?;
                    if index_type != ValueType::Base(BaseType::U32) {
                        return Err(anyhow!(
                            "SeqIx `index` param: expected U32, found {index_type:?}"
                        ));
                    }
                    Ok(ValueType::clone(&t))
                }
                other => Err(anyhow!("SeqIx: expected Seq, found {other:?}")),
            },
            Expr::SubSeq(seq, start, length) => match seq.infer_type(scope)? {
                ValueType::Seq(t) => {
                    let start_type = start.infer_type(scope)?;
                    let length_type = length.infer_type(scope)?;
                    if start_type != ValueType::Base(BaseType::U32) {
                        return Err(anyhow!(
                            "SubSeq `start` param: expected U32, found {start_type:?}"
                        ));
                    }
                    if length_type != ValueType::Base(BaseType::U32) {
                        return Err(anyhow!(
                            "SubSeq length must be numeric, found {length_type:?}"
                        ));
                    }
                    Ok(ValueType::Seq(t))
                }
                other => Err(anyhow!("SubSeq: expected Seq, found {other:?}")),
            },
            Expr::SubSeqInflate(seq, start, length) => match seq.infer_type(scope)? {
                ValueType::Seq(t) => {
                    let start_type = start.infer_type(scope)?;
                    let length_type = length.infer_type(scope)?;
                    if start_type != ValueType::Base(BaseType::U32) {
                        return Err(anyhow!(
                            "SubSeqInflate `start` param: expected U32, found {start_type:?}"
                        ));
                    }
                    if length_type != ValueType::Base(BaseType::U32) {
                        return Err(anyhow!(
                            "SubSeqInflate length must be numeric, found {length_type:?}"
                        ));
                    }
                    Ok(ValueType::Seq(t))
                }
                other => Err(anyhow!("SubSeqInflate: expected Seq, found {other:?}")),
            },
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
                        let accum_type = accum.infer_type(scope)?.unify(accum_type.as_ref())?;
                        let mut child_scope = TypeScope::child(scope);
                        child_scope
                            .push(name.clone(), ValueType::Tuple(vec![accum_type.clone(), *t]));
                        match expr
                            .infer_type(&child_scope)?
                            .unwrap_tuple_type()?
                            .as_mut_slice()
                        {
                            [accum_result, ValueType::Seq(t2)] => {
                                accum_result.unify(&accum_type)?;
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
                        let accum_type = accum.infer_type(scope)?.unify(accum_type.as_ref())?;
                        let mut child_scope = TypeScope::child(scope);
                        child_scope
                            .push(name.clone(), ValueType::Tuple(vec![accum_type.clone(), *t]));
                        Ok(expr.infer_type(&child_scope)?.unify(&accum_type)?)
                    }
                    other => Err(anyhow!("LeftFold: expected Seq, found {other:?}")),
                },
                other => Err(anyhow!("LeftFold: expected Lambda, found {other:?}")),
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
            Expr::Dup(count, expr) => {
                if count.infer_type(scope)? != ValueType::Base(BaseType::U32) {
                    return Err(anyhow!("Dup: count is not U32: {count:?}"));
                }
                let t = expr.infer_type(scope)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Expr::LiftOption(expr) => match expr {
                Some(expr) => Ok(ValueType::Option(Box::new(expr.infer_type(scope)?))),
                None => Ok(ValueType::Option(Box::new(ValueType::Any))),
            },
        }
    }

    /// Returns `true` if the evaluation of `self` contains any references to an external variable with a given identifier
    /// of `name`. This occurs when the expression contains `Expr::Var(name)` that it does not itself provide a local binding for
    /// (e.g. in a pattern-match or lambda head).
    pub fn is_shadowed_by(&self, name: &str) -> bool {
        match self {
            Expr::Var(vname) => vname == name,
            Expr::Lambda(hvar, body) => {
                if hvar == name {
                    false
                } else {
                    body.is_shadowed_by(name)
                }
            }
            Expr::Arith(_, x, y) | Expr::IntRel(_, x, y) => {
                x.is_shadowed_by(name) || y.is_shadowed_by(name)
            }
            Expr::Dup(x, y) => x.is_shadowed_by(name) || y.is_shadowed_by(name),
            Expr::Bool(_) | Expr::U8(_) | Expr::U16(_) | Expr::U32(_) | Expr::U64(_) => false,
            Expr::Tuple(ts) => ts.iter().any(|x| x.is_shadowed_by(name)),
            Expr::TupleProj(tup, _) => tup.is_shadowed_by(name),
            Expr::Record(fs) => {
                for (fname, fexpr) in fs.iter() {
                    if fexpr.is_shadowed_by(name) {
                        // NOTE - the first field-expr that is shadowed by `name` wins
                        return true;
                    } else if fname == name {
                        // NOTE - if a field-name matches `name` exactly, then it itself will shadow any binding of `name` external to the record
                        return false;
                    }
                }
                return false;
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
            Expr::Arith(Arith::Add, a, b) => a.bounds() + b.bounds(),
            Expr::Arith(Arith::Sub, a, b) => a.bounds() - b.bounds(),
            Expr::Arith(Arith::Mul, a, b) => a.bounds() * b.bounds(),
            Expr::Arith(Arith::Div, a, b) => a.bounds() / b.bounds(),
            Expr::Arith(Arith::BitOr, a, b) => a.bounds() | b.bounds(),
            Expr::Arith(Arith::BitAnd, a, b) => a.bounds() & b.bounds(),
            Expr::Arith(Arith::Shl, a, b) => a.bounds() << b.bounds(),
            Expr::Arith(Arith::Shr, a, b) => a.bounds() >> b.bounds(),
            _ => Bounds::any(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum DynFormat {
    Huffman(Box<Expr>, Option<Box<Expr>>),
}

/// Binary format descriptions
///
/// # Binary formats as regular expressions
///
/// Given a language of [regular expressions]:
///
/// ```text
/// r ∈ Regexp ::=
///   | ∅           empty set
///   | ε           empty byte string
///   | .           any byte
///   | b           literal byte
///   | r|r         alternation
///   | r r         concatenation
///   | r*          Kleene star
/// ```
///
/// We can use these to model a subset of our binary format descriptions:
///
/// ```text
/// ⟦ _ ⟧ : Format ⇀ Regexp
/// ⟦ Fail ⟧                                = ∅
/// ⟦ Byte({}) ⟧                            = ∅
/// ⟦ Byte(!{}) ⟧                           = .
/// ⟦ Byte({b}) ⟧                           = b
/// ⟦ Byte({b₀, ... bₙ}) ⟧                  = b₀ | ... | bₙ
/// ⟦ Union([]) ⟧                           = ∅
/// ⟦ Union([(l₀, f₀), ..., (lₙ, fₙ)]) ⟧    = ⟦ f₀ ⟧ | ... | ⟦ fₙ ⟧
/// ⟦ Tuple([]) ⟧                           = ε
/// ⟦ Tuple([f₀, ..., fₙ]) ⟧                = ⟦ f₀ ⟧ ... ⟦ fₙ ⟧
/// ⟦ Repeat(f) ⟧                           = ⟦ f ⟧*
/// ⟦ Repeat1(f) ⟧                          = ⟦ f ⟧ ⟦ f ⟧*
/// ⟦ RepeatCount(n, f) ⟧                   = ⟦ f ⟧ ... ⟦ f ⟧
///                                           ╰── n times ──╯
/// ```
///
/// Note that the data dependency present in record formats means that these
/// formats no longer describe regular languages.
///
/// [regular expressions]: https://en.wikipedia.org/wiki/Regular_expression#Formal_definition
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(tag = "tag", content = "data")]
pub enum Format {
    /// Reference to a top-level item
    ItemVar(usize, Vec<Expr>), // FIXME - do the exprs here need type(+) info?
    /// A format that never matches
    Fail,
    /// Matches if the end of the input has been reached
    EndOfInput,
    /// Skips bytes if necessary to align the current offset to a multiple of N
    Align(usize),
    /// Matches a byte in the given byte set
    Byte(ByteSet),
    /// Wraps the value from the inner format in a variant
    Variant(Label, Box<Format>),
    /// Matches the union of all the formats, which must have the same type
    Union(Vec<Format>),
    /// Nondeterministic unions, where the formats are not mutually exclusive
    UnionNondet(Vec<Format>),
    /// Matches a sequence of concatenated formats
    Tuple(Vec<Format>),
    /// Matches a sequence of named formats where later formats can depend on
    /// the decoded value of earlier formats
    Record(Vec<(Label, Format)>),
    /// Repeat a format zero-or-more times
    Repeat(Box<Format>),
    /// Repeat a format one-or-more times
    Repeat1(Box<Format>),
    /// Repeat a format an exact number of times
    RepeatCount(Box<Expr>, Box<Format>),
    /// Repeat a format at least N and at most M times
    RepeatBetween(Box<Expr>, Box<Expr>, Box<Format>),
    /// Repeat a format until a condition is satisfied by its last item
    RepeatUntilLast(Box<Expr>, Box<Format>),
    /// Repeat a format until a condition is satisfied by the sequence
    RepeatUntilSeq(Box<Expr>, Box<Format>),
    /// Apply a parametric format for each element of a sequence-typed Expr using a fused lambda binding
    ForEach(Box<Expr>, Label, Box<Format>),
    /// Parse a format if and only if the given expression evaluates to true, otherwise skip
    Maybe(Box<Expr>, Box<Format>),
    /// Parse a format without advancing the stream position afterwards
    Peek(Box<Format>),
    /// Attempt to parse a format and fail if it succeeds
    PeekNot(Box<Format>),
    /// Restrict a format to a sub-stream of a given number of bytes (skips any leftover bytes in the sub-stream)
    Slice(Box<Expr>, Box<Format>),
    /// Parse bitstream
    Bits(Box<Format>),
    /// Matches a format at a byte offset relative to the current stream position
    WithRelativeOffset(Box<Expr>, Box<Format>),
    /// Map a value with a lambda expression
    Map(Box<Format>, Box<Expr>),
    /// Assert that a boolean condition holds on a value
    Where(Box<Format>, Box<Expr>),
    /// Compute a value
    Compute(Box<Expr>),
    /// Let binding
    Let(Label, Box<Expr>, Box<Format>),
    /// Pattern match on an expression
    Match(Box<Expr>, Vec<(Pattern, Format)>),
    /// Format generated dynamically
    Dynamic(Label, DynFormat, Box<Format>),
    /// Apply a dynamic format from a named variable in the scope
    Apply(Label),
    /// Current byte-offset relative to start-of-buffer (as a U64(?))
    Pos,
    /// Skip the remainder of the stream, up until the end of input or the last available byte within a Slice
    SkipRemainder,
    /// Given an expression corresponding to a byte-sequence, decode it again using the provided Format. This can be used to reparse the initial decode of formats that output Vec<u8> or similar
    DecodeBytes(Box<Expr>, Box<Format>),
    /// Process one format, bind the result to a label, and process a second format, discarding the result of the first
    LetFormat(Box<Format>, Label, Box<Format>),
}

impl Format {
    pub const EMPTY: Format = Format::Tuple(Vec::new());

    pub fn alts<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
        Format::Union(
            fields
                .into_iter()
                .map(|(label, format)| Format::Variant(label.into(), Box::new(format)))
                .collect(),
        )
    }

    pub fn record<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
        Format::Record(
            fields
                .into_iter()
                .map(|(label, format)| (label.into(), format))
                .collect(),
        )
    }
}

impl Format {
    /// Conservative bounds for number of byte-positions advanced after a format is matched (i.e. parsed)
    fn match_bounds(&self, module: &FormatModule) -> Bounds {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).match_bounds(module),
            Format::Fail => Bounds::exact(0),
            Format::EndOfInput => Bounds::exact(0),
            Format::SkipRemainder => Bounds::any(),
            Format::Align(0) => unreachable!("illegal Format::Align modulus (== 0)"),
            Format::Align(n) => Bounds::new(0, n - 1),
            Format::Byte(_) => Bounds::exact(1),
            Format::Variant(_label, f) => f.match_bounds(module),
            Format::Union(branches) | Format::UnionNondet(branches) => branches
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Tuple(fields) => fields
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Record(fields) => fields
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Repeat(_) => Bounds::any(),
            Format::Repeat1(f) => f.match_bounds(module) * Bounds::at_least(1),
            Format::RepeatCount(expr, f) => f.match_bounds(module) * expr.bounds(),
            Format::RepeatBetween(xmin, xmax, f) => {
                f.match_bounds(module) * (Bounds::union(xmin.bounds(), xmax.bounds()))
            }
            Format::RepeatUntilLast(_, f) => f.match_bounds(module) * Bounds::at_least(1),
            Format::RepeatUntilSeq(_, _f) => Bounds::any(),
            Format::Maybe(_, f) => Bounds::union(Bounds::exact(0), f.match_bounds(module)),
            Format::Peek(_) => Bounds::exact(0),
            Format::PeekNot(_) => Bounds::exact(0),
            Format::Slice(expr, _) => expr.bounds(),
            Format::Bits(f) => f.match_bounds(module).bits_to_bytes(),
            Format::WithRelativeOffset(_, _) => Bounds::exact(0),
            Format::Map(f, _expr) => f.match_bounds(module),
            Format::Where(f, _expr) => f.match_bounds(module),
            Format::Compute(_) | Format::Pos => Bounds::exact(0),
            Format::Let(_name, _expr, f) => f.match_bounds(module),
            Format::Match(_, branches) => branches
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Dynamic(_name, _dynformat, f) => f.match_bounds(module),
            Format::Apply(_) => Bounds::at_least(1),
            // FIXME - do we have any way of approximating this better?
            Format::ForEach(_expr, _lbl, _f) => Bounds::any(),
            // NOTE - because we are parsing a sequence of bytes, we do not interact with the actual buffer
            Format::DecodeBytes(_bytes, _f) => Bounds::exact(0),
            Format::LetFormat(first, _, second) => {
                first.match_bounds(module) + second.match_bounds(module)
            }
        }
    }

    /// Conservative bounds for number of bytes that may be read in order to fully parse the given Format, regardless of how many
    /// are consumed as opposed to being left untouched in the buffer.
    pub(crate) fn lookahead_bounds(&self, module: &FormatModule) -> Bounds {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).lookahead_bounds(module),
            Format::Fail => Bounds::exact(0),
            Format::EndOfInput => Bounds::exact(0),
            // NOTE - for PeekNot purposes it is not fully clear how to treat SkipRemainder, but we want to mirror the behavior of `Repeat(Byte)`
            Format::SkipRemainder => Bounds::any(),
            Format::Align(0) => unreachable!("illegal Format::Align modulus (== 0)"),
            Format::Align(n) => Bounds::new(0, n - 1),
            Format::Byte(_) => Bounds::exact(1),
            Format::Variant(_label, f) => f.lookahead_bounds(module),
            Format::Union(branches) | Format::UnionNondet(branches) => branches
                .iter()
                .map(|f| f.lookahead_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Tuple(fields) => fields
                .iter()
                .map(|f| f.lookahead_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Record(fields) => fields
                .iter()
                .map(|(_, f)| f.lookahead_bounds(module))
                .reduce(Bounds::add)
                .unwrap_or(Bounds::exact(0)),
            Format::Repeat(_) => Bounds::any(),
            // FIXME - do we have any way of approximating this better?
            Format::ForEach(_expr, _lbl, _f) => Bounds::any(),
            Format::Repeat1(f) => f.lookahead_bounds(module) * Bounds::at_least(1),
            Format::RepeatCount(expr, f) => f.lookahead_bounds(module) * expr.bounds(),
            Format::RepeatBetween(xmin, xmax, f) => {
                f.lookahead_bounds(module) * Bounds::union(xmin.bounds(), xmax.bounds())
            }
            Format::RepeatUntilLast(_, f) => f.lookahead_bounds(module) * Bounds::at_least(1),
            Format::RepeatUntilSeq(_, _f) => Bounds::any(),
            Format::Maybe(_, f) => Bounds::union(Bounds::exact(0), f.lookahead_bounds(module)),
            Format::Peek(f) => f.lookahead_bounds(module),
            Format::PeekNot(f) => f.lookahead_bounds(module),
            Format::Slice(expr, _) => expr.bounds(),
            Format::Bits(f) => f.lookahead_bounds(module).bits_to_bytes(),
            Format::WithRelativeOffset(expr, f) => expr.bounds() + f.lookahead_bounds(module),
            Format::Map(f, _expr) => f.lookahead_bounds(module),
            Format::Where(f, _expr) => f.lookahead_bounds(module),
            Format::Compute(_) | Format::Pos => Bounds::exact(0),
            Format::Let(_name, _expr, f) => f.lookahead_bounds(module),
            Format::Match(_, branches) => branches
                .iter()
                .map(|(_, f)| f.lookahead_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            Format::Dynamic(_name, _dynformat, f) => f.lookahead_bounds(module),
            Format::Apply(_) => Bounds::at_least(1),
            Format::DecodeBytes(_bytes, _f) => Bounds::exact(0),
            Format::LetFormat(f0, _, f) => Bounds::union(
                f0.lookahead_bounds(module),
                f0.match_bounds(module) + f.lookahead_bounds(module),
            ),
        }
    }

    /// Returns `true` if the format could match the empty byte string
    fn is_nullable(&self, module: &FormatModule) -> bool {
        self.match_bounds(module).min == 0
    }

    /// True if the compilation of this format depends on the format that follows it
    fn depends_on_next(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => module.get_format(*level).depends_on_next(module),
            Format::Fail => false,
            Format::EndOfInput => false,
            // NOTE - compiling SkipRemainder doesn't depend on the next format because the next format can only ever match the empty byte string at that point
            Format::SkipRemainder => false,
            Format::Align(..) => false,
            Format::Byte(..) => false,
            Format::Variant(_label, f) => f.depends_on_next(module),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                Format::union_depends_on_next(branches, module)
            }
            Format::Tuple(fields) => fields.iter().any(|f| f.depends_on_next(module)),
            Format::Record(fields) => fields.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Repeat(..) => true,
            Format::Repeat1(..) => true,
            Format::RepeatBetween(..) => true,
            Format::RepeatCount(..) => false,
            Format::RepeatUntilLast(..) => false,
            Format::RepeatUntilSeq(..) => false,
            Format::Maybe(..) => true,
            Format::Peek(..) => false,
            Format::PeekNot(..) => false,
            Format::Slice(..) => false,
            Format::Bits(..) => false,
            Format::WithRelativeOffset(..) => false,
            Format::Map(f, _expr) => f.depends_on_next(module),
            Format::Where(f, _expr) => f.depends_on_next(module),
            Format::Compute(..) | Format::Pos => false,
            Format::Let(_name, _expr, f) => f.depends_on_next(module),
            Format::Match(_, branches) => branches.iter().any(|(_, f)| f.depends_on_next(module)),
            Format::Dynamic(_name, _dynformat, f) => f.depends_on_next(module),
            Format::Apply(..) => false,
            Format::ForEach(_expr, _lbl, f) => f.depends_on_next(module),
            Format::DecodeBytes(_bytes, _f) => false,
            Format::LetFormat(first, _, second) => {
                first.depends_on_next(module) || second.depends_on_next(module)
            }
        }
    }

    fn union_depends_on_next(branches: &[Format], module: &FormatModule) -> bool {
        let mut fs = Vec::with_capacity(branches.len());
        for f in branches {
            if f.depends_on_next(module) {
                return true;
            }
            fs.push(f.clone());
        }
        MatchTree::build(module, &fs, Rc::new(Next::Empty)).is_none()
    }
}

impl Format {
    /// Returns `true` if values associated to this format should be handled as single ASCII characters
    pub fn is_ascii_char_format(&self, module: &FormatModule) -> bool {
        match self {
            // NOTE - currently only true for named formats matching 'base\.ascii-char.*'
            Format::ItemVar(level, _args) => module.get_name(*level).starts_with("base.ascii-char"),
            _ => false,
        }
    }

    /// Returns `true` if values associated to this format should be handled as multi-character ASCII strings
    pub fn is_ascii_string_format(&self, module: &FormatModule) -> bool {
        match self {
            Format::ItemVar(level, _args) => {
                let fmt_name = module.get_name(*level);
                // REVIEW - consider different heuristic for short-circuit
                if fmt_name.contains("ascii-string") || fmt_name.contains("asciiz-string") {
                    return true;
                }
                module.get_format(*level).is_ascii_string_format(module)
            }
            Format::Tuple(formats) => {
                !formats.is_empty() && formats.iter().all(|f| f.is_ascii_char_format(module))
            }
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => format.is_ascii_char_format(module),
            Format::Slice(_, format) => format.is_ascii_string_format(module),
            // NOTE there may be other cases we should consider ASCII
            _ => false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct FormatRef(usize);

impl FormatRef {
    pub fn get_level(&self) -> usize {
        self.0
    }

    pub fn call(&self) -> Format {
        Format::ItemVar(self.0, vec![])
    }

    pub fn call_args(&self, args: Vec<Expr>) -> Format {
        Format::ItemVar(self.0, args)
    }
}

#[derive(Debug, Serialize)]
pub struct FormatModule {
    names: Vec<Label>,
    args: Vec<Vec<(Label, ValueType)>>,
    formats: Vec<Format>,
    format_types: Vec<ValueType>,
}

impl FormatModule {
    pub fn new() -> FormatModule {
        FormatModule {
            names: Vec::new(),
            args: Vec::new(),
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
        let mut scope = TypeScope::new();
        for (arg_name, arg_type) in &args {
            scope.push(arg_name.clone(), arg_type.clone());
        }
        let format_type = match self.infer_format_type(&scope, &format) {
            Ok(t) => t,
            Err(msg) => panic!("{msg}"),
        };
        let level = self.names.len();
        self.names.push(name.into());
        self.args.push(args);
        self.formats.push(format);
        self.format_types.push(format_type);
        FormatRef(level)
    }

    pub fn get_name(&self, level: usize) -> &str {
        &self.names[level]
    }

    pub fn iter_formats(&self) -> impl Iterator<Item = (usize, Format)> + '_ {
        (0..self.formats.len()).filter_map(|ix| {
            let mut x_args = Vec::with_capacity(self.args[ix].len());
            for (_, vt) in self.args[ix].iter() {
                x_args.push(mk_value_expr(vt)?);
            }
            Some((ix, Format::ItemVar(ix, x_args)))
        })
    }

    fn get_args(&self, level: usize) -> &[(Label, ValueType)] {
        &self.args[level]
    }

    fn get_format(&self, level: usize) -> &Format {
        &self.formats[level]
    }

    pub fn get_format_type(&self, level: usize) -> &ValueType {
        &self.format_types[level]
    }

    fn infer_format_type(&self, scope: &TypeScope<'_>, f: &Format) -> AResult<ValueType> {
        match f {
            Format::ItemVar(level, arg_exprs) => {
                let arg_names = self.get_args(*level);
                if arg_names.len() != arg_exprs.len() {
                    return Err(anyhow!(
                        "Expected {} arguments, found {}",
                        arg_names.len(),
                        arg_exprs.len()
                    ));
                }
                for ((_name, arg_type), expr) in Iterator::zip(arg_names.iter(), arg_exprs.iter()) {
                    let t = expr.infer_type(scope)?;
                    let _t = arg_type.unify(&t)?;
                }
                Ok(self.get_format_type(*level).clone())
            }
            Format::DecodeBytes(bytes, f) => {
                let bytes_type = bytes.infer_type(scope)?;
                match bytes_type {
                    ValueType::Seq(bt) if matches!(*bt, ValueType::Base(BaseType::U8)) => {
                        self.infer_format_type(scope, f)
                    }
                    other => Err(anyhow!("DecodeBytes first argument type should be Seq(U8), found {other:?} instead")),
                }
            }
            Format::Fail => Ok(ValueType::Empty),
            Format::SkipRemainder | Format::EndOfInput => Ok(ValueType::Tuple(vec![])),
            Format::Align(_n) => Ok(ValueType::Tuple(vec![])),
            Format::Byte(_bs) => Ok(ValueType::Base(BaseType::U8)),
            Format::Variant(label, f) => Ok(ValueType::Union(BTreeMap::from([(
                label.clone(),
                self.infer_format_type(scope, f)?,
            )]))),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut t = ValueType::Any;
                for f in branches {
                    t = t.unify(&self.infer_format_type(scope, f)?)?;
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
            Format::Record(fields) => {
                let mut ts = Vec::with_capacity(fields.len());
                let mut record_scope = TypeScope::child(scope);
                for (label, f) in fields {
                    let t = self.infer_format_type(&record_scope, f)?;
                    ts.push((label.clone(), t.clone()));
                    record_scope.push(label.clone(), t);
                }
                Ok(ValueType::Record(ts))
            }
            Format::Repeat(a) | Format::Repeat1(a) => {
                let t = self.infer_format_type(scope, a)?;
                Ok(ValueType::Seq(Box::new(t)))
            }
            Format::RepeatCount(count, a) => {
                match count.infer_type(scope)? {
                    ValueType::Base(b) if b.is_numeric() => {
                        let t = self.infer_format_type(scope, a)?;
                        Ok(ValueType::Seq(Box::new(t)))
                    }
                    other => Err(anyhow!("RepeatCount first argument type should be numeric, found {other:?} instead")),
                }
            }

            Format::RepeatBetween(min, max, a) => {
                match min.infer_type(scope)? {
                    ref t0 @ ValueType::Base(b0) if b0.is_numeric() => {
                        match max.infer_type(scope)? {
                            ValueType::Base(b1) if b0 == b1 => {
                                let t = self.infer_format_type(scope, a)?;
                                Ok(ValueType::Seq(Box::new(t)))
                            }
                            other => return Err(anyhow!("RepeatBetween second argument type should be the same as the first, found {other:?} (!= {t0:?})")),
                        }
                    }
                    other => return Err(anyhow!("RepeatBetween first argument type should be numeric, found {other:?} instead")),
                }
            }
            Format::RepeatUntilLast(lambda_elem, a) => {
                match lambda_elem.as_ref() {
                    Expr::Lambda(head, expr) => {
                        let t = self.infer_format_type(scope, a)?;
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(head.clone(), t.clone());
                        let ret_type = expr.infer_type(&child_scope)?;
                        match ret_type {
                            ValueType::Base(BaseType::Bool) => Ok(ValueType::Seq(Box::new(t))),
                            other => Err(anyhow!("RepeatUntilLast first argument (lambda) return type should be Bool, found {other:?} instead")),
                        }
                    }
                    other => return Err(anyhow!("RepeatUntilLast first argument type should be lambda, found {other:?} instead")),
                }
            }
            Format::RepeatUntilSeq(lambda_seq, a) => {
                match lambda_seq.as_ref() {
                    Expr::Lambda(head, expr) => {
                        let t = self.infer_format_type(scope, a)?;
                        let mut child_scope = TypeScope::child(scope);
                        child_scope.push(head.clone(), ValueType::Seq(Box::new(t.clone())));
                        let ret_type = expr.infer_type(&child_scope)?;
                        match ret_type {
                            ValueType::Base(BaseType::Bool) => Ok(ValueType::Seq(Box::new(t))),
                            other => Err(anyhow!("RepeatUntilSeq first argument (lambda) return type should be Bool, found {other:?} instead")),
                        }
                    }
                    other => return Err(anyhow!("RepeatUntilSeq first argument type should be lambda, found {other:?} instead")),
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
            Format::Slice(_expr, a) => self.infer_format_type(scope, a),
            Format::Bits(a) => self.infer_format_type(scope, a),
            Format::WithRelativeOffset(_expr, a) => self.infer_format_type(scope, a),
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
            Format::LetFormat(f0, name, f) => {
                let t0 = self.infer_format_type(scope, f0)?;
                let mut new_scope = TypeScope::child(scope);
                new_scope.push(name.clone(), t0);
                self.infer_format_type(&new_scope, f)
            }
            Format::Match(head, branches) => {
                if branches.is_empty() {
                    return Err(anyhow!("infer_format_type: empty Match"));
                }
                let head_type = Rc::new(head.infer_type(scope)?);
                let mut t = ValueType::Any;
                for (pattern, branch) in branches {
                    t = t.unify(&pattern.infer_format_branch_type(
                        scope,
                        head_type.clone(),
                        self,
                        branch,
                    )?)?;
                }
                Ok(t)
            }
            Format::Dynamic(name, dynformat, format) => {
                match dynformat {
                    DynFormat::Huffman(lengths_expr, _opt_values_expr) => {
                        match lengths_expr.infer_type(scope)? {
                            ValueType::Seq(t) => match &*t {
                                ValueType::Base(BaseType::U8) | ValueType::Base(BaseType::U16) => {}
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
                        // FIXME check opt_values_expr type
                    }
                }
                let mut child_scope = TypeScope::child(scope);
                child_scope.push_format(name.clone(), ValueType::Base(BaseType::U16));
                self.infer_format_type(&child_scope, format)
            }
            Format::Apply(name) => match scope.get_type_by_name(name) {
                ValueKind::Format(t) => Ok(t.clone()),
                ValueKind::Value(t) => Err(anyhow!("Apply: expected format, found {t:?}")),
            },
            // REVIEW - do we want to hard-code this as U64 or make it a flexibly abstract integer type?
            Format::Pos => Ok(ValueType::Base(BaseType::U64)),
            Format::ForEach(expr, lbl, format) => {
                let expr_t = expr.infer_type(scope)?;
                let elem_t = match expr_t {
                    ValueType::Seq(elem_t) => (*elem_t).clone(),
                    _ => return Err(anyhow!("ForEach: expected Seq, found {expr_t:?}")),
                };
                let mut child_scope = TypeScope::child(scope);
                child_scope.push(lbl.clone(), elem_t);
                let inner_t = self.infer_format_type(&child_scope, &format)?;
                Ok(ValueType::Seq(Box::new(inner_t)))
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
        match self {
            Self::Untyped(arg0) => Self::Untyped(*arg0),
            Self::Typed(arg0) => Self::Typed(*arg0),
        }
    }
}

impl<'a, U: ?Sized + 'a, T: ?Sized + 'a> Copy for MaybeTyped<'a, U, T> {}

type MTFormatRef<'a> = MaybeTyped<'a, Format, TypedFormat<GenType>>;
type MTFormatSlice<'a> = MaybeTyped<'a, [Format], [TypedFormat<GenType>]>;
type MTFieldSlice<'a> = MaybeTyped<'a, [(Label, Format)], [(Label, TypedFormat<GenType>)]>;

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
    Tuple(MTFormatSlice<'a>, Rc<Next<'a>>),
    Record(MTFieldSlice<'a>, Rc<Next<'a>>),
    Repeat(MTFormatRef<'a>, Rc<Next<'a>>),
    RepeatCount(usize, MTFormatRef<'a>, Rc<Next<'a>>),
    RepeatMax(usize, MTFormatRef<'a>, Rc<Next<'a>>), // dual to [RepeatCount] for 0..=N repeats
    RepeatBetween(usize, usize, MTFormatRef<'a>, Rc<Next<'a>>), // extension of RepeatMax/RepeatCount for N..=M repeats
    Slice(usize, Rc<Next<'a>>, Rc<Next<'a>>),
    Peek(Rc<Next<'a>>, Rc<Next<'a>>),
    PeekNot(Rc<Next<'a>>, Rc<Next<'a>>),
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

/// A byte-level prefix-tree evaluated to a fixed depth.
#[derive(Clone, Debug, Hash)]
pub struct MatchTree {
    accept: Option<usize>,
    branches: Vec<(ByteSet, MatchTree)>,
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
                for (bs2, next2) in &peek.branches {
                    let common = bs1.intersection(bs2);
                    let diff = bs1.difference(bs2);
                    if !common.is_empty() {
                        let next = Rc::new(Next::PeekNot(next1.clone(), next2.clone()));
                        branches.push((common, next));
                    }
                    if !diff.is_empty() {
                        branches.push((diff, next1.clone()));
                    }
                }
            }
            self.branches = branches;
        }
        self
    }

    /// Constructs a [MatchTreeStep] that accepts a given tuple of sequential formats, with a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_tuple(
        module: &'a FormatModule,
        fields: &'a [Format],
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::from_next(module, next),
            Some((f, fs)) => Self::from_format(
                module,
                f,
                Rc::new(Next::Tuple(MaybeTyped::Untyped(fs), next)),
            ),
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a given record of sequential formats, with a trailing sequence of partially-consumed formats ([`Next`]s).
    ///
    /// This is mostly equivalent to `from_tuple`, as the name of a given field does not have implications on the prefix tree of the overall format.
    fn from_record(
        module: &'a FormatModule,
        fields: &'a [(Label, Format)],
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        match fields.split_first() {
            None => Self::from_next(module, next),
            Some(((_label, f), fs)) => Self::from_format(
                module,
                f,
                Rc::new(Next::Record(MaybeTyped::Untyped(fs), next)),
            ),
        }
    }

    /// Constructs a [MatchTreeStep] that accepts a fixed-count repetition of a given format, with a trailing sequence of partially-consumed formats ([`Next`]s).
    fn from_repeat_count(
        module: &'a FormatModule,
        n: usize,
        format: &'a Format,
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            Self::from_format(
                module,
                format,
                Rc::new(Next::RepeatCount(n - 1, MaybeTyped::Untyped(format), next)),
            )
        } else {
            Self::from_next(module, next)
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
    ) -> MatchTreeStep<'a> {
        let (min, max) = min_max;
        assert!(
            min <= max,
            "min-max pair ({}, {}) incoherent (min > max)",
            min,
            max
        );
        if min == max {
            Self::from_repeat_count(module, min, format, next)
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
            )
        } else {
            Self::from_next(
                module,
                Rc::new(Next::RepeatMax(
                    max,
                    MaybeTyped::Untyped(format),
                    next.clone(),
                )),
            )
        }
    }

    /// Constructs a [MatchTreeStep] from a given (partial) format `inner` and a slice-length `n`
    pub fn from_slice(
        module: &'a FormatModule,
        n: usize,
        inner: Rc<Next<'a>>,
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        if n > 0 {
            let mut tree = Self::from_next(module, inner);
            tree.accept = false;
            if tree.branches.is_empty() {
                let next = Rc::new(Next::Slice(n - 1, Rc::new(Next::Empty), next.clone()));
                tree.branches.push((ByteSet::full(), next));
            } else {
                for (_bs, ref mut inside) in tree.branches.iter_mut() {
                    *inside = Rc::new(Next::Slice(n - 1, inside.clone(), next.clone()));
                }
            }
            tree
        } else {
            Self::from_next(module, next.clone())
        }
    }

    /// Constructs a [MatchTreeStep] from a [`Next`]
    fn from_next(module: &'a FormatModule, next: Rc<Next<'a>>) -> MatchTreeStep<'a> {
        match next.as_ref() {
            Next::Empty => Self::accept(),
            Next::Union(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone());
                let tree2 = Self::from_next(module, next2.clone());
                tree1.union(tree2)
            }
            Next::Cat(f, next) => {
                let next0: Rc<Next<'a>> = next.clone();
                MatchTreeStep::<'a>::from_mt_format(module, *f, next0)
            }
            Next::Tuple(fields, next) => {
                let next = next.clone();
                match fields {
                    MaybeTyped::Untyped(fields) => match fields.split_first() {
                        None => Self::from_next(module, next),
                        Some((f, fs)) => Self::from_format(
                            module,
                            f,
                            Rc::new(Next::Tuple(MaybeTyped::Untyped(fs), next)),
                        ),
                    },
                    MaybeTyped::Typed(fields) => match fields.split_first() {
                        None => Self::from_next(module, next),
                        Some((f, fs)) => Self::from_gt_format(
                            module,
                            f,
                            Rc::new(Next::Tuple(MaybeTyped::Typed(fs), next)),
                        ),
                    },
                }
            }
            Next::Record(fields, next) => {
                let next = next.clone();
                match fields {
                    MaybeTyped::Untyped(fields) => match fields.split_first() {
                        None => Self::from_next(module, next),
                        Some(((_label, f), fs)) => Self::from_format(
                            module,
                            f,
                            Rc::new(Next::Record(MaybeTyped::Untyped(fs), next)),
                        ),
                    },
                    MaybeTyped::Typed(fields) => match fields.split_first() {
                        None => Self::from_next(module, next),
                        Some(((_label, f), fs)) => Self::from_gt_format(
                            module,
                            f,
                            Rc::new(Next::Record(MaybeTyped::Typed(fs), next)),
                        ),
                    },
                }
            }
            Next::Repeat(a, next0) => {
                let tree = MatchTreeStep::<'a>::from_next(module, next0.clone());
                let next1 = next.clone();
                tree.union(MatchTreeStep::<'a>::from_mt_format(module, *a, next1))
            }
            Next::RepeatBetween(n, m, a, next0) => {
                let min = *n;
                let max = *m;
                if min == max {
                    // FIXME - this is technically allowable but we don't expect to get here...
                    unreachable!("RepeatBetween(x, y, ..) precludes x == y");
                }
                if min > 0 {
                    Self::from_mt_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatBetween(min - 1, max - 1, *a, next0.clone())),
                    )
                } else {
                    Self::from_next(module, Rc::new(Next::RepeatMax(max, *a, next0.clone())))
                }
            }
            Next::RepeatMax(n, a, next0) => {
                let n = *n;
                if n == 0 {
                    Self::from_next(module, next0.clone())
                } else {
                    let tree0 = MatchTreeStep::<'a>::from_next(module, next0.clone());
                    tree0.union(MatchTreeStep::<'a>::from_mt_format(
                        module,
                        *a,
                        Rc::new(Next::RepeatMax(n - 1, *a, next0.clone())),
                    ))
                }
            }
            Next::RepeatCount(n, a, next0) => {
                let n = *n;
                let next = next0.clone();
                if n > 0 {
                    Self::from_mt_format(module, *a, Rc::new(Next::RepeatCount(n - 1, *a, next)))
                } else {
                    Self::from_next(module, next)
                }
            }
            Next::Slice(n, inside, next0) => {
                Self::from_slice(module, *n, inside.clone(), next0.clone())
            }
            Next::Peek(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone());
                let tree2 = Self::from_next(module, next2.clone());
                tree1.peek(tree2)
            }
            Next::PeekNot(next1, next2) => {
                let tree1 = Self::from_next(module, next1.clone());
                let tree2 = Self::from_next(module, next2.clone());
                tree1.peek_not(tree2)
            }
        }
    }

    pub fn from_mt_format(
        module: &'a FormatModule,
        f: MTFormatRef<'a>,
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        match f {
            MaybeTyped::Untyped(f) => Self::from_format(module, f, next),
            MaybeTyped::Typed(tf) => Self::from_gt_format(module, tf, next),
        }
    }

    pub fn from_gt_format(
        module: &'a FormatModule,
        f: &'a TypedFormat<GenType>,
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        match f {
            TypedFormat::FormatCall(_, level, ..) => {
                Self::from_format(module, module.get_format(*level), next)
            }
            TypedFormat::Fail => Self::reject(),
            TypedFormat::EndOfInput => Self::accept(),
            TypedFormat::Align(_) => {
                Self::accept() // FIXME
            }
            TypedFormat::SkipRemainder => Self::accept(),
            TypedFormat::Byte(bs) => Self::branch(*bs, next),
            TypedFormat::Variant(_, _label, f) => Self::from_gt_format(module, f, next.clone()),
            TypedFormat::Union(_, branches) | TypedFormat::UnionNondet(_, branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_gt_format(module, f, next.clone()));
                }
                tree
            }
            TypedFormat::Tuple(_, fields) => match fields.split_first() {
                None => Self::from_next(module, next),
                Some((f, fs)) => Self::from_gt_format(
                    module,
                    f,
                    Rc::new(Next::Tuple(MaybeTyped::Typed(fs), next)),
                ),
            },
            TypedFormat::Record(_, fields) => match fields.split_first() {
                None => Self::from_next(module, next),
                Some(((_label, f), fs)) => Self::from_gt_format(
                    module,
                    f,
                    Rc::new(Next::Record(MaybeTyped::Typed(fs), next)),
                ),
            },

            TypedFormat::Repeat(_, a) => {
                let tree = Self::from_next(module, next.clone());
                tree.union(Self::from_gt_format(
                    module,
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                ))
            }
            TypedFormat::ForEach(_, _expr, _lbl, a) => {
                let tree = Self::from_next(module, next.clone());
                // FIXME - we might want a more robust solution to nullable formats in ForEach
                let bounds = a.match_bounds();
                if bounds.min() == 0 {
                    match bounds.max() {
                        Some(0) => tree,
                        _ => {
                            // format is nullable but might match something
                            // FIXME - this is a stopgap for the complex logic we would otherwise need to model
                            Self::accept()
                        }
                    }
                } else {
                    tree.union(Self::from_gt_format(
                        module,
                        a,
                        Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
                    ))
                }
            }
            TypedFormat::Repeat1(_, a) => Self::from_gt_format(
                module,
                a,
                Rc::new(Next::Repeat(MaybeTyped::Typed(a), next.clone())),
            ),
            TypedFormat::RepeatCount(_, expr, a) => {
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    {
                        let next = next.clone();
                        if n > 0 {
                            Self::from_gt_format(
                                module,
                                a,
                                Rc::new(Next::RepeatCount(n - 1, MaybeTyped::Typed(a), next)),
                            )
                        } else {
                            Self::from_next(module, next)
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
                            )
                        } else {
                            Self::from_next(module, next)
                        }
                    }
                }
            }
            TypedFormat::RepeatBetween(_, xmin, xmax, a) => {
                let min_bounds = xmin.bounds();
                let max_bounds = xmax.bounds();
                match (min_bounds.is_exact(), max_bounds.is_exact()) {
                    (Some(min), Some(max)) => match min.cmp(&max) {
                        Ordering::Less => {
                            if min > 0 {
                                Self::from_gt_format(
                                    module,
                                    &**a,
                                    Rc::new(Next::RepeatBetween(
                                        min - 1,
                                        max - 1,
                                        MaybeTyped::Typed(&**a),
                                        next,
                                    )),
                                )
                            } else {
                                Self::from_next(
                                    module,
                                    Rc::new(Next::RepeatMax(
                                        max,
                                        MaybeTyped::Typed(&**a),
                                        next.clone(),
                                    )),
                                )
                            }
                        }
                        Ordering::Equal => {
                            let next = next.clone();
                            if min > 0 {
                                Self::from_gt_format(
                                    module,
                                    &**a,
                                    Rc::new(Next::RepeatCount(
                                        min - 1,
                                        MaybeTyped::Typed(&**a),
                                        next,
                                    )),
                                )
                            } else {
                                Self::from_next(module, next)
                            }
                        }
                        Ordering::Greater => {
                            panic!("incoherent repeat-between: min {} > max {}", min, max)
                        }
                    },
                    _ => {
                        unreachable!("inexact repeat-between bounds (not technically a problem but not what the combinator was designed for...");
                    }
                }
            }
            TypedFormat::RepeatUntilLast(_, _expr, _a) => {
                Self::accept() // FIXME
            }
            TypedFormat::RepeatUntilSeq(_, _expr, _a) => {
                Self::accept() // FIXME
            }
            TypedFormat::Maybe(_, _cond, a) => {
                let tree_some = Self::from_gt_format(module, a, next.clone());
                let tree_none = Self::from_next(module, next);
                tree_none.union(tree_some)
            }
            TypedFormat::Peek(_, a) => {
                let tree = Self::from_next(module, next.clone());
                let peek = Self::from_gt_format(module, a, Rc::new(Next::Empty));
                tree.peek(peek)
            }
            TypedFormat::PeekNot(_, a) => {
                let tree = Self::from_next(module, next.clone());
                let peek = Self::from_gt_format(module, a, Rc::new(Next::Empty));
                tree.peek_not(peek)
            }
            TypedFormat::Slice(_, expr, f) => {
                let inside = Rc::new(Next::Cat(
                    MaybeTyped::Typed(f.as_ref()),
                    Rc::new(Next::Empty),
                ));
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    Self::from_slice(module, n, inside, next)
                } else {
                    Self::from_slice(module, bounds.min, inside, Rc::new(Next::Empty))
                }
            }
            TypedFormat::Bits(_, _a) => {
                Self::accept() // FIXME
            }
            TypedFormat::WithRelativeOffset(_, expr, a) => {
                // REVIEW - this is a bit hackish but it is at least somewhat better than before
                let tree = Self::from_next(module, next.clone());
                let bounds = expr.bounds();
                match bounds.is_exact() {
                    None => tree, // if the lookahead is indeterminate, ignore it
                    Some(n) => {
                        let peek = match n {
                            0 => Self::from_gt_format(module, a, Rc::new(Next::Empty)),
                            _ => Self::from_slice(
                                module,
                                n,
                                Rc::new(Next::Empty),
                                Rc::new(Next::Tuple(
                                    MaybeTyped::Typed(std::slice::from_ref(a.as_ref())),
                                    next,
                                )),
                            ),
                        };
                        tree.peek(peek)
                    }
                }
            }
            TypedFormat::Map(_, f, _expr) | TypedFormat::Where(_, f, _expr) => {
                Self::from_gt_format(module, f, next)
            }
            TypedFormat::DecodeBytes(..) | TypedFormat::Compute(..) => {
                Self::from_next(module, next)
            }
            TypedFormat::Pos => Self::from_next(module, next),
            TypedFormat::Let(_, _name, _expr, f) => Self::from_gt_format(module, f, next),
            TypedFormat::Match(_, _, branches) => {
                let mut tree = Self::reject();
                for (_, f) in branches {
                    tree = tree.union(Self::from_gt_format(module, f, next.clone()));
                }
                tree
            }
            TypedFormat::Dynamic(_, _name, _expr, f) => Self::from_gt_format(module, f, next),
            TypedFormat::Apply(..) => Self::accept(),
            TypedFormat::LetFormat(_, f0, _name, f) => {
                let next0 = Rc::new(Next::Cat(MaybeTyped::Typed(f), next));
                Self::from_gt_format(module, f0, next0)
            }
        }
    }

    /// Constructs a [MatchTreeStep] from an Format and a trailing [`Next`] value
    pub fn from_format(
        module: &'a FormatModule,
        f: &'a Format,
        next: Rc<Next<'a>>,
    ) -> MatchTreeStep<'a> {
        match f {
            Format::ItemVar(level, _args) => {
                Self::from_format(module, module.get_format(*level), next)
            }
            Format::Fail => Self::reject(),
            Format::EndOfInput => Self::accept(),
            Format::SkipRemainder => Self::accept(),
            Format::Align(n) => Self::from_align(module, next, *n),
            Format::DecodeBytes(_bytes, _f) => Self::from_next(module, next),
            Format::Byte(bs) => Self::branch(*bs, next),
            Format::Variant(_label, f) => Self::from_format(module, f, next.clone()),
            Format::Union(branches) | Format::UnionNondet(branches) => {
                let mut tree = Self::reject();
                for f in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone()));
                }
                tree
            }
            Format::Tuple(fields) => Self::from_tuple(module, fields, next),
            Format::Record(fields) => Self::from_record(module, fields, next),
            Format::Repeat(a) => {
                let tree = Self::from_next(module, next.clone());
                tree.union(Self::from_format(
                    module,
                    a,
                    Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                ))
            }
            Format::ForEach(_expr, _lbl, a) => {
                let tree = Self::from_next(module, next.clone());
                // FIXME - we might want a more robust solution to nullable formats in ForEach
                let bounds = a.match_bounds(module);
                if bounds.min() == 0 {
                    match bounds.max() {
                        Some(0) => tree,
                        _ => {
                            // format is nullable but might match something
                            // FIXME - this is a stopgap for the complex logic we would otherwise need to model
                            Self::accept()
                        }
                    }
                } else {
                    tree.union(Self::from_format(
                        module,
                        a,
                        Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
                    ))
                }
            }
            Format::Repeat1(a) => Self::from_format(
                module,
                a,
                Rc::new(Next::Repeat(MaybeTyped::Untyped(a), next.clone())),
            ),
            Format::RepeatCount(expr, a) => {
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    Self::from_repeat_count(module, n, a, next.clone())
                } else {
                    Self::from_repeat_count(module, bounds.min, a, Rc::new(Next::Empty))
                }
            }
            Format::RepeatBetween(xmin, xmax, a) => {
                let min_bounds = xmin.bounds();
                let max_bounds = xmax.bounds();
                match (min_bounds.is_exact(), max_bounds.is_exact()) {
                    (Some(min), Some(max)) => match min.cmp(&max) {
                        Ordering::Less => {
                            Self::from_repeat_between(module, (min, max), a, next.clone())
                        }
                        Ordering::Equal => Self::from_repeat_count(module, min, a, next.clone()),
                        Ordering::Greater => {
                            panic!("incoherent repeat-between: min {} > max {}", min, max)
                        }
                    },
                    _ => {
                        // FIXME: if there is a cleaner way to address this case, attempt to apply it
                        unreachable!("inexact repeat-between bounds (not technically a problem but not what the combinator was designed for...");
                    }
                }
            }
            Format::RepeatUntilLast(_expr, _a) => {
                Self::accept() // FIXME
            }
            Format::RepeatUntilSeq(_expr, _a) => {
                Self::accept() // FIXME
            }
            Format::Maybe(_expr, a) => {
                let tree_some = Self::from_format(module, a, next.clone());
                let tree_none = Self::from_next(module, next);
                tree_some.union(tree_none)
            }
            Format::Peek(a) => {
                let tree = Self::from_next(module, next.clone());
                let peek = Self::from_format(module, a, Rc::new(Next::Empty));
                tree.peek(peek)
            }
            Format::PeekNot(a) => {
                let tree = Self::from_next(module, next.clone());
                let peek = Self::from_format(module, a, Rc::new(Next::Empty));
                tree.peek_not(peek)
            }
            Format::Slice(expr, f) => {
                let inside = Rc::new(Next::Cat(
                    MaybeTyped::Untyped(f.as_ref()),
                    Rc::new(Next::Empty),
                ));
                let bounds = expr.bounds();
                if let Some(n) = bounds.is_exact() {
                    Self::from_slice(module, n, inside, next)
                } else {
                    Self::from_slice(module, bounds.min, inside, Rc::new(Next::Empty))
                }
            }
            Format::Bits(_a) => {
                Self::accept() // FIXME
            }
            Format::WithRelativeOffset(expr, a) => {
                // REVIEW - this is a bit hackish but it is at least somewhat better than before
                let tree = Self::from_next(module, next.clone());
                let bounds = expr.bounds();
                match bounds.is_exact() {
                    None => tree, // if the lookahead is indeterminate, ignore it
                    Some(n) => {
                        let peek = match n {
                            0 => Self::from_format(module, a, Rc::new(Next::Empty)),
                            _ => Self::from_slice(
                                module,
                                n,
                                Rc::new(Next::Empty),
                                Rc::new(Next::Tuple(
                                    MaybeTyped::Untyped(std::slice::from_ref(a.as_ref())),
                                    next,
                                )),
                            ),
                        };
                        tree.peek(peek)
                    }
                }
            }
            Format::Map(f, _expr) => Self::from_format(module, f, next),
            Format::Where(f, _expr) => Self::from_format(module, f, next),
            Format::Pos => Self::from_next(module, next),
            Format::Compute(_expr) => Self::from_next(module, next),
            Format::Let(_name, _expr, f) => Self::from_format(module, f, next),
            Format::Match(_, branches) => {
                let mut tree = Self::reject();
                for (_, f) in branches {
                    tree = tree.union(Self::from_format(module, f, next.clone()));
                }
                tree
            }
            Format::Dynamic(_name, _expr, f) => Self::from_format(module, f, next),
            Format::Apply(_name) => Self::accept(),
            Format::LetFormat(f0, _name, f) => {
                let next0 = Rc::new(Next::Cat(MaybeTyped::Untyped(f), next));
                Self::from_format(module, f0, next0)
            }
        }
    }

    /// Constructs a [MatchTreeStep] that matches the various possible align-offset versions of `next`, for small enough `n`,
    /// and otherwise fudges the return value with a universal-acceptance.
    ///
    /// NOTE - currently 'small enough' just means that `n` is 0 (and illegal) or `1` (and irrefutable as an alignment modulus).
    ///
    /// # Panics
    ///
    /// Will panic if `n` happens to be `0`, as it is impossible to align modulo `0`.
    fn from_align(module: &'a FormatModule, next: Rc<Next<'a>>, n: usize) -> MatchTreeStep<'a> {
        match n {
            // FIXME - we might want to construct an auto-rejecting tree here, but this is perhaps less murky in terms of expected behavior
            0 => unreachable!("alignment modulus 0 has no valid possible interpretation"),
            1 => Self::from_next(module, next), // guaranteed to already be in alignment
            2.. => {
                // FIXME - this is still hackish but it is at least somewhat better than before
                // TODO - consider handling very small cases like 2..=4, with bespoke tree-unions over each potential distance from `next` we might skip over
                Self::accept()
            }
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
    /// Returns `Err(())` if a different index was already marked as accepting, and `Ok(())` otherwise.
    fn merge_accept(&mut self, index: usize) -> Result<(), ()> {
        match self.accept {
            None => {
                self.accept = Some(index);
                Ok(())
            }
            Some(i) if i == index => Ok(()),
            Some(_) => Err(()),
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
    ) -> Result<MatchTreeLevel<'a>, ()> {
        if step.accept {
            self.merge_accept(index)?;
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
    /// If the depth limit has been reached without a decisive choice of which index to accept, returns None.
    ///
    /// Otherwise, returns a `MatchTree` that is guaranteed to decide on a unique branch for
    /// all input within at most `depth` bytes of lookahead.
    fn grow(module: &'a FormatModule, nexts: LevelBranch<'a>, depth: usize) -> Option<MatchTree> {
        if let Some(tree) = Self::accepts(&nexts) {
            Some(tree)
        } else if depth > 0 {
            let mut tree = Self::reject();
            let mut tmp = Vec::from_iter(nexts);
            tmp.sort_by_key(|(ix, _)| *ix);
            for (i, next) in tmp.into_iter() {
                let subtree = MatchTreeStep::from_next(module, next);
                tree = tree.merge_step(i, subtree).ok()?;
            }
            let mut branches = Vec::new();
            for (bs, nexts) in tree.branches {
                let t = Self::grow(module, nexts, depth - 1)?;
                branches.push((bs, t));
            }
            Some(MatchTree {
                accept: tree.accept,
                branches,
            })
        } else {
            None
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
    fn build(module: &FormatModule, branches: &[Format], next: Rc<Next<'_>>) -> Option<MatchTree> {
        let mut nexts = HashSet::new();
        for (i, f) in branches.iter().enumerate() {
            nexts.insert((i, Rc::new(Next::Cat(MaybeTyped::Untyped(f), next.clone()))));
        }
        const MAX_DEPTH: usize = 80;
        MatchTreeLevel::grow(module, nexts, MAX_DEPTH)
    }
}

pub struct TypeScope<'a> {
    parent: Option<&'a TypeScope<'a>>,
    names: Vec<Label>,
    types: Vec<ValueKind>,
}

impl<'a> TypeScope<'a> {
    fn new() -> Self {
        let parent = None;
        let names = Vec::new();
        let types = Vec::new();
        TypeScope {
            parent,
            names,
            types,
        }
    }

    fn child(parent: &'a TypeScope<'a>) -> Self {
        let parent = Some(parent);
        let names = Vec::new();
        let types = Vec::new();
        TypeScope {
            parent,
            names,
            types,
        }
    }

    fn push(&mut self, name: Label, t: ValueType) {
        self.names.push(name);
        self.types.push(ValueKind::Value(t));
    }

    fn push_format(&mut self, name: Label, t: ValueType) {
        self.names.push(name);
        self.types.push(ValueKind::Format(t));
    }

    fn get_type_by_name(&self, name: &str) -> &ValueKind {
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
    use decoder::Value;

    use super::*;

    #[test]
    fn format_let_eval_precedence() {
        let fmt = Format::Let(
            Label::Borrowed("y"),
            Box::new(Expr::U8(10)),
            Box::new(Format::Let(
                Label::Borrowed("x"),
                Box::new(Expr::Var(Label::Borrowed("y"))),
                Box::new(Format::Record(vec![
                    (Label::Borrowed("y"), Format::Compute(Box::new(Expr::U8(5)))),
                    (
                        Label::Borrowed("z"),
                        Format::Compute(Box::new(Expr::Var(Label::Borrowed("x")))),
                    ),
                ])),
            )),
        );
        let mut module = FormatModule::new();
        let fref = module.define_format("test", fmt);
        let prog = super::decoder::Compiler::compile_program(&module, &fref.call()).unwrap();
        let buf = ReadCtxt::new(&[]);
        let (ret, _) = prog.run(buf).unwrap();
        let expected = Value::Record(vec![
            (Label::Borrowed("y"), Value::U8(5)),
            (Label::Borrowed("z"), Value::U8(10)),
        ]);
        assert_eq!(expected, ret);
    }
}
