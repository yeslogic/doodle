use super::*;

/// DSIG Header format definiton
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/dsig
pub fn table(module: &mut FormatModule) -> FormatRef {
    let signature_record = signature_record(module);
    let flags = bit_fields_u16([
        // NOTE - spec is unclear about what the flag-bits other than bit 0 are actually for, and only specifies 1-7 as being reserved (set to 0)
        BitFieldKind::Reserved {
            bit_width: 8,
            check_zero: false,
        }, // Bits 8-15 : padding
        BitFieldKind::Reserved {
            bit_width: 7,
            check_zero: true,
        }, // Bits 1-7 : reserved
        BitFieldKind::FlagBit("cannot_be_resigned"), // Bit 0 : Cannot be resigned
    ]);
    module.define_format(
        "opentype.dsig.table",
        let_view(
            "table_view",
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                ("version", expect_within(u32be(), 0x0000_0001u32)), // version is 0x0000_0001
                ("num_signatures", u16be()),
                ("flags", flags),
                (
                    "signature_records",
                    repeat_count(
                        var("num_signatures"),
                        signature_record.invoke_views([vvar("table_view")]),
                    ),
                ),
            ]),
        ),
    )
}

fn signature_record(module: &mut FormatModule) -> DepFormat<0, 1> {
    let sig_format1 = sig_format1(module);
    module.register_format_views(
        "opentype.dsig.signature_record",
        [Label::Borrowed("_table_view")],
        record([
            ("format", u32be()),
            ("length", u32be()),
            (
                "signature_offset",
                read_phantom_view_offset32(
                    vvar("_table_view"),
                    fmt_match(
                        var("format"),
                        [(Pattern::U32(1), slice(var("length"), sig_format1.call()))],
                    ),
                ),
            ),
        ]),
    )
}

fn sig_format1(module: &mut FormatModule) -> FormatRef {
    module.define_format(
        "opentype.dsig.sig_format1",
        record_auto([
            ("__reserved1", expect_u16be(0)),
            ("__reserved2", expect_u16be(0)),
            ("signature_length", u32be()),
            (
                "signature",
                capture_bytes_from_here(var("signature_length")),
            ),
        ]),
    )
}
