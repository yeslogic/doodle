use doodle::{Expr, Format, FormatModule};

use crate::format::base::*;

#[allow(clippy::redundant_clone)]
pub fn main(module: &mut FormatModule, base: &BaseModule) -> Format {
    let chunk = |tag: Format, data: Format| {
        record([
            ("length", base.u32be()), // FIXME < 2^31
            ("tag", tag),
            ("data", Format::Slice(Expr::Var(1), Box::new(data))),
            ("crc", base.u32be()), // FIXME check this
        ])
    };

    // let any_tag = module.define_format(
    //     "png.any-tag",
    //     tuple([base.u8(), base.u8(), base.u8(), base.u8()]), // FIXME: ASCII
    // );

    let ihdr_tag = module.define_format("png.ihdr-tag", is_bytes(b"IHDR"));
    let ihdr_data = module.define_format(
        "png.ihdr-data",
        record([
            ("width", base.u32be()),
            ("height", base.u32be()),
            ("bit-depth", base.u8()),
            ("color-type", base.u8()),
            ("compression-method", base.u8()),
            ("filter-method", base.u8()),
            ("interlace-method", base.u8()),
        ]),
    );

    let idat_tag = module.define_format("png.idat-tag", is_bytes(b"IDAT"));
    let idat_data = module.define_format("png.idat-data", repeat(base.u8()));

    let iend_tag = module.define_format("png.iend-tag", is_bytes(b"IEND"));
    let iend_data = module.define_format("png.iend-data", Format::EMPTY); // FIXME ensure IEND length = 0

    let other_tag = module.define_format(
        "png.other-tag",
        alts([
            ("PLTE", is_bytes(b"PLTE")),
            ("bKGD", is_bytes(b"bKGD")),
            ("pHYs", is_bytes(b"pHYs")),
            ("tIME", is_bytes(b"tIME")),
            ("tRNS", is_bytes(b"tRNS")),
            // FIXME other tags excluding IHDR/IDAT/IEND
        ]),
    );

    module.define_format(
        "png.main",
        record([
            ("signature", is_bytes(b"\x89PNG\r\n\x1A\n")),
            ("ihdr", chunk(ihdr_tag, ihdr_data)),
            (
                "chunks",
                repeat(chunk(other_tag.clone(), repeat(base.u8()))),
            ),
            ("idat", repeat1(chunk(idat_tag, idat_data))),
            (
                "more-chunks",
                repeat(chunk(other_tag.clone(), repeat(base.u8()))),
            ),
            ("iend", chunk(iend_tag, iend_data)),
        ]),
    )
}
