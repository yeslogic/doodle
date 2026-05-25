use super::*;

/// Opentype OS/2 table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/os2
pub(crate) fn table(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
    module.define_format_args(
        "opentype.os2.table",
        vec![(
            Label::Borrowed("table_length"),
            ValueType::Base(BaseType::U32),
        )],
        record([
            ("version", u16be()),
            ("x_avg_char_width", i16be()),
            ("us_weight_class", u16be()),
            ("us_width_class", u16be()),
            ("fs_type", u16be()),
            ("y_subscript_x_size", i16be()),
            ("y_subscript_y_size", i16be()),
            ("y_subscript_x_offset", i16be()),
            ("y_subscript_y_offset", i16be()),
            ("y_superscript_x_size", i16be()),
            ("y_superscript_y_size", i16be()),
            ("y_superscript_x_offset", i16be()),
            ("y_superscript_y_offset", i16be()),
            ("y_strikeout_size", i16be()),
            ("y_strikeout_position", i16be()),
            ("s_family_class", i16be()),
            ("panose", repeat_count(Expr::U8(10), u8())),
            ("ul_unicode_range1", u32be()),
            ("ul_unicode_range2", u32be()),
            ("ul_unicode_range3", u32be()),
            ("ul_unicode_range4", u32be()),
            ("ach_vend_id", tag.call()),
            ("fs_selection", u16be()),
            ("us_first_char_index", u16be()),
            ("us_last_char_index", u16be()),
            ("data", version_record("version", var("table_length"))),
        ]),
    )
}

/// Conditional record-format consisting of OS/2 table fields for each version of the OS/2 table
///
/// Takes a variable-identifier `version_ident` corresponding to the scoped variable storing the version-number
/// as a u16, and a table-length expression `table_length`, both inherited from this function's caller, [`table`].
/// # Notes
///
/// Based on the notes in the Microsoft documentation for legacy OS/2 table version 0,
/// (https://learn.microsoft.com/en-us/typography/opentype/spec/os2#version-0),
/// a version 0 table with no more than 78 bytes is a valid OS/2 table whose final field
/// is `usLastCharIndex`, skipping the formally specified final 5 fields.
///
/// If the version is greater than 0, or the table is longer than 78 bytes, then the final 5 fields will be parsed,
/// and otherwise the Format returned by this function will yield `None`.
///
/// Each version of the OS/2 table has a different number of fields, but as they are strictly additive and do not
/// change between versions in which they are present, each version that adds more fields has its fields stored
/// as an optional nested-record in the previous version's record-of-extra-fields.
///
/// As versions 2, 3, and 4 have the same basic fields, only versions 1, 2, and 5 act as thresholds
/// for including extra fields (w.r.t. `version >= N` predicates)
fn version_record(version_ident: &'static str, table_length: Expr) -> Format {
    const V0_MIN_LENGTH: u32 = 78;
    cond_maybe(
        or(
            is_nonzero_u16(var(version_ident)),
            expr_gte(table_length, Expr::U32(V0_MIN_LENGTH)),
        ),
        record([
            ("s_typo_ascender", i16be()),
            ("s_typo_descender", i16be()),
            ("s_typo_line_gap", i16be()),
            ("us_win_ascent", u16be()),
            ("us_win_descent", u16be()),
            (
                "extra_fields_v1",
                cond_maybe(
                    is_within(var(version_ident), Bounds::at_least(1)),
                    record([
                        ("ul_code_page_range_1", u32be()),
                        ("ul_code_page_range_2", u32be()),
                        (
                            "extra_fields_v2",
                            cond_maybe(
                                is_within(var(version_ident), Bounds::at_least(2)),
                                record([
                                    ("sx_height", i16be()),
                                    ("s_cap_height", i16be()),
                                    ("us_default_char", u16be()),
                                    ("us_break_char", u16be()),
                                    ("us_max_context", u16be()),
                                    (
                                        "extra_fields_v5",
                                        cond_maybe(
                                            is_within(var(version_ident), Bounds::at_least(5)),
                                            record([
                                                ("us_lower_optical_point_size", u16be()),
                                                ("us_upper_optical_point_size", u16be()),
                                            ]),
                                        ),
                                    ),
                                ]),
                            ),
                        ),
                    ]),
                ),
            ),
        ]),
    )
}
