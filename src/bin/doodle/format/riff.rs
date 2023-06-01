use doodle::{Expr, Format, FormatModule};

use crate::format::base::*;

#[allow(clippy::redundant_clone)]
pub fn main(module: &mut FormatModule, base: &BaseModule) -> Format {
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
            (
                "data",
                Format::Slice(
                    Expr::RecordProj(Box::new(Expr::Var(0)), "@value".to_string()),
                    Box::new(data),
                ),
            ),
            (
                "pad",
                if_then_else(
                    is_even(Expr::RecordProj(
                        Box::new(Expr::Var(1)),
                        "@value".to_string(),
                    )),
                    Format::EMPTY,
                    is_byte(0x00),
                ),
            ),
        ])
    };

    let any_tag = module.define_format(
        "riff.any-tag",
        tuple([base.u8(), base.u8(), base.u8(), base.u8()]), // FIXME: ASCII
    );

    let subchunks = module.define_format(
        "riff.subchunks",
        record([
            ("tag", any_tag.clone()),
            ("chunks", repeat(chunk(any_tag, repeat(base.u8())))),
        ]),
    );

    module.define_format("riff.main", chunk(is_bytes(b"RIFF"), subchunks.clone()))
}
