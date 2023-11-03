use crate::format::base::*;
use doodle::{FormatModule, FormatRef};

pub fn main(module: &mut FormatModule, base: &BaseModule) -> FormatRef {
    module.define_format("text", repeat1(base.ascii_char_strict()))
}
