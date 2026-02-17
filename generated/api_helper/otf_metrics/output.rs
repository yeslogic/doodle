use std::backtrace;

use super::display::{
    Token::{self, LineBreak},
    TokenStream, tok, toks,
};
use super::*;

// TODO - refactor into more cleanly encapsulated TokenStream producers and consumers

pub fn show_opentype_stats(metrics: &OpentypeMetrics, conf: &Config) {
    match metrics {
        OpentypeMetrics::MultiFont(multi) => {
            println!(
                "TTC: version {} ({} fonts)",
                format_version_major_minor(multi.version.0, multi.version.1),
                multi.num_fonts
            );
            for (i, o_font) in multi.font_metrics.iter().enumerate() {
                match o_font.as_ref() {
                    Some(font) => {
                        println!("=== Font @ Index {i} ===");
                        show_font_metrics(font, conf);
                    }
                    None => {
                        println!("=== Skipping Index {i} ===");
                    }
                }
            }
        }
        OpentypeMetrics::SingleFont(single) => show_font_metrics(single, conf),
    }
}

/// Tokenizer specialized for displaying table-directory `sfntVersion` (inline)
fn display_sfnt_version(magic: u32) -> TokenStream<'static> {
    tok("sfntVersion: ").then(display_u32_ascii(magic)).paren()
}

/// Tokenizer used for displaying 32-bit binary values as four ASCII characters interpreted in big-endian order. (inline)
fn display_u32_ascii(val: u32) -> TokenStream<'static> {
    let bytes = val.to_be_bytes();
    let mut buf: [u8; 6] = *b"'    '";
    let mut space = false;
    let mut non_graphic = false;
    for (c, byte) in buf[1..5].iter_mut().zip(bytes) {
        *c = byte;
        if *c == b' ' {
            space = true
        } else if !c.is_ascii_graphic() {
            non_graphic = true;
            break;
        }
    }

    if non_graphic {
        toks(format!("0x{:08x}", val))
    } else if space {
        let s = str::from_utf8(&buf).unwrap();
        toks(s.to_string())
    } else {
        let s = str::from_utf8(&buf[1..5]).unwrap();
        toks(s.to_string())
    }
}

fn show_font_metrics(font: &SingleFontMetrics, conf: &Config) {
    if !conf.extra_only {
        display_sfnt_version(font.sfnt_version).println();
        // WIP - display_required_metrics(..).println();
        show_required_metrics(&font.required, conf);
        // WIP - show_optional_metrics(..).println();
        show_optional_metrics(&font.optional, conf);
    }
    display_extra_tags(&font.extraMagic).println();
}

fn display_extra_tags(table_ids: &[u32]) -> TokenStream<'static> {
    let lines = table_ids
        .iter()
        .map(|id| display_u32_ascii(*id).chain(toks(": [MISSING IMPL]")))
        .collect();
    TokenStream::join_with(lines, LineBreak)
}

fn show_required_metrics(required: &RequiredTableMetrics, conf: &Config) {
    display_cmap_metrics(&required.cmap, conf).println();
    // WIP - display_head_metrics(..).println();
    show_head_metrics(&required.head, conf);
    // WIP - display_hhea_metrics(..).println();
    show_hhea_metrics(&required.hhea, conf);
    display_hmtx_metrics(&required.hmtx, conf).println();
    // WIP - display_maxp_metrics(..).println();
    show_maxp_metrics(&required.maxp, conf);
    // WIP - display_name_metrics(..).println();
    show_name_metrics(&required.name, conf);
    // WIP - display_os2_metrics(..).println();
    show_os2_metrics(&required.os2, conf);
    // WIP - display_post_metrics(..).println();
    show_post_metrics(&required.post, conf);
}

fn show_optional_metrics(optional: &OptionalTableMetrics, conf: &Config) {
    // WIP - display_cvt_metrics(..).println();
    show_cvt_metrics(&optional.cvt, conf);
    // WIP - display_fpgm_metrics(..).println();
    show_fpgm_metrics(&optional.fpgm, conf);
    // WIP - display_loca_metrics(..).println();
    show_loca_metrics(&optional.loca, conf);
    // WIP - display_glyf_metrics(..).println();
    glyf::show_glyf_metrics(&optional.glyf, conf);
    // WIP - display_prep_metrics(..).println();
    show_prep_metrics(&optional.prep, conf);
    display_gasp_metrics(&optional.gasp, conf).println();

    // STUB - anything between gasp and base go here

    // TODO - refactor into proper TokenStreams
    display_base_metrics(&optional.base, conf).println();
    // WIP - display_gdef_metrics(..).println();
    show_gdef_metrics(optional.gdef.as_deref(), conf);
    display_layout_metrics(
        optional.gpos.as_deref(),
        Ctxt::from(TableDiscriminator::Gpos),
        conf,
    )
    .println();
    display_layout_metrics(
        optional.gsub.as_deref(),
        Ctxt::from(TableDiscriminator::Gsub),
        conf,
    )
    .println();

    // WIP - display_fvar_metrics(..).println();
    show_fvar_metrics(optional.fvar.as_deref(), conf);
    // WIP - display_gvar_metrics(..).println();
    show_gvar_metrics(optional.gvar.as_deref(), conf);

    // WIP - display_kern_metrics(..).println();
    show_kern_metrics(&optional.kern, conf);
    // WIP - display_stat_metrics(..).println();
    show_stat_metrics(optional.stat.as_deref(), conf);
    // WIP - display_vhea_metrics(..).println();
    show_vhea_metrics(&optional.vhea, conf);
    // WIP - display_vmtx_metrics(..).println();
    show_vmtx_metrics(&optional.vmtx, conf);
}

use gvar::show_gvar_metrics;
mod gvar {
    use super::*;

    // FIXME - rewrite into pure TokenStream
    pub(super) fn show_gvar_metrics(gvar: Option<&GvarMetrics>, conf: &Config) {
        let Some(gvar) = gvar else { return };
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            // WIP
            println!(
                "gvar: version {}",
                format_version_major_minor(gvar.major_version, gvar.minor_version)
            );
            println!("\tShared Tuples ({} total):", gvar.shared_tuples.len());
            display_shared_tuples(&gvar.shared_tuples).println();
            println!(
                "\tGlyph Variation Data ({} glyphs):",
                gvar.glyph_variation_data_array.len()
            );
            display_glyph_variation_data_array(&gvar.glyph_variation_data_array).println();
            // WIP
        } else {
            print!(
                "gvar: version {}",
                format_version_major_minor(gvar.major_version, gvar.minor_version)
            );
            println!(
                "; {} shared tuples, {} glyph variation data tables",
                gvar.shared_tuples.len(),
                gvar.glyph_variation_data_array.len(),
            );
        }
    }

    fn display_tuple_variation_header(header: &GvarTupleVariationHeader) -> TokenStream<'static> {
        toks(format!(
            "Header (size: {} bytes)",
            header.variation_data_size
        ))
        // STUB
    }

    fn display_glyph_variation_data(table: &GlyphVariationData) -> TokenStream<'static> {
        match &table.tuple_variation_headers[..] {
            [] => unreachable!("empty tuple variation headers"), // FIXME - final version should not panic, but we want to figure out whether this case happens
            [header] => {
                display_tuple_variation_header(header)
                // WIP - we may want to show more than just the header in future
            }
            headers => {
                let headers_str = arrayfmt::display_items_elided(
                    headers,
                    |ix, header| {
                        toks(format!("[{ix}]: "))
                            .pre_indent(6)
                            .chain(display_tuple_variation_header(header))
                    },
                    4,
                    |start, stop| {
                        toks(format!(
                            "(skipping tuple variation headers {start}..{stop})"
                        ))
                        .pre_indent(5)
                    },
                );
                // WIP - we may want to show more than just the headers in future
                LineBreak.then(headers_str)
            }
        }
    }

    fn display_glyph_variation_data_array(array: &[Option<GlyphVariationData>]) -> TokenStream<'_> {
        const TABLE_BOOKEND: usize = 4;
        arrayfmt::display_nullable(
            array,
            |ix, table| {
                toks(format!("[{ix}]: "))
                    .pre_indent(4)
                    .chain(display_glyph_variation_data(table))
            },
            TABLE_BOOKEND,
            |n, (start, stop)| {
                toks(format!(
                    "(skipping {n} glyph variation data tables between indices {start}..{stop})"
                ))
                .pre_indent(3)
            },
        )
    }

    fn display_shared_tuple_record(tuple: &GvarTupleRecord) -> TokenStream<'static> {
        const COORD_BOOKEND: usize = 4;
        arrayfmt::display_items_inline(
            &tuple.coordinates,
            |coord| toks(format!("{}", coord)),
            COORD_BOOKEND,
            |n_skipped| toks(format!("...({n_skipped} skipped)...")),
        )
    }

    fn display_shared_tuples(shared_tuples: &[GvarTupleRecord]) -> TokenStream<'static> {
        const RECORDS_BOOKEND: usize = 4;
        arrayfmt::display_items_elided(
            shared_tuples,
            |shared_tuple_ix, record| {
                toks(format!("[{shared_tuple_ix}]: "))
                    .chain(display_shared_tuple_record(record))
                    .pre_indent(4)
            },
            RECORDS_BOOKEND,
            |start, stop| toks(format!("(skipping shared tuples {start}..{stop})")).pre_indent(3),
        )
    }
}

use fvar::show_fvar_metrics;
mod fvar {
    use super::*;

    // FIXME - rewrite into pure TokenStream
    pub(super) fn show_fvar_metrics(fvar: Option<&FvarMetrics>, conf: &Config) {
        let Some(fvar) = fvar else { return };
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            println!(
                "fvar: version {}",
                format_version_major_minor(fvar.major_version, fvar.minor_version)
            );
            println!("\tAxes:");

            // FIXME: rewerite into pure TokenStream
            arrayfmt::display_items_elided(
                &fvar.axes,
                |ix, axis| tok(format!("\t\t[{ix}]: ")).then(display_variation_axis_record(axis)),
                conf.bookend_size,
                |start, stop| tok(format!("\t(skipping axis records {start}..{stop})")).into(),
            )
            .println();
            println!("\tInstances:");
            arrayfmt::display_items_elided(
                &fvar.instances,
                |ix, instance| {
                    tok(format!("\t\t[{ix}]: ")).then(display_instance_record(instance, conf))
                },
                conf.bookend_size,
                |start, stop| tok(format!("\t(skipping instance records {start}..{stop})")).into(),
            )
            .println();
        } else {
            // FIXME - rewrite into pure TokenStream
            print!(
                "fvar: version {}",
                format_version_major_minor(fvar.major_version, fvar.minor_version)
            );
            println!(
                "; {} axes, {} instances",
                fvar.axes.len(),
                fvar.instances.len()
            );
        }
    }

    fn display_instance_record(instance: &InstanceRecord, conf: &Config) -> TokenStream<'static> {
        // FIXME - rewrite into more natively TokenStream-oriented production
        tok(format!(
            "Subfamily={:?};{} ",
            instance.subfamily_nameid,
            match instance.postscript_nameid {
                None => String::new(),
                Some(name_id) => format!(" Postscript={name_id:?};"),
            },
        ))
        .then(arrayfmt::display_items_inline(
            &instance.coordinates,
            |coord| toks(format!("{coord:+}")),
            conf.inline_bookend,
            |n_skipped| toks(format!("...(skipping {n_skipped} coordinates)...")),
        ))
    }

    fn display_variation_axis_record(axis: &VariationAxisRecord) -> TokenStream<'static> {
        // TODO - rewrite in more natural TokenStream style
        toks(format!(
            "'{}' axis: [{}, {}] (default: {}){}{:?}",
            axis.axis_tag,
            axis.min_value,
            axis.max_value,
            axis.default_value,
            if axis.flags.hidden_axis {
                " (hidden) "
            } else {
                " "
            },
            axis.axis_name_id,
        ))
    }
}

// FIXME - rewrite into pure TokenStream
fn show_cvt_metrics(cvt: &Option<CvtMetrics>, _conf: &Config) {
    let Some(RawArrayMetrics(count)) = cvt else {
        return;
    };

    println!("cvt: FWORD[{count}]")
}

// FIXME - rewrite into pure TokenStream
fn show_fpgm_metrics(fpgm: &Option<FpgmMetrics>, _conf: &Config) {
    if let Some(RawArrayMetrics(count)) = fpgm {
        println!("fpgm: uint8[{count}]")
    }
}

// FIXME - rewrite into pure TokenStream
fn show_prep_metrics(prep: &Option<PrepMetrics>, _conf: &Config) {
    if let Some(RawArrayMetrics(count)) = prep {
        println!("prep: uint8[{count}]")
    }
}

// FIXME - rewrite into pure TokenStream
fn show_loca_metrics(loca: &Option<LocaMetrics>, _conf: &Config) {
    if let Some(()) = loca {
        println!("loca: (details omitted)")
    }
}

use gdef::show_gdef_metrics;

mod gdef {
    use super::*;

    // FIXME - rewrite into pure TokenStream
    pub(super) fn show_gdef_metrics(gdef: Option<&GdefMetrics>, conf: &Config) {
        if let Some(GdefMetrics {
            major_version,
            minor_version,
            glyph_class_def,
            attach_list,
            lig_caret_list,
            mark_attach_class_def,
            data,
        }) = gdef
        {
            // WIP
            println!(
                "GDEF: version {}",
                format_version_major_minor(*major_version, *minor_version)
            );
            if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
                if let Some(glyph_class_def) = glyph_class_def {
                    show_glyph_class_def(glyph_class_def, conf);
                }
                if let Some(attach_list) = attach_list {
                    display_attach_list(attach_list, conf).println(); // WIP
                }
                if let Some(lig_caret_list) = lig_caret_list {
                    // WIP
                    display_lig_caret_list(lig_caret_list, conf).println();
                }
                if let Some(mark_attach_class_def) = mark_attach_class_def {
                    show_mark_attach_class_def(mark_attach_class_def, conf);
                }
                match &data.mark_glyph_sets_def {
                    // WIP
                    None => println!("\tMarkGlyphSet: <none>"),
                    Some(mgs) => display_mark_glyph_set(mgs, conf).println(),
                }
                match &data.item_var_store {
                    None => println!("\tItemVariationStore: <none>"),
                    Some(ivs) => display_item_variation_store(ivs, conf).println(),
                }
            }
        }
    }
    fn display_attach_point(point_indices: &[u16], conf: &Config) -> TokenStream<'static> {
        arrayfmt::display_items_inline(
            point_indices,
            |point_ix| toks(u16::to_string(point_ix)),
            conf.inline_bookend,
            |num_skipped| toks(format!("...({num_skipped})...")),
        )
    }

    fn display_attach_list(attach_list: &AttachList, conf: &Config) -> TokenStream<'static> {
        toks("AttachList:")
            .pre_indent(2)
            .glue(
                LineBreak,
                display_coverage_table_full(&attach_list.coverage, conf).pre_indent(4),
            )
            .glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    &attach_list.attach_points,
                    |ix, AttachPoint { point_indices }| {
                        toks(format!("[{ix}]: "))
                            .pre_indent(4)
                            .chain(display_attach_point(point_indices, conf))
                    },
                    conf.bookend_size,
                    |start, stop| {
                        toks(format!(
                            "(skipping attach points for glyphs {start}..{stop})"
                        ))
                        .pre_indent(3)
                    },
                ),
            )
    }

    fn display_lig_caret_list(
        lig_caret_list: &LigCaretList,
        conf: &Config,
    ) -> TokenStream<'static> {
        toks("LigCaretList:")
            .glue(
                LineBreak,
                display_coverage_table_full(&lig_caret_list.coverage, conf).pre_indent(4),
            )
            .glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    &lig_caret_list.lig_glyphs,
                    |ix, lig_glyph| {
                        toks(format!("[{ix}]: ")).pre_indent(4).chain(
                            arrayfmt::display_items_inline(
                                &lig_glyph.caret_values,
                                display_caret_value,
                                conf.inline_bookend,
                                |num_skipped| toks(format!("...({num_skipped})...")),
                            ),
                        )
                    },
                    conf.bookend_size,
                    |start, stop| {
                        toks(format!("(skipping LigGlyphs {start}..{stop})")).pre_indent(3)
                    },
                ),
            )

        // NOTE - since coverage tables are used in MarkGlyphSet, we don't want to force-indent within the `show_coverage_table` function, so we do it before instead.
    }

    // FIXME - rewrite into pure TokenStream
    fn show_mark_attach_class_def(mark_attach_class_def: &ClassDef, conf: &Config) {
        println!("\tMarkAttachClassDef:");
        display_class_def(mark_attach_class_def, display_mark_attach_class, conf).println();
    }

    // FIXME - rewrite into pure TokenStream
    fn show_glyph_class_def(class_def: &ClassDef, conf: &Config) {
        println!("\tGlyphClassDef:");
        display_class_def(class_def, display_glyph_class, conf).println();
    }

    fn display_mark_attach_class(mark_attach_class: &u16) -> TokenStream<'static> {
        // STUB - if we come up with a semantic association for specific numbers, add branches here
        toks(format!("{mark_attach_class}"))
    }

    fn display_glyph_class(class: &u16) -> TokenStream<'static> {
        toks(format_glyph_class(class))
    }

    fn display_mark_glyph_set(mgs: &MarkGlyphSet, conf: &Config) -> TokenStream<'static> {
        tok("\tMarkGlyphSet:").then(LineBreak.then(arrayfmt::display_items_elided(
            &mgs.coverage,
            |ix, item| match item {
                None => toks(format!("\t\t[{ix}]: <none>")).into(),
                Some(covt) => {
                    tok(format!("\t\t[{ix}]: ")).then(display_coverage_table_full(covt, conf))
                }
            },
            conf.bookend_size,
            |start, stop| toks(format!("\t    (skipping coverage tables {start}..{stop})")),
        )))
    }

    fn display_caret_value(cv: &Option<CaretValue>) -> TokenStream<'static> {
        match cv {
            None => unreachable!("caret value null link"),
            Some(cv) => match cv {
                // REVIEW - this isn't really a canonical abbreviation, so we might adjust what we show for Design Units (Format 1)
                CaretValue::DesignUnits(du) => toks(format!("{du}du")),
                CaretValue::ContourPoint(ix) => toks(format!("#{ix}")),
                CaretValue::DesignUnitsWithTable { coordinate, device } => match device {
                    None => unreachable!("dev-table in caret value format 3 with null offset"),
                    Some(table) => tok(format!("{}du+", coordinate))
                        .then(display_device_or_variation_index_table(table)),
                },
            },
        }
    }

    fn display_item_variation_store(
        ivs: &ItemVariationStore,
        conf: &Config,
    ) -> TokenStream<'static> {
        tok("\tItemVariationStore:")
            .then(LineBreak.then(display_variation_regions(&ivs.variation_region_list, conf)))
            .chain(LineBreak.then(display_variation_data_array(
                &ivs.item_variation_data_list,
                conf,
            )))
    }
    fn display_variation_regions(vrl: &VariationRegionList, conf: &Config) -> TokenStream<'static> {
        tok(format!(
            "\t    VariationRegions: {} regions ({} axes)",
            vrl.0.len(),
            vrl.0[0].len()
        ))
        .then(LineBreak.then(arrayfmt::display_items_elided(
            &vrl.0,
            |ix, per_region| {
                tok(format!("\t\t[{ix}]:"))
                    .then(LineBreak.then(display_variation_axes(per_region, conf)))
            },
            conf.bookend_size,
            |start_ix, end_ix| toks(format!("\t    (skipping regions {start_ix}..{end_ix})")),
        )))
    }
    fn display_variation_data_array(
        ivda: &[Option<ItemVariationData>],
        conf: &Config,
    ) -> TokenStream<'static> {
        // WIP - refactor using pre-indent+glue
        tok(format!("\t    ItemVariationData[{}]", ivda.len())).then(LineBreak.then(
            arrayfmt::display_items_elided(
                ivda,
                |ix, o_ivd| match o_ivd {
                    Some(ivd) => {
                        tok(format!("\t\t[{ix}]: ")).then(display_variation_data(ivd, conf))
                    }
                    None => toks(format!("\t\t[{ix}]: <NONE>")),
                },
                conf.bookend_size,
                |start_ix, stop_ix| {
                    toks(format!(
                        "\t    ...(skipping entries {start_ix}..{stop_ix})..."
                    ))
                },
            ),
        ))
    }
    fn display_variation_data(ivd: &ItemVariationData, conf: &Config) -> TokenStream<'static> {
        let full_bits = if ivd.long_words { 32 } else { 16 };

        tok("ItemVariationData:")
            .then(LineBreak.then(
                tok(format!("\t\t\t{} region indices: ", ivd.region_index_count)).then(
                    arrayfmt::display_items_inline(
                        &ivd.region_indices,
                        |ix| toks(format!("{ix}")),
                        conf.inline_bookend,
                        |n_skipped| toks(format!("...({n_skipped})...")),
                    ),
                ),
            ))
            .chain(
                tok(format!(
                    "\t\t\t{} delta-sets ({} full [{}-bit], {} half [{}-bit]): ",
                    ivd.item_count,
                    ivd.word_count,
                    full_bits,
                    ivd.region_index_count - ivd.word_count,
                    full_bits >> 1
                ))
                .then(LineBreak.then(display_delta_sets(&ivd.delta_sets, conf))),
            )
    }

    /// Tokenizer for DeltaSets (inline).
    // STUB - scaffolding-only implementation
    fn display_delta_sets(_sets: &DeltaSets, _conf: &Config) -> TokenStream<'static> {
        // STUB - figure out what we actually want to show
        toks(format!("<display_delta_sets: incomplete>")).pre_indent(6)
    }

    fn display_variation_axes(
        per_region: &[RegionAxisCoordinates],
        conf: &Config,
    ) -> TokenStream<'static> {
        TokenStream::join_with(
            vec![
                arrayfmt::display_table_column_horiz(
                    " start |",
                    per_region,
                    |coords| toks(format!("{:.03}", coords.start_coord)),
                    conf.inline_bookend,
                    |n_skipped| toks(format!("..{n_skipped:02}..")),
                )
                .pre_indent(4),
                arrayfmt::display_table_column_horiz(
                    "  peak |",
                    per_region,
                    |coords| toks(format!("{:.03}", coords.peak_coord)),
                    conf.inline_bookend,
                    |n_skipped| toks(format!("..{n_skipped:02}..")),
                )
                .pre_indent(4),
                arrayfmt::display_table_column_horiz(
                    "   end |",
                    per_region,
                    |coords| toks(format!("{:.03}", coords.end_coord)),
                    conf.inline_bookend,
                    |n_skipped| toks(format!("..{n_skipped:02}..")),
                )
                .pre_indent(4),
            ],
            LineBreak,
        )
    }
}

/// Tokenizer for `BaseMetrics` (inline)
fn display_base_metrics(base: &Option<BaseMetrics>, _conf: &Config) -> TokenStream<'static> {
    if let Some(BaseMetrics {
        major_version,
        minor_version,
    }) = base
    {
        toks(format!(
            "BASE: version {}",
            format_version_major_minor(*major_version, *minor_version)
        ))
        // STUB - add more display functions and local calls (possibly gated by verbosity levels) as BaseMetrics gets more fields
    } else {
        TokenStream::empty()
    }
}

use layout::display_layout_metrics;
mod layout {
    use super::*;

    // TODO - convert to TokenStream producer
    pub(super) fn display_layout_metrics(
        layout: Option<&LayoutMetrics>,
        ctxt: Ctxt,
        conf: &Config,
    ) -> TokenStream<'static> {
        if let Some(LayoutMetrics {
            major_version,
            minor_version,
            script_list,
            feature_list,
            lookup_list,
            feature_variations: _feature_variations,
        }) = layout
        {
            let minimal = toks(format!(
                "{}: version {}",
                name_for_table_disc(ctxt.get_disc().expect("Ctxt missing TableDiscriminator")),
                format_version_major_minor(*major_version, *minor_version)
            ));
            if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
                minimal
                    .glue(LineBreak, display_script_list(script_list, conf))
                    .glue(LineBreak, display_feature_list(feature_list, conf))
                    .glue(LineBreak, display_lookup_list(lookup_list, ctxt, conf))
            } else {
                minimal
            }
        } else {
            TokenStream::empty()
        }
    }

    /// Returns the table-identifier associated with a Layout table discriminator.
    fn name_for_table_disc(disc: TableDiscriminator) -> &'static str {
        match disc {
            TableDiscriminator::Gpos => "GPOS",
            TableDiscriminator::Gsub => "GSUB",
        }
    }

    // TODO - convert to tokenstream producer
    fn display_script_list(script_list: &ScriptList, conf: &Config) -> TokenStream<'static> {
        if script_list.is_empty() {
            toks("ScriptList [empty]").pre_indent(2)
        } else {
            toks("ScriptList").pre_indent(2).glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    script_list,
                    |ix, item| {
                        let Some(ScriptTable {
                            default_lang_sys,
                            lang_sys_records,
                        }) = &item.script
                        else {
                            unreachable!("missing ScriptTable at index {ix} in ScriptList");
                        };

                        toks(format!("[{ix}]: {}", item.script_tag))
                            .pre_indent(4)
                            .glue(LineBreak, {
                                match default_lang_sys {
                                    None => display_lang_sys_records(lang_sys_records, conf),
                                    langsys @ Some(..) => toks("[Default LangSys]: ")
                                        .pre_indent(5)
                                        .chain(display_langsys(langsys, conf))
                                        .glue(
                                            LineBreak,
                                            display_lang_sys_records(lang_sys_records, conf),
                                        ),
                                }
                            })
                    },
                    conf.bookend_size,
                    |start, stop| toks(format!("skipping ScriptRecords {start}..{stop}")),
                ),
            )
        }
    }

    fn display_lang_sys_records(
        lang_sys_records: &[LangSysRecord],
        conf: &Config,
    ) -> TokenStream<'static> {
        let token_stream = if lang_sys_records.is_empty() {
            toks("LangSysRecords: <empty list>")
        } else {
            toks("LangSysRecords:").glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    lang_sys_records,
                    |ix, item| {
                        toks(format!("[{ix}]: {}; ", item.lang_sys_tag))
                            .chain(display_langsys(&item.lang_sys, conf))
                            .pre_indent(6)
                    },
                    conf.bookend_size,
                    |start, stop| {
                        toks(format!("(skipping LangSysRecords {start}..{stop})")).pre_indent(5)
                    },
                ),
            )
        };
        token_stream.pre_indent(5)
    }

    /// Tokenizer for (optional) LangSys (inline)
    ///
    /// # Panics
    ///
    /// Will panic if `lang_sys` is `None`.
    fn display_langsys(lang_sys: &Option<LangSys>, conf: &Config) -> TokenStream<'static> {
        let Some(LangSys {
            lookup_order_offset,
            required_feature_index,
            feature_indices,
        }) = lang_sys
        else {
            unreachable!("missing langsys");
        };
        debug_assert_eq!(*lookup_order_offset, 0);
        let header = match required_feature_index {
            0xFFFF => tok("feature-indices: ".to_string()),
            other => tok(format!("feature-indices (required: {other}): ")),
        };
        header.then(arrayfmt::display_items_inline(
            feature_indices,
            |ix: &u16| toks(format!("{ix}")),
            conf.inline_bookend,
            |num_skipped: usize| toks(format!("...({num_skipped} skipped)...")),
        ))
    }

    fn display_feature_list(feature_list: &FeatureList, conf: &Config) -> TokenStream<'static> {
        let stream = if feature_list.is_empty() {
            toks("FeatureList [empty]")
        } else {
            toks("FeatureList").glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    feature_list,
                    |ix,
                     FeatureRecord {
                         feature_tag,
                         feature,
                     }| {
                        tok(format!("[{ix}]: {feature_tag}"))
                            .then(display_feature_table(feature, conf))
                            .pre_indent(4)
                    },
                    conf.bookend_size,
                    |start, stop| {
                        toks(format!("(skipping FeatureIndices {start}..{stop})")).pre_indent(3)
                    },
                ),
            )
        };
        stream.pre_indent(2)
    }

    /// Tokenizer for FeatureTable (inline)
    fn display_feature_table(table: &FeatureTable, conf: &Config) -> TokenStream<'static> {
        let FeatureTable {
            feature_params,
            lookup_list_indices,
        } = table;

        let stream = arrayfmt::display_items_inline(
            lookup_list_indices,
            |index| toks(format!("{index}")),
            conf.inline_bookend,
            |num_skipped| toks(format!("...({num_skipped} skipped)...")),
        );
        match feature_params {
            0 => stream,
            offset => tok(format!("[parameters located at SoF+{offset}B]")).then(stream),
        }
    }

    /// Tokenizer for `LookupList` (multiline)
    fn display_lookup_list(
        lookup_list: &LookupList,
        ctxt: Ctxt,
        conf: &Config,
    ) -> TokenStream<'static> {
        toks("LookupList:")
            .glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    lookup_list,
                    move |ix, table| {
                        toks(format!("[{ix}]: "))
                            .chain(display_lookup_table(table, ctxt, conf))
                            .pre_indent(4)
                    },
                    conf.bookend_size,
                    |start, stop| {
                        toks(format!("(skipping LookupTables {start}..{stop})")).pre_indent(3)
                    },
                ),
            )
            .pre_indent(2)
    }

    /// Tokenizer for `LookupTable` (inline).
    fn display_lookup_table(
        table: &LookupTable,
        ctxt: Ctxt,
        conf: &Config,
    ) -> TokenStream<'static> {
        // NOTE - because we print the kind of the lookup here, we don't need to list it for every element
        // LINK[format-lookup-subtable] -  (see display_lookup_subtable below)
        let mut stream = tok(format!(
            "LookupTable: kind={}",
            name_for_lookup_type(ctxt, table.lookup_type),
        ))
        .then(display_lookup_flags(&table.lookup_flag));
        if let Some(filtering_set) = table.mark_filtering_set {
            stream = stream.chain(
                tok(format!(
                    ", markFilteringSet=GDEF->MarkGlyphSet[{filtering_set}]"
                ))
                .into(),
            );
        }
        stream.glue(
            tok(": "),
            arrayfmt::display_items_inline(
                &table.subtables,
                |subtable| display_lookup_subtable(subtable, false, conf),
                conf.inline_bookend,
                |n_skipped| toks(format!("...({n_skipped} skipped)...")),
            ),
        )
    }

    // ANCHOR[format-lookup-subtable]
    fn display_lookup_subtable(
        subtable: &LookupSubtable,
        show_lookup_type: bool,
        conf: &Config,
    ) -> TokenStream<'static> {
        let (label, contents) = match subtable {
            LookupSubtable::SinglePos(single_pos) => ("SinglePos", display_single_pos(single_pos)),
            LookupSubtable::PairPos(pair_pos) => ("PairPos", display_pair_pos(pair_pos)),
            LookupSubtable::CursivePos(cursive_pos) => {
                ("CursivePos", display_cursive_pos(cursive_pos))
            }
            LookupSubtable::MarkBasePos(mark_base_pos) => {
                ("MarkBasePos", display_mark_base_pos(mark_base_pos))
            }
            LookupSubtable::MarkLigPos(mark_lig_pos) => {
                ("MarkLigPos", display_mark_lig_pos(mark_lig_pos))
            }
            LookupSubtable::MarkMarkPos(mark_mark_pos) => {
                ("MarkMarkPos", display_mark_mark_pos(mark_mark_pos))
            }
            LookupSubtable::SingleSubst(single_subst) => {
                ("SingleSubst", display_single_subst(single_subst, conf))
            }
            LookupSubtable::MultipleSubst(multi_subst) => {
                ("MultipleSubst", display_multi_subst(multi_subst))
            }
            LookupSubtable::AlternateSubst(alt_subst) => {
                ("AlternateSubst", display_alt_subst(alt_subst))
            }
            LookupSubtable::LigatureSubst(lig_subst) => {
                ("LigatureSubst", display_ligature_subst(lig_subst))
            }
            LookupSubtable::ReverseChainSingleSubst(rcs_subst) => (
                "RevChainSingleSubst",
                display_reverse_chain_single_subst(rcs_subst),
            ),
            LookupSubtable::SequenceContext(seq_ctx) => {
                ("SeqCtx", display_sequence_context(seq_ctx))
            }
            LookupSubtable::ChainedSequenceContext(chain_ctx) => {
                let contents = display_chained_sequence_context(chain_ctx, conf);
                ("ChainSeqCtx", contents)
            }
        };
        let label = tok(label);
        // WIP
        if show_lookup_type {
            label.then(contents)
        } else {
            contents
        }
    }

    fn display_chained_rule<Sem: SemDisplay>(rule: &ChainedRule<Sem>) -> TokenStream<'static> {
        // FIXME[epic=magic-const]
        const THIS_BOOKEND: usize = 1;
        let backtrack = if rule.backtrack_sequence.is_empty() {
            TokenStream::empty()
        } else {
            toks(format!("(?<=#{})", rule.backtrack_sequence))
        };
        let input = if rule.input_sequence.is_empty() {
            toks("_")
        } else {
            toks(format!("#_.{}", rule.input_sequence))
        };
        let lookahead = if rule.lookahead_sequence.is_empty() {
            TokenStream::empty()
        } else {
            toks(format!("(?=#{})", rule.lookahead_sequence))
        };
        let seq_lookups = arrayfmt::display_items_inline(
            &rule.seq_lookup_records,
            display_sequence_lookup,
            THIS_BOOKEND,
            |n| toks(format!("(..{n}..)")),
        );
        backtrack
            .chain(input)
            .chain(lookahead)
            .glue(tok("=>"), seq_lookups)
    }

    fn display_chained_rule_set<Sem: SemDisplay>(
        glyph: u16,
        set: &ChainedRuleSet<Sem>,
    ) -> TokenStream<'static> {
        // FIXME[epic=magic-const]
        const THIS_BOOKEND: usize = 1;
        tok(format!("{{{}=>", format_glyphid_hex(glyph, true)))
            .then(arrayfmt::display_items_inline(
                set,
                display_chained_rule,
                THIS_BOOKEND,
                |n| toks(format!("...({n})...")),
            ))
            .chain(toks("}"))
    }

    fn display_chained_sequence_context(
        chain_ctx: &ChainedSequenceContext,
        _conf: &Config,
    ) -> TokenStream<'static> {
        // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
        // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
        // TODO - move into conf and potentially revise how bookending is determined (e.g. dynamic array, closure)
        const INLINE_INLINE_BOOKEND: usize = 1;

        match chain_ctx {
            ChainedSequenceContext::Format1(format1) => {
                let coverage = display_coverage_table_overview(&format1.coverage);
                let cov_iter = format1.coverage.iter();
                let rule_sets = arrayfmt::display_coverage_linked_array(
                    &format1.chained_seq_rule_sets,
                    cov_iter,
                    display_chained_rule_set,
                    INLINE_INLINE_BOOKEND,
                    |n| toks(format!("...(skipping {n} glyph-rule sets)...")),
                );
                // REVIEW - consider what other details (e.g. class-def summary metrics) to show in implicitly- or explicitly-verbose display format
                tok("Glyph").then(TokenStream::paren(coverage.glue(tok("=>"), rule_sets)))
            }
            ChainedSequenceContext::Format2(format2) => {
                let coverage = display_coverage_table_overview(&format2.coverage);
                // NOTE - because class-def display is multiline, omitting class-def from display
                let cov_iter = format2.coverage.iter();
                let rule_sets = arrayfmt::display_coverage_linked_array(
                    &format2.chained_class_seq_rule_sets,
                    cov_iter,
                    display_chained_rule_set,
                    INLINE_INLINE_BOOKEND,
                    |n| toks(format!("...(skipping {n} glyph-rule sets)...")),
                );
                // REVIEW - consider what other details (e.g. class-def summary metrics) to show in implicitly- or explicitly-verbose display format
                tok("Class").then(TokenStream::paren(coverage.glue(tok("=>"), rule_sets)))
            }
            ChainedSequenceContext::Format3(ChainedSequenceContextFormat3 {
                backtrack_coverages,
                input_coverages,
                lookahead_coverages,
                seq_lookup_records,
                ..
            }) => {
                let backtrack_pattern = if backtrack_coverages.is_empty() {
                    TokenStream::empty()
                } else {
                    arrayfmt::display_items_inline(
                        backtrack_coverages,
                        display_coverage_table_overview,
                        INLINE_INLINE_BOOKEND,
                        |n| toks(format!("(..{n}..)")),
                    )
                    .surround(tok("(?<="), tok(")"))
                };
                let input_pattern = arrayfmt::display_items_inline(
                    input_coverages,
                    display_coverage_table_overview,
                    INLINE_INLINE_BOOKEND,
                    |n| toks(format!("(..{n}..)")),
                );
                let lookahead_pattern = if lookahead_coverages.is_empty() {
                    TokenStream::empty()
                } else {
                    arrayfmt::display_items_inline(
                        lookahead_coverages,
                        display_coverage_table_overview,
                        INLINE_INLINE_BOOKEND,
                        |n| toks(format!("(..{n}..)")),
                    )
                    .surround(tok("(?="), tok(")"))
                };
                let seq_lookups = arrayfmt::display_items_inline(
                    seq_lookup_records,
                    display_sequence_lookup,
                    INLINE_INLINE_BOOKEND,
                    |n| toks(format!("(..{n}..)")),
                );
                backtrack_pattern
                    .chain(input_pattern)
                    .chain(lookahead_pattern)
                    .glue(tok("=>"), seq_lookups)
            }
        }
    }

    fn display_single_subst(single_subst: &SingleSubst, conf: &Config) -> TokenStream<'static> {
        match single_subst {
            SingleSubst::Format1(SingleSubstFormat1 {
                coverage,
                delta_glyph_id,
            }) => display_coverage_table_overview(coverage)
                .chain(toks(format!("=>({delta_glyph_id:+})"))),
            SingleSubst::Format2(SingleSubstFormat2 {
                coverage,
                substitute_glyph_ids,
            }) => {
                let iter = coverage.iter();
                display_coverage_table_overview(coverage).glue(
                    tok("=>"),
                    arrayfmt::display_coverage_linked_array(
                        substitute_glyph_ids,
                        iter,
                        |orig_glyph, subst_glyph| {
                            toks(format!(
                                "{}->{}",
                                format_glyphid_hex(orig_glyph, true),
                                format_glyphid_hex(*subst_glyph, true),
                            ))
                        },
                        conf.inline_bookend,
                        |n_skipped| toks(format!("...(skipping {n_skipped} substs)...")),
                    ),
                )
            }
        }
    }

    fn display_multi_subst(multi_subst: &MultipleSubst) -> TokenStream<'static> {
        display_coverage_table_overview(&multi_subst.coverage)
            // REVIEW - is this the right balance of specificity and brevity?
            .chain(toks(format!(
                "=>SequenceTable[{}]",
                multi_subst.subst.sequences.len()
            )))
    }

    fn display_alt_subst(alt_subst: &AlternateSubst) -> TokenStream<'static> {
        display_coverage_table_overview(&alt_subst.coverage)
            .glue(tok("=>"), display_alternate_sets(&alt_subst.alternate_sets))
    }

    fn display_ligature_subst(lig_subst: &LigatureSubst) -> TokenStream<'static> {
        display_ligature_sets(&lig_subst.ligature_sets, lig_subst.coverage.iter())
    }

    fn display_reverse_chain_single_subst(
        rcs_subst: &ReverseChainSingleSubst,
    ) -> TokenStream<'static> {
        // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
        // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
        const INLINE_INLINE_BOOKEND: usize = 1;

        let backtrack_pattern = if rcs_subst.backtrack_coverages.is_empty() {
            TokenStream::empty()
        } else {
            arrayfmt::display_items_inline(
                &rcs_subst.backtrack_coverages,
                display_coverage_table_overview,
                INLINE_INLINE_BOOKEND,
                |n| toks(format!("(..{n}..)")),
            )
            .surround(tok("(?<="), tok(")"))
        };
        let input_pattern = display_coverage_table_overview(&rcs_subst.coverage);
        let lookahead_pattern = if rcs_subst.lookahead_coverages.is_empty() {
            TokenStream::empty()
        } else {
            arrayfmt::display_items_inline(
                &rcs_subst.lookahead_coverages,
                display_coverage_table_overview,
                INLINE_INLINE_BOOKEND,
                |n| toks(format!("(..{n}..)")),
            )
            .surround(tok("(?="), tok(")"))
        };
        let substitute_ids = &rcs_subst.substitute_glyph_ids;

        backtrack_pattern
            .chain(input_pattern)
            .chain(lookahead_pattern)
            .chain(toks(format!("=>{substitute_ids}")))
    }

    fn display_sequence_context(seq_ctx: &SequenceContext) -> TokenStream<'static> {
        match seq_ctx {
            SequenceContext::Format1(SequenceContextFormat1 { coverage, .. }) => tok("Glyphs")
                .then(TokenStream::paren(display_coverage_table_overview(
                    coverage,
                ))),
            SequenceContext::Format2(SequenceContextFormat2 { coverage, .. }) => tok("Classes")
                .then(TokenStream::paren(display_coverage_table_overview(
                    coverage,
                ))),
            SequenceContext::Format3(SequenceContextFormat3 {
                coverage_tables,
                seq_lookup_records,
                ..
            }) => {
                // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                const INLINE_INLINE_BOOKEND: usize = 1;
                let input_pattern = arrayfmt::display_items_inline(
                    coverage_tables,
                    display_coverage_table_overview,
                    INLINE_INLINE_BOOKEND,
                    |n| toks(format!("(..{n}..)")),
                );
                let seq_lookups = arrayfmt::display_items_inline(
                    seq_lookup_records,
                    display_sequence_lookup,
                    INLINE_INLINE_BOOKEND,
                    |n| toks(format!("(..{n}..)")),
                );
                input_pattern.glue(tok("=>"), seq_lookups)
            }
        }
    }

    fn display_single_pos(single_pos: &SinglePos) -> TokenStream<'static> {
        match single_pos {
            SinglePos::Format1(SinglePosFormat1 { value_record, .. }) => {
                tok("single").then(TokenStream::paren(display_value_record(value_record)))
            }
            SinglePos::Format2(SinglePosFormat2 { coverage, .. }) => tok("array").then(
                TokenStream::paren(display_coverage_table_overview(coverage)),
            ),
        }
    }

    fn display_pair_pos(pair_pos: &PairPos) -> TokenStream<'static> {
        match pair_pos {
            PairPos::Format1(PairPosFormat1 { coverage, .. }) => {
                tok("byGlyph").then(display_coverage_table_overview(coverage).paren())
            }
            PairPos::Format2(PairPosFormat2 {
                coverage,
                class_def1,
                class_def2,
                class1_records,
            }) => {
                let rows = class1_records.rows();
                let cols = class1_records.width();

                class_def1.validate_class_count(rows);
                class_def2.validate_class_count(cols);

                // REVIEW - if not too verbose, we might want a compact overview of the Class1Record array, specifically which index-pairs constitute actual adjustments
                let populated_class_pairs: Vec<(usize, usize)> = {
                    Iterator::zip(0..rows, 0..cols)
                        .filter(|ixpair| {
                            let it = &class1_records[*ixpair];
                            it.value_record1.is_some() || it.value_record2.is_some()
                        })
                        .collect()
                };

                // TODO - should this be a more general parameter in the Config type?
                // maximum number of index-pairs we are willing to display inline (chosen arbitrarily)
                const MAX_POPULATION: usize = 3;

                if populated_class_pairs.len() <= MAX_POPULATION {
                    tok(format!("byClass{:?}", populated_class_pairs,)).then(
                        display_coverage_table_overview(coverage).surround(tok("("), tok(")")),
                    )
                } else {
                    tok(format!(
                        "byClass[{} ∈ {rows} x {cols}]",
                        populated_class_pairs.len(),
                    ))
                    .then(display_coverage_table_overview(coverage).paren())
                }
            }
        }
    }

    fn display_cursive_pos(cursive_pos: &CursivePos) -> TokenStream<'static> {
        tok("entryExit").then(TokenStream::paren(display_coverage_table_overview(
            &cursive_pos.coverage,
        )))
        // STUB - display EntryExit-record array
    }

    fn display_mark_base_pos(mb_pos: &MarkBasePos) -> TokenStream<'static> {
        let lhs = {
            let mark_lhs = tok("Mark").then(TokenStream::paren(display_coverage_table_overview(
                &mb_pos.mark_coverage,
            )));
            let base_lhs = tok("Base").then(TokenStream::paren(display_coverage_table_overview(
                &mb_pos.base_coverage,
            )));
            mark_lhs.glue(tok("+"), base_lhs)
        };

        let rhs = {
            let mut mark_iter = mb_pos.mark_coverage.iter();
            let mut base_iter = mb_pos.base_coverage.iter();

            let mark_rhs = tok("MarkArray").then(TokenStream::bracket(display_mark_array(
                &mb_pos.mark_array,
                &mut mark_iter,
            )));
            let base_rhs = tok("BaseArray").then(TokenStream::bracket(display_base_array(
                &mb_pos.base_array,
                &mut base_iter,
            )));
            mark_rhs.glue(tok("+"), base_rhs)
        };

        lhs.glue(tok("=>"), rhs)
    }

    fn display_mark_lig_pos(ml_pos: &MarkLigPos) -> TokenStream<'static> {
        let lhs = {
            let mark_lhs = tok("Mark").then(TokenStream::paren(display_coverage_table_overview(
                &ml_pos.mark_coverage,
            )));
            let lig_lhs = tok("Ligature").then(TokenStream::paren(
                display_coverage_table_overview(&ml_pos.ligature_coverage),
            ));
            mark_lhs.glue(tok("+"), lig_lhs)
        };

        let rhs = {
            let mut mark_iter = ml_pos.mark_coverage.iter();
            let mut ligature_iter = ml_pos.ligature_coverage.iter();

            let mark_rhs = tok("MarkArray").then(TokenStream::bracket(display_mark_array(
                &ml_pos.mark_array,
                &mut mark_iter,
            )));
            let lig_rhs = tok("LigatureArray").then(TokenStream::bracket(display_ligature_array(
                &ml_pos.ligature_array,
                &mut ligature_iter,
            )));
            mark_rhs.glue(tok("+"), lig_rhs)
        };

        lhs.glue(tok("=>"), rhs)
    }

    fn display_mark_mark_pos(mm_pos: &MarkMarkPos) -> TokenStream<'static> {
        let lhs = {
            let mark_lhs = tok("Mark").then(TokenStream::paren(display_coverage_table_overview(
                &mm_pos.mark1_coverage,
            )));
            let mark2_lhs = tok("Mark").then(TokenStream::paren(display_coverage_table_overview(
                &mm_pos.mark2_coverage,
            )));
            mark_lhs.glue(tok("+"), mark2_lhs)
        };

        let rhs = {
            let mut mark1_iter = mm_pos.mark1_coverage.iter();
            let mut mark2_iter = mm_pos.mark2_coverage.iter();

            let mark_rhs = tok("MarkArray").then(TokenStream::bracket(display_mark_array(
                &mm_pos.mark1_array,
                &mut mark1_iter,
            )));
            let mark2_rhs = tok("Mark2Array").then(TokenStream::bracket(display_mark2_array(
                &mm_pos.mark2_array,
                &mut mark2_iter,
            )));
            mark_rhs.glue(tok("+"), mark2_rhs)
        };
        lhs.glue(tok("=>"), rhs)
    }

    fn display_ligature_sets(
        lig_sets: &[LigatureSet],
        mut coverage: impl Iterator<Item = u16>,
    ) -> TokenStream<'static> {
        match lig_sets {
            [set] => display_ligature_set(set, coverage.next().expect("missing coverage")),
            more => {
                // FIXME[epic=magic] - arbitrary local bookending const
                const LIG_SET_BOOKEND: usize = 1;

                arrayfmt::display_coverage_linked_array(
                    more,
                    coverage,
                    |cov, lig_set| display_ligature_set(lig_set, cov),
                    LIG_SET_BOOKEND,
                    |_| toks(".."),
                )
            }
        }
    }
    fn display_ligature_set(lig_set: &LigatureSet, cov: u16) -> TokenStream<'static> {
        // FIXME[epic=magic] - arbitrary local bookending const
        const LIG_BOOKEND: usize = 2;

        arrayfmt::display_items_inline(
            &lig_set.ligatures,
            |lig| display_ligature(lig, cov),
            LIG_BOOKEND,
            |n_skipped| toks(format!("...({n_skipped} skipped)...")),
        )
    }

    fn display_ligature(lig: &Ligature, cov: u16) -> TokenStream<'static> {
        toks(format!(
            "(#{cov:04x}.{} => {})",
            &lig.component_glyph_ids, lig.ligature_glyph,
        ))
    }

    fn display_alternate_sets(alt_sets: &[AlternateSet]) -> TokenStream<'static> {
        debug_assert!(
            !alt_sets.is_empty(),
            "unexpected empty AlternateSet-array in display_alternate_sets"
        );
        match alt_sets {
            [set] => display_alternate_set(set),
            more => {
                const ALT_SET_BOOKEND: usize = 1;
                arrayfmt::display_items_inline(
                    more,
                    display_alternate_set,
                    ALT_SET_BOOKEND,
                    |count| toks(format!("...({count} skipped)...")),
                )
            }
        }
    }

    /// Tokenizer for `AlternateSubst->AlternateSet` (inline)
    fn display_alternate_set(alt_set: &AlternateSet) -> TokenStream<'static> {
        const ALT_GLYPH_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &alt_set.alternate_glyph_ids,
            |glyph_id| toks(format_glyphid_hex(*glyph_id, true)),
            ALT_GLYPH_BOOKEND,
            |_| toks("..".to_string()),
        )
    }

    fn display_sequence_lookup(sl: &SequenceLookup) -> TokenStream<'static> {
        let s_ix = sl.sequence_index;
        let ll_ix = sl.lookup_list_index;
        // NOTE - the number in `\[_\]` is meant to mimic the index display of the display_items_elided formatting of LookupList, so it is the lookup index. The number after `@` is the positional index to apply the lookup to
        toks(format!("[{ll_ix}]@{s_ix}"))
    }

    fn display_mark2_array(
        arr: &Mark2Array,
        coverage: &mut impl Iterator<Item = u16>,
    ) -> TokenStream<'static> {
        fn display_mark2_record(mark2_record: &Mark2Record, cov: u16) -> TokenStream<'static> {
            const CLASS_ANCHORS: usize = 2;

            tok(format!("{}: ", format_glyphid_hex(cov, true),)).then(
                arrayfmt::display_inline_nullable(
                    &mark2_record.mark2_anchors,
                    |ix, anchor| tok(format!("[{ix}]=>")).then(display_anchor_table(anchor)),
                    CLASS_ANCHORS,
                    |n, (start, end)| {
                        toks(format!(
                            "...(skipping {n} indices spanning {start}..={end})..."
                        ))
                    },
                ),
            )
        }

        const MARK2_ARRAY_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &arr.mark2_records,
            |mark2_record| {
                display_mark2_record(mark2_record, coverage.next().expect("missing coverage"))
            },
            MARK2_ARRAY_BOOKEND,
            |n| toks(format!("...(skipping {n} Mark2Records)...")),
        )
    }

    fn display_ligature_array(
        ligature_array: &LigatureArray,
        coverage: &mut impl Iterator<Item = u16>,
    ) -> TokenStream<'static> {
        fn display_ligature_attach(
            ligature_attach: &LigatureAttach,
            cov: u16,
        ) -> TokenStream<'static> {
            fn display_component_record(
                component_record: &ComponentRecord,
            ) -> TokenStream<'static> {
                const CLASS_ANCHOR_BOOKEND: usize = 2;
                arrayfmt::display_inline_nullable(
                    &component_record.ligature_anchors,
                    |ix, anchor| tok(format!("[{ix}]=>")).then(display_anchor_table(anchor)),
                    CLASS_ANCHOR_BOOKEND,
                    |n_skipped, (first, last)| {
                        toks(format!(
                            "...(skipping {n_skipped} indices from {first} to {last})..."
                        ))
                    },
                )
            }

            const COMPONENTS_BOOKEND: usize = 1;
            tok(format!("{cov:04x}=")).then(arrayfmt::display_items_inline(
                &ligature_attach.component_records,
                display_component_record,
                COMPONENTS_BOOKEND,
                |_| toks(".."),
            ))
        }

        const ATTACHES_INLINE: usize = 2;
        arrayfmt::display_items_inline(
            &ligature_array.ligature_attach,
            |attach| display_ligature_attach(attach, coverage.next().expect("missing coverage")),
            ATTACHES_INLINE,
            |n_skipped| toks(format!("...(skipping {n_skipped})...")),
        )
    }

    fn display_base_array(
        base_array: &BaseArray,
        coverage: &mut impl Iterator<Item = u16>,
    ) -> TokenStream<'static> {
        fn display_base_record(base_record: &BaseRecord, cov: u16) -> TokenStream<'static> {
            const CLASS_ANCHOR_BOOKEND: usize = 2;
            tok(format!("{cov:04x}: ")).then(arrayfmt::display_inline_nullable(
                &base_record.base_anchors,
                |ix, anchor| tok(format!("[{ix}]=>")).then(display_anchor_table(anchor)),
                CLASS_ANCHOR_BOOKEND,
                |n_skipped, (first, last)| {
                    toks(format!(
                        "...(skipping {n_skipped} indices from {first} to {last})..."
                    ))
                },
            ))
        }

        use toks;
        const BASE_ARRAY_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &base_array.base_records,
            |base_record| {
                display_base_record(base_record, coverage.next().expect("missing coverage"))
            },
            BASE_ARRAY_BOOKEND,
            |n_skipped| toks(format!("...({n_skipped} skipped)...")),
        )
    }

    fn display_mark_array(
        mark_array: &MarkArray,
        coverage: &mut impl Iterator<Item = u16>,
    ) -> TokenStream<'static> {
        fn display_mark_record(mark_record: &MarkRecord, cov: u16) -> TokenStream<'static> {
            tok(format!("{cov:04x}=({}, ", mark_record.mark_class,)).then(
                display_anchor_table(mark_record.mark_anchor.as_ref().expect("broken link"))
                    .chain(toks(")")),
            )
        }

        // FIXME[magic] - arbitrary local bookending const
        const MARK_ARRAY_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &mark_array.mark_records,
            |mark_record| {
                display_mark_record(mark_record, coverage.next().expect("missing coverage"))
            },
            MARK_ARRAY_BOOKEND,
            |n_skipped| toks(format!("...({n_skipped} skipped)...")),
        )
    }

    fn display_anchor_table(anchor: &AnchorTable) -> TokenStream<'static> {
        match anchor {
            AnchorTable::Format1(AnchorTableFormat1 {
                x_coordinate,
                y_coordinate,
            }) => toks(format!(
                "({}, {})",
                as_s16(*x_coordinate),
                as_s16(*y_coordinate)
            )),
            AnchorTable::Format2(f2) => toks(format!(
                "({}, {})@[{}]",
                as_s16(f2.x_coordinate),
                as_s16(f2.y_coordinate),
                f2.anchor_point
            )),
            AnchorTable::Format3(AnchorTableFormat3 {
                x_coordinate,
                y_coordinate,
                x_device,
                y_device,
            }) => {
                let extra = {
                    let tokenize =
                        |opt_table: &Option<DeviceOrVariationIndexTable>| -> TokenStream<'static> {
                            opt_table
                                .as_ref()
                                .map_or(toks("ⅈ"), display_device_or_variation_index_table)
                        };
                    debug_assert!(
                        x_device.is_some() || y_device.is_some(),
                        "unexpected both-Null DeviceOrVariationIndexTable offsets in AnchorTable::Format3"
                    );
                    let x_tokens = tokenize(x_device);
                    let y_tokens = tokenize(y_device);
                    tok("×").then(TokenStream::paren(TokenStream::glue(
                        x_tokens,
                        tok(", "),
                        y_tokens,
                    )))
                };
                tok(format!(
                    "({}, {})",
                    as_s16(*x_coordinate),
                    as_s16(*y_coordinate),
                ))
                .then(extra)
            }
        }
    }

    /// Raw tokenizer for displaying a `LookupFlag` value (inline).
    fn display_lookup_flag(flags: &LookupFlag) -> TokenStream<'static> {
        let mut set_flags = Vec::new();
        if flags.right_to_left {
            set_flags.push(toks("RIGHT_TO_LEFT"));
        }
        if flags.ignore_base_glyphs {
            set_flags.push(toks("IGNORE_BASE_GLYPHS"));
        }
        if flags.ignore_ligatures {
            set_flags.push(toks("IGNORE_LIGATURES"));
        }
        if flags.ignore_marks {
            set_flags.push(toks("IGNORE_MARKS"));
        }
        if flags.use_mark_filtering_set {
            set_flags.push(toks("USE_MARK_FILTERING_SET"));
        }

        let toks_flags = if set_flags.is_empty() {
            toks("∅")
        } else {
            TokenStream::join_with(set_flags, tok(" | "))
        };

        let toks_macf = match flags.mark_attachment_class_filter {
            0 => TokenStream::empty(),
            // REVIEW - if horizontal space is at a premium, we may want to shorten or partially elide the label-string
            n => toks(format!("; mark_attachment_class_filter = {n}")),
        };

        tok("LookupFlag ").then(TokenStream::chain(toks_flags, toks_macf).paren())
    }

    /// Smart tokenizer for pretty-printing `LookupFlag` values.
    ///
    /// When the `LookupFlag` value contains no significant information (MACF=0, all boolean flags unset), the
    /// resultant TokenStream will be empty.
    ///
    /// Otherwise, includes a prefix that separates the displayed content from the previous field and indicates that
    /// a flag-value is being displayed.
    fn display_lookup_flags(flags: &LookupFlag) -> TokenStream<'static> {
        if flags.mark_attachment_class_filter != 0
            || flags.right_to_left
            || flags.ignore_ligatures
            || flags.ignore_base_glyphs
            || flags.ignore_marks
            || flags.use_mark_filtering_set
        {
            tok(", flags=").then(display_lookup_flag(flags))
        } else {
            TokenStream::empty()
        }
    }

    fn name_for_lookup_type(ctxt: Ctxt, ltype: u16) -> &'static str {
        match ctxt.get_disc() {
            None => unreachable!("format_lookup_kind called with neutral (whence := None) Ctxt"),
            Some(TableDiscriminator::Gpos) => match ltype {
                1 => "SinglePos",
                2 => "PairPos",
                3 => "CursivePos",
                4 => "MarkBasePos",
                5 => "MarkLigPos",
                6 => "MarkMarkPos",
                7 => "SequenceContext",
                8 => "ChainedSequenceContext",
                9 => "PosExt",
                _ => unreachable!("unexpected GPOS lookup-type {ltype} (expected 1..=9)"),
            },
            Some(TableDiscriminator::Gsub) => match ltype {
                1 => "SingleSubst",
                2 => "MultipleSubst",
                3 => "AlternateSubst",
                4 => "LigatureSubst",
                5 => "SequenceContext",
                6 => "ChainedSequenceContext",
                7 => "SubstExt",
                8 => "ReverseChainedSingleSubst",
                _ => unreachable!("unexpected GSUB lookup-type {ltype} (expected 1..=8)"),
            },
        }
    }

    /// Helper for displaying independently-optional x-value y-value pairs as compactly as possible.
    ///
    /// If both are `None`, returns `None`.
    /// Otherwise, returns an unambiguous display of all non-None values.
    fn display_opt_xy<T>(what: &str, x: Option<T>, y: Option<T>) -> Option<TokenStream<'static>>
    where
        T: std::fmt::Display,
    {
        use toks;
        match (x, y) {
            (None, None) => None,
            (Some(x), Some(y)) => Some(toks(format!("{what}: ({x},{y})"))),
            (Some(x), None) => Some(toks(format!("{what}[x]: {x}"))),
            (None, Some(y)) => Some(toks(format!("{what}[y]: {y}"))),
        }
    }

    /// Tokenizer for ValueRecord (inline).
    fn display_value_record(record: &ValueRecord) -> TokenStream<'static> {
        let ValueRecord {
            x_placement,
            y_placement,
            x_advance,
            y_advance,
            x_placement_device,
            y_placement_device,
            x_advance_device,
            y_advance_device,
        } = record;

        const NUM_FRAGMENTS: usize = 4;
        let mut buf = Vec::with_capacity(NUM_FRAGMENTS);

        // helper for indicating a field is present without attempting to display its value
        let elide =
            |opt_val: &Option<_>| -> Option<&'static str> { opt_val.as_ref().map(|_| "(..)") };

        buf.extend(display_opt_xy("placement", *x_placement, *y_placement));
        buf.extend(display_opt_xy("advance", *x_advance, *y_advance));
        buf.extend(display_opt_xy(
            "placement(device)",
            elide(x_placement_device),
            elide(y_placement_device),
        ));
        buf.extend(display_opt_xy(
            "advance(device)",
            elide(x_advance_device),
            elide(y_advance_device),
        ));

        if buf.is_empty() {
            // REVIEW - determine whether this case will ever happen in practice
            toks("<Empty ValueRecord>")
        } else {
            TokenStream::join_with(buf, tok("; "))
        }
    }
}

use stat::show_stat_metrics;
mod stat {
    use super::*;

    // FIXME - refactor into pure TokenStream
    pub(crate) fn show_stat_metrics(stat: Option<&StatMetrics>, conf: &Config) {
        if let Some(stat) = stat {
            println!(
                "STAT: version {} (elidedFallbackName: name[id={}])",
                format_version_major_minor(stat.major_version, stat.minor_version),
                stat.elided_fallback_name_id.0
            );
            if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
                match stat.design_axes.len() {
                    0 => (),
                    n => {
                        // FIXME - promote to first-class tokenstream
                        println!("\tDesignAxes: {n} total");
                        arrayfmt::display_items_elided(
                            &stat.design_axes,
                            |ix, d_axis| {
                                tok(format!("\t\t[{ix}]: ")).then(display_design_axis(d_axis, conf))
                            },
                            conf.bookend_size,
                            |start, stop| {
                                toks(format!(
                                    "\t(skipping design-axes from index {start}..{stop})"
                                ))
                            },
                        )
                        .println()
                    }
                }
                match stat.axis_values.len() {
                    0 => (),
                    n => {
                        println!("\tAxisValues: {n} total");
                        arrayfmt::display_items_elided(
                            &stat.axis_values,
                            |ix, a_value| {
                                tok(format!("\t\t[{ix}]: ")).then(display_axis_value(a_value, conf))
                            },
                            conf.bookend_size,
                            |start, stop| {
                                toks(format!(
                                    "\t(skipping design-axes from index {start}..{stop})"
                                ))
                            },
                        )
                        .println()
                    }
                }
            }
        }
    }

    fn display_design_axis(axis: &DesignAxis, _conf: &Config) -> TokenStream<'static> {
        toks(format!(
            "Tag={} ; Axis NameID={} ; Ordering={}",
            axis.axis_tag, axis.axis_name_id.0, axis.axis_ordering
        ))
    }

    fn format_axis_value_flags(flags: &AxisValueFlags) -> String {
        let mut set_flags = Vec::new();
        if flags.elidable_axis_value_name {
            // REVIEW - is there a pithier, but not obfuscating, string we can use instead?
            set_flags.push("ELIDABLE_AXIS_VALUE_NAME");
        }
        if flags.older_sibling_font_attribute {
            // REVIEW - is there a pithier, but not obfuscating, string we can use instead?
            set_flags.push("OLDER_SIBLING_FONT_ATTRIBUTE");
        }
        if set_flags.is_empty() {
            String::new()
        } else {
            format!(" (Flags: {})", set_flags.join(" | "))
        }
    }

    fn display_axis_value(value: &AxisValue, conf: &Config) -> TokenStream<'static> {
        match value {
            AxisValue::Format1(AxisValueFormat1 {
                axis_index,
                flags,
                value_name_id,
                value,
            }) => toks(format!(
                "Axis[{}]{}: {:?} = {}",
                axis_index,
                format_axis_value_flags(flags),
                value_name_id,
                value
            )),
            AxisValue::Format2(AxisValueFormat2 {
                axis_index,
                flags,
                value_name_id,
                nominal_value,
                range_min_value,
                range_max_value,
            }) => toks(format!(
                "Axis[{}]{}: {:?} = {} ∈ [{}, {}]",
                axis_index,
                format_axis_value_flags(flags),
                value_name_id,
                nominal_value,
                range_min_value,
                range_max_value
            )),
            AxisValue::Format3(AxisValueFormat3 {
                axis_index,
                flags,
                value_name_id,
                value,
                linked_value,
            }) => toks(format!(
                "Axis[{}]{}: {:?} = {} (-> {})",
                axis_index,
                format_axis_value_flags(flags),
                value_name_id,
                value,
                linked_value
            )),
            AxisValue::Format4(AxisValueFormat4 {
                flags,
                value_name_id,
                axis_values,
            }) => tok(format!(
                "{:?}{}: ",
                value_name_id,
                format_axis_value_flags(flags),
            ))
            .then(arrayfmt::display_items_inline(
                axis_values,
                |axis_value| {
                    toks(format!(
                        "Axis[{}] = {}",
                        axis_value.axis_index, axis_value.value
                    ))
                },
                conf.inline_bookend,
                |n_skipped| toks(format!("...(skipping {n_skipped} AxisValue records)...")),
            )),
        }
    }
}

use kern::show_kern_metrics;
mod kern {
    use super::*;
    pub(super) fn show_kern_metrics(kern: &Option<KernMetrics>, conf: &Config) {
        fn display_kern_subtable(subtable: &KernSubtable, conf: &Config) -> TokenStream<'static> {
            fn format_kern_flags(flags: KernFlags) -> String {
                let mut params = Vec::new();
                if flags.r#override {
                    params.push("override");
                }
                if flags.cross_stream {
                    params.push("x-stream")
                }
                if flags.minimum {
                    params.push("min")
                } else {
                    params.push("kern")
                }
                if flags.horizontal {
                    params.push("h")
                } else {
                    params.push("v")
                }

                params.join(" | ")
            }

            fn display_kern_subtable_data(
                subtable_data: &KernSubtableData,
                conf: &Config,
            ) -> TokenStream<'static> {
                match subtable_data {
                    KernSubtableData::Format0(KernSubtableFormat0 { kern_pairs }) => {
                        arrayfmt::display_items_inline(
                            kern_pairs,
                            |kern_pair| {
                                toks(format!(
                                    "({},{}) {:+}",
                                    format_glyphid_hex(kern_pair.left, true),
                                    format_glyphid_hex(kern_pair.right, true),
                                    kern_pair.value
                                ))
                            },
                            conf.inline_bookend,
                            |n| toks(format!("(..{n}..)")),
                        )
                    }
                    KernSubtableData::Format2(KernSubtableFormat2 {
                        left_class,
                        right_class,
                        kerning_array,
                    }) => {
                        fn display_kern_class_table(
                            table: &KernClassTable,
                            conf: &Config,
                        ) -> TokenStream<'static> {
                            tok(format!(
                                "Classes[first={}, nGlyphs={}]: ",
                                format_glyphid_hex(table.first_glyph, true),
                                table.n_glyphs,
                            ))
                            .then(arrayfmt::display_items_inline(
                                &table.class_values,
                                |id| toks(u16::to_string(id)),
                                conf.inline_bookend,
                                |n| toks(format!("(..{n}..)")),
                            ))
                        }
                        fn display_kerning_array(
                            array: &KerningArray,
                            conf: &Config,
                        ) -> TokenStream<'static> {
                            arrayfmt::display_wec_rows_elided(
                                &array.0,
                                |ix, row| {
                                    tok(format!("\t\t[{ix}]: ")).then(
                                        arrayfmt::display_items_inline(
                                            row,
                                            |kern_val| toks(format!("{kern_val:+}")),
                                            conf.inline_bookend,
                                            |n| toks(format!("(..{n}..)")),
                                        ),
                                    )
                                },
                                conf.bookend_size / 2, // FIXME - magic constant adjustment
                                |start, stop| {
                                    toks(format!(
                                        "\t\t(skipping kerning array rows {start}..{stop})"
                                    ))
                                },
                            )
                        }
                        let left =
                            tok("LeftClass=").then(display_kern_class_table(left_class, conf));

                        let right =
                            tok("RightClass=").then(display_kern_class_table(right_class, conf));
                        let kern =
                            tok("KerningArray:").then(display_kerning_array(kerning_array, conf));
                        left.glue(tok("\t"), right).glue(tok("\t"), kern)
                    }
                }
            }

            tok(format!(
                "KernSubtable ({}): ",
                format_kern_flags(subtable.flags)
            ))
            .then(display_kern_subtable_data(&subtable.data, conf))
        }

        if let Some(kern) = kern {
            if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
                println!("kern");
                arrayfmt::display_items_elided(
                    &kern.subtables,
                    |ix, subtable| {
                        toks(format!("[{ix}]: "))
                            .pre_indent(2)
                            .chain(display_kern_subtable(subtable, conf))
                    },
                    conf.bookend_size,
                    |start, stop| {
                        toks(format!("(skipping kern subtables {start}..{stop})")).pre_indent(1)
                    },
                )
                .println();
            } else {
                println!("kern: {} kerning subtables", kern.subtables.len());
            }
        }
    }
}

/// Tokenizer for ClassDef tables (multi-line)
fn display_class_def(
    class_def: &ClassDef,
    show_fn: impl Fn(&u16) -> TokenStream<'static>,
    conf: &Config,
) -> TokenStream<'static> {
    match *class_def {
        ClassDef::Format1 {
            start_glyph_id,
            ref class_value_array,
        } => {
            let toks_init = match start_glyph_id {
                0 => TokenStream::empty(),
                1 => toks("(skipping uncovered glyph 0)").pre_indent(3),
                n => toks(format!("(skipping uncovered glyphs 0..{n})")).pre_indent(3),
            };
            let toks_array = arrayfmt::display_items_elided(
                class_value_array,
                |ix, item| {
                    let gix = start_glyph_id as usize + ix;
                    tok(format!("Glyph {}: ", format_glyphid_hex(gix as u16, false)))
                        .then(show_fn(item))
                        .pre_indent(4)
                },
                conf.bookend_size,
                |start, stop| {
                    toks(format!(
                        "(skipping glyphs {}..{})",
                        format_glyphid_hex(start_glyph_id + start as u16, false),
                        format_glyphid_hex(start_glyph_id + stop as u16, false),
                    ))
                    .pre_indent(3)
                },
            );
            // WIP
            TokenStream::join_with(vec![toks_init, toks_array], LineBreak)
        }
        ClassDef::Format2 {
            ref class_range_records,
        } => arrayfmt::display_items_elided(
            class_range_records,
            |_, class_range| {
                tok(format!(
                    "({} -> {}): ",
                    format_glyphid_hex(class_range.start_glyph_id, false),
                    format_glyphid_hex(class_range.end_glyph_id, false),
                ))
                .then(show_fn(&class_range.value))
                .pre_indent(4)
            },
            conf.bookend_size,
            |start, stop| {
                let low_end = class_range_records[start].start_glyph_id;
                let high_end = class_range_records[stop - 1].end_glyph_id;
                toks(format!(
                    "(skipping ranges covering glyphs {low_end}..={high_end})",
                ))
                .pre_indent(3)
            },
        ),
    }
}

/// Tokenizer for `DeviceOrVariationIndexTable` (inline)
fn display_device_or_variation_index_table(
    table: &DeviceOrVariationIndexTable,
) -> TokenStream<'static> {
    match table {
        DeviceOrVariationIndexTable::Device(dev_table) => display_device_table(dev_table),
        DeviceOrVariationIndexTable::VariationIndex(var_ix_table) => {
            display_variation_index_table(var_ix_table)
        }
        DeviceOrVariationIndexTable::NonStandard { delta_format } => {
            toks(format!("[<DeltaFormat {delta_format}>]"))
        }
    }
}

/// Tokenizer for `DeviceTable` (inline)
fn display_device_table(dev_table: &DeviceTable) -> TokenStream<'static> {
    // NOTE - in the callstacks where this function is called, horizontal space economy is at an ultra-premium so we don't show the actual deltas in the current implementation
    toks(format!("{}..{}", dev_table.start_size, dev_table.end_size))
}

/// Tokenizer for `VariationIndexTable` (inline)
fn display_variation_index_table(var_ix_table: &VariationIndexTable) -> TokenStream<'static> {
    toks(format!(
        "{}->{}",
        var_ix_table.delta_set_outer_index, var_ix_table.delta_set_inner_index
    ))
}

/// Formats a glyphId as a 4-digit hexadecimal string, optionally prefixed with `#` to indicate that the string represents a glyphId.
fn format_glyphid_hex(glyph: u16, include_prefix: bool) -> String {
    if include_prefix {
        format!("#{glyph:04x}")
    } else {
        format!("{glyph:04x}")
    }
}

/// Compact inline display of an array representing a sequence (rather than a set) of glyphIds
///
/// If `include_prefix` is true, the resultant string will be prefixed with `#` to mark the array as being a sequence of glyphIds.
/// Otherwise, does not include any special prefixes.
///
// REVIEW - we have no cap on how long a glyphId sequence we are willing to show unabridged and we might want one in theory
fn format_glyphid_array_hex(glyphs: &impl AsRef<[u16]>, include_prefix: bool) -> String {
    let glyph_array = glyphs.as_ref();

    const BYTES_PER_GLYPH: usize = 2;

    // Display prefix and associated overhead in bytes
    const PREFIX: &str = "#";
    const BYTE_OVERHEAD_PREFIX: usize = PREFIX.len();

    // how many extra String-bytes we use per glyph, not counting the glyph itself
    const GLUE: &str = ".";
    const BYTE_OVERHEAD_PER_GLYPH: usize = GLUE.len();

    // If the number of GLUE-strings we need is less than N (viz. the number of glyphs), this is the difference between N and the actual number we use
    const PER_GLYPH_OVERCOUNT: usize = 1;

    if glyph_array.is_empty() {
        // Short-circuit for empty-glyph array
        return String::from("ε");
    }
    let nglyphs = glyph_array.len();

    // We would use saturating-sub instead of raw `-` on nglyphs and PER_GLYPH_OVERCOUNT but nglyphs is not zero if we are here.
    let nbytes = (if include_prefix {
        BYTE_OVERHEAD_PREFIX
    } else {
        0
    }) + (nglyphs * BYTES_PER_GLYPH)
        + (BYTE_OVERHEAD_PER_GLYPH * (nglyphs - PER_GLYPH_OVERCOUNT));

    // Initialize a buffer with enough capacity it ought not need to reallocate or grow
    let mut buffer = String::with_capacity(nbytes);

    // Fill the buffer
    if include_prefix {
        buffer.push_str(PREFIX);
    }

    for (ix, glyph) in glyph_array.iter().enumerate() {
        if ix > 0 {
            buffer.push_str(GLUE);
        }
        // REVIEW - do we want to eliminate zero-padding for compactness, or keep it for consistency/legibility?
        buffer.push_str(&format_glyphid_hex(*glyph, false));
    }
    buffer
}

/// Minimalistic tokenizer for `CoverageTable` (inline).
///
/// In contrast to [`display_coverage_table_full`], which displays at least the full details of the leading and trailing `N`
/// glyphs/glyph-ranges covered where `N` is the Config-specified bookending value, this function produces a far more
/// terse display suitable for contexts where space is at an absolute premium and only a very high-level overview of the coverage is desired.
///
/// # Notes
///
/// In the rare case of coverage tables with no more than two glyphs/glyph-ranges provided, the display is lossless and shows the actual covered glyphs/glyph-ranges.
///
/// In all other cases, the display only shows the total number of covered glyphs (and, for Format2, glyph-ranges)
/// and the overall span of glyphIds covered (from the first covered glyphId to the last covered glyphId, inclusive).
fn display_coverage_table_overview(cov: &CoverageTable) -> TokenStream<'static> {
    match cov {
        CoverageTable::Format1 { glyph_array } => {
            let num_glyphs = glyph_array.len();
            match glyph_array.as_slice() {
                [] => toks("∅"),
                [id] => toks(format!("[{id}]")),
                [first, .., last] => toks(format!("[{num_glyphs} ∈ [{first},{last}]]")),
            }
        }
        CoverageTable::Format2 { range_records } => match range_records.as_slice() {
            [] => toks("∅"),
            [rr] => toks(format!("[∀: {}..={}]", rr.start_glyph_id, rr.end_glyph_id)),
            [first, .., last] => {
                let num_glyphs: u16 = range_records
                    .iter()
                    .map(|rr| rr.end_glyph_id - rr.start_glyph_id + 1)
                    .sum();
                let num_ranges = range_records.len();
                let min_glyph = first.start_glyph_id;
                let max_glyph = last.end_glyph_id;
                toks(format!(
                    "[{num_ranges} ranges; {num_glyphs} ∈ [{min_glyph},{max_glyph}]]"
                ))
            }
        },
    }
}

/// Verbose tokenizer for `CoverageTable` (inline).
///
/// In contrast to [`display_coverage_table_overview`], which only outputs the number of covered glyphs and their overall range, this function outputs the actual covered glyphs/glyph-ranges, albeit with elision if the coverage is large enough to warrant it.
///
/// # Notes
/// The number of glyphs or glyph-ranges explicitly shown at the beginning and end of the coverage is determined by the Config-specified bookending value (`inline_bookend`).
///
/// As the output of this function will almost always be more verbose than that of `display_coverage_table_overview`, it is suitable only for contexts where space is less constrained and a more detailed picture of the coverage is preferred.
fn display_coverage_table_full(cov: &CoverageTable, conf: &Config) -> TokenStream<'static> {
    match cov {
        CoverageTable::Format1 { glyph_array } => {
            tok("Glyphs Covered: ").then(arrayfmt::display_items_inline(
                glyph_array,
                |item| toks(format_glyphid_hex(*item, false)),
                conf.inline_bookend,
                |num_skipped| toks(format!("...({num_skipped})...")),
            ))
        }
        CoverageTable::Format2 { range_records } => {
            tok("Glyph Ranges Covered: ").then(arrayfmt::display_items_inline(
                range_records,
                display_coverage_range_record,
                conf.inline_bookend,
                |num_skipped| toks(format!("...({num_skipped})...")),
            ))
        }
    }
}

fn display_coverage_range_record(coverage_range: &CoverageRangeRecord) -> TokenStream<'static> {
    let span = coverage_range.end_glyph_id - coverage_range.start_glyph_id;
    let end_coverage_index = coverage_range.value + span;
    toks(format!(
        "({:04x} -> {:04x}): {}(->{})",
        coverage_range.start_glyph_id,
        coverage_range.end_glyph_id,
        coverage_range.value,
        end_coverage_index
    ))
}

use gasp::display_gasp_metrics;
mod gasp {
    use super::*;

    pub(crate) fn display_gasp_metrics(
        gasp: &Option<GaspMetrics>,
        conf: &Config,
    ) -> TokenStream<'static> {
        if let Some(GaspMetrics {
            version,
            num_ranges,
            ranges,
        }) = gasp
        {
            let heading = toks(format!("gasp: version {version}, {num_ranges} ranges"));
            if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
                heading.glue(
                    LineBreak,
                    arrayfmt::display_items_elided(
                        ranges,
                        display_gasp_range,
                        conf.bookend_size,
                        |start, stop| {
                            toks(format!(
                                "skipping gasp ranges for max_ppem values {}..={}",
                                ranges[start].range_max_ppem,
                                ranges[stop - 1].range_max_ppem
                            ))
                            .pre_indent(1)
                        },
                    ),
                )
            } else {
                heading
            }
        } else {
            TokenStream::empty()
        }
    }

    fn display_gasp_range(range_index: usize, range: &GaspRange) -> TokenStream<'static> {
        let GaspBehaviorFlags {
            symmetric_smoothing: syms,
            symmetric_gridfit: symgrift,
            dogray: dg,
            gridfit: grift,
        } = range.range_gasp_behavior;
        // NOTE - Meanings attributed [here](https://learn.microsoft.com/en-us/typography/opentype/spec/gasp)
        let disp = {
            let mut buffer = String::new();

            // Dynamic separator that starts out empty but becomes " | " if any flag-string is pushed
            let mut sep = "";

            let flag_strings = [
                if syms { "SYMMETRIC_SMOOTHING" } else { "" },
                if symgrift { "SYMMETRIC_GRIDFIT" } else { "" },
                if dg { "DOGRAY" } else { "" },
                if grift { "GRIDFIT" } else { "" },
            ];

            for flag in flag_strings.iter() {
                if flag.is_empty() {
                    continue;
                } else {
                    buffer.push_str(sep);
                    buffer.push_str(flag);
                    sep = " | ";
                }
            }

            if buffer.is_empty() {
                toks("(no flags)")
            } else {
                toks(buffer).paren()
            }
        };
        if range_index == 0 && range.range_max_ppem == 0xFFFF {
            Token::then(tok("[∀ PPEM] "), disp)
        } else {
            Token::then(tok(format!("[PPEM <= {}]  ", range.range_max_ppem)), disp)
        }
        .pre_indent(2)
    }
}

fn format_version16dot16(v: u32) -> String {
    let major = (v >> 16) as u16;
    let minor = ((v & 0xf000) >> 12) as u16;
    format_version_major_minor(major, minor)
}

fn format_version_major_minor(major: u16, minor: u16) -> String {
    format!("{major}.{minor}")
}

use cmap::display_cmap_metrics;
mod cmap {
    use super::*;

    pub(crate) fn display_cmap_metrics(cmap: &Cmap, conf: &Config) -> TokenStream<'static> {
        let heading = toks(format!("cmap: version {}", cmap.version));

        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            heading.glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    &cmap.encoding_records,
                    display_encoding_record,
                    conf.bookend_size,
                    |start, stop| toks(format!("\t(skipping encoding records {start}..{stop})")),
                ),
            )
        } else {
            heading.chain(toks(format!(
                ", {} encoding tables",
                cmap.encoding_records.len()
            )))
        }
    }

    fn display_encoding_record(ix: usize, record: &EncodingRecord) -> TokenStream<'static> {
        // TODO[epic=enrichment]: if we implement subtables and more verbosity levels, show subtable details
        let EncodingRecord {
            platform,
            encoding,
            subtable: _subtable,
        } = record;
        toks(format!("[{ix}]: platform={platform}, encoding={encoding}")).pre_indent(2)
    }
}

// FIXME - Refactor into pure TokenStream
fn show_head_metrics(head: &HeadMetrics, _conf: &Config) {
    println!(
        "head: version {}, {}",
        format_version_major_minor(head.major_version, head.minor_version),
        head.dir_hint,
    );
}

// FIXME - Refactor into pure TokenStream
fn show_hhea_metrics(hhea: &HheaMetrics, _conf: &Config) {
    println!(
        "hhea: table version {}, {} horizontal long metrics",
        format_version_major_minor(hhea.major_version, hhea.minor_version),
        hhea.num_lhm,
    );
}

// FIXME - Refactor into pure TokenStream
fn show_vhea_metrics(vhea: &Option<VheaMetrics>, _conf: &Config) {
    if let Some(vhea) = vhea {
        println!(
            "vhea: table version {}, {} vertical long metrics",
            format_version_major_minor(vhea.major_version, vhea.minor_version),
            vhea.num_lvm,
        )
    }
}

use hmtx::display_hmtx_metrics;
mod hmtx {
    use super::*;

    pub(crate) fn display_hmtx_metrics(hmtx: &HmtxMetrics, conf: &Config) -> TokenStream<'static> {
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            toks("hmtx").glue(
                LineBreak,
                arrayfmt::display_items_elided(
                    &hmtx.0,
                    display_unified_bearing,
                    conf.bookend_size,
                    |start, stop| toks(format!("{HT}(skipping hmetrics {start}..{stop})")),
                ),
            )
        } else {
            toks(format!("hmtx: {} hmetrics", hmtx.0.len()))
        }
    }
    fn display_unified_bearing(ix: usize, hmet: &UnifiedBearing) -> TokenStream<'static> {
        match &hmet.advance_width {
            Some(width) => toks(format!(
                "\tGlyph ID [{ix}]: advanceWidth={width}, lsb={}",
                hmet.left_side_bearing
            )),
            None => toks(format!("\tGlyph ID [{ix}]: lsb={}", hmet.left_side_bearing)),
        }
    }
}

use vmtx::show_vmtx_metrics;
mod vmtx {
    use super::*;

    // FIXME - refactor into pure TokenStream
    pub(crate) fn show_vmtx_metrics(vmtx: &Option<VmtxMetrics>, conf: &Config) {
        // TODO - add mechanism for auto-computation of current indent level
        const INDENT_LEVEL: u8 = 1;
        if let Some(vmtx) = vmtx {
            let disp = if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
                toks("vmtx").glue(
                    LineBreak,
                    arrayfmt::display_items_elided(
                        &vmtx.0,
                        display_unified_bearing,
                        conf.bookend_size,
                        |start, stop| {
                            toks(format!("(skipping vmetrics {start}..{stop})"))
                                .pre_indent(INDENT_LEVEL)
                        },
                    ),
                )
            } else {
                toks(format!("vmtx: {} vmetrics", vmtx.0.len()))
            };
            disp.println()
        }
    }

    fn display_unified_bearing(ix: usize, vmet: &UnifiedBearing) -> TokenStream<'static> {
        // TODO - add mechanism for auto-computation of current indent level
        const INDENT_LEVEL: u8 = 2;

        // FIXME - `_width` is a misnomer, should be `_height`
        match &vmet.advance_width {
            Some(height) => toks(format!(
                "Glyph ID [{ix}]: advanceHeight={height}, tsb={}",
                vmet.left_side_bearing // FIXME - `left` is a misnomer, should be `top`
            )),
            // FIXME - `left` is a misnomer, should be `top`
            None => toks(format!("Glyph ID [{ix}]: tsb={}", vmet.left_side_bearing)),
        }
        .pre_indent(INDENT_LEVEL)
    }
}

// FIXME - rewrite into pure TokenStream
fn show_maxp_metrics(maxp: &MaxpMetrics, _conf: &Config) {
    match maxp {
        MaxpMetrics::Postscript { version } => println!(
            "maxp: version {} (PostScript)",
            format_version16dot16(*version)
        ),
        MaxpMetrics::UnknownVersion { version } => println!(
            "maxp: version {} (not recognized)",
            format_version16dot16(*version)
        ),
        // STUB - currently limited by definition of Version1 variant, but the information available in the type may be enriched later
        MaxpMetrics::Version1 { version } => println!(
            "maxp: version {} (contents omitted)",
            format_version16dot16(*version)
        ),
    }
}

// FIXME - rewrite into pure TokenStream
fn show_name_metrics(name: &NameMetrics, conf: &Config) {
    // STUB - add more details if appropriate
    match &name.lang_tag_records {
        Some(records) => {
            println!(
                "name: version {}, {} name_records, {} language tag records",
                name.version,
                name.name_count,
                records.len()
            );
        }
        None => println!(
            "name: version {}, {} name_records",
            name.version, name.name_count
        ),
    }
    if conf.verbosity.is_at_least(VerboseLevel::Baseline) {
        let mut missing_name = true;
        for record in name.name_records.iter() {
            match record {
                &NameRecord {
                    name_id: NameId::FULL_FONT_NAME,
                    plat_encoding_lang,
                    ref buf,
                } => {
                    if missing_name && plat_encoding_lang.matches_locale(buf, conf.locale) {
                        println!("\tFull Font Name: {buf}");
                        missing_name = false;
                    }
                }
                // STUB - if there are any more name records we care about, add them here
                _ => continue,
            }
        }
    }
}

fn show_os2_metrics(os2: &Os2Metrics, _conf: &Config) {
    // TODO - Metrics type is a stub, enrich if anything is 'interesting'
    println!("os/2: version {}", os2.version);
}

fn show_post_metrics(post: &PostMetrics, _conf: &Config) {
    // STUB - Metrics is just an alias for the raw type, enrich and refactor if appropriate
    println!(
        "post: version {} ({})",
        format_version16dot16(post.version),
        if post.is_fixed_pitch {
            "monospaced"
        } else {
            "proportionally spaced"
        }
    );
}

use glyf::show_glyf_metrics;
mod glyf {
    use super::*;

    // FIXME - refactor into pure TokenStream
    pub(crate) fn show_glyf_metrics(glyf: &Option<GlyfMetrics>, conf: &Config) {
        // TODO - add mechanism for auto-computation of current indent level
        const INDENT_LEVEL: u8 = 1;
        let disp = if let Some(glyf) = glyf.as_ref() {
            let hdr = tok(format!("glyf: {} glyphs", glyf.num_glyphs));
            if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
                hdr.then(LineBreak.then(arrayfmt::display_items_elided(
                    glyf.glyphs.as_slice(),
                    display_glyph_metric,
                    conf.bookend_size,
                    |start, stop| {
                        toks(format!("(skipping glyphs {start}..{stop})")).pre_indent(INDENT_LEVEL)
                    },
                )))
            } else {
                hdr.into()
            }
        } else {
            toks("glyf: <not present>")
        };
        disp.println()
    }

    fn display_glyph_metric(ix: usize, glyf: &GlyphMetric) -> TokenStream<'static> {
        // TODO - add mechanism for auto-computation of current indent level
        const INDENT_LEVEL: u8 = 2;

        tok(format!("[{ix}]: "))
            .then(match glyf {
                GlyphMetric::Empty => toks("<empty>"),
                GlyphMetric::Simple(simple) => toks(format!(
                    "Simple Glyph [{} contours, {} coordinates, {} instructions, xy: {}]",
                    simple.contours, simple.coordinates, simple.instructions, simple.bounding_box
                )),
                GlyphMetric::Composite(composite) => toks(format!(
                    "Composite Glyph [{} components, {} instructions, xy: {}]",
                    composite.components, composite.instructions, composite.bounding_box,
                )),
            })
            .pre_indent(INDENT_LEVEL)
    }
}
