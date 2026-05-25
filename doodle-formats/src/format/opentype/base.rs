use super::*;

/// BASE table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base
pub(crate) fn table(
    module: &mut FormatModule,
    tag: FormatRef,
    device_or_variation_index_table: FormatRef,
    item_variation_store: FormatRef,
) -> FormatRef {
    let base_coord = base_coord(module, device_or_variation_index_table);
    let min_max = min_max(module, tag, base_coord);
    let base_values = base_values(module, base_coord);

    let base_lang_sys = module.define_format_views(
        "opentype.base.base-langsys",
        vec![Label::Borrowed("table_view")],
        record([
            ("base_lang_sys_tag", tag.call()),
            (
                "min_max",
                util::read_phantom_view_offset16(vvar("table_view"), min_max.call()),
            ),
        ]),
    );
    let base_script = module.define_format(
        "opentype.layout.base_script",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "base_values_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), base_values.call()),
                ),
                (
                    "default_min_max_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), min_max.call()),
                ),
                ("base_lang_sys_count", u16be()),
                (
                    "base_lang_sys_records",
                    repeat_count(
                        var("base_lang_sys_count"),
                        base_lang_sys.call_views(vec![vvar("table_view")]),
                    ),
                ),
            ]),
        ),
    );
    let base_script_record = module.define_format_views(
        "opentype.base.base-script-record",
        vec![Label::Borrowed("table_view")],
        record([
            ("base_script_tag", tag.call()),
            (
                "base_script",
                util::read_phantom_view_offset16(vvar("table_view"), base_script.call()),
            ),
        ]),
    );
    let base_script_list = let_view(
        "table_view",
        record([
            ("table_scope", reify_view(vvar("table_view"))),
            ("base_script_count", u16be()),
            (
                "base_script_records",
                repeat_count(
                    var("base_script_count"),
                    base_script_record.call_view(vvar("table_view")),
                ),
            ),
        ]),
    );
    let base_tag_list = record([
        ("base_tag_count", u16be()),
        (
            "baseline_tags",
            repeat_count(var("base_tag_count"), tag.call()),
        ), // TODO[epic=sorting-validation] - must appear in alphabetical order (not enforced locally)
    ]);
    let axis_table = module.define_format(
        "opentype.layout.axis_table",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "base_tag_list_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), base_tag_list),
                ),
                (
                    "base_script_list_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), base_script_list),
                ),
            ]),
        ),
    );
    module.define_format(
        "opentype.base.table",
        let_view(
            "table_view",
            record([
                // WIP
                ("table_scope", reify_view(vvar("table_view"))),
                ("major_version", util::expect_u16be(1)),
                ("minor_version", where_between_u16(u16be(), 0, 1)), // v1.0 and v1.1
                (
                    "horiz_axis_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), axis_table.call()),
                ),
                (
                    "vert_axis_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), axis_table.call()),
                ),
                (
                    "item_var_store_offset",
                    cond_maybe(
                        expr_gt(var("minor_version"), Expr::U16(0)),
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            item_variation_store.call(),
                        ),
                    ),
                ),
            ]),
        ),
    )
}

/// BaseValues table
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base#basevalues-table
fn base_values(module: &mut FormatModule, base_coord: FormatRef) -> FormatRef {
    let base_values = module.define_format(
        "opentype.layout.base_values",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("default_baseline_index", u16be()),
                ("base_coord_count", u16be()), // NOTE - should be equal to baseTagCount in BaseTagList
                (
                    "base_coord_offsets",
                    repeat_count(
                        var("base_coord_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                    ),
                ),
            ]),
        ),
    );
    base_values
}

/// MinMax table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base#the-minmax-table-and-featminmax-record
fn min_max(module: &mut FormatModule, tag: FormatRef, base_coord: FormatRef) -> FormatRef {
    let feat_min_max = module.define_format_views(
        "opentype.layout.feat_min_max",
        vec![Label::Borrowed("table_view")],
        record([
            ("feature_tag", tag.call()),
            (
                "min_coord_offset",
                util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
            ),
            (
                "max_coord_offset",
                util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
            ),
        ]),
    );
    module.define_format(
        "opentype.layout.min_max",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "min_coord_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                ),
                (
                    "max_coord_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                ),
                ("feat_min_max_count", u16be()),
                (
                    "feat_min_max_records",
                    repeat_count(
                        var("feat_min_max_count"),
                        feat_min_max.call_views(vec![vvar("table_view")]),
                    ),
                ),
            ]),
        ),
    )
}

/// BaseCoord table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base#basecoord-tables
fn base_coord(module: &mut FormatModule, device_or_variation_index_table: FormatRef) -> FormatRef {
    // NOTE - 'data' field is a nested record of any fields beyond `{ format, coordinate }` used in a given format
    let format1_data = Format::EMPTY;
    let format2_data = record([("reference_glyph", u16be()), ("base_coord_point", u16be())]);
    let format3_data = |table_view: ViewExpr| {
        record([(
            "device",
            util::read_phantom_view_offset16(table_view, device_or_variation_index_table.call()),
        )])
    };
    module.define_format(
        "opentype.layout.base_coord",
        let_view(
            "table_view",
            record([
                // WIP
                ("table_scope", reify_view(vvar("table_view"))),
                ("format", u16be()),
                ("coordinate", i16be()),
                (
                    "data",
                    match_variant(
                        var("format"),
                        [
                            (Pattern::U16(1), "NoData", format1_data),
                            (Pattern::U16(2), "GlyphData", format2_data),
                            (
                                Pattern::U16(3),
                                "DeviceData",
                                format3_data(vvar("table_view")),
                            ),
                            (Pattern::Wildcard, "UnknownFormat", Format::Fail),
                        ],
                    ),
                ),
            ]),
        ),
    )
}
