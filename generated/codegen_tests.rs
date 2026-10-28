#![cfg(test)]
use super::api_helper::*;
use super::*;

/// Directory holding the test-files of the kind whose location is given by `TEST_OBJ_SUFFIX_<$kind>`
/// (see `.cargo/config.toml`).
macro_rules! test_dir {
    ($kind:literal) => {
        concat!(
            env!("TEST_OBJ_ROOT"),
            "/",
            env!(concat!("TEST_OBJ_SUFFIX_", $kind))
        )
    };
}

fn testpath(dir: &str, filename: &str) -> String {
    format!("{dir}/{filename}")
}

#[test]
fn test_decoder_tgz() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("TGZ"),
        "test.tgz",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder5(&mut input)?;
    for block in parsed_data.iter().flat_map(|x| x.contents.iter()) {
        println!(
            "Filename: {:?}",
            std::str::from_utf8(&block.header.name.string).expect("bad text in tar filename")
        );
        println!("File: {:#?}", String::from_utf8_lossy(&block.file));
    }
    Ok(())
}

#[test]
fn test_decoder_font() -> TestResult {
    let metrics = otf_metrics::analyze_font(
        &testpath(test_dir!("FONT"), "SourceCodePro-Regular.otf"),
        false,
    )?;
    otf_metrics::output::show_opentype_stats(&metrics, &otf_metrics::Config::default());
    Ok(())
}

#[test]
fn test_decoder_gif() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("GIF"),
        "test.gif",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::gif(dat) => println!("{:?}", dat),
        other => unreachable!("expected gif, found {other:?}"),
    }
    Ok(())
}

mod gzip {
    use super::*;

    fn test_gzip_decode(filename: &str) -> TestResult {
        let dat = try_decode_gzip(&testpath(test_dir!("GZIP"), filename))?;
        println!("{:?}", &dat[0].header);
        Ok(())
    }

    #[test]
    fn test_decoder_gzip_test1() -> TestResult {
        test_gzip_decode("test1.gz")
    }
    #[test]
    fn test_decoder_gzip_test2() -> TestResult {
        test_gzip_decode("test2.gz")
    }
    #[test]
    fn test_decoder_gzip_test3() -> TestResult {
        test_gzip_decode("test3.gz")
    }
    #[test]
    fn test_decoder_gzip_test4() -> TestResult {
        test_gzip_decode("test4.gz")
    }
    #[test]
    fn test_decoder_gzip_test5() -> TestResult {
        test_gzip_decode("test5.gz")
    }
    #[test]
    fn test_decoder_gzip_test6() -> TestResult {
        test_gzip_decode("test6.gz")
    }
}

mod jpeg {
    use super::*;
    fn test_decoder_jpeg(test_file: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(&testpath(
            test_dir!("JPEG"),
            test_file,
        )))?;
        let mut input = Parser::new(&buffer);
        let parsed_data = Decoder1(&mut input)?.data;
        match parsed_data {
            Top::jpeg(dat) => println!("{:?}", dat.frame.header),
            other => unreachable!("expected jpeg, found {other:?}"),
        }
        Ok(())
    }

    #[test]
    fn test_decoder_jpeg_test_() -> TestResult {
        test_decoder_jpeg("test.jpg")
    }

    #[test]
    fn test_decoder_jpeg_test2() -> TestResult {
        test_decoder_jpeg("test2.jpg")
    }
}

#[test]
fn test_decoder_waldo() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("WALDO"),
        "test.waldo",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::waldo(x) => println!(
            "Waldo: Found at offset {} (noise length: {}): \"{}\"",
            x.r#where,
            x.noise.len(),
            String::from_utf8_lossy(x.waldo),
        ),
        other => unreachable!("expected waldo, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_peano() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("PEANO"),
        "test.peano",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::peano(x) => println!("PEANO: {x:#?}"),
        other => unreachable!("expected peano, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_mpeg4() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("MP4"),
        "test.mp4",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::mpeg4(dat) => println!("{:?}", dat),
        other => unreachable!("expected mpeg4, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_png() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("PNG"),
        "test.png",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::png(dat) => println!("{:?}", dat),
        other => unreachable!("expected png, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_riff() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("WEBP"),
        "test.webp",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::riff(dat) => println!("{:?}", dat),
        other => unreachable!("expected riff, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_tar() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("TAR"),
        "test.tar",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder_tar_header_with_data(&mut input)?;
    match parsed_data {
        TarBlock { header, file } => {
            println!("HEADER\n{header:?}");
            println!("\nFILE\n{file:?}");
        }
    }
    Ok(())
}

#[test]
fn test_decoder_text_ascii() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("TEXT"),
        "test.txt",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::text(chars) => {
            assert_eq!(
                chars,
                String::from_utf8(buffer)?.chars().collect::<Vec<char>>()
            );
        }
        other => unreachable!("expected text, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_text_utf8() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("UTF8"),
        "test.utf8",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::text(chars) => {
            assert_eq!(
                chars,
                String::from_utf8(buffer)?.chars().collect::<Vec<char>>()
            );
        }
        other => unreachable!("expected text, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_text_mixed() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new(&testpath(
        test_dir!("UTF8"),
        "mixed.utf8",
    )))?;
    let mut input = Parser::new(&buffer);
    let parsed_data = Decoder1(&mut input)?.data;
    match parsed_data {
        Top::text(chars) => {
            assert_eq!(
                chars,
                String::from_utf8(buffer)?.chars().collect::<Vec<char>>()
            );
        }
        other => unreachable!("expected text, found {other:?}"),
    }
    Ok(())
}

mod test_files {
    use super::*;

    fn mk_sig_hex(sig: &[u8]) -> String {
        format!(
            "{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
            sig[0], sig[1], sig[2], sig[3], sig[4], sig[5], sig[6], sig[7]
        )
    }

    fn print_png_breadcrumb(png_data: PngData) {
        let sig_hex = mk_sig_hex(&png_data.signature);
        println!(
            "SIG ({}) | IHDR (h {}px * w {}px)",
            sig_hex, png_data.ihdr.data.height, png_data.ihdr.data.width,
        );
    }

    fn print_jpeg_breadcrumb(jpg_data: JpegData) {
        let (k, app01) = mk_app01(jpg_data.frame.initial_segment);
        println!("APP{k} ({})", app01);
    }

    fn print_gif_breadcrumb(gif_data: GifData) {
        let GifLogicalScreenDesc {
            screen_width,
            screen_height,
            ..
        } = gif_data.logical_screen.descriptor;
        println!("GIF ({}w x {}h)", screen_width, screen_height);
    }

    fn print_riff_breadcrumb(dat: RiffData) {
        let RiffData { length, data, .. } = dat;
        let sub_tag = match data.tag {
            (hh, hl, lh, ll) => String::from_utf8_lossy(&[hh, hl, lh, ll]).into_owned(),
        };
        println!(
            "RIFF Container (length: {} bytes, data tag: `{}`)",
            length, sub_tag
        );
    }

    fn print_tiff_breadcrumb(tiff_data: TiffData) {
        let TiffData { byte_order, .. } = tiff_data;
        let bc_byte_order = match byte_order {
            ExifByteOrder::be(..) => "Big-Endian",
            ExifByteOrder::le(..) => "Little-Endian",
        };
        println!("TIFF ({})", bc_byte_order);
    }

    fn mk_app01(seg: JpegApp01) -> (u8, String) {
        match seg {
            JpegApp01::app0(dat0) => {
                match dat0.data.data {
                    App0Data::jfif(JfifData {
                        version_major,
                        version_minor,
                        density_units,
                        density_x,
                        density_y,
                        ..
                    }) => {
                        // 0 = density ratio w/o units, 1 = pixels per inch, 2 = pixels per cm
                        let density = match density_units {
                            0 => format!("{}:{}", density_x, density_y),
                            1 => format!("{}x{} ppi", density_x, density_y),
                            2 => format!("{}x{} ppcm", density_x, density_y),
                            _other => unreachable!("bad density units (!= 0, 1, 2): {_other}"),
                        };
                        (
                            0,
                            format!("JFIF v{}.{:02} | {density} ", version_major, version_minor),
                        )
                    }
                    App0Data::other(_other) => (0, String::from("Other <...>")),
                }
            }
            JpegApp01::app1(dat1) => match dat1.data.data {
                App1Data::exif(ExifData { exif: _exif, .. }) => (1, String::from("EXIF <...>")),
                App1Data::xmp(XmpData { xmp: _xmp, .. }) => (1, String::from("XMP <...>")),
                App1Data::other(_) => (1, String::from("Other <...>")),
            },
        }
    }

    fn check_png(filename: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(filename))?;
        let mut input = Parser::new(&buffer);
        print!("[{filename}]: ");
        let dat = Decoder_png_main(&mut input)?;
        print_png_breadcrumb(dat);
        Ok(())
    }

    fn check_jpeg(filename: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(filename))?;
        let mut input = Parser::new(&buffer);
        print!("[{filename}]: ");
        let dat = Decoder_jpeg_main(&mut input)?;
        print_jpeg_breadcrumb(dat);
        Ok(())
    }

    fn check_gif(filename: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(filename))?;
        let mut input = Parser::new(&buffer);
        print!("[{filename}]: ");
        let dat = Decoder_gif_main(&mut input)?;
        print_gif_breadcrumb(dat);
        Ok(())
    }

    fn check_tiff(filename: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(filename))?;
        let mut input = Parser::new(&buffer);
        print!("[{filename}]: ");
        let dat = Decoder_tiff_main(&mut input)?;
        print_tiff_breadcrumb(dat);
        Ok(())
    }

    fn check_riff(filename: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(filename))?;
        let mut input = Parser::new(&buffer);
        print!("[{filename}]: ");
        let dat = Decoder_riff_main(&mut input)?;
        print_riff_breadcrumb(dat);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "UnstackableSlices")]
    fn test_errant_png() {
        check_png(&testpath(test_dir!("IMAGE_CORPUS"), "broken.png")).unwrap()
    }

    #[test]
    fn test_all_extra_images() -> TestResult {
        let mut residue = Vec::new();
        for entry in std::fs::read_dir(test_dir!("IMAGE_CORPUS"))?.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            match () {
                _ if name.contains("broken.png") => {
                    assert!(
                        check_png(testpath(test_dir!("IMAGE_CORPUS"), &name).as_str()).is_err()
                    );
                    println!("Broken PNG (expected)");
                }
                _ if name.ends_with(".png") => {
                    check_png(testpath(test_dir!("IMAGE_CORPUS"), &name).as_str())?;
                }
                _ if name.ends_with(".jpg") || name.ends_with(".jpeg") => {
                    check_jpeg(testpath(test_dir!("IMAGE_CORPUS"), &name).as_str())?;
                }
                _ if name.ends_with(".gif") => {
                    check_gif(testpath(test_dir!("IMAGE_CORPUS"), &name).as_str())?;
                }
                _ if name.ends_with(".tif") => {
                    check_tiff(testpath(test_dir!("IMAGE_CORPUS"), &name).as_str())?;
                }
                _ if name.ends_with(".webp") => {
                    check_riff(testpath(test_dir!("IMAGE_CORPUS"), &name).as_str())?;
                }
                // FIXME: add more cases as we add handlers for each image type
                _ => {
                    residue.push(format!(
                        "[{}]: Skipping...",
                        testpath(test_dir!("IMAGE_CORPUS"), &name)
                    ));
                }
            }
        }
        if !residue.is_empty() {
            for mesg in residue {
                eprintln!("{}", mesg);
            }
        } else {
            println!("==== ALL FILES COVERED ====");
        }
        Ok(())
    }
}

mod bson {
    use super::*;

    /// Summary statistics gathered by walking a decoded BSON document tree.
    #[derive(Debug, Default, PartialEq, Eq)]
    struct Stats {
        /// Maximum nesting depth of `document`/`array`/scope bodies (top-level = 1).
        max_depth: usize,
        /// Sum of every `int32` value at any depth.
        int32_sum: i64,
        /// Number of `code_w_scope` elements at any depth.
        scopes: usize,
    }

    fn walk(doc: &bson_document, depth: usize, stats: &mut Stats) {
        stats.max_depth = stats.max_depth.max(depth);
        for elem in doc.elements.iter() {
            match elem {
                bson_element::int32(x) => stats.int32_sum += x.value as i64,
                bson_element::document(x) => walk(&x.value, depth + 1, stats),
                bson_element::array(x) => walk(&x.value, depth + 1, stats),
                bson_element::code_w_scope(x) => {
                    stats.scopes += 1;
                    walk(&x.value.scope, depth + 1, stats)
                }
                _ => {}
            }
        }
    }

    fn check_bson(filename: &str, expected: Stats) -> TestResult {
        let buffer = std::fs::read(testpath(test_dir!("BSON"), filename))?;
        let mut input = Parser::new(&buffer);
        let parsed_data = Decoder1(&mut input)?.data;
        match parsed_data {
            Top::bson(dat) => {
                let mut stats = Stats::default();
                walk(&dat.document, 1, &mut stats);
                assert_eq!(stats, expected, "{filename}");
            }
            other => unreachable!("expected bson, found {other:?}"),
        }
        Ok(())
    }

    #[test]
    fn test_decoder_bson_mixed_nesting() -> TestResult {
        let expected = Stats {
            max_depth: 6,
            int32_sum: 61,
            scopes: 1,
        };
        check_bson("test_document_mixed_nesting.bson", expected)
    }

    #[test]
    fn test_decoder_bson_array_nested() -> TestResult {
        let expected = Stats {
            max_depth: 5,
            int32_sum: 15,
            scopes: 0,
        };
        check_bson("test_document_array_nested.bson", expected)
    }

    #[test]
    fn test_decoder_bson_scope_nested() -> TestResult {
        let expected = Stats {
            max_depth: 4,
            int32_sum: 9,
            scopes: 2,
        };
        check_bson("test_document_scope_nested.bson", expected)
    }
}
