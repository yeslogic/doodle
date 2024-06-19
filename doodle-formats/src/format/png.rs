use crate::format::BaseModule;
use doodle::byte_set::ByteSet;
use doodle::helper::*;
use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

fn null_terminated(f: Format) -> Format {
    map(
        tuple(vec![f, is_byte(0)]),
        lambda("x", Expr::TupleProj(Box::new(var("x")), 0)),
    )
}

pub fn main(module: &mut FormatModule, _deflate: FormatRef, base: &BaseModule) -> FormatRef {
    let chunk = |tag: Format, data: Format| {
        record([
            ("length", base.u32be()), // FIXME < 2^31
            ("tag", tag),
            ("data", Format::Slice(var("length"), Box::new(data))),
            ("crc", base.u32be()), // FIXME check this
        ])
    };

    // FIXME: implement an actual zlib-style DEFLATE specification once we know how to do that
    let zlib = module.define_format("png.zlib", repeat(base.u8()));

    // PNG keyword for iTXt, zTXt, tEXt, and other contexts
    //   - Length >= 1, < 80 characters
    //   - Consists only of Latin-1 characters and spaces: 32..=126 | 161..=255
    //   - No leading or trailing spaces, nor consecutive spaces
    //   - Non-breaking space (160) not permitted in particular
    let keyword = module.define_format(
        "png.keyword",
        // FIXME - all we can enforce for now without more complex logic is the character set, space-rules are not something we can enforce easily
        // FIXME - this is incorrect, and we have to fix it using new primitives
        repeat_between(
            Expr::U32(1),
            Expr::U32(79),
            byte_in(ByteSet::union(
                &ByteSet::from(32..=126),
                &ByteSet::from(161..=255),
            )),
        ),
    );

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
    let ihdr_type = module.get_format_type(ihdr.get_level()).clone();

    let idat_tag = module.define_format("png.idat-tag", is_bytes(b"IDAT"));
    let idat_data = module.define_format("png.idat-data", repeat(base.u8()));
    let idat = module.define_format("png.idat", chunk(idat_tag.call(), idat_data.call()));

    let iend_tag = module.define_format("png.iend-tag", is_bytes(b"IEND"));
    let iend_data = module.define_format("png.iend-data", Format::EMPTY); // FIXME ensure IEND length = 0
    let iend = module.define_format("png.iend", chunk(iend_tag.call(), iend_data.call()));

    let bkgd_data = match_variant(
        record_proj(record_proj(var("ihdr"), "data"), "color-type"),
        vec![
            (
                Pattern::U8(0),
                "color-type-0",
                record([("greyscale", base.u16be())]),
            ),
            (
                Pattern::U8(4),
                "color-type-4",
                record([("greyscale", base.u16be())]),
            ),
            (
                Pattern::U8(2),
                "color-type-2",
                record([
                    ("red", base.u16be()),
                    ("green", base.u16be()),
                    ("blue", base.u16be()),
                ]),
            ),
            (
                Pattern::U8(6),
                "color-type-6",
                record([
                    ("red", base.u16be()),
                    ("green", base.u16be()),
                    ("blue", base.u16be()),
                ]),
            ),
            (
                Pattern::U8(3),
                "color-type-3",
                record([("palette-index", base.u8())]),
            ),
        ],
    );
    let bkgd = module.define_format_args(
        "png.bkgd",
        vec![("ihdr".into(), ihdr_type.clone())],
        chunk(is_bytes(b"bKGD"), bkgd_data),
    );

    let chrm_data = record(vec![
        ("whitepoint-x", base.u32be()),
        ("whitepoint-y", base.u32be()),
        ("red-x", base.u32be()),
        ("red-y", base.u32be()),
        ("green-x", base.u32be()),
        ("green-y", base.u32be()),
        ("blue-x", base.u32be()),
        ("blue-y", base.u32be()),
    ]);

    let chrm = module.define_format("png.chrm", chunk(is_bytes(b"cHRM"), chrm_data));

    // FIXME: do we want to map the value to its intended scale (y := x / 100_000)?
    let gama_data = record(vec![("gamma", base.u32be())]);

    let gama = module.define_format("png.gama", chunk(is_bytes(b"gAMA"), gama_data));

    let itxt_data = record([
        ("keyword", null_terminated(keyword.call())),
        ("compression-flag", byte_in([0, 1])),
        ("compression-method", is_byte(0)),
        ("language-tag", base.asciiz_string()), // REVIEW - there are specific rules to this (1-8--character ascii-only words separated by hyphens)
        ("translated-keyword", base.asciiz_string()),
        (
            "text",
            if_then_else(
                Expr::IntRel(
                    doodle::IntRel::Eq,
                    Box::new(var("compression-flag")),
                    Box::new(Expr::U8(1)),
                ),
                zlib.call(),
                // FIXME - replace with UTF8-compatible text format (e.g. text.rs)
                repeat(base.u8()),
            ),
        ),
    ]);

    let itxt = module.define_format("png.itxt", chunk(is_bytes(b"iTXt"), itxt_data));

    let iccp_data = record(vec![
        ("profile-name", null_terminated(keyword.call())),
        ("compression-method", is_byte(0)), // FIXME: technically the value is unrestricted but 0 := deflate is the only defined value
        ("compressed-profile", zlib.call()),
    ]);

    let iccp = module.define_format("png.iccp", chunk(is_bytes(b"iCCP"), iccp_data));

    let phys_data = record([
        ("pixels-per-unit-x", base.u32be()),
        ("pixels-per-unit-y", base.u32be()),
        ("unit-specifier", base.u8()),
    ]);
    let phys = module.define_format("png.phys", chunk(is_bytes(b"pHYs"), phys_data));

    let palette_entry = record([("r", base.u8()), ("g", base.u8()), ("b", base.u8())]);
    let plte_data = repeat1(palette_entry);
    let plte = module.define_format("png.plte", chunk(is_bytes(b"PLTE"), plte_data));

    let text_data = record([
        ("keyword", null_terminated(keyword.call())),
        ("text", repeat(base.ascii_char())),
    ]);

    let text = module.define_format("png.text", chunk(is_bytes(b"tEXt"), text_data));

    let time_data = record([
        ("year", base.u16be()),
        ("month", base.u8()),
        ("day", base.u8()),
        ("hour", base.u8()),
        ("minute", base.u8()),
        ("second", base.u8()),
    ]);
    let time = module.define_format("png.time", chunk(is_bytes(b"tIME"), time_data));

    let trns_data = match_variant(
        record_proj(record_proj(var("ihdr"), "data"), "color-type"),
        vec![
            (
                Pattern::U8(0),
                "color-type-0",
                record([("greyscale", base.u16be())]),
            ),
            (
                Pattern::U8(2),
                "color-type-2",
                record([
                    ("red", base.u16be()),
                    ("green", base.u16be()),
                    ("blue", base.u16be()),
                ]),
            ),
            (
                Pattern::U8(3),
                "color-type-3",
                repeat(record([("palette-index", base.u8())])),
            ),
        ],
    );
    let trns = module.define_format_args(
        "png.trns",
        vec![("ihdr".into(), ihdr_type.clone())],
        chunk(is_bytes(b"tRNS"), trns_data),
    );

    let ztxt_data = record([
        ("keyword", null_terminated(keyword.call())),
        ("compression-method", is_byte(0)),
        ("compressed-text", zlib.call()),
    ]);

    let ztxt = module.define_format("png.ztxt", chunk(is_bytes(b"zTXt"), ztxt_data));

    let png_chunk = module.define_format_args(
        "png.chunk",
        vec![("ihdr".into(), ihdr_type)],
        alts([
            ("bKGD", bkgd.call_args(vec![var("ihdr")])),
            ("cHRM", chrm.call()),
            ("iCCP", iccp.call()),
            ("iTXt", itxt.call()),
            ("gAMA", gama.call()),
            ("pHYs", phys.call()),
            ("PLTE", plte.call()),
            ("tEXt", text.call()),
            ("tIME", time.call()),
            ("tRNS", trns.call_args(vec![var("ihdr")])),
            ("zTXt", ztxt.call()),
            // FIXME other tags excluding IHDR/IDAT/IEND
        ]),
    );

    let png_signature = module.define_format("png.signature", is_bytes(b"\x89PNG\r\n\x1A\n"));

    module.define_format(
        "png.main",
        record([
            ("signature", png_signature.call()),
            ("ihdr", ihdr.call()),
            ("chunks", repeat(png_chunk.call_args(vec![var("ihdr")]))),
            ("idat", repeat1(idat.call())),
            (
                "more-chunks",
                repeat(png_chunk.call_args(vec![var("ihdr")])),
            ),
            ("iend", iend.call()),
        ]),
    )
}
