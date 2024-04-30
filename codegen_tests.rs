#![cfg(test)]
use super::*;

#[test]
fn test_decoder_unicode() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("test.utf8"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder9(&mut input);
    match oput {
        Ok(chars) => {
            assert_eq!(
                chars,
                String::from_utf8(buffer)?.chars().collect::<Vec<char>>()
            );
        },
        Err(err) => panic!("bad parse: {err:?}"),
    }
    Ok(())
}

#[test]
fn test_decoder_unicode_ascii_prefix() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let buffer = std::fs::read(std::path::Path::new("mixed.utf8"))?;
    let mut input = ParseMonad::new(&buffer);
    let oput = Decoder9(&mut input);
    match oput {
        Ok(chars) => {
            assert_eq!(
                chars,
                String::from_utf8(buffer)?.chars().collect::<Vec<char>>()
            );
        },
        Err(err) => panic!("bad parse: {err:?}"),
    }
    Ok(())
}


#[test]

fn test_decoder_26() {
    // PNG signature
    let input = b"\x89PNG\r\n\x1A\n";
    let mut parser = ParseMonad::new(input);
    let ret = Decoder26(&mut parser);
    assert!(ret.is_ok());
}
