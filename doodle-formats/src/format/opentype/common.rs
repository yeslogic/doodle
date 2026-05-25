use super::*;

pub(crate) fn item_variation_store(module: &mut FormatModule) -> FormatRef {
    let variation_region_list = variation_region_list(module);
    let item_variation_data = item_variation_data(module);
    module.define_format(
        "opentype.common.item_variation_store",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("format", util::expect_u16be(1)),
                (
                    "variation_region_list",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        variation_region_list.call(),
                    ),
                ),
                ("item_variation_data_count", u16be()),
                (
                    "item_variation_data_list",
                    repeat_count(
                        var("item_variation_data_count"),
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            item_variation_data.call(),
                        ),
                    ),
                ),
            ]),
        ),
    )
}

fn variation_region_list(module: &mut FormatModule) -> FormatRef {
    // NOTE - all coordinates should be in range [-1.0, +1.0], and start <= peak <= end; must either all be non-positive or non-negative, or else peak must be 0 for negative start and non-negative end.
    let region_axis_coordinates =
        record_repeat(["start_coord", "peak_coord", "end_coord"], util::f2dot14());
    let variation_region = |axis_count: Expr| {
        record([(
            "region_axes",
            repeat_count(axis_count, region_axis_coordinates),
        )])
    };
    module.define_format(
        "opentype.common.variation-region-list",
        record([
            ("axis_count", u16be()), // NOTE - number of variation axes; should be the same as `axis_cout` in `'fvar'` table
            (
                "region_count",
                where_within(u16be(), Bounds::at_most(i16::MAX as usize)),
            ),
            (
                "variation_regions",
                repeat_count(var("region_count"), variation_region(var("axis_count"))),
            ),
        ]),
    )
}

/// Constructor for `DeltaSet` records used in ItemVariationStore
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store-header-and-item-variation-data-subtables
///
/// Takes two formats, one for full-width delta-values (`full_format`)
/// and one for half-width delta-values (`half_format`), which will either be
/// i32/i16 or i16/i8, based on the `long_words` flag  of the `word_delta_count` field
/// in the encompassing `ItemVariationData` table (see [`item_variation_data`]).
///
/// The number of full-width deltas to parse is parametrized by the expression `word_count`,
/// and the total number of delta-values (of either width) is given by `region_index_count`;
/// we therefore derive the number of half-width deltas as the difference between these two values.
///
/// Due to implementation limits, we cannot construct a contiguous array holding all deltas in a single
/// run, and instead store a record with a separate array for each run of homogenously-typed deltas.
fn deltas(
    full_format: Format,
    half_format: Format,
    word_count: Expr,
    region_index_count: Expr,
) -> Format {
    // REVIEW - does a pair of `(ReadArray<U32BE>, ReadArray<U16BE>)` (and similarly for U16Be/U8) work better?
    record([
        (
            "delta_data_full_word",
            repeat_count(word_count.clone(), full_format),
        ),
        (
            "delta_data_half_word",
            repeat_count(sub(region_index_count, word_count), half_format),
        ),
    ])
}

pub(crate) fn item_variation_data(module: &mut FormatModule) -> FormatRef {
    let delta_sets = |item_count: Expr, word_delta_count: Expr, region_index_count: Expr| {
        if_then_else(
            record_proj(word_delta_count.clone(), "long_words"),
            fmt_variant(
                "Delta32Sets",
                repeat_count(
                    item_count.clone(),
                    deltas(
                        i32be(),
                        i16be(),
                        record_proj(word_delta_count.clone(), "word_count"),
                        region_index_count.clone(),
                    ),
                ),
            ),
            fmt_variant(
                "Delta16Sets",
                repeat_count(
                    item_count,
                    deltas(
                        i16be(),
                        i8(),
                        record_proj(word_delta_count.clone(), "word_count"),
                        region_index_count,
                    ),
                ),
            ),
        )
    };
    module.define_format(
        "opentype.common.item-variation-data",
        record([
            ("item_count", u16be()),
            (
                "word_delta_count",
                util::hi_flag_u15be("long_words", "word_count"),
            ),
            ("region_index_count", u16be()),
            (
                "region_indices",
                repeat_count(var("region_index_count"), u16be()),
            ),
            (
                "delta_sets",
                delta_sets(
                    var("item_count"),
                    var("word_delta_count"),
                    var("region_index_count"),
                ),
            ),
        ]),
    )
}

pub(crate) fn device_or_variation_index_table(module: &mut FormatModule) -> FormatRef {
    let device_table = device_table();
    let variation_index_table = record([
        ("delta_set_outer_index", u16be()),
        ("delta_set_inner_index", u16be()),
        ("delta_format", is_bytes(&(0x8000u16).to_be_bytes())),
    ]);
    let other_table = |delta_format: Expr| {
        record([
            // FIXME - placeholder names `field0` and `field1`, rename as appropriate or remove this comment
            ("field0", u16be()),
            ("field1", u16be()),
            ("delta_format", compute(delta_format)),
        ])
    };
    module.define_format(
        "opentype.common.device_or_variation_index_table",
        util::peek_field_then(
            &[
                ("__skipped0", u16be()), // `startSize` or `deltaSetOuterIndex`
                ("__skipped1", u16be()), // `endSize` or `deltaSetInnerIndex`
                ("delta_format", u16be()),
            ],
            match_variant(
                var("delta_format"),
                [
                    (Pattern::Int(Bounds::new(1, 3)), "DeviceTable", device_table),
                    (
                        Pattern::U16(0x8000),
                        "VariationIndexTable",
                        variation_index_table,
                    ),
                    // Construct a raw variant for nonce-values without any further interpretation
                    (bind("other"), "OtherTable", other_table(var("other"))),
                ],
            ),
        ),
    )
}

pub(crate) fn device_table() -> Format {
    // quotient = numerator / denominator # int division (u16 -> u16 -> u16)
    // if quotient * denominator < numerator:
    //     quotient + 1
    // else:
    //     quotient
    let u16_div_ceil = |numerator: Expr, denominator: Expr| {
        let quotient = div(numerator.clone(), denominator.clone());
        expr_if_else(
            expr_lt(mul(quotient.clone(), denominator), numerator),
            succ(quotient.clone()),
            quotient,
        )
    };

    // NOTE - Converts a 'number of delta-values' to a `number of 16-bit words', based on the implied bit-width of a single delta-value,
    let packed_array_length = |delta_format: Expr, num_sizes: Expr| {
        let divide_by = |divisor: u16| u16_div_ceil(num_sizes.clone(), Expr::U16(divisor));
        expr_match(
            delta_format,
            [
                (Pattern::U16(1), divide_by(8)),   // 2-bit deltas, 8 per Uint16
                (Pattern::U16(2), divide_by(4)),   // 4-bit deltas, 4 per Uint16
                (Pattern::U16(3), divide_by(2)),   // 8-bit deltas, 2 per Uint16
                (Pattern::Wildcard, Expr::U16(0)), // Wrong Branch
            ],
        )
    };

    let num_sizes = |start: Expr, end: Expr| succ(sub(end, start));

    // REVIEW - should this be a module definition (to shorten type-name)?
    record([
        ("start_size", u16be()),
        ("end_size", u16be()),
        ("delta_format", u16be()),
        (
            "delta_values",
            repeat_count(
                packed_array_length(
                    var("delta_format"),
                    num_sizes(var("start_size"), var("end_size")),
                ),
                u16be(),
            ),
        ),
    ])
}

pub(crate) fn coverage_table(module: &mut FormatModule) -> FormatRef {
    // REVIEW - should this be a module definition (to shorten type-name)?
    let coverage_format_1 = record([
        ("glyph_count", u16be()),
        ("glyph_array", repeat_count(var("glyph_count"), u16be())),
    ]);

    // REVIEW - should this be a module definition (to shorten type-name)?
    let coverage_format_2 = {
        // REVIEW - should this be a module definition (to shorten type-name)?
        let range_record = record([
            ("start_glyph_id", u16be()),
            ("end_glyph_id", u16be()),
            ("start_coverage_index", u16be()),
        ]);

        record([
            ("range_count", u16be()),
            (
                "range_records",
                repeat_count(var("range_count"), range_record),
            ),
        ])
    };

    module.define_format(
        "opentype.coverage_table",
        record([
            ("coverage_format", u16be()),
            (
                "data",
                match_variant(
                    var("coverage_format"),
                    [
                        (Pattern::U16(1), "Format1", coverage_format_1),
                        (Pattern::U16(2), "Format2", coverage_format_2),
                        // REVIEW[epic=catchall-policy] - do we need this catch-all?
                        (Pattern::Wildcard, "BadFormat", Format::Fail),
                    ],
                ),
            ),
        ]),
    )
}

/// Class Definition Table
///
/// | Class | Description                                               |
/// |-------|-----------------------------------------------------------|
/// | 1     | Base glyph (single character, spacing glyph)              |
/// | 2     | Ligature glyph (multiple character, spacing glyph)        |
/// | 3     | Mark glyph (non-spacing combining glyph)                  |
/// | 4     | Component glyph (part of single character, spacing glyph) |
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table
pub(crate) fn class_def(module: &mut FormatModule) -> FormatRef {
    // - [Microsoft's OpenType Spec: Class Definition Table Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table-format-1)
    let class_format_1 = record([
        ("start_glyph_id", u16be()),
        ("glyph_count", u16be()),
        (
            "class_value_array",
            repeat_count(var("glyph_count"), u16be()),
        ),
    ]);
    // - [Microsoft's OpenType Spec: Class Definition Table Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table-format-2)
    let class_format_2 = {
        let class_range_record = record([
            ("start_glyph_id", u16be()),
            ("end_glyph_id", u16be()),
            ("class", u16be()),
        ]);

        record([
            ("class_range_count", u16be()),
            (
                "class_range_records",
                repeat_count(var("class_range_count"), class_range_record),
            ),
        ])
    };

    module.define_format(
        "opentype.class_def",
        record([
            ("class_format", u16be()),
            (
                "data",
                match_variant(
                    var("class_format"),
                    [
                        (Pattern::U16(1), "Format1", class_format_1),
                        (Pattern::U16(2), "Format2", class_format_2),
                        // REVIEW[epic=catchall-policy] - do we need this catch-all?
                        (Pattern::Wildcard, "BadFormat", Format::Fail),
                    ],
                ),
            ),
        ]),
    )
}
