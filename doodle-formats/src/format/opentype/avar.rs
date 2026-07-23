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
            // readarray-eligible once `analyze_fixed_shape`/`as_base_kind_read` can see through
            // `Format::Variant` wrapping: `axis_value_map` (both fields `f2dot14()`, i.e.
            // `fmt_variant("F2Dot14", u16be())`) is otherwise a closed, flat, all-primitive
            // record -- the `Format::Variant` node around each `u16be()` is the sole blocker.
            // No new `FormatRef` needed: `axis_value_map` is already registered, reuse it
            // directly (drop `.call()`). Not reached via any offset/view, so would need
            // `from_here(read_array(var("position_map_count"), axis_value_map))`.
            (
                "axis_value_maps",
                repeat_count(var("position_map_count"), axis_value_map.call()),
            ),
        ]),
    )
}
