use doodle::helper::*;
use doodle::{FormatModule, FormatRef};
use doodle_numexpr_macro::numexpr;

pub fn main(module: &mut FormatModule) -> FormatRef {
    let num_value = module.define_format(
        "numbers.num_value",
        alts([
            ("U8Value", monad_seq(is_byte(0x00), u8())),
            ("U16BEValue", monad_seq(is_byte(0x01), u16be())),
            ("U32BEValue", monad_seq(is_byte(0x02), u32be())),
            ("U64BEValue", monad_seq(is_byte(0x03), u64be())),
            (
                "I8Value",
                monad_seq(
                    is_byte(0x04),
                    // `map_numeric` binds the parsed value as the numeric
                    // variable `"raw"` before applying the given transform.
                    map_numeric(u8(), "raw", numexpr!("raw" as i8)),
                ),
            ),
            (
                "I16BEValue",
                monad_seq(
                    is_byte(0x05),
                    map_numeric(u16be(), "raw", numexpr!("raw" as i16)),
                ),
            ),
            (
                "I32BEValue",
                monad_seq(
                    is_byte(0x06),
                    map_numeric(u32be(), "raw", numexpr!("raw" as i32)),
                ),
            ),
            (
                "I64BEValue",
                monad_seq(
                    is_byte(0x07),
                    map_numeric(u64be(), "raw", numexpr!("raw" as i64)),
                ),
            ),
        ]),
    );
    module.define_format(
        "numbers.main",
        record_auto([
            ("__magic", is_bytes(b"NUMS")),
            ("values", repeat(num_value.call())),
        ]),
    )
}
