use doodle::byte_set::ByteSet;
use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

pub fn var(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn tuple(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Tuple(formats.into_iter().collect())
}

pub fn alts<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
    Format::Union(
        (fields.into_iter())
            .map(|(label, format)| (label.into(), format))
            .collect(),
    )
}

pub fn record<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
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
    Format::MatchVariant(
        cond,
        vec![
            (Pattern::Bool(true), "yes".to_string(), format0),
            (Pattern::Bool(false), "no".to_string(), format1),
        ],
    )
}

pub fn is_byte(b: u8) -> Format {
    Format::Byte(ByteSet::from([b]))
}

pub fn not_byte(b: u8) -> Format {
    Format::Byte(!ByteSet::from([b]))
}

pub fn is_bytes(bytes: &[u8]) -> Format {
    tuple(bytes.iter().copied().map(is_byte))
}

pub struct BaseModule {
    bit: FormatRef,
    u8: FormatRef,
    u16be: FormatRef,
    u16le: FormatRef,
    u32be: FormatRef,
    u32le: FormatRef,
    ascii_char: FormatRef,
    asciiz_string: FormatRef,
}

#[rustfmt::skip]
impl BaseModule {
    pub fn bit(&self) -> Format { self.bit.call() }
    pub fn u8(&self) -> Format { self.u8.call() }
    pub fn u16be(&self) -> Format { self.u16be.call() }
    pub fn u16le(&self) -> Format { self.u16le.call() }
    pub fn u32be(&self) -> Format { self.u32be.call() }
    pub fn u32le(&self) -> Format { self.u32le.call() }
    pub fn ascii_char(&self) -> Format { self.ascii_char.call() }
    pub fn asciiz_string(&self) -> Format { self.asciiz_string.call() }
}

pub fn main(module: &mut FormatModule) -> BaseModule {
    let bit = module.define_format("base.bit", Format::Byte(ByteSet::full()));

    let u8 = module.define_format("base.u8", Format::Byte(ByteSet::full()));

    let u16be = module.define_format(
        "base.u16be",
        record([
            ("bytes", tuple([u8.call(), u8.call()])),
            (
                "@value",
                Format::Compute(Expr::U16Be(Box::new(var("bytes")))),
            ),
        ]),
    );

    let u16le = module.define_format(
        "base.u16le",
        record([
            ("bytes", tuple([u8.call(), u8.call()])),
            (
                "@value",
                Format::Compute(Expr::U16Le(Box::new(var("bytes")))),
            ),
        ]),
    );

    let u32be = module.define_format(
        "base.u32be",
        record([
            ("bytes", tuple([u8.call(), u8.call(), u8.call(), u8.call()])),
            (
                "@value",
                Format::Compute(Expr::U32Be(Box::new(var("bytes")))),
            ),
        ]),
    );

    let u32le = module.define_format(
        "base.u32le",
        record([
            ("bytes", tuple([u8.call(), u8.call(), u8.call(), u8.call()])),
            (
                "@value",
                Format::Compute(Expr::U32Le(Box::new(var("bytes")))),
            ),
        ]),
    );

    let ascii_char = module.define_format("base.ascii-char", Format::Byte(ByteSet::full()));

    let asciiz_string = module.define_format(
        "base.asciiz-string",
        record([("string", repeat(not_byte(0x00))), ("null", is_byte(0x00))]),
    );

    BaseModule {
        bit,
        u8,
        u16be,
        u16le,
        u32be,
        u32le,
        ascii_char,
        asciiz_string,
    }
}
