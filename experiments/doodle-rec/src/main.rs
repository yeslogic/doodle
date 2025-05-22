use doodle_rec::{Format, FormatModule, FormatRef, Label};

fn bson(module: &mut FormatModule) -> FormatRef {
    todo!();
}

fn main() {
    let mut module = FormatModule::new();
    let bson = bson(&mut module);
    let main = module.declare_format(
        Label::Borrowed("main"),
        Format::Tuple(vec![bson.call(), Format::EndOfInput]),
    );
    println!("Hello, world!");
}
