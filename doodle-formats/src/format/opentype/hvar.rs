use super::*;

/// HVAR Table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/hvar#table-formats
///
/// Only appears if `hmtx` is present, and like all variable-font tables,
/// requires `fvar` and `STAT` to be present.
pub(crate) fn table(module: &mut FormatModule, item_variation_store: FormatRef) -> FormatRef {
    let dsim = delta_set_index_map(module);
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
                    util::read_phantom_view_offset32(vvar("table_view"), dsim.call()),
                ),
                (
                    "lsb_mapping",
                    util::read_phantom_view_offset32(vvar("table_view"), dsim.call()),
                ),
                (
                    "rsb_mapping",
                    util::read_phantom_view_offset32(vvar("table_view"), dsim.call()),
                ),
            ]),
        ),
    )
}

/// DeltaSetIndexMap format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#associating-target-items-to-variation-data
// TODO: extract to common format across variations
fn delta_set_index_map(module: &mut FormatModule) -> FormatRef {
    let entry_format = module.define_format(
        "opentype.var.dsim-entry_format",
        bit_fields_u8([
            // 0xC0 - reserved  (Set to 0)
            BitFieldKind::Reserved {
                bit_width: 2,
                check_zero: true,
            },
            // 2 bits - <size in bytes of map entry> = mapEntrySize + 1
            BitFieldKind::BitsField {
                bit_width: 2,
                field_name: "map_entry_size",
            },
            // 4 bits - <number of bits for each entry in inner-level index> = innerIndexBitCount + 1
            BitFieldKind::BitsField {
                bit_width: 4,
                field_name: "inner_index_bit_count",
            },
        ]),
    );
    module.define_format(
        "opentype.var.delta_set_index_map",
        record_auto([
            ("format", expect_between_u8(u8(), 0, 1)),
            ("_entry_format", entry_format.call()),
            (
                "entry_size",
                compute(succ(record_proj(var("_entry_format"), "map_entry_size"))),
            ),
            (
                "inner_index_bits",
                compute(succ(record_proj(
                    var("_entry_format"),
                    "inner_index_bit_count",
                ))),
            ),
            (
                "map_count",
                fmt_match(
                    var("format"),
                    [
                        (
                            Pattern::U8(0),
                            map(u16be(), lambda("val", as_u32(var("val")))),
                        ),
                        (Pattern::U8(1), u32be()),
                    ],
                ),
            ),
            // REVIEW - the current strategy is to merely capture the map data and leave it as an uninterpreted byte-array; any processing can be done downstream of generated code
            /*
             * MapData: captured as an uninterpreted byte-data array
             *
             * Each logical entry occupies `entry_size` (1-4) bytes, and is to be subsequently interpreted as an `(outerIx, innerIx)` pair:
             *   innerIx is stored within the N least-significant-bits of the entry (N = `inner_index_bits`)
             *   outerIx is stored within the M most-significant-bits of the entry (M = `entry_size * 8 - inner_index_bits`)
             *
             * Hence:
             *   <entry> = (<outerIx> << inner_index_bits) || <innerIx>
             */
            (
                "map_data",
                capture_bytes_from_here(mul(var("map_count"), as_u32(var("entry_size")))),
            ),
        ]),
    )
}
