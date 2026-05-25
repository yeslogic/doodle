use super::*;

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    module.define_format_args(
        "opentype.loca.table",
        vec![
            (Label::Borrowed("num_glyphs"), ValueType::U16),
            (Label::Borrowed("index_to_loc_format"), ValueType::U16),
        ],
        record([(
            "offsets",
            match_variant(
                var("index_to_loc_format"),
                [
                    (
                        Pattern::U16(SHORT_OFFSET16),
                        "Offsets16",
                        repeat_count(succ(var("num_glyphs")), u16be()),
                    ),
                    (
                        Pattern::U16(LONG_OFFSET32),
                        "Offsets32",
                        repeat_count(succ(var("num_glyphs")), u32be()),
                    ),
                ],
            ),
        )]),
    )
}
