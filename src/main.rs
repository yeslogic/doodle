#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use doodle::byte_set::ByteSet;
use doodle::{Decoder, Expr, Format, FormatModule, Func, Pattern};

#[derive(Copy, Clone, ValueEnum)]
enum OutputFormat {
    /// Use the debug formatter
    Debug,
    /// Serialize to JSON
    Json,
    /// Display as a human-readable tree
    Tree,
}

/// Decode a binary file
#[derive(Parser)]
struct Args {
    /// How decoded values are rendered
    #[arg(long, default_value = "tree")]
    output: OutputFormat,
    /// The binary file to decode
    #[arg(default_value = "test.jpg")]
    filename: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();
    let input = fs::read(args.filename)?;

    let mut module = FormatModule::new();
    let base = base_module(&mut module);

    let tiff = tiff_format(&mut module, &base);
    let gif = gif_format(&mut module, &base);
    let jpeg = jpeg_format(&mut module, &base, &tiff);
    let png = png_format(&mut module, &base);
    let riff = riff_format(&mut module, &base);

    let format = Format::Map(
        Func::RecordProj("data".to_string()),
        Box::new(record([
            (
                "data",
                alts([("gif", gif), ("jpeg", jpeg), ("png", png), ("riff", riff)]),
            ),
            ("end", Format::EndOfInput),
        ])),
    );

    let (val, _) = Decoder::compile(&module, &format)?
        .parse(&mut Vec::new(), &input)
        .ok_or("parse failure")?;

    match args.output {
        OutputFormat::Debug => println!("{val:?}"),
        OutputFormat::Json => serde_json::to_writer(std::io::stdout(), &val).unwrap(),
        OutputFormat::Tree => doodle::output::tree::print_decoded_value(&module, &val, &format),
    }

    Ok(())
}

fn tuple(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Tuple(formats.into_iter().collect())
}

fn alts<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
    Format::Union(
        (fields.into_iter())
            .map(|(label, format)| (label.into(), format))
            .collect(),
    )
}

fn record<Label: Into<String>>(fields: impl IntoIterator<Item = (Label, Format)>) -> Format {
    Format::Record(
        (fields.into_iter())
            .map(|(label, format)| (label.into(), format))
            .collect(),
    )
}

fn optional(format: Format) -> Format {
    alts([("some", format), ("none", Format::EMPTY)])
}

fn repeat(format: Format) -> Format {
    Format::Repeat(Box::new(format))
}

fn repeat1(format: Format) -> Format {
    Format::Repeat1(Box::new(format))
}

fn repeat_count(len: Expr, format: Format) -> Format {
    Format::RepeatCount(len, Box::new(format))
}

fn if_then_else(cond: Expr, format0: Format, format1: Format) -> Format {
    Format::Match(
        cond,
        vec![
            (Pattern::Bool(true), format0),
            (Pattern::Bool(false), format1),
        ],
    )
}

fn is_byte(b: u8) -> Format {
    Format::Byte(ByteSet::from([b]))
}

fn not_byte(b: u8) -> Format {
    Format::Byte(!ByteSet::from([b]))
}

fn is_bytes(bytes: &[u8]) -> Format {
    tuple(bytes.iter().copied().map(is_byte))
}

struct BaseModule {
    u8: Format,
    u16be: Format,
    u16le: Format,
    u32be: Format,
    u32le: Format,
    asciiz_string: Format,
}

#[rustfmt::skip]
impl BaseModule {
    fn u8(&self) -> Format { self.u8.clone() }
    fn u16be(&self) -> Format { self.u16be.clone() }
    fn u16le(&self) -> Format { self.u16le.clone() }
    fn u32be(&self) -> Format { self.u32be.clone() }
    fn u32le(&self) -> Format { self.u32le.clone() }
    fn asciiz_string(&self) -> Format { self.asciiz_string.clone() }
}

fn base_module(module: &mut FormatModule) -> BaseModule {
    let u8 = module.define_format("base.u8", Format::Byte(ByteSet::full()));

    let u16be = module.define_format(
        "base.u16be",
        Format::Map(Func::U16Be, Box::new(tuple([u8.clone(), u8.clone()]))),
    );

    let u16le = module.define_format(
        "base.u16le",
        Format::Map(Func::U16Le, Box::new(tuple([u8.clone(), u8.clone()]))),
    );

    let u32be = module.define_format(
        "base.u32be",
        Format::Map(
            Func::U32Be,
            Box::new(tuple([u8.clone(), u8.clone(), u8.clone(), u8.clone()])),
        ),
    );

    let u32le = module.define_format(
        "base.u32le",
        Format::Map(
            Func::U32Le,
            Box::new(tuple([u8.clone(), u8.clone(), u8.clone(), u8.clone()])),
        ),
    );

    let asciiz_string = module.define_format(
        "base.asciiz-string",
        Format::Map(
            Func::RecordProj("string".to_string()),
            Box::new(record([
                ("string", repeat(not_byte(0x00))),
                ("null", is_byte(0x00)),
            ])),
        ),
    );

    BaseModule {
        u8,
        u16be,
        u16le,
        u32be,
        u32le,
        asciiz_string,
    }
}

/// Graphics Interchange Format (GIF)
///
/// - [Graphics Interchange Format Version 89a](https://www.w3.org/Graphics/GIF/spec-gif89a.txt)
#[allow(clippy::redundant_clone)]
fn gif_format(module: &mut FormatModule, base: &BaseModule) -> Format {
    fn has_color_table(flags: Expr) -> Expr {
        // (flags & 0b10000000) != 0
        Expr::Ne(
            Box::new(Expr::BitAnd(
                Box::new(flags),
                Box::new(Expr::U8(0b10000000)),
            )),
            Box::new(Expr::U8(0)),
        )
    }

    fn color_table_len(flags: Expr) -> Expr {
        // 2 << (flags & 7)
        Expr::Shl(
            Box::new(Expr::U8(2)),
            Box::new(Expr::BitAnd(Box::new(flags), Box::new(Expr::U8(7)))),
        )
    }

    let color_table_entry = module.define_format(
        "gif.color-table-entry",
        record([("r", base.u8()), ("g", base.u8()), ("b", base.u8())]),
    );

    let color_table = |flags: Expr| {
        if_then_else(
            has_color_table(flags.clone()),
            repeat_count(color_table_len(flags), color_table_entry),
            Format::EMPTY,
        )
    };

    // 15. Data Sub-blocks
    let subblock = module.define_format(
        "gif.subblock",
        record([
            ("len-bytes", not_byte(0x00)),
            ("data", repeat_count(Expr::Var(0), base.u8())),
        ]),
    );

    // 16. Block Terminator
    let block_terminator = module.define_format("gif.block-terminator", is_byte(0x00));

    // 17. Header
    let header = module.define_format(
        "gif.header",
        record([
            ("signature", is_bytes(b"GIF")),
            ("version", repeat_count(Expr::U8(3), base.u8())), // "87a" | "89a" | ...
        ]),
    );

    // 18. Logical Screen Descriptor
    let logical_screen_descriptor = module.define_format(
        "gif.logical-screen-descriptor",
        record([
            ("screen-width", base.u16le()),
            ("screen-height", base.u16le()),
            ("flags", base.u8()),
            // TODO: Bit data
            // <Packed Fields>  =      Global Color Table Flag       1 Bit
            //                         Color Resolution              3 Bits
            //                         Sort Flag                     1 Bit
            //                         Size of Global Color Table    3 Bits
            ("bg-color-index", base.u8()),
            ("pixel-aspect-ratio", base.u8()),
        ]),
    );

    // 19. Global Color Table
    let global_color_table = color_table.clone();

    // 20. Image Descriptor
    let image_descriptor = module.define_format(
        "gif.image-descriptor",
        record([
            ("separator", is_byte(0x2C)),
            ("image-left-position", base.u16le()),
            ("image-top-position", base.u16le()),
            ("image-width", base.u16le()),
            ("image-height", base.u16le()),
            ("flags", base.u8()),
            // TODO: Bit data
            // <Packed Fields>  =      Local Color Table Flag        1 Bit
            //                         Interlace Flag                1 Bit
            //                         Sort Flag                     1 Bit
            //                         Reserved                      2 Bits
            //                         Size of Local Color Table     3 Bits
        ]),
    );

    // 21. Local Color Table
    let local_color_table = color_table.clone();

    // 22. Table Based Image Data
    let table_based_image_data = module.define_format(
        "gif.table-based-image-data",
        record([
            ("lzw-min-code-size", base.u8()),
            ("image-data", repeat(subblock.clone())),
            ("terminator", block_terminator.clone()),
        ]),
    );

    // 23. Graphic Control Extension
    let graphic_control_extension = module.define_format(
        "gif.graphic-control-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0xF9)),
            ("block-size", is_byte(4)),
            ("flags", base.u8()),
            // TODO: Bit data
            // <Packed Fields>  =     Reserved                      3 Bits
            //                        Disposal Method               3 Bits
            //                        User Input Flag               1 Bit
            //                        Transparent Color Flag        1 Bit
            ("delay-time", base.u16le()),
            ("transparent-color-index", base.u8()),
            ("terminator", block_terminator.clone()),
        ]),
    );

    // 24. Comment Extension
    let comment_extension = module.define_format(
        "gif.comment-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0xFE)),
            ("comment-data", repeat(subblock.clone())),
            ("terminator", block_terminator.clone()),
        ]),
    );

    // 25. Plain Text Extension
    let plain_text_extension = module.define_format(
        "gif.plain-text-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0x01)),
            ("block-size", is_byte(12)),
            ("text-grid-left-position", base.u16le()),
            ("text-grid-top-position", base.u16le()),
            ("text-grid-width", base.u16le()),
            ("text-grid-height", base.u16le()),
            ("character-cell-width", base.u8()),
            ("character-cell-height", base.u8()),
            ("text-foreground-color-index", base.u8()),
            ("text-background-color-index", base.u8()),
            ("plain-text-data", repeat(subblock.clone())),
            ("terminator", block_terminator.clone()),
        ]),
    );

    // 26. Application Extension
    let application_extension = module.define_format(
        "gif.application-extension",
        record([
            ("separator", is_byte(0x21)),
            ("label", is_byte(0xFF)),
            ("block-size", is_byte(11)),
            ("identifier", repeat_count(Expr::U8(8), base.u8())),
            ("authentication-code", repeat_count(Expr::U8(3), base.u8())),
            ("application-data", repeat(subblock.clone())),
            ("terminator", block_terminator.clone()),
        ]),
    );

    // 27. Trailer
    let trailer = module.define_format("gif.trailer", record([("separator", is_byte(0x3b))]));

    // Appendix B. GIF Grammar

    let logical_screen = module.define_format(
        "gif.logical-screen",
        record([
            ("descriptor", logical_screen_descriptor),
            (
                "global-color-table",
                global_color_table(Expr::record_proj(Expr::Var(0), "flags")),
            ),
        ]),
    );

    let table_based_image = module.define_format(
        "gif.table-based-image",
        record([
            ("descriptor", image_descriptor),
            (
                "local-color-table",
                local_color_table(Expr::record_proj(Expr::Var(0), "flags")),
            ),
            ("data", table_based_image_data),
        ]),
    );

    let graphic_rendering_block = module.define_format(
        "gif.graphic-rendering-block",
        alts([
            ("table-based-image", table_based_image),
            ("plain-text-extension", plain_text_extension),
        ]),
    );

    let graphic_block = module.define_format(
        "gif.graphic-block",
        record([
            (
                "graphic-control-extension",
                optional(graphic_control_extension),
            ),
            ("graphic-rendering-block", graphic_rendering_block),
        ]),
    );

    let special_purpose_block = module.define_format(
        "gif.special-purpose-block",
        alts([
            ("application-extension", application_extension),
            ("comment-extension", comment_extension),
        ]),
    );

    let block = module.define_format(
        "gif.block",
        alts([
            ("graphic-block", graphic_block),
            ("special-purpose-block", special_purpose_block),
        ]),
    );

    module.define_format(
        "gif.main",
        record([
            ("header", header),
            ("logical-screen", logical_screen),
            ("blocks", repeat(block)),
            ("trailer", trailer),
        ]),
    )
}

/// JPEG File Interchange Format
///
/// - [JPEG File Interchange Format Version 1.02](https://www.w3.org/Graphics/JPEG/jfif3.pdf)
/// - [ITU T.81 | ISO IEC 10918-1](https://www.w3.org/Graphics/JPEG/itu-t81.pdf)
#[allow(clippy::redundant_clone)]
fn jpeg_format(module: &mut FormatModule, base: &BaseModule, tiff: &Format) -> Format {
    fn marker(id: u8) -> Format {
        Format::Map(
            Func::TupleProj(1),
            Box::new(tuple([is_byte(0xFF), is_byte(id)])),
        )
    }

    let marker_segment = |id: u8, data: Format| {
        record([
            ("marker", marker(id)),
            ("length", base.u16be()),
            (
                "data",
                Format::Slice(
                    Expr::Sub(
                        Box::new(Expr::Var(0)), // length
                        Box::new(Expr::U16(2)),
                    ),
                    Box::new(data),
                ),
            ),
        ])
    };

    // SOF_n: Frame header (See ITU T.81 Section B.2.2)
    let sof_data = {
        let sof_image_component = module.define_format(
            "jpeg.sof-image-component",
            record([
                ("id", base.u8()),
                ("sampling-factor", base.u8()), // TODO: Bit data: { horizontal <- u4, vertical <- u4 }
                ("quantization-table-id", base.u8()),
            ]),
        );

        module.define_format(
            "jpeg.sof-data",
            record([
                ("sample-precision", base.u8()),
                ("num-lines", base.u16be()),
                ("num-samples-per-line", base.u16be()),
                ("num-image-components", base.u8()),
                (
                    "image-components",
                    repeat_count(
                        Expr::Var(0), // num-image-components
                        sof_image_component,
                    ),
                ),
            ]),
        )
    };

    // DHT: Define Huffman table (See ITU T.81 Section B.2.4.2)
    let dht_data = module.define_format(
        "jpeg.dht-data",
        record([
            // class <- u4 //= 0 | 1;
            // table-id <- u4 //= 1 |..| 4;
            ("class-table-id", base.u8()),
            ("num-codes", repeat_count(Expr::U8(16), base.u8())),
            ("values", repeat(base.u8())), // List.map num-codes (\n => repeat-count n u8);
        ]),
    );

    // DAC: Define arithmetic conditioning table (See ITU T.81 Section B.2.4.3)
    let dac_data = module.define_format(
        "jpeg.dac-data",
        record([
            // class <- u4 //= 0 | 1;
            // table-id <- u4 //= 1 |..| 4;
            ("class-table-id", base.u8()),
            ("value", base.u8()),
        ]),
    );

    // SOS: Scan header (See ITU T.81 Section B.2.3)
    let sos_data = {
        let sos_image_component = module.define_format(
            "jpeg.sos-image-component",
            record([
                ("component-selector", base.u8()), // ???
                // TODO: Bit data
                // dc-entropy-coding-table-id <- u4;
                // ac-entropy-coding-table-id <- u4;
                ("entropy-coding-table-ids", base.u8()),
            ]),
        );

        module.define_format(
            "jpeg.sos-data",
            record([
                ("num-image-components", base.u8()), // 1 |..| 4
                (
                    "image-components",
                    repeat_count(
                        Expr::Var(0), // num-image-components
                        sos_image_component,
                    ),
                ),
                ("start-spectral-selection", base.u8()), // ???
                ("end-spectral-selection", base.u8()),   // ???
                ("approximation-bit-position", base.u8()), // TODO: Bit data: { high <- u4, low <- u4 }
            ]),
        )
    };

    // DQT: Define quantization table (See ITU T.81 Section B.2.4.1)
    let dqt_data = module.define_format(
        "jpeg.dqt-data",
        record([
            // precision <- u4 //= 0 | 1;
            // table-id <- u4 //= 1 |..| 4;
            ("precision-table-id", base.u8()),
            // elements <- match precision {
            //   0 => repeat-count 64 u8,
            //   1 => repeat-count 64 u16be,
            // };
            ("elements", repeat(base.u8())),
        ]),
    );

    // DNL: Define number of lines (See ITU T.81 Section B.2.5)
    let dnl_data = module.define_format("jpeg.dnl-data", record([("num-lines", base.u16be())]));

    // DRI: Define restart interval (See ITU T.81 Section B.2.4.4)
    let dri_data = module.define_format(
        "jpeg.dri-data",
        record([("restart-interval", base.u16be())]),
    );

    // DHP: Define hierarchial progression (See ITU T.81 Section B.3.2)
    // NOTE: Same as SOF except for quantization-table-id
    let dhp_data = {
        let dhp_image_component = module.define_format(
            "jpeg.dhp-image-component",
            record([
                ("id", base.u8()),
                ("sampling-factor", base.u8()), // TODO: Bit data: { horizontal <- u4, vertical <- u4 }
                ("quantization-table-id", is_byte(0)),
            ]),
        );

        module.define_format(
            "jpeg.dhp-data",
            record([
                ("sample-precision", base.u8()),
                ("num-lines", base.u16be()),
                ("num-samples-per-line", base.u16be()),
                ("num-image-components", base.u8()),
                (
                    "image-components",
                    repeat_count(
                        Expr::Var(0), // num-image-components
                        dhp_image_component,
                    ),
                ),
            ]),
        )
    };

    // EXP: Expand reference components (See ITU T.81 Section B.3.3)
    let exp_data = module.define_format(
        "jpeg.exp-data",
        record([
            // TODO: Bit data
            // expand-horizontal <- u4 // 0 | 1;
            // expand-vertical <- u4 // 0 | 1;
            ("expand-horizontal-vertical", base.u8()),
        ]),
    );

    // APP0: Application segment 0 (JFIF)
    let app0_jfif = {
        let thumbnail_pixel = module.define_format(
            "jpeg.thumbnail-pixel",
            record([("r", base.u8()), ("g", base.u8()), ("b", base.u8())]),
        );

        module.define_format(
            "jpeg.app0-jfif",
            record([
                ("version-major", base.u8()),
                ("version-minor", base.u8()),
                ("density-units", base.u8()), // 0 | 1 | 2
                ("density-x", base.u16be()),  // != 0
                ("density-y", base.u16be()),  // != 0
                ("thumbnail-width", base.u8()),
                ("thumbnail-height", base.u8()),
                (
                    "thumbnail-pixels",
                    repeat_count(
                        Expr::Var(0), // thumbnail-height
                        repeat_count(
                            Expr::Var(1), // thumbnail-width
                            thumbnail_pixel,
                        ),
                    ),
                ),
            ]),
        )
    };

    let app0_data = module.define_format(
        "jpeg.app0-data",
        record([
            ("identifier", base.asciiz_string()),
            (
                "data",
                Format::Match(
                    Expr::Var(0), // identifier
                    vec![
                        (Pattern::from_bytes(b"JFIF"), app0_jfif),
                        // FIXME: there are other APP0 formats
                        // see https://exiftool.org/TagNames/JPEG.html
                        (Pattern::Wildcard, repeat(base.u8())),
                    ],
                ),
            ),
        ]),
    );

    // APP1: Application segment 1 (EXIF)
    //
    // - [Exif Version 2.32, Section 4.5.4](https://www.cipa.jp/std/documents/e/DC-X008-Translation-2019-E.pdf#page=24)
    let app1_exif = module.define_format(
        "jpeg.app1-exif",
        record([("padding", is_byte(0x00)), ("exif", tiff.clone())]),
    );

    // APP1: Application segment 1 (XMP)
    let app1_xmp = module.define_format("jpeg.app1-xmp", record([("xmp", repeat(base.u8()))]));

    let app1_data = module.define_format(
        "jpeg.app1-data",
        record([
            ("identifier", base.asciiz_string()),
            (
                "data",
                Format::Match(
                    Expr::Var(0), // identifier
                    vec![
                        (Pattern::from_bytes(b"Exif"), app1_exif),
                        (
                            Pattern::from_bytes(b"http://ns.adobe.com/xap/1.0/"),
                            app1_xmp,
                        ),
                        // FIXME: there are other APP1 formats
                        // see https://exiftool.org/TagNames/JPEG.html
                        (Pattern::Wildcard, repeat(base.u8())),
                    ],
                ),
            ),
        ]),
    );

    let sof0 = module.define_format("jpeg.sof0", marker_segment(0xC0, sof_data.clone())); // Start of frame (baseline jpeg)
    let sof1 = module.define_format("jpeg.sof1", marker_segment(0xC1, sof_data.clone())); // Start of frame (extended sequential, huffman)
    let sof2 = module.define_format("jpeg.sof2", marker_segment(0xC2, sof_data.clone())); // Start of frame (progressive, huffman)
    let sof3 = module.define_format("jpeg.sof3", marker_segment(0xC3, sof_data.clone())); // Start of frame (lossless, huffman)
    let dht = module.define_format("jpeg.dht", marker_segment(0xC4, dht_data.clone())); // Define Huffman Table
    let sof5 = module.define_format("jpeg.sof5", marker_segment(0xC5, sof_data.clone())); // Start of frame (differential sequential, huffman)
    let sof6 = module.define_format("jpeg.sof6", marker_segment(0xC6, sof_data.clone())); // Start of frame (differential progressive, huffman)
    let sof7 = module.define_format("jpeg.sof7", marker_segment(0xC7, sof_data.clone())); // Start of frame (differential lossless, huffman)
    let _jpeg = module.define_format("jpeg.jpeg", marker_segment(0xC8, repeat(base.u8()))); // Reserved for JPEG extension
    let sof9 = module.define_format("jpeg.sof9", marker_segment(0xC9, sof_data.clone())); // Start of frame (extended sequential, arithmetic)
    let sof10 = module.define_format("jpeg.sof10", marker_segment(0xCA, sof_data.clone())); // Start of frame (progressive, arithmetic)
    let sof11 = module.define_format("jpeg.sof11", marker_segment(0xCB, sof_data.clone())); // Start of frame (lossless, arithmetic)
    let dac = module.define_format("jpeg.dac", marker_segment(0xCC, dac_data.clone())); // Define arithmetic coding conditioning
    let sof13 = module.define_format("jpeg.sof13", marker_segment(0xCD, sof_data.clone())); // Start of frame (differential sequential, arithmetic)
    let sof14 = module.define_format("jpeg.sof14", marker_segment(0xCE, sof_data.clone())); // Start of frame (differential progressive, arithmetic)
    let sof15 = module.define_format("jpeg.sof15", marker_segment(0xCF, sof_data.clone())); // Start of frame (differential lossless, arithmetic)
    let rst0 = module.define_format("jpeg.rst0", marker(0xD0)); // Restart marker 0
    let rst1 = module.define_format("jpeg.rst1", marker(0xD1)); // Restart marker 1
    let rst2 = module.define_format("jpeg.rst2", marker(0xD2)); // Restart marker 2
    let rst3 = module.define_format("jpeg.rst3", marker(0xD3)); // Restart marker 3
    let rst4 = module.define_format("jpeg.rst4", marker(0xD4)); // Restart marker 4
    let rst5 = module.define_format("jpeg.rst5", marker(0xD5)); // Restart marker 5
    let rst6 = module.define_format("jpeg.rst6", marker(0xD6)); // Restart marker 6
    let rst7 = module.define_format("jpeg.rst7", marker(0xD7)); // Restart marker 7
    let soi = module.define_format("jpeg.soi", marker(0xD8)); // Start of image
    let eoi = module.define_format("jpeg.eoi", marker(0xD9)); // End of of image
    let sos = module.define_format("jpeg.sos", marker_segment(0xDA, sos_data.clone())); // Start of scan
    let dqt = module.define_format("jpeg.dqt", marker_segment(0xDB, dqt_data.clone())); // Define quantization table
    let dnl = module.define_format("jpeg.dnl", marker_segment(0xDC, dnl_data.clone())); // Define number of lines
    let dri = module.define_format("jpeg.dri", marker_segment(0xDD, dri_data.clone())); // Define restart interval
    let _dhp = module.define_format("jpeg.dhp", marker_segment(0xDE, dhp_data.clone())); // Define hierarchical progression
    let _exp = module.define_format("jpeg.exp", marker_segment(0xDF, exp_data.clone())); // Expand reference components
    let app0 = module.define_format("jpeg.app0", marker_segment(0xE0, app0_data.clone())); // Application segment 0 (JFIF/JFXX/AVI1/...)
    let app1 = module.define_format("jpeg.app1", marker_segment(0xE1, app1_data.clone())); // Application segment 1 (EXIF/XMP/XAP/...)
    let app2 = module.define_format("jpeg.app2", marker_segment(0xE2, repeat(base.u8()))); // Application segment 2 (FlashPix/ICC/...)
    let app3 = module.define_format("jpeg.app3", marker_segment(0xE3, repeat(base.u8()))); // Application segment 3 (Kodak/...)
    let app4 = module.define_format("jpeg.app4", marker_segment(0xE4, repeat(base.u8()))); // Application segment 4 (FlashPix/...)
    let app5 = module.define_format("jpeg.app5", marker_segment(0xE5, repeat(base.u8()))); // Application segment 5 (Ricoh/...)
    let app6 = module.define_format("jpeg.app6", marker_segment(0xE6, repeat(base.u8()))); // Application segment 6 (GoPro/...)
    let app7 = module.define_format("jpeg.app7", marker_segment(0xE7, repeat(base.u8()))); // Application segment 7 (Pentax/Qualcomm/...)
    let app8 = module.define_format("jpeg.app8", marker_segment(0xE8, repeat(base.u8()))); // Application segment 8 (Spiff/...)
    let app9 = module.define_format("jpeg.app9", marker_segment(0xE9, repeat(base.u8()))); // Application segment 9 (MediaJukebox/...)
    let app10 = module.define_format("jpeg.app10", marker_segment(0xEA, repeat(base.u8()))); // Application segment 10 (PhotoStudio)
    let app11 = module.define_format("jpeg.app11", marker_segment(0xEB, repeat(base.u8()))); // Application segment 11 (HDR)
    let app12 = module.define_format("jpeg.app12", marker_segment(0xEC, repeat(base.u8()))); // Application segment 12 (PictureInfo/Ducky)
    let app13 = module.define_format("jpeg.app13", marker_segment(0xED, repeat(base.u8()))); // Application segment 13 (PhotoShop/Adobe_CM)
    let app14 = module.define_format("jpeg.app14", marker_segment(0xEE, repeat(base.u8()))); // Application segment 14 (Adobe)
    let app15 = module.define_format("jpeg.app15", marker_segment(0xEF, repeat(base.u8()))); // Application segment 15 (GraphicConverter)
    let com = module.define_format("jpeg.com", marker_segment(0xFE, repeat(base.u8()))); // Extension data (comment)

    let table_or_misc = module.define_format(
        "jpeg.table-or-misc",
        alts([
            ("dqt", dqt.clone()), // Define quantization table
            ("dht", dht.clone()), // Define Huffman Table
            ("dac", dac.clone()), // Define arithmetic coding conditioning
            ("dri", dri.clone()), // Define restart interval
            ("app0", app0.clone()),
            ("app1", app1.clone()),
            ("app2", app2.clone()),
            ("app3", app3.clone()),
            ("app4", app4.clone()),
            ("app5", app5.clone()),
            ("app6", app6.clone()),
            ("app7", app7.clone()),
            ("app8", app8.clone()),
            ("app9", app9.clone()),
            ("app10", app10.clone()),
            ("app11", app11.clone()),
            ("app12", app12.clone()),
            ("app13", app13.clone()),
            ("app14", app14.clone()),
            ("app15", app15.clone()),
            ("com", com.clone()), // Comment
        ]),
    );

    let frame_header = module.define_format(
        "jpeg.frame-header",
        alts([
            ("sof0", sof0.clone()),
            ("sof1", sof1.clone()),
            ("sof2", sof2.clone()),
            ("sof3", sof3.clone()),
            ("sof5", sof5.clone()),
            ("sof6", sof6.clone()),
            ("sof7", sof7.clone()),
            ("sof9", sof9.clone()),
            ("sof10", sof10.clone()),
            ("sof11", sof11.clone()),
            ("sof13", sof13.clone()),
            ("sof14", sof14.clone()),
            ("sof15", sof15.clone()),
        ]),
    );

    // MCU: Minimum coded unit
    let mcu = module.define_format(
        "jpeg.mcu",
        Format::Map(
            Func::Match(vec![
                (Pattern::variant("byte", Pattern::Binding), Expr::Var(0)),
                (Pattern::variant("zero", Pattern::Wildcard), Expr::U8(0xFF)),
            ]),
            Box::new(alts([
                ("byte", not_byte(0xFF)),
                ("zero", tuple([is_byte(0xFF), is_byte(0x00)])),
            ])),
        ),
    );

    // A series of entropy coded segments separated by restart markers
    let scan_data = module.define_format(
        "jpeg.scan-data",
        Format::Map(
            Func::Stream,
            Box::new(repeat(Format::Map(
                Func::Match(vec![
                    (Pattern::variant("mcu", Pattern::Binding), Expr::Var(0)),
                    (Pattern::variant("rst0", Pattern::Wildcard), Expr::UNIT),
                    (Pattern::variant("rst1", Pattern::Wildcard), Expr::UNIT),
                    (Pattern::variant("rst2", Pattern::Wildcard), Expr::UNIT),
                    (Pattern::variant("rst3", Pattern::Wildcard), Expr::UNIT),
                    (Pattern::variant("rst4", Pattern::Wildcard), Expr::UNIT),
                    (Pattern::variant("rst5", Pattern::Wildcard), Expr::UNIT),
                    (Pattern::variant("rst6", Pattern::Wildcard), Expr::UNIT),
                    (Pattern::variant("rst7", Pattern::Wildcard), Expr::UNIT),
                ]),
                Box::new(alts([
                    // FIXME: Extract into separate ECS repetition
                    ("mcu", mcu), // TODO: repeat(mcu),
                    // FIXME: Restart markers should cycle in order from rst0-rst7
                    ("rst0", rst0),
                    ("rst1", rst1),
                    ("rst2", rst2),
                    ("rst3", rst3),
                    ("rst4", rst4),
                    ("rst5", rst5),
                    ("rst6", rst6),
                    ("rst7", rst7),
                ])),
            ))),
        ),
    );

    let scan = module.define_format(
        "jpeg.scan",
        record([
            ("segments", repeat(table_or_misc.clone())),
            ("sos", sos.clone()),
            ("data", scan_data.clone()),
        ]),
    );

    let frame = module.define_format(
        "jpeg.frame",
        record([
            (
                "initial-segment",
                alts([("app0", app0.clone()), ("app1", app1.clone())]),
            ),
            ("segments", repeat(table_or_misc.clone())),
            ("header", frame_header.clone()),
            ("scan", scan.clone()),
            ("dnl", optional(dnl.clone())),
            ("scans", repeat(scan)),
        ]),
    );

    module.define_format(
        "jpeg.main",
        record([
            ("soi", soi.clone()),
            ("frame", frame.clone()),
            ("eoi", eoi.clone()),
        ]),
    )
}

#[allow(clippy::redundant_clone)]
fn png_format(module: &mut FormatModule, base: &BaseModule) -> Format {
    let chunk = |tag: Format, data: Format| {
        record([
            ("length", base.u32be()), // FIXME < 2^31
            ("tag", tag),
            ("data", Format::Slice(Expr::Var(1), Box::new(data))),
            ("crc", base.u32be()), // FIXME check this
        ])
    };

    // let any_tag = module.define_format(
    //     "png.any-tag",
    //     tuple([base.u8(), base.u8(), base.u8(), base.u8()]), // FIXME: ASCII
    // );

    let ihdr_tag = module.define_format("png.ihdr-tag", is_bytes(b"IHDR"));
    let ihdr_data = module.define_format(
        "png.ihdr-data",
        record([
            ("width", base.u32be()),
            ("height", base.u32be()),
            ("bit-depth", base.u8()),
            ("color-type", base.u8()),
            ("compression-method", base.u8()),
            ("filter-method", base.u8()),
            ("interlace-method", base.u8()),
        ]),
    );

    let idat_tag = module.define_format("png.idat-tag", is_bytes(b"IDAT"));
    let idat_data = module.define_format("png.idat-data", repeat(base.u8()));

    let iend_tag = module.define_format("png.iend-tag", is_bytes(b"IEND"));
    let iend_data = module.define_format("png.iend-data", Format::EMPTY); // FIXME ensure IEND length = 0

    let other_tag = module.define_format(
        "png.other-tag",
        alts([
            ("PLTE", is_bytes(b"PLTE")),
            ("bKGD", is_bytes(b"bKGD")),
            ("pHYs", is_bytes(b"pHYs")),
            ("tIME", is_bytes(b"tIME")),
            ("tRNS", is_bytes(b"tRNS")),
            // FIXME other tags excluding IHDR/IDAT/IEND
        ]),
    );

    module.define_format(
        "png.main",
        record([
            ("signature", is_bytes(b"\x89PNG\r\n\x1A\n")),
            ("ihdr", chunk(ihdr_tag, ihdr_data)),
            (
                "chunks",
                repeat(chunk(other_tag.clone(), repeat(base.u8()))),
            ),
            ("idat", repeat1(chunk(idat_tag, idat_data))),
            (
                "more-chunks",
                repeat(chunk(other_tag.clone(), repeat(base.u8()))),
            ),
            ("iend", chunk(iend_tag, iend_data)),
        ]),
    )
}

#[allow(clippy::redundant_clone)]
fn riff_format(module: &mut FormatModule, base: &BaseModule) -> Format {
    fn is_even(num: Expr) -> Expr {
        // (num % 2) == 0
        Expr::Eq(
            Box::new(Expr::Rem(Box::new(num), Box::new(Expr::U32(2)))),
            Box::new(Expr::U32(0)),
        )
    }

    let chunk = |tag: Format, data: Format| {
        record([
            ("tag", tag),
            ("length", base.u32le()),
            ("data", Format::Slice(Expr::Var(0), Box::new(data))),
            (
                "pad",
                if_then_else(is_even(Expr::Var(1)), Format::EMPTY, is_byte(0x00)),
            ),
        ])
    };

    let any_tag = module.define_format(
        "riff.any-tag",
        tuple([base.u8(), base.u8(), base.u8(), base.u8()]), // FIXME: ASCII
    );

    let subchunks = module.define_format(
        "riff.subchunks",
        record([
            ("tag", any_tag.clone()),
            ("chunks", repeat(chunk(any_tag, repeat(base.u8())))),
        ]),
    );

    module.define_format("riff.main", chunk(is_bytes(b"RIFF"), subchunks.clone()))
}

/// TIFF Image file header
///
/// - [TIFF 6.0 Specification, Section 4.5](https://developer.adobe.com/content/dam/udp/en/open/standards/tiff/TIFF6.pdf#page=13)
/// - [Exif Version 2.32, Section 4.5.2](https://www.cipa.jp/std/documents/e/DC-X008-Translation-2019-E.pdf#page=23)
fn tiff_format(module: &mut FormatModule, base: &BaseModule) -> Format {
    // Image file directory field
    let ifd_field = |is_be: bool| {
        record([
            ("tag", if is_be { base.u16be() } else { base.u16le() }),
            ("type", if is_be { base.u16be() } else { base.u16le() }),
            ("length", if is_be { base.u32be() } else { base.u32le() }),
            (
                "offset-or-data",
                if is_be { base.u32be() } else { base.u32le() },
            ),
            // TODO: Offset from start of the TIFF header for values longer than 4 bytes
        ])
    };

    // Image file directory
    let ifd = |is_be: bool| {
        record([
            (
                "num-fields",
                if is_be { base.u16be() } else { base.u16le() },
            ),
            ("fields", repeat_count(Expr::Var(0), ifd_field(is_be))),
            (
                "next-ifd-offset",
                if is_be { base.u32be() } else { base.u32le() },
            ),
            // TODO: Offset from start of the TIFF header (i.e. `offset + 2 + num-fields * 12`)
            // TODO: Recursive call to `ifd(is_be)`
            ("next-ifd", repeat(base.u8())),
        ])
    };

    module.define_format(
        "tiff.main",
        record([
            (
                "byte-order",
                alts([
                    (
                        "le",
                        Format::Map(Func::Expr(Expr::UNIT), Box::new(is_bytes(b"II"))),
                    ),
                    (
                        "be",
                        Format::Map(Func::Expr(Expr::UNIT), Box::new(is_bytes(b"MM"))),
                    ),
                ]),
            ),
            (
                "magic",
                Format::Match(
                    Expr::Var(0), // byte-order
                    vec![
                        (Pattern::variant("le", Pattern::UNIT), base.u16le()), // 42
                        (Pattern::variant("be", Pattern::UNIT), base.u16be()), // 42
                    ],
                ),
            ),
            (
                "offset",
                Format::Match(
                    Expr::Var(1), // byte-order
                    vec![
                        (Pattern::variant("le", Pattern::UNIT), base.u32le()),
                        (Pattern::variant("be", Pattern::UNIT), base.u32be()),
                    ],
                ),
            ),
            (
                "ifd",
                Format::WithRelativeOffset(
                    // TODO: Offset from start of the TIFF header
                    Expr::Sub(Box::new(Expr::Var(0)), Box::new(Expr::U32(8))),
                    Box::new(Format::Match(
                        Expr::Var(2), // byte-order
                        vec![
                            (Pattern::variant("le", Pattern::UNIT), ifd(false)),
                            (Pattern::variant("be", Pattern::UNIT), ifd(true)),
                        ],
                    )),
                ),
            ),
        ]),
    )
}
