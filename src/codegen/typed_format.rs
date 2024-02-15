use crate::byte_set::ByteSet;
use crate::{ Arith, DynFormat, Expr, Format, FormatModule, IntRel, Label, Pattern, ValueType };
use crate::typecheck::{ TypeChecker, UType, UVar, VMId };
use super::rust_ast::{ RustType, RustTypeDef };

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GenType {
    Void,
    Inline(RustType),
    Def((usize, Label), RustTypeDef),
}

#[derive(Debug, Clone)]
pub struct IndexTree<'a> {
    top_format: TypedFormat,
    module: &'a FormatModule,
}

// NOTE - we might use this as a generic param later if it seems useful to do so
type VarTypeId = VMId;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TypedFormat {
    ItemVar(usize, Vec<TypedExpr>),
    Fail,
    EndOfInput,
    Align(usize),
    Byte(ByteSet),
    Variant(VarTypeId, Label, Box<TypedFormat>),
    Union(GenType, Vec<TypedFormat>),
    UnionNondet(GenType, Vec<TypedFormat>),


}
