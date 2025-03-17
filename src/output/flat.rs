use std::io;

use crate::decoder::Value;
use crate::{Format, FormatModule};
use crate::{Label, StyleHint};

pub fn print_decoded_value(module: &FormatModule, value: &Value, format: &Format) {
    let mut path = Vec::new();
    check_covered(module, &mut path, format).unwrap();
    Context::new(io::stdout(), module)
        .write_flat(value, format)
        .unwrap()
}

pub struct Context<'module, W: io::Write> {
    writer: W,
    module: &'module FormatModule,
}

fn is_show_format(name: &str) -> Option<&'static str> {
    match name {
        // gzip
        "gzip.header" => Some("gzip header"),
        "gzip.fname" => Some("gzip filename"),
        "gzip.footer" => Some("gzip footer"),

        // deflate
        "deflate.block" => Some("deflate block"),

        // GIF
        "gif.header" => Some("Header"),
        "gif.trailer" => Some("Trailer"),
        "gif.logical-screen" => Some("Logical Screen"),
        "gif.graphic-control-extension" => Some("Graphic Control Extension"),
        "gif.plain-text-extension" => Some("Plain Text Extension"),
        "gif.table-based-image" => Some("Table Based Image"),
        "gif.special-purpose-block" => Some("Special Purpose Block"),

        // PNG
        "png.signature" => Some("PNG signature"),
        "png.ihdr" => Some("Image Header"),
        "png.iend" => Some("Image Trailer"),
        "png.idat" => Some("Image Data"),
        "png.bkgd" => Some("Background color"),
        "png.phys" => Some("Physical Pixel Dimensions"),
        "png.plte" => Some("Palette"),
        "png.time" => Some("Last-modification Time"),
        "png.trns" => Some("Transparency"),
        "png.other-chunk" => Some("Other Chunk"),

        // RIFF, WebP
        "riff.main" => Some("riff.main"),

        // JPEG
        "jpeg.soi" => Some("Start of Image"),
        "jpeg.eoi" => Some("End of Image"),
        "jpeg.app0" => Some("Application Segment 0"),
        "jpeg.app1" => Some("Application Segment 1"),
        "jpeg.app2" => Some("Application Segment 2"),
        "jpeg.app3" => Some("Application Segment 3"),
        "jpeg.app4" => Some("Application Segment 4"),
        "jpeg.app5" => Some("Application Segment 5"),
        "jpeg.app6" => Some("Application Segment 6"),
        "jpeg.app7" => Some("Application Segment 7"),
        "jpeg.app8" => Some("Application Segment 8"),
        "jpeg.app9" => Some("Application Segment 9"),
        "jpeg.app10" => Some("Application Segment 10"),
        "jpeg.app11" => Some("Application Segment 11"),
        "jpeg.app12" => Some("Application Segment 12"),
        "jpeg.app13" => Some("Application Segment 13"),
        "jpeg.app14" => Some("Application Segment 14"),
        "jpeg.app15" => Some("Application Segment 15"),
        "jpeg.com" => Some("Comment"),
        "jpeg.dac" => Some("Define Arithmetic Coding"),
        "jpeg.dht" => Some("Define Huffman Tables"),
        "jpeg.dnl" => Some("Define Number of Lines"),
        "jpeg.dqt" => Some("Define Quantization Tables"),
        "jpeg.dri" => Some("Define Restart Interval"),
        "jpeg.sof0" => Some("Start of Frame (baseline)"),
        "jpeg.sof1" => Some("Start of Frame (extended sequential, huffman)"),
        "jpeg.sof2" => Some("Start of Frame (progressive, huffman)"),
        "jpeg.sof3" => Some("Start of Frame (lossless, huffman)"),
        "jpeg.sof5" => Some("Start of Frame (differential sequential, huffman)"),
        "jpeg.sof6" => Some("Start of Frame (differential progressive, huffman)"),
        "jpeg.sof7" => Some("Start of Frame (differential lossless, huffman)"),
        "jpeg.sof9" => Some("Start of Frame (extended sequential, arithmetic)"),
        "jpeg.sof10" => Some("Start of Frame (progressive, arithmetic)"),
        "jpeg.sof11" => Some("Start of Frame (lossless, arithmetic)"),
        "jpeg.sof13" => Some("Start of Frame (differential sequential, arithmetic)"),
        "jpeg.sof14" => Some("Start of Frame (differential progressive, arithmetic)"),
        "jpeg.sof15" => Some("Start of Frame (differential lossless, arithmetic)"),
        "jpeg.sos" => Some("Start of Scan"),
        "jpeg.scan-data" => Some("Entropy-Coded Segment"),

        // mpeg4
        "mpeg4.atom" => Some("MPEG4 atom"),

        // Tar
        "tar.main" => Some("tar.main"),
        "tar.header" => Some("Tar Header"),
        "tar.header_with_data" => Some("Tar File Entry"),

        // Text
        "text.string.ascii" => Some("ASCII Text"),
        "text.string.utf8" => Some("UTF-8 Text"),

        _ => None,
    }
}

fn check_covered(
    module: &FormatModule,
    path: &mut Vec<Label>,
    format: &Format,
) -> Result<(), String> {
    match format {
        Format::ItemVar(level, _args) => {
            let name = module.get_name(*level).to_string();
            if is_show_format(&name).is_none() {
                path.push(name.into());
                check_covered(module, path, module.get_format(*level))?;
                path.pop();
            }
        }
        Format::Fail => {}
        Format::EndOfInput => {}
        Format::SkipRemainder => {}
        Format::Pos => {}
        Format::Align(_) => {}
        Format::Byte(_) => {
            return Err(format!("uncovered byte: {:?}", path));
        }
        Format::DecodeBytes(_, format) => {
            check_covered(module, path, format)?;
        }
        Format::Variant(label, format) => {
            path.push(label.clone());
            check_covered(module, path, format)?;
            path.pop();
        }
        Format::Union(branches) | Format::UnionNondet(branches) => {
            for format in branches {
                check_covered(module, path, format)?;
            }
        }
        Format::Tuple(formats) => {
            for format in formats {
                check_covered(module, path, format)?;
            }
        }
        Format::Repeat(format)
        | Format::Repeat1(format)
        | Format::RepeatCount(_, format)
        | Format::RepeatBetween(_, _, format)
        | Format::ForEach(_, _, format)
        | Format::RepeatUntilLast(_, format)
        | Format::RepeatUntilSeq(_, format)
        | Format::AccumUntil(.., format) => {
            check_covered(module, path, format)?;
        }
        Format::Maybe(_expr, format) => {
            // check Some branch
            path.push(Label::from("Some"));
            check_covered(module, path, format)?;
            path.pop();

            // check None branch
            path.push(Label::from("None"));
            check_covered(module, path, &Format::EMPTY)?;
            path.pop();
        }
        Format::Peek(_) => {}    // FIXME
        Format::PeekNot(_) => {} // FIXME
        Format::Slice(_, format) => {
            check_covered(module, path, format)?;
        }

        Format::Bits(format) => {
            check_covered(module, path, format)?;
        }
        Format::WithRelativeOffset(_, _, _) => {} // FIXME
        Format::Map(format, _expr) => check_covered(module, path, format)?,
        Format::Where(format, _expr) => check_covered(module, path, format)?,
        Format::Compute(_expr) => {}
        Format::Let(_name, _expr, format) => check_covered(module, path, format)?,
        Format::Match(_head, branches) => {
            for (_pattern, format) in branches {
                check_covered(module, path, format)?;
            }
        }
        Format::Dynamic(_name, _dynformat, format) => check_covered(module, path, format)?,
        Format::Apply(_) => {}
        Format::MonadSeq(f0, f) | Format::LetFormat(f0, _, f) => {
            check_covered(module, path, f0)?;
            check_covered(module, path, f)?;
        }
        Format::Hint(_hint, format) => check_covered(module, path, format)?,
    }
    Ok(())
}

impl<'module, W: io::Write> Context<'module, W> {
    pub fn new(writer: W, module: &'module FormatModule) -> Context<'module, W> {
        Context { writer, module }
    }

    pub fn write_flat(&mut self, value: &Value, format: &Format) -> io::Result<()> {
        match format {
            Format::ItemVar(level, _args) => {
                let label = self.module.get_name(*level);
                if let Some(title) = is_show_format(label) {
                    writeln!(&mut self.writer, "{label} - {title}")
                } else {
                    self.write_flat(value, self.module.get_format(*level))
                }
            }
            Format::Fail => Ok(()),
            Format::EndOfInput => Ok(()),
            Format::SkipRemainder => Ok(()),
            Format::Align(_) => Ok(()),
            Format::Pos => Ok(()),
            Format::Byte(_) => Ok(()),
            Format::Variant(label, format) => match value {
                Value::Variant(label2, value) => {
                    if label == label2 {
                        self.write_flat(value, format)
                    } else {
                        panic!("expected variant label {label}, found {label2}");
                    }
                }
                _ => panic!("expected variant, found {value:?}"),
            },
            Format::Union(branches) | Format::UnionNondet(branches) => match value {
                Value::Branch(index, value) => {
                    let format = &branches[*index];
                    self.write_flat(value, format)
                }
                _ => panic!("expected branch, found {value:?}"),
            },
            Format::Tuple(formats) => match value {
                Value::Tuple(values) => {
                    for (index, value) in values.iter().enumerate() {
                        let format = &formats[index];
                        self.write_flat(value, format)?;
                    }
                    Ok(())
                }
                _ => panic!("expected tuple, found {value:?}"),
            },
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::ForEach(_, _, format)
            | Format::RepeatCount(_, format)
            | Format::RepeatBetween(_, _, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => match value {
                Value::Seq(values) => {
                    for v in values.iter() {
                        self.write_flat(v, format)?;
                    }
                    Ok(())
                }
                _ => panic!("expected sequence, found {value:?}"),
            },
            Format::AccumUntil(.., format) => match value {
                // REVIEW - is this the correct approach?
                Value::Tuple(vs) => match vs.as_slice() {
                    [_, v] => match &v {
                        Value::Seq(values) => {
                            for v in values.iter() {
                                self.write_flat(v, format)?;
                            }
                            Ok(())
                        }
                        _ => panic!("expected sequence, found {v:?}"),
                    },
                    _ => panic!("expected 2-tuple, found {vs:#?}"),
                },
                _ => panic!("expected tuple, found {value:?}"),
            },
            Format::Maybe(_expr, format) => match value {
                Value::Option(inner) => match inner.as_ref() {
                    Some(val) => self.write_flat(val, format),
                    None => Ok(()),
                },
                other => unreachable!("expected Option, found {other:?}"),
            },
            Format::DecodeBytes(_bytes, format) => self.write_flat(value, format),
            Format::Peek(format) => self.write_flat(value, format),
            Format::PeekNot(format) => self.write_flat(value, format),
            Format::Slice(_, format) => self.write_flat(value, format),
            Format::Bits(format) => self.write_flat(value, format),
            Format::WithRelativeOffset(_addr, _offs, format) => self.write_flat(value, format),
            Format::Map(_format, _expr) => Ok(()),
            Format::Where(_format, _expr) => Ok(()),
            Format::Compute(_expr) => Ok(()),
            Format::Let(_name, _expr, format) => self.write_flat(value, format),
            Format::Match(_head, branches) => match value {
                Value::Branch(index, value) => {
                    let (_pattern, format) = &branches[*index];
                    self.write_flat(value, format)?;
                    Ok(())
                }
                _ => panic!("expected branch, found {value:?}"),
            },
            Format::Dynamic(_name, _dynformat, format) => self.write_flat(value, format),
            Format::Apply(_) => Ok(()), // FIXME
            Format::LetFormat(_f0, _name, f) => self.write_flat(value, f),
            Format::MonadSeq(_f0, f) => self.write_flat(value, f),
            Format::Hint(StyleHint::Record { .. }, record_format) => {
                self.write_record(value, record_format)
            }
        }
    }

    fn write_record(&mut self, value: &Value, record_format: &Format) -> io::Result<()> {
        let rec = record_format.synthesize_record();
        match value {
            Value::Record(value_fields) => {
                for (label, value) in value_fields.iter() {
                    let Some((fmt, _)) = rec.lookup_value_field(label) else {
                        panic!("missing field {label}")
                    };
                    self.write_flat(value, fmt)?;
                }
                Ok(())
            }
            _ => panic!("expected record, found {value:?}"),
        }
    }
}
