use super::*;

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let long_horizontal_metric =
        record([("advance_width", u16be()), ("left_side_bearing", i16be())]);

    module.define_format_args(
        "opentype.hmtx.table",
        vec![
            (Label::Borrowed("num_long_metrics"), ValueType::U16),
            (Label::Borrowed("num_glyphs"), ValueType::U16),
        ],
        record([
            (
                "long_metrics",
                repeat_count(var("num_long_metrics"), long_horizontal_metric),
            ),
            (
                "left_side_bearings", // REVIEW - 'top_side_bearings' in vmtx
                repeat_count(sub(var("num_glyphs"), var("num_long_metrics")), i16be()),
            ),
        ]),
    )
}
