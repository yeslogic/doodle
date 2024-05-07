#![cfg(test)]

use super::*;

#[test]
fn test_decoder_26() {
    // PNG signature
    let input = b"\x89PNG\r\n\x1A\n";
    let mut parser = ParseMonad::new(input);
    let ret = Decoder26(&mut parser);
    assert!(ret.is_ok());
}

#[test]
fn test_decoder_gif() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.gif"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::gif(dat) => println!("{:?}", dat),
        other => unreachable!("expected gif, found {other:?}"),
    }
    Ok(())
}

mod gzip {
    use super::*;
    fn test_decoder_gzip(testfile: &str) -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
        let buffer = std::fs::read(std::path::Path::new(testfile))?;
        let mut input = ParseMonad::new(&buffer);
        let oput = Decoder1(&mut input)?.data;
        match oput {
            Type192::gzip(dat) => println!("{:?}", &dat[0].header),
            other => unreachable!("expected gzip, found {other:?}"),
        }
        Ok(())
    }

    #[test]
    fn test_decoder_gzip_test1() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
        test_decoder_gzip("test1.gz")
    }
    #[test]
    fn test_decoder_gzip_test2() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
        test_decoder_gzip("test2.gz")
    }
    #[test]
    fn test_decoder_gzip_test3() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
        test_decoder_gzip("test3.gz")
    }
    #[test]
    fn test_decoder_gzip_test4() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
        test_decoder_gzip("test4.gz")
    }
    #[test]
    fn test_decoder_gzip_test5() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
        test_decoder_gzip("test5.gz")
    }
    #[test]
    fn test_decoder_gzip_test6() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
        test_decoder_gzip("test6.gz")
    }
}

#[test]
fn test_decoder_jpeg() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.jpg"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::jpeg(dat) => println!("{:?}", dat),
        other => unreachable!("expected jpeg, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_mpeg4() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.mp4"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::mpeg4(dat) => println!("{:?}", dat),
        other => unreachable!("expected mpeg4, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_png() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.png"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::png(dat) => println!("{:?}", dat),
        other => unreachable!("expected png, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_riff() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.webp"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::riff(dat) => println!("{:?}", dat),
        other => unreachable!("expected riff, found {other:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_tar() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.tar"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder13(&mut input)?;
    match oput {
        Type190 {
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
fn test_decoder_text_ascii() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.txt"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::text(chars) => {
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
fn test_decoder_text_utf8() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.utf8"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::text(chars) => {
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
fn test_decoder_text_mixed() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("mixed.utf8"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder1(&mut input)?.data;
    match oput {
        Type192::text(chars) => {
            assert_eq!(
                chars,
                String::from_utf8(buffer)?.chars().collect::<Vec<char>>()
            );
        }
        other => unreachable!("expected text, found {other:?}"),
    }
    Ok(())
}
