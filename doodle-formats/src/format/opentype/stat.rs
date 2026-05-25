use super::*;

/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/stat#style-attributes-header
pub(crate) fn table(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
    let design_axes_array = design_axes_array(module, tag);
    let axis_value_array = axis_value_array(module);
    module.define_format(
        "opentype.stat.table",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("major_version", util::expect_u16be(1)),
                ("minor_version", util::expects_u16be([1, 2])), // Version 1.0 is deprecated
                ("design_axis_size", u16be()), // size (in bytes) of each axis record
                ("design_axis_count", u16be()), // number of axis records
                (
                    "design_axes",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        design_axes_array.call_args(vec![var("design_axis_count")]),
                    ),
                ), // offset is 0 iff design_axis_count is 0
                ("axis_value_count", u16be()),
                (
                    "axis_value_offsets",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        axis_value_array.call_args(vec![var("axis_value_count")]),
                    ),
                ), // offset is 0 iff axis_value_count is 0
                ("elided_fallback_name_id", u16be()), // omitted in version 1.0, but said version is deprecated
            ]),
        ),
    )
}

fn design_axes_array(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
    let axis_record = record([
        ("axis_tag", tag.call()),
        ("axis_name_id", u16be()),
        ("axis_ordering", u16be()),
    ]);
    module.define_format_args(
        "opentype.stat.design_axes_array",
        vec![(Label::Borrowed("design_axis_count"), ValueType::U16)],
        record([(
            "design_axes",
            repeat_count(var("design_axis_count"), axis_record),
        )]),
    )
}

fn axis_value_array(module: &mut FormatModule) -> FormatRef {
    let axis_value_table = axis_value_table(module);
    module.define_format_args(
        "opentype.stat.axis_value_array",
        vec![(Label::Borrowed("axis_value_count"), ValueType::U16)],
        let_view(
            "array_view",
            record([
                ("array_scope", reify_view(vvar("array_view"))),
                (
                    "axis_values",
                    repeat_count(
                        var("axis_value_count"),
                        util::read_phantom_view_offset16(
                            vvar("array_view"),
                            axis_value_table.call(),
                        ),
                    ),
                ),
            ]),
        ),
    )
}

fn axis_value_table(module: &mut FormatModule) -> FormatRef {
    use BitFieldKind::*;
    let axis_flags = bit_fields_u16([
        Reserved {
            bit_width: 14,
            check_zero: false,
        },
        FlagBit("elidable_axis_value_name"), // Bit 1 - When set, indicates the 'normal' value for this axis and implies it may be omitted when composing name-strings
        FlagBit("older_sibling_font_attribute"), // Bit 0 - When set, indicates that the axis information applies to previously released fonts in the same font-family
    ]);
    let axis_value_record = record([("axis_index", u16be()), ("value", util::fixed32be())]);
    let f1_fields = vec![
        ("axis_index", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
        ("value", fixed32be()),
    ];
    let f2_fields = vec![
        ("axis_index", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
        ("nominal_value", fixed32be()),
        ("range_min_value", fixed32be()),
        ("range_max_value", fixed32be()),
    ];
    let f3_fields = vec![
        ("axis_index", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
        ("value", fixed32be()),
        ("linked_value", fixed32be()),
    ];
    let f4_fields = vec![
        ("axis_count", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this combination of axis values
        (
            "axis_values",
            repeat_count(var("axis_count"), axis_value_record),
        ),
    ];
    module.define_format(
        "opentype.stat.axis_value_table",
        util::embedded_variadic_alternation(
            [("format", where_between_u16(u16be(), 1, 4))],
            "format",
            [
                (1u16, "Format1", f1_fields),
                (2, "Format2", f2_fields),
                (3, "Format3", f3_fields),
                (4, "Format4", f4_fields),
            ],
            "data",
            util::NestingKind::MinimalVariation,
        ),
    )
}
