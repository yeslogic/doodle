use doodle::{
    BaseType, Expr, Format, FormatModule, FormatRef, Label, Pattern, ValueType, ViewExpr,
    bounds::Bounds, helper::*,
};

mod util {
    use super::*;
    use doodle::IntoLabel;
    #[cfg(feature = "alt")]
    use doodle::alt::FormatExt;

    pub(crate) fn id<T>(x: T) -> T {
        x
    }

    pub(crate) fn shadow_check(x: &Expr, name: &'static str) {
        if x.is_shadowed_by(name) {
            panic!("Shadow! Variable-name {name} already occurs in Expr {x:?}!");
        }
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
    ///   not flattened (see `nesting kind`)
    /// - `variant_name` is the constructor-name for the sole variant of the enum that holds the `inner_fields` record
    /// - `nesting_kind` is a template-selector that determines how to construct the return-value from the given arguments.
    ///
    /// We have two choices: `SingletonADT`, which constructs an embedded ADT using `intermediate` and `variant_name`; and `FlattenInner`,
    ///   which ignores those fields and instead constructs a single flattened record, concatenating `outer_fields` and `inner_fields`
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
                // REVIEW - it is not necessarily obvious that all FlatternInner defs can be changed to SingletonADT versions if they refer to variables in the outer record, but it seems plausible at least
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

    pub(crate) fn embedded_variadic_alternation<C, const OUTER: usize, const BRANCHES: usize>(
        shared_fields: [(&'static str, Format); OUTER],
        discriminant: &'static str,
        branches: [(u16, &'static str, C); BRANCHES],
        intermediate: &'static str,
        nesting_kind: NestingKind,
    ) -> Format
    where
        C: IntoIterator<Item = (&'static str, Format), IntoIter: DoubleEndedIterator>,
    {
        match nesting_kind {
            NestingKind::MinimalVariation => {
                let mut pat_branches = Vec::with_capacity(BRANCHES);
                for (value, vname, c) in branches.into_iter() {
                    let record_inner = record(c);
                    pat_branches.push((Pattern::U16(value), vname, record_inner));
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
                    pat_branches.push((Pattern::U16(value), vname, record_inner));
                }
                peek_field_then(
                    field_prefix.as_slice(),
                    match_variant(var(discriminant), pat_branches),
                )
            }
        }
    }

    pub(crate) fn hi_flag_u15be(flag_name: &'static str, field_name: &'static str) -> Format {
        bit_fields_u16([
            BitFieldKind::FlagBit(flag_name),
            BitFieldKind::BitsField {
                field_name,
                bit_width: 15,
            },
        ])
    }

    /// Extracts the final element of a sequence-Expr if it is not empty
    ///
    /// If the sequence is empty, the behavior is unspecified
    pub(crate) fn last_elem(seq: Expr) -> Expr {
        let last_ix = pred(seq_length(seq.clone()));
        index_unchecked(seq, last_ix)
    }

    /// Helper function to handle the fact that though vmtx only appears alongside vhea, both are optional tables
    /// so direct record projection is not possible (as vhea will be an option-wrapped record)
    pub(crate) fn vhea_long_metrics(vhea: Expr) -> Expr {
        record_proj(expr_unwrap(vhea), "number_of_long_metrics")
    }

    /// Attemptis to index on the `offsets` key of `loca` through an option-unpacking indirection.
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

    /// Converts a `u8` value to an `i16` value within the `Expr` model
    /// according to a bit-flag for signedness `pos_bit` (`1` for positive, `0` for negative).
    // FIXME - this currently yields the u16 value with the same machine-rep as the nominal i16 value we want
    pub(crate) fn u8_to_i16(x: Expr, is_positive: Expr) -> Expr {
        expr_if_else(
            is_positive,
            as_u16(x.clone()),
            expr_match(
                x,
                [
                    (Pattern::U8(0), Expr::U16(0)),
                    (bind("n"), sub(Expr::U16(u16::MAX), pred(as_u16(var("n"))))),
                ],
            ),
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

    /// FIXME[epic=signedness-hack] - scaffolding to signal intent to use i8 format before it is implemented
    pub(crate) fn s8() -> Format {
        u8()
    }

    /// FIXME[epic=signedness-hack] - scaffolding to signal intent to use i16 format before it is implemented
    pub(crate) fn s16be() -> Format {
        u16be()
    }

    /// FIXME[epic=signedness-hack] - scaffolding to signal intent to use i32 format before it is implemented
    pub(crate) fn s32be() -> Format {
        u32be()
    }

    /// FIXME[epic=signedness-hack] - scaffolding to signal intent to use i64 format before it is implemented
    pub(crate) fn s64be() -> Format {
        u64be()
    }

    /// Helper function for parsing a big-endian u24 (3-byte) value
    pub(crate) fn u24be() -> Format {
        // REVIEW - should U24Be be a CommonOp?
        map(
            Format::Tuple(vec![compute(Expr::U8(0)), u8(), u8(), u8()]),
            lambda("x", Expr::U32Be(Box::new(var("x")))),
        )
    }

    // Placeholder for a `(u16, u16)` value-pair packed as a big-endian u32
    pub(crate) fn version16_16() -> Format {
        u32be()
    }

    /// Helper function for compile-time conversion of b"..." literals into u32 (big-endian) values.
    pub(crate) const fn magic(tag: &'static [u8; 4]) -> u32 {
        u32::from_be_bytes(*tag)
    }

    /// Parses a `U16Be` value that is expected to be equal to `val`
    pub(crate) fn expect_u16be(val: u16) -> Format {
        // REVIEW - if we cared to do it, we could use `chain(is_bytes(val.to_be_bytes()), "_", compute(Expr::U16(val)))` (at the cost of worsening error reporting)
        where_lambda(u16be(), "x", expr_eq(var("x"), Expr::U16(val)))
    }

    /// Parses a `U16Be` value that is expected to be equal to one of `N` values in `vals`
    pub(crate) fn expects_u16be<const N: usize>(vals: [u16; N]) -> Format {
        where_lambda(
            u16be(),
            "x",
            expr_match(
                var("x"),
                vals.into_iter()
                    .map(|v| (Pattern::U16(v), Expr::Bool(true)))
                    .chain(std::iter::once((Pattern::Wildcard, Expr::Bool(false)))),
            ),
        )
    }

    /// Constructs a format that peeks the value of a specific field in a given
    /// record (or the common prefix of a union of related records), discarding the
    /// values of all fields that come before it; the result of this speculative
    /// parse is then associated to the original field-name (in `field_prefix`) before
    /// finally parsing the format `dep_format` that depends on its value.
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
            Format::Peek(Box::new(monad_seq(
                // Process all the fields before the one we care about and discard their cumulative value
                record(init.iter().cloned()),
                // Process the field we *do* care about, while still in the peek context, and yield its value as the result of the entire parse
                field_format.clone(),
            ))),
            // Scope-capture the final field of `field_prefix` under the identifier it is paired
            field_name.clone(),
            dep_format,
        )
    }

    /// Specialized format-construction designed for supporting `cmap` and `kern` sub-tables.
    ///
    /// Speculatively peeks the shortest prefix of fields required to witness a field with the
    /// indicated label (`length_field`), which is interpreted as a positive integer byte-length
    /// constraining the entire record (and not just subsequent fields); this value is extracted
    /// and forms the length of a slice around parsing the complete record.
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
            ValueType::Option(Box::new(ValueType::Base(BaseType::U16))),
            seq,
        ))
    }

    // WIP
    pub(crate) const START_VAR: Expr = Expr::Var(Label::Borrowed("start"));

    // WIP
    pub(crate) const START_ARG: (Label, ValueType) =
        (Label::Borrowed("start"), ValueType::Base(BaseType::U32));

    pub(crate) const TABLE_VIEW: Label = Label::Borrowed("table_view");

    /// Given `Expr`s `table_records` and a `query_table_id` of the appropriate Rust-type (`u32`),
    /// applies `dep_format` to the `Option<T>`-kinded `Expr` yielded by a binary search over
    /// `table_records ~ Seq<T>`.
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

    pub(crate) const CONTENT_AT_OFFSET_IDENT: &str = "link";

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
            ("offset", u32be()),
            (
                "#_data",
                phantom(parse_view_offset::<U32>(view, var("offset"), format)),
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

    /// Given a value of `base_offset` (the absolute stream-position relative to which offsets are to be interpreted),
    /// parses a u32be as a positive delta from `base_offset` and returns the linked content parsed according
    /// to `format` at that location.
    ///
    /// Returns a record `{ offset: u32, link := (offset > 0) ? Some(format) : None }`
    ///
    /// (Implicitly includes a semantic shortcut whereby an offset-value (parsed) of `0` signals
    /// that there is no associated data, in which case `None` is yielded for the `link`.)
    ///
    /// # Notes
    ///
    /// To handle irregular inputs that would otherwise require moving *backwards* to reach the desired offset,
    /// `None` is returned in any case where the relative-delta to reach the target offset is non-positive.
    pub(crate) fn offset32(base_offset: Expr, format: Format) -> Format {
        shadow_check(&base_offset, "offset");
        // FIXME - should we use `chain` instead of `record` to elide the offset and flatten the link?
        record([
            ("offset", u32be()),
            (
                CONTENT_AT_OFFSET_IDENT,
                if_then_else(
                    is_nonzero_u32(var("offset")),
                    linked_offset32(base_offset, var("offset"), fmt_some(format)),
                    fmt_none(),
                ),
            ),
        ])
    }

    /// Given the appropriate Start-of-Frame absolute-stream-offset (`base_offset`) and
    /// an SOF-relative `rel_offset`, produce a relative-seek format that
    /// seeks to the appropriate stream-location and parses `format`.
    ///
    /// # Notes
    ///
    /// Though not directly stated, the assumed type of `sof_offset` and `target_offset` is
    /// `U32`, and if this is not satisfied, the invocation of this function will produce a
    /// type-error when expanded.
    ///
    /// Will fail at time-of-parse if seeking to the target requires going backwards from the immediate
    /// offset we are reading from.
    // REVIEW - double-check whether the claim that retrograde seeks will fail is still true
    pub(crate) fn linked_offset32(base_offset: Expr, rel_offset: Expr, format: Format) -> Format {
        with_relative_offset(Some(base_offset), rel_offset, format)
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

    let table_links = {
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

        let value_record =
            layout::value_record(module, device_or_variation_index_table, vf_flags_type);

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
        let chained_sequence_context = layout::chained_sequence_context(
            module,
            class_def,
            coverage_table,
            sequence_lookup_record,
        );
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

        // `kern` table [https://learn.microsoft.com/en-us/typography/opentype/spec/kern]
        let kern_table = kern::table(module);

        let stat_table = stat::table(module, tag);
        let fvar_table = fvar::table(module, tag);
        let gvar_table = gvar::table(module);

        module.define_format_args(
            "opentype.table_directory.table_links",
            vec![
                // WIP
                START_ARG,
                (
                    Label::Borrowed("tables"),
                    ValueType::Seq(Box::new(table_type)),
                ),
            ],
            record_auto([
                (
                    "cmap",
                    required_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"cmap"),
                        cmap_table.call(),
                    ),
                ),
                (
                    "head",
                    required_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"head"),
                        head_table.call(),
                    ),
                ),
                (
                    "hhea",
                    required_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"hhea"),
                        hhea_table.call(),
                    ),
                ),
                (
                    "maxp",
                    required_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"maxp"),
                        maxp_table.call(),
                    ),
                ),
                (
                    "hmtx",
                    required_table(
                        util::START_VAR,
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
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"name"),
                        name_table.call(),
                    ),
                ),
                (
                    "os2",
                    required_table_with_len(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"OS/2"),
                        os2_table,
                    ),
                ),
                (
                    "post",
                    required_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"post"),
                        post_table.call(),
                    ),
                ),
                // SECTION - TrueType Outline
                (
                    "cvt",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"cvt "),
                        cvt_table,
                    ),
                ),
                (
                    "fpgm",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"fpgm"),
                        fpgm_table,
                    ),
                ),
                (
                    "loca",
                    optional_table(
                        util::START_VAR,
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
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"glyf"),
                        glyf_table.call_args(vec![loca_offsets(var("loca"))]),
                    ),
                ),
                (
                    "prep",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"prep"),
                        prep_table,
                    ),
                ),
                (
                    "gasp",
                    optional_table(
                        util::START_VAR,
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
                // FIXM - `EBLC` postponed due to rarity (15 of 659 tested fonts)
                // FIXME - `EBSC` postponed due to rarity (no occurrences among 659 tested fonts)
                // FIXME - `CBDT` postponed due to rarity (2 of 659 tested fonts)
                // FIXME - `CBLC` postponed due to rarity (2 of 659 tested fonts)
                // FIXME - `sbix` postponed due to rarity (1 of 659 tested fonts)
                // !SECTION
                // SECTION - Advanced Typography
                (
                    "base",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"BASE"),
                        base_table.call(),
                    ),
                ),
                (
                    "gdef",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"GDEF"),
                        gdef_table.call(),
                    ),
                ),
                (
                    "gpos",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"GPOS"),
                        gpos_table.call(),
                    ),
                ),
                (
                    "gsub",
                    optional_table(
                        util::START_VAR,
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
                    "fvar",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"fvar"),
                        fvar_table.call(),
                    ),
                ),
                (
                    "gvar",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"gvar"),
                        gvar_table.call(),
                    ),
                ),
                // !SECTION
                // STUB - add more table sections
                // SECTION - other tables
                // STUB - add more tables
                (
                    "kern",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"kern"),
                        kern_table.call(),
                    ),
                ),
                (
                    "stat",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"STAT"),
                        stat_table.call(),
                    ),
                ),
                (
                    "vhea",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"vhea"),
                        vhea_table.call(),
                    ),
                ),
                (
                    "vmtx",
                    optional_table(
                        util::START_VAR,
                        var("tables"),
                        util::magic(b"vmtx"),
                        vmtx_table.call_args(vec![
                            vhea_long_metrics(var("vhea")),
                            record_proj(var("maxp"), "num_glyphs"),
                        ]),
                    ),
                ),
                // !SECTION
                ("__skip", Format::SkipRemainder),
            ]),
        )
    };

    let table_directory = module.define_format_args(
        "opentype.table_directory",
        vec![(
            Label::Borrowed("font_start"),
            ValueType::Base(BaseType::U32),
        )],
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
            ("search_range", u16be()), // TODO[validation] - should be (maximum power of 2 <= num_tables) x 16
            ("entry_selector", u16be()), // TODO[validation] - should be Log2(maximum power of 2 <= num_tables)
            ("range_shift", u16be()), // TODO[validation] - should be (NumTables x 16) - searchRange
            (
                "table_records",
                repeat_count(var("num_tables"), table_record.call()),
            ),
            (
                "table_links",
                table_links.call_args(vec![var("font_start"), var("table_records")]),
            ),
        ]),
    );

    let ttc_header = {
        // Version 1.0
        // WIP
        let ttc_header1 = |start: Expr| {
            record([
                ("num_fonts", u32be()),
                (
                    "table_directories",
                    repeat_count(
                        var("num_fonts"),
                        util::offset32(start.clone(), table_directory.call_args(vec![start])),
                    ),
                ),
            ])
        };

        // Version 2.0
        // WIP
        let ttc_header2 = |start: Expr| {
            record([
                ("num_fonts", u32be()),
                (
                    "table_directories",
                    repeat_count(
                        var("num_fonts"),
                        util::offset32(start.clone(), table_directory.call_args(vec![start])),
                    ),
                ),
                ("dsig_tag", u32be()),    // either b"DSIG" or 0 if none
                ("dsig_length", u32be()), // byte-length or 0 if none
                ("dsig_offset", u32be()), // byte-offset or 0 if none
            ])
        };

        module.define_format_args(
            "opentype.ttc_header",
            vec![START_ARG],
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
                            (Pattern::U16(1), "Version1", ttc_header1(util::START_VAR)),
                            (Pattern::U16(2), "Version2", ttc_header2(util::START_VAR)),
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
        record([
            ("file_start", pos32()),
            ("magic", Format::Peek(Box::new(u32be()))),
            (
                "directory",
                match_variant(
                    var("magic"),
                    [
                        (
                            Pattern::U32(0x00010000),
                            "TableDirectory",
                            table_directory.call_args(vec![var("file_start")]),
                        ),
                        (
                            Pattern::U32(util::magic(b"OTTO")),
                            "TableDirectory",
                            table_directory.call_args(vec![var("file_start")]),
                        ),
                        (
                            Pattern::U32(util::magic(b"ttcf")),
                            "TTCHeader",
                            ttc_header.call_args(vec![var("file_start")]),
                        ),
                        // TODO - not yet sure if TrueType fonts will parse correctly under our current table_directory implementation...
                        (
                            Pattern::U32(util::magic(b"true")),
                            "TableDirectory",
                            table_directory.call_args(vec![var("file_start")]),
                        ),
                        (Pattern::Wildcard, "UnknownTable", unknown_table),
                    ],
                ),
            ),
        ]),
    )
}

mod gvar {
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
                            fmt_variant("Delta16", repeat_count(var("run_length"), util::s16be())),
                            fmt_variant("Delta8", repeat_count(var("run_length"), util::s8())),
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
            ValueType::Base(BaseType::U16),
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
    /// - `data_table`: Format definition for GlyphVariationData table
    fn data_table_array_entry(
        axis_count: Expr,
        array_view: ViewExpr,
        this_offset32: Expr,
        next_offset32: Expr,
        data_table: FormatRef,
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
                    slice(var("len"), data_table.call_args(vec![axis_count])),
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
    /// - `data_table`: Format definition for GlyphVariationData table
    fn data_table_array(axis_count: Expr, offsets: Expr, data_table: FormatRef) -> Format {
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
            "opentype.gvar_table",
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
                                tuple_record.call_args(vec![var("axis_count")]),
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
    fn glyph_variation_data(module: &mut FormatModule, tuple_record: FormatRef) -> FormatRef {
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

        module.define_format_args(
            "opentype.gvar.glyph_variation_data",
            vec![(
                Label::Borrowed("axis_count"),
                ValueType::Base(BaseType::U16),
            )],
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
                            tuple_variation_header.call_args(vec![var("axis_count")]),
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
        tuple_variation_header: FormatRef,
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
                                        Box::new(packed_deltas(mul(
                                            var("point_count"),
                                            Expr::U16(2),
                                        ))),
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
    fn point_number_runs(module: &mut FormatModule) -> FormatRef {
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
        module.define_format_args(
            "opentype.var.packed-point-numbers.runs",
            vec![(Label::Borrowed("point_count"), ValueType::U16)],
            accum_until(
                lambda_tuple(
                    ["totlen", "_seq"],
                    expr_gte(var("totlen"), var("point_count")),
                ),
                update_totlen,
                Expr::U16(0),
                ValueType::Base(BaseType::U16),
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
                    runs.call_args(vec![as_u16(var("point_count"))]),
                ),
                chain(
                    byte_in(128..=255),
                    "hi",
                    chain(
                        u8(),
                        "lo",
                        runs.call_args(vec![u15be(var("hi"), var("lo"))]),
                    ),
                ),
            ]),
        )
    }

    fn tuple_variation_header(module: &mut FormatModule, tuple_record: FormatRef) -> FormatRef {
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
        module.define_format_args(
            "opentype.gvar.tuple_variation_header",
            vec![(Label::Borrowed("axis_count"), ValueType::U16)],
            record([
                ("variation_data_size", u16be()), // size, in bytes, of serialized data for this tuple variation table
                ("tuple_index", tuple_index),
                (
                    "peak_tuple",
                    cond_maybe(
                        record_proj(var("tuple_index"), "embedded_peak_tuple"),
                        tuple_record.call_args(vec![var("axis_count")]),
                    ),
                ),
                (
                    "intermediate_tuples",
                    cond_maybe(
                        record_proj(var("tuple_index"), "intermediate_region"),
                        record_repeat(
                            ["start_tuple", "end_tuple"],
                            tuple_record.call_args(vec![var("axis_count")]),
                        ),
                    ),
                ),
            ]),
        )
    }

    /// Definition for Tuple Records used in variation tables
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#tuple-records
    /// TODO - change namespace from `gvar` to `var`, move to common submodule for multi-table sub-formats
    fn tuple_record(module: &mut FormatModule) -> FormatRef {
        module.define_format_args(
            "opentype.gvar.tuple_record",
            vec![(Label::Borrowed("axis_count"), ValueType::U16)],
            record([(
                "coordinates",
                repeat_count(var("axis_count"), util::f2dot14()),
            )]),
        )
    }
}

pub(crate) mod fvar {
    use super::*;

    pub(crate) fn table(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let variation_axis_record = variation_axis_record(module, tag);
        let instance_record = instance_record(module);

        // First half of `fvar` table: fixed-size header
        let fvar_header = record_auto([
            ("major_version", util::expect_u16be(1)),
            ("minor_version", util::expect_u16be(0)),
            (
                "offset_axes",
                where_lambda(u16be(), "raw", is_nonzero_u16(var("raw"))),
            ),
            ("__reserved", util::expect_u16be(2)),
            ("axis_count", u16be()),
            ("axis_size", util::expect_u16be(20)), // For fvar version 1.0, axis record are fixed-size == 20 (0x0014) bytes
            ("instance_count", u16be()),
            ("instance_size", u16be()), // not yet enforced, but should be axisCount * sizeOf(Fixed32Be) + (4 or 6)
        ]);
        // Second half of `fvar` table: offset-linked axes and instances
        let fvar_arrays = record_auto([
            (
                "_axes_length",
                compute(mul(var("axis_count"), var("axis_size"))),
            ),
            (
                "#_axes",
                // TODO - this becomes a lot easier if we use ViewFormats instead of offset-parse patterns
                // NOTE - because we delay interpretation of the offset above to collect additional fields, we inline and specialize offset16 based on the captured value
                phantom(parse_from_view(
                    vvar("table_view").offset(var("offset_axes")),
                    slice(
                        var("_axes_length"),
                        repeat_count(
                            var("axis_count"),
                            // because axis_size is fixed at 20 and variation_axis_record is a fixed-width (20 byte) parse, we don't need a slice here
                            variation_axis_record.call(),
                        ),
                    ),
                )),
            ),
            (
                "offset_instances",
                compute(add(var("offset_axes"), var("_axes_length"))),
            ),
            (
                "#_instances",
                // NOTE - because we delay interpretation of the offset above to collect additional fields, we inline and specialize offset16 based on the captured value
                phantom(parse_from_view(
                    vvar("table_view").offset(var("offset_instances")),
                    repeat_count(
                        var("instance_count"),
                        slice(
                            var("instance_size"),
                            instance_record
                                .call_args(vec![var("axis_count"), var("instance_size")]),
                        ),
                    ),
                )),
            ),
        ]);
        let scope_field = record([("table_scope", reify_view(vvar("table_view")))]);
        module.define_format(
            "opentype.fvar.table",
            let_view(
                "table_view",
                merge_records([scope_field, fvar_header, fvar_arrays]),
            ),
        )
    }

    /// InstanceRecord format implementation
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/fvar#instancerecord
    fn instance_record(module: &mut FormatModule) -> FormatRef {
        let user_tuple = user_tuple(module);
        module.define_format_args(
            "opentype.fvar.instance_record",
            vec![
                (Label::Borrowed("axis_count"), ValueType::U16),
                (Label::Borrowed("instance_size"), ValueType::U16),
            ],
            record([
                ("subfamily_nameid", u16be()),
                ("flags", util::expect_u16be(0)), // reserved for future use, should be set to 0,
                ("coordinates", user_tuple.call_args(vec![var("axis_count")])),
                (
                    "postscript_nameid",
                    cond_maybe(
                        // Only included if the extra 2 bytes are implied by `instance_size`, which is otherwise divisible by 4
                        expr_eq(rem(var("instance_size"), Expr::U16(4)), Expr::U16(2)),
                        u16be(),
                    ),
                ),
            ]),
        )
    }

    /// UserTuple record (part of `InstanceRecord`)
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/fvar#instancerecord
    fn user_tuple(module: &mut FormatModule) -> FormatRef {
        module.define_format_args(
            "opentype.fvar.user_tuple",
            vec![(Label::Borrowed("axis_count"), ValueType::U16)],
            record([(
                "coordinates",
                repeat_count(var("axis_count"), util::fixed32be()),
            )]),
        )
    }

    fn variation_axis_record(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        use BitFieldKind::*;
        let axis_qual_flags = bit_fields_u16([
            Reserved {
                bit_width: 15,
                check_zero: false,
            },
            FlagBit("hidden_axis"),
        ]);
        module.define_format(
            "opentype.fvar.variation_axis_record",
            record([
                ("axis_tag", tag.call()),             // 4 bytes
                ("min_value", util::fixed32be()),     // + 4 = 8 bytes
                ("default_value", util::fixed32be()), // + 4 = 12 bytes
                ("max_value", util::fixed32be()),     // +4 = 16 bytes
                ("flags", axis_qual_flags),           // + 2 = 18 bytes
                ("axis_name_id", u16be()),            // + 2 = 20 bytes
            ]),
        )
    }
}

pub(crate) mod kern {
    use super::*;

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

    // REVIEW - consider refactoring into FormatRef registration
    /// Kern subtable format 0
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/kern#format-0
    fn subtable_format0(module: &mut FormatModule) -> FormatRef {
        let kern_pair = record([
            ("left", u16be()),        // glyph index for left-hand glyph in kerning pair
            ("right", u16be()),       // glyph index for right-hand glyph in kerning pair
            ("value", util::s16be()), // kerning value for given pair, in design-units. Positive values move characters apart, negative values move characters closer together.
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

    /// Helper function used to compute the number of glyphs in a left-or-right class table (for Format 2 kern subtables)
    fn glyph_count(
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
    fn kerning_array(
        table_view: ViewExpr,
        left_class_offset: Expr,
        right_class_offset: Expr,
        class_table: FormatRef,
    ) -> Format {
        pseudo_record(
            [
                (
                    "left_glyph_count",
                    glyph_count(table_view.clone(), left_class_offset, class_table),
                ),
                (
                    "right_glyph_count",
                    glyph_count(table_view, right_class_offset, class_table),
                ),
            ],
            repeat_count(
                expr_unwrap(var("left_glyph_count")), // N rows where there are N left-hand classes
                repeat_count(
                    expr_unwrap(var("right_glyph_count")), // M columns
                    util::s16be(),                         // FWORD value at index (i, j)
                ),
            ),
        )
    }

    /// Kern subtable format 2
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/kern#format-2
    fn subtable_format2(module: &mut FormatModule) -> FormatRef {
        let class_table = module.define_format(
            "opentype.kern.class_table",
            record([
                ("first_glyph", u16be()), // first glyph in class range
                ("n_glyphs", u16be()),    // number of glyphs in class range
                ("class_values", repeat_count(var("n_glyphs"), u16be())), // class values for each glyph in class range
            ]),
        );
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
                            kerning_array(
                                vvar("table_view"),
                                var("left_class_offset"),
                                var("right_class_offset"),
                                class_table,
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
}

mod base {
    use super::*;

    /// BASE table format definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base
    pub(crate) fn table(
        module: &mut FormatModule,
        tag: FormatRef,
        device_or_variation_index_table: FormatRef,
        item_variation_store: FormatRef,
    ) -> FormatRef {
        let base_coord = base_coord(module, device_or_variation_index_table);
        let min_max = min_max(module, tag, base_coord);
        let base_values = base_values(module, base_coord);

        let base_lang_sys = module.define_format_views(
            "opentype.base.base-langsys",
            vec![Label::Borrowed("table_view")],
            record([
                ("base_lang_sys_tag", tag.call()),
                (
                    "min_max",
                    util::read_phantom_view_offset16(vvar("table_view"), min_max.call()),
                ),
            ]),
        );
        let base_script = module.define_format(
            "opentype.layout.base_script",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    (
                        "base_values_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), base_values.call()),
                    ),
                    (
                        "default_min_max_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), min_max.call()),
                    ),
                    ("base_lang_sys_count", u16be()),
                    (
                        "base_lang_sys_records",
                        repeat_count(
                            var("base_lang_sys_count"),
                            base_lang_sys.call_views(vec![vvar("table_view")]),
                        ),
                    ),
                ]),
            ),
        );
        let base_script_record = module.define_format_views(
            "opentype.base.base-script-record",
            vec![Label::Borrowed("table_view")],
            record([
                ("base_script_tag", tag.call()),
                (
                    "base_script",
                    util::read_phantom_view_offset16(vvar("table_view"), base_script.call()),
                ),
            ]),
        );
        let base_script_list = let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("base_script_count", u16be()),
                (
                    "base_script_records",
                    repeat_count(
                        var("base_script_count"),
                        base_script_record.call_view(vvar("table_view")),
                    ),
                ),
            ]),
        );
        let base_tag_list = record([
            ("base_tag_count", u16be()),
            (
                "baseline_tags",
                repeat_count(var("base_tag_count"), tag.call()),
            ), // must appear in alphabetical order (not enforced locally)
        ]);
        let axis_table = module.define_format(
            "opentype.layout.axis_table",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    (
                        "base_tag_list_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), base_tag_list),
                    ),
                    (
                        "base_script_list_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), base_script_list),
                    ),
                ]),
            ),
        );
        module.define_format(
            "opentype.base.table",
            let_view(
                "table_view",
                record([
                    // WIP
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("major_version", util::expect_u16be(1)),
                    ("minor_version", where_between_u16(u16be(), 0, 1)), // v1.0 and v1.1
                    (
                        "horiz_axis_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), axis_table.call()),
                    ),
                    (
                        "vert_axis_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), axis_table.call()),
                    ),
                    (
                        "item_var_store_offset",
                        cond_maybe(
                            expr_gt(var("minor_version"), Expr::U16(0)),
                            util::read_phantom_view_offset32(
                                vvar("table_view"),
                                item_variation_store.call(),
                            ),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// BaseValues table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base#basevalues-table
    fn base_values(module: &mut FormatModule, base_coord: FormatRef) -> FormatRef {
        let base_values = module.define_format(
            "opentype.layout.base_values",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("default_baseline_index", u16be()),
                    ("base_coord_count", u16be()), // NOTE - should be equal to baseTagCount in BaseTagList
                    (
                        "base_coord_offsets",
                        repeat_count(
                            var("base_coord_count"),
                            util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                        ),
                    ),
                ]),
            ),
        );
        base_values
    }

    /// MinMax table format definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base#the-minmax-table-and-featminmax-record
    fn min_max(module: &mut FormatModule, tag: FormatRef, base_coord: FormatRef) -> FormatRef {
        let feat_min_max = module.define_format_views(
            "opentype.layout.feat_min_max",
            vec![Label::Borrowed("table_view")],
            record([
                ("feature_tag", tag.call()),
                (
                    "min_coord_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                ),
                (
                    "max_coord_offset",
                    util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.min_max",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    (
                        "min_coord_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                    ),
                    (
                        "max_coord_offset",
                        util::read_phantom_view_offset16(vvar("table_view"), base_coord.call()),
                    ),
                    ("feat_min_max_count", u16be()),
                    (
                        "feat_min_max_records",
                        repeat_count(
                            var("feat_min_max_count"),
                            feat_min_max.call_views(vec![vvar("table_view")]),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// BaseCoord table format definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/base#basecoord-tables
    fn base_coord(
        module: &mut FormatModule,
        device_or_variation_index_table: FormatRef,
    ) -> FormatRef {
        // NOTE - 'data' field is a nested record of any fields beyond `{ format, coordinate }` used in a given format
        let format1_data = Format::EMPTY;
        let format2_data = record([("reference_glyph", u16be()), ("base_coord_point", u16be())]);
        let format3_data = |table_view: ViewExpr| {
            record([(
                "device",
                util::read_phantom_view_offset16(
                    table_view,
                    device_or_variation_index_table.call(),
                ),
            )])
        };
        module.define_format(
            "opentype.layout.base_coord",
            let_view(
                "table_view",
                record([
                    // WIP
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("format", u16be()),
                    ("coordinate", util::s16be()),
                    (
                        "data",
                        match_variant(
                            var("format"),
                            [
                                (Pattern::U16(1), "NoData", format1_data),
                                (Pattern::U16(2), "GlyphData", format2_data),
                                (
                                    Pattern::U16(3),
                                    "DeviceData",
                                    format3_data(vvar("table_view")),
                                ),
                                (Pattern::Wildcard, "UnknownFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }
}

mod gpos {
    use super::*;

    /// GPOS-specific LookupSubtable implementation
    fn lookup_subtable(
        module: &mut FormatModule,
        pos_extension: FormatRef,
        ground_pos: FormatRef,
    ) -> FormatRef {
        const EXTENSION_TYPE: u16 = 9;
        module.define_format_args(
            "opentype.gpos.lookup_subtable",
            vec![(Label::Borrowed("lookup_type"), ValueType::U16)],
            match_variant(
                var("lookup_type"),
                [
                    (
                        Pattern::U16(EXTENSION_TYPE),
                        "PosExtension",
                        pos_extension.call(),
                    ),
                    (
                        Pattern::Wildcard,
                        "GroundPos",
                        ground_pos.call_args(vec![var("lookup_type")]),
                    ),
                ],
            ),
        )
    }

    /// Lookup type 1 subtable: single adjustment positioning
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-1-subtable-single-adjustment-positioning
    pub(crate) fn single_pos(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        value_format_flags: FormatRef,
        value_record: FormatRef,
    ) -> FormatRef {
        let single_pos_format1 = module.define_format_views(
            "opentype.layout.single_pos.format1",
            vec![Label::Borrowed("table_view")],
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "coverage",
                    read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("value_format", value_format_flags.call()),
                (
                    "value_record",
                    value_record
                        .call_args_views(vec![var("value_format")], vec![vvar("table_view")]),
                ),
            ]),
        );
        let single_pos_format2 = module.define_format_views(
            "opentype.layout.single_pos.format2",
            vec![Label::Borrowed("table_view")],
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "coverage",
                    read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("value_format", value_format_flags.call()),
                ("value_count", u16be()),
                (
                    "value_records",
                    repeat_count(
                        var("value_count"),
                        value_record
                            .call_args_views(vec![var("value_format")], vec![vvar("table_view")]),
                    ),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.single_pos",
            let_view(
                "table_view",
                record([
                    ("pos_format", u16be()),
                    (
                        "subtable",
                        match_variant(
                            var("pos_format"),
                            [
                                (
                                    Pattern::U16(1),
                                    "Format1",
                                    single_pos_format1
                                        .call_args_views(Vec::new(), vec![vvar("table_view")]),
                                ),
                                (
                                    Pattern::U16(2),
                                    "Format2",
                                    single_pos_format2
                                        .call_args_views(Vec::new(), vec![vvar("table_view")]),
                                ),
                                // REVIEW - should this be a permanent hard-failure?
                                (Pattern::Wildcard, "BadFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Lookup type 2 subtable: pair adjustmnet positioning
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-2-subtable-pair-adjustment-positioning
    pub(crate) fn pair_pos(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        value_format_flags: FormatRef,
        value_record: FormatRef,
    ) -> FormatRef {
        let vf_flags_type = module
            .get_format_type(value_format_flags.get_level())
            .clone();

        let pair_value_record = module.define_format_args_views(
            "opentype.layout.pair_pos.pair_value_record",
            vec![
                (Label::Borrowed("value_format1"), vf_flags_type.clone()),
                (Label::Borrowed("value_format2"), vf_flags_type.clone()),
            ],
            vec![Label::Borrowed("set_view")],
            record([
                // NOTE - first glyph id is listed in the Coverage table
                ("second_glyph", u16be()),
                (
                    "value_record1",
                    layout::optional_value_record(
                        value_record,
                        vvar("set_view"),
                        var("value_format1"),
                    ),
                ),
                (
                    "value_record2",
                    layout::optional_value_record(
                        value_record,
                        vvar("set_view"),
                        var("value_format2"),
                    ),
                ),
            ]),
        );
        let pair_set = module.define_format_args(
            "opentype.layout.pair_pos.pair_set",
            vec![
                (Label::Borrowed("value_format1"), vf_flags_type.clone()),
                (Label::Borrowed("value_format2"), vf_flags_type.clone()),
            ],
            let_view(
                "set_view",
                record([
                    ("set_scope", reify_view(vvar("set_view"))),
                    ("pair_value_count", u16be()),
                    (
                        "pair_value_records",
                        repeat_count(
                            var("pair_value_count"),
                            pair_value_record.call_args_views(
                                vec![var("value_format1"), var("value_format2")],
                                vec![vvar("set_view")],
                            ),
                        ),
                    ),
                ]),
            ),
        );
        // TODO - refactor into dep-format or standalone function
        let pair_pos_format1 = module.define_format_views(
            "opentype.layout.pair_pos.format1",
            vec![Label::Borrowed("table_view")],
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("value_format1", value_format_flags.call()),
                ("value_format2", value_format_flags.call()),
                ("pair_set_count", u16be()),
                (
                    "pair_sets",
                    repeat_count(
                        var("pair_set_count"),
                        util::read_phantom_view_offset16(
                            vvar("table_view"),
                            pair_set.call_args(vec![var("value_format1"), var("value_format2")]),
                        ),
                    ),
                ),
            ]),
        );
        let class2_record = module.define_format_args_views(
            "opentype.layout.pair_pos.class2_record",
            vec![
                (Label::Borrowed("value_format1"), vf_flags_type.clone()),
                (Label::Borrowed("value_format2"), vf_flags_type.clone()),
            ],
            vec![Label::Borrowed("table_view")],
            record([
                (
                    "value_record1",
                    layout::optional_value_record(
                        value_record,
                        vvar("table_view"),
                        var("value_format1"),
                    ),
                ),
                (
                    "value_record2",
                    layout::optional_value_record(
                        value_record,
                        vvar("table_view"),
                        var("value_format2"),
                    ),
                ),
            ]),
        );

        // TODO - refactor into dep-format or standalone function
        fn class1_record(
            table_view: ViewExpr,
            class2_count: Expr,
            value_format1: Expr,
            value_format2: Expr,
            class2_record: FormatRef,
        ) -> Format {
            record([(
                "class2_records",
                repeat_count(
                    class2_count,
                    class2_record
                        .call_args_views(vec![value_format1, value_format2], vec![table_view]),
                ),
            )])
        }
        // TODO - refactor into dep-format or standalone function
        let pair_pos_format2 = module.define_format_views(
            "opentype.layout.pair_pos.format2",
            vec![Label::Borrowed("table_view")],
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("value_format1", value_format_flags.call()),
                ("value_format2", value_format_flags.call()),
                (
                    "class_def1",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                (
                    "class_def2",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                ("class1_count", u16be()),
                ("class2_count", u16be()),
                (
                    "class1_records",
                    repeat_count(
                        var("class1_count"),
                        class1_record(
                            vvar("table_view"),
                            var("class2_count"),
                            var("value_format1"),
                            var("value_format2"),
                            class2_record,
                        ),
                    ),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.pair_pos",
            let_view(
                "table_view",
                record([
                    ("pos_format", u16be()),
                    (
                        "subtable",
                        match_variant(
                            var("pos_format"),
                            [
                                (
                                    Pattern::U16(1),
                                    "Format1",
                                    pair_pos_format1
                                        .call_args_views(vec![], vec![vvar("table_view")]),
                                ),
                                (
                                    Pattern::U16(2),
                                    "Format2",
                                    pair_pos_format2
                                        .call_args_views(vec![], vec![vvar("table_view")]),
                                ),
                                // REVIEW - should this be a permanent hard-failure?
                                (Pattern::Wildcard, "BadFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Lookup type 3 subtable: cursive attachment positioning
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-3-subtable-cursive-attachment-positioning
    pub(crate) fn cursive_pos(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        anchor_table: FormatRef,
    ) -> FormatRef {
        let entry_exit_record = module.define_format_views(
            "opentype.layout.entry_exit_record",
            vec![Label::Borrowed("table_view")],
            record_repeat(
                ["entry_anchor", "exit_anchor"],
                util::read_phantom_view_offset16(vvar("table_view"), anchor_table.call()),
            ),
        );
        module.define_format(
            "opentype.layout.cursive_pos",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [("pos_format", u16be())],
                    ("pos_format", 1),
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        (
                            "coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        ("entry_exit_count", u16be()),
                        (
                            "entry_exit_records",
                            repeat_count(
                                var("entry_exit_count"),
                                entry_exit_record.call_views(vec![vvar("table_view")]),
                            ),
                        ),
                    ],
                    "subtable",
                    "Format1",
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    /// Mark array table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#mark-array-table
    pub(crate) fn mark_array(module: &mut FormatModule, anchor_table: FormatRef) -> FormatRef {
        // TODO - refactor into dep-format or standalone function
        let mark_record = module.define_format_views(
            "opentype.layout.mark_record",
            vec![Label::Borrowed("array_view")],
            record_auto([
                ("mark_class", u16be()),
                (
                    "mark_anchor",
                    util::read_phantom_view_offset16(vvar("array_view"), anchor_table.call()),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.mark_array",
            let_view(
                "array_view",
                record([
                    ("array_scope", reify_view(vvar("array_view"))),
                    ("mark_count", u16be()),
                    (
                        "mark_records",
                        repeat_count(
                            var("mark_count"),
                            mark_record.call_views(vec![vvar("array_view")]),
                        ),
                    ),
                ]),
            ),
        )
    }

    #[cfg(feature = "alt")]
    pub(crate) mod alt {
        use super::*;
        use doodle::alt::{self, FormatModuleExt};

        /// Mark array table (alternate definition)
        ///
        /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#mark-array-table
        #[expect(unused)]
        pub(crate) fn mark_array(
            module: &mut FormatModuleExt,
            anchor_table: FormatRef,
        ) -> FormatRef {
            // TODO - refactor into dep-format or standalone function
            let mark_record = module.define_format_args(
                "opentype.layout.mark_record",
                vec![],
                vec![Label::Borrowed("array_view")],
                alt::helper::record_auto([
                    ("mark_class", u16be().into()),
                    (
                        "mark_anchor",
                        util::alt_read_u16be_view_offset(vvar("array_view"), anchor_table.call()),
                    ),
                ]),
            );
            module.define_format(
                "opentype.layout.mark_array",
                let_view(
                    "array_view",
                    record([
                        ("array_scope", reify_view(vvar("array_view"))),
                        ("mark_count", u16be()),
                        (
                            "mark_records",
                            repeat_count(
                                var("mark_count"),
                                mark_record.call_args_views(Vec::new(), vec![vvar("array_view")]),
                            ),
                        ),
                    ]),
                ),
            )
        }
    }

    /// Lookup type 4 subtable: mark-to-base attachment positioning
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-4-subtable-mark-to-base-attachment-positioning
    pub(crate) fn mark_base_pos(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        anchor_table: FormatRef,
        mark_array: FormatRef,
    ) -> FormatRef {
        let base_record = module.define_format_args_views(
            "opentype.layout.base_array.base_record",
            vec![(Label::Borrowed("mark_class_count"), ValueType::U16)],
            vec![Label::Borrowed("_array_view")],
            // REVIEW[epic=many-offsets-design-pattern] - for-each style
            record_auto([
                (
                    "base_anchor_offsets",
                    repeat_count(var("mark_class_count"), u16be()),
                ),
                (
                    "#_base_anchors",
                    // REVIEW - instead of for-each, do we want to express the phantom parse in the offset repeat_count itself?
                    phantom(for_each(
                        var("base_anchor_offsets"),
                        "offset",
                        util::parse_view_offset::<U16>(
                            vvar("_array_view"),
                            var("offset"),
                            anchor_table.call(),
                        ),
                    )),
                ),
            ]),
        );
        let base_array = module.define_format_args(
            "opentype.layout.base_array",
            vec![(Label::Borrowed("mark_class_count"), ValueType::U16)],
            let_view(
                "array_view",
                record_auto([
                    ("array_scope", reify_view(vvar("array_view"))),
                    ("base_count", u16be()),
                    (
                        "base_records",
                        repeat_count(
                            var("base_count"),
                            base_record.call_args_views(
                                vec![var("mark_class_count")],
                                vec![vvar("array_view")],
                            ),
                        ),
                    ),
                ]),
            ),
        );
        module.define_format(
            "opentype.layout.mark_base_pos",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [("format", u16be())],
                    ("format", 1),
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        (
                            "mark_coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        (
                            "base_coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        ("mark_class_count", u16be()),
                        (
                            "mark_array",
                            util::read_phantom_view_offset16(vvar("table_view"), mark_array.call()),
                        ),
                        (
                            "base_array",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                base_array.call_args(vec![var("mark_class_count")]),
                            ),
                        ),
                    ],
                    "pos",
                    "Format1",
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    mod mark_lig {
        use super::*;
        fn component_record(module: &mut FormatModule, anchor_table: FormatRef) -> FormatRef {
            module.define_format_args_views(
                "opentype.layout.ligature_attach.component_record",
                vec![(Label::Borrowed("mark_class_count"), ValueType::U16)],
                vec![Label::Borrowed("table_view")],
                record_auto([
                    // REVIEW[epic=nested-format-reify-layer] - outer-format view reified locally
                    ("record_scope", reify_view(vvar("table_view"))),
                    // REVIEW[epic=many-offsets-design-pattern] - for-each style
                    (
                        "ligature_anchor_offsets",
                        // REVIEW[epic=read-fixedwidth-array] - does ReadArray work better here?
                        repeat_count(var("mark_class_count"), u16be()),
                    ),
                    (
                        "#_ligature_anchors",
                        phantom(for_each(
                            var("ligature_anchor_offsets"),
                            "offset",
                            util::parse_view_offset::<U16>(
                                vvar("table_view"),
                                var("offset"),
                                anchor_table.call(),
                            ),
                        )),
                    ),
                ]),
            )
        }

        fn ligature_attach(module: &mut FormatModule, anchor_table: FormatRef) -> FormatRef {
            let component_record = component_record(module, anchor_table);

            module.define_format_args(
                "opentype.layout.ligature_attach",
                vec![(Label::Borrowed("mark_class_count"), ValueType::U16)],
                let_view(
                    "table_view",
                    record([
                        // REVIEW[epic=nested-format-reify-layer] - scope reified in inner format
                        ("component_count", u16be()),
                        (
                            "component_records",
                            repeat_count(
                                var("component_count"),
                                component_record.call_args_views(
                                    vec![var("mark_class_count")],
                                    vec![vvar("table_view")],
                                ),
                            ),
                        ),
                    ]),
                ),
            )
        }

        /// Subformat definition helper for MarkLigPos LigatureArray
        pub(super) fn ligature_array(
            module: &mut FormatModule,
            anchor_table: FormatRef,
        ) -> FormatRef {
            let ligature_attach = ligature_attach(module, anchor_table);

            module.define_format_args(
                "opentype.layout.ligature_array",
                vec![(Label::Borrowed("mark_class_count"), ValueType::U16)],
                let_view(
                    "array_view",
                    record_auto([
                        ("array_scope", reify_view(vvar("array_view"))),
                        ("ligature_count", u16be()),
                        // REVIEW[epic=many-offsets-design-pattern] - for-each style
                        (
                            "ligature_attach_offsets",
                            repeat_count(var("ligature_count"), u16be()),
                        ),
                        (
                            "#_ligature_attaches",
                            phantom(for_each(
                                var("ligature_attach_offsets"),
                                "offset",
                                util::parse_view_offset::<U16>(
                                    vvar("array_view"),
                                    var("offset"),
                                    ligature_attach.call_args(vec![var("mark_class_count")]),
                                ),
                            )),
                        ),
                    ]),
                ),
            )
        }
    }

    /// Looukup type 5 subtable: mark-to-ligature attachment positioning
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-5-subtable-mark-to-ligature-attachment-positioning
    pub(crate) fn mark_lig_pos(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        anchor_table: FormatRef,
        mark_array: FormatRef,
    ) -> FormatRef {
        let ligature_array = mark_lig::ligature_array(module, anchor_table);

        module.define_format(
            "opentype.layout.mark_lig_pos",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        ("format", u16be()),
                    ],
                    ("format", 1),
                    [
                        (
                            "mark_coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        (
                            "ligature_coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        ("mark_class_count", u16be()),
                        (
                            "mark_array",
                            util::read_phantom_view_offset16(vvar("table_view"), mark_array.call()),
                        ),
                        (
                            "ligature_array",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                ligature_array.call_args(vec![var("mark_class_count")]),
                            ),
                        ),
                    ],
                    "pos",
                    "Format1",
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    mod mark_mark {
        use super::*;

        fn mark2_record(module: &mut FormatModule, anchor_table: FormatRef) -> FormatRef {
            module.define_format_args_views(
                "opentype.layout.mark2_array.mark2_record",
                vec![(Label::Borrowed("mark_class_count"), ValueType::U16)],
                vec![Label::Borrowed("_array_view")],
                record_auto([
                    // REVIEW[epic=many-offsets-design-pattern] - for-each style
                    (
                        "mark2_anchor_offsets",
                        repeat_count(var("mark_class_count"), u16be()),
                    ),
                    // REVIEW - eliminate foreach and fold phantom offsetting into repeat_count ?
                    (
                        "#_mark2_anchors",
                        phantom(for_each(
                            var("mark2_anchor_offsets"),
                            "offset",
                            util::parse_view_offset::<U16>(
                                vvar("_array_view"),
                                var("offset"),
                                anchor_table.call(),
                            ),
                        )),
                    ),
                ]),
            )
        }

        pub(super) fn mark2_array(module: &mut FormatModule, anchor_table: FormatRef) -> FormatRef {
            let mark2_record = mark2_record(module, anchor_table);

            module.define_format_args(
                "opentype.layout.mark2_array",
                vec![(Label::Borrowed("mark_class_count"), ValueType::U16)],
                let_view(
                    "array_view",
                    record([
                        // REVIEW[epic=nested-format-reify-layer] - scope reified locally in outer format
                        ("array_scope", reify_view(vvar("array_view"))),
                        ("mark2_count", u16be()),
                        (
                            "mark2_records",
                            repeat_count(
                                var("mark2_count"),
                                mark2_record.call_args_views(
                                    vec![var("mark_class_count")],
                                    vec![vvar("array_view")],
                                ),
                            ),
                        ),
                    ]),
                ),
            )
        }
    }

    /// Looukup type 6 subtable: mark-to-mark attachment positioning
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-6-subtable-mark-to-mark-attachment-positioning
    pub(crate) fn mark_mark_pos(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        anchor_table: FormatRef,
        mark_array: FormatRef,
    ) -> FormatRef {
        let mark2_array = mark_mark::mark2_array(module, anchor_table);
        module.define_format(
            "opentype.layout.mark_mark_pos",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [("format", u16be())],
                    ("format", 1),
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        (
                            "mark1_coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        (
                            "mark2_coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        ("mark_class_count", u16be()),
                        (
                            "mark1_array",
                            util::read_phantom_view_offset16(vvar("table_view"), mark_array.call()),
                        ),
                        (
                            "mark2_array",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                mark2_array.call_args(vec![var("mark_class_count")]),
                            ),
                        ),
                    ],
                    "pos",
                    "Format1",
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    /// Ground (non-recursive) GPOS lookup subtable type enumeration
    pub(crate) fn ground_pos(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        value_format_flags: FormatRef,
        value_record: FormatRef,
        anchor_table: FormatRef,
        sequence_context: FormatRef,
        chained_sequence_context: FormatRef,
    ) -> FormatRef {
        let single_pos = single_pos(module, coverage_table, value_format_flags, value_record);
        let pair_pos = pair_pos(
            module,
            class_def,
            coverage_table,
            value_format_flags,
            value_record,
        );
        let cursive_pos = cursive_pos(module, coverage_table, anchor_table);
        let mark_array = mark_array(module, anchor_table);
        let mark_base_pos = mark_base_pos(module, coverage_table, anchor_table, mark_array);
        let mark_lig_pos = mark_lig_pos(module, coverage_table, anchor_table, mark_array);
        let mark_mark_pos = mark_mark_pos(module, coverage_table, anchor_table, mark_array);
        module.define_format_args(
            "opentype.layout.ground_pos",
            vec![(Label::from("lookup_type"), ValueType::Base(BaseType::U16))],
            match_variant(
                var("lookup_type"),
                [
                    (Pattern::U16(1), "SinglePos", single_pos.call()),
                    (Pattern::U16(2), "PairPos", pair_pos.call()),
                    (Pattern::U16(3), "CursivePos", cursive_pos.call()),
                    (Pattern::U16(4), "MarkBasePos", mark_base_pos.call()),
                    (Pattern::U16(5), "MarkLigPos", mark_lig_pos.call()),
                    (Pattern::U16(6), "MarkMarkPos", mark_mark_pos.call()),
                    (Pattern::U16(7), "SequenceContext", sequence_context.call()),
                    (
                        Pattern::U16(8),
                        "ChainedSequenceContext",
                        chained_sequence_context.call(),
                    ),
                    (Pattern::U16(9), "NestedExtensionSubtable", Format::Fail),
                    (Pattern::Wildcard, "UnknownLookupSubtable", Format::Fail),
                ],
            ),
        )
    }

    /// Lookup type 9 subtable: positioning suhbtable extension
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-9-subtable-positioning-subtable-extension
    pub(crate) fn pos_extension(module: &mut FormatModule, ground_pos: FormatRef) -> FormatRef {
        module.define_format(
            "opentype.layout.pos_extension",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [("format", u16be())],
                    ("format", 1),
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        (
                            "extension_lookup_type",
                            where_within(u16be(), Bounds::new(1, 8)),
                        ),
                        (
                            "extension_offset",
                            util::read_phantom_view_offset32(
                                vvar("table_view"),
                                ground_pos.call_args(vec![var("extension_lookup_type")]),
                            ),
                        ),
                    ],
                    "pos",
                    "Format1",
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    pub(crate) fn table(
        module: &mut FormatModule,
        script_list: FormatRef,
        feature_list: FormatRef,
        ground_pos: FormatRef,
        pos_extension: FormatRef,
        feature_variations: FormatRef,
    ) -> FormatRef {
        let lookup_subtable = lookup_subtable(module, pos_extension, ground_pos);
        let lookup_table = module.define_format(
            "opentype.gpos.lookup_table",
            layout::lookup_table(lookup_subtable),
        );
        let lookup_list = module.define_format(
            "opentype.gpos.lookup_list",
            layout::lookup_list(lookup_table),
        );
        module.define_format(
            "opentype.gpos_table",
            layout::table(script_list, feature_list, lookup_list, feature_variations),
        )
    }
}

mod gsub {
    use super::*;

    /// GSUB-specific LookupSubtable implementation
    pub(crate) fn lookup_subtable(
        module: &mut FormatModule,
        subst_extension: FormatRef,
        ground_subst: FormatRef,
    ) -> FormatRef {
        const EXTENSION_TYPE: u16 = 7;
        module.define_format_args(
            "opentype.gsub.lookup_subtable",
            vec![(Label::from("lookup_type"), ValueType::U16)],
            match_variant(
                var("lookup_type"),
                [
                    (
                        Pattern::U16(EXTENSION_TYPE),
                        "SubstExtension",
                        subst_extension.call(),
                    ),
                    (
                        Pattern::Wildcard,
                        "GroundSubst",
                        ground_subst.call_args(vec![var("lookup_type")]),
                    ),
                ],
            ),
        )
    }
    /// Lookup type 1 subtable: single substitution
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-1-subtable-single-substitution
    pub(crate) fn single_subst(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
        // Single substitution format 1
        let format1 = module.define_format_views(
            "opentype.layout.single_subst.format1",
            vec![Label::Borrowed("table_view")],
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("delta_glyph_id", util::s16be()),
            ]),
        );
        // Single substitution format 2
        let format2 = module.define_format_views(
            "opentype.layout.single_subst.format2",
            vec![Label::Borrowed("table_view")],
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("glyph_count", u16be()),
                (
                    "substitute_glyph_ids",
                    repeat_count(var("glyph_count"), u16be()),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.single_subst",
            let_view(
                "table_view",
                record([
                    ("subst_format", u16be()),
                    (
                        "subst",
                        match_variant(
                            var("subst_format"),
                            [
                                (
                                    Pattern::U16(1),
                                    "Format1",
                                    format1.call_args_views(vec![], vec![vvar("table_view")]),
                                ),
                                (
                                    Pattern::U16(2),
                                    "Format2",
                                    format2.call_args_views(vec![], vec![vvar("table_view")]),
                                ),
                                // REVIEW[epic=catchall-policy] - do we need this catchall?
                                (Pattern::Wildcard, "BadFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Lookup type 2 subtable: multiple substitution
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-2-subtable-multiple-substitution
    pub(crate) fn multiple_subst(
        module: &mut FormatModule,
        coverage_table: FormatRef,
    ) -> FormatRef {
        let sequence_table = module.define_format(
            "opentype.layout.multiple_subst.sequence_table",
            record([
                // NOTE - formally (according to the spec) this must never be 0, but some fonts ignore this so we don't enforce it as a mandate
                ("glyph_count", u16be()),
                (
                    "substitute_glyph_ids",
                    repeat_count(var("glyph_count"), u16be()),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.multiple_subst",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        ("subst_format", u16be()),
                        (
                            "coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                    ],
                    ("subst_format", 1),
                    [
                        ("sequence_count", u16be()),
                        // REVIEW[epic=many-offsets-design-pattern] - for-each style
                        (
                            "sequence_offsets",
                            repeat_count(var("sequence_count"), u16be()),
                        ),
                        (
                            "#_sequences",
                            phantom(for_each(
                                var("sequence_offsets"),
                                "offset",
                                util::parse_view_offset::<U16>(
                                    vvar("table_view"),
                                    var("offset"),
                                    sequence_table.call(),
                                ),
                            )),
                        ),
                    ],
                    "subst",
                    "Format1",
                    // REVIEW - Consider what style we want to adopt more generally for MultipleSubst, AlternateSubst, LigatureSubst
                    util::NestingKind::MinimalVariation,
                ),
            ),
        )
    }

    /// Lookup type 3 subtable: alternate substitution
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-3-subtable-alternate-substitution
    pub(crate) fn alternate_subst(
        module: &mut FormatModule,
        coverage_table: FormatRef,
    ) -> FormatRef {
        let alternate_set = record([
            ("glyph_count", u16be()),
            (
                "alternate_glyph_ids",
                repeat_count(var("glyph_count"), u16be()),
            ),
        ]);
        module.define_format(
            "opentype.layout.alternate_subst",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        ("subst_format", u16be()),
                        (
                            "coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                    ],
                    ("subst_format", 1),
                    [
                        ("alternate_set_count", u16be()),
                        (
                            "alternate_sets",
                            repeat_count(
                                var("alternate_set_count"),
                                util::read_phantom_view_offset16(vvar("table_view"), alternate_set),
                            ),
                        ),
                    ],
                    "subst",
                    "Format1",
                    // REVIEW - Consider what style we want to adopt more generally for MultipleSubst, AlternateSubst, LigatureSubst
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    /// Loookup type 4 subtable: ligature substitution
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-4-subtable-ligature-substitution
    pub(crate) fn ligature_subst(
        module: &mut FormatModule,
        coverage_table: FormatRef,
    ) -> FormatRef {
        let ligature_table = module.define_format(
            "opentype.gsub.ligature_subst.ligature_table",
            record([
                ("ligature_glyph", u16be()),
                ("component_count", u16be()),
                (
                    "component_glyph_ids",
                    repeat_count(pred(var("component_count")), u16be()),
                ),
            ]),
        );
        let ligature_set = module.define_format(
            "opentype.gsub.ligature_subst.ligature_set",
            let_view(
                "set_view",
                record([
                    ("set_scope", reify_view(vvar("set_view"))),
                    ("ligature_count", u16be()),
                    (
                        "ligatures",
                        repeat_count(
                            var("ligature_count"),
                            util::read_phantom_view_offset16(
                                vvar("set_view"),
                                ligature_table.call(),
                            ),
                        ),
                    ),
                ]),
            ),
        );
        module.define_format(
            "opentype.layout.ligature_subst",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        ("subst_format", u16be()),
                        (
                            "coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                    ],
                    ("subst_format", 1),
                    [
                        ("ligature_set_count", u16be()),
                        (
                            "ligature_sets",
                            repeat_count(
                                var("ligature_set_count"),
                                util::read_phantom_view_offset16(
                                    vvar("table_view"),
                                    ligature_set.call(),
                                ),
                            ),
                        ),
                    ],
                    "subst",
                    "Format1",
                    // REVIEW - Consider what style we want to adopt more generally for MultipleSubst, AlternateSubst, LigatureSubst
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    /// Lookup type 8 subtable: reverse chained contexts single substitution
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-8-subtable-reverse-chained-contexts-single-substitution
    pub(crate) fn reverse_chain_single_subst(
        module: &mut FormatModule,
        coverage_table: FormatRef,
    ) -> FormatRef {
        module.define_format(
            "opentype.layout.reverse_chain_single_subst",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [("subst_format", u16be())],
                    ("subst_format", 1),
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        (
                            "coverage",
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                        ("backtrack_glyph_count", u16be()),
                        (
                            "backtrack_coverage_tables",
                            repeat_count(
                                var("backtrack_glyph_count"),
                                util::read_phantom_view_offset16(
                                    vvar("table_view"),
                                    coverage_table.call(),
                                ),
                            ),
                        ),
                        ("lookahead_glyph_count", u16be()),
                        (
                            "lookahead_coverage_tables",
                            repeat_count(
                                var("lookahead_glyph_count"),
                                util::read_phantom_view_offset16(
                                    vvar("table_view"),
                                    coverage_table.call(),
                                ),
                            ),
                        ),
                        ("glyph_count", u16be()),
                        (
                            "substitute_glyph_ids",
                            repeat_count(var("glyph_count"), u16be()),
                        ),
                    ],
                    "subst",
                    "Format1",
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    /// Ground (non-recursive) GSUB lookup subtable type enumeration
    pub(crate) fn ground_subst(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        sequence_context: FormatRef,
        chained_sequence_context: FormatRef,
    ) -> FormatRef {
        let single_subst = single_subst(module, coverage_table);
        let multiple_subst = multiple_subst(module, coverage_table);
        let alternate_subst = alternate_subst(module, coverage_table);
        let ligature_subst = ligature_subst(module, coverage_table);
        let reverse_chain_single_subst = reverse_chain_single_subst(module, coverage_table);
        module.define_format_args(
            "opentype.layout.ground_subst",
            vec![(Label::from("lookup_type"), ValueType::Base(BaseType::U16))],
            match_variant(
                var("lookup_type"),
                [
                    (Pattern::U16(1), "SingleSubst", single_subst.call()),
                    (Pattern::U16(2), "MultipleSubst", multiple_subst.call()),
                    (Pattern::U16(3), "AlternateSubst", alternate_subst.call()),
                    (Pattern::U16(4), "LigatureSubst", ligature_subst.call()),
                    (Pattern::U16(5), "SequenceContext", sequence_context.call()),
                    (
                        Pattern::U16(6),
                        "ChainedSequenceContext",
                        chained_sequence_context.call(),
                    ),
                    (
                        Pattern::U16(8),
                        "ReverseChainSingleSubst",
                        reverse_chain_single_subst.call(),
                    ),
                    (Pattern::U16(7), "NestedExtensionSubtable", Format::Fail),
                    (Pattern::Wildcard, "UnknownLookupSubtable", Format::Fail),
                ],
            ),
        )
    }

    /// Lookup type 7 subtable: extension substitution
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-7-subtable-substitution-subtable-extension
    pub(crate) fn subst_extension(module: &mut FormatModule, ground_subst: FormatRef) -> FormatRef {
        module.define_format(
            "opentype.layout.subst_extension",
            let_view(
                "table_view",
                util::embedded_singleton_alternation(
                    [("format", u16be())],
                    ("format", 1),
                    [
                        ("table_scope", reify_view(vvar("table_view"))),
                        (
                            "extension_lookup_type",
                            where_within_any(u16be(), [Bounds::new(1, 6), Bounds::exact(8)]),
                        ),
                        (
                            "extension_offset",
                            util::read_phantom_view_offset32(
                                vvar("table_view"),
                                ground_subst.call_args(vec![var("extension_lookup_type")]),
                            ),
                        ),
                    ],
                    "subst",
                    "Format1",
                    util::NestingKind::UnifiedRecord,
                ),
            ),
        )
    }

    pub(crate) fn table(
        module: &mut FormatModule,
        script_list: FormatRef,
        feature_list: FormatRef,
        ground_subst: FormatRef,
        subst_extension: FormatRef,
        feature_variations: FormatRef,
    ) -> FormatRef {
        let lookup_subtable = lookup_subtable(module, subst_extension, ground_subst);
        let lookup_table = module.define_format(
            "opentype.gsub.lookup_table",
            layout::lookup_table(lookup_subtable),
        );
        let lookup_list = module.define_format(
            "opentype.gsub.lookup_list",
            layout::lookup_list(lookup_table),
        );
        module.define_format(
            "opentype.gsub_table",
            layout::table(script_list, feature_list, lookup_list, feature_variations),
        )
    }
}

/// Module for sub-formats used in both GSUB and GPOS
mod layout {
    use super::*;

    /// Format definition for ChainedSequenceContext tables
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#common-formats-for-contextual-lookup-subtables
    pub(crate) fn chained_sequence_context(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        let rule_set = chained_sequence_rule_set(module, sequence_lookup_record);
        let format1 = chained_sequence_context_format1(module, coverage_table, rule_set);
        let format2 = chained_sequence_context_format2(module, class_def, coverage_table, rule_set);
        let format3 =
            chained_sequence_colntext_format3(module, coverage_table, sequence_lookup_record);
        module.define_format(
            "opentype.layout.chained_sequence_context",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("format", u16be()),
                    (
                        "subst", // REVIEW - this is a GSUB-biased field-name, do we have a better field-name for this?
                        match_variant(
                            var("format"),
                            [
                                (
                                    Pattern::U16(1),
                                    "Format1",
                                    format1.call_views(vec![vvar("table_view")]),
                                ),
                                (
                                    Pattern::U16(2),
                                    "Format2",
                                    format2.call_views(vec![vvar("table_view")]),
                                ),
                                (
                                    Pattern::U16(3),
                                    "Format3",
                                    format3.call_views(vec![vvar("table_view")]),
                                ),
                                // REVIEW[epic=catchall-policy] - do we need this catch-all?
                                (Pattern::Wildcard, "BadFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Format definition for `ChainedSequenceRuleSet` table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-1-simple-glyph-contexts
    fn chained_sequence_rule_set(
        module: &mut FormatModule,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        let chained_sequence_rule = chained_sequence_rule(module, sequence_lookup_record);
        module.define_format(
            "opentype.layout.chained-sequence-rule-set",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("chained_seq_rule_count", u16be()),
                    (
                        "chained_seq_rules",
                        repeat_count(
                            var("chained_seq_rule_count"),
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                chained_sequence_rule.call(),
                            ),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Format definition for `ChainedSequenceRule` table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-1-simple-glyph-contexts
    fn chained_sequence_rule(
        module: &mut FormatModule,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        module.define_format(
            "opentype.layout.chained-sequence-rule",
            record([
                ("backtrack_glyph_count", u16be()),
                (
                    "backtrack_sequence",
                    repeat_count(var("backtrack_glyph_count"), u16be()),
                ),
                ("input_glyph_count", u16be()),
                (
                    "input_sequence",
                    repeat_count(pred(var("input_glyph_count")), u16be()),
                ),
                ("lookahead_glyph_count", u16be()),
                (
                    "lookahead_sequence",
                    repeat_count(var("lookahead_glyph_count"), u16be()),
                ),
                ("seq_lookup_count", u16be()),
                (
                    "seq_lookup_records",
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        )
    }

    /// Format definition for ChainedSequenceContext Format 1
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-1-simple-glyph-contexts
    fn chained_sequence_context_format1(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        rule_set: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.chained-sequence-context.format1",
            vec![(Label::Borrowed("table_view"))],
            record([
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("chained_seq_rule_set_count", u16be()),
                (
                    "chained_seq_rule_sets",
                    repeat_count(
                        var("chained_seq_rule_set_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), rule_set.call()),
                    ),
                ),
            ]),
        )
    }

    /// Format definition for ChainedSequenceContext Format 2
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-2-class-based-glyph-contexts
    fn chained_sequence_context_format2(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        rule_set: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.chained-sequence-context.format2",
            vec![(Label::Borrowed("table_view"))],
            record([
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                (
                    "backtrack_class_def",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                (
                    "input_class_def",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                (
                    "lookahead_class_def",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                ("chained_class_seq_rule_set_count", u16be()),
                (
                    "chained_class_seq_rule_sets",
                    repeat_count(
                        var("chained_class_seq_rule_set_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), rule_set.call()),
                    ),
                ),
            ]),
        )
    }

    /// Format definition for ChainedSequenceContext Format 3
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-3-coverage-based-glyph-contexts
    fn chained_sequence_colntext_format3(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.chained-sequence-context.format3",
            vec![(Label::Borrowed("table_view"))],
            record([
                ("backtrack_glyph_count", u16be()),
                (
                    "backtrack_coverages",
                    repeat_count(
                        var("backtrack_glyph_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                ),
                ("input_glyph_count", u16be()),
                (
                    "input_coverages",
                    repeat_count(
                        var("input_glyph_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                ),
                ("lookahead_glyph_count", u16be()),
                (
                    "lookahead_coverages",
                    repeat_count(
                        var("lookahead_glyph_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                ),
                ("seq_lookup_count", u16be()),
                (
                    "seq_lookup_records",
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        )
    }

    pub(crate) fn sequence_context(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        let rule_set = { rule_set(module, sequence_lookup_record) };
        let format1 = module.define_format_views(
            "opentype.layout.sequence-context.format1",
            vec![(Label::Borrowed("table_view"))],
            record([
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                ("seq_rule_set_count", u16be()),
                (
                    "seq_rule_sets",
                    repeat_count(
                        var("seq_rule_set_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), rule_set.call()),
                    ),
                ),
            ]),
        );
        let format2 = module.define_format_views(
            "opentype.layout.sequence-context.format2",
            vec![(Label::Borrowed("table_view"))],
            record([
                (
                    "coverage",
                    util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                ),
                (
                    "class_def",
                    util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                ),
                ("class_seq_rule_set_count", u16be()),
                (
                    "class_seq_rule_sets",
                    repeat_count(
                        var("class_seq_rule_set_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), rule_set.call()),
                    ),
                ),
            ]),
        );
        let format3 = module.define_format_views(
            "opentype.layout.sequence-context.format3",
            vec![(Label::Borrowed("table_view"))],
            record([
                ("glyph_count", u16be()),
                ("seq_lookup_count", u16be()),
                (
                    "coverage_tables",
                    repeat_count(
                        var("glyph_count"),
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                ),
                (
                    "seq_lookup_records",
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.sequence_context",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("format", u16be()),
                    (
                        // FIXME - this name is biased to GSUB, is there a better identifier?
                        "subst",
                        match_variant(
                            var("format"),
                            [
                                (
                                    Pattern::U16(1),
                                    "Format1",
                                    format1.call_views(vec![vvar("table_view")]),
                                ),
                                (
                                    Pattern::U16(2),
                                    "Format2",
                                    format2.call_views(vec![vvar("table_view")]),
                                ),
                                (
                                    Pattern::U16(3),
                                    "Format3",
                                    format3.call_views(vec![vvar("table_view")]),
                                ),
                                // REVIEW[epic=catchall-policy] - do we need this catchall
                                (Pattern::Wildcard, "BadFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    fn rule_set(module: &mut FormatModule, sequence_lookup_record: FormatRef) -> FormatRef {
        let rule = module.define_format(
            "opentype.layout.sequence-context.rule",
            record([
                ("glyph_count", where_nonzero::<U16>(u16be())),
                ("seq_lookup_count", u16be()),
                (
                    "input_sequence",
                    repeat_count(pred(var("glyph_count")), u16be()),
                ),
                (
                    "seq_lookup_records",
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.sequence-context.rule-set",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("rule_count", u16be()),
                    (
                        "rules",
                        repeat_count(
                            var("rule_count"),
                            util::read_phantom_view_offset16(vvar("table_view"), rule.call()),
                        ),
                    ),
                ]),
            ),
        )
    }
    /// Format definition for `SequenceLookup`
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#sequence-lookup-record
    pub(crate) fn sequence_lookup_record(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.layout.sequence_lookup",
            record([("sequence_index", u16be()), ("lookup_list_index", u16be())]),
        )
    }

    /// Format definition for `FeatureList` table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#featurelist-table
    pub(crate) fn feature_list(
        module: &mut FormatModule,
        tag: FormatRef,
        feature_table: FormatRef,
    ) -> FormatRef {
        let feature_record = feature_record(module, tag, feature_table);
        module.define_format(
            "opentype.layout.feature_list",
            let_view(
                "list_view",
                record([
                    ("list_scope", reify_view(vvar("list_view"))),
                    ("feature_count", u16be()),
                    (
                        "feature_records",
                        repeat_count(
                            var("feature_count"),
                            feature_record.call_args_views(vec![], vec![vvar("list_view")]),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Format definition for `FeatureRecord`, used for FeatureList table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#featurelist-table
    fn feature_record(
        module: &mut FormatModule,
        tag: FormatRef,
        feature_table: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.feature_record",
            vec![Label::Borrowed("list_view")],
            record([
                ("feature_tag", tag.call()),
                (
                    "feature",
                    util::read_phantom_view_offset16(vvar("list_view"), feature_table.call()),
                ),
            ]),
        )
    }

    /// Format definition for `FeatureTable`
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#feature-table
    pub(crate) fn feature_table(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.layout.feature_table",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    // WIP - `feature_params` is technically an offset16 but we don't have a good handle on what data is stored at the offset, or what FeatureRecord tags allow for parameters
                    ("feature_params", u16be()), // TODO - format of params table depends on the feature tag,
                    ("lookup_index_count", u16be()),
                    // Array of 0-based indices into LookupList (first lookup at LookupListIndex = 0)
                    (
                        "lookup_list_indices",
                        repeat_count(var("lookup_index_count"), u16be()),
                    ),
                ]),
            ),
        )
    }

    /// Format definition for ScriptRecord, used in ScriptList table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#scriptlist-table
    fn script_record(
        module: &mut FormatModule,
        tag: FormatRef,
        script_table: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.script_record",
            vec![Label::Borrowed("table_view")],
            record([
                ("script_tag", tag.call()),
                (
                    "script",
                    util::read_phantom_view_offset16(vvar("table_view"), script_table.call()),
                ),
            ]),
        )
    }

    /// Format definition for a ScriptList
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#scriptlist-table
    pub(crate) fn script_list(
        module: &mut FormatModule,
        tag: FormatRef,
        script_table: FormatRef,
    ) -> FormatRef {
        let script_record = script_record(module, tag, script_table);
        module.define_format(
            "opentype.layout.script_list",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("script_count", u16be()),
                    (
                        "script_records",
                        repeat_count(
                            var("script_count"),
                            script_record.call_args_views(vec![], vec![vvar("table_view")]),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Format definition for the Script-tables (elemments of a ScriptList)
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#script-table
    pub(crate) fn script_table(
        module: &mut FormatModule,
        tag: FormatRef,
        lang_sys: FormatRef,
    ) -> FormatRef {
        let lang_sys_record = lang_sys_record(module, tag, lang_sys);
        module.define_format(
            "opentype.layout.script_table",
            let_view(
                "script_view",
                record([
                    ("script_scope", reify_view(vvar("script_view"))),
                    (
                        "default_lang_sys",
                        util::read_phantom_view_offset16(vvar("script_view"), lang_sys.call()),
                    ),
                    ("lang_sys_count", u16be()),
                    (
                        "lang_sys_records",
                        repeat_count(
                            var("lang_sys_count"),
                            lang_sys_record.call_args_views(vec![], vec![vvar("script_view")]),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Tests whether any flag of a`value_format_flag` record is set.
    fn any_set(flags: Expr) -> Expr {
        balance_merge(
            [
                "x_placement",
                "y_placement",
                "x_advance",
                "y_advance",
                "x_placement_device",
                "y_placement_device",
                "x_advance_device",
                "y_advance_device",
            ],
            |fields| {
                fields
                    .into_iter()
                    .map(|field| record_proj(flags.clone(), field))
                    .collect()
            },
            or,
        )
    }

    pub(crate) fn optional_value_record(
        value_record: FormatRef,
        table_view: ViewExpr,
        flags: Expr,
    ) -> Format {
        cond_maybe(
            any_set(flags.clone()),
            value_record.call_args_views(vec![flags], vec![table_view]),
        )
    }

    pub(crate) fn value_format_flags(module: &mut FormatModule) -> FormatRef {
        use BitFieldKind::*;
        module.define_format(
            "opentype.layout.value-format-flags",
            bit_fields_u16([
                Reserved {
                    bit_width: 8,
                    check_zero: true,
                },
                FlagBit("y_advance_device"),
                FlagBit("x_advance_device"),
                FlagBit("y_placement_device"),
                FlagBit("x_placement_device"),
                FlagBit("y_advance"),
                FlagBit("x_advance"),
                FlagBit("y_placement"),
                FlagBit("x_placement"),
            ]),
        )
    }

    pub(crate) fn value_record(
        module: &mut FormatModule,
        device_or_variation_index_table: FormatRef,
        vf_flags_type: ValueType,
    ) -> FormatRef {
        let opt_field = |field_name: &'static str, format: Format| {
            (
                field_name,
                cond_maybe(record_proj(var("flags"), field_name), format),
            )
        };
        module.define_format_args_views(
            "opentype.layout.value_record",
            vec![(Label::Borrowed("flags"), vf_flags_type.clone())],
            vec![Label::Borrowed("table_view")],
            record([
                opt_field("x_placement", util::s16be()),
                opt_field("y_placement", util::s16be()),
                opt_field("x_advance", util::s16be()),
                opt_field("y_advance", util::s16be()),
                opt_field(
                    "x_placement_device",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        device_or_variation_index_table.call(),
                    ),
                ),
                opt_field(
                    "y_placement_device",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        device_or_variation_index_table.call(),
                    ),
                ),
                opt_field(
                    "x_advance_device",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        device_or_variation_index_table.call(),
                    ),
                ),
                opt_field(
                    "y_advance_device",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        device_or_variation_index_table.call(),
                    ),
                ),
            ]),
        )
    }

    /// Format registration for LangSysRecord
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#script-table
    pub(crate) fn lang_sys_record(
        module: &mut FormatModule,
        tag: FormatRef,
        lang_sys: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.lang_sys_record",
            vec![(Label::Borrowed("script_view"))],
            record([
                ("lang_sys_tag", tag.call()),
                (
                    "lang_sys",
                    util::read_phantom_view_offset16(vvar("script_view"), lang_sys.call()),
                ),
            ]),
        )
    }

    /// LangSys (language system) table definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#language-system-table
    pub(crate) fn lang_sys(module: &mut FormatModule) -> FormatRef {
        // Language System Table
        module.define_format(
            "opentype.layout.langsys",
            record([
                ("lookup_order_offset", util::expect_u16be(0x0000)), // RESERVED - set to NULL [Offset16 type but it doesn't point to anything]
                ("required_feature_index", u16be()), // 0xFFFF if no features required
                ("feature_index_count", u16be()),
                (
                    "feature_indices",
                    repeat_count(var("feature_index_count"), u16be()),
                ),
            ]),
        )
    }

    pub(crate) fn anchor_table(
        module: &mut FormatModule,
        device_or_variation_index_table: FormatRef,
    ) -> FormatRef {
        // REVIEW - should formats 1 and 2 be defined as well?
        let anchor_format1 = record([
            ("x_coordinate", util::s16be()),
            ("y_coordinate", util::s16be()),
        ]);
        let anchor_format2 = record([
            ("x_coordinate", util::s16be()),
            ("y_coordinate", util::s16be()),
            ("anchor_point", u16be()),
        ]);
        // REVIEW[epic=closure-dep-formats] - should this be a Dep-Format registration (module.define_format_args) instead?
        let anchor_format3 = module.define_format_views(
            "opentype.layout.anchor_table.format3",
            vec![Label::Borrowed("table_view")],
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                ("x_coordinate", util::s16be()),
                ("y_coordinate", util::s16be()),
                // REVIEW - each offset below is individually nullable if the other is set, but it may be invalid for them to both be null simultaneously...?
                (
                    "x_device",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        device_or_variation_index_table.call(),
                    ),
                ),
                (
                    "y_device",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        device_or_variation_index_table.call(),
                    ),
                ),
            ]),
        );
        module.define_format(
            "opentype.layout.anchor_table",
            let_view(
                "table_view",
                record([
                    ("anchor_format", u16be()),
                    (
                        "table",
                        match_variant(
                            var("anchor_format"),
                            [
                                (Pattern::U16(1), "Format1", anchor_format1),
                                (Pattern::U16(2), "Format2", anchor_format2),
                                (
                                    Pattern::U16(3),
                                    "Format3",
                                    anchor_format3
                                        .call_args_views(vec![], vec![vvar("table_view")]),
                                ),
                                // REVIEW[epic=catchall-policy] - do we need this catchall?
                                (Pattern::Wildcard, "BadFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Feature Variation Record
    fn feature_variation_record(module: &mut FormatModule, feature_table: FormatRef) -> FormatRef {
        let condition_table = util::embedded_singleton_alternation(
            [("format", u16be())],
            ("format", 1),
            [
                ("axis_index", u16be()),
                ("filter_range_min_value", util::f2dot14()),
                ("filter_range_max_value", util::f2dot14()),
            ],
            "cond",
            "Format1",
            util::NestingKind::UnifiedRecord,
        );
        let condition_set = let_view(
            "set_view",
            record_auto([
                ("set_scope", reify_view(vvar("set_view"))),
                ("condition_count", u16be()),
                // REVIEW[epic=many-offsets-design-pattern] - for-each style
                (
                    "condition_offsets",
                    repeat_count(var("condition_count"), u32be()),
                ),
                (
                    "#_conditions",
                    phantom(for_each(
                        var("condition_offsets"),
                        "offset",
                        util::parse_view_offset::<U32>(
                            vvar("set_view"),
                            var("offset"),
                            condition_table,
                        ),
                    )),
                ),
            ]),
        );

        let feature_table_substitution_record = module.define_format_views(
            "opentype.layout.feature-table-substitution-record",
            vec![Label::Borrowed("_table_view")],
            record([
                ("feature_index", u16be()),
                ("alternate_feature_offset", u32be()),
                (
                    "alternate_feature",
                    phantom(util::parse_view_offset::<U32>(
                        vvar("_table_view"),
                        var("alternate_feature_offset"),
                        feature_table.call(),
                    )),
                ),
            ]),
        );
        let feature_table_substitution = module.define_format(
            "opentype.layout.feature-table-substitution",
            let_view(
                "table_view",
                record_auto([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("major_version", util::expect_u16be(1)),
                    ("minor_version", util::expect_u16be(0)),
                    ("substitution_count", u16be()),
                    (
                        "substitutions",
                        repeat_count(
                            var("substitution_count"),
                            feature_table_substitution_record.call_views(vec![vvar("table_view")]),
                        ),
                    ),
                ]),
            ),
        );
        module.define_format_views(
            "opentype.layout.feature-variation-record",
            vec![Label::Borrowed("table_view")],
            record([
                (
                    "condition_set",
                    util::read_phantom_view_offset32(vvar("table_view"), condition_set),
                ),
                (
                    "feature_table_substitution",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        feature_table_substitution.call(),
                    ),
                ),
            ]),
        )
    }

    /// FeatureVariations table
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#featVarTbl
    pub(crate) fn feature_variations(
        module: &mut FormatModule,
        feature_table: FormatRef,
    ) -> FormatRef {
        let feature_variation_record = feature_variation_record(module, feature_table);

        module.define_format(
            "opentype.layout.feature_variations",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("major_version", util::expect_u16be(1)),
                    ("minor_version", util::expect_u16be(0)),
                    ("feature_variation_record_count", u32be()),
                    (
                        "feature_variation_records",
                        repeat_count(
                            var("feature_variation_record_count"),
                            feature_variation_record
                                .call_args_views(vec![], vec![vvar("table_view")]),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Format-factory taking a `{GPOS,GSUB}`-specific `lookup_subtable` and constructing the shape of a LookupTable around it
    pub(crate) fn lookup_table(lookup_subtable: FormatRef) -> Format {
        // NOTE - tag is a model-external value, lookup-type is model-internal.

        let lookup_flag = {
            use BitFieldKind::*;
            // REVIEW[epic=check-zero] - consider whether this should be set to true
            const SHOULD_CHECK_ZERO: bool = false;
            bit_fields_u16([
                BitsField {
                    bit_width: 8,
                    field_name: "mark_attachment_class_filter",
                },
                Reserved {
                    bit_width: 3,
                    check_zero: SHOULD_CHECK_ZERO,
                },
                FlagBit("use_mark_filtering_set"), // Bit 4 (0x10) - indicator flag for presence of markFilteringSet field in Lookup table structure
                FlagBit("ignore_marks"), // Bit 3 (0x8) - if set, skips  over combining marks
                FlagBit("ignore_ligatures"), // Bit 2 (0x4) - if set, skips over ligatures
                FlagBit("ignore_base_glyphs"), // Bit 1 (0x2) - if set, skips over base glyphs
                FlagBit("right_to_left"), // Bit 0 (0x1) - [GPOS type 3 only] when set, last glyph matched input will be positioned on baseline
            ])
        };
        let_view(
            "table_view",
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                ("lookup_type", u16be()),
                ("lookup_flag", lookup_flag),
                ("sub_table_count", u16be()),
                (
                    "subtables",
                    repeat_count(
                        var("sub_table_count"),
                        util::read_phantom_view_offset16(
                            vvar("table_view"),
                            lookup_subtable.call_args(vec![var("lookup_type")]),
                        ),
                    ),
                ),
                (
                    "mark_filtering_set",
                    if_then_else(
                        record_proj(var("lookup_flag"), "use_mark_filtering_set"),
                        fmt_some(u16be()),
                        fmt_none(),
                    ),
                ),
            ]),
        )
    }

    /// LookupList table
    ///
    /// Takes `lookup_table` as a GPOS/GSUB-specific definition of LookupTable (via [`lookup_table`])
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#lookuplist-table
    pub(crate) fn lookup_list(lookup_table: FormatRef) -> Format {
        let_view(
            "list_view",
            record([
                ("list_scope", reify_view(vvar("list_view"))),
                ("lookup_count", u16be()),
                (
                    "lookups",
                    repeat_count(
                        var("lookup_count"),
                        util::read_phantom_view_offset16(vvar("list_view"), lookup_table.call()),
                    ),
                ),
            ]),
        )
    }

    /// Factory funtion used for defining GPOS and GSUB table-formats
    pub(crate) fn table(
        script_list: FormatRef,
        feature_list: FormatRef,
        lookup_list: FormatRef,
        feature_variations: FormatRef,
    ) -> Format {
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("major_version", util::expect_u16be(1)),
                ("minor_version", u16be()),
                (
                    "script_list",
                    util::read_phantom_view_offset16(vvar("table_view"), script_list.call()),
                ),
                (
                    "feature_list",
                    util::read_phantom_view_offset16(vvar("table_view"), feature_list.call()),
                ),
                (
                    "lookup_list",
                    util::read_phantom_view_offset16(vvar("table_view"), lookup_list.call()),
                ),
                (
                    "feature_variations_offset",
                    cond_maybe(
                        expr_gt(var("minor_version"), Expr::U16(0)), // Since Major == 1 by assertion, minor > 0 implies v1.1 or (as yet unimplemented) greater
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            feature_variations.call(),
                        ),
                    ),
                ),
            ]),
        )
    }
}

mod gdef {
    use super::*;

    pub(crate) fn table(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        device_or_variation_index_table: FormatRef,
        item_variation_store: FormatRef,
    ) -> FormatRef {
        let mark_glyph_set = mark_glyph_set(module, coverage_table);
        let gdef_header_version_1_2 = |table_view: ViewExpr| {
            record([(
                "mark_glyph_sets_def",
                util::read_phantom_view_offset16(table_view, mark_glyph_set.call()),
            )])
        };
        let gdef_header_version_1_3 = |table_view: ViewExpr| {
            record([
                (
                    "mark_glyph_sets_def",
                    util::read_phantom_view_offset16(table_view.clone(), mark_glyph_set.call()),
                ),
                (
                    "item_var_store",
                    util::read_phantom_view_offset32(table_view, item_variation_store.call()),
                ),
            ])
        };
        let attach_list = attach_list(module, coverage_table);
        let lig_caret_list =
            lig_caret_list(module, coverage_table, device_or_variation_index_table);
        module.define_format(
            "opentype.gdef.table",
            let_view(
                "table_view",
                record([
                    // Starting offset of `GDEF` table
                    ("table_scope", reify_view(vvar("table_view"))),
                    // Major Version of `GDEF` table - only 1[.x] defined
                    ("major_version", util::expect_u16be(1)), // NOTE - only major version 1 is defined: https://learn.microsoft.com/en-us/typography/opentype/spec/gdef#gdef-table-structures
                    // Minor Version (can be [1.]0, [1.]2, or [1.]3)
                    ("minor_version", u16be()),
                    // Class definition table for glyph type (may be NULL)
                    (
                        "glyph_class_def",
                        util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                    ),
                    // Attachment point list table (may be NULL)
                    (
                        "attach_list",
                        util::read_phantom_view_offset16(vvar("table_view"), attach_list.call()),
                    ),
                    // Ligature caret list table (may be NULL)
                    (
                        "lig_caret_list",
                        util::read_phantom_view_offset16(vvar("table_view"), lig_caret_list.call()),
                    ),
                    // Class definition table for mark attachment type (may be NULL)
                    (
                        "mark_attach_class_def",
                        util::read_phantom_view_offset16(vvar("table_view"), class_def.call()),
                    ),
                    // Version-specific data, if > 1.0
                    // REVIEW - do we want to flatten this variant abstraction into two Option<...> fields instead?
                    (
                        "data",
                        match_variant(
                            var("minor_version"),
                            [
                                (Pattern::U16(0), "Version1_0", Format::EMPTY),
                                // NOTE - the variant `Version1_1` will not actually appear in the generated type due to Void-pruning
                                (Pattern::U16(1), "Version1_1", Format::Fail), // FIXME - should this be EMPTY instead?
                                (
                                    Pattern::U16(2),
                                    "Version1_2",
                                    gdef_header_version_1_2(vvar("table_view")),
                                ),
                                (
                                    Pattern::U16(3),
                                    "Version1_3",
                                    gdef_header_version_1_3(vvar("table_view")),
                                ),
                                // NOTE - this case covers everything after version 1.3 - following the Fathom definition that falls back onto the latest version we support
                                (
                                    Pattern::Wildcard,
                                    "Version1_3",
                                    gdef_header_version_1_3(vvar("table_view")),
                                ),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    fn lig_caret_list(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        device_or_variation_index_table: FormatRef,
    ) -> FormatRef {
        let caret_value = caret_value(module, device_or_variation_index_table);
        let lig_glyph = module.define_format(
            "opentype.gdef.lig_glyph",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("caret_count", u16be()),
                    (
                        "caret_values",
                        repeat_count(
                            var("caret_count"),
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                caret_value.call(),
                            ),
                        ),
                    ),
                ]),
            ),
        );
        module.define_format(
            "opentype.gdef.lig_caret_list",
            let_view(
                "list_view",
                record([
                    ("list_scope", reify_view(vvar("list_view"))),
                    (
                        "coverage",
                        util::read_phantom_view_offset16(vvar("list_view"), coverage_table.call()),
                    ),
                    ("lig_glyph_count", u16be()),
                    (
                        "lig_glyph_offsets",
                        repeat_count(
                            var("lig_glyph_count"),
                            util::read_phantom_view_offset16(vvar("list_view"), lig_glyph.call()),
                        ),
                    ),
                ]),
            ),
        )
    }

    fn caret_value(
        module: &mut FormatModule,
        device_or_variation_index_table: FormatRef,
    ) -> FormatRef {
        let caret_value_format_1 = record([("coordinate", util::s16be())]);

        let caret_value_format_2 = record([("caret_value_point_index", u16be())]);

        let caret_value_format_3 = |table_view: ViewExpr| {
            record([
                ("coordinate", util::s16be()),
                (
                    "table",
                    util::read_phantom_view_offset16(
                        table_view,
                        device_or_variation_index_table.call(),
                    ),
                ),
            ])
        };

        module.define_format(
            "opentype.gdef.caret_value",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("caret_value_format", u16be()),
                    (
                        "data",
                        match_variant(
                            var("caret_value_format"),
                            [
                                (Pattern::U16(1), "Format1", caret_value_format_1),
                                (Pattern::U16(2), "Format2", caret_value_format_2),
                                (
                                    Pattern::U16(3),
                                    "Format3",
                                    caret_value_format_3(vvar("table_view")),
                                ),
                                // REVIEW[epic=catchall-policy] - do we need this catch-all?
                                (Pattern::Wildcard, "BadFormat", Format::Fail),
                            ],
                        ),
                    ),
                ]),
            ),
        )
    }

    fn attach_list(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
        let attach_point_table = record([
            ("point_count", u16be()),
            ("point_indices", repeat_count(var("point_count"), u16be())),
        ]);

        module.define_format(
            "opentype.gdef.attach_list",
            let_view(
                "list_view",
                record([
                    ("list_scope", reify_view(vvar("list_view"))),
                    (
                        "coverage",
                        util::read_phantom_view_offset16(vvar("list_view"), coverage_table.call()),
                    ),
                    ("glyph_count", u16be()),
                    (
                        "attach_point_offsets",
                        repeat_count(
                            var("glyph_count"),
                            util::read_phantom_view_offset16(vvar("list_view"), attach_point_table),
                        ),
                    ),
                ]),
            ),
        )
    }

    fn mark_glyph_set(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
        module.define_format(
            "opentype.gdef.mark_glyph_set",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("format", util::expect_u16be(1)), // FIXME - u16be() instead if this is validation fails
                    ("mark_glyph_set_count", u16be()),
                    (
                        "coverage",
                        repeat_count(
                            var("mark_glyph_set_count"),
                            util::read_phantom_view_offset32(
                                vvar("table_view"),
                                coverage_table.call(),
                            ),
                        ),
                    ),
                ]),
            ),
        )
    }
}

mod common {
    use super::*;

    pub(crate) fn item_variation_store(module: &mut FormatModule) -> FormatRef {
        let variation_region_list = {
            // NOTE - all coordinates should be in range [-1.0, +1.0], and start <= peak <= end; must either all be non-positive or non-negative, or else peak must be 0 for negative start and non-negative end.
            let region_axis_coordinates =
                record_repeat(["start_coord", "peak_coord", "end_coord"], util::f2dot14());
            let variation_region = |axis_count: Expr| {
                record([(
                    "region_axes",
                    repeat_count(axis_count, region_axis_coordinates),
                )])
            };
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
            ])
        };
        let item_variation_data = item_variation_data();
        module.define_format(
            "opentype.common.item_variation_store",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("format", util::expect_u16be(1)),
                    (
                        "variation_region_list_offset",
                        util::read_phantom_view_offset32(vvar("table_view"), variation_region_list),
                    ),
                    ("item_variation_data_count", u16be()),
                    (
                        "item_variation_data_offsets",
                        repeat_count(
                            var("item_variation_data_count"),
                            util::read_phantom_view_offset32(
                                vvar("table_view"),
                                item_variation_data,
                            ),
                        ),
                    ),
                ]),
            ),
        )
    }

    /// Item Variation Store - deltas array
    fn deltas(
        full_format: Format,
        half_format: Format,
        word_count: Expr,
        region_index_count: Expr,
    ) -> Format {
        record([
            // FIXME - due to implementation limits, currently broken into two separate arrays rather than fused together
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

    pub(crate) fn item_variation_data() -> Format {
        let delta_sets = |item_count: Expr, word_delta_count: Expr, region_index_count: Expr| {
            if_then_else(
                record_proj(word_delta_count.clone(), "long_words"),
                fmt_variant(
                    "Delta32Sets",
                    repeat_count(
                        item_count.clone(),
                        deltas(
                            util::s32be(),
                            util::s16be(),
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
                            util::s16be(),
                            util::s8(),
                            record_proj(word_delta_count.clone(), "word_count"),
                            region_index_count,
                        ),
                    ),
                ),
            )
        };
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
        ])
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
}

mod prep {
    use super::*;

    // REVIEW - this function breaks the convention of `-> FormatRef` but it's an edge-case already
    pub(crate) fn table(_module: &mut FormatModule) -> Format {
        // REVIEW[epic=view-opaque-bytes] - we may wish to use ViewFormat in place of opaque_bytes to avoid vector allocation
        opaque_bytes()
    }
}
// REVIEW - the generated names for gasp subtypes can be run-on, consider pruning name tokens or module.define_format(_args) for brevity
mod gasp {
    use super::*;

    /// Format for a gasp-record, parametric in the version of the `gasp` table.
    fn gasp_record(version: Expr) -> Format {
        use BitFieldKind::*;
        let ver0flags = bit_fields_u16([
            Reserved {
                bit_width: 12,
                check_zero: false,
            }, // Reserved in all versions
            Reserved {
                bit_width: 2,
                check_zero: false,
            }, // Version 1 only, but not actually reserved
            FlagBit("dogray"),
            FlagBit("gridfit"),
        ]);
        let ver1flags = bit_fields_u16([
            Reserved {
                bit_width: 12,
                check_zero: false,
            }, // Reserved in all versions
            FlagBit("symmetric_smoothing"),
            FlagBit("symmetric_gridfit"),
            FlagBit("dogray"),
            FlagBit("gridfit"),
        ]);
        record([
            ("range_max_ppem", u16be()),
            (
                "range_gasp_behavior",
                match_variant(
                    version,
                    [
                        (Pattern::U16(0), "Version0", ver0flags),
                        (Pattern::U16(1), "Version1", ver1flags),
                        // REVIEW[epic=catchall-policy] - do we need this catch-all?
                        (Pattern::Wildcard, "BadVersion", Format::Fail), // NOTE - the name of this variant is arbitrary since it won't actually appear anywhere
                    ],
                ),
            ),
        ])
    }

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.gasp.table",
            record([
                ("version", u16be()),
                ("num_ranges", u16be()),
                (
                    "gasp_ranges",
                    repeat_count(var("num_ranges"), gasp_record(var("version"))),
                ),
            ]),
        )
    }
}

mod glyf {
    use super::*;

    mod simple {
        use super::*;

        pub(crate) fn flags_raw(module: &mut FormatModule) -> FormatRef {
            use BitFieldKind::*;

            const SHOULD_CHECK_ZERO: bool = false;
            module.define_format(
                "opentype.glyph-description.simple.flags-raw",
                bit_fields_u8([
                    Reserved {
                        bit_width: 1,
                        check_zero: SHOULD_CHECK_ZERO,
                    },
                    FlagBit("overlap_simple"),
                    FlagBit("y_is_same_or_positive_y_short_vector"),
                    FlagBit("x_is_same_or_positive_x_short_vector"),
                    FlagBit("repeat_flag"),
                    FlagBit("y_short_vector"),
                    FlagBit("x_short_vector"),
                    FlagBit("on_curve_point"),
                ]),
            )
        }

        pub(crate) fn flags(simple_flags_raw: FormatRef, num_coordinates: Expr) -> Format {
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
                            u8(),
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
                    succ(as_u16(record_proj(var("flags"), "repeats"))),
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
                                    Expr::AsU32(Box::new(record_proj(var("packed"), "repeats"))),
                                    Expr::U32(1),
                                ),
                                record_proj(var("packed"), "field_set"),
                            ),
                        ),
                        var("arr_flags"),
                    ),
                ),
            )
        }
        /// Given an individual field-set (flag-record) from an array, parse the appropriate x-coordinate value for the corresponding glyph
        fn x_coords(field_set: Expr) -> Format {
            if_then_else(
                record_proj(field_set.clone(), "x_short_vector"),
                // this wants to be i16
                map(
                    u8(),
                    lambda(
                        "abs",
                        util::u8_to_i16(
                            var("abs"),
                            record_proj(field_set.clone(), "x_is_same_or_positive_x_short_vector"),
                        ),
                    ),
                ),
                if_then_else(
                    record_proj(field_set.clone(), "x_is_same_or_positive_x_short_vector"),
                    // this wants to be i16
                    compute(Expr::U16(0)),
                    util::s16be(),
                ),
            )
        }

        /// Given an individual field-set (flag-record) from an array, parse the appropriate y-coordinate value for the corresponding glyph
        // TODO - consider a generic `read_coord` function that takes extra parameters to determine x-vs-y specialization
        fn y_coords(field_set: Expr) -> Format {
            if_then_else(
                record_proj(field_set.clone(), "y_short_vector"),
                // this wants to be i16
                map(
                    u8(),
                    lambda(
                        "abs",
                        util::u8_to_i16(
                            var("abs"),
                            record_proj(field_set.clone(), "y_is_same_or_positive_y_short_vector"),
                        ),
                    ),
                ),
                if_then_else(
                    record_proj(field_set.clone(), "y_is_same_or_positive_y_short_vector"),
                    // this wants to be i16
                    compute(Expr::U16(0)),
                    util::s16be(),
                ),
            )
        }

        pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
            let simple_flags_raw = flags_raw(module);
            module.define_format_args(
                "opentype.glyf.simple",
                vec![(
                    Label::Borrowed("n_contours"),
                    ValueType::Base(BaseType::U16),
                )],
                record([
                    (
                        "end_points_of_contour",
                        repeat_count(var("n_contours"), u16be()),
                    ),
                    ("instruction_length", u16be()),
                    (
                        "instructions",
                        repeat_count(var("instruction_length"), u8()),
                    ),
                    (
                        "number_of_coordinates",
                        compute(succ(util::last_elem(var("end_points_of_contour")))),
                    ),
                    (
                        "flags",
                        flags(simple_flags_raw, var("number_of_coordinates")),
                    ),
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
        }
    }

    mod composite {
        use super::*;
        use BitFieldKind::*;

        pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
            let glyf_arg = |are_words: Expr, are_xy_values: Expr| -> Format {
                if_then_else(
                    are_words,
                    if_then_else(
                        are_xy_values.clone(),
                        fmt_variant("Int16", util::s16be()),
                        fmt_variant("Uint16", u16be()),
                    ),
                    if_then_else(
                        are_xy_values,
                        fmt_variant("Int8", util::s8()),
                        fmt_variant("Uint8", u8()),
                    ),
                )
            };
            let glyf_flags_composite = flags();
            let glyf_scale = |flags: Expr| -> Format {
                if_then_else(
                    record_proj(flags.clone(), "we_have_a_scale"),
                    fmt_some(fmt_variant("Scale", util::f2dot14())),
                    if_then_else(
                        record_proj(flags.clone(), "we_have_an_x_and_y_scale"),
                        fmt_some(fmt_variant(
                            "XY",
                            record_repeat(["x_scale", "y_scale"], util::f2dot14()),
                        )),
                        if_then_else(
                            record_proj(flags, "we_have_a_two_by_two"),
                            fmt_some(fmt_variant(
                                "Matrix",
                                tuple_repeat(2, tuple_repeat(2, util::f2dot14())),
                            )),
                            fmt_none(),
                        ),
                    ),
                )
            };
            let glyf_component = record([
                ("flags", glyf_flags_composite),
                ("glyph_index", u16be()),
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
                    |elt| expr_not(record_lens(elt, &["flags", "more_components"])),
                    seq_last_checked(var("seq")),
                ),
            );
            let update_any_instructions = lambda_tuple(
                ["acc", "glyph"],
                or(
                    var("acc"),
                    record_lens(var("glyph"), &["flags", "we_have_instructions"]),
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
                                    u16be(),
                                    "instructions_length",
                                    repeat_count(var("instructions_length"), u8()),
                                ),
                                compute(seq_empty()),
                            ),
                        ),
                    ]),
                ),
            )
        }

        pub(crate) fn flags() -> Format {
            bit_fields_u16([
                Reserved {
                    bit_width: 3,
                    check_zero: false,
                },
                FlagBit("unscaled_component_offset"), // bit 12 - set if component offset is not to be scaled
                FlagBit("scaled_component_offset"), // bit 11 - set if component offset is to be scaled
                FlagBit("overlap_compound"), // bit 10 - hint for whether the component overlap
                FlagBit("use_my_metrics"), // bit 9 - when set, composite glyph inherits aw, lsb, rsb of current component glyph
                FlagBit("we_have_instructions"), // bit 8 - instructions present after final component
                FlagBit("we_have_a_two_by_two"), // bit 7 - we have a two by two transformation that will be used to scale the glyph
                FlagBit("we_have_an_x_and_y_scale"), // bit 6 - when set, x has a different scale from y
                FlagBit("more_components"), // bit 5 - continuation bit (1 when more follow, 0 if final)
                Reserved {
                    bit_width: 1,
                    check_zero: false,
                }, // bit 4 - reserved, should be 0
                FlagBit("we_have_a_scale"), // bit 3 - when 1, component has simple scale; otherwise scale is 1.0
                FlagBit("round_xy_to_grid"), // bit 2 - when set (and when `args_are_xy_values` is set), xy values are rounded to nearest grid line
                FlagBit("args_are_xy_values"), // bit 1 - when set, args are signed xy values; otherwise, they are unsigned point numbers
                FlagBit("arg_1_and_2_are_words"), // bit 0 - set for args of type u16 or i16; clear for args of type u8 or i8
            ])
        }
    }

    fn table_entry(
        start_offset: Expr,
        this_offset32: Expr,
        next_offset32: Expr,
        glyf_description: FormatRef,
    ) -> Format {
        if_then_else(
            // NOTE - checks that the glyph is non-vacuous
            expr_gt(next_offset32, this_offset32.clone()),
            util::linked_offset32(
                start_offset,
                this_offset32,
                fmt_variant(
                    "Glyph",
                    record([
                        ("number_of_contours", util::s16be()),
                        ("x_min", util::s16be()),
                        ("y_min", util::s16be()),
                        ("x_max", util::s16be()),
                        ("y_max", util::s16be()),
                        (
                            "description",
                            glyf_description.call_args(vec![var("number_of_contours")]),
                        ),
                    ]),
                ),
            ),
            fmt_variant("EmptyGlyph", Format::EMPTY),
        )
    }

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        let simple_glyf_table = simple::table(module);
        let composite_glyf_table = composite::table(module);
        let glyf_description = glyf_description(module, simple_glyf_table, composite_glyf_table);

        let offsets_type = {
            let mk_branch = |elem_t: ValueType| ValueType::Seq(Box::new(elem_t));
            let mut branches = std::collections::BTreeMap::new();
            // NOTE - at this layer, the u16-valued offsets are still half-value
            branches.insert(
                Label::Borrowed("Offsets16"),
                mk_branch(ValueType::Base(BaseType::U16)),
            );
            branches.insert(
                Label::Borrowed("Offsets32"),
                mk_branch(ValueType::Base(BaseType::U32)),
            );
            ValueType::Union(branches)
        };

        module.define_format_args(
            "opentype.glyf.table",
            vec![(Label::Borrowed("offsets"), offsets_type)],
            chain(
                pos32(),
                "start_offset",
                Format::Match(
                    Box::new(var("offsets")),
                    vec![
                        (
                            Pattern::Variant(
                                Label::Borrowed("Offsets16"),
                                Box::new(bind("half16s")),
                            ),
                            for_each_pair(
                                var("half16s"),
                                (scale2, scale2),
                                ["this_offs", "next_offs"],
                                table_entry(
                                    var("start_offset"),
                                    var("this_offs"),
                                    var("next_offs"),
                                    glyf_description,
                                ),
                            ),
                        ),
                        (
                            Pattern::Variant(
                                Label::Borrowed("Offsets32"),
                                Box::new(bind("off32s")),
                            ),
                            for_each_pair(
                                var("off32s"),
                                (id, id),
                                ["this_offs", "next_offs"],
                                table_entry(
                                    var("start_offset"),
                                    var("this_offs"),
                                    var("next_offs"),
                                    glyf_description,
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        )
    }

    fn glyf_description(
        module: &mut FormatModule,
        simple_glyf_table: FormatRef,
        composite_glyf_table: FormatRef,
    ) -> FormatRef {
        module.define_format_args(
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
        )
    }
}

pub(crate) mod loca {
    use super::*;

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        module.define_format_args(
            "opentype.loca.table",
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
}

pub(crate) mod fpgm {
    use super::*;

    // REVIEW - this function breaks the convention of `-> FormatRef` but it's an edge-case already
    pub(crate) fn table(_module: &mut FormatModule) -> Format {
        // REVIEW[epic=view-opaque-bytes] - we may wish to use ViewFormat in place of opaque_bytes to avoid vector allocation
        opaque_bytes()
    }
}

pub(crate) mod cvt {
    use super::*;

    pub(crate) fn table(_module: &mut FormatModule) -> Format {
        repeat(util::s16be())
    }
}
pub(crate) mod post {
    use super::*;

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        let postv2 = record([
            ("num_glyphs", u16be()),
            ("glyph_name_index", repeat_count(var("num_glyphs"), u16be())),
            ("string_data", pos32()),
        ]);

        let postv2dot5 = record([
            ("num_glyphs", u16be()),
            ("offset", repeat_count(var("num_glyphs"), util::s8())),
        ]);

        module.define_format(
            "opentype.post.table",
            record([
                ("version", util::version16_16()),
                ("italic_angle", util::fixed32be()),
                ("underline_position", util::s16be()),
                ("underline_thickness", util::s16be()),
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
                            (Pattern::U32(0x0002_0000), "Version2", postv2),
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
}

mod os2 {
    use super::*;

    /// Conditional r1ecord-format consisting of OS/2 table fields for each version of the OS/2 table
    ///
    /// Takes a variable-identifier `version_ident` corresponding to the scoped variable storing the version-number
    /// as a u16, and a table-length expression `table_length`, both inherited from this function's caller, [`table`].
    /// # Notes
    ///
    /// Based on the notes in the Microsoft documentation for legacy OS/2 table version 0,
    /// (https://learn.microsoft.com/en-us/typography/opentype/spec/os2#version-0),
    /// a version 0 table with no more than 78 bytes is a valid OS/2 table whose final field
    /// is `usLastCharIndex`, skipping the formally specified final 5 fields.
    ///
    /// If the version is greater than 0, or the table is longer than 78 bytes, then the final 5 fields will be parsed,
    /// and otherwise the Format returned by this function will yield `None`.
    ///
    /// Each version of the OS/2 table has a different number of fields, but as they are strictly additive and do not
    /// change between versions in which they are present, each version that adds more fields has its fields stored
    /// as an optional nested-record in the previous version's record-of-extra-fields.
    ///
    /// As versions 2, 3, and 4 have the same basic fields, only versions 1, 2, and 5 act as thresholds
    /// for including extra fields (w.r.t. `version >= N` predicates)
    fn version_record(version_ident: &'static str, table_length: Expr) -> Format {
        const V0_MIN_LENGTH: u32 = 78;
        cond_maybe(
            or(
                is_nonzero_u16(var(version_ident)),
                expr_gte(table_length, Expr::U32(V0_MIN_LENGTH)),
            ),
            record([
                ("s_typo_ascender", util::s16be()),
                ("s_typo_descender", util::s16be()),
                ("s_typo_line_gap", util::s16be()),
                ("us_win_ascent", u16be()),
                ("us_win_descent", u16be()),
                (
                    "extra_fields_v1",
                    cond_maybe(
                        is_within(var(version_ident), Bounds::at_least(1)),
                        record([
                            ("ul_code_page_range_1", u32be()),
                            ("ul_code_page_range_2", u32be()),
                            (
                                "extra_fields_v2",
                                cond_maybe(
                                    is_within(var(version_ident), Bounds::at_least(2)),
                                    record([
                                        ("sx_height", util::s16be()),
                                        ("s_cap_height", util::s16be()),
                                        ("us_default_char", u16be()),
                                        ("us_break_char", u16be()),
                                        ("us_max_context", u16be()),
                                        (
                                            "extra_fields_v5",
                                            cond_maybe(
                                                is_within(var(version_ident), Bounds::at_least(5)),
                                                record([
                                                    ("us_lower_optical_point_size", u16be()),
                                                    ("us_upper_optical_point_size", u16be()),
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
    }

    pub(crate) fn table(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        module.define_format_args(
            "opentype.os2.table",
            vec![(
                Label::Borrowed("table_length"),
                ValueType::Base(BaseType::U32),
            )],
            record([
                ("version", u16be()),
                ("x_avg_char_width", util::s16be()),
                ("us_weight_class", u16be()),
                ("us_width_class", u16be()),
                ("fs_type", u16be()),
                ("y_subscript_x_size", util::s16be()),
                ("y_subscript_y_size", util::s16be()),
                ("y_subscript_x_offset", util::s16be()),
                ("y_subscript_y_offset", util::s16be()),
                ("y_superscript_x_size", util::s16be()),
                ("y_superscript_y_size", util::s16be()),
                ("y_superscript_x_offset", util::s16be()),
                ("y_superscript_y_offset", util::s16be()),
                ("y_strikeout_size", util::s16be()),
                ("y_strikeout_position", util::s16be()),
                ("s_family_class", util::s16be()),
                ("panose", repeat_count(Expr::U8(10), u8())),
                ("ul_unicode_range1", u32be()),
                ("ul_unicode_range2", u32be()),
                ("ul_unicode_range3", u32be()),
                ("ul_unicode_range4", u32be()),
                ("ach_vend_id", tag.call()),
                ("fs_selection", u16be()),
                ("us_first_char_index", u16be()),
                ("us_last_char_index", u16be()),
                ("data", version_record("version", var("table_length"))),
            ]),
        )
    }
}

mod name {
    use super::*;

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        let name_record = name_record(module);
        let name_version_1 = name_version_1(module);

        module.define_format(
            "opentype.name.table",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("version", u16be()),
                    ("name_count", u16be()),
                    ("storage_offset", u16be()),
                    (
                        "name_records",
                        repeat_count(
                            var("name_count"),
                            name_record
                                .call_views(vec![vvar("table_view").offset(var("storage_offset"))]),
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
                                    name_version_1.call_views(vec![
                                        vvar("table_view").offset(var("storage_offset")),
                                    ]),
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
            ),
        )
    }

    fn name_record(module: &mut FormatModule) -> FormatRef {
        let name_id = name_id();

        module.define_format_views(
            "opentype.name.name-record",
            vec![Label::Borrowed("storage_view")],
            record([
                ("platform", u16be()),
                ("encoding", encoding_id(var("platform"))),
                ("language", language_id()),
                ("name_id", name_id),
                ("length", u16be()),
                (
                    "string",
                    // REVIEW - do we want to use util::capture_bytes_view_offset16 here instead?
                    record([
                        ("offset", u16be()),
                        (
                            "data", // TODO - add interpretation of raw bytes
                            with_view(
                                vvar("storage_view").offset(var("offset")),
                                capture_bytes(var("length")),
                            ),
                        ),
                    ]),
                ),
            ]),
        )
    }

    // TODO - attach semantics to common subset of name_id values
    fn name_id() -> Format {
        #![allow(dead_code)]
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

        u16be()
    }

    /// Naming table version 1
    /// C.f. [https://learn.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-1]
    fn name_version_1(module: &mut FormatModule) -> FormatRef {
        let lang_tag_record = module.define_format_views(
            "opentype.name.lang-tag-record",
            vec![Label::Borrowed("storage_view")],
            record([
                ("length", u16be()),
                (
                    "lang_tag",
                    util::capture_bytes_view_offset16(vvar("storage_view"), var("length")),
                ),
            ]),
        );

        module.define_format_views(
            "opentype.name.name_version_1",
            vec![Label::Borrowed("storage_view")],
            record([
                ("lang_tag_count", u16be()),
                (
                    "lang_tag_records",
                    repeat_count(
                        var("lang_tag_count"),
                        lang_tag_record.call_views(vec![vvar("storage_view")]),
                    ),
                ),
            ]),
        )
    }
}

pub(crate) mod vmtx {
    // STUB[epic=horizontal-for-vertical] - this technically works as-is, but certain fields might want to be named differently
    pub(crate) use super::hmtx::table;
}
pub(crate) mod hmtx {
    use super::*;

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        let long_horizontal_metric = record([
            ("advance_width", u16be()),
            ("left_side_bearing", util::s16be()),
        ]);

        module.define_format_args(
            "opentype.hmtx.table",
            vec![
                (
                    Label::Borrowed("num_long_metrics"),
                    ValueType::Base(BaseType::U16),
                ),
                (
                    Label::Borrowed("num_glyphs"),
                    ValueType::Base(BaseType::U16),
                ),
            ],
            record([
                (
                    "long_metrics",
                    repeat_count(var("num_long_metrics"), long_horizontal_metric),
                ),
                (
                    "left_side_bearings", // REVIEW - 'top_side_bearings' in vmtx
                    repeat_count(
                        sub(var("num_glyphs"), var("num_long_metrics")),
                        util::s16be(),
                    ),
                ),
            ]),
        )
    }
}

pub(crate) mod maxp {
    use super::*;

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        const NO_Z0: u16 = 1;
        const YES_Z0: u16 = 2;

        let maxp_version_1 = module.define_format(
            "opentype.maxp.version1",
            record([
                ("max_points", u16be()),
                ("max_contours", u16be()),
                ("max_composite_points", u16be()),
                ("max_composite_contours", u16be()),
                ("max_zones", where_between_u16(u16be(), NO_Z0, YES_Z0)),
                ("max_twilight_points", u16be()),
                ("max_storage", u16be()),
                ("max_function_defs", u16be()),
                ("max_instruction_defs", u16be()),
                ("max_stack_elements", u16be()),
                ("max_size_of_instructions", u16be()),
                ("max_component_elements", u16be()),
                ("max_component_depth", where_between_u16(u16be(), 0, 16)),
            ]),
        );

        module.define_format(
            "opentype.maxp.table",
            record([
                ("version", util::version16_16()),
                ("num_glyphs", u16be()),
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
    }
}

pub(crate) mod hhea {
    use super::*;

    pub(crate) fn table_def() -> Format {
        record_auto([
            ("major_version", util::expect_u16be(1)),
            (
                "minor_version",
                util::expects_u16be([0x0000, 0x1000]), // NOTE - due to how versions are encoded for hhea/vhea tables v1.1 is `00 01 . 10 00`
            ), // FIXME - hhea only has 1.0, but vhea has 1.1 as well, so we compromise by allowing it in both to re-use it properly
            ("ascent", util::s16be()), // distance from baseline to highest ascender, in font design units
            ("descent", util::s16be()), // distance from baseline to lowest descender, in font design units
            ("line_gap", util::s16be()), // intended gap between baselines, in font design units
            ("advance_width_max", u16be()), // must be consistent with horizontal metrics
            ("min_left_side_bearing", util::s16be()), // must be consistent with horizontal metrics
            ("min_right_side_bearing", util::s16be()), // must be consistent with horizontal metrics
            ("x_max_extent", util::s16be()), // `max(left_side_bearing + (x_max - x_min))`
            // slope of the caret (rise/run), (1/0) for vertical caret
            ("caret_slope", record_repeat(["rise", "run"], util::s16be())),
            ("caret_offset", util::s16be()), // 0 for non-slanted fonts
            ("__reservedX4", tuple_repeat(4, util::expect_u16be(0))), // NOTE: 4 separate isolated fields in fathom
            ("metric_data_format", util::expect_u16be(0)),
            // number of `long_horizontal_metric` records in the `htmx_table`, `long_vertical_metrics` in `vmtx_table`
            ("number_of_long_metrics", u16be()),
        ])
    }

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        module.define_format("opentype.hhea.table", table_def())
    }
}
pub(crate) mod vhea {
    use super::*;

    // STUB[epic=horizontal-for-vertical] - this currently works, but the field-names are misleading because they are implicitly biased for hhea
    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        module.define_format("opentype.vhea.table", super::hhea::table_def())
    }
}

// ANCHOR - `cmap` table
pub(crate) mod cmap {
    use super::*;

    /// Format for language-ids appearing within the `cmap` table-scop
    #[inline]
    pub(crate) fn cmap_language_id(_platform: Expr) -> Format {
        language_id()
    }
    /// Format for 32-bit language-ids appearing within the `cmap` table-scop

    #[inline]
    pub(crate) fn cmap_language_id32(_platform: Expr) -> Format {
        u32be()
    }

    #[inline]
    pub(crate) fn small_glyph_id() -> Format {
        u8()
    }

    /// Table format definition-function for `cmap`
    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        let sequential_map_group = module.define_format(
            "opentype.types.sequential_map_record",
            record([
                ("start_char_code", u32be()),
                ("end_char_code", u32be()),
                ("start_glyph_id", u32be()),
            ]),
        );

        let cmap_subtable_format0 = subtable_format0(module);
        let cmap_subtable_format2 = subtable_format2(module);
        let cmap_subtable_format4 = subtable_format4(module);

        let cmap_subtable_format6 = subtable_format6(module);

        let cmap_subtable_format8 = subtable_format8(module, sequential_map_group);

        let cmap_subtable_format10 = subtable_format10(module);

        let cmap_subtable_format12 = subtable_format12(module, sequential_map_group);

        let cmap_subtable_format13 = subtable_format13(module, sequential_map_group);

        let cmap_subtable_format14 = subtable_format14(module);

        let cmap_subtable = module.define_format_args(
            "opentype.cmap.subtable",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("format", Format::Peek(Box::new(u16be()))),
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
                                    cmap_subtable_format14.call_views(vec![vvar("table_view")]),
                                ),
                                // FIXME - leaving out unknown-table for now
                            ],
                        ),
                    ),
                ]),
            ),
        );

        let encoding_record = module.define_format_views(
            "opentype.encoding_record",
            vec![Label::Borrowed("table_view")],
            record([
                ("platform", u16be()), // platform identifier
                // NOTE - encoding_id nominally depends on platform_id but no recorded dependencies in fathom def
                ("encoding", encoding_id(var("platform"))), // encoding identifier
                (
                    "subtable",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        cmap_subtable.call_args(vec![var("platform")]),
                    ),
                ),
            ]),
        );

        module.define_format(
            "opentype.cmap.table",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))), // start of character mapping table
                    ("version", u16be()),                            // table version number
                    ("num_tables", u16be()), // number of subsequent encoding tables
                    (
                        "encoding_records",
                        repeat_count(
                            var("num_tables"),
                            encoding_record.call_views(vec![vvar("table_view")]),
                        ),
                    ),
                ]),
            ),
        )
    }

    // Format 0 : Byte encoding table
    fn subtable_format0(module: &mut FormatModule) -> FormatRef {
        module.define_format_args(
            "opentype.cmap_subtable.format0",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            util::slice_record(
                "length",
                [
                    ("format", u16be()), // == 0
                    ("length", u16be()),
                    ("language", cmap_language_id(var("_platform"))),
                    (
                        "glyph_id_array",
                        repeat_count(Expr::U16(256), small_glyph_id()),
                    ),
                ],
            ),
        )
    }

    fn subtable_format2(module: &mut FormatModule) -> FormatRef {
        let subheader = record([
            ("first_code", u16be()),
            ("entry_count", u16be()),
            // FIXME - this is actually a signed 16-bit value but we don't support that; it can be unsigned as long as we do the right wrapping addition
            ("id_delta", util::s16be()),
            ("id_range_offset", u16be()),
        ]);

        // Format 2: High-byte mapping through table
        module.define_format_args(
            "opentype.cmap_subtable.format2",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            util::slice_record(
                "length",
                [
                    ("format", util::expect_u16be(2)),
                    (
                        "length",
                        where_lambda(
                            u16be(),
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
                    ("sub_header_keys", repeat_count(Expr::U16(256), u16be())),
                    (
                        "sub_headers",
                        repeat_count(
                            succ(util::subheader_index(var("sub_header_keys"))),
                            subheader,
                        ),
                    ),
                    ("glyph_array", repeat(u16be())),
                ],
            ),
        )
    }

    /// cmap subtable Format 4: Segment mapping to delta values
    fn subtable_format4(module: &mut FormatModule) -> FormatRef {
        module.define_format_args(
            "opentype.cmap_subtable.format4",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            util::slice_record(
                "length",
                [
                    ("format", util::expect_u16be(4)),
                    ("length", u16be()),
                    ("language", cmap_language_id(var("_platform"))),
                    (
                        "seg_count",
                        map(
                            u16be(),
                            lambda("seg_count_x2", div(var("seg_count_x2"), Expr::U16(2))),
                        ),
                    ),
                    ("search_range", u16be()), // := 2x the maximum power of 2 <= seg_count
                    ("entry_selector", u16be()), // := ilog2(seg_count)
                    ("range_shift", u16be()),  // := seg_count * 2 - search_range
                    ("end_code", repeat_count(var("seg_count"), u16be())), // end character-code for each seg, last is 0xFFFF
                    ("__reserved_pad", util::expect_u16be(0)),
                    ("start_code", repeat_count(var("seg_count"), u16be())),
                    ("id_delta", repeat_count(var("seg_count"), u16be())), // ought to be signed but will work if we perform as unsigned addition mod-0xFFFF
                    ("id_range_offset", repeat_count(var("seg_count"), u16be())), // offsets into glyphIdArray or 0
                    ("glyph_array", repeat(u16be())),
                ],
            ),
        )
    }

    fn subtable_format6(module: &mut FormatModule) -> FormatRef {
        module.define_format_args(
            "opentype.cmap_subtable.format6",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            /* Previously defined as a slice_record but sufficiently large `entry_count` values
             * could cause length to wrap around mod 65536 and lead to slice boundary violation
             * while reading `glyph_id_array`
             */
            record([
                ("format", util::expect_u16be(6)),
                ("length", u16be()),
                ("language", cmap_language_id(var("_platform"))),
                ("first_code", u16be()),
                ("entry_count", u16be()),
                ("glyph_id_array", repeat_count(var("entry_count"), u16be())),
            ]),
        )
    }

    fn subtable_format8(module: &mut FormatModule, sequential_map_group: FormatRef) -> FormatRef {
        module.define_format_args(
            "opentype.cmap_subtable.format8",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            util::slice_record(
                "length",
                [
                    ("format", util::expect_u16be(8)),
                    ("__reserved", util::expect_u16be(0)),
                    ("length", u32be()),
                    ("language", cmap_language_id32(var("_platform"))),
                    // REVIEW - should this be 8x as long and consist of bits?
                    ("is32", repeat_count(Expr::U16(8192), u8())), // packed bit-array where a bit at index `i` signals whether the 16-bit value index `i` is the start of a 32-bit character code
                    ("num_groups", u32be()),
                    (
                        "groups",
                        repeat_count(var("num_groups"), sequential_map_group.call()),
                    ),
                ],
            ),
        )
    }

    fn subtable_format10(module: &mut FormatModule) -> FormatRef {
        module.define_format_args(
            "opentype.cmap_subtable.format10",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            util::slice_record(
                "length",
                [
                    ("format", util::expect_u16be(10)),
                    ("__reserved", util::expect_u16be(0)),
                    ("length", u32be()),
                    ("language", cmap_language_id32(var("_platform"))),
                    ("start_char_code", u32be()),
                    ("num_chars", u32be()),
                    ("glyph_id_array", repeat_count(var("num_chars"), u16be())),
                ],
            ),
        )
    }

    fn subtable_format12(module: &mut FormatModule, sequential_map_group: FormatRef) -> FormatRef {
        module.define_format_args(
            "opentype.cmap_subtable.format12",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            util::slice_record(
                "length",
                [
                    ("format", util::expect_u16be(12)),
                    ("__reserved", util::expect_u16be(0)),
                    ("length", u32be()),
                    ("language", cmap_language_id32(var("_platform"))),
                    ("num_groups", u32be()),
                    (
                        "groups",
                        repeat_count(var("num_groups"), sequential_map_group.call()),
                    ),
                ],
            ),
        )
    }

    fn subtable_format13(module: &mut FormatModule, sequential_map_group: FormatRef) -> FormatRef {
        let constant_map_group = sequential_map_group.call();

        module.define_format_args(
            "opentype.cmap_subtable.format13",
            vec![(Label::Borrowed("_platform"), ValueType::Base(BaseType::U16))],
            util::slice_record(
                "length",
                [
                    ("format", util::expect_u16be(13)),
                    ("__reserved", util::expect_u16be(0)),
                    ("length", u32be()),
                    ("language", cmap_language_id32(var("_platform"))),
                    ("num_groups", u32be()),
                    (
                        "groups",
                        repeat_count(var("num_groups"), constant_map_group),
                    ),
                ],
            ),
        )
    }

    fn subtable_format14(module: &mut FormatModule) -> FormatRef {
        let unicode_range = record([
            ("start_unicode_value", util::u24be()),
            ("additional_count", u8()),
        ]);

        let uvs_mapping = record([("unicode_value", util::u24be()), ("glyph_id", u16be())]);

        let default_uvs_table = record([
            ("num_unicode_value_ranges", u32be()),
            (
                "ranges",
                repeat_count(var("num_unicode_value_ranges"), unicode_range),
            ),
        ]);

        let non_default_uvs_table = record([
            ("num_uvs_mappings", u32be()),
            (
                "uvs_mappings",
                repeat_count(var("num_uvs_mappings"), uvs_mapping),
            ),
        ]);

        let variation_selector = module.define_format_views(
            "opentype.variation_selector",
            vec![TABLE_VIEW],
            record([
                ("var_selector", util::u24be()),
                (
                    "default_uvs_offset",
                    util::read_phantom_view_offset32(vvar("table_view"), default_uvs_table),
                ),
                (
                    "non_default_uvs_offset",
                    util::read_phantom_view_offset32(vvar("table_view"), non_default_uvs_table),
                ),
            ]),
        );

        module.define_format_views(
            "opentype.cmap_subtable.format14",
            [util::TABLE_VIEW].to_vec(),
            util::slice_record(
                "length",
                [
                    ("format", util::expect_u16be(14)),
                    ("length", u32be()),
                    ("num_var_selector_records", u32be()),
                    (
                        "var_selector",
                        repeat_count(
                            var("num_var_selector_records"),
                            variation_selector.call_views(vec![vvar("table_view")]),
                        ),
                    ),
                ],
            ),
        )
    }
}

// ANCHOR - `head` table
pub(crate) mod head {
    use super::*;
    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        // FIXME - replace with bit_fields_u16 if appropriate
        let head_table_flags = u16be();

        let long_date_time = module.define_format("opentype.types.long_date_time", util::s64be());

        let xy_min_max = record_repeat(["x_min", "y_min", "x_max", "y_max"], util::s16be());

        // REVIEW[epic=check-zero] - determine whether we should check for zeroing of reserved bit-fields positions
        const SHOULD_CHECK_ZERO: bool = false;

        let head_table_style_flags = bit_fields_u16([
            BitFieldKind::Reserved {
                bit_width: 9,
                check_zero: SHOULD_CHECK_ZERO,
            },
            BitFieldKind::FlagBit("extended"),
            BitFieldKind::FlagBit("condensed"),
            BitFieldKind::FlagBit("shadow"),
            BitFieldKind::FlagBit("outline"),
            BitFieldKind::FlagBit("underline"),
            BitFieldKind::FlagBit("italic"),
            BitFieldKind::FlagBit("bold"),
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
        let glyph_dir_hint = util::s16be();

        module.define_format(
            "opentype.head_table",
            record([
                ("major_version", util::expect_u16be(1)),
                ("minor_version", util::expect_u16be(0)),
                ("font_revision", util::fixed32be()),
                ("checksum_adjustment", u32be()),
                ("magic_number", is_bytes(&[0x5F, 0x0F, 0x3C, 0xF5])),
                ("flags", head_table_flags),
                ("units_per_em", where_between_u16(u16be(), 16, 16384)),
                ("created", long_date_time.call()),
                ("modified", long_date_time.call()),
                ("glyph_extents", xy_min_max),
                ("mac_style", head_table_style_flags),
                ("lowest_rec_ppem", u16be()),
                ("font_direction_hint", glyph_dir_hint),
                (
                    "index_to_loc_format",
                    where_between_u16(u16be(), SHORT_OFFSET16, LONG_OFFSET32),
                ),
                ("glyph_data_format", util::expect_u16be(0)),
            ]),
        )
    }
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

/// Module for subformat functions related to tables in general
pub(crate) mod table {
    use super::*;

    /// Table that is required to appear
    ///
    /// Takes an expr for the start-of-file offset `sof_offset`, an Expr containing the parsed sequence-of-table-records `table-records`,
    /// a table id `id` unique to the table we are defining, and the format of the table `table_format`.
    pub(crate) fn required_table(
        // WIP
        sof_offset: Expr,
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
                        util::linked_offset32(
                            sof_offset,
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
        // WIP
        sof_offset: Expr,
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
                        util::linked_offset32(
                            sof_offset,
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
        // WIP
        sof_offset: Expr,
        table_records: Expr,
        id: u32,
        table_format: Format,
    ) -> Format {
        let cond_fmt = |table_match: Expr| -> Format {
            util::linked_offset32(
                sof_offset,
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

// ANCHOR - `stat` table
pub(crate) mod stat {
    use super::*;

    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/stat#style-attributes-header
    pub(crate) fn table(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let axis_record = {
            record([
                ("axis_tag", tag.call()),
                ("axis_name_id", u16be()),
                ("axis_ordering", u16be()),
            ])
        };
        let axis_value_table = {
            use BitFieldKind::*;
            let axis_flags = bit_fields_u16([
                Reserved {
                    bit_width: 14,
                    check_zero: false,
                },
                FlagBit("elidable_axis_value_name"), // Bit 1 - When set, indicates the 'normal' value for this axis and implies it may be omitted when composing name-strings
                FlagBit("older_sibling_font_attribute"), // Bit 0 - When set, indicates that the axis information applies to previously released fonts in the same font-family
            ]);
            let axis_value = record([("axis_index", u16be()), ("value", util::fixed32be())]);
            let f1_fields = vec![
                ("axis_index", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
                ("value", fixed32be()),
            ];
            let f2_fields = vec![
                ("axis_index", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
                ("nominal_value", fixed32be()),
                ("range_min_value", fixed32be()),
                ("range_max_value", fixed32be()),
            ];
            let f3_fields = vec![
                ("axis_index", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
                ("value", fixed32be()),
                ("linked_value", fixed32be()),
            ];
            let f4_fields = vec![
                ("axis_count", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this combination of axis values
                ("axis_values", repeat_count(var("axis_count"), axis_value)),
            ];
            util::embedded_variadic_alternation(
                [("format", where_between_u16(u16be(), 1, 4))],
                "format",
                [
                    (1, "Format1", f1_fields),
                    (2, "Format2", f2_fields),
                    (3, "Format3", f3_fields),
                    (4, "Format4", f4_fields),
                ],
                "data",
                util::NestingKind::MinimalVariation,
            )
        };
        let design_axes_array = |design_axis_count: Expr| {
            record([("design_axes", repeat_count(design_axis_count, axis_record))])
        };
        let axis_value_offsets_array = |axis_value_count: Expr| {
            let_view(
                "array_view",
                record([
                    ("array_scope", reify_view(vvar("array_view"))),
                    (
                        "axis_value_offsets",
                        repeat_count(
                            axis_value_count,
                            util::read_phantom_view_offset16(vvar("array_view"), axis_value_table),
                        ),
                    ),
                ]),
            )
        };
        module.define_format(
            "opentype.stat_table",
            let_view(
                "table_view",
                record([
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("major_version", util::expect_u16be(1)),
                    ("minor_version", util::expects_u16be([1, 2])), // Version 1.0 is deprecated
                    ("design_axis_size", u16be()), // size (in bytes) of each axis record
                    ("design_axis_count", u16be()), // number of axis records
                    (
                        "design_axes",
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            design_axes_array(var("design_axis_count")),
                        ),
                    ), // offset is 0 iff design_axis_count is 0
                    ("axis_value_count", u16be()),
                    (
                        "axis_value_offsets",
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            axis_value_offsets_array(var("axis_value_count")),
                        ),
                    ), // offset is 0 iff axis_value_count is 0
                    ("elided_fallback_name_id", u16be()), // omitted in version 1.0, but said version is deprecated
                ]),
            ),
        )
    }
}

/// Alternate definitions for experimental purposes
pub(crate) mod alt {
    use super::*;
    pub(crate) fn main(module: &mut FormatModule) -> FormatRef {
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

        let table_links = {
            let stat_table = stat_table(module, tag);

            module.define_format_args(
                "opentype.table_directory.table_links",
                vec![
                    START_ARG,
                    (
                        Label::Borrowed("tables"),
                        ValueType::Seq(Box::new(table_type)),
                    ),
                ],
                record_auto([
                    (
                        "stat",
                        optional_table(
                            util::START_VAR,
                            var("tables"),
                            util::magic(b"STAT"),
                            stat_table.call(),
                        ),
                    ),
                    ("__skip", Format::SkipRemainder),
                ]),
            )
        };

        let table_directory = module.define_format_args(
            "opentype.table_directory",
            vec![(
                Label::Borrowed("font_start"),
                ValueType::Base(BaseType::U32),
            )],
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
                ("search_range", u16be()), // TODO[validation] - should be (maximum power of 2 <= num_tables) x 16
                ("entry_selector", u16be()), // TODO[validation] - should be Log2(maximum power of 2 <= num_tables)
                ("range_shift", u16be()), // TODO[validation] - should be (NumTables x 16) - searchRange
                (
                    "table_records",
                    repeat_count(var("num_tables"), table_record.call()),
                ),
                (
                    "table_links",
                    table_links.call_args(vec![var("font_start"), var("table_records")]),
                ),
            ]),
        );

        let ttc_header = {
            // Version 1.0
            // WIP
            let ttc_header1 = |start: Expr| {
                record([
                    ("num_fonts", u32be()),
                    (
                        "table_directories",
                        repeat_count(
                            var("num_fonts"),
                            util::offset32(start.clone(), table_directory.call_args(vec![start])),
                        ),
                    ),
                ])
            };

            // Version 2.0
            // WIP
            let ttc_header2 = |start: Expr| {
                record([
                    ("num_fonts", u32be()),
                    (
                        "table_directories",
                        repeat_count(
                            var("num_fonts"),
                            util::offset32(start.clone(), table_directory.call_args(vec![start])),
                        ),
                    ),
                    ("dsig_tag", u32be()),    // either b"DSIG" or 0 if none
                    ("dsig_length", u32be()), // byte-length or 0 if none
                    ("dsig_offset", u32be()), // byte-offset or 0 if none
                ])
            };

            module.define_format_args(
                "opentype.ttc_header",
                vec![START_ARG],
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
                                (Pattern::U16(1), "Version1", ttc_header1(util::START_VAR)),
                                (Pattern::U16(2), "Version2", ttc_header2(util::START_VAR)),
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
            record([
                ("file_start", pos32()),
                ("magic", Format::Peek(Box::new(u32be()))),
                (
                    "directory",
                    match_variant(
                        var("magic"),
                        [
                            (
                                Pattern::U32(0x00010000),
                                "TableDirectory",
                                table_directory.call_args(vec![var("file_start")]),
                            ),
                            (
                                Pattern::U32(util::magic(b"OTTO")),
                                "TableDirectory",
                                table_directory.call_args(vec![var("file_start")]),
                            ),
                            (
                                Pattern::U32(util::magic(b"ttcf")),
                                "TTCHeader",
                                ttc_header.call_args(vec![var("file_start")]),
                            ),
                            // TODO - not yet sure if TrueType fonts will parse correctly under our current table_directory implementation...
                            (
                                Pattern::U32(util::magic(b"true")),
                                "TableDirectory",
                                table_directory.call_args(vec![var("file_start")]),
                            ),
                            (Pattern::Wildcard, "UnknownTable", unknown_table),
                        ],
                    ),
                ),
            ]),
        )
    }

    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/stat#style-attributes-header
    pub(crate) fn stat_table(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let _axis_record = {
            module.define_format(
                "opentype.stat.axis_record",
                record([
                    ("axis_tag", tag.call()),
                    ("axis_name_id", u16be()),
                    ("axis_ordering", u16be()),
                ]),
            )
        };
        let _axis_value_table = {
            use BitFieldKind::*;
            let axis_flags = bit_fields_u16([
                Reserved {
                    bit_width: 14,
                    check_zero: false,
                },
                FlagBit("elidable_axis_value_name"), // Bit 1 - When set, indicates the 'normal' value for this axis and implies it may be omitted when composing name-strings
                FlagBit("older_sibling_font_attribute"), // Bit 0 - When set, indicates that the axis information applies to previously released fonts in the same font-family
            ]);
            let axis_value = record([("axis_index", u16be()), ("value", util::fixed32be())]);
            let f1_fields = vec![
                ("axis_index", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
                ("value", fixed32be()),
            ];
            let f2_fields = vec![
                ("axis_index", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
                ("nominal_value", fixed32be()),
                ("range_min_value", fixed32be()),
                ("range_max_value", fixed32be()),
            ];
            let f3_fields = vec![
                ("axis_index", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this attribute value
                ("value", fixed32be()),
                ("linked_value", fixed32be()),
            ];
            let f4_fields = vec![
                ("axis_count", u16be()),
                ("flags", axis_flags.clone()),
                ("value_name_id", u16be()), // NameId for entries in 'name' table that provide display-string for this combination of axis values
                ("axis_values", repeat_count(var("axis_count"), axis_value)),
            ];
            module.define_format(
                "opentype.stat.axis_value_table",
                util::embedded_variadic_alternation(
                    [("format", where_between_u16(u16be(), 1, 4))],
                    "format",
                    [
                        (1, "Format1", f1_fields),
                        (2, "Format2", f2_fields),
                        (3, "Format3", f3_fields),
                        (4, "Format4", f4_fields),
                    ],
                    "data",
                    util::NestingKind::MinimalVariation,
                ),
            )
        };
        fn design_axes_array(view: ViewExpr, size: Expr, count: Expr, offs: Expr) -> Format {
            /* offset32(var("table_start"), record([("design_axes", repeat_count(count, axis_record))])) */
            fmt_let(
                "len",
                mul(size, count),
                with_view(view.offset(offs), capture_bytes(var("len"))),
            )
        }
        fn axis_value_offsets_array(
            top_view: ViewExpr,
            count: Expr,
            offset_to_start: Expr,
        ) -> Format {
            // REVIEW - do we want to wrap in phantom??
            parse_from_view(
                top_view.offset(offset_to_start),
                // REVIEW - do we want to extract into FormatRef??
                let_view(
                    "axis_value_view",
                    record([
                        ("axis_value_scope", reify_view(vvar("axis_value_view"))),
                        (
                            "axis_value_offsets",
                            with_view(vvar("axis_value_view"), read_array(count, BaseKind::U16BE)),
                        ), // TODO - ForEach(offset: u16) -> offsetu16(offset, axis_value_table)
                    ]),
                ),
            )
        }
        module.define_format(
            "opentype.stat.table",
            let_view(
                "table_view",
                record_auto([
                    ("major_version", util::expect_u16be(1)),
                    ("minor_version", util::expects_u16be([1, 2])), // Version 1.0 is deprecated
                    ("design_axis_size", u16be()), // size (in bytes) of each axis record
                    ("design_axis_count", u16be()), // number of axis records
                    ("design_axes_offset", u32be()),
                    (
                        "design_axes_array",
                        design_axes_array(
                            vvar("table_view"),
                            var("design_axis_size"),
                            var("design_axis_count"),
                            var("_design_axes_offset"),
                        ),
                    ),
                    ("axis_value_count", u16be()),
                    ("offset_to_axis_value_offsets", u32be()),
                    (
                        "axis_value_offsets",
                        axis_value_offsets_array(
                            vvar("table_scope"),
                            var("axis_value_count"),
                            var("offset_to_axis_value_offsets"),
                        ),
                    ), // offset is 0 iff axis_value_count is 0
                    ("elided_fallback_name_id", u16be()), // omitted in version 1.0, but said version is deprecated
                ]),
            ),
        )
    }
}
