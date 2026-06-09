use clap::Parser;
use doodle_gencode::api_helper::otf_metrics::{
    Config, ConfigBuilder, VerboseLevel, Verbosity, analyze_font, analyze_font_fast,
    font_has_table,
    lookup_subtable::{analyze_font_lookups, collate_lookups_table},
    output::show_opentype_stats,
    table::TableKind,
};

#[derive(Parser)]
struct Params {
    #[arg(long, default_value_t = false)]
    tabulate_lookups: bool,
    #[arg(long, default_value_t = false)]
    extra_only: bool,
    #[arg(long, default_value_t = false)]
    fast: bool,
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(long, short = 'q', conflicts_with = "verbose", default_value_t = false)]
    quiet: bool,
    #[arg(long, value_name = "TABLE")]
    scan_for: Option<String>,
    paths: Vec<String>,
}

#[derive(Clone, Copy)]
struct CliFlags {
    tabulate_lookups: bool,
    fast: bool,
}

type RunError = Box<dyn std::error::Error + Sync + Send + 'static>;
type RunResult<T> = Result<T, RunError>;

pub fn main() -> RunResult<()> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(log::Level::Info)
        .init()?;
    let mut conf_builder = ConfigBuilder::default();
    let params = Params::parse();

    if let Some(tag_str) = params.scan_for {
        let table_kind: TableKind = tag_str.parse().map_err(|e| format!("{e}") as String)?;
        let tag = table_kind as u32;
        let paths = if !params.paths.is_empty() {
            params.paths
        } else {
            std::fs::read_dir("test-fonts")?
                .flatten()
                .map(|entry| format!("test-fonts/{}", entry.file_name().to_string_lossy()))
                .collect()
        };
        for path in &paths {
            match font_has_table(path.as_str(), tag) {
                Ok(true) => {
                    println!("{path}");
                    return Ok(());
                }
                Ok(false) => {}
                Err(e) => eprintln!("[{path}]: Failed! ({e})"),
            }
        }
        println!("No font found containing table '{tag_str}'");
        return Ok(());
    }

    conf_builder.extra_only(params.extra_only);
    if params.quiet {
        conf_builder.verbosity(Verbosity::Minimal);
    } else {
        conf_builder.verbosity(VerboseLevel::from(params.verbose));
    }
    let conf = conf_builder.build()?;
    let flags = CliFlags {
        tabulate_lookups: params.tabulate_lookups,
        fast: params.fast,
    };

    let spec_files = params.paths;
    let iter: Box<dyn Iterator<Item = String>> = if !spec_files.is_empty() {
        Box::new(spec_files.into_iter())
    } else {
        Box::new(
            std::fs::read_dir("test-fonts")?
                .flatten()
                .map(|entry| format!("test-fonts/{}", entry.file_name().to_string_lossy())),
        )
    };
    do_work(iter, conf, flags);
    Ok(())
}

fn do_work(iter: impl Iterator<Item = String>, conf: Config, flags: CliFlags) {
    if flags.fast {
        for name in iter {
            eprint!("[{name}]: ...");
            match analyze_font_fast(name.as_str()) {
                Ok(_) => {
                    eprintln!("Success!");
                }
                Err(e) => {
                    eprintln!("Failed! ({e})")
                }
            }
        }
    } else if flags.tabulate_lookups {
        let mut samples = Vec::new();
        for name in iter {
            match analyze_font_lookups(name.as_str()) {
                Ok(lookups) => {
                    eprintln!("Success!");
                    samples.push((name.to_string(), lookups))
                }
                Err(e) => {
                    eprintln!("Failed! ({e})")
                }
            }
        }
        collate_lookups_table(&samples);
    } else {
        let mut accum = Vec::new();
        for name in iter {
            eprint!("[{name}]: ...");
            match analyze_font(name.as_str(), conf.extra_only) {
                Ok(metric) => {
                    eprintln!("Success!");
                    accum.push((name, metric))
                }
                Err(e) => {
                    eprintln!("Failed! ({e})")
                }
            }
        }
        for (filename, metrics) in accum {
            let filename = std::path::Path::new(&filename)
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new(&filename))
                .to_string_lossy();
            println!("====== [Font File]: {filename} =======");
            show_opentype_stats(&metrics, &conf);
            println!("====== END OF FONT FILE ======\n\n");
        }
    }
}
