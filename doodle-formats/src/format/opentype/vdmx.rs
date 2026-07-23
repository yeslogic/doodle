use super::*;

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let vdmx_group = vdmx_group(module);
    let ratio_range = record_repeat(
        ["b_char_set", "x_ratio", "y_start_ratio", "y_end_ratio"],
        u8(),
    );

    module.define_format(
        "opentype.vdmx.table",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("version", expects_u16be([0, 1])),
                // REVIEW[epic=validation] - we do not expect num_recs and num_ratios to ever differ
                ("num_recs", u16be()),
                ("num_ratios", expect_eq(u16be(), var("num_recs"))),
                // readarray-eligible: `ratio_range` is a flat record of four bare u8 fields
                // (b_char_set, x_ratio, y_start_ratio, y_end_ratio) via record_repeat(..., u8()),
                // the same shape as cpal.rs::color_record. It's only a local Format value here,
                // not a registered FormatRef, so one would need to be registered (e.g. via
                // module.define_format) to use as the FixedReadKind::FixedFormat element. It's
                // parsed in-line in `table_view`'s record with no offset indirection, so this
                // would need `from_here(read_array(var("num_ratios"), <new_ratio_range_ref>))`,
                // matching the pattern used for `widths` in hdmx.rs::device_record.
                // TODO[epic=adhoc-readarray]
                ("ratio_range", repeat_count(var("num_ratios"), ratio_range)),
                (
                    "vdmx_group_offsets",
                    repeat_count(
                        // NOTE - the specification uses `numRatios` as the array-length, and not `numRecs` as might otherwise be expected
                        var("num_ratios"),
                        util::read_phantom_view_offset16(vvar("table_view"), vdmx_group.call()),
                    ),
                ),
            ]),
        ),
    )
}

fn vdmx_group(module: &mut FormatModule) -> FormatRef {
    let v_table = module.define_format(
        "opentype.vdmx.group.v_table",
        record([
            ("y_pel_height", u16be()), // yPelHeight to which values apply
            ("y_max", i16be()),        // maximum value (in pels) for this yPelHeight
            ("y_min", i16be()),        // minimum value (in pels) for this yPelHeight
        ]),
    );
    module.define_format(
        "opentype.vdmx.group",
        record([
            ("recs", u16be()),  // Number of height records in this group
            ("start_sz", u8()), // Starting yPelHeight
            ("end_sz", u8()),   // Ending yPelHeight
            ("entry", repeat_count(var("recs"), v_table.call())),
        ]),
    )
}
