use super::*;

/// SVG document record format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/svg#svg-documents
///
/// The SVG document itself is not interpreted nor is a specific parser defined. It is
/// either GZIP-compressed or plaintext, and can be parsed as if arbitary UTF-8 encoded
/// text for the purposes of validation.
///
/// If gzip-compressed, the leading bytes will be `0x1f 0x8b 0x08`.
fn svg_document_record(module: &mut FormatModule, text_or_ztext: FormatRef) -> DepFormat<0, 1> {
    module.register_format_view(
        "opentype.svg.document_record",
        Label::Borrowed("list_view"),
        record_auto([
            ("start_glyph_id", u16be()),
            ("end_glyph_id", u16be()),
            ("svg_document_offset", expect_nonzero::<U32>(u32be())),
            ("svg_document_length", u32be()),
            // document is either plaintext or gzip-encoded, and the character encoding of the svg document (uncompressed) must be UTF-8
            (
                "#_svg_document",
                with_view(
                    vvar("list_view").offset(var("svg_document_offset")),
                    capture_bytes(var("svg_document_length")),
                ),
            ),
            (
                "#_svg_document_utf8",
                phantom(parse_from_view(
                    vvar("list_view").offset(var("svg_document_offset")),
                    text_or_ztext.call(),
                )),
            ),
        ]),
    )
}

fn svg_document_list(module: &mut FormatModule, text_or_ztext: FormatRef) -> FormatRef {
    let svg_document_record = svg_document_record(module, text_or_ztext);
    module.define_format(
        "opentype.svg.document_list",
        let_view(
            "list_view",
            record_auto([
                ("num_entries", expect_nonzero::<U16>(u16be())),
                (
                    "document_records",
                    repeat_count(
                        var("num_entries"),
                        svg_document_record.invoke_view(vvar("list_view")),
                    ),
                ),
            ]),
        ),
    )
}

pub(crate) fn table(module: &mut FormatModule, text_or_ztext: FormatRef) -> FormatRef {
    let svg_document_list = svg_document_list(module, text_or_ztext);
    module.define_format(
        "opentype.svg.table",
        let_view(
            "table_view",
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                ("version", expect_u16be(0)),
                (
                    "svg_document_list",
                    read_phantom_view_offset32(vvar("table_view"), svg_document_list.call()),
                ),
                ("__reserved", expect_eq(u32be(), poly_zero())),
            ]),
        ),
    )
}
