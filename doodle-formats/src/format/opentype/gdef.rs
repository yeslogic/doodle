use super::*;

pub(crate) fn table(
    module: &mut FormatModule,
    class_def: FormatRef,
    coverage_table: FormatRef,
    device_or_variation_index_table: FormatRef,
    item_variation_store: FormatRef,
) -> FormatRef {
    let mark_glyph_set = mark_glyph_set(module, coverage_table);
    let gdef_header_version_1_2 = |table_view: ViewExpr| {
        record([(
            "mark_glyph_sets_def",
            util::read_phantom_view_offset16(table_view, mark_glyph_set.call()),
        )])
    };
    let gdef_header_version_1_3 = |table_view: ViewExpr| {
        record([
            (
                "mark_glyph_sets_def",
                util::read_phantom_view_offset16(table_view.clone(), mark_glyph_set.call()),
            ),
            (
                "item_var_store",
                util::read_phantom_view_offset32(table_view, item_variation_store.call()),
            ),
        ])
    };
    let attach_list = attach_list(module, coverage_table);
    let lig_caret_list = lig_caret_list(module, coverage_table, device_or_variation_index_table);
    module.define_format(
        "opentype.gdef.table",
        let_view(
            "table_view",
            record([
                // Starting offset of `GDEF` table
                ("table_scope", reify_view(vvar("table_view"))),
                // Major Version of `GDEF` table - only 1[.x] defined
                ("major_version", util::expect_u16be(1)), // NOTE - only major version 1 is defined: https://learn.microsoft.com/en-us/typography/opentype/spec/gdef#gdef-table-structures
                // Minor Version (can be [1.]0, [1.]2, or [1.]3)
                ("minor_version", u16be()),
                // Class definition table for glyph type (may be NULL)
                (
                    "glyph_class_def",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                // Attachment point list table (may be NULL)
                (
                    "attach_list",
                    util::read_phantom_view_offset16(vvar("table_view"), attach_list.call()),
                ),
                // Ligature caret list table (may be NULL)
                (
                    "lig_caret_list",
                    util::read_phantom_view_offset16(vvar("table_view"), lig_caret_list.call()),
                ),
                // Class definition table for mark attachment type (may be NULL)
                (
                    "mark_attach_class_def",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                // Version-specific data, if > 1.0
                // REVIEW - do we want to flatten this variant abstraction into two Option<...> fields instead?
                (
                    "data",
                    match_variant(
                        var("minor_version"),
                        [
                            (Pattern::U16(0), "Version1_0", Format::EMPTY),
                            // NOTE - the variant `Version1_1` will not actually appear in the generated type due to Void-pruning
                            (Pattern::U16(1), "Version1_1", Format::Fail), // FIXME - should this be EMPTY instead?
                            (
                                Pattern::U16(2),
                                "Version1_2",
                                gdef_header_version_1_2(vvar("table_view")),
                            ),
                            (
                                Pattern::U16(3),
                                "Version1_3",
                                gdef_header_version_1_3(vvar("table_view")),
                            ),
                            // NOTE - this case covers everything after version 1.3 - following the Fathom definition that falls back onto the latest version we support
                            (
                                Pattern::Wildcard,
                                "Version1_3",
                                gdef_header_version_1_3(vvar("table_view")),
                            ),
                        ],
                    ),
                ),
            ]),
        ),
    )
}

fn lig_caret_list(
    module: &mut FormatModule,
    coverage_table: FormatRef,
    device_or_variation_index_table: FormatRef,
) -> FormatRef {
    let caret_value = caret_value(module, device_or_variation_index_table);
    let lig_glyph = module.define_format(
        "opentype.gdef.lig_glyph",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("caret_count", u16be()),
                (
                    "caret_values",
                    repeat_count(
                        var("caret_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), caret_value.call()),
                    ),
                ),
            ]),
        ),
    );
    module.define_format(
        "opentype.gdef.lig_caret_list",
        let_view(
            "list_view",
            record([
                ("list_scope", reify_view(vvar("list_view"))),
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("list_view"), coverage_table.call()),
                ),
                ("lig_glyph_count", u16be()),
                (
                    "lig_glyph_offsets",
                    repeat_count(
                        var("lig_glyph_count"),
                        util::read_phantom_view_offset16(vvar("list_view"), lig_glyph.call()),
                    ),
                ),
            ]),
        ),
    )
}

fn caret_value(module: &mut FormatModule, device_or_variation_index_table: FormatRef) -> FormatRef {
    // REVIEW - should we make formatrefs for formats 1 and 2 for consistency?
    let caret_value_format_1 = record([("coordinate", i16be())]);

    let caret_value_format_2 = record([("caret_value_point_index", u16be())]);

    let caret_value_format_3 = module.define_format_views(
        "opentype.gdef.caret_value.data.format3",
        vec![Label::Borrowed("table_view")],
        record([
            // REVIEW[epic=nested-format-reify-layer] - reified into local scope
            ("table_scope", reify_view(vvar("table_view"))),
            ("coordinate", i16be()),
            (
                "table",
                util::read_phantom_view_offset16(
                    vvar("table_view"),
                    device_or_variation_index_table.call(),
                ),
            ),
        ]),
    );

    module.define_format(
        "opentype.gdef.caret_value",
        let_view(
            "table_view",
            record([
                ("format", u16be()),
                (
                    "data",
                    match_variant(
                        var("format"),
                        [
                            (Pattern::U16(1), "Format1", caret_value_format_1),
                            (Pattern::U16(2), "Format2", caret_value_format_2),
                            (
                                Pattern::U16(3),
                                "Format3",
                                caret_value_format_3.call_view(vvar("table_view")),
                            ),
                            // REVIEW[epic=catchall-policy] - do we need this catch-all?
                            (Pattern::Wildcard, "BadFormat", Format::Fail),
                        ],
                    ),
                ),
            ]),
        ),
    )
}

fn attach_list(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
    let attach_point = module.define_format(
        "opentype.gdef.attach_point",
        record([
            ("point_count", u16be()),
            ("point_indices", repeat_count(var("point_count"), u16be())),
        ]),
    );

    module.define_format(
        "opentype.gdef.attach_list",
        let_view(
            "list_view",
            record([
                ("list_scope", reify_view(vvar("list_view"))),
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("list_view"), coverage_table.call()),
                ),
                ("glyph_count", u16be()),
                (
                    "attach_point_offsets",
                    repeat_count(
                        var("glyph_count"),
                        util::read_phantom_view_offset16(vvar("list_view"), attach_point.call()),
                    ),
                ),
            ]),
        ),
    )
}

fn mark_glyph_set(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
    module.define_format(
        "opentype.gdef.mark_glyph_set",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("format", util::expect_u16be(1)), // FIXME - u16be() instead if this is validation fails
                ("mark_glyph_set_count", u16be()),
                (
                    "coverage",
                    repeat_count(
                        var("mark_glyph_set_count"),
                        util::read_phantom_view_offset32(vvar("table_view"), coverage_table.call()),
                    ),
                ),
            ]),
        ),
    )
}
