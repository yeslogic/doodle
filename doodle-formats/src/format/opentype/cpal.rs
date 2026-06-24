use super::*;

/// Format specification for `CPAL` table
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/cpal
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let color_record_array = color_record_array(module);
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
                    ("version", expects_u16be([0, 1])),
                    // TODO[epic=validation] - the specification does not explicitly state that numPaletteEntries must be >0, but it is implied by other fields being required-nonzero
                    ("num_palette_entries", expect_nonzero::<U16>(u16be())),
                    ("num_palettes", expect_nonzero::<U16>(u16be())),
                    ("num_color_records", expect_nonzero::<U16>(u16be())),
                    (
                        "color_records_array",
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            color_record_array.invoke_args([var("num_color_records")]),
                        ),
                    ),
                    (
                        "color_record_indices",
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
                            (
                                "palette_types_array",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    palette_types_array.invoke_args([var("num_palettes")]),
                                ),
                            ),
                            (
                                "palette_labels_array",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    palette_labels_array.invoke_args([var("num_palettes")]),
                                ),
                            ),
                            (
                                "palette_entry_labels_array",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    palette_entry_labels_array
                                        .invoke_args([var("num_palette_entries")]),
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
fn palette_types_array(module: &mut FormatModule) -> DepFormat<1, 0> {
    let flags = bit_fields_u32([
        BitFieldKind::Reserved {
            bit_width: 30,
            check_zero: true,
        }, // Bits 31-2: reserved (set to 0)
        BitFieldKind::FlagBit("usable_with_dark_background"), // Bit 1: palette is usable with dark background
        BitFieldKind::FlagBit("usable_with_light_background"), // Bit 0: palette is usable with light background
    ]);
    module.register_format_args(
        "opentype.cpal.palette_types_array",
        [(Label::Borrowed("num_palettes"), ValueType::U16)],
        // TODO[epic=adhoc-readarray] - we ideally want this to be a ReadArray of flags values, but the required machinery isn't yet implemented
        repeat_count(var("num_palettes"), flags),
    )
}

/// CPAL Palette Labels Array (Version 1)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-labels-array
fn palette_labels_array(module: &mut FormatModule) -> DepFormat<1, 0> {
    module.register_format_args(
        "opentype.cpal.palette_labels_array",
        [(Label::Borrowed("num_palettes"), ValueType::U16)],
        repeat_count(var("num_palettes"), super::name::name_id()),
        // from_here(read_array(var("num_palettes"), BaseKind::U16BE)),
    )
}

/// CPAL Palette Entry Labels Array (Version 1)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entry-label-array
fn palette_entry_labels_array(module: &mut FormatModule) -> DepFormat<1, 0> {
    module.register_format_args(
        "opentype.cpal.palette_entry_labels_array",
        [(Label::Borrowed("num_palette_entries"), ValueType::U16)],
        repeat_count(var("num_palette_entries"), super::name::name_id()),
        // from_here(read_array(var("num_palette_entries"), BaseKind::U16BE)),
    )
}

fn color_record_array(module: &mut FormatModule) -> DepFormat<1, 0> {
    let color_record = color_record(module);
    module.register_format_args(
        "opentype.cpal.color_record_array",
        [(Label::Borrowed("num_color_records"), ValueType::U16)],
        // TODO[epic=adhoc-readarray] - we ideally want this to be a ReadArray of ColorRecord, but the required machinery isn't yet implemented
        repeat_count(var("num_color_records"), color_record.call()),
    )
}

fn color_record(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.cpal.color_record",
        record_repeat(["blue", "green", "red", "alpha"], u8()),
    )
}
