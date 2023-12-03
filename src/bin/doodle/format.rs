use doodle::{Format, FormatModule, FormatRef};

use crate::format::base::*;

mod base;

mod deflate;
mod gif;
mod gzip;
mod jpeg;
mod png;
mod riff;
mod tar;
mod text;
mod tiff;

pub fn main(module: &mut FormatModule) -> FormatRef {
    let base = base::main(module);

    let deflate = deflate::main(module, &base);
    let tiff = tiff::main(module, &base);
    let gif = gif::main(module, &base);
    let gzip = gzip::main(module, deflate, &base);
    let jpeg = jpeg::main(module, &base, &tiff);
    let png = png::main(module, &base);
    let riff = riff::main(module, &base);
    let tar = tar::main(module, &base);
    let text = text::main(module, &base);

    module.define_format(
        "main",
        record([
            (
                "data",
                Format::UnionNondet(vec![
                    ("gif".into(), gif.call()),
                    ("gzip".into(), gzip.call()),
                    ("jpeg".into(), jpeg.call()),
                    ("png".into(), png.call()),
                    ("riff".into(), riff.call()),
                    ("tar".into(), tar.call()),
                    ("text".into(), text.call()),
                ]),
            ),
            ("end", Format::EndOfInput),
        ]),
    )
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::*;
    use doodle::{byte_set::ByteSet, decoder::Value, error::ParseError, read::ReadCtxt, Expr};

    #[test]
    fn with_relative_offset_format() -> Result<(), ParseError> {
        let mut module = FormatModule::new();
        let base = base::main(&mut module);

        let mask_bytes = {
            let mut tmp = ByteSet::new();
            tmp.insert(0x7f);
            tmp
        };

        let f = record([
            ("len", base.u32be()),
            (
                "mask",
                Format::WithRelativeOffset(var("len"), Box::new(Format::Byte(mask_bytes))),
            ),
            (
                "data",
                repeat_count(
                    var("len"),
                    Format::Map(
                        Box::new(base.u8()),
                        Expr::Lambda(
                            "byte".into(),
                            Box::new(Expr::BitAnd(Box::new(var("mask")), Box::new(var("byte")))),
                        ),
                    ),
                ),
            ),
        ]);
        let fref = module.define_format("test.lenpref_wro_mask", f);
        let mut data = Vec::with_capacity(37); // 4 (len) + 32 (data) + 1 (mask)
        let len_bytes = [0, 0, 0, 32];
        let mask = 0x7F;

        data.extend_from_slice(&len_bytes);
        for i in 0..32 {
            data.push(0x80 | i);
        }
        data.push(mask);

        let program = doodle::decoder::Compiler::compile_program(&module, &fref.call())
            .unwrap_or_else(|msg| panic!("Failed to compile: {msg}"));
        let (output, _) = program.run(ReadCtxt::new(&data))?;
        match output {
            Value::Record(ref fields) => match fields.as_slice() {
                &[(Cow::Borrowed("len"), ref len), (Cow::Borrowed("mask"), ref mask), (Cow::Borrowed("data"), ref data)] =>
                {
                    match len.coerce_mapped_value() {
                        &Value::U32(n) => assert_eq!(n, 32),
                        other => panic!("Unexpected Value for `len` field: {other:?}"),
                    }
                    assert!(matches!(mask, Value::U8(0x7f)));
                    match data {
                        Value::Seq(ref seq) => {
                            assert_eq!(seq.len(), 32);
                            for i in 0..32u8 {
                                match seq[i as usize] {
                                    Value::Mapped(ref orig, ref mapped) => {
                                        match orig.as_ref() {
                                            &Value::U8(orig_byte) => {
                                                assert_eq!(orig_byte, 0x80 | i)
                                            }
                                            _ => panic!("Unexpected non-U8 value in `orig`"),
                                        }
                                        match mapped.as_ref() {
                                            &Value::U8(masked_byte) => assert_eq!(masked_byte, i),
                                            _ => panic!("Unexpected non-U8 value in `mapped`"),
                                        }
                                    }
                                    _ => panic!("Unexpected non-Mapped value in sequence `data`"),
                                }
                            }
                        }
                        _ => panic!("Unexpected non-Seq value in field `data`"),
                    }
                }
                _ => panic!("Record layout and field names do not match expectation"),
            },
            _ => panic!("Unexpected non-Record value in output"),
        }
        Ok(())
    }
}
