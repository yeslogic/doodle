use super::*;

/// Format specification for `CPAL` table
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/cpal
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let color_record = color_record(module);
    let palette_types_array = palette_types_array(module);
    let palette_labels_array = palette_labels_array(module);
    let palette_entry_labels_array = palette_entry_labels_array(module);
    module.define_format(
        "opentype.cpal.table",
        let_view(
            "table_view",
            util::embedded_variadic_alternation(
                [
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("version", expect_range_u16be(0, 1)),
                    // NOTE - the specification does not explicitly state that numPaletteEntries must be >0, but it is required in practice by other related fields being stated as nonzero
                    ("num_palette_entries", expect_nonzero::<U16>(u16be())),
                    ("num_palettes", expect_nonzero::<U16>(u16be())),
                    ("num_color_records", expect_nonzero::<U16>(u16be())),
                    (
                        "color_records_array",
                        pseudo_record(
                            [("offset", u32be())],
                            with_view(
                                vvar("table_view").offset(var("offset")),
                                read_array(var("num_color_records"), color_record),
                            ),
                        ),
                    ),
                    (
                        "color_record_indices",
                        // readarray-eligible: bare u16be() primitives, so `kind` is
                        // `BaseKind::U16BE` with no FormatRef required. Parsed in-line at the
                        // current cursor position with no pre-existing view-offset context, so
                        // migrating needs from_here(read_array(var("num_palettes"),
                        // BaseKind::U16BE)), mirroring the `widths` field in hdmx.rs's
                        // device_record.
                        repeat_count(var("num_palettes"), u16be()),
                    ),
                ],
                "version",
                [
                    (0u16, "Version0", Vec::new()),
                    (
                        1u16,
                        "Version1",
                        vec![
                            // NOTE - we duplicate `num_palettes` here because the `palette_types_array` phase-two parser needs that argument and it is not locally visible in the `extra` type unless we use UnifiedRecord
                            ("num_palettes", compute(var("num_palettes"))),
                            (
                                "palette_types_array",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    palette_types_array.invoke_args([var("num_palettes")]),
                                ),
                            ),
                            (
                                "palette_labels_array",
                                palette_labels_array
                                    .invoke_args_views([var("num_palettes")], [vvar("table_view")]),
                            ),
                            (
                                "palette_entry_labels_array",
                                palette_entry_labels_array.invoke_args_views(
                                    [var("num_palette_entries")],
                                    [vvar("table_view")],
                                ),
                            ),
                        ],
                    ),
                ],
                "extra",
                NestingKind::MinimalVariation,
            ),
        ),
    )
}

/// CPAL Palette Types Array (Version 1)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-types-array
///
/// Potentially readarray-eligible - needs `bitflags` support. `palette_type` is a
/// `bit_fields_u32` record whose only raw read is the packed-bits primitive; every exposed
/// field is a derived `Format::Compute` bit-mask, so `analyze_fixed_shape` rejects it as-is.
/// Also unlike `palette_labels_array`/`palette_entry_labels_array` below, this helper has no
/// view parameter (`DepFormat<1, 0>`, via `register_format_args`) - the offset/view handling
/// is done by the caller in `table()` via `read_phantom_view_offset32(vvar("table_view"), ..)`.
/// If bitflags support lands, migrating would also mean switching this to
/// `register_format_args_views` (taking `table_view`) so it can do its own
/// offset-field + `with_view` + `read_array` construction the way the other two arrays do,
/// at which point `palette_type` (already a bare FormatRef) would be passed directly to
/// `read_array`, dropping the `.call()`.
fn palette_types_array(module: &mut FormatModule) -> DepFormat<1, 0> {
    let flags = bit_fields_u32([
        BitFieldKind::Reserved {
            bit_width: 30,
            check_zero: true,
        }, // Bits 31-2: reserved (set to 0)
        BitFieldKind::FlagBit("usable_with_dark_background"), // Bit 1: palette is usable with dark background
        BitFieldKind::FlagBit("usable_with_light_background"), // Bit 0: palette is usable with light background
    ]);
    let palette_type = module.define_format("opentype.cpal.palette_type", flags);
    module.register_format_args(
        "opentype.cpal.palette_types_array",
        [(Label::Borrowed("num_palettes"), ValueType::U16)],
        // TODO[epic=adhoc-readarray] - we ideally want this to be a ReadArray of flags values, but the required machinery isn't yet implemented
        repeat_count(var("num_palettes"), palette_type.call()),
    )
}

/// CPAL Palette Labels Array (Version 1)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-labels-array
fn palette_labels_array(module: &mut FormatModule) -> DepFormat<1, 1> {
    module.register_format_args_views(
        "opentype.cpal.palette_labels_array",
        [(Label::Borrowed("num_palettes"), ValueType::U16)],
        [(Label::Borrowed("table_view"))],
        read_array_view_offset32(vvar("table_view"), var("num_palettes"), BaseKind::U16BE),
    )
}

/// CPAL Palette Entry Labels Array (Version 1)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entry-label-array
fn palette_entry_labels_array(module: &mut FormatModule) -> DepFormat<1, 1> {
    module.register_format_args_views(
        "opentype.cpal.palette_entry_labels_array",
        [(Label::Borrowed("num_palette_entries"), ValueType::U16)],
        [(Label::Borrowed("table_view"))],
        read_array_view_offset32(
            vvar("table_view"),
            var("num_palette_entries"),
            BaseKind::U16BE,
        ),
    )
}

fn color_record(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.cpal.color_record",
        record_repeat(["blue", "green", "red", "alpha"], u8()),
    )
}
