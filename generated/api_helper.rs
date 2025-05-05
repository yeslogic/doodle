use super::*;

pub type TestResult<T = ()> = Result<T, Box<dyn Send + Sync + std::error::Error>>;

// Stabilization aliases to avoid hard-coding shifting numbers as formats are enriched with more possibilities
pub type Top = main_data;
pub type OpentypeData = opentype_main;
pub type TarBlock = tar_header_with_data;
pub type PngData = png_main;
pub type JpegData = jpeg_main;
pub type JpegApp01 = jpeg_frame_initial_segment;
pub type JfifData = jpeg_app0_jfif;
pub type TiffData = tiff_main;
pub type App0Data = jpeg_app0_data_data;
pub type App1Data = jpeg_app1_data_data;
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
        let dat = Decoder_png_main(&mut input)?;
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

            let show_count_opt_zlib = |buf: &mut String, metrics: &Vec<OptZlibMetrics>| {
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
            show_count_opt_zlib(buf, &metrics.iTXt);
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

pub mod elf_info {
    use super::*;

    pub fn scan_elf(input: &mut impl std::io::Read) -> TestResult<()> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        let mut parser = Parser::new(&buf);
        let elf = Decoder_elf_main(&mut parser)?;
        println!("{:?}", elf);
        Ok(())
    }

    pub fn analyze_elf(filename: &str) -> TestResult<()> {
        let file = std::fs::File::open(std::path::Path::new(filename))?;
        let mut input = std::io::BufReader::new(file);
        scan_elf(&mut input)
    }
}

#[cfg(feature = "rle")]
pub mod rle_scan {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
    pub enum Style {
        OldStyle = 0,
        NewStyle = 1,
    }

    impl std::fmt::Display for Style {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Style::OldStyle => write!(f, "old-style"),
                Style::NewStyle => write!(f, "new-style"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct RunInfo {
        run_count: u32,
        run_length_bounds: (u16, u16),
    }

    #[derive(Debug, Clone)]
    pub struct RleStat {
        style: Style,
        input_len: u32,
        run_info: Option<RunInfo>,
        data: String,
    }

    pub fn scan_rle(input: &mut impl std::io::Read) -> TestResult<RleStat> {
        let mut buf = Vec::new();
        input.read_to_end(&mut buf)?;
        // subtract one byte for the leading sentinel byte, which is artificially tacked on
        let input_len = buf.len() as u32 - 1;
        let mut parser = Parser::new(&buf);
        match Decoder_rle_main(&mut parser)? {
            rle_main::new_style(new_style) => {
                let data = String::from_utf8(new_style.data)?;
                Ok(RleStat {
                    style: Style::NewStyle,
                    input_len,
                    run_info: None,
                    data,
                })
            }
            rle_main::old_style(old_style) => {
                let data = String::from_utf8(old_style.data)?;
                let mut run_length_min = 256;
                let mut run_length_max = 0;
                for run in old_style.runs.iter() {
                    let len = run.len;
                    run_length_min = std::cmp::min(run_length_min, len as u16);
                    run_length_max = std::cmp::max(run_length_max, len as u16);
                }
                if run_length_min > 255 {
                    run_length_min = 0;
                }
                let run_info = Some(RunInfo {
                    run_count: old_style.runs.len() as u32,
                    run_length_bounds: (run_length_min, run_length_max),
                });
                Ok(RleStat {
                    style: Style::OldStyle,
                    input_len,
                    run_info,
                    data,
                })
            }
        }
    }

    pub fn analyze_rle(filename: &str) -> TestResult<()> {
        let file = std::fs::File::open(std::path::Path::new(filename))?;
        let mut input = std::io::BufReader::new(file);
        let RleStat {
            style,
            input_len,
            run_info,
            data,
        } = scan_rle(&mut input)?;
        if let Some(RunInfo {
            run_count,
            run_length_bounds,
        }) = run_info
        {
            let (run_length_min, run_length_max) = run_length_bounds;
            println!(
                "RLE ({filename}): style={style}, len={input_len}, data={data:?}, run_count={run_count}, [min={run_length_min}, max={run_length_max}] (compression: {:.2}%)",
                input_len as f64 / data.len() as f64 * 100.0
            )
        } else {
            println!(
                "RLE ({filename}): style={style}, len={input_len}, data={data:?} (compression: {:.2}%)",
                input_len as f64 / data.len() as f64 * 100.0
            )
        }
        Ok(())
    }
}

pub mod otf_metrics;
pub(crate) mod util;

pub use png_metrics::*;
