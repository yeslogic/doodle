use doodle::{Expr, Format, FormatModule};

use crate::format::base::*;

/// gzip
#[allow(clippy::redundant_clone)]
pub fn main(module: &mut FormatModule, deflate: Format, base: &BaseModule) -> Format {
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

    let fname_flag = Expr::Ne(
        Box::new(Expr::BitAnd(
            Box::new(Expr::RecordProj(
                Box::new(Expr::Var(0)),
                "file-flags".to_string(),
            )),
            Box::new(Expr::U8(0x08)),
        )),
        Box::new(Expr::U8(0x00)),
    );

    let fname = module.define_format("gzip.fname", base.asciiz_string());

    module.define_format(
        "gzip.main",
        repeat1(record([
            ("header", header),
            ("fname", if_then_else(fname_flag, fname, Format::EMPTY)),
            // FIXME fextra
            // FIXME fcomment
            // FIXME fhcrc
            ("data", Format::Bits(Box::new(deflate))),
            ("footer", footer),
        ])),
    )
}