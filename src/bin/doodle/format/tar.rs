use doodle::{byte_set::ByteSet, Expr, Format, FormatModule, FormatRef};

use crate::format::base::*;

const BLOCK_SIZE: u32 = 512;

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
    let tar_asciiz = module.define_format(
        "tar.ascii-string",
        record([
            ("string", repeat(not_byte(0x00))),
            ("padding", repeat1(is_byte(0x00))),
        ]),
    );

    const MAGIC: &[u8; 6] = b"ustar\x00";

    // Terminating characters for various fields, which for reasons of backwards-compatibility, may either be the
    // ASCII space character or NUL
    let nul_or_wsp = module.define_format(
        "tar.padding-char",
        Format::Byte(ByteSet::from([0x00, b' '])),
    );

    // Octal-encoded numeric value (as of UStar) fitting in N bytes, terminated by the space character or NUL
    let o_numeric = |len: u16| {
        Format::Slice(
            Expr::U16(len),
            Box::new(record([
                ("string", repeat(base.ascii_octal_digit())),
                ("__nul_or_wsp", nul_or_wsp.call()),
                ("__padding", repeat(is_byte(0x00))),
            ])),
        )
    };

    let cstr_arr =
        |len: u16| -> Format { Format::Slice(Expr::U16(len), Box::new(tar_asciiz.call())) };
    let magic = is_bytes(MAGIC);
    let size_field = {
        let octal_digit = record([
            ("bit", base.ascii_octal_digit()),
            (
                "@value",
                Format::Compute(Expr::Sub(
                    Box::new(Expr::AsU8(Box::new(var("bit")))),
                    Box::new(Expr::U8(b'0')),
                )),
            ),
        ]);

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
            ("__nil", nul_or_wsp.call()),
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

    // USTAR allows `filename`, `linkname` and `prefix` to omit a trailing NUL if they fully occupy their respective array-fields
    // Therefore, we eagerly parse all non-NUL characters within an N-byte slice, and in doing so either terminate
    // by seeing an in-range NUL or running out of bytes to read (reaching the end of the sub-stream).
    // However, as unused bytes are further required to be zeroed out, we can be more rigorous and demand the next characters,
    // if any, after reaching the end of the first parse, are all NUL
    let tar_str_optz = module.define_format(
        "tar.ascii-string.opt0",
        record([
            ("string", repeat(not_byte(0x00))),
            ("__padding", repeat(is_byte(0x00))),
        ]),
    );

    let tar_str_optz_ne = module.define_format(
        "tar.ascii-string.opt0.nonempty",
        record([
            ("string", repeat1(not_byte(0x00))),
            ("__padding", repeat(is_byte(0x00))),
        ]),
    );

    let filename = Format::Slice(Expr::U16(100), Box::new(tar_str_optz_ne.call()));

    let linkname = Format::Slice(Expr::U16(100), Box::new(tar_str_optz.call()));

    let prefix = Format::Slice(Expr::U16(155), Box::new(tar_str_optz.call()));

    // This specification is only guaranteed to work for UStar archives. Anything else might work
    // by happenstance but may fail just as easily.
    let header = module.define_format(
        "tar.header",
        Format::Slice(
            Expr::U32(BLOCK_SIZE),
            Box::new(record([
                ("name", filename),              // bytes 0 - 99
                ("mode", o_numeric(8)),          // bytes 100 - 107
                ("uid", o_numeric(8)),           // bytes 108 - 115
                ("gid", o_numeric(8)),           // bytes 116 - 123
                ("size", size_field),            // bytes 124 - 135
                ("mtime", o_numeric(12)),        // bytes 136 - 147
                ("chksum", o_numeric(8)),        // bytes 148 - 155
                ("typeflag", base.ascii_char()), // byte 156
                ("linkname", linkname),          // bytes 157 - 256
                ("magic", magic),                // bytes 257 - 262
                ("version", is_bytes(b"00")),    // bytes 263 - 264
                ("uname", cstr_arr(32)),         // bytes 265 - 296
                ("gname", cstr_arr(32)),         // bytes 297 - 328
                ("devmajor", o_numeric(8)),      // bytes 329 - 336
                ("devminor", o_numeric(8)),      // bytes 337 - 344
                ("prefix", prefix),              // bytes 345 - 499
                ("pad", repeat_byte(12, 0x00)),  // bytes 500 - 511
            ])),
        ),
    );

    let header_with_data = module.define_format(
        "tar.header_with_data",
        record([
            ("header", header.call()),
            (
                "file",
                record([
                    (
                        "@value",
                        repeat_count(
                            Expr::RecordProj(Box::new(var("header")), "size".into()),
                            base.u8(),
                        ),
                    ),
                    ("__padding", Format::Align(512)),
                ]),
            ),
        ]),
    );

    module.define_format(
        "tar.main",
        record([
            ("contents", repeat1(header_with_data.call())),
            (
                "__padding",
                repeat_count(Expr::U32(2 * BLOCK_SIZE), is_byte(0x00)),
            ),
            ("__trailing", repeat(is_byte(0x00))),
        ]),
    )
}
