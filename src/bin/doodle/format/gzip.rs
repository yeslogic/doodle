use doodle::{Expr, Format, FormatModule, FormatRef};

use crate::format::base::*;

/// gzip
pub fn main(module: &mut FormatModule, deflate: FormatRef, base: &BaseModule) -> FormatRef {
    let header = module.define_format(
        "gzip.header",
        record([
            ("magic", is_bytes(b"\x1F\x8B")),
            ("method", base.u8()),
            ("file-flags", base.u8()),
            ("timestamp", base.u32le()),
            ("compression-flags", base.u8()),
            ("os-id", base.u8()),
        ]),
    );

    let footer = module.define_format(
        "gzip.footer",
        record([("crc", base.u32le()), ("length", base.u32le())]),
    );

    let fname_flag = expr_ne(
        bit_and(record_proj(var("header"), "file-flags"), Expr::U8(0x08)),
        Expr::U8(0x00),
    );

    let fname = module.define_format("gzip.fname", base.asciiz_string());

    module.define_format(
        "gzip.main",
        repeat1(record([
            ("header", header.call()),
            (
                "fname",
                if_then_else_variant(fname_flag, fname.call(), Format::EMPTY),
            ),
            // FIXME fextra
            // FIXME fcomment
            // FIXME fhcrc
            ("data", Format::Bits(Box::new(deflate.call()))),
            ("footer", footer.call()),
        ])),
    )
}
