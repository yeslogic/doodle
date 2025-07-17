use doodle::byte_set::ByteSet;
use doodle::helper::*;
use doodle::{Expr, Format, FormatModule, FormatRef};

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
#[allow(dead_code)]
impl BaseModule {
    pub fn bit(&self) -> Format { self.bit.call() }
    pub fn u8(&self) -> Format { self.u8.call() }
    pub fn u16be(&self) -> Format { self.u16be.call() }
    pub fn u16le(&self) -> Format { self.u16le.call() }
    pub fn u32be(&self) -> Format { self.u32be.call() }
    pub fn u32le(&self) -> Format { self.u32le.call() }
    pub fn u64be(&self) -> Format { self.u64be.call() }
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
            tuple_repeat(2, u8.call()),
            lambda("x", Expr::U16Be(Box::new(var("x")))),
        ),
    );

    let u16le = module.define_format(
        "base.u16le",
        map(
            tuple_repeat(2, u8.call()),
            lambda("x", Expr::U16Le(Box::new(var("x")))),
        ),
    );

    let u32be = module.define_format(
        "base.u32be",
        map(
            tuple_repeat(4, u8.call()),
            lambda("x", Expr::U32Be(Box::new(var("x")))),
        ),
    );

    let u32le = module.define_format(
        "base.u32le",
        map(
            tuple_repeat(4, u8.call()),
            lambda("x", Expr::U32Le(Box::new(var("x")))),
        ),
    );

    let u64be = module.define_format(
        "base.u64be",
        map(
            tuple_repeat(8, u8.call()),
            lambda("x", Expr::U64Be(Box::new(var("x")))),
        ),
    );

    let u64le = module.define_format(
        "base.u64le",
        map(
            tuple_repeat(8, u8.call()),
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
