use doodle::numeric::core::MachineRep;
use doodle::numeric::helper as num;
use doodle::{
    BaseType, DepFormat, Expr, Format, FormatModule, FormatRef, Label, Pattern, ValueType,
    ViewExpr, bounds::Bounds, helper::*,
};

mod util {
    use super::*;
    use doodle::IntoLabel;
    #[cfg(feature = "alt")]
    use doodle::alt::FormatExt;

    pub(crate) fn id<T>(x: T) -> T {
        x
    }

    /// Given an Expr `seq` that represents a sequence of N+1 members, constructs a Format that parses a dependent
    /// format `dep_format` N times, iterating over each pair of adjacent elements in the sequence as input.
    ///
    /// # Notes
    ///
    /// The `premap` function-pair is used to transform each of the two elements in the pair before applying `dep_format`, in case
    /// there is a transformation we wish to perform without having to map over `seq`. The first function applies to the first member
    /// of each given pair, and likewise the second function to the second member.
    ///
    /// The `labels` parameter specifies what variable-identifiers the first and second member of each pair should be bound to in
    /// in order that they are properly in-scope when processing `dep_format`. These names must be distinct, and must not conflict with any
    /// other higher-scoped bindings we want to avoid shadowing when processing `dep_format`.
    pub(crate) fn for_each_pair(
        seq: Expr,
        premap: (impl FnOnce(Expr) -> Expr, impl FnOnce(Expr) -> Expr),
        labels: [&'static str; 2],
        dep_format: Format,
    ) -> Format {
        Format::Let(
            Label::Borrowed("len"),
            Box::new(pred(seq_length(seq.clone()))),
            Box::new(for_each(
                enum_from_to(Expr::U32(0), var("len")),
                "ix",
                with_tuple(
                    Expr::Tuple(vec![
                        premap.0(index_unchecked(seq.clone(), var("ix"))),
                        premap.1(index_unchecked(seq.clone(), succ(var("ix")))),
                    ]),
                    labels,
                    dep_format,
                ),
            )),
        )
    }

    /// Marker-type for controlling how records-with-alternation are composed
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
    pub(crate) enum NestingKind {
        #[default]
        /// `MinimalVariation`: Fields that may alternate are extracted into their own enum
        MinimalVariation,
        /// `UnifiedRecord`: Common fields and non-common fields are distributively unified into a single record, for each possible variant
        UnifiedRecord,
    }

    /// Helper function for generically constructing a Format that consists of a
    ///   set of invariant fields, a discriminant field, and an alternation over
    ///   exactly one inhabited sub-format based on the value of the discriminant.
    ///
    /// - `outer_fields` is a list of the `(name, format)` pairs that are invariant and precede
    ///   the dependent field-set.
    /// - `discriminant` consists of the name of the discriminant field and its sole value.
    ///   The field-name in question must be present in `outer_fields`, and this function
    ///   will panic if it is missing.
    /// - `inner_fields` is the list of fields that belong to the sub-format dependent on `discriminant`.
    /// - `intermediate` is the name of the field that will be used to hold the `inner_fields` ADT when
    ///   not flattened (see [`NestingKind`])
    /// - `variant_name` is the constructor-name for the sole variant of the enum that holds the `inner_fields` record
    /// - `nesting_kind` is a template-selector that controls what shape of Format is returned.
    ///
    /// The shape of the format is determined based on `nesting_kind`:
    ///
    /// - `MinimalVariation` produces a record containing the `outer_fields` and a single field storing an enum-value that encompasses the `inner_fields`, making use of `intermediate` and `variant_name`
    /// - `UnifiedRecord` ignores those fields and instead constructs a single flattened record, concatenating `outer_fields` and `inner_fields`
    ///   in the expected order, and wrapping the discriminant-field in a `Format::Where` context that ensures that the
    ///   field in question has the appropriate value.
    ///
    /// # Panics
    ///
    /// Will panic if `discriminant` specifies a field-name that is not present in `outer_fields`.
    pub(crate) fn embedded_singleton_alternation<const OUTER: usize, const INNER: usize>(
        outer_fields: [(&'static str, Format); OUTER],
        discriminant: (&'static str, u16),
        inner_fields: [(&'static str, Format); INNER],
        intermediate: &'static str,
        variant_name: &'static str,
        nesting_kind: NestingKind,
    ) -> Format {
        let (disc_field, disc_value) = discriminant;
        let accum = match nesting_kind {
            NestingKind::MinimalVariation => {
                // REVIEW - it is not necessarily obvious that all UnifiedRecord defs can be changed to MinimalVariation versions if they refer to variables in the outer record, but it seems plausible at least
                let mut has_discriminant = false;
                let record_inner = record_auto(inner_fields);
                let mut accum = Vec::with_capacity(OUTER + 1);
                for (name, format) in outer_fields {
                    has_discriminant = has_discriminant || name == disc_field;
                    accum.push((Label::Borrowed(name), format));
                }
                accum.push((
                    Label::Borrowed(intermediate),
                    match_variant(
                        var(disc_field),
                        [
                            (Pattern::U16(disc_value), variant_name, record_inner),
                            // REVIEW - we could technically add an explicit catch-all but it might be simpler to leave it as an implicit unhandled case
                        ],
                    ),
                ));
                assert!(
                    has_discriminant,
                    "missing discriminant field `{disc_field}` in outer-field set"
                );
                accum
            }
            NestingKind::UnifiedRecord => {
                let mut accum = Vec::with_capacity(OUTER + INNER);
                for (name, format) in outer_fields {
                    if name == disc_field {
                        accum.push((
                            Label::Borrowed(name),
                            where_lambda(format, name, expr_eq(var(name), Expr::U16(disc_value))),
                        ));
                    } else {
                        accum.push((Label::Borrowed(name), format));
                    }
                }
                for (name, format) in inner_fields {
                    accum.push((Label::Borrowed(name), format));
                }
                accum
            }
        };
        record_auto(accum)
    }

    /// Helper function to generically construct a record-union over a set of common fields (including an unsigned integer discriminant)
    /// and a fixed-size set of variadic continuation-fields chosen based on the discriminant value.
    ///
    /// - `shared_fields` is a list of the `(name, format)` pairs that are invariant and precede the dependent field-set.
    /// - `discriminant` is the label (in `outer_fields`) corresponding to a numeric-kinded field that will be used as a discriminant for selecting the branch-specific fields.
    /// - `branches` is a list of `(value, variant-name, inner-fields)` 3-tuples that are the (assumed-)exhaustive set of valid sub-format extensions for each legal value of `discriminant`. Any
    ///   value of `discriminant` that is not present in `branches` will be handled as a parse-failure.
    /// - `intermediate` is the name of the field holding the ADT-value that stores the discriminant-specific fields, when `nesting_kind` is `MinimalVariation` (in `UnifiedRecord`, this argument is ignored)
    /// - `nesting_kind` is a template-selector that controls what shape of Format is returned.
    ///
    /// The shape of the format is determined based on `nesting_kind`:
    ///
    /// - `MinimalVariation` produces a record-format consisting of `shared_fields` as well as a final field (named by `intermediate`) for a value of the enum storing the inner-fields of `branches` one-to-one
    /// - `UnifiedRecord` produces an enum each of whose members is the fusion of `shared_fields` and the respective inner-fields of `branches`, using parse-lookahead to determine the discriminant before
    ///   parsing any fields destructively.
    ///
    /// # Panics
    ///
    /// Will panic if `discriminant` specifies a field-name that is not present in `outer_fields`.
    pub(crate) fn embedded_variadic_alternation<D, C, const OUTER: usize, const BRANCHES: usize>(
        shared_fields: [(&'static str, Format); OUTER],
        discriminant: &'static str,
        branches: [(D, &'static str, C); BRANCHES],
        intermediate: &'static str,
        nesting_kind: NestingKind,
    ) -> Format
    where
        D: Into<Bounds>,
        C: IntoIterator<Item = (&'static str, Format), IntoIter: DoubleEndedIterator>,
    {
        match nesting_kind {
            NestingKind::MinimalVariation => {
                let mut pat_branches = Vec::with_capacity(BRANCHES);
                for (value, vname, c) in branches.into_iter() {
                    let record_inner = record(c);
                    pat_branches.push((Pattern::Int(value.into()), vname, record_inner));
                }
                let final_field = (intermediate, match_variant(var(discriminant), pat_branches));
                let mut has_discriminant = false;
                let mut accum = Vec::with_capacity(OUTER + 1);
                for (name, format) in shared_fields {
                    has_discriminant = has_discriminant || name == discriminant;
                    accum.push((name, format));
                }
                accum.push(final_field);
                assert!(
                    has_discriminant,
                    "missing discriminant field `{discriminant}` in outer-field set"
                );
                record(accum)
            }
            NestingKind::UnifiedRecord => {
                let mut field_prefix = Vec::with_capacity(OUTER);
                let mut has_discriminant = false;
                for (name, format) in shared_fields.iter() {
                    field_prefix.push((Label::Borrowed(name), format.clone()));
                    if *name == discriminant {
                        has_discriminant = true;
                        break;
                    }
                }
                assert!(
                    has_discriminant,
                    "missing discriminant field `{discriminant}` in outer-field set"
                );
                let mut pat_branches = Vec::with_capacity(BRANCHES);
                for (value, vname, c) in branches {
                    let unified = Iterator::chain(shared_fields.iter().cloned(), c.into_iter())
                        .collect::<Vec<(&'static str, Format)>>();
                    let record_inner = record(unified);
                    pat_branches.push((Pattern::Int(value.into()), vname, record_inner));
                }
                peek_field_then(
                    field_prefix.as_slice(),
                    match_variant(var(discriminant), pat_branches),
                )
            }
        }
    }

    /// Helper function for parsing two bytes of binary data as a big-endian packed-bits record, with the MSB as a flag-bit
    /// with the specified identifier `flag_name` and the remaining 15 bits as a `u16`-kinded numeric field with the specified identifier `field_name`.
    pub(crate) fn hi_flag_u15be(flag_name: &'static str, field_name: &'static str) -> Format {
        bit_fields_u16([
            BitFieldKind::FlagBit(flag_name),
            BitFieldKind::BitsField {
                field_name,
                bit_width: 15,
            },
        ])
    }

    /// Extracts the final element of a sequence-Expr provided that it is non-empty.
    ///
    /// If the sequence is empty, the behavior is unspecified.
    pub(crate) fn last_elem(seq: Expr) -> Expr {
        let last_ix = pred(seq_length(seq.clone()));
        index_unchecked(seq, last_ix)
    }

    /// Helper function to handle the fact that though vmtx only appears alongside vhea, both are optional tables
    /// so direct record projection is not possible (as vhea will be an option-wrapped record)
    pub(crate) fn vhea_long_metrics(vhea: Expr) -> Expr {
        record_proj(expr_unwrap(vhea), "number_of_long_metrics")
    }

    /// Attempts to index on the `offsets` key of a `loca` table through an option-unpacking indirection.
    ///
    /// Helper function to handle the fact that though loca only appears alongside glyf, both are optional tables
    pub(crate) fn loca_offsets(loca: Expr) -> Expr {
        let f = |loca_table: Expr| record_proj(loca_table, "offsets");
        let loca_empty = variant("Offsets32", seq_empty());
        expr_option_map_or(loca_empty, f, loca)
    }

    /// Doubles a `U16`-kinded Expr into a `U32`-kinded output.
    pub(crate) fn scale2(half: Expr) -> Expr {
        mul(as_u32(half), Expr::U32(2))
    }

    /// Given a bit-flag `is_positive` (`1` for positive, `0` for negative),
    /// parses a `u8` value and maps it into an expression of type `i16` with the magnitude of the original `u8` value and the appropriate signedness.
    pub(crate) fn parse_u8_to_i16(is_positive: Expr) -> Format {
        use doodle::numeric::BasicUnaryOp;
        if_then_else(
            is_positive,
            map_numeric(u8(), |v| num::cast(MachineRep::I16, v)),
            map_numeric(u8(), |v| {
                num::unary_with_rep(BasicUnaryOp::Negate, Some(MachineRep::I16), v)
            }),
        )
    }

    /// Parses a u32 serving as the de-facto representation of a signed, 16.16 bit fixed-point number
    pub(crate) fn fixed32be() -> Format {
        fmt_variant("Fixed32", u32be())
    }

    // Custom type for fixed-point values that are interpreted as (2bits . 14bits) within a u16be raw-parse
    pub(crate) fn f2dot14() -> Format {
        fmt_variant("F2Dot14", u16be())
    }

    /// Helper function for parsing a big-endian u24 (3-byte) value
    pub(crate) fn u24be() -> Format {
        // REVIEW - should U24Be be a CommonOp?
        map(
            Format::Tuple(vec![compute(Expr::U8(0)), u8(), u8(), u8()]),
            lambda("x", Expr::U32Be(Box::new(var("x")))),
        )
    }

    /// Helper-function that parses a big-endian 32-bit value meant to be interpreted as a `(u16, u16)` value-pair (e.g. major, minor version fields)
    pub(crate) fn version16_16() -> Format {
        u32be()
    }

    /// Helper function for compile-time conversion of b"..." literals into u32 (big-endian) values.
    pub(crate) const fn magic(tag: &'static [u8; 4]) -> u32 {
        u32::from_be_bytes(*tag)
    }

    /// Parses a `U16Be` value that is expected to be equal to `val`
    ///
    /// Raises warnings, not errors, if the value does not match.
    pub(crate) fn expect_u16be(val: u16) -> Format {
        expect_lambda(u16be(), "x", expr_eq(var("x"), Expr::U16(val)))
    }

    /// Parses a `U16Be` value that is expected to be equal to one of the values in `vals`
    ///
    /// If only one value is expected, use [`expect_u16be`] instead.
    ///
    /// Raises warnings, not errors, if the value does not match any of the provided cases.
    pub(crate) fn expects_u16be<const N: usize>(vals: [u16; N]) -> Format {
        expect_lambda(
            u16be(),
            "x",
            expr_match(
                var("x"),
                // REVIEW - do we want to introduce pattern-OR to simplify the expression?
                vals.into_iter()
                    .map(|v| (Pattern::U16(v), Expr::Bool(true)))
                    .chain(std::iter::once((Pattern::Wildcard, Expr::Bool(false)))),
            ),
        )
    }

    /// Given a logical record-structure (or common-prefix of a union-of-records),
    /// speculatively parses the shortest prefix of fields including a field we wish to
    /// know the value of in advance of committing to a particular parse; using the value of
    /// said field under its original in-record identifier binding, we then parse `dep_format`.
    ///
    /// # NOTES
    ///
    /// Every field except for the final field in `field_prefix` are discarded immediately after they are parsed,
    /// and only the value of the final field is retained in scope for the parse of `dep_format`.
    ///
    /// Because the lookahead-parse of `field_prefix` is unrelated to the final record-structure of `dep_format`,
    /// the field-labels in `field_prefix` can be chosen freely. In most cases, the field-names of the record-structure
    /// we are ultimately parsing will typically be the most sensible choice.
    pub(crate) fn peek_field_then<Name>(
        field_prefix: &[(Name, Format)],
        dep_format: Format,
    ) -> Format
    where
        Name: IntoLabel + Clone + AsRef<str>,
    {
        let Some(((field_name, field_format), init)) = field_prefix.split_last() else {
            panic!("field_prefix must be non-empty")
        };

        chain(
            Format::Peek(Box::new(pseudo_record(
                init.iter().cloned(),
                field_format.clone(),
            ))),
            field_name.clone(),
            dep_format,
        )
    }

    /// Specialized format-construction designed for supporting `cmap` and `kern` sub-tables.
    ///
    /// Speculatively peeks the shortest prefix of fields required to witness a field with the
    /// indicated label (`length_field`), which is interpreted as a positive integer byte-length
    /// constraining the entire record (and not just subsequent fields); this value is extracted
    /// and used as the constraining size for a slice around parsing the complete record.
    ///
    /// Handles the construction of the record format from the given fields, which are provided
    /// in a raw form to allow for ease of introspection.
    pub(crate) fn slice_record<Name, const N: usize>(
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

        peek_field_then(&prefix[..], slice(var(length_field), record_auto(full)))
    }

    /// Computes the maximum value of `x / 8` for `x: U16` in seq (return value wrapped in Option to handle empty list)
    ///
    /// Used exclusively for `cmap` subtable format 2.
    pub(crate) fn subheader_index(seq: Expr) -> Expr {
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
            ValueType::option(ValueType::U16),
            seq,
        ))
    }

    /// Template ViewExpr that represents the entire contents of an OTF file buffer
    pub(crate) const FONTVIEW_VAR: ViewExpr = ViewExpr::Var(FONTVIEW_LBL);

    /// Template identifier for the View representing the entire contents of an OTF file buffer
    pub(crate) const FONTVIEW_LBL: Label = Label::Borrowed("font_view");

    /// Default identifier for the per-table local View for a given table (whether a component table or sub-format thereof)
    pub(crate) const TABLE_VIEW: Label = Label::Borrowed("table_view");

    /// Given a  a 32-bit opentype tag-value `query_table_id`,
    /// applies `dep_format` to the `Option<T>`-kinded `Expr` yielded by a binary search over
    /// `table_records` for a table with the given tag.
    ///
    /// # Notes
    ///
    /// When constructing the `dep_format` closure, callers should be aware that the `Expr`
    /// parameter it accepts will implicitly have the ValueType `Option<opentype_table_record>`,
    /// where `table_records` has ValueType `Seq<opentype_table_record>`.
    ///
    /// As the search is hardcoded to be binary, this method should only be called when the
    /// only cases where `table_records` might be unsorted are deemed definitionally invalid
    /// OpenType streams.
    ///
    /// Care should also be taken that only OpenType streams are parsed to the point where
    /// this function's output would be parsed, and that any non-OpenType streams are filtered
    /// out by that point (either as a result of delaying OpenType alternatives until very few
    /// formats remain, or precluding invalid streams via parse-level invariants such as magic
    /// bytes).
    pub(crate) fn with_table(
        table_records: Expr,
        query_table_id: u32,
        dep_format: impl FnOnce(Expr) -> Format,
    ) -> Format {
        // Not all fonts are actually sorted: https://github.com/harfbuzz/harfbuzz/issues/3065
        // NOTE - while technically, we could refactor to make the sortedness a runtime-dependant parameter and check (once) whether the directory is sorted, this may yield only marginal benefits
        const TABLE_RECORDS_ARE_SORTED: bool = false;
        let f_get_table_id = |table_record: Expr| record_proj(table_record, "table_id");
        let opt_match = find_by_key(
            TABLE_RECORDS_ARE_SORTED,
            f_get_table_id,
            Expr::U32(query_table_id),
            table_records,
        );
        dep_format(opt_match)
    }

    /// Scaffolding aid for migration from Pos-arithmetic model to ViewFormat model
    ///
    /// Given a ViewExpr `base_view` that stands in for the base-position for offset arithmetic,
    /// a possibly-zero `offset`, and a Format `format`, constructs a Format that
    /// parses `format` at the location `base_view + offset` iff `offset` is non-zero, and returns
    /// None otherwise (meaning that non-zero offset parses are wrapped in Some).
    // TODO[epic=eager-view-parse] - This should be phased out in favor of less-eager ViewFormat processing.
    pub(crate) fn parse_view_offset<K: ZeroMarker>(
        base_view: ViewExpr,
        offset: Expr,
        format: Format,
    ) -> Format {
        cond_maybe(
            is_nonzero::<K>(offset.clone()),
            parse_from_view(base_view.offset(offset), format),
        )
    }

    /// Given a ViewExpr `base_view` that stands in for the base-position for offset arithmetic,
    /// an `offset` expression, and a Format `format`, constructs a Format that
    /// parses `format` at the location `base_view + offset`, without any decoration and without
    /// wrapping in `Option`.
    ///
    /// # NOTES
    ///
    /// Unlike `parse_view_offset`, this method does not wrap the result in an Option, and instead
    /// returns the result directly. This may produce nonsensical values or parse-failures if the
    /// offset is zero, or otherwise does not directly correspond to the true offset of a valid parse
    /// of `format`.
    pub(crate) fn parse_view_offset_mandatory(
        base_view: ViewExpr,
        offset: Expr,
        format: Format,
    ) -> Format {
        parse_from_view(base_view.offset(offset), format)
    }

    /// Parses a u16be offset and captures `nbytes` bytes starting at that offset (relative to `view`).
    ///
    /// Returns a record `{ offset: u16, data: &[u8] }`
    pub(crate) fn capture_bytes_view_offset16(view: ViewExpr, nbytes: Expr) -> Format {
        record([
            ("offset", u16be()),
            (
                "data",
                with_view(view.offset(var("offset")), capture_bytes(nbytes)),
            ),
        ])
    }

    /// Record-format that reads (and stores) a u16be offset, along with a field `_data` for the phantom-parse of `format` at that offset (relative to `view`).
    pub(crate) fn read_phantom_view_offset16(view: ViewExpr, format: Format) -> Format {
        record_auto([
            // TODO: rename "offset" -> "value" and ensure all calling-fields have 'offset' in field-identifier
            ("offset", u16be()),
            (
                "#_data",
                phantom(parse_view_offset::<U16>(view, var("offset"), format)),
            ),
        ])
    }

    /// Record-format that reads (and stores) a u32be offset, along with a field `_data` for the phantom-parse of `format` at that offset (relative to `view`).
    pub(crate) fn read_phantom_view_offset32(view: ViewExpr, format: Format) -> Format {
        record_auto([
            // TODO: rename "offset" -> "value" and ensure all calling-fields have 'offset' in field-identifier
            ("offset", u32be()),
            (
                "#_data",
                phantom(parse_view_offset::<U32>(view, var("offset"), format)),
            ),
        ])
    }

    /// Record-format that reads (and stores) a u32be offset, along with a field `data` for the strict parse of `format` at that offset (relative to `view`).
    ///
    /// # NOTES
    ///
    /// This is a non-phantom version of `read_phantom_view_offset32` whose intended purpose is to simplify
    /// model-migration by keeping the highest-level font-formats free of phantom-parses. Ideally, this will be
    /// phased out, or the intended processing model otherwise clarified to determine where and when phantom-parses
    /// are demanded.
    pub(crate) fn read_view_offset32(view: ViewExpr, format: Format) -> Format {
        record_auto([
            // TODO: rename "offset" -> "value" and ensure all calling-fields have 'offset' in field-identifier
            ("offset", u32be()),
            (
                "data",
                parse_view_offset::<U32>(view, var("offset"), format),
            ),
        ])
    }
    /// Reads a U16Be offset value and conditionally applies `format` to the location found at that offset
    /// relative to `view`, depending on the processing model.
    #[cfg(feature = "alt")]
    pub(crate) fn alt_read_u16be_view_offset(view: ViewExpr, format: Format) -> FormatExt {
        use doodle::alt::MetaFormat;
        let base_model = Box::new(FormatExt::from(chain(
            u16be(),
            "offset",
            parse_view_offset(view.clone(), var("offset"), format.clone()),
        )));
        let alt_model = Box::new(FormatExt::from(read_phantom_view_offset16(view, format)));
        FormatExt::Meta(MetaFormat::EngineSpecific {
            base_model,
            alt_model,
        })
    }

    /// Produces a `Format` that evaluates the delayed parse of `read_phantom_view_offset{16|32}`.
    ///
    /// Takes the original Format and the correct `ViewExpr` to parse relative to.
    ///
    /// # Notes
    ///
    /// Returns a `Format @ Option<T>` where `format :~ T`
    pub(crate) fn get_content_at_offset<K: ZeroMarker>(
        orig_view: ViewExpr,
        offset_record: Expr,
        format: Format,
    ) -> Format {
        let offset = record_proj(offset_record, "offset");
        cond_maybe(
            is_nonzero::<K>(offset.clone()),
            parse_from_view(orig_view.offset(offset), format),
        )
    }
}
use util::*;

/// Flag-value used in `head` table to mark `loca` offsets as being 16-bit
const SHORT_OFFSET16: u16 = 0;
/// Flag-value used in `head` table to mark `loca` offsets as being 32-bit
const LONG_OFFSET32: u16 = 1;

pub(crate) fn table_links(
    module: &mut FormatModule,
    tag: FormatRef,
    table_type: ValueType,
) -> FormatRef {
    // character mapping table
    let cmap_table = cmap::table(module);
    let head_table = head::table(module);
    let hhea_table = hhea::table(module);
    let vhea_table = vhea::table(module);
    let maxp_table = maxp::table(module);
    let hmtx_table = hmtx::table(module);
    let vmtx_table = vmtx::table(module);
    let name_table = name::table(module);
    let os2_table = os2::table(module, tag);
    let post_table = post::table(module);
    let cvt_table = cvt::table(module);
    let fpgm_table = fpgm::table(module);
    let loca_table = loca::table(module);
    let glyf_table = glyf::table(module);
    let prep_table = prep::table(module);
    let gasp_table = gasp::table(module);
    let class_def = common::class_def(module);
    let coverage_table = common::coverage_table(module);
    let device_or_variation_index_table = common::device_or_variation_index_table(module);
    let item_variation_store = common::item_variation_store(module);
    let gdef_table = gdef::table(
        module,
        class_def,
        coverage_table,
        device_or_variation_index_table,
        item_variation_store,
    );
    // SECTION - bulk common definitions for GSUB and GPOS
    let value_format_flags = layout::value_format_flags(module);
    let vf_flags_type = module
        .get_format_type(value_format_flags.get_level())
        .clone();
    let value_record = layout::value_record(module, device_or_variation_index_table, vf_flags_type);
    let anchor_table = layout::anchor_table(module, device_or_variation_index_table);
    let lang_sys = layout::lang_sys(module);
    let script_table = layout::script_table(module, tag, lang_sys);
    let script_list = layout::script_list(module, tag, script_table);
    let feature_table = layout::feature_table(module);
    let feature_list = layout::feature_list(module, tag, feature_table);
    let sequence_lookup_record = layout::sequence_lookup_record(module);
    // Sub-tables used by both GSUB and GPOS
    let sequence_context =
        layout::sequence_context(module, class_def, coverage_table, sequence_lookup_record);
    let chained_sequence_context =
        layout::chained_sequence_context(module, class_def, coverage_table, sequence_lookup_record);
    // !SECTION
    // SECTION - high-level definitions to support GSUB and GPOS
    let ground_subst = gsub::ground_subst(
        module,
        coverage_table,
        sequence_context,
        chained_sequence_context,
    );
    let subst_extension = gsub::subst_extension(module, ground_subst);
    let ground_pos = gpos::ground_pos(
        module,
        class_def,
        coverage_table,
        value_format_flags,
        value_record,
        anchor_table,
        sequence_context,
        chained_sequence_context,
    );
    let pos_extension = gpos::pos_extension(module, ground_pos);
    let feature_variations = layout::feature_variations(module, feature_table);
    // !SECTION
    // REVIEW - we might consider rewriting `layout::table` to spin off `gpos::table` and `gsub::table` more easily (self-contained)
    let gpos_table = gpos::table(
        module,
        script_list,
        feature_list,
        ground_pos,
        pos_extension,
        feature_variations,
    );
    let gsub_table = gsub::table(
        module,
        script_list,
        feature_list,
        ground_subst,
        subst_extension,
        feature_variations,
    );
    let base_table = base::table(
        module,
        tag,
        device_or_variation_index_table,
        item_variation_store,
    );
    let kern_table = kern::table(module);
    let stat_table = stat::table(module, tag);
    let avar_table = avar::table(module);
    let fvar_table = fvar::table(module, tag);
    let gvar_table = gvar::table(module);
    let hvar_table = hvar::table(module, item_variation_store);

    let dsig_table = dsig::table(module);
    let hdmx_table = hdmx::table(module);
    let vdmx_table = vdmx::table(module);

    module.define_format_args_views(
        "opentype.table_directory.table_links",
        vec![(
            Label::Borrowed("tables"),
            ValueType::Seq(Box::new(table_type)),
        )],
        vec![FONTVIEW_LBL],
        record_auto([
            (
                "cmap",
                required_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"cmap"),
                    cmap_table.call(),
                ),
            ),
            (
                "head",
                required_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"head"),
                    head_table.call(),
                ),
            ),
            (
                "hhea",
                required_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"hhea"),
                    hhea_table.call(),
                ),
            ),
            (
                "maxp",
                required_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"maxp"),
                    maxp_table.call(),
                ),
            ),
            (
                "hmtx",
                required_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"hmtx"),
                    hmtx_table.call_args(vec![
                        record_proj(var("hhea"), "number_of_long_metrics"),
                        record_proj(var("maxp"), "num_glyphs"),
                    ]),
                ),
            ),
            (
                "name",
                required_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"name"),
                    name_table.call(),
                ),
            ),
            (
                "os2",
                required_table_with_len(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"OS/2"),
                    os2_table,
                ),
            ),
            (
                "post",
                required_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"post"),
                    post_table.call(),
                ),
            ),
            // SECTION - TrueType Outline
            (
                "cvt",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"cvt "),
                    cvt_table,
                ),
            ),
            (
                "fpgm",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"fpgm"),
                    fpgm_table,
                ),
            ),
            (
                "loca",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"loca"),
                    loca_table.call_args(vec![
                        record_proj(var("maxp"), "num_glyphs"),
                        record_proj(var("head"), "index_to_loc_format"),
                    ]),
                ),
            ),
            (
                "glyf",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"glyf"),
                    glyf_table.call_args(vec![loca_offsets(var("loca"))]),
                ),
            ),
            (
                "prep",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"prep"),
                    prep_table,
                ),
            ),
            (
                "gasp",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"gasp"),
                    gasp_table.call(),
                ),
            ),
            // !SECTION
            // SECTION - CFF Outline
            // TODO - `CFF ` deferred for reasons of complexity
            // TODO - `CFF2` deferred for reasons of complexity
            // TODO - `VORG` deferred as it collocates with CFF{ ,2}
            // !SECTION
            // SECTION - SVG Outline
            // FIXME - `SVG ` postponed due to rarity (15 of 659 tested fonts)
            // !SECTION
            // SECTION - Bitmap Glyphs
            // FIXME - `EBDT` postponed due to rarity (15 of 659 tested fonts)
            // FIXME - `EBLC` postponed due to rarity (15 of 659 tested fonts)
            // FIXME - `EBSC` postponed due to rarity (no occurrences among 659 tested fonts)
            // FIXME - `CBDT` postponed due to rarity (2 of 659 tested fonts)
            // FIXME - `CBLC` postponed due to rarity (2 of 659 tested fonts)
            // FIXME - `sbix` postponed due to rarity (1 of 659 tested fonts)
            // !SECTION
            // SECTION - Advanced Typography
            (
                "base",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"BASE"),
                    base_table.call(),
                ),
            ),
            (
                "gdef",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"GDEF"),
                    gdef_table.call(),
                ),
            ),
            (
                "gpos",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"GPOS"),
                    gpos_table.call(),
                ),
            ),
            (
                "gsub",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"GSUB"),
                    gsub_table.call(),
                ),
            ),
            // !SECTION
            // STUB - add more table sections
            // SECTION - Font Variations
            // STUB - add more tables
            (
                "avar",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"avar"),
                    avar_table.call(),
                ),
            ),
            (
                "fvar",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"fvar"),
                    fvar_table.call(),
                ),
            ),
            (
                "gvar",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"gvar"),
                    gvar_table.call(),
                ),
            ),
            (
                "hvar",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"HVAR"),
                    hvar_table.call(),
                ),
            ),
            // !SECTION
            // STUB - add more table sections
            // SECTION - other tables
            // STUB - add more tables
            (
                "kern",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"kern"),
                    kern_table.call(),
                ),
            ),
            (
                "stat",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"STAT"),
                    stat_table.call(),
                ),
            ),
            (
                "vhea",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"vhea"),
                    vhea_table.call(),
                ),
            ),
            (
                "vmtx",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"vmtx"),
                    vmtx_table.call_args(vec![
                        vhea_long_metrics(var("vhea")),
                        record_proj(var("maxp"), "num_glyphs"),
                    ]),
                ),
            ),
            (
                "dsig",
                permit(
                    optional_table(
                        util::FONTVIEW_VAR,
                        var("tables"),
                        util::magic(b"DSIG"),
                        dsig_table.call(),
                    ),
                    expr_none(),
                ),
            ),
            (
                "hdmx",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"hdmx"),
                    hdmx_table.invoke_args([record_proj(var("maxp"), "num_glyphs")]),
                ),
            ),
            (
                "vdmx",
                optional_table(
                    util::FONTVIEW_VAR,
                    var("tables"),
                    util::magic(b"VDMX"),
                    vdmx_table.call(),
                ),
            ),
            // ANCHOR - table frontier
            // !SECTION
            ("__skip", Format::SkipRemainder),
        ]),
    )
}

// ANCHOR[epic=main-fn]
pub fn main(module: &mut FormatModule) -> FormatRef {
    // NOTE - Microsoft defines a tag as consisting on printable ascii characters in the range 0x20 -- 0x7E (inclusive), but some vendors are non-standard so we accept anything
    let tag = opentype_tag(module);

    let table_record = module.define_format(
        "opentype.table_record",
        record([
            ("table_id", tag.call()), // should be ascending within the repetition "table_records" field in table_directory
            ("checksum", u32be()),
            ("offset", u32be()),
            ("length", u32be()),
        ]),
    );

    let table_type = module.get_format_type(table_record.get_level()).clone();

    // let stub_table = module.define_format("opentype.table_stub", Format::EMPTY);

    let table_links = table_links(module, tag, table_type);

    let table_directory = module.define_format_views(
        "opentype.table_directory",
        vec![FONTVIEW_LBL],
        record([
            (
                "sfnt_version",
                where_lambda(
                    u32be(),
                    "version",
                    expr_match(
                        var("version"),
                        [
                            (Pattern::U32(0x0001_0000), Expr::Bool(true)),
                            (Pattern::U32(util::magic(b"OTTO")), Expr::Bool(true)),
                            (Pattern::U32(util::magic(b"true")), Expr::Bool(true)),
                            (Pattern::Wildcard, Expr::Bool(false)),
                        ],
                    ),
                ),
            ),
            ("num_tables", u16be()),     // number of tables in directory
            ("search_range", u16be()), // TODO[epic=validation] - should be (maximum power of 2 <= num_tables) x 16
            ("entry_selector", u16be()), // TODO[epic=validation] - should be Log2(maximum power of 2 <= num_tables)
            ("range_shift", u16be()), // TODO[epic=validation] - should be (NumTables x 16) - searchRange
            (
                "table_records",
                repeat_count(var("num_tables"), table_record.call()),
            ),
            (
                "table_links",
                table_links.call_args_views(vec![var("table_records")], vec![FONTVIEW_VAR]),
            ),
        ]),
    );

    let ttc_header = {
        // Version 1.0
        // WIP
        let ttc_header1 = |font_view: ViewExpr| {
            record([
                ("num_fonts", u32be()),
                (
                    "table_directories",
                    repeat_count(
                        var("num_fonts"),
                        // REVIEW - avoiding phantom for the moment to avoid too much delayed evaluation all at once
                        util::read_view_offset32(
                            font_view.clone(),
                            table_directory.call_view(font_view),
                        ),
                    ),
                ),
            ])
        };

        // Version 2.0
        // WIP
        let ttc_header2 = |font_view: ViewExpr| {
            record([
                ("num_fonts", u32be()),
                (
                    "table_directories",
                    repeat_count(
                        var("num_fonts"),
                        // REVIEW - avoiding phantom for the moment to avoid too much delayed evaluation all at once
                        util::read_view_offset32(
                            font_view.clone(),
                            table_directory.call_view(font_view),
                        ),
                    ),
                ),
                ("dsig_tag", u32be()),    // either b"DSIG" or 0 if none
                ("dsig_length", u32be()), // byte-length or 0 if none
                ("dsig_offset", u32be()), // byte-offset or 0 if none
            ])
        };

        module.define_format_views(
            "opentype.ttc_header",
            vec![FONTVIEW_LBL],
            record_auto([
                (
                    "ttc_tag",
                    where_lambda(
                        u32be(),
                        "tag",
                        expr_eq(var("tag"), Expr::U32(util::magic(b"ttcf"))),
                    ),
                ),
                ("major_version", u16be()),
                ("minor_version", u16be()),
                (
                    "header",
                    match_variant(
                        var("major_version"),
                        [
                            (Pattern::U16(1), "Version1", ttc_header1(util::FONTVIEW_VAR)),
                            (Pattern::U16(2), "Version2", ttc_header2(util::FONTVIEW_VAR)),
                            // REVIEW - is this the preferred pattern (i.e. apply broadly) or do we want to fail here as well?
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

    module.define_format(
        "opentype.main",
        let_view(
            FONTVIEW_LBL,
            record([
                ("magic", Format::Peek(Box::new(u32be()))),
                (
                    "directory",
                    match_variant(
                        var("magic"),
                        [
                            (
                                Pattern::U32(0x00010000),
                                "TableDirectory",
                                table_directory.call_view(FONTVIEW_VAR),
                            ),
                            (
                                Pattern::U32(util::magic(b"OTTO")),
                                "TableDirectory",
                                table_directory.call_view(FONTVIEW_VAR),
                            ),
                            (
                                Pattern::U32(util::magic(b"ttcf")),
                                "TTCHeader",
                                ttc_header.call_view(FONTVIEW_VAR),
                            ),
                            // TODO - not yet sure if TrueType fonts will parse correctly under our current table_directory implementation...
                            (
                                Pattern::U32(util::magic(b"true")),
                                "TableDirectory",
                                table_directory.call_view(FONTVIEW_VAR),
                            ),
                            (Pattern::Wildcard, "UnknownTable", unknown_table),
                        ],
                    ),
                ),
            ]),
        ),
    )
}

// SECTION - Common low-level formats
// TODO - document this function as appropriate
pub(crate) fn encoding_id(_platform_id: Expr) -> Format {
    u16be()
}

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
pub(crate) fn language_id() -> Format {
    u16be()
}
// !SECTION

/// Common sub-formats used in multiple tables
pub(crate) mod common;

/// Module for subformat functions related to tables in general
pub(crate) mod table {
    use super::*;

    /// Table that is required to appear
    ///
    /// Takes an expr for the start-of-file offset `sof_offset`, an Expr containing the parsed sequence-of-table-records `table-records`,
    /// a table id `id` unique to the table we are defining, and the format of the table `table_format`.
    pub(crate) fn required_table(
        font_view: ViewExpr,
        table_records: Expr,
        id: u32,
        table_format: Format,
    ) -> Format {
        let dep_format = |opt_table_match: Expr| -> Format {
            fmt_match(
                opt_table_match,
                [
                    (
                        pat_some(bind("matching_table")),
                        // REVIEW - should these offsets be eagerly expanded, or is phantom-read more appropriate?
                        util::parse_view_offset_mandatory(
                            font_view,
                            record_proj(var("matching_table"), "offset"),
                            slice(record_proj(var("matching_table"), "length"), table_format),
                        ),
                    ),
                    // NOTE - the line below is not strictly necessary as an ExcludedBranch catchall will be generate
                    // (pat_none(), Format::Fail)
                ],
            )
        };
        util::with_table(table_records, id, dep_format)
    }

    /// Variation of [`required_table`]where the table-format is dependent on the length of the table (as accessed through the `.length` projection on the corresponding table-record)
    ///
    /// Instead of a table-`Format`, we take a `FormatRef` that is expected to take a single argument of kind `U32` that specifies the table-length.
    pub(crate) fn required_table_with_len(
        font_view: ViewExpr,
        table_records: Expr,
        id: u32,
        table_format_ref: FormatRef,
    ) -> Format {
        let dep_format = |opt_table_match: Expr| -> Format {
            fmt_match(
                opt_table_match,
                [
                    (
                        pat_some(bind("matching_table")),
                        // REVIEW - should these offsets be eagerly expanded, or is phantom-read more appropriate?
                        util::parse_view_offset_mandatory(
                            font_view,
                            record_proj(var("matching_table"), "offset"),
                            fmt_let(
                                "table_len",
                                record_proj(var("matching_table"), "length"),
                                slice(
                                    var("table_len"),
                                    table_format_ref.call_args(vec![var("table_len")]),
                                ),
                            ),
                        ),
                    ),
                    // NOTE - the line below is not strictly necessary as an ExcludedBranch catchall will be generate
                    // (pat_none(), Format::Fail)
                ],
            )
        };
        util::with_table(table_records, id, dep_format)
    }

    pub(crate) fn optional_table(
        font_view: ViewExpr,
        table_records: Expr,
        id: u32,
        table_format: Format,
    ) -> Format {
        let cond_fmt = |table_match: Expr| -> Format {
            // REVIEW - should these offsets be eagerly expanded, or is phantom-read more appropriate?
            // NOTE - even though the table is optional, the dep-format we process when it is present is never nullable
            util::parse_view_offset_mandatory(
                font_view,
                record_proj(table_match.clone(), "offset"),
                slice(record_proj(table_match, "length"), table_format),
            )
        };
        let dep_format = move |opt_table_match: Expr| -> Format {
            map_option(opt_table_match, "table", cond_fmt)
        };
        util::with_table(table_records, id, dep_format)
    }
}
use table::{optional_table, required_table, required_table_with_len};

pub(crate) fn opentype_tag(module: &mut FormatModule) -> FormatRef {
    module.define_format("opentype.types.tag", u32be())
}

// ANCHOR - `cmap` table
pub(crate) mod cmap;

// ANCHOR - `head` table
pub(crate) mod head;

// ANCHOR - hhea table
pub(crate) mod hhea;

// ANCHOR - maxp table
pub(crate) mod maxp;

// ANCHOR - hmtx table
pub(crate) mod hmtx;

// ANCHOR - name table
pub(crate) mod name;

// ANCHOR - os2 table
pub(crate) mod os2;

// ANCHOR - post table
pub(crate) mod post;

// ANCHOR - cvt table
pub(crate) mod cvt;

// ANCHOR - fpgm table
pub(crate) mod fpgm;

// ANCHOR - loca table
pub(crate) mod loca;

// ANCHOR - glyf table
pub(crate) mod glyf;

// ANCHOR - prep table
pub(crate) mod prep;

// REVIEW - the generated names for gasp subtypes can be run-on, consider pruning name tokens or module.define_format(_args) for brevity
pub(crate) mod gasp;

// SECTION - advanced typographic features
/// Module for sub-formats used in both GSUB and GPOS
pub(crate) mod layout;

pub(crate) mod base;

pub(crate) mod gdef;

pub(crate) mod gsub;

pub(crate) mod gpos;
// !SECTION

// SECTION - Variable Fonts tables
pub(crate) mod avar;

pub(crate) mod fvar;

pub(crate) mod gvar;

pub(crate) mod hvar;
// !SECTION

// ANCHOR - hdmx
pub(crate) mod hdmx;

// ANCHOR - dsig table
pub(crate) mod dsig;

pub(crate) mod kern;

// ANCHOR - vdmx
pub(crate) mod vdmx;

// ANCHOR - `stat` table
pub(crate) mod stat;

pub(crate) mod vhea;

// ANCHOR - vmtx table
pub(crate) mod vmtx;
