use doodle::DepFormat;

use super::*;

pub(crate) fn table(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
    let variation_axis_record = variation_axis_record(module, tag);
    let instance_record = instance_record(module);

    // First half of `fvar` table: fixed-size header
    let fvar_header = record_auto([
        ("major_version", util::expect_u16be(1)),
        ("minor_version", util::expect_u16be(0)),
        (
            "offset_axes",
            where_lambda(u16be(), "raw", is_nonzero_u16(var("raw"))),
        ),
        ("__reserved", util::expect_u16be(2)),
        ("axis_count", u16be()),
        ("axis_size", util::expect_u16be(20)), // For fvar version 1.0, axis record are fixed-size == 20 (0x0014) bytes
        ("instance_count", u16be()),
        ("instance_size", u16be()), // TODO[epic=validation] - not yet enforced, but should be axisCount * sizeOf(Fixed32Be) + (4 or 6)
    ]);
    // Second half of `fvar` table: offset-linked axes and instances
    let fvar_arrays = record_auto([
        (
            "_axes_length",
            compute(mul(var("axis_count"), var("axis_size"))),
        ),
        (
            "#_axes",
            // TODO - this becomes a lot easier if we use ViewFormats instead of offset-parse patterns
            // NOTE - because we delay interpretation of the offset above to collect additional fields, we inline and specialize offset16 based on the captured value
            phantom(parse_from_view(
                vvar("table_view").offset(var("offset_axes")),
                slice(
                    var("_axes_length"),
                    repeat_count(
                        var("axis_count"),
                        // because axis_size is fixed at 20 and variation_axis_record is a fixed-width (20 byte) parse, we don't need a slice here
                        variation_axis_record.call(),
                    ),
                ),
            )),
        ),
        (
            "offset_instances",
            compute(add(var("offset_axes"), var("_axes_length"))),
        ),
        (
            "#_instances",
            // NOTE - because we delay interpretation of the offset above to collect additional fields, we inline and specialize offset16 based on the captured value
            phantom(parse_from_view(
                vvar("table_view").offset(var("offset_instances")),
                repeat_count(
                    var("instance_count"),
                    slice(
                        var("instance_size"),
                        instance_record.invoke_args([var("axis_count"), var("instance_size")]),
                    ),
                ),
            )),
        ),
    ]);
    let scope_field = record([("table_scope", reify_view(vvar("table_view")))]);
    module.define_format(
        "opentype.fvar.table",
        let_view(
            "table_view",
            merge_records([scope_field, fvar_header, fvar_arrays]),
        ),
    )
}

/// InstanceRecord format implementation
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/fvar#instancerecord
///
/// Parametric over `axis_count :~ U16` and `instance_size :~ U16`.
fn instance_record(module: &mut FormatModule) -> DepFormat<2, 0> {
    let user_tuple = user_tuple(module);
    module.register_format_args(
        "opentype.fvar.instance_record",
        [
            (Label::Borrowed("axis_count"), ValueType::U16),
            (Label::Borrowed("instance_size"), ValueType::U16),
        ],
        record([
            ("subfamily_nameid", u16be()),
            ("flags", util::expect_u16be(0)), // reserved for future use, should be set to 0,
            ("coordinates", user_tuple.invoke_args([var("axis_count")])),
            (
                "postscript_nameid",
                cond_maybe(
                    // Only included if the extra 2 bytes are implied by `instance_size`, which is otherwise divisible by 4
                    expr_eq(rem(var("instance_size"), Expr::U16(4)), Expr::U16(2)),
                    u16be(),
                ),
            ),
        ]),
    )
}

/// UserTuple record (part of `InstanceRecord`)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/fvar#instancerecord
///
/// Parametric over `axis_count :~ U16`.
fn user_tuple(module: &mut FormatModule) -> DepFormat<1, 0> {
    module.register_format_args(
        "opentype.fvar.user_tuple",
        [(Label::Borrowed("axis_count"), ValueType::U16)],
        record([(
            "coordinates",
            repeat_count(var("axis_count"), util::fixed32be()),
        )]),
    )
}

fn variation_axis_record(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
    use BitFieldKind::*;
    let axis_qual_flags = bit_fields_u16([
        Reserved {
            bit_width: 15,
            check_zero: false,
        },
        FlagBit("hidden_axis"),
    ]);
    module.define_format(
        "opentype.fvar.variation_axis_record",
        record([
            ("axis_tag", tag.call()),             // 4 bytes
            ("min_value", util::fixed32be()),     // + 4 = 8 bytes
            ("default_value", util::fixed32be()), // + 4 = 12 bytes
            ("max_value", util::fixed32be()),     // +4 = 16 bytes
            ("flags", axis_qual_flags),           // + 2 = 18 bytes
            ("axis_name_id", u16be()),            // + 2 = 20 bytes
        ]),
    )
}
