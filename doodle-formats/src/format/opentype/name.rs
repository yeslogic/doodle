use super::*;

/// Opentype `name` table format definition
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/name
pub(crate) fn table(module: &mut FormatModule) -> FormatRef {
    let name_record = name_record(module);
    let name_version_1 = name_version_1(module);

    module.define_format(
        "opentype.name.table",
        let_view(
            "table_view",
            record([
                ("table_scope", reify_view(vvar("table_view"))),
                ("version", u16be()),
                ("name_count", u16be()),
                ("storage_offset", u16be()),
                (
                    "name_records",
                    repeat_count(
                        var("name_count"),
                        name_record
                            .call_views(vec![vvar("table_view").offset(var("storage_offset"))]),
                    ),
                ),
                (
                    "data",
                    match_variant(
                        var("version"),
                        [
                            (Pattern::U16(0), "NameVersion0", Format::EMPTY),
                            (
                                Pattern::U16(1),
                                "NameVersion1",
                                name_version_1
                                    .invoke_view(vvar("table_view").offset(var("storage_offset"))),
                            ),
                            (
                                Pattern::binding("unknown"),
                                "NameVersionUnknown",
                                compute(var("unknown")),
                            ),
                        ],
                    ),
                ),
            ]),
        ),
    )
}

/// NameRecord
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/name#name-records
fn name_record(module: &mut FormatModule) -> FormatRef {
    let name_id = name_id();

    module.define_format_views(
        "opentype.name.name-record",
        vec![Label::Borrowed("storage_view")],
        record([
            ("platform", u16be()),
            ("encoding", encoding_id(var("platform"))),
            ("language", language_id()),
            ("name_id", name_id),
            ("length", u16be()),
            (
                "string",
                // REVIEW - do we want to use util::capture_bytes_view_offset16 here instead?
                record([
                    ("offset", u16be()),
                    (
                        "data", // TODO - add interpretation of raw bytes
                        with_view(
                            vvar("storage_view").offset(var("offset")),
                            capture_bytes(var("length")),
                        ),
                    ),
                ]),
            ),
        ]),
    )
}

// TODO[epic=const-enum] - attach semantics to common subset of name_id values
/// NameID format (U16Be)
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/name#name-ids
pub(crate) fn name_id() -> Format {
    #![allow(dead_code)]
    const NID_COPYRIGHT_NOTICE: u16 = 0;
    const NID_FAMILY_NAME: u16 = 1;
    const NID_SUBFAMILY_NAME: u16 = 2;
    const NID_UNIQUE_FONT_IDENTIFICATION: u16 = 3;
    const NID_FULL_FONT_NAME: u16 = 4;
    const NID_VERSION_STRING: u16 = 5;
    const NID_POSTSCRIPT_NAME: u16 = 6;
    const NID_TRADEMARK: u16 = 7;
    const NID_MANUFACTURER_NAME: u16 = 8;
    const NID_DESIGNER_NAME: u16 = 9;
    const NID_DESCRIPTION: u16 = 10;
    const NID_VENDOR_URL: u16 = 11;
    const NID_DESIGNER_URL: u16 = 12;
    const NID_LICENSE_DESCRIPTION: u16 = 13;
    const NID_LICENSE_INFO_URL: u16 = 14;
    // 15 - reserved
    const NID_TYPOGRAPHIC_FAMILY_NAME: u16 = 16;
    const NID_TYPOGRAPHIC_SUBFAMILY_NAME: u16 = 17;
    const NID_COMPAT_FULL_NAME: u16 = 18;
    const NID_SAMPLE_TEXT: u16 = 19;
    const NID_POSTSCRIPT_FONT_NAME: u16 = 20;
    const NID_WWS_FAMILY_NAME: u16 = 21;
    const NID_WWS_SUBFAMILY_NAME: u16 = 22;
    const NID_LIGHT_BACKGROUND_PALETTE: u16 = 23;
    const NID_DARK_BACKGROUND_PALETTE: u16 = 24;
    const NID_VARIATIONS_POSTSCRIPT_NAME_PREFIX: u16 = 25;
    // 26..=255 - reserved
    // 256..=32767 - font-specific names

    u16be()
}

/// Naming table version 1
///
/// C.f. https://learn.microsoft.com/en-us/typography/opentype/spec/name#naming-table-version-1
fn name_version_1(module: &mut FormatModule) -> DepFormat<0, 1> {
    let lang_tag_record = module.define_format_views(
        "opentype.name.lang-tag-record",
        vec![Label::Borrowed("storage_view")],
        record([
            ("length", u16be()),
            (
                "lang_tag",
                util::capture_bytes_view_offset16(vvar("storage_view"), var("length")),
            ),
        ]),
    );

    module.register_format_views(
        "opentype.name.name_version_1",
        [Label::Borrowed("storage_view")],
        record([
            ("lang_tag_count", u16be()),
            (
                "lang_tag_records",
                repeat_count(
                    var("lang_tag_count"),
                    lang_tag_record.call_views(vec![vvar("storage_view")]),
                ),
            ),
        ]),
    )
}
