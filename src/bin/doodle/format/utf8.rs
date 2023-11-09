use crate::format::BaseModule;
use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

use super::base::*;

// mask tabel for bitwise and in order to drop N bits, for N = 0 ..= 5
// We technically don't need a mask to drop 0, but it keeps the other indices intuitively correct
const DROPMASKS: [u8; 6] = [
    0b1111_1111, // Drop 0
    0b0111_1111, // Drop 1
    0b0011_1111, // Drop 2
    0b0001_1111, // Drop 3
    0b0000_1111, // Drop 4
    0b0000_0111, // Drop 5
];

fn drop_n_msb(n: usize, format: Format) -> Format {
    record([
        ("raw", format),
        (
            "@value",
            Format::Compute(Expr::BitAnd(
                Box::new(var("raw")),
                Box::new(Expr::U8(DROPMASKS[n])),
            )),
        ),
    ])
}

pub fn main(module: &mut FormatModule, _base: &BaseModule) -> FormatRef {
    let utf8_tail = drop_n_msb(2, byte_in(0x80..=0xBF));

    let utf8_1 = record([
        ("byte", Format::Byte(VALID_ASCII)),
        (
            "@value",
            Format::Compute(Expr::AsU32(Box::new(var("byte")))),
        ),
    ]);
    let utf8_2 = record([
        (
            "bytes",
            tuple([drop_n_msb(3, byte_in(0xC2..=0xDF)), utf8_tail.clone()]),
        ),
        (
            "@value",
            Format::Match(
                var("bytes"),
                vec![(
                    Pattern::Tuple(vec![
                        Pattern::Binding("x1".to_string()),
                        Pattern::Binding("x0".to_string()),
                    ]),
                    Format::Compute(shift6_2(var("x1"), var("x0"))),
                )],
            ),
        ),
    ]);

    let pat3 = |lab: &str| {
        Pattern::variant(
            lab,
            Pattern::Tuple(vec![
                Pattern::Binding("x2".to_string()),
                Pattern::Binding("x1".to_string()),
                Pattern::Binding("x0".to_string()),
            ]),
        )
    };

    let utf8_3 = record([
        (
            "bytes",
            alts([
                (
                    "#e0",
                    tuple([
                        drop_n_msb(4, is_byte(0xE0)),
                        drop_n_msb(2, byte_in(0xA0..=0xBF)),
                        utf8_tail.clone(),
                    ]),
                ),
                (
                    "#e1",
                    tuple([
                        drop_n_msb(4, byte_in(0xE1..=0xEC)),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
                (
                    "#ed",
                    tuple([
                        drop_n_msb(4, is_byte(0xED)),
                        drop_n_msb(2, byte_in(0x80..=0x9F)),
                        utf8_tail.clone(),
                    ]),
                ),
                (
                    "#ee",
                    tuple([
                        drop_n_msb(4, byte_in(0xEE..=0xEF)),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
            ]),
        ),
        (
            "@value",
            Format::Match(
                var("bytes"),
                [
                    (
                        pat3("#e0"),
                        Format::Compute(shift6_3(var("x2"), var("x1"), var("x0"))),
                    ),
                    (
                        pat3("#e1"),
                        Format::Compute(shift6_3(var("x2"), var("x1"), var("x0"))),
                    ),
                    (
                        pat3("#ed"),
                        Format::Compute(shift6_3(var("x2"), var("x1"), var("x0"))),
                    ),
                    (
                        pat3("#ee"),
                        Format::Compute(shift6_3(var("x2"), var("x1"), var("x0"))),
                    ),
                ]
                .to_vec(),
            ),
        ),
    ]);

    let pat4 = |lab: &str| {
        Pattern::variant(
            lab,
            Pattern::Tuple(vec![
                Pattern::Binding("x3".to_string()),
                Pattern::Binding("x2".to_string()),
                Pattern::Binding("x1".to_string()),
                Pattern::Binding("x0".to_string()),
            ]),
        )
    };

    let utf8_4 = record([
        (
            "bytes",
            alts([
                (
                    "#f0",
                    tuple([
                        drop_n_msb(5, is_byte(0xF0)),
                        drop_n_msb(2, byte_in(0x90..=0xBF)),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
                (
                    "#f1",
                    tuple([
                        drop_n_msb(5, byte_in(0xF1..=0xF3)),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
                (
                    "#f4",
                    tuple([
                        drop_n_msb(5, is_byte(0xF4)),
                        drop_n_msb(2, byte_in(0x80..=0x8F)),
                        utf8_tail.clone(),
                        utf8_tail.clone(),
                    ]),
                ),
            ]),
        ),
        (
            "@value",
            Format::Match(
                var("bytes"),
                vec![
                    (
                        pat4("#f0"),
                        Format::Compute(shift6_4(var("x3"), var("x2"), var("x1"), var("x0"))),
                    ),
                    (
                        pat4("#f1"),
                        Format::Compute(shift6_4(var("x3"), var("x2"), var("x1"), var("x0"))),
                    ),
                    (
                        pat4("#f4"),
                        Format::Compute(shift6_4(var("x3"), var("x2"), var("x1"), var("x0"))),
                    ),
                ],
            ),
        ),
    ]);

    // https://datatracker.ietf.org/doc/html/rfc3629#section-4
    let utf8_char = module.define_format(
        "utf8.char",
        record([
            (
                "bytes",
                alts([
                    ("utf8-1", utf8_1),
                    ("utf8-2", utf8_2),
                    ("utf8-3", utf8_3),
                    ("utf8-4", utf8_4),
                ]),
            ),
            (
                "@value",
                Format::Match(
                    var("bytes"),
                    vec![
                        (
                            Pattern::variant("utf8-1", Pattern::Binding("x".to_string())),
                            Format::Compute(Expr::AsChar(Box::new(var("x")))),
                        ),
                        (
                            Pattern::variant("utf8-2", Pattern::Binding("x".to_string())),
                            Format::Compute(Expr::AsChar(Box::new(var("x")))),
                        ),
                        (
                            Pattern::variant("utf8-3", Pattern::Binding("x".to_string())),
                            Format::Compute(Expr::AsChar(Box::new(var("x")))),
                        ),
                        (
                            Pattern::variant("utf8-4", Pattern::Binding("x".to_string())),
                            Format::Compute(Expr::AsChar(Box::new(var("x")))),
                        ),
                    ],
                ),
            ),
        ]),
    );

    module.define_format("utf8.string", repeat(utf8_char.call()))
}

fn shift6_2(hi: Expr, lo: Expr) -> Expr {
    bitor(
        shl(Expr::AsU32(Box::new(hi)), Expr::U32(6)),
        Expr::AsU32(Box::new(lo)),
    )
}

fn shift6_3(hi: Expr, mid: Expr, lo: Expr) -> Expr {
    bitor(
        shl(Expr::AsU32(Box::new(hi)), Expr::U32(12)),
        shift6_2(mid, lo),
    )
}

fn shift6_4(hh: Expr, hl: Expr, lh: Expr, ll: Expr) -> Expr {
    bitor(
        shl(Expr::AsU32(Box::new(hh)), Expr::U32(18)),
        shift6_3(hl, lh, ll),
    )
}
