use super::*;

/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/stat#style-attributes-header
pub(crate) fn table(om: &mut OpentypeModule<'_>) -> FormatRef {
    let tag = om.tag();
    let fixed32be = om.fixed32be();
    let design_axes_array = design_axes_array(om.module(), tag);
    let axis_value_array = axis_value_array(om.module(), fixed32be);
    om.module().define_format(
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
    // readarray-eligible once `as_base_kind_read` can resolve `FormatRef::call()` indirection:
    // `axis_tag: tag.call()` is the sole blocker (`tag` itself resolves to a bare `u32be()`);
    // `axis_name_id`/`axis_ordering` are already bare `u16be()`. `axis_record` is currently
    // inline -- a new `FormatRef` (e.g. `opentype.stat.axis_record`) would need to be defined
    // for it. Unlike most sites in this file, this one needs *no* `from_here`: the whole target
    // of this function's `read_phantom_view_offset32` call site (in `table()`) is nothing but
    // this array (`design_axis_count` is supplied from the caller, exactly mirroring the
    // pre-migration shape of `cpal.rs::color_record_array`) -- so this can go straight to
    // `pseudo_record([("offset", u32be())], with_view(table_view.offset(var("offset")),
    // read_array(var("design_axis_count"), axis_record_ref)))`, following the migrated
    // `cpal.rs::table` pattern exactly (commit f9835b9).
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

fn axis_value_array(module: &mut FormatModule, fixed32be: FormatRef) -> FormatRef {
    let axis_value_table = axis_value_table(module, fixed32be);
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

fn axis_value_table(module: &mut FormatModule, fixed32be: FormatRef) -> FormatRef {
    use BitFieldKind::*;
    let axis_flags = bit_fields_u16([
        Reserved {
            bit_width: 14,
            check_zero: false,
        },
        FlagBit("elidable_axis_value_name"), // Bit 1 - When set, indicates the 'normal' value for this axis and implies it may be omitted when composing name-strings
        FlagBit("older_sibling_font_attribute"), // Bit 0 - When set, indicates that the axis information applies to previously released fonts in the same font-family
    ]);
    let axis_value_record = record([("axis_index", u16be()), ("value", fixed32be.call())]);
    let f1_fields = vec![
        ("axis_index", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
        ("value", fixed32be.call()),
    ];
    let f2_fields = vec![
        ("axis_index", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
        ("nominal_value", fixed32be.call()),
        ("range_min_value", fixed32be.call()),
        ("range_max_value", fixed32be.call()),
    ];
    let f3_fields = vec![
        ("axis_index", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
        ("value", fixed32be.call()),
        ("linked_value", fixed32be.call()),
    ];
    let f4_fields = vec![
        ("axis_count", u16be()),
        ("flags", axis_flags.clone()),
        ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this combination of axis values
        // readarray-eligible once `as_base_kind_read` can see through `Format::Variant`
        // wrapping: `axis_value_record`'s `value` field is `fixed32be.call()`; `axis_index` is
        // already a bare `u16be()`. No other blocker. `axis_value_record` is currently an
        // inline (unregistered) record -- a new `FormatRef` (e.g.
        // `opentype.stat.axis_value_record`) would need to be defined for it. `axis_value_table`
        // (this function's enclosing format) does not bind any view of its own even though it's
        // reached via `read_phantom_view_offset16` from `axis_value_array`, so this would still
        // need `from_here(read_array(var("axis_count"), axis_value_record_ref))`.
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
