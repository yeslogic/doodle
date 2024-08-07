#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]

use anyhow::{anyhow, Result as AResult};
use doodle::codegen::{generate_code, ToFragment};
use doodle::Format;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use doodle::decoder::Compiler;
use doodle::read::ReadCtxt;
use doodle::typecheck;
use doodle::FormatModule;

mod format;

#[derive(Copy, Clone, ValueEnum)]
enum FormatOutput {
    /// Use the debug formatter
    Debug,
    /// Serialize to JSON
    Json,
    /// Generate Rust code
    Rust,
}

#[derive(Copy, Clone, ValueEnum)]
enum FileOutput {
    /// Use the debug formatter
    Debug,
    /// Serialize to JSON
    Json,
    /// Display as a human-readable tree
    Tree,
    /// Display as a filtered flat list
    Flat,
}

#[derive(Parser)]
enum Command {
    /// Dump the format used when decoding files
    Format {
        /// How the format is rendered
        #[arg(long)]
        output: FormatOutput,
        #[arg(long, default_value = None)]
        dest: Option<PathBuf>,
    },
    /// Decode a binary file
    File {
        /// How decoded values are rendered
        #[arg(long, default_value = "tree")]
        output: FileOutput,
        /// The binary file to decode
        filename: PathBuf,
        #[arg(long)]
        trace: bool,
    },
    /// Typecheck the main FormatModule
    TypeCheck,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    match Command::parse() {
        Command::Format { output, dest } => {
            let mut module = FormatModule::new();
            let format = format::main(&mut module).call();

            match output {
                FormatOutput::Debug => println!("{module:?}"),
                FormatOutput::Json => serde_json::to_writer(std::io::stdout(), &module).unwrap(),
                FormatOutput::Rust => {
                    print_generated_code(&module, &format, dest);
                }
            }

            Ok(())
        }
        Command::File {
            output,
            filename,
            trace,
        } => {
            let mut module = FormatModule::new();
            let format = format::main(&mut module).call();
            let program = Compiler::compile_program(&module, &format)?;

            let input = fs::read(filename)?;
            let (value, _) = program.run(ReadCtxt::new(&input))?;

            match output {
                FileOutput::Debug => println!("{value:?}"),
                FileOutput::Json => serde_json::to_writer(std::io::stdout(), &value).unwrap(),
                FileOutput::Tree if !trace => {
                    doodle::output::tree::print_decoded_value(&module, &value, &format);
                }
                FileOutput::Tree => {
                    let (p_value, _) = program.run_with_loc(ReadCtxt::new(&input))?;
                    doodle::output::tree::print_parsed_decoded_value(&module, &p_value, &format);
                }
                FileOutput::Flat => {
                    doodle::output::flat::print_decoded_value(&module, &value, &format);
                }
            }

            Ok(())
        }
        Command::TypeCheck => {
            let mut module = FormatModule::new();
            let _top_format = format::main(&mut module);
            Ok(check_all(&module)?)
        }
    }
}

fn check_all(module: &FormatModule) -> AResult<()> {
    for (level, f) in module.iter_formats() {
        if let Some(vt) = typecheck(module, &f).map_err(|err| anyhow!("{err}"))? {
            let mod_vt = module.get_format_type(level);
            assert_eq!(&vt, mod_vt);
        }
    }
    Ok(())
}

fn print_generated_code(
    module: &FormatModule,
    top_format: &Format,
    dest: Option<std::path::PathBuf>,
) {
    let content = generate_code(module, top_format);

    fn write_to(mut f: impl std::io::Write, content: impl ToFragment) -> std::io::Result<()> {
        write!(f, "{}", content.to_fragment())
    }

    match dest {
        None => write_to(std::io::stdout().lock(), content).expect("failed to write"),
        Some(path) => {
            if !path.exists()
                || (path.is_file()
                    && path
                        .file_name()
                        .is_some_and(|s| s.to_string_lossy().contains("codegen.rs")))
            {
                let f = std::fs::File::create(path).unwrap_or_else(|err| panic!("error: {err}"));
                write_to(f, content).expect("failed to write");
            } else {
                panic!(
                    "will not overwrite directory or protected file: {}",
                    path.to_string_lossy()
                );
            }
        }
    }
}

#[test]
fn test_codegen() {
    let mut module = FormatModule::new();
    let format = format::main(&mut module).call();
    let _ = generate_code(&module, &format);
}
