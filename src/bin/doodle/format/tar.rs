use doodle::{byte_set::ByteSet, Expr, Format, FormatModule, FormatRef};

use crate::format::base::*;

const BLOCK_SIZE: u32 = 512;

const OCTAL: [u8; 8] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7'];

fn shl(value: Expr, places: Expr) -> Expr {
    Expr::Shl(Box::new(value), Box::new(places))
}

fn bitor(x: Expr, y: Expr) -> Expr {
    Expr::BitOr(Box::new(x), Box::new(y))
}

// octal pair to u32 numeric evalue
fn o2u32(hi: Expr, lo: Expr) -> Expr {
    let hi32 = shl(Expr::AsU32(Box::new(hi)), Expr::U32(3));
    let lo32 = Expr::AsU32(Box::new(lo));
    bitor(hi32, lo32)
}

// octal quartet to u32 numeric value
fn o4u32(hh: Expr, hl: Expr, lh: Expr, ll: Expr) -> Expr {
    let hi32 = shl(o2u32(hh, hl), Expr::U32(6));
    let lo32 = o2u32(lh, ll);
    bitor(hi32, lo32)
}

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    // A format for C-style `char[N]` fields for static `N`, representing
    // CString values. All unused bytes following the terminal NUL must also be NUL
    let cstr_arr = |len: u16| -> Format {
        Format::Slice(
            Expr::U16(len),
            Box::new(tuple([base.asciiz_string(), repeat(is_byte(0x00))])),
        )
    };

    // A format for uninterpreted N-byte values
    let cbytes =
        |len: u16| -> Format { repeat_count(Expr::U16(len), Format::Byte(ByteSet::full())) };

    // USTAR allows `filename`, `linkname` and `prefix` to omit a trailing NUL if they fully occupy their respective array-fields
    // Therefore, we eagerly parse all non-NUL characters within an N-byte slice, and in doing so either terminate
    // by seeing an in-range NUL or running out of bytes to read (reaching the end of the sub-stream).
    // However, as unused bytes are further required to be zeroed out, we can be more rigorous and demand the next characters,
    // if any, after reaching the end of the first parse, are all NUL
    let cstr_arr_opt0 = |len: u16| -> Format {
        Format::Slice(
            Expr::U16(len),
            Box::new(record([
                ("data", repeat(not_byte(0x00))),
                ("padding", repeat(is_byte(0x00))),
            ])),
        )
    };

    const MAGIC: &[u8; 6] = b"ustar\x00";

    let magic = is_bytes(MAGIC);
    let size_field = {
        let octal = Format::Byte(ByteSet::from(OCTAL));

        let octal_digit = record([
            ("bit", octal),
            (
                "@value",
                Format::Compute(Expr::Sub(
                    Box::new(Expr::AsU8(Box::new(var("bit")))),
                    Box::new(Expr::U8(b'0')),
                )),
            ),
        ]);

        let nul_or_wsp = Format::Byte(ByteSet::from([0x00, b' ']));

        record([
            ("oA", octal_digit.clone()),
            ("o9", octal_digit.clone()),
            ("o8", octal_digit.clone()),
            ("o7", octal_digit.clone()),
            ("o6", octal_digit.clone()),
            ("o5", octal_digit.clone()),
            ("o4", octal_digit.clone()),
            ("o3", octal_digit.clone()),
            ("o2", octal_digit.clone()),
            ("o1", octal_digit.clone()),
            ("o0", octal_digit.clone()),
            ("nil", nul_or_wsp),
            (
                "@value",
                Format::Compute(bitor(
                    shl(
                        o4u32(Expr::U8(0), var("oA"), var("o9"), var("o8")),
                        Expr::U32(24),
                    ),
                    bitor(
                        shl(
                            o4u32(var("o7"), var("o6"), var("o5"), var("o4")),
                            Expr::U32(12),
                        ),
                        o4u32(var("o3"), var("o2"), var("o1"), var("o0")),
                    ),
                )),
            ),
        ])
    };

    let header = module.define_format(
        "tar.header",
        tuple([
            Format::Peek(Box::new(not_byte(0x00))),
            Format::FixedSlice(
                BLOCK_SIZE as usize,
                Box::new(record([
                    ("name", cstr_arr_opt0(100)),                       // bytes 0 - 99
                    ("mode", cbytes(8)),                                // bytes 100 - 107
                    ("uid", cbytes(8)),                                 // bytes 108 - 115
                    ("gid", cbytes(8)),                                 // bytes 116 - 123
                    ("size", size_field),                               // bytes 124 - 135
                    ("mtime", cbytes(12)),                              // bytes 136 - 147
                    ("chksum", cstr_arr(8)),                            // bytes 148 - 155
                    ("typeflag", base.u8()),                            // byte 156
                    ("linkname", cstr_arr_opt0(100)),                   // bytes 157 - 256
                    ("magic", magic),                                   // bytes 257 - 262
                    ("version", tuple([is_byte(b'0'), is_byte(b'0')])), // bytes 263 - 264
                    ("uname", cstr_arr(32)),                            // bytes 265 - 296
                    ("gname", cstr_arr(32)),                            // bytes 297 - 328
                    ("devmajor", cbytes(8)),                            // bytes 329 - 336
                    ("devminor", cbytes(8)),                            // bytes 337 - 344
                    ("prefix", cstr_arr_opt0(155)),                     // bytes 345 - 500
                    (
                        "@padding",
                        repeat_count(Expr::U16(12), Format::Byte(ByteSet::full())),
                    ),
                ])),
            ),
        ]),
    );

    let full_block = repeat_count(Expr::U32(BLOCK_SIZE), Format::Byte(ByteSet::full()));
    let partial_block = |nbytes: Expr| {
        Format::FixedSlice(
            BLOCK_SIZE as usize,
            Box::new(tuple([
                repeat_count(nbytes, Format::Byte(ByteSet::full())),
                repeat(is_byte(0x00)),
            ])),
        )
    };

    let header_with_data = module.define_format(
        "tar.header_with_data",
        record([
            ("header", header.call()),
            (
                "file",
                tuple([
                    repeat_count(
                        Expr::Div(
                            Box::new(Expr::RecordProj(
                                Box::new(Expr::TupleProj(Box::new(var("header")), 1)),
                                "size".to_string(),
                            )),
                            Box::new(Expr::U32(BLOCK_SIZE)),
                        ),
                        full_block,
                    ),
                    partial_block(Expr::Rem(
                        Box::new(Expr::RecordProj(
                            Box::new(Expr::TupleProj(Box::new(var("header")), 1)),
                            "size".to_string(),
                        )),
                        Box::new(Expr::U32(BLOCK_SIZE)),
                    )),
                ]),
            ),
        ]),
    );

    module.define_format(
        "tar.main",
        record([
            ("contents", repeat1(header_with_data.call())),
            (
                "padding",
                repeat_count(Expr::U32(2 * BLOCK_SIZE), is_byte(0x00)),
            ),
            ("trailing", repeat(is_byte(0x00))),
        ]),
    )
}