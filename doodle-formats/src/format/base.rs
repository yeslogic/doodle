use doodle::byte_set::ByteSet;
use doodle::helper::*;
use doodle::{Format, FormatModule, FormatRef};

/// ByteSet consisting of 0..=127, or the valid ASCII range (including control characters)
pub const VALID_ASCII: ByteSet = ByteSet::from_bits([u64::MAX, u64::MAX, 0, 0]);

pub struct BaseModule {
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
        assert!(!u8().is_ascii_char_format(&module));
    }
}
