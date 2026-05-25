use super::*;

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    const NO_Z0: u16 = 1;
    const YES_Z0: u16 = 2;

    let maxp_version_1 = module.define_format(
        "opentype.maxp.version1",
        record([
            ("max_points", u16be()),
            ("max_contours", u16be()),
            ("max_composite_points", u16be()),
            ("max_composite_contours", u16be()),
            ("max_zones", where_between_u16(u16be(), NO_Z0, YES_Z0)),
            ("max_twilight_points", u16be()),
            ("max_storage", u16be()),
            ("max_function_defs", u16be()),
            ("max_instruction_defs", u16be()),
            ("max_stack_elements", u16be()),
            ("max_size_of_instructions", u16be()),
            ("max_component_elements", u16be()),
            ("max_component_depth", where_between_u16(u16be(), 0, 16)),
        ]),
    );

    module.define_format(
        "opentype.maxp.table",
        record([
            ("version", util::version16_16()),
            ("num_glyphs", u16be()),
            (
                "data",
                match_variant(
                    var("version"),
                    [
                        (Pattern::U32(0x0001_0000), "MaxpV1", maxp_version_1.call()),
                        (Pattern::U32(0x0000_5000), "MaxpPostScript", Format::EMPTY),
                        (bind("unknown"), "MaxpUnknown", compute(var("unknown"))), // FIXME - do we need this at all?
                    ],
                ),
            ),
        ]),
    )
}
