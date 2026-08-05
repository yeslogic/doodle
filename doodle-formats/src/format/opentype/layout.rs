use doodle::DepFormat;

use super::*;

/// Format definition for `SequenceLookup`
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#sequence-lookup-record
pub(crate) fn sequence_lookup_record(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.layout.sequence_lookup",
        record([("sequence_index", u16be()), ("lookup_list_index", u16be())]),
    )
}

/// Format definition for ChainedSequenceContext tables
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#common-formats-for-contextual-lookup-subtables
pub(crate) fn chained_sequence_context(
    om: &mut OpentypeModule<'_>,
    sequence_lookup_record: FormatRef,
) -> FormatRef {
    let class_def = om.class_def();
    let coverage_table = om.coverage_table();

    let rule_set = chained_sequence::rule_set(om.module(), sequence_lookup_record);
    let format1 = chained_sequence::format1(om.module(), coverage_table, rule_set);
    let format2 = chained_sequence::format2(om.module(), class_def, coverage_table, rule_set);
    let format3 = chained_sequence::format3(om.module(), coverage_table, sequence_lookup_record);
    om.module().define_format(
        "opentype.layout.chained_sequence_context",
        let_view(
            "table_view",
            record([
                // REVIEW[epic=nested-format-reify-layer] - scope reified locally in outer format
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

mod chained_sequence {
    use super::*;

    /// Format definition for the corresponding `*Set` table for `ChainedSequenceRule` and `ChainedClassSequenceRule` tables.
    ///
    /// The common format encompassing both `ChainedSequenceRule` and `ChainedClassSequenceRule` is registered internally.
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-1-simple-glyph-contexts
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-2-class-based-glyph-contexts
    pub(crate) fn rule_set(
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
    ///
    /// # Notes
    ///
    /// This format is overloaded and used also for `ChainedClassSequenceRule`, which has identical structure with the only
    /// difference being the semantics of certain raw-numeric field data (viz. u16-arrays are class-ids instead of glyph-ids).
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-2-class-based-glyph-contexts
    pub(crate) fn chained_sequence_rule(
        module: &mut FormatModule,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        module.define_format(
            "opentype.layout.chained-sequence-rule",
            record([
                ("backtrack_glyph_count", u16be()),
                (
                    "backtrack_sequence",
                    // readarray-eligible: bare u16be element -> BaseKind::U16BE, no FormatRef needed.
                    // `chained_sequence_rule` is a plain (non-view-bound) record parsed in-line at the
                    // cursor, so this would need `from_here(read_array(var("backtrack_glyph_count"), BaseKind::U16BE))`.
                    repeat_count(var("backtrack_glyph_count"), u16be()), // GlyphId (format1) or ClassId (format2)
                ),
                ("input_glyph_count", u16be()),
                (
                    "input_sequence",
                    // readarray-eligible: bare u16be element -> BaseKind::U16BE, no FormatRef needed.
                    // Same in-line/non-view-bound context as `backtrack_sequence`; would need
                    // `from_here(read_array(pred(var("input_glyph_count")), BaseKind::U16BE))`.
                    repeat_count(pred(var("input_glyph_count")), u16be()), // GlyphId (format1) or ClassId (format2)
                ),
                ("lookahead_glyph_count", u16be()),
                (
                    "lookahead_sequence",
                    // readarray-eligible: bare u16be element -> BaseKind::U16BE, no FormatRef needed.
                    // Same in-line/non-view-bound context; would need
                    // `from_here(read_array(var("lookahead_glyph_count"), BaseKind::U16BE))`.
                    repeat_count(var("lookahead_glyph_count"), u16be()), // GlyphId (format1) or ClassId (format2)
                ),
                ("seq_lookup_count", u16be()),
                (
                    "seq_lookup_records",
                    // readarray-eligible: `sequence_lookup_record` (defined above, this file) is a
                    // closed zero-arg FormatRef whose record is two bare u16be fields, satisfying
                    // `analyze_fixed_shape`. Reuse `sequence_lookup_record` directly (drop `.call()`)
                    // as the FixedReadKind::FixedFormat. In-line/non-view-bound context -> would need
                    // `from_here(read_array(var("seq_lookup_count"), sequence_lookup_record))`.
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        )
    }

    /// Format definition for ChainedSequenceContext Format 1
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-1-simple-glyph-contexts
    pub(crate) fn format1(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        rule_set: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.chained-sequence-context.format1",
            vec![(Label::Borrowed("table_view"))],
            record([
                // REVIEW[epic=nested-format-reify-layer] - OUTER
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
    pub(crate) fn format2(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        rule_set: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.chained-sequence-context.format2",
            vec![(Label::Borrowed("table_view"))],
            record([
                // REVIEW[epic=nested-format-reify-layer] - OUTER
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
    pub(crate) fn format3(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
            "opentype.layout.chained-sequence-context.format3",
            vec![(Label::Borrowed("table_view"))],
            record([
                // REVIEW[epic=nested-format-reify-layer] - OUTER
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
                    // readarray-eligible: `sequence_lookup_record` (top of file) is a closed zero-arg
                    // FormatRef with two bare u16be fields, satisfying `analyze_fixed_shape`. Reuse
                    // `sequence_lookup_record` directly (drop `.call()`) as the FixedReadKind::FixedFormat.
                    // This field is read sequentially at the cursor (not via a `table_view` offset), so it
                    // would need `from_here(read_array(var("seq_lookup_count"), sequence_lookup_record))`.
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        )
    }
}

pub(crate) fn sequence_context(
    om: &mut OpentypeModule<'_>,
    sequence_lookup_record: FormatRef,
) -> FormatRef {
    let class_def = om.class_def();
    let coverage_table = om.coverage_table();

    let rule_set = sequence::rule_set(om.module(), sequence_lookup_record);
    let format1 = sequence::format1(om.module(), coverage_table, rule_set);
    let format2 = sequence::format2(om.module(), class_def, coverage_table, rule_set);
    let format3 = sequence::format3(om.module(), coverage_table, sequence_lookup_record);
    om.module().define_format(
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

mod sequence {
    use super::*;

    fn rule(module: &mut FormatModule, sequence_lookup_record: FormatRef) -> FormatRef {
        module.define_format(
            "opentype.layout.sequence-context.rule",
            record([
                ("glyph_count", where_nonzero::<U16>(u16be())),
                ("seq_lookup_count", u16be()),
                (
                    "input_sequence",
                    // readarray-eligible: bare u16be element -> BaseKind::U16BE, no FormatRef needed.
                    // `rule` is a plain (non-view-bound) record parsed in-line at the cursor, so this
                    // would need `from_here(read_array(pred(var("glyph_count")), BaseKind::U16BE))`.
                    repeat_count(pred(var("glyph_count")), u16be()),
                ),
                (
                    "seq_lookup_records",
                    // readarray-eligible: `sequence_lookup_record` (top of file) is a closed zero-arg
                    // FormatRef with two bare u16be fields, satisfying `analyze_fixed_shape`. Reuse
                    // `sequence_lookup_record` directly (drop `.call()`) as the FixedReadKind::FixedFormat.
                    // In-line/non-view-bound context -> would need
                    // `from_here(read_array(var("seq_lookup_count"), sequence_lookup_record))`.
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        )
    }

    pub(crate) fn rule_set(
        module: &mut FormatModule,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        let rule = rule(module, sequence_lookup_record);
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

    pub(crate) fn format1(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        rule_set: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
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
        )
    }

    pub(crate) fn format2(
        module: &mut FormatModule,
        class_def: FormatRef,
        coverage_table: FormatRef,
        rule_set: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
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
        )
    }

    pub(crate) fn format3(
        module: &mut FormatModule,
        coverage_table: FormatRef,
        sequence_lookup_record: FormatRef,
    ) -> FormatRef {
        module.define_format_views(
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
                    // readarray-eligible: `sequence_lookup_record` (top of file) is a closed zero-arg
                    // FormatRef with two bare u16be fields, satisfying `analyze_fixed_shape`. Reuse
                    // `sequence_lookup_record` directly (drop `.call()`) as the FixedReadKind::FixedFormat.
                    // Read sequentially at the cursor (not via a `table_view` offset), so it would need
                    // `from_here(read_array(var("seq_lookup_count"), sequence_lookup_record))`.
                    repeat_count(var("seq_lookup_count"), sequence_lookup_record.call()),
                ),
            ]),
        )
    }
}

/// Format definition for `FeatureList` table
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#featurelist-table
pub(crate) fn feature_list(om: &mut OpentypeModule<'_>, feature_table: FormatRef) -> FormatRef {
    let tag = om.tag();
    let feature_record = feature_record(om.module(), tag, feature_table);
    om.module().define_format(
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
                // TODO - if we ever get a working model for the Params mapping from feature-tag, we should make feature-table a dep-format and pass in `feature_tag` as an argument.
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
                // NOTE - the format of the Params table is opaquely dependent on the precise feature tag, so while we want to parse an offset16 here, we don't have a good handle on what the offset is pointing to (or even how many bytes it occupies), nor on wehich FeatureRecord tags allow for parameters to be present
                ("feature_params", u16be()),
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
fn script_record(module: &mut FormatModule, tag: FormatRef, script_table: FormatRef) -> FormatRef {
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
pub(crate) fn script_list(om: &mut OpentypeModule<'_>, script_table: FormatRef) -> FormatRef {
    let tag = om.tag();
    let script_record = script_record(om.module(), tag, script_table);
    om.module().define_format(
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
pub(crate) fn script_table(om: &mut OpentypeModule<'_>, lang_sys: FormatRef) -> FormatRef {
    let tag = om.tag();
    let lang_sys_record = lang_sys_record(om.module(), tag, lang_sys);
    om.module().define_format(
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

pub(crate) fn value_record(om: &mut OpentypeModule<'_>, vf_flags_type: ValueType) -> FormatRef {
    let device_or_variation_index_table = om.device_or_variation_index_table();
    let opt_field = |field_name: &'static str, format: Format| {
        (
            field_name,
            cond_maybe(record_proj(var("flags"), field_name), format),
        )
    };
    om.module().define_format_args_views(
        "opentype.layout.value_record",
        vec![(Label::Borrowed("flags"), vf_flags_type.clone())],
        vec![Label::Borrowed("table_view")],
        record([
            opt_field("x_placement", i16be()),
            opt_field("y_placement", i16be()),
            opt_field("x_advance", i16be()),
            opt_field("y_advance", i16be()),
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
            ("required_feature_index", u16be()),                 // 0xFFFF if no features required
            ("feature_index_count", u16be()),
            (
                "feature_indices",
                // readarray-eligible: bare u16be element -> BaseKind::U16BE, no FormatRef needed.
                // `lang_sys` is a plain (non-view-bound) record parsed in-line at the cursor, so this
                // would need `from_here(read_array(var("feature_index_count"), BaseKind::U16BE))`.
                repeat_count(var("feature_index_count"), u16be()),
            ),
        ]),
    )
}

pub(crate) fn anchor_table(om: &mut OpentypeModule<'_>) -> FormatRef {
    let device_or_variation_index_table = om.device_or_variation_index_table();
    // REVIEW - should formats 1 and 2 be defined as well?
    let anchor_format1 = record([("x_coordinate", i16be()), ("y_coordinate", i16be())]);
    let anchor_format2 = record([
        ("x_coordinate", i16be()),
        ("y_coordinate", i16be()),
        ("anchor_point", u16be()),
    ]);
    // REVIEW[epic=closure-dep-formats] - should this be a Dep-Format registration (module.define_format_args) instead?
    let anchor_format3 = om.module().define_format_views(
        "opentype.layout.anchor_table.format3",
        vec![Label::Borrowed("table_view")],
        record_auto([
            ("table_scope", reify_view(vvar("table_view"))),
            ("x_coordinate", i16be()),
            ("y_coordinate", i16be()),
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
    om.module().define_format(
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
                                anchor_format3.call_args_views(vec![], vec![vvar("table_view")]),
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
fn feature_variation_record(
    module: &mut FormatModule,
    feature_table: FormatRef,
    f2dot14: FormatRef,
) -> FormatRef {
    let condition_table = util::embedded_singleton_alternation(
        [("format", u16be())],
        ("format", 1),
        [
            ("axis_index", u16be()),
            ("filter_range_min_value", f2dot14.call()),
            ("filter_range_max_value", f2dot14.call()),
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
                // readarray-eligible: bare u32be element -> BaseKind::U32BE, no FormatRef needed.
                // Read sequentially at the cursor inside the `set_view`-scoped record (the offsets
                // are only dereferenced afterward, via the `#_conditions` for_each below), so it
                // would need `from_here(read_array(var("condition_count"), BaseKind::U32BE))`.
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
    om: &mut OpentypeModule<'_>,
    feature_table: FormatRef,
) -> FormatRef {
    let f2dot14 = om.f2dot14();
    let module = om.module();
    let feature_variation_record = feature_variation_record(module, feature_table, f2dot14);

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
                        feature_variation_record.call_args_views(vec![], vec![vvar("table_view")]),
                    ),
                ),
            ]),
        ),
    )
}

/// Format-factory taking a `{GPOS,GSUB}`-specific `lookup_subtable` and constructing the shape of a LookupTable around it
pub(crate) fn lookup_table(lookup_subtable: DepFormat<1, 0>) -> Format {
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
            FlagBit("ignore_marks"),           // Bit 3 (0x8) - if set, skips  over combining marks
            FlagBit("ignore_ligatures"),       // Bit 2 (0x4) - if set, skips over ligatures
            FlagBit("ignore_base_glyphs"),     // Bit 1 (0x2) - if set, skips over base glyphs
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
                        lookup_subtable.invoke_args([var("lookup_type")]),
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
                    util::read_phantom_view_offset32(vvar("table_view"), feature_variations.call()),
                ),
            ),
        ]),
    )
}
