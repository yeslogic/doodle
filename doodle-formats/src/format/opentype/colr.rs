use super::*;

/// Format specification for `COLR` table (header)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#colr-header
///
/// Reuses the `item_variation_store` and `delta_set_index_map` formats
pub(crate) fn table(
    module: &mut FormatModule,
    item_variation_store: FormatRef,
    delta_set_index_map: FormatRef,
) -> FormatRef {
    let paint_table = paint_table(module);
    let base_glyph_record: FormatRef = base_glyph_record(module);
    let layer_record: FormatRef = layer_record(module);
    let base_glyph_list: FormatRef = base_glyph_list(module, paint_table);
    let layer_list: FormatRef = layer_list(module, paint_table);
    let clip_list: FormatRef = clip_list(module);
    module.define_format(
        "opentype.colr.table",
        let_view(
            "table_view",
            embedded_variadic_alternation(
                [
                    ("table_scope", reify_view(vvar("table_view"))),
                    ("version", expect_range_u16be(0, 1)),
                    ("num_base_glyph_records", u16be()),
                    // NOTE - because of phantom, `repeat` is technically acceptable to inform the typechecking, but it is unsafe if the phantom-format is actually parsed
                    // NOTE - beause of how `num_layer_records` is ordered, we only choose not to use `repeat_count` here to establish consistency with how `layer_records` is repeated below
                    (
                        "base_glyph_records",
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            // TODO - this should technically be `repeat_count(var("num_base_glyph_records"), ..)` but there is no practical difference due to how phantoms are handled
                            repeat(base_glyph_record.call()),
                        ),
                    ),
                    // NOTE - because of phantom, `repeat` is technically acceptable to inform the typechecking, but it is unsafe if the phantom-format is actually parsed
                    // NOTE - we ideally would like to be able to use `repeat_count` but there is no clean way of doing this without jumping through extra hoops for no appreciable benefit
                    (
                        "layer_records",
                        util::read_phantom_view_offset32(
                            vvar("table_view"),
                            // TODO - this should technically be `repeat_count(var("num_layer_records"), ..)` but it isn't possible to do this without extra work and there isn't any real upside
                            repeat(layer_record.call()),
                        ),
                    ),
                    ("num_layer_records", u16be()),
                ],
                "version",
                [
                    (0u16, "Version0", Vec::new()),
                    (
                        1u16,
                        "Version1",
                        vec![
                            (
                                "base_glyph_list",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    base_glyph_list.call(),
                                ),
                            ),
                            // the following are optional (i.e. nullable)
                            (
                                "layer_list",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    layer_list.call(),
                                ),
                            ),
                            (
                                "clip_list",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    clip_list.call(),
                                ),
                            ),
                            (
                                "var_index_map",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    delta_set_index_map.call(),
                                ),
                            ),
                            (
                                "item_variation_store",
                                util::read_phantom_view_offset32(
                                    vvar("table_view"),
                                    item_variation_store.call(),
                                ),
                            ),
                        ],
                    ),
                ],
                "extra",
                NestingKind::MinimalVariation,
            ),
        ),
    )
}

/// Paint table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#paint-tables
fn paint_table(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.colr.paint_table",
        // FIXME - provide actual implementation
        // STUB
        Format::EMPTY,
    )
}

/// BaseGlyph record format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyph-and-layer-records
fn base_glyph_record(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.colr.base_glyph_record",
        record([
            ("glyph_id", u16be()),          // glyph id of base glyph
            ("first_layer_index", u16be()), // Index (base 0) into layerRecords array
            ("num_layers", u16be()),        // number of color layers associated with glyph
        ]),
    )
}

/// Layer record format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyph-and-layer-records
fn layer_record(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.colr.layer_record",
        record([
            ("glyph_id", u16be()),      // glyph id of glyph used for a given layer
            ("palette_index", u16be()), // Index (base 0) for a palette entry in CPAL table
        ]),
    )
}

/// BaseGlyphList format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist
fn base_glyph_list(module: &mut FormatModule, paint_table: FormatRef) -> FormatRef {
    let base_glyph_paint_record: DepFormat<0, 1> = base_glyph_paint_record(module, paint_table);
    module.define_format(
        "opentype.colr.base_glyph_list",
        let_view(
            "list_view",
            record([
                ("list_scope", reify_view(vvar("list_view"))),
                ("num_base_glyph_paint_records", u32be()),
                (
                    "base_glyph_paint_records",
                    repeat_count(
                        var("num_base_glyph_paint_records"),
                        base_glyph_paint_record.invoke_view(vvar("list_view")),
                    ),
                ),
            ]),
        ),
    )
}

fn base_glyph_paint_record(module: &mut FormatModule, paint_table: FormatRef) -> DepFormat<0, 1> {
    module.register_format_view(
        "opentype.colr.base_glyph_paint_record",
        Label::Borrowed("list_view"),
        record([
            ("glyph_id", u16be()),
            (
                "paint",
                util::read_phantom_view_offset32(vvar("list_view"), paint_table.call()),
            ),
        ]),
    )
}

/// LayerList format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist
fn layer_list(module: &mut FormatModule, paint_table: FormatRef) -> FormatRef {
    module.define_format(
        "opentype.colr.layer_list",
        let_view(
            "list_view",
            record([
                ("list_scope", reify_view(vvar("list_view"))),
                ("num_layers", u32be()),
                (
                    "paint_tables",
                    repeat_count(
                        var("num_layers"),
                        read_phantom_view_offset32(vvar("list_view"), paint_table.call()),
                    ),
                ),
            ]),
        ),
    )
}

/// ClipList format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#baseglyphlist-layerlist-and-cliplist
fn clip_list(module: &mut FormatModule) -> FormatRef {
    let clip_box: FormatRef = clip_box(module);
    let clip_record = module.register_format_view(
        "opentype.colr.clip_record",
        Label::Borrowed("list_view"),
        record([
            ("start_glyph_id", u16be()),
            ("end_glyph_id", u16be()),
            (
                "clip_box",
                util::read_phantom_view_offset24(vvar("list_view"), clip_box.call()),
            ),
        ]),
    );
    module.define_format(
        "opentype.colr.clip_list",
        let_view(
            "list_view",
            record([
                ("list_scope", reify_view(vvar("list_view"))),
                ("format", expect_eq(u8(), Expr::U8(1))),
                ("num_clips", u32be()),
                (
                    "clips",
                    repeat_count(var("num_clips"), clip_record.invoke_view(vvar("list_view"))),
                ),
            ]),
        ),
    )
}

fn clip_box(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.colr.clip_box",
        embedded_variadic_alternation(
            [
                ("format", u8()),
                ("x_min", i16be()),
                ("y_min", i16be()),
                ("x_max", i16be()),
                ("y_max", i16be()),
            ],
            "format",
            [
                (1u8, "Format1", Vec::new()),
                (2u8, "Format2", vec![("var_index_base", u32be())]),
            ],
            "extra_fields",
            NestingKind::UnifiedRecord,
        ),
    )
}
