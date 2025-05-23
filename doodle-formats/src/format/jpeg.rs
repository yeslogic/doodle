use crate::format::BaseModule;
use doodle::helper::*;
use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

/// JPEG File Interchange Format
///
/// - [JPEG File Interchange Format Version 1.02](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
/// - [ITU T.81 | ISO IEC 10918-1](https://www.w3.org/Graphics/JPEG/itu-t81.pdf)
pub fn main(module: &mut FormatModule, base: &BaseModule, tiff: &FormatRef) -> FormatRef {
    fn marker(id: u8) -> Format {
        record([("ff", is_byte(0xFF)), ("marker", is_byte(id))])
    }

    let marker_segment = |id: u8, data: Format| {
        record([
            ("marker", marker(id)),
            ("length", base.u16be()),
            ("data", slice(sub(var("length"), Expr::U16(2)), data)),
        ])
    };

    // NOTE -  Bit data: { horizontal <- u4, vertical <- u4 }
    // FIXME[epic=refactor] - replace with bit_fields_u8
    let sampling_factor = packed_bits_u8([4, 4], ["horizontal", "vertical"]);

    // SOF_n: Frame header (See ITU T.81 Section B.2.2)
    let sof_data = {
        let sof_image_component = module.define_format(
            "jpeg.sof-image-component",
            record([
                ("id", base.u8()), // NOTE: should be distinct from all other ids in the repetition
                ("sampling-factor", sampling_factor),
                (
                    "quantization-table-id",
                    where_lambda(base.u8(), "x", expr_lte(var("x"), Expr::U8(3))),
                ), // 0..=3 if DQT, 0 if lossless
            ]),
        );

        module.define_format(
            "jpeg.sof-data",
            record([
                ("sample-precision", where_between_u8(base.u8(), 2, 16)), // 8 in Sequential DCT (extended allows 12), 8 or 12 in Progressive DCT, 2-16 lossless
                ("num-lines", base.u16be()),
                ("num-samples-per-line", where_nonzero::<U16>(base.u16be())),
                ("num-image-components", where_nonzero::<U8>(base.u8())), // 1..=4 if progressive DCT, 1..=255 otherwise
                (
                    "image-components",
                    repeat_count(var("num-image-components"), sof_image_component.call()),
                ),
            ]),
        )
    };

    // NOTE - bit data (common bit-packed record between DHT and DAC, up to and including numeric constraints)
    // class <- u4 = 0 | 1;
    // table-id <- u4 = 0 |..| 3;
    let class_table_id = where_lambda(
        // FIXME[epic=refactor] - replace with bit_fields_u8
        packed_bits_u8([4, 4], ["class", "table-id"]),
        "class-table-id",
        and(
            expr_lt(record_proj(var("class-table-id"), "class"), Expr::U8(2)),
            expr_lt(record_proj(var("class-table-id"), "table-id"), Expr::U8(4)),
        ),
    );

    // DHT: Define Huffman table (See ITU T.81 Section B.2.4.2)
    let dht_data = module.define_format(
        "jpeg.dht-data",
        record([
            ("class-table-id", class_table_id.clone()),
            ("num-codes", repeat_count(Expr::U8(16), base.u8())),
            (
                "values",
                for_each(var("num-codes"), "n", repeat_count(var("n"), base.u8())),
            ),
        ]),
    );

    // DAC: Define arithmetic conditioning table (See ITU T.81 Section B.2.4.3)
    let dac_data = module.define_format(
        "jpeg.dac-data",
        record([("class-table-id", class_table_id), ("value", base.u8())]),
    );

    // NOTE - packed-bits field
    // dc-entropy-coding-table-id <- u4 //= 0 | .. | 3 (restricted to 0 | 1 when baseline sequential DCT)
    // ac-entropy-coding-table-id <- u4 //= 0 | .. | 3 (restricted to 0 | 1 when baseline sequential DCT, or simply 0 when lossless)
    let entropy_coding_table_ids = where_lambda(
        // FIXME[epic=refactor] - replace with bit_fields_u8
        packed_bits_u8(
            [4, 4],
            ["dc-entropy-coding-table-id", "ac-entropy-coding-table-id"],
        ),
        "entropy-coding-table-ids",
        and(
            expr_lte(
                record_proj(
                    var("entropy-coding-table-ids"),
                    "dc-entropy-coding-table-id",
                ),
                Expr::U8(3),
            ),
            expr_lte(
                record_proj(
                    var("entropy-coding-table-ids"),
                    "ac-entropy-coding-table-id",
                ),
                Expr::U8(3),
            ),
        ),
    );

    // SOS: Scan header (See ITU T.81 Section B.2.3)
    let sos_data = {
        let sos_image_component = module.define_format(
            "jpeg.sos-image-component",
            record([
                ("component-selector", base.u8()), // NOTE: should all be distinct members of the set of `id` values in `jpeg.sof-image-component`
                ("entropy-coding-table-ids", entropy_coding_table_ids),
            ]),
        );

        // NOTE: Bit data: { high <- u4, low <- u4 }
        // FIXME[epic=refactor] - replace with bit_fields_u8
        let approximation_bit_position = packed_bits_u8([4, 4], ["high", "low"]);

        module.define_format(
            "jpeg.sos-data",
            record([
                ("num-image-components", where_between_u8(base.u8(), 1, 4)), // 1 |..| 4
                (
                    "image-components",
                    repeat_count(var("num-image-components"), sos_image_component.call()),
                ),
                (
                    "start-spectral-selection",
                    where_between_u8(base.u8(), 0, 63),
                ), // FIXME -  0 in sequential DCT, 0..=63 in progressive DCT, 1-7 in lossless but 0 for lossless differential frames in hierarchical mode
                ("end-spectral-selection", where_between_u8(base.u8(), 0, 63)), // FIXME - 63 in sequential DCT, start..=63 in in progressive DCT (but 0 if start is 0), 0 in lossless (differential or otherwise)
                ("approximation-bit-position", approximation_bit_position),
            ]),
        )
    };

    // NOTE - bits data
    // precision <- u4 //= 0 | 1;
    // table-id <- u4 //= 0 |..| 3;
    let precision_table_id = where_lambda(
        // FIXME[epic=refactor] - replace with bit_fields_u8
        packed_bits_u8([4, 4], ["precision", "table-id"]),
        "precision-table-id",
        and(
            expr_lte(
                record_proj(var("precision-table-id"), "precision"),
                Expr::U8(1),
            ),
            expr_lte(
                record_proj(var("precision-table-id"), "table-id"),
                Expr::U8(3),
            ),
        ),
    );

    // DQT: Define quantization table (See ITU T.81 Section B.2.4.1)
    let dqt_data = module.define_format(
        "jpeg.dqt-data",
        record([
            ("precision-table-id", precision_table_id),
            // NOTE - conditional semantics on precision field:
            // elements <- match precision {
            //   0 => repeat-count 64 u8,
            //   1 => repeat-count 64 u16be,
            // };
            (
                "elements",
                match_variant(
                    record_proj(var("precision-table-id"), "precision"),
                    [
                        (
                            Pattern::U8(0),
                            "Bytes",
                            repeat_count(Expr::U32(64), base.u8()),
                        ),
                        (
                            Pattern::U8(1),
                            "Shorts",
                            repeat_count(Expr::U32(64), base.u16be()),
                        ),
                    ],
                ),
            ),
        ]),
    );

    // DNL: Define number of lines (See ITU T.81 Section B.2.5)
    let dnl_data = module.define_format(
        "jpeg.dnl-data",
        record([("num-lines", where_nonzero::<U16>(base.u16be()))]),
    );

    // DRI: Define restart interval (See ITU T.81 Section B.2.4.4)
    let dri_data = module.define_format(
        "jpeg.dri-data",
        record([("restart-interval", base.u16be())]),
    );

    // NOTE: Bit data: { horizontal <- u4, vertical <- u4 }
    // FIXME[epic=refactor] - replace with bit_fields_u8
    let sampling_factor = packed_bits_u8([4, 4], ["horizontal", "vertical"]);

    // DHP: Define hierarchial progression (See ITU T.81 Section B.3.2)
    // NOTE: Same as SOF except for quantization-table-id
    let dhp_data = {
        let dhp_image_component = module.define_format(
            "jpeg.dhp-image-component",
            record([
                ("id", base.u8()),
                ("sampling-factor", sampling_factor),
                ("quantization-table-id", is_byte(0)),
            ]),
        );

        module.define_format(
            "jpeg.dhp-data",
            record([
                ("sample-precision", base.u8()),
                ("num-lines", base.u16be()),
                ("num-samples-per-line", where_nonzero::<U16>(base.u16be())), // != 0
                ("num-image-components", where_nonzero::<U8>(base.u8())),     // != 0
                (
                    "image-components",
                    repeat_count(var("num-image-components"), dhp_image_component.call()),
                ),
            ]),
        )
    };

    // NOTE: Bit data
    // expand-horizontal <- u4 // 0 | 1;
    // expand-vertical <- u4 // 0 | 1;
    let expand_horizontal_vertical = where_lambda(
        // FIXME[epic=refactor] - replace with bit_fields_u8
        packed_bits_u8([4, 4], ["expand-horizontal", "expand-vertical"]),
        "x",
        and(
            expr_lte(record_proj(var("x"), "expand-horizontal"), Expr::U8(1)),
            expr_lte(record_proj(var("x"), "expand-vertical"), Expr::U8(1)),
        ),
    );

    // EXP: Expand reference components (See ITU T.81 Section B.3.3)
    let exp_data = module.define_format(
        "jpeg.exp-data",
        record([("expand-horizontal-vertical", expand_horizontal_vertical)]),
    );

    // APP0: Application segment 0 (JFIF)
    let app0_jfif = {
        let thumbnail_pixel = module.define_format(
            "jpeg.thumbnail-pixel",
            record([("r", base.u8()), ("g", base.u8()), ("b", base.u8())]),
        );

        module.define_format(
            "jpeg.app0-jfif",
            record([
                ("version-major", base.u8()),
                ("version-minor", base.u8()),
                (
                    "density-units",
                    where_lambda(base.u8(), "x", expr_lte(var("x"), Expr::U8(2))),
                ), // 0 | 1 | 2
                ("density-x", where_nonzero::<U16>(base.u16be())), // != 0
                ("density-y", where_nonzero::<U16>(base.u16be())), // != 0
                ("thumbnail-width", base.u8()),
                ("thumbnail-height", base.u8()),
                (
                    "thumbnail-pixels",
                    repeat_count(
                        var("thumbnail-height"),
                        repeat_count(var("thumbnail-width"), thumbnail_pixel.call()),
                    ),
                ),
            ]),
        )
    };

    let app0_data = module.define_format(
        "jpeg.app0-data",
        record([
            ("identifier", base.asciiz_string()),
            (
                "data",
                match_variant(
                    record_proj(var("identifier"), "string"),
                    vec![
                        (Pattern::from_bytes(b"JFIF"), "jfif", app0_jfif.call()),
                        // FIXME: there are other APP0 formats
                        // TODO: implement JFXX, CIFF, AVI1, Ocad
                        // see https://exiftool.org/TagNames/JPEG.html
                        (Pattern::Wildcard, "other", repeat(base.u8())),
                    ],
                ),
            ),
        ]),
    );

    // APP1: Application segment 1 (EXIF)
    //
    // - [Exif Version 2.32, Section 4.5.4](https://www.cipa.jp/std/documents/e/DC-X008-Translation-2019-E.pdf#page=24)
    let app1_exif = module.define_format(
        "jpeg.app1-exif",
        record([("padding", is_byte(0x00)), ("exif", tiff.call())]),
    );

    // APP1: Application segment 1 (XMP)
    // TODO[epic=refinement] - implement APP1 XMP header as non-opaque format, if feasible
    let app1_xmp = module.define_format("jpeg.app1-xmp", record([("xmp", repeat(base.u8()))]));

    let app1_data = module.define_format(
        "jpeg.app1-data",
        record([
            ("identifier", base.asciiz_string()),
            (
                "data",
                match_variant(
                    record_proj(var("identifier"), "string"),
                    vec![
                        (Pattern::from_bytes(b"Exif"), "exif", app1_exif.call()),
                        (
                            Pattern::from_bytes(b"http://ns.adobe.com/xap/1.0/"),
                            "xmp",
                            app1_xmp.call(),
                        ),
                        // FIXME: there are other APP1 formats
                        // TODO: implement
                        // see https://exiftool.org/TagNames/JPEG.html
                        (Pattern::Wildcard, "other", repeat(base.u8())),
                    ],
                ),
            ),
        ]),
    );

    let sof0 = module.define_format("jpeg.sof0", marker_segment(0xC0, sof_data.call())); // Start of frame (baseline jpeg)
    let sof1 = module.define_format("jpeg.sof1", marker_segment(0xC1, sof_data.call())); // Start of frame (extended sequential, huffman)
    let sof2 = module.define_format("jpeg.sof2", marker_segment(0xC2, sof_data.call())); // Start of frame (progressive, huffman)
    let sof3 = module.define_format("jpeg.sof3", marker_segment(0xC3, sof_data.call())); // Start of frame (lossless, huffman)
    let dht = module.define_format("jpeg.dht", marker_segment(0xC4, dht_data.call())); // Define Huffman Table
    let sof5 = module.define_format("jpeg.sof5", marker_segment(0xC5, sof_data.call())); // Start of frame (differential sequential, huffman)
    let sof6 = module.define_format("jpeg.sof6", marker_segment(0xC6, sof_data.call())); // Start of frame (differential progressive, huffman)
    let sof7 = module.define_format("jpeg.sof7", marker_segment(0xC7, sof_data.call())); // Start of frame (differential lossless, huffman)
    let _jpeg = module.define_format("jpeg.jpeg", marker_segment(0xC8, repeat(base.u8()))); // Reserved for JPEG extension
    let sof9 = module.define_format("jpeg.sof9", marker_segment(0xC9, sof_data.call())); // Start of frame (extended sequential, arithmetic)
    let sof10 = module.define_format("jpeg.sof10", marker_segment(0xCA, sof_data.call())); // Start of frame (progressive, arithmetic)
    let sof11 = module.define_format("jpeg.sof11", marker_segment(0xCB, sof_data.call())); // Start of frame (lossless, arithmetic)
    let dac = module.define_format("jpeg.dac", marker_segment(0xCC, dac_data.call())); // Define arithmetic coding conditioning
    let sof13 = module.define_format("jpeg.sof13", marker_segment(0xCD, sof_data.call())); // Start of frame (differential sequential, arithmetic)
    let sof14 = module.define_format("jpeg.sof14", marker_segment(0xCE, sof_data.call())); // Start of frame (differential progressive, arithmetic)
    let sof15 = module.define_format("jpeg.sof15", marker_segment(0xCF, sof_data.call())); // Start of frame (differential lossless, arithmetic)
    let rst0 = module.define_format("jpeg.rst0", marker(0xD0)); // Restart marker 0
    let rst1 = module.define_format("jpeg.rst1", marker(0xD1)); // Restart marker 1
    let rst2 = module.define_format("jpeg.rst2", marker(0xD2)); // Restart marker 2
    let rst3 = module.define_format("jpeg.rst3", marker(0xD3)); // Restart marker 3
    let rst4 = module.define_format("jpeg.rst4", marker(0xD4)); // Restart marker 4
    let rst5 = module.define_format("jpeg.rst5", marker(0xD5)); // Restart marker 5
    let rst6 = module.define_format("jpeg.rst6", marker(0xD6)); // Restart marker 6
    let rst7 = module.define_format("jpeg.rst7", marker(0xD7)); // Restart marker 7
    let soi = module.define_format("jpeg.soi", marker(0xD8)); // Start of image
    let eoi = module.define_format("jpeg.eoi", marker(0xD9)); // End of of image
    let sos = module.define_format("jpeg.sos", marker_segment(0xDA, sos_data.call())); // Start of scan
    let dqt = module.define_format("jpeg.dqt", marker_segment(0xDB, repeat1(dqt_data.call()))); // Define quantization table
    let dnl = module.define_format("jpeg.dnl", marker_segment(0xDC, dnl_data.call())); // Define number of lines
    let dri = module.define_format("jpeg.dri", marker_segment(0xDD, dri_data.call())); // Define restart interval
    let _dhp = module.define_format("jpeg.dhp", marker_segment(0xDE, dhp_data.call())); // Define hierarchical progression
    let _exp = module.define_format("jpeg.exp", marker_segment(0xDF, exp_data.call())); // Expand reference components
    let app0 = module.define_format("jpeg.app0", marker_segment(0xE0, app0_data.call())); // Application segment 0 (JFIF/JFXX/AVI1/...)
    let app1 = module.define_format("jpeg.app1", marker_segment(0xE1, app1_data.call())); // Application segment 1 (EXIF/XMP/XAP/...)
    let app2 = module.define_format("jpeg.app2", marker_segment(0xE2, repeat(base.u8()))); // Application segment 2 (FlashPix/ICC/...)
    let app3 = module.define_format("jpeg.app3", marker_segment(0xE3, repeat(base.u8()))); // Application segment 3 (Kodak/...)
    let app4 = module.define_format("jpeg.app4", marker_segment(0xE4, repeat(base.u8()))); // Application segment 4 (FlashPix/...)
    let app5 = module.define_format("jpeg.app5", marker_segment(0xE5, repeat(base.u8()))); // Application segment 5 (Ricoh/...)
    let app6 = module.define_format("jpeg.app6", marker_segment(0xE6, repeat(base.u8()))); // Application segment 6 (GoPro/...)
    let app7 = module.define_format("jpeg.app7", marker_segment(0xE7, repeat(base.u8()))); // Application segment 7 (Pentax/Qualcomm/...)
    let app8 = module.define_format("jpeg.app8", marker_segment(0xE8, repeat(base.u8()))); // Application segment 8 (Spiff/...)
    let app9 = module.define_format("jpeg.app9", marker_segment(0xE9, repeat(base.u8()))); // Application segment 9 (MediaJukebox/...)
    let app10 = module.define_format("jpeg.app10", marker_segment(0xEA, repeat(base.u8()))); // Application segment 10 (PhotoStudio)
    let app11 = module.define_format("jpeg.app11", marker_segment(0xEB, repeat(base.u8()))); // Application segment 11 (HDR)
    let app12 = module.define_format("jpeg.app12", marker_segment(0xEC, repeat(base.u8()))); // Application segment 12 (PictureInfo/Ducky)
    let app13 = module.define_format("jpeg.app13", marker_segment(0xED, repeat(base.u8()))); // Application segment 13 (PhotoShop/Adobe_CM)
    let app14 = module.define_format("jpeg.app14", marker_segment(0xEE, repeat(base.u8()))); // Application segment 14 (Adobe)
    let app15 = module.define_format("jpeg.app15", marker_segment(0xEF, repeat(base.u8()))); // Application segment 15 (GraphicConverter)
    let com = module.define_format("jpeg.com", marker_segment(0xFE, repeat(base.u8()))); // Extension data (comment)

    let table_or_misc = module.define_format(
        "jpeg.table-or-misc",
        alts([
            ("dqt", dqt.call()), // Define quantization table
            ("dht", dht.call()), // Define Huffman Table
            ("dac", dac.call()), // Define arithmetic coding conditioning
            ("dri", dri.call()), // Define restart interval
            ("app0", app0.call()),
            ("app1", app1.call()),
            ("app2", app2.call()),
            ("app3", app3.call()),
            ("app4", app4.call()),
            ("app5", app5.call()),
            ("app6", app6.call()),
            ("app7", app7.call()),
            ("app8", app8.call()),
            ("app9", app9.call()),
            ("app10", app10.call()),
            ("app11", app11.call()),
            ("app12", app12.call()),
            ("app13", app13.call()),
            ("app14", app14.call()),
            ("app15", app15.call()),
            ("com", com.call()), // Comment
        ]),
    );

    let frame_header = module.define_format(
        "jpeg.frame-header",
        alts([
            ("sof0", sof0.call()),
            ("sof1", sof1.call()),
            ("sof2", sof2.call()),
            ("sof3", sof3.call()),
            ("sof5", sof5.call()),
            ("sof6", sof6.call()),
            ("sof7", sof7.call()),
            ("sof9", sof9.call()),
            ("sof10", sof10.call()),
            ("sof11", sof11.call()),
            ("sof13", sof13.call()),
            ("sof14", sof14.call()),
            ("sof15", sof15.call()),
        ]),
    );

    // MCU: Minimum coded unit
    let mcu = module.define_format(
        "jpeg.mcu",
        Format::Union(vec![
            not_byte(0xFF),
            map(
                tuple([is_byte(0xFF), is_byte(0x00)]),
                lambda("_", Expr::U8(0xFF)),
            ),
        ]),
    );

    // A series of entropy coded segments separated by restart markers
    let scan_data = module.define_format(
        "jpeg.scan-data",
        record([
            (
                "scan-data",
                repeat(alts([
                    // FIXME: Extract into separate ECS repetition
                    ("mcu", mcu.call()), // TODO: repeat(mcu),
                    // FIXME: Restart markers should cycle in order from rst0-rst7
                    ("rst0", rst0.call()),
                    ("rst1", rst1.call()),
                    ("rst2", rst2.call()),
                    ("rst3", rst3.call()),
                    ("rst4", rst4.call()),
                    ("rst5", rst5.call()),
                    ("rst6", rst6.call()),
                    ("rst7", rst7.call()),
                ])),
            ),
            (
                "scan-data-stream",
                compute(flat_map(
                    lambda(
                        "x",
                        expr_match(
                            var("x"),
                            vec![
                                (
                                    Pattern::variant("mcu", Pattern::binding("v")),
                                    Expr::Seq(vec![var("v")]),
                                ),
                                (
                                    Pattern::variant("rst0", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                                (
                                    Pattern::variant("rst1", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                                (
                                    Pattern::variant("rst2", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                                (
                                    Pattern::variant("rst3", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                                (
                                    Pattern::variant("rst4", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                                (
                                    Pattern::variant("rst5", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                                (
                                    Pattern::variant("rst6", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                                (
                                    Pattern::variant("rst7", Pattern::Wildcard),
                                    Expr::Seq(vec![]),
                                ),
                            ],
                        ),
                    ),
                    var("scan-data"),
                )),
            ),
        ]),
    );

    let scan = module.define_format(
        "jpeg.scan",
        record([
            ("segments", repeat(table_or_misc.call())),
            ("sos", sos.call()),
            ("data", scan_data.call()),
        ]),
    );

    let frame = module.define_format(
        "jpeg.frame",
        record([
            (
                "initial-segment",
                alts([("app0", app0.call()), ("app1", app1.call())]),
            ),
            ("segments", repeat(table_or_misc.call())),
            ("header", frame_header.call()),
            ("scan", scan.call()),
            ("dnl", optional(dnl.call())),
            ("scans", repeat(scan.call())),
        ]),
    );

    module.define_format(
        "jpeg.main",
        record([
            ("soi", soi.call()),
            ("frame", frame.call()),
            ("eoi", eoi.call()),
        ]),
    )
}
