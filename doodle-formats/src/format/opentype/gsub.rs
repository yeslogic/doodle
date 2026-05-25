use doodle::DepFormat;

use super::*;

/// GSUB-specific LookupSubtable implementation
pub(crate) fn lookup_subtable(
    module: &mut FormatModule,
    subst_extension: FormatRef,
    ground_subst: DepFormat<1, 0>,
) -> DepFormat<1, 0> {
    const EXTENSION_TYPE: u16 = 7;
    module.register_format_args(
        "opentype.gsub.lookup_subtable",
        [(Label::from("lookup_type"), ValueType::U16)],
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
                    ground_subst.invoke_args([var("lookup_type")]),
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
            ("delta_glyph_id", i16be()),
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
pub(crate) fn multiple_subst(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
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
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
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
pub(crate) fn alternate_subst(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
    let alternate_set = module.define_format(
        "opentype.gsub.alternate_subst.alternate_set",
        record([
            ("glyph_count", u16be()),
            (
                "alternate_glyph_ids",
                repeat_count(var("glyph_count"), u16be()),
            ),
        ]),
    );
    module.define_format(
        "opentype.gsub.alternate_subst",
        let_view(
            "table_view",
            util::embedded_singleton_alternation(
                [
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("subst_format", u16be()),
                    (
                        "coverage",
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                ],
                ("subst_format", 1),
                [
                    ("alternate_set_count", u16be()),
                    (
                        "alternate_sets",
                        repeat_count(
                            var("alternate_set_count"),
                            util::read_phantom_view_offset16(
                                vvar("table_view"),
                                alternate_set.call(),
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

/// Loookup type 4 subtable: ligature substitution
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gsub#lookup-type-4-subtable-ligature-substitution
pub(crate) fn ligature_subst(module: &mut FormatModule, coverage_table: FormatRef) -> FormatRef {
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
                        util::read_phantom_view_offset16(vvar("set_view"), ligature_table.call()),
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
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
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
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
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
) -> DepFormat<1, 0> {
    let single_subst = single_subst(module, coverage_table);
    let multiple_subst = multiple_subst(module, coverage_table);
    let alternate_subst = alternate_subst(module, coverage_table);
    let ligature_subst = ligature_subst(module, coverage_table);
    let reverse_chain_single_subst = reverse_chain_single_subst(module, coverage_table);
    module.register_format_args(
        "opentype.layout.ground_subst",
        [(Label::from("lookup_type"), ValueType::U16)],
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
pub(crate) fn subst_extension(
    module: &mut FormatModule,
    ground_subst: DepFormat<1, 0>,
) -> FormatRef {
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
                            ground_subst.invoke_args([var("extension_lookup_type")]),
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
    ground_subst: DepFormat<1, 0>,
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
        "opentype.gsub.table",
        layout::table(script_list, feature_list, lookup_list, feature_variations),
    )
}
