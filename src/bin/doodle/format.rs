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

    module.define_format(
        "main",
        record([
            (
                "data",
                alts([
                    ("gif", gif.call()),
                    ("gzip", gzip.call()),
                    ("jpeg", jpeg.call()),
                    ("png", png.call()),
                    ("riff", riff.call()),
                    ("tar", tar.call()),
                ]),
            ),
            ("end", Format::EndOfInput),
        ]),
    )
}
