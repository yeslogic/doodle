use doodle::byte_set::ByteSet;
use doodle::{BaseFormat, Expr, Format, FormatModule, Func, Pattern};

pub fn tuple<T>(formats: impl IntoIterator<Item = BaseFormat<T>>) -> BaseFormat<T> {
    BaseFormat::Tuple(formats.into_iter().collect())
}

pub fn alts<Label: Into<String>, T>(
    fields: impl IntoIterator<Item = (Label, BaseFormat<T>)>,
) -> BaseFormat<T> {
    BaseFormat::Union(
        (fields.into_iter())
            .map(|(label, format)| (label.into(), format))
            .collect(),
    )
}

pub fn record<Label: Into<String>, T>(
    fields: impl IntoIterator<Item = (Label, BaseFormat<T>)>,
) -> BaseFormat<T> {
    BaseFormat::Record(
        (fields.into_iter())
            .map(|(label, format)| (label.into(), format))
            .collect(),
    )
}

pub fn optional<T>(format: BaseFormat<T>) -> BaseFormat<T> {
    alts([("some", format), ("none", BaseFormat::EMPTY)])
}

pub fn repeat<T>(format: BaseFormat<T>) -> BaseFormat<T> {
    BaseFormat::Repeat(Box::new(format))
}

pub fn repeat1<T>(format: BaseFormat<T>) -> BaseFormat<T> {
    BaseFormat::Repeat1(Box::new(format))
}

pub fn repeat_count<T>(len: Expr, format: BaseFormat<T>) -> BaseFormat<T> {
    BaseFormat::RepeatCount(len, Box::new(format))
}

pub fn if_then_else<T>(
    cond: Expr,
    format0: BaseFormat<T>,
    format1: BaseFormat<T>,
) -> BaseFormat<T> {
    BaseFormat::Match(
        cond,
        vec![
            (Pattern::Bool(true), format0),
            (Pattern::Bool(false), format1),
        ],
    )
}

pub fn is_byte(b: u8) -> Format {
    Format::Token(ByteSet::from([b]))
}

pub fn not_byte(b: u8) -> Format {
    Format::Token(!ByteSet::from([b]))
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
    let u8 = module.define_format("base.u8", Format::Token(ByteSet::full()));

    let u16be = module.define_format(
        "base.u16be",
        Format::Map(Func::U16Be, Box::new(tuple([u8.clone(), u8.clone()]))),
    );

    let u16le = module.define_format(
        "base.u16le",
        Format::Map(Func::U16Le, Box::new(tuple([u8.clone(), u8.clone()]))),
    );

    let u32be = module.define_format(
        "base.u32be",
        Format::Map(
            Func::U32Be,
            Box::new(tuple([u8.clone(), u8.clone(), u8.clone(), u8.clone()])),
        ),
    );

    let u32le = module.define_format(
        "base.u32le",
        Format::Map(
            Func::U32Le,
            Box::new(tuple([u8.clone(), u8.clone(), u8.clone(), u8.clone()])),
        ),
    );

    let asciiz_string = module.define_format(
        "base.asciiz-string",
        Format::Map(
            Func::RecordProj("string".to_string()),
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
