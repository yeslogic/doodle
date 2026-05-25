use super::*;

/// Determines the number of pascal-strings to read based on the largest glyph name index in the provided array
///
/// If the array is empty, returns `0`.
///
/// If the array is non-empty, returns the largest glyph name index + 1,
/// or 0 if that result is less than or equal to `258` (the number of standard Macintosh glyph-names which are treated as
/// indices 0..=257).
fn extra_name_count(index_array: Expr) -> Expr {
    const MACINTOSH_NAME_COUNT: u16 = 258;
    /// Computes the max-value of an array of u16s, None if empty
    fn array_max_u16(array: Expr) -> Expr {
        left_fold(
            lambda_tuple(["acc", "x"], expr_max(var("acc"), var("x"))),
            Expr::U16(0),
            ValueType::U16,
            array,
        )
    }
    let max_index = array_max_u16(index_array);
    let tot_len = succ(max_index);
    // TODO - add monus/saturating-subtraction operator
    expr_if_else(
        expr_lte(tot_len.clone(), Expr::U16(MACINTOSH_NAME_COUNT)),
        Expr::U16(0),
        sub(tot_len, Expr::U16(MACINTOSH_NAME_COUNT)),
    )
}

fn version2(module: &mut FormatModule) -> FormatRef {
    let pascal_string = module.define_format(
        "opentype.post.pascal_string",
        // REVIEW - is 'record with length' better than just the string itself?
        record([
            ("length", u8()),
            (
                "string",
                let_view(
                    "pascal_string_data",
                    mk_ascii_string(with_view(
                        vvar("pascal_string_data"),
                        capture_bytes(var("length")),
                    )),
                ),
            ),
        ]),
    );
    module.define_format(
        "opentype.post.version2",
        record([
            ("num_glyphs", u16be()),
            ("glyph_name_index", repeat_count(var("num_glyphs"), u16be())),
            (
                "string_data",
                repeat_count(
                    extra_name_count(var("glyph_name_index")),
                    pascal_string.call(),
                ),
            ),
        ]),
    )
}

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let version2 = version2(module);

    let postv2dot5 = record([
        ("num_glyphs", u16be()),
        // TODO - ReadArray<'_, I8> would work here if we had a model compatible with it
        ("offset", repeat_count(var("num_glyphs"), i8())),
    ]);

    module.define_format(
        "opentype.post.table",
        record([
            ("version", util::version16_16()),
            ("italic_angle", util::fixed32be()),
            ("underline_position", i16be()),
            ("underline_thickness", i16be()),
            ("is_fixed_pitch", u32be()), // nonzero <=> fixed pitch
            ("min_mem_type42", u32be()),
            ("max_mem_type42", u32be()),
            ("min_mem_type1", u32be()),
            ("max_mem_type1", u32be()),
            (
                "names",
                match_variant(
                    var("version"),
                    [
                        (Pattern::U32(0x0001_0000), "Version1", Format::EMPTY),
                        (Pattern::U32(0x0002_0000), "Version2", version2.call()),
                        (Pattern::U32(0x0002_5000), "Version2Dot5", postv2dot5),
                        (Pattern::U32(0x0003_0000), "Version3", Format::EMPTY),
                        // NOTE - as-is, we store the unexpected version-value in the variant for debugging purposes
                        (bind("unknown"), "VersionUnknown", compute(var("unknown"))),
                    ],
                ),
            ),
        ]),
    )
}
