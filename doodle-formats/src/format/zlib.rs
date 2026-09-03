use doodle::{Expr, Format, FormatModule, FormatRef, helper::*};

pub fn main(module: &mut FormatModule, deflate: FormatRef) -> FormatRef {
    let method_and_flags = {
        use BitFieldKind::*;
        where_lambda(
            bit_fields_u8([
                BitsField {
                    field_name: "compression-info",
                    bit_width: 4,
                },
                BitsField {
                    field_name: "compression-method",
                    bit_width: 4,
                },
            ]),
            "method-info",
            expr_eq(
                record_proj(var("method-info"), "compression-method"),
                Expr::U8(8),
            ),
        )
    };

    // helper for checking whether a dictionary is present according to the flags
    let has_dict = |flags: Expr| record_proj(flags, "fdict");

    use BitFieldKind::*;

    module.define_format(
        "zlib.main",
        record([
            ("compression-method-flags", method_and_flags),
            // REVIEW - fcheck is chosen such that the first 16 bits of the zlib block as a u16be is 0 mod 31
            (
                "flags",
                bit_fields_u8([
                    BitsField {
                        bit_width: 2,
                        field_name: "flevel",
                    },
                    FlagBit("fdict"),
                    BitsField {
                        bit_width: 5,
                        field_name: "fcheck",
                    },
                ]),
            ),
            // TODO - this should be a 'known' dictionary if it appears, but that is domain-specific and hard to get a handle on
            ("dict-id", cond_maybe(has_dict(var("flags")), u32be())),
            ("data", Format::Bits(Box::new(deflate.call()))),
            // NOTE - adler32 is supposed to be an Adler-32 checksum of the decompressed bytes
            ("adler32", u32be()),
        ]),
    )
}
