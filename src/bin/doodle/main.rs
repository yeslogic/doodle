#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use doodle::{Decoder, FormatModule};

mod format;

#[derive(Copy, Clone, ValueEnum)]
enum OutputFormat {
    /// Use the debug formatter
    Debug,
    /// Serialize to JSON
    Json,
    /// Display as a human-readable tree
    Tree,
}

/// Decode a binary file
#[derive(Parser)]
struct Args {
    /// How decoded values are rendered
    #[arg(long, default_value = "tree")]
    output: OutputFormat,
    /// The binary file to decode
    #[arg(default_value = "test.jpg")]
    filename: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();
    let input = fs::read(args.filename)?;

    let mut module = FormatModule::new();
    let format = format::main(&mut module);

    let (val, _) = Decoder::compile(&module, &format)?
        .parse(&mut Vec::new(), &input)
        .ok_or("parse failure")?;

    match args.output {
        OutputFormat::Debug => println!("{val:?}"),
        OutputFormat::Json => serde_json::to_writer(std::io::stdout(), &val).unwrap(),
        OutputFormat::Tree => doodle::output::tree::print_decoded_value(&module, &val, &format),
    }

    Ok(())
}
