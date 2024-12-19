use clap::Parser;
use doodle_gencode::api_helper::otf_metrics::{
    analyze_font, analyze_font_fast,
    lookup_subtable::{analyze_font_lookups, collate_lookups_table},
    show_opentype_stats, Config, ConfigBuilder, VerboseLevel, Verbosity,
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
    let mut conf_builder = ConfigBuilder::default();
    let params = Params::parse();
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
                .into_iter()
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
            match analyze_font(name.as_str()) {
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
            println!("====== [Font File]: {filename} =======");
            show_opentype_stats(&metrics, &conf);
            println!("====== END OF FONT FILE ======\n\n");
        }
    }
}
