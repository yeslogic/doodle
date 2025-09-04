use doodle::{Format, FormatModule, FormatRef, Label, Pattern, helper::*};

/// TIFF Image file header
///
/// - [TIFF 6.0 Specification, Section 4.5](https://developer.adobe.com/content/dam/udp/en/open/standards/tiff/TIFF6.pdf#page=13)
/// - [Exif Version 2.32, Section 4.5.2](https://www.cipa.jp/std/documents/e/DC-X008-Translation-2019-E.pdf#page=23)
pub fn main(module: &mut FormatModule) -> FormatRef {
    let ifd_le = module.define_format("tiff.ifd.le", ifd(false));
    let ifd_be = module.define_format("tiff.ifd.be", ifd(true));

    let tiff_byte_order = module.define_format(
        "tiff.byte-order",
        alts([("le", is_bytes(b"II")), ("be", is_bytes(b"MM"))]),
    );

    let byte_order_type = module.get_format_type(tiff_byte_order.get_level());

    let ifd_variant = module.define_format_args(
        "tiff.ifd",
        vec![(Label::Borrowed("byte-order"), byte_order_type.clone())],
        Format::Match(
            Box::new(var("byte-order")),
            vec![
                (Pattern::variant("le", Pattern::Wildcard), ifd_le.call()),
                (Pattern::variant("be", Pattern::Wildcard), ifd_be.call()),
            ],
        ),
    );

    module.define_format(
        "tiff.main",
        record([
            ("start_of_header", pos32()),
            ("byte-order", tiff_byte_order.call()),
            (
                "magic",
                Format::Match(
                    Box::new(var("byte-order")),
                    vec![
                        (Pattern::variant("le", Pattern::Wildcard), u16le()), // 42
                        (Pattern::variant("be", Pattern::Wildcard), u16be()), // 42
                    ],
                ),
            ),
            (
                "offset",
                Format::Match(
                    Box::new(var("byte-order")),
                    vec![
                        (Pattern::variant("le", Pattern::Wildcard), u32le()),
                        (Pattern::variant("be", Pattern::Wildcard), u32be()),
                    ],
                ),
            ),
            (
                "ifd",
                with_relative_offset(
                    Some(var("start_of_header")),
                    var("offset"),
                    ifd_variant.call_args(vec![var("byte-order")]),
                ),
            ),
        ]),
    )
}

/// Image file directory sub-format

fn ifd(is_be: bool) -> Format {
    record([
        ("num-fields", if is_be { u16be() } else { u16le() }),
        ("fields", repeat_count(var("num-fields"), ifd_field(is_be))),
        ("next-ifd-offset", if is_be { u32be() } else { u32le() }),
        // TODO: Offset from start of the TIFF header (i.e. `offset + 2 + num-fields * 12`)
        // TODO: Recursive call to `ifd(is_be)`
        ("next-ifd", opaque_bytes()),
    ])
}

// Image file directory field sub-format
fn ifd_field(is_be: bool) -> Format {
    record([
        ("tag", if is_be { u16be() } else { u16le() }),
        ("type", if is_be { u16be() } else { u16le() }),
        ("length", if is_be { u32be() } else { u32le() }),
        ("offset-or-data", if is_be { u32be() } else { u32le() }),
        // TODO: Offset from start of the TIFF header for values longer than 4 bytes
    ])
}
