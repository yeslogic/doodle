use crate::format::base::BaseModule;
use doodle::helper::*;
use doodle::{Format, FormatModule, FormatRef};

pub mod base;

mod deflate;
mod elf;
mod gif;
mod gzip;
mod jpeg;
mod mpeg4;
mod opentype;
mod peano;
mod png;
mod riff;
mod tar;
mod text;
mod tiff;
mod waldo;
mod zlib;

pub fn main(module: &mut FormatModule) -> FormatRef {
    let base = base::main(module);

    let deflate = deflate::main(module, &base);

    let zlib = zlib::main(module, &base, deflate);

    let tiff = tiff::main(module, &base);
    let (text, utf8nz) = text::main(module, &base);
    let gif = gif::main(module, &base);
    let gzip = gzip::main(module, deflate, &base);
    let jpeg = jpeg::main(module, &base, &tiff);
    let mpeg4 = mpeg4::main(module, &base);
    let peano = peano::main(module);
    let png = png::main(module, zlib, text, utf8nz, &base);
    let riff = riff::main(module, &base);
    let tar = tar::main(module, &base);
    let elf = elf::main(module, &base);
    let waldo = waldo::main(module, &base);
    let opentype = opentype::main(module, &base);

    let tgz = {
        module.define_format(
            "tgz.main",
            chain(
                gzip.call(),
                "gzip-raw",
                for_each(
                    var("gzip-raw"),
                    "item",
                    Format::DecodeBytes(
                        Box::new(record_lens(var("item"), &["data", "inflate"])),
                        Box::new(tar.call()),
                    ),
                ),
            ),
        )
    };

    module.define_format(
        "main",
        record([
            (
                "data",
                union_nondet(vec![
                    ("waldo", waldo.call()),
                    ("peano", peano.call()),
                    ("gif", gif.call()),
                    ("tgz", tgz.call()),
                    ("gzip", gzip.call()),
                    ("jpeg", jpeg.call()),
                    ("mpeg4", mpeg4.call()),
                    ("png", png.call()),
                    ("riff", riff.call()),
                    ("tiff", tiff.call()),
                    ("tar", tar.call()),
                    ("elf", elf.call()),
                    ("opentype", opentype.call()),
                    ("text", text.call()),
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
    use doodle::{byte_set::ByteSet, decoder::Value, error::DecodeError, read::ReadCtxt};

    #[test]
    fn with_relative_offset_format() -> Result<(), DecodeError> {
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
                Format::WithRelativeOffset(
                    Box::new(var("len")),
                    Box::new(Format::Byte(mask_bytes)),
                ),
            ),
            (
                "data",
                repeat_count(
                    var("len"),
                    Format::Map(
                        Box::new(base.u8()),
                        Box::new(lambda("byte", bit_and(var("mask"), var("byte")))),
                    ),
                ),
            ),
        ]);
        let forward_ref = module.define_format("test.wro_mask", f);
        let mut data = Vec::with_capacity(37); // 4 (len) + 32 (data) + 1 (mask)
        let len_bytes = [0, 0, 0, 32];
        let mask = 0x7F;

        data.extend_from_slice(&len_bytes);
        for i in 0..32 {
            data.push(0x80 | i);
        }
        data.push(mask);

        let program = doodle::decoder::Compiler::compile_program(&module, &forward_ref.call())
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
