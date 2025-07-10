use doodle::{helper::*, StyleHint};
use doodle::{Expr, Format, FormatModule, FormatRef, Label, ViewFormat};

use super::base::BaseModule;

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    module.define_format(
        "waldo.main",
        record_auto([
            ("where", base.u64be()),
            ("noise", repeat(is_byte(0xFF))),
            ("__sep", is_byte(0x00)),
            ("_here", Format::Pos),
            (
                "waldo",
                Format::LetView(
                    Label::Borrowed("scope"),
                    Box::new(Format::Hint(
                        StyleHint::AsciiStr,
                        Box::new(Format::WithView(
                            Label::Borrowed("scope"),
                            ViewFormat::ReadOffsetLen(
                                Box::new(sub(var("where"), var("_here"))),
                                Box::new(Expr::U8(5)),
                            ),
                        )),
                    )),
                ),
            ),
            // (
            //     "waldo",
            //     Format::WithRelativeOffset(
            //         Box::new(var("_here")),
            //         Box::new(sub(var("where"), var("_here"))),
            //         Box::new(is_bytes(b"Waldo")),
            //     ),
            // ),
            ("__rem", Format::SkipRemainder),
        ]),
    )
}
