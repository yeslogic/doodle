use super::*;

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let segment_maps = segment_maps(module);

    module.define_format(
        "opentype.avar.table",
        record_auto([
            ("major_version", expect_u16be(1)),
            ("minor_version", expect_u16be(0)),
            ("__reserved", expect_u16be(0)),
            ("axis_count", u16be()), // NOTE - should agree with `axis_count` in `fvar`, which is required in all variable fonts
            (
                "axis_segment_maps",
                repeat_count(var("axis_count"), segment_maps.call()),
            ),
        ]),
    )
}

fn segment_maps(module: &mut FormatModule) -> FormatRef {
    let axis_value_map = module.define_format(
        "opentype.avar.axis_value_map",
        record_repeat(["from_coordinate", "to_coordinate"], f2dot14()),
    );

    module.define_format(
        "opentype.avar.segment_maps",
        record_auto([
            ("position_map_count", u16be()),
            (
                "axis_value_maps",
                repeat_count(var("position_map_count"), axis_value_map.call()),
            ),
        ]),
    )
}
