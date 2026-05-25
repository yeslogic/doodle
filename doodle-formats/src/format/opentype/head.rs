use super::*;

/// Opentype `head` table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/head
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    // FIXME - replace with bit_fields_u16 if appropriate
    let head_table_flags = u16be();

    let long_date_time = module.define_format("opentype.types.long_date_time", i64be());

    let xy_min_max = record_repeat(["x_min", "y_min", "x_max", "y_max"], i16be());

    // REVIEW[epic=check-zero] - determine whether we should check for zeroing of reserved bit-fields positions
    const SHOULD_CHECK_ZERO: bool = false;

    let head_table_style_flags = bit_fields_u16([
        BitFieldKind::Reserved {
            bit_width: 9,
            check_zero: SHOULD_CHECK_ZERO,
        },
        BitFieldKind::FlagBit("extended"),
        BitFieldKind::FlagBit("condensed"),
        BitFieldKind::FlagBit("shadow"),
        BitFieldKind::FlagBit("outline"),
        BitFieldKind::FlagBit("underline"),
        BitFieldKind::FlagBit("italic"),
        BitFieldKind::FlagBit("bold"),
    ]);

    // NOTE - Should be 2 for modern fonts but we shouldn't enforce that too strongly
    /* ConstEnum(i16be) {
     *     Mixed    =  0,
     *     StrongLR =  1,
     *     WeakLR   =  2,
     *     StrongRL = -1,
     *     WeakRL   = -2,
     * }
     */
    let glyph_dir_hint = chain(
        i16be(),
        "disc",
        compute(interpret_as_enum(
            var("disc"),
            [
                (Pattern::z_const(0), "Mixed"),
                (Pattern::z_const(1), "StrongLR"),
                (Pattern::z_const(2), "WeakLR"),
                (Pattern::z_const(-1), "StrongRL"),
                (Pattern::z_const(-2), "WeakRL"),
            ],
            Some("UnknownDirHint"),
        )),
    );

    module.define_format(
        "opentype.head_table",
        record([
            ("major_version", util::expect_u16be(1)),
            ("minor_version", util::expect_u16be(0)),
            ("font_revision", util::fixed32be()),
            ("checksum_adjustment", u32be()),
            ("magic_number", is_bytes(&[0x5F, 0x0F, 0x3C, 0xF5])),
            ("flags", head_table_flags),
            ("units_per_em", where_between_u16(u16be(), 16, 16384)),
            ("created", long_date_time.call()),
            ("modified", long_date_time.call()),
            ("glyph_extents", xy_min_max),
            ("mac_style", head_table_style_flags),
            ("lowest_rec_ppem", u16be()),
            ("font_direction_hint", glyph_dir_hint),
            (
                "index_to_loc_format",
                where_between_u16(u16be(), SHORT_OFFSET16, LONG_OFFSET32),
            ),
            ("glyph_data_format", util::expect_u16be(0)),
        ]),
    )
}
