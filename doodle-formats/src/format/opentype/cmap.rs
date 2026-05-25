use super::*;

/// Table format definition-function for `cmap`
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let sequential_map_group = module.define_format(
        "opentype.types.sequential_map_record",
        record([
            ("start_char_code", u32be()),
            ("end_char_code", u32be()),
            ("start_glyph_id", u32be()),
        ]),
    );

    let cmap_subtable_format0 = subtable_format0(module);
    let cmap_subtable_format2 = subtable_format2(module);
    let cmap_subtable_format4 = subtable_format4(module);

    let cmap_subtable_format6 = subtable_format6(module);

    let cmap_subtable_format8 = subtable_format8(module, sequential_map_group);

    let cmap_subtable_format10 = subtable_format10(module);

    let cmap_subtable_format12 = subtable_format12(module, sequential_map_group);

    let cmap_subtable_format13 = subtable_format13(module, sequential_map_group);

    let cmap_subtable_format14 = subtable_format14(module);

    let cmap_subtable = module.define_format_args(
        "opentype.cmap.subtable",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("format", Format::Peek(Box::new(u16be()))),
                (
                    "data",
                    match_variant(
                        var("format"),
                        [
                            (
                                Pattern::U16(0),
                                "Format0",
                                cmap_subtable_format0.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(2),
                                "Format2",
                                cmap_subtable_format2.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(4),
                                "Format4",
                                cmap_subtable_format4.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(6),
                                "Format6",
                                cmap_subtable_format6.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(8),
                                "Format8",
                                cmap_subtable_format8.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(10),
                                "Format10",
                                cmap_subtable_format10.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(12),
                                "Format12",
                                cmap_subtable_format12.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(13),
                                "Format13",
                                cmap_subtable_format13.call_args(vec![var("_platform")]),
                            ),
                            (
                                Pattern::U16(14),
                                "Format14",
                                cmap_subtable_format14.call_views(vec![vvar("table_view")]),
                            ),
                            // FIXME - leaving out unknown-table for now
                        ],
                    ),
                ),
            ]),
        ),
    );

    let encoding_record = module.define_format_views(
        "opentype.encoding_record",
        vec![Label::Borrowed("table_view")],
        record([
            ("platform", u16be()), // platform identifier
            // NOTE - encoding_id nominally depends on platform_id but no recorded dependencies in fathom def
            ("encoding", encoding_id(var("platform"))), // encoding identifier
            (
                "subtable",
                util::read_phantom_view_offset32(
                    vvar("table_view"),
                    cmap_subtable.call_args(vec![var("platform")]),
                ),
            ),
        ]),
    );

    module.define_format(
        "opentype.cmap.table",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))), // start of character mapping table
                ("version", u16be()),                            // table version number
                ("num_tables", u16be()), // number of subsequent encoding tables
                (
                    "encoding_records",
                    repeat_count(
                        var("num_tables"),
                        encoding_record.call_views(vec![vvar("table_view")]),
                    ),
                ),
            ]),
        ),
    )
}

// Format 0 : Byte encoding table
fn subtable_format0(module: &mut FormatModule) -> FormatRef {
    module.define_format_args(
        "opentype.cmap_subtable.format0",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        util::slice_record(
            "length",
            [
                ("_format", expect_u16be(0)), // == 0
                ("length", u16be()),
                ("language", cmap_language_id(var("_platform"))),
                (
                    "glyph_id_array",
                    repeat_count(Expr::U16(256), small_glyph_id()),
                ),
            ],
        ),
    )
}

fn subtable_format2(module: &mut FormatModule) -> FormatRef {
    let subheader = record([
        ("first_code", u16be()),
        ("entry_count", u16be()),
        ("id_delta", i16be()),
        ("id_range_offset", u16be()),
    ]);

    // Format 2: High-byte mapping through table
    module.define_format_args(
        "opentype.cmap_subtable.format2",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        util::slice_record(
            "length",
            [
                ("_format", util::expect_u16be(2)),
                (
                    "length",
                    where_lambda(
                        u16be(),
                        "l",
                        and(
                            // NOTE - strictly speaking we don't expect length == 518 exactly, but this is a rough check
                            expr_gte(var("l"), Expr::U16(518)),
                            // NOTE - all fields are entirely comprised of 16-bit tokens, so overall length must be a multiple of 2
                            expr_eq(rem(var("l"), Expr::U16(2)), Expr::U16(0)),
                        ),
                    ),
                ),
                ("language", cmap_language_id(var("_platform"))),
                ("sub_header_keys", repeat_count(Expr::U16(256), u16be())),
                (
                    "sub_headers",
                    repeat_count(
                        succ(util::subheader_index(var("sub_header_keys"))),
                        subheader,
                    ),
                ),
                ("glyph_array", repeat(u16be())),
            ],
        ),
    )
}

/// cmap subtable Format 4: Segment mapping to delta values
fn subtable_format4(module: &mut FormatModule) -> FormatRef {
    module.define_format_args(
        "opentype.cmap_subtable.format4",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        util::slice_record(
            "length",
            [
                ("_format", util::expect_u16be(4)),
                ("length", u16be()),
                ("language", cmap_language_id(var("_platform"))),
                (
                    "seg_count",
                    map(
                        u16be(),
                        lambda("seg_count_x2", div(var("seg_count_x2"), Expr::U16(2))),
                    ),
                ),
                ("search_range", u16be()), // := 2x the maximum power of 2 <= seg_count
                ("entry_selector", u16be()), // := ilog2(seg_count)
                ("range_shift", u16be()),  // := seg_count * 2 - search_range
                ("end_code", repeat_count(var("seg_count"), u16be())), // end character-code for each seg, last is 0xFFFF
                ("__reserved_pad", util::expect_u16be(0)),
                ("start_code", repeat_count(var("seg_count"), u16be())),
                ("id_delta", repeat_count(var("seg_count"), u16be())), // ought to be signed but will work if we perform as unsigned addition mod-0xFFFF
                ("id_range_offset", repeat_count(var("seg_count"), u16be())), // offsets into glyphIdArray or 0
                ("glyph_array", repeat(u16be())),
            ],
        ),
    )
}

fn subtable_format6(module: &mut FormatModule) -> FormatRef {
    module.define_format_args(
        "opentype.cmap_subtable.format6",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        /* Previously defined as a slice_record but sufficiently large `entry_count` values
         * could cause length to wrap around mod 65536 and lead to slice boundary violation
         * while reading `glyph_id_array`
         */
        record([
            ("_format", util::expect_u16be(6)),
            ("length", u16be()),
            ("language", cmap_language_id(var("_platform"))),
            ("first_code", u16be()),
            ("entry_count", u16be()),
            ("glyph_id_array", repeat_count(var("entry_count"), u16be())),
        ]),
    )
}

fn subtable_format8(module: &mut FormatModule, sequential_map_group: FormatRef) -> FormatRef {
    module.define_format_args(
        "opentype.cmap_subtable.format8",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        util::slice_record(
            "length",
            [
                ("_format", util::expect_u16be(8)),
                ("__reserved", util::expect_u16be(0)),
                ("length", u32be()),
                ("language", cmap_language_id32(var("_platform"))),
                // REVIEW - should this be 8x as long and consist of bits?
                ("is32", repeat_count(Expr::U16(8192), u8())), // packed bit-array where a bit at index `i` signals whether the 16-bit value index `i` is the start of a 32-bit character code
                ("num_groups", u32be()),
                (
                    "groups",
                    repeat_count(var("num_groups"), sequential_map_group.call()),
                ),
            ],
        ),
    )
}

fn subtable_format10(module: &mut FormatModule) -> FormatRef {
    module.define_format_args(
        "opentype.cmap_subtable.format10",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        util::slice_record(
            "length",
            [
                ("_format", util::expect_u16be(10)),
                ("__reserved", util::expect_u16be(0)),
                ("length", u32be()),
                ("language", cmap_language_id32(var("_platform"))),
                ("start_char_code", u32be()),
                ("num_chars", u32be()),
                ("glyph_id_array", repeat_count(var("num_chars"), u16be())),
            ],
        ),
    )
}

fn subtable_format12(module: &mut FormatModule, sequential_map_group: FormatRef) -> FormatRef {
    module.define_format_args(
        "opentype.cmap_subtable.format12",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        util::slice_record(
            "length",
            [
                ("_format", util::expect_u16be(12)),
                ("__reserved", util::expect_u16be(0)),
                ("length", u32be()),
                ("language", cmap_language_id32(var("_platform"))),
                ("num_groups", u32be()),
                (
                    "groups",
                    repeat_count(var("num_groups"), sequential_map_group.call()),
                ),
            ],
        ),
    )
}

fn subtable_format13(module: &mut FormatModule, sequential_map_group: FormatRef) -> FormatRef {
    let constant_map_group = sequential_map_group.call();

    module.define_format_args(
        "opentype.cmap_subtable.format13",
        vec![(Label::Borrowed("_platform"), ValueType::U16)],
        util::slice_record(
            "length",
            [
                ("_format", util::expect_u16be(13)),
                ("__reserved", util::expect_u16be(0)),
                ("length", u32be()),
                ("language", cmap_language_id32(var("_platform"))),
                ("num_groups", u32be()),
                (
                    "groups",
                    repeat_count(var("num_groups"), constant_map_group),
                ),
            ],
        ),
    )
}

fn subtable_format14(module: &mut FormatModule) -> FormatRef {
    let unicode_range = record([
        ("start_unicode_value", util::u24be()),
        ("additional_count", u8()),
    ]);

    let uvs_mapping = record([("unicode_value", util::u24be()), ("glyph_id", u16be())]);

    let default_uvs_table = record([
        ("num_unicode_value_ranges", u32be()),
        (
            "ranges",
            repeat_count(var("num_unicode_value_ranges"), unicode_range),
        ),
    ]);

    let non_default_uvs_table = record([
        ("num_uvs_mappings", u32be()),
        (
            "uvs_mappings",
            repeat_count(var("num_uvs_mappings"), uvs_mapping),
        ),
    ]);

    let variation_selector = module.define_format_views(
        "opentype.variation_selector",
        vec![TABLE_VIEW],
        record([
            ("var_selector", util::u24be()),
            (
                "default_uvs_offset",
                util::read_phantom_view_offset32(vvar("table_view"), default_uvs_table),
            ),
            (
                "non_default_uvs_offset",
                util::read_phantom_view_offset32(vvar("table_view"), non_default_uvs_table),
            ),
        ]),
    );

    module.define_format_views(
        "opentype.cmap_subtable.format14",
        [util::TABLE_VIEW].to_vec(),
        util::slice_record(
            "length",
            [
                ("_format", util::expect_u16be(14)),
                ("length", u32be()),
                ("num_var_selector_records", u32be()),
                (
                    "var_selector",
                    repeat_count(
                        var("num_var_selector_records"),
                        variation_selector.call_views(vec![vvar("table_view")]),
                    ),
                ),
            ],
        ),
    )
}

/// Format for language-ids appearing within the `cmap` table-scop
#[inline]
pub(crate) fn cmap_language_id(_platform: Expr) -> Format {
    language_id()
}
/// Format for 32-bit language-ids appearing within the `cmap` table-scop

#[inline]
pub(crate) fn cmap_language_id32(_platform: Expr) -> Format {
    u32be()
}

#[inline]
pub(crate) fn small_glyph_id() -> Format {
    u8()
}
