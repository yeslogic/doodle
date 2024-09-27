use super::*;

pub type TestResult<T = ()> = Result<T, Box<dyn Send + Sync + std::error::Error>>;

// Stabilization aliases to avoid hard-coding shifting numbers as formats are enriched with more possibilities
pub type Top = main_data;
pub type OpentypeData = opentype_main;
pub type TarBlock = tar_main;
pub type PngData = png_main;
pub type JpegData = jpeg_main;
pub type JpegApp01 = jpeg_frame_initial_segment;
pub type JfifData = jpeg_app0_jfif;
pub type TiffData = tiff_main;
pub type App0Data = jpeg_app0_data_data;
pub type App1Data = jpeg_app1_data;
pub type ExifData = jpeg_app1_exif;
pub type XmpData = jpeg_app1_xmp;
pub type GifData = gif_main;
pub type GifLogicalScreenDesc = gif_logical_screen_descriptor;
pub type RiffData = riff_main;
pub type ExifByteOrder = tiff_main_byte_order;
pub type GzipChunk = gzip_main;

pub fn try_decode_gzip(test_file: &str) -> TestResult<Vec<GzipChunk>> {
    let buffer = std::fs::read(std::path::Path::new(test_file))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::gzip(dat) => Ok(dat),
        other => unreachable!("expected gzip, found {other:?}"),
    }
}

pub mod png_metrics {
    use super::*;
    use std::fmt::Write;

    fn abbrev(buf: &mut String, data: &[u8]) -> std::fmt::Result {
        const CUTOFF: usize = 16;
        const MARGIN: usize = 4;
        write!(buf, "[")?;
        if data.len() > CUTOFF {
            let lead = &data[..MARGIN];
            let trail = &data[data.len() - MARGIN..];
            let skip = data.len() - 2 * MARGIN;
            for byte in lead {
                write!(buf, "{:02x}", byte)?;
            }
            write!(buf, "...({} bytes skipped)...", skip)?;
            for byte in trail {
                write!(buf, "{:02x}", byte)?;
            }
        } else {
            for byte in data {
                write!(buf, "{:02x}", byte)?;
            }
        }
        write!(buf, "]")
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct GenericMetrics {
        count: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct OptZlibMetrics {
        is_compressed: bool,
    }

    #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct SingleZlibMetrics {
        is_present: bool,
        // opt_invalid_bytes: Option<Vec<u8>>,
    }

    pub type SbitMetrics = GenericMetrics;
    pub type SpltMetrics = GenericMetrics;
    pub type HistMetrics = GenericMetrics;
    pub type SrgbMetrics = GenericMetrics;
    pub type BkgdMetrics = GenericMetrics;
    pub type ChrmMetrics = GenericMetrics;
    pub type GamaMetrics = GenericMetrics;
    pub type IccpMetrics = SingleZlibMetrics;
    pub type PhysMetrics = GenericMetrics;

    pub type ItxtMetrics = Vec<OptZlibMetrics>;
    pub type ZtxtMetrics = GenericMetrics;

    pub type TextMetrics = GenericMetrics;
    pub type TimeMetrics = GenericMetrics;
    pub type TrnsMetrics = GenericMetrics;

    #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct PngMetrics {
        tRNS: TrnsMetrics,
        cHRM: ChrmMetrics,
        gAMA: GamaMetrics,
        iCCP: IccpMetrics,
        sBIT: SbitMetrics,
        sRGB: SrgbMetrics,
        iTXt: ItxtMetrics,
        tEXt: TextMetrics,
        zTXt: ZtxtMetrics,
        bKGD: BkgdMetrics,
        hIST: HistMetrics,
        pHYs: PhysMetrics,
        sPLT: SpltMetrics,
        tIME: TimeMetrics,
    }

    pub fn analyze_png(test_file: &str) -> TestResult<PngMetrics> {
        let buffer = std::fs::read(std::path::Path::new(test_file))?;
        let mut input = Parser::new(&buffer);
        let dat = Decoder9(&mut input)?;
        let mut metrics = PngMetrics::default();
        for chunk in dat.chunks.iter().chain(dat.more_chunks.iter()) {
            match &chunk.data {
                png_chunk_data::PLTE(_) => (), // ignoring critical chunk PLTE
                png_chunk_data::sRGB(_) => metrics.sRGB.count += 1,
                png_chunk_data::bKGD(_) => metrics.bKGD.count += 1,
                png_chunk_data::cHRM(_) => metrics.cHRM.count += 1,
                png_chunk_data::gAMA(_) => metrics.gAMA.count += 1,
                png_chunk_data::iCCP(_) => {
                    metrics.iCCP.is_present = true;
                }
                png_chunk_data::iTXt(x) => match x.compression_flag {
                    0 => metrics.iTXt.push(OptZlibMetrics {
                        is_compressed: false,
                    }),
                    1 => metrics.iTXt.push(OptZlibMetrics {
                        is_compressed: true,
                    }),
                    other => unreachable!("compression flag {other} is not recognized"),
                },
                png_chunk_data::pHYs(_) => metrics.pHYs.count += 1,
                png_chunk_data::tEXt(_) => metrics.tEXt.count += 1,
                png_chunk_data::tIME(_) => metrics.tIME.count += 1,
                png_chunk_data::tRNS(_) => metrics.tRNS.count += 1,
                png_chunk_data::zTXt(_) => metrics.zTXt.count += 1,
                png_chunk_data::hIST(_) => metrics.hIST.count += 1,
                png_chunk_data::sBIT(_) => metrics.sBIT.count += 1,
                png_chunk_data::sPLT(_) => metrics.sPLT.count += 1,
                png_chunk_data::unknown(_) => eprintln!(
                    "unknown png chunk type: {}",
                    String::from_utf8_lossy(&chunk.tag)
                ),
            }
        }
        Ok(metrics)
    }

    pub fn collate_png_table<S: std::fmt::Display>(samples: &[(S, PngMetrics)]) {
        let header = [
            "bKGD", "cHRM", "gAMA", "hIST", "iCCP", "iTXt", "pHYs", "sBIT", "sPLT", "sRGB", "tEXt",
            "tIME", "tRNS", "zTXt", "Filename",
        ];
        let header_line = header.join("\t");

        fn write_metrics(buf: &mut String, metrics: &PngMetrics) {
            let show_count = |buf: &mut String, metrics: &GenericMetrics| match metrics.count {
                0 => buf.push_str("❌\t"),
                1 => buf.push_str("✅\t"),
                2.. => buf.push_str("➕\t"),
            };

            let show_single_zlib =
                |buf: &mut String, metrics: &SingleZlibMetrics| match metrics.is_present {
                    true => buf.push_str("✅\t"),
                    false => buf.push_str("❌\t"),
                };

            let show_count_optzlib = |buf: &mut String, metrics: &Vec<OptZlibMetrics>| {
                let mut all = true;
                let mut any = false;
                for m in metrics {
                    all = all && m.is_compressed;
                    any = any || m.is_compressed;
                }
                match (all, any) {
                    (true, false) => buf.push_str("❌\t"), // only possible when empty
                    (true, true) => buf.push_str("✅\t"),
                    (false, true) => buf.push_str("⭕\t"),
                    (false, false) => buf.push_str("❓\t"),
                }
            };

            show_count(buf, &metrics.bKGD);
            show_count(buf, &metrics.cHRM);
            show_count(buf, &metrics.gAMA);
            show_count(buf, &metrics.hIST);
            show_single_zlib(buf, &metrics.iCCP);
            show_count_optzlib(buf, &metrics.iTXt);
            show_count(buf, &metrics.pHYs);
            show_count(buf, &metrics.sBIT);
            show_count(buf, &metrics.sPLT);
            show_count(buf, &metrics.sRGB);
            show_count(buf, &metrics.tEXt);
            show_count(buf, &metrics.tIME);
            show_count(buf, &metrics.tRNS);
            show_count(buf, &metrics.zTXt);
        }

        println!("{header_line}");
        for (sample, metrics) in samples.iter() {
            let mut line = String::new();
            write_metrics(&mut line, metrics);
            println!("{line}{sample}");
        }
    }
}

pub mod oft_metrics {
    use super::*;

    pub type OpentypeFontDirectory = opentype_table_directory;
    pub type OpentypeGlyph = opentype_glyf_table;
    pub type GlyphDescription = opentype_glyf_description;
    pub type SimpleGlyph = opentype_glyf_simple;
    pub type OpentypeCmap = opentype_cmap_table;
    pub type OpentypeHead = opentype_head_table;

    pub fn show_opentype_stats(parsed_data: &OpentypeData) {
        // STUB - show more specific data
        show_magic(parsed_data.font.magic);
        match &parsed_data.font.directory {
            opentype_font_directory::TTCHeader(ttc) => {
                println!("TTC: version {}.{}", ttc.major_version, ttc.minor_version);
                match &ttc.header {
                    opentype_ttc_header_header::UnknownVersion(n) => {
                        println!("<unrecognized version {}.*", n)
                    }
                    opentype_ttc_header_header::Version1(hv1) => {
                        println!("TTC Version 1 ({} fonts)", hv1.num_fonts);
                        for (i, table_dir) in hv1.table_directories.iter().enumerate() {
                            match table_dir.link.as_ref() {
                                Some(table_dir) => {
                                    println!("=== Font @ Index {i} ===");
                                    show_font_directory(table_dir);
                                }
                                None => {
                                    println!("=== Skipping Index {i} ===");
                                }
                            }
                        }
                    }
                    opentype_ttc_header_header::Version2(hv2) => {
                        println!("TTC Version 2 ({} fonts)", hv2.num_fonts);
                        for (i, table_dir) in hv2.table_directories.iter().enumerate() {
                            match table_dir.link.as_ref() {
                                Some(table_dir) => {
                                    println!("=== Font @ Index {i} ===");
                                    show_font_directory(table_dir);
                                }
                                None => {
                                    println!("=== Skipping Index {i} ===");
                                }
                            }
                        }
                    }
                }
            }
            opentype_font_directory::TableDirectory(table_dir) => show_font_directory(table_dir),
            opentype_font_directory::UnknownTable => println!("<unknown>"),
        }
    }

    fn show_magic(magic: u32) {
        let bytes = magic.to_be_bytes();
        let show = |b: u8| {
            if b.is_ascii_alphanumeric() {
                String::from(b as char)
            } else {
                format!("{:02x}", b)
            }
        };
        println!(
            "Magic: {}{}{}{}",
            show(bytes[0]),
            show(bytes[1]),
            show(bytes[2]),
            show(bytes[3])
        );
    }

    fn show_font_directory(table_dir: &OpentypeFontDirectory) {
        let links = &table_dir.table_links;
        // STUB - add in more tailored printing of each table according to what it contains
        show_cmap_table(&links.cmap);
        show_head_table(&links.head);
        // println!("hhea: {:?}", &links.hhea);
        // println!("hmtx: {:?}", &links.hmtx);
        // println!("maxp: {:?}", &links.maxp);
        // println!("name: {:?}", &links.name);
        // println!("os2: {:?}", &links.os2);
        // println!("post: {:?}", &links.post);
        // println!("cvt: {:?}", &links.cvt);
        // println!("fpgm: {:?}", &links.fpgm);
        // println!("loca: {:?}", &links.loca);
        show_glyph_table(&links.glyf);
    }

    fn format_version16dot16(v: u32) -> String {
        let major = v >> 16;
        let minor = (v & 0xf000) >> 12;
        format!("{}.{}", major, minor)
    }

    fn show_cmap_table(cmap: &OpentypeCmap) {
        println!(
            "cmap: table version {}, {} encoding tables",
            cmap.version, cmap.num_tables
        );
    }

    fn show_head_table(head: &OpentypeHead) {
        println!("head: {:?}", head);
    }

    fn show_glyph_table(glyf: &Option<Vec<OpentypeGlyph>>) {
        let Some(glyf) = glyf.as_ref() else {
            println!("glyf: <not present>");
            return;
        };

        let n_glyphs = glyf.len();
        println!("glyf: {} glyphs", n_glyphs);
        const SHOW_CUTOFF: usize = 16;
        const BEFORE_ELIPSIS: usize = 8;
        const AFTER_ELIPSIS: usize = 8;
        if n_glyphs > SHOW_CUTOFF {
            for i in 0..BEFORE_ELIPSIS {
                show_single_glyph(i, &glyf[i]);
            }
            println!(
                "skipping glyphs {}..{}",
                BEFORE_ELIPSIS,
                n_glyphs - AFTER_ELIPSIS
            );
            for i in n_glyphs - AFTER_ELIPSIS..n_glyphs {
                show_single_glyph(i, &glyf[i]);
            }
        } else {
            for i in 0..n_glyphs {
                show_single_glyph(i, &glyf[i]);
            }
        }
    }

    fn show_single_glyph(ix: usize, glyf: &OpentypeGlyph) {
        print!("[{ix}]: ");
        match glyf {
            OpentypeGlyph::EmptyGlyph => println!("<empty>"),
            OpentypeGlyph::Glyph(glyph) => match &glyph.description {
                GlyphDescription::HeaderOnly => println!("<empty (implied)>"),
                GlyphDescription::Simple(sglyph) => {
                    println!("Simple Glyph [{} contours, {} coordinates, {} instructions, xy: [({}, {}) <-> ({}, {})]]",
                        glyph.number_of_contours,
                        sglyph.end_points_of_contour.last().unwrap() + 1,
                        sglyph.instruction_length,
                        glyph.x_min, glyph.y_min, glyph.x_max, glyph.y_max
                    );
                }
                GlyphDescription::Composite(cglyph) => {
                    println!("Composite Glyph [{} components, {} instructions, xy: [({}, {}) <-> ({}, {})]]",
                        cglyph.glyphs.len(),
                        cglyph.instructions.len(),
                        glyph.x_min, glyph.y_min, glyph.x_max, glyph.y_max
                    );
                }
            },
        }
    }
}

pub use oft_metrics::*;
pub use png_metrics::*;
