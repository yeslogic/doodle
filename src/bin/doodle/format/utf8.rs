use crate::format::BaseModule;
use doodle::{Format, FormatModule, FormatRef, Expr};

use super::base::*;

pub fn main(module: &mut FormatModule, _base: &BaseModule) -> FormatRef {
    let utf8_tail = byte_in(0x80..=0xBF);

    let utf8_1 = Format::Byte(VALID_ASCII);
    let utf8_2 = record([
        ("bytes", tuple([byte_in(0xC2..=0xDF), utf8_tail.clone()])),
        (
            "@value",
            Format::Compute(Expr::AsChar(Box::new(Expr::U16Be(Box::new(var("bytes")))))),
        ),
    ]);
    let utf8_3 = record([
        (
            "bytes",
            alts([
                (
                    "#e0",
                    tuple([is_byte(0xE0), byte_in(0xA0..=0xBF), utf8_tail.clone()]),
                ),
                (
                    "#e1",
                    tuple([byte_in(0xE1..=0xEC), utf8_tail.clone(), utf8_tail.clone()]),
                ),
                (
                    "#ed",
                    tuple([is_byte(0xED), byte_in(0x80..=0x9F), utf8_tail.clone()]),
                ),
                (
                    "#ee",
                    tuple([byte_in(0xEE..=0xEF), utf8_tail.clone(), utf8_tail.clone()]),
                ),
            ]),
        ),
        (
            "@value",
            Format::Compute(Expr::AsChar(Box::new(Expr::U32Be(Box::new(var("bytes")))))),
        ),
    ]);
    let utf8_4 = record([
        (
            "bytes",
            alts([
                (
                    "#f0",
                    tuple([
                        is_byte(0xF0),
                        byte_in(0x90..=0xBF),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
                (
                    "#f1",
                    tuple([
                        byte_in(0xF1..=0xF3),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
                (
                    "#f4",
                    tuple([
                        is_byte(0xED),
                        byte_in(0x80..=0x8F),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
            ]),
        ),
        (
            "@value",
            Format::Compute(Expr::AsChar(Box::new(Expr::U32Be(Box::new(var("bytes")))))),
        ),
    ]);

    // https://datatracker.ietf.org/doc/html/rfc3629#section-4
    let utf8_char = module.define_format(
        "utf8.char",
        alts([
            ("utf8-1", utf8_1),
            ("utf8-2", utf8_2),
            ("utf8-3", utf8_3),
            ("utf8-4", utf8_4),
        ]),
    );

    module.define_format("utf8.string",
        repeat(utf8_char.call())
    )
}
