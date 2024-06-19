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
