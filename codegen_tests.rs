#![cfg(test)]

use super::*;

type TestResult<T = ()> = Result<T, Box<dyn Send + Sync + std::error::Error>>;

// Stablization aliases to avoid hard-coding shifting numbers as formats are enriched with more possibilities
type Top = Type204;
type TarBlock = Type202;
type PngData = Type193;
type JpegData = Type80;

#[test]
fn test_pngsig_decoder() {
    // PNG signature
    let input = b"\x89PNG\r\n\x1A\n";
    let mut parser = Parser::new(input);
    let ret = Decoder27(&mut parser);
    assert!(ret.is_ok());
}

#[test]
fn test_decoder_gif() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new("test.gif"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Top::gif(dat) => println!("{:?}", dat),
        other => unreachable!("expected gif, found {other:?}"),
    }
    Ok(())
}

mod gzip {
    use super::*;
    fn test_decoder_gzip(testfile: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(testfile))?;
        let mut input = Parser::new(&buffer);
        let oput = Decoder1(&mut input)?.data;
        match oput {
            Top::gzip(dat) => println!("{:?}", &dat[0].header),
            other => unreachable!("expected gzip, found {other:?}"),
        }
        Ok(())
    }

    #[test]
    fn test_decoder_gzip_test1() -> TestResult {
        test_decoder_gzip("test1.gz")
    }
    #[test]
    fn test_decoder_gzip_test2() -> TestResult {
        test_decoder_gzip("test2.gz")
    }
    #[test]
    fn test_decoder_gzip_test3() -> TestResult {
        test_decoder_gzip("test3.gz")
    }
    #[test]
    fn test_decoder_gzip_test4() -> TestResult {
        test_decoder_gzip("test4.gz")
    }
    #[test]
    fn test_decoder_gzip_test5() -> TestResult {
        test_decoder_gzip("test5.gz")
    }
    #[test]
    fn test_decoder_gzip_test6() -> TestResult {
        test_decoder_gzip("test6.gz")
    }
}

mod jpeg {
    use super::*;
    fn test_decoder_jpeg(testfile: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(testfile))?;
        let mut input = Parser::new(&buffer);
        let oput = Decoder1(&mut input)?.data;
        match oput {
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
fn test_decoder_peano() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new("test.peano"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Top::peano(x) => println!("PEANO: {x:#?}"),
        other => unreachable!("expected peano, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_mpeg4() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new("test.mp4"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Top::mpeg4(dat) => println!("{:?}", dat),
        other => unreachable!("expected mpeg4, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_png() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new("test.png"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Top::png(dat) => println!("{:?}", dat),
        other => unreachable!("expected png, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_riff() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new("test.webp"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Top::riff(dat) => println!("{:?}", dat),
        other => unreachable!("expected riff, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_tar() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new("test.tar"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder14(&mut input)?;
    match oput {
        TarBlock {
            header,
            file,
            __padding,
        } => {
            println!("HEADER\n{header:?}");
            println!("\nFILE\n{file:?}");
        }
    }
    Ok(())
}

#[test]
fn test_decoder_text_ascii() -> TestResult {
    let buffer = std::fs::read(std::path::Path::new("test.txt"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
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
    let buffer = std::fs::read(std::path::Path::new("test.utf8"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
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
    let buffer = std::fs::read(std::path::Path::new("mixed.utf8"))?;
    let mut input = Parser::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
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

    fn mk_sig_hex(sig: (u8, u8, u8, u8, u8, u8, u8, u8)) -> String {
        format!(
            "{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}",
            sig.0, sig.1, sig.2, sig.3, sig.4, sig.5, sig.6, sig.7
        )
    }

    fn print_png_breadcrumb(png_data: PngData) {
        let sig_hex = mk_sig_hex(png_data.signature);
        println!(
            "SIG ({}) | IHDR (len: {} | h {}px * w {}px)",
            sig_hex, png_data.ihdr.length, png_data.ihdr.data.height, png_data.ihdr.data.width,
        );
    }

    fn print_jpeg_breadcrumb(jpg_data: JpegData) {
        let (k, app01) = mk_app01(jpg_data.frame.initial_segment);
        println!("APP{k} ({})", app01);
    }

    fn mk_app01(seg: Type55) -> (u8, String) {
        match seg {
            Type55::app0(dat0) => {
                match dat0.data.data {
                    Type43::jfif(Type42 {
                        version_major,
                        version_minor,
                        density_units,
                        density_x,
                        density_y,
                        ..
                    }) => {
                        // 0 = unitless, 1 = ppi, 2 = ppcm
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
                    Type43::other(_other) => (0, String::from("Other <...>")),
                }
            }
            Type55::app1(dat1) => match dat1.data.data {
                Type52::exif(Type50 { exif: _exif, .. }) => (1, String::from("EXIF <...>")),
                Type52::xmp(Type51 { xmp: _xmp, .. }) => (1, String::from("XMP <...>")),
                Type52::other(_) => (1, String::from("Other <...>")),
            },
        }
    }

    fn check_png(filename: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(filename))?;
        let mut input = Parser::new(&buffer);
        print!("[{filename}]: ");
        let dat = Decoder7(&mut input)?;
        print_png_breadcrumb(dat);
        Ok(())
    }

    fn check_jpeg(filename: &str) -> TestResult {
        let buffer = std::fs::read(std::path::Path::new(filename))?;
        let mut input = Parser::new(&buffer);
        print!("[{filename}]: ");
        let dat = Decoder5(&mut input)?;
        print_jpeg_breadcrumb(dat);
        Ok(())
    }

    #[test]
    fn test_errant_png() -> TestResult {
        check_png("test-images/broken.png")
    }

    #[test]
    fn test_all_extra_images() -> TestResult {
        for entry in std::fs::read_dir("test-images")?.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            match () {
                _ if name.contains("broken.png") => {
                    assert!(check_png(format!("test-images/{}", name).as_str()).is_err());
                }
                _ if name.ends_with(".png") => {
                    check_png(format!("test-images/{}", name).as_str())?;
                }
                _ if name.ends_with(".jpg") || name.ends_with(".jpeg") => {
                    check_jpeg(format!("test-images/{}", name).as_str())?;
                }
                // FIXME: add more cases as we add handlers for each image type
                _ => continue,
            }
        }
        Ok(())
    }
}
