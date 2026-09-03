use super::*;

pub(crate) fn table_def() -> Format {
    record_auto([
        ("major_version", util::expect_u16be(1)),
        (
            "minor_version",
            util::expects_u16be([0x0000, 0x1000]), // NOTE - due to how versions are encoded for hhea/vhea tables v1.1 is `00 01 . 10 00`
        ), // FIXME - hhea only has 1.0, but vhea has 1.1 as well, so we compromise by allowing it in both to re-use it properly
        ("ascent", i16be()), // distance from baseline to highest ascender, in font design units
        ("descent", i16be()), // distance from baseline to lowest descender, in font design units
        ("line_gap", i16be()), // intended gap between baselines, in font design units
        ("advance_width_max", u16be()), // must be consistent with horizontal metrics
        ("min_left_side_bearing", i16be()), // must be consistent with horizontal metrics
        ("min_right_side_bearing", i16be()), // must be consistent with horizontal metrics
        ("x_max_extent", i16be()), // `max(left_side_bearing + (x_max - x_min))`
        // slope of the caret (rise/run), (1/0) for vertical caret
        ("caret_slope", record_repeat(["rise", "run"], i16be())),
        ("caret_offset", i16be()), // 0 for non-slanted fonts
        ("__reservedX4", tuple_repeat(4, util::expect_u16be(0))), // NOTE: 4 separate isolated fields in fathom
        ("metric_data_format", util::expect_u16be(0)),
        // number of `long_horizontal_metric` records in the `htmx_table`, `long_vertical_metrics` in `vmtx_table`
        ("number_of_long_metrics", u16be()),
    ])
}

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    module.define_format("opentype.hhea.table", table_def())
}
