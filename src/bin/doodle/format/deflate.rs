use doodle::{DynFormat, Expr, Format, FormatModule, FormatRef, Pattern, ValueType};

use crate::format::base::*;

fn tuple_proj(x: Expr, i: usize) -> Expr {
    Expr::TupleProj(Box::new(x), i)
}

fn add(x: Expr, y: Expr) -> Expr {
    Expr::Add(Box::new(x), Box::new(y))
}

fn shl_u8(x: Expr, r: u8) -> Expr {
    Expr::Shl(Box::new(x), Box::new(Expr::U8(r)))
}

fn shl_u16(x: Expr, r: u16) -> Expr {
    Expr::Shl(Box::new(Expr::AsU16(Box::new(x))), Box::new(Expr::U16(r)))
}

fn bit_or(x: Expr, y: Expr) -> Expr {
    Expr::BitOr(Box::new(x), Box::new(y))
}

fn bits_value_u8(name: &'static str, n: usize) -> Expr {
    if n > 1 {
        bit_or(
            shl_u8(tuple_proj(var(name), n - 1), (n - 1).try_into().unwrap()),
            bits_value_u8(name, n - 1),
        )
    } else {
        tuple_proj(var(name), 0)
    }
}

fn bits_value_u16(name: &'static str, n: usize) -> Expr {
    if n > 1 {
        bit_or(
            shl_u16(tuple_proj(var(name), n - 1), (n - 1).try_into().unwrap()),
            bits_value_u16(name, n - 1),
        )
    } else {
        Expr::AsU16(Box::new(tuple_proj(var(name), 0)))
    }
}

fn bits8(n: usize, base: &BaseModule) -> Format {
    let mut fs = Vec::with_capacity(n);
    for _ in 0..n {
        fs.push(base.bit());
    }
    if n > 0 {
        Format::Map(
            Box::new(tuple(fs)),
            Expr::Lambda("bits".into(), Box::new(bits_value_u8("bits", n))),
        )
    } else {
        /* if n == 0 */
        Format::Compute(Expr::U8(0))
    }
}

fn bits16(n: usize, base: &BaseModule) -> Format {
    let mut fs = Vec::with_capacity(n);
    for _ in 0..n {
        fs.push(base.bit());
    }
    if n > 0 {
        Format::Map(
            Box::new(tuple(fs)),
            Expr::Lambda("bits".into(), Box::new(bits_value_u16("bits", n))),
        )
    } else {
        /* if n == 0 */
        Format::Compute(Expr::U16(0))
    }
}

fn distance_record0(start: usize, base: &BaseModule, extra_bits: usize) -> Format {
    record([
        ("distance-extra-bits", bits16(extra_bits, base)),
        (
            "distance",
            Format::Compute(add(Expr::U16(start as u16), var("distance-extra-bits"))),
        ),
    ])
}

fn distance_record(base: &BaseModule) -> Format {
    Format::Match(
        Expr::AsU8(Box::new(var("distance-code"))),
        vec![
            (Pattern::U8(0), distance_record0(1, base, 0)),
            (Pattern::U8(1), distance_record0(2, base, 0)),
            (Pattern::U8(2), distance_record0(3, base, 0)),
            (Pattern::U8(3), distance_record0(4, base, 0)),
            (Pattern::U8(4), distance_record0(5, base, 1)),
            (Pattern::U8(5), distance_record0(7, base, 1)),
            (Pattern::U8(6), distance_record0(9, base, 2)),
            (Pattern::U8(7), distance_record0(13, base, 2)),
            (Pattern::U8(8), distance_record0(17, base, 3)),
            (Pattern::U8(9), distance_record0(25, base, 3)),
            (Pattern::U8(10), distance_record0(33, base, 4)),
            (Pattern::U8(11), distance_record0(49, base, 4)),
            (Pattern::U8(12), distance_record0(65, base, 5)),
            (Pattern::U8(13), distance_record0(97, base, 5)),
            (Pattern::U8(14), distance_record0(129, base, 6)),
            (Pattern::U8(15), distance_record0(193, base, 6)),
            (Pattern::U8(16), distance_record0(257, base, 7)),
            (Pattern::U8(17), distance_record0(385, base, 7)),
            (Pattern::U8(18), distance_record0(513, base, 8)),
            (Pattern::U8(19), distance_record0(769, base, 8)),
            (Pattern::U8(20), distance_record0(1025, base, 9)),
            (Pattern::U8(21), distance_record0(1537, base, 9)),
            (Pattern::U8(22), distance_record0(2049, base, 10)),
            (Pattern::U8(23), distance_record0(3073, base, 10)),
            (Pattern::U8(24), distance_record0(4097, base, 11)),
            (Pattern::U8(25), distance_record0(6145, base, 11)),
            (Pattern::U8(26), distance_record0(8193, base, 12)),
            (Pattern::U8(27), distance_record0(12289, base, 12)),
            (Pattern::U8(28), distance_record0(16385, base, 13)),
            (Pattern::U8(29), distance_record0(24577, base, 13)),
        ],
    )
}

fn length_record(start: usize, base: &BaseModule, extra_bits: usize) -> Format {
    record([
        ("length-extra-bits", bits8(extra_bits, base)),
        (
            "length",
            Format::Compute(add(
                Expr::U16(start as u16),
                Expr::AsU16(Box::new(var("length-extra-bits"))),
            )),
        ),
        (
            "distance-code",
            Format::Apply("distance-alphabet-format".into()),
        ),
        ("distance-record", distance_record(base)),
    ])
}

fn length_record_fixed(start: usize, base: &BaseModule, extra_bits: usize) -> Format {
    record([
        ("length-extra-bits", bits8(extra_bits, base)),
        (
            "length",
            Format::Compute(add(
                Expr::U16(start as u16),
                Expr::AsU16(Box::new(var("length-extra-bits"))),
            )),
        ),
        ("distance-code", bits8(5, base)),
        ("distance-record", distance_record(base)),
    ])
}

fn reference_record() -> Expr {
    Expr::Match(
        Box::new(Expr::RecordProj(Box::new(var("x")), "extra".into())),
        vec![(
            Pattern::Variant("some".into(), Box::new(Pattern::Binding("rec".into()))),
            Expr::Seq(vec![Expr::Variant(
                "reference".into(),
                Box::new(Expr::Record(vec![
                    (
                        "length".into(),
                        Expr::RecordProj(Box::new(var("rec")), "length".into()),
                    ),
                    (
                        "distance".into(),
                        Expr::RecordProj(
                            Box::new(Expr::RecordProj(
                                Box::new(var("rec")),
                                "distance-record".into(),
                            )),
                            "distance".into(),
                        ),
                    ),
                ])),
            )]),
        )],
    )
}

fn fixed_code_lengths() -> Expr {
    let mut ls = Vec::new();
    for _ in 0..=143 {
        ls.push(Expr::U8(8));
    }
    for _ in 144..=255 {
        ls.push(Expr::U8(9));
    }
    for _ in 256..=279 {
        ls.push(Expr::U8(7));
    }
    for _ in 280..=287 {
        ls.push(Expr::U8(8));
    }
    Expr::Seq(ls)
}

/// Deflate
///
#[allow(clippy::redundant_clone)]
pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    let bits2 = bits8(2, base);
    let bits3 = bits8(3, base);
    let bits4 = bits8(4, base);
    let bits5 = bits8(5, base);
    let bits7 = bits8(7, base);

    let uncompressed = module.define_format(
        "deflate.uncompressed",
        record([
            ("align", Format::Align(8)),
            ("len", bits16(16, base)),
            ("nlen", bits16(16, base)),
            ("bytes", repeat_count(var("len"), bits8(8, base))),
            (
                "codes-values",
                Format::Compute(Expr::FlatMap(
                    Box::new(Expr::Lambda(
                        "x".into(),
                        Box::new(Expr::Seq(vec![Expr::Variant(
                            "literal".into(),
                            Box::new(var("x")),
                        )])),
                    )),
                    Box::new(var("bytes")),
                )),
            ),
        ]),
    );

    let fixed_huffman = module.define_format(
        "deflate.fixed_huffman",
        record([
            (
                "codes",
                Format::Dynamic(
                    "format".into(),
                    DynFormat::Huffman(fixed_code_lengths(), None),
                    Box::new(repeat_until_last(
                        Expr::Lambda(
                            "x".into(),
                            Box::new(Expr::Eq(
                                Box::new(Expr::AsU16(Box::new(Expr::RecordProj(
                                    Box::new(var("x")),
                                    "code".into(),
                                )))),
                                Box::new(Expr::U16(256)),
                            )),
                        ),
                        record([
                            ("code", Format::Apply("format".into())),
                            (
                                "extra",
                                match_variant(
                                    var("code"),
                                    vec![
                                        (
                                            Pattern::U16(257),
                                            "some",
                                            length_record_fixed(3, base, 0),
                                        ),
                                        (
                                            Pattern::U16(258),
                                            "some",
                                            length_record_fixed(4, base, 0),
                                        ),
                                        (
                                            Pattern::U16(259),
                                            "some",
                                            length_record_fixed(5, base, 0),
                                        ),
                                        (
                                            Pattern::U16(260),
                                            "some",
                                            length_record_fixed(6, base, 0),
                                        ),
                                        (
                                            Pattern::U16(261),
                                            "some",
                                            length_record_fixed(7, base, 0),
                                        ),
                                        (
                                            Pattern::U16(262),
                                            "some",
                                            length_record_fixed(8, base, 0),
                                        ),
                                        (
                                            Pattern::U16(263),
                                            "some",
                                            length_record_fixed(9, base, 0),
                                        ),
                                        (
                                            Pattern::U16(264),
                                            "some",
                                            length_record_fixed(10, base, 0),
                                        ),
                                        (
                                            Pattern::U16(265),
                                            "some",
                                            length_record_fixed(11, base, 1),
                                        ),
                                        (
                                            Pattern::U16(266),
                                            "some",
                                            length_record_fixed(13, base, 1),
                                        ),
                                        (
                                            Pattern::U16(267),
                                            "some",
                                            length_record_fixed(15, base, 1),
                                        ),
                                        (
                                            Pattern::U16(268),
                                            "some",
                                            length_record_fixed(17, base, 1),
                                        ),
                                        (
                                            Pattern::U16(269),
                                            "some",
                                            length_record_fixed(19, base, 2),
                                        ),
                                        (
                                            Pattern::U16(270),
                                            "some",
                                            length_record_fixed(23, base, 2),
                                        ),
                                        (
                                            Pattern::U16(271),
                                            "some",
                                            length_record_fixed(27, base, 2),
                                        ),
                                        (
                                            Pattern::U16(272),
                                            "some",
                                            length_record_fixed(31, base, 2),
                                        ),
                                        (
                                            Pattern::U16(273),
                                            "some",
                                            length_record_fixed(35, base, 3),
                                        ),
                                        (
                                            Pattern::U16(274),
                                            "some",
                                            length_record_fixed(43, base, 3),
                                        ),
                                        (
                                            Pattern::U16(275),
                                            "some",
                                            length_record_fixed(51, base, 3),
                                        ),
                                        (
                                            Pattern::U16(276),
                                            "some",
                                            length_record_fixed(59, base, 3),
                                        ),
                                        (
                                            Pattern::U16(277),
                                            "some",
                                            length_record_fixed(67, base, 4),
                                        ),
                                        (
                                            Pattern::U16(278),
                                            "some",
                                            length_record_fixed(83, base, 4),
                                        ),
                                        (
                                            Pattern::U16(279),
                                            "some",
                                            length_record_fixed(99, base, 4),
                                        ),
                                        (
                                            Pattern::U16(280),
                                            "some",
                                            length_record_fixed(115, base, 4),
                                        ),
                                        (
                                            Pattern::U16(281),
                                            "some",
                                            length_record_fixed(131, base, 5),
                                        ),
                                        (
                                            Pattern::U16(282),
                                            "some",
                                            length_record_fixed(163, base, 5),
                                        ),
                                        (
                                            Pattern::U16(283),
                                            "some",
                                            length_record_fixed(195, base, 5),
                                        ),
                                        (
                                            Pattern::U16(284),
                                            "some",
                                            length_record_fixed(227, base, 5),
                                        ),
                                        (
                                            Pattern::U16(285),
                                            "some",
                                            length_record_fixed(258, base, 0),
                                        ),
                                        (Pattern::Wildcard, "none", Format::EMPTY),
                                    ],
                                ),
                            ),
                        ]),
                    )),
                ),
            ),
            (
                "codes-values",
                Format::Compute(Expr::FlatMap(
                    Box::new(Expr::Lambda(
                        "x".into(),
                        Box::new(Expr::Match(
                            Box::new(Expr::RecordProj(Box::new(var("x")), "code".into())),
                            vec![
                                (Pattern::U16(256), Expr::Seq(vec![])),
                                (Pattern::U16(257), reference_record()),
                                (Pattern::U16(258), reference_record()),
                                (Pattern::U16(259), reference_record()),
                                (Pattern::U16(260), reference_record()),
                                (Pattern::U16(261), reference_record()),
                                (Pattern::U16(262), reference_record()),
                                (Pattern::U16(263), reference_record()),
                                (Pattern::U16(264), reference_record()),
                                (Pattern::U16(265), reference_record()),
                                (Pattern::U16(266), reference_record()),
                                (Pattern::U16(267), reference_record()),
                                (Pattern::U16(268), reference_record()),
                                (Pattern::U16(269), reference_record()),
                                (Pattern::U16(270), reference_record()),
                                (Pattern::U16(271), reference_record()),
                                (Pattern::U16(272), reference_record()),
                                (Pattern::U16(273), reference_record()),
                                (Pattern::U16(274), reference_record()),
                                (Pattern::U16(275), reference_record()),
                                (Pattern::U16(276), reference_record()),
                                (Pattern::U16(277), reference_record()),
                                (Pattern::U16(278), reference_record()),
                                (Pattern::U16(279), reference_record()),
                                (Pattern::U16(280), reference_record()),
                                (Pattern::U16(281), reference_record()),
                                (Pattern::U16(282), reference_record()),
                                (Pattern::U16(283), reference_record()),
                                (Pattern::U16(284), reference_record()),
                                (Pattern::U16(285), reference_record()),
                                (
                                    Pattern::Wildcard,
                                    Expr::Seq(vec![Expr::Variant(
                                        "literal".into(),
                                        Box::new(Expr::AsU8(Box::new(Expr::RecordProj(
                                            Box::new(var("x")),
                                            "code".into(),
                                        )))),
                                    )]),
                                ),
                            ],
                        )),
                    )),
                    Box::new(var("codes")),
                )),
            ),
        ]),
    );

    let dynamic_huffman = module.define_format(
        "deflate.dynamic_huffman",
        record([
            ("hlit", bits5.clone()),
            ("hdist", bits5.clone()),
            ("hclen", bits4.clone()),
            (
                "code-length-alphabet-code-lengths",
                repeat_count(add(var("hclen"), Expr::U8(4)), bits3.clone()),
            ),
            (
                "literal-length-distance-alphabet-code-lengths",
                Format::Dynamic(
                    "code-length-alphabet-format".into(),
                    DynFormat::Huffman(
                        var("code-length-alphabet-code-lengths"),
                        Some(Expr::Seq(vec![
                            Expr::U8(16),
                            Expr::U8(17),
                            Expr::U8(18),
                            Expr::U8(0),
                            Expr::U8(8),
                            Expr::U8(7),
                            Expr::U8(9),
                            Expr::U8(6),
                            Expr::U8(10),
                            Expr::U8(5),
                            Expr::U8(11),
                            Expr::U8(4),
                            Expr::U8(12),
                            Expr::U8(3),
                            Expr::U8(13),
                            Expr::U8(2),
                            Expr::U8(14),
                            Expr::U8(1),
                            Expr::U8(15),
                        ])),
                    ),
                    Box::new(repeat_until_seq(
                        Expr::Lambda(
                            "y".into(),
                            Box::new(Expr::Gte(
                                Box::new(Expr::SeqLength(Box::new(Expr::FlatMapAccum(
                                    Box::new(Expr::Lambda(
                                        "x".into(),
                                        Box::new(Expr::Match(
                                            Box::new(Expr::AsU8(Box::new(Expr::RecordProj(
                                                Box::new(Expr::TupleProj(Box::new(var("x")), 1)),
                                                "code".into(),
                                            )))),
                                            vec![
                                                (
                                                    Pattern::U8(16),
                                                    Expr::Tuple(vec![
                                                        Expr::TupleProj(Box::new(var("x")), 0),
                                                        Expr::Dup(
                                                            Box::new(add(
                                                                Expr::RecordProj(
                                                                    Box::new(Expr::TupleProj(
                                                                        Box::new(var("x")),
                                                                        1,
                                                                    )),
                                                                    "extra".into(),
                                                                ),
                                                                Expr::U8(3),
                                                            )),
                                                            Box::new(Expr::Match(
                                                                Box::new(Expr::TupleProj(
                                                                    Box::new(var("x")),
                                                                    0,
                                                                )),
                                                                vec![(
                                                                    Pattern::Variant(
                                                                        "some".into(),
                                                                        Box::new(Pattern::Binding(
                                                                            "y".into(),
                                                                        )),
                                                                    ),
                                                                    Expr::Var("y".into()),
                                                                )],
                                                            )),
                                                        ),
                                                    ]),
                                                ),
                                                (
                                                    Pattern::U8(17),
                                                    Expr::Tuple(vec![
                                                        Expr::TupleProj(Box::new(var("x")), 0),
                                                        Expr::Dup(
                                                            Box::new(add(
                                                                Expr::RecordProj(
                                                                    Box::new(Expr::TupleProj(
                                                                        Box::new(var("x")),
                                                                        1,
                                                                    )),
                                                                    "extra".into(),
                                                                ),
                                                                Expr::U8(3),
                                                            )),
                                                            Box::new(Expr::U8(0)),
                                                        ),
                                                    ]),
                                                ),
                                                (
                                                    Pattern::U8(18),
                                                    Expr::Tuple(vec![
                                                        Expr::TupleProj(Box::new(var("x")), 0),
                                                        Expr::Dup(
                                                            Box::new(add(
                                                                Expr::RecordProj(
                                                                    Box::new(Expr::TupleProj(
                                                                        Box::new(var("x")),
                                                                        1,
                                                                    )),
                                                                    "extra".into(),
                                                                ),
                                                                Expr::U8(11),
                                                            )),
                                                            Box::new(Expr::U8(0)),
                                                        ),
                                                    ]),
                                                ),
                                                (
                                                    Pattern::Binding("v".into()),
                                                    Expr::Tuple(vec![
                                                        Expr::Variant(
                                                            "some".into(),
                                                            Box::new(var("v")),
                                                        ),
                                                        Expr::Seq(vec![var("v")]),
                                                    ]),
                                                ),
                                            ],
                                        )),
                                    )),
                                    Box::new(Expr::Variant("none".into(), Box::new(Expr::UNIT))),
                                    ValueType::Union(vec![
                                        ("none".into(), ValueType::Tuple(vec![])),
                                        ("some".into(), ValueType::U8),
                                    ]),
                                    Box::new(var("y")),
                                )))),
                                Box::new(add(
                                    Expr::AsU32(Box::new(add(var("hlit"), var("hdist")))),
                                    Expr::U32(258),
                                )),
                            )),
                        ),
                        record([
                            ("code", Format::Apply("code-length-alphabet-format".into())),
                            (
                                "extra",
                                Format::Match(
                                    Expr::AsU8(Box::new(var("code"))),
                                    vec![
                                        (Pattern::U8(16), bits2.clone()),
                                        (Pattern::U8(17), bits3.clone()),
                                        (Pattern::U8(18), bits7.clone()),
                                        (Pattern::Wildcard, Format::Compute(Expr::U8(0))),
                                    ],
                                ),
                            ),
                        ]),
                    )),
                ),
            ),
            (
                "literal-length-distance-alphabet-code-lengths-value",
                Format::Compute(Expr::FlatMapAccum(
                    Box::new(Expr::Lambda(
                        "x".into(),
                        Box::new(Expr::Match(
                            Box::new(Expr::AsU8(Box::new(Expr::RecordProj(
                                Box::new(Expr::TupleProj(Box::new(var("x")), 1)),
                                "code".into(),
                            )))),
                            vec![
                                (
                                    Pattern::U8(16),
                                    Expr::Tuple(vec![
                                        Expr::TupleProj(Box::new(var("x")), 0),
                                        Expr::Dup(
                                            Box::new(add(
                                                Expr::RecordProj(
                                                    Box::new(Expr::TupleProj(
                                                        Box::new(var("x")),
                                                        1,
                                                    )),
                                                    "extra".into(),
                                                ),
                                                Expr::U8(3),
                                            )),
                                            Box::new(Expr::Match(
                                                Box::new(Expr::TupleProj(Box::new(var("x")), 0)),
                                                vec![(
                                                    Pattern::Variant(
                                                        "some".into(),
                                                        Box::new(Pattern::Binding("y".into())),
                                                    ),
                                                    Expr::Var("y".into()),
                                                )],
                                            )),
                                        ),
                                    ]),
                                ),
                                (
                                    Pattern::U8(17),
                                    Expr::Tuple(vec![
                                        Expr::TupleProj(Box::new(var("x")), 0),
                                        Expr::Dup(
                                            Box::new(add(
                                                Expr::RecordProj(
                                                    Box::new(Expr::TupleProj(
                                                        Box::new(var("x")),
                                                        1,
                                                    )),
                                                    "extra".into(),
                                                ),
                                                Expr::U8(3),
                                            )),
                                            Box::new(Expr::U8(0)),
                                        ),
                                    ]),
                                ),
                                (
                                    Pattern::U8(18),
                                    Expr::Tuple(vec![
                                        Expr::TupleProj(Box::new(var("x")), 0),
                                        Expr::Dup(
                                            Box::new(add(
                                                Expr::RecordProj(
                                                    Box::new(Expr::TupleProj(
                                                        Box::new(var("x")),
                                                        1,
                                                    )),
                                                    "extra".into(),
                                                ),
                                                Expr::U8(11),
                                            )),
                                            Box::new(Expr::U8(0)),
                                        ),
                                    ]),
                                ),
                                (
                                    Pattern::Binding("v".into()),
                                    Expr::Tuple(vec![
                                        Expr::Variant("some".into(), Box::new(var("v"))),
                                        Expr::Seq(vec![var("v")]),
                                    ]),
                                ),
                            ],
                        )),
                    )),
                    Box::new(Expr::Variant("none".into(), Box::new(Expr::UNIT))),
                    ValueType::Union(vec![
                        ("none".into(), ValueType::Tuple(vec![])),
                        ("some".into(), ValueType::U8),
                    ]),
                    Box::new(var("literal-length-distance-alphabet-code-lengths")),
                )),
            ),
            (
                "literal-length-alphabet-code-lengths-value",
                Format::Compute(Expr::SubSeq(
                    Box::new(var("literal-length-distance-alphabet-code-lengths-value")),
                    Box::new(Expr::U8(0)),
                    Box::new(add(Expr::AsU16(Box::new(var("hlit"))), Expr::U16(257))),
                )),
            ),
            (
                "distance-alphabet-code-lengths-value",
                Format::Compute(Expr::SubSeq(
                    Box::new(var("literal-length-distance-alphabet-code-lengths-value")),
                    Box::new(add(Expr::AsU16(Box::new(var("hlit"))), Expr::U16(257))),
                    Box::new(add(Expr::AsU16(Box::new(var("hdist"))), Expr::U16(1))),
                )),
            ),
            (
                "codes",
                Format::Dynamic(
                    "distance-alphabet-format".into(),
                    DynFormat::Huffman(var("distance-alphabet-code-lengths-value"), None),
                    Box::new(Format::Dynamic(
                        "literal-length-alphabet-format".into(),
                        DynFormat::Huffman(var("literal-length-alphabet-code-lengths-value"), None),
                        Box::new(repeat_until_last(
                            Expr::Lambda(
                                "x".into(),
                                Box::new(Expr::Eq(
                                    Box::new(Expr::AsU16(Box::new(Expr::RecordProj(
                                        Box::new(var("x")),
                                        "code".into(),
                                    )))),
                                    Box::new(Expr::U16(256)),
                                )),
                            ),
                            record([
                                (
                                    "code",
                                    Format::Apply("literal-length-alphabet-format".into()),
                                ),
                                (
                                    "extra",
                                    match_variant(
                                        var("code"),
                                        vec![
                                            (Pattern::U16(257), "some", length_record(3, base, 0)),
                                            (Pattern::U16(258), "some", length_record(4, base, 0)),
                                            (Pattern::U16(259), "some", length_record(5, base, 0)),
                                            (Pattern::U16(260), "some", length_record(6, base, 0)),
                                            (Pattern::U16(261), "some", length_record(7, base, 0)),
                                            (Pattern::U16(262), "some", length_record(8, base, 0)),
                                            (Pattern::U16(263), "some", length_record(9, base, 0)),
                                            (Pattern::U16(264), "some", length_record(10, base, 0)),
                                            (Pattern::U16(265), "some", length_record(11, base, 1)),
                                            (Pattern::U16(266), "some", length_record(13, base, 1)),
                                            (Pattern::U16(267), "some", length_record(15, base, 1)),
                                            (Pattern::U16(268), "some", length_record(17, base, 1)),
                                            (Pattern::U16(269), "some", length_record(19, base, 2)),
                                            (Pattern::U16(270), "some", length_record(23, base, 2)),
                                            (Pattern::U16(271), "some", length_record(27, base, 2)),
                                            (Pattern::U16(272), "some", length_record(31, base, 2)),
                                            (Pattern::U16(273), "some", length_record(35, base, 3)),
                                            (Pattern::U16(274), "some", length_record(43, base, 3)),
                                            (Pattern::U16(275), "some", length_record(51, base, 3)),
                                            (Pattern::U16(276), "some", length_record(59, base, 3)),
                                            (Pattern::U16(277), "some", length_record(67, base, 4)),
                                            (Pattern::U16(278), "some", length_record(83, base, 4)),
                                            (Pattern::U16(279), "some", length_record(99, base, 4)),
                                            (
                                                Pattern::U16(280),
                                                "some",
                                                length_record(115, base, 4),
                                            ),
                                            (
                                                Pattern::U16(281),
                                                "some",
                                                length_record(131, base, 5),
                                            ),
                                            (
                                                Pattern::U16(282),
                                                "some",
                                                length_record(163, base, 5),
                                            ),
                                            (
                                                Pattern::U16(283),
                                                "some",
                                                length_record(195, base, 5),
                                            ),
                                            (
                                                Pattern::U16(284),
                                                "some",
                                                length_record(227, base, 5),
                                            ),
                                            (
                                                Pattern::U16(285),
                                                "some",
                                                length_record(258, base, 0),
                                            ),
                                            (Pattern::Wildcard, "none", Format::EMPTY),
                                        ],
                                    ),
                                ),
                            ]),
                        )),
                    )),
                ),
            ),
            (
                "codes-values",
                Format::Compute(Expr::FlatMap(
                    Box::new(Expr::Lambda(
                        "x".into(),
                        Box::new(Expr::Match(
                            Box::new(Expr::RecordProj(Box::new(var("x")), "code".into())),
                            vec![
                                (Pattern::U16(256), Expr::Seq(vec![])),
                                (Pattern::U16(257), reference_record()),
                                (Pattern::U16(258), reference_record()),
                                (Pattern::U16(259), reference_record()),
                                (Pattern::U16(260), reference_record()),
                                (Pattern::U16(261), reference_record()),
                                (Pattern::U16(262), reference_record()),
                                (Pattern::U16(263), reference_record()),
                                (Pattern::U16(264), reference_record()),
                                (Pattern::U16(265), reference_record()),
                                (Pattern::U16(266), reference_record()),
                                (Pattern::U16(267), reference_record()),
                                (Pattern::U16(268), reference_record()),
                                (Pattern::U16(269), reference_record()),
                                (Pattern::U16(270), reference_record()),
                                (Pattern::U16(271), reference_record()),
                                (Pattern::U16(272), reference_record()),
                                (Pattern::U16(273), reference_record()),
                                (Pattern::U16(274), reference_record()),
                                (Pattern::U16(275), reference_record()),
                                (Pattern::U16(276), reference_record()),
                                (Pattern::U16(277), reference_record()),
                                (Pattern::U16(278), reference_record()),
                                (Pattern::U16(279), reference_record()),
                                (Pattern::U16(280), reference_record()),
                                (Pattern::U16(281), reference_record()),
                                (Pattern::U16(282), reference_record()),
                                (Pattern::U16(283), reference_record()),
                                (Pattern::U16(284), reference_record()),
                                (Pattern::U16(285), reference_record()),
                                (
                                    Pattern::Wildcard,
                                    Expr::Seq(vec![Expr::Variant(
                                        "literal".into(),
                                        Box::new(Expr::AsU8(Box::new(Expr::RecordProj(
                                            Box::new(var("x")),
                                            "code".into(),
                                        )))),
                                    )]),
                                ),
                            ],
                        )),
                    )),
                    Box::new(var("codes")),
                )),
            ),
        ]),
    );

    let block = module.define_format(
        "deflate.block",
        record([
            ("final", base.bit()),
            ("type", bits2.clone()),
            (
                "data",
                match_variant(
                    var("type"),
                    vec![
                        (Pattern::U8(0), "uncompressed", uncompressed.call()),
                        (Pattern::U8(1), "fixed_huffman", fixed_huffman.call()),
                        (Pattern::U8(2), "dynamic_huffman", dynamic_huffman.call()),
                    ],
                ),
            ),
        ]),
    );

    module.define_format(
        "deflate.main",
        record([
            (
                "blocks",
                repeat_until_last(
                    Expr::Lambda(
                        "x".into(),
                        Box::new(Expr::Eq(
                            Box::new(Expr::RecordProj(Box::new(var("x")), "final".into())),
                            Box::new(Expr::U8(1)),
                        )),
                    ),
                    block.call(),
                ),
            ),
            (
                "codes",
                Format::Compute(Expr::FlatMap(
                    Box::new(Expr::Lambda(
                        "x".into(),
                        Box::new(Expr::Match(
                            Box::new(Expr::RecordProj(Box::new(var("x")), "data".into())),
                            vec![
                                (
                                    Pattern::Variant(
                                        "uncompressed".into(),
                                        Box::new(Pattern::Binding("y".into())),
                                    ),
                                    Expr::RecordProj(Box::new(var("y")), "codes-values".into()),
                                ),
                                (
                                    Pattern::Variant(
                                        "fixed_huffman".into(),
                                        Box::new(Pattern::Binding("y".into())),
                                    ),
                                    Expr::RecordProj(Box::new(var("y")), "codes-values".into()),
                                ),
                                (
                                    Pattern::Variant(
                                        "dynamic_huffman".into(),
                                        Box::new(Pattern::Binding("y".into())),
                                    ),
                                    Expr::RecordProj(Box::new(var("y")), "codes-values".into()),
                                ),
                            ],
                        )),
                    )),
                    Box::new(var("blocks")),
                )),
            ),
            (
                "inflate",
                Format::Compute(Expr::Inflate(Box::new(var("codes")))),
            ),
        ]),
    )
}
