use crate::format::BaseModule;
use doodle::{Expr, FormatModule, FormatRef, helper::*};

/// Graphics Interchange Format (GIF)
///
/// - [Graphics Interchange Format Version 89a](https://www.w3.org/Graphics/GIF/spec-gif89a.txt)
#[allow(clippy::redundant_clone)]
pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    fn has_color_table(flags: Expr) -> Expr {
        // (flags->table-flag) != 0
        is_nonzero_u8(record_proj(flags, "table-flag"))
    }

    fn color_table_len(flags: Expr) -> Expr {
        // 2 << (flags->table-size)
        shl(Expr::U16(2), as_u16(record_proj(flags, "table-size")))
    }

    let color_table_entry = module.define_format(
        "gif.color-table-entry",
        record_repeat(["r", "g", "b"], u8()),
    );

    let color_table = |flags: Expr| {
        cond_maybe(
            has_color_table(flags.clone()),
            repeat_count(color_table_len(flags), color_table_entry.call()),
        )
    };

    // 15. Data Sub-blocks
    let subblock = module.define_format(
        "gif.subblock",
        record([
            ("len-bytes", not_byte(0x00)),
            ("data", repeat_count(var("len-bytes"), u8())),
        ]),
    );

    // 16. Block Terminator
    let block_terminator = module.define_format("gif.block-terminator", is_byte(0x00));

    // 17. Header
    let header = module.define_format(
        "gif.header",
        record([
            ("signature", is_bytes(b"GIF")),
            ("version", seq_repeat(3, base.ascii_char())),
        ]),
    );

    // NOTE: Bit data
    // <Packed Fields>  =      Global Color Table Flag       1 Bit
    //                         Color Resolution              3 Bits
    //                         Sort Flag                     1 Bit
    //                         Size of Global Color Table    3 Bits
    // NOTE: Global and local Color Tables have to have the same field-names for
    // common fields in order to ensure that the helpers that extract those values
    // are applicable to both cases without any extra work.
    // FIXME[epic=refactor] - replace with bit_fields_u8
    let logical_screen_descriptor_flags = packed_bits_u8(
        [1, 3, 1, 3],
        ["table-flag", "color-resolution", "sort-flag", "table-size"],
    );

    // 18. Logical Screen Descriptor
    let logical_screen_descriptor = module.define_format(
        "gif.logical-screen-descriptor",
        record([
            ("screen-width", u16le()),
            ("screen-height", u16le()),
            ("flags", logical_screen_descriptor_flags),
            ("bg-color-index", u8()),
            ("pixel-aspect-ratio", u8()),
        ]),
    );

    // 19. Global Color Table
    let global_color_table = color_table;

    // NOTE: Bit data
    // <Packed Fields>  =      Local Color Table Flag        1 Bit
    //                         Interlace Flag                1 Bit
    //                         Sort Flag                     1 Bit
    //                         Reserved                      2 Bits
    //                         Size of Local Color Table     3 Bits
    // NOTE: Global and local Color Tables have to have the same field-names for
    // common fields in order to ensure that the helpers that extract those values
    // are applicable to both cases without any extra work.
    // FIXME[epic=refactor] - replace with bit_fields_u8
    let image_descriptor_flags = packed_bits_u8(
        [1, 1, 1, 2, 3],
        [
            "table-flag",
            "interlace-flag",
            "sort-flag",
            "reserved",
            "table-size",
        ],
    );

    // 20. Image Descriptor
    let image_descriptor = module.define_format(
        "gif.image-descriptor",
        record([
            ("separator", is_byte(0x2C)),
            ("image-left-position", u16le()),
            ("image-top-position", u16le()),
            ("image-width", u16le()),
            ("image-height", u16le()),
            ("flags", image_descriptor_flags),
        ]),
    );

    // 21. Local Color Table
    let local_color_table = color_table;

    // 22. Table Based Image Data
    let table_based_image_data = module.define_format(
        "gif.table-based-image-data",
        record([
            ("lzw-min-code-size", u8()),
            ("image-data", repeat(subblock.call())),
            ("terminator", block_terminator.call()),
        ]),
    );

    // NOTE: Bit data
    // <Packed Fields>  =     Reserved                      3 Bits
    //                        Disposal Method               3 Bits
    //                        User Input Flag               1 Bit
    //                        Transparent Color Flag        1 Bit
    // FIXME[epic=refactor] - replace with bit_fields_u8
    let graphic_control_extension_flags = packed_bits_u8(
        [3, 3, 1, 1],
        [
            "reserved",
            "disposal-method",
            "user-input-flag",
            "transparent-color-flag",
        ],
    );

    // 23. Graphic Control Extension
    let graphic_control_extension = module.define_format(
        "gif.graphic-control-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0xF9)),
            ("block-size", is_byte(4)),
            ("flags", graphic_control_extension_flags),
            ("delay-time", u16le()),
            ("transparent-color-index", u8()),
            ("terminator", block_terminator.call()),
        ]),
    );

    // 24. Comment Extension
    let comment_extension = module.define_format(
        "gif.comment-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0xFE)),
            ("comment-data", repeat(subblock.call())),
            ("terminator", block_terminator.call()),
        ]),
    );

    // 25. Plain Text Extension
    let plain_text_extension = module.define_format(
        "gif.plain-text-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0x01)),
            ("block-size", is_byte(12)),
            ("text-grid-left-position", u16le()),
            ("text-grid-top-position", u16le()),
            ("text-grid-width", u16le()),
            ("text-grid-height", u16le()),
            ("character-cell-width", u8()),
            ("character-cell-height", u8()),
            ("text-foreground-color-index", u8()),
            ("text-background-color-index", u8()),
            ("plain-text-data", repeat(subblock.call())),
            ("terminator", block_terminator.call()),
        ]),
    );

    // 26. Application Extension
    let application_extension = module.define_format(
        "gif.application-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0xFF)),
            ("block-size", is_byte(11)),
            ("identifier", repeat_count(Expr::U8(8), u8())),
            ("authentication-code", seq_repeat(3, u8())),
            ("application-data", repeat(subblock.call())),
            ("terminator", block_terminator.call()),
        ]),
    );

    // 27. Trailer
    let trailer = module.define_format("gif.trailer", record([("separator", is_byte(0x3b))]));

    // Appendix B. GIF Grammar

    let logical_screen = module.define_format(
        "gif.logical-screen",
        record([
            ("descriptor", logical_screen_descriptor.call()),
            (
                "global-color-table",
                global_color_table(Expr::record_proj(var("descriptor"), "flags")),
            ),
        ]),
    );

    let table_based_image = module.define_format(
        "gif.table-based-image",
        record([
            ("descriptor", image_descriptor.call()),
            (
                "local-color-table",
                local_color_table(Expr::record_proj(var("descriptor"), "flags")),
            ),
            ("data", table_based_image_data.call()),
        ]),
    );

    let graphic_rendering_block = module.define_format(
        "gif.graphic-rendering-block",
        alts([
            ("table-based-image", table_based_image.call()),
            ("plain-text-extension", plain_text_extension.call()),
        ]),
    );

    let graphic_block = module.define_format(
        "gif.graphic-block",
        record([
            (
                "graphic-control-extension",
                optional(graphic_control_extension.call()),
            ),
            ("graphic-rendering-block", graphic_rendering_block.call()),
        ]),
    );

    let special_purpose_block = module.define_format(
        "gif.special-purpose-block",
        alts([
            ("application-extension", application_extension.call()),
            ("comment-extension", comment_extension.call()),
        ]),
    );

    let block = module.define_format(
        "gif.block",
        alts([
            ("graphic-block", graphic_block.call()),
            ("special-purpose-block", special_purpose_block.call()),
        ]),
    );

    module.define_format(
        "gif.main",
        record([
            ("header", header.call()),
            ("logical-screen", logical_screen.call()),
            ("blocks", repeat(block.call())),
            ("trailer", trailer.call()),
        ]),
    )
}
