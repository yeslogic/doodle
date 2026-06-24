use super::*;

/// HVAR Table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/hvar#table-formats
///
/// Only appears if `hmtx` is present, and like all variable-font tables,
/// requires `fvar` and `STAT` to be present.
pub(crate) fn table(
    module: &mut FormatModule,
    item_variation_store: FormatRef,
    delta_set_index_map: FormatRef,
) -> FormatRef {
    module.define_format(
        "opentype.hvar.table",
        let_view(
            "table_view",
            record_auto([
                ("table_scope", reify_view(vvar("table_view"))),
                ("major_version", expect_u16be(1)),
                ("minor_version", expect_u16be(0)),
                // NOTE - the HVAR specification implies that the following offset is non-NULL, but the OpenType spec includes the caveat that even not-explicitly NULLable offsets should be handled gracefully if null
                // REVIEW[epic=validation] - this IVS must contain sufficient delta-sets that the maximum index found in any DSIM entry is in-bounds, but we cannot check this at this layer.
                (
                    "item_variation_store",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        item_variation_store.call(),
                    ),
                ),
                // NOTE - each of the three following offset-fields are specified 'may be NULL', and that glyph ids are to be used directly as indices in place of a DSIM for any which are not provided
                (
                    "advance_width_mapping",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        delta_set_index_map.call(),
                    ),
                ),
                (
                    "lsb_mapping",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        delta_set_index_map.call(),
                    ),
                ),
                (
                    "rsb_mapping",
                    util::read_phantom_view_offset32(
                        vvar("table_view"),
                        delta_set_index_map.call(),
                    ),
                ),
            ]),
        ),
    )
}
