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

fn show_magic(magic: u32) {
    println!("(sfntVersion: {})", format_magic(magic));
}

// REVIEW - less commonly used due to implementation of `Tag` type, which diverges in Display slightly
fn format_magic(magic: u32) -> String {
    let bytes = magic.to_be_bytes();
    let show = |b: u8| {
        if b.is_ascii_alphanumeric() {
            String::from(b as char)
        } else {
            format!("{b:02x}")
        }
    };
    format!(
        "{}{}{}{}",
        show(bytes[0]),
        show(bytes[1]),
        show(bytes[2]),
        show(bytes[3])
    )
}

fn show_font_metrics(font: &SingleFontMetrics, conf: &Config) {
    if !conf.extra_only {
        show_magic(font.sfnt_version);
        show_required_metrics(&font.required, conf);
        show_optional_metrics(&font.optional, conf);
    }
    show_extra_magic(&font.extraMagic);
}

fn show_extra_magic(table_ids: &[u32]) {
    for id in table_ids.iter() {
        println!("{}: [MISSING IMPL]", format_magic(*id));
    }
}

fn show_required_metrics(required: &RequiredTableMetrics, conf: &Config) {
    show_cmap_metrics(&required.cmap, conf);
    show_head_metrics(&required.head, conf);
    show_hhea_metrics(&required.hhea, conf);
    show_hmtx_metrics(&required.hmtx, conf);
    show_maxp_metrics(&required.maxp, conf);
    show_name_metrics(&required.name, conf);
    show_os2_metrics(&required.os2, conf);
    show_post_metrics(&required.post, conf);
}

fn show_optional_metrics(optional: &OptionalTableMetrics, conf: &Config) {
    show_cvt_metrics(&optional.cvt, conf);
    show_fpgm_metrics(&optional.fpgm, conf);
    show_loca_metrics(&optional.loca, conf);
    show_glyf_metrics(&optional.glyf, conf);
    show_prep_metrics(&optional.prep, conf);
    show_gasp_metrics(&optional.gasp, conf);

    // STUB - anything between gasp and gdef go here

    // TODO - refactor into proper TokenStreams
    display_base_metrics(&optional.base, conf).println();
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

    show_fvar_metrics(optional.fvar.as_deref(), conf);
    show_gvar_metrics(optional.gvar.as_deref(), conf);

    show_kern_metrics(&optional.kern, conf);
    show_stat_metrics(optional.stat.as_deref(), conf);
    show_vhea_metrics(&optional.vhea, conf);
    show_vmtx_metrics(&optional.vmtx, conf);
}

fn show_gvar_metrics(gvar: Option<&GvarMetrics>, conf: &Config) {
    let Some(gvar) = gvar else { return };
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        fn display_shared_tuples(
            shared_tuples: &[GvarTupleRecord],
        ) -> display::TokenStream<'static> {
            fn display_shared_tuple_record(
                tuple: &GvarTupleRecord,
            ) -> display::TokenStream<'static> {
                use display::{tok, toks};
                const COORD_BOOKEND: usize = 4;
                arrayfmt::display_items_inline(
                    &tuple.coordinates,
                    |coord| toks(format!("{}", coord)),
                    COORD_BOOKEND,
                    |n_skipped| toks(format!("...({n_skipped} skipped)...")),
                )
            }

            use display::toks;
            const RECORDS_BOOKEND: usize = 4;
            arrayfmt::display_items_elided(
                shared_tuples,
                |shared_tuple_ix, record| {
                    toks(format!("[{shared_tuple_ix}]: "))
                        .chain(display_shared_tuple_record(record))
                        .pre_indent(4)
                },
                RECORDS_BOOKEND,
                |start, stop| {
                    toks(format!("(skipping shared tuples {start}..{stop})")).pre_indent(3)
                },
            )
        }
        fn display_glyph_variation_data_array(
            array: &[Option<GlyphVariationData>],
        ) -> display::TokenStream<'_> {
            fn display_glyph_variation_data(
                table: &GlyphVariationData,
            ) -> display::TokenStream<'static> {
                fn display_tuple_variation_header(
                    header: &GvarTupleVariationHeader,
                ) -> display::TokenStream<'static> {
                    display::Token::from(format!(
                        "Header (size: {} bytes)",
                        header.variation_data_size
                    ))
                    .into()
                    // WIP
                }

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

            use display::{Token::LineBreak, toks};
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

fn show_fvar_metrics(fvar: Option<&FvarMetrics>, conf: &Config) {
    let Some(fvar) = fvar else { return };
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        println!(
            "fvar: version {}",
            format_version_major_minor(fvar.major_version, fvar.minor_version)
        );
        println!("\tAxes:");
        fn display_variation_axis_record(
            axis: &VariationAxisRecord,
        ) -> display::TokenStream<'static> {
            display::Token::from(format!(
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
            .into()
        }
        fn display_instance_record(
            instance: &InstanceRecord,
            conf: &Config,
        ) -> display::TokenStream<'static> {
            display::Token::from(format!(
                "Subfamily={:?};{} ",
                instance.subfamily_nameid,
                match instance.postscript_nameid {
                    None => String::new(),
                    Some(name_id) => format!(" Postscript={name_id:?};"),
                },
            ))
            .then(arrayfmt::display_items_inline(
                &instance.coordinates,
                |coord| display::Token::from(format!("{coord:+}")).into(),
                conf.inline_bookend,
                |n_skipped| {
                    display::Token::from(format!("...(skipping {n_skipped} coordinates)...")).into()
                },
            ))
        }

        // FIXME: rewerite into pure TokenStream
        arrayfmt::display_items_elided(
            &fvar.axes,
            |ix, axis| {
                display::Token::from(format!("\t\t[{ix}]: "))
                    .then(display_variation_axis_record(axis))
            },
            conf.bookend_size,
            |start, stop| {
                display::Token::from(format!("\t(skipping axis records {start}..{stop})")).into()
            },
        )
        .println();
        println!("\tInstances:");
        arrayfmt::display_items_elided(
            &fvar.instances,
            |ix, instance| {
                display::Token::from(format!("\t\t[{ix}]: "))
                    .then(display_instance_record(instance, conf))
            },
            conf.bookend_size,
            |start, stop| {
                display::Token::from(format!("\t(skipping instance records {start}..{stop})"))
                    .into()
            },
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

fn show_cvt_metrics(cvt: &Option<CvtMetrics>, _conf: &Config) {
    let Some(RawArrayMetrics(count)) = cvt else {
        return;
    };

    println!("cvt: FWORD[{count}]")
}

fn show_fpgm_metrics(fpgm: &Option<FpgmMetrics>, _conf: &Config) {
    if let Some(RawArrayMetrics(count)) = fpgm {
        println!("fpgm: uint8[{count}]")
    }
}

fn show_prep_metrics(prep: &Option<PrepMetrics>, _conf: &Config) {
    if let Some(RawArrayMetrics(count)) = prep {
        println!("prep: uint8[{count}]")
    }
}

fn show_loca_metrics(loca: &Option<LocaMetrics>, _conf: &Config) {
    if let Some(()) = loca {
        println!("loca: (details omitted)")
    }
}

fn show_gdef_metrics(gdef: Option<&GdefMetrics>, conf: &Config) {
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

/// Tokenizer for `BaseMetrics` (inline)
fn display_base_metrics(
    base: &Option<BaseMetrics>,
    _conf: &Config,
) -> display::TokenStream<'static> {
    use display::{TokenStream, toks};
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

/// Returns the table-identifier associated with a Layout table discriminator.
fn format_table_disc(disc: TableDiscriminator) -> &'static str {
    match disc {
        TableDiscriminator::Gpos => "GPOS",
        TableDiscriminator::Gsub => "GSUB",
    }
}

// TODO - convert to TokenStream producer
fn display_layout_metrics(
    layout: Option<&LayoutMetrics>,
    ctxt: Ctxt,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, toks};
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
            format_table_disc(ctxt.get_disc().expect("Ctxt missing TableDiscriminator")),
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
        display::TokenStream::empty()
    }
}

// TODO - convert to tokenstream producer
fn display_script_list(script_list: &ScriptList, conf: &Config) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, toks};
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
) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, toks};
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
fn display_langsys(lang_sys: &Option<LangSys>, conf: &Config) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
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

fn display_feature_list(
    feature_list: &FeatureList,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
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
fn display_feature_table(table: &FeatureTable, conf: &Config) -> display::TokenStream<'static> {
    use display::{tok, toks};

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
) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
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
                    display::toks(format!("(skipping LookupTables {start}..{stop})")).pre_indent(3)
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
) -> display::TokenStream<'static> {
    // NOTE - because we print the kind of the lookup here, we don't need to list it for every element
    // LINK[format-lookup-subtable] -  (see format_lookup_subtable below)
    let mut stream = display::Token::from(format!(
        "LookupTable: kind={}",
        format_lookup_type(ctxt, table.lookup_type),
    ))
    .then(display_lookup_flags(&table.lookup_flag));
    if let Some(filtering_set) = table.mark_filtering_set {
        stream = stream.chain(
            display::Token::from(format!(
                ", markFilteringSet=GDEF->MarkGlyphSet[{filtering_set}]"
            ))
            .into(),
        );
    }
    stream.chain(
        display::Token::InlineText(": ".into()).then(arrayfmt::display_items_inline(
            &table.subtables,
            |subtable| display_lookup_subtable(subtable, false, conf),
            conf.inline_bookend,
            |n_skipped| display::Token::from(format!("...({n_skipped} skipped)...")).into(),
        )),
    )
}

// ANCHOR[format-lookup-subtable]
fn display_lookup_subtable(
    subtable: &LookupSubtable,
    show_lookup_type: bool,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{tok, toks};
    // STUB - because the subtables are both partial (more variants exist) and abridged (existing variants are missing details), reimplement as necessary
    // FIXME - refactor so contents is a pure tokenstream
    // WIP
    let (label, contents) = match subtable {
        LookupSubtable::SinglePos(single_pos) => {
            let contents = {
                match single_pos {
                    SinglePos::Format1(SinglePosFormat1 { value_record, .. }) => {
                        tok("single").then(display_value_record(value_record).paren())
                    }
                    SinglePos::Format2(SinglePosFormat2 { coverage, .. }) => {
                        tok("array").then(display_coverage_table(coverage).paren())
                    }
                }
            };
            ("SinglePos", contents)
        }
        LookupSubtable::PairPos(pair_pos) => {
            let contents = {
                match pair_pos {
                    PairPos::Format1(PairPosFormat1 { coverage, .. }) => {
                        tok("byGlyph").then(display_coverage_table(coverage).paren())
                    }
                    PairPos::Format2(PairPosFormat2 {
                        coverage,
                        class_def1,
                        class_def2,
                        class1_records,
                    }) => {
                        let rows = class1_records.rows();
                        let cols = class1_records.width();

                        validate_class_count(class_def1, rows);
                        validate_class_count(class_def2, cols);

                        // REVIEW - if not too verbose, we might want a compact overview of the Class1Record array, specifically which index-pairs constitute actual adjustments
                        let _populated_class_pairs: Vec<(usize, usize)> = {
                            Iterator::zip(0..rows, 0..cols)
                                .filter(|ixpair| {
                                    let it = &class1_records[*ixpair];
                                    it.value_record1.is_some() || it.value_record2.is_some()
                                })
                                .collect()
                        };
                        // maximum number of index-pairs we are willing to display inline (chosen arbitrarily)
                        // TODO - should this be a more general parameter in the Config type?
                        const MAX_POPULATION: usize = 3;
                        if _populated_class_pairs.len() <= MAX_POPULATION {
                            tok(format!("byClass{:?}", _populated_class_pairs,))
                                .then(display_coverage_table(coverage).surround(tok("("), tok(")")))
                        } else {
                            tok(format!(
                                "byClass[{} ∈ {rows} x {cols}]",
                                _populated_class_pairs.len(),
                            ))
                            .then(display_coverage_table(coverage).paren())
                        }
                    }
                }
            };
            ("PairPos", contents)
        }
        LookupSubtable::CursivePos(CursivePos { coverage, .. }) => {
            let contents = tok("entryExit").then(display_coverage_table(coverage).paren());
            ("CursivePos", contents)
        }
        LookupSubtable::MarkBasePos(MarkBasePos {
            mark_coverage,
            base_coverage,
            mark_array,
            base_array,
        }) => {
            let contents = {
                let mut mark_iter = mark_coverage.iter();
                let mut base_iter = base_coverage.iter();
                let mark_lhs = tok("Mark").then(display_coverage_table(mark_coverage).paren());
                let base_lhs = tok("Base").then(display_coverage_table(base_coverage).paren());
                let mark_rhs =
                    tok("MarkArray").then(display_mark_array(mark_array, &mut mark_iter).bracket());
                let base_rhs =
                    tok("BaseArray").then(display_base_array(base_array, &mut base_iter).bracket());
                let lhs = mark_lhs.glue(tok("+"), base_lhs);
                let rhs = mark_rhs.glue(tok("+"), base_rhs);
                lhs.glue(tok("=>"), rhs)
            };
            ("MarkBasePos", contents)
        }
        LookupSubtable::MarkLigPos(MarkLigPos {
            mark_coverage,
            ligature_coverage,
            mark_array,
            ligature_array,
        }) => {
            let contents = {
                let mut mark_iter = mark_coverage.iter();
                let mut ligature_iter = ligature_coverage.iter();
                let mark_lhs = tok("Mark").then(display_coverage_table(mark_coverage).paren());
                let lig_lhs =
                    tok("Ligature").then(display_coverage_table(ligature_coverage).paren());
                let mark_rhs =
                    tok("MarkArray").then(display_mark_array(mark_array, &mut mark_iter).bracket());
                let lig_rhs = tok("LigatureArray")
                    .then(display_ligature_array(ligature_array, &mut ligature_iter).bracket());
                let lhs = mark_lhs.glue(tok("+"), lig_lhs);
                let rhs = mark_rhs.glue(tok("+"), lig_rhs);
                lhs.glue(tok("=>"), rhs)
            };
            ("MarkLigPos", contents)
        }
        LookupSubtable::MarkMarkPos(MarkMarkPos {
            mark1_coverage,
            mark2_coverage,
            mark1_array,
            mark2_array,
        }) => {
            let contents = {
                let mut mark1_iter = mark1_coverage.iter();
                let mut mark2_iter = mark2_coverage.iter();
                let mark_lhs = tok("Mark").then(display_coverage_table(mark1_coverage).paren());
                let mark2_lhs = tok("Mark").then(display_coverage_table(mark2_coverage).paren());
                let mark_rhs = tok("MarkArray")
                    .then(display_mark_array(mark1_array, &mut mark1_iter).bracket());
                let mark2_rhs = tok("Mark2Array")
                    .then(display_mark2_array(mark2_array, &mut mark2_iter).bracket());
                let lhs = mark_lhs.glue(tok("+"), mark2_lhs);
                let rhs = mark_rhs.glue(tok("+"), mark2_rhs);
                lhs.glue(tok("=>"), rhs)
            };
            ("MarkMarkPos", contents)
        }
        LookupSubtable::SingleSubst(single_subst) => {
            let contents = match single_subst {
                SingleSubst::Format1(SingleSubstFormat1 {
                    coverage,
                    delta_glyph_id,
                }) => {
                    display_coverage_table(coverage).chain(toks(format!("=>({delta_glyph_id:+})")))
                }
                SingleSubst::Format2(SingleSubstFormat2 {
                    coverage,
                    substitute_glyph_ids,
                }) => {
                    let iter = coverage.iter();
                    display_coverage_table(coverage).glue(
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
            };
            ("SingleSubst", contents)
        }
        LookupSubtable::MultipleSubst(multi_subst) => {
            let contents = {
                let MultipleSubst {
                    coverage,
                    subst: MultipleSubstFormat1 { sequences },
                } = &multi_subst;
                // REVIEW - is this the right balance of specificity and brevity?
                display_coverage_table(coverage)
                    .chain(toks(format!("=>SequenceTable[{}]", sequences.len())))
            };
            ("MultipleSubst", contents)
        }
        LookupSubtable::AlternateSubst(AlternateSubst {
            coverage,
            alternate_sets,
        }) => {
            let contents = {
                display_coverage_table(coverage)
                    .glue(tok("=>"), display_alternate_sets(alternate_sets))
            };
            ("AlternateSubst", contents)
        }
        LookupSubtable::LigatureSubst(LigatureSubst {
            coverage,
            ligature_sets,
        }) => {
            // WIP
            let contents = display_ligature_sets(ligature_sets, coverage.iter());
            ("LigatureSubst", contents)
        }
        LookupSubtable::ReverseChainSingleSubst(ReverseChainSingleSubst {
            coverage,
            backtrack_coverages,
            lookahead_coverages,
            substitute_glyph_ids,
            ..
        }) => {
            let contents = {
                // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                const INLINE_INLINE_BOOKEND: usize = 1;
                // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                let backtrack_pattern = if backtrack_coverages.is_empty() {
                    display::TokenStream::empty()
                } else {
                    arrayfmt::display_items_inline(
                        backtrack_coverages,
                        display_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| display::toks(format!("(..{n}..)")),
                    )
                    .surround(tok("(?<="), tok(")"))
                };
                let input_pattern = display_coverage_table(coverage);
                let lookahead_pattern = if lookahead_coverages.is_empty() {
                    display::TokenStream::empty()
                } else {
                    arrayfmt::display_items_inline(
                        lookahead_coverages,
                        display_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| display::toks(format!("(..{n}..)")),
                    )
                    .surround(tok("(?="), tok(")"))
                };
                let substitute_ids = format_glyphid_array_hex(substitute_glyph_ids, true);

                backtrack_pattern
                    .chain(input_pattern)
                    .chain(lookahead_pattern)
                    .chain(toks(format!("=>{substitute_ids}")))
            };
            ("RevChainSingleSubst", contents)
        }
        LookupSubtable::SequenceContext(seq_ctx) => {
            let contents = match seq_ctx {
                SequenceContext::Format1(SequenceContextFormat1 { coverage, .. }) => {
                    tok("Glyphs").then(display_coverage_table(coverage).paren())
                }
                SequenceContext::Format2(SequenceContextFormat2 { coverage, .. }) => {
                    tok("Classes").then(display_coverage_table(coverage).paren())
                }
                SequenceContext::Format3(SequenceContextFormat3 {
                    coverage_tables,
                    seq_lookup_records,
                    ..
                }) => {
                    // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                    const INLINE_INLINE_BOOKEND: usize = 1;
                    // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                    let input_pattern = arrayfmt::display_items_inline(
                        coverage_tables,
                        display_coverage_table,
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
            };
            ("SeqCtx", contents)
        }
        LookupSubtable::ChainedSequenceContext(chain_ctx) => {
            let contents = match chain_ctx {
                ChainedSequenceContext::Format1(ChainedSequenceContextFormat1 {
                    coverage, ..
                }) => {
                    // TODO - even if it means overly verbose output, this might be too little info to be useful compared to discriminant-only display
                    tok("ChainedGlyphs").then(display_coverage_table(coverage).paren())
                }
                ChainedSequenceContext::Format2(ChainedSequenceContextFormat2 {
                    coverage, ..
                }) => {
                    // TODO - even if it means overly verbose output, this might be too little info to be useful compared to discriminant-only display
                    // REVIEW - consider what other details (e.g. class-def summary metrics) to show in implicitly- or explicitly-verbose display format
                    tok("ChainedClasses").then(display_coverage_table(coverage).paren())
                }
                ChainedSequenceContext::Format3(ChainedSequenceContextFormat3 {
                    backtrack_coverages,
                    input_coverages,
                    lookahead_coverages,
                    seq_lookup_records,
                    ..
                }) => {
                    // REVIEW - since we are already within an inline elision context, try to avoid taking up too much space per item, but this might not want to be a hardcoded value
                    const INLINE_INLINE_BOOKEND: usize = 1;
                    // FIXME - show_lookup_table calls this function through display_items_inline already, so we might want to reduce how many values we are willing to show proportionally
                    let backtrack_pattern = if backtrack_coverages.is_empty() {
                        display::TokenStream::empty()
                    } else {
                        arrayfmt::display_items_inline(
                            backtrack_coverages,
                            display_coverage_table,
                            INLINE_INLINE_BOOKEND,
                            |n| toks(format!("(..{n}..)")),
                        )
                        .surround(tok("(?<="), tok(")"))
                    };
                    let input_pattern = arrayfmt::display_items_inline(
                        input_coverages,
                        display_coverage_table,
                        INLINE_INLINE_BOOKEND,
                        |n| toks(format!("(..{n}..)")),
                    );
                    let lookahead_pattern = if lookahead_coverages.is_empty() {
                        display::TokenStream::empty()
                    } else {
                        arrayfmt::display_items_inline(
                            lookahead_coverages,
                            display_coverage_table,
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
            };
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

fn show_stat_metrics(stat: Option<&StatMetrics>, conf: &Config) {
    fn display_design_axis(axis: &DesignAxis, _conf: &Config) -> display::TokenStream<'static> {
        display::toks(format!(
            "Tag={} ; Axis NameID={} ; Ordering={}",
            axis.axis_tag, axis.axis_name_id.0, axis.axis_ordering
        ))
    }
    fn display_axis_value(value: &AxisValue, conf: &Config) -> display::TokenStream<'static> {
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

        use display::{tok, toks};
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
                    display::toks(format!(
                        "Axis[{}] = {}",
                        axis_value.axis_index, axis_value.value
                    ))
                },
                conf.inline_bookend,
                |n_skipped| {
                    display::toks(format!("...(skipping {n_skipped} AxisValue records)..."))
                },
            )),
        }
    }
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
                            display::tok(format!("\t\t[{ix}]: "))
                                .then(display_design_axis(d_axis, conf))
                        },
                        conf.bookend_size,
                        |start, stop| {
                            display::toks(format!(
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
                            display::tok(format!("\t\t[{ix}]: "))
                                .then(display_axis_value(a_value, conf))
                        },
                        conf.bookend_size,
                        |start, stop| {
                            display::toks(format!(
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

fn show_kern_metrics(kern: &Option<KernMetrics>, conf: &Config) {
    use display::{tok, toks};
    fn display_kern_subtable(
        subtable: &KernSubtable,
        conf: &Config,
    ) -> display::TokenStream<'static> {
        use display::{tok, toks};
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
        ) -> display::TokenStream<'static> {
            use display::{tok, toks};
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
                    ) -> display::TokenStream<'static> {
                        use display::{tok, toks};
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
                    ) -> display::TokenStream<'static> {
                        arrayfmt::display_wec_rows_elided(
                            &array.0,
                            |ix, row| {
                                display::tok(format!("\t\t[{ix}]: ")).then(
                                    arrayfmt::display_items_inline(
                                        row,
                                        |kern_val| display::toks(format!("{kern_val:+}")),
                                        conf.inline_bookend,
                                        |n| display::toks(format!("(..{n}..)")),
                                    ),
                                )
                            },
                            conf.bookend_size / 2, // FIXME - magic constant adjustment
                            |start, stop| {
                                display::toks(format!(
                                    "\t\t(skipping kerning array rows {start}..{stop})"
                                ))
                            },
                        )
                    }
                    let left = tok("LeftClass=").then(display_kern_class_table(left_class, conf));

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

fn display_mark2_array(
    arr: &Mark2Array,
    coverage: &mut impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    use display::toks;

    fn display_mark2_record(mark2_record: &Mark2Record, cov: u16) -> display::TokenStream<'static> {
        const CLASS_ANCHORS: usize = 2;
        use display::{tok, toks};
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
) -> display::TokenStream<'static> {
    fn display_ligature_attach(
        ligature_attach: &LigatureAttach,
        cov: u16,
    ) -> display::TokenStream<'static> {
        fn display_component_record(
            component_record: &ComponentRecord,
        ) -> display::TokenStream<'static> {
            use display::{tok, toks};
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

        use display::{tok, toks};
        const COMPONENTS_BOOKEND: usize = 1;
        tok(format!("{cov:04x}=")).then(arrayfmt::display_items_inline(
            &ligature_attach.component_records,
            display_component_record,
            COMPONENTS_BOOKEND,
            |_| toks(".."),
        ))
    }

    use display::toks;
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
) -> display::TokenStream<'static> {
    fn display_base_record(base_record: &BaseRecord, cov: u16) -> display::TokenStream<'static> {
        use display::{tok, toks};
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

    use display::toks;
    const BASE_ARRAY_BOOKEND: usize = 2;
    arrayfmt::display_items_inline(
        &base_array.base_records,
        |base_record| display_base_record(base_record, coverage.next().expect("missing coverage")),
        BASE_ARRAY_BOOKEND,
        |n_skipped| toks(format!("...({n_skipped} skipped)...")),
    )
}

fn display_mark_array(
    mark_array: &MarkArray,
    coverage: &mut impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    fn display_mark_record(mark_record: &MarkRecord, cov: u16) -> display::TokenStream<'static> {
        use display::{tok, toks};
        tok(format!("{cov:04x}=({}, ", mark_record.mark_class,)).then(
            display_anchor_table(mark_record.mark_anchor.as_ref().expect("broken link"))
                .chain(toks(")")),
        )
    }

    // FIXME[magic] - arbitrary local bookending const
    const MARK_ARRAY_BOOKEND: usize = 2;
    arrayfmt::display_items_inline(
        &mark_array.mark_records,
        |mark_record| display_mark_record(mark_record, coverage.next().expect("missing coverage")),
        MARK_ARRAY_BOOKEND,
        |n_skipped| display::toks(format!("...({n_skipped} skipped)...")),
    )
}

fn display_anchor_table(anchor: &AnchorTable) -> display::TokenStream<'static> {
    use display::{tok, toks};
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
            let extra = match (x_device, y_device) {
                (None, None) => unreachable!(
                    "unexpected both-Null DeviceOrVariationIndexTable-offsets in AnchorTable::Format3"
                ),
                (Some(x), Some(y)) => {
                    // FIXME - refactor format_device_or_variation_index_table to be TokenStream
                    toks(format!(
                        "×({}, {})",
                        format_device_or_variation_index_table(x),
                        format_device_or_variation_index_table(y)
                    ))
                }
                (Some(x), None) => {
                    // FIXME - refactor format_device_or_variation_index_table to be TokenStream
                    toks(format!(
                        "×({}, ⅈ)",
                        format_device_or_variation_index_table(x)
                    ))
                }
                (None, Some(y)) => {
                    // FIXME - refactor format_device_or_variation_index_table to be TokenStream
                    toks(format!(
                        "×(ⅈ, {})",
                        format_device_or_variation_index_table(y)
                    ))
                }
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

fn display_ligature_sets(
    lig_sets: &[LigatureSet],
    mut coverage: impl Iterator<Item = u16>,
) -> display::TokenStream<'static> {
    fn display_ligature_set(lig_set: &LigatureSet, cov: u16) -> display::TokenStream<'static> {
        fn display_ligature(lig: &Ligature, cov: u16) -> display::TokenStream<'static> {
            // FIXME - refactor format_glyphid_array_hex to be TokenStream
            display::toks(format!(
                "(#{cov:04x}.{} => {})",
                format_glyphid_array_hex(&lig.component_glyph_ids, false),
                lig.ligature_glyph,
            ))
        }
        // FIXME[magic] - arbitrary local bookending const
        const LIG_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &lig_set.ligatures,
            |lig| display_ligature(lig, cov),
            LIG_BOOKEND,
            |n_skipped| display::toks(format!("...({n_skipped} skipped)...")),
        )
    }
    match lig_sets {
        [set] => display_ligature_set(set, coverage.next().expect("missing coverage")),
        more => {
            const LIG_SET_BOOKEND: usize = 1;
            arrayfmt::display_coverage_linked_array(
                more,
                coverage,
                |cov, lig_set| display_ligature_set(lig_set, cov),
                LIG_SET_BOOKEND,
                |_| display::toks(".."),
            )
        }
    }
}

fn display_alternate_sets(alt_sets: &[AlternateSet]) -> display::TokenStream<'static> {
    fn display_alternate_set(alt_set: &AlternateSet) -> display::TokenStream<'static> {
        use display::toks;
        const ALT_GLYPH_BOOKEND: usize = 2;
        arrayfmt::display_items_inline(
            &alt_set.alternate_glyph_ids,
            |glyph_id| toks(format_glyphid_hex(*glyph_id, true)),
            ALT_GLYPH_BOOKEND,
            |_| toks("..".to_string()),
        )
    }
    match alt_sets {
        [set] => display_alternate_set(set),
        more => {
            const ALT_SET_BOOKEND: usize = 1;
            arrayfmt::display_items_inline(more, display_alternate_set, ALT_SET_BOOKEND, |count| {
                display::toks(format!("...({count} skipped)..."))
            })
        }
    }
}

fn display_sequence_lookup(sl: &SequenceLookup) -> display::TokenStream<'static> {
    let s_ix = sl.sequence_index;
    let ll_ix = sl.lookup_list_index;
    // NOTE - the number in `\[_\]` is meant to mimic the index display of the display_items_elided formatting of LookupList, so it is the lookup index. The number after `@` is the positional index to apply the lookup to
    display::toks(format!("[{ll_ix}]@{s_ix}"))
}

/// Checks that the given ClassDef (assumed to be Some) contains the expected number of classes.
///
/// Panics if opt_classdef is None.
fn validate_class_count(class_def: &ClassDef, expected_classes: usize) {
    match class_def {
        ClassDef::Format1 {
            class_value_array,
            start_glyph_id: _start_id,
        } => {
            let max = expected_classes as u16;
            let mut actual_set = U16Set::new();
            actual_set.insert(0);
            for (_ix, value) in class_value_array.iter().enumerate() {
                if *value >= max {
                    panic!(
                        "expecting {expected_classes} starting at 0, found ClassValue {value} (>= {max}) at index {_ix} (glyph id: {})",
                        *_start_id + _ix as u16
                    );
                }
                let _ = actual_set.insert(*value);
            }
            assert_eq!(
                actual_set.len(),
                expected_classes,
                "expected to find {expected_classes} ClassDefs, found {}-element set {:?}",
                actual_set.len(),
                actual_set
            );
        }
        ClassDef::Format2 {
            class_range_records,
        } => {
            let max = expected_classes as u16;
            let mut actual_set = U16Set::new();
            actual_set.insert(0);
            for (_ix, rr) in class_range_records.iter().enumerate() {
                let value = rr.value;
                if value >= max {
                    panic!(
                        "expecting {expected_classes} starting at 0, found ClassValue {value} (>= {max}) at index {_ix} (glyph range: {} -> {})",
                        rr.start_glyph_id, rr.end_glyph_id
                    );
                }
                let _ = actual_set.insert(value);
            }
            assert_eq!(
                actual_set.len(),
                expected_classes,
                "expected to find {expected_classes} ClassDefs, found {}-element set {:?}",
                actual_set.len(),
                actual_set
            );
        }
    }
}

fn display_value_record(record: &ValueRecord) -> display::TokenStream<'static> {
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
    use display::{tok, toks};
    const NUM_FRAGMENTS: usize = 4;
    let mut buf = Vec::with_capacity(NUM_FRAGMENTS);

    // helper to indicate whether a field exists
    let elide = |opt_val: &Option<_>| -> Option<&'static str> { opt_val.as_ref().map(|_| "(..)") };

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
        // REVIEW - this is highly unlikely, right..?
        toks("<Empty ValueRecord>")
    } else {
        display::TokenStream::join_with(buf, tok("; "))
    }
}

fn display_opt_xy<T>(
    what: &str,
    x: Option<T>,
    y: Option<T>,
) -> Option<display::TokenStream<'static>>
where
    T: std::fmt::Display,
{
    use display::toks;
    match (x, y) {
        (None, None) => None,
        (Some(x), Some(y)) => Some(toks(format!("{what}: ({x},{y})"))),
        (Some(x), None) => Some(toks(format!("{what}[x]: {x}"))),
        (None, Some(y)) => Some(toks(format!("{what}[y]: {y}"))),
    }
}

/// Prints a summary of a given `LookupFlag` value, including logic to avoid printing anything for the default flag value.
///
/// Because of this elision, will also print a prefix that separates the displayed content from the previous field
fn display_lookup_flags(flags: &LookupFlag) -> display::TokenStream<'static> {
    fn display_lookup_flag(flags: &LookupFlag) -> display::TokenStream<'static> {
        let mut set_flags = Vec::new();
        if flags.right_to_left {
            set_flags.push("RIGHT_TO_LEFT");
        }
        if flags.ignore_base_glyphs {
            set_flags.push("IGNORE_BASE_GLYPHS");
        }
        if flags.ignore_ligatures {
            set_flags.push("IGNORE_LIGATURES");
        }
        if flags.ignore_marks {
            set_flags.push("IGNORE_MARKS");
        }
        if flags.use_mark_filtering_set {
            set_flags.push("USE_MARK_FILTERING_SET");
        }

        let str_flags = if set_flags.is_empty() {
            String::from("∅")
        } else {
            set_flags.join(" | ")
        };

        let str_macf = match flags.mark_attachment_class_filter {
            // NOTE - If we are not filtering by mark attachment class, we don't need to print anything for that field
            0 => String::new(),
            // REVIEW - if horizontal space is at a premium, we may want to shorten or partially elide the label-string
            n => format!("; mark_attachment_class_filter = {n}"),
        };

        display::Token::from(format!("LookupFlag ({str_flags}{str_macf})")).into()
    }

    if flags.mark_attachment_class_filter != 0
        || flags.right_to_left
        || flags.ignore_ligatures
        || flags.ignore_base_glyphs
        || flags.ignore_marks
        || flags.use_mark_filtering_set
    {
        display::tok(", flags=").then(display_lookup_flag(flags))
    } else {
        // FIXME - add special case for empty-string TokenStream
        display::toks("")
    }
}

fn format_lookup_type(ctxt: Ctxt, ltype: u16) -> &'static str {
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

fn display_mark_glyph_set(mgs: &MarkGlyphSet, conf: &Config) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
    tok("\tMarkGlyphSet:").then(LineBreak.then(arrayfmt::display_items_elided(
        &mgs.coverage,
        |ix, item| match item {
            None => toks(format!("\t\t[{ix}]: <none>")).into(),
            Some(covt) => {
                tok(format!("\t\t[{ix}]: ")).then(display_coverage_table_summary(covt, conf))
            }
        },
        conf.bookend_size,
        |start, stop| toks(format!("\t    (skipping coverage tables {start}..{stop})")),
    )))
}

fn display_item_variation_store(
    ivs: &ItemVariationStore,
    conf: &Config,
) -> display::TokenStream<'static> {
    fn display_variation_regions(
        vrl: &VariationRegionList,
        conf: &Config,
    ) -> display::TokenStream<'static> {
        fn display_variation_axes(
            per_region: &[RegionAxisCoordinates],
            conf: &Config,
        ) -> display::TokenStream<'static> {
            use display::toks;
            display::TokenStream::join_with(
                vec![
                    arrayfmt::display_table_column_horiz(
                        "\t\t start |",
                        per_region,
                        |coords| toks(format!("{:.03}", coords.start_coord)),
                        conf.inline_bookend,
                        |n_skipped| toks(format!("..{n_skipped:02}..")),
                    ),
                    arrayfmt::display_table_column_horiz(
                        "\t\t  peak |",
                        per_region,
                        |coords| toks(format!("{:.03}", coords.peak_coord)),
                        conf.inline_bookend,
                        |n_skipped| toks(format!("..{n_skipped:02}..")),
                    ),
                    arrayfmt::display_table_column_horiz(
                        "\t\t   end |",
                        per_region,
                        |coords| toks(format!("{:.03}", coords.end_coord)),
                        conf.inline_bookend,
                        |n_skipped| toks(format!("..{n_skipped:02}..")),
                    ),
                ],
                display::Token::LineBreak,
            )
        }
        use display::{Token::LineBreak, tok, toks};

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
    ) -> display::TokenStream<'static> {
        fn display_variation_data(
            ivd: &ItemVariationData,
            conf: &Config,
        ) -> display::TokenStream<'static> {
            fn display_delta_sets(
                _sets: &DeltaSets,
                _conf: &Config,
            ) -> display::TokenStream<'static> {
                // STUB - figure out what we actually want to show
                toks(format!("\t\t\t<show_delta_sets: incomplete>"))
            }

            use display::{Token::LineBreak, tok, toks};

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
        use display::{Token::LineBreak, tok, toks};
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
    use display::{Token::LineBreak, tok, toks};

    tok("\tItemVariationStore:")
        .then(LineBreak.then(display_variation_regions(&ivs.variation_region_list, conf)))
        .chain(LineBreak.then(display_variation_data_array(
            &ivs.item_variation_data_list,
            conf,
        )))
}

fn display_lig_caret_list(
    lig_caret_list: &LigCaretList,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
    toks("LigCaretList:")
        .glue(
            LineBreak,
            display_coverage_table_summary(&lig_caret_list.coverage, conf).pre_indent(4),
        )
        .glue(
            LineBreak,
            arrayfmt::display_items_elided(
                &lig_caret_list.lig_glyphs,
                |ix, lig_glyph| {
                    toks(format!("[{ix}]: "))
                        .pre_indent(4)
                        .chain(arrayfmt::display_items_inline(
                            &lig_glyph.caret_values,
                            display_caret_value,
                            conf.inline_bookend,
                            |num_skipped| toks(format!("...({num_skipped})...")),
                        ))
                },
                conf.bookend_size,
                |start, stop| toks(format!("(skipping LigGlyphs {start}..{stop})")).pre_indent(3),
            ),
        )

    // NOTE - since coverage tables are used in MarkGlyphSet, we don't want to force-indent within the `show_coverage_table` function, so we do it before instead.
}

fn display_caret_value(cv: &Link<CaretValue>) -> display::TokenStream<'static> {
    // FIXME - refactor each branch to produce tokenstream
    display::toks(match cv {
        None => unreachable!("caret value null link"),
        Some(cv) => match cv {
            // REVIEW - this isn't really a canonical abbreviation, so we might adjust what we show for Design Units (Format 1)
            CaretValue::DesignUnits(du) => format!("{du}du"),
            CaretValue::ContourPoint(ix) => format!("#{ix}"),
            CaretValue::DesignUnitsWithTable { coordinate, device } => match device {
                None => unreachable!("dev-table in caret value format 3 with null offset"),
                Some(table) => {
                    format!(
                        "{}du+{}",
                        coordinate,
                        format_device_or_variation_index_table(table) // WIP
                    )
                }
            },
        },
    })
}

// TODO - refactor into display model
fn format_device_or_variation_index_table(table: &DeviceOrVariationIndexTable) -> String {
    match table {
        DeviceOrVariationIndexTable::Device(dev_table) => format_device_table(dev_table),
        DeviceOrVariationIndexTable::VariationIndex(var_ix_table) => {
            format_variation_index_table(var_ix_table)
        }
        DeviceOrVariationIndexTable::NonStandard { delta_format } => {
            format!("[<DeltaFormat {delta_format}>]")
        }
    }
}

// TODO - refactor into display model
fn format_device_table(dev_table: &DeviceTable) -> String {
    // REVIEW - we are so far down the stack there is very little we can display inline for the delta-values, but we have them on hand if we wish to show them in some abbreviated form...
    format!("{}..{}", dev_table.start_size, dev_table.end_size)
}

// TODO - refactor into display model
fn format_variation_index_table(var_ix_table: &VariationIndexTable) -> String {
    format!(
        "{}->{}",
        var_ix_table.delta_set_outer_index, var_ix_table.delta_set_inner_index
    )
}

fn display_attach_list(attach_list: &AttachList, conf: &Config) -> display::TokenStream<'static> {
    use display::{Token::LineBreak, tok, toks};
    fn display_attach_point(point_indices: &[u16], conf: &Config) -> display::TokenStream<'static> {
        arrayfmt::display_items_inline(
            point_indices,
            |point_ix| toks(u16::to_string(point_ix)),
            conf.inline_bookend,
            |num_skipped| toks(format!("...({num_skipped})...")),
        )
    }

    // WIP
    toks("AttachList:")
        .pre_indent(2)
        .glue(
            LineBreak,
            display_coverage_table_summary(&attach_list.coverage, conf).pre_indent(4),
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

fn format_glyphid_hex(glyph: u16, is_standalone: bool) -> String {
    if is_standalone {
        format!("#{glyph:04x}")
    } else {
        format!("{glyph:04x}")
    }
}

/// Compact inline display of an array representing a sequence (rather than a set) of glyphIds
// REVIEW - we have no cap on how long a glyphId sequence we are willing to show unabridged and we might want one in theory
fn format_glyphid_array_hex(glyphs: &impl AsRef<[u16]>, is_standalone: bool) -> String {
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
    let nbytes = (if is_standalone {
        BYTE_OVERHEAD_PREFIX
    } else {
        0
    }) + (nglyphs * BYTES_PER_GLYPH)
        + (BYTE_OVERHEAD_PER_GLYPH * (nglyphs - PER_GLYPH_OVERCOUNT));

    // Initialize a buffer with enough capacity it ought not need to reallocate or grow
    let mut buffer = String::with_capacity(nbytes);

    // Fill the buffer
    if is_standalone {
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

fn display_coverage_table(cov: &CoverageTable) -> display::TokenStream<'static> {
    use display::{tok, toks};
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

fn display_coverage_table_summary(
    cov: &CoverageTable,
    conf: &Config,
) -> display::TokenStream<'static> {
    use display::{tok, toks};
    match cov {
        CoverageTable::Format1 { glyph_array } => {
            tok("Glyphs Covered: ").then(arrayfmt::display_items_inline(
                glyph_array,
                |item| toks(format!("{item}")),
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

fn show_mark_attach_class_def(mark_attach_class_def: &ClassDef, conf: &Config) {
    println!("\tMarkAttachClassDef:");
    show_class_def(mark_attach_class_def, format_mark_attach_class, conf);
}

fn format_mark_attach_class(mark_attach_class: &u16) -> String {
    // STUB - if we come up with a semantic association for specific numbers, add branches here
    format!("{mark_attach_class}")
}

fn show_glyph_class_def(class_def: &ClassDef, conf: &Config) {
    println!("\tGlyphClassDef:");
    show_class_def(class_def, show_glyph_class, conf)
}

fn show_class_def<L: Into<Label>>(
    class_def: &ClassDef,
    show_fn: impl Fn(&u16) -> L,
    conf: &Config,
) {
    use display::{tok, toks};
    // WIP
    match *class_def {
        ClassDef::Format1 {
            start_glyph_id,
            ref class_value_array,
        } => {
            match start_glyph_id {
                0 => (),
                // REVIEW - indent level model might be useful instead of ad-hoc tabs and spaces
                1 => println!("\t    (skipping uncovered glyph 0)"),
                n => println!("\t    (skipping uncovered glyphs 0..{n})"),
            }
            arrayfmt::display_items_elided(
                class_value_array,
                |ix, item| {
                    let gix = start_glyph_id as usize + ix;
                    tok(format!("\t\tGlyph [{gix}]: ")).then(toks(show_fn(item)))
                },
                conf.bookend_size,
                |start, stop| {
                    toks(format!(
                        "\t    (skipping glyphs {}..{})",
                        format_glyphid_hex(start_glyph_id + start as u16, false),
                        format_glyphid_hex(start_glyph_id + stop as u16, false),
                    ))
                },
            )
            .println() // WIP
        }
        ClassDef::Format2 {
            ref class_range_records,
        } => arrayfmt::display_items_elided(
            class_range_records,
            |_ix, class_range| {
                tok(format!(
                    "\t\t({} -> {}): ",
                    class_range.start_glyph_id, class_range.end_glyph_id,
                ))
                .then(toks(show_fn(&class_range.value)))
            },
            conf.bookend_size,
            |start, stop| {
                let low_end = class_range_records[start].start_glyph_id;
                let high_end = class_range_records[stop - 1].end_glyph_id;
                toks(format!(
                    "\t    (skipping ranges covering glyphs {low_end}..={high_end})",
                ))
            },
        )
        .println(), // WIP
    }
}

fn display_coverage_range_record(
    coverage_range: &CoverageRangeRecord,
) -> display::TokenStream<'static> {
    let span = coverage_range.end_glyph_id - coverage_range.start_glyph_id;
    let end_coverage_index = coverage_range.value + span;
    display::toks(format!(
        "({} -> {}): {}(->{})",
        coverage_range.start_glyph_id,
        coverage_range.end_glyph_id,
        coverage_range.value,
        end_coverage_index
    ))
}

fn show_gasp_metrics(gasp: &Option<GaspMetrics>, conf: &Config) {
    if let Some(GaspMetrics {
        version,
        num_ranges,
        ranges,
    }) = gasp
    {
        fn display_gasp_range(_ix: usize, range: &GaspRange) -> display::TokenStream<'static> {
            use display::{tok, toks};

            let GaspBehaviorFlags {
                symmetric_smoothing: syms,
                symmetric_gridfit: symgrift,
                dogray: dg,
                gridfit: grift,
            } = range.range_gasp_behavior;
            // NOTE - Meanings attributed [here](https://learn.microsoft.com/en-us/typography/opentype/spec/gasp)
            let disp = {
                let mut sep = ""; // Dynamic separator that starts out empty but becomes " | " if any flag-string is pushed
                let mut buffer = String::new();
                for flag in [
                    if syms { "SYMMETRIC_SMOOTHING" } else { "" },
                    if symgrift { "SYMMETRIC_GRIDFIT" } else { "" },
                    if dg { "DOGRAY" } else { "" },
                    if grift { "GRIDFIT" } else { "" },
                ]
                .iter()
                {
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
                    toks(format!("({buffer})"))
                }
            };
            if _ix == 0 && range.range_max_ppem == 0xFFFF {
                tok("\t[∀ PPEM] ").then(disp)
            } else {
                tok(format!("\t[PPEM <= {}]  ", range.range_max_ppem)).then(disp)
            }
        }
        println!("gasp: version {version}, {num_ranges} ranges");
        if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            arrayfmt::display_items_elided(
                ranges,
                display_gasp_range,
                conf.bookend_size,
                |start, stop| {
                    display::toks(format!(
                        "    skipping gasp ranges for max_ppem values {}..={}",
                        ranges[start].range_max_ppem,
                        ranges[stop - 1].range_max_ppem
                    ))
                },
            )
            .println(); // WIP
        }
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

fn show_cmap_metrics(cmap: &Cmap, conf: &Config) {
    use display::{Token::LineBreak, tok, toks};

    tok(format!("cmap: version {}", cmap.version))
        .then(if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            let show_record = |ix: usize, record: &EncodingRecord| {
                // TODO[enrichment]: if we implement subtables and more verbosity levels, show subtable details
                let EncodingRecord {
                    platform,
                    encoding,
                    subtable: _subtable,
                } = record;
                toks(format!(
                    "\t[{ix}]: platform={platform}, encoding={encoding}"
                ))
            };
            LineBreak.then(arrayfmt::display_items_elided(
                &cmap.encoding_records,
                show_record,
                conf.bookend_size,
                |start, stop| toks(format!("\t(skipping encoding records {start}..{stop})")),
            ))
        } else {
            toks(format!(", {} encoding tables", cmap.encoding_records.len()))
        })
        .println() // WIP
}

fn show_head_metrics(head: &HeadMetrics, _conf: &Config) {
    println!(
        "head: version {}, {}",
        format_version_major_minor(head.major_version, head.minor_version),
        head.dir_hint,
    );
}

fn show_hhea_metrics(hhea: &HheaMetrics, _conf: &Config) {
    println!(
        "hhea: table version {}, {} horizontal long metrics",
        format_version_major_minor(hhea.major_version, hhea.minor_version),
        hhea.num_lhm,
    );
}

fn show_vhea_metrics(vhea: &Option<VheaMetrics>, _conf: &Config) {
    if let Some(vhea) = vhea {
        println!(
            "vhea: table version {}, {} vertical long metrics",
            format_version_major_minor(vhea.major_version, vhea.minor_version),
            vhea.num_lvm,
        )
    }
}

fn show_hmtx_metrics(hmtx: &HmtxMetrics, conf: &Config) {
    fn display_unified(ix: usize, hmet: &UnifiedBearing) -> display::TokenStream<'static> {
        use display::toks;
        match &hmet.advance_width {
            Some(width) => toks(format!(
                "\tGlyph ID [{ix}]: advanceWidth={width}, lsb={}",
                hmet.left_side_bearing
            )),
            None => toks(format!("\tGlyph ID [{ix}]: lsb={}", hmet.left_side_bearing)),
        }
    }
    use display::{Token::LineBreak, tok, toks};
    if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
        tok("hmtx")
            .then(LineBreak.then(arrayfmt::display_items_elided(
                &hmtx.0,
                display_unified,
                conf.bookend_size,
                |start, stop| toks(format!("{HT}(skipping hmetrics {start}..{stop})")),
            )))
            .println()
    } else {
        println!("hmtx: {} hmetrics", hmtx.0.len())
    }
}

fn show_vmtx_metrics(vmtx: &Option<VmtxMetrics>, conf: &Config) {
    fn display_unified(ix: usize, vmet: &UnifiedBearing) -> display::TokenStream<'static> {
        use display::{tok, toks};
        const INDENT_LEVEL: u8 = 2;

        match &vmet.advance_width {
            // FIXME - `_width` is a misnomer, should be `_height`
            Some(height) => toks(format!(
                "Glyph ID [{ix}]: advanceHeight={height}, tsb={}",
                vmet.left_side_bearing // FIXME - `left` is a misnomer, should be `top`
            ))
            .pre_indent(INDENT_LEVEL),
            None => toks(format!("Glyph ID [{ix}]: tsb={}", vmet.left_side_bearing))
                .pre_indent(INDENT_LEVEL), // FIXME - `left` is a misnomer, should be `top`
        }
    }
    use display::{Token::LineBreak, tok, toks};

    // FIXME - add mechanism for auto-computation of current indent level
    const INDENT_LEVEL: u8 = 1;
    if let Some(vmtx) = vmtx {
        let disp = if conf.verbosity.is_at_least(VerboseLevel::Detailed) {
            tok("vmtx").then(LineBreak.then(arrayfmt::display_items_elided(
                &vmtx.0,
                display_unified,
                conf.bookend_size,
                |start, stop| {
                    toks(format!("(skipping vmetrics {start}..{stop})")).pre_indent(INDENT_LEVEL)
                },
            )))
        } else {
            toks(format!("vmtx: {} vmetrics", vmtx.0.len()))
        };
        disp.println()
    }
}

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
        let mut no_name_yet = true;
        for record in name.name_records.iter() {
            match record {
                &NameRecord {
                    name_id: NameId::FULL_FONT_NAME,
                    plat_encoding_lang,
                    ref buf,
                } => {
                    if no_name_yet && plat_encoding_lang.matches_locale(buf, conf.locale) {
                        println!("\tFull Font Name: {buf}");
                        no_name_yet = false;
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

fn show_glyf_metrics(glyf: &Option<GlyfMetrics>, conf: &Config) {
    fn display_glyph_metric(ix: usize, glyf: &GlyphMetric) -> display::TokenStream<'static> {
        use display::{tok, toks};
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

    use display::{Token::LineBreak, tok, toks};
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
