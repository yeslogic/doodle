use crate::byte_set::ByteSet;
use crate::{Arith, Expr, Format, IntRel, IntoLabel, Pattern, ValueType};

pub fn var<Name: IntoLabel>(name: Name) -> Expr {
    Expr::Var(name.into())
}

pub fn lambda<Name: IntoLabel>(name: Name, body: Expr) -> Expr {
    Expr::Lambda(name.into(), Box::new(body))
}

pub fn variant<Name: IntoLabel>(name: Name, value: Expr) -> Expr {
    Expr::Variant(name.into(), Box::new(value))
}

pub fn bind<Name: IntoLabel>(name: Name) -> Pattern {
    Pattern::binding(name)
}

pub fn tuple(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Tuple(formats.into_iter().collect())
}

pub fn alts<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
    Format::Union(
        (fields.into_iter())
            .map(|(label, format)| Format::Variant(label.into(), Box::new(format)))
            .collect(),
    )
}

pub fn expr_match(head: Expr, branches: impl Into<Vec<(Pattern, Expr)>>) -> Expr {
    Expr::Match(Box::new(head), branches.into())
}

pub fn match_variant<Name: IntoLabel>(
    head: Expr,
    branches: impl IntoIterator<Item = (Pattern, Name, Format)>,
) -> Format {
    Format::Match(
        head,
        (branches.into_iter())
            .map(|(pattern, label, format)| {
                (pattern, Format::Variant(label.into(), Box::new(format)))
            })
            .collect(),
    )
}

pub fn union(branches: impl IntoIterator<Item = Format>) -> Format {
    Format::Union(branches.into_iter().collect())
}

pub fn union_nondet<Name: IntoLabel>(branches: impl IntoIterator<Item = (Name, Format)>) -> Format {
    Format::UnionNondet(
        (branches.into_iter())
            .map(|(label, format)| Format::Variant(label.into(), Box::new(format)))
            .collect(),
    )
}

pub fn record<Name: IntoLabel>(fields: impl IntoIterator<Item = (Name, Format)>) -> Format {
    Format::Record(
        (fields.into_iter())
            .map(|(label, format)| (label.into(), format))
            .collect(),
    )
}

pub fn optional(format: Format) -> Format {
    alts([("some", format), ("none", Format::EMPTY)])
}

pub fn repeat(format: Format) -> Format {
    Format::Repeat(Box::new(format))
}

pub fn repeat1(format: Format) -> Format {
    Format::Repeat1(Box::new(format))
}

pub fn repeat_count(len: Expr, format: Format) -> Format {
    Format::RepeatCount(len, Box::new(format))
}

#[allow(dead_code)]
pub fn repeat_until_last(cond: Expr, format: Format) -> Format {
    Format::RepeatUntilLast(cond, Box::new(format))
}

#[allow(dead_code)]
pub fn repeat_until_seq(cond: Expr, format: Format) -> Format {
    Format::RepeatUntilSeq(cond, Box::new(format))
}

pub fn if_then_else(cond: Expr, format0: Format, format1: Format) -> Format {
    Format::Match(
        cond,
        vec![
            (Pattern::Bool(true), format0),
            (Pattern::Bool(false), format1),
        ],
    )
}

pub fn if_then_else_variant(cond: Expr, format0: Format, format1: Format) -> Format {
    if_then_else(
        cond,
        Format::Variant("yes".into(), Box::new(format0)),
        Format::Variant("no".into(), Box::new(format1)),
    )
}

pub fn map(f: Format, expr: Expr) -> Format {
    Format::Map(Box::new(f), expr)
}

pub fn is_byte(b: u8) -> Format {
    Format::Byte(ByteSet::from([b]))
}

pub fn byte_in<I>(v: I) -> Format
where
    I: Into<ByteSet>,
{
    Format::Byte(v.into())
}

pub fn repeat_byte(count: u32, b: u8) -> Format {
    Format::RepeatCount(Expr::U32(count), Box::new(is_byte(b)))
}

pub fn not_byte(b: u8) -> Format {
    Format::Byte(!ByteSet::from([b]))
}

pub fn is_bytes(bytes: &[u8]) -> Format {
    tuple(bytes.iter().copied().map(is_byte))
}

pub fn record_proj(head: impl Into<Expr>, label: impl IntoLabel) -> Expr {
    Expr::RecordProj(Box::new(head.into()), label.into())
}

pub fn expr_eq(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Eq, Box::new(x), Box::new(y))
}

pub fn expr_ne(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Ne, Box::new(x), Box::new(y))
}

pub fn expr_lt(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Lt, Box::new(x), Box::new(y))
}

pub fn expr_gt(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Gt, Box::new(x), Box::new(y))
}

pub fn expr_gte(x: Expr, y: Expr) -> Expr {
    Expr::IntRel(IntRel::Gte, Box::new(x), Box::new(y))
}

pub fn as_u8(x: Expr) -> Expr {
    Expr::AsU8(Box::new(x))
}

pub fn as_u16(x: Expr) -> Expr {
    Expr::AsU16(Box::new(x))
}

pub fn as_u32(x: Expr) -> Expr {
    Expr::AsU32(Box::new(x))
}

pub fn as_u64(x: Expr) -> Expr {
    Expr::AsU64(Box::new(x))
}

pub fn as_char(x: Expr) -> Expr {
    Expr::AsChar(Box::new(x))
}

pub fn add(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Add, Box::new(x), Box::new(y))
}

pub fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Sub, Box::new(x), Box::new(y))
}

pub fn rem(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::Rem, Box::new(x), Box::new(y))
}

pub fn bit_or(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BitOr, Box::new(x), Box::new(y))
}

pub fn bit_and(x: Expr, y: Expr) -> Expr {
    Expr::Arith(Arith::BitAnd, Box::new(x), Box::new(y))
}

pub fn shl(value: Expr, places: Expr) -> Expr {
    Expr::Arith(Arith::Shl, Box::new(value), Box::new(places))
}

pub fn shr(value: Expr, places: Expr) -> Expr {
    Expr::Arith(Arith::Shr, Box::new(value), Box::new(places))
}

pub fn seq_length(seq: Expr) -> Expr {
    Expr::SeqLength(Box::new(seq))
}

pub fn sub_seq(seq: Expr, start: Expr, length: Expr) -> Expr {
    Expr::SubSeq(Box::new(seq), Box::new(start), Box::new(length))
}

pub fn flat_map(f: Expr, seq: Expr) -> Expr {
    Expr::FlatMap(Box::new(f), Box::new(seq))
}

pub fn flat_map_accum(f: Expr, accum: Expr, accum_type: ValueType, seq: Expr) -> Expr {
    Expr::FlatMapAccum(Box::new(f), Box::new(accum), accum_type, Box::new(seq))
}

pub fn dup(count: Expr, expr: Expr) -> Expr {
    Expr::Dup(Box::new(count), Box::new(expr))
}
