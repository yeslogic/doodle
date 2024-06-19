use crate::format::base::*;
use doodle::byte_set::ByteSet;
use doodle::{Format, FormatModule, FormatRef};

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    let mut control = ByteSet::from(0..=31);
    control.remove(b'\t');
    control.remove(b'\r');
    control.remove(b'\n');
    let high_bit = ByteSet::from(128..=255);
    let non_ascii_byte = Format::Byte(control.union(&high_bit));
    //let ascii_string = tuple([repeat1(base.ascii_char_strict()), is_byte(0x00)]);
    module.define_format(
        "binary-strings",

        repeat1(
            Format::Union(vec![
                ("binary".to_string(), repeat1(non_ascii_byte.clone())),
                ("ascii".to_string(), repeat1(base.ascii_char_strict())),
                //("ascii-non-string".to_string(), tuple([Format::PeekNot(Box::new(ascii_string)), repeat1(base.ascii_char_strict())])) 
            ])
            )
    )
}
