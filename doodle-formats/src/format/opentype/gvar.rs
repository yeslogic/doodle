use super::*;

// REVIEW - do we consider it sensible to set this to `true`?
const SHOULD_CHECK_ZERO: bool = false;

/// Packed deltas format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#packed-deltas
fn packed_deltas(total_deltas: Expr) -> Format {
    use BitFieldKind::*;
    let control_byte = bit_fields_u8([
        FlagBit("deltas_are_zero"), // If set, no values are stored but the logical count is incremented as if explicit all-zeroes were listed
        FlagBit("deltas_are_words"), // If set, each delta is i16-based; i8 otherwise
        BitsField {
            bit_width: 6,
            field_name: "delta_run_count",
        }, // 6-bit run-length
    ]);
    let run = record([
        ("control", control_byte),
        (
            "deltas",
            Format::Let(
                Label::Borrowed("run_length"),
                Box::new(succ(record_proj(var("control"), "delta_run_count"))),
                Box::new(if_then_else(
                    record_proj(var("control"), "deltas_are_zero"),
                    fmt_variant("Delta0", compute(var("run_length"))),
                    if_then_else(
                        record_proj(var("control"), "deltas_are_words"),
                        fmt_variant("Delta16", repeat_count(var("run_length"), i16be())),
                        fmt_variant("Delta8", repeat_count(var("run_length"), i8())),
                    ),
                )),
            ),
        ),
    ]);
    let is_finished = lambda_tuple(["totlen", "_seq"], expr_gte(var("totlen"), total_deltas));
    let update_totlen = lambda_tuple(
        ["acc", "run"],
        add(
            var("acc"),
            succ(as_u16(record_lens(
                var("run"),
                &["control", "delta_run_count"],
            ))),
        ),
    );
    accum_until(
        is_finished,
        update_totlen,
        Expr::U16(0),
        ValueType::U16,
        run,
    )
}

/// Format for the bit-field `flags` in the gvar header record
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#gvar-header
fn header_flags() -> Format {
    use BitFieldKind::*;
    // NOTE - controls whether or not a ParseError is raised if reserved bits of a packed-word are not all cleared
    const SHOULD_CHECK_ZERO: bool = false;

    bit_fields_u16([
        Reserved {
            bit_width: 15,
            check_zero: SHOULD_CHECK_ZERO,
        },
        FlagBit("is_long_offset"),
    ])
}

/// Helper for processing a `GlyphVariationData` table at a specific offset within
/// the `glyphVariationDataOffsets` array at the end of the gvar header.
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#gvar-header
///
/// - `axis_count :~ U16`: axis-count passed in from the gvar header
/// - `array_start :~ U32`: absolute position corresponding to the logical start-of-array (which offsets are implicitly relative to)
/// - `this_offset32 :~ U32`: relative offset where the GlyphVariationData table begins
/// - `next_offset32 :~ U32`: relative offset where the immediately following GlyphVariationData table begins
/// - `data_table`: Format definition for GlyphVariationData table, parametric over `axis_count`
fn data_table_array_entry(
    axis_count: Expr,
    array_view: ViewExpr,
    this_offset32: Expr,
    next_offset32: Expr,
    data_table: DepFormat<1, 0>,
) -> Format {
    cond_maybe(
        // NOTE - checks that the GlyphVariationData table is non-zero length
        expr_gt(next_offset32.clone(), this_offset32.clone()),
        fmt_let(
            "len",
            sub(next_offset32, this_offset32.clone()),
            // FIXME[epic=eager-view-parse] - this parse is more eager than we actually want
            parse_from_view(
                array_view.offset(this_offset32),
                slice(var("len"), data_table.invoke_args([axis_count])),
            ),
        ),
    )
}

/// Helper for processing the full array of GlypohVariationData tables at linked offsets, given
/// in an array at the end of the `gvar` header
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#gvar-header
///
/// - `axis_count :~ U16`: axis-count passed in from the gvar header
/// - `offsets :~ Offsets16([U16]) | Offsets32([U32])`: array of offsets stored in the gvar header
/// - `data_table`: Format definition for GlyphVariationData table, parametric in `axis_count`
fn data_table_array(axis_count: Expr, offsets: Expr, data_table: DepFormat<1, 0>) -> Format {
    let_view(
        "array_view",
        Format::Match(
            Box::new(offsets),
            vec![
                (
                    Pattern::Variant(Label::Borrowed("Offsets16"), Box::new(bind("half16s"))),
                    for_each_pair(
                        var("half16s"),
                        (scale2, scale2),
                        ["this_offs", "next_offs"],
                        data_table_array_entry(
                            axis_count.clone(),
                            vvar("array_view"),
                            var("this_offs"),
                            var("next_offs"),
                            data_table,
                        ),
                    ),
                ),
                (
                    Pattern::Variant(Label::Borrowed("Offsets32"), Box::new(bind("off32s"))),
                    for_each_pair(
                        var("off32s"),
                        (id, id),
                        ["this_offs", "next_offs"],
                        data_table_array_entry(
                            axis_count,
                            vvar("array_view"),
                            var("this_offs"),
                            var("next_offs"),
                            data_table,
                        ),
                    ),
                ),
            ],
        ),
    )
}

/// Format for processing the array-of-offsets in a gvar header, which can be either
/// U16Be or U32Be depending on the flag `is_long_offsets`. When the values are U16,
/// the actual offset requires scaling the raw u16be value by a factor of 2.
///
/// The number of entries in the array is `glyph_count + 1` (as the offsets are processed
/// pairwise to determine entry-length), similar to `loca` tables.
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#glyph-variations-table-format
fn offsets_array(is_long_offsets: Expr, glyph_count: Expr) -> Format {
    if_then_else(
        is_long_offsets,
        fmt_variant(
            "Offsets32",
            repeat_count(succ(glyph_count.clone()), u32be()),
        ),
        fmt_variant("Offsets16", repeat_count(succ(glyph_count), u16be())),
    )
}

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let gvar_flags = header_flags();
    let tuple_record = tuple_record(module);
    let glyph_variation_data_table = glyph_variation_data(module, tuple_record);

    // NOTE - can only appear in font files with fvar and glyf tables also present
    module.define_format(
        "opentype.gvar.table",
        let_view(
            "table_view",
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                ("major_version", util::expect_u16be(1)),
                ("minor_version", util::expect_u16be(0)),
                ("axis_count", u16be()),
                ("shared_tuple_count", u16be()),
                (
                    "shared_tuples",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        repeat_count(
                            var("shared_tuple_count"),
                            tuple_record.invoke_args([var("axis_count")]),
                        ),
                    ),
                ),
                ("glyph_count", u16be()),
                ("flags", gvar_flags),
                ("glyph_variation_data_array_offset", u32be()),
                (
                    "glyph_variation_data_offsets",
                    offsets_array(
                        record_proj(var("flags"), "is_long_offset"),
                        var("glyph_count"),
                    ),
                ),
                (
                    "#_glyph_variation_data_array",
                    // FIXME[epic=eager-view-parse] - this is more eager than we actually want
                    phantom(parse_from_view(
                        vvar("table_view").offset(var("glyph_variation_data_array_offset")),
                        data_table_array(
                            var("axis_count"),
                            var("glyph_variation_data_offsets"),
                            glyph_variation_data_table,
                        ),
                    )),
                ),
            ]),
        ),
    )
}

/// GlyphVariationData table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#the-glyphvariationdata-table-array
fn glyph_variation_data(
    module: &mut FormatModule,
    tuple_record: DepFormat<1, 0>,
) -> DepFormat<1, 0> {
    use BitFieldKind::*;
    let tuple_variation_header = tuple_variation_header(module, tuple_record);
    let packed_point_numbers = packed_point_numbers(module);
    let serialized_data = serialized_data(module, packed_point_numbers, tuple_variation_header);

    let tuple_variation_count = bit_fields_u16([
        FlagBit("shared_point_numbers"),
        Reserved {
            bit_width: 3,
            check_zero: SHOULD_CHECK_ZERO,
        },
        BitsField {
            bit_width: 12,
            field_name: "tuple_count",
        },
    ]);

    module.register_format_args(
        "opentype.gvar.glyph_variation_data",
        [(Label::Borrowed("axis_count"), ValueType::U16)],
        let_view(
            "data_view",
            record_auto([
                ("data_scope", reify_view(vvar("data_view"))),
                ("tuple_variation_count", tuple_variation_count),
                ("data_offset", where_nonzero::<U16>(u16be())),
                (
                    "tuple_variation_headers",
                    repeat_count(
                        record_proj(var("tuple_variation_count"), "tuple_count"),
                        tuple_variation_header.invoke_args([var("axis_count")]),
                    ),
                ),
                (
                    "#_data",
                    phantom(parse_from_view(
                        vvar("data_view").offset(var("data_offset")),
                        serialized_data.call_args(vec![
                            record_proj(var("tuple_variation_count"), "shared_point_numbers"),
                            var("tuple_variation_headers"),
                        ]),
                    )),
                ),
            ]),
        ),
    )
}

/// GVAR-specific Serialiezd Data section
///
/// Defined as a dep-format that takes two argumnts
///  - `shared_point_numbers :~ Bool`
///  - `tuple_var_headers :~ Seq(TypeOf(TupleVarHeader))``
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#serialized-data
fn serialized_data(
    module: &mut FormatModule,
    packed_point_numbers: FormatRef,
    tuple_variation_header: DepFormat<1, 0>,
) -> FormatRef {
    let header_type = module
        .get_format_type(tuple_variation_header.get_level())
        .clone();
    module.define_format_args(
        "opentype.gvar.serialized-data",
        vec![
            (Label::Borrowed("shared_point_numbers"), ValueType::BOOL),
            (
                Label::Borrowed("tuple_var_headers"),
                ValueType::Seq(Box::new(header_type)),
            ),
        ],
        record([
            (
                "shared_point_numbers",
                cond_maybe(var("shared_point_numbers"), packed_point_numbers.call()),
            ),
            (
                "per_tuple_variation_data",
                for_each(
                    var("tuple_var_headers"),
                    "header",
                    slice(
                        record_proj(var("header"), "variation_data_size"),
                        record([
                            (
                                "private_point_numbers",
                                cond_maybe(
                                    record_lens(
                                        var("header"),
                                        &["tuple_index", "private_point_numbers"],
                                    ),
                                    packed_point_numbers.call(),
                                ),
                            ),
                            (
                                "x_and_y_coordinate_deltas",
                                Format::Let(
                                    Label::Borrowed("point_count"),
                                    Box::new(tuple_proj(
                                        expr_option_unwrap_first(
                                            var("private_point_numbers"),
                                            var("shared_point_numbers"),
                                        ),
                                        0,
                                    )),
                                    Box::new(packed_deltas(mul(var("point_count"), Expr::U16(2)))),
                                ),
                            ),
                        ]),
                    ),
                ),
            ),
        ]),
    )
}

/// Given two U8-kinded `Expr`s, `hi` and `lo`, computes a 16-bit value whose high byte
/// is `hi` with its MSB zeroed out, and whose low byte is `lo`.
fn u15be(hi: Expr, lo: Expr) -> Expr {
    bit_or(
        shl(bit_and(as_u16(hi), Expr::U16(0x7f)), Expr::U16(8)),
        as_u16(lo),
    )
}

/// Individual run of point-number data, consisting of a control-byte that dictates the width of a point-number
/// (u8 or u16) and how many such values are stored, followed by an array of the indicated length and element-width.
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#packed-point-numbers
fn point_number_run(module: &mut FormatModule) -> FormatRef {
    use BitFieldKind::*;
    let control_byte = bit_fields_u8([
        FlagBit("points_are_words"), // If set, each point is a u16-based delta; u8 otherwise
        BitsField {
            bit_width: 7,
            field_name: "point_run_count",
        }, // 7-bit run-length
    ]);
    module.define_format(
        "opentype.var.packed-point-numbers.run",
        record([
            ("control", control_byte),
            (
                "points",
                Format::Let(
                    // REVIEW - should this be a synthetic field of the record, to make AccumUntil loop easier to specify?
                    Label::Borrowed("run_length"),
                    // Value stored in low 7 bits of control-byte is one less than the actual run-length
                    Box::new(succ(record_proj(var("control"), "point_run_count"))),
                    Box::new(if_then_else(
                        record_proj(var("control"), "points_are_words"),
                        fmt_variant("Points16", repeat_count(var("run_length"), u16be())),
                        fmt_variant("Points8", repeat_count(var("run_length"), u8())),
                    )),
                ),
            ),
        ]),
    )
}

/// Dependent format (taking U16-kinded argument `point_count`) that accumulates [`point_number_run`] parses
/// until a total of `point_count` point-numbers have been parsed.
///
/// # Notes
///
/// If the `point_count` is only satisfied after reading a run that contains more than enough point-numbers
/// proceeds no differently than if the exact count of point-number values were read.
fn point_number_runs(module: &mut FormatModule) -> DepFormat<1, 0> {
    let run = point_number_run(module);
    let update_totlen = lambda_tuple(
        ["acc", "run"],
        add(
            var("acc"),
            // NOTE - the value stored in the control-byte is 1 less than the actual number of points in the run
            succ(as_u16(record_lens(
                var("run"),
                &["control", "point_run_count"],
            ))),
        ),
    );
    module.register_format_args(
        "opentype.var.packed-point-numbers.runs",
        [(Label::Borrowed("point_count"), ValueType::U16)],
        accum_until(
            lambda_tuple(
                ["totlen", "_seq"],
                expr_gte(var("totlen"), var("point_count")),
            ),
            update_totlen,
            Expr::U16(0),
            ValueType::U16,
            run.call(),
        ),
    )
}

/// Packed "point" numbers format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#packed-point-numbers
fn packed_point_numbers(module: &mut FormatModule) -> FormatRef {
    let runs = point_number_runs(module);
    // Variable-precision count-field that is one-byte if it fits in 7 bits, or 15-bit if it doesn't (U16Be ignoring MSB in first byte read)
    module.define_format(
        "opentype.var.packed-point-numbers",
        union([
            map(
                is_byte(0),
                lambda("_", Expr::Tuple(vec![Expr::U16(0), seq_empty()])),
            ),
            chain(
                byte_in(1..=127),
                "point_count",
                runs.invoke_args([as_u16(var("point_count"))]),
            ),
            chain(
                byte_in(128..=255),
                "hi",
                chain(u8(), "lo", runs.invoke_args([u15be(var("hi"), var("lo"))])),
            ),
        ]),
    )
}

/// GVAR TupleVariationHeader format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#tuplevariationheader
///
/// Parametric over `axis_count :~ U16`.
fn tuple_variation_header(
    module: &mut FormatModule,
    tuple_record: DepFormat<1, 0>,
) -> DepFormat<1, 0> {
    use BitFieldKind::*;
    const SHOULD_CHECK_ZERO: bool = false;
    let tuple_index = bit_fields_u16([
        FlagBit("embedded_peak_tuple"), // if set, includes an embedded peak tuple record, immediately after tupleIndex, and that the low 12 bits (field `tuple_index`) are to be ignored
        FlagBit("intermediate_region"), // if set, header includes a start- and end-tuple-record (2 tuple records total) immediately after peak-tuple-record logical position (whether present or not)
        FlagBit("private_point_numbers"), // if set, serialized data includes packed "point" number data; when not set, the shared number data at the start of serialized data is used by default
        Reserved {
            bit_width: 1,
            check_zero: SHOULD_CHECK_ZERO,
        },
        BitsField {
            bit_width: 12,
            field_name: "tuple_index",
        },
    ]);
    module.register_format_args(
        "opentype.gvar.tuple_variation_header",
        [(Label::Borrowed("axis_count"), ValueType::U16)],
        record([
            ("variation_data_size", u16be()), // size, in bytes, of serialized data for this tuple variation table
            ("tuple_index", tuple_index),
            (
                "peak_tuple",
                cond_maybe(
                    record_proj(var("tuple_index"), "embedded_peak_tuple"),
                    tuple_record.invoke_args([var("axis_count")]),
                ),
            ),
            (
                "intermediate_tuples",
                cond_maybe(
                    record_proj(var("tuple_index"), "intermediate_region"),
                    record_repeat(
                        ["start_tuple", "end_tuple"],
                        tuple_record.invoke_args([var("axis_count")]),
                    ),
                ),
            ),
        ]),
    )
}

/// Definition for Tuple Records used in variation tables
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#tuple-records
///
/// Parametric over `axis_count :~ U16`.
// TODO - change namespace from `gvar` to `var`, move to common submodule for multi-table sub-formats
fn tuple_record(module: &mut FormatModule) -> DepFormat<1, 0> {
    module.register_format_args(
        "opentype.gvar.tuple_record",
        [(Label::Borrowed("axis_count"), ValueType::U16)],
        record([(
            "coordinates",
            repeat_count(var("axis_count"), util::f2dot14()),
        )]),
    )
}
