use super::*;

/// `gasp` table definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gasp
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let gasp_record = gasp_record(module);
    module.define_format(
        "opentype.gasp.table",
        record([
            ("version", u16be()),
            ("num_ranges", u16be()),
            (
                "gasp_ranges",
                repeat_count(var("num_ranges"), gasp_record.invoke_args([var("version")])),
            ),
        ]),
    )
}

/// Format for a gasp-record, parametric in the version of the `gasp` table.
fn gasp_record(module: &mut FormatModule) -> DepFormat<1, 0> {
    use BitFieldKind::*;

    let ver0flags = bit_fields_u16([
        Reserved {
            bit_width: 12,
            check_zero: false,
        }, // Reserved in all versions
        Reserved {
            bit_width: 2,
            check_zero: false,
        }, // Version 1 only, but not actually reserved
        FlagBit("dogray"),
        FlagBit("gridfit"),
    ]);
    let ver1flags = bit_fields_u16([
        Reserved {
            bit_width: 12,
            check_zero: false,
        }, // Reserved in all versions
        FlagBit("symmetric_smoothing"),
        FlagBit("symmetric_gridfit"),
        FlagBit("dogray"),
        FlagBit("gridfit"),
    ]);

    module.register_format_args(
        "opentype.gasp.gasp_record",
        [(Label::Borrowed("version"), ValueType::U16)],
        record([
            ("range_max_ppem", u16be()),
            (
                "range_gasp_behavior",
                match_variant(
                    var("version"),
                    [
                        (Pattern::U16(0), "Version0", ver0flags),
                        (Pattern::U16(1), "Version1", ver1flags),
                        // REVIEW[epic=catchall-policy] - do we need this catch-all?
                        (Pattern::Wildcard, "BadVersion", Format::Fail), // NOTE - the name of this variant is arbitrary since it won't actually appear anywhere
                    ],
                ),
            ),
        ]),
    )
}
