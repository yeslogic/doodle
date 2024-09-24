use doodle::{helper::*, Expr, Format, FormatModule, FormatRef};

use crate::format::BaseModule;

pub fn main(module: &mut FormatModule, base: &BaseModule, deflate: FormatRef) -> FormatRef {
    let method_and_flags = where_lambda(
        packed_bits_u8([4, 4], ["compression-info", "compression-method"]),
        "method-info",
        expr_eq(
            record_proj(var("method-info"), "compression-method"),
            Expr::U8(8),
        ),
    );

    // helper for checking whether a dictionary is present according to the flags
    let has_dict = |flags: Expr| is_nonzero_u8(record_proj(flags, "fdict"));

    module.define_format(
        "zlib.main",
        record([
            ("compression-method-flags", method_and_flags),
            // REVIEW - fcheck is chosen such that the first 16 bits of the zlib block as a u16be is 0 mod 31
            (
                "flags",
                packed_bits_u8([2, 1, 5], ["flevel", "fdict", "fcheck"]),
            ),
            // TODO - this should be a 'known' dictionary if it appears, but that is domain-specific and hard to get a handle on
            ("dict-id", cond_maybe(has_dict(var("flags")), base.u32be())),
            ("data", Format::Bits(Box::new(deflate.call()))),
            // NOTE - adler32 is supposed to be an Adler-32 checksum of the decompressed bytes
            ("adler32", base.u32be()),
        ]),
    )
}
