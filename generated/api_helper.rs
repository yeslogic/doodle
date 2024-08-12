use super::*;

pub type TestResult<T = ()> = Result<T, Box<dyn Send + Sync + std::error::Error>>;

// Stabilization aliases to avoid hard-coding shifting numbers as formats are enriched with more possibilities
pub type Top = main;
pub type TarBlock = main_tar_contents_inSeq;
pub type PngData = main_png;
pub type JpegData = main_jpeg;
pub type JpegApp01 = main_jpeg_frame_initial_segment;
pub type JfifData = main_jpeg_frame_initial_segment_app0_data_data_jfif;
pub type TiffData = main_jpeg_frame_initial_segment_app1_data_data_exif_exif;
pub type App0Data = main_jpeg_frame_initial_segment_app0_data_data;
pub type App1Data = main_jpeg_frame_initial_segment_app1_data_data;
pub type ExifData = main_jpeg_frame_initial_segment_app1_data_data_exif;
pub type XmpData = main_jpeg_frame_initial_segment_app1_data_data_xmp;
pub type GifData = main_gif;
pub type GifLogicalScreenDesc = main_gif_logical_screen_descriptor;
pub type RiffData = main_riff;
pub type ExifByteOrder = main_jpeg_frame_initial_segment_app1_data_data_exif_exif_byte_order;
pub type GzipChunk = main_gzip_inSeq;

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
    use doodle::output::Fragment;

    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct GenericMetrics {
        count: usize,
    }


    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct OptZlibMetrics {
        is_compressed: bool,
    }



    pub type BkgdMetrics = GenericMetrics;
    pub type ChrmMetrics = GenericMetrics;
    pub type GamaMetrics = GenericMetrics;
    pub type IccpMetrics = GenericMetrics;
    pub type PhysMetrics = GenericMetrics;

    pub type ItxtMetrics = Vec<OptZlibMetrics>;
    pub type ZtxtMetrics = GenericMetrics;

    pub type TextMetrics = GenericMetrics;
    pub type TimeMetrics = GenericMetrics;
    pub type TrnsMetrics = GenericMetrics;



    #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    pub struct PngMetrics {
        bKGD: BkgdMetrics,
        cHRM: ChrmMetrics,
        gAMA: GamaMetrics,
        iCCP: IccpMetrics,
        iTXt: ItxtMetrics,
        pHYs: PhysMetrics,
        tEXt: TextMetrics,
        tIME: TimeMetrics,
        tRNS: TrnsMetrics,
        zTXt: ZtxtMetrics,
    }

    pub fn analyze_png(test_file: &str) -> TestResult<PngMetrics> {
        let buffer = std::fs::read(std::path::Path::new(test_file))?;
        let mut input = Parser::new(&buffer);
        let dat = Decoder9(&mut input)?;
        let mut metrics = PngMetrics::default();
        for chunk in dat.chunks.iter().chain(dat.more_chunks.iter()) {
            match chunk {
                main_png_chunks_inSeq::PLTE(_) => (), // ignoring critical chunk PLTE
                main_png_chunks_inSeq::bKGD(_) => metrics.bKGD.count += 1,
                main_png_chunks_inSeq::cHRM(_) => metrics.cHRM.count += 1,
                main_png_chunks_inSeq::gAMA(_) => metrics.gAMA.count += 1,
                main_png_chunks_inSeq::iCCP(_) => metrics.iCCP.count += 1,
                main_png_chunks_inSeq::iTXt(x) => {
                    match x.data.compression_flag {
                        0 => metrics.iTXt.push(OptZlibMetrics { is_compressed: false }),
                        1 => metrics.iTXt.push(OptZlibMetrics { is_compressed: true }),
                        other => unreachable!("compression flag {other} is not recognized"),
                    }
                }
                main_png_chunks_inSeq::pHYs(_) => metrics.pHYs.count += 1,
                main_png_chunks_inSeq::tEXt(_) => metrics.tEXt.count += 1,
                main_png_chunks_inSeq::tIME(_) => metrics.tIME.count += 1,
                main_png_chunks_inSeq::tRNS(_) => metrics.tRNS.count += 1,
                main_png_chunks_inSeq::zTXt(_) => metrics.zTXt.count += 1,
            }
        }
        Ok(metrics)
    }

    pub fn collate_png_table<S: std::fmt::Display>(samples: &[(S, PngMetrics)]) {
        let header = ["bKGD", "cHRM", "gAMA", "iCCP", "iTXt", "pHYs", "tEXt", "tIME", "tRNS", "zTXt", "Filename"];
        let header_line = header.join("\t");

        fn write_metrics(buf: &mut String, metrics: &PngMetrics) {
            let show_count = |buf: &mut String, metrics: &GenericMetrics| {
                match metrics.count {
                    0 => buf.push_str("❌\t"),
                    1 => buf.push_str("✅\t"),
                    2.. => buf.push_str("➕\t"),
                }
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
            show_count(buf, &metrics.iCCP);
            show_count_optzlib(buf, &metrics.iTXt);
            show_count(buf, &metrics.pHYs);
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

pub use png_metrics::*;
