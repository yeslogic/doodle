use doodle::{Expr, Format, FormatModule, FormatRef};

use crate::format::base::*;

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    fn is_even(num: Expr) -> Expr {
        // (num % 2) == 0
        Expr::Eq(
            Box::new(Expr::Rem(Box::new(num), Box::new(Expr::U32(2)))),
            Box::new(Expr::U32(0)),
        )
    }

    let chunk = |tag: Format, data: Format| {
        record([
            ("tag", tag),
            ("length", base.u32le()),
            ("data", Format::Slice(Expr::Var(0), Box::new(data))),
            (
                "pad",
                if_then_else(is_even(Expr::Var(1)), Format::EMPTY, is_byte(0x00)),
            ),
        ])
    };

    let any_tag = module.define_format(
        "riff.tag",
        tuple([
            base.ascii_char(),
            base.ascii_char(),
            base.ascii_char(),
            base.ascii_char(),
        ]),
    );

    let any_chunk = module.define_format("riff.chunk", chunk(any_tag.call(), repeat(base.u8())));

    let subchunks = module.define_format(
        "riff.subchunks",
        record([
            ("tag", any_tag.call()),
            ("chunks", repeat(any_chunk.call())),
        ]),
    );

    module.define_format("riff.main", chunk(is_bytes(b"RIFF"), subchunks.call()))
}
