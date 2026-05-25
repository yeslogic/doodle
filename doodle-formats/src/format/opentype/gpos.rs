use super::*;
use doodle::DepFormat;

/// GPOS-specific LookupSubtable implementation
///
/// Parametric over `lookup_type :~ U16`.
fn lookup_subtable(
    module: &mut FormatModule,
    pos_extension: FormatRef,
    ground_pos: DepFormat<1, 0>,
) -> DepFormat<1, 0> {
    const EXTENSION_TYPE: u16 = 9;
    module.register_format_args(
        "opentype.gpos.lookup_subtable",
        [(Label::Borrowed("lookup_type"), ValueType::U16)],
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
                    ground_pos.invoke_args([var("lookup_type")]),
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
                value_record.call_args_views(vec![var("value_format")], vec![vvar("table_view")]),
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
                layout::optional_value_record(value_record, vvar("set_view"), var("value_format1")),
            ),
            (
                "value_record2",
                layout::optional_value_record(value_record, vvar("set_view"), var("value_format2")),
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
                class2_record.call_args_views(vec![value_format1, value_format2], vec![table_view]),
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
                                pair_pos_format1.call_args_views(vec![], vec![vvar("table_view")]),
                            ),
                            (
                                Pattern::U16(2),
                                "Format2",
                                pair_pos_format2.call_args_views(vec![], vec![vvar("table_view")]),
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
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
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
    pub(crate) fn mark_array(module: &mut FormatModuleExt, anchor_table: FormatRef) -> FormatRef {
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
    let base_record = module.register_format_args_views(
        "opentype.layout.base_array.base_record",
        [(Label::Borrowed("mark_class_count"), ValueType::U16)],
        [Label::Borrowed("_array_view")],
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
    let base_array = module.register_format_args(
        "opentype.layout.base_array",
        [(Label::Borrowed("mark_class_count"), ValueType::U16)],
        let_view(
            "array_view",
            record_auto([
                ("array_scope", reify_view(vvar("array_view"))),
                ("base_count", u16be()),
                (
                    "base_records",
                    repeat_count(
                        var("base_count"),
                        base_record
                            .invoke_args_views([var("mark_class_count")], [vvar("array_view")]),
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
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                    (
                        "base_coverage",
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
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
                            base_array.invoke_args([var("mark_class_count")]),
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

    /// Opentype Component Record (GPOS Mark-Lig Attachment) format definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-5-subtable-mark-to-ligature-attachment-positioning
    ///
    /// Parametric in `mark_class_count :~ U16` and `table_view ~ <LigatureAttach table frame>`.
    fn component_record(module: &mut FormatModule, anchor_table: FormatRef) -> DepFormat<1, 1> {
        module.register_format_args_views(
            "opentype.layout.ligature_attach.component_record",
            [(Label::Borrowed("mark_class_count"), ValueType::U16)],
            [Label::Borrowed("table_view")],
            record_auto([
                // REVIEW[epic=nested-format-reify-layer] - INNER(local)
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

    /// Opentype Ligature Attach (GPOS Mark-Lig Attachment) format definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-5-subtable-mark-to-ligature-attachment-positioning
    ///
    /// Parametric in `mark_class_count :~ U16`
    fn ligature_attach(module: &mut FormatModule, anchor_table: FormatRef) -> DepFormat<1, 0> {
        let component_record = component_record(module, anchor_table);

        module.register_format_args(
            "opentype.layout.ligature_attach",
            [(Label::Borrowed("mark_class_count"), ValueType::U16)],
            let_view(
                "table_view",
                record([
                    // REVIEW[epic=nested-format-reify-layer] - scope reified in inner format
                    ("component_count", u16be()),
                    (
                        "component_records",
                        repeat_count(
                            var("component_count"),
                            component_record
                                .invoke_args_views([var("mark_class_count")], [vvar("table_view")]),
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
    ) -> DepFormat<1, 0> {
        let ligature_attach = ligature_attach(module, anchor_table);

        module.register_format_args(
            "opentype.layout.ligature_array",
            [(Label::Borrowed("mark_class_count"), ValueType::U16)],
            let_view(
                "array_view",
                record_auto([
                    ("array_scope", reify_view(vvar("array_view"))),
                    // FIXME - reduplicated from outer format for context-free expansion
                    ("mark_class_count", compute(var("mark_class_count"))),
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
                                ligature_attach.invoke_args([var("mark_class_count")]),
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
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                    (
                        "ligature_coverage",
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
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
                            ligature_array.invoke_args([var("mark_class_count")]),
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

    /// Opentype Mark2 Record (GPOS Mark-to-Mark Attachment) format definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-6-subtable-mark-to-mark-attachment-positioning
    ///
    /// Parametric in `mark_class_count :~ U16` and `view ~ <Mark2Array table frame>`
    fn mark2_record(module: &mut FormatModule, anchor_table: FormatRef) -> DepFormat<1, 1> {
        module.register_format_args_views(
            "opentype.layout.mark2_array.mark2_record",
            [(Label::Borrowed("mark_class_count"), ValueType::U16)],
            [Label::Borrowed("_array_view")],
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

    /// Mark2Array (GPOS Mark-to-Mark Attachment) format definition
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-6-subtable-mark-to-mark-attachment-positioning
    ///
    /// Parametric in `mark_class_count :~ U16`
    pub(super) fn mark2_array(
        module: &mut FormatModule,
        anchor_table: FormatRef,
    ) -> DepFormat<1, 0> {
        let mark2_record = mark2_record(module, anchor_table);

        module.register_format_args(
            "opentype.layout.mark2_array",
            [(Label::Borrowed("mark_class_count"), ValueType::U16)],
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
                            mark2_record
                                .invoke_args_views([var("mark_class_count")], [vvar("array_view")]),
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
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
                    ),
                    (
                        "mark2_coverage",
                        util::read_phantom_view_offset16(vvar("table_view"), coverage_table.call()),
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
                            mark2_array.invoke_args([var("mark_class_count")]),
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
///
/// Parametric over `lookup_type :~ U16`
pub(crate) fn ground_pos(
    module: &mut FormatModule,
    class_def: FormatRef,
    coverage_table: FormatRef,
    value_format_flags: FormatRef,
    value_record: FormatRef,
    anchor_table: FormatRef,
    sequence_context: FormatRef,
    chained_sequence_context: FormatRef,
) -> DepFormat<1, 0> {
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

    module.register_format_args(
        "opentype.layout.ground_pos",
        [(Label::from("lookup_type"), ValueType::U16)],
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
pub(crate) fn pos_extension(module: &mut FormatModule, ground_pos: DepFormat<1, 0>) -> FormatRef {
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
                            ground_pos.invoke_args([var("extension_lookup_type")]),
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
    ground_pos: DepFormat<1, 0>,
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
        "opentype.gpos.table",
        layout::table(script_list, feature_list, lookup_list, feature_variations),
    )
}
