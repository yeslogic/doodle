use doodle::{Expr, helper::*};
use doodle::{Format, FormatModule, FormatRef};

/// gzip
pub fn main(module: &mut FormatModule, deflate: FormatRef) -> FormatRef {
    // NOTE: Packed bits
    //   0 0 0 x x x x x
    // [|7 6 5 4 3 2 1 0|]
    //   ^ ^ ^ | | | | |   reserved [MUST all be zero cf. RFC 1952]
    //         ^ | | | |   FCOMMENT
    //           ^ | | |   FNAME
    //             ^ | |   FEXTRA
    //               ^ |   FHCRC
    //                 ^   FTEXT
    let file_flags = {
        use BitFieldKind::*;
        module.define_format(
            "gzip.header.file-flags",
            bit_fields_u8([
                Reserved {
                    bit_width: 3,
                    check_zero: true,
                },
                FlagBit("fcomment"),
                FlagBit("fname"),
                FlagBit("fextra"),
                FlagBit("fhcrc"),
                FlagBit("ftext"),
            ]),
        )
    };

    let header = module.define_format(
        "gzip.header",
        record([
            ("magic", is_bytes(b"\x1F\x8B")),
            ("method", u8()),
            ("file-flags", file_flags.call()),
            ("timestamp", u32le()),
            ("compression-flags", u8()),
            ("os-id", u8()),
        ]),
    );

    let footer = module.define_format(
        "gzip.footer",
        record([("crc", u32le()), ("length", u32le())]),
    );

    let cond_fname = {
        let fname = module.define_format("gzip.fname", asciiz_string());
        move |header: Expr| {
            let has_fname = record_lens(header, &["file-flags", "fname"]);
            cond_maybe(has_fname, fname.call())
        }
    };

    let cond_fextra = {
        let fextra_subfield = module.define_format(
            "gzip.fextra.subfield",
            record([
                ("si1", ascii_char()),
                ("si2", ascii_char()),
                ("len", u16le()),
                ("data", repeat_count(var("len"), u8())),
            ]),
        );
        let fextra = module.define_format(
            "gzip.fextra",
            record([
                ("xlen", u16le()),
                (
                    "subfields",
                    slice(var("xlen"), repeat(fextra_subfield.call())),
                ),
            ]),
        );
        move |header: Expr| {
            let has_fextra = record_lens(header, &["file-flags", "fextra"]);
            cond_maybe(has_fextra, fextra.call())
        }
    };

    let cond_fcomment = {
        let fcomment = module.define_format(
            "gzip.fcomment",
            record([
                // NOTE - The actual string-contents is Latin-1 but asciiz seems to work (for now)
                ("comment", asciiz_string()),
            ]),
        );
        move |header: Expr| {
            let has_comment = record_lens(header, &["file-flags", "fcomment"]);
            cond_maybe(has_comment, fcomment.call())
        }
    };

    let cond_fhcrc = {
        let fhcrc = module.define_format(
            "gzip.fhcrc",
            record([
                ("crc", u16le()), // two least significant bytes of CRC32 of all prior bytes in the header
            ]),
        );
        move |header: Expr| {
            let has_crc = record_lens(header, &["file-flags", "fhcrc"]);
            cond_maybe(has_crc, fhcrc.call())
        }
    };

    // TODO - consider registering as a module-level definition
    let gzip_single = record([
        ("header", header.call()),
        ("fextra", cond_fextra(var("header"))),
        ("fname", cond_fname(var("header"))),
        ("fcomment", cond_fcomment(var("header"))),
        ("fhcrc", cond_fhcrc(var("header"))),
        ("data", Format::Bits(Box::new(deflate.call()))),
        ("footer", footer.call()),
    ]);

    module.define_format("gzip.main", repeat1(gzip_single))
}
