use doodle::helper::{u8, *};
use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

fn tag_pattern(tag: [char; 4]) -> Pattern {
    Pattern::Tuple(vec![
        Pattern::U8(tag[0] as u8),
        Pattern::U8(tag[1] as u8),
        Pattern::U8(tag[2] as u8),
        Pattern::U8(tag[3] as u8),
    ])
}

fn tag_pattern3(tag: [char; 3]) -> Pattern {
    Pattern::Tuple(vec![
        Pattern::U8(0xA9),
        Pattern::U8(tag[0] as u8),
        Pattern::U8(tag[1] as u8),
        Pattern::U8(tag[2] as u8),
    ])
}

pub fn main(module: &mut FormatModule) -> FormatRef {
    let tag = module.define_format("mpeg4.tag", tuple_repeat(4, ascii_char()));

    fn make_atom(tag: FormatRef, data: Format) -> Format {
        record([
            ("size-field", u32be()),
            ("type", tag.call()),
            (
                "size",
                Format::Match(
                    Box::new(var("size-field")),
                    vec![
                        (Pattern::U32(0), compute(Expr::U64(0))), // FIXME
                        (
                            Pattern::U32(1),
                            map(u64be(), lambda("x", sub(var("x"), Expr::U64(16)))),
                        ),
                        (
                            Pattern::Wildcard,
                            compute(as_u64(sub(var("size-field"), Expr::U32(8)))),
                        ),
                    ],
                ),
            ),
            ("data", slice(var("size"), data)),
        ])
    }

    let ftyp_data = record([
        ("major_brand", tag.call()),
        ("minor_version", u32be()),
        ("compatible_brands", repeat(tag.call())),
    ]);

    let mdia_hdlr_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("component_type", u32be()),
        ("component_subtype", tag.call()),
        ("component_manufacturer", u32be()),
        ("component_flags", u32be()),
        ("component_flags_mask", u32be()),
        ("component_name", asciiz_string()),
    ]);

    let meta_hdlr_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("predefined", u32be()),
        ("handler_type", tag.call()),
        ("reserved", tuple_repeat(3, u32be())),
        ("name", asciiz_string()),
    ]);

    let pitm_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        (
            "item_ID",
            if_then_else_variant(expr_eq(var("version"), Expr::U8(0)), u16be(), u32be()),
        ),
    ]);

    let infe_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        (
            "fields",
            if_then_else_variant(
                expr_lt(var("version"), Expr::U8(2)),
                record([
                    ("item_ID", u16be()),
                    ("item_protection_index", u16be()),
                    ("item_name", asciiz_string()),
                    ("content_type", asciiz_string()),
                    ("content_encoding", asciiz_string()),
                    // FIXME unfinished
                ]),
                record([
                    (
                        "item_ID",
                        if_then_else(
                            expr_eq(var("version"), Expr::U8(2)),
                            map(u16be(), lambda("x", as_u32(var("x")))),
                            u32be(),
                        ),
                    ),
                    ("item_protection_index", u16be()),
                    ("item_type", tag.call()),
                    ("item_name", asciiz_string()),
                    (
                        "extra_fields",
                        match_variant(
                            var("item_type"),
                            vec![
                                (
                                    tag_pattern(['m', 'i', 'm', 'e']),
                                    "mime",
                                    record([
                                        ("content_type", asciiz_string()),
                                        //FIXME optional ("content_encoding", asciiz_string()),
                                    ]),
                                ),
                                (
                                    tag_pattern(['u', 'r', 'i', ' ']),
                                    "uri",
                                    record([("item_uri_type", asciiz_string())]),
                                ),
                                (
                                    Pattern::Wildcard,
                                    "unknown",
                                    Format::EMPTY, // FIXME
                                ),
                            ],
                        ),
                    ),
                ]),
            ),
        ),
    ]);

    let iinf_atom = module.define_format(
        "mpeg4.iinf-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['i', 'n', 'f', 'e']), "infe", infe_data),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let iinf_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        (
            "entry_count",
            if_then_else(
                expr_eq(var("version"), Expr::U8(0)),
                map(u16be(), lambda("x", as_u32(var("x")))),
                u32be(),
            ),
        ),
        (
            "item_info_entry",
            repeat_count(var("entry_count"), iinf_atom.call()),
        ),
    ]);

    let single_item_reference_data = record([
        ("from_item_ID", u16be()),
        ("reference_count", u16be()),
        ("to_item_ID", repeat_count(var("reference_count"), u16be())),
    ]);

    let single_item_reference_large_data = record([
        ("from_item_ID", u32be()),
        ("reference_count", u16be()),
        ("to_item_ID", repeat_count(var("reference_count"), u32be())),
    ]);

    let iref_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        (
            "single_item_reference",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "small",
                        repeat(make_atom(tag, single_item_reference_data)),
                    ),
                    (
                        Pattern::U8(1),
                        "large",
                        Format::Repeat(Box::new(make_atom(tag, single_item_reference_large_data))),
                    ),
                ],
            ),
        ),
    ]);

    let iloc_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("offset_size_length_size", u8()), // two four-bit fields
        ("base_offset_size_index_size", u8()), // two four-bit fields
        (
            "offset_size",
            compute(shr(var("offset_size_length_size"), Expr::U8(4))),
        ),
        (
            "length_size",
            compute(bit_and(var("offset_size_length_size"), Expr::U8(7))),
        ),
        (
            "base_offset_size",
            compute(shr(var("base_offset_size_index_size"), Expr::U8(4))),
        ),
        (
            "index_size",
            if_then_else(
                expr_gt(var("version"), Expr::U8(0)),
                compute(bit_and(var("base_offset_size_index_size"), Expr::U8(7))),
                compute(Expr::U8(0)),
            ),
        ),
        (
            "item_count",
            if_then_else(
                expr_lt(var("version"), Expr::U8(2)),
                map(u16be(), lambda("x", as_u32(var("x")))),
                u32be(),
            ),
        ),
        (
            "items",
            repeat_count(
                var("item_count"),
                record([
                    (
                        "item_ID",
                        if_then_else(
                            expr_lt(var("version"), Expr::U8(2)),
                            map(u16be(), lambda("x", as_u32(var("x")))),
                            u32be(),
                        ),
                    ),
                    (
                        "construction_method",
                        cond_maybe(expr_gt(var("version"), Expr::U8(0)), u16be()),
                    ),
                    ("data_reference_index", u16be()),
                    (
                        "base_offset",
                        Format::Match(
                            Box::new(var("base_offset_size")),
                            vec![
                                (Pattern::U8(0), compute(Expr::U64(0))),
                                (Pattern::U8(4), map(u32be(), lambda("x", as_u64(var("x"))))),
                                (Pattern::U8(8), u64be()),
                            ],
                        ),
                    ),
                    ("extent_count", u16be()),
                    (
                        "extents",
                        repeat_count(
                            var("extent_count"),
                            record([
                                (
                                    "extent_index",
                                    Format::Match(
                                        Box::new(var("index_size")),
                                        vec![
                                            (Pattern::U8(0), compute(Expr::U64(0))),
                                            (
                                                Pattern::U8(4),
                                                map(u32be(), lambda("x", as_u64(var("x")))),
                                            ),
                                            (Pattern::U8(8), u64be()),
                                        ],
                                    ),
                                ),
                                (
                                    "extent_offset",
                                    Format::Match(
                                        Box::new(var("offset_size")),
                                        vec![
                                            (Pattern::U8(0), compute(Expr::U64(0))),
                                            (
                                                Pattern::U8(4),
                                                map(u32be(), lambda("x", as_u64(var("x")))),
                                            ),
                                            (Pattern::U8(8), u64be()),
                                        ],
                                    ),
                                ),
                                (
                                    "extent_length",
                                    Format::Match(
                                        Box::new(var("length_size")),
                                        vec![
                                            (Pattern::U8(0), compute(Expr::U64(0))),
                                            (
                                                Pattern::U8(4),
                                                map(u32be(), lambda("x", as_u64(var("x")))),
                                            ),
                                            (Pattern::U8(8), u64be()),
                                        ],
                                    ),
                                ),
                            ]),
                        ),
                    ),
                ]),
            ),
        ),
    ]);

    let dref_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("number_of_entries", u32be()),
        ("data", repeat(make_atom(tag, opaque_bytes()))),
    ]);

    let elst_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("number_of_entries", u32be()),
        (
            "edit_list_table",
            repeat_count(
                var("number_of_entries"),
                record([
                    ("track_duration", u32be()),
                    ("media_time", u32be()),
                    ("media_rate", u32be()),
                ]),
            ),
        ),
    ]);

    let stsd_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("entry_count", u32be()),
        (
            "sample_entries",
            repeat_count(var("entry_count"), make_atom(tag, opaque_bytes())),
        ),
    ]);

    let stts_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("entry_count", u32be()),
        (
            "sample_entries",
            repeat_count(
                var("entry_count"),
                record([("sample_count", u32be()), ("sample_delta", u32be())]),
            ),
        ),
    ]);

    let ctts_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("entry_count", u32be()),
        (
            "sample_entries",
            repeat_count(
                var("entry_count"),
                record([
                    ("sample_count", u32be()),
                    // FIXME signed if version == 1
                    ("sample_offset", u32be()),
                ]),
            ),
        ),
    ]);

    let stss_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("entry_count", u32be()),
        ("sample_number", repeat_count(var("entry_count"), u32be())),
    ]);

    let stsc_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("entry_count", u32be()),
        (
            "chunk_entries",
            repeat_count(
                var("entry_count"),
                record([
                    ("first_chunk", u32be()),
                    ("samples_per_chunk", u32be()),
                    ("sample_description_index", u32be()),
                ]),
            ),
        ),
    ]);

    let stsz_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("sample_size", u32be()),
        ("sample_count", u32be()),
        (
            "entry_size",
            cond_maybe(
                expr_eq(var("sample_size"), Expr::U32(0)),
                repeat_count(var("sample_count"), u32be()),
            ),
        ),
    ]);

    let stco_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("entry_count", u32be()),
        ("chunk_offset", repeat_count(var("entry_count"), u32be())),
    ]);

    let co64_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("entry_count", u32be()),
        ("chunk_offset", repeat_count(var("entry_count"), u64be())),
    ]);

    let sgpd_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("grouping_type", u32be()),
        // FIXME handle version >= 2
        ("default_length", u32be()),
        ("entry_count", u32be()),
        (
            "sample_groups",
            repeat_count(
                var("entry_count"),
                record([
                    (
                        "description_length",
                        if_then_else(
                            expr_eq(var("default_length"), Expr::U32(0)),
                            u32be(),
                            compute(var("default_length")),
                        ),
                    ),
                    (
                        "sample_group_entry",
                        repeat_count(var("description_length"), u8()),
                    ),
                ]),
            ),
        ),
    ]);

    let sbgp_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("grouping_type", u32be()),
        (
            "grouping_type_parameter",
            cond_maybe(expr_eq(var("version"), Expr::U8(1)), u32be()),
        ),
        ("entry_count", u32be()),
        (
            "sample_groups",
            repeat_count(
                var("entry_count"),
                record([
                    ("sample_count", u32be()),
                    ("group_description_index", u32be()),
                ]),
            ),
        ),
    ]);

    let vmhd_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("graphicsmode", u16be()),
        ("opcolor", repeat_count(Expr::U8(3), u16be())),
    ]);

    let smhd_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        ("balance", u16be()),
        ("reserved", u16be()),
    ]);

    let mdhd_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        (
            "fields",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "version0",
                        record([
                            ("creation_time", u32be()),
                            ("modification_time", u32be()),
                            ("timescale", u32be()),
                            ("duration", u32be()),
                        ]),
                    ),
                    (
                        Pattern::U8(1),
                        "version1",
                        record([
                            ("creation_time", u64be()),
                            ("modification_time", u64be()),
                            ("timescale", u32be()),
                            ("duration", u64be()),
                        ]),
                    ),
                ],
            ),
        ),
        ("language", u16be()),
        ("pre_defined", u16be()),
    ]);

    let mvhd_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        (
            "fields",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "version0",
                        record([
                            ("creation_time", u32be()),
                            ("modification_time", u32be()),
                            ("timescale", u32be()),
                            ("duration", u32be()),
                        ]),
                    ),
                    (
                        Pattern::U8(1),
                        "version1",
                        record([
                            ("creation_time", u64be()),
                            ("modification_time", u64be()),
                            ("timescale", u32be()),
                            ("duration", u64be()),
                        ]),
                    ),
                ],
            ),
        ),
        ("rate", u32be()),
        ("volume", u16be()),
        ("reserved1", u16be()),
        ("reserved2", tuple([u32be(), u32be()])),
        ("matrix", repeat_count(Expr::U8(9), u32be())),
        ("pre_defined", repeat_count(Expr::U8(6), u32be())),
        ("next_track_ID", u32be()),
    ]);

    let tkhd_data = record([
        ("version", u8()),
        ("flags", tuple_repeat(3, u8())),
        (
            "fields",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "version0",
                        record([
                            ("creation_time", u32be()),
                            ("modification_time", u32be()),
                            ("track_ID", u32be()),
                            ("reserved", u32be()),
                            ("duration", u32be()),
                        ]),
                    ),
                    (
                        Pattern::U8(1),
                        "version1",
                        record([
                            ("creation_time", u64be()),
                            ("modification_time", u64be()),
                            ("track_ID", u32be()),
                            ("reserved", u32be()),
                            ("duration", u64be()),
                        ]),
                    ),
                ],
            ),
        ),
        ("reserved2", tuple_repeat(2, u32be())),
        ("layer", u16be()),
        ("alternate_group", u16be()),
        ("volume", u16be()),
        ("reserved1", u16be()),
        ("matrix", repeat_count(Expr::U8(9), u32be())),
        ("width", u32be()),
        ("height", u32be()),
    ]);

    let data_data = record([
        ("type_indicator", u32be()),
        ("locale_indicator", u32be()),
        ("value", repeat(ascii_char())),
    ]);

    let edts_atom = module.define_format(
        "mpeg4.edts-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['e', 'l', 's', 't']), "elst", elst_data),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let dinf_atom = module.define_format(
        "mpeg4.dinf-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['d', 'r', 'e', 'f']), "dref", dref_data),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let stbl_atom = module.define_format(
        "mpeg4.stbl-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['s', 't', 's', 'd']), "stsd", stsd_data),
                    (tag_pattern(['s', 't', 't', 's']), "stts", stts_data),
                    (tag_pattern(['c', 't', 't', 's']), "ctts", ctts_data),
                    (tag_pattern(['s', 't', 's', 's']), "stss", stss_data),
                    (tag_pattern(['s', 't', 's', 'c']), "stsc", stsc_data),
                    (tag_pattern(['s', 't', 's', 'z']), "stsz", stsz_data),
                    (tag_pattern(['s', 't', 'c', 'o']), "stco", stco_data),
                    (tag_pattern(['c', 'o', '6', '4']), "co64", co64_data),
                    (tag_pattern(['s', 'g', 'p', 'd']), "sgpd", sgpd_data),
                    (tag_pattern(['s', 'b', 'g', 'p']), "sbgp", sbgp_data),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let minf_atom = module.define_format(
        "mpeg4.minf-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['v', 'm', 'h', 'd']), "vmhd", vmhd_data),
                    (tag_pattern(['s', 'm', 'h', 'd']), "smhd", smhd_data),
                    (
                        tag_pattern(['d', 'i', 'n', 'f']),
                        "dinf",
                        repeat(dinf_atom.call()),
                    ),
                    (
                        tag_pattern(['s', 't', 'b', 'l']),
                        "stbl",
                        repeat(stbl_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let mdia_atom = module.define_format(
        "mpeg4.mdia-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['h', 'd', 'l', 'r']), "hdlr", mdia_hdlr_data),
                    (tag_pattern(['m', 'd', 'h', 'd']), "mdhd", mdhd_data),
                    (
                        tag_pattern(['m', 'i', 'n', 'f']),
                        "minf",
                        repeat(minf_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let trak_atom = module.define_format(
        "mpeg4.trak-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['t', 'k', 'h', 'd']), "tkhd", tkhd_data),
                    (
                        tag_pattern(['e', 'd', 't', 's']),
                        "edts",
                        repeat(edts_atom.call()),
                    ),
                    (
                        tag_pattern(['m', 'd', 'i', 'a']),
                        "mdia",
                        repeat(mdia_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let tool_atom = module.define_format(
        "mpeg4.tool-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['d', 'a', 't', 'a']), "data", data_data),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let ilst_atom = module.define_format(
        "mpeg4.ilst-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern3(['t', 'o', 'o']),
                        "tool",
                        repeat(tool_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let meta_atom = module.define_format(
        "mpeg4.meta-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern(['d', 'i', 'n', 'f']),
                        "dinf",
                        repeat(dinf_atom.call()),
                    ),
                    (tag_pattern(['h', 'd', 'l', 'r']), "hdlr", meta_hdlr_data),
                    (tag_pattern(['p', 'i', 't', 'm']), "pitm", pitm_data),
                    (tag_pattern(['i', 'i', 'n', 'f']), "iinf", iinf_data),
                    (tag_pattern(['i', 'r', 'e', 'f']), "iref", iref_data),
                    (tag_pattern(['i', 'l', 'o', 'c']), "iloc", iloc_data),
                    (
                        tag_pattern(['i', 'l', 's', 't']),
                        "ilst",
                        repeat(ilst_atom.call()),
                    ),
                    (tag_pattern(['i', 'd', 'a', 't']), "idat", opaque_bytes()),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let udta_atom = module.define_format(
        "mpeg4.udta-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern(['m', 'e', 't', 'a']),
                        "meta",
                        tuple(vec![
                            u32be(), // 8-bit version, 24-bit flags
                            repeat(meta_atom.call()),
                        ]),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let moov_atom = module.define_format(
        "mpeg4.moov-atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['m', 'v', 'h', 'd']), "mvhd", mvhd_data),
                    (
                        tag_pattern(['t', 'r', 'a', 'k']),
                        "trak",
                        repeat(trak_atom.call()),
                    ),
                    (
                        tag_pattern(['u', 'd', 't', 'a']),
                        "udta",
                        repeat(udta_atom.call()),
                        // FIXME can be followed by optional 32-bit zero value
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    let atom = module.define_format(
        "mpeg4.atom",
        make_atom(
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['f', 't', 'y', 'p']), "ftyp", ftyp_data),
                    (tag_pattern(['f', 'r', 'e', 'e']), "free", Format::EMPTY),
                    (
                        tag_pattern(['m', 'd', 'a', 't']),
                        "mdat",
                        Format::EMPTY, // FIXME
                    ),
                    (
                        tag_pattern(['m', 'e', 't', 'a']),
                        "meta",
                        tuple(vec![
                            u32be(), // 8-bit version, 24-bit flags
                            repeat(meta_atom.call()),
                        ]),
                    ),
                    (
                        tag_pattern(['m', 'o', 'o', 'v']),
                        "moov",
                        repeat(moov_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        ),
    );

    module.define_format(
        "mpeg4.main",
        record([
            ("atoms", repeat(atom.call())),
            //("atoms", repeat_count(Expr::U8(4), atom.call())),
            //("trailer", opaque_bytes())
        ]),
    )
}
