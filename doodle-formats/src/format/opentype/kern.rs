use super::*;

/// Format for `kern` table
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/kern
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let kern_subtable = subtable(module);
    module.define_format(
        "opentype.kern.table",
        record([
            ("version", util::expect_u16be(0)), // Table version number (KernHeader)
            ("n_tables", u16be()),
            (
                "subtables",
                repeat_count(var("n_tables"), kern_subtable.call()),
            ),
        ]),
    )
}

/// Kern subtable format
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/kern
fn subtable(module: &mut FormatModule) -> FormatRef {
    let kern_cov_flags = kern_cov_flags();
    // SECTION - `kern` subtable record-formats

    let format0 = subtable_format0(module);
    let format2 = subtable_format2(module);
    // !SECTION
    /* Previously defined as a slice_record but sufficiently large `n_pairs` values for Format0
     * could cause length to wrap around mod 65536 and lead to slice boundary violation
     * while reading `kern_pairs`
     */
    module.define_format(
        "opentype.kern.kern_subtable",
        record([
            ("version", util::expect_u16be(0)),
            ("length", u16be()), // NOTE - Cannot be trusted as overflow exists in the wild
            ("coverage", kern_cov_flags),
            (
                "data",
                match_variant(
                    record_proj(var("coverage"), "format"),
                    [
                        (Pattern::U16(0), "Format0", format0.call()),
                        (Pattern::U16(2), "Format2", format2.call()),
                        // REVIEW - do we even want to bother with an explicit catch-all failure branch?
                        (Pattern::Wildcard, "UnknownFormat", Format::Fail),
                    ],
                ),
            ),
        ]),
    )
}

/// Kern subtable format 0
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/kern#format-0
fn subtable_format0(module: &mut FormatModule) -> FormatRef {
    let kern_pair = record([
        ("left", u16be()),  // glyph index for left-hand glyph in kerning pair
        ("right", u16be()), // glyph index for right-hand glyph in kerning pair
        ("value", i16be()), // kerning value for given pair, in design-units. Positive values move characters apart, negative values move characters closer together.
    ]);
    module.define_format(
        "opentype.kern.subtable.format0",
        record([
            ("n_pairs", u16be()),
            ("search_range", u16be()), // sizeof(table_entry) * (2^(ilog2(n_pairs)))
            ("entry_selector", u16be()), // ilog2(n_pairs) [number of iterations of binary search algo to find a query]
            ("range_shift", u16be()),    // (nPairs - 2^(ilog2(nPairs))) * sizeof(table_entry)
            // NOTE - kern-pairs array is sorted by the value of the packed Word32 consisting of the bytes of `left` and `right` in that order (big-endian).
            ("kern_pairs", repeat_count(var("n_pairs"), kern_pair)),
        ]),
    )
}

mod format2 {
    use super::*;

    /// Helper function used to compute the number of glyphs in a left-or-right class table (for Format 2 kern subtables)
    pub(super) fn glyph_count(
        table_view: ViewExpr,
        class_table_offset: Expr,
        class_table: FormatRef,
    ) -> Format {
        chain(
            util::get_content_at_offset::<U16>(table_view, class_table_offset, class_table.call()),
            "opt_table",
            map_option(var("opt_table"), "class_table", |table| {
                compute(record_proj(table, "n_glyphs"))
            }),
        )
    }

    /// Simultaneously 2D/1D array for kerning values
    ///
    /// 'Left-by-right': each row represents a left-hand glyph class, each column represents a right-hand glyph class
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/kern#format-2
    ///
    /// # Notes
    ///
    /// The indices in ClassTables are scaled (J = 2 x j ; I = 2 x M x i) to facilitate offset-arithmetic for random access (TargetOffset(i,j) = BaseOffset + I + J)
    ///
    /// Requires additional parameters `table_view` and `class_table` to correctly parse the content at each class offset
    pub(super) fn kerning_array(module: &mut FormatModule) -> DepFormat<2, 0> {
        module.register_format_args(
            "opentype.kern.kerning_array",
            [
                (Label::Borrowed("left_glyph_count"), ValueType::U16),
                (Label::Borrowed("right_glyph_count"), ValueType::U16),
            ],
            record([
                ("left_glyph_count", compute(var("left_glyph_count"))),
                ("right_glyph_count", compute(var("right_glyph_count"))),
                (
                    "kerning_values",
                    // REVIEW - consider ReadArray<S16> instead
                    repeat_count(
                        var("left_glyph_count"), // N rows where there are N left-hand classes
                        repeat_count(
                            var("right_glyph_count"), // M columns
                            i16be(),                  // FWORD value at index (i, j)
                        ),
                    ),
                ),
            ]),
        )
    }

    pub(super) fn class_table(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.kern.class_table",
            record([
                ("first_glyph", u16be()), // first glyph in class range
                ("n_glyphs", u16be()),    // number of glyphs in class range
                ("class_values", repeat_count(var("n_glyphs"), u16be())), // class values for each glyph in class range
            ]),
        )
    }
}

/// Kern subtable format 2
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/kern#format-2
fn subtable_format2(module: &mut FormatModule) -> FormatRef {
    let class_table = format2::class_table(module);
    let kerning_array = format2::kerning_array(module);
    module.define_format(
        "opentype.kern.subtable.format2",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("row_width", u16be()), // width (in bytes) of a table row
                (
                    "left_class_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), class_table.call()),
                ),
                (
                    "right_class_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), class_table.call()),
                ),
                (
                    "kerning_array_offset",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        pseudo_record(
                            [
                                (
                                    "left_glyph_count",
                                    format2::glyph_count(
                                        vvar("table_view"),
                                        var("left_class_offset"),
                                        class_table,
                                    ),
                                ),
                                (
                                    "right_glyph_count",
                                    format2::glyph_count(
                                        vvar("table_view"),
                                        var("right_class_offset"),
                                        class_table,
                                    ),
                                ),
                            ],
                            kerning_array.invoke_args([
                                expr_unwrap(var("left_glyph_count")),
                                expr_unwrap(var("right_glyph_count")),
                            ]),
                        ),
                    ),
                ),
            ]),
        ),
    )
}

// REVIEW - consider refactoring into FormatRef registration
fn kern_cov_flags() -> Format {
    use BitFieldKind::*;
    // REVIEW[epic=check-zero] - should we consider changing this constant to `true`
    const SHOULD_CHECK_ZERO: bool = false;
    bit_fields_u16([
        BitsField {
            bit_width: 8,
            field_name: "format",
        },
        Reserved {
            bit_width: 4,
            check_zero: SHOULD_CHECK_ZERO,
        },
        FlagBit("override"), // Bit 3 - when true, value in this table replaces the current accumulator value
        FlagBit("cross_stream"), // Bit 2 - when true, kerning is perpendicular to text-flow (reset by 0x8000 in kerning data)
        FlagBit("minimum"), // Bit 1 - when true, table contains minimum values, otherwise the table has kerning values
        FlagBit("horizontal"), // Bit 0 - when true, table has horizontal data, otherwise vertical
    ])
}
