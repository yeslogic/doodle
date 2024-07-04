use crate::format::BaseModule;
use doodle::helper::*;
use doodle::{Format, FormatModule, FormatRef};

/// gzip
pub fn main(module: &mut FormatModule, deflate: FormatRef, base: &BaseModule) -> FormatRef {
    // NOTE: Packed bits
    //
    // [|7 6 5 4 3 2 1 0|]
    //   ^ ^ ^ | | | | |   reserved
    //         ^ | | | |   FCOMMENT
    //           ^ | | |   FNAME
    //             ^ | |   FEXTRA
    //               ^ |   FHCRC
    //                 ^   FTEXT
    let flg = packed_bits_u8(
        [3, 1, 1, 1, 1, 1],
        [
            "__reserved",
            "fcomment",
            "fname",
            "fextra",
            "fhcrc",
            "ftext",
        ],
    );

    let header = module.define_format(
        "gzip.header",
        record([
            ("magic", is_bytes(b"\x1F\x8B")),
            ("method", base.u8()),
            ("file-flags", flg),
            ("timestamp", base.u32le()),
            ("compression-flags", base.u8()),
            ("os-id", base.u8()),
        ]),
    );

    let footer = module.define_format(
        "gzip.footer",
        record([("crc", base.u32le()), ("length", base.u32le())]),
    );

    let fname_flag = is_nonzero_u8(record_projs(var("header"), &["file-flags", "fname"]));
    let fname = module.define_format("gzip.fname", base.asciiz_string());

    let fextra_flag = is_nonzero_u8(record_projs(var("header"), &["file-flags", "fextra"]));
    let fextra_subfield = module.define_format(
        "gzip.fextra.subfield",
        record([
            ("si1", base.ascii_char()),
            ("si2", base.ascii_char()),
            ("len", base.u16le()),
            ("data", repeat_count(var("len"), base.u8())),
        ]),
    );
    let fextra = module.define_format(
        "gzip.fextra",
        record([
            ("xlen", base.u16le()),
            (
                "subfields",
                Format::Slice(var("xlen"), Box::new(repeat(fextra_subfield.call()))),
            ),
        ]),
    );

    module.define_format(
        "gzip.main",
        repeat1(record([
            ("header", header.call()),
            (
                "fextra",
                if_then_else_variant(fextra_flag, fextra.call(), Format::EMPTY),
            ),
            (
                "fname",
                if_then_else_variant(fname_flag, fname.call(), Format::EMPTY),
            ),
            // FIXME fcomment
            // FIXME fhcrc
            ("data", Format::Bits(Box::new(deflate.call()))),
            ("footer", footer.call()),
        ])),
    )
}
