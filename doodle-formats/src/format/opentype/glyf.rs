use super::*;
use doodle::numeric::BasicUnaryOp;

pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let simple_glyf_table = simple::table(module);
    let composite_glyf_table = composite::table(module);
    let glyf_description = glyf_description(module, simple_glyf_table, composite_glyf_table);
    let glyf_entry = glyf_entry(module, glyf_description);

    let offsets_type = {
        let mk_branch = |elem_t: ValueType| ValueType::Seq(Box::new(elem_t));
        let mut branches = std::collections::BTreeMap::new();
        // NOTE - at this layer, the u16-valued offsets are still half-value
        branches.insert(Label::Borrowed("Offsets16"), mk_branch(ValueType::U16));
        branches.insert(Label::Borrowed("Offsets32"), mk_branch(ValueType::U32));
        ValueType::Union(branches)
    };

    module.define_format_args(
        "opentype.glyf.table",
        vec![(Label::Borrowed("offsets"), offsets_type)],
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                (
                    "glyphs",
                    Format::Match(
                        Box::new(var("offsets")),
                        vec![
                            (
                                Pattern::Variant(
                                    Label::Borrowed("Offsets16"),
                                    Box::new(bind("half16s")),
                                ),
                                for_each_pair(
                                    var("half16s"),
                                    (scale2, scale2),
                                    ["this_offs", "next_offs"],
                                    table_entry(
                                        vvar("table_view"),
                                        var("this_offs"),
                                        var("next_offs"),
                                        glyf_entry,
                                    ),
                                ),
                            ),
                            (
                                Pattern::Variant(
                                    Label::Borrowed("Offsets32"),
                                    Box::new(bind("off32s")),
                                ),
                                for_each_pair(
                                    var("off32s"),
                                    (id, id),
                                    ["this_offs", "next_offs"],
                                    table_entry(
                                        vvar("table_view"),
                                        var("this_offs"),
                                        var("next_offs"),
                                        glyf_entry,
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            ]),
        ),
    )
}

mod simple {
    use super::*;

    pub(crate) fn flags_raw(module: &mut FormatModule) -> FormatRef {
        use BitFieldKind::*;

        const SHOULD_CHECK_ZERO: bool = false;
        module.define_format(
            "opentype.glyph-description.simple.flags-raw",
            bit_fields_u8([
                Reserved {
                    bit_width: 1,
                    check_zero: SHOULD_CHECK_ZERO,
                },
                FlagBit("overlap_simple"),
                FlagBit("y_is_same_or_positive_y_short_vector"),
                FlagBit("x_is_same_or_positive_x_short_vector"),
                FlagBit("repeat_flag"),
                FlagBit("y_short_vector"),
                FlagBit("x_short_vector"),
                FlagBit("on_curve_point"),
            ]),
        )
    }

    pub(crate) fn flags(simple_flags_raw: FormatRef, num_coordinates: Expr) -> Format {
        // Format that parses a flag-entry into its conditionally-parsed repetition-count and relevant, reordered fields
        let flag_list_entry = chain(
            simple_flags_raw.call(),
            "flags",
            record([
                // NOTE - indicates number of additional repeats, base value 0 for singleton or N for run of N+1 overall
                (
                    "repeats",
                    if_then_else(
                        record_proj(var("flags"), "repeat_flag"),
                        u8(),
                        compute(Expr::U8(0)),
                    ),
                ),
                (
                    "field_set",
                    compute(subset_fields(
                        var("flags"),
                        [
                            "on_curve_point",
                            "x_short_vector",
                            "y_short_vector",
                            "x_is_same_or_positive_x_short_vector",
                            "y_is_same_or_positive_y_short_vector",
                            "overlap_simple",
                        ],
                    )),
                ),
            ]),
        );
        // Lambda that tells us whether we are done reading flags
        let is_finished =
            lambda_tuple(["totlen", "_seq"], expr_gte(var("totlen"), num_coordinates));
        let update_totlen = lambda_tuple(
            ["acc", "flags"],
            add(
                var("acc"),
                succ(as_u16(record_proj(var("flags"), "repeats"))),
            ),
        );
        // Format that parses the flags as a packed (unexpanded repeats) array
        let raw_flags = map(
            accum_until(
                is_finished,
                update_totlen,
                Expr::U16(0),
                ValueType::U16,
                flag_list_entry,
            ),
            lambda_tuple(["_len", "flags"], var("flags")),
        );
        // flattens the flag-array after parsing it, into the final format with expanded repetitions
        map(
            raw_flags,
            lambda(
                "arr_flags",
                flat_map(
                    lambda(
                        "packed",
                        dup(
                            add(
                                Expr::AsU32(Box::new(record_proj(var("packed"), "repeats"))),
                                Expr::U32(1),
                            ),
                            record_proj(var("packed"), "field_set"),
                        ),
                    ),
                    var("arr_flags"),
                ),
            ),
        )
    }
    /// Given an individual field-set (flag-record) from an array, parse the appropriate x-coordinate value for the corresponding glyph
    fn x_coords(field_set: Expr) -> Format {
        if_then_else(
            record_proj(field_set.clone(), "x_short_vector"),
            parse_u8_to_i16(record_proj(
                field_set.clone(),
                "x_is_same_or_positive_x_short_vector",
            )),
            if_then_else(
                record_proj(field_set.clone(), "x_is_same_or_positive_x_short_vector"),
                compute(poly_zero()),
                i16be(),
            ),
        )
    }

    /// Given an individual field-set (flag-record) from an array, parse the appropriate y-coordinate value for the corresponding glyph
    // TODO - consider a generic `read_coord` function that takes extra parameters to determine x-vs-y specialization
    fn y_coords(field_set: Expr) -> Format {
        if_then_else(
            record_proj(field_set.clone(), "y_short_vector"),
            parse_u8_to_i16(record_proj(
                field_set.clone(),
                "y_is_same_or_positive_y_short_vector",
            )),
            if_then_else(
                record_proj(field_set.clone(), "y_is_same_or_positive_y_short_vector"),
                compute(poly_zero()),
                i16be(),
            ),
        )
    }

    pub(crate) fn table(module: &mut FormatModule) -> DepFormat<1, 0> {
        let simple_flags_raw = flags_raw(module);
        module.register_format_args(
            "opentype.glyf.simple",
            [(Label::Borrowed("n_contours"), ValueType::U16)],
            record([
                (
                    "end_points_of_contour",
                    repeat_count(var("n_contours"), u16be()),
                ),
                ("instruction_length", u16be()),
                (
                    "instructions",
                    repeat_count(var("instruction_length"), u8()),
                ),
                (
                    "number_of_coordinates",
                    compute(succ(util::last_elem(var("end_points_of_contour")))),
                ),
                (
                    "flags",
                    flags(simple_flags_raw, var("number_of_coordinates")),
                ),
                (
                    "x_coordinates",
                    for_each(var("flags"), "flag_vals", x_coords(var("flag_vals"))),
                ),
                (
                    "y_coordinates",
                    for_each(var("flags"), "flag_vals", y_coords(var("flag_vals"))),
                ),
            ]),
        )
    }
}

mod composite {
    use super::*;
    use BitFieldKind::*;

    pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
        let glyf_arg = |are_words: Expr, are_xy_values: Expr| -> Format {
            if_then_else(
                are_words,
                if_then_else(
                    are_xy_values.clone(),
                    fmt_variant("Int16", i16be()),
                    fmt_variant("Uint16", u16be()),
                ),
                if_then_else(
                    are_xy_values,
                    fmt_variant("Int8", i8()),
                    fmt_variant("Uint8", u8()),
                ),
            )
        };
        let glyf_flags_composite = flags();
        let glyf_scale = |flags: Expr| -> Format {
            if_then_else(
                record_proj(flags.clone(), "we_have_a_scale"),
                fmt_some(fmt_variant("Scale", util::f2dot14())),
                if_then_else(
                    record_proj(flags.clone(), "we_have_an_x_and_y_scale"),
                    fmt_some(fmt_variant(
                        "XY",
                        record_repeat(["x_scale", "y_scale"], util::f2dot14()),
                    )),
                    if_then_else(
                        record_proj(flags, "we_have_a_two_by_two"),
                        fmt_some(fmt_variant(
                            "Matrix",
                            tuple_repeat(2, tuple_repeat(2, util::f2dot14())),
                        )),
                        fmt_none(),
                    ),
                ),
            )
        };
        let glyf_component = record([
            ("flags", glyf_flags_composite),
            ("glyph_index", u16be()),
            (
                "argument1",
                glyf_arg(
                    record_proj(var("flags"), "arg_1_and_2_are_words"),
                    record_proj(var("flags"), "args_are_xy_values"),
                ),
            ),
            (
                "argument2",
                glyf_arg(
                    record_proj(var("flags"), "arg_1_and_2_are_words"),
                    record_proj(var("flags"), "args_are_xy_values"),
                ),
            ),
            ("scale", glyf_scale(var("flags"))),
        ]);
        let is_last = lambda_tuple(
            ["_has_instructions", "seq"],
            expr_option_map_or(
                Expr::Bool(false),
                |elt| expr_not(record_lens(elt, &["flags", "more_components"])),
                seq_last_checked(var("seq")),
            ),
        );
        let update_any_instructions = lambda_tuple(
            ["acc", "glyph"],
            or(
                var("acc"),
                record_lens(var("glyph"), &["flags", "we_have_instructions"]),
            ),
        );
        module.define_format(
            "opentype.glyf.composite",
            chain(
                accum_until(
                    is_last,
                    update_any_instructions,
                    Expr::Bool(false),
                    ValueType::Base(BaseType::Bool),
                    glyf_component,
                ),
                "acc_glyphs",
                record([
                    ("glyphs", compute(tuple_proj(var("acc_glyphs"), 1))),
                    (
                        "instructions",
                        if_then_else(
                            tuple_proj(var("acc_glyphs"), 0),
                            chain(
                                u16be(),
                                "instructions_length",
                                repeat_count(var("instructions_length"), u8()),
                            ),
                            compute(seq_empty()),
                        ),
                    ),
                ]),
            ),
        )
    }

    pub(crate) fn flags() -> Format {
        bit_fields_u16([
            Reserved {
                bit_width: 3,
                check_zero: false,
            },
            FlagBit("unscaled_component_offset"), // bit 12 - set if component offset is not to be scaled
            FlagBit("scaled_component_offset"), // bit 11 - set if component offset is to be scaled
            FlagBit("overlap_compound"),        // bit 10 - hint for whether the component overlap
            FlagBit("use_my_metrics"), // bit 9 - when set, composite glyph inherits aw, lsb, rsb of current component glyph
            FlagBit("we_have_instructions"), // bit 8 - instructions present after final component
            FlagBit("we_have_a_two_by_two"), // bit 7 - we have a two by two transformation that will be used to scale the glyph
            FlagBit("we_have_an_x_and_y_scale"), // bit 6 - when set, x has a different scale from y
            FlagBit("more_components"), // bit 5 - continuation bit (1 when more follow, 0 if final)
            Reserved {
                bit_width: 1,
                check_zero: false,
            }, // bit 4 - reserved, should be 0
            FlagBit("we_have_a_scale"), // bit 3 - when 1, component has simple scale; otherwise scale is 1.0
            FlagBit("round_xy_to_grid"), // bit 2 - when set (and when `args_are_xy_values` is set), xy values are rounded to nearest grid line
            FlagBit("args_are_xy_values"), // bit 1 - when set, args are signed xy values; otherwise, they are unsigned point numbers
            FlagBit("arg_1_and_2_are_words"), // bit 0 - set for args of type u16 or i16; clear for args of type u8 or i8
        ])
    }
}

fn table_entry(
    table_view: ViewExpr,
    this_offset32: Expr,
    next_offset32: Expr,
    glyf_entry: FormatRef,
) -> Format {
    if_then_else(
        // NOTE - checks that the glyph is non-vacuous
        expr_gt(next_offset32, this_offset32.clone()),
        fmt_variant(
            "Glyph",
            record_auto([
                ("offset", compute(this_offset32)),
                (
                    "#_data",
                    phantom(util::parse_view_offset::<U32>(
                        table_view,
                        var("offset"),
                        glyf_entry.call(),
                    )),
                ),
            ]),
        ),
        fmt_variant("EmptyGlyph", Format::EMPTY),
    )
}

fn glyf_entry(module: &mut FormatModule, glyf_description: DepFormat<1, 0>) -> FormatRef {
    module.define_format(
        "opentype.glyf.entry",
        record([
            ("number_of_contours", i16be()),
            ("x_min", i16be()),
            ("y_min", i16be()),
            ("x_max", i16be()),
            ("y_max", i16be()),
            (
                "description",
                glyf_description.invoke_args([var("number_of_contours")]),
            ),
        ]),
    )
}

/// Glyph description (empty, simple or composite) format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/glyf#table-organization
///
/// Parametric in `n_contours :~ I16`
fn glyf_description(
    module: &mut FormatModule,
    simple_glyf_table: DepFormat<1, 0>,
    composite_glyf_table: FormatRef,
) -> DepFormat<1, 0> {
    module.register_format_args(
        "opentype.glyf.description",
        // actually I16 but we don't have that yet
        [(Label::Borrowed("n_contours"), ValueType::UnknownNumeric)],
        match_variant(
            var("n_contours"),
            [
                (Pattern::z_const(0), "HeaderOnly", Format::EMPTY),
                (
                    Pattern::z_range(1, i16::MAX),
                    "Simple",
                    simple_glyf_table.invoke_args([numeric(num::unary_with_rep(
                        BasicUnaryOp::AbsVal,
                        Some(MachineRep::U16),
                        num::num_var("n_contours"),
                    ))]),
                ),
                (Pattern::Wildcard, "Composite", composite_glyf_table.call()),
            ],
        ),
    )
}
