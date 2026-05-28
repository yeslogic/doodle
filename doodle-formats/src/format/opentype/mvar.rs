use super::*;

pub(crate) fn table(
    module: &mut FormatModule,
    tag: FormatRef,
    item_variation_store: FormatRef,
) -> FormatRef {
    let value_record = value_record(module, tag);
    module.define_format(
        "opentype.mvar.table",
        let_view(
            "table_view",
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                ("major_version", expect_u16be(1)),
                ("minor_version", expect_u16be(0)),
                ("__reserved", expect_u16be(0)),
                ("value_record_size", expect_nonzero::<U16>(u16be())),
                ("value_record_count", u16be()),
                // NOTE - `value_record_count == 0` iff `item_variation_store.offset == 0`
                (
                    "item_variation_store",
                    util::read_phantom_view_offset16(
                        vvar("table_view"),
                        item_variation_store.call(),
                    ),
                ),
                (
                    "value_records",
                    repeat_count(var("value_record_count"), value_record.call()),
                ),
            ]),
        ),
    )
}

/// MVar value record format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/mvar#table-formats
fn value_record(module: &mut FormatModule, tag: FormatRef) -> FormatRef {
    // Only a select set of valueTags are explicitly defined in the spec: https://learn.microsoft.com/en-us/typography/opentype/spec/mvar#value-tags
    // Private tags are also permitted but are mandated to begin with an uppercase letter and contain only `[A-Z0-9]`
    module.define_format(
        "opentype.mvar.value_record",
        record_auto([
            ("value_tag", tag.call()),
            ("delta_set_outer_index", u16be()),
            ("delta_set_inner_index", u16be()),
        ]),
    )
}
