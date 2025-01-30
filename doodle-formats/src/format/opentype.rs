use crate::format::BaseModule;
use doodle::bounds::Bounds;
use doodle::{helper::*, Expr, IntoLabel, Label};
use doodle::{BaseType, Format, FormatModule, FormatRef, Pattern, ValueType};

fn shadow_check(x: &Expr, name: &'static str) {
    if x.is_shadowed_by(name) {
        panic!("Shadow! Variable-name {name} already occurs in Expr {x:?}!");
    }
}

fn last_elem(seq_var: &'static str) -> Expr {
    let last_ix = sub(seq_length(var(seq_var)), Expr::U32(1));
    index_unchecked(var(seq_var), last_ix)
}

fn loca_offset_pairs(loca: Expr) -> Expr {
    let f = |loca_table: Expr| {
        flat_map_accum(
            lambda_tuple(
                ["last_value", "value"],
                pair(
                    expr_some(var("value")),
                    expr_option_map_or(
                        seq_empty(),
                        |last| singleton(pair(last, var("value"))),
                        var("last_value"),
                    ),
                ),
            ),
            expr_none(),
            ValueType::Option(Box::new(ValueType::Base(BaseType::U32))),
            expr_match(
                record_proj(loca_table, "offsets"),
                [
                    (
                        Pattern::Variant(Label::Borrowed("Offsets16"), Box::new(bind("half16s"))),
                        flat_map(
                            lambda(
                                "half16",
                                singleton(mul(Expr::AsU32(Box::new(var("half16"))), Expr::U32(2))),
                            ),
                            var("half16s"),
                        ),
                    ),
                    (
                        Pattern::Variant(Label::Borrowed("Offsets32"), Box::new(bind("off32s"))),
                        var("off32s"),
                    ),
                ],
            ),
        )
    };
    expr_option_map_or(seq_empty(), f, loca)
}

/// Converts a `u8` value to an `i16` value within the `Expr` model
/// according to a bit-flag for signedness `pos_bit` (`1` for positive, `0` for negative).
// FIXME - this currently uses the two's-complement u16 value that maps to the proper i16 value
fn u8_to_i16(x: Expr, is_positive: Expr) -> Expr {
    expr_if_else(
        is_positive,
        Expr::AsU16(Box::new(x.clone())),
        expr_match(
            x,
            [
                (Pattern::U8(0), Expr::U16(0)),
                (
                    bind("n"),
                    sub(
                        Expr::U16(u16::MAX),
                        sub(Expr::AsU16(Box::new(var("n"))), Expr::U16(1)),
                    ),
                ),
            ],
        ),
    )
}

fn pos_add_u16(pos32: Expr, offset16: Expr) -> Expr {
    add(pos32, Expr::AsU32(Box::new(offset16)))
}

/// Gets the current stream-position and casts down from U64->U32
fn pos32() -> Format {
    map(Format::Pos, lambda("x", Expr::AsU32(Box::new(var("x")))))
}

/// Parses a u32 serving as the de-facto representation of a signed, 16.16 bit fixed-point number
fn fixed32be(base: &BaseModule) -> Format {
    map(base.u32be(), lambda("x", variant("Fixed32", var("x"))))
}

// Custom type for fixed-point values that are interpreted as (2sigbits, 14sigbits) within a u16be raw-parse
fn f2dot14(base: &BaseModule) -> Format {
    map(base.u16be(), lambda("x", variant("F2Dot14", var("x"))))
}

/// FIXME - scaffolding to signal intent to use i8 format before it is implemented
fn s8(base: &BaseModule) -> Format {
    base.u8()
}

/// FIXME - scaffolding to signal intent to use i16 format before it is implemented
fn s16be(base: &BaseModule) -> Format {
    base.u16be()
}

/// FIXME - scaffolding to signal intent to use i64 format before it is implemented
fn s64be(base: &BaseModule) -> Format {
    base.u64be()
}

fn u24be(base: &BaseModule) -> Format {
    map(
        Format::Tuple(vec![compute(Expr::U8(0)), base.u8(), base.u8(), base.u8()]),
        lambda("x", Expr::U32Be(Box::new(var("x")))),
    )
}

// Placeholder for a `(u16, u16)` value-pair packed as a big-endian u32
fn version16_16(base: &BaseModule) -> Format {
    base.u32be()
}

// helper to turn b"..." literals into u32 at compile-time
const fn magic(tag: &'static [u8; 4]) -> u32 {
    u32::from_be_bytes(*tag)
}

/// Parses a `U16Be` value that is expected to be equal to `val`
fn expect_u16be(base: &BaseModule, val: u16) -> Format {
    where_lambda(base.u16be(), "x", expr_eq(var("x"), Expr::U16(val)))
}

fn slice_record<Name, const N: usize>(
    length_field: &'static str,
    fields: [(Name, Format); N],
) -> Format
where
    Name: IntoLabel + Clone + AsRef<str>,
{
    let mut prefix = Vec::new();
    let mut full = Vec::with_capacity(fields.len());

    let mut prefix_done = false;

    for (name, format) in fields.into_iter() {
        if !prefix_done {
            prefix.push((name.clone(), format.clone()));
            if name.as_ref() == length_field {
                prefix_done = true;
            }
        }
        full.push((name, format));
    }

    chain(
        Format::Peek(Box::new(record(prefix))),
        "x",
        Format::Slice(
            Box::new(record_proj(var("x"), length_field)),
            Box::new(record(full)),
        ),
    )
}

// Computes the maximum value of `x / 8` for `x: U16` in seq (return value wrapped in Option)
fn subheader_index(seq: Expr) -> Expr {
    // REVIEW - because of how narrow the use-case is, we might be able to use 0 as the init-accum value and avoid Option entirely
    expr_unwrap(left_fold(
        lambda_tuple(
            ["acc", "y"],
            expr_match(
                var("acc"),
                [
                    (
                        pat_some(bind("x")),
                        expr_some(expr_max(var("x"), div(var("y"), Expr::U16(8)))),
                    ),
                    (pat_none(), expr_some(div(var("y"), Expr::U16(8)))),
                ],
            ),
        ),
        expr_none(),
        ValueType::Option(Box::new(ValueType::Base(BaseType::U16))),
        seq,
    ))
}

const START_VAR: Expr = Expr::Var(Label::Borrowed("start"));
const START_ARG: (Label, ValueType) = (Label::Borrowed("start"), ValueType::Base(BaseType::U32));

// FIXME - this is a crude approximation that could be improved with the right primitives in-hand
fn find_table(table_records: Expr, query_table_id: u32) -> Expr {
    // TODO: accelerate using binary search
    // TODO: make use of `search_range` etc.
    let matches_query = |table_record: Expr| {
        expr_eq(
            record_proj(table_record, "table_id"),
            Expr::U32(query_table_id),
        )
    };
    let matching_tables = flat_map(
        lambda(
            "table",
            expr_if_else(
                matches_query(var("table")),
                singleton(var("table")),
                seq_empty(),
            ),
        ),
        table_records,
    );
    // FIXME - we need either an Expr::Let or Expr::ListToOption primitive to distinguish no-match, 1-match, and multi-match
    index_checked(matching_tables, Expr::U32(0))
}

fn link_offset(sof_offset: Expr, table_offset: Expr, format: Format) -> Format {
    let rel_offset = |sof_offset: Expr, here_offset: Expr, target_offset: Expr| -> Expr {
        /*
         * Here := here_offset (abs)
         * SOF  := sof_offset (abs)
         * Tgt  := target_offset (rel)
         *
         * We want to return X such that
         *   Here (abs) + X (rel) == SOF (abs) + Tgt (rel)
         * so we compute
         *   X (rel) := SOF + Tgt - Here
         */
        relativize_offset(add(sof_offset, target_offset), here_offset)
    };
    chain(
        pos32(),
        "offset",
        Format::WithRelativeOffset(
            Box::new(rel_offset(sof_offset, var("offset"), table_offset)),
            Box::new(format),
        ),
    )
}

/// Converts an absolute offset (or, relative to SOF) into a relative offset
/// given the absolute file-offset we are currently parsing from.
fn relativize_offset(abs_offset: Expr, here: Expr) -> Expr {
    sub(abs_offset, here)
}

/// Parse a format at a given (absolute) file offset
fn link(abs_offset: Expr, format: Format) -> Format {
    chain(
        pos32(),
        "__here",
        Format::WithRelativeOffset(
            Box::new(relativize_offset(abs_offset, var("__here"))),
            Box::new(format),
        ),
    )
}

// FIXME - should we use `chain` instead of `record` to elide the offset and flatten the link?
fn offset16(base_offset: Expr, format: Format, base: &BaseModule) -> Format {
    shadow_check(&base_offset, "offset");
    record([
        ("offset", base.u16be()),
        (
            "link",
            if_then_else(
                is_nonzero_u16(var("offset")),
                link(pos_add_u16(base_offset, var("offset")), format_some(format)),
                format_none(),
            ),
        ),
    ])
}

// FIXME - should we use `chain` instead of `record` to elide the offset and flatten the link?
fn offset32(base_offset: Expr, format: Format, base: &BaseModule) -> Format {
    shadow_check(&base_offset, "offset");
    record([
        ("offset", base.u32be()),
        (
            "link",
            cond_maybe(
                is_nonzero_u32(var("offset")),
                linked_offset32(base_offset, var("offset"), format),
            ),
        ),
    ])
}

fn linked_offset32(base_offset: Expr, rel_offset: Expr, format: Format) -> Format {
    link(add(base_offset, rel_offset), format)
}

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    // NOTE - Microsoft defines a tag as consisting on printable ascii characters in the range 0x20 -- 0x7E (inclusive), but some vendors are non-standard so we accept anything
    let tag = module.define_format("opentype.types.tag", base.u32be());

    const SHORT_OFFSET16: u16 = 0;
    const LONG_OFFSET32: u16 = 1;

    let table_record = module.define_format_args(
        "opentype.table_record",
        vec![START_ARG],
        record([
            ("table_id", tag.call()), // should be ascending within the repetition "table_records" field in table_directory
            ("checksum", base.u32be()),
            ("offset", base.u32be()),
            ("length", base.u32be()),
        ]),
    );

    let table_type = module.get_format_type(table_record.get_level()).clone();

    let table_links = {
        fn required_table(
            sof_offset32_vname: &'static str,
            table_records_var: &'static str,
            id: u32,
            table_format: Format,
        ) -> Format {
            Format::Let(
                Label::Borrowed("matching_table"),
                Box::new(expr_unwrap(find_table(var(table_records_var), id))),
                Box::new(link_offset(
                    var(sof_offset32_vname),
                    record_proj(var("matching_table"), "offset"),
                    Format::Slice(
                        Box::new(record_proj(var("matching_table"), "length")),
                        Box::new(table_format),
                    ),
                )),
            )
        }

        fn required_table_with_len(
            sof_offset_var: &'static str,
            table_records_var: &'static str,
            id: u32,
            table_format_ref: FormatRef,
        ) -> Format {
            Format::Let(
                Label::Borrowed("matching_table"),
                Box::new(expr_unwrap(find_table(var(table_records_var), id))),
                Box::new(link_offset(
                    var(sof_offset_var),
                    record_proj(var("matching_table"), "offset"),
                    Format::Slice(
                        Box::new(record_proj(var("matching_table"), "length")),
                        Box::new(
                            table_format_ref
                                .call_args(vec![record_proj(var("matching_table"), "length")]),
                        ),
                    ),
                )),
            )
        }

        fn optional_table(
            sof_offset_var: &'static str,
            table_records_var: &'static str,
            id: u32,
            table_format: Format,
        ) -> Format {
            Format::Let(
                Label::Borrowed("matching_table"),
                Box::new(find_table(var(table_records_var), id)),
                Box::new(Format::Match(
                    Box::new(var("matching_table")),
                    vec![
                        (
                            pat_some(bind("table")),
                            format_some(link_offset(
                                var(sof_offset_var),
                                record_proj(var("table"), "offset"),
                                Format::Slice(
                                    Box::new(record_proj(var("table"), "length")),
                                    Box::new(table_format),
                                ),
                            )),
                        ),
                        (pat_none(), format_none()),
                    ],
                )),
            )
        }

        let encoding_id = |_platform_id: Expr| base.u16be();

        // # Language identifiers
        //
        // This must be set to `0` for all subtables that have a platform ID other than
        // ‘Macintosh’.
        //
        // ## References
        //
        // - [Microsoft's OpenType Spec: Use of the language field in 'cmap' subtables](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#use-of-the-language-field-in-cmap-subtables)
        // - [Apple's TrueType Reference Manual: The `'cmap'` table and language codes](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
        //
        // TODO: add more details to docs
        let language_id = || base.u16be();

        // character mapping table
        let cmap_table = {
            let cmap_language_id = |_platform: Expr| language_id();
            let cmap_language_id32 = |_platform: Expr| base.u32be();

            let small_glyph_id = base.u8();

            // Format 0 : Byte encoding table
            let cmap_subtable_format0 = module.define_format_args(
                "opentype.cmap_subtable.format0",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", base.u16be()), // == 0
                        ("length", base.u16be()),
                        ("language", cmap_language_id(var("_platform"))),
                        (
                            "glyph_id_array",
                            repeat_count(Expr::U16(256), small_glyph_id),
                        ),
                    ],
                ),
            );

            // FIXME - this is actually a signed 16-bit value but we don't support that; it can be unsigned as long as we do the right wrapping addition

            let subheader = record([
                ("first_code", base.u16be()),
                ("entry_count", base.u16be()),
                ("id_delta", s16be(base)),
                ("id_range_offset", base.u16be()),
            ]);

            // Format 2: High-byte mapping through table
            let cmap_subtable_format2 = module.define_format_args(
                "opentype.cmap_subtable.format2",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 2)),
                        (
                            "length",
                            where_lambda(
                                base.u16be(),
                                "l",
                                and(
                                    // NOTE - strictly speaking we don't expect length == 518 exactly, but this is a rough check
                                    expr_gte(var("l"), Expr::U16(518)),
                                    // NOTE - all fields are entirely comprised of 16-bit tokens, so overall length must be a multiple of 2
                                    expr_eq(rem(var("l"), Expr::U16(2)), Expr::U16(0)),
                                ),
                            ),
                        ),
                        ("language", cmap_language_id(var("_platform"))),
                        (
                            "sub_header_keys",
                            repeat_count(Expr::U16(256), base.u16be()),
                        ),
                        (
                            "sub_headers",
                            repeat_count(
                                add(Expr::U16(1), subheader_index(var("sub_header_keys"))),
                                subheader,
                            ),
                        ),
                        ("glyph_array", repeat(base.u16be())),
                    ],
                ),
            );

            // # Format 4: Segment mapping to delta values
            let cmap_subtable_format4 = module.define_format_args(
                "opentype.cmap_subtable.format4",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 4)),
                        ("length", base.u16be()),
                        ("language", cmap_language_id(var("_platform"))),
                        (
                            "seg_count",
                            map(
                                base.u16be(),
                                lambda("seg_count_x2", div(var("seg_count_x2"), Expr::U16(2))),
                            ),
                        ),
                        ("search_range", base.u16be()), // := 2x the maximum power of 2 <= seg_count
                        ("entry_selector", base.u16be()), // := ilog2(seg_count)
                        ("range_shift", base.u16be()),  // := seg_count * 2 - search_range
                        ("end_code", repeat_count(var("seg_count"), base.u16be())), // end charcode for each seg, last is 0xFFFF
                        ("__reserved_pad", expect_u16be(base, 0)),
                        ("start_code", repeat_count(var("seg_count"), base.u16be())),
                        ("id_delta", repeat_count(var("seg_count"), base.u16be())), // ought to be signed but will work if we perform as unsigned addition mod-0xFFFF
                        (
                            "id_range_offset",
                            repeat_count(var("seg_count"), base.u16be()),
                        ), // offsets into glyphIdArray or 0
                        ("glyph_array", repeat(base.u16be())),
                    ],
                ),
            );

            let cmap_subtable_format6 = module.define_format_args(
                "opentype.cmap_subtable.format6",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 6)),
                        ("length", base.u16be()),
                        ("language", cmap_language_id(var("_platform"))),
                        ("first_code", base.u16be()),
                        ("entry_count", base.u16be()),
                        (
                            "glyph_id_array",
                            repeat_count(var("entry_count"), base.u16be()),
                        ),
                    ],
                ),
            );

            let sequential_map_group = module.define_format(
                "opentype.types.sequential_map_record",
                record([
                    ("start_char_code", base.u32be()),
                    ("end_char_code", base.u32be()),
                    ("start_glyph_id", base.u32be()),
                ]),
            );

            let cmap_subtable_format8 = module.define_format_args(
                "opentype.cmap_subtable.format8",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 8)),
                        ("__reserved", expect_u16be(base, 0)),
                        ("length", base.u32be()),
                        ("language", cmap_language_id32(var("_platform"))),
                        // REVIEW - should this be 8x as long and consist of bits?
                        ("is32", repeat_count(Expr::U16(8192), base.u8())), // packed bit-array where a bit at index `i` signals whether the 16-bit value index `i` is the start of a 32-bit character code
                        ("num_groups", base.u32be()),
                        (
                            "groups",
                            repeat_count(var("num_groups"), sequential_map_group.call()),
                        ),
                    ],
                ),
            );

            let cmap_subtable_format10 = module.define_format_args(
                "opentype.cmap_subtable.format10",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 10)),
                        ("__reserved", expect_u16be(base, 0)),
                        ("length", base.u32be()),
                        ("language", cmap_language_id32(var("_platform"))),
                        ("start_char_code", base.u32be()),
                        ("num_chars", base.u32be()),
                        (
                            "glyph_id_array",
                            repeat_count(var("num_chars"), base.u16be()),
                        ),
                    ],
                ),
            );

            let cmap_subtable_format12 = module.define_format_args(
                "opentype.cmap_subtable.format12",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 12)),
                        ("__reserved", expect_u16be(base, 0)),
                        ("length", base.u32be()),
                        ("language", cmap_language_id32(var("_platform"))),
                        ("num_groups", base.u32be()),
                        (
                            "groups",
                            repeat_count(var("num_groups"), sequential_map_group.call()),
                        ),
                    ],
                ),
            );

            let constant_map_group = sequential_map_group.call();

            let cmap_subtable_format13 = module.define_format_args(
                "opentype.cmap_subtable.format13",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 13)),
                        ("__reserved", expect_u16be(base, 0)),
                        ("length", base.u32be()),
                        ("language", cmap_language_id32(var("_platform"))),
                        ("num_groups", base.u32be()),
                        (
                            "groups",
                            repeat_count(var("num_groups"), constant_map_group),
                        ),
                    ],
                ),
            );

            let unicode_range = record([
                ("start_unicode_value", u24be(base)),
                ("additional_count", base.u8()),
            ]);

            let uvs_mapping = record([("unicode_value", u24be(base)), ("glyph_id", base.u16be())]);

            let default_uvs_table = record([
                ("num_unicode_value_ranges", base.u32be()),
                (
                    "ranges",
                    repeat_count(var("num_unicode_value_ranges"), unicode_range),
                ),
            ]);

            let non_default_uvs_table = record([
                ("num_uvs_mappings", base.u32be()),
                (
                    "uvs_mappings",
                    repeat_count(var("num_uvs_mappings"), uvs_mapping),
                ),
            ]);

            let variation_selector = module.define_format_args(
                "opentype.variation_selector",
                vec![(
                    Label::Borrowed("table_start"),
                    ValueType::Base(BaseType::U32),
                )],
                record([
                    ("var_selector", u24be(base)),
                    (
                        "default_uvs_offset",
                        offset32(var("table_start"), default_uvs_table, base),
                    ),
                    (
                        "non_default_uvs_offset",
                        offset32(var("table_start"), non_default_uvs_table, base),
                    ),
                ]),
            );

            let cmap_subtable_format14 = module.define_format_args(
                "opentype.cmap_subtable.format14",
                vec![(
                    Label::Borrowed("table_start"),
                    ValueType::Base(BaseType::U32),
                )],
                slice_record(
                    "length",
                    [
                        ("format", expect_u16be(base, 14)),
                        ("length", base.u32be()),
                        ("num_var_selector_records", base.u32be()),
                        (
                            "var_selector",
                            repeat_count(
                                var("num_var_selector_records"),
                                variation_selector.call_args(vec![var("table_start")]),
                            ),
                        ),
                    ],
                ),
            );

            let cmap_subtable = module.define_format_args(
                "opentype.cmap_subtable",
                vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
                record([
                    ("table_start", pos32()),
                    ("format", Format::Peek(Box::new(base.u16be()))),
                    (
                        "data",
                        match_variant(
                            var("format"),
                            [
                                (
                                    Pattern::U16(0),
                                    "Format0",
                                    cmap_subtable_format0.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(2),
                                    "Format2",
                                    cmap_subtable_format2.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(4),
                                    "Format4",
                                    cmap_subtable_format4.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(6),
                                    "Format6",
                                    cmap_subtable_format6.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(8),
                                    "Format8",
                                    cmap_subtable_format8.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(10),
                                    "Format10",
                                    cmap_subtable_format10.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(12),
                                    "Format12",
                                    cmap_subtable_format12.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(13),
                                    "Format13",
                                    cmap_subtable_format13.call_args(vec![var("_platform")]),
                                ),
                                (
                                    Pattern::U16(14),
                                    "Format14",
                                    cmap_subtable_format14.call_args(vec![var("table_start")]),
                                ),
                                // FIXME - leaving out unknown-table for now
                            ],
                        ),
                    ),
                ]),
            );

            let encoding_record = module.define_format_args(
                "opentype.encoding_record",
                vec![START_ARG],
                record([
                    ("platform", base.u16be()), // platform identifier
                    // NOTE - encoding_id nominally depends on platform_id but no recorded dependencies in fathom def
                    ("encoding", encoding_id(var("platform"))), // encoding identifier
                    (
                        "subtable_offset",
                        offset32(
                            START_VAR,
                            cmap_subtable.call_args(vec![var("platform")]),
                            base,
                        ),
                    ),
                ]),
            );

            module.define_format(
                "opentype.cmap_table",
                record([
                    ("table_start", pos32()),     // start of character mapping table
                    ("version", base.u16be()),    // version of the the character
                    ("num_tables", base.u16be()), // number of subsequent encoding tables
                    (
                        "encoding_records",
                        repeat_count(
                            var("num_tables"),
                            encoding_record.call_args(vec![var("table_start")]),
                        ),
                    ),
                ]),
            )
        };

        let head_table = {
            // FIXME - replace with packed_bits_u16 of fields if appropriate
            let head_table_flags = base.u16be();

            let long_date_time = module.define_format("opentype.types.long_date_time", s64be(base));

            let xy_min_max = record_repeat(["x_min", "y_min", "x_max", "y_max"], s16be(base));

            let head_table_style_flags = flags_bits16([
                None, // Bit 15 (MSB)
                None, // Bit 14
                None, // Bit 13
                None, // Bit 12
                None, // Bit 11
                None, // Bit 10
                None, // Bit 9
                None, // Bit 8
                None, // Bit 7
                Some("extended"),
                Some("condensed"),
                Some("shadow"),
                Some("outline"),
                Some("underline"),
                Some("italic"),
                Some("bold"),
            ]);

            // NOTE - Should be 2 for modern fonts but we shouldn't enforce that too strongly
            /* ConstEnum(s16be) {
             *     Mixed    =  0,
             *     StrongLR =  1,
             *     WeakLR   =  2,
             *     StrongRL = -1,
             *     WeakRL   = -2,
             * }
             */
            let glyph_dir_hint = s16be(base);

            module.define_format(
                "opentype.head_table",
                record([
                    ("major_version", expect_u16be(base, 1)),
                    ("minor_version", expect_u16be(base, 0)),
                    ("font_revision", fixed32be(base)),
                    ("checksum_adjustment", base.u32be()),
                    ("magic_number", is_bytes(&[0x5F, 0x0F, 0x3C, 0xF5])),
                    ("flags", head_table_flags),
                    (
                        "units_per_em",
                        where_between(base.u16be(), Expr::U16(16), Expr::U16(16384)),
                    ),
                    ("created", long_date_time.call()),
                    ("modified", long_date_time.call()),
                    ("glyph_extents", xy_min_max),
                    ("mac_style", head_table_style_flags),
                    ("lowest_rec_ppem", base.u16be()),
                    ("font_direction_hint", glyph_dir_hint),
                    (
                        "index_to_loc_format",
                        where_between(
                            base.u16be(),
                            Expr::U16(SHORT_OFFSET16),
                            Expr::U16(LONG_OFFSET32),
                        ),
                    ),
                    ("glyph_data_format", expect_u16be(base, 0)),
                ]),
            )
        };

        let hhea_table = {
            module.define_format(
                "opentype.hhea_table",
                record([
                    ("major_version", expect_u16be(base, 1)),
                    ("minor_version", expect_u16be(base, 0)),
                    ("ascent", s16be(base)), // distance from baseline to highest ascender, in font design units
                    ("descent", s16be(base)), // distance from baseline to lowest descender, in font design units
                    ("line_gap", s16be(base)), // intended gap between baselines, in font design units
                    ("advance_width_max", base.u16be()), // must be consistent with horizontal metrics
                    ("min_left_side_bearing", s16be(base)), // must be consistent with horizontal metrics
                    ("min_right_side_bearing", s16be(base)), // must be consistent with horizontal metrics
                    ("x_max_extent", s16be(base)), // `max(left_side_bearing + (x_max - x_min))`
                    // slope of the caret (rise/run), (1/0) for vertical caret
                    ("caret_slope", record_repeat(["rise", "run"], s16be(base))),
                    ("caret_offset", s16be(base)), // 0 for non-slanted fonts
                    ("__reservedX4", tuple_repeat(4, expect_u16be(base, 0))), // NOTE: 4 separate isolated fields in fathom
                    ("metric_data_format", expect_u16be(base, 0)),
                    // number of `long_horizontal_metric` records in the `htmx_table`
                    ("number_of_long_horizontal_metrics", base.u16be()),
                ]),
            )
        };

        let maxp_table = {
            const NO_Z0: u16 = 1;
            const YES_Z0: u16 = 2;

            let maxp_version_1 = module.define_format(
                "opentype.maxp_table.version1",
                record([
                    ("max_points", base.u16be()),
                    ("max_contours", base.u16be()),
                    ("max_composite_points", base.u16be()),
                    ("max_composite_contours", base.u16be()),
                    (
                        "max_zones",
                        where_between(base.u16be(), Expr::U16(NO_Z0), Expr::U16(YES_Z0)),
                    ),
                    ("max_twilight_points", base.u16be()),
                    ("max_storage", base.u16be()),
                    ("max_function_defs", base.u16be()),
                    ("max_instruction_defs", base.u16be()),
                    ("max_stack_elements", base.u16be()),
                    ("max_size_of_instructions", base.u16be()),
                    ("max_component_elements", base.u16be()),
                    (
                        "max_component_depth",
                        where_between(base.u16be(), Expr::U16(0), Expr::U16(16)),
                    ),
                ]),
            );

            module.define_format(
                "opentype.maxp_table",
                record([
                    ("version", version16_16(base)),
                    ("num_glyphs", base.u16be()),
                    (
                        "data",
                        match_variant(
                            var("version"),
                            [
                                (Pattern::U32(0x0001_0000), "MaxpV1", maxp_version_1.call()),
                                (Pattern::U32(0x0000_5000), "MaxpPostScript", Format::EMPTY),
                                (bind("unknown"), "MaxpUnknown", compute(var("unknown"))), // FIXME - do we need this at all?
                            ],
                        ),
                    ),
                ]),
            )
        };

        let hmtx_table = {
            let long_horizontal_metric = record([
                ("advance_width", base.u16be()),
                ("left_side_bearing", s16be(base)),
            ]);

            module.define_format_args(
                "opentype.hmtx_table",
                vec![
                    (
                        Label::Borrowed("num_h_metrics"),
                        ValueType::Base(BaseType::U16),
                    ),
                    (
                        Label::Borrowed("num_glyphs"),
                        ValueType::Base(BaseType::U16),
                    ),
                ],
                record([
                    (
                        "h_metrics",
                        repeat_count(var("num_h_metrics"), long_horizontal_metric),
                    ),
                    (
                        "left_side_bearings",
                        repeat_count(sub(var("num_glyphs"), var("num_h_metrics")), s16be(base)),
                    ),
                ]),
            )
        };

        let name_table = {
            #[allow(dead_code)]
            let name_id = {
                const NID_COPYRIGHT_NOTICE: u16 = 0;
                const NID_FAMILY_NAME: u16 = 1;
                const NID_SUBFAMILY_NAME: u16 = 2;
                const NID_UNIQUE_FONT_IDENTIFICATION: u16 = 3;
                const NID_FULL_FONT_NAME: u16 = 4;
                const NID_VERSION_STRING: u16 = 5;
                const NID_POSTSCRIPT_NAME: u16 = 6;
                const NID_TRADEMARK: u16 = 7;
                const NID_MANUFACTURER_NAME: u16 = 8;
                const NID_DESIGNER_NAME: u16 = 9;
                const NID_DESCRIPTION: u16 = 10;
                const NID_VENDOR_URL: u16 = 11;
                const NID_DESIGNER_URL: u16 = 12;
                const NID_LICENSE_DESCRIPTION: u16 = 13;
                const NID_LICENSE_INFO_URL: u16 = 14;
                // 15 - reserved
                const NID_TYPOGRAPHIC_FAMILY_NAME: u16 = 16;
                const NID_TYPOGRAPHIC_SUBFAMILY_NAME: u16 = 17;
                const NID_COMPAT_FULL_NAME: u16 = 18;
                const NID_SAMPLE_TEXT: u16 = 19;
                const NID_POSTSCRIPT_FONT_NAME: u16 = 20;
                const NID_WWS_FAMILY_NAME: u16 = 21;
                const NID_WWS_SUBFAMILY_NAME: u16 = 22;
                const NID_LIGHT_BACKGROUND_PALETTE: u16 = 23;
                const NID_DARK_BACKGROUND_PALETTE: u16 = 24;
                const NID_VARIATIONS_POSTSCRIPT_NAME_PREFIX: u16 = 25;
                // 26..=255 - reserved
                // 256..=32767 - font-specific names

                base.u16be()
            };

            let name_record = |storage_start: Expr| -> Format {
                record([
                    ("platform", base.u16be()),
                    ("encoding", encoding_id(var("platform"))),
                    ("language", language_id()),
                    ("name_id", name_id),
                    ("length", base.u16be()),
                    (
                        "offset",
                        offset16(storage_start, repeat_count(var("length"), base.u8()), base),
                    ),
                ])
            };

            let name_version_1 = {
                let lang_tag_record = |storage_start: Expr| -> Format {
                    record([
                        ("length", base.u16be()),
                        (
                            "offset",
                            offset16(storage_start, repeat_count(var("length"), base.u8()), base),
                        ),
                    ])
                };

                module.define_format_args(
                    "opentype.name_table.name_version_1",
                    vec![(
                        Label::Borrowed("storage_start"),
                        ValueType::Base(BaseType::U32),
                    )],
                    record([
                        ("lang_tag_count", base.u16be()),
                        (
                            "lang_tag_records",
                            repeat_count(
                                var("lang_tag_count"),
                                lang_tag_record(var("storage_start")),
                            ),
                        ),
                    ]),
                )
            };

            module.define_format(
                "opentype.name_table",
                record([
                    ("table_start", pos32()),
                    ("version", base.u16be()),
                    ("name_count", base.u16be()),
                    ("storage_offset", base.u16be()),
                    (
                        "name_records",
                        repeat_count(
                            var("name_count"),
                            name_record(pos_add_u16(var("table_start"), var("storage_offset"))),
                        ),
                    ),
                    (
                        "data",
                        match_variant(
                            var("version"),
                            [
                                (Pattern::U16(0), "NameVersion0", Format::EMPTY),
                                (
                                    Pattern::U16(1),
                                    "NameVersion1",
                                    name_version_1.call_args(vec![pos_add_u16(
                                        var("table_start"),
                                        var("storage_offset"),
                                    )]),
                                ),
                                (
                                    Pattern::binding("unknown"),
                                    "NameVersionUnknown",
                                    compute(var("unknown")),
                                ),
                            ],
                        ),
                    ),
                ]),
            )
        };

        let os2_table = {
            let version_record = |version_ident: &'static str, table_length: Expr| -> Format {
                const V0_MIN_LENGTH: u32 = 78;
                cond_maybe(
                    or(
                        is_nonzero_u16(var(version_ident)),
                        expr_gte(table_length, Expr::U32(V0_MIN_LENGTH)),
                    ),
                    record([
                        ("s_typo_ascender", s16be(base)),
                        ("s_typo_descender", s16be(base)),
                        ("s_typo_line_gap", s16be(base)),
                        ("us_win_ascent", base.u16be()),
                        ("us_win_descent", base.u16be()),
                        (
                            "extra_fields_v1",
                            cond_maybe(
                                expr_gte(var(version_ident), Expr::U16(1)),
                                record([
                                    ("ul_code_page_range_1", base.u32be()),
                                    ("ul_code_page_range_2", base.u32be()),
                                    (
                                        "extra_fields_v2",
                                        cond_maybe(
                                            expr_gte(var(version_ident), Expr::U16(2)),
                                            record([
                                                ("sx_height", s16be(base)),
                                                ("s_cap_height", s16be(base)),
                                                ("us_default_char", base.u16be()),
                                                ("us_break_char", base.u16be()),
                                                ("us_max_context", base.u16be()),
                                                (
                                                    "extra_fields_v5",
                                                    cond_maybe(
                                                        expr_gte(var(version_ident), Expr::U16(5)),
                                                        record([
                                                            (
                                                                "us_lower_optical_point_size",
                                                                base.u16be(),
                                                            ),
                                                            (
                                                                "us_upper_optical_point_size",
                                                                base.u16be(),
                                                            ),
                                                        ]),
                                                    ),
                                                ),
                                            ]),
                                        ),
                                    ),
                                ]),
                            ),
                        ),
                    ]),
                )
            };

            module.define_format_args(
                "opentype.os2_table",
                vec![(
                    Label::Borrowed("table_length"),
                    ValueType::Base(BaseType::U32),
                )],
                record([
                    ("version", base.u16be()),
                    ("x_avg_char_width", s16be(base)),
                    ("us_weight_class", base.u16be()),
                    ("us_width_class", base.u16be()),
                    ("fs_type", base.u16be()),
                    ("y_subscript_x_size", s16be(base)),
                    ("y_subscript_y_size", s16be(base)),
                    ("y_subscript_x_offset", s16be(base)),
                    ("y_subscript_y_offset", s16be(base)),
                    ("y_superscript_x_size", s16be(base)),
                    ("y_superscript_y_size", s16be(base)),
                    ("y_superscript_x_offset", s16be(base)),
                    ("y_superscript_y_offset", s16be(base)),
                    ("y_strikeout_size", s16be(base)),
                    ("y_strikeout_position", s16be(base)),
                    ("s_family_class", s16be(base)),
                    ("panose", repeat_count(Expr::U8(10), base.u8())),
                    ("ul_unicode_range1", base.u32be()),
                    ("ul_unicode_range2", base.u32be()),
                    ("ul_unicode_range3", base.u32be()),
                    ("ul_unicode_range4", base.u32be()),
                    ("ach_vend_id", tag.call()),
                    ("fs_selection", base.u16be()),
                    ("us_first_char_index", base.u16be()),
                    ("us_last_char_index", base.u16be()),
                    ("data", version_record("version", var("table_length"))),
                ]),
            )
        };

        let post_table = {
            let postv2 = record([
                ("num_glyphs", base.u16be()),
                (
                    "glyph_name_index",
                    repeat_count(var("num_glyphs"), base.u16be()),
                ),
                ("string_data", pos32()),
            ]);

            let postv2dot5 = record([
                ("num_glyphs", base.u16be()),
                ("offset", repeat_count(var("num_glyphs"), s8(base))),
            ]);

            module.define_format(
                "opentype.post_table",
                record([
                    ("version", version16_16(base)),
                    ("italic_angle", fixed32be(base)),
                    ("underline_position", s16be(base)),
                    ("underline_thickness", s16be(base)),
                    ("is_fixed_pitch", base.u32be()), // nonzero <=> fixed pitch
                    ("min_mem_type42", base.u32be()),
                    ("max_mem_type42", base.u32be()),
                    ("min_mem_type1", base.u32be()),
                    ("max_mem_type1", base.u32be()),
                    (
                        "names",
                        match_variant(
                            var("version"),
                            [
                                (Pattern::U32(0x0001_0000), "Version1", Format::EMPTY),
                                (Pattern::U32(0x0002_0000), "Version2", postv2),
                                (Pattern::U32(0x0002_5000), "Version2Dot5", postv2dot5),
                                (Pattern::U32(0x0003_0000), "Version3", Format::EMPTY),
                                (bind("unknown"), "VersionUnknown", compute(var("unknown"))),
                            ],
                        ),
                    ),
                ]),
            )
        };

        let cvt_table = repeat(s16be(base));
        let fpgm_table = repeat(base.u8());

        let loca_table = {
            module.define_format_args(
                "opentype.loca_table",
                vec![
                    (
                        Label::Borrowed("num_glyphs"),
                        ValueType::Base(BaseType::U16),
                    ),
                    (
                        Label::Borrowed("index_to_loc_format"),
                        ValueType::Base(BaseType::U16),
                    ),
                ],
                record([(
                    "offsets",
                    match_variant(
                        var("index_to_loc_format"),
                        [
                            (
                                Pattern::U16(SHORT_OFFSET16),
                                "Offsets16",
                                repeat_count(add(var("num_glyphs"), Expr::U16(1)), base.u16be()),
                            ),
                            (
                                Pattern::U16(LONG_OFFSET32),
                                "Offsets32",
                                repeat_count(add(var("num_glyphs"), Expr::U16(1)), base.u32be()),
                            ),
                        ],
                    ),
                )]),
            )
        };
        let glyf_table = {
            let simple_flags_raw = module.define_format(
                "opentype.glyph-description.simple.flags-raw",
                flags_bits8([
                    None,
                    Some("overlap_simple"),
                    Some("y_is_same_or_positive_y_short_vector"),
                    Some("x_is_same_or_positive_x_short_vector"),
                    Some("repeat_flag"),
                    Some("y_short_vector"),
                    Some("x_short_vector"),
                    Some("on_curve_point"),
                ]),
            );

            // NOTE - variable-length expanded version of the packed flags being parsed
            let glyf_flags_simple = |num_coordinates: Expr| -> Format {
                // individual flag-set we are parsing

                // Format that parses a flag-entry into its conditionally-parsed repetition-count and relevant, reordered fields
                let flag_list_entry = chain(
                    simple_flags_raw.call(),
                    "flags",
                    record([
                        // NOTE - indicates number of additional repeats, base value 0 for singleton or N for run of N+1 overall
                        (
                            "repeats",
                            if_then_else(
                                record_proj(var("flags"), "repeat_flag"),
                                base.u8(),
                                compute(Expr::U8(0)),
                            ),
                        ),
                        (
                            "field_set",
                            compute(subset_fields(
                                var("flags"),
                                [
                                    "on_curve_point",
                                    "x_short_vector",
                                    "y_short_vector",
                                    "x_is_same_or_positive_x_short_vector",
                                    "y_is_same_or_positive_y_short_vector",
                                    "overlap_simple",
                                ],
                            )),
                        ),
                    ]),
                );
                // Lambda that tells us whether we are done reading flags
                let is_finished =
                    lambda_tuple(["totlen", "_seq"], expr_gte(var("totlen"), num_coordinates));
                let update_totlen = lambda_tuple(
                    ["acc", "flags"],
                    add(
                        var("acc"),
                        add(
                            Expr::AsU16(Box::new(record_proj(var("flags"), "repeats"))),
                            Expr::U16(1),
                        ),
                    ),
                );
                // Format that parses the flags as a packed (unexpanded repeats) array
                let raw_flags = map(
                    accum_until(
                        is_finished,
                        update_totlen,
                        Expr::U16(0),
                        ValueType::Base(BaseType::U16),
                        flag_list_entry,
                    ),
                    lambda_tuple(["_len", "flags"], var("flags")),
                );
                // flattens the flag-array after parsing it, into the final format with expanded repetitions
                map(
                    raw_flags,
                    lambda(
                        "arr_flags",
                        flat_map(
                            lambda(
                                "packed",
                                dup(
                                    add(
                                        Expr::AsU32(Box::new(record_proj(
                                            var("packed"),
                                            "repeats",
                                        ))),
                                        Expr::U32(1),
                                    ),
                                    record_proj(var("packed"), "field_set"),
                                ),
                            ),
                            var("arr_flags"),
                        ),
                    ),
                )
            };

            let x_coords = |field_set: Expr| -> Format {
                if_then_else(
                    record_proj(field_set.clone(), "x_short_vector"),
                    // this wants to be i16
                    map(
                        base.u8(),
                        lambda(
                            "abs",
                            u8_to_i16(
                                var("abs"),
                                record_proj(
                                    field_set.clone(),
                                    "x_is_same_or_positive_x_short_vector",
                                ),
                            ),
                        ),
                    ),
                    if_then_else(
                        record_proj(field_set.clone(), "x_is_same_or_positive_x_short_vector"),
                        // this wants to be i16
                        compute(Expr::U16(0)),
                        s16be(base),
                    ),
                )
            };

            let y_coords = |field_set: Expr| -> Format {
                if_then_else(
                    record_proj(field_set.clone(), "y_short_vector"),
                    // this wants to be i16
                    map(
                        base.u8(),
                        lambda(
                            "abs",
                            u8_to_i16(
                                var("abs"),
                                record_proj(
                                    field_set.clone(),
                                    "y_is_same_or_positive_y_short_vector",
                                ),
                            ),
                        ),
                    ),
                    if_then_else(
                        record_proj(field_set.clone(), "y_is_same_or_positive_y_short_vector"),
                        // this wants to be i16
                        compute(Expr::U16(0)),
                        s16be(base),
                    ),
                )
            };

            let simple_glyf_table = {
                module.define_format_args(
                    "opentype.glyf.simple",
                    vec![(
                        Label::Borrowed("n_contours"),
                        ValueType::Base(BaseType::U16),
                    )],
                    record([
                        (
                            "end_points_of_contour",
                            repeat_count(var("n_contours"), base.u16be()),
                        ),
                        ("instruction_length", base.u16be()),
                        (
                            "instructions",
                            repeat_count(var("instruction_length"), base.u8()),
                        ),
                        (
                            "number_of_coordinates",
                            compute(add(Expr::U16(1), last_elem("end_points_of_contour"))),
                        ),
                        ("flags", glyf_flags_simple(var("number_of_coordinates"))),
                        (
                            "x_coordinates",
                            for_each(var("flags"), "flag_vals", x_coords(var("flag_vals"))),
                        ),
                        (
                            "y_coordinates",
                            for_each(var("flags"), "flag_vals", y_coords(var("flag_vals"))),
                        ),
                    ]),
                )
            };

            let composite_glyf_table = {
                let glyf_arg = |are_words: Expr, are_xy_values: Expr| -> Format {
                    if_then_else(
                        are_words,
                        if_then_else(
                            are_xy_values.clone(),
                            fmt_variant("Int16", s16be(base)),
                            fmt_variant("Uint16", base.u16be()),
                        ),
                        if_then_else(
                            are_xy_values,
                            fmt_variant("Int8", s8(base)),
                            fmt_variant("Uint8", base.u8()),
                        ),
                    )
                };
                let glyf_flags_composite = flags_bits16([
                    None,                              // bit 15 - reserved (0 implied but not enforced)
                    None, // bit 14 - reserved (0 implied but not enforced)
                    None, // bit 13 - reserved (0 implied but not enforced)
                    Some("unscaled_component_offset"), // bit 12 - set if component offset is not to be scaled
                    Some("scaled_component_offset"), // bit 11 - set if component offset is to be scaled
                    Some("overlap_compound"), // bit 10 - hint for whether the component overlap
                    Some("use_my_metrics"), // bit 9 - when set, composite glyph inherits aw, lsb, rsb of current component glyph
                    Some("we_have_instructions"), // bit 8 - instructions present after final component
                    Some("we_have_a_two_by_two"), // bit 7 - we have a two by two transformation that will be used to scale the glyph
                    Some("we_have_an_x_and_y_scale"), // bit 6 - when set, x has a different scale from y
                    Some("more_components"), // bit 5 - continuation bit (1 when more follow, 0 if final)
                    Some("__reserved_bit4"), // bit 4 - reserved, should be 0
                    Some("we_have_a_scale"), // bit 3 - when 1, component has simple scale; otherwise scale is 1.0
                    Some("round_xy_to_grid"), // bit 2 - when set (and when `args_are_xy_values` is set), xy values are rounded to nearest grid line
                    Some("args_are_xy_values"), // bit 1 - when set, args are signed xy values; otherwise, they are unsigned point numbers
                    Some("arg_1_and_2_are_words"), // bit 0 - set for args of type u16 or i16; clear for args of type u8 or i8
                ]);

                let glyf_scale = |flags: Expr| -> Format {
                    if_then_else(
                        record_proj(flags.clone(), "we_have_a_scale"),
                        format_some(fmt_variant("Scale", f2dot14(base))),
                        if_then_else(
                            record_proj(flags.clone(), "we_have_an_x_and_y_scale"),
                            format_some(fmt_variant(
                                "XY",
                                record_repeat(["x_scale", "y_scale"], f2dot14(base)),
                            )),
                            if_then_else(
                                record_proj(flags, "we_have_a_two_by_two"),
                                format_some(fmt_variant(
                                    "Matrix",
                                    tuple_repeat(2, tuple_repeat(2, f2dot14(base))),
                                )),
                                format_none(),
                            ),
                        ),
                    )
                };

                let glyf_component = record([
                    ("flags", glyf_flags_composite),
                    ("glyph_index", base.u16be()),
                    (
                        "argument1",
                        glyf_arg(
                            record_proj(var("flags"), "arg_1_and_2_are_words"),
                            record_proj(var("flags"), "args_are_xy_values"),
                        ),
                    ),
                    (
                        "argument2",
                        glyf_arg(
                            record_proj(var("flags"), "arg_1_and_2_are_words"),
                            record_proj(var("flags"), "args_are_xy_values"),
                        ),
                    ),
                    ("scale", glyf_scale(var("flags"))),
                ]);

                let is_last = lambda_tuple(
                    ["_has_instructions", "seq"],
                    expr_option_map_or(
                        Expr::Bool(false),
                        |elt| expr_not(record_projs(elt, &["flags", "more_components"])),
                        seq_opt_last(var("seq")),
                    ),
                );
                let update_any_instructions = lambda_tuple(
                    ["acc", "glyph"],
                    or(
                        var("acc"),
                        record_projs(var("glyph"), &["flags", "we_have_instructions"]),
                    ),
                );

                module.define_format(
                    "opentype.glyf.composite",
                    chain(
                        accum_until(
                            is_last,
                            update_any_instructions,
                            Expr::Bool(false),
                            ValueType::Base(BaseType::Bool),
                            glyf_component,
                        ),
                        "acc_glyphs",
                        record([
                            ("glyphs", compute(tuple_proj(var("acc_glyphs"), 1))),
                            (
                                "instructions",
                                if_then_else(
                                    tuple_proj(var("acc_glyphs"), 0),
                                    chain(
                                        base.u16be(),
                                        "instructions_length",
                                        repeat_count(var("instructions_length"), base.u8()),
                                    ),
                                    compute(seq_empty()),
                                ),
                            ),
                        ]),
                    ),
                )
            };

            let glyf_description = module.define_format_args(
                "opentype.glyf.description",
                vec![(
                    Label::Borrowed("n_contours"),
                    ValueType::Base(BaseType::U16),
                )],
                match_variant(
                    var("n_contours"),
                    [
                        (Pattern::U16(0), "HeaderOnly", Format::EMPTY),
                        (
                            Pattern::Int(Bounds::new(1, i16::MAX as usize)),
                            "Simple",
                            simple_glyf_table.call_args(vec![var("n_contours")]),
                        ),
                        (Pattern::Wildcard, "Composite", composite_glyf_table.call()),
                    ],
                ),
            );

            let glyf_table_entry = |start_offset: Expr, offset_pair: Expr| {
                if_then_else(
                    // NOTE - checks that the glyph is non-vacuous
                    expr_gt(
                        tuple_proj(offset_pair.clone(), 1),
                        tuple_proj(offset_pair.clone(), 0),
                    ),
                    linked_offset32(
                        start_offset,
                        tuple_proj(offset_pair, 0),
                        fmt_variant(
                            "Glyph",
                            record([
                                ("number_of_contours", s16be(base)),
                                ("x_min", s16be(base)),
                                ("y_min", s16be(base)),
                                ("x_max", s16be(base)),
                                ("y_max", s16be(base)),
                                (
                                    "description",
                                    glyf_description.call_args(vec![var("number_of_contours")]),
                                ),
                            ]),
                        ),
                    ),
                    fmt_variant("EmptyGlyph", Format::EMPTY),
                )
            };

            module.define_format_args(
                "opentype.glyf_table",
                vec![(
                    Label::Borrowed("offset_pairs"),
                    ValueType::Seq(Box::new(ValueType::Tuple(vec![
                        ValueType::Base(BaseType::U32),
                        ValueType::Base(BaseType::U32),
                    ]))),
                )],
                chain(
                    pos32(),
                    "start_offset",
                    for_each(
                        var("offset_pairs"),
                        "offset_pair",
                        glyf_table_entry(var("start_offset"), var("offset_pair")),
                    ),
                ),
            )
        };

        let prep_table = repeat(base.u8());
        // REVIEW - the generated names for gasp subtypes can be run-on, consider pruning name tokens or module.define_format(_args) for brevity
        let gasp_table = {
            let ver0flags = flags_bits16([
                None, // Bit 15 - Reserved
                None, // Bit 14 - Reserved
                None, // Bit 13 - Reserved
                None, // Bit 12 - Reserved
                None, // Bit 11 - Reserved
                None, // Bit 10 - Reserved
                None, // Bit 9 - Reserved
                None, // Bit 8 - Reserved
                None, // Bit 7 - Reserved
                None, // Bit 6 - Reserved
                None, // Bit 5 - Reserved
                None, // Bit 4 - Reserved
                None, // Bit 3 - Version 1 Only
                None, // Bit 2 - Version 1 Only
                Some("dogray"),
                Some("gridfit"),
            ]);

            let ver1flags = flags_bits16([
                None, // Bit 15 - Reserved
                None, // Bit 14 - Reserved
                None, // Bit 13 - Reserved
                None, // Bit 12 - Reserved
                None, // Bit 11 - Reserved
                None, // Bit 10 - Reserved
                None, // Bit 9 - Reserved
                None, // Bit 8 - Reserved
                None, // Bit 7 - Reserved
                None, // Bit 6 - Reserved
                None, // Bit 5 - Reserved
                None, // Bit 4 - Reserved
                Some("symmetric_smoothing"),
                Some("symmetric_gridfit"),
                Some("dogray"),
                Some("gridfit"),
            ]);

            let gasp_record = |ver: Expr| -> Format {
                record([
                    ("range_max_ppem", base.u16be()),
                    (
                        "range_gasp_behavior",
                        match_variant(
                            ver,
                            [
                                (Pattern::U16(0), "Version0", ver0flags),
                                (Pattern::U16(1), "Version1", ver1flags),
                                (Pattern::Wildcard, "BadVersion", Format::Fail), // NOTE - the name of this variant is arbitrary since it won't actually appear anywhere
                            ],
                        ),
                    ),
                ])
            };

            module.define_format(
                "opentype.gasp_table",
                record([
                    ("version", base.u16be()),
                    ("num_ranges", base.u16be()),
                    (
                        "gasp_ranges",
                        repeat_count(var("num_ranges"), gasp_record(var("version"))),
                    ),
                ]),
            )
        };

        module.define_format_args(
            "opentype.table_directory.table_links",
            vec![
                START_ARG,
                (
                    Label::Borrowed("tables"),
                    ValueType::Seq(Box::new(table_type)),
                ),
            ],
            record([
                (
                    "cmap",
                    required_table("start", "tables", magic(b"cmap"), cmap_table.call()),
                ),
                (
                    "head",
                    required_table("start", "tables", magic(b"head"), head_table.call()),
                ),
                (
                    "hhea",
                    required_table("start", "tables", magic(b"hhea"), hhea_table.call()),
                ),
                (
                    "maxp",
                    required_table("start", "tables", magic(b"maxp"), maxp_table.call()),
                ),
                (
                    "hmtx",
                    required_table(
                        "start",
                        "tables",
                        magic(b"hmtx"),
                        hmtx_table.call_args(vec![
                            record_proj(var("hhea"), "number_of_long_horizontal_metrics"),
                            record_proj(var("maxp"), "num_glyphs"),
                        ]),
                    ),
                ),
                (
                    "name",
                    required_table("start", "tables", magic(b"name"), name_table.call()),
                ),
                (
                    "os2",
                    required_table_with_len("start", "tables", magic(b"OS/2"), os2_table),
                ),
                (
                    "post",
                    required_table("start", "tables", magic(b"post"), post_table.call()),
                ),
                // SECTION - truetype outline tables
                (
                    "cvt",
                    optional_table("start", "tables", magic(b"cvt "), cvt_table),
                ),
                (
                    "fpgm",
                    optional_table("start", "tables", magic(b"fpgm"), fpgm_table),
                ),
                (
                    "loca",
                    optional_table(
                        "start",
                        "tables",
                        magic(b"loca"),
                        loca_table.call_args(vec![
                            record_proj(var("maxp"), "num_glyphs"),
                            record_proj(var("head"), "index_to_loc_format"),
                        ]),
                    ),
                ),
                (
                    "glyf",
                    optional_table(
                        "start",
                        "tables",
                        magic(b"glyf"),
                        glyf_table.call_args(vec![loca_offset_pairs(var("loca"))]),
                    ),
                ),
                // !SECTION
                (
                    "prep",
                    optional_table("start", "tables", magic(b"prep"), prep_table),
                ),
                (
                    "gasp",
                    optional_table("start", "tables", magic(b"gasp"), gasp_table.call()),
                ),
                // STUB - add more tables
                ("__skip", Format::SkipRemainder),
            ]),
        )
    };

    let table_directory = module.define_format_args(
        "opentype.table_directory",
        vec![START_ARG],
        record([
            (
                "sfnt_version",
                where_lambda(
                    base.u32be(),
                    "v",
                    or(
                        expr_eq(var("v"), Expr::U32(0x00010000)),
                        expr_eq(var("v"), Expr::U32(magic(b"OTTO"))),
                    ),
                ),
            ),
            ("num_tables", base.u16be()), // number of tables in directory
            ("search_range", base.u16be()), // TODO[validation] - should be (maximum power of 2 <= num_tables) x 16
            ("entry_selector", base.u16be()), // TODO[validation] - should be Log2(maximum power of 2 <= num_tables)
            ("range_shift", base.u16be()), // TODO[validation] - should be (NumTables x 16) - searchRange
            (
                "table_records",
                repeat_count(
                    var("num_tables"),
                    table_record.call_args(vec![var("start")]),
                ),
            ),
            (
                "table_links",
                table_links.call_args(vec![START_VAR, var("table_records")]),
            ),
        ]),
    );

    let ttc_header = {
        // Version 1.0
        let ttc_header1 = |start: Expr| {
            record([
                ("num_fonts", base.u32be()),
                (
                    "table_directories",
                    repeat_count(
                        var("num_fonts"),
                        offset32(start, table_directory.call_args(vec![var("start")]), base),
                    ),
                ),
            ])
        };

        // Version 2.0
        let ttc_header2 = |start: Expr| {
            record([
                ("num_fonts", base.u32be()),
                (
                    "table_directories",
                    repeat_count(
                        var("num_fonts"),
                        offset32(start, table_directory.call_args(vec![var("start")]), base),
                    ),
                ),
                ("dsig_tag", base.u32be()), // either b"DSIG" or 0 if none
                ("dsig_length", base.u32be()), // byte-length or 0 if none
                ("dsig_offset", base.u32be()), // byte-offset or 0 if none
            ])
        };

        module.define_format_args(
            "opentype.ttc_header",
            vec![START_ARG],
            record([
                (
                    "ttc_tag",
                    where_lambda(
                        base.u32be(),
                        "tag",
                        expr_eq(var("tag"), Expr::U32(magic(b"ttcf"))),
                    ),
                ),
                ("major_version", base.u16be()),
                ("minor_version", base.u16be()),
                (
                    "header",
                    match_variant(
                        var("major_version"),
                        [
                            (Pattern::U16(1), "Version1", ttc_header1(var("start"))),
                            (Pattern::U16(2), "Version2", ttc_header2(var("start"))),
                            (bind("unknown"), "UnknownVersion", compute(var("unknown"))),
                        ],
                    ),
                ),
                ("__skip", Format::SkipRemainder),
            ]),
        )
    };

    // NOTE - we have to fail to let text have its chance to parse
    let unknown_table = Format::Fail;

    let opentype_font = module.define_format_args(
        "opentype.font",
        vec![START_ARG],
        record([
            ("magic", Format::Peek(Box::new(base.u32be()))),
            (
                "directory",
                match_variant(
                    var("magic"),
                    [
                        (
                            Pattern::U32(0x00010000),
                            "TableDirectory",
                            table_directory.call_args(vec![var("start")]),
                        ),
                        (
                            Pattern::U32(magic(b"OTTO")),
                            "TableDirectory",
                            table_directory.call_args(vec![var("start")]),
                        ),
                        (
                            Pattern::U32(magic(b"ttcf")),
                            "TTCHeader",
                            ttc_header.call_args(vec![var("start")]),
                        ),
                        (Pattern::Wildcard, "UnknownTable", unknown_table),
                    ],
                ),
            ),
        ]),
    );

    module.define_format(
        "opentype.main",
        record([
            ("start", pos32()),
            ("font", opentype_font.call_args(vec![var("start")])),
        ]),
    )
}

/*
    //! # OpenType Font File Format

    def main = {
        /// The start of the font file.
        start <- stream_pos,
        /// The directory of tables in the font.
        font <- overlap {
            magic <- u32be,
            directory <- match magic {
                // TrueType font
                0x00010000 => table_directory start,
                // CFF font
                "OTTO" => table_directory start,
                // OpenType Font Collection
                "ttcf" => ttc_header start,
                _ => unknown_table,
            },
        },
    };

    /// # Table Directory
    ///
    /// A directory of the top-level tables in the font.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Table Directory](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
    /// - [Apple's TrueType Reference Manual: The Font Directory](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html#Directory)
    def table_directory (file_start : Pos) = {
        /// Version of the font.
        ///
        /// | Value         | Meaning                                   |
        /// | ------------- | ----------------------------------------- |
        /// | `0x00010000`  | for fonts containing TrueType outlines    |
        /// | `0x4F54544F`  | (`'OTTO'`) for fonts containing CFF data  |
        ///
        /// Apple allows 'true' and 'typ1', but this should not be found in OpenType files.
        sfnt_version <- u32be where
            bool_or (sfnt_version == (0x00010000 : U32)) (sfnt_version == ("OTTO" : U32)),

        /// Number of tables in the directory.
        num_tables <- u16be,
        /// For enabling quick binary searches.
        search_range <- u16be,       // TODO: (Maximum power of 2 <= num_tables) x 16
        /// For enabling quick binary searches.
        entry_selector <- u16be,     // TODO: Log2(maximum power of 2 <= num_tables)
        /// For enabling quick binary searches.
        range_shift <- u16be,        // TODO: NumTables x 16-searchRange

        /// An array of table records
        // FIXME: sorted in ascending order by tag
        table_records <- repeat_len16 num_tables table_record,

        /// Font table links
        ///
        /// ## References
        ///
        /// - [Microsoft's OpenType Spec: Font Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#font-tables)
        /// - [Apple's TrueType Reference Manual: TrueType Font files](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html#Overview)
        table_links <- (
            let required_table =
                fun (table_id : tag) =>
                fun (table_format : Format) => {
                    // TODO: let formats
                    table_record <- unwrap (find_table table_records table_id),
                    link <- link_table file_start table_record table_format,
                };

            let required_table_with_len =
                fun (table_id : tag) =>
                fun (table_format : (U32 -> Format)) => {
                    // TODO: let formats
                    table_record <- unwrap (find_table table_records table_id),
                    link <- link_table file_start table_record (table_format table_record.length),
                };

            let optional_table =
                fun (table_id : tag) =>
                fun (table_format : Format) =>
                    option_fold ({} : Format)
                        (fun record => link_table file_start record table_format)
                        (find_table table_records table_id);

            {
                // Required Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#required-tables
                cmap <- required_table "cmap" cmap_table,
                head <- required_table "head" head_table,
                hhea <- required_table "hhea" hhea_table,
                maxp <- required_table "maxp" maxp_table,
                htmx <- required_table "hmtx" {
                    // TODO: let formats
                    hhea <- deref hhea.link,
                    maxp <- deref maxp.link,
                    table <- htmx_table
                        hhea.number_of_long_horizontal_metrics
                        maxp.num_glyphs,
                },
                name <- required_table "name" name_table,
                os2 <- required_table_with_len "OS/2" os2_table,
                post <- required_table "post" post_table,

                // TrueType Outline Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#tables-related-to-truetype-outlines
                cvt <- optional_table "cvt " unknown_table,
                fpgm <- optional_table "fpgm" unknown_table,
                glyf <- optional_table "glyf" {
                    // TODO: let formats
                    maxp <- deref maxp.link,
                    table <- glyf_table 1,
                    // TODO: use `loca` entries when parsing the glyphs
                    // table <- glyf_table maxp.num_glyphs,
                },
                loca <- optional_table "loca" {
                    // TODO: let formats
                    maxp <- deref maxp.link,
                    head <- deref head.link,
                    table <- loca_table maxp.num_glyphs head.index_to_loc_format,
                },
                prep <- optional_table "prep" unknown_table,
                gasp <- optional_table "gasp" unknown_table,

                // CFF Outline Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#tables-related-to-cff-outlines
                cff <- optional_table "CFF " unknown_table,
                cff2 <- optional_table "CFF2" unknown_table,
                vorg <- optional_table "VORG" unknown_table,

                // SVG Outline Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#table-related-to-svg-outlines
                svg <- optional_table "SVG " unknown_table,

                // Bitmap Glyph Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#tables-related-to-bitmap-glyphs
                ebdt <- optional_table "EBDT" unknown_table,
                eblc <- optional_table "EBLC" unknown_table,
                ebsc <- optional_table "EBSC" unknown_table,

                // Color Bitmap Glyph Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#tables-related-to-bitmap-glyphs
                cbdt <- optional_table "CBDT" unknown_table,
                cblc <- optional_table "CBLC" unknown_table,
                sbix <- optional_table "sbix" unknown_table,

                // Advanced Typographic Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#advanced-typographic-tables
                base <- optional_table "BASE" base_table,
                gdef <- optional_table "GDEF" gdef_table,
                gpos <- optional_table "GPOS" gpos_table,
                gsub <- optional_table "GSUB" gsub_table,
                jstf <- optional_table "JSTF" jstf_table,
                math <- optional_table "MATH" math_table,

                // OpenType Font Variation Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#tables-used-for-opentype-font-variations
                avar <- optional_table "avar" unknown_table,
                cvar <- optional_table "cvar" unknown_table,
                fvar <- optional_table "fvar" unknown_table,
                gvar <- optional_table "gvar" unknown_table,
                hvar <- optional_table "HVAR" unknown_table,
                mvar <- optional_table "MVAR" unknown_table,
                stat <- optional_table "STAT" unknown_table,
                vvar <- optional_table "VVAR" unknown_table,

                // Color Font Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#tables-related-to-color-fonts
                colr <- optional_table "COLR" unknown_table,
                cpal <- optional_table "CPAL" unknown_table,

                // Other OpenType Tables
                //
                // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#other-opentype-tables
                dsig <- optional_table "DSIG" unknown_table,
                hdmx <- optional_table "hdmx" unknown_table,
                kern <- optional_table "kern" unknown_table,
                ltsh <- optional_table "LTSH" unknown_table,
                merg <- optional_table "MERG" unknown_table,
                meta <- optional_table "meta" unknown_table,
                pclt <- optional_table "PCLT" unknown_table,
                vdmx <- optional_table "VDMX" unknown_table,
                vhea <- optional_table "vhea" unknown_table,
                vmtx <- optional_table "vmtx" unknown_table,
            }
        ),
    };

    /// # TTC Header (OpenType Font Collection)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: TTC Header](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#ttc-header)
    def ttc_header (start : Pos) = (
        /// TTC Header Version 1.0
        let ttc_header1 = fun (start : Pos) => {
            /// Number of fonts in TTC
            num_fonts <- u32be,
            /// Array of offsets to the TableDirectory for each font from the beginning of the file
            table_directories <- repeat_len32 num_fonts (offset32 start (table_directory start)),
        };

        /// TTC Header Version 2.0
        let ttc_header2 = fun (start : Pos) => {
            /// Number of fonts in TTC
            num_fonts <- u32be,
            /// Array of offsets to the TableDirectory for each font from the beginning of the file
            table_directories <- repeat_len32 num_fonts (offset32 start (table_directory start)),
            /// Tag indicating that a DSIG table exists, 0x44534947 ('DSIG') (null if no signature)
            dsig_tag <- u32be,
            /// The length (in bytes) of the DSIG table (null if no signature)
            dsig_length <- u32be,
            /// The offset (in bytes) of the DSIG table from the beginning of the TTC file (null if no
            /// signature)
            dsig_offset <- u32be,
        };

        {
            /// Font Collection ID string: 'ttcf' (used for fonts with CFF or CFF2 outlines as well as
            /// TrueType outlines)
            ttc_tag <- tag,
            /// Major version of the TTC Header
            major_version <- u16be,
            /// Minor version of the TTC Header
            minor_version <- u16be,
            /// Version specific fields
            header <- match major_version {
                1 => ttc_header1 start,
                2 => ttc_header2 start,
                _ => unknown_table,
            },
        }
    );


    /// # Table Record
    ///
    /// A record that stores an offset to another table in the font file.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Table Directory](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
    /// - [Apple's TrueType Reference Manual: The Font Directory](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html#Directory)
    def table_record = {
        /// Table identifier.
        table_id <- tag,
        /// CheckSum for this table.
        ///
        /// ## References
        ///
        /// - [Microsoft's OpenType Spec: Calculating Checksums](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#calculating-checksums)
        checksum <- u32be,
        /// Offset from the beginning of the TrueType font file.
        offset <- u32be,
        /// Length of this table.
        length <- u32be,
    };

    /// Find a table record using the given `table_id`.
    def find_table =
        fun (@num_tables : U16) =>
        fun (table_records : Array16 num_tables (Repr table_record)) =>
        fun (table_id : Repr tag) =>
            // TODO: accelerate using binary search
            // TODO: make use of `table_record.search_range`
            // TODO: make use of `table_record.entry_selector`
            // TODO: make use of `table_record.range_shift`
            array16_find
                (fun (table_record : Repr table_record) => table_record.table_id == table_id)
                table_records;

    /// Create a link to the given `table_format`.
    def link_table =
        fun (file_start : Pos) =>
        fun (table_record : table_record) =>
        fun (table_format : Format) =>
            // TODO: make use of `table_record.checksum`
            link (pos_add_u32 file_start table_record.offset)
                (limit32 table_record.length table_format);

    // -----------------------------------------------------------------------------

    /// Reserved formats
    def reserved (format : Format) (default : format) =
        format; // TODO: set to `default` during serialisation

    /// Deprecated formats
    def deprecated (format : Format) (default : format) =
        format; // TODO: set to `default` during serialisation


    // -----------------------------------------------------------------------------

    // # Common Formats
    //
    // Common formats to be used in the OpenType specification.
    //
    // ## References
    //
    // - [Microsoft's OpenType Spec: Data Types](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#data-types)
    // - [Apple's TrueType Reference Manual: Data Types](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html#Types)

    // TODO: move to separate module

    /// Signed 32-bit fixed-point number (16.16)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Fixed](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_Fixed)
    def fixed : Format = u32be;

    /// Signed, 16-bit integer that describes a quantity in font design units.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: FWORD](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_FWORD)
    def fword : Format = s16be;

    /// Unsigned, 16-bit integer that describes a quantity in font design units.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: UFWORD](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_UFWORD)
    def ufword : Format = u16be;

    /// Signed 16-bit fixed number with the low 14 bits of fraction (2.14).
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: F2DOT14](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_F2DOT14)
    def f2dot14 : Format = s16be;

    /// Unsigned 24-bit integer
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: uint24](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_uint24)
    def u24be : Format = repeat_len8 3 u8;

    /// Date represented in number of seconds since 12:00 midnight, January 1, 1904.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LONGDATETIME](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_LONGDATETIME)
    def long_date_time : Format = s64be;

    /// Array of four `U8`s used to identify a table, design-variation axis, script,
    /// language system, feature, or baseline.
    ///
    /// The elements of the array are expected to be in the range [0x20, 0x7E].
    /// This corresponds to the range of printable ASCII characters.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Tag](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_Tag)
    def tag : Format =
        // TODO: constrain array elements to the range 0x20 to 0x7E.
        // TODO: pattern matching on arrays
        // repeat_len8 4 u8;
        u32be;

    /// # Unknown table format
    ///
    /// This is a placeholder for a table that has an unknown identifier (due to the
    /// file conforming to a newer version of the specification), or for a table has
    /// not yet been implemented.
    def unknown_table : Format = {};

    /// A format that consumes no input.
    def empty : Format = {};

    /// 16-bit offset to a `format`, relative to some `base` position.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Offset16](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_Offset16)
    def offset16 (base : Pos) (format : Format) = {
        offset <- u16be,
        link <- match offset {
            0 => empty,
            _ => link (pos_add_u16 base offset) format, // TODO: Use an option type?
        },
    };

    /// 32-bit offset to a `format`, relative to some `base` position.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Offset32](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_Offset32)
    def offset32 (base : Pos) (format : Format) = {
        offset <- u32be,
        link <- match offset {
            0 => empty,
            _ => link (pos_add_u32 base offset) format, // TODO: Use an option type?
        },
    };

    /// Packed 32-bit value with major and minor version numbers.
    ///
    /// Used only in the 'maxp', 'post' and 'vhea' tables, for backward
    /// compatibility reasons.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Version16Dot16](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#dt_Version16Dot16)
    def version16dot16 = u32be;


    // -----------------------------------------------------------------------------

    /// # Platform identifiers
    ///
    /// | Value         | Meaning                           |
    /// | ------------- | --------------------------------- |
    /// | `0`           | Unicode                           |
    /// | `1`           | Macintosh                         |
    /// | `2`           | ISO (deprecated in OpenType v1.3) |
    /// | `3`           | Windows                           |
    /// | `4`           | Custom                            |
    /// | `5..<240`     | Reserved                          |
    /// | `240..<256`   | User-defined                      |
    ///
    /// Value `1` (Macintosh) is discouraged on current platforms – prefer a value
    /// of `3` (Windows) for maximum compatibility.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Platform IDs](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#platform-ids)
    /// - [Microsoft's OpenType Spec: Platform, encoding and language](https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-encoding-and-language)
    /// - [Apple's TrueType Reference Manual: The `'cmap'` encoding subtables](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    /// - [Apple's TrueType Reference Manual: The platform identifier](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html)
    def platform_id =
        u16be;

    /// # Platform-specific encoding identifiers
    ///
    // TODO: document encoding IDs
    def encoding_id (platform : platform_id) =
        u16be;

    /// # Language identifiers
    ///
    /// This must be set to `0` for all subtables that have a platform ID other than
    /// ‘Macintosh’.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Use of the language field in 'cmap' subtables](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#use-of-the-language-field-in-cmap-subtables)
    /// - [Apple's TrueType Reference Manual: The `'cmap'` table and language codes](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    ///
    // TODO: add more details to docs
    def language_id =
        u16be;

    def language_id32 =
        u32be;


    // # Common Table Formats

    /// # Class Definition Table
    ///
    /// | Class | Description                                               |
    /// |-------|-----------------------------------------------------------|
    /// | 1     | Base glyph (single character, spacing glyph)              |
    /// | 2     | Ligature glyph (multiple character, spacing glyph)        |
    /// | 3     | Mark glyph (non-spacing combining glyph)                  |
    /// | 4     | Component glyph (part of single character, spacing glyph) |
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Class Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table)

    /// # Class Definition Table Format 1
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Class Definition Table Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table-format-1)
    def class_def_format_1 = {
        /// First glyph ID of the class_value_array
        start_glyph_id <- u16be,
        /// Size of the class_value_array
        glyph_count <- u16be,
        /// Array of Class Values — one per glyph ID
        class_value_array <- repeat_len16 glyph_count u16be,
    };

    /// # Class Definition Table Format 2
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Class Definition Table Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table-format-2)
    def class_def_format_2 = (
        /// ClassRangeRecord
        let class_range_record = {
            /// First glyph ID in the range
            start_glyph_id <- u16be,
            /// Last glyph ID in the range
            end_glyph_id <- u16be,
            /// Applied to all glyphs in the range
            class <- u16be,
        };

        {
            /// Number of ClassRangeRecords
            class_range_count <- u16be,
            /// Array of ClassRangeRecords — ordered by startGlyphID
            class_range_records <- repeat_len16 class_range_count class_range_record,
        }
    );

    /// # Class Definition Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Class Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table)
    def class_def = {
        /// Format identifier
        class_format <- u16be,
        /// Format specific data
        data <- match class_format {
            1 => class_def_format_1,
            2 => class_def_format_2,
            _ => unknown_table,
        },
    };

    /// # Coverage Format 1
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Coverage Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#coverage-format-1)
    def coverage_format_1 = {
        /// Number of glyphs in the glyph array
        glyph_count <- u16be,
        /// Array of glyph IDs — in numerical order
        glyph_array <- repeat_len16 glyph_count u16be,
    };

    /// # Coverage Format 2
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Coverage Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#coverage-format-2)
    def coverage_format_2 = (
        /// RangeRecord
        let range_record = {
            /// First glyph ID in the range
            start_glyph_id <- u16be,
            /// Last glyph ID in the range
            end_glyph_id <- u16be,
            /// Coverage Index of first glyph ID in range
            start_coverage_index <- u16be,
        };

        {
            /// Number of RangeRecords
            range_count <- u16be,
            /// Array of glyph ranges — ordered by startGlyphID
            range_records <- repeat_len16 range_count range_record,
        }
    );

    /// # Coverage Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Coverage Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#coverageTbl)
    def coverage_table = {
        /// Format identifier
        coverage_format <- u16be,
        /// Format specific data
        data <- match coverage_format {
            1 => coverage_format_1,
            2 => coverage_format_2,
            _ => unknown_table,
        }
    };

    /// # Sequence Lookup Record
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Sequence Lookup Record](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#sequence-lookup-record)
    def sequence_lookup_record = {
        /// Index (zero-based) into the input glyph sequence
        sequence_index <- u16be,
        /// Index (zero-based) into the LookupList
        lookup_list_index <- u16be,
    };

    /// # Device and VariationIndex Tables
    ///
    /// Device tables and VariationIndex tables are used to provide adjustments to font-unit values in
    /// GPOS, JSTF, GDEF or BASE tables, such as the X and Y coordinates of an attachment anchor
    /// position.
    ///
    /// Curiously the table has two interpretations. The second interprtation appears to be have been
    /// tacked on for variable fonts. The gist being that if the delta format is 0x8000 then the table
    /// is a VariationIndex table, which names the fields differently and does not contain a delta
    /// value array. E.g.
    ///
    /// let variation_index_table = {
    ///     /// A delta-set outer index — used to select an item variation data subtable within the item variation store.
    ///     delta_set_outer_index <- u16be,
    ///     /// A delta-set inner index — used to select a delta-set row within an item variation data subtable.
    ///     delta_set_inner_index <- u16be,
    ///     /// Format, = 0x8000
    ///     delta_format <- u16be,
    /// };
    ///
    /// We only define `device_table` and have it conditionally read the delta value array.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Device and VariationIndex Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#device-and-variationindex-tables)
    ///
    def device_table = (
        // quotient = numerator / denominator # int division
        // if quotient * denominator < numerator:
        //     quotient + 1
        // else:
        //     quotient
        let u16_div_ceil = fun (numerator : U16) (denominator : U16) => (
            let quotient = u16_div numerator denominator;
            match ((u16_mul quotient denominator) < numerator) {
                true => u16_add quotient 1,
                false => quotient,
            }
        );

        let delta_bits = fun (delta_format : U16) (num_sizes : U16) =>
            match delta_format {
                // Signed 2-bit value, 8 values per u16be
                0x0001 => u16_mul num_sizes 2,
                // Signed 4-bit value, 4 values per u16be
                0x0002 => u16_mul num_sizes 4,
                // Signed 8-bit value, 2 values per u16be
                0x0003 => u16_mul num_sizes 8,
                // Unreachable due to match done in device_or_variation_index_table
                _ => 0,
            };

        let num_sizes = fun (start : U16) (end : U16) =>
            u16_add (u16_sub end start) 1;

        {
            /// Smallest size to correct, in ppem
            start_size <- u16be,
            /// Largest size to correct, in ppem
            end_size <- u16be,
            /// Format of deltaValue array data
            delta_format <- u16be,
            /// Array of compressed data
            delta_values <-
                let delta_bits = delta_bits delta_format (num_sizes start_size end_size);
                repeat_len16 (u16_div_ceil delta_bits 16) u16be,
        }
    );

    /// VariationIndex table
    def variation_index_table = {
        /// A delta-set outer index — used to select an item variation data subtable within the item
        /// variation store.
        delta_set_outer_index <- u16be,
        /// A delta-set inner index — used to select a delta-set row within an item variation data
        /// subtable.
        delta_set_inner_index <- u16be,
    };

    def device_or_variation_index_table = overlap {
        // Initial pass to figure out the table format
        init <- {
            _skipped <- repeat_len8 4 u8,
            table_format <- u16be,
        },
        // Device and VariationIndex Tables
        table <- match init.table_format {
            0x0001 => device_table,
            0x0002 => device_table,
            0x0003 => device_table,
            0x8000 => variation_index_table,
            _ => unknown_table,
        },
    };

    /// # Language System Table
    ///
    /// Also known as LangSys table.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Language System Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#language-system-table)
    def lang_sys = {
        /// = NULL (reserved for an offset to a reordering table)
        lookup_order_offset <- u16be,
        /// Index of a feature required for this language system; if no required features = 0xFFFF
        required_feature_index <- u16be,
        /// Number of feature index values for this language system — excludes the required feature
        feature_index_count <- u16be,
        /// Array of indices into the FeatureList, in arbitrary order
        feature_indices <- repeat_len16 feature_index_count u16be,
    };

    /// # Language System Record
    ///
    /// Also known as LangSys record.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Script Table and Language System Record](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#script-table-and-language-system-record)
    def lang_sys_record (script_start : Pos) = {
        /// 4-byte LangSysTag identifier
        lang_sys_tag <- tag,
        /// Offset to LangSys table, from beginning of Script table
        lang_sys <- offset16 script_start lang_sys,
    };

    /// # Script table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Script Table and Language System Record](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#script-table-and-language-system-record)
    def script_table = {
        /// The start of the script table
        table_start <- stream_pos,
        /// Offset to default LangSys table, from beginning of Script table — may be NULL
        default_lang_sys <- offset16 table_start lang_sys,
        /// Number of LangSysRecords for this script — excluding the default LangSys
        lang_sys_count <- u16be,
        /// Array of LangSysRecords, listed alphabetically by LangSys tag
        lang_sys_records <- repeat_len16 lang_sys_count (lang_sys_record table_start),
    };

    /// # Script list table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Script List Table and Script Record](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#script-list-table-and-script-record)
    def script_list = (
        /// ScriptRecord
        let script_record = fun (script_list_start : Pos) => {
            /// 4-byte script tag identifier
            script_tag <- tag,
            /// Offset to Script table, from beginning of ScriptList
            script <- offset16 script_list_start script_table,
        };

        {
            /// The start of the script list table
            table_start <- stream_pos,
            /// Number of ScriptRecords
            script_count <- u16be,
            /// Array of ScriptRecords, listed alphabetically by script tag
            script_records <- repeat_len16 script_count (script_record table_start),
        }
    );

    /// # Feature Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Feature Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#feature-table)
    def feature_table = {
        /// The start of the feature table
        table_start <- stream_pos,
        /// Offset from start of Feature table to FeatureParams table, if defined for the feature and
        /// present, else NULL
        feature_params <- u16be, // TODO: The format of the params table depends on the feature tag
        /// Number of LookupList indices for this feature
        lookup_index_count <- u16be,
        /// Array of indices into the LookupList — zero-based (first lookup is LookupListIndex = 0)
        lookup_list_indices <- repeat_len16 lookup_index_count u16be,
    };

    /// # Feature List Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Feature List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#feature-list-table)
    def feature_list = (
        let feature_record = fun (feature_list_start : Pos) => {
            /// 4-byte feature identification tag
            feature_tag <- tag,
            /// Offset to Feature table, from beginning of FeatureList
            feature <- offset16 feature_list_start feature_table,
        };

        {
            /// The start of the feature list table
            table_start <- stream_pos,
            /// Number of FeatureRecords in this table
            feature_count <- u16be,
            /// Array of FeatureRecords — zero-based (first feature has FeatureIndex = 0), listed
            /// alphabetically by feature tag
            feature_records <- repeat_len16 feature_count (feature_record table_start),
        }
    );

    /// # Sequence Context Format 1: simple glyph contexts
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Sequence Context Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#sequence-context-format-1-simple-glyph-contexts)
    def sequence_context_format1 = (
        /// SequenceRule table
        let sequence_rule = {
            /// Number of glyphs in the input glyph sequence
            glyph_count <- u16be,
            /// Number of SequenceLookupRecords
            seq_lookup_count <- u16be,
            /// Array of input glyph IDs—starting with the second glyph
            input_sequence <- repeat_len16 (u16_sub glyph_count 1) u16be,
            /// Array of Sequence lookup records
            seq_lookup_records <- repeat_len16 seq_lookup_count sequence_lookup_record,
        };

        /// SequenceRuleSet table—all contexts beginning with the same glyph
        let sequence_rule_set = {
            /// The start of the table
            table_start <- stream_pos,
            /// Number of SequenceRule tables
            seq_rule_count <- u16be,
            /// Array of offsets to SequenceRule tables, from beginning of the SequenceRuleSet table
            seq_rules <- repeat_len16 seq_rule_count (offset16 table_start sequence_rule),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Offset to Coverage table, from beginning of SequenceContextFormat1 table
            coverage <- offset16 table_start coverage_table,
            /// Number of SequenceRuleSet tables
            seq_rule_set_count <- u16be,
            /// Array of offsets to SequenceRuleSet tables, from beginning of
            /// SequenceContextFormat1 table (offsets may be NULL)
            seq_rule_sets <- repeat_len16 seq_rule_set_count (offset16 table_start sequence_rule_set),
        }
    );

    /// # Sequence Context Format 2: class-based glyph contexts
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Sequence Context Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#sequence-context-format-2-class-based-glyph-contexts)
    def sequence_context_format2 = (
        /// ClassSequenceRule table
        let class_sequence_rule = {
            /// Number of glyphs to be matched
            glyph_count <- u16be,
            /// Number of SequenceLookupRecords
            seq_lookup_count <- u16be,
            /// Sequence of classes to be matched to the input glyph sequence, beginning with the
            /// second glyph position
            input_sequence <- repeat_len16 (u16_sub glyph_count 1) u16be,
            /// Array of SequenceLookupRecords
            seqLookupRecords <- repeat_len16 seq_lookup_count sequence_lookup_record,
        };

        /// ClassSequenceRuleSet table
        let class_sequence_rule_set = {
            /// The start of the table
            table_start <- stream_pos,
            /// Number of ClassSequenceRule tables
            class_seq_rule_count <- u16be,
            /// Array of offsets to ClassSequenceRule tables, from beginning of ClassSequenceRuleSet
            /// table
            class_seq_rules <- repeat_len16 class_seq_rule_count (offset16 table_start class_sequence_rule),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Offset to Coverage table, from beginning of SequenceContextFormat1 table
            coverage <- offset16 table_start coverage_table,
            /// Offset to ClassDef table, from beginning of SequenceContextFormat2 table
            class_def <- (offset16 table_start class_def),
            /// Number of ClassSequenceRuleSet tables
            class_seq_rule_set_count <- u16be,
            /// Array of offsets to ClassSequenceRuleSet tables, from beginning of
            /// SequenceContextFormat2 table (may be NULL)
            class_seq_rule_sets <- repeat_len16 class_seq_rule_set_count (offset16 table_start class_sequence_rule_set),
        }
    );

    /// # Sequence Context Format 3: coverage-based glyph contexts
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Sequence Context Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#sequence-context-format-3-coverage-based-glyph-contexts)
    def sequence_context_format3 = {
        /// The start of the table
        table_start <- stream_pos,
        /// Number of glyphs in the input sequence
        glyph_count <- u16be,
        /// Number of SequenceLookupRecords
        seq_lookup_count <- u16be,
        /// Array of offsets to Coverage tables, from beginning of SequenceContextFormat3 subtable
        coverage_tables <- repeat_len16 glyph_count (offset16 table_start coverage_table),
        /// Array of SequenceLookupRecords
        seq_lookup_records <- repeat_len16 seq_lookup_count sequence_lookup_record,
    };

    def sequence_context = {
        /// Format identifier
        format <- u16be,
        /// Format specific substitutions
        subst <- match format {
            1 => sequence_context_format1,
            2 => sequence_context_format2,
            3 => sequence_context_format3,
            _ => unknown_table,
        }
    };

    /// # Chained Sequence Context Format 1: simple glyph contexts
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Chained Sequence Context Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-1-simple-glyph-contexts)
    def chained_sequence_context_format_1 = (
        let chained_sequence_rule = {
            /// Number of glyphs in the backtrack sequence
            backtrack_glyph_count <- u16be,
            /// Array of backtrack glyph IDs
            backtrack_sequence <- repeat_len16 backtrack_glyph_count u16be,
            /// Number of glyphs in the input sequence
            input_glyph_count <- u16be,
            /// Array of input glyph IDs—start with second glyph
            input_sequence <- repeat_len16 (u16_sub input_glyph_count 1) u16be,
            /// Number of glyphs in the lookahead sequence
            lookahead_glyph_count <- u16be,
            /// Array of lookahead glyph IDs
            lookahead_sequence <- repeat_len16 lookahead_glyph_count u16be,
            /// Number of SequenceLookupRecords
            seq_lookup_count <- u16be,
            /// Array of SequenceLookupRecords
            seq_lookup_records <- repeat_len16 seq_lookup_count sequence_lookup_record,
        };

        let chained_sequence_rule_set = {
            /// The start of the table
            table_start <- stream_pos,
            /// Number of ChainedSequenceRule tables
            chained_seq_rule_count <- u16be,
            /// Array of offsets to ChainedSequenceRule tables, from beginning of
            /// ChainedSequenceRuleSet table
            chained_seq_rules <- repeat_len16 chained_seq_rule_count (offset16 table_start chained_sequence_rule),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Offset to Coverage table, from beginning of ChainSequenceContextFormat1 table
            coverage <- (offset16 table_start coverage_table),
            /// Number of ChainedSequenceRuleSet tables
            chained_seq_rule_set_count <- u16be,
            /// Array of offsets to ChainedSeqRuleSet tables, from beginning of
            /// ChainedSequenceContextFormat1 table (may be NULL)
            chained_seq_rule_sets <- repeat_len16 chained_seq_rule_set_count (offset16 table_start chained_sequence_rule_set),
        }
    );

    /// # Chained Sequence Context Format 2: class-based glyph contexts
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Chained Sequence Context Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-2-class-based-glyph-contexts)
    def chained_sequence_context_format_2 = (
        /// ChainedClassSequenceRule table
        let chained_class_sequence_rule = {
            /// Number of glyphs in the backtrack sequence
            backtrack_glyph_count <- u16be,
            /// Array of backtrack-sequence classes
            backtrack_sequence <- repeat_len16 backtrack_glyph_count u16be,
            /// Total number of glyphs in the input sequence
            input_glyph_count <- u16be,
            /// Array of input sequence classes, beginning with the second glyph position
            input_sequence <- repeat_len16 (u16_sub input_glyph_count 1) u16be,
            /// Number of glyphs in the lookahead sequence
            lookahead_glyph_count <- u16be,
            /// Array of lookahead-sequence classes
            lookahead_sequence <- repeat_len16 lookahead_glyph_count u16be,
            /// Number of SequenceLookupRecords
            seq_lookup_count <- u16be,
            /// Array of SequenceLookupRecords
            seq_lookup_records <- repeat_len16 seq_lookup_count sequence_lookup_record,
        };

        /// ChainedClassSequenceRuleSet table
        let chained_class_sequence_rule_set = {
            /// The start of the table
            table_start <- stream_pos,
            /// Number of ChainedClassSequenceRule tables
            chained_class_seq_rule_count <- u16be,
            /// Array of offsets to ChainedClassSequenceRule tables, from beginning of
            /// ChainedClassSequenceRuleSet
            chained_class_seq_rules <- repeat_len16 chained_class_seq_rule_count (offset16 table_start chained_class_sequence_rule),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Offset to Coverage table, from beginning of ChainedSequenceContextFormat2 table
            coverage <- offset16 table_start coverage_table,
            /// Offset to ClassDef table containing backtrack sequence context, from beginning of
            /// ChainedSequenceContextFormat2 table
            backtrack_class_def <- offset16 table_start class_def,
            /// Offset to ClassDef table containing input sequence context, from beginning of
            /// ChainedSequenceContextFormat2 table
            input_class_def <- offset16 table_start class_def,
            /// Offset to ClassDef table containing lookahead sequence context, from beginning of
            /// ChainedSequenceContextFormat2 table
            lookahead_class_def <- offset16 table_start class_def,
            /// Number of ChainedClassSequenceRuleSet tables
            chained_class_seq_rule_set_count <- u16be,
            /// Array of offsets to ChainedClassSequenceRuleSet tables, from beginning of
            /// ChainedSequenceContextFormat2 table (may be NULL)
            chained_class_seq_rule_sets <- repeat_len16 chained_class_seq_rule_set_count (offset16 table_start chained_class_sequence_rule_set),
        }
    );

    /// # Chained Sequence Context Format 3: coverage-based glyph contexts
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Chained Sequence Context Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-3-coverage-based-glyph-contexts)
    def chained_sequence_context_format_3 = {
        /// The start of the table
        table_start <- stream_pos,
        /// Number of glyphs in the backtrack sequence
        backtrack_glyph_count <- u16be,
        /// Array of offsets to coverage tables for the backtrack sequence
        backtrack_coverages <- repeat_len16 backtrack_glyph_count (offset16 table_start coverage_table),
        /// Number of glyphs in the input sequence
        input_glyph_count <- u16be,
        /// Array of offsets to coverage tables for the input sequence
        input_coverage_tables <- repeat_len16 input_glyph_count (offset16 table_start coverage_table),
        /// Number of glyphs in the lookahead sequence
        lookahead_glyph_count <- u16be,
        /// Array of offsets to coverage tables for the lookahead sequence
        lookahead_coverages <- repeat_len16 lookahead_glyph_count (offset16 table_start coverage_table),
        /// Number of SequenceLookupRecords
        seq_lookup_count <- u16be,
        /// Array of SequenceLookupRecords
        seq_lookup_records <- repeat_len16 seq_lookup_count sequence_lookup_record,
    };

    def chained_sequence_context = {
        /// Format identifier
        format <- u16be,
        /// Format specific substitutions
        subst <- match format {
            1 => sequence_context_format1,
            2 => sequence_context_format2,
            3 => sequence_context_format3,
            _ => unknown_table,
        }
    };

    // # Lookup sub-tables

    /// # LookupType 1: Single Substitution Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-1-single-substitution-subtable)
    def single_substitution = {
        /// The start of the sub-table
        table_start <- stream_pos,
        /// Format identifier
        subst_format <- u16be,

        subst <- match subst_format {
            1 => {
                /// Coverage table
                coverage <- offset16 table_start coverage_table,
                /// Add to original glyph ID to get substitute glyph ID
                delta_glyph_id <- s16be,
            },
            2 => {
                /// Coverage table
                coverage <- offset16 table_start coverage_table,
                /// Number of glyph IDs in the substitute_glyph_ids array
                glyph_count <- u16be,
                /// Array of substitute glyph IDs — ordered by Coverage index
                substitute_glyph_ids <- repeat_len16 glyph_count u16be,
            },
            _ => unknown_table
        },
    };

    /// LookupType 2: Multiple Substitution Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-2-multiple-substitution-subtable)
    def multiple_substitution = (
        let sequence_table = {
            /// Number of glyph IDs in the substitute_glyph_ids array. This must always be greater than
            /// 0.
            glyph_count <- u16be,
            /// String of glyph IDs to substitute
            substitute_glyph_ids <- repeat_len16 glyph_count u16be,
        };

        {
            /// The start of the sub-table
            table_start <- stream_pos,
            /// Format identifier
            subst_format <- u16be,
            /// Coverage table
            coverage <- offset16 table_start coverage_table,
            subst <- match subst_format {
                1 => {
                    /// Number of Sequence table offsets in the sequences array
                    sequence_count <- u16be,
                    /// Array of offsets to Sequence tables. Offsets are from beginning of substitution
                    /// subtable, ordered by Coverage index
                    sequences <- repeat_len16 sequence_count (offset16 table_start sequence_table),
                },
                _ => unknown_table,
            },
        }
    );

    /// # LookupType 3: Alternate Substitution Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-3-alternate-substitution-subtable)
    def alternate_substitution = (
        /// AlternateSet table
        let alternate_set = {
            /// Number of glyph IDs in the alternate_glyph_ids array
            glyph_count <- u16be,
            /// Array of alternate glyph IDs, in arbitrary order
            alternate_glyph_ids <- repeat_len16 glyph_count u16be,
        };

        {
            /// The start of the sub-table
            table_start <- stream_pos,
            /// Format identifier
            subst_format <- u16be,
            /// Coverage table
            coverage <- offset16 table_start coverage_table,
            subst <- match subst_format {
                1 => {
                    /// Number of AlternateSet tables
                    alternate_set_count <- u16be,
                    /// Array of offsets to AlternateSet tables. Offsets are from beginning of
                    /// substitution subtable, ordered by Coverage index
                    alternate_sets <- repeat_len16 alternate_set_count (offset16 table_start alternate_set),
                },
                _ => unknown_table,
            },
        }
    );

    /// # LookupType 4: Ligature Substitution Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 4](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-4-ligature-substitution-subtable)
    def ligature_substitution = (
        /// Ligature table: Glyph components for one ligature
        let ligature_table = {
            /// glyph ID of ligature to substitute
            ligature_glyph <- u16be,
            /// Number of components in the ligature
            component_count <- u16be,
            /// Array of component glyph IDs — start with the second component, ordered in writing
            /// direction
            component_glyph_ids <- repeat_len16 (u16_sub component_count 1) u16be,
        };

        /// LigatureSet table: All ligatures beginning with the same glyph
        let ligature_set = {
            /// The start of the LigatureSet table
            table_start <- stream_pos,
            /// Number of Ligature tables
            ligature_count <- u16be,
            /// Array of offsets to Ligature tables. Offsets are from beginning of LigatureSet table,
            /// ordered by preference.
            ///
            /// For each ligature in the set, a Ligature table specifies the glyph ID of the output
            /// ligature glyph (ligatureGlyph); a count of the total number of component glyphs in the
            /// ligature, including the first component (componentCount); and an array of glyph IDs for
            /// the components (componentGlyphIDs). The array starts with the second component glyph in
            /// the ligature (glyph sequence index = 1, componentGlyphIDs array index = 0) because the
            /// first component glyph is specified in the Coverage table.
            ligatures <- repeat_len16 ligature_count (offset16 table_start ligature_table),
        };

        {
            /// The start of the sub-table
            table_start <- stream_pos,
            /// Format identifier
            subst_format <- u16be,
            /// Coverage table
            coverage <- offset16 table_start coverage_table,
            subst <- match subst_format {
                1 => {
                    /// Number of LigatureSet tables
                    ligature_set_count <- u16be,
                    /// Array of offsets to LigatureSet tables. Offsets are from beginning of
                    /// substitution subtable, ordered by Coverage index
                    ligature_sets <- repeat_len16 ligature_set_count (offset16 table_start ligature_set),
                },
                _ => unknown_table,
            },
        }
    );

    /// # LookupType 5: Contextual Substitution Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 5](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-5-contextual-substitution-subtable)
    def contextual_substitution = sequence_context;

    /// # LookupType 6: Chained Contexts Substitution Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 6](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-6-chained-contexts-substitution-subtable)
    def chained_contexts_substitution = chained_sequence_context;

    /// # LookupType 8: Reverse Chaining Contextual Single Substitution Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 8](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-8-reverse-chaining-contextual-single-substitution-subtable)
    def reverse_chaining_contextual_single_substitution = (
        /// ReverseChainSingleSubstFormat1 Subtable
        let reverse_chain_single_subst_format1 = {
            /// The start of the table
            table_start <- stream_pos,
            /// Offset to Coverage table, from beginning of substitution subtable.
            coverage <- offset16 table_start coverage_table,
            /// Number of glyphs in the backtrack sequence.
            backtrack_glyph_count <- u16be,
            /// Array of offsets to coverage tables in backtrack sequence, in glyph sequence order.
            backtrack_coverage_tables <- repeat_len16 backtrack_glyph_count (offset16 table_start coverage_table),
            /// Number of glyphs in lookahead sequence.
            lookahead_glyph_count <- u16be,
            /// Array of offsets to coverage tables in lookahead sequence, in glyph sequence order.
            lookahead_coverage_tables <- repeat_len16 lookahead_glyph_count (offset16 table_start coverage_table),
            /// Number of glyph IDs in the substituteGlyphIDs array.
            glyph_count <- u16be,
            /// Array of substitute glyph IDs — ordered by Coverage index.
            substitute_glyph_ids <- repeat_len16 glyph_count u16be,
        };

        {
            /// Format identifier
            subst_format <- u16be,
            subtable <- match subst_format {
                1 => reverse_chain_single_subst_format1,
                _ => unknown_table,
            }
        }
    );

    // This one is out of order as it needs to refer to all the other substitutions
    /// # LookupType 7: Extension Substitution
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 7](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub#lookuptype-7-extension-substitution)
    def extension_substitution = (
        /// ExtensionSubstFormat1 subtable
        let extension_subst_format1 = {
            /// The start of the table
            table_start <- stream_pos,
            /// Lookup type of subtable referenced by extensionOffset (that is, the extension subtable).
            ///
            /// The extensionLookupType field must be set to any lookup type other than 7.
            extension_lookup_type <- u16be,
            /// Offset to the extension subtable, of lookup type extensionLookupType, relative to the
            /// start of the ExtensionSubstFormat1 subtable.
            extension_subtable <- match extension_lookup_type {
                // Keep in sync with lookup_table GSUB block
                1 => single_substitution,
                2 => multiple_substitution,
                3 => alternate_substitution,
                4 => ligature_substitution,
                5 => contextual_substitution,
                6 => chained_contexts_substitution,
                7 => fail,
                8 => reverse_chaining_contextual_single_substitution,
                _ => unknown_table,

                // TODO: just call back into lookup_table when that's possible
                // 7 => fail,
                // _ => offset32 table_start (lookup_table "GSUB"),
            }
        };

        {
            /// Format identifier
            subst_format <- u16be,
            subtable <- match subst_format {
                1 => extension_subst_format1,
                _ => unknown_table,
            }
        }
    );


    // GPOS:
    //
    // | Type | Name                        | Description                                    |
    // |------|-----------------------------|------------------------------------------------|
    // | 1    | Single adjustment           | Adjust position of a single glyph              |
    // | 2    | Pair adjustment             | Adjust position of a pair of glyphs            |
    // | 3    | Cursive attachment          | Attach cursive glyphs                          |
    // | 4    | MarkToBase attachment       | Attach a combining mark to a base glyph        |
    // | 5    | MarkToLigature attachment   | Attach a combining mark to a ligature          |
    // | 6    | MarkToMark attachment       | Attach a combining mark to another mark        |
    // | 7    | Context positioning         | Position one or more glyphs in context         |
    // | 8    | Chained Context positioning | Position one or more glyphs in chained context |
    // | 9    | Extension positioning       | Extension mechanism for other positionings     |
    // | 10   | Reserved                    | For future use (set to zero)                   |

    /// # Value Record
    ///
    /// Note that all fields of a ValueRecord are optional: to save space, only the fields that are
    /// required need be included in a given instance. Because the GPOS table uses ValueRecords for
    /// many purposes, the sizes and contents of ValueRecords may vary from subtable to subtable.
    ///
    /// A ValueRecord is always accompanied by a ValueFormat flags field that specifies which of the
    /// ValueRecord fields is present. If a ValueRecord specifies more than one value, the values must
    /// be listed in the order shown in the ValueRecord definition. If the associated ValueFormat flags
    /// indicate that a field is not present, then the next present field follows immediately after the
    /// last preceding, present field.
    ///
    /// ## Value Format Flags
    ///
    /// |  Mask  |        Name        |                                                Description                                                |
    /// |:------:|:------------------:|:---------------------------------------------------------------------------------------------------------:|
    /// | 0x0001 | X_PLACEMENT        | Includes horizontal adjustment for placement                                                              |
    /// | 0x0002 | Y_PLACEMENT        | Includes vertical adjustment for placement                                                                |
    /// | 0x0004 | X_ADVANCE          | Includes horizontal adjustment for advance                                                                |
    /// | 0x0008 | Y_ADVANCE          | Includes vertical adjustment for advance                                                                  |
    /// | 0x0010 | X_PLACEMENT_DEVICE | Includes Device table (non-variable font) / VariationIndex table (variable font) for horizontal placement |
    /// | 0x0020 | Y_PLACEMENT_DEVICE | Includes Device table (non-variable font) / VariationIndex table (variable font) for vertical placement   |
    /// | 0x0040 | X_ADVANCE_DEVICE   | Includes Device table (non-variable font) / VariationIndex table (variable font) for horizontal advance   |
    /// | 0x0080 | Y_ADVANCE_DEVICE   | Includes Device table (non-variable font) / VariationIndex table (variable font) for vertical advance     |
    /// | 0xFF00 | Reserved           | For future use (set to zero)                                                                              |
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Value Record](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#value-record)
    def value_record (table_start : Pos) (flags : U16) = (
        let X_PLACEMENT : U16 = 0x0001;
        let Y_PLACEMENT : U16 = 0x0002;
        let X_ADVANCE : U16 = 0x0004;
        let Y_ADVANCE : U16 = 0x0008;
        let X_PLACEMENT_DEVICE : U16 = 0x0010;
        let Y_PLACEMENT_DEVICE : U16 = 0x0020;
        let X_ADVANCE_DEVICE : U16 = 0x0040;
        let Y_ADVANCE_DEVICE : U16 = 0x0080;

        let optional_field = fun (field : U16) (format : Format) =>
            match (u16_and flags field != (0 : U16)) {
                true => format,
                false => empty,
            };

        {
            /// Horizontal adjustment for placement, in design units.
            x_placement <- optional_field X_PLACEMENT s16be,
            /// Vertical adjustment for placement, in design units.
            y_placement <- optional_field Y_PLACEMENT s16be,
            /// Horizontal adjustment for advance, in design units — only used for horizontal layout.
            x_advance <- optional_field X_ADVANCE s16be,
            /// Vertical adjustment for advance, in design units — only used for vertical layout.
            y_advance <- optional_field Y_ADVANCE s16be,
            /// Offset to Device table (non-variable font) / VariationIndex table (variable font) for
            /// horizontal placement, from beginning of the immediate parent table (SinglePos or
            /// PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable)
            /// — may be NULL.
            x_pla_device_offset <- optional_field X_PLACEMENT_DEVICE (offset16 table_start device_or_variation_index_table),
            /// Offset to Device table (non-variable font) / VariationIndex table (variable font) for
            /// vertical placement, from beginning of the immediate parent table (SinglePos or
            /// PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable)
            /// — may be NULL.
            y_pla_device_offset <- optional_field Y_PLACEMENT_DEVICE (offset16 table_start device_or_variation_index_table),
            /// Offset to Device table (non-variable font) / VariationIndex table (variable font) for
            /// horizontal advance, from beginning of the immediate parent table (SinglePos or
            /// PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable)
            /// — may be NULL.
            x_adv_device_offset <- optional_field X_ADVANCE_DEVICE (offset16 table_start device_or_variation_index_table),
            /// Offset to Device table (non-variable font) / VariationIndex table (variable font) for
            /// vertical advance, from beginning of the immediate parent table (SinglePos or
            /// PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable)
            /// — may be NULL.
            y_adv_device_offset <- optional_field Y_ADVANCE_DEVICE (offset16 table_start device_or_variation_index_table),
        }
    );

    /// A value record that is `empty` if flags is 0
    def optional_value_record (table_start : Pos) (flags : U16) =
        match (flags == (0 : U16)) {
            true => empty,
            false => value_record table_start flags
        };

    /// # Anchor Tables
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Anchor Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#anchor-tables)
    def anchor_table = {
        /// The start of the table
        table_start <- stream_pos,
        /// Format identifier
        anchor_format <- u16be,
        table <- match anchor_format {
            // Anchor Table Format 1: Design Units
            1 => {
                /// Horizontal value, in design units
                x_coordinate <- s16be,
                /// Vertical value, in design units
                y_coordinate <- s16be,
            },
            // Anchor Table Format 2: Design Units Plus Contour Point
            2 => {
                /// Horizontal value, in design units
                x_coordinate <- s16be,
                /// Vertical value, in design units
                y_coordinate <- s16be,
                /// Index to glyph contour point
                anchor_point <- u16be,
            },
            // Anchor Table Format 3: Design Units Plus Device or VariationIndex Tables
            3 => {
                /// Horizontal value, in design units
                x_coordinate <- s16be,
                /// Vertical value, in design units
                y_coordinate <- s16be,
                /// Offset to Device table (non-variable font) / VariationIndex table (variable font)
                /// for X coordinate, from beginning of Anchor table (may be NULL)
                x_device_offset <- offset16 table_start device_or_variation_index_table,
                /// Offset to Device table (non-variable font) / VariationIndex table (variable font)
                /// for Y coordinate, from beginning of Anchor table (may be NULL)
                y_device_offset <- offset16 table_start device_or_variation_index_table,
            },
            _ => unknown_table,
        }
    };

    /// # Mark Array Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Mark Array Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#mark-array-table)
    def mark_array_table = (
        /// MarkRecord
        let mark_record = fun (table_start : Pos) => {
            /// Class defined for the associated mark.
            mark_class <- u16be,
            /// Offset to Anchor table, from beginning of MarkArray table.
            mark_anchor_offset <- offset16 table_start anchor_table,
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Number of MarkRecords
            mark_count <- u16be,
            /// Array of MarkRecords, ordered by corresponding glyphs in the associated mark Coverage table.
            mark_records <- repeat_len16 mark_count (mark_record table_start),
        }
    );

    /// # GPOS Lookup Type 1: Single Adjustment Positioning Subtable
    ///
    /// Also known as SinglePos.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-1-single-adjustment-positioning-subtable)
    def single_adjustment = (
        /// Single Adjustment Positioning Format 1: Single Positioning Value
        let single_pos_format1 = fun (table_start : Pos) => {
            /// Offset to Coverage table, from beginning of SinglePos subtable.
            coverage_offset <- offset16 table_start coverage_table,
            /// Defines the types of data in the ValueRecord.
            value_format <- u16be,
            /// Defines positioning value(s) — applied to all glyphs in the Coverage table.
            value_record <- value_record table_start value_format,
        };

        /// Single Adjustment Positioning Format 2: Array of Positioning Values
        let single_pos_format2 = fun (table_start : Pos) => {
            /// Offset to Coverage table, from beginning of SinglePos subtable.
            coverage_offset <- offset16 table_start coverage_table,
            /// Defines the types of data in the ValueRecords.
            value_format <- u16be,
            /// Number of ValueRecords — must equal glyphCount in the Coverage table.
            value_count <- u16be,
            /// Array of ValueRecords — positioning values applied to glyphs.
            value_records <- repeat_len16 value_count (value_record table_start value_format),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Format identifier
            pos_format <- u16be,
            /// Format specific table
            subtable <- match pos_format {
                1 => single_pos_format1 table_start,
                2 => single_pos_format2 table_start,
                _ => unknown_table,
            }
        }
    );

    /// # GPOS Lookup Type 2: Pair Adjustment Positioning Subtable
    ///
    /// A pair adjustment positioning subtable (PairPos).
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-2-pair-adjustment-positioning-subtable)
    def pair_adjustment = (
        /// PairValueRecord
        let pair_value_record = fun (table_start : Pos) (value_format1 : U16) (value_format2 : U16) => {
            /// Glyph ID of second glyph in the pair (first glyph is listed in the Coverage table).
            second_glyph <- u16be,
            /// Positioning data for the first glyph in the pair.
            value_record1 <- optional_value_record table_start value_format1,
            /// Positioning data for the second glyph in the pair.
            value_record2 <- optional_value_record table_start value_format2,
        };

        /// PairSet Table
        let pair_set = fun (value_format1 : U16) (value_format2 : U16) => {
            /// The start of the table
            table_start <- stream_pos,
            /// Number of PairValueRecords
            pair_value_count <- u16be,
            /// Array of PairValueRecords, ordered by glyph ID of the second glyph.
            pair_value_records <- repeat_len16 pair_value_count (pair_value_record table_start value_format1 value_format2),
        };

        /// Pair Adjustment Positioning Format 1: Adjustments for Glyph Pairs (PairPosFormat1)
        let pair_pos_format1 = fun (table_start : Pos) => {
            /// Offset to Coverage table, from beginning of PairPos subtable.
            coverage <- offset16 table_start coverage_table,
            /// Defines the types of data in valueRecord1 — for the first glyph in the pair (may be
            /// zero).
            value_format1 <- u16be,
            /// Defines the types of data in valueRecord2 — for the second glyph in the pair (may be
            /// zero).
            value_format2 <- u16be,
            /// Number of PairSet tables
            pair_set_count <- u16be,
            /// Array of offsets to PairSet tables. Offsets are from beginning of PairPos subtable,
            /// ordered by Coverage Index.
            pair_sets <- repeat_len16 pair_set_count (offset16 table_start (pair_set value_format1 value_format2)),
        };

        /// Class2Record
        let class2_record = fun (table_start : Pos) (value_format1 : U16) (value_format2 : U16) => {
            /// Positioning for first glyph — empty if valueFormat1 = 0.
            value_record1 <- optional_value_record table_start value_format1,
            /// Positioning for second glyph — empty if valueFormat2 = 0.
            value_record2 <- optional_value_record table_start value_format2,
        };

        /// Class1Record
        let class1_record = fun (table_start : Pos) (class2_count : U16) (value_format1 : U16) (value_format2 : U16) => {
            /// Array of Class2 records, ordered by classes in classDef2.
            class2_records <- repeat_len16 class2_count (class2_record table_start value_format1 value_format2),
        };

        /// Pair Adjustment Positioning Format 2: Class Pair Adjustment (PairPosFormat2)
        let pair_pos_format2 = fun (pair_pos_start : Pos) => {
            /// Offset to Coverage table, from beginning of PairPos subtable.
            coverage <- offset16 pair_pos_start coverage_table,
            /// ValueRecord definition — for the first glyph of the pair (may be zero).
            value_format1 <- u16be,
            /// ValueRecord definition — for the second glyph of the pair (may be zero).
            value_format2 <- u16be,
            /// Offset to ClassDef table, from beginning of PairPos subtable — for the first glyph of
            /// the pair.
            class_def1 <- offset16 pair_pos_start class_def,
            /// Offset to ClassDef table, from beginning of PairPos subtable — for the second glyph of
            /// the pair.
            class_def2 <- offset16 pair_pos_start class_def,
            /// Number of classes in classDef1 table — includes Class 0.
            class1_count <- u16be,
            /// Number of classes in classDef2 table — includes Class 0.
            class2_count <- u16be,
            /// Array of Class1 records, ordered by classes in classDef1.
            class1_records <- repeat_len16 class1_count (class1_record pair_pos_start class2_count value_format1 value_format2),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Format identifier
            pos_format <- u16be,
            /// Format specific table
            subtable <- match pos_format {
                1 => pair_pos_format1 table_start,
                2 => pair_pos_format2 table_start,
                _ => unknown_table,
            }
        }
    );

    /// # GPOS Lookup Type 3: Cursive Attachment Positioning Subtable
    ///
    /// Also known as CursivePos.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-3-cursive-attachment-positioning-subtable)
    def cursive_attachment = (
        /// EntryExitRecord
        let entry_exit_record = fun (table_start : Pos) => {
            /// Offset to entry Anchor table, from beginning of CursivePos subtable (may be NULL).
            entry_anchor <- offset16 table_start anchor_table,
            /// Offset to exit Anchor table, from beginning of CursivePos subtable (may be NULL).
            exit_anchor <- offset16 table_start anchor_table,
        };


        /// CursivePosFormat1 Subtable
        let cursive_pos_format1 = fun (table_start : Pos) => {
            /// Offset to Coverage table, from beginning of CursivePos subtable.
            coverage <- offset16 table_start coverage_table,
            /// Number of EntryExit records
            entry_exit_count <- u16be,
            /// Array of EntryExit records, in Coverage index order.
            entry_exit_record <- repeat_len16 entry_exit_count (entry_exit_record table_start),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Format identifier
            pos_format <- u16be,
            /// Format specific table
            subtable <- match pos_format {
                1 => cursive_pos_format1 table_start,
                _ => unknown_table,
            }
        }
    );

    /// # GPOS Lookup Type 4: Mark-to-Base Attachment Positioning Subtable
    ///
    /// Also known as MarkBasePos.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 4](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-4-mark-to-base-attachment-positioning-subtable)
    def mark_to_base_attachment = (
        /// BaseRecord
        let base_record = fun (table_start : Pos) (mark_class_count : U16) => {
            /// Array of offsets (one per mark class) to Anchor tables. Offsets are from beginning of
            /// BaseArray table, ordered by class (offsets may be NULL).
            base_anchors <- repeat_len16 mark_class_count (offset16 table_start anchor_table),
        };

        /// BaseArray Table
        let base_array_table = fun (table_start : Pos) (mark_class_count : U16) => {
            /// Number of BaseRecords
            base_count <- u16be,
            /// Array of BaseRecords, in order of baseCoverage Index.
            base_records <- repeat_len16 base_count (base_record table_start mark_class_count),
        };

        /// MarkBasePosFormat1 Subtable
        let mark_base_pos_format1 = fun (table_start : Pos) => {
            /// Offset to markCoverage table, from beginning of MarkBasePos subtable.
            mark_coverage <- offset16 table_start coverage_table,
            /// Offset to baseCoverage table, from beginning of MarkBasePos subtable.
            base_coverage <- offset16 table_start coverage_table,
            /// Number of classes defined for marks
            mark_class_count <- u16be,
            /// Offset to MarkArray table, from beginning of MarkBasePos subtable.
            mark_array <- offset16 table_start mark_array_table,
            /// Offset to BaseArray table, from beginning of MarkBasePos subtable.
            base_array <- offset16 table_start (base_array_table table_start mark_class_count),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Format identifier
            pos_format <- u16be,
            /// Format specific table
            subtable <- match pos_format {
                1 => mark_base_pos_format1 table_start,
                _ => unknown_table,
            }
        }
    );

    /// # GPOS Lookup Type 5: Mark-to-Ligature Attachment Positioning Subtable
    ///
    /// Also known as MarkLigPos.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 5](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-5-mark-to-ligature-attachment-positioning-subtable)
    def mark_to_ligature_attachment = (
        let component_record = fun (table_start : Pos) (mark_class_count : U16) => {
            /// Array of offsets (one per class) to Anchor tables. Offsets are from beginning of
            /// LigatureAttach table, ordered by class (offsets may be NULL).
            ligature_anchors <- repeat_len16 mark_class_count (offset16 table_start anchor_table),
        };

        /// LigatureAttach Table
        let ligature_attach = fun (table_start : Pos) (mark_class_count : U16) => {
            /// Number of ComponentRecords in this ligature
            component_count <- u16be,
            /// Array of Component records, ordered in writing direction.
            component_records <- repeat_len16 component_count (component_record table_start mark_class_count),
        };

        /// LigatureArray Table
        let ligature_array = fun (table_start : Pos) (mark_class_count : U16) => {
            /// Number of LigatureAttach table offsets
            ligature_count <- u16be,
            /// Array of offsets to LigatureAttach tables. Offsets are from beginning of LigatureArray
            /// table, ordered by ligatureCoverage index.
            ligature_attaches <- repeat_len16 ligature_count (offset16 table_start (ligature_attach table_start mark_class_count)),
        };

        /// MarkLigPosFormat1 Subtable
        let mark_lig_pos_format1 = fun (table_start : Pos) => {
            /// Offset to markCoverage table, from beginning of MarkLigPos subtable.
            mark_coverage <- offset16 table_start coverage_table,
            /// Offset to ligatureCoverage table, from beginning of MarkLigPos subtable.
            ligature_coverage <- offset16 table_start coverage_table,
            /// Number of defined mark classes
            mark_class_count <- u16be,
            /// Offset to MarkArray table, from beginning of MarkLigPos subtable.
            mark_array <- offset16 table_start mark_array_table,
            /// Offset to LigatureArray table, from beginning of MarkLigPos subtable.
            ligature_array <- offset16 table_start (ligature_array table_start mark_class_count),
        };

        {
            /// The start of the table
            table_start <- stream_pos,
            /// Format identifier
            pos_format <- u16be,
            /// Format specific table
            subtable <- match pos_format {
                1 => mark_lig_pos_format1 table_start,
                _ => unknown_table,
            }
        }
    );

    /// # GPOS Lookup Type 6: Mark-to-Mark Attachment Positioning Subtable
    ///
    /// The MarkToMark attachment (MarkMarkPos) subtable is identical in form to the MarkToBase
    /// attachment subtable, although its function is different.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 6](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-6-mark-to-mark-attachment-positioning-subtable)
    def mark_to_mark_attachment = mark_to_base_attachment;

    /// # GPOS Lookup Type 7: Contextual Positioning Subtables
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 7](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-7-contextual-positioning-subtables)
    def contextual_positioning = sequence_context;

    /// # GPOS Lookup Type 8: Chained Contexts Positioning Subtable
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GPOS Lookup Type 8](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookuptype-8-chained-contexts-positioning-subtable)
    def chained_contexts_positioning = chained_sequence_context;

    /// # LookupType 9: Extension Positioning
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: LookupType 9](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookuptype-9-extension-positioning)
    def extension_positioning = (
        /// ExtensionPosFormat1 subtable
        let extension_pos_format1 = {
            /// The start of the table
            table_start <- stream_pos,
            /// Lookup type of subtable referenced by extensionOffset (that is, the extension subtable).
            ///
            /// The extensionLookupType field must be set to any lookup type other than 9.
            extension_lookup_type <- u16be,
            /// Offset to the extension subtable, of lookup type extensionLookupType, relative to the
            /// start of the ExtensionPosFormat1 subtable.
            extension_subtable <- match extension_lookup_type {
                // Keep in sync with lookup_table GPOS block
                1 => single_adjustment,
                2 => pair_adjustment,
                3 => cursive_attachment,
                4 => mark_to_base_attachment,
                5 => mark_to_ligature_attachment,
                6 => mark_to_mark_attachment,
                7 => contextual_positioning,
                8 => chained_contexts_positioning,
                _ => unknown_table,

                // TODO: just call back into lookup_table when that's possible
                // 9 => fail,
                // _ => offset32 table_start (lookup_table "GPOS"),
            }
        };

        {
            /// Format identifier
            pos_format <- u16be,
            subtable <- match pos_format {
                1 => extension_pos_format1,
                _ => unknown_table,
            }
        }
    );

    /// # Lookup table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Lookup Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#lookup-table)
    def lookup_table (tag : U32) = (
        let USE_MARK_FILTERING_SET : U16 = 0x0010;

        let lookup_subtable = fun (tag : U32) (lookup_type : U16) =>
            match tag {
                "GSUB" => match lookup_type {
                    1 => single_substitution,
                    2 => multiple_substitution,
                    3 => alternate_substitution,
                    4 => ligature_substitution,
                    5 => contextual_substitution,
                    6 => chained_contexts_substitution,
                    7 => extension_substitution,
                    8 => reverse_chaining_contextual_single_substitution,
                    _ => unknown_table,
                },
                "GPOS" => match lookup_type {
                    1 => single_adjustment,
                    2 => pair_adjustment,
                    3 => cursive_attachment,
                    4 => mark_to_base_attachment,
                    5 => mark_to_ligature_attachment,
                    6 => mark_to_mark_attachment,
                    7 => contextual_positioning,
                    8 => chained_contexts_positioning,
                    9 => extension_positioning,
                    _ => unknown_table,
                },
                _ => fail,
            };

        {
            /// The start of the lookup table
            table_start <- stream_pos,
            /// Different enumerations for GSUB and GPOS
            lookup_type <- u16be,
            /// Lookup qualifiers
            lookup_flag <- u16be,
            /// Number of subtables for this lookup
            sub_table_count <- u16be,
            /// Array of offsets to lookup subtables, from beginning of Lookup table
            subtables <- repeat_len16 sub_table_count (offset16 table_start (lookup_subtable tag lookup_type)),
            /// Index (base 0) into GDEF mark glyph sets structure. This field is only present if the
            /// USE_MARK_FILTERING_SET lookup flag is set.
            mark_filtering_set <- match (u16_and lookup_flag USE_MARK_FILTERING_SET != (0 : U16)) {
                true => u16be,
                false => empty,
            },
        }
    );

    /// # Lookup List Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Lookup List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#lookup-list-table)
    def lookup_list (tag : U32) = {
        /// The start of the lookup list table
        table_start <- stream_pos,
        /// Number of lookups in this table
        lookup_count <- u16be,
        /// Array of offsets to Lookup tables, from beginning of LookupList
        lookups <- repeat_len16 lookup_count (offset16 table_start (lookup_table tag)),
    };

    // -----------------------------------------------------------------------------

    // # Chararacter to Glyph Index Mappings
    //
    // Formats related to the character mapping table. This table is responsible for
    // mapping character codes to glyph indices used in the font.
    //
    // ## References
    //
    // - [Microsoft's OpenType Spec: cmap — Character to Glyph Index Mapping Table](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap)
    // - [Apple's TrueType Reference Manual: The `'cmap'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)


    /// # Language identifiers
    ///
    /// This must be set to `0` for all subtables that have a platform ID other than
    /// ‘Macintosh’.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Use of the language field in 'cmap' subtables](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#use-of-the-language-field-in-cmap-subtables)
    /// - [Apple's TrueType Reference Manual: : The `'cmap'` table and language codes](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    ///
    // TODO: add more details to docs
    def cmap_language_id (platform : platform_id) =
        language_id;

    // cmap sub-table format 8 has a 32-bit language code
    def cmap_language_id32 (platform : platform_id) =
        language_id32;

    /// A small glyph ID, limited to a glyph set of 256 glyphs.
    def small_glyph_id = u8;

    /// # SequentialMapGroup Record
    ///
    /// Each sequential map group record specifies a character range and the starting glyph ID mapped
    /// from the first character. Glyph IDs for subsequent characters follow in sequence.
    ///
    /// Used in `cmap` sub-table formats 8 and 12.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: cmap sub-table format 8](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-8-mixed-16-bit-and-32-bit-coverage)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 8](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def sequential_map_group = {
        /// First character code in this group; note that if this group is for one or more 16-bit
        /// character codes (which is determined from the is32 array), this 32-bit value will have the
        /// high 16-bits set to zero
        start_char_code <- u32be,
        /// Last character code in this group; same condition as listed above for the startCharCode
        end_char_code <- u32be,
        /// Glyph index corresponding to the starting character code
        start_glyph_id <- u32be,
    };

    /// # ConstantMapGroup Record
    ///
    /// The constant map group record has the same structure as the sequential map group record, with
    /// start and end character codes and a mapped glyph ID. However, the same glyph ID applies to all
    /// characters in the specified range rather than sequential glyph IDs.
    ///
    /// Used in `cmap` sub-table format 13.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: cmap sub-table format 13](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-13-many-to-one-range-mappings)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 13](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def constant_map_group = sequential_map_group;

    /// # UnicodeRange Record
    ///
    /// A range record from the DefaultUVS Table used in `cmap` sub-table format 14.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: cmap sub-table format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 14](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def unicode_range = {
        /// First value in this range
        start_unicode_value <- u24be,
        /// Number of additional values in this range
        additional_count <- u8,
    };

    /// # DefaultUVS Table
    ///
    /// A range-compressed list of Unicode scalar values used in `cmap` sub-table format 14.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: cmap sub-table format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 14](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def default_uvs_table = {
        /// Number of Unicode character ranges.
        num_unicode_value_ranges <- u32be,
        /// Array of UnicodeRange records.
        ranges <- repeat_len32 num_unicode_value_ranges unicode_range,
    };

    /// # UVSMapping Record
    ///
    /// A glyph ID mapping for one base Unicode character used in `cmap` sub-table format 14
    /// NonDefaultUVS Table.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: cmap sub-table format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 14](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def uvs_mapping = {
        /// Base Unicode value of the UVS
        unicode_value <- u24be,
        /// Glyph ID of the UVS
        glyph_id <- u16be,
    };

    /// # NonDefaultUVS Table
    ///
    /// A Non-Default UVS Table is a list of pairs of Unicode scalar values and glyph IDs.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: cmap sub-table format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 14](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def non_default_uvs_table = {
        /// Number of UVS Mappings that follow
        num_uvs_mappings <- u32be,
        /// Array of UVSMapping records.
        uvs_mappings <- repeat_len32 num_uvs_mappings uvs_mapping,
    };

    /// # VariationSelector Record for cmap sub-table format 14
    ///
    /// Each variation selector record specifies a variation selector character, and offsets to
    /// default and non-default tables used to map variation sequences using that variation selector.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: cmap sub-table format 14](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 14](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def variation_selector (table_start : Pos) = {
        /// Variation selector
        var_selector <- u24be,
        /// Offset from the start of the format 14 subtable to Default UVS Table. May be 0.
        default_uvs_offset <- offset32 table_start default_uvs_table,
        /// Offset from the start of the format 14 subtable to Non-Default UVS Table. May be 0.
        non_default_uvs_offset <- offset32 table_start non_default_uvs_table,
    };

    /// # Format 0: Byte encoding table
    ///
    /// A character mapping table for fonts with character codes and glyph indices
    /// that can be stored within single bytes.
    ///
    /// This table but was originally used as the standard character mapping table
    /// on older Macintosh platforms when TrueType was first introduced, but is no
    /// longer required on as fonts have become larger.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 0: Byte encoding table](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-0-byte-encoding-table)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 0](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format0 (platform : platform_id) = {
        /// The length of the subtable in bytes
        length <- u16be,
        /// The language ID of the subtable
        language <- cmap_language_id platform,
        /// A 1 to 1 mapping that converts character codes to glyph indexes (limited
        /// to 256 glyphs). Only the first 256 glyphs will be accessible for larger
        /// glyph sets.
        glyph_id_array <- repeat_len16 256 small_glyph_id,
    };

    /// # Format 2: High-byte mapping through table
    ///
    /// This subtable format was created for “double-byte” encodings following the national character
    /// code standards used for Japanese, Chinese, and Korean characters. These code standards use a
    /// mixed 8-/16-bit encoding. This format is not commonly used today.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 2: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-2-high-byte-mapping-through-table)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 2](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format2 (platform : platform_id) = {
        /// The length of the subtable in bytes
        length <- u16be,
        /// The language ID of the subtable
        language <- cmap_language_id platform,
        /// Array that maps high bytes to subHeaders: value is subHeader index × 8.
        sub_header_keys <- repeat_len16 256 u16be,
        // TODO: These probably need length limiting formats
        // https://github.com/yeslogic/fathom/pull/310
        // /// Variable-length array of SubHeader records.
        // sub_headers[ ]  <- SubHeader,
        // /// Variable-length array containing subarrays used for mapping the low byte of 2-byte characters.
        // glyph_id_array[ ]  <- u16be,
    };

    /// # Format 4: Segment mapping to delta values
    ///
    /// This is the standard character-to-glyph-index mapping subtable for fonts that support only
    /// Unicode Basic Multilingual Plane characters (U+0000 to U+FFFF).
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 4: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-4-segment-mapping-to-delta-values)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 4](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format4 (platform : platform_id) = {
        /// The length of the subtable in bytes
        length <- u16be,
        /// The language ID of the subtable
        language <- cmap_language_id platform,
        /// 2 × segCount.
        seg_count_x2 <- u16be,
        /// Number of contiguous ranges of character codes
        let seg_count = u16_div seg_count_x2 2,
        /// Maximum power of 2 less than or equal to segCount, times 2 ((2**floor(log₂(segCount))) * 2,
        /// where “**” is an exponentiation operator)
        search_range <- u16be,
        /// Log₂ of the maximum power of 2 less than or equal to numTables (log₂(searchRange/2), which
        /// is equal to floor(log₂(segCount)))
        entry_selector <- u16be,
        /// segCount times 2, minus searchRange ((segCount * 2) - searchRange)
        range_shift <- u16be,
        /// End characterCode for each segment, last=0xFFFF.
        end_code <- repeat_len16 seg_count u16be,
        /// Set to 0.
        _reserved_pad <- reserved s16be 0,
        /// Start character code for each segment.
        start_code <- repeat_len16 seg_count u16be,
        /// Delta for all character codes in segment.
        id_delta <- repeat_len16 seg_count s16be,
        /// Offsets into glyphIdArray or 0
        id_range_offsets <- repeat_len16 seg_count u16be,
        // TODO: Needs length limiting formats
        // /// Glyph index array (arbitrary length)
        // glyph_id_array[ ]  <- u16be,
    };

    /// # Format 6: Trimmed table mapping
    ///
    /// Format 6 was designed to map 16-bit characters to glyph indexes when the character codes for a
    /// font fall into a single contiguous range.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 6: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-6-trimmed-table-mapping)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 6](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format6 (platform : platform_id) = {
        /// The length of the subtable in bytes
        length <- u16be,
        /// The language ID of the subtable
        language <- cmap_language_id platform,
        /// First character code of subrange.
        first_code <- u16be,
        /// Number of character codes in subrange.
        entry_count <- u16be,
        /// Array of glyph index values for character codes in the range.
        glyph_id_array <- repeat_len16 entry_count u16be,
    };

    /// # Format 8: mixed 16-bit and 32-bit coverage
    ///
    /// Subtable format 8 was designed to support Unicode supplementary-plane characters in UTF-16
    /// encoding, though it is not commonly used. Format 8 is similar to format 2, in that it provides
    /// for mixed-length character codes. Instead of allowing for 8- and 16-bit character codes,
    /// however, it allows for 16- and 32-bit character codes.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 8: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-8-mixed-16-bit-and-32-bit-coverage)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 8](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format8 (platform : platform_id) = {
        /// Set to 0.
        _reserved <- reserved u16be 0,
        /// The length of the subtable in bytes (including the header)
        length <- u32be,
        /// The language ID of the subtable
        language <- cmap_language_id32 platform,
        /// Tightly packed array of bits (8K bytes total) indicating whether the particular 16-bit
        /// (index) value is the start of a 32-bit character code
        is32 <- repeat_len16 8192 u8,
        /// Number of groupings which follow
        num_groups <- u32be,
        /// Array of SequentialMapGroup records.
        groups <- repeat_len32 num_groups sequential_map_group,
    };

    /// # Format 10: Trimmed table mapping
    ///
    /// Subtable format 10 was designed to support Unicode supplementary-plane characters, though it is
    /// not commonly used. Format 10 is similar to format 6, in that it defines a trimmed array for a
    /// tight range of character codes. It differs, however, in that is uses 32-bit character codes.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 10: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-10-trimmed-array)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 10](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format10 (platform : platform_id) = {
        /// Set to 0.
        _reserved <- reserved u16be 0,
        /// The length of the subtable in bytes (including the header)
        length <- u32be,
        /// The language ID of the subtable
        language <- cmap_language_id32 platform,
        /// First character code covered
        start_char_code <- u32be,
        /// Number of character codes covered
        num_chars <- u32be,
        /// Array of glyph indices for the character codes covered
        glyph_id_array <- repeat_len32 num_chars u16be,
    };

    /// # Format 12: Segmented coverage
    ///
    /// This is the standard character-to-glyph-index mapping subtable for fonts supporting Unicode
    /// character repertoires that include supplementary-plane characters (U+10000 to U+10FFFF).
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 12: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-12-segmented-coverage)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 12](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format12 (platform : platform_id) = {
        /// Set to 0.
        _reserved <- reserved u16be 0,
        /// The length of the subtable in bytes (including the header)
        length <- u32be,
        /// The language ID of the subtable
        language <- cmap_language_id32 platform,
        /// Number of groupings which follow
        num_groups <- u32be,
        /// Array of SequentialMapGroup records.
        groups <- repeat_len32 num_groups sequential_map_group,
    };

    /// # Format 13: Many-to-one range mappings
    ///
    /// This subtable provides for situations in which the same glyph is used for hundreds or even
    /// thousands of consecutive characters spanning across multiple ranges of the code space. This
    /// subtable format may be useful for “last resort” fonts, although these fonts may use other
    /// suitable subtable formats as well.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 13: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-13-many-to-one-range-mappings)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 13](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format13 (platform : platform_id) = {
        /// Set to 0.
        _reserved <- reserved u16be 0,
        /// The length of the subtable in bytes (including the header)
        length <- u32be,
        /// The language ID of the subtable
        language <- cmap_language_id32 platform,
        /// Number of groupings which follow
        num_groups <- u32be,
        /// Array of ConstantMapGroup records.
        groups <- repeat_len32 num_groups constant_map_group,
    };

    /// # Format 14: Unicode Variation Sequences
    ///
    /// Subtable format 14 specifies the Unicode Variation Sequences (UVSes) supported by the font. A
    /// Variation Sequence, according to the Unicode Standard, comprises a base character followed by a
    /// variation selector. For example, <U+82A6, U+E0101>.
    ///
    /// This subtable format must only be used under platform ID 0 and encoding ID 5.
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Format 14: Segment mapping to delta values](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences)
    /// - [Apple's TrueType Reference Manual: `'cmap'` format 14](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_subtable_format14 (platform : platform_id) (table_start : Pos) = {
        /// The length of the subtable in bytes (including the header)
        length <- u32be,
        /// Number of variation Selector Records
        num_var_selector_records <- u32be,
        /// Array of VariationSelector records.
        var_selector <- repeat_len32 num_var_selector_records (variation_selector table_start),
    };

    /// # Character Mapping subtable
    def cmap_subtable (platform : platform_id) = {
        /// The start of the character mapping sub-table
        table_start <- stream_pos,
        /// Format number of the subtable
        format <- u16be,
        data <- match format {
            0 => cmap_subtable_format0 platform,
            2 => cmap_subtable_format2 platform,
            4 => cmap_subtable_format4 platform,
            6 => cmap_subtable_format6 platform,
            8 => cmap_subtable_format8 platform,
            10 => cmap_subtable_format10 platform,
            12 => cmap_subtable_format12 platform,
            13 => cmap_subtable_format13 platform,
            14 => cmap_subtable_format14 platform table_start,
            _ => unknown_table,
        },
    };


    /// # Encoding record
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Encoding records and encodings](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#encoding-records-and-encodings)
    def encoding_record (table_start : Pos) = {
        /// Platform identifier
        platform <- platform_id,
        /// Platform-specific encoding identifier
        encoding <- encoding_id platform,
        /// Byte offset to the subtable data
        subtable_offset <- offset32 table_start (cmap_subtable platform),
    };

    /// # Character Mapping Table (`cmap`)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: 'cmap' Header](https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#cmap-header)
    /// - [Apple's TrueType Reference Manual: The `'cmap'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6cmap.html)
    def cmap_table = {
        /// The start of the character mapping table
        table_start <- stream_pos,
        /// The version of the character
        version <- u16be,
        /// The number of encoding tables that follow
        num_tables <- u16be,
        /// An array of encoding records in the character mapping table
        encoding_records <- repeat_len16 num_tables (encoding_record table_start),
    };


    // -----------------------------------------------------------------------------

    // # General Font Information
    //
    // Global information about the font.
    //
    // ## References
    //
    // - [Microsoft's OpenType Spec: head — Font Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/head)
    // - [Apple's TrueType Reference Manual: The `'head'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6head.html)

    /// # Font Header Table (`head`)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: head — Font Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/head)
    /// - [Apple's TrueType Reference Manual: The `'head'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6head.html)
    def head_table = {
        /// Major version number of the font header table.
        major_version <- u16be where major_version == ( 1 : U16),
        /// Minor version number of the font header table.
        minor_version <- u16be, // TODO: where minor_version == 0
        /// Set by the font manufacturer.
        ///
        /// This field exists for legacy reasons - Windows ignores this, instead
        /// using the version string (ID 5) in the `name` table.
        font_revision <- fixed,
        // TODO: document computation of checksum adjustment
        checksum_adjustment <- u32be,
        /// [Magic number](https://en.wikipedia.org/wiki/File_format#Magic_number), always set to
        /// 0x5F0F3CF5
        magic_number <- u32be where magic_number == (0x5F0F3CF5 : U32),
        /// General font flags.
        ///
        // TODO: Document flags
        flags <- u16be,  // TODO: bit patterns?
        /// The granularity of the font's coordinate grid.
        units_per_em <- u16be where bool_and (units_per_em >= (16 : U16)) (units_per_em <= (16384 : U16)),
        /// The date when the font was created.
        created <- long_date_time,
        /// The date when the font was modified.
        modified <- long_date_time,
        /// Describes a bounding box that contains all glyphs in the font.
        ///
        /// Glyphs that do not contain contours should be ignored when computing
        /// these values.
        glyph_extents <- {
            /// Minimum x coordinate.
            x_min <- s16be,
            /// Minimum y coordinate.
            y_min <- s16be,
            /// Maximum x coordinate.
            x_max <- s16be,
            /// Maximum y coordinate.
            y_max <- s16be,
        },
        /// Style flags.
        ///
        /// This should agree with the `fs_selection` bits in the `OS/2` table.
        /// Note that this is ignored on Windows.
        ///
        /// | Bit   | Meaning               |
        /// | ----- | --------------------- |
        /// | 0     | bold                  |
        /// | 1     | italic                |
        /// | 2     | underline             |
        /// | 3     | outline               |
        /// | 4     | shadow                |
        /// | 5     | condensed (narrow)    |
        /// | 6     | extended              |
        /// | 7-15  | reserved (Set to `0`) |
        mac_style <- u16be, // TODO: bit patterns?
        /// Smallest readable size in pixels
        lowest_rec_ppem <- u16be,
        /// Glyph direction hint. Deprecated (set to `2`).
        ///
        /// | Value | Meaning                               |
        /// | ----- | ------------------------------------- |
        /// | `0`   | mixed directional glyphs              |
        /// | `1`   | only strongly left to right glyphs    |
        /// | `2`   | like `1` but also contains neutrals   |
        /// | `-1`  | only strongly right to left glyphs    |
        /// | `-2`  | like `-1` but also contains neutrals  |
        font_direction_hint <- deprecated s16be 2, // TODO: use an enum format?
        /// The type of offsets to use when mapping glyph indices to offsets in the
        /// file (see the `loca_table` table).
        ///
        /// | Value | Meaning                       |
        /// | ----- | ----------------------------- |
        /// | `0`   | short offsets (`offset16`)    |
        /// | `1`   | long offsets (`offset32`)     |
        index_to_loc_format <- s16be, // TODO: use an enum format?
        /// The format to use for glyph data.
        ///
        /// | Value | Meaning                       |
        /// | ----- | ----------------------------- |
        /// | `0`   | current format                |
        glyph_data_format <- s16be, // NOTE: Currently unused?
    };


    // -----------------------------------------------------------------------------

    // # Horizontal Layout Information
    //
    // Information related to fonts whose characters are written horizontally
    // (either right-to-left or left-to-right).
    //
    // ## References
    //
    // - [Microsoft's OpenType Spec: hhea — Horizontal Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea)
    // - [Apple's TrueType Reference Manual: The `'hhea'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6hhea.html)

    /// # Horizontal Header Table (`hhea`)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: hhea — Horizontal Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea)
    /// - [Apple's TrueType Reference Manual: The `'hhea'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6hhea.html)
    def hhea_table = {
        /// Major version number of the horizontal header table.
        major_version <- u16be where major_version == (1 : U16),
        /// Minor version number of the horizontal header table.
        minor_version <- u16be, // TODO: where minor_version == 0
        /// Distance from the baseline to the highest ascender.
        ascent <- fword,
        /// Distance from the baseline to the lowest descender.
        descent <- fword,
        /// The intended gap between baselines.
        line_gap <- fword,
        /// Must be consistent with horizontal metrics.
        advance_width_max <- ufword,
        /// Must be consistent with horizontal metrics.
        min_left_side_bearing <- fword,
        /// Must be consistent with horizontal metrics.
        min_right_side_bearing <- fword,
        /// `max(left_side_bearing + (x_max - x_min))`
        x_max_extent <- fword,
        /// Used to calculate the slope of the caret (rise/run).
        caret_slope <- {
            /// Set to `1` for vertical caret.
            rise <- s16be,
            /// Set to `0` for vertical caret.
            run <- s16be,
        },
        /// Set to `0` for non-slanted fonts
        caret_offset <- s16be,

        _reserved0 <- reserved s16be 0, // TODO: allow `_` as label
        _reserved1 <- reserved s16be 0, // TODO: allow `_` as label
        _reserved2 <- reserved s16be 0, // TODO: allow `_` as label
        _reserved3 <- reserved s16be 0, // TODO: allow `_` as label

        /// Set to `0` for current format.
        metric_data_format <- s16be,
        /// Number of `long_horizontal_metric` records in the in the `htmx_table`.
        number_of_long_horizontal_metrics <- u16be,
    };


    // -----------------------------------------------------------------------------

    // # Horizontal Metrics
    //
    // Information about the metrics used for horizontal layout for each of the
    // glyphs in the font.
    //
    // ## References
    //
    // - [Microsoft's OpenType Spec: hmtx — Horizontal Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx)
    // - [Apple's TrueType Reference Manual: The `'hmtx'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6hmtx.html)

    /// Horizontal metrics that provide an `advance_width`.
    def long_horizontal_metric = {
        /// Advance width, in font design units.
        advance_width <- u16be,
        /// Glyph left side bearing, in font design units.
        left_side_bearing <- s16be
    };

    /// # Horizontal Metrics Table (`hmtx`)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: hmtx — Horizontal Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx)
    /// - [Apple's TrueType Reference Manual: The `'hmtx'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6hmtx.html)
    def htmx_table (number_of_long_horizontal_metrics : U16) (num_glyphs : U16) = {
        /// Long horizontal metrics, indexed by the glyph ID.
        h_metrics <- repeat_len16 number_of_long_horizontal_metrics long_horizontal_metric,
        /// Left side bearings for glyph IDs greater than or equal to the
        /// `number_of_long_horizontal_metrics`.
        left_side_bearings <- repeat_len16 (u16_sub num_glyphs number_of_long_horizontal_metrics) s16be,
    };


    // -----------------------------------------------------------------------------

    // # Maximium Profile
    //
    // Information about the memory requirements of a font.
    //
    // ## References
    //
    // - [Microsoft's OpenType Spec: maxp — Maximum Profile](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp)
    // - [Apple's TrueType Reference Manual: The `'maxp'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6maxp.html)

    /// Fields specific to maxp version 1.0
    def maxp_version_1 = {
        /// Maximum points in non-composite glyphs.
        max_points <- u16be,
        /// Maximum contours in non-composite glyphs.
        max_contours <- u16be,
        /// Maximum points in composite glyphs.
        max_composite_points <- u16be,
        /// Maximum contours in composite glyphs.
        max_composite_contours <- u16be,
        /// Should be set to `2` in most cases.
        ///
        /// | Value | Meaning                                           |
        /// | ----- | ------------------------------------------------- |
        /// | `1`   | instructions do not use the twilight zone (Z0)    |
        /// | `2`   | instructions do use Z0                            |
        max_zones <- u16be,
        /// Maximum points used in in the twilight zone (Z0).
        max_twilight_points <- u16be,
        /// Number of Storage Area locations.
        max_storage <- u16be,
        /// Number of function definitions (FDEFs).
        max_function_defs <- u16be,
        /// Number of instruction definitions (IDEFs).
        max_instruction_defs <- u16be,
        /// Maximum stack depth across the Font Program ('fpgm' table), the
        /// Control Value Program ('prep' table), and all glyph instructions
        /// (in the 'glyf' table)
        max_stack_elements <- u16be,
        /// Maximum size in bytes used for all glyph instructions.
        max_size_of_instructions <- u16be,
        /// Maximum number of components referenced at “top level” of all
        /// composite glyphs.
        max_component_elements <- u16be,
        /// Maximum levels of recursion used when constructing compound glyphs.
        ///
        /// | Value | Meaning                                           |
        /// | ----- | ------------------------------------------------- |
        /// | `0`   | the font only contains simple glyphs              |
        /// | `1`   | compound glyphs only contain simple glyphs (there are no components within components) |
        max_component_depth <- u16be where max_component_depth <= (16 : U16),
    };


    /// # Maximium Profile Table (`maxp`)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: maxp — Maximum Profile](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp)
    /// - [Apple's TrueType Reference Manual: The `'maxp'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6maxp.html)
    def maxp_table = {
        /// The version of the table
        version <- version16dot16,
        /// The number of glyphs in the font.
        ///
        /// Both versions currently defined (0.5 and 1.0) start with num_glyphs.
        num_glyphs <- u16be,
        /// Version specific data.
        data <- match version {
            0x00010000 => maxp_version_1,
            _ => unknown_table,
        },
    };


    // -----------------------------------------------------------------------------

    // # Name storage
    //
    // Multilingual name storage for OpenType fonts.
    //
    // This describes the storage of string data for use in other areas of OpenType
    // fonts, for example in font names, family names, style names, descriptions,
    // etc.
    //
    // ## References
    //
    // - [Microsoft's OpenType Spec: name — Naming Table](https://docs.microsoft.com/en-us/typography/opentype/spec/name)
    // - [Apple's TrueType Reference Manual: The `'name'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html)

    /// Name record
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Name records](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records)
    /// - [Apple's TrueType Reference Manual: The `'name'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html)
    def name_record (storage_start : Pos) = {
        /// Platform identifier
        platform <- platform_id,
        /// Platform-specific encoding identifier
        encoding <- encoding_id platform,
        /// Language identifier
        language <- language_id,
        /// Name identifier
        ///
        /// | Value         | Meaning                               |
        /// | ------------- | ------------------------------------- |
        /// | `0`           | copyright notice                      |
        /// | `1`           | font family name                      |
        /// | `2`           | font subfamily name                   |
        /// | `3`           | unique font identification            |
        /// | `4`           | full font name                        |
        /// | `5`           | version string                        |
        /// | `6`           | PostScript name                       |
        /// | `7`           | trademark notice                      |
        /// | `8`           | manufacturer name                     |
        /// | `9`           | typeface designer name                |
        /// | `10`          | typeface description                  |
        /// | `11`          | font vendor url                       |
        /// | `12`          | font designer url                     |
        /// | `13`          | license description                   |
        /// | `14`          | license info url                      |
        /// | `15`          | reserved                              |
        /// | `16`          | typographic family name               |
        /// | `17`          | typographic subfamily name            |
        /// | `18`          | compatible full name                  |
        /// | `19`          | sample text                           |
        /// | `20`          | PostScript font name                  |
        /// | `21`          | WWS family name                       |
        /// | `22`          | WWS subfamily name                    |
        /// | `23`          | light background padefte              |
        /// | `24`          | dark background padefte               |
        /// | `25`          | variations PostScript name prefix     |
        /// | `26..<256`    | reserved                              |
        /// | `256..<32768` | font-specific names                   |
        name_id <- u16be,
        /// String length
        length <- u16be,
        /// Offset to the string data, relative to the start of the storage area
        offset <- offset16 storage_start (repeat_len16 length u8),
    };

    /// # Language tag record
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Naming table header](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-header)
    /// - [Apple's TrueType Reference Manual: The `'name'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html)
    def lang_tag_record (storage_start : Pos) = {
        /// Language tag string length
        length <- u16be,
        /// Offset to the language tag string data
        offset <- offset16 storage_start (repeat_len16 length u8),
    };

    def name_version_1 (storage_start : Pos) = {
        /// The number of language tags to expect
        lang_tag_count <- u16be,
        /// The array of language tag records
        lang_tag_records <- repeat_len16 lang_tag_count (lang_tag_record storage_start),
    };

    /// # Naming table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Naming table header](https://docs.microsoft.com/en-us/typography/opentype/spec/name#naming-table-header)
    /// - [Apple's TrueType Reference Manual: The `'name'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html)
    def name_table = {
        /// The start of the naming table
        table_start <- stream_pos,

        /// Table version
        version <- u16be,
        /// The number of `name_records` to expect
        name_count <- u16be,
        /// The offset to the string storage area, relative to the start of the naming table.
        storage_offset <- u16be,
        /// The array of name records
        name_records <- repeat_len16 name_count (name_record (pos_add_u16 table_start storage_offset)),

        /// Version specific data
        data <- match version {
            0 => {},
            1 => name_version_1 (pos_add_u16 table_start storage_offset),
            _ => unknown_table,
        },
    };

    /// # Index to location table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: `loca` table](https://docs.microsoft.com/en-us/typography/opentype/spec/loca)
    /// - [Apple's TrueType Reference Manual: The `'loca'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6loca.html)
    def loca_table (num_glyphs : U16) (index_to_loc_format : S16) = {
        offsets <- match index_to_loc_format {
            // short offsets
            0 => repeat_len16 (u16_add num_glyphs 1) u16be, // TODO Offset16
            // long offsets
            1 => repeat_len16 (u16_add num_glyphs 1) u32be, // TODO Offset32
            _ => unknown_table
        }
    };

    /// # Glyph Header
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Headers](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#glyph-headers)
    /// - [Apple's TrueType Reference Manual: The `'loca'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6glyf.html)
    def glyph_header = {
        /// If the number of contours is greater than or equal to zero, this is a simple glyph. If
        /// negative, this is a composite glyph — the value -1 should be used for composite glyphs.
        number_of_contours <- s16be,
        /// Minimum x for coordinate data.
        x_min <- s16be,
        /// Minimum y for coordinate data.
        y_min <- s16be,
        /// Maximum x for coordinate data.
        x_max <- s16be,
        /// Maximum y for coordinate data.
        y_max <- s16be,
    };

    /// # Simple glyph description
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Headers](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#simple-glyph-description)
    /// - [Apple's TrueType Reference Manual: The `'loca'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6glyf.html)
    def simple_glyph (number_of_contours : U16) = {
        /// Array of point indices for the last point of each contour, in increasing numeric order.
        end_pts_of_contours <- repeat_len16 number_of_contours u16be,
        /// Total number of bytes for instructions. If instructionLength is zero, no instructions are
        /// present for this glyph, and this field is followed directly by the flags field.
        instruction_length <- u16be,
        /// Array of instruction byte code for the glyph.
        instructions <- repeat_len16 instruction_length u8,
        let last_end_point_index = array16_index (number_of_contours - (1 : U16)) end_pts_of_contours,
        let number_of_coords = last_end_point_index + (1 : U16),
        /// Array of flag elements.
        // flags[variable] <- uint8,
        /// xCoordinates[variable] 	Contour point x-coordinates. Coordinate for the first point is relative to (0,0);
        /// others are relative to previous point.
        // or int16 <- uint8,
        /// yCoordinates[variable] 	Contour point y-coordinates. Coordinate for the first point is relative to (0,0);
        /// others are relative to previous point.
        // or int16 <- uint8,
    };

    def args_are_signed (flags : U16) =
        u16_and flags 0x0002 != (0 : U16);

    def arg_format (flags : U16) =
        match (u16_and flags 0x0001 != (0 : U16)) {
            // If the bit is set the arguments are 16-bit
            true => match (args_are_signed flags) {
                true => s16be,
                false => u16be,
            },
            // Otherwise they are 8-bit
            false => match (args_are_signed flags) {
                true => s8,
                false => u8,
            },
        };

    /// # Composite glyph description
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Headers](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf#composite-glyph-description)
    /// - [Apple's TrueType Reference Manual: The `'loca'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6glyf.html)
    def composite_glyph = {
        /// component flag
        flags <- u16be,
        /// glyph index of component
        glyphIndex <- u16be,
        /// x-offset for component or point number; type depends on bits 0 and 1 in component flags
        argument1 <- arg_format flags,
        /// y-offset for component or point number; type depends on bits 0 and 1 in component flags
        argument2 <- arg_format flags,
    };

    /// # TrueType glyph
    def glyph = {
        header <- glyph_header,
        data <- match (header.number_of_contours < (0 : S16)) {
            true => composite_glyph,
            false => simple_glyph (s16_unsigned_abs header.number_of_contours),
        }
    };

    /// # Glyph data table (TrueType)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf)
    /// - [Apple's TrueType Reference Manual: The `'glyf'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6glyf.html)
    def glyf_table (num_glyphs : U16) = {
        glyphs <- repeat_len16 num_glyphs glyph,
    };

    // -----------------------------------------------------------------------------

    /// # OS/2 Version 0
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#version-0)
    /// - [Apple's TrueType Reference Manual: The `'OS/2'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6OS2.html)
    def os2_version_0 = {
        s_typo_ascender <- s16be,
        s_typo_descender <- s16be,
        s_typo_line_gap <- s16be,
        us_win_ascent <- u16be,
        usWinDescent <- u16be,
    };

    /// # OS/2 Version 1
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#version-1)
    /// - [Apple's TrueType Reference Manual: The `'OS/2'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6OS2.html)
    def os2_version_1 = {
        version_0 <- os2_version_0,
        ul_code_page_range1 <- u32be,
        ul_code_page_range2 <- u32be,
    };

    /// # OS/2 Version 2, 3, 4
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#version-2)
    /// - [Apple's TrueType Reference Manual: The `'OS/2'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6OS2.html)
    def os2_version_2_3_4 = {
        version_1 <- os2_version_1,
        sx_height <- s16be,
        s_cap_height <- s16be,
        us_default_char <- u16be,
        us_break_char <- u16be,
        us_max_context <- u16be,
    };

    /// # OS/2 Version 5
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#version-0)
    /// - [Apple's TrueType Reference Manual: The `'OS/2'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6OS2.html)
    def os2_version_5 = {
        parent <- os2_version_2_3_4,
        usLowerOpticalPointSize <- u16be,
        usUpperOpticalPointSize <- u16be,
    };

    /// # OS/2 and Windows Metrics Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/os2)
    /// - [Apple's TrueType Reference Manual: The `'OS/2'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6OS2.html)
    def os2_table (table_length : U32) = {
        version <- u16be,
        x_avg_char_width <- s16be,
        us_weight_class <- u16be,
        us_width_class <- u16be,
        fs_type <- u16be,
        y_subscript_x_size <- s16be,
        y_subscript_y_size <- s16be,
        y_subscript_x_offset <- s16be,
        y_subscript_y_offset <- s16be,
        y_superscript_x_size <- s16be,
        y_superscript_y_size <- s16be,
        y_superscript_x_offset <- s16be,
        y_superscript_y_offset <- s16be,
        y_strikeout_size <- s16be,
        y_strikeout_position <- s16be,
        s_family_class <- s16be,
        panose <- repeat_len8 10 u8,
        ul_unicode_range1 <- u32be,
        ul_unicode_range2 <- u32be,
        ul_unicode_range3 <- u32be,
        ul_unicode_range4 <- u32be,
        ach_vend_id <- tag,
        fs_selection <- u16be,
        us_first_char_index <- u16be,
        us_last_char_index <- u16be,
        data <- match version {
            // Note: Documentation for OS/2 version 0 in Apple’s TrueType Reference Manual stops at the
            // usLastCharIndex field and does not include the last five fields of the table as it was
            // defined by Microsoft. Some legacy TrueType fonts may have been built with a shortened
            // version 0 OS/2 table. Applications should check the table length for a version 0 OS/2
            // table before reading these fields.
            0 => match (table_length >= (78 : U32)) {
                true => os2_version_0,
                false => {},
            },
            1 => os2_version_1,
            2 => os2_version_2_3_4,
            3 => os2_version_2_3_4,
            4 => os2_version_2_3_4,
            5 => os2_version_5,
            // The previous OS/2 versions are all additive. So if we encounter a newer version try
            // reading it as the newest one we know about.
            _ => os2_version_5,
        }
    };

    // -----------------------------------------------------------------------------



    /// # PostScript Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Data](https://docs.microsoft.com/en-us/typography/opentype/spec/post)
    /// - [Apple's TrueType Reference Manual: The `'post'` table](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html)
    def post_table = {
        /// 0x00010000 for version 1.0 0x00020000 for version 2.0
        /// 0x00025000 for version 2.5 (deprecated) 0x00030000 for version 3.0
        version <- version16dot16,
        /// Italic angle in counter-clockwise degrees from the vertical. Zero for upright text,
        /// negative for text that leans to the right (forward).
        italic_angle <- fixed,
        /// This is the suggested distance of the top of the underline from the baseline (negative
        /// values indicate below baseline). The PostScript definition of this FontInfo dictionary key
        /// (the y coordinate of the center of the stroke) is not used for historical reasons. The
        /// value of the PostScript key may be calculated by subtracting half the underlineThickness
        /// from the value of this field.
        underline_position <- fword,
        /// Suggested values for the underline thickness. In general, the underline thickness should
        /// match the thickness of the underscore character (U+005F LOW LINE), and should also match
        /// the strikeout thickness, which is specified in the OS/2 table.
        underline_thickness <- fword,
        /// Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally
        /// spaced (i.e. monospaced).
        is_fixed_pitch <- u32be,
        /// Minimum memory usage when an OpenType font is downloaded.
        min_mem_type42 <- u32be,
        /// Maximum memory usage when an OpenType font is downloaded.
        max_mem_type42 <- u32be,
        /// Minimum memory usage when an OpenType font is downloaded as a Type 1 font.
        min_mem_type1 <- u32be,
        /// Maximum memory usage when an OpenType font is downloaded as a Type 1 font.
        max_mem_type1 <- u32be,
        names <- match version {
            /// Version 1, standard Macintosh names
            0x00010000 => {},
            /// Version 2, non-stanard names stored in table as Pascal strings
            0x00020000 => {
                /// Number of glyphs (this should be the same as numGlyphs in 'maxp' table).
                num_glyphs <- u16be,
                /// Array of indices into the string data.
                glyph_name_index <- repeat_len16 num_glyphs u16be,
                /// Storage for the string data.
                string_data <- stream_pos,
            },
            /// Version 2.5 (deprecated), offset from standard Macintosh index
            0x00025000 => {
                /// Number of glyphs
                num_glyphs <- u16be,
                /// Difference between graphic index and standard order of glyph
                offset <- repeat_len16 num_glyphs s8,
            },
            /// Version 3, no glyph names stored in font
            ///
            /// This version is required for CFF fonts.
            0x00030000 => {},
            // Apple defines a version 4 but it's not part of OpenType and says it
            // should be avoided:
            // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html
            _ => {},
        }
    };


    // -----------------------------------------------------------------------------

    // TODO: "PCLT" .. "sbix" tables


    // -----------------------------------------------------------------------------

    // TODO: "BASE" table
    //
    // - [Microsoft's OpenType Spec: BASE — Baseline Table](https://docs.microsoft.com/en-us/typography/opentype/spec/base)

    def base_table = unknown_table;


    // -----------------------------------------------------------------------------

    // TODO: "GDEF" table
    //
    // - [Microsoft's OpenType Spec: GDEF — Glyph Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef)

    /// # Attachment Point List table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Attachment Point List table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#attachment-point-list-table)
    def attach_list = (
        /// AttachPoint table
        let attach_point_table = {
            /// Number of attachment points on this glyph
            point_count <- u16be,
            /// Array of contour point indices -in increasing numerical order
            point_indices <- repeat_len16 point_count u16be,
        };

        {
            /// The start of the AttachList table
            table_start <- stream_pos,
            /// Offset to Coverage table - from beginning of AttachList table
            coverage <- offset16 table_start coverage_table,
            /// Number of glyphs with attachment points
            glyph_count <- u16be,
            /// Array of offsets to AttachPoint tables-from beginning of AttachList table-in Coverage Index
            /// order
            attach_point_offsets <- repeat_len16 glyph_count (offset16 table_start attach_point_table),
        }
    );

    /// # Caret Value Tables
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Caret Value Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caret-value-tables)
    def caret_value = (
        /// # Caret Value Format 1
        ///
        /// ## References
        ///
        /// - [Microsoft's OpenType Spec: Caret Value Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-1)
        let caret_value_format_1 = {
            /// X or Y value, in design units
            coordinate <- s16be,
        };

        /// # CaretValue Format 2
        ///
        /// ## References
        ///
        /// - [Microsoft's OpenType Spec: Caret Value Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-2)
        let caret_value_format_2 = {
            /// Contour point index on glyph
            caret_value_point_index <- u16be,
        };

        /// # CaretValue Format 3
        ///
        /// ## References
        ///
        /// - [Microsoft's OpenType Spec: Caret Value Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-3)
        let caret_value_format_3 = fun (table_start : Pos) => {
            /// X or Y value, in design units
            coordinate <- s16be,
            /// Offset to Device table (non-variable font) / Variation Index table (variable font) for
            /// X or Y value-from beginning of CaretValue table
            table <- offset16 table_start device_or_variation_index_table,
        };

        {
            /// The start of the Caret Value table
            table_start <- stream_pos,
            /// Format identifier
            caret_value_format <- u16be,
            data <- match caret_value_format {
                1 => caret_value_format_1,
                2 => caret_value_format_2,
                3 => caret_value_format_3 table_start,
                _ => unknown_table,
            },
        }
    );

    /// # Ligature Glyph Table (LigGlyph)
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Ligature Glyph Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-glyph-table)
    def lig_glyph = {
        /// The start of the LigGlyph table
        table_start <- stream_pos,
        /// Number of CaretValue tables for this ligature (components - 1)
        caret_count <- u16be,
        /// Array of offsets to CaretValue tables, from beginning of LigGlyph table — in increasing
        /// coordinate order
        caret_values <- repeat_len16 caret_count (offset16 table_start caret_value),
    };

    /// # Ligature Caret List Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Ligature Caret List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-caret-list-table)
    def lig_caret_list = {
        /// The start of the LigCaretList table
        table_start <- stream_pos,
        /// Offset to Coverage table - from beginning of LigCaretList table
        coverage <- offset16 table_start coverage_table,
        /// Number of ligature glyphs
        lig_glyph_count <- u16be,
        /// Array of offsets to LigGlyph tables, from beginning of LigCaretList table — in Coverage
        /// Index order
        lig_glyph_offsets <- repeat_len16 lig_glyph_count (offset16 table_start lig_glyph),
    };

    /// # Mark Glyph Sets table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Mark Glyph Sets Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-glyph-sets-table)
    def mark_glyph_sets = {
        /// The start of the MarkGlyphSets table
        table_start <- stream_pos,
        /// Format identifier == 1
        format <- u16be,
        /// Number of mark glyph sets defined
        mark_glyph_set_count <- u16be,
        /// Array of offsets to mark glyph set coverage tables, from the start of the MarkGlyphSets
        /// table.
        coverage <- repeat_len16 mark_glyph_set_count (offset32 table_start coverage_table),
    };

    /// # GDEF — Glyph Definition Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: Glyph Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef)
    def gdef_table = (
        let gdef_header_version_1_2 = fun (gdef_start : Pos) => {
            /// Offset to the table of mark glyph set definitions, from beginning of GDEF header (may be
            /// NULL)
            mark_glyph_sets_def <- offset16 gdef_start mark_glyph_sets,
        };

        let gdef_header_version_1_3 = fun (gdef_start : Pos) => {
            /// Offset to the Item Variation Store table, from beginning of GDEF header (may be NULL)
            // TODO: Implement [Item Variation Store](https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store)
            item_var_store <- u32be,
        };

        {
            /// The start of the `GDEF` table
            table_start <- stream_pos,
            /// Major version of the GDEF table, = 1
            major_version <- u16be where major_version == (1 : U16),
            /// Minor version of the GDEF table
            minor_version <- u16be,
            /// Class definition table for glyph type, from beginning of GDEF header (may be NULL)
            glyph_class_def <- offset16 table_start class_def,
            /// Attachment point list table, from beginning of GDEF header (may be NULL)
            attach_list <- offset16 table_start attach_list,
            /// Ligature caret list table, from beginning of GDEF header (may be NULL)
            lig_caret_list <- offset16 table_start lig_caret_list,
            /// Class definition table for mark attachment type, from beginning of GDEF header (may be
            /// NULL)
            mark_attach_class_def <- offset16 table_start class_def,
            /// Version > 1.0 specific data
            data <- match minor_version {
                // 1.0 fields are above, shared with later versions
                0 => {},
                // 1.1 is not defined in the spec
                1 => {},
                2 => gdef_header_version_1_2 table_start,
                3 => gdef_header_version_1_3 table_start,
                // read unknown later versions as the last version we know about
                _ => gdef_header_version_1_3 table_start,
            },
        }
    );


    // -----------------------------------------------------------------------------

    /// Shared structure of GSUB and GPOS tables
    def layout_table (tag : U32) = {
        /// The start of the table
        table_start <- stream_pos,
        /// Major version of the table
        major_version <- u16be where major_version == (1 : U16),
        /// Minor version of the table
        minor_version <- u16be,
        /// ScriptList table
        script_list <- offset16 table_start script_list,
        /// FeatureList table
        feature_list <- offset16 table_start feature_list,
        /// LookupList table
        lookup_list <- offset16 table_start (lookup_list tag),
        // TODO: fields from GPOS/GSUB version 1.1 (variable fonts)
    };

    // TODO: "GPOS" table
    //
    // - [Microsoft's OpenType Spec: GPOS — Glyph Positioning Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos)

    def gpos_table = layout_table "GPOS";


    // -----------------------------------------------------------------------------

    /// # GSUB — Glyph Substitution Table
    ///
    /// ## References
    ///
    /// - [Microsoft's OpenType Spec: GSUB — Glyph Substitution Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gsub)
    def gsub_table = layout_table "GSUB";


    // -----------------------------------------------------------------------------

    // TODO: "JSTF" table
    //
    // - [Microsoft's OpenType Spec: JSTF — Justification Table](https://docs.microsoft.com/en-us/typography/opentype/spec/jstf)

    def jstf_table = unknown_table;


    // -----------------------------------------------------------------------------

    // TODO: "MATH" table
    //
    // - [Microsoft's OpenType Spec: MATH - The Mathematical Typesetting Table](https://docs.microsoft.com/en-us/typography/opentype/spec/math)

    def math_table = unknown_table;


    // -----------------------------------------------------------------------------

    // TODO: "avar" .. "vmtx" tables
*/
