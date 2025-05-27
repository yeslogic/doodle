use std::{io::Read, path::PathBuf};

use doodle::{prelude::ByteSet, read::ReadCtxt};
use doodle_rec::{decoder::{Compiler, Value}, helper::*, Format, FormatModule, FormatRef, Label, RecurseCtx};
use clap::Parser;

fn surround(before: u8, after: u8, f: Format) -> Format {
    Format::Tuple(vec![is_byte(before), f, is_byte(after)])
}

fn alpha() -> Format {
    repeat(Format::Byte(ByteSet::union(&ByteSet::from(b'a'..b'z'), &ByteSet::from(b'A'..b'Z'))))
}

/// Restricted JSON object
///
/// Rules
/// - all keys consist of [A-Za-z]+ without spaces or special characters/numbers
/// - values cannot be strings

/// - all numbers are natural
/// - trailing commas are mandatory
fn json_lite(module: &mut FormatModule) -> FormatRef {
    let key = surround(b'"', b'"', repeat1(alpha()));
    let key_prefix = tuple([key, is_byte(b':')]);
    let value0 = alts([
        ("JNull", byte_seq(*b"null")),
        ("JTrue", byte_seq(*b"true")),
        ("JFalse", byte_seq(*b"false")),
        ("JNum", repeat1(Format::Byte(ByteSet::from(b'0'..=b'9')))),
        ("JObj", Format::RecVar(2)),
        ("JArr", Format::RecVar(3)),
    ]);
    let kv1 = tuple([key_prefix, var(0)]);
    let obj2 = surround(b'{', b'}', repeat(tuple([var(1), is_byte(b',')])));
    let arr3 = surround(b'[', b']', repeat(tuple([is_byte(b','), var(0)])));
    let decls = module.declare_rec_formats(
        [(Label::Borrowed("json.value"), value0),
         (Label::Borrowed("json.key_value"), kv1),
         (Label::Borrowed("json.object"), obj2),
         (Label::Borrowed("json.array"), arr3),
        ].to_vec()
    );
    decls[0]
}

#[derive(Parser)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Send + Sync + std::error::Error + 'static>> {
    // Parse arguments and open file-stream
    let args = Args::parse();
    let mut fh = std::fs::File::open(args.file)?;
    let mut buffer = Vec::new();
    fh.read_to_end(&mut buffer)?;

    let mut module = FormatModule::new();
    let json = json_lite(&mut module);
    let main = module.declare_format(
        Label::Borrowed("main"),
        Format::Tuple(vec![json.call(), Format::EndOfInput]),
    );

    let first_set = module.get_decl(main.get_level()).first_set(&module)?;
    println!("jsonValue.first: {first_set:?}");

    /*
    let program = Compiler::compile_program(&module, &main.call(), RecurseCtx::NonRec)?;
    let (v, rest) = program.run(ReadCtxt::new(&buffer))?;
    assert!(rest.remaining().is_empty());
    println!("{v:?}");
    */
    Ok(())
}
