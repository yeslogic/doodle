use crate::format::BaseModule;
use doodle::{bounds::Bounds, helper::*};

use doodle::{BaseType, DynFormat, Expr, Format, FormatModule, FormatRef, Pattern, ValueType};

fn shl_u8(x: Expr, r: u8) -> Expr {
    if r == 0 {
        x
    } else {
        shl(x, Expr::U8(r))
    }
}

fn shl_u16(x: Expr, r: u16) -> Expr {
    let y = as_u16(x);
    if r == 0 {
        y
    } else {
        shl(y, Expr::U16(r))
    }
}

fn cast<N>(x: usize) -> N
where
    N: TryFrom<usize>,
    N::Error: std::fmt::Debug,
{
    N::try_from(x).unwrap()
}

/// Maps an `Expr::Tuple` consisting of `n` bit-values into a `u8`-typed Expr
/// where the first tuple position is the LSB and the last tuple
/// position is the MSB.
///
/// This is suitable for 'number'-kinded values injected into a deflate bitstream.
fn bits_value_u8(name: &'static str, n: usize) -> Expr {
    if n == 0 {
        return Expr::U8(0);
    } else if n == 1 {
        return tuple_proj(var(name), 0);
    }

    // initialize a flat array of AST nodes (simple expressions)
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(shl_u8(tuple_proj(var(name), i), cast(i)));
    }
    // construct a balanced binary tree of bitor operations
    balanced_bitor(nodes)
}

/// Maps an `Expr::Tuple` consisting of 5 bit-values into the `u8`-typed Expr
/// where the first tuple position is the MSB and the last tuple position is the LSB.
///
/// This is specifically engineered for translating bit distance prefix-codes into their 5-bit symbols from 0-29 (30 and 31 being reserved)
/// in fixed Huffman (TYPE==1) blocks.
///
/// A tuple whose arity is not 5 will be mishandled by this function given its specialization.
fn dist_value_u8(x: &'static str) -> Expr {
    let b4 = shl_u8(seq_proj(var(x), 0), 4);
    let b3 = shl_u8(seq_proj(var(x), 1), 3);
    let b2 = shl_u8(seq_proj(var(x), 2), 2);
    let b1 = shl_u8(seq_proj(var(x), 3), 1);
    let b0 = seq_proj(var(x), 4);
    bit_or(bit_or(b4, b3), bit_or(bit_or(b2, b1), b0))
}

/// Maps an `Expr::Tuple` consisting of `n` bit-values into a `u16`-typed Expr
/// where the first tuple position is the LSB and the last tuple
/// position is the MSB.
///
/// This is suitable for 'number'-kinded values injected into a deflate bitstream.
fn bits_value_u16(name: &'static str, n: usize) -> Expr {
    if n == 0 {
        return Expr::U16(0);
    } else if n == 1 {
        return as_u16(tuple_proj(var(name), 0));
    }

    // initialize a flat array of AST nodes (simple expressions)
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        nodes.push(shl_u16(tuple_proj(var(name), i), cast(i)));
    }

    // construct a balanced binary tree of bitor operations
    balanced_bitor(nodes)
}

/// Parse a 5-bit Fixed Huffman distance-code and map it into its corresponding distance-symbol
fn dist8(base: &BaseModule) -> Format {
    map(
        repeat_count(Expr::U32(5), base.bit()),
        lambda("bits", dist_value_u8("bits")),
    )
}

/// Parses `n` bits and maps them into a `u8`-typed Value according to the
/// standard LSB-to-MSB write order for numeric values in the DEFLATE specification.
fn bits8(n: usize, base: &BaseModule) -> Format {
    if n == 0 {
        return compute(Expr::U8(0));
    }

    map(
        tuple_repeat(n, base.bit()),
        lambda("bits", bits_value_u8("bits", n)),
    )
}

/// Parses `n` bits and maps them into a `u16`-typed Value according to the
/// standard LSB-to-MSB write order for numeric values in the DEFLATE specification.
fn bits16(n: usize, base: &BaseModule) -> Format {
    if n == 0 {
        return compute(Expr::U16(0));
    }

    map(
        tuple_repeat(n, base.bit()),
        lambda("bits", bits_value_u16("bits", n)),
    )
}

fn length_record(
    start: usize,
    base: &BaseModule,
    extra_bits: usize,
    distance_record: FormatRef,
) -> Format {
    record([
        ("length-extra-bits", bits8(extra_bits, base)),
        (
            "length",
            compute(add(
                Expr::U16(start as u16),
                as_u16(var("length-extra-bits")),
            )),
        ),
        (
            "distance-code",
            Format::Apply("distance-alphabet-format".into()),
        ),
        (
            "distance-record",
            distance_record.call_args(vec![var("distance-code")]),
        ),
    ])
}

fn length_record_fixed(
    start: usize,
    base: &BaseModule,
    extra_bits: usize,
    distance_record: FormatRef,
) -> Format {
    record([
        ("length-extra-bits", bits8(extra_bits, base)),
        (
            "length",
            compute(add(
                Expr::U16(start as u16),
                as_u16(var("length-extra-bits")),
            )),
        ),
        ("distance-code", dist8(base)),
        (
            "distance-record",
            distance_record.call_args(vec![as_u16(var("distance-code"))]),
        ),
    ])
}

fn reference_record() -> Expr {
    expr_match(
        record_proj(var("x"), "extra"),
        vec![(
            pat_some(Pattern::binding("rec")),
            Expr::Seq(vec![variant(
                "reference",
                Expr::Record(vec![
                    ("length".into(), record_proj(var("rec"), "length")),
                    (
                        "distance".into(),
                        record_proj(record_proj(var("rec"), "distance-record"), "distance"),
                    ),
                ]),
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

    let distance_record0 = module.define_format_args(
        "deflate.distance-record0",
        vec![
            ("extra-bits".into(), ValueType::Base(BaseType::U8)),
            ("start".into(), ValueType::Base(BaseType::U16)),
        ],
        record([
            (
                "distance-extra-bits",
                Format::Match(
                    Box::new(var("extra-bits")),
                    vec![
                        (Pattern::U8(0), bits16(0, base)),
                        (Pattern::U8(1), bits16(1, base)),
                        (Pattern::U8(2), bits16(2, base)),
                        (Pattern::U8(3), bits16(3, base)),
                        (Pattern::U8(4), bits16(4, base)),
                        (Pattern::U8(5), bits16(5, base)),
                        (Pattern::U8(6), bits16(6, base)),
                        (Pattern::U8(7), bits16(7, base)),
                        (Pattern::U8(8), bits16(8, base)),
                        (Pattern::U8(9), bits16(9, base)),
                        (Pattern::U8(10), bits16(10, base)),
                        (Pattern::U8(11), bits16(11, base)),
                        (Pattern::U8(12), bits16(12, base)),
                        (Pattern::U8(13), bits16(13, base)),
                    ],
                ),
            ),
            (
                "distance",
                compute(add(var("start"), var("distance-extra-bits"))),
            ),
        ]),
    );

    let distance_record = module.define_format_args(
        "deflate.distance-record",
        vec![("distance-code".into(), ValueType::Base(BaseType::U16))],
        Format::Match(
            Box::new(as_u8(var("distance-code"))),
            vec![
                (
                    Pattern::U8(0),
                    distance_record0.call_args(vec![Expr::U8(0), Expr::U16(1)]),
                ),
                (
                    Pattern::U8(1),
                    distance_record0.call_args(vec![Expr::U8(0), Expr::U16(2)]),
                ),
                (
                    Pattern::U8(2),
                    distance_record0.call_args(vec![Expr::U8(0), Expr::U16(3)]),
                ),
                (
                    Pattern::U8(3),
                    distance_record0.call_args(vec![Expr::U8(0), Expr::U16(4)]),
                ),
                (
                    Pattern::U8(4),
                    distance_record0.call_args(vec![Expr::U8(1), Expr::U16(5)]),
                ),
                (
                    Pattern::U8(5),
                    distance_record0.call_args(vec![Expr::U8(1), Expr::U16(7)]),
                ),
                (
                    Pattern::U8(6),
                    distance_record0.call_args(vec![Expr::U8(2), Expr::U16(9)]),
                ),
                (
                    Pattern::U8(7),
                    distance_record0.call_args(vec![Expr::U8(2), Expr::U16(13)]),
                ),
                (
                    Pattern::U8(8),
                    distance_record0.call_args(vec![Expr::U8(3), Expr::U16(17)]),
                ),
                (
                    Pattern::U8(9),
                    distance_record0.call_args(vec![Expr::U8(3), Expr::U16(25)]),
                ),
                (
                    Pattern::U8(10),
                    distance_record0.call_args(vec![Expr::U8(4), Expr::U16(33)]),
                ),
                (
                    Pattern::U8(11),
                    distance_record0.call_args(vec![Expr::U8(4), Expr::U16(49)]),
                ),
                (
                    Pattern::U8(12),
                    distance_record0.call_args(vec![Expr::U8(5), Expr::U16(65)]),
                ),
                (
                    Pattern::U8(13),
                    distance_record0.call_args(vec![Expr::U8(5), Expr::U16(97)]),
                ),
                (
                    Pattern::U8(14),
                    distance_record0.call_args(vec![Expr::U8(6), Expr::U16(129)]),
                ),
                (
                    Pattern::U8(15),
                    distance_record0.call_args(vec![Expr::U8(6), Expr::U16(193)]),
                ),
                (
                    Pattern::U8(16),
                    distance_record0.call_args(vec![Expr::U8(7), Expr::U16(257)]),
                ),
                (
                    Pattern::U8(17),
                    distance_record0.call_args(vec![Expr::U8(7), Expr::U16(385)]),
                ),
                (
                    Pattern::U8(18),
                    distance_record0.call_args(vec![Expr::U8(8), Expr::U16(513)]),
                ),
                (
                    Pattern::U8(19),
                    distance_record0.call_args(vec![Expr::U8(8), Expr::U16(769)]),
                ),
                (
                    Pattern::U8(20),
                    distance_record0.call_args(vec![Expr::U8(9), Expr::U16(1025)]),
                ),
                (
                    Pattern::U8(21),
                    distance_record0.call_args(vec![Expr::U8(9), Expr::U16(1537)]),
                ),
                (
                    Pattern::U8(22),
                    distance_record0.call_args(vec![Expr::U8(10), Expr::U16(2049)]),
                ),
                (
                    Pattern::U8(23),
                    distance_record0.call_args(vec![Expr::U8(10), Expr::U16(3073)]),
                ),
                (
                    Pattern::U8(24),
                    distance_record0.call_args(vec![Expr::U8(11), Expr::U16(4097)]),
                ),
                (
                    Pattern::U8(25),
                    distance_record0.call_args(vec![Expr::U8(11), Expr::U16(6145)]),
                ),
                (
                    Pattern::U8(26),
                    distance_record0.call_args(vec![Expr::U8(12), Expr::U16(8193)]),
                ),
                (
                    Pattern::U8(27),
                    distance_record0.call_args(vec![Expr::U8(12), Expr::U16(12289)]),
                ),
                (
                    Pattern::U8(28),
                    distance_record0.call_args(vec![Expr::U8(13), Expr::U16(16385)]),
                ),
                (
                    Pattern::U8(29),
                    distance_record0.call_args(vec![Expr::U8(13), Expr::U16(24577)]),
                ),
                (Pattern::Int(Bounds::new(30, 31)), Format::Fail), // 30 and 31 are reserved symbols in the DEFLATE distance code alphabet
            ],
        ),
    );

    let uncompressed = module.define_format(
        "deflate.uncompressed",
        record_auto([
            ("__align", Format::Align(8)),
            ("len", bits16(16, base)),
            ("nlen", bits16(16, base)),
            ("bytes", repeat_count(var("len"), bits8(8, base))),
            (
                "codes-values",
                compute(flat_map(
                    lambda("x", Expr::Seq(vec![variant("literal", var("x"))])),
                    var("bytes"),
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
                    DynFormat::Huffman(Box::new(fixed_code_lengths()), None),
                    Box::new(repeat_until_last(
                        lambda(
                            "x",
                            expr_eq(as_u16(record_proj(var("x"), "code")), Expr::U16(256)),
                        ),
                        record([
                            ("code", Format::Apply("format".into())),
                            (
                                "extra",
                                Format::Match(
                                    Box::new(var("code")),
                                    vec![
                                        (
                                            Pattern::U16(257),
                                            fmt_some(length_record_fixed(
                                                3,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(258),
                                            fmt_some(length_record_fixed(
                                                4,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(259),
                                            fmt_some(length_record_fixed(
                                                5,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(260),
                                            fmt_some(length_record_fixed(
                                                6,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(261),
                                            fmt_some(length_record_fixed(
                                                7,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(262),
                                            fmt_some(length_record_fixed(
                                                8,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(263),
                                            fmt_some(length_record_fixed(
                                                9,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(264),
                                            fmt_some(length_record_fixed(
                                                10,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(265),
                                            fmt_some(length_record_fixed(
                                                11,
                                                base,
                                                1,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(266),
                                            fmt_some(length_record_fixed(
                                                13,
                                                base,
                                                1,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(267),
                                            fmt_some(length_record_fixed(
                                                15,
                                                base,
                                                1,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(268),
                                            fmt_some(length_record_fixed(
                                                17,
                                                base,
                                                1,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(269),
                                            fmt_some(length_record_fixed(
                                                19,
                                                base,
                                                2,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(270),
                                            fmt_some(length_record_fixed(
                                                23,
                                                base,
                                                2,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(271),
                                            fmt_some(length_record_fixed(
                                                27,
                                                base,
                                                2,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(272),
                                            fmt_some(length_record_fixed(
                                                31,
                                                base,
                                                2,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(273),
                                            fmt_some(length_record_fixed(
                                                35,
                                                base,
                                                3,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(274),
                                            fmt_some(length_record_fixed(
                                                43,
                                                base,
                                                3,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(275),
                                            fmt_some(length_record_fixed(
                                                51,
                                                base,
                                                3,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(276),
                                            fmt_some(length_record_fixed(
                                                59,
                                                base,
                                                3,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(277),
                                            fmt_some(length_record_fixed(
                                                67,
                                                base,
                                                4,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(278),
                                            fmt_some(length_record_fixed(
                                                83,
                                                base,
                                                4,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(279),
                                            fmt_some(length_record_fixed(
                                                99,
                                                base,
                                                4,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(280),
                                            fmt_some(length_record_fixed(
                                                115,
                                                base,
                                                4,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(281),
                                            fmt_some(length_record_fixed(
                                                131,
                                                base,
                                                5,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(282),
                                            fmt_some(length_record_fixed(
                                                163,
                                                base,
                                                5,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(283),
                                            fmt_some(length_record_fixed(
                                                195,
                                                base,
                                                5,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(284),
                                            fmt_some(length_record_fixed(
                                                227,
                                                base,
                                                5,
                                                distance_record,
                                            )),
                                        ),
                                        (
                                            Pattern::U16(285),
                                            fmt_some(length_record_fixed(
                                                258,
                                                base,
                                                0,
                                                distance_record,
                                            )),
                                        ),
                                        // REVIEW - consider whether we want to use Format::Fail instead
                                        (Pattern::Int(Bounds::new(286, 287)), fmt_none()),
                                        (Pattern::Wildcard, fmt_none()),
                                    ],
                                ),
                            ),
                        ]),
                    )),
                ),
            ),
            (
                "codes-values",
                compute(flat_map(
                    lambda(
                        "x",
                        expr_match(
                            record_proj(var("x"), "code"),
                            vec![
                                (Pattern::U16(256), Expr::Seq(vec![])),
                                (Pattern::Int(Bounds::new(257, 285)), reference_record()),
                                (Pattern::Int(Bounds::new(286, 287)), Expr::Seq(vec![])), // 286, 287 are illegal but we don't want to fail completely
                                (
                                    Pattern::Wildcard,
                                    Expr::Seq(vec![variant(
                                        "literal",
                                        as_u8(record_proj(var("x"), "code")),
                                    )]),
                                ),
                            ],
                        ),
                    ),
                    var("codes"),
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
                        Box::new(var("code-length-alphabet-code-lengths")),
                        Some(Box::new(Expr::Seq(vec![
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
                        ]))),
                    ),
                    Box::new(repeat_until_seq(
                        lambda(
                            "y",
                            expr_gte(
                                seq_length(flat_map_accum(
                                    lambda_tuple(
                                        ["last-symbol", "cl-code-extra"],
                                        expr_match(
                                            as_u8(record_proj(var("cl-code-extra"), "code")),
                                            vec![
                                                (
                                                    Pattern::U8(16),
                                                    Expr::Tuple(vec![
                                                        var("last-symbol"),
                                                        dup(
                                                            as_u32(add(
                                                                record_proj(
                                                                    var("cl-code-extra"),
                                                                    "extra",
                                                                ),
                                                                Expr::U8(3),
                                                            )),
                                                            expr_unwrap(var("last-symbol")),
                                                        ),
                                                    ]),
                                                ),
                                                (
                                                    Pattern::U8(17),
                                                    Expr::Tuple(vec![
                                                        expr_some(Expr::U8(0)),
                                                        dup(
                                                            as_u32(add(
                                                                record_proj(
                                                                    var("cl-code-extra"),
                                                                    "extra",
                                                                ),
                                                                Expr::U8(3),
                                                            )),
                                                            Expr::U8(0),
                                                        ),
                                                    ]),
                                                ),
                                                (
                                                    Pattern::U8(18),
                                                    Expr::Tuple(vec![
                                                        expr_some(Expr::U8(0)),
                                                        dup(
                                                            as_u32(add(
                                                                record_proj(
                                                                    var("cl-code-extra"),
                                                                    "extra",
                                                                ),
                                                                Expr::U8(11),
                                                            )),
                                                            Expr::U8(0),
                                                        ),
                                                    ]),
                                                ),
                                                (
                                                    Pattern::binding("v"),
                                                    Expr::Tuple(vec![
                                                        expr_some(var("v")),
                                                        Expr::Seq(vec![var("v")]),
                                                    ]),
                                                ),
                                            ],
                                        ),
                                    ),
                                    expr_none(),
                                    ValueType::Option(Box::new(ValueType::Base(BaseType::U8))),
                                    var("y"),
                                )),
                                add(as_u32(add(var("hlit"), var("hdist"))), Expr::U32(258)),
                            ),
                        ),
                        record([
                            ("code", Format::Apply("code-length-alphabet-format".into())),
                            (
                                "extra",
                                Format::Match(
                                    Box::new(as_u8(var("code"))),
                                    vec![
                                        (Pattern::U8(16), bits2.clone()),
                                        (Pattern::U8(17), bits3.clone()),
                                        (Pattern::U8(18), bits7.clone()),
                                        (Pattern::Wildcard, compute(Expr::U8(0))),
                                    ],
                                ),
                            ),
                        ]),
                    )),
                ),
            ),
            (
                "literal-length-distance-alphabet-code-lengths-value",
                compute(flat_map_accum(
                    lambda_tuple(
                        ["last-symbol", "cl-code-extra"],
                        expr_match(
                            as_u8(record_proj(var("cl-code-extra"), "code")),
                            vec![
                                (
                                    Pattern::U8(16),
                                    Expr::Tuple(vec![
                                        var("last-symbol"),
                                        dup(
                                            as_u32(add(
                                                record_proj(var("cl-code-extra"), "extra"),
                                                Expr::U8(3),
                                            )),
                                            expr_unwrap(var("last-symbol")),
                                        ),
                                    ]),
                                ),
                                (
                                    Pattern::U8(17),
                                    Expr::Tuple(vec![
                                        expr_some(Expr::U8(0)),
                                        dup(
                                            as_u32(add(
                                                record_proj(var("cl-code-extra"), "extra"),
                                                Expr::U8(3),
                                            )),
                                            Expr::U8(0),
                                        ),
                                    ]),
                                ),
                                (
                                    Pattern::U8(18),
                                    Expr::Tuple(vec![
                                        expr_some(Expr::U8(0)),
                                        dup(
                                            as_u32(add(
                                                record_proj(var("cl-code-extra"), "extra"),
                                                Expr::U8(11),
                                            )),
                                            Expr::U8(0),
                                        ),
                                    ]),
                                ),
                                (
                                    Pattern::binding("v"),
                                    Expr::Tuple(vec![
                                        expr_some(var("v")),
                                        Expr::Seq(vec![var("v")]),
                                    ]),
                                ),
                            ],
                        ),
                    ),
                    expr_none(),
                    ValueType::Option(Box::new(ValueType::Base(BaseType::U8))),
                    var("literal-length-distance-alphabet-code-lengths"),
                )),
            ),
            (
                "literal-length-alphabet-code-lengths-value",
                compute(sub_seq(
                    var("literal-length-distance-alphabet-code-lengths-value"),
                    Expr::U32(0),
                    add(as_u32(var("hlit")), Expr::U32(257)),
                )),
            ),
            (
                "distance-alphabet-code-lengths-value",
                compute(sub_seq(
                    var("literal-length-distance-alphabet-code-lengths-value"),
                    add(as_u32(var("hlit")), Expr::U32(257)),
                    add(as_u32(var("hdist")), Expr::U32(1)),
                )),
            ),
            (
                "codes",
                Format::Dynamic(
                    "distance-alphabet-format".into(),
                    DynFormat::Huffman(Box::new(var("distance-alphabet-code-lengths-value")), None),
                    Box::new(Format::Dynamic(
                        "literal-length-alphabet-format".into(),
                        DynFormat::Huffman(
                            Box::new(var("literal-length-alphabet-code-lengths-value")),
                            None,
                        ),
                        Box::new(repeat_until_last(
                            lambda(
                                "x",
                                expr_eq(as_u16(record_proj(var("x"), "code")), Expr::U16(256)),
                            ),
                            record([
                                (
                                    "code",
                                    Format::Apply("literal-length-alphabet-format".into()),
                                ),
                                (
                                    "extra",
                                    Format::Match(
                                        Box::new(var("code")),
                                        vec![
                                            (
                                                Pattern::U16(257),
                                                fmt_some(length_record(
                                                    3,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(258),
                                                fmt_some(length_record(
                                                    4,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(259),
                                                fmt_some(length_record(
                                                    5,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(260),
                                                fmt_some(length_record(
                                                    6,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(261),
                                                fmt_some(length_record(
                                                    7,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(262),
                                                fmt_some(length_record(
                                                    8,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(263),
                                                fmt_some(length_record(
                                                    9,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(264),
                                                fmt_some(length_record(
                                                    10,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(265),
                                                fmt_some(length_record(
                                                    11,
                                                    base,
                                                    1,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(266),
                                                fmt_some(length_record(
                                                    13,
                                                    base,
                                                    1,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(267),
                                                fmt_some(length_record(
                                                    15,
                                                    base,
                                                    1,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(268),
                                                fmt_some(length_record(
                                                    17,
                                                    base,
                                                    1,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(269),
                                                fmt_some(length_record(
                                                    19,
                                                    base,
                                                    2,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(270),
                                                fmt_some(length_record(
                                                    23,
                                                    base,
                                                    2,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(271),
                                                fmt_some(length_record(
                                                    27,
                                                    base,
                                                    2,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(272),
                                                fmt_some(length_record(
                                                    31,
                                                    base,
                                                    2,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(273),
                                                fmt_some(length_record(
                                                    35,
                                                    base,
                                                    3,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(274),
                                                fmt_some(length_record(
                                                    43,
                                                    base,
                                                    3,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(275),
                                                fmt_some(length_record(
                                                    51,
                                                    base,
                                                    3,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(276),
                                                fmt_some(length_record(
                                                    59,
                                                    base,
                                                    3,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(277),
                                                fmt_some(length_record(
                                                    67,
                                                    base,
                                                    4,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(278),
                                                fmt_some(length_record(
                                                    83,
                                                    base,
                                                    4,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(279),
                                                fmt_some(length_record(
                                                    99,
                                                    base,
                                                    4,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(280),
                                                fmt_some(length_record(
                                                    115,
                                                    base,
                                                    4,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(281),
                                                fmt_some(length_record(
                                                    131,
                                                    base,
                                                    5,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(282),
                                                fmt_some(length_record(
                                                    163,
                                                    base,
                                                    5,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(283),
                                                fmt_some(length_record(
                                                    195,
                                                    base,
                                                    5,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(284),
                                                fmt_some(length_record(
                                                    227,
                                                    base,
                                                    5,
                                                    distance_record,
                                                )),
                                            ),
                                            (
                                                Pattern::U16(285),
                                                fmt_some(length_record(
                                                    258,
                                                    base,
                                                    0,
                                                    distance_record,
                                                )),
                                            ),
                                            // NOTE - currently no difference in behavior compared to wildcard pattern, but adds specificity/clarity, and gives us an easy hook to treat as erroneous
                                            (Pattern::Int(Bounds::new(286, 287)), fmt_none()),
                                            (Pattern::Wildcard, fmt_none()),
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
                compute(flat_map(
                    lambda(
                        "x",
                        expr_match(
                            record_proj(var("x"), "code"),
                            vec![
                                (Pattern::U16(256), Expr::Seq(vec![])),
                                (Pattern::Int(Bounds::new(257, 285)), reference_record()),
                                // NOTE - whether or not we might see 286 or 287, we should elide them for code-value interpretation purposes
                                (Pattern::Int(Bounds::new(286, 287)), Expr::Seq(vec![])),
                                (
                                    Pattern::Wildcard,
                                    Expr::Seq(vec![variant(
                                        "literal",
                                        as_u8(record_proj(var("x"), "code")),
                                    )]),
                                ),
                            ],
                        ),
                    ),
                    var("codes"),
                )),
            ),
        ]),
    );

    let block = module.define_format(
        "deflate.block",
        record([
            ("final", base.bit()),
            ("type", bits2.clone()),
            ("data", {
                Format::Match(
                    Box::new(var("type")),
                    vec![
                        (
                            Pattern::U8(0),
                            fmt_variant("uncompressed", uncompressed.call()),
                        ),
                        (
                            Pattern::U8(1),
                            fmt_variant("fixed_huffman", fixed_huffman.call()),
                        ),
                        (
                            Pattern::U8(2),
                            fmt_variant("dynamic_huffman", dynamic_huffman.call()),
                        ),
                        // (Pattern::U8(3), Format::Fail), // Reserved - never constructed due to Fail, so no variant needed
                    ],
                )
            }),
        ]),
    );

    module.define_format(
        "deflate.main",
        record([
            (
                "blocks",
                repeat_until_last(
                    lambda("x", expr_eq(record_proj(var("x"), "final"), Expr::U8(1))),
                    block.call(),
                ),
            ),
            (
                "codes",
                compute(flat_map(
                    lambda(
                        "x",
                        expr_match(
                            record_proj(var("x"), "data"),
                            vec![
                                (
                                    Pattern::variant("uncompressed", Pattern::binding("y")),
                                    record_proj(var("y"), "codes-values"),
                                ),
                                (
                                    Pattern::variant("fixed_huffman", Pattern::binding("y")),
                                    record_proj(var("y"), "codes-values"),
                                ),
                                (
                                    Pattern::variant("dynamic_huffman", Pattern::binding("y")),
                                    record_proj(var("y"), "codes-values"),
                                ),
                            ],
                        ),
                    ),
                    var("blocks"),
                )),
            ),
            (
                "inflate",
                compute(flat_map_list(
                    lambda_tuple(
                        ["buffer", "symbol"],
                        expr_match(
                            var("symbol"),
                            vec![
                                (
                                    Pattern::variant("literal", Pattern::binding("b")),
                                    Expr::Seq(vec![var("b")]),
                                ),
                                (
                                    Pattern::variant("reference", Pattern::binding("r")),
                                    sub_seq_inflate(
                                        var("buffer"),
                                        sub(
                                            seq_length(var("buffer")),
                                            as_u32(record_proj(var("r"), "distance")),
                                        ),
                                        as_u32(record_proj(var("r"), "length")),
                                    ),
                                ),
                            ],
                        ),
                    ),
                    ValueType::Base(BaseType::U8),
                    var("codes"),
                )),
            ),
        ]),
    )
}
