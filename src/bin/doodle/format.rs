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
