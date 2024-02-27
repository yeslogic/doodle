use std::rc::Rc;

use super::rust_ast::{RustType, RustTypeDef};
use crate::byte_set::ByteSet;
use crate::typecheck::VMId;
use crate::{Arith, FormatModule, IntRel, Label, ValueType};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GenType {
    Void,
    Inline(RustType),
    Def((usize, Label), RustTypeDef),
}

impl GenType {
    pub(crate) fn to_rust_type(self) -> RustType {
        match self {
            GenType::Void => unreachable!("encountered GenType::Void in to_rust_type call"),
            GenType::Inline(rt) => rt,
            GenType::Def((ix, lbl), _) => RustType::defined(ix, lbl.clone()),
        }
    }
}

#[derive(Debug)]
pub struct IndexTree<'a, TypeRep> {
    top_format: TypedFormat<TypeRep>,
    module: &'a FormatModule,
}

// NOTE - we might use this as a generic param later if it seems useful to do so
type VarTypeId = VMId;

#[derive(Clone, Debug, PartialEq)]
pub enum TypedFormat<TypeRep> {
    FormatCall(
        TypeRep,
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
    Apply(TypeRep, Rc<TypedDynFormat<TypeRep>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypedDynFormat<TypeRep> {
    Huffman(TypedExpr<TypeRep>, Option<TypedExpr<TypeRep>>),
}

#[derive(Clone, Debug, PartialEq)]
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

    IntRel(TypeRep, IntRel, Box<TypedExpr<TypeRep>>, Box<TypedExpr<TypeRep>>),
    Arith(TypeRep, Arith, Box<TypedExpr<TypeRep>>, Box<TypedExpr<TypeRep>>),

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

#[derive(Clone, Debug, PartialEq)]
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
    use crate::{
        codegen::rust_ast::{AtomType, CompType, PrimType, RustType, RustTypeDef},
        Label,
    };

    impl From<RustType> for GenType {
        fn from(value: RustType) -> Self {
            GenType::Inline(value)
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
