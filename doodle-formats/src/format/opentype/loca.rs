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
                        // readarray-eligible: element is bare `u16be()` -> BaseKind::U16BE, no
                        // FormatRef needed. `opentype.loca.table` has no local ViewExpr (no
                        // let_view in this file) -- would need
                        // `from_here(read_array(succ(var("num_glyphs")), BaseKind::U16BE))`.
                        repeat_count(succ(var("num_glyphs")), u16be()),
                    ),
                    (
                        Pattern::U16(LONG_OFFSET32),
                        "Offsets32",
                        // readarray-eligible: element is bare `u32be()` -> BaseKind::U32BE, no
                        // FormatRef needed. Same no-local-view situation as the Offsets16 arm
                        // above -- would need
                        // `from_here(read_array(succ(var("num_glyphs")), BaseKind::U32BE))`.
                        repeat_count(succ(var("num_glyphs")), u32be()),
                    ),
                ],
            ),
        )]),
    )
}
