use doodle::byte_set::ByteSet;
use doodle::{Expr, Format, FormatModule, Pattern};

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
    Format::Match(
        cond,
        vec![
            (Pattern::Bool(true), format0),
            (Pattern::Bool(false), format1),
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
    u8: Format,
    u16be: Format,
    u16le: Format,
    u32be: Format,
    u32le: Format,
    asciiz_string: Format,
}

#[rustfmt::skip]
impl BaseModule {
    pub fn u8(&self) -> Format { self.u8.clone() }
    pub fn u16be(&self) -> Format { self.u16be.clone() }
    pub fn u16le(&self) -> Format { self.u16le.clone() }
    pub fn u32be(&self) -> Format { self.u32be.clone() }
    pub fn u32le(&self) -> Format { self.u32le.clone() }
    pub fn asciiz_string(&self) -> Format { self.asciiz_string.clone() }
}

pub fn main(module: &mut FormatModule) -> BaseModule {
    let u8 = module.define_format("base.u8", Format::Byte(ByteSet::full()));

    let u16be = module.define_format(
        "base.u16be",
        Format::Map(
            Expr::U16Be(Box::new(Expr::Var(0))),
            Box::new(tuple([u8.clone(), u8.clone()])),
        ),
    );

    let u16le = module.define_format(
        "base.u16le",
        Format::Map(
            Expr::U16Le(Box::new(Expr::Var(0))),
            Box::new(tuple([u8.clone(), u8.clone()])),
        ),
    );

    let u32be = module.define_format(
        "base.u32be",
        Format::Map(
            Expr::U32Be(Box::new(Expr::Var(0))),
            Box::new(tuple([u8.clone(), u8.clone(), u8.clone(), u8.clone()])),
        ),
    );

    let u32le = module.define_format(
        "base.u32le",
        Format::Map(
            Expr::U32Le(Box::new(Expr::Var(0))),
            Box::new(tuple([u8.clone(), u8.clone(), u8.clone(), u8.clone()])),
        ),
    );

    let asciiz_string = module.define_format(
        "base.asciiz-string",
        Format::Map(
            Expr::RecordProj(Box::new(Expr::Var(0)), "string".to_string()),
            Box::new(record([
                ("string", repeat(not_byte(0x00))),
                ("null", is_byte(0x00)),
            ])),
        ),
    );

    BaseModule {
        u8,
        u16be,
        u16le,
        u32be,
        u32le,
        asciiz_string,
    }
}
