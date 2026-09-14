use doodle::{Format, FormatModule, FormatRef, helper::*};
use doodle_numexpr_macro::numexpr;

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

/// Internal helper for BSON boolean values. A literal byte of `0` is mapped to `false` and a value
/// of 1 (or greater) is mapped to `true`.
///
/// Per the specification, `0x01` is the only proper byte value for `true`; other non-zero values
/// are still mapped to `true`, but a soft-validation using [Expect](doodle::validation::Severity::Expect)
/// may result in a warning being logged while parsing.
fn bool() -> Format {
    map(
        expect_between_u8(u8(), 0, 1),
        lambda("b", is_nonzero_u8(var("b"))),
    )
}

/// Internal helper for BSON object IDs
fn objectid() -> Format {
    record([
        ("timestamp", u32be()),
        ("randval", seq_repeat(5, Format::ANY_BYTE)),
        ("counter", u24be()),
    ])
}

fn mk_element(tag: i8, cstring: FormatRef, content: Format) -> Format {
    record_auto([
        ("__tag", is_byte(tag as u8)),
        ("key", cstring.call()),
        ("value", content),
    ])
}

const BSON_TAG_OBJECTID: i8 = 0x07;
const BSON_TAG_BOOL: i8 = 0x08;
const BSON_TAG_NULL: i8 = 0x0A;
const BSON_TAG_INT32: i8 = 0x10;
const BSON_TAG_MAXKEY: i8 = 0x7F;
const BSON_TAG_MINKEY: i8 = -1;

fn element(module: &mut FormatModule, cstring: FormatRef) -> FormatRef {
    let e_null = module.define_format(
        "bson.element.null",
        mk_element(BSON_TAG_NULL, cstring, Format::EMPTY),
    );
    let e_minkey = module.define_format(
        "bson.element.minkey",
        mk_element(BSON_TAG_MINKEY, cstring, Format::EMPTY),
    );
    let e_maxkey = module.define_format(
        "bson.element.maxkey",
        mk_element(BSON_TAG_MAXKEY, cstring, Format::EMPTY),
    );
    let e_bool = module.define_format(
        "bson.element.bool",
        mk_element(BSON_TAG_BOOL, cstring, bool()),
    );
    let e_int32 = module.define_format(
        "bson.element.int32",
        mk_element(BSON_TAG_INT32, cstring, i32le()),
    );
    let e_objectid = module.define_format(
        "bson.element.objectid",
        mk_element(BSON_TAG_OBJECTID, cstring, objectid()),
    );
    module.define_format(
        "bson.element",
        alts([
            ("null", e_null.call()),
            ("minkey", e_minkey.call()),
            ("maxkey", e_maxkey.call()),
            ("bool", e_bool.call()),
            ("int32", e_int32.call()),
            ("objectid", e_objectid.call()),
        ]),
    )
}

fn document(module: &mut FormatModule, element: FormatRef) -> FormatRef {
    module.define_format(
        "bson.document",
        chain(
            // NOTE - guard against underflow (len < 4)
            where_within_z(i32le(), 4i32..),
            "len",
            slice(
                numeric(numexpr!("len" -u32 4)),
                pseudo_record(
                    [
                        ("elements", repeat(element.call())),
                        ("__null", is_byte(0x00)),
                    ],
                    compute(var("elements")),
                ),
            ),
        ),
    )
}

pub fn main(module: &mut FormatModule, utf8_nz: FormatRef) -> FormatRef {
    let cstring = cstring(module, utf8_nz);
    let element = element(module, cstring);
    let document = document(module, element);
    module.define_format("bson.main", record([("document", document.call())]))
}
