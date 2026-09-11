use doodle::{FormatModule, FormatRef, helper::*};

pub fn main(module: &mut FormatModule) -> FormatRef {
    let bson_main = opaque_bytes();
    module.define_format("bson", bson_main)
}
