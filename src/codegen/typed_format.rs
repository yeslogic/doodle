use std::borrow::Cow;
use std::ops::Add;
use std::rc::Rc;

use super::rust_ast::{RustType, RustTypeDef};
use super::{AtomType, LocalType, PrimType};
use crate::bounds::Bounds;
use crate::byte_set::ByteSet;
use crate::{Arith, FormatModule, IntRel, Label, ValueType};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum GenType {
    Inline(RustType),
    Def((usize, Label), RustTypeDef),
}

impl GenType {
    pub(crate) fn to_rust_type(self) -> RustType {
        match self {
            GenType::Inline(rt) => rt,
            GenType::Def((ix, lbl), _) => RustType::defined(ix, lbl.clone()),
        }
    }
}

// FIXME - we have to add Hash and Eq impls for HashMap to work properly in typed_decoder
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum TypedFormat<TypeRep> {
    FormatCall(
        TypeRep,
        usize,
        Vec<(Label, TypedExpr<TypeRep>)>,
        Rc<TypedFormat<TypeRep>>,
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
    RepeatUntilLast(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    RepeatUntilSeq(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    Peek(TypeRep, Box<TypedFormat<TypeRep>>),
    PeekNot(TypeRep, Box<TypedFormat<TypeRep>>),
    Slice(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    Bits(TypeRep, Box<TypedFormat<TypeRep>>),
    WithRelativeOffset(TypeRep, TypedExpr<TypeRep>, Box<TypedFormat<TypeRep>>),
    Map(TypeRep, Box<TypedFormat<TypeRep>>, TypedExpr<TypeRep>),
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
}

impl TypedFormat<GenType> {
    pub const EMPTY: Self = TypedFormat::Tuple(GenType::Inline(RustType::UNIT), Vec::new());

    pub(crate) fn expect_rust_type(&self, expected: &RustType) {
        match self {
            TypedFormat::FormatCall(_, _, _, gt_f) => gt_f.expect_rust_type(expected),
            TypedFormat::Fail => unreachable!("TypedFormat::Fail has no equivalent RustType, expected {expected:?}"),
            | TypedFormat::Align(_)
            | TypedFormat::EndOfInput => assert!(matches!(expected, RustType::Atom(AtomType::Prim(PrimType::Unit)))),
            TypedFormat::Byte(_) => assert!(matches!(expected, RustType::Atom(AtomType::Prim(PrimType::U8)))),
            | TypedFormat::Tuple(gt, ..)
            | TypedFormat::Record(gt, ..)
            | TypedFormat::Repeat(gt, ..)
            | TypedFormat::Repeat1(gt, ..)
            | TypedFormat::RepeatCount(gt, ..)
            | TypedFormat::RepeatUntilLast(gt, ..)
            | TypedFormat::RepeatUntilSeq(gt, ..)
            | TypedFormat::Peek(gt, ..)
            | TypedFormat::PeekNot(gt, ..)
            | TypedFormat::Slice(gt, ..)
            | TypedFormat::Bits(gt, ..)
            | TypedFormat::WithRelativeOffset(gt, ..)
            | TypedFormat::Map(gt, ..)
            | TypedFormat::Compute(gt, ..)
            | TypedFormat::Let(gt, ..)
            | TypedFormat::Match(gt, ..)
            | TypedFormat::Dynamic(gt, ..)
            | TypedFormat::Apply(gt, ..)
            | TypedFormat::Union(gt, ..)
            | TypedFormat::UnionNondet(gt, ..)
            | TypedFormat::Variant(gt, ..) => match gt {
                GenType::Inline(actual) => assert_eq!(actual, expected),
                GenType::Def((ix1, lb1), ..) => match expected {
                    RustType::Atom(AtomType::TypeRef(LocalType::LocalDef(ix0, lb0))) => {
                        assert_eq!((ix0, lb0), (ix1, lb1));
                    }
                    _ => unreachable!("actual type GenType::Def({ix1}, {lb1}) != expected type ({expected:?}) [{self:?}]")
                }
            }

        }
    }

    pub(crate) fn match_bounds(&self, module: &FormatModule) -> Bounds {
        match self {
            TypedFormat::FormatCall(_gt, _lvl, _args, def) => def.match_bounds(module),

            TypedFormat::Compute(_, _)
            | TypedFormat::Peek(_, _)
            | TypedFormat::PeekNot(_, _)
            | TypedFormat::EndOfInput
            | TypedFormat::Fail => Bounds::exact(0),

            TypedFormat::Align(n) => Bounds::new(0, Some(n - 1)),
            TypedFormat::Byte(_) => Bounds::exact(1),
            TypedFormat::Variant(_, _, f) => f.match_bounds(module),
            TypedFormat::Union(_, branches) | TypedFormat::UnionNondet(_, branches) => branches
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),
            TypedFormat::Tuple(_, elts) => elts
                .iter()
                .map(|f| f.match_bounds(module))
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),
            TypedFormat::Record(_, flds) => flds
                .iter()
                .map(|(_l, f)| f.match_bounds(module))
                .reduce(<Bounds as Add>::add)
                .unwrap_or(Bounds::exact(0)),

            TypedFormat::RepeatCount(_, t_exp, f) => f.match_bounds(module) * t_exp.bounds(),

            TypedFormat::Repeat1(_, f) | TypedFormat::RepeatUntilLast(_, _, f) => {
                f.match_bounds(module) * Bounds::new(1, None)
            }

            TypedFormat::Repeat(_, _f) | TypedFormat::RepeatUntilSeq(_, _, _f) => {
                Bounds::new(0, None)
            }

            TypedFormat::Slice(_, t_expr, _) => t_expr.bounds(),

            TypedFormat::Bits(_, f) => f.match_bounds(module).bits_to_bytes(),

            TypedFormat::WithRelativeOffset(_, _, _) => Bounds::exact(0),

            TypedFormat::Map(_, f, _)
            | TypedFormat::Dynamic(_, _, _, f)
            | TypedFormat::Let(_, _, _, f) => f.match_bounds(module),

            TypedFormat::Match(_, _, branches) => branches
                .iter()
                .map(|(_, f)| f.match_bounds(module))
                .reduce(Bounds::union)
                .unwrap(),

            TypedFormat::Apply(_, _, _) => Bounds::new(1, None),
        }
    }

    pub(crate) fn is_nullable(&self, module: &FormatModule) -> bool {
        self.match_bounds(module).min == 0
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
            TypedFormat::EndOfInput | TypedFormat::Align(_) => {
                Some(Cow::Owned(GenType::from(RustType::UNIT)))
            }
            TypedFormat::Byte(_) => Some(Cow::Owned(GenType::from(PrimType::U8))),

            TypedFormat::FormatCall(gt, ..)
            | TypedFormat::Variant(gt, ..)
            | TypedFormat::Union(gt, ..)
            | TypedFormat::UnionNondet(gt, ..)
            | TypedFormat::Tuple(gt, ..)
            | TypedFormat::Record(gt, ..)
            | TypedFormat::Repeat(gt, ..)
            | TypedFormat::Repeat1(gt, ..)
            | TypedFormat::RepeatCount(gt, ..)
            | TypedFormat::RepeatUntilLast(gt, ..)
            | TypedFormat::RepeatUntilSeq(gt, ..)
            | TypedFormat::Peek(gt, ..)
            | TypedFormat::PeekNot(gt, ..)
            | TypedFormat::Slice(gt, ..)
            | TypedFormat::Bits(gt, ..)
            | TypedFormat::WithRelativeOffset(gt, ..)
            | TypedFormat::Map(gt, ..)
            | TypedFormat::Compute(gt, ..)
            | TypedFormat::Let(gt, ..)
            | TypedFormat::Match(gt, ..)
            | TypedFormat::Dynamic(gt, ..)
            | TypedFormat::Apply(gt, ..) => Some(Cow::Borrowed(gt)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypedDynFormat<TypeRep> {
    Huffman(TypedExpr<TypeRep>, Option<TypedExpr<TypeRep>>),
}

// FIXME - same as TypedFormat, Eq+Hash required transitively by HashMap preconditions
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    FlatMap(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedExpr<TypeRep>>),
    FlatMapAccum(
        TypeRep,
        Box<TypedExpr<TypeRep>>,
        Box<TypedExpr<TypeRep>>,
        ValueType,
        Box<TypedExpr<TypeRep>>,
    ),
    Dup(TypeRep, Box<TypedExpr<TypeRep>>, Box<TypedExpr<TypeRep>>),
    Inflate(TypeRep, Box<TypedExpr<TypeRep>>),
}

impl<TypeRep> TypedExpr<TypeRep> {
    fn bounds(&self) -> Bounds {
        match self {
            TypedExpr::U8(n) => Bounds::exact(usize::from(*n)),
            TypedExpr::U16(n) => Bounds::exact(usize::from(*n)),
            TypedExpr::U32(n) => Bounds::exact(*n as usize),
            TypedExpr::U64(n) => Bounds::exact(*n as usize),
            TypedExpr::Arith(_t, Arith::Add, a, b) => a.bounds() + b.bounds(),
            TypedExpr::Arith(_t, Arith::Mul, a, b) => a.bounds() * b.bounds(),
            _ => Bounds::new(0, None),
        }
    }
}

// FIXME - same as TypedExpr, requirements of HashMap include Eq and Hash for this type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypedPattern<TypeRep> {
    Binding(TypeRep, Label),
    Wildcard(TypeRep),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Char(char),
    Tuple(TypeRep, Vec<TypedPattern<TypeRep>>),
    Variant(TypeRep, Label, Box<TypedPattern<TypeRep>>),
    Seq(TypeRep, Vec<TypedPattern<TypeRep>>),
}

mod __impls {
    use super::GenType;
    use crate::codegen::{
        rust_ast::{AtomType, CompType, PrimType, RustType, RustTypeDef},
        IxLabel,
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
}
