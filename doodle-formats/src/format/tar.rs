use crate::format::BaseModule;
use doodle::helper::*;
use doodle::{Expr, Format, FormatModule, FormatRef, byte_set::ByteSet};

const BLOCK_SIZE: u32 = 512;

// octal pair to u32 numeric evalue
fn o2u32(hi: Expr, lo: Expr) -> Expr {
    let hi32 = shl(as_u32(hi), Expr::U32(3));
    let lo32 = as_u32(lo);
    bit_or(hi32, lo32)
}

// octal quartet to u32 numeric value
fn o4u32(hh: Expr, hl: Expr, lh: Expr, ll: Expr) -> Expr {
    let hi32 = shl(o2u32(hh, hl), Expr::U32(6));
    let lo32 = o2u32(lh, ll);
    bit_or(hi32, lo32)
}

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    // A format for C-style `char[N]` fields for static `N`, representing
    // CString values. All unused bytes following the terminal NUL must also be NUL
    let tar_asciiz = module.define_format(
        "tar.ascii-string",
        record_auto([
            ("string", mk_ascii_string(repeat(not_byte(0x00)))),
            ("__padding", repeat1(is_byte(0x00))),
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
        slice(
            Expr::U16(len),
            record_auto([
                ("string", repeat(base.ascii_octal_digit())),
                ("__nul_or_wsp", nul_or_wsp.call()),
                ("__padding", repeat(is_byte(0x00))),
            ]),
        )
    };

    let cstr_arr = |len: u16| -> Format { slice(Expr::U16(len), tar_asciiz.call()) };
    let magic = is_bytes(MAGIC);
    let size_field = {
        let octal_digit = map(
            base.ascii_octal_digit(),
            lambda("bit", sub(as_u8(var("bit")), Expr::U8(b'0'))),
        );

        map(
            // REVIEW - since record_auto is already a monadic chain, we might consider a non-record final-value alternative with similar syntax
            record_auto([
                ("_oA", octal_digit.clone()),
                ("_o9", octal_digit.clone()),
                ("_o8", octal_digit.clone()),
                ("_o7", octal_digit.clone()),
                ("_o6", octal_digit.clone()),
                ("_o5", octal_digit.clone()),
                ("_o4", octal_digit.clone()),
                ("_o3", octal_digit.clone()),
                ("_o2", octal_digit.clone()),
                ("_o1", octal_digit.clone()),
                ("_o0", octal_digit.clone()),
                ("__nil", nul_or_wsp.call()),
                (
                    "value",
                    compute(bit_or(
                        shl(
                            o4u32(Expr::U8(0), var("_oA"), var("_o9"), var("_o8")),
                            Expr::U32(24),
                        ),
                        bit_or(
                            shl(
                                o4u32(var("_o7"), var("_o6"), var("_o5"), var("_o4")),
                                Expr::U32(12),
                            ),
                            o4u32(var("_o3"), var("_o2"), var("_o1"), var("_o0")),
                        ),
                    )),
                ),
            ]),
            lambda("rec", Expr::record_proj(var("rec"), "value")),
        )
    };

    // USTAR allows `filename`, `linkname` and `prefix` to omit a trailing NUL if they fully occupy their respective array-fields
    // Therefore, we eagerly parse all non-NUL characters within an N-byte slice, and in doing so either terminate
    // by seeing an in-range NUL or running out of bytes to read (reaching the end of the sub-stream).
    // However, as unused bytes are further required to be zeroed out, we can be more rigorous and demand the next characters,
    // if any, after reaching the end of the first parse, are all NUL
    let tar_str_optz = module.define_format(
        "tar.ascii-string.opt0",
        record_auto([
            ("string", mk_ascii_string(repeat(not_byte(0x00)))),
            ("__padding", repeat(is_byte(0x00))),
        ]),
    );

    let tar_str_optz_ne = module.define_format(
        "tar.ascii-string.opt0.nonempty",
        record_auto([
            ("string", mk_ascii_string(repeat1(not_byte(0x00)))),
            ("__padding", repeat(is_byte(0x00))),
        ]),
    );

    let filename = slice(Expr::U16(100), tar_str_optz_ne.call());

    let linkname = slice(Expr::U16(100), tar_str_optz.call());

    let prefix = slice(Expr::U16(155), tar_str_optz.call());

    // This specification is only guaranteed to work for UStar archives. Anything else might work
    // by happenstance but may fail just as easily.
    let header = module.define_format(
        "tar.header",
        slice(
            Expr::U32(BLOCK_SIZE),
            record([
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
            ]),
        ),
    );

    let header_with_data = module.define_format(
        "tar.header_with_data",
        record_auto([
            ("header", header.call()),
            (
                "file",
                repeat_count(record_proj(var("header"), "size"), u8()),
            ),
            ("__padding", Format::Align(512)),
        ]),
    );

    module.define_format(
        "tar.main",
        record_auto([
            ("contents", repeat1(header_with_data.call())),
            (
                "__padding",
                repeat_count(Expr::U32(2 * BLOCK_SIZE), is_byte(0x00)),
            ),
            ("__trailing", repeat(is_byte(0x00))),
        ]),
    )
}
