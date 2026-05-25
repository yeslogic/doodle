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
                // TODO - RatioRange is a fixed 32-bit record so it ought to be compatible with ReadArray, eventually
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
