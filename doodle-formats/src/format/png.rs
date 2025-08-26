use doodle::byte_set::ByteSet;
use doodle::helper::*;
use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

fn null_terminated(f: Format) -> Format {
    chain(f, "val", monad_seq(is_byte(0), compute(var("val"))))
}

pub fn main(
    module: &mut FormatModule,
    zlib: FormatRef,
    utf8text: FormatRef,
    utf8text_nz: FormatRef,
) -> FormatRef {
    let chunk = |tag: Format, data: Format| {
        record([
            (
                "length",
                where_lambda(
                    u32be(),
                    "length",
                    expr_lte(var("length"), Expr::U32(0x7fff_ffff)),
                ),
            ), // NOTE: < 2^31
            ("tag", tag),
            ("data", slice(var("length"), data)),
            ("crc", u32be()), // REVIEW - do we want to attempt to validate this?
        ])
    };
    let chunk_unit = |tag: Format| {
        record([
            (
                "length",
                where_lambda(u32be(), "length", expr_eq(var("length"), Expr::U32(0))),
            ),
            ("tag", tag),
            ("crc", u32be()), // REVIEW - do we want to attempt to validate this?
        ])
    };

    // PNG keyword for iTXt, zTXt, tEXt, and other contexts
    //   - Length >= 1, < 80 characters
    //   - Consists only of Latin-1 characters and spaces: 32..=126 | 161..=255
    //   - No leading or trailing spaces, nor consecutive spaces
    //   - Non-breaking space (160) not permitted in particular
    let keyword = module.define_format(
        "png.keyword",
        // TODO - all we can enforce for now without more complex logic is the character set, space-rules are not something we can enforce easily
        // REVIEW - should we wrap this in mk_ascii_str (?)
        repeat_between(
            Expr::U32(1),
            Expr::U32(79),
            byte_in(ByteSet::from(32..=126).union(&ByteSet::from(161..=255))),
        ),
    );

    // let any_tag = module.define_format(
    //     "png.any-tag",
    //     tuple([u8(), u8(), u8(), u8()]), // FIXME: ASCII
    // );

    let ihdr_tag = module.define_format("png.ihdr-tag", is_bytes(b"IHDR"));
    let ihdr_data = module.define_format(
        "png.ihdr-data",
        record([
            ("width", u32be()),
            ("height", u32be()),
            ("bit-depth", u8()),
            ("color-type", u8()),
            ("compression-method", u8()),
            ("filter-method", u8()),
            ("interlace-method", u8()),
        ]),
    );
    let ihdr = module.define_format("png.ihdr", chunk(ihdr_tag.call(), ihdr_data.call()));
    let ihdr_type = module.get_format_type(ihdr.get_level()).clone();

    let idat_tag = module.define_format("png.idat-tag", is_bytes(b"IDAT"));
    let idat_data = module.define_format("png.idat-data", opaque_bytes());
    let idat = module.define_format("png.idat", chunk(idat_tag.call(), idat_data.call()));

    let iend_tag = module.define_format("png.iend-tag", is_bytes(b"IEND"));
    let iend = module.define_format("png.iend", chunk_unit(iend_tag.call()));

    let bkgd = module.define_format_args(
        "png.bkgd",
        vec![("ihdr".into(), ihdr_type.clone())],
        match_variant(
            record_proj(record_proj(var("ihdr"), "data"), "color-type"),
            vec![
                (
                    Pattern::U8(0),
                    "color-type-0",
                    record([("greyscale", u16be())]),
                ),
                (
                    Pattern::U8(4),
                    "color-type-4",
                    record([("greyscale", u16be())]),
                ),
                (
                    Pattern::U8(2),
                    "color-type-2",
                    record_repeat(["red", "green", "blue"], u16be()),
                ),
                (
                    Pattern::U8(6),
                    "color-type-6",
                    record_repeat(["red", "green", "blue"], u16be()),
                ),
                (
                    Pattern::U8(3),
                    "color-type-3",
                    record([("palette-index", u8())]),
                ),
            ],
        ),
    );

    let chrm = module.define_format(
        "png.chrm",
        record(vec![
            ("whitepoint-x", u32be()),
            ("whitepoint-y", u32be()),
            ("red-x", u32be()),
            ("red-y", u32be()),
            ("green-x", u32be()),
            ("green-y", u32be()),
            ("blue-x", u32be()),
            ("blue-y", u32be()),
        ]),
    );

    // REVIEW: do we want to map the value to its intended scale (y := x / 100_000)?
    let gama = module.define_format("png.gama", record(vec![("gamma", u32be())]));

    let zlib_utf8text = chain(
        zlib.call(),
        "zlib",
        Format::DecodeBytes(
            Box::new(record_lens(var("zlib"), &["data", "inflate"])),
            Box::new(utf8text_nz.call()),
        ),
    );

    let itxt = module.define_format(
        "png.itxt",
        record([
            ("keyword", null_terminated(keyword.call())),
            ("compression-flag", byte_in([0, 1])),
            ("compression-method", is_byte(0)),
            ("language-tag", asciiz_string()), // REVIEW - there are specific rules to this (1-8--character ascii-only words separated by hyphens)
            ("translated-keyword", null_terminated(utf8text_nz.call())),
            ("text", {
                if_then_else(
                    Expr::IntRel(
                        doodle::IntRel::Eq,
                        Box::new(var("compression-flag")),
                        Box::new(Expr::U8(1)),
                    ),
                    Format::UnionNondet(vec![
                        fmt_variant("compressed", fmt_variant("valid", zlib_utf8text)),
                        fmt_variant("compressed", fmt_variant("invalid", opaque_bytes())),
                    ]),
                    Format::Variant("uncompressed".into(), Box::new(utf8text.call())),
                )
            }),
        ]),
    );

    let iccp = module.define_format(
        "png.iccp",
        record(vec![
            ("profile-name", null_terminated(keyword.call())),
            ("compression-method", is_byte(0)), // NOTE: 0 := deflate is the only defined value
            ("compressed-profile", zlib.call()),
        ]),
    );

    let phys = module.define_format(
        "png.phys",
        record([
            ("pixels-per-unit-x", u32be()),
            ("pixels-per-unit-y", u32be()),
            ("unit-specifier", u8()),
        ]),
    );

    let plte = module.define_format("png.plte", repeat1(record_repeat(["r", "g", "b"], u8())));

    let text = module.define_format(
        "png.text",
        record([
            ("keyword", null_terminated(keyword.call())),
            ("text", mk_ascii_string(repeat(ascii_char()))),
        ]),
    );

    let time = module.define_format(
        "png.time",
        record([
            ("year", u16be()),
            ("month", u8()),
            ("day", u8()),
            ("hour", u8()),
            ("minute", u8()),
            ("second", u8()),
        ]),
    );

    let trns = module.define_format_args(
        "png.trns",
        vec![("ihdr".into(), ihdr_type.clone())],
        match_variant(
            record_proj(record_proj(var("ihdr"), "data"), "color-type"),
            vec![
                (
                    Pattern::U8(0),
                    "color-type-0",
                    record([("greyscale", u16be())]),
                ),
                (
                    Pattern::U8(2),
                    "color-type-2",
                    record_repeat(["red", "green", "blue"], u16be()),
                ),
                (
                    Pattern::U8(3),
                    "color-type-3",
                    repeat(record([("palette-index", u8())])),
                ),
            ],
        ),
    );

    let zlib_latin1 = chain(
        zlib.call(),
        "zlib",
        Format::DecodeBytes(
            Box::new(record_lens(var("zlib"), &["data", "inflate"])),
            // TODO - we need to define a new format for latin1 without a null terminal (viz. why asciiz_string won't work)
            Box::new(utf8text.call()),
        ),
    );

    // rendering intent constants for sRGB
    const RENDINT_PERCEPTUAL: u8 = 0; // perceptual
    #[allow(dead_code)]
    const RENDINT_RELCOLOR: u8 = 1; // relative colorimetric
    #[allow(dead_code)]
    const RENDINT_SATURATION: u8 = 2; // saturation
    const RENDINT_ABSCOLOR: u8 = 3; // absolute colorimetric

    let srgb = module.define_format(
        "png.srgb",
        record([(
            "rendering-intent",
            where_between_u8(u8(), RENDINT_PERCEPTUAL, RENDINT_ABSCOLOR),
        )]),
    );

    let sbit = module.define_format_args(
        "png.sbit",
        vec![("ihdr".into(), ihdr_type.clone())],
        match_variant(
            record_proj(record_proj(var("ihdr"), "data"), "color-type"),
            vec![
                (
                    Pattern::U8(0),
                    "color-type-0",
                    record([("sig-greyscale-bits", u8())]),
                ),
                (
                    Pattern::U8(2),
                    "color-type-2",
                    record([
                        ("sig-red-bits", u8()),
                        ("sig-green-bits", u8()),
                        ("sig-blue-bits", u8()),
                    ]),
                ),
                (
                    Pattern::U8(3),
                    "color-type-3",
                    record([
                        ("sig-red-bits", u8()),
                        ("sig-green-bits", u8()),
                        ("sig-blue-bits", u8()),
                    ]),
                ),
                (
                    Pattern::U8(4),
                    "color-type-4",
                    record([("sig-greyscale-bits", u8()), ("sig-alpha-bits", u8())]),
                ),
                (
                    Pattern::U8(6),
                    "color-type-6",
                    record([
                        ("sig-red-bits", u8()),
                        ("sig-green-bits", u8()),
                        ("sig-blue-bits", u8()),
                        ("sig-alpha-bits", u8()),
                    ]),
                ),
            ],
        ),
    );

    let ztxt = module.define_format(
        "png.ztxt",
        record([
            ("keyword", null_terminated(keyword.call())),
            ("compression-method", is_byte(0)),
            ("compressed-text", zlib_latin1),
        ]),
    );

    // NOTE - intended to correspond to PLTE data (?)
    let hist = module.define_format("png.hist", record([("histogram", repeat(u16be()))]));

    let palette_entries = |depth: Expr| {
        // NOTE - the only constraint on the sequence of entries (aside from implicitly sharing the same depth) is that they are in descending frequency order
        match_variant(
            depth,
            [
                (
                    Pattern::U8(8),
                    "sample-depth-u8",
                    repeat(record([
                        ("red", u8()),
                        ("green", u8()),
                        ("blue", u8()),
                        ("alpha", u8()),
                        ("frequency", u16be()),
                    ])),
                ),
                (
                    Pattern::U8(16),
                    "sample-depth-u16",
                    repeat(record([
                        ("red", u16be()),
                        ("green", u16be()),
                        ("blue", u16be()),
                        ("alpha", u16be()),
                        ("frequency", u16be()),
                    ])),
                ),
            ],
        )
    };

    let splt = module.define_format(
        "png.splt",
        record([
            ("palette-name", null_terminated(keyword.call())),
            // Sample depth is 8 or 16
            ("sample-depth", byte_in([8, 16])),
            ("palette", palette_entries(var("sample-depth"))),
        ]),
    );

    let png_tag = png_tag(module);

    let png_chunk = module.define_format_args(
        "png.chunk",
        vec![("ihdr".into(), ihdr_type)],
        record([
            (
                "length",
                where_lambda(
                    u32be(),
                    "length",
                    expr_lte(var("length"), Expr::U32(0x7fff_ffff)),
                ),
            ),
            ("tag", png_tag.call()),
            (
                "data",
                Format::Slice(
                    Box::new(var("length")),
                    Box::new(match_variant(
                        var("tag"),
                        [
                            (pattern_bytestring(b"PLTE"), "PLTE", plte.call()),
                            (
                                pattern_bytestring(b"tRNS"),
                                "tRNS",
                                trns.call_args(vec![var("ihdr")]),
                            ),
                            (pattern_bytestring(b"cHRM"), "cHRM", chrm.call()),
                            (pattern_bytestring(b"gAMA"), "gAMA", gama.call()),
                            (pattern_bytestring(b"iCCP"), "iCCP", iccp.call()),
                            (
                                pattern_bytestring(b"sBIT"),
                                "sBIT",
                                sbit.call_args(vec![var("ihdr")]),
                            ),
                            (pattern_bytestring(b"sRGB"), "sRGB", srgb.call()),
                            (pattern_bytestring(b"iTXt"), "iTXt", itxt.call()),
                            (pattern_bytestring(b"tEXt"), "tEXt", text.call()),
                            (pattern_bytestring(b"zTXt"), "zTXt", ztxt.call()),
                            (
                                pattern_bytestring(b"bKGD"),
                                "bKGD",
                                bkgd.call_args(vec![var("ihdr")]),
                            ),
                            (pattern_bytestring(b"hIST"), "hIST", hist.call()), // TODO - hist can only occur when there is a PLTE chunk to correspond it to
                            (pattern_bytestring(b"pHYs"), "pHYs", phys.call()),
                            (pattern_bytestring(b"sPLT"), "sPLT", splt.call()),
                            (pattern_bytestring(b"tIME"), "tIME", time.call()),
                            (Pattern::Wildcard, "unknown", opaque_bytes()), // TODO - preserve an artefact of the unknown tag
                        ],
                    )),
                ),
            ),
            ("crc", u32be()), // REVIEW - do we want to attempt to validate this?
        ]),
    );

    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1A\n";

    module.define_format(
        "png.main",
        record_auto([
            ("signature", byte_seq(PNG_SIGNATURE)),
            ("ihdr", ihdr.call()),
            ("chunks", repeat(png_chunk.call_args(vec![var("ihdr")]))),
            (
                "idat",
                chain(
                    map(
                        repeat1(idat.call()),
                        lambda(
                            "xs",
                            flat_map(lambda("x", record_proj(var("x"), "data")), var("xs")),
                        ),
                    ),
                    "idat",
                    Format::DecodeBytes(Box::new(var("idat")), Box::new(zlib.call())),
                ),
            ),
            (
                "more-chunks",
                repeat(png_chunk.call_args(vec![var("ihdr")])),
            ),
            ("iend", iend.call()),
        ]),
    )
}

pub fn png_tag(module: &mut FormatModule) -> FormatRef {
    let anti_pattern = any_of([is_bytes(b"IDAT"), is_bytes(b"IEND")]);
    module.define_format(
        "png.tag",
        excluding(anti_pattern, fixed_len_string(ascii_alpha(), 4)),
    )
}
