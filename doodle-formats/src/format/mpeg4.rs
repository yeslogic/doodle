use doodle::helper::{u8, *};
use doodle::{
    BaseType, Expr, Format, FormatModule, FormatRef, IntoLabel, Label, Pattern, ValueType,
};

/// Helper for reading the 24-bit `flags` field common to many mpeg4 boxes
fn u24be() -> Format {
    tuple_repeat(3, u8())
}

fn tag_pattern(tag: [char; 4]) -> Pattern {
    Pattern::Tuple(vec![
        Pattern::U8(tag[0] as u8),
        Pattern::U8(tag[1] as u8),
        Pattern::U8(tag[2] as u8),
        Pattern::U8(tag[3] as u8),
    ])
}

fn tag_pattern3(tag: [char; 3]) -> Pattern {
    Pattern::Tuple(vec![
        Pattern::U8(0xA9),
        Pattern::U8(tag[0] as u8),
        Pattern::U8(tag[1] as u8),
        Pattern::U8(tag[2] as u8),
    ])
}

/// Given the correct `FormatRef` for mpeg4 tag-sequences, as well as the inner `datta` format
/// for an mpeg4 atom-kind, constructs an mpeg4 atom.
///
// TODO - add a relevant reference hint for where this appears in the spec
// TODO - refactor so that either `data` is a FormatRef, or creates a definition corresponding to it
fn make_atom(tag: FormatRef, data: Format) -> Format {
    record([
        ("size-field", u32be()),
        ("type", tag.call()),
        (
            "size",
            Format::Match(
                Box::new(var("size-field")),
                vec![
                    (Pattern::U32(0), compute(Expr::U64(0))), // FIXME
                    (
                        Pattern::U32(1),
                        map(u64be(), lambda("x", sub(var("x"), Expr::U64(16)))),
                    ),
                    (
                        Pattern::Wildcard,
                        compute(as_u64(sub(var("size-field"), Expr::U32(8)))),
                    ),
                ],
            ),
        ),
        // TODO: refactor so `data: FormatRef`
        ("data", slice(var("size"), data)),
    ])
}

fn define_atom(
    module: &mut FormatModule,
    name: impl IntoLabel,
    tag: FormatRef,
    data: Format,
) -> FormatRef {
    module.define_format(name, make_atom(tag, data))
}

// TODO - use this function proactively, or remove it
#[expect(dead_code)]
fn define_atom_args(
    module: &mut FormatModule,
    name: impl IntoLabel,
    tag: FormatRef,
    args: Vec<(Label, ValueType)>,
    data: Format,
) -> FormatRef {
    module.define_format_args(name, args, make_atom(tag, data))
}

pub fn main(module: &mut FormatModule) -> FormatRef {
    use subformats::*;
    // REVIEW - is it better for this to be a tuple than a sequential type that can be shown as a string more easily?
    let tag = mpeg4_tag(module);

    let ftyp_data = module.define_format(
        "mpeg4.ftyp-data",
        record([
            ("major_brand", tag.call()),
            ("minor_version", u32be()),
            ("compatible_brands", repeat(tag.call())),
        ]),
    );

    let meta_hdlr_data = module.define_format(
        "mpeg4.meta-hdlr-data",
        record([
            ("version", u8()),
            ("flags", tuple_repeat(3, u8())),
            ("predefined", u32be()),
            ("handler_type", tag.call()),
            ("reserved", tuple_repeat(3, u32be())),
            ("name", asciiz_string()),
        ]),
    );

    let pitm_data = pitm_data(module);
    let iinf = iinf(module, tag);
    let iref_data = iref_data(module, tag);
    let iloc_data = iloc_data(module);
    let dinf_atom = dinf_atom(module, tag);

    let mvhd_data = mvhd_data(module);

    // TODO: apply this pattern more broadly to all make_atom parameter-formats

    let trak_atom = trak_atom(module, tag, dinf_atom);

    let ilst_atom = ilst_atom(module, tag);

    let meta_atom = {
        meta_atom(
            module,
            tag,
            meta_hdlr_data,
            pitm_data,
            iinf,
            iref_data,
            iloc_data,
            dinf_atom,
            ilst_atom,
        )
    };

    let udta_atom = define_atom(
        module,
        "mpeg4.udta-atom",
        tag,
        match_variant(
            var("type"),
            vec![
                (
                    tag_pattern(['m', 'e', 't', 'a']),
                    "meta",
                    tuple(vec![
                        u32be(), // 8-bit version, 24-bit flags
                        repeat(meta_atom.call()),
                    ]),
                ),
                (Pattern::Wildcard, "unknown", opaque_bytes()),
            ],
        ),
    );

    let moov_atom = moov_atom(module, tag, mvhd_data, trak_atom, udta_atom);

    let mpeg4_atom = subformats::mpeg4_atom(module, tag, ftyp_data, meta_atom, moov_atom);

    module.define_format(
        "mpeg4.main",
        record([
            ("atoms", repeat(mpeg4_atom.call())),
            //("atoms", repeat_count(Expr::U8(4), atom.call())),
            //("trailer", opaque_bytes())
        ]),
    )
}

pub(crate) mod subformats {
    use super::*;
    pub(crate) fn mdia_atom_data(
        module: &mut FormatModule,
        tag: FormatRef,
        minf_atom: FormatRef,
    ) -> FormatRef {
        let mdia_hdlr_data = module.define_format(
            "mpeg4.mdia-hdlr-data",
            record([
                ("version", u8()),
                ("flags", tuple_repeat(3, u8())),
                ("component_type", u32be()),
                ("component_subtype", tag.call()),
                ("component_manufacturer", u32be()),
                ("component_flags", u32be()),
                ("component_flags_mask", u32be()),
                ("component_name", asciiz_string()),
            ]),
        );

        let mdhd_data = mdhd_data(module);
        module.define_format_args(
            "mpeg4.mdia-atom.data",
            vec![(
                "type".into(),
                module.get_format_type(tag.get_level()).clone(),
            )],
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern(['h', 'd', 'l', 'r']),
                        "hdlr",
                        mdia_hdlr_data.call(),
                    ),
                    (tag_pattern(['m', 'd', 'h', 'd']), "mdhd", mdhd_data.call()),
                    (
                        tag_pattern(['m', 'i', 'n', 'f']),
                        "minf",
                        repeat(minf_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }

    pub(crate) fn trak_atom(
        module: &mut FormatModule,
        tag: FormatRef,
        dinf_atom: FormatRef,
    ) -> FormatRef {
        let edts_atom = edts_atom(module, tag);
        let minf_atom = minf_atom(module, tag, dinf_atom);
        let mdia_atom_data = mdia_atom_data(module, tag, minf_atom);
        let mdia_atom = mdia_atom(module, tag, mdia_atom_data);
        let tkhd_data = tkhd_data(module);
        let trak_atom = define_atom(
            module,
            "mpeg4.trak-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['t', 'k', 'h', 'd']), "tkhd", tkhd_data.call()),
                    (
                        tag_pattern(['e', 'd', 't', 's']),
                        "edts",
                        repeat(edts_atom.call()),
                    ),
                    (
                        tag_pattern(['m', 'd', 'i', 'a']),
                        "mdia",
                        repeat(mdia_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        );
        trak_atom
    }

    pub(crate) fn mdia_atom(
        module: &mut FormatModule,
        tag: FormatRef,
        mdia_atom_data: FormatRef,
    ) -> FormatRef {
        define_atom(
            module,
            "mpeg4.mdia-atom",
            tag,
            // NOTE - make_atom binds `type` so even though this looks unscoped it is fine.
            mdia_atom_data.call_args(vec![var("type")]),
        )
    }

    pub(crate) mod stbl {
        use super::*;
        pub(crate) fn stsd_data(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
            let sample_entry =
                module.define_format("mpeg4.stsd.sample-entry", make_atom(tag, opaque_bytes()));
            module.define_format(
                // TODO - review naming schema
                "mpeg4.stsd.data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("entry_count", u32be()),
                    (
                        "sample_entries",
                        repeat_count(var("entry_count"), sample_entry.call()),
                    ),
                ]),
            )
        }
        pub(crate) fn stts_data(module: &mut FormatModule) -> FormatRef {
            let sample_entry = module.define_format(
                "mpeg4.stts.sample-entry",
                record_repeat(["sample_count", "sample_delta"], u32be()),
            );
            module.define_format(
                "mpeg4.stts-data",
                record([
                    ("version", u8()),
                    // TODO - refine into actual semantics, as appropriate
                    ("flags", u24be()),
                    ("entry_count", u32be()),
                    (
                        "sample_entries",
                        repeat_count(var("entry_count"), sample_entry.call()),
                    ),
                ]),
            )
        }
        pub(crate) fn ctts_data(module: &mut FormatModule) -> FormatRef {
            let sample_entry = module.define_format(
                "mpeg4.ctts.sample-entry",
                record([
                    ("sample_count", u32be()),
                    ("sample_offset", u32be()), // FIXME - signed if version == 1
                ]),
            );
            module.define_format(
                "mpeg4.ctts-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("entry_count", u32be()),
                    (
                        "sample_entries",
                        repeat_count(var("entry_count"), sample_entry.call()),
                    ),
                ]),
            )
        }

        pub(crate) fn stss_data(module: &mut FormatModule) -> FormatRef {
            module.define_format(
                "mpeg4.stss-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("entry_count", u32be()),
                    ("sample_number", repeat_count(var("entry_count"), u32be())),
                ]),
            )
        }

        pub(crate) fn stsc_data(module: &mut FormatModule) -> FormatRef {
            module.define_format(
                "mpeg4.stsc-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("entry_count", u32be()),
                    (
                        "chunk_entries",
                        repeat_count(
                            var("entry_count"),
                            record([
                                ("first_chunk", u32be()),
                                ("samples_per_chunk", u32be()),
                                ("sample_description_index", u32be()),
                            ]),
                        ),
                    ),
                ]),
            )
        }

        pub(crate) fn stsz_data(module: &mut FormatModule) -> FormatRef {
            module.define_format(
                "mpeg4.stsz-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("sample_size", u32be()),
                    ("sample_count", u32be()),
                    (
                        "entry_size",
                        cond_maybe(
                            expr_eq(var("sample_size"), Expr::U32(0)),
                            repeat_count(var("sample_count"), u32be()),
                        ),
                    ),
                ]),
            )
        }
        pub(crate) fn stco_data(module: &mut FormatModule) -> FormatRef {
            module.define_format(
                "mpeg4.stco-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("entry_count", u32be()),
                    ("chunk_offset", repeat_count(var("entry_count"), u32be())),
                ]),
            )
        }

        pub(crate) fn co64_data(module: &mut FormatModule) -> FormatRef {
            module.define_format(
                "mpeg4.co64-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("entry_count", u32be()),
                    ("chunk_offset", repeat_count(var("entry_count"), u64be())),
                ]),
            )
        }
        pub(crate) fn sgpd_data(module: &mut FormatModule) -> FormatRef {
            module.define_format(
                "mpeg4.sgpd-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("grouping_type", u32be()),
                    // FIXME handle version >= 2
                    ("default_length", u32be()),
                    ("entry_count", u32be()),
                    (
                        "sample_groups",
                        repeat_count(
                            var("entry_count"),
                            record([
                                (
                                    "description_length",
                                    if_then_else(
                                        expr_eq(var("default_length"), Expr::U32(0)),
                                        u32be(),
                                        compute(var("default_length")),
                                    ),
                                ),
                                (
                                    "sample_group_entry",
                                    repeat_count(var("description_length"), u8()),
                                ),
                            ]),
                        ),
                    ),
                ]),
            )
        }

        pub(crate) fn sbgp_data(module: &mut FormatModule) -> FormatRef {
            module.define_format(
                "mpeg4.sbgp-data",
                record([
                    ("version", u8()),
                    ("flags", u24be()),
                    ("grouping_type", u32be()),
                    (
                        "grouping_type_parameter",
                        cond_maybe(expr_eq(var("version"), Expr::U8(1)), u32be()),
                    ),
                    ("entry_count", u32be()),
                    (
                        "sample_groups",
                        repeat_count(
                            var("entry_count"),
                            record([
                                ("sample_count", u32be()),
                                ("group_description_index", u32be()),
                            ]),
                        ),
                    ),
                ]),
            )
        }
    }

    pub(crate) fn stbl_atom(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        use stbl::*;

        let stsd_data = stsd_data(module, tag);
        let stts_data = stts_data(module);
        let ctts_data = ctts_data(module);
        let stss_data = stss_data(module);
        let stsc_data = stsc_data(module);
        let stsz_data = stsz_data(module);
        let stco_data = stco_data(module);
        let co64_data = co64_data(module);
        let sgpd_data = sgpd_data(module);
        let sbgp_data = sbgp_data(module);

        define_atom(
            module,
            "mpeg4.stbl-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['s', 't', 's', 'd']), "stsd", stsd_data.call()),
                    (tag_pattern(['s', 't', 't', 's']), "stts", stts_data.call()),
                    (tag_pattern(['c', 't', 't', 's']), "ctts", ctts_data.call()),
                    (tag_pattern(['s', 't', 's', 's']), "stss", stss_data.call()),
                    (tag_pattern(['s', 't', 's', 'c']), "stsc", stsc_data.call()),
                    (tag_pattern(['s', 't', 's', 'z']), "stsz", stsz_data.call()),
                    (tag_pattern(['s', 't', 'c', 'o']), "stco", stco_data.call()),
                    (tag_pattern(['c', 'o', '6', '4']), "co64", co64_data.call()),
                    (tag_pattern(['s', 'g', 'p', 'd']), "sgpd", sgpd_data.call()),
                    (tag_pattern(['s', 'b', 'g', 'p']), "sbgp", sbgp_data.call()),
                    // REVIEW - do we want to make unknown-variants a 'standard pattern'?
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }

    pub(crate) fn dinf_atom(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let dref_data = dref_data(module, tag);
        define_atom(
            module,
            "mpeg4.dinf-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['d', 'r', 'e', 'f']), "dref", dref_data.call()),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }

    pub(crate) fn minf_atom(
        module: &mut FormatModule,
        tag: FormatRef,
        dinf_atom: FormatRef,
    ) -> FormatRef {
        let vmhd_data = vmhd_data(module);
        let smhd_data = smhd_data(module);
        let stbl_atom = stbl_atom(module, tag);

        define_atom(
            module,
            "mpeg4.minf-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['v', 'm', 'h', 'd']), "vmhd", vmhd_data.call()),
                    (tag_pattern(['s', 'm', 'h', 'd']), "smhd", smhd_data.call()),
                    (
                        tag_pattern(['d', 'i', 'n', 'f']),
                        "dinf",
                        repeat(dinf_atom.call()),
                    ),
                    (
                        tag_pattern(['s', 't', 'b', 'l']),
                        "stbl",
                        repeat(stbl_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }
    pub(crate) fn edts_atom(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let elst_data = elst_data(module);
        define_atom(
            module,
            "mpeg4.edts-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['e', 'l', 's', 't']), "elst", elst_data.call()),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }

    pub(crate) fn mdhd_data(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "mpeg4.mdhd-data",
            record([
                ("version", u8()),
                ("flags", u24be()),
                (
                    "fields",
                    match_variant(
                        var("version"),
                        vec![
                            (
                                Pattern::U8(0),
                                "version0",
                                record([
                                    ("creation_time", u32be()),
                                    ("modification_time", u32be()),
                                    ("timescale", u32be()),
                                    ("duration", u32be()),
                                ]),
                            ),
                            (
                                Pattern::U8(1),
                                "version1",
                                record([
                                    ("creation_time", u64be()),
                                    ("modification_time", u64be()),
                                    ("timescale", u32be()),
                                    ("duration", u64be()),
                                ]),
                            ),
                        ],
                    ),
                ),
                ("language", u16be()),
                ("pre_defined", u16be()),
            ]),
        )
    }
    /// `smhd` data; used in [`minf_atom`]

    pub(crate) fn smhd_data(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "mpeg4.smhd-data",
            record([
                ("version", u8()),
                ("flags", u24be()),
                ("balance", u16be()),
                // REVIEW - does the 'reserved' value need to be captured, or is it semantically vacuous?
                ("reserved", u16be()),
            ]),
        )
    }

    /// `vmhd` data; used in [`minf_atom`]
    pub(crate) fn vmhd_data(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "mpeg4.vmhd-data",
            record([
                ("version", u8()),
                ("flags", u24be()),
                ("graphicsmode", u16be()),
                ("opcolor", repeat_count(Expr::U8(3), u16be())),
            ]),
        )
    }

    pub(crate) fn moov_atom(
        module: &mut FormatModule,
        tag: FormatRef,
        mvhd_data: FormatRef,
        trak_atom: FormatRef,
        udta_atom: FormatRef,
    ) -> FormatRef {
        define_atom(
            module,
            "mpeg4.moov-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['m', 'v', 'h', 'd']), "mvhd", mvhd_data.call()),
                    (
                        tag_pattern(['t', 'r', 'a', 'k']),
                        "trak",
                        repeat(trak_atom.call()),
                    ),
                    (
                        tag_pattern(['u', 'd', 't', 'a']),
                        "udta",
                        repeat(udta_atom.call()),
                        // FIXME can be followed by optional 32-bit zero value
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }

    pub(crate) fn meta_atom(
        module: &mut FormatModule,
        tag: FormatRef,
        meta_hdlr_data: FormatRef,
        pitm_data: FormatRef,
        iinf: FormatRef,
        iref_data: FormatRef,
        iloc_data: FormatRef,
        dinf_atom: FormatRef,
        ilst_atom: FormatRef,
    ) -> FormatRef {
        let meta_atom_data = module.define_format_args(
            "mpeg4.meta-atom.data",
            vec![(
                "type".into(),
                module.get_format_type(tag.get_level()).clone(),
            )],
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern(['d', 'i', 'n', 'f']),
                        "dinf",
                        repeat(dinf_atom.call()),
                    ),
                    (
                        tag_pattern(['h', 'd', 'l', 'r']),
                        "hdlr",
                        meta_hdlr_data.call(),
                    ),
                    (tag_pattern(['p', 'i', 't', 'm']), "pitm", pitm_data.call()),
                    // REVIEW - consider naming schema for captures and functions - no-suffix, `-data`, `-atom`, etc.
                    (tag_pattern(['i', 'i', 'n', 'f']), "iinf", iinf.call()),
                    (tag_pattern(['i', 'r', 'e', 'f']), "iref", iref_data.call()),
                    (tag_pattern(['i', 'l', 'o', 'c']), "iloc", iloc_data.call()),
                    (
                        tag_pattern(['i', 'l', 's', 't']),
                        "ilst",
                        repeat(ilst_atom.call()),
                    ),
                    (tag_pattern(['i', 'd', 'a', 't']), "idat", opaque_bytes()),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        );
        define_atom(
            module,
            "mpeg4.meta-atom",
            tag,
            // NOTE - make_atom binds `type` so even though this looks unscoped it is fine.
            meta_atom_data.call_args(vec![var("type")]),
        )
    }
    pub(crate) fn mvhd_data(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "mpeg4.mvhd-data",
            record([
                ("version", u8()),
                ("flags", tuple_repeat(3, u8())),
                (
                    "fields",
                    match_variant(
                        var("version"),
                        vec![
                            (
                                Pattern::U8(0),
                                "version0",
                                record([
                                    ("creation_time", u32be()),
                                    ("modification_time", u32be()),
                                    ("timescale", u32be()),
                                    ("duration", u32be()),
                                ]),
                            ),
                            (
                                Pattern::U8(1),
                                "version1",
                                record([
                                    ("creation_time", u64be()),
                                    ("modification_time", u64be()),
                                    ("timescale", u32be()),
                                    ("duration", u64be()),
                                ]),
                            ),
                        ],
                    ),
                ),
                ("rate", u32be()),
                ("volume", u16be()),
                ("reserved1", u16be()),
                ("reserved2", tuple([u32be(), u32be()])),
                ("matrix", repeat_count(Expr::U8(9), u32be())),
                ("pre_defined", repeat_count(Expr::U8(6), u32be())),
                ("next_track_ID", u32be()),
            ]),
        )
    }
    pub(crate) fn tkhd_data(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "mpeg4.tkhd-data",
            record([
                ("version", u8()),
                ("flags", tuple_repeat(3, u8())),
                (
                    "fields",
                    match_variant(
                        var("version"),
                        vec![
                            (
                                Pattern::U8(0),
                                "version0",
                                record([
                                    ("creation_time", u32be()),
                                    ("modification_time", u32be()),
                                    ("track_ID", u32be()),
                                    ("reserved", u32be()),
                                    ("duration", u32be()),
                                ]),
                            ),
                            (
                                Pattern::U8(1),
                                "version1",
                                record([
                                    ("creation_time", u64be()),
                                    ("modification_time", u64be()),
                                    ("track_ID", u32be()),
                                    ("reserved", u32be()),
                                    ("duration", u64be()),
                                ]),
                            ),
                        ],
                    ),
                ),
                ("reserved2", tuple_repeat(2, u32be())),
                ("layer", u16be()),
                ("alternate_group", u16be()),
                ("volume", u16be()),
                ("reserved1", u16be()),
                ("matrix", repeat_count(Expr::U8(9), u32be())),
                ("width", u32be()),
                ("height", u32be()),
            ]),
        )
    }

    pub(crate) fn mpeg4_atom(
        module: &mut FormatModule,
        tag: FormatRef,
        ftyp_data: FormatRef,
        meta_atom: FormatRef,
        moov_atom: FormatRef,
    ) -> FormatRef {
        define_atom(
            module,
            "mpeg4.atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['f', 't', 'y', 'p']), "ftyp", ftyp_data.call()),
                    (tag_pattern(['f', 'r', 'e', 'e']), "free", Format::EMPTY),
                    (
                        tag_pattern(['m', 'd', 'a', 't']),
                        "mdat",
                        Format::EMPTY, // FIXME
                    ),
                    (
                        tag_pattern(['m', 'e', 't', 'a']),
                        "meta",
                        tuple(vec![
                            u32be(), // 8-bit version, 24-bit flags
                            repeat(meta_atom.call()),
                        ]),
                    ),
                    (
                        tag_pattern(['m', 'o', 'o', 'v']),
                        "moov",
                        repeat(moov_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }

    // REVIEW - find source for information on ILST (not in spec-archive PDF)
    pub(crate) fn ilst_atom(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let data_data = module.define_format(
            "mpeg4.tool_atom.data.data",
            record([
                ("type_indicator", u32be()),
                ("locale_indicator", u32be()),
                ("value", repeat(ascii_char())),
            ]),
        );
        let tool_atom = define_atom(
            module,
            "mpeg4.tool-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (tag_pattern(['d', 'a', 't', 'a']), "data", data_data.call()),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        );
        define_atom(
            module,
            "mpeg4.ilst-atom",
            tag,
            match_variant(
                var("type"),
                vec![
                    (
                        tag_pattern3(['t', 'o', 'o']),
                        "tool",
                        repeat(tool_atom.call()),
                    ),
                    (Pattern::Wildcard, "unknown", opaque_bytes()),
                ],
            ),
        )
    }

    pub(crate) fn elst_data(module: &mut FormatModule) -> FormatRef {
        let elst_entry = module.define_format(
            "mpeg4.elst-data.entry",
            record([
                ("track_duration", u32be()),
                ("media_time", u32be()),
                ("media_rate", u32be()),
            ]),
        );
        module.define_format(
            "mpeg4.elst-data",
            record([
                ("version", u8()),
                ("flags", tuple_repeat(3, u8())),
                ("number_of_entries", u32be()),
                (
                    "edit_list_table",
                    repeat_count(var("number_of_entries"), elst_entry.call()),
                ),
            ]),
        )
    }

    pub(crate) fn dref_data(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let data = define_atom(module, "mpeg4.dref-data.data", tag, opaque_bytes());
        module.define_format(
            "mpeg4.dref-data",
            record([
                ("version", u8()),
                ("flags", tuple_repeat(3, u8())),
                ("number_of_entries", u32be()),
                ("data", repeat(data.call())),
            ]),
        )
    }

    pub(crate) fn iloc_data(module: &mut FormatModule) -> FormatRef {
        let mpeg4_extent = module.define_format_args(
            "mpeg4.iloc-extent",
            vec![
                ("offset_size".into(), ValueType::Base(BaseType::U8)),
                ("length_size".into(), ValueType::Base(BaseType::U8)),
                ("index_size".into(), ValueType::Base(BaseType::U8)),
            ],
            record([
                (
                    "extent_index",
                    Format::Match(
                        Box::new(var("index_size")),
                        vec![
                            (Pattern::U8(0), compute(Expr::U64(0))),
                            (Pattern::U8(4), map(u32be(), lambda("x", as_u64(var("x"))))),
                            (Pattern::U8(8), u64be()),
                        ],
                    ),
                ),
                (
                    "extent_offset",
                    Format::Match(
                        Box::new(var("offset_size")),
                        vec![
                            (Pattern::U8(0), compute(Expr::U64(0))),
                            (Pattern::U8(4), map(u32be(), lambda("x", as_u64(var("x"))))),
                            (Pattern::U8(8), u64be()),
                        ],
                    ),
                ),
                (
                    "extent_length",
                    Format::Match(
                        Box::new(var("length_size")),
                        vec![
                            (Pattern::U8(0), compute(Expr::U64(0))),
                            (Pattern::U8(4), map(u32be(), lambda("x", as_u64(var("x"))))),
                            (Pattern::U8(8), u64be()),
                        ],
                    ),
                ),
            ]),
        );

        module.define_format(
            "mpeg4.iloc-atom.data",
            merge_records([
                record([("version", u8()), ("flags", tuple_repeat(3, u8()))]),
                // two four-bit fields, for offset_size and length_size
                u4_pair("offset_size", "length_size"),
                // two four-bit fields; index_size is reserved and should be treated as `0` for version 0
                remap_field(
                    "index_size",
                    |f| {
                        if_then_else(
                            expr_gt(var("version"), Expr::U8(0)),
                            f,
                            compute(Expr::U8(0)),
                        )
                    },
                    u4_pair("base_offset_size", "index_size"),
                ),
                record([
                    (
                        "item_count",
                        if_then_else(
                            expr_lt(var("version"), Expr::U8(2)),
                            map(u16be(), lambda("x", as_u32(var("x")))),
                            u32be(),
                        ),
                    ),
                    (
                        "items",
                        repeat_count(
                            var("item_count"),
                            record([
                                (
                                    "item_ID",
                                    if_then_else(
                                        expr_lt(var("version"), Expr::U8(2)),
                                        map(u16be(), lambda("x", as_u32(var("x")))),
                                        u32be(),
                                    ),
                                ),
                                (
                                    "construction_method",
                                    cond_maybe(expr_gt(var("version"), Expr::U8(0)), u16be()),
                                ),
                                ("data_reference_index", u16be()),
                                (
                                    "base_offset",
                                    Format::Match(
                                        Box::new(var("base_offset_size")),
                                        vec![
                                            (Pattern::U8(0), compute(Expr::U64(0))),
                                            (
                                                Pattern::U8(4),
                                                map(u32be(), lambda("x", as_u64(var("x")))),
                                            ),
                                            (Pattern::U8(8), u64be()),
                                        ],
                                    ),
                                ),
                                ("extent_count", u16be()),
                                (
                                    "extents",
                                    repeat_count(
                                        var("extent_count"),
                                        mpeg4_extent.call_args(vec![
                                            var("offset_size"),
                                            var("length_size"),
                                            var("index_size"),
                                        ]),
                                    ),
                                ),
                            ]),
                        ),
                    ),
                ]),
            ]),
        )
    }

    pub(crate) fn iref_data(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let single_item_reference_data = record([
            ("from_item_ID", u16be()),
            ("reference_count", u16be()),
            ("to_item_ID", repeat_count(var("reference_count"), u16be())),
        ]);

        let single_item_reference_large_data = record([
            ("from_item_ID", u32be()),
            ("reference_count", u16be()),
            ("to_item_ID", repeat_count(var("reference_count"), u32be())),
        ]);
        let small_ref = define_atom(
            module,
            "mpeg4.iref-data.single-item-reference.small",
            tag,
            single_item_reference_data,
        );
        let large_ref = define_atom(
            module,
            "mpeg4.iref-data.single-item-reference.large",
            tag,
            single_item_reference_large_data,
        );
        module.define_format(
            "mpeg4.iref-data",
            record([
                ("version", u8()),
                ("flags", tuple_repeat(3, u8())),
                (
                    "single_item_reference",
                    match_variant(
                        var("version"),
                        vec![
                            (Pattern::U8(0), "small", repeat(small_ref.call())),
                            (Pattern::U8(1), "large", repeat(large_ref.call())),
                            // FIXME - do we want to do anything in particular for versions higher than 1?
                        ],
                    ),
                ),
            ]),
        )
    }

    pub(crate) fn mpeg4_tag(module: &mut FormatModule) -> FormatRef {
        module.define_format("mpeg4.tag", tuple_repeat(4, ascii_char()))
    }

    /// Subformat registration for `pitm` atom data.
    pub(crate) fn pitm_data(module: &mut FormatModule) -> FormatRef {
        module.define_format(
            "mpeg4.pitm-atom.data",
            record([
                ("version", u8()),
                // TODO - refine flags into actual semantics
                ("flags", tuple_repeat(3, u8())),
                (
                    "item_ID",
                    if_then_else_variant(
                        expr_eq(var("version"), Expr::U8(0)),
                        ("Id16", u16be()),
                        ("Id32", u32be()),
                    ),
                ),
            ]),
        )
    }

    /// Subformat registration for `iinf` atom data.
    pub(crate) fn iinf(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
        let infe_data_extra_mime = module.define_format(
            "mpeg4.infe-atom.data.extra-fields.mime",
            record([
                ("content_type", asciiz_string()),
                //FIXME optional ("content_encoding", asciiz_string()),
            ]),
        );
        let infe_data_extra_uri = module.define_format(
            "mpeg4.infe-atom.data.extra-fields.uri",
            record([("item_uri_type", asciiz_string())]),
        );

        let infe_extra_fields = module.define_format_args(
            "mpeg4.infe-atom.data.extra-fields",
            vec![(
                "item_type".into(),
                module.get_format_type(tag.get_level()).clone(),
            )],
            match_variant(
                var("item_type"),
                vec![
                    (
                        tag_pattern(['m', 'i', 'm', 'e']),
                        "mime",
                        infe_data_extra_mime.call(),
                    ),
                    (
                        tag_pattern(['u', 'r', 'i', ' ']),
                        "uri",
                        infe_data_extra_uri.call(),
                    ),
                    (
                        Pattern::Wildcard,
                        "unknown",
                        Format::EMPTY, // FIXME
                    ),
                ],
            ),
        );

        let infe_data_ver1 = module.define_format(
            "mpeg4.infe-data.fields.version-lt2",
            record([
                ("item_ID", u16be()),
                ("item_protection_index", u16be()),
                ("item_name", asciiz_string()),
                ("content_type", asciiz_string()),
                ("content_encoding", asciiz_string()),
                // FIXME unfinished
            ]),
        );

        let infe_data_ver2 = module.define_format_args(
            "mpeg4.infe-data.fields.version-gte2",
            vec![("version".into(), ValueType::Base(BaseType::U8))],
            record([
                (
                    "item_ID",
                    if_then_else(
                        expr_eq(var("version"), Expr::U8(2)),
                        map(u16be(), lambda("x", as_u32(var("x")))),
                        u32be(),
                    ),
                ),
                ("item_protection_index", u16be()),
                ("item_type", tag.call()),
                ("item_name", asciiz_string()),
                (
                    "extra_fields",
                    infe_extra_fields.call_args(vec![var("item_type")]),
                ),
            ]),
        );

        let infe_data_fields = module.define_format_args(
            "mpeg4.infe-data.fields",
            vec![("version".into(), ValueType::Base(BaseType::U8))],
            if_then_else_variant(
                expr_lt(var("version"), Expr::U8(2)),
                ("Version1", infe_data_ver1.call()),
                ("Version2", infe_data_ver2.call_args(vec![var("version")])),
            ),
        );

        let infe_data = module.define_format(
            "mpeg4.iinf-atom.data-infe",
            record([
                ("version", u8()),
                // TODO - refine into actual semantics, as appropriate
                ("flags", u24be()),
                ("fields", infe_data_fields.call_args(vec![var("version")])),
            ]),
        );

        let iinf_atom = module.define_format(
            "mpeg4.iinf-atom",
            make_atom(
                tag,
                match_variant(
                    var("type"),
                    vec![
                        (tag_pattern(['i', 'n', 'f', 'e']), "infe", infe_data.call()),
                        (Pattern::Wildcard, "unknown", opaque_bytes()),
                    ],
                ),
            ),
        );

        module.define_format(
            "mpeg4.iinf",
            record([
                ("version", u8()),
                // TODO - refine into actual semantics, as appropriate
                ("flags", u24be()),
                (
                    "entry_count",
                    if_then_else(
                        expr_eq(var("version"), Expr::U8(0)),
                        map(u16be(), lambda("x", as_u32(var("x")))),
                        u32be(),
                    ),
                ),
                (
                    "item_info_entry",
                    repeat_count(var("entry_count"), iinf_atom.call()),
                ),
            ]),
        )
    }
}
