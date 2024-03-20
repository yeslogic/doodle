use crate::byte_set::ByteSet;
use crate::codegen::{AtomType, LocalType, PrimType};
use crate::error::{ParseError, ParseResult};
use crate::read::ReadCtxt;
use crate::{Arith, FormatModule, IntRel, MatchTree, Next, TypeScope};
use crate::{IntoLabel, Label};
use anyhow::{anyhow, Result as AResult};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::codegen::typed_format::{GenType, TypedPattern};

use super::typed_format::{TypedDynFormat, TypedExpr, TypedFormat};
use super::{GTFormat, RustType};

mod __typed_value {
    use super::*;

    #[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
    #[serde(tag = "tag", content = "data")]
    pub enum TypedValue<TypeRep> {
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        Char(char),
        Tuple(TypeRep, Vec<TypedValue<TypeRep>>),
        Record(TypeRep, Vec<(Label, TypedValue<TypeRep>)>),
        Variant(TypeRep, Label, Box<TypedValue<TypeRep>>),
        Seq(TypeRep, Vec<TypedValue<TypeRep>>),
        Mapped(TypeRep, Box<TypedValue<TypeRep>>, Box<TypedValue<TypeRep>>),
        Branch(TypeRep, usize, Box<TypedValue<TypeRep>>),
    }

    impl TypedValue<GenType> {
        pub const UNIT: Self = TypedValue::Tuple(GenType::Inline(RustType::UNIT), Vec::new());
    }

    impl<TypeRep> TypedValue<TypeRep> {
        pub fn record<Name: IntoLabel>(
            t: TypeRep,
            fields: impl IntoIterator<Item = (Name, Self)>,
        ) -> Self {
            TypedValue::Record(
                t,
                fields
                    .into_iter()
                    .map(|(label, value)| (label.into(), value))
                    .collect(),
            )
        }

        // FIXME - we cannot use this as-is due to lack of type info
        pub fn variant(t: TypeRep, label: impl IntoLabel, value: impl Into<Box<Self>>) -> Self {
            TypedValue::Variant(t, label.into(), value.into())
        }

        pub fn record_proj(&self, label: &str) -> &Self
        where
            TypeRep: std::fmt::Debug,
        {
            match self {
                TypedValue::Record(_t, fields) => match fields.iter().find(|(l, _)| label == l) {
                    Some((_, v)) => v,
                    None => panic!("{label} not found in record"),
                },
                _ => panic!("expected record, found {self:?}"),
            }
        }

        pub fn tuple_proj(&self, index: usize) -> &Self {
            match self.coerce_mapped_value() {
                TypedValue::Tuple(_t, vs) => &vs[index],
                _ => panic!("expected tuple"),
            }
        }

        pub fn coerce_mapped_value(&self) -> &Self {
            match self {
                TypedValue::Mapped(_t, _orig, v) => v.coerce_mapped_value(),
                TypedValue::Branch(_t, _n, v) => v.coerce_mapped_value(),
                v => v,
            }
        }

        fn unwrap_usize(self) -> usize {
            match self {
                TypedValue::U8(n) => usize::from(n),
                TypedValue::U16(n) => usize::from(n),
                TypedValue::U32(n) => usize::try_from(n).unwrap(),
                TypedValue::U64(n) => usize::try_from(n).unwrap(),
                _ => panic!("value is not a number"),
            }
        }

        fn unwrap_tuple(self) -> Vec<Self> {
            match self {
                TypedValue::Tuple(_t, values) => values,
                _ => panic!("value is not a tuple"),
            }
        }

        fn unwrap_bool(self) -> bool {
            match self {
                TypedValue::Bool(b) => b,
                _ => panic!("value is not a bool"),
            }
        }

        #[allow(dead_code)]
        fn unwrap_char(self) -> char {
            match self {
                TypedValue::Char(c) => c,
                _ => panic!("value is not a char"),
            }
        }
    }

    impl TypedValue<GenType> {
        /// Returns `true` if the pattern successfully matches the value, pushing
        /// any values bound by the pattern onto the scope
        pub fn matches<'a>(
            &self,
            scope: &'a TScope<'a, GenType>,
            pattern: &TypedPattern<GenType>,
        ) -> Option<TMultiScope<'a, GenType>> {
            let mut pattern_scope = TMultiScope::new(scope);
            self.coerce_mapped_value()
                .matches_inner(&mut pattern_scope, pattern)
                .then_some(pattern_scope)
        }

        fn matches_type(&self, t: &GenType) -> bool {
            match self {
                TypedValue::Bool(_) => {
                    matches!(
                        t,
                        GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::Bool)))
                    )
                }
                TypedValue::U8(_) => {
                    matches!(
                        t,
                        GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U8)))
                    )
                }
                TypedValue::U16(_) => {
                    matches!(
                        t,
                        GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U16)))
                    )
                }
                TypedValue::U32(_) => {
                    matches!(
                        t,
                        GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U32)))
                    )
                }
                TypedValue::U64(_) => {
                    matches!(
                        t,
                        GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::U64)))
                    )
                }
                TypedValue::Char(_) => {
                    matches!(
                        t,
                        GenType::Inline(RustType::Atom(AtomType::Prim(PrimType::Char)))
                    )
                }
                TypedValue::Tuple(t1, _elts) => {
                    if _elts.is_empty() {
                        match t {
                            GenType::Inline(RustType::UNIT) => true,
                            GenType::Inline(RustType::AnonTuple(v)) if v.is_empty() => true,
                            _ => false,
                        }
                    } else {
                        match (t, t1) {
                            (
                                GenType::Inline(RustType::AnonTuple(v)),
                                GenType::Inline(RustType::AnonTuple(v1)),
                            ) => {
                                for (e, e1) in Iterator::zip(v.iter(), v1.iter()) {
                                    if e != e1 {
                                        return false;
                                    }
                                }
                                true
                            }
                            _ => false,
                        }
                    }
                }
                TypedValue::Record(t1, _flds) => {
                    assert!(!_flds.is_empty(), "empty record found: {self:?}");
                    match (t, t1) {
                        (
                            GenType::Def((ix0, tn0), _)
                            | GenType::Inline(RustType::Atom(AtomType::TypeRef(
                                LocalType::LocalDef(ix0, tn0),
                            ))),
                            GenType::Def((ix1, tn1), _)
                            | GenType::Inline(RustType::Atom(AtomType::TypeRef(
                                LocalType::LocalDef(ix1, tn1),
                            ))),
                        ) => ix0 == ix1 && tn0.as_ref() == tn1.as_ref(),
                        _ => unreachable!(),
                    }
                }
                TypedValue::Variant(_, _, _) => todo!(),
                TypedValue::Seq(_, _) => todo!(),
                TypedValue::Mapped(_, _, _) => todo!(),
                TypedValue::Branch(_, _, _) => todo!(),
            }
        }

        fn matches_inner(
            &self,
            scope: &mut TMultiScope<'_, GenType>,
            pattern: &TypedPattern<GenType>,
        ) -> bool {
            match (pattern, self) {
                (TypedPattern::Binding(_t, name), head) => {
                    assert!(self.matches_type(_t));
                    scope.push(name.clone(), head.clone());
                    true
                }
                (TypedPattern::Wildcard(_tp), _) => self.matches_type(_tp),
                (TypedPattern::Bool(b0), TypedValue::Bool(b1)) => b0 == b1,
                (TypedPattern::U8(i0), TypedValue::U8(i1)) => i0 == i1,
                (TypedPattern::U16(i0), TypedValue::U16(i1)) => i0 == i1,
                (TypedPattern::U32(i0), TypedValue::U32(i1)) => i0 == i1,
                (TypedPattern::U64(i0), TypedValue::U64(i1)) => i0 == i1,
                (TypedPattern::Char(c0), TypedValue::Char(c1)) => c0 == c1,
                (TypedPattern::Tuple(_t0, ps), TypedValue::Tuple(_t1, vs))
                | (TypedPattern::Seq(_t0, ps), TypedValue::Seq(_t1, vs))
                    if ps.len() == vs.len() =>
                {
                    for (p, v) in Iterator::zip(ps.iter(), vs.iter()) {
                        if !v.matches_inner(scope, p) {
                            return false;
                        }
                    }
                    true
                }
                (TypedPattern::Variant(t0, label0, p), TypedValue::Variant(t1, label1, v))
                    if label0 == label1 =>
                {
                    self.matches_type(t0) && v.matches_inner(scope, p)
                }
                _ => false,
            }
        }
    }

    impl TypedExpr<GenType> {
        pub fn eval<'a>(&'a self, scope: &'a TScope<'a, GenType>) -> Cow<'a, TypedValue<GenType>> {
            match self {
                TypedExpr::Var(t, name) => Cow::Borrowed(scope.get_value_by_name(name)),
                TypedExpr::Bool(b) => Cow::Owned(TypedValue::Bool(*b)),
                TypedExpr::U8(i) => Cow::Owned(TypedValue::U8(*i)),
                TypedExpr::U16(i) => Cow::Owned(TypedValue::U16(*i)),
                TypedExpr::U32(i) => Cow::Owned(TypedValue::U32(*i)),
                TypedExpr::U64(i) => Cow::Owned(TypedValue::U64(*i)),
                TypedExpr::Tuple(t, exprs) => Cow::Owned(TypedValue::Tuple(
                    t.clone(),
                    exprs.iter().map(|expr| expr.eval_value(scope)).collect(),
                )),
                TypedExpr::TupleProj(_t, head, index) => match head.eval(scope) {
                    Cow::Owned(v) => Cow::Owned(v.coerce_mapped_value().tuple_proj(*index).clone()),
                    Cow::Borrowed(v) => Cow::Borrowed(v.coerce_mapped_value().tuple_proj(*index)),
                },
                TypedExpr::Record(t, fields) => Cow::Owned(TypedValue::record(
                    t.clone(),
                    fields
                        .iter()
                        .map(|(label, expr)| (label.clone(), expr.eval_value(scope))),
                )),
                TypedExpr::RecordProj(_t, head, label) => match head.eval(scope) {
                    Cow::Owned(v) => Cow::Owned(v.coerce_mapped_value().record_proj(label).clone()),
                    Cow::Borrowed(v) => Cow::Borrowed(v.coerce_mapped_value().record_proj(label)),
                },
                TypedExpr::Variant(t, label, expr) => Cow::Owned(TypedValue::variant(
                    t.clone(),
                    label.clone(),
                    expr.eval_value(scope),
                )),
                TypedExpr::Seq(t, exprs) => Cow::Owned(TypedValue::Seq(
                    t.clone(),
                    exprs.iter().map(|expr| expr.eval_value(scope)).collect(),
                )),
                TypedExpr::Match(t, head, branches) => {
                    let head = head.eval(scope);
                    for (pattern, expr) in branches {
                        if let Some(pattern_scope) = head.matches(scope, pattern) {
                            let value = expr.eval_value(&TScope::Multi(&pattern_scope));
                            return Cow::Owned(value);
                        }
                    }
                    panic!("non-exhaustive patterns");
                }
                TypedExpr::Lambda(..) => panic!("cannot eval lambda"),

                TypedExpr::IntRel(_t, IntRel::Eq, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::Bool(x == y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::Bool(x == y),
                        (TypedValue::U32(x), TypedValue::U32(y)) => TypedValue::Bool(x == y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::Bool(x == y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::IntRel(_t, IntRel::Ne, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::Bool(x != y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::Bool(x != y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::Bool(x != y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::IntRel(_t, IntRel::Lt, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::Bool(x < y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::Bool(x < y),
                        (TypedValue::U32(x), TypedValue::U32(y)) => TypedValue::Bool(x < y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::Bool(x < y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::IntRel(_t, IntRel::Gt, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::Bool(x > y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::Bool(x > y),
                        (TypedValue::U32(x), TypedValue::U32(y)) => TypedValue::Bool(x > y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::Bool(x > y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::IntRel(_t, IntRel::Lte, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::Bool(x <= y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::Bool(x <= y),
                        (TypedValue::U32(x), TypedValue::U32(y)) => TypedValue::Bool(x <= y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::Bool(x <= y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::IntRel(_t, IntRel::Gte, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::Bool(x >= y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::Bool(x >= y),
                        (TypedValue::U32(x), TypedValue::U32(y)) => TypedValue::Bool(x >= y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::Bool(x >= y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::Add, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => {
                            TypedValue::U8(u8::checked_add(x, y).unwrap())
                        }
                        (TypedValue::U16(x), TypedValue::U16(y)) => {
                            TypedValue::U16(u16::checked_add(x, y).unwrap())
                        }
                        (TypedValue::U32(x), TypedValue::U32(y)) => {
                            TypedValue::U32(u32::checked_add(x, y).unwrap())
                        }
                        (TypedValue::U64(x), TypedValue::U64(y)) => {
                            TypedValue::U64(u64::checked_add(x, y).unwrap())
                        }
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::Sub, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => {
                            TypedValue::U8(u8::checked_sub(x, y).unwrap())
                        }
                        (TypedValue::U16(x), TypedValue::U16(y)) => {
                            TypedValue::U16(u16::checked_sub(x, y).unwrap())
                        }
                        (TypedValue::U32(x), TypedValue::U32(y)) => {
                            TypedValue::U32(u32::checked_sub(x, y).unwrap())
                        }
                        (TypedValue::U64(x), TypedValue::U64(y)) => {
                            TypedValue::U64(u64::checked_sub(x, y).unwrap())
                        }
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::Mul, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => {
                            TypedValue::U8(u8::checked_mul(x, y).unwrap())
                        }
                        (TypedValue::U16(x), TypedValue::U16(y)) => {
                            TypedValue::U16(u16::checked_mul(x, y).unwrap())
                        }
                        (TypedValue::U32(x), TypedValue::U32(y)) => {
                            TypedValue::U32(u32::checked_mul(x, y).unwrap())
                        }
                        (TypedValue::U64(x), TypedValue::U64(y)) => {
                            TypedValue::U64(u64::checked_mul(x, y).unwrap())
                        }
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::Div, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => {
                            TypedValue::U8(u8::checked_div(x, y).unwrap())
                        }
                        (TypedValue::U16(x), TypedValue::U16(y)) => {
                            TypedValue::U16(u16::checked_div(x, y).unwrap())
                        }
                        (TypedValue::U32(x), TypedValue::U32(y)) => {
                            TypedValue::U32(u32::checked_div(x, y).unwrap())
                        }
                        (TypedValue::U64(x), TypedValue::U64(y)) => {
                            TypedValue::U64(u64::checked_div(x, y).unwrap())
                        }
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::Rem, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => {
                            TypedValue::U8(u8::checked_rem(x, y).unwrap())
                        }
                        (TypedValue::U16(x), TypedValue::U16(y)) => {
                            TypedValue::U16(u16::checked_rem(x, y).unwrap())
                        }
                        (TypedValue::U32(x), TypedValue::U32(y)) => {
                            TypedValue::U32(u32::checked_rem(x, y).unwrap())
                        }
                        (TypedValue::U64(x), TypedValue::U64(y)) => {
                            TypedValue::U64(u64::checked_rem(x, y).unwrap())
                        }
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::BitAnd, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::U8(x & y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::U16(x & y),
                        (TypedValue::U32(x), TypedValue::U32(y)) => TypedValue::U32(x & y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::U64(x & y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::BitOr, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => TypedValue::U8(x | y),
                        (TypedValue::U16(x), TypedValue::U16(y)) => TypedValue::U16(x | y),
                        (TypedValue::U32(x), TypedValue::U32(y)) => TypedValue::U32(x | y),
                        (TypedValue::U64(x), TypedValue::U64(y)) => TypedValue::U64(x | y),
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::Shl, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => {
                            TypedValue::U8(u8::checked_shl(x, u32::from(y)).unwrap())
                        }
                        (TypedValue::U16(x), TypedValue::U16(y)) => {
                            TypedValue::U16(u16::checked_shl(x, u32::from(y)).unwrap())
                        }
                        (TypedValue::U32(x), TypedValue::U32(y)) =>
                            TypedValue::U32(u32::checked_shl(x, y).unwrap()),
                        (TypedValue::U64(x), _y) =>
                            match _y {
                                // FIXME - we handle u32 as it is the rust-expected value and u64 since it is homogenous with the lhs operand. are other cases sensible as well?
                                TypedValue::U32(y) =>
                                    TypedValue::U64(u64::checked_shl(x, y).unwrap()),
                                TypedValue::U64(y) => {
                                    if y <= 0xffff_ffff {
                                        TypedValue::U64(
                                            u64::checked_shl(x, u32::try_from(y).unwrap()).unwrap()
                                        )
                                    } else {
                                        panic!("Shl rhs operand too large: {y:#0x}");
                                    }
                                }
                                other =>
                                    unreachable!(
                                        "Unexpected ValueType for rhs operand of U64-typed Shl expression: {other:?}"
                                    ),
                            }
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }
                TypedExpr::Arith(_t, Arith::Shr, x, y) => {
                    Cow::Owned(match (x.eval_value(scope), y.eval_value(scope)) {
                        (TypedValue::U8(x), TypedValue::U8(y)) => {
                            TypedValue::U8(u8::checked_shr(x, u32::from(y)).unwrap())
                        }
                        (TypedValue::U16(x), TypedValue::U16(y)) => {
                            TypedValue::U16(u16::checked_shr(x, u32::from(y)).unwrap())
                        }
                        (TypedValue::U32(x), TypedValue::U32(y)) =>
                            TypedValue::U32(u32::checked_shr(x, y).unwrap()),
                        (TypedValue::U64(x), _y) =>
                            match _y {
                                // FIXME - we handle u32 as it is the rust-expected value and u64 since it is homogenous with the lhs operand. are other cases sensible as well?
                                TypedValue::U32(y) =>
                                    TypedValue::U64(u64::checked_shr(x, y).unwrap()),
                                TypedValue::U64(y) => {
                                    if y <= 0xffff_ffff {
                                        TypedValue::U64(
                                            u64::checked_shr(x, u32::try_from(y).unwrap()).unwrap()
                                        )
                                    } else {
                                        panic!("Shr rhs operand too large: {y:#0x}");
                                    }
                                }
                                other =>
                                    unreachable!(
                                        "Unexpected ValueType for rhs operand of U64-typed Shr expression: {other:?}"
                                    ),
                            }
                        (x, y) => panic!("mismatched operands {x:?}, {y:?}"),
                    })
                }

                TypedExpr::AsU8(x) => Cow::Owned(match x.eval_value(scope) {
                    TypedValue::U8(x) => TypedValue::U8(x),
                    TypedValue::U16(x) if x < 0xff => TypedValue::U8(x as u8),
                    TypedValue::U32(x) if x < 0xff => TypedValue::U8(x as u8),
                    TypedValue::U64(x) if x < 0xff => TypedValue::U8(x as u8),
                    x => panic!("cannot convert {x:?} to U8"),
                }),
                TypedExpr::AsU16(x) => Cow::Owned(match x.eval_value(scope) {
                    TypedValue::U8(x) => TypedValue::U16(u16::from(x)),
                    TypedValue::U16(x) => TypedValue::U16(x),
                    TypedValue::U32(x) if x < 0xffff => TypedValue::U16(x as u16),
                    TypedValue::U64(x) if x < 0xffff => TypedValue::U16(x as u16),
                    x => panic!("cannot convert {x:?} to U16"),
                }),
                TypedExpr::AsU32(x) => Cow::Owned(match x.eval_value(scope) {
                    TypedValue::U8(x) => TypedValue::U32(u32::from(x)),
                    TypedValue::U16(x) => TypedValue::U32(u32::from(x)),
                    TypedValue::U32(x) => TypedValue::U32(x),
                    TypedValue::U64(x) if x < 0xffff_ffff => TypedValue::U32(x as u32),
                    x => panic!("cannot convert {x:?} to U32"),
                }),
                TypedExpr::AsU64(x) => Cow::Owned(match x.eval_value(scope) {
                    TypedValue::U8(x) => TypedValue::U64(u64::from(x)),
                    TypedValue::U16(x) => TypedValue::U64(u64::from(x)),
                    TypedValue::U32(x) => TypedValue::U64(u64::from(x)),
                    TypedValue::U64(x) => TypedValue::U64(x),
                    x => panic!("cannot convert {x:?} to U64"),
                }),

                TypedExpr::U16Be(bytes) => {
                    match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                        [TypedValue::U8(hi), TypedValue::U8(lo)] => {
                            Cow::Owned(TypedValue::U16(u16::from_be_bytes([*hi, *lo])))
                        }
                        _ => panic!("U16Be: expected (U8, U8)"),
                    }
                }
                TypedExpr::U16Le(bytes) => {
                    match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                        [TypedValue::U8(lo), TypedValue::U8(hi)] => {
                            Cow::Owned(TypedValue::U16(u16::from_le_bytes([*lo, *hi])))
                        }
                        _ => panic!("U16Le: expected (U8, U8)"),
                    }
                }
                TypedExpr::U32Be(bytes) => {
                    match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                        [TypedValue::U8(a), TypedValue::U8(b), TypedValue::U8(c), TypedValue::U8(d)] => {
                            Cow::Owned(TypedValue::U32(u32::from_be_bytes([*a, *b, *c, *d])))
                        }
                        _ => panic!("U32Be: expected (U8, U8, U8, U8)"),
                    }
                }
                TypedExpr::U32Le(bytes) => {
                    match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                        [TypedValue::U8(a), TypedValue::U8(b), TypedValue::U8(c), TypedValue::U8(d)] => {
                            Cow::Owned(TypedValue::U32(u32::from_le_bytes([*a, *b, *c, *d])))
                        }
                        _ => panic!("U32Le: expected (U8, U8, U8, U8)"),
                    }
                }
                TypedExpr::U64Be(bytes) => {
                    match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                        [TypedValue::U8(a), TypedValue::U8(b), TypedValue::U8(c), TypedValue::U8(d), TypedValue::U8(e), TypedValue::U8(f), TypedValue::U8(g), TypedValue::U8(h)] => {
                            Cow::Owned(TypedValue::U64(u64::from_be_bytes([
                                *a, *b, *c, *d, *e, *f, *g, *h,
                            ])))
                        }
                        _ => panic!("U64Be: expected (U8, U8, U8, U8, U8, U8, U8, U8)"),
                    }
                }
                TypedExpr::U64Le(bytes) => {
                    match bytes.eval_value(scope).unwrap_tuple().as_slice() {
                        [TypedValue::U8(a), TypedValue::U8(b), TypedValue::U8(c), TypedValue::U8(d), TypedValue::U8(e), TypedValue::U8(f), TypedValue::U8(g), TypedValue::U8(h)] => {
                            Cow::Owned(TypedValue::U64(u64::from_le_bytes([
                                *a, *b, *c, *d, *e, *f, *g, *h,
                            ])))
                        }
                        _ => panic!("U64Le: expected (U8, U8, U8, U8, U8, U8, U8, U8)"),
                    }
                }
                TypedExpr::AsChar(bytes) => Cow::Owned(match bytes.eval_value(scope) {
                    TypedValue::U8(x) => TypedValue::Char(char::from(x)),
                    TypedValue::U16(x) => TypedValue::Char(
                        char::from_u32(x as u32).unwrap_or(char::REPLACEMENT_CHARACTER),
                    ),
                    TypedValue::U32(x) => {
                        TypedValue::Char(char::from_u32(x).unwrap_or(char::REPLACEMENT_CHARACTER))
                    }
                    TypedValue::U64(x) if x <= 0xffff_ffff => TypedValue::Char(
                        char::from_u32(x as u32).unwrap_or(char::REPLACEMENT_CHARACTER),
                    ),
                    _ => panic!("AsChar: expected U8, U16, U32, or U64"),
                }),
                TypedExpr::SeqLength(seq) => match seq.eval(scope).coerce_mapped_value() {
                    TypedValue::Seq(_t, values) => {
                        let len = values.len();
                        Cow::Owned(TypedValue::U32(len as u32))
                    }
                    _ => panic!("SeqLength: expected Seq"),
                },
                TypedExpr::SubSeq(_t, seq, start, length) => {
                    match seq.eval(scope).coerce_mapped_value() {
                        TypedValue::Seq(t, values) => {
                            let start = start.eval_value(scope).unwrap_usize();
                            let length = length.eval_value(scope).unwrap_usize();
                            let values = &values[start..];
                            let values = &values[..length];
                            Cow::Owned(TypedValue::Seq(t.clone(), values.to_vec()))
                        }
                        _ => panic!("SubSeq: expected Seq"),
                    }
                }
                TypedExpr::FlatMap(t, expr, seq) => match seq.eval(scope).coerce_mapped_value() {
                    TypedValue::Seq(_, values) => {
                        let mut vs = Vec::new();
                        for v in values {
                            if let TypedValue::Seq(t0, vn) = expr.eval_lambda(scope, v) {
                                vs.extend(vn);
                            } else {
                                panic!("FlatMap: expected Seq");
                            }
                        }
                        Cow::Owned(TypedValue::Seq(t.clone(), vs))
                    }
                    _ => panic!("FlatMap: expected Seq"),
                },
                TypedExpr::FlatMapAccum(t, expr, accum, _accum_type, seq) => {
                    match seq.eval_value(scope) {
                        TypedValue::Seq(seq_t, values) => {
                            let mut accum = accum.eval_value(scope);
                            let mut vs = Vec::new();
                            for v in values {
                                let ret = expr.eval_lambda(
                                    scope,
                                    &TypedValue::Tuple(
                                        GenType::Inline(RustType::AnonTuple(vec![])),
                                        vec![accum, v],
                                    ),
                                );
                                accum = match ret.unwrap_tuple().as_mut_slice() {
                                    [accum, TypedValue::Seq(_, vn)] => {
                                        vs.extend_from_slice(vn);
                                        accum.clone()
                                    }
                                    _ => panic!("FlatMapAccum: expected two values"),
                                };
                            }
                            let ret_t = match accum {
                                TypedValue::Seq(trep, _) => trep,
                                other => unreachable!("unexpected type: {:?}", other),
                            };
                            Cow::Owned(TypedValue::Seq(ret_t, vs))
                        }
                        _ => panic!("FlatMapAccum: expected Seq"),
                    }
                }
                TypedExpr::Dup(t, count, expr) => {
                    let count = count.eval_value(scope).unwrap_usize();
                    let v = expr.eval_value(scope);
                    let mut vs = Vec::new();
                    for _ in 0..count {
                        vs.push(v.clone());
                    }
                    Cow::Owned(TypedValue::Seq(t.clone(), vs))
                }
                TypedExpr::Inflate(_, seq) => match seq.eval(scope).coerce_mapped_value() {
                    TypedValue::Seq(t, values) => {
                        let vs = inflate(values);
                        Cow::Owned(TypedValue::Seq(
                            GenType::Inline(RustType::vec_of(RustType::from(PrimType::U8))),
                            vs,
                        ))
                    }
                    _ => panic!("Inflate: expected Seq"),
                },
            }
        }

        pub fn eval_value<'a>(&self, scope: &'a TScope<'a, GenType>) -> TypedValue<GenType> {
            self.eval(scope).coerce_mapped_value().clone()
        }

        fn eval_lambda<'a>(
            &self,
            scope: &'a TScope<'a, GenType>,
            arg: &TypedValue<GenType>,
        ) -> TypedValue<GenType> {
            match self {
                TypedExpr::Lambda(_, name, expr) => {
                    let child_scope = TSingleScope::new(scope, name, arg);
                    expr.eval_value(&TScope::Single(child_scope))
                }
                _ => panic!("expected Lambda"),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) enum TypedScopeEntry<TypeRep> {
        Value(TypedValue<TypeRep>),
        Decoder(TypedDecoder<TypeRep>),
    }

    pub enum TScope<'a, TypeRep> {
        Empty,
        Multi(&'a TMultiScope<'a, TypeRep>),
        Single(TSingleScope<'a, TypeRep>),
        Decoder(TypedDecoderScope<'a, TypeRep>),
    }

    pub struct TMultiScope<'a, TypeRep> {
        parent: &'a TScope<'a, TypeRep>,
        entries: Vec<(Label, TypedValue<TypeRep>)>,
    }

    pub struct TSingleScope<'a, TypeRep> {
        parent: &'a TScope<'a, TypeRep>,
        name: &'a str,
        value: &'a TypedValue<TypeRep>,
    }

    pub struct TypedDecoderScope<'a, TypeRep> {
        parent: &'a TScope<'a, TypeRep>,
        name: &'a str,
        decoder: TypedDecoder<TypeRep>,
    }

    impl<'a, TypeRep> TScope<'a, TypeRep> {
        fn get_value_by_name(&self, name: &str) -> &TypedValue<TypeRep> {
            match self {
                TScope::Empty => panic!("value not found: {name}"),
                TScope::Multi(multi) => multi.get_value_by_name(name),
                TScope::Single(single) => single.get_value_by_name(name),
                TScope::Decoder(decoder) => decoder.parent.get_value_by_name(name),
            }
        }

        fn get_decoder_by_name(&self, name: &str) -> &TypedDecoder<TypeRep> {
            match self {
                TScope::Empty => panic!("decoder not found: {name}"),
                TScope::Multi(multi) => multi.parent.get_decoder_by_name(name),
                TScope::Single(single) => single.parent.get_decoder_by_name(name),
                TScope::Decoder(decoder) => decoder.get_decoder_by_name(name),
            }
        }
    }

    impl<'a> TScope<'a, GenType> {
        pub fn get_bindings(
            &self,
            bindings: &mut Vec<(Label, Box<dyn std::fmt::Debug + 'static>)>,
        ) {
            match self {
                TScope::Empty => {}
                TScope::Multi(multi) => multi.get_bindings(bindings),
                TScope::Single(single) => single.get_bindings(bindings),
                TScope::Decoder(decoder) => decoder.get_bindings(bindings),
            }
        }
    }

    impl<'a, TypeRep> TMultiScope<'a, TypeRep> {
        fn new(parent: &'a TScope<'a, TypeRep>) -> TMultiScope<'a, TypeRep> {
            let entries = Vec::new();
            TMultiScope { parent, entries }
        }

        pub fn with_capacity(
            parent: &'a TScope<'a, TypeRep>,
            capacity: usize,
        ) -> TMultiScope<'a, TypeRep> {
            let entries = Vec::with_capacity(capacity);
            TMultiScope { parent, entries }
        }

        pub fn into_record(self, trep: TypeRep) -> TypedValue<TypeRep> {
            TypedValue::Record(trep, self.entries)
        }

        pub fn push(&mut self, name: Label, v: TypedValue<TypeRep>) {
            self.entries.push((name, v));
        }

        fn get_value_by_name(&self, name: &str) -> &TypedValue<TypeRep> {
            for (n, v) in self.entries.iter().rev() {
                if n == name {
                    return v;
                }
            }
            self.parent.get_value_by_name(name)
        }
    }

    impl<'a> TMultiScope<'a, GenType> {
        fn get_bindings(&self, bindings: &mut Vec<(Label, Box<dyn std::fmt::Debug + 'static>)>) {
            for (name, value) in self.entries.iter().rev() {
                bindings.push((
                    name.clone(),
                    Box::new(TypedScopeEntry::<GenType>::Value(value.clone())),
                ));
            }
            self.parent.get_bindings(bindings);
        }
    }

    impl<'a, TypeRep> TSingleScope<'a, TypeRep> {
        pub fn new(
            parent: &'a TScope<'a, TypeRep>,
            name: &'a str,
            value: &'a TypedValue<TypeRep>,
        ) -> TSingleScope<'a, TypeRep> {
            TSingleScope {
                parent,
                name,
                value,
            }
        }

        fn get_value_by_name(&self, name: &str) -> &TypedValue<TypeRep> {
            if self.name == name {
                self.value
            } else {
                self.parent.get_value_by_name(name)
            }
        }
    }

    impl<'a> TSingleScope<'a, GenType> {
        fn get_bindings(&self, bindings: &mut Vec<(Label, Box<dyn std::fmt::Debug + 'static>)>) {
            bindings.push((
                self.name.to_string().into(),
                Box::new(TypedScopeEntry::Value(self.value.clone())),
            ));
            self.parent.get_bindings(bindings);
        }
    }

    impl<'a, TypeRep> TypedDecoderScope<'a, TypeRep> {
        fn new(
            parent: &'a TScope<'a, TypeRep>,
            name: &'a str,
            decoder: TypedDecoder<TypeRep>,
        ) -> TypedDecoderScope<'a, TypeRep> {
            TypedDecoderScope {
                parent,
                name,
                decoder,
            }
        }

        fn get_decoder_by_name(&self, name: &str) -> &TypedDecoder<TypeRep> {
            if self.name == name {
                &self.decoder
            } else {
                self.parent.get_decoder_by_name(name)
            }
        }
    }

    impl<'a> TypedDecoderScope<'a, GenType> {
        fn get_bindings(&self, bindings: &mut Vec<(Label, Box<dyn std::fmt::Debug + 'static>)>) {
            bindings.push((
                self.name.to_string().into(),
                Box::new(TypedScopeEntry::Decoder(self.decoder.clone())),
            ));
            self.parent.get_bindings(bindings);
        }
    }

    impl TypedDecoder<GenType> {
        // pub fn parse<'input>(
        //     &self,
        //     program: &Program<GenType>,
        //     scope: &TScope<'_, GenType>,
        //     input: ReadCtxt<'input>
        // ) -> ParseResult<(TypedValue<GenType>, ReadCtxt<'input>)> {
        //     match self {
        //         TypedDecoder::Call(gt, n, es) => {
        //             let mut new_scope = TMultiScope::with_capacity(&TScope::Empty, es.len());
        //             for (name, e) in es {
        //                 let v = e.eval_value(scope);
        //                 new_scope.push(name.clone(), v);
        //             }
        //             program.decoders[*n].0.parse(program, &TScope::Multi(&new_scope), input)
        //         }
        //         TypedDecoder::Fail => Err(ParseError::fail(scope, input)),
        //         TypedDecoder::EndOfInput =>
        //             match input.read_byte() {
        //                 None => Ok((TypedValue::UNIT, input)),
        //                 Some((b, _)) => Err(ParseError::trailing(b, input.offset)),
        //             }
        //         TypedDecoder::Align(n) => {
        //             let skip = (n - (input.offset % n)) % n;
        //             let (_, input) = input
        //                 .split_at(skip)
        //                 .ok_or(ParseError::overrun(skip, input.offset))?;
        //             Ok((TypedValue::UNIT, input))
        //         }
        //         TypedDecoder::Byte(bs) => {
        //             let (b, input) = input.read_byte().ok_or(ParseError::overbyte(input.offset))?;
        //             if bs.contains(b) {
        //                 Ok((TypedValue::U8(b), input))
        //             } else {
        //                 Err(ParseError::unexpected(b, *bs, input.offset))
        //             }
        //         }
        //         TypedDecoder::Variant(gt, label, d) => {
        //             let (v, input) = d.parse(program, scope, input)?;
        //             Ok((TypedValue::Variant(gt.clone(), label.clone(), Box::new(v)), input))
        //         }
        //         TypedDecoder::Branch(gt, tree, branches) => {
        //             let index = tree.matches(input).ok_or(ParseError::NoValidBranch {
        //                 offset: input.offset,
        //             })?;
        //             let d = &branches[index];
        //             let (v, input) = d.parse(program, scope, input)?;
        //             Ok((TypedValue::Branch(gt.clone(), index, Box::new(v)), input))
        //         }
        //         TypedDecoder::Parallel(gt, branches) => {
        //             for (index, d) in branches.iter().enumerate() {
        //                 let res = d.parse(program, scope, input);
        //                 if let Ok((v, input)) = res {
        //                     return Ok((TypedValue::Branch(gt.clone(), index, Box::new(v)), input));
        //                 }
        //             }
        //             Err(ParseError::fail_typed(scope, input))
        //         }
        //         TypedDecoder::Tuple(gt, fields) => {
        //             let mut input = input;
        //             let mut v = Vec::with_capacity(fields.len());
        //             for f in fields {
        //                 let (vf, next_input) = f.parse(program, scope, input)?;
        //                 input = next_input;
        //                 v.push(vf.clone());
        //             }
        //             Ok((TypedValue::Tuple(gt.clone(), v), input))
        //         }
        //         TypedDecoder::Record(gt, fields) => {
        //             let mut input = input;
        //             let mut record_scope = TMultiScope::with_capacity(scope, fields.len());
        //             for (name, f) in fields {
        //                 let (vf, next_input) = f.parse(
        //                     program,
        //                     &TScope::Multi(&record_scope),
        //                     input
        //                 )?;
        //                 record_scope.push(name.clone(), vf);
        //                 input = next_input;
        //             }
        //             Ok((record_scope.into_record(gt.clone()), input))
        //         }
        //         TypedDecoder::While(gt, tree, a) => {
        //             let mut input = input;
        //             let mut v = Vec::new();
        //             while
        //                 tree.matches(input).ok_or(ParseError::NoValidBranch {
        //                     offset: input.offset,
        //                 })? == 0
        //             {
        //                 let (va, next_input) = a.parse(program, scope, input)?;
        //                 input = next_input;
        //                 v.push(va);
        //             }
        //             Ok((TypedValue::Seq(gt.clone(), v), input))
        //         }
        //         TypedDecoder::Until(gt, tree, a) => {
        //             let mut input = input;
        //             let mut v = Vec::new();
        //             loop {
        //                 let (va, next_input) = a.parse(program, scope, input)?;
        //                 input = next_input;
        //                 v.push(va);
        //                 if
        //                     tree.matches(input).ok_or(ParseError::NoValidBranch {
        //                         offset: input.offset,
        //                     })? == 0
        //                 {
        //                     break;
        //                 }
        //             }
        //             Ok((TypedValue::Seq(gt.clone(), v), input))
        //         }
        //         TypedDecoder::RepeatCount(gt, expr, a) => {
        //             let mut input = input;
        //             let count = expr.eval_value(scope).unwrap_usize();
        //             let mut v = Vec::with_capacity(count);
        //             for _ in 0..count {
        //                 let (va, next_input) = a.parse(program, scope, input)?;
        //                 input = next_input;
        //                 v.push(va);
        //             }
        //             Ok((TypedValue::Seq(gt.clone(), v), input))
        //         }
        //         TypedDecoder::RepeatUntilLast(gt, expr, a) => {
        //             let mut input = input;
        //             let mut v = Vec::new();
        //             loop {
        //                 let (va, next_input) = a.parse(program, scope, input)?;
        //                 input = next_input;
        //                 let done = expr.eval_lambda(scope, &va).unwrap_bool();
        //                 v.push(va);
        //                 if done {
        //                     break;
        //                 }
        //             }
        //             Ok((TypedValue::Seq(gt.clone(), v), input))
        //         }
        //         TypedDecoder::RepeatUntilSeq(gt, expr, a) => {
        //             let mut input = input;
        //             let mut v = Vec::new();
        //             loop {
        //                 let (va, next_input) = a.parse(program, scope, input)?;
        //                 input = next_input;
        //                 v.push(va);
        //                 let vs = TypedValue::Seq(gt.clone(), v);
        //                 let done = expr.eval_lambda(scope, &vs).unwrap_bool();
        //                 v = match vs {
        //                     TypedValue::Seq(_gt, v) => v,
        //                     _ => unreachable!(),
        //                 };
        //                 if done {
        //                     break;
        //                 }
        //             }
        //             Ok((TypedValue::Seq(gt.clone(), v), input))
        //         }
        //         TypedDecoder::Peek(_gt, a) => {
        //             let (v, _next_input) = a.parse(program, scope, input)?;
        //             Ok((v, input))
        //         }
        //         TypedDecoder::PeekNot(_gt, a) => {
        //             if a.parse(program, scope, input).is_ok() {
        //                 Err(ParseError::fail_typed(scope, input))
        //             } else {
        //                 Ok((TypedValue::Tuple(GenType::Inline(RustType::UNIT), vec![]), input))
        //             }
        //         }
        //         TypedDecoder::Slice(_gt, expr, a) => {
        //             let size = expr.eval_value(scope).unwrap_usize();
        //             let (slice, input) = input
        //                 .split_at(size)
        //                 .ok_or(ParseError::overrun(size, input.offset))?;
        //             let (v, _) = a.parse(program, scope, slice)?;
        //             Ok((v, input))
        //         }
        //         TypedDecoder::Bits(_gt, a) => {
        //             let mut bits = Vec::with_capacity(input.remaining().len() * 8);
        //             for b in input.remaining() {
        //                 for i in 0..8 {
        //                     bits.push((b & (1 << i)) >> i);
        //                 }
        //             }
        //             let (v, bits) = a.parse(program, scope, ReadCtxt::new(&bits))?;
        //             let bytes_remain = bits.remaining().len() >> 3;
        //             let bytes_read = input.remaining().len() - bytes_remain;
        //             let (_, input) = input
        //                 .split_at(bytes_read)
        //                 .ok_or(ParseError::overrun(bytes_read, input.offset))?;
        //             Ok((v, input))
        //         }
        //         TypedDecoder::WithRelativeOffset(_gt, expr, a) => {
        //             let offset = expr.eval_value(scope).unwrap_usize();
        //             let (_, slice) = input
        //                 .split_at(offset)
        //                 .ok_or(ParseError::overrun(offset, input.offset))?;
        //             let (v, _) = a.parse(program, scope, slice)?;
        //             Ok((v, input))
        //         }
        //         TypedDecoder::Map(gt, d, expr) => {
        //             let (orig, input) = d.parse(program, scope, input)?;
        //             let v = expr.eval_lambda(scope, &orig);
        //             Ok((TypedValue::Mapped(gt.clone(), Box::new(orig), Box::new(v)), input))
        //         }
        //         TypedDecoder::Compute(_gt, expr) => {
        //             let v = expr.eval_value(scope);
        //             Ok((v, input))
        //         }
        //         TypedDecoder::Let(_gt, name, expr, d) => {
        //             let v = expr.eval_value(scope);
        //             let let_scope = TSingleScope::new(scope, name, &v);
        //             d.parse(program, &TScope::Single(let_scope), input)
        //         }
        //         TypedDecoder::Match(_gt, head, branches) => {
        //             let head = head.eval(scope);
        //             for (index, (pattern, decoder)) in branches.iter().enumerate() {
        //                 if let Some(pattern_scope) = head.matches(scope, pattern) {
        //                     let (v, input) = decoder.parse(
        //                         program,
        //                         &TScope::Multi(&pattern_scope),
        //                         input
        //                     )?;
        //                     return Ok((TypedValue::Branch(_gt.clone(), index, Box::new(v)), input));
        //                 }
        //             }
        //             panic!("non-exhaustive patterns");
        //         }
        //         TypedDecoder::Dynamic(
        //             gt,
        //             name,
        //             TypedDynFormat::Huffman(lengths_expr, opt_values_expr),
        //             d,
        //         ) => {
        //             let lengths_val = lengths_expr.eval(scope);
        //             let lengths = value_to_vec_usize(&lengths_val);
        //             let lengths = match opt_values_expr {
        //                 None => lengths,
        //                 Some(e) => {
        //                     let values = value_to_vec_usize(&e.eval(scope));
        //                     let mut new_lengths = [0].repeat(values.len());
        //                     for i in 0..lengths.len() {
        //                         new_lengths[values[i]] = lengths[i];
        //                     }
        //                     new_lengths
        //                 }
        //             };
        //             let f = make_huffman_codes(&lengths);
        //             let dyn_d = Compiler::compile_one(&f).unwrap();
        //             let child_scope = TypedDecoderScope::new(scope, name, dyn_d);
        //             d.parse(program, &TScope::Decoder(child_scope), input)
        //         }
        //         TypedDecoder::Apply(gt, label) => {
        //             let deref = scope.get_decoder_by_name(label);
        //             deref.parse(program, scope, input)
        //         }
        //     }
        // }
    }

    fn value_to_vec_usize(v: &TypedValue<GenType>) -> Vec<usize> {
        let vs = match v {
            TypedValue::Seq(_, vs) => vs,
            _ => panic!("expected Seq"),
        };
        vs.iter()
            .map(|v| match v.coerce_mapped_value() {
                TypedValue::U8(n) => *n as usize,
                TypedValue::U16(n) => *n as usize,
                _ => panic!("expected U8 or U16"),
            })
            .collect::<Vec<usize>>()
    }

    fn make_huffman_codes(lengths: &[usize]) -> GTFormat {
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
                let range = bit_range(len, next_code[len]);
                let range_t = range.get_type().unwrap().into_owned();
                codes.push(TypedFormat::Map(
                    GenType::from(PrimType::U16),
                    Box::new(range),
                    TypedExpr::Lambda(
                        // FIXME - this is obviously wrong but we want it to compile sooner so we will fix it later
                        (range_t, GenType::from(PrimType::U16)),
                        "_".into(),
                        Box::new(TypedExpr::U16(n.try_into().unwrap())),
                    ),
                ));
                //println!("{:?}", codes[codes.len()-1]);
                next_code[len] += 1;
            } else {
                //codes.push((n.to_string(), Format::Fail));
            }
        }

        GTFormat::Union(PrimType::U16.into(), codes)
    }

    fn bit_range(n: usize, bits: usize) -> GTFormat {
        let mut fs = Vec::with_capacity(n);
        for i in 0..n {
            let r = n - 1 - i;
            let b = ((bits & (1 << r)) >> r) != 0;
            fs.push(is_bit(b));
        }
        GTFormat::tuple(fs)
    }

    fn is_bit(b: bool) -> GTFormat {
        GTFormat::Byte(ByteSet::from([if b { 1 } else { 0 }]))
    }

    fn inflate(codes: &[TypedValue<GenType>]) -> Vec<TypedValue<GenType>> {
        let mut vs = Vec::new();
        for code in codes {
            match code {
                TypedValue::Variant(t, name, v) => match (name.as_ref(), v.as_ref()) {
                    ("literal", v) => match v.coerce_mapped_value() {
                        TypedValue::U8(b) => vs.push(TypedValue::U8(*b)),
                        _ => panic!("inflate: expected U8"),
                    },
                    ("reference", TypedValue::Record(r_t, fields)) => {
                        let length = &fields
                            .iter()
                            .find(|(label, _)| label == "length")
                            .unwrap()
                            .1;
                        let distance = &fields
                            .iter()
                            .find(|(label, _)| label == "distance")
                            .unwrap()
                            .1;
                        match (length, distance) {
                            (TypedValue::U16(length), TypedValue::U16(distance)) => {
                                let length = *length as usize;
                                let distance = *distance as usize;
                                if distance > vs.len() {
                                    panic!("inflate: distance out of range");
                                }
                                let start = vs.len() - distance;
                                for i in 0..length {
                                    vs.push(vs[start + i].clone());
                                }
                            }
                            _ => panic!(
                                "inflate: unexpected length/distance {:?} {:?}",
                                length, distance
                            ),
                        }
                    }
                    _ => panic!("inflate: unknown code"),
                },
                _ => panic!("inflate: expected variant"),
            }
        }
        vs
    }
}

/// Decoders with a fixed amount of lookahead
#[derive(Clone, Debug)]
pub(crate) enum TypedDecoder<TypeRep> {
    Call(TypeRep, usize, Vec<(Label, TypedExpr<TypeRep>)>),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Variant(TypeRep, Label, Box<TypedDecoder<TypeRep>>),
    Parallel(TypeRep, Vec<TypedDecoder<TypeRep>>),
    Branch(TypeRep, MatchTree, Vec<TypedDecoder<TypeRep>>),
    Tuple(TypeRep, Vec<TypedDecoder<TypeRep>>),
    Record(TypeRep, Vec<(Label, TypedDecoder<TypeRep>)>),
    While(TypeRep, MatchTree, Box<TypedDecoder<TypeRep>>),
    Until(TypeRep, MatchTree, Box<TypedDecoder<TypeRep>>),
    RepeatCount(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    RepeatUntilLast(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    RepeatUntilSeq(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    Peek(TypeRep, Box<TypedDecoder<TypeRep>>),
    PeekNot(TypeRep, Box<TypedDecoder<TypeRep>>),
    Slice(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    Bits(TypeRep, Box<TypedDecoder<TypeRep>>),
    WithRelativeOffset(TypeRep, TypedExpr<TypeRep>, Box<TypedDecoder<TypeRep>>),
    Map(TypeRep, Box<TypedDecoder<TypeRep>>, TypedExpr<TypeRep>),
    Compute(TypeRep, TypedExpr<TypeRep>),
    Let(
        TypeRep,
        Label,
        TypedExpr<TypeRep>,
        Box<TypedDecoder<TypeRep>>,
    ),
    Match(
        TypeRep,
        TypedExpr<TypeRep>,
        Vec<(TypedPattern<TypeRep>, TypedDecoder<TypeRep>)>,
    ),
    Dynamic(
        TypeRep,
        Label,
        TypedDynFormat<TypeRep>,
        Box<TypedDecoder<TypeRep>>,
    ),
    Apply(TypeRep, Label),
}

#[derive(Clone, Debug)]
pub(crate) struct TypedProgram<TypeRep> {
    pub decoders: Vec<(TypedDecoder<TypeRep>, TypeRep)>,
}

impl TypedProgram<GenType> {
    fn new() -> Self {
        let decoders = Vec::new();
        TypedProgram { decoders }
    }

    // pub fn run<'input>(
    //     &self,
    //     input: ReadCtxt<'input>
    // ) -> ParseResult<(TypedValue<GenType>, ReadCtxt<'input>)> {
    //     self.decoders[0].0.parse(self, &TScope::Empty, input)
    // }
}

pub struct GTCompiler<'a> {
    module: &'a FormatModule,
    program: TypedProgram<GenType>,
    decoder_map: HashMap<(usize, Rc<Next<'a, GTFormat>>), usize>,
    compile_queue: Vec<(&'a GTFormat, Rc<Next<'a, GTFormat>>, usize)>,
}

type GTDecoder = TypedDecoder<GenType>;

impl<'a> GTCompiler<'a> {
    fn new(module: &'a FormatModule) -> Self {
        let program = TypedProgram::new();
        let decoder_map = HashMap::new();
        let compile_queue = Vec::new();
        GTCompiler {
            module,
            program,
            decoder_map,
            compile_queue,
        }
    }

    pub(crate) fn compile_program(
        module: &FormatModule,
        format: &GTFormat,
    ) -> AResult<TypedProgram<GenType>> {
        let mut compiler = GTCompiler::new(module);
        // type
        let scope = TypeScope::new();
        let t = match format.get_type() {
            None => unreachable!("cannot compile program from Void top-level format-type"),
            Some(t) => t.into_owned(),
        };
        // decoder
        compiler.queue_compile(t, format, Rc::new(Next::Empty));
        while let Some((f, next, n)) = compiler.compile_queue.pop() {
            let d = compiler.compile_gt_format(f, next)?;
            compiler.program.decoders[n].0 = d;
        }
        Ok(compiler.program)
    }

    fn queue_compile(
        &mut self,
        t: GenType,
        f: &'a GTFormat,
        next: Rc<Next<'a, GTFormat>>,
    ) -> usize {
        let n = self.program.decoders.len();
        self.program.decoders.push((TypedDecoder::Fail, t));
        self.compile_queue.push((f, next, n));
        n
    }

    pub(crate) fn compile_one(format: &GTFormat) -> AResult<GTDecoder> {
        let module = FormatModule::new();
        let mut compiler = GTCompiler::new(&module);
        compiler.compile_gt_format(format, Rc::new(Next::Empty))
    }

    fn compile_gt_format(
        &mut self,
        format: &'a GTFormat,
        next: Rc<Next<'a, GTFormat>>,
    ) -> AResult<GTDecoder> {
        match format {
            GTFormat::FormatCall(gt, level, arg_exprs, deref) => {
                let _f = self.module.get_format(*level);
                let next = if _f.depends_on_next(self.module) {
                    next
                } else {
                    Rc::new(Next::Empty)
                };
                let n = if let Some(n) = self.decoder_map.get(&(*level, next.clone())) {
                    *n
                } else {
                    let n = self.queue_compile(gt.clone(), deref, next.clone());
                    self.decoder_map.insert((*level, next.clone()), n);
                    n
                };
                let mut args = Vec::new();
                for (label, expr) in arg_exprs.iter() {
                    args.push((label.clone(), expr.clone()));
                }
                Ok(TypedDecoder::Call(gt.clone(), n, args))
            }
            GTFormat::Fail => Ok(TypedDecoder::Fail),
            GTFormat::EndOfInput => Ok(TypedDecoder::EndOfInput),
            GTFormat::Align(n) => Ok(TypedDecoder::Align(*n)),
            GTFormat::Byte(bs) => Ok(TypedDecoder::Byte(*bs)),
            GTFormat::Variant(gt, label, f) => {
                let d = self.compile_gt_format(f, next.clone())?;
                Ok(TypedDecoder::Variant(
                    gt.clone(),
                    label.clone(),
                    Box::new(d),
                ))
            }
            GTFormat::Union(gt, branches) => {
                let mut fs = Vec::with_capacity(branches.len());
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    ds.push(self.compile_gt_format(f, next.clone())?);
                    fs.push(f.clone());
                }
                if let Some(tree) = MatchTree::build_typed(self.module, &fs, next) {
                    Ok(TypedDecoder::Branch(gt.clone(), tree, ds))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::UnionNondet(gt, branches) => {
                let mut ds = Vec::with_capacity(branches.len());
                for f in branches {
                    let d = self.compile_gt_format(f, next.clone())?;
                    ds.push(d);
                }
                Ok(TypedDecoder::Parallel(gt.clone(), ds))
            }
            GTFormat::Tuple(gt, fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some(f) = fields.next() {
                    let next = Rc::new(Next::<GTFormat>::Tuple(fields.as_slice(), next.clone()));
                    let df = self.compile_gt_format(f, next)?;
                    dfields.push(df);
                }
                Ok(TypedDecoder::Tuple(gt.clone(), dfields))
            }
            GTFormat::Record(gt, fields) => {
                let mut dfields = Vec::with_capacity(fields.len());
                let mut fields = fields.iter();
                while let Some((name, f)) = fields.next() {
                    let next = Rc::new(Next::Record(fields.as_slice(), next.clone()));
                    let df = self.compile_gt_format(f, next)?;
                    dfields.push((name.clone(), df));
                }
                Ok(TypedDecoder::Record(gt.clone(), dfields))
            }
            GTFormat::Repeat(gt, a) => {
                if a.as_ref().is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da = self.compile_gt_format(a, Rc::new(Next::Repeat(a, next.clone())))?;
                let astar = TypedFormat::Repeat(gt.clone(), a.clone());
                let fa = TypedFormat::tuple(vec![(**a).clone(), astar]);
                let fb = TypedFormat::EMPTY;
                if let Some(tree) = MatchTree::build_typed(self.module, &[fa, fb], next) {
                    Ok(TypedDecoder::While(gt.clone(), tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::Repeat1(gt, a) => {
                if a.is_nullable(self.module) {
                    return Err(anyhow!("cannot repeat nullable format: {a:?}"));
                }
                let da =
                    self.compile_gt_format(a, Rc::new(Next::Repeat(a.as_ref(), next.clone())))?;
                let astar = TypedFormat::Repeat(gt.clone(), a.clone());
                let fa = TypedFormat::EMPTY;
                let fb = TypedFormat::tuple(vec![(**a).clone(), astar]);
                if let Some(tree) = MatchTree::build_typed(self.module, &[fa, fb], next) {
                    Ok(TypedDecoder::Until(gt.clone(), tree, Box::new(da)))
                } else {
                    Err(anyhow!("cannot build match tree for {:?}", format))
                }
            }
            GTFormat::RepeatCount(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, next)?);
                Ok(TypedDecoder::RepeatCount(gt.clone(), expr.clone(), da))
            }
            GTFormat::RepeatUntilLast(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, next)?);
                Ok(TypedDecoder::RepeatUntilLast(gt.clone(), expr.clone(), da))
            }
            GTFormat::RepeatUntilSeq(gt, expr, a) => {
                // FIXME probably not right
                let da = Box::new(self.compile_gt_format(a, next)?);
                Ok(TypedDecoder::RepeatUntilSeq(gt.clone(), expr.clone(), da))
            }
            GTFormat::Peek(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Peek(gt.clone(), da))
            }
            GTFormat::PeekNot(_t, a) => {
                const MAX_LOOKAHEAD: usize = 1024;
                match a.match_bounds(self.module).max {
                    None => {
                        return Err(anyhow!("PeekNot cannot require unbounded lookahead"));
                    }
                    Some(n) if n > MAX_LOOKAHEAD => {
                        return Err(anyhow!(
                            "PeekNot cannot require > {MAX_LOOKAHEAD} bytes lookahead"
                        ));
                    }
                    _ => {}
                }
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::PeekNot(_t.clone(), da))
            }
            GTFormat::Slice(gt, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Slice(gt.clone(), expr.clone(), da))
            }
            GTFormat::Bits(gt, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::Bits(gt.clone(), da))
            }
            GTFormat::WithRelativeOffset(gt, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, Rc::new(Next::Empty))?);
                Ok(TypedDecoder::WithRelativeOffset(
                    gt.clone(),
                    expr.clone(),
                    da,
                ))
            }
            GTFormat::Map(gt, a, expr) => {
                let da = Box::new(self.compile_gt_format(a, next.clone())?);
                Ok(TypedDecoder::Map(gt.clone(), da, expr.clone()))
            }
            GTFormat::Compute(gt, expr) => Ok(TypedDecoder::Compute(gt.clone(), expr.clone())),
            GTFormat::Let(gt, name, expr, a) => {
                let da = Box::new(self.compile_gt_format(a, next.clone())?);
                Ok(TypedDecoder::Let(
                    gt.clone(),
                    name.clone(),
                    expr.clone(),
                    da,
                ))
            }
            GTFormat::Match(gt, head, branches) => {
                let branches = branches
                    .iter()
                    .map(|(pattern, f)| {
                        Ok((pattern.clone(), self.compile_gt_format(f, next.clone())?))
                    })
                    .collect::<AResult<_>>()?;
                Ok(TypedDecoder::Match(gt.clone(), head.clone(), branches))
            }
            GTFormat::Dynamic(gt, name, dynformat, a) => {
                let da = Box::new(self.compile_gt_format(a, next.clone())?);
                Ok(TypedDecoder::Dynamic(
                    gt.clone(),
                    name.clone(),
                    dynformat.clone(),
                    da,
                ))
            }
            GTFormat::Apply(gt, name, _) => Ok(TypedDecoder::Apply(gt.clone(), name.clone())),
        }
    }
}
