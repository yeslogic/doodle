use doodle::{Expr, Format, FormatModule, Func, Pattern};

use crate::format::base::*;

/// TIFF Image file header
///
/// - [TIFF 6.0 Specification, Section 4.5](https://developer.adobe.com/content/dam/udp/en/open/standards/tiff/TIFF6.pdf#page=13)
/// - [Exif Version 2.32, Section 4.5.2](https://www.cipa.jp/std/documents/e/DC-X008-Translation-2019-E.pdf#page=23)
pub fn main(module: &mut FormatModule, base: &BaseModule) -> Format {
    // Image file directory field
    let ifd_field = |is_be: bool| {
        record([
            ("tag", if is_be { base.u16be() } else { base.u16le() }),
            ("type", if is_be { base.u16be() } else { base.u16le() }),
            ("length", if is_be { base.u32be() } else { base.u32le() }),
            (
                "offset-or-data",
                if is_be { base.u32be() } else { base.u32le() },
            ),
            // TODO: Offset from start of the TIFF header for values longer than 4 bytes
        ])
    };

    // Image file directory
    let ifd = |is_be: bool| {
        record([
            (
                "num-fields",
                if is_be { base.u16be() } else { base.u16le() },
            ),
            ("fields", repeat_count(Expr::Var(0), ifd_field(is_be))),
            (
                "next-ifd-offset",
                if is_be { base.u32be() } else { base.u32le() },
            ),
            // TODO: Offset from start of the TIFF header (i.e. `offset + 2 + num-fields * 12`)
            // TODO: Recursive call to `ifd(is_be)`
            ("next-ifd", repeat(base.u8())),
        ])
    };

    module.define_format(
        "tiff.main",
        record([
            (
                "byte-order",
                alts([
                    (
                        "le",
                        Format::Map(Func::Expr(Expr::UNIT), Box::new(is_bytes(b"II"))),
                    ),
                    (
                        "be",
                        Format::Map(Func::Expr(Expr::UNIT), Box::new(is_bytes(b"MM"))),
                    ),
                ]),
            ),
            (
                "magic",
                Format::Match(
                    Expr::Var(0), // byte-order
                    vec![
                        (Pattern::variant("le", Pattern::UNIT), base.u16le()), // 42
                        (Pattern::variant("be", Pattern::UNIT), base.u16be()), // 42
                    ],
                ),
            ),
            (
                "offset",
                Format::Match(
                    Expr::Var(1), // byte-order
                    vec![
                        (Pattern::variant("le", Pattern::UNIT), base.u32le()),
                        (Pattern::variant("be", Pattern::UNIT), base.u32be()),
                    ],
                ),
            ),
            (
                "ifd",
                Format::WithRelativeOffset(
                    // TODO: Offset from start of the TIFF header
                    Expr::Sub(Box::new(Expr::Var(0)), Box::new(Expr::U32(8))),
                    Box::new(Format::Match(
                        Expr::Var(2), // byte-order
                        vec![
                            (Pattern::variant("le", Pattern::UNIT), ifd(false)),
                            (Pattern::variant("be", Pattern::UNIT), ifd(true)),
                        ],
                    )),
                ),
            ),
        ]),
    )
}
