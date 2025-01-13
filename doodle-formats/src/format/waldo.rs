use doodle::helper::*;
use doodle::{Format, FormatModule, FormatRef};

use super::base::BaseModule;

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    module.define_format(
        "waldo.main",
        record([
            ("where", base.u64be()),
            ("noise", repeat(is_byte(0xFF))),
            ("sep", is_byte(0x00)),
            ("here", Format::Pos),
            (
                "waldo",
                Format::WithRelativeOffset(
                    Box::new(var("here")),
                    Box::new(sub(var("where"), var("here"))),
                    Box::new(is_bytes(b"Waldo")),
                ),
            ),
            ("__rem", Format::SkipRemainder),
        ]),
    )
}
