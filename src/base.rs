use crate::byte_set::ByteSet;
use crate::IntoLabel;
use crate::{Arith, Expr, Format, FormatModule, FormatRef, IntRel, Pattern, ValueType};

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

/// ByteSet consisting of 0..=127, or the valid ASCII range (including control characters)
pub const VALID_ASCII: ByteSet = ByteSet::from_bits([u64::MAX, u64::MAX, 0, 0]);

pub struct BaseModule {
    bit: FormatRef,
    u8: FormatRef,
    u16be: FormatRef,
    u16le: FormatRef,
    u32be: FormatRef,
    u32le: FormatRef,
    u64be: FormatRef,
    u64le: FormatRef,
    ascii_char: FormatRef,
    ascii_char_strict: FormatRef,
    asciiz_string: FormatRef,

    // extensions to ascii-char
    ascii_octal_digit: FormatRef,
    #[allow(dead_code)]
    ascii_decimal_digit: FormatRef,
    ascii_hex_lower: FormatRef,
    ascii_hex_upper: FormatRef,
    ascii_hex_any: FormatRef,
}

#[rustfmt::skip]
impl BaseModule {
    pub fn bit(&self) -> Format { self.bit.call() }
    pub fn u8(&self) -> Format { self.u8.call() }
    pub fn u16be(&self) -> Format { self.u16be.call() }
    pub fn u16le(&self) -> Format { self.u16le.call() }
    pub fn u32be(&self) -> Format { self.u32be.call() }
    pub fn u32le(&self) -> Format { self.u32le.call() }
    pub fn u64be(&self) -> Format { self.u64be.call() }
    #[allow(dead_code)]
    pub fn u64le(&self) -> Format { self.u64le.call() }
    pub fn ascii_char(&self) -> Format { self.ascii_char.call() }
    pub fn ascii_char_strict(&self) -> Format { self.ascii_char_strict.call() }
    pub fn asciiz_string(&self) -> Format { self.asciiz_string.call() }
}

#[rustfmt::skip]
impl BaseModule {
    pub const ASCII_OCTAL_DIGIT: [u8; 8] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7'];

    pub fn ascii_octal_digit(&self) -> Format { self.ascii_octal_digit.call() }

    pub const ASCII_DECIMAL_DIGIT: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];

    #[allow(dead_code)]
    pub fn ascii_decimal_digit(&self) -> Format { self.ascii_decimal_digit.call() }

    pub const ASCII_HEX_LOWER: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
        b'a', b'b', b'c', b'd', b'e', b'f'
    ];

    pub const ASCII_HEX_UPPER: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
        b'A', b'B', b'C', b'D', b'E', b'F'
    ];

    pub const ASCII_HEX_ANY: [u8; 22] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
        b'A', b'B', b'C', b'D', b'E', b'F',
        b'a', b'b', b'c', b'd', b'e', b'f'
    ];

    #[allow(dead_code)]
    pub fn ascii_hex_lower(&self) -> Format { self.ascii_hex_lower.call() }
    #[allow(dead_code)]
    pub fn ascii_hex_upper(&self) -> Format { self.ascii_hex_upper.call() }
    #[allow(dead_code)]
    pub fn ascii_hex_any(&self) -> Format { self.ascii_hex_any.call() }
}

pub fn main(module: &mut FormatModule) -> BaseModule {
    let bit = module.define_format("base.bit", Format::Byte(ByteSet::full()));

    let u8 = module.define_format("base.u8", Format::Byte(ByteSet::full()));

    let u16be = module.define_format(
        "base.u16be",
        map(
            tuple([u8.call(), u8.call()]),
            lambda("x", Expr::U16Be(Box::new(var("x")))),
        ),
    );

    let u16le = module.define_format(
        "base.u16le",
        map(
            tuple([u8.call(), u8.call()]),
            lambda("x", Expr::U16Le(Box::new(var("x")))),
        ),
    );

    let u32be = module.define_format(
        "base.u32be",
        map(
            tuple([u8.call(), u8.call(), u8.call(), u8.call()]),
            lambda("x", Expr::U32Be(Box::new(var("x")))),
        ),
    );

    let u32le = module.define_format(
        "base.u32le",
        map(
            tuple([u8.call(), u8.call(), u8.call(), u8.call()]),
            lambda("x", Expr::U32Le(Box::new(var("x")))),
        ),
    );

    let u64be = module.define_format(
        "base.u64be",
        map(
            tuple([
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
            ]),
            lambda("x", Expr::U64Be(Box::new(var("x")))),
        ),
    );

    let u64le = module.define_format(
        "base.u64le",
        map(
            tuple([
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
                u8.call(),
            ]),
            lambda("x", Expr::U64Le(Box::new(var("x")))),
        ),
    );

    let ascii_char = module.define_format("base.ascii-char", Format::Byte(ByteSet::full()));

    let mut bs = ByteSet::from(32..=127);
    bs.insert(b'\t');
    bs.insert(b'\n');
    bs.insert(b'\r');
    let ascii_char_strict = module.define_format("base.ascii-char.strict", Format::Byte(bs));

    let asciiz_string = module.define_format(
        "base.asciiz-string",
        record([("string", repeat(not_byte(0x00))), ("null", is_byte(0x00))]),
    );

    let ascii_octal_digit = module.define_format(
        "base.ascii-char.octal",
        Format::Byte(ByteSet::from(BaseModule::ASCII_OCTAL_DIGIT)),
    );

    let ascii_decimal_digit = module.define_format(
        "base.ascii-char.decimal",
        Format::Byte(ByteSet::from(BaseModule::ASCII_DECIMAL_DIGIT)),
    );

    let ascii_hex_lower = module.define_format(
        "base.ascii-char.hex.lower",
        Format::Byte(ByteSet::from(BaseModule::ASCII_HEX_LOWER)),
    );
    let ascii_hex_upper = module.define_format(
        "base.ascii-char.hex.upper",
        Format::Byte(ByteSet::from(BaseModule::ASCII_HEX_UPPER)),
    );
    let ascii_hex_any = module.define_format(
        "base.ascii-char.hex.any",
        Format::Byte(ByteSet::from(BaseModule::ASCII_HEX_ANY)),
    );

    BaseModule {
        bit,
        u8,
        u16be,
        u16le,
        u32be,
        u32le,
        u64be,
        u64le,
        ascii_char,
        ascii_char_strict,
        asciiz_string,
        ascii_octal_digit,
        ascii_decimal_digit,
        ascii_hex_lower,
        ascii_hex_upper,
        ascii_hex_any,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ascii_char_sanity() {
        let mut module = FormatModule::new();
        let base = super::main(&mut module);

        assert!(base.ascii_char().is_ascii_char_format(&module));
        assert!(base.ascii_char_strict().is_ascii_char_format(&module));
        assert!(!base.u8().is_ascii_char_format(&module));
    }
}
