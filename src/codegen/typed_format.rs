use std::borrow::Cow;
use std::ops::Add;
use std::rc::Rc;

use super::rust_ast::{PrimType, RustType, RustTypeDecl};
use super::{AtomType, LocalType};
use crate::bounds::Bounds;
use crate::byte_set::ByteSet;
use crate::codegen::rust_ast::{RustLt, RustParams, UseParams};
use crate::{Arith, BaseKind, Endian, IntRel, Label, StyleHint, TypeHint, UnaryOp};

pub(crate) mod variables;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum GenType {
    Inline(RustType),
    Def((usize, Label), RustTypeDecl),
}

impl GenType {
    pub(crate) fn to_rust_type(&self) -> RustType {
        match self {
            GenType::Inline(rt) => rt.clone(),
            GenType::Def((ix, lbl), RustTypeDecl { lt, .. }) => {
                let params = match lt {
                    Some(lt) => RustParams {
                        lt_params: vec![lt.clone()],
                        ..Default::default()
                    },
                    None => Default::default(),
                };
                RustType::defined(*ix, lbl.clone(), params)
            }
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
    pub(crate) fn try_as_adhoc(&self) -> Option<(usize, &Label, Option<Box<UseParams>>)> {
        match self {
            GenType::Def((ix, lbl), RustTypeDecl { lt, .. }) => Some((
                *ix,
                lbl,
                lt.clone().map(|lt| Box::new(RustParams::from_lt(lt))),
            )),
            GenType::Inline(RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(
                ix,
                lbl,
                params,
            )))) => Some((*ix, lbl, params.clone())),
            _ => None,
        }
    }

    /// Determines whether a given [`GenType`] implements the `Copy` trait.
    pub(crate) fn is_copy(&self) -> bool {
        match self {
            GenType::Inline(rust_type) => rust_type.can_be_copy(),
            // TODO - infer recursive Copy of local definitions, if possible
            GenType::Def(_, rust_type_decl) => rust_type_decl.def.can_be_copy(),
        }
    }

    pub(crate) fn lt_param(&self) -> Option<&RustLt> {
        match self {
            GenType::Inline(rust_type) => rust_type.lt_param(),
            GenType::Def(_, rust_type_decl) => rust_type_decl.lt_param(),
        }
    }
}

impl<TypeRep> std::hash::Hash for TypedFormat<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let disc = core::mem::discriminant(self);
        disc.hash(state);
        match self {
            TypedFormat::FormatCall(_, level, args, views, _) => {
                level.hash(state);
                args.hash(state);
                views.hash(state);
            }
            TypedFormat::SkipRemainder
            | TypedFormat::Pos
            | TypedFormat::Fail
            | TypedFormat::EndOfInput => {}
            TypedFormat::DecodeBytes(_, expr, inner) => {
                expr.hash(state);
                inner.hash(state);
            }
            TypedFormat::Phantom(_, inner) => inner.hash(state),
            TypedFormat::ParseFromView(_, view, inner) => {
                view.hash(state);
                inner.hash(state);
            }
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
            // REVIEW - do we want to dodge collision between Tuple and Sequence with a salt, or is this okay?
            TypedFormat::Sequence(_, elts) => elts.hash(state),
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
            TypedFormat::AccumUntil(_, f, g, init, _, inner) => {
                f.hash(state);
                g.hash(state);
                init.hash(state);
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
            TypedFormat::WithRelativeOffset(_, base_addr, ofs, inner) => {
                base_addr.hash(state);
                ofs.hash(state);
                inner.hash(state);
            }
            TypedFormat::Map(_, orig, f) | TypedFormat::Where(_, orig, f) => {
                orig.hash(state);
                f.hash(state);
            }
            TypedFormat::Compute(_, expr) => expr.hash(state),
            TypedFormat::LetView(_, lb, inner) => {
                lb.hash(state);
                inner.hash(state);
            }
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
            TypedFormat::LetFormat(_, f0, lbl, f) => {
                f0.hash(state);
                lbl.hash(state);
                f.hash(state);
            }
            TypedFormat::MonadSeq(_, f0, f) => {
                f0.hash(state);
                f.hash(state);
            }
            TypedFormat::Hint(_, hint, f) => {
                hint.hash(state);
                f.hash(state);
            }
            TypedFormat::LiftedOption(_, opt_f) => opt_f.hash(state),
            TypedFormat::WithView(_, ident, vf) => {
                ident.hash(state);
                vf.hash(state);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedFormat<TypeRep> {
    FormatCall(
        TypeRep,
        usize,
        Vec<(Label, TypedExpr<TypeRep>)>,
        Option<Vec<(Label, TypedViewExpr<TypeRep>)>>,
        Rc<TypedFormat<TypeRep>>,
    ),
    ForEach(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
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
    Sequence(TypeRep, Vec<TypedFormat<TypeRep>>),
    Repeat(TypeRep, Box<TypedFormat<TypeRep>>),
    Repeat1(TypeRep, Box<TypedFormat<TypeRep>>),
    RepeatCount(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedFormat<TypeRep>>),
    RepeatBetween(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedFormat<TypeRep>>,
    ),
    RepeatUntilLast(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedFormat<TypeRep>>),
    RepeatUntilSeq(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedFormat<TypeRep>>),
    Maybe(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedFormat<TypeRep>>),
    Peek(TypeRep, Box<TypedFormat<TypeRep>>),
    PeekNot(TypeRep, Box<TypedFormat<TypeRep>>),
    Slice(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedFormat<TypeRep>>),
    Bits(TypeRep, Box<TypedFormat<TypeRep>>),
    WithRelativeOffset(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedFormat<TypeRep>>,
    ),
    Map(TypeRep, Box<TypedFormat<TypeRep>>, Box<TypedExpr<TypeRep>>),
    Where(TypeRep, Box<TypedFormat<TypeRep>>, Box<TypedExpr<TypeRep>>),
    Compute(TypeRep, Box<TypedExpr<TypeRep>>),
    Let(
        TypeRep,
        Label,
        Box<TypedExpr<TypeRep>>,
        Box<TypedFormat<TypeRep>>,
    ),
    Match(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
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
    DecodeBytes(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedFormat<TypeRep>>),
    ParseFromView(TypeRep, TypedViewExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    LetFormat(
        TypeRep,
        Box<TypedFormat<TypeRep>>,
        Label,
        Box<TypedFormat<TypeRep>>,
    ),
    MonadSeq(
        TypeRep,
        Box<TypedFormat<TypeRep>>,
        Box<TypedFormat<TypeRep>>,
    ),
    Hint(TypeRep, StyleHint, Box<TypedFormat<TypeRep>>),
    AccumUntil(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        TypeHint,
        Box<TypedFormat<TypeRep>>,
    ),
    LiftedOption(TypeRep, Option<Box<TypedFormat<TypeRep>>>),
    LetView(TypeRep, Label, Box<TypedFormat<TypeRep>>),
    WithView(TypeRep, TypedViewExpr<TypeRep>, TypedViewFormat<TypeRep>),
    Phantom(TypeRep, Box<TypedFormat<TypeRep>>),
}

impl TypedFormat<GenType> {
    pub const EMPTY: Self = TypedFormat::Tuple(GenType::Inline(RustType::UNIT), Vec::new());

    pub(crate) fn lookahead_bounds(&self) -> Bounds {
        match self {
            TypedFormat::FormatCall(_gt, _lvl, _args, _views, def) => def.lookahead_bounds(),

            TypedFormat::DecodeBytes(_, _, _)
            | TypedFormat::SkipRemainder
            | TypedFormat::Pos
            | TypedFormat::Compute(_, _)
            | TypedFormat::EndOfInput
            | TypedFormat::ParseFromView(_, _, _)
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
            // REVIEW - we have a more sophisticated algorithm in Format::lookahead_bounds for Sequence, should we use that here?
            TypedFormat::Tuple(_, elts) | TypedFormat::Sequence(_, elts) => elts
                .iter()
                .map(TypedFormat::lookahead_bounds)
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),
            TypedFormat::RepeatCount(_, t_exp, f) => f.lookahead_bounds() * t_exp.bounds(),
            TypedFormat::RepeatBetween(_, t_min, t_max, f) => {
                f.lookahead_bounds() * Bounds::union(t_min.bounds(), t_max.bounds())
            }

            TypedFormat::Repeat1(_, f) | TypedFormat::RepeatUntilLast(_, _, f) => {
                f.lookahead_bounds() * Bounds::at_least(1)
            }

            TypedFormat::Repeat(_, _f)
            | TypedFormat::RepeatUntilSeq(_, _, _f)
            | TypedFormat::AccumUntil(.., _f) => Bounds::any(),
            // REVIEW - can we do any better than this?
            TypedFormat::ForEach(_, _expr, _lbl, _f) => Bounds::any(),
            TypedFormat::Maybe(_, _, f) => Bounds::union(Bounds::exact(0), f.lookahead_bounds()),

            TypedFormat::Slice(_, t_expr, _) => t_expr.bounds(),

            TypedFormat::Bits(_, f) => f.lookahead_bounds().bits_to_bytes(),

            TypedFormat::WithRelativeOffset(_, _base_addr_expr, _offset_expr, _inner) => {
                Bounds::any()
            }

            TypedFormat::Map(_, f, _)
            | TypedFormat::Where(_, f, _)
            | TypedFormat::Dynamic(_, _, _, f)
            | TypedFormat::Let(_, _, _, f)
            | TypedFormat::LetView(_, _, f) => f.lookahead_bounds(),

            TypedFormat::Match(_, _, branches) => branches
                .iter()
                .map(|(_, f)| f.lookahead_bounds())
                .reduce(Bounds::union)
                .unwrap(),

            TypedFormat::Apply(_, _, _) => Bounds::at_least(1),
            TypedFormat::LetFormat(_, f0, _, f) | TypedFormat::MonadSeq(_, f0, f) => Bounds::union(
                f0.lookahead_bounds(),
                f0.match_bounds() + f.lookahead_bounds(),
            ),
            TypedFormat::Hint(.., inner) => inner.lookahead_bounds(),
            TypedFormat::LiftedOption(_, f) => f
                .as_ref()
                .map_or(Bounds::exact(0), |f| f.lookahead_bounds()),
            // REVIEW[epic=view-format] - is this correct?
            TypedFormat::WithView(_, _ident, _vf) => Bounds::exact(0),
            TypedFormat::Phantom(..) => Bounds::exact(0),
        }
    }

    pub(crate) fn match_bounds(&self) -> Bounds {
        match self {
            TypedFormat::FormatCall(_gt, _lvl, _args, _views, def) => def.match_bounds(),

            TypedFormat::DecodeBytes(_, _, _)
            | TypedFormat::ParseFromView(_, _, _)
            | TypedFormat::Compute(_, _)
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
            TypedFormat::Sequence(_, elts) | TypedFormat::Tuple(_, elts) => elts
                .iter()
                .map(TypedFormat::match_bounds)
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),
            TypedFormat::RepeatCount(_, t_exp, f) => f.match_bounds() * t_exp.bounds(),
            TypedFormat::RepeatBetween(_, t_min, t_max, f) => {
                f.match_bounds() * Bounds::union(t_min.bounds(), t_max.bounds())
            }

            TypedFormat::Repeat1(_, f) | TypedFormat::RepeatUntilLast(_, _, f) => {
                f.match_bounds() * Bounds::at_least(1)
            }

            TypedFormat::Repeat(_, _f)
            | TypedFormat::RepeatUntilSeq(_, _, _f)
            | TypedFormat::AccumUntil(.., _f) => Bounds::any(),
            // REVIEW - can we do any better than this?
            TypedFormat::ForEach(_, _expr, _lbl, _f) => Bounds::any(),
            TypedFormat::Maybe(_, _, f) => Bounds::union(Bounds::exact(0), f.match_bounds()),

            TypedFormat::SkipRemainder => Bounds::any(),

            TypedFormat::Slice(_, t_expr, _) => t_expr.bounds(),

            TypedFormat::Bits(_, f) => f.match_bounds().bits_to_bytes(),

            TypedFormat::WithRelativeOffset(_, _, _, _) => Bounds::exact(0),

            TypedFormat::Map(_, f, _)
            | TypedFormat::Where(_, f, _)
            | TypedFormat::Dynamic(_, _, _, f)
            | TypedFormat::Let(_, _, _, f)
            | TypedFormat::LetView(_, _, f) => f.match_bounds(),

            TypedFormat::Match(_, _, branches) => branches
                .iter()
                .map(|(_, f)| f.match_bounds())
                .reduce(Bounds::union)
                .unwrap(),

            TypedFormat::Apply(_, _, _) => Bounds::at_least(1),
            TypedFormat::LetFormat(_, f0, _, f1) | TypedFormat::MonadSeq(_, f0, f1) => {
                f0.match_bounds() + f1.match_bounds()
            }
            TypedFormat::Hint(.., inner) => inner.match_bounds(),
            TypedFormat::LiftedOption(_, f) => {
                f.as_ref().map_or(Bounds::exact(0), |f| f.match_bounds())
            }
            // REVIEW[epic=view-format] - is this correct?
            TypedFormat::WithView(_, _ident, _vf) => Bounds::exact(0),
            TypedFormat::Phantom(..) => Bounds::exact(0),
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
            TypedFormat::SkipRemainder
            | TypedFormat::EndOfInput
            | TypedFormat::Align(_)
            | TypedFormat::Phantom(..) => Some(Cow::Owned(GenType::from(RustType::UNIT))),
            TypedFormat::Byte(_) => Some(Cow::Owned(GenType::from(PrimType::U8))),
            // REVIEW - forcing Pos to be a U64-valued format
            TypedFormat::Pos => Some(Cow::Owned(GenType::from(PrimType::U64))),

            TypedFormat::LetFormat(gt, ..)
            | TypedFormat::MonadSeq(gt, ..)
            | TypedFormat::Hint(gt, ..)
            | TypedFormat::DecodeBytes(gt, ..)
            | TypedFormat::ParseFromView(gt, ..)
            | TypedFormat::FormatCall(gt, ..)
            | TypedFormat::Variant(gt, ..)
            | TypedFormat::Union(gt, ..)
            | TypedFormat::UnionNondet(gt, ..)
            | TypedFormat::Tuple(gt, ..)
            | TypedFormat::Sequence(gt, ..)
            | TypedFormat::Repeat(gt, ..)
            | TypedFormat::Repeat1(gt, ..)
            | TypedFormat::ForEach(gt, ..)
            | TypedFormat::RepeatCount(gt, ..)
            | TypedFormat::RepeatBetween(gt, ..)
            | TypedFormat::RepeatUntilLast(gt, ..)
            | TypedFormat::RepeatUntilSeq(gt, ..)
            | TypedFormat::AccumUntil(gt, ..)
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
            | TypedFormat::LetView(gt, ..)
            | TypedFormat::WithView(gt, ..)
            | TypedFormat::Match(gt, ..)
            | TypedFormat::Dynamic(gt, ..)
            | TypedFormat::LiftedOption(gt, ..)
            | TypedFormat::Apply(gt, ..) => Some(Cow::Borrowed(gt)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedDynFormat<TypeRep> {
    Huffman(Box<TypedExpr<TypeRep>>, Option<Box<TypedExpr<TypeRep>>>),
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
pub enum TypedViewFormat<TypeRep> {
    CaptureBytes(Box<TypedExpr<TypeRep>>),
    ReadArray(Box<TypedExpr<TypeRep>>, BaseKind<Endian>),
    ReifyView,
}

impl<TypeRep> std::hash::Hash for TypedViewFormat<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            TypedViewFormat::CaptureBytes(len) => {
                len.hash(state);
            }
            TypedViewFormat::ReadArray(len, kind) => {
                len.hash(state);
                kind.hash(state);
            }
            TypedViewFormat::ReifyView => (),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedViewExpr<TypeRep> {
    Var(Label),
    Offset(Box<TypedViewExpr<TypeRep>>, Box<TypedExpr<TypeRep>>),
}

impl<TypeRep> std::hash::Hash for TypedViewExpr<TypeRep> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            TypedViewExpr::Var(l) => l.hash(state),
            TypedViewExpr::Offset(base, offs) => {
                base.hash(state);
                offs.hash(state);
            }
        }
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for crate::Label {}
    impl Sealed for u32 {}
}

pub trait Ident:
    Clone
    + std::fmt::Debug
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::hash::Hash
    + 'static
    + private::Sealed
{
}

impl Ident for Label {}
impl Ident for u32 {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypedExpr<TypeRep, VarId = Label>
where
    VarId: Ident,
{
    Var(TypeRep, VarId),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Tuple(TypeRep, Vec<TypedExpr<TypeRep, VarId>>),
    TupleProj(TypeRep, Box<TypedExpr<TypeRep, VarId>>, usize),
    Record(TypeRep, Vec<(Label, TypedExpr<TypeRep, VarId>)>),
    RecordProj(TypeRep, Box<TypedExpr<TypeRep, VarId>>, Label),
    Variant(TypeRep, Label, Box<TypedExpr<TypeRep, VarId>>),
    Seq(TypeRep, Vec<TypedExpr<TypeRep, VarId>>),
    Match(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Vec<(TypedPattern<TypeRep>, TypedExpr<TypeRep, VarId>)>,
    ),
    Lambda((TypeRep, TypeRep), Label, Box<TypedExpr<TypeRep, VarId>>),
    IntRel(
        TypeRep,
        IntRel,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    Arith(
        TypeRep,
        Arith,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),

    AsU8(Box<TypedExpr<TypeRep, VarId>>),
    AsU16(Box<TypedExpr<TypeRep, VarId>>),
    AsU32(Box<TypedExpr<TypeRep, VarId>>),
    AsU64(Box<TypedExpr<TypeRep, VarId>>),
    AsChar(Box<TypedExpr<TypeRep, VarId>>),

    U16Be(Box<TypedExpr<TypeRep, VarId>>),
    U16Le(Box<TypedExpr<TypeRep, VarId>>),
    U32Be(Box<TypedExpr<TypeRep, VarId>>),
    U32Le(Box<TypedExpr<TypeRep, VarId>>),
    U64Be(Box<TypedExpr<TypeRep, VarId>>),
    U64Le(Box<TypedExpr<TypeRep, VarId>>),

    SeqLength(Box<TypedExpr<TypeRep, VarId>>),
    SeqIx(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    SubSeq(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    SubSeqInflate(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    FlatMap(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    FlatMapAccum(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
        TypeHint,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    LeftFold(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
        TypeHint,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    FindByKey(
        TypeRep,
        bool,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    FlatMapList(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        TypeHint,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    Dup(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    EnumFromTo(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    LiftOption(TypeRep, Option<Box<TypedExpr<TypeRep, VarId>>>),
    Unary(TypeRep, UnaryOp, Box<TypedExpr<TypeRep, VarId>>),
    Destructure(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        TypedPattern<TypeRep>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
    Append(
        TypeRep,
        Box<TypedExpr<TypeRep, VarId>>,
        Box<TypedExpr<TypeRep, VarId>>,
    ),
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
            TypedExpr::Destructure(_, head, pat, expr) => {
                head.hash(state);
                pat.hash(state);
                expr.hash(state);
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
            TypedExpr::Unary(_, op, inner) => {
                op.hash(state);
                inner.hash(state);
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
            TypedExpr::SeqIx(_, sq, ix) => {
                sq.hash(state);
                ix.hash(state);
            }
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
            TypedExpr::LeftFold(_, f, acc, _vt, seq) => {
                f.hash(state);
                acc.hash(state);
                seq.hash(state);
            }
            TypedExpr::FindByKey(_, is_sorted, f, key, seq) => {
                is_sorted.hash(state);
                f.hash(state);
                key.hash(state);
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
            TypedExpr::EnumFromTo(_, from, to) => {
                from.hash(state);
                to.hash(state);
            }
            TypedExpr::LiftOption(_, opt) => opt.hash(state),
            TypedExpr::Append(_, lhs, rhs) => {
                lhs.hash(state);
                rhs.hash(state);
            }
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
    /// Returns the `GenType` associated with `self`.
    ///
    /// Returns `None` if and only if `self` is `TypedExpr::Lambda`.
    pub(crate) fn get_type(&self) -> Option<Cow<'_, GenType>> {
        match self {
            TypedExpr::Lambda(..) => None,

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
            TypedExpr::Var(gt, ..)
            | TypedExpr::Tuple(gt, ..)
            | TypedExpr::TupleProj(gt, ..)
            | TypedExpr::Record(gt, ..)
            | TypedExpr::RecordProj(gt, ..)
            | TypedExpr::Variant(gt, ..)
            | TypedExpr::Seq(gt, ..)
            | TypedExpr::SeqIx(gt, ..)
            | TypedExpr::Match(gt, ..)
            | TypedExpr::Destructure(gt, ..)
            | TypedExpr::IntRel(gt, ..)
            | TypedExpr::Arith(gt, ..)
            | TypedExpr::Unary(gt, ..)
            | TypedExpr::SubSeq(gt, ..)
            | TypedExpr::SubSeqInflate(gt, ..)
            | TypedExpr::Append(gt, ..)
            | TypedExpr::FlatMap(gt, ..)
            | TypedExpr::FlatMapAccum(gt, ..)
            | TypedExpr::LeftFold(gt, ..)
            | TypedExpr::FindByKey(gt, ..)
            | TypedExpr::FlatMapList(gt, ..)
            | TypedExpr::LiftOption(gt, ..)
            | TypedExpr::Dup(gt, ..)
            | TypedExpr::EnumFromTo(gt, ..) => Some(Cow::Borrowed(gt)),
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

impl TypedPattern<GenType> {
    pub(crate) fn get_type(&self) -> Cow<'_, GenType> {
        match self {
            TypedPattern::U8(..) => Cow::Owned(GenType::from(PrimType::U8)),
            TypedPattern::U16(..) => Cow::Owned(GenType::from(PrimType::U16)),
            TypedPattern::U32(..) => Cow::Owned(GenType::from(PrimType::U32)),
            TypedPattern::U64(..) => Cow::Owned(GenType::from(PrimType::U64)),
            TypedPattern::Char(..) => Cow::Owned(GenType::from(PrimType::Char)),
            TypedPattern::Bool(..) => Cow::Owned(GenType::from(PrimType::Bool)),

            TypedPattern::Wildcard(gt)
            | TypedPattern::Binding(gt, ..)
            | TypedPattern::Tuple(gt, ..)
            | TypedPattern::Option(gt, ..)
            | TypedPattern::Int(gt, ..)
            | TypedPattern::Variant(gt, ..)
            | TypedPattern::Seq(gt, ..) => Cow::Borrowed(gt),
        }
    }
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
    use super::{GenType, TypedDynFormat, TypedExpr, TypedFormat, TypedPattern, TypedViewFormat};
    use crate::{
        DynFormat, Expr, Format, Pattern, ViewExpr, ViewFormat,
        codegen::{
            IxLabel,
            rust_ast::{AtomType, CompType, PrimType, RustType, RustTypeDecl},
            typed_format::TypedViewExpr,
        },
    };

    impl From<RustType> for GenType {
        fn from(value: RustType) -> Self {
            GenType::Inline(value)
        }
    }

    impl From<(IxLabel, RustTypeDecl)> for GenType {
        fn from(value: (IxLabel, RustTypeDecl)) -> Self {
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

    #[allow(clippy::boxed_local)]
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
                TypedExpr::Destructure(_, head, pattern, expr) => {
                    Expr::Destructure(rebox(head), pattern.into(), rebox(expr))
                }
                TypedExpr::Lambda(_, name, inner) => Expr::Lambda(name, rebox(inner)),
                TypedExpr::IntRel(_, rel, x, y) => Expr::IntRel(rel, rebox(x), rebox(y)),
                TypedExpr::Arith(_, op, x, y) => Expr::Arith(op, rebox(x), rebox(y)),
                TypedExpr::Unary(_, op, x) => Expr::Unary(op, rebox(x)),
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
                TypedExpr::SeqIx(_, seq, index) => Expr::SeqIx(rebox(seq), rebox(index)),
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
                TypedExpr::LeftFold(_, lambda, acc, vt, seq) => {
                    Expr::LeftFold(rebox(lambda), rebox(acc), vt, rebox(seq))
                }
                TypedExpr::FindByKey(_, is_sorted, lambda, key, seq) => {
                    Expr::FindByKey(is_sorted, rebox(lambda), rebox(key), rebox(seq))
                }
                TypedExpr::FlatMapList(_, lambda, vt, seq) => {
                    Expr::FlatMapList(rebox(lambda), vt, rebox(seq))
                }
                TypedExpr::Dup(_, count, x) => Expr::Dup(rebox(count), rebox(x)),
                TypedExpr::Append(_, lhs, rhs) => Expr::Append(rebox(lhs), rebox(rhs)),
                TypedExpr::EnumFromTo(_, start, stop) => {
                    Expr::EnumFromTo(rebox(start), rebox(stop))
                }
                TypedExpr::LiftOption(_, None) => Expr::LiftOption(None),
                TypedExpr::LiftOption(_, Some(x)) => Expr::LiftOption(Some(rebox(x))),
            }
        }
    }

    impl<TypeRep> From<TypedFormat<TypeRep>> for Format {
        fn from(value: TypedFormat<TypeRep>) -> Self {
            match value {
                TypedFormat::FormatCall(_gt, level, t_args, t_views, _) => {
                    let args = t_args
                        .into_iter()
                        .map(|(_lbl, arg)| Expr::from(arg))
                        .collect();
                    let views = t_views.map(|views| {
                        views
                            .into_iter()
                            .map(|(_lbl, view)| ViewExpr::from(view))
                            .collect()
                    });
                    Format::ItemVar(level, args, views)
                }
                TypedFormat::DecodeBytes(_, expr, inner) => {
                    Format::DecodeBytes(rebox(expr), rebox(inner))
                }
                TypedFormat::ParseFromView(_, view, inner) => {
                    Format::ParseFromView(ViewExpr::from(view), rebox(inner))
                }
                TypedFormat::SkipRemainder => Format::SkipRemainder,
                TypedFormat::Pos => Format::Pos,
                TypedFormat::Fail => Format::Fail,
                TypedFormat::EndOfInput => Format::EndOfInput,
                TypedFormat::Align(n) => Format::Align(n),
                TypedFormat::Byte(b) => Format::Byte(b),
                TypedFormat::Variant(_, lbl, inner) => Format::Variant(lbl, rebox(inner)),
                TypedFormat::LetFormat(_, f0, name, f) => {
                    Format::LetFormat(rebox(f0), name, rebox(f))
                }
                TypedFormat::MonadSeq(_, f0, f) => Format::MonadSeq(rebox(f0), rebox(f)),
                TypedFormat::Hint(_, hint, f) => Format::Hint(hint.clone(), rebox(f)),
                TypedFormat::Union(_, branches) => {
                    Format::Union(branches.into_iter().map(Format::from).collect())
                }
                TypedFormat::UnionNondet(_, branches) => {
                    Format::UnionNondet(branches.into_iter().map(Format::from).collect())
                }
                TypedFormat::Tuple(_, elts) => Format::Tuple(revec(elts)),
                TypedFormat::Sequence(_, elts) => Format::Sequence(revec(elts)),
                TypedFormat::Repeat(_, inner) => Format::Repeat(rebox(inner)),
                TypedFormat::Repeat1(_, inner) => Format::Repeat1(rebox(inner)),
                TypedFormat::RepeatCount(_, count, inner) => {
                    Format::RepeatCount(rebox(count), rebox(inner))
                }
                TypedFormat::RepeatBetween(_, min, max, inner) => {
                    Format::RepeatBetween(rebox(min), rebox(max), rebox(inner))
                }
                TypedFormat::RepeatUntilLast(_, lambda, inner) => {
                    Format::RepeatUntilLast(rebox(lambda), rebox(inner))
                }
                TypedFormat::RepeatUntilSeq(_, lambda, inner) => {
                    Format::RepeatUntilSeq(rebox(lambda), rebox(inner))
                }
                TypedFormat::AccumUntil(_, cond, update, init, vt, inner) => {
                    Format::AccumUntil(rebox(cond), rebox(update), rebox(init), vt, rebox(inner))
                }
                TypedFormat::Maybe(_, is_present, inner) => {
                    Format::Maybe(rebox(is_present), rebox(inner))
                }
                TypedFormat::ForEach(_, expr, lbl, inner) => {
                    Format::ForEach(rebox(expr), lbl, rebox(inner))
                }
                TypedFormat::Peek(_, inner) => Format::Peek(rebox(inner)),
                TypedFormat::PeekNot(_, inner) => Format::PeekNot(rebox(inner)),
                TypedFormat::Slice(_, sz, inner) => Format::Slice(rebox(sz), rebox(inner)),
                TypedFormat::Bits(_, inner) => Format::Bits(rebox(inner)),
                TypedFormat::WithRelativeOffset(_, base_addr, ofs, inner) => {
                    Format::WithRelativeOffset(rebox(base_addr), rebox(ofs), rebox(inner))
                }
                TypedFormat::Map(_, inner, lambda) => Format::Map(rebox(inner), rebox(lambda)),
                TypedFormat::Where(_, inner, lambda) => Format::Where(rebox(inner), rebox(lambda)),
                TypedFormat::Compute(_, expr) => Format::Compute(rebox(expr)),
                TypedFormat::Let(_, name, val, inner) => {
                    Format::Let(name, rebox(val), rebox(inner))
                }
                TypedFormat::LetView(_, name, inner) => Format::LetView(name, rebox(inner)),
                TypedFormat::WithView(_, view, vf) => {
                    Format::WithView(ViewExpr::from(view), ViewFormat::from(vf))
                }
                TypedFormat::Match(_, head, t_branches) => {
                    let branches = t_branches
                        .into_iter()
                        .map(|(p, f)| (Pattern::from(p), Format::from(f)))
                        .collect();
                    Format::Match(rebox(head), branches)
                }
                TypedFormat::Dynamic(_, name, dynf, inner) => {
                    Format::Dynamic(name, DynFormat::from(dynf), rebox(inner))
                }
                TypedFormat::Apply(_, name, _) => Format::Apply(name),
                TypedFormat::LiftedOption(_, inner) => Format::LiftedOption(inner.map(rebox)),
                TypedFormat::Phantom(_, inner) => Format::Phantom(rebox(inner)),
            }
        }
    }

    impl<TypeRep> From<TypedDynFormat<TypeRep>> for DynFormat {
        fn from(value: TypedDynFormat<TypeRep>) -> Self {
            match value {
                TypedDynFormat::Huffman(code_values, opt_code_lengths) => {
                    DynFormat::Huffman(rebox(code_values), opt_code_lengths.map(rebox))
                }
            }
        }
    }

    impl<TypeRep> From<TypedViewFormat<TypeRep>> for ViewFormat {
        fn from(value: TypedViewFormat<TypeRep>) -> Self {
            match value {
                TypedViewFormat::CaptureBytes(len) => ViewFormat::CaptureBytes(rebox(len)),
                TypedViewFormat::ReadArray(len, kind) => ViewFormat::ReadArray(rebox(len), kind),
                TypedViewFormat::ReifyView => ViewFormat::ReifyView,
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

    impl<TypeRep> From<TypedViewExpr<TypeRep>> for ViewExpr {
        fn from(value: TypedViewExpr<TypeRep>) -> Self {
            match value {
                TypedViewExpr::Var(ident) => ViewExpr::Var(ident),
                TypedViewExpr::Offset(base, offs) => ViewExpr::Offset(rebox(base), rebox(offs)),
            }
        }
    }
}
