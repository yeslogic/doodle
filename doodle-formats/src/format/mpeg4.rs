use crate::format::BaseModule;
use doodle::helper::*;
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

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    let tag = module.define_format(
        "mpeg4.tag",
        tuple([
            base.ascii_char(),
            base.ascii_char(),
            base.ascii_char(),
            base.ascii_char(),
        ]),
    );

    fn make_atom(base: &BaseModule, tag: FormatRef, data: Format) -> Format {
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                Format::Match(
                    Box::new(var("size-field")),
                    vec![
                        (Pattern::U32(0), compute(Expr::U64(0))), // FIXME
                        (
                            Pattern::U32(1),
                            map(base.u64be(), lambda("x", sub(var("x"), Expr::U64(16)))),
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
        ("minor_version", base.u32be()),
        ("compatible_brands", Format::Repeat(Box::new(tag.call()))),
    ]);

    let mdia_hdlr_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("component_type", base.u32be()),
        ("component_subtype", tag.call()),
        ("component_manufacturer", base.u32be()),
        ("component_flags", base.u32be()),
        ("component_flags_mask", base.u32be()),
        ("component_name", base.asciiz_string()),
    ]);

    let meta_hdlr_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("predefined", base.u32be()),
        ("handler_type", tag.call()),
        (
            "reserved",
            tuple([base.u32be(), base.u32be(), base.u32be()]),
        ),
        ("name", base.asciiz_string()),
    ]);

    let pitm_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        (
            "item_ID",
            if_then_else_variant(
                expr_eq(var("version"), Expr::U8(0)),
                base.u16be(),
                base.u32be(),
            ),
        ),
    ]);

    let infe_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        (
            "fields",
            if_then_else_variant(
                expr_lt(var("version"), Expr::U8(2)),
                record([
                    ("item_ID", base.u16be()),
                    ("item_protection_index", base.u16be()),
                    ("item_name", base.asciiz_string()),
                    ("content_type", base.asciiz_string()),
                    ("content_encoding", base.asciiz_string()),
                    // FIXME unfinished
                ]),
                record([
                    (
                        "item_ID",
                        if_then_else(
                            expr_eq(var("version"), Expr::U8(2)),
                            map(base.u16be(), lambda("x", as_u32(var("x")))),
                            base.u32be(),
                        ),
                    ),
                    ("item_protection_index", base.u16be()),
                    ("item_type", tag.call()),
                    ("item_name", base.asciiz_string()),
                    (
                        "extra_fields",
                        match_variant(
                            var("item_type"),
                            vec![
                                (
                                    tag_pattern(['m', 'i', 'm', 'e']),
                                    "mime",
                                    record([
                                        ("content_type", base.asciiz_string()),
                                        //FIXME optional ("content_encoding", base.asciiz_string()),
                                    ]),
                                ),
                                (
                                    tag_pattern(['u', 'r', 'i', ' ']),
                                    "uri",
                                    record([("item_uri_type", base.asciiz_string())]),
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
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['i', 'n', 'f', 'e']), "infe", infe_data),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let iinf_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        (
            "entry_count",
            if_then_else(
                expr_eq(var("version"), Expr::U8(0)),
                map(base.u16be(), lambda("x", as_u32(var("x")))),
                base.u32be(),
            ),
        ),
        (
            "item_info_entry",
            repeat_count(var("entry_count"), iinf_atom.call()),
        ),
    ]);

    let single_item_reference_data = record([
        ("from_item_ID", base.u16be()),
        ("reference_count", base.u16be()),
        (
            "to_item_ID",
            repeat_count(var("reference_count"), base.u16be()),
        ),
    ]);

    let single_item_reference_large_data = record([
        ("from_item_ID", base.u32be()),
        ("reference_count", base.u16be()),
        (
            "to_item_ID",
            repeat_count(var("reference_count"), base.u32be()),
        ),
    ]);

    let iref_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        (
            "single_item_reference",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "small",
                        Format::Repeat(Box::new(make_atom(base, tag, single_item_reference_data))),
                    ),
                    (
                        Pattern::U8(1),
                        "large",
                        Format::Repeat(Box::new(make_atom(
                            base,
                            tag,
                            single_item_reference_large_data,
                        ))),
                    ),
                ],
            ),
        ),
    ]);

    let iloc_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("offset_size_length_size", base.u8()), // two four-bit fields
        ("base_offset_size_index_size", base.u8()), // two four-bit fields
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
                map(base.u16be(), lambda("x", as_u32(var("x")))),
                base.u32be(),
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
                            map(base.u16be(), lambda("x", as_u32(var("x")))),
                            base.u32be(),
                        ),
                    ),
                    (
                        "construction_method",
                        cond_maybe(expr_gt(var("version"), Expr::U8(0)), base.u16be()),
                    ),
                    ("data_reference_index", base.u16be()),
                    (
                        "base_offset",
                        Format::Match(
                            Box::new(var("base_offset_size")),
                            vec![
                                (Pattern::U8(0), compute(Expr::U64(0))),
                                (
                                    Pattern::U8(4),
                                    map(base.u32be(), lambda("x", as_u64(var("x")))),
                                ),
                                (Pattern::U8(8), base.u64be()),
                            ],
                        ),
                    ),
                    ("extent_count", base.u16be()),
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
                                                map(base.u32be(), lambda("x", as_u64(var("x")))),
                                            ),
                                            (Pattern::U8(8), base.u64be()),
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
                                                map(base.u32be(), lambda("x", as_u64(var("x")))),
                                            ),
                                            (Pattern::U8(8), base.u64be()),
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
                                                map(base.u32be(), lambda("x", as_u64(var("x")))),
                                            ),
                                            (Pattern::U8(8), base.u64be()),
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
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("number_of_entries", base.u32be()),
        (
            "data",
            Format::Repeat(Box::new(make_atom(
                base,
                tag,
                Format::Repeat(Box::new(base.u8())),
            ))),
        ),
    ]);

    let elst_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("number_of_entries", base.u32be()),
        (
            "edit_list_table",
            repeat_count(
                var("number_of_entries"),
                record([
                    ("track_duration", base.u32be()),
                    ("media_time", base.u32be()),
                    ("media_rate", base.u32be()),
                ]),
            ),
        ),
    ]);

    let stsd_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("entry_count", base.u32be()),
        (
            "sample_entries",
            repeat_count(
                var("entry_count"),
                make_atom(base, tag, Format::Repeat(Box::new(base.u8()))),
            ),
        ),
    ]);

    let stts_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("entry_count", base.u32be()),
        (
            "sample_entries",
            repeat_count(
                var("entry_count"),
                record([
                    ("sample_count", base.u32be()),
                    ("sample_delta", base.u32be()),
                ]),
            ),
        ),
    ]);

    let ctts_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("entry_count", base.u32be()),
        (
            "sample_entries",
            repeat_count(
                var("entry_count"),
                record([
                    ("sample_count", base.u32be()),
                    // FIXME signed if version == 1
                    ("sample_offset", base.u32be()),
                ]),
            ),
        ),
    ]);

    let stss_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("entry_count", base.u32be()),
        (
            "sample_number",
            repeat_count(var("entry_count"), base.u32be()),
        ),
    ]);

    let stsc_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("entry_count", base.u32be()),
        (
            "chunk_entries",
            repeat_count(
                var("entry_count"),
                record([
                    ("first_chunk", base.u32be()),
                    ("samples_per_chunk", base.u32be()),
                    ("sample_description_index", base.u32be()),
                ]),
            ),
        ),
    ]);

    let stsz_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("sample_size", base.u32be()),
        ("sample_count", base.u32be()),
        (
            "entry_size",
            cond_maybe(
                expr_eq(var("sample_size"), Expr::U32(0)),
                repeat_count(var("sample_count"), base.u32be()),
            ),
        ),
    ]);

    let stco_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("entry_count", base.u32be()),
        (
            "chunk_offset",
            repeat_count(var("entry_count"), base.u32be()),
        ),
    ]);

    let co64_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("entry_count", base.u32be()),
        (
            "chunk_offset",
            repeat_count(var("entry_count"), base.u64be()),
        ),
    ]);

    let sgpd_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("grouping_type", base.u32be()),
        // FIXME handle version >= 2
        ("default_length", base.u32be()),
        ("entry_count", base.u32be()),
        (
            "sample_groups",
            repeat_count(
                var("entry_count"),
                record([
                    (
                        "description_length",
                        if_then_else(
                            expr_eq(var("default_length"), Expr::U32(0)),
                            base.u32be(),
                            compute(var("default_length")),
                        ),
                    ),
                    (
                        "sample_group_entry",
                        repeat_count(var("description_length"), base.u8()),
                    ),
                ]),
            ),
        ),
    ]);

    let sbgp_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("grouping_type", base.u32be()),
        (
            "grouping_type_parameter",
            cond_maybe(expr_eq(var("version"), Expr::U8(1)), base.u32be()),
        ),
        ("entry_count", base.u32be()),
        (
            "sample_groups",
            repeat_count(
                var("entry_count"),
                record([
                    ("sample_count", base.u32be()),
                    ("group_description_index", base.u32be()),
                ]),
            ),
        ),
    ]);

    let vmhd_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("graphicsmode", base.u16be()),
        ("opcolor", repeat_count(Expr::U8(3), base.u16be())),
    ]);

    let smhd_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        ("balance", base.u16be()),
        ("reserved", base.u16be()),
    ]);

    let mdhd_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        (
            "fields",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "version0",
                        record([
                            ("creation_time", base.u32be()),
                            ("modification_time", base.u32be()),
                            ("timescale", base.u32be()),
                            ("duration", base.u32be()),
                        ]),
                    ),
                    (
                        Pattern::U8(1),
                        "version1",
                        record([
                            ("creation_time", base.u64be()),
                            ("modification_time", base.u64be()),
                            ("timescale", base.u32be()),
                            ("duration", base.u64be()),
                        ]),
                    ),
                ],
            ),
        ),
        ("language", base.u16be()),
        ("pre_defined", base.u16be()),
    ]);

    let mvhd_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        (
            "fields",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "version0",
                        record([
                            ("creation_time", base.u32be()),
                            ("modification_time", base.u32be()),
                            ("timescale", base.u32be()),
                            ("duration", base.u32be()),
                        ]),
                    ),
                    (
                        Pattern::U8(1),
                        "version1",
                        record([
                            ("creation_time", base.u64be()),
                            ("modification_time", base.u64be()),
                            ("timescale", base.u32be()),
                            ("duration", base.u64be()),
                        ]),
                    ),
                ],
            ),
        ),
        ("rate", base.u32be()),
        ("volume", base.u16be()),
        ("reserved1", base.u16be()),
        ("reserved2", tuple([base.u32be(), base.u32be()])),
        ("matrix", repeat_count(Expr::U8(9), base.u32be())),
        ("pre_defined", repeat_count(Expr::U8(6), base.u32be())),
        ("next_track_ID", base.u32be()),
    ]);

    let tkhd_data = record([
        ("version", base.u8()),
        ("flags", tuple([base.u8(), base.u8(), base.u8()])),
        (
            "fields",
            match_variant(
                var("version"),
                vec![
                    (
                        Pattern::U8(0),
                        "version0",
                        record([
                            ("creation_time", base.u32be()),
                            ("modification_time", base.u32be()),
                            ("track_ID", base.u32be()),
                            ("reserved", base.u32be()),
                            ("duration", base.u32be()),
                        ]),
                    ),
                    (
                        Pattern::U8(1),
                        "version1",
                        record([
                            ("creation_time", base.u64be()),
                            ("modification_time", base.u64be()),
                            ("track_ID", base.u32be()),
                            ("reserved", base.u32be()),
                            ("duration", base.u64be()),
                        ]),
                    ),
                ],
            ),
        ),
        ("reserved2", tuple([base.u32be(), base.u32be()])),
        ("layer", base.u16be()),
        ("alternate_group", base.u16be()),
        ("volume", base.u16be()),
        ("reserved1", base.u16be()),
        ("matrix", repeat_count(Expr::U8(9), base.u32be())),
        ("width", base.u32be()),
        ("height", base.u32be()),
    ]);

    let data_data = record([
        ("type_indicator", base.u32be()),
        ("locale_indicator", base.u32be()),
        ("value", Format::Repeat(Box::new(base.ascii_char()))),
    ]);

    let edts_atom = module.define_format(
        "mpeg4.edts-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['e', 'l', 's', 't']), "elst", elst_data),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let dinf_atom = module.define_format(
        "mpeg4.dinf-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['d', 'r', 'e', 'f']), "dref", dref_data),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let stbl_atom = module.define_format(
        "mpeg4.stbl-atom",
        make_atom(
            base,
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
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let minf_atom = module.define_format(
        "mpeg4.minf-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['v', 'm', 'h', 'd']), "vmhd", vmhd_data),
                    (tag_pattern(['s', 'm', 'h', 'd']), "smhd", smhd_data),
                    (
                        tag_pattern(['d', 'i', 'n', 'f']),
                        "dinf",
                        Format::Repeat(Box::new(dinf_atom.call())),
                    ),
                    (
                        tag_pattern(['s', 't', 'b', 'l']),
                        "stbl",
                        Format::Repeat(Box::new(stbl_atom.call())),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let mdia_atom = module.define_format(
        "mpeg4.mdia-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['h', 'd', 'l', 'r']), "hdlr", mdia_hdlr_data),
                    (tag_pattern(['m', 'd', 'h', 'd']), "mdhd", mdhd_data),
                    (
                        tag_pattern(['m', 'i', 'n', 'f']),
                        "minf",
                        Format::Repeat(Box::new(minf_atom.call())),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let trak_atom = module.define_format(
        "mpeg4.trak-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['t', 'k', 'h', 'd']), "tkhd", tkhd_data),
                    (
                        tag_pattern(['e', 'd', 't', 's']),
                        "edts",
                        Format::Repeat(Box::new(edts_atom.call())),
                    ),
                    (
                        tag_pattern(['m', 'd', 'i', 'a']),
                        "mdia",
                        Format::Repeat(Box::new(mdia_atom.call())),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let tool_atom = module.define_format(
        "mpeg4.tool-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['d', 'a', 't', 'a']), "data", data_data),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let ilst_atom = module.define_format(
        "mpeg4.ilst-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern3(['t', 'o', 'o']),
                        "tool",
                        Format::Repeat(Box::new(tool_atom.call())),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let meta_atom = module.define_format(
        "mpeg4.meta-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern(['d', 'i', 'n', 'f']),
                        "dinf",
                        Format::Repeat(Box::new(dinf_atom.call())),
                    ),
                    (tag_pattern(['h', 'd', 'l', 'r']), "hdlr", meta_hdlr_data),
                    (tag_pattern(['p', 'i', 't', 'm']), "pitm", pitm_data),
                    (tag_pattern(['i', 'i', 'n', 'f']), "iinf", iinf_data),
                    (tag_pattern(['i', 'r', 'e', 'f']), "iref", iref_data),
                    (tag_pattern(['i', 'l', 'o', 'c']), "iloc", iloc_data),
                    (
                        tag_pattern(['i', 'l', 's', 't']),
                        "ilst",
                        Format::Repeat(Box::new(ilst_atom.call())),
                    ),
                    (
                        tag_pattern(['i', 'd', 'a', 't']),
                        "idat",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let udta_atom = module.define_format(
        "mpeg4.udta-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern(['m', 'e', 't', 'a']),
                        "meta",
                        tuple(vec![
                            base.u32be(), // 8-bit version, 24-bit flags
                            Format::Repeat(Box::new(meta_atom.call())),
                        ]),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let moov_atom = module.define_format(
        "mpeg4.moov-atom",
        make_atom(
            base,
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['m', 'v', 'h', 'd']), "mvhd", mvhd_data),
                    (
                        tag_pattern(['t', 'r', 'a', 'k']),
                        "trak",
                        Format::Repeat(Box::new(trak_atom.call())),
                    ),
                    (
                        tag_pattern(['u', 'd', 't', 'a']),
                        "udta",
                        Format::Repeat(Box::new(udta_atom.call())),
                        // FIXME can be followed by optional 32-bit zero value
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    let atom = module.define_format(
        "mpeg4.atom",
        make_atom(
            base,
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
                            base.u32be(), // 8-bit version, 24-bit flags
                            Format::Repeat(Box::new(meta_atom.call())),
                        ]),
                    ),
                    (
                        tag_pattern(['m', 'o', 'o', 'v']),
                        "moov",
                        Format::Repeat(Box::new(moov_atom.call())),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::Repeat(Box::new(base.u8())),
                    ),
                ],
            ),
        ),
    );

    module.define_format(
        "mpeg4.main",
        record([
            ("atoms", Format::Repeat(Box::new(atom.call()))),
            //("atoms", repeat_count(Expr::U8(4), atom.call())),
            //("trailer", Format::Repeat(Box::new(base.u8())))
        ]),
    )
}
