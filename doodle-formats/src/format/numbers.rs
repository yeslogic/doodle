use doodle::helper::*;
use doodle::numeric::{MachineRep, helper as num};
use doodle::{FormatModule, FormatRef};

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
                    map_numeric(u8(), |v| num::cast_bitwise(MachineRep::I8, v)),
                ),
            ),
            (
                "I16BEValue",
                monad_seq(
                    is_byte(0x05),
                    map_numeric(u16be(), |v| num::cast_bitwise(MachineRep::I16, v)),
                ),
            ),
            (
                "I32BEValue",
                monad_seq(
                    is_byte(0x06),
                    map_numeric(u32be(), |v| num::cast_bitwise(MachineRep::I32, v)),
                ),
            ),
            (
                "I64BEValue",
                monad_seq(
                    is_byte(0x07),
                    map_numeric(u64be(), |v| num::cast_bitwise(MachineRep::I64, v)),
                ),
            ),
        ]),
    );
    module.define_format(
        "numbers.main",
        record([("values", repeat(num_value.call()))]),
    )
}
