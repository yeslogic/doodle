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

mod paint_table {
    use super::*;

    // --- Helper sub-table builders ---

    /// ColorStop record
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#colorstop-record
    fn color_stop() -> Format {
        record([
            ("stop_offset", util::f2dot14()),
            ("palette_index", u16be()),
            ("alpha", util::f2dot14()),
        ])
    }

    /// VarColorStop record
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#colorstop-record
    fn var_color_stop() -> Format {
        record([
            ("stop_offset", util::f2dot14()),
            ("palette_index", u16be()),
            ("alpha", util::f2dot14()),
            ("var_index_base", u32be()),
        ])
    }

    /// ColorLine table (non-var)
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#colorline-table
    fn color_line(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.colr.color_line",
            record([
                ("extend", u8()),
                ("num_stops", u16be()),
                ("color_stops", repeat_count(var("num_stops"), color_stop())),
            ]),
        )
    }

    /// VarColorLine table
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#colorline-table
    fn var_color_line(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.colr.var_color_line",
            record([
                ("extend", u8()),
                ("num_stops", u16be()),
                (
                    "color_stops",
                    repeat_count(var("num_stops"), var_color_stop()),
                ),
            ]),
        )
    }

    /// Affine2x3 record (non-var)
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#affine2x3-record
    fn affine2x3(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.colr.affine2x3",
            record([
                ("xx", util::fixed32be()), // Fixed 16.16
                ("yx", util::fixed32be()),
                ("xy", util::fixed32be()),
                ("yy", util::fixed32be()),
                ("dx", util::fixed32be()),
                ("dy", util::fixed32be()),
            ]),
        )
    }

    /// VarAffine2x3 record
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#affine2x3-record
    fn var_affine2x3(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "opentype.colr.var_affine2x3",
            record([
                ("xx", util::fixed32be()),
                ("yx", util::fixed32be()),
                ("xy", util::fixed32be()),
                ("yy", util::fixed32be()),
                ("dx", util::fixed32be()),
                ("dy", util::fixed32be()),
                ("var_index_base", u32be()),
            ]),
        )
    }

    // --- Non-recursive paint format factories ---

    /// Format 1: PaintColrLayers
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#format-1-paintcolrlayers
    fn paint_colr_layers() -> Format {
        record_auto([
            ("_format", is_byte(1)),
            ("num_layers", u8()),
            ("first_layer_index", u32be()),
        ])
    }

    /// Format 2: PaintSolid
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-2-and-3-paintsolid-paintvarsolid
    fn paint_solid() -> Format {
        record_auto([
            ("_format", is_byte(2)),
            ("palette_index", u16be()),
            ("alpha", util::f2dot14()),
        ])
    }

    /// Format 3: PaintVarSolid
    fn paint_var_solid() -> Format {
        record_auto([
            ("_format", is_byte(3)),
            ("palette_index", u16be()),
            ("alpha", util::f2dot14()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 11: PaintColrGlyph
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#format-11-paintcolrglyph
    fn paint_colr_glyph() -> Format {
        record_auto([("_format", is_byte(11)), ("glyph_id", u16be())])
    }

    // --- Gradient paint format factories (Offset24 to ColorLine sub-table) ---

    /// Format 4: PaintLinearGradient
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-4-and-5-paintlineargradient-paintvarlineargradient
    fn paint_linear_gradient(view: ViewExpr, color_line: FormatRef) -> Format {
        record_auto([
            ("_format", is_byte(4)),
            (
                "color_line",
                util::read_phantom_view_offset24(view, color_line.call()),
            ),
            ("x0", i16be()),
            ("y0", i16be()),
            ("x1", i16be()),
            ("y1", i16be()),
            ("x2", i16be()),
            ("y2", i16be()),
        ])
    }

    /// Format 5: PaintVarLinearGradient
    fn paint_var_linear_gradient(view: ViewExpr, var_color_line: FormatRef) -> Format {
        record_auto([
            ("_format", is_byte(5)),
            (
                "color_line",
                util::read_phantom_view_offset24(view, var_color_line.call()),
            ),
            ("x0", i16be()),
            ("y0", i16be()),
            ("x1", i16be()),
            ("y1", i16be()),
            ("x2", i16be()),
            ("y2", i16be()),
        ])
    }

    /// Format 6: PaintRadialGradient
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-6-and-7-paintradialgradient-paintvarradialgradient
    fn paint_radial_gradient(view: ViewExpr, color_line: FormatRef) -> Format {
        record_auto([
            ("_format", is_byte(6)),
            (
                "color_line",
                util::read_phantom_view_offset24(view, color_line.call()),
            ),
            ("x0", i16be()),
            ("y0", i16be()),
            ("radius0", u16be()),
            ("x1", i16be()),
            ("y1", i16be()),
            ("radius1", u16be()),
        ])
    }

    /// Format 7: PaintVarRadialGradient
    fn paint_var_radial_gradient(view: ViewExpr, var_color_line: FormatRef) -> Format {
        record_auto([
            ("_format", is_byte(7)),
            (
                "color_line",
                util::read_phantom_view_offset24(view, var_color_line.call()),
            ),
            ("x0", i16be()),
            ("y0", i16be()),
            ("radius0", u16be()),
            ("x1", i16be()),
            ("y1", i16be()),
            ("radius1", u16be()),
        ])
    }

    /// Format 8: PaintSweepGradient
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-8-and-9-paintsweepgradient-paintvarsweepgradient
    fn paint_sweep_gradient(view: ViewExpr, color_line: FormatRef) -> Format {
        record_auto([
            ("_format", is_byte(8)),
            (
                "color_line",
                util::read_phantom_view_offset24(view, color_line.call()),
            ),
            ("center_x", i16be()),
            ("center_y", i16be()),
            ("start_angle", util::f2dot14()),
            ("end_angle", util::f2dot14()),
        ])
    }

    /// Format 9: PaintVarSweepGradient
    fn paint_var_sweep_gradient(view: ViewExpr, var_color_line: FormatRef) -> Format {
        record_auto([
            ("_format", is_byte(9)),
            (
                "color_line",
                util::read_phantom_view_offset24(view, var_color_line.call()),
            ),
            ("center_x", i16be()),
            ("center_y", i16be()),
            ("start_angle", util::f2dot14()),
            ("end_angle", util::f2dot14()),
        ])
    }

    // --- Recursive paint format factories ---
    //
    // Child paint references are 3-byte (Offset24) offsets per the COLRv1 spec.
    // `phantom_embed` is already `Format::Phantom(...)`, so it must NOT be passed through
    // `read_phantom_view_offset*` (which would add a second Phantom layer). Instead, the raw
    // 24-bit offset is read as a plain `util::u24be()` field and the phantom is embedded
    // directly as a sibling field for type-tracking purposes.

    /// Format 10: PaintGlyph
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#format-10-paintglyph
    fn paint_glyph(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(10)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("glyph_id", u16be()),
        ])
    }

    /// Format 12: PaintTransform
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-12-and-13-painttransform-paintvartransform
    fn paint_transform(view: ViewExpr, phantom_embed: Format, affine2x3: FormatRef) -> Format {
        record_auto([
            ("_format", is_byte(12)),
            ("table_scope", reify_view(view.clone())),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            (
                "transform",
                util::read_phantom_view_offset24(view, affine2x3.call()),
            ),
        ])
    }

    /// Format 13: PaintVarTransform
    fn paint_var_transform(
        view: ViewExpr,
        phantom_embed: Format,
        var_affine2x3: FormatRef,
    ) -> Format {
        record_auto([
            ("_format", is_byte(13)),
            ("table_scope", reify_view(view.clone())),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            (
                "transform",
                util::read_phantom_view_offset24(view, var_affine2x3.call()),
            ),
        ])
    }

    /// Format 14: PaintTranslate
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-14-and-15-painttranslate-paintvartranslate
    fn paint_translate(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(14)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("dx", i16be()),
            ("dy", i16be()),
        ])
    }

    /// Format 15: PaintVarTranslate
    fn paint_var_translate(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(15)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("dx", i16be()),
            ("dy", i16be()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 16: PaintScale
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-16-to-23-paintscale-and-variant-paint-scale-formats
    fn paint_scale(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(16)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale_x", util::f2dot14()),
            ("scale_y", util::f2dot14()),
        ])
    }

    /// Format 17: PaintVarScale
    fn paint_var_scale(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(17)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale_x", util::f2dot14()),
            ("scale_y", util::f2dot14()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 18: PaintScaleAroundCenter
    fn paint_scale_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(18)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale_x", util::f2dot14()),
            ("scale_y", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
        ])
    }

    /// Format 19: PaintVarScaleAroundCenter
    fn paint_var_scale_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(19)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale_x", util::f2dot14()),
            ("scale_y", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 20: PaintScaleUniform
    fn paint_scale_uniform(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(20)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale", util::f2dot14()),
        ])
    }

    /// Format 21: PaintVarScaleUniform
    fn paint_var_scale_uniform(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(21)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale", util::f2dot14()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 22: PaintScaleUniformAroundCenter
    fn paint_scale_uniform_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(22)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
        ])
    }

    /// Format 23: PaintVarScaleUniformAroundCenter
    fn paint_var_scale_uniform_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(23)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("scale", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 24: PaintRotate
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-24-to-27-paintrotate-and-variant-paint-rotate-formats
    fn paint_rotate(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(24)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("angle", util::f2dot14()),
        ])
    }

    /// Format 25: PaintVarRotate
    fn paint_var_rotate(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(25)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("angle", util::f2dot14()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 26: PaintRotateAroundCenter
    fn paint_rotate_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(26)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("angle", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
        ])
    }

    /// Format 27: PaintVarRotateAroundCenter
    fn paint_var_rotate_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(27)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("angle", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 28: PaintSkew
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#formats-28-to-31-paintskew-and-variant-paint-skew-formats
    fn paint_skew(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(28)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("x_skew_angle", util::f2dot14()),
            ("y_skew_angle", util::f2dot14()),
        ])
    }

    /// Format 29: PaintVarSkew
    fn paint_var_skew(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(29)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("x_skew_angle", util::f2dot14()),
            ("y_skew_angle", util::f2dot14()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 30: PaintSkewAroundCenter
    fn paint_skew_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(30)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("x_skew_angle", util::f2dot14()),
            ("y_skew_angle", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
        ])
    }

    /// Format 31: PaintVarSkewAroundCenter
    fn paint_var_skew_around_center(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(31)),
            ("table_scope", reify_view(view)),
            (
                "paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
            ("x_skew_angle", util::f2dot14()),
            ("y_skew_angle", util::f2dot14()),
            ("center_x", i16be()),
            ("center_y", i16be()),
            ("var_index_base", u32be()),
        ])
    }

    /// Format 32: PaintComposite
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#format-32-paintcomposite
    fn paint_composite(view: ViewExpr, phantom_embed: Format) -> Format {
        record_auto([
            ("_format", is_byte(32)),
            ("table_scope", reify_view(view)),
            (
                "source_paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed.clone())]),
            ),
            ("composite_mode", u8()),
            (
                "backdrop_paint",
                record_auto([("offset", util::u24be()), ("#_data", phantom_embed)]),
            ),
        ])
    }

    /// Paint table format definition — a self-referential union of all 32 COLRv1 PaintFormat variants.
    ///
    /// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/colr#paint-tables
    pub(super) fn paint_table(module: &mut FormatModule) -> FormatRef {
        let color_line_ref = color_line(module);
        let var_color_line_ref = var_color_line(module);
        let affine2x3_ref = affine2x3(module);
        let var_affine2x3_ref = var_affine2x3(module);

        let mk_table = move |phantom_embed: Format| -> Format {
            let_view(
                "table_view",
                alts([
                    ("PaintColrLayers", paint_colr_layers()), // Format  1
                    ("PaintSolid", paint_solid()),            // Format  2
                    ("PaintVarSolid", paint_var_solid()),     // Format  3
                    (
                        "PaintLinearGradient",
                        paint_linear_gradient(vvar("table_view"), color_line_ref),
                    ), // Format  4
                    (
                        "PaintVarLinearGradient",
                        paint_var_linear_gradient(vvar("table_view"), var_color_line_ref),
                    ), // Format  5
                    (
                        "PaintRadialGradient",
                        paint_radial_gradient(vvar("table_view"), color_line_ref),
                    ), // Format  6
                    (
                        "PaintVarRadialGradient",
                        paint_var_radial_gradient(vvar("table_view"), var_color_line_ref),
                    ), // Format  7
                    (
                        "PaintSweepGradient",
                        paint_sweep_gradient(vvar("table_view"), color_line_ref),
                    ), // Format  8
                    (
                        "PaintVarSweepGradient",
                        paint_var_sweep_gradient(vvar("table_view"), var_color_line_ref),
                    ), // Format  9
                    (
                        "PaintGlyph",
                        paint_glyph(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 10
                    ("PaintColrGlyph", paint_colr_glyph()),   // Format 11
                    (
                        "PaintTransform",
                        paint_transform(
                            vvar("table_view").clone(),
                            phantom_embed.clone(),
                            affine2x3_ref,
                        ),
                    ), // Format 12
                    (
                        "PaintVarTransform",
                        paint_var_transform(
                            vvar("table_view").clone(),
                            phantom_embed.clone(),
                            var_affine2x3_ref,
                        ),
                    ), // Format 13
                    (
                        "PaintTranslate",
                        paint_translate(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 14
                    (
                        "PaintVarTranslate",
                        paint_var_translate(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 15
                    (
                        "PaintScale",
                        paint_scale(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 16
                    (
                        "PaintVarScale",
                        paint_var_scale(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 17
                    (
                        "PaintScaleAroundCenter",
                        paint_scale_around_center(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 18
                    (
                        "PaintVarScaleAroundCenter",
                        paint_var_scale_around_center(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 19
                    (
                        "PaintScaleUniform",
                        paint_scale_uniform(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 20
                    (
                        "PaintVarScaleUniform",
                        paint_var_scale_uniform(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 21
                    (
                        "PaintScaleUniformAroundCenter",
                        paint_scale_uniform_around_center(
                            vvar("table_view"),
                            phantom_embed.clone(),
                        ),
                    ), // Format 22
                    (
                        "PaintVarScaleUniformAroundCenter",
                        paint_var_scale_uniform_around_center(
                            vvar("table_view"),
                            phantom_embed.clone(),
                        ),
                    ), // Format 23
                    (
                        "PaintRotate",
                        paint_rotate(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 24
                    (
                        "PaintVarRotate",
                        paint_var_rotate(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 25
                    (
                        "PaintRotateAroundCenter",
                        paint_rotate_around_center(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 26
                    (
                        "PaintVarRotateAroundCenter",
                        paint_var_rotate_around_center(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 27
                    (
                        "PaintSkew",
                        paint_skew(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 28
                    (
                        "PaintVarSkew",
                        paint_var_skew(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 29
                    (
                        "PaintSkewAroundCenter",
                        paint_skew_around_center(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 30
                    (
                        "PaintVarSkewAroundCenter",
                        paint_var_skew_around_center(vvar("table_view"), phantom_embed.clone()),
                    ), // Format 31
                    (
                        "PaintComposite",
                        paint_composite(vvar("table_view"), phantom_embed),
                    ), // Format 32
                ]),
            )
        };

        let embed = |view: ViewExpr| {
            |self_ref: FormatRef| -> Format {
                parse_view_offset::<U32>(view, var("offset"), self_ref.call())
            }
        };
        module.define_format_phantom_rec(
            "opentype.colr.paint_table",
            mk_table,
            embed(vvar("table_view")),
        )
    }
}
use paint_table::paint_table;

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
