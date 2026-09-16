use doodle::{Format, FormatModule, FormatRef, helper::*};
use doodle_numexpr_macro::numexpr;

/// Given a field-name holding a captured byte-sequence, attempts to decode it as non-empty-null
/// UTF-8 text, falling back to the raw bytes on failure. Shared tail logic for [`cstring`] and
/// [`string`], which differ only in how the byte-sequence itself is bounded (scan-to-null vs.
/// length-prefixed slice).
fn decode_utf8_nz_permit(bytes_field: &'static str, utf8_nz: FormatRef) -> Format {
    let utf8_nz_total = pseudo_record(
        [("text", utf8_nz.call()), ("__eof", Format::EndOfInput)],
        compute(var("text")),
    );
    permit(
        fmt_variant("valid", decode_bytes(var(bytes_field), utf8_nz_total)),
        variant("invalid", var(bytes_field)),
    )
}

fn cstring(module: &mut FormatModule, utf8_nz: FormatRef) -> FormatRef {
    module.define_format(
        "bson.cstring",
        pseudo_record(
            [("bytes", repeat(not_byte(0x00))), ("__null", is_byte(0x00))],
            decode_utf8_nz_permit("bytes", utf8_nz),
        ),
    )
}

/// BSON `string`: a length-prefixed UTF-8 string. The leading `int32` (`len`) is the byte-count of
/// the content *plus* the trailing null terminator, and is enforced as a hard boundary via `slice`
/// (an out-of-range or too-short buffer is a decode error), but full consumption within that slice
/// is *not* additionally asserted — an embedded `0x00` before the declared length silently truncates
/// the captured text early rather than hard-failing, matching the lenient style established for
/// `bool`.
fn string(module: &mut FormatModule, utf8_nz: FormatRef) -> FormatRef {
    module.define_format(
        "bson.string",
        chain(
            where_within_z(i32le(), 1i32..),
            "len",
            pseudo_record(
                [
                    (
                        "bytes",
                        slice(numeric(numexpr!("len" -u32 1)), repeat(not_byte(0x00))),
                    ),
                    ("__null", is_byte(0x00)),
                ],
                decode_utf8_nz_permit("bytes", utf8_nz),
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

/// Scaffolding for BSON double values
fn double() -> Format {
    // NOTE - we have no support for non-integer numbers in any layer, so we parse a 64-bit int we can later convert
    fmt_variant("F64", u64le())
}

/// BSON UTC datetime: a signed 64-bit integer of milliseconds since the Unix epoch, tagged
/// distinctly from a plain `int64` since it carries different semantics despite the same bits.
fn datetime() -> Format {
    fmt_variant("DateTime", i64le())
}

/// BSON `Timestamp`: a MongoDB-internal type distinct from UTC `datetime`, consisting of an
/// `increment` ordinal and `seconds`-since-epoch, each a little-endian `u32`. `increment` (the
/// least-significant 32 bits of the conceptual 64-bit value) precedes `seconds` in the byte stream.
fn timestamp() -> Format {
    record([("increment", u32le()), ("seconds", u32le())])
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

const BSON_TAG_DOUBLE: i8 = 0x01;
const BSON_TAG_STRING: i8 = 0x02;
const BSON_TAG_OBJECTID: i8 = 0x07;
const BSON_TAG_BOOL: i8 = 0x08;
const BSON_TAG_DATETIME: i8 = 0x09;
const BSON_TAG_NULL: i8 = 0x0A;
const BSON_TAG_INT32: i8 = 0x10;
const BSON_TAG_TIMESTAMP: i8 = 0x11;
const BSON_TAG_INT64: i8 = 0x12;
const BSON_TAG_MAXKEY: i8 = 0x7F;
const BSON_TAG_MINKEY: i8 = -1;

fn element(module: &mut FormatModule, cstring: FormatRef, string: FormatRef) -> FormatRef {
    let e_double = module.define_format(
        "bson.element.double",
        mk_element(BSON_TAG_DOUBLE, cstring, double()),
    );
    let e_string = module.define_format(
        "bson.element.string",
        mk_element(BSON_TAG_STRING, cstring, string.call()),
    );
    let e_objectid = module.define_format(
        "bson.element.objectid",
        mk_element(BSON_TAG_OBJECTID, cstring, objectid()),
    );
    let e_bool = module.define_format(
        "bson.element.bool",
        mk_element(BSON_TAG_BOOL, cstring, bool()),
    );
    let e_datetime = module.define_format(
        "bson.element.datetime",
        mk_element(BSON_TAG_DATETIME, cstring, datetime()),
    );
    let e_null = module.define_format(
        "bson.element.null",
        mk_element(BSON_TAG_NULL, cstring, Format::EMPTY),
    );
    let e_int32 = module.define_format(
        "bson.element.int32",
        mk_element(BSON_TAG_INT32, cstring, i32le()),
    );
    let e_timestamp = module.define_format(
        "bson.element.timestamp",
        mk_element(BSON_TAG_TIMESTAMP, cstring, timestamp()),
    );
    let e_int64 = module.define_format(
        "bson.element.int64",
        mk_element(BSON_TAG_INT64, cstring, i64le()),
    );
    let e_maxkey = module.define_format(
        "bson.element.maxkey",
        mk_element(BSON_TAG_MAXKEY, cstring, Format::EMPTY),
    );
    let e_minkey = module.define_format(
        "bson.element.minkey",
        mk_element(BSON_TAG_MINKEY, cstring, Format::EMPTY),
    );
    module.define_format(
        "bson.element",
        alts([
            ("double", e_double.call()),
            ("string", e_string.call()),
            ("objectid", e_objectid.call()),
            ("bool", e_bool.call()),
            ("datetime", e_datetime.call()),
            ("null", e_null.call()),
            ("int32", e_int32.call()),
            ("timestamp", e_timestamp.call()),
            ("int64", e_int64.call()),
            ("maxkey", e_maxkey.call()),
            ("minkey", e_minkey.call()),
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
    let string = string(module, utf8_nz);
    let element = element(module, cstring, string);
    let document = document(module, element);
    module.define_format("bson.main", record([("document", document.call())]))
}
