use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

use crate::format::base::*;

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    let chunk = |tag: Format, data: Format| {
        record([
            ("length", base.u32be()), // FIXME < 2^31
            ("tag", tag),
            (
                "data",
                Format::Slice(Expr::VarName("length".to_string()), Box::new(data)),
            ),
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
    let ihdr = module.define_format("png.ihdr", chunk(ihdr_tag.call(), ihdr_data.call()));

    let idat_tag = module.define_format("png.idat-tag", is_bytes(b"IDAT"));
    let idat_data = module.define_format("png.idat-data", repeat(base.u8()));
    let idat = module.define_format("png.idat", chunk(idat_tag.call(), idat_data.call()));

    let iend_tag = module.define_format("png.iend-tag", is_bytes(b"IEND"));
    let iend_data = module.define_format("png.iend-data", Format::EMPTY); // FIXME ensure IEND length = 0
    let iend = module.define_format("png.iend", chunk(iend_tag.call(), iend_data.call()));

    let bkgd_data = Format::Match(
        Expr::RecordProj(
            Box::new(Expr::RecordProj(Box::new(Expr::Var(2)), "data".to_string())),
            "color-type".to_string(),
        ),
        vec![
            (Pattern::U8(0), record([("greyscale", base.u16be())])),
            (Pattern::U8(4), record([("greyscale", base.u16be())])),
            (
                Pattern::U8(2),
                record([
                    ("red", base.u16be()),
                    ("green", base.u16be()),
                    ("blue", base.u16be()),
                ]),
            ),
            (
                Pattern::U8(6),
                record([
                    ("red", base.u16be()),
                    ("green", base.u16be()),
                    ("blue", base.u16be()),
                ]),
            ),
            (Pattern::U8(3), record([("palette-index", base.u8())])),
        ],
    );
    let bkgd = module.define_format("png.bkgd", chunk(is_bytes(b"bKGD"), bkgd_data));

    let phys_data = record([
        ("pixels-per-unit-x", base.u32be()),
        ("pixels-per-unit-y", base.u32be()),
        ("unit-specifier", base.u8()),
    ]);
    let phys = module.define_format("png.phys", chunk(is_bytes(b"pHYs"), phys_data));

    let palette_entry = record([("r", base.u8()), ("g", base.u8()), ("b", base.u8())]);
    let plte_data = repeat1(palette_entry);
    let plte = module.define_format("png.plte", chunk(is_bytes(b"PLTE"), plte_data));

    let time_data = record([
        ("year", base.u16be()),
        ("month", base.u8()),
        ("day", base.u8()),
        ("hour", base.u8()),
        ("minute", base.u8()),
        ("second", base.u8()),
    ]);
    let time = module.define_format("png.time", chunk(is_bytes(b"tIME"), time_data));

    let trns_data = Format::Match(
        Expr::RecordProj(
            Box::new(Expr::RecordProj(Box::new(Expr::Var(2)), "data".to_string())),
            "color-type".to_string(),
        ),
        vec![
            (Pattern::U8(0), record([("greyscale", base.u16be())])),
            (
                Pattern::U8(2),
                record([
                    ("red", base.u16be()),
                    ("green", base.u16be()),
                    ("blue", base.u16be()),
                ]),
            ),
            (
                Pattern::U8(3),
                repeat(record([("palette-index", base.u8())])),
            ),
        ],
    );
    let trns = module.define_format("png.trns", chunk(is_bytes(b"tRNS"), trns_data));

    let png_chunk = module.define_format(
        "png.chunk",
        alts([
            ("bKGD", bkgd.call_args(vec![Expr::Var(0)])),
            ("pHYs", phys.call()),
            ("PLTE", plte.call()),
            ("tIME", time.call()),
            ("tRNS", trns.call_args(vec![Expr::Var(0)])),
            // FIXME other tags excluding IHDR/IDAT/IEND
        ]),
    );

    let png_signature = module.define_format("png.signature", is_bytes(b"\x89PNG\r\n\x1A\n"));

    module.define_format(
        "png.main",
        record([
            ("signature", png_signature.call()),
            ("ihdr", ihdr.call()),
            (
                "chunks",
                repeat(png_chunk.call_args(vec![Expr::VarName("ihdr".to_string())])),
            ),
            ("idat", repeat1(idat.call())),
            ("more-chunks", repeat(png_chunk.call())),
            ("iend", iend.call()),
        ]),
    )
}
