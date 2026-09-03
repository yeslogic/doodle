use super::*;

/// HDMX table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/hdmx
///
/// Parametric in `num_glyphs :~ U16` (taken from `maxp`)
pub(crate) fn table(module: &mut FormatModule) -> DepFormat<1, 0> {
    let device_record = device_record(module);
    // helper format for the `size_device_record` field, which is a u32be that should be divisible by 4 (32-bit alignment)
    let size32 = where_lambda(
        u32be(),
        "size",
        expr_eq(rem(var("size"), Expr::U32(4)), Expr::U32(0)),
    );
    module.register_format_args(
        "opentype.hdmx.table",
        [(Label::Borrowed("num_glyphs"), ValueType::U16)],
        record([
            ("version", expect_u16be(0)),   // table version, should be 0
            ("num_records", u16be()),       // number of device records
            ("size_device_record", size32), // should be 32-bit aligned
            (
                "records",
                repeat_count(
                    var("num_records"),
                    slice(
                        var("size_device_record"),
                        device_record.invoke_args([var("num_glyphs")]),
                    ),
                ),
            ),
        ]),
    )
}

/// HDMX Device Record format definition
///
/// Parametric in `num_glyphs :~ U16`
fn device_record(module: &mut FormatModule) -> DepFormat<1, 0> {
    module.register_format_args(
        "opentype.hdmx.device_record",
        [(Label::Borrowed("num_glyphs"), ValueType::U16)],
        record_auto([
            ("pixel_size", u8()),
            ("max_width", u8()),
            (
                "widths",
                // TODO - should this be capture_bytes instead?
                from_here(read_array(var("num_glyphs"), BaseKind::U8)),
            ),
            ("__pad", align_to_size::<u32>()),
        ]),
    )
}
