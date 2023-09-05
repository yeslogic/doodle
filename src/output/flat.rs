use std::io;

use crate::{Format, FormatModule, Value};

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
    names: Vec<String>,
    values: Vec<Value>,
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
        _ => None,
    }
}

fn check_covered(
    module: &FormatModule,
    path: &mut Vec<String>,
    format: &Format,
) -> Result<(), String> {
    match format {
        Format::ItemVar(level, _args) => {
            let name = module.get_name(*level);
            if is_show_format(name).is_none() {
                path.push(name.to_string());
                check_covered(module, path, module.get_format(*level))?;
                path.pop();
            }
        }
        Format::Fail => {}
        Format::EndOfInput => {}
        Format::Align(_) => {}
        Format::Byte(_) => {
            return Err(format!("uncovered byte: {:?}", path));
        }
        Format::Union(branches) => {
            for (label, format) in branches {
                path.push(label.clone());
                check_covered(module, path, format)?;
                path.pop();
            }
        }
        Format::Tuple(formats) => {
            for format in formats {
                check_covered(module, path, format)?;
            }
        }
        Format::Record(format_fields) => {
            for (label, format) in format_fields {
                path.push(label.clone());
                check_covered(module, path, format)?;
                path.pop();
            }
        }
        Format::Repeat(format)
        | Format::Repeat1(format)
        | Format::RepeatCount(_, format)
        | Format::RepeatUntilLast(_, format)
        | Format::RepeatUntilSeq(_, format) => {
            check_covered(module, path, format)?;
        }
        Format::Peek(_) => {} // FIXME
        Format::Slice(_, format) => {
            check_covered(module, path, format)?;
        }
        Format::Bits(format) => {
            check_covered(module, path, format)?;
        }
        Format::WithRelativeOffset(_, _) => {} // FIXME
        Format::Compute(_expr) => {}
        Format::Match(_head, branches) => {
            for (_pattern, format) in branches {
                check_covered(module, path, format)?;
            }
        }
        Format::Dynamic(_) => {} // FIXME
    }
    Ok(())
}

impl<'module, W: io::Write> Context<'module, W> {
    pub fn new(writer: W, module: &'module FormatModule) -> Context<'module, W> {
        Context {
            writer,
            module,
            names: Vec::new(),
            values: Vec::new(),
        }
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
            Format::Align(_) => Ok(()),
            Format::Byte(_) => Ok(()),
            Format::Union(branches) => match value {
                Value::Variant(label, value) => {
                    let (_, format) = branches.iter().find(|(l, _)| l == label).unwrap();
                    self.write_flat(value, format)
                }
                _ => panic!("expected variant"),
            },
            Format::Tuple(formats) => match value {
                Value::Tuple(values) => {
                    for (index, value) in values.iter().enumerate() {
                        let format = &formats[index];
                        self.write_flat(value, format)?;
                    }
                    Ok(())
                }
                _ => panic!("expected tuple"),
            },
            Format::Record(format_fields) => match value {
                Value::Record(value_fields) => {
                    let initial_len = self.names.len();
                    for (index, (label, value)) in value_fields.iter().enumerate() {
                        let format = &format_fields[index].1;
                        self.write_flat(value, format)?;
                        self.names.push(label.clone());
                        self.values.push(value.clone());
                    }
                    self.names.truncate(initial_len);
                    self.values.truncate(initial_len);
                    Ok(())
                }
                _ => panic!("expected record"),
            },
            Format::Repeat(format)
            | Format::Repeat1(format)
            | Format::RepeatCount(_, format)
            | Format::RepeatUntilLast(_, format)
            | Format::RepeatUntilSeq(_, format) => match value {
                Value::Seq(values) => {
                    for v in values {
                        self.write_flat(v, format)?;
                    }
                    Ok(())
                }
                _ => panic!("expected sequence"),
            },
            Format::Peek(format) => self.write_flat(value, format),
            Format::Slice(_, format) => self.write_flat(value, format),
            Format::Bits(format) => self.write_flat(value, format),
            Format::WithRelativeOffset(_, format) => self.write_flat(value, format),
            Format::Compute(_expr) => Ok(()),
            Format::Match(head, branches) => {
                let head = head.eval(&mut self.values);
                let initial_len = self.values.len();
                let (_, format) = branches
                    .iter()
                    .find(|(pattern, _)| head.matches(&mut self.values, pattern))
                    .expect("exhaustive patterns");
                for i in 0..(self.values.len() - initial_len) {
                    self.names.push(format!("x{i}")); // TODO: use better names
                }
                self.write_flat(value, format)?;
                self.names.truncate(initial_len);
                self.values.truncate(initial_len);
                Ok(())
            }
            Format::Dynamic(_) => Ok(()), // FIXME
        }
    }
}
