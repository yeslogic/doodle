use doodle::{Expr, Format, FormatModule, FormatRef, Pattern};

use crate::format::base::*;

fn tag_pattern(tag: [char; 4]) -> Pattern {
    Pattern::Tuple(vec![
        Pattern::U8(tag[0] as u8),
        Pattern::U8(tag[1] as u8),
        Pattern::U8(tag[2] as u8),
        Pattern::U8(tag[3] as u8),
    ])
}

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    let tag = module.define_format(
        "mpeg4.tag",
        tuple([
            base.ascii_char(),
            base.ascii_char(),
            base.ascii_char(),
            base.ascii_char(),
        ]),
    );

    let ftyp_data = record([
        ("major_brand", base.u32be()),
        ("minor_version", base.u32be()),
        ("compatible_brands", Format::Repeat(Box::new(base.u32be()))),
    ]);

    let edts_atom = module.define_format(
        "mpeg4.edts-atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![(
                            Pattern::Wildcard,
                            "unknown",
                            Format::Repeat(Box::new(base.u8())),
                        )],
                    )),
                ),
            ),
        ]),
    );

    let dinf_atom = module.define_format(
        "mpeg4.dinf-atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![(
                            Pattern::Wildcard,
                            "unknown",
                            Format::Repeat(Box::new(base.u8())),
                        )],
                    )),
                ),
            ),
        ]),
    );

    let stbl_atom = module.define_format(
        "mpeg4.stbl-atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![(
                            Pattern::Wildcard,
                            "unknown",
                            Format::Repeat(Box::new(base.u8())),
                        )],
                    )),
                ),
            ),
        ]),
    );

    let minf_atom = module.define_format(
        "mpeg4.minf-atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![
                            (
                                tag_pattern(['d', 'i', 'n', 'f']),
                                "dinf",
                                Format::Repeat(Box::new(dinf_atom.call())),
                            ),
                            (
                                tag_pattern(['s', 't', 'b', 'l']),
                                "stbl",
                                Format::Repeat(Box::new(stbl_atom.call())),
                            ),
                            (
                                Pattern::Wildcard,
                                "unknown",
                                Format::Repeat(Box::new(base.u8())),
                            ),
                        ],
                    )),
                ),
            ),
        ]),
    );

    let mdia_atom = module.define_format(
        "mpeg4.mdia-atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![
                            (
                                tag_pattern(['m', 'i', 'n', 'f']),
                                "minf",
                                Format::Repeat(Box::new(minf_atom.call())),
                            ),
                            (
                                Pattern::Wildcard,
                                "unknown",
                                Format::Repeat(Box::new(base.u8())),
                            ),
                        ],
                    )),
                ),
            ),
        ]),
    );

    let trak_atom = module.define_format(
        "mpeg4.trak-atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![
                            (
                                tag_pattern(['e', 'd', 't', 's']),
                                "edts",
                                Format::Repeat(Box::new(edts_atom.call())),
                            ),
                            (
                                tag_pattern(['m', 'd', 'i', 'a']),
                                "mdia",
                                Format::Repeat(Box::new(mdia_atom.call())),
                            ),
                            (
                                Pattern::Wildcard,
                                "unknown",
                                Format::Repeat(Box::new(base.u8())),
                            ),
                        ],
                    )),
                ),
            ),
        ]),
    );

    let moov_atom = module.define_format(
        "mpeg4.moov-atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![
                            (
                                tag_pattern(['t', 'r', 'a', 'k']),
                                "trak",
                                Format::Repeat(Box::new(trak_atom.call())),
                            ),
                            (
                                Pattern::Wildcard,
                                "unknown",
                                Format::Repeat(Box::new(base.u8())),
                            ),
                        ],
                    )),
                ),
            ),
        ]),
    );

    let atom = module.define_format(
        "mpeg4.atom",
        record([
            ("size-field", base.u32be()),
            ("type", tag.call()),
            (
                "size",
                if_then_else(
                    expr_eq(var("size-field"), Expr::U32(1)),
                    base.u64be(),
                    Format::Compute(as_u64(var("size-field"))),
                ),
            ),
            (
                "data",
                Format::Slice(
                    sub(var("size"), Expr::U64(8)),
                    Box::new(match_variant(
                        var("type"),
                        vec![
                            (tag_pattern(['f', 't', 'y', 'p']), "ftyp", ftyp_data),
                            (
                                tag_pattern(['m', 'o', 'o', 'v']),
                                "moov",
                                Format::Repeat(Box::new(moov_atom.call())),
                            ),
                            (
                                Pattern::Wildcard,
                                "unknown",
                                Format::Repeat(Box::new(base.u8())),
                            ),
                        ],
                    )),
                ),
            ),
        ]),
    );

    module.define_format(
        "mpeg4.main",
        record([
            ("atoms", Format::Repeat(Box::new(atom.call()))),
            //("atoms", repeat_count(Expr::U8(4), atom.call())),
            //("trailer", Format::Repeat(Box::new(base.u8())))
        ]),
    )
}
