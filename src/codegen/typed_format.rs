use std::borrow::Cow;
use std::ops::Add;
use std::rc::Rc;

use super::rust_ast::{PrimType, RustType, RustTypeDef};
use super::{AtomType, LocalType};
use crate::bounds::Bounds;
use crate::byte_set::ByteSet;
use crate::{Arith, IntRel, Label, ValueType};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum GenType {
    Inline(RustType),
    Def((usize, Label), RustTypeDef),
}

impl GenType {
    pub(crate) fn to_rust_type(&self) -> RustType {
        match self {
            GenType::Inline(rt) => rt.clone(),
            GenType::Def((ix, lbl), _) => RustType::defined(*ix, lbl.clone()),
        }
    }

    // pub(crate) fn into_rust_type(self) -> RustType {
    //     match self {
    //         GenType::Inline(rt) => rt,
    //         GenType::Def((ix, lbl), _) => RustType::defined(ix, lbl.clone()),
    //     }
    // }

    /// Attempt to extract the type-index and corresponding name (`Label`) from `self`.
    ///
    /// Returns `None` if the type in question is not itself a concrete definition (`GenType::Def`)
    /// or an abstract reference to a locally-defined adhoc type (`GenType::Inline` of nested `LocalType::LocalDef`).
    pub(crate) fn try_as_adhoc(&self) -> Option<(usize, &Label)> {
        match self {
            GenType::Def((ix, lbl), ..)
            | GenType::Inline(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix, lbl)))) => {
                Some((*ix, lbl))
            }
            _ => None,
        }
    }
}

impl<TypeRep> std::hash::Hash for TypedFormat<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let disc = core::mem::discriminant(self);
        disc.hash(state);
        match self {
            TypedFormat::FormatCall(_, level, args, _) => {
                level.hash(state);
                args.hash(state);
            }
            TypedFormat::SkipRemainder
            | TypedFormat::Pos
            | TypedFormat::Fail
            | TypedFormat::EndOfInput => {}
            TypedFormat::Align(n) => n.hash(state),
            TypedFormat::Byte(bs) => bs.hash(state),
            TypedFormat::Variant(_tr, lbl, inner) => {
                lbl.hash(state);
                inner.hash(state);
            }
            TypedFormat::Union(_, branches) | TypedFormat::UnionNondet(_, branches) => {
                branches.hash(state)
            }
            TypedFormat::Tuple(_, elts) => elts.hash(state),
            TypedFormat::Record(_, flds) => flds.hash(state),
            TypedFormat::RepeatCount(_, n, inner) => {
                n.hash(state);
                inner.hash(state);
            }
            TypedFormat::ForEach(_, expr, lbl, inner) => {
                expr.hash(state);
                lbl.hash(state);
                inner.hash(state);
            }
            TypedFormat::RepeatBetween(_, lo, hi, inner) => {
                lo.hash(state);
                hi.hash(state);
                inner.hash(state);
            }
            TypedFormat::RepeatUntilLast(_, f, inner)
            | TypedFormat::RepeatUntilSeq(_, f, inner) => {
                f.hash(state);
                inner.hash(state);
            }
            TypedFormat::Maybe(_, cond, inner) => {
                cond.hash(state);
                inner.hash(state);
            }
            TypedFormat::Repeat(_, inner)
            | TypedFormat::Repeat1(_, inner)
            | TypedFormat::Bits(_, inner)
            | TypedFormat::Peek(_, inner)
            | TypedFormat::PeekNot(_, inner) => inner.hash(state),
            TypedFormat::Slice(_, sz, inner) => {
                sz.hash(state);
                inner.hash(state);
            }
            TypedFormat::WithRelativeOffset(_, ofs, inner) => {
                ofs.hash(state);
                inner.hash(state);
            }
            TypedFormat::Map(_, orig, f) | TypedFormat::Where(_, orig, f) => {
                orig.hash(state);
                f.hash(state);
            }
            TypedFormat::Compute(_, expr) => expr.hash(state),
            TypedFormat::Let(_, lb, x, inner) => {
                lb.hash(state);
                x.hash(state);
                inner.hash(state);
            }
            TypedFormat::Match(_, head, cases) => {
                head.hash(state);
                cases.hash(state);
            }
            TypedFormat::Dynamic(_, lb, dynf, inner) => {
                lb.hash(state);
                dynf.hash(state);
                inner.hash(state);
            }
            TypedFormat::Apply(_, lbl, _) => lbl.hash(state),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedFormat<TypeRep> {
    FormatCall(
        TypeRep,
        usize,
        Vec<(Label, TypedExpr<TypeRep>)>,
        Rc<TypedFormat<TypeRep>>,
    ),
    ForEach(
        TypeRep,
        TypedExpr<TypeRep>,
        Label,
        Box<TypedFormat<TypeRep>>,
    ),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Variant(TypeRep, Label, Box<TypedFormat<TypeRep>>),
    Union(TypeRep, Vec<TypedFormat<TypeRep>>),
    UnionNondet(TypeRep, Vec<TypedFormat<TypeRep>>),
    Tuple(TypeRep, Vec<TypedFormat<TypeRep>>),
    Record(TypeRep, Vec<(Label, TypedFormat<TypeRep>)>),
    Repeat(TypeRep, Box<TypedFormat<TypeRep>>),
    Repeat1(TypeRep, Box<TypedFormat<TypeRep>>),
    RepeatCount(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    RepeatBetween(
        TypeRep,
        TypedExpr<TypeRep>,
        TypedExpr<TypeRep>,
        Box<TypedFormat<TypeRep>>,
    ),
    RepeatUntilLast(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    RepeatUntilSeq(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    Maybe(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    Peek(TypeRep, Box<TypedFormat<TypeRep>>),
    PeekNot(TypeRep, Box<TypedFormat<TypeRep>>),
    Slice(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    Bits(TypeRep, Box<TypedFormat<TypeRep>>),
    WithRelativeOffset(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    Map(TypeRep, Box<TypedFormat<TypeRep>>, TypedExpr<TypeRep>),
    Where(TypeRep, Box<TypedFormat<TypeRep>>, TypedExpr<TypeRep>),
    Compute(TypeRep, TypedExpr<TypeRep>),
    Let(
        TypeRep,
        Label,
        TypedExpr<TypeRep>,
        Box<TypedFormat<TypeRep>>,
    ),
    Match(
        TypeRep,
        TypedExpr<TypeRep>,
        Vec<(TypedPattern<TypeRep>, TypedFormat<TypeRep>)>,
    ),
    Dynamic(
        TypeRep,
        Label,
        TypedDynFormat<TypeRep>,
        Box<TypedFormat<TypeRep>>,
    ),
    Apply(TypeRep, Label, Rc<TypedDynFormat<TypeRep>>),
    Pos,
    SkipRemainder,
}

impl TypedFormat<GenType> {
    pub const EMPTY: Self = TypedFormat::Tuple(GenType::Inline(RustType::UNIT), Vec::new());

    pub(crate) fn lookahead_bounds(&self) -> Bounds {
        match self {
            TypedFormat::FormatCall(_gt, _lvl, _args, def) => def.lookahead_bounds(),

            TypedFormat::SkipRemainder
            | TypedFormat::Pos
            | TypedFormat::Compute(_, _)
            | TypedFormat::EndOfInput
            | TypedFormat::Fail => Bounds::exact(0),

            TypedFormat::Peek(_, inner) | TypedFormat::PeekNot(_, inner) => {
                inner.lookahead_bounds()
            }

            TypedFormat::Align(n) => Bounds::new(0, n - 1),
            TypedFormat::Byte(_) => Bounds::exact(1),
            TypedFormat::Variant(_, _, f) => f.lookahead_bounds(),
            TypedFormat::Union(_, branches) | TypedFormat::UnionNondet(_, branches) => branches
                .iter()
                .map(TypedFormat::lookahead_bounds)
                .reduce(Bounds::union)
                .unwrap(),
            TypedFormat::Tuple(_, elts) => elts
                .iter()
                .map(TypedFormat::lookahead_bounds)
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),
            TypedFormat::Record(_, flds) => flds
                .iter()
                .map(|(_l, f)| f.lookahead_bounds())
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),

            TypedFormat::RepeatCount(_, t_exp, f) => f.lookahead_bounds() * t_exp.bounds(),
            TypedFormat::RepeatBetween(_, t_min, t_max, f) => {
                f.lookahead_bounds() * Bounds::union(t_min.bounds(), t_max.bounds())
            }

            TypedFormat::Repeat1(_, f) | TypedFormat::RepeatUntilLast(_, _, f) => {
                f.lookahead_bounds() * Bounds::at_least(1)
            }

            TypedFormat::Repeat(_, _f) | TypedFormat::RepeatUntilSeq(_, _, _f) => Bounds::any(),
            // REVIEW - can we do any better than this?
            TypedFormat::ForEach(_, _expr, _lbl, _f) => Bounds::any(),
            TypedFormat::Maybe(_, _, f) => Bounds::union(Bounds::exact(0), f.lookahead_bounds()),

            TypedFormat::Slice(_, t_expr, _) => t_expr.bounds(),

            TypedFormat::Bits(_, f) => f.lookahead_bounds().bits_to_bytes(),

            TypedFormat::WithRelativeOffset(_, offset_expr, inner) => {
                offset_expr.bounds() + inner.lookahead_bounds()
            }

            TypedFormat::Map(_, f, _)
            | TypedFormat::Where(_, f, _)
            | TypedFormat::Dynamic(_, _, _, f)
            | TypedFormat::Let(_, _, _, f) => f.lookahead_bounds(),

            TypedFormat::Match(_, _, branches) => branches
                .iter()
                .map(|(_, f)| f.lookahead_bounds())
                .reduce(Bounds::union)
                .unwrap(),

            TypedFormat::Apply(_, _, _) => Bounds::at_least(1),
        }
    }

    pub(crate) fn match_bounds(&self) -> Bounds {
        match self {
            TypedFormat::FormatCall(_gt, _lvl, _args, def) => def.match_bounds(),

            TypedFormat::Compute(_, _)
            | TypedFormat::Peek(_, _)
            | TypedFormat::PeekNot(_, _)
            | TypedFormat::EndOfInput
            | TypedFormat::Pos
            | TypedFormat::Fail => Bounds::exact(0),

            TypedFormat::Align(n) => Bounds::new(0, n - 1),
            TypedFormat::Byte(_) => Bounds::exact(1),
            TypedFormat::Variant(_, _, f) => f.match_bounds(),
            TypedFormat::Union(_, branches) | TypedFormat::UnionNondet(_, branches) => branches
                .iter()
                .map(TypedFormat::match_bounds)
                .reduce(Bounds::union)
                .unwrap(),
            TypedFormat::Tuple(_, elts) => elts
                .iter()
                .map(TypedFormat::match_bounds)
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),
            TypedFormat::Record(_, flds) => flds
                .iter()
                .map(|(_l, f)| f.match_bounds())
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),

            TypedFormat::RepeatCount(_, t_exp, f) => f.match_bounds() * t_exp.bounds(),
            TypedFormat::RepeatBetween(_, t_min, t_max, f) => {
                f.match_bounds() * Bounds::union(t_min.bounds(), t_max.bounds())
            }

            TypedFormat::Repeat1(_, f) | TypedFormat::RepeatUntilLast(_, _, f) => {
                f.match_bounds() * Bounds::at_least(1)
            }

            TypedFormat::Repeat(_, _f) | TypedFormat::RepeatUntilSeq(_, _, _f) => Bounds::any(),
            // REVIEW - can we do any better than this?
            TypedFormat::ForEach(_, _expr, _lbl, _f) => Bounds::any(),
            TypedFormat::Maybe(_, _, f) => Bounds::union(Bounds::exact(0), f.match_bounds()),

            TypedFormat::SkipRemainder => Bounds::any(),

            TypedFormat::Slice(_, t_expr, _) => t_expr.bounds(),

            TypedFormat::Bits(_, f) => f.match_bounds().bits_to_bytes(),

            TypedFormat::WithRelativeOffset(_, _, _) => Bounds::exact(0),

            TypedFormat::Map(_, f, _)
            | TypedFormat::Where(_, f, _)
            | TypedFormat::Dynamic(_, _, _, f)
            | TypedFormat::Let(_, _, _, f) => f.match_bounds(),

            TypedFormat::Match(_, _, branches) => branches
                .iter()
                .map(|(_, f)| f.match_bounds())
                .reduce(Bounds::union)
                .unwrap(),

            TypedFormat::Apply(_, _, _) => Bounds::at_least(1),
        }
    }

    pub(crate) fn is_nullable(&self) -> bool {
        self.match_bounds().min == 0
    }

    pub(crate) fn tuple(elts: Vec<TypedFormat<GenType>>) -> Self {
        let mut elt_ts = Vec::with_capacity(elts.len());
        for elt in elts.iter() {
            let Some(elt_t) = elt.get_type() else {
                unreachable!("tuple with 'Fail' element can never be parsed successfully")
            };
            elt_ts.push(elt_t.as_ref().clone().to_rust_type());
        }
        let gt = GenType::Inline(RustType::anon_tuple(elt_ts));
        TypedFormat::Tuple(gt, elts)
    }

    pub(crate) fn get_type(&self) -> Option<Cow<'_, GenType>> {
        match self {
            TypedFormat::Fail => None,
            TypedFormat::SkipRemainder | TypedFormat::EndOfInput | TypedFormat::Align(_) => {
                Some(Cow::Owned(GenType::from(RustType::UNIT)))
            }
            TypedFormat::Byte(_) => Some(Cow::Owned(GenType::from(PrimType::U8))),
            // REVIEW - forcing Pos to be a U64-valued format
            TypedFormat::Pos => Some(Cow::Owned(GenType::from(PrimType::U64))),

            TypedFormat::FormatCall(gt, ..)
            | TypedFormat::Variant(gt, ..)
            | TypedFormat::Union(gt, ..)
            | TypedFormat::UnionNondet(gt, ..)
            | TypedFormat::Tuple(gt, ..)
            | TypedFormat::Record(gt, ..)
            | TypedFormat::Repeat(gt, ..)
            | TypedFormat::Repeat1(gt, ..)
            | TypedFormat::ForEach(gt, ..)
            | TypedFormat::RepeatCount(gt, ..)
            | TypedFormat::RepeatBetween(gt, ..)
            | TypedFormat::RepeatUntilLast(gt, ..)
            | TypedFormat::RepeatUntilSeq(gt, ..)
            | TypedFormat::Maybe(gt, ..)
            | TypedFormat::Peek(gt, ..)
            | TypedFormat::PeekNot(gt, ..)
            | TypedFormat::Slice(gt, ..)
            | TypedFormat::Bits(gt, ..)
            | TypedFormat::WithRelativeOffset(gt, ..)
            | TypedFormat::Map(gt, ..)
            | TypedFormat::Where(gt, ..)
            | TypedFormat::Compute(gt, ..)
            | TypedFormat::Let(gt, ..)
            | TypedFormat::Match(gt, ..)
            | TypedFormat::Dynamic(gt, ..)
            | TypedFormat::Apply(gt, ..) => Some(Cow::Borrowed(gt)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedDynFormat<TypeRep> {
    Huffman(TypedExpr<TypeRep>, Option<TypedExpr<TypeRep>>),
}

impl<TypeRep> std::hash::Hash for TypedDynFormat<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            TypedDynFormat::Huffman(code_lengths, opt_code_values) => {
                code_lengths.hash(state);
                opt_code_values.hash(state);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedExpr<TypeRep> {
    Var(TypeRep, Label),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Tuple(TypeRep, Vec<TypedExpr<TypeRep>>),
    TupleProj(TypeRep, Box<TypedExpr<TypeRep>>, usize),
    Record(TypeRep, Vec<(Label, TypedExpr<TypeRep>)>),
    RecordProj(TypeRep, Box<TypedExpr<TypeRep>>, Label),
    Variant(TypeRep, Label, Box<TypedExpr<TypeRep>>),
    Seq(TypeRep, Vec<TypedExpr<TypeRep>>),
    Match(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Vec<(TypedPattern<TypeRep>, TypedExpr<TypeRep>)>,
    ),
    Lambda((TypeRep, TypeRep), Label, Box<TypedExpr<TypeRep>>),

    IntRel(
        TypeRep,
        IntRel,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
    Arith(
        TypeRep,
        Arith,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),

    AsU8(Box<TypedExpr<TypeRep>>),
    AsU16(Box<TypedExpr<TypeRep>>),
    AsU32(Box<TypedExpr<TypeRep>>),
    AsU64(Box<TypedExpr<TypeRep>>),
    AsChar(Box<TypedExpr<TypeRep>>),

    U16Be(Box<TypedExpr<TypeRep>>),
    U16Le(Box<TypedExpr<TypeRep>>),
    U32Be(Box<TypedExpr<TypeRep>>),
    U32Le(Box<TypedExpr<TypeRep>>),
    U64Be(Box<TypedExpr<TypeRep>>),
    U64Le(Box<TypedExpr<TypeRep>>),

    SeqLength(Box<TypedExpr<TypeRep>>),
    SubSeq(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
    SubSeqInflate(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
    ),
    FlatMap(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedExpr<TypeRep>>),
    FlatMapAccum(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        ValueType,
        Box<TypedExpr<TypeRep>>,
    ),
    FlatMapList(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        ValueType,
        Box<TypedExpr<TypeRep>>,
    ),
    Dup(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedExpr<TypeRep>>),
    LiftOption(TypeRep, Option<Box<TypedExpr<TypeRep>>>),
}

impl<TypeRep> std::hash::Hash for TypedExpr<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            TypedExpr::Var(_, lbl) => lbl.hash(state),
            TypedExpr::Bool(b) => b.hash(state),
            TypedExpr::U8(n) => n.hash(state),
            TypedExpr::U16(n) => n.hash(state),
            TypedExpr::U32(n) => n.hash(state),
            TypedExpr::U64(n) => n.hash(state),
            TypedExpr::Tuple(_, ts) => ts.hash(state),
            TypedExpr::TupleProj(_, tup, ix) => {
                tup.hash(state);
                ix.hash(state);
            }
            TypedExpr::Record(_, fs) => fs.hash(state),
            TypedExpr::RecordProj(_, rec, fld) => {
                rec.hash(state);
                fld.hash(state);
            }
            TypedExpr::Variant(_, lbl, inner) => {
                lbl.hash(state);
                inner.hash(state);
            }
            TypedExpr::Seq(_, sq) => sq.hash(state),
            TypedExpr::Match(_, head, cases) => {
                head.hash(state);
                cases.hash(state);
            }
            TypedExpr::Lambda(_, var, body) => {
                var.hash(state);
                body.hash(state);
            }
            TypedExpr::IntRel(_, rel, lhs, rhs) => {
                rel.hash(state);
                lhs.hash(state);
                rhs.hash(state);
            }
            TypedExpr::Arith(_, ath, lhs, rhs) => {
                ath.hash(state);
                lhs.hash(state);
                rhs.hash(state);
            }
            TypedExpr::AsU8(inner)
            | TypedExpr::AsU16(inner)
            | TypedExpr::AsU32(inner)
            | TypedExpr::AsU64(inner)
            | TypedExpr::AsChar(inner)
            | TypedExpr::U16Be(inner)
            | TypedExpr::U16Le(inner)
            | TypedExpr::U32Be(inner)
            | TypedExpr::U32Le(inner)
            | TypedExpr::U64Be(inner)
            | TypedExpr::U64Le(inner)
            | TypedExpr::SeqLength(inner) => inner.hash(state),
            TypedExpr::SubSeq(_, sq, start, len) | TypedExpr::SubSeqInflate(_, sq, start, len) => {
                sq.hash(state);
                start.hash(state);
                len.hash(state);
            }
            TypedExpr::FlatMap(_, f, sq) => {
                f.hash(state);
                sq.hash(state);
            }
            TypedExpr::FlatMapAccum(_, f, acc, _vt, seq) => {
                f.hash(state);
                acc.hash(state);
                seq.hash(state);
            }
            TypedExpr::FlatMapList(_, f, _vt, seq) => {
                f.hash(state);
                seq.hash(state);
            }
            TypedExpr::Dup(_, n, x) => {
                n.hash(state);
                x.hash(state);
            }
            TypedExpr::LiftOption(_, opt) => opt.hash(state),
        }
    }
}

impl<TypeRep> TypedExpr<TypeRep> {
    pub(crate) fn bounds(&self) -> Bounds {
        match self {
            TypedExpr::U8(n) => Bounds::exact(usize::from(*n)),
            TypedExpr::U16(n) => Bounds::exact(usize::from(*n)),
            TypedExpr::U32(n) => Bounds::exact(*n as usize),
            TypedExpr::U64(n) => Bounds::exact(*n as usize),
            TypedExpr::Arith(_t, Arith::Add, a, b) => a.bounds() + b.bounds(),
            TypedExpr::Arith(_t, Arith::Mul, a, b) => a.bounds() * b.bounds(),
            _ => Bounds::any(),
        }
    }
}

impl TypedExpr<GenType> {
    pub(crate) fn get_type(&self) -> Option<Cow<'_, GenType>> {
        match self {
            TypedExpr::Bool(_) => Some(Cow::Owned(GenType::from(PrimType::Bool))),
            TypedExpr::AsU8(_) | TypedExpr::U8(_) => Some(Cow::Owned(GenType::from(PrimType::U8))),
            TypedExpr::U16Le(_) | TypedExpr::U16Be(_) | TypedExpr::AsU16(_) | TypedExpr::U16(_) => {
                Some(Cow::Owned(GenType::from(PrimType::U16)))
            }
            TypedExpr::U32Be(_)
            | TypedExpr::U32Le(_)
            | TypedExpr::AsU32(_)
            | TypedExpr::U32(_)
            | TypedExpr::SeqLength(_) => Some(Cow::Owned(GenType::from(PrimType::U32))),
            TypedExpr::U64Be(_) | TypedExpr::U64Le(_) | TypedExpr::AsU64(_) | TypedExpr::U64(_) => {
                Some(Cow::Owned(GenType::from(PrimType::U64)))
            }
            TypedExpr::AsChar(_) => Some(Cow::Owned(GenType::from(PrimType::Char))),
            TypedExpr::Lambda(..) => None,
            TypedExpr::Var(gt, _)
            | TypedExpr::Tuple(gt, _)
            | TypedExpr::TupleProj(gt, _, _)
            | TypedExpr::Record(gt, _)
            | TypedExpr::RecordProj(gt, _, _)
            | TypedExpr::Variant(gt, _, _)
            | TypedExpr::Seq(gt, _)
            | TypedExpr::Match(gt, _, _)
            | TypedExpr::IntRel(gt, _, _, _)
            | TypedExpr::Arith(gt, _, _, _)
            | TypedExpr::SubSeq(gt, _, _, _)
            | TypedExpr::SubSeqInflate(gt, _, _, _)
            | TypedExpr::FlatMap(gt, _, _)
            | TypedExpr::FlatMapAccum(gt, _, _, _, _)
            | TypedExpr::FlatMapList(gt, _, _, _)
            | TypedExpr::LiftOption(gt, _)
            | TypedExpr::Dup(gt, _, _) => Some(Cow::Borrowed(gt)),
        }
    }
}

// FIXME - same as TypedExpr, requirements of HashMap include Eq and Hash for this type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedPattern<TypeRep> {
    Binding(TypeRep, Label),
    Wildcard(TypeRep),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Int(TypeRep, Bounds),
    Char(char),
    Tuple(TypeRep, Vec<TypedPattern<TypeRep>>),
    Variant(TypeRep, Label, Box<TypedPattern<TypeRep>>),
    Seq(TypeRep, Vec<TypedPattern<TypeRep>>),
    Option(TypeRep, Option<Box<TypedPattern<TypeRep>>>),
}

impl<TypeRep> std::hash::Hash for TypedPattern<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            TypedPattern::Binding(_, lbl) => lbl.hash(state),
            TypedPattern::Wildcard(_) => {}
            TypedPattern::Bool(b) => b.hash(state),
            TypedPattern::U8(n) => n.hash(state),
            TypedPattern::U16(n) => n.hash(state),
            TypedPattern::U32(n) => n.hash(state),
            TypedPattern::U64(n) => n.hash(state),
            TypedPattern::Int(_, bounds) => bounds.hash(state),
            TypedPattern::Char(c) => c.hash(state),
            TypedPattern::Tuple(_, tup) => tup.hash(state),
            TypedPattern::Variant(_, lbl, inner) => {
                lbl.hash(state);
                inner.hash(state);
            }
            TypedPattern::Seq(_, elts) => elts.hash(state),
            TypedPattern::Option(_, opt) => opt.hash(state),
        }
    }
}

mod __impls {
    use super::{GenType, TypedDynFormat, TypedExpr, TypedFormat, TypedPattern};
    use crate::{
        codegen::{
            rust_ast::{AtomType, CompType, PrimType, RustType, RustTypeDef},
            IxLabel,
        },
        DynFormat, Expr, Format, Pattern,
    };

    impl From<RustType> for GenType {
        fn from(value: RustType) -> Self {
            GenType::Inline(value)
        }
    }

    impl From<(IxLabel, RustTypeDef)> for GenType {
        fn from(value: (IxLabel, RustTypeDef)) -> Self {
            let ix = value.0.to_usize();
            let lbl = value.0.into();
            GenType::Def((ix, lbl), value.1)
        }
    }

    impl From<PrimType> for GenType {
        fn from(value: PrimType) -> Self {
            GenType::Inline(RustType::from(value))
        }
    }

    impl From<CompType> for GenType {
        fn from(value: CompType) -> Self {
            GenType::Inline(RustType::from(value))
        }
    }

    impl From<AtomType> for GenType {
        fn from(value: AtomType) -> Self {
            GenType::Inline(RustType::from(value))
        }
    }

    fn revec<T, U: From<T>>(v: Vec<T>) -> Vec<U> {
        v.into_iter().map(U::from).collect()
    }

    fn revec_pair<A, X: From<A>, B, Y: From<B>>(v: Vec<(A, B)>) -> Vec<(X, Y)> {
        v.into_iter()
            .map(|(a, b)| (X::from(a), Y::from(b)))
            .collect()
    }

    fn rebox<T, U: From<T>>(b: Box<T>) -> Box<U> {
        Box::new(U::from(*b))
    }

    impl<TypeRep> From<TypedExpr<TypeRep>> for Expr {
        fn from(value: TypedExpr<TypeRep>) -> Self {
            match value {
                TypedExpr::Var(_, lbl) => Expr::Var(lbl),
                TypedExpr::Bool(b) => Expr::Bool(b),
                TypedExpr::U8(n) => Expr::U8(n),
                TypedExpr::U16(n) => Expr::U16(n),
                TypedExpr::U32(n) => Expr::U32(n),
                TypedExpr::U64(n) => Expr::U64(n),
                TypedExpr::Tuple(_, t_elts) => Expr::Tuple(revec(t_elts)),
                TypedExpr::TupleProj(_, tup, ix) => Expr::TupleProj(rebox(tup), ix),
                TypedExpr::Record(_, t_flds) => Expr::Record(revec_pair(t_flds)),
                TypedExpr::RecordProj(_, rec, fld) => Expr::RecordProj(rebox(rec), fld),
                TypedExpr::Variant(_, name, inner) => Expr::Variant(name, rebox(inner)),
                TypedExpr::Seq(_, t_elems) => Expr::Seq(revec(t_elems)),
                TypedExpr::Match(_, head, branches) => {
                    Expr::Match(rebox(head), revec_pair(branches))
                }
                TypedExpr::Lambda(_, name, inner) => Expr::Lambda(name, rebox(inner)),
                TypedExpr::IntRel(_, rel, x, y) => Expr::IntRel(rel, rebox(x), rebox(y)),
                TypedExpr::Arith(_, op, x, y) => Expr::Arith(op, rebox(x), rebox(y)),
                TypedExpr::AsU8(x) => Expr::AsU8(rebox(x)),
                TypedExpr::AsU16(x) => Expr::AsU16(rebox(x)),
                TypedExpr::AsU32(x) => Expr::AsU32(rebox(x)),
                TypedExpr::AsU64(x) => Expr::AsU64(rebox(x)),
                TypedExpr::AsChar(x) => Expr::AsChar(rebox(x)),
                TypedExpr::U16Be(x) => Expr::U16Be(rebox(x)),
                TypedExpr::U16Le(x) => Expr::U16Le(rebox(x)),
                TypedExpr::U32Be(x) => Expr::U32Be(rebox(x)),
                TypedExpr::U32Le(x) => Expr::U32Le(rebox(x)),
                TypedExpr::U64Be(x) => Expr::U64Be(rebox(x)),
                TypedExpr::U64Le(x) => Expr::U64Le(rebox(x)),
                TypedExpr::SeqLength(x) => Expr::SeqLength(rebox(x)),
                TypedExpr::SubSeq(_, seq, start, len) => {
                    Expr::SubSeq(rebox(seq), rebox(start), rebox(len))
                }
                TypedExpr::SubSeqInflate(_, seq, start, len) => {
                    Expr::SubSeqInflate(rebox(seq), rebox(start), rebox(len))
                }
                TypedExpr::FlatMap(_, lambda, seq) => Expr::FlatMap(rebox(lambda), rebox(seq)),
                TypedExpr::FlatMapAccum(_, lambda, acc, vt, seq) => {
                    Expr::FlatMapAccum(rebox(lambda), rebox(acc), vt, rebox(seq))
                }
                TypedExpr::FlatMapList(_, lambda, vt, seq) => {
                    Expr::FlatMapList(rebox(lambda), vt, rebox(seq))
                }
                TypedExpr::Dup(_, count, x) => Expr::Dup(rebox(count), rebox(x)),
                TypedExpr::LiftOption(_, None) => Expr::LiftOption(None),
                TypedExpr::LiftOption(_, Some(x)) => Expr::LiftOption(Some(rebox(x))),
            }
        }
    }

    impl<TypeRep> From<TypedFormat<TypeRep>> for Format {
        fn from(value: TypedFormat<TypeRep>) -> Self {
            match value {
                TypedFormat::FormatCall(_gt, level, t_args, _) => {
                    let args = t_args
                        .into_iter()
                        .map(|(_lbl, arg)| Expr::from(arg))
                        .collect();
                    Format::ItemVar(level, args)
                }
                TypedFormat::SkipRemainder => Format::SkipRemainder,
                TypedFormat::Pos => Format::Pos,
                TypedFormat::Fail => Format::Fail,
                TypedFormat::EndOfInput => Format::EndOfInput,
                TypedFormat::Align(n) => Format::Align(n),
                TypedFormat::Byte(b) => Format::Byte(b),
                TypedFormat::Variant(_, lbl, inner) => Format::Variant(lbl, rebox(inner)),
                TypedFormat::Union(_, branches) => {
                    Format::Union(branches.into_iter().map(Format::from).collect())
                }
                TypedFormat::UnionNondet(_, branches) => {
                    Format::UnionNondet(branches.into_iter().map(Format::from).collect())
                }
                TypedFormat::Tuple(_, elts) => Format::Tuple(revec(elts)),
                TypedFormat::Record(_, flds) => Format::Record(revec_pair(flds)),
                TypedFormat::Repeat(_, inner) => Format::Repeat(rebox(inner)),
                TypedFormat::Repeat1(_, inner) => Format::Repeat1(rebox(inner)),
                TypedFormat::RepeatCount(_, count, inner) => {
                    Format::RepeatCount(Expr::from(count), rebox(inner))
                }
                TypedFormat::RepeatBetween(_, min, max, inner) => {
                    Format::RepeatBetween(Expr::from(min), Expr::from(max), rebox(inner))
                }
                TypedFormat::RepeatUntilLast(_, lambda, inner) => {
                    Format::RepeatUntilLast(Expr::from(lambda), rebox(inner))
                }
                TypedFormat::RepeatUntilSeq(_, lambda, inner) => {
                    Format::RepeatUntilSeq(Expr::from(lambda), rebox(inner))
                }
                TypedFormat::Maybe(_, is_present, inner) => {
                    Format::Maybe(Expr::from(is_present), rebox(inner))
                }
                TypedFormat::ForEach(_, expr, lbl, inner) => {
                    Format::ForEach(Expr::from(expr), lbl, rebox(inner))
                }
                TypedFormat::Peek(_, inner) => Format::Peek(rebox(inner)),
                TypedFormat::PeekNot(_, inner) => Format::PeekNot(rebox(inner)),
                TypedFormat::Slice(_, sz, inner) => Format::Slice(Expr::from(sz), rebox(inner)),
                TypedFormat::Bits(_, inner) => Format::Bits(rebox(inner)),
                TypedFormat::WithRelativeOffset(_, ofs, inner) => {
                    Format::WithRelativeOffset(ofs.into(), rebox(inner))
                }
                TypedFormat::Map(_, inner, lambda) => Format::Map(rebox(inner), Expr::from(lambda)),
                TypedFormat::Where(_, inner, lambda) => {
                    Format::Where(rebox(inner), Expr::from(lambda))
                }
                TypedFormat::Compute(_, expr) => Format::Compute(Expr::from(expr)),
                TypedFormat::Let(_, name, val, inner) => {
                    Format::Let(name, Expr::from(val), rebox(inner))
                }
                TypedFormat::Match(_, head, t_branches) => {
                    let branches = t_branches
                        .into_iter()
                        .map(|(p, f)| (Pattern::from(p), Format::from(f)))
                        .collect();
                    Format::Match(Expr::from(head), branches)
                }
                TypedFormat::Dynamic(_, name, dynf, inner) => {
                    Format::Dynamic(name, DynFormat::from(dynf), rebox(inner))
                }
                TypedFormat::Apply(_, name, _) => Format::Apply(name),
            }
        }
    }

    impl<TypeRep> From<TypedDynFormat<TypeRep>> for DynFormat {
        fn from(value: TypedDynFormat<TypeRep>) -> Self {
            match value {
                TypedDynFormat::Huffman(code_values, opt_code_lengths) => {
                    DynFormat::Huffman(Expr::from(code_values), opt_code_lengths.map(Expr::from))
                }
            }
        }
    }

    impl<TypeRep> From<TypedPattern<TypeRep>> for Pattern {
        fn from(value: TypedPattern<TypeRep>) -> Self {
            match value {
                TypedPattern::Binding(_, name) => Pattern::Binding(name),
                TypedPattern::Wildcard(_) => Pattern::Wildcard,
                TypedPattern::Bool(b) => Pattern::Bool(b),
                TypedPattern::U8(n) => Pattern::U8(n),
                TypedPattern::U16(n) => Pattern::U16(n),
                TypedPattern::U32(n) => Pattern::U32(n),
                TypedPattern::U64(n) => Pattern::U64(n),
                TypedPattern::Int(_, bounds) => Pattern::Int(bounds),
                TypedPattern::Char(c) => Pattern::Char(c),
                TypedPattern::Tuple(_, elts) => Pattern::Tuple(revec(elts)),
                TypedPattern::Variant(_, name, inner) => Pattern::Variant(name, rebox(inner)),
                TypedPattern::Seq(_, elts) => Pattern::Seq(revec(elts)),
                TypedPattern::Option(_, Some(inner)) => Pattern::Option(Some(rebox(inner))),
                TypedPattern::Option(_, None) => Pattern::Option(None),
            }
        }
    }
}
