use doodle::{Format, FormatModule, FormatRef, helper::*};

fn cstring(module: &mut FormatModule, utf8_nz: FormatRef) -> FormatRef {
    let utf8_nz_total = pseudo_record(
        [("text", utf8_nz.call()), ("__eof", Format::EndOfInput)],
        compute(var("text")),
    );
    module.define_format(
        "bson.cstring",
        pseudo_record(
            [("bytes", repeat(not_byte(0x00))), ("__null", is_byte(0x00))],
            permit(
                fmt_variant("valid", decode_bytes(var("bytes"), utf8_nz_total)),
                variant("invalid", var("bytes")),
            ),
        ),
    )
}

pub fn main(module: &mut FormatModule, utf8_nz: FormatRef) -> FormatRef {
    let cstring = cstring(module, utf8_nz);
    module.define_format("bson", alts([("cstring", cstring.call())]))
}
