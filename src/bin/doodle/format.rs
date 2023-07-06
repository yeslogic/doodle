use doodle::{Format, FormatModule};

use crate::format::base::*;

mod base;

mod gif;
mod jpeg;
mod png;
mod riff;
mod tiff;

pub fn main(module: &mut FormatModule) -> Format {
    let base = base::main(module);

    let tiff = tiff::main(module, &base);
    let gif = gif::main(module, &base);
    let jpeg = jpeg::main(module, &base, &tiff);
    let png = png::main(module, &base);
    let riff = riff::main(module, &base);

    module.define_format(
        "main",
        record([
            (
                "data",
                alts([("gif", gif), ("jpeg", jpeg), ("png", png), ("riff", riff)]),
            ),
            ("end", Format::EndOfInput),
        ]),
    )
}
