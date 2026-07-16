use std::{ffi::OsString, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use doodle_gencode::api_helper::otf_metrics::{
    Config, ConfigBuilder, VerboseLevel, Verbosity, VersionSelector, analyze_font,
    analyze_font_fast, font_has_table,
    lookup_subtable::{analyze_font_lookups, collate_lookups_table},
    output::show_opentype_stats,
    table::TableKind,
};

#[derive(Parser)]
struct Cli {
    /// Subcommand to invoke
    #[command(subcommand)]
    command: Command,

    /// Paths of the fonts to scan - if none provided, scans each file in `./test-fonts/*`.
    #[arg(global = true)]
    paths: Vec<OsString>,
}

#[deny(clippy::missing_docs_in_private_items)]
/// Invocation subcommand for `fontinfo`
#[derive(Subcommand)]
enum Command {
    /// Fast mode: skips promotion and display, just checks that the font can be parsed (and minimally validated)
    Fast,
    /// Lookup tabulation: scans the lookup-types in GSUB/GPOS tables and generates a tabulation of which lookups are present/absent in each font that contains at least one
    Lookups,
    /// Analyze mode: scan fonts one-by-one and outputs their detailed metrics to stdout one-after-another.
    Analyze {
        /// Verbosity level
        #[arg(long, short = 'v', action = clap::ArgAction::Count)]
        verbose: u8,
        /// Overrides the default verbosity level to display the minimal amount of information
        #[arg(long, short = 'q', conflicts_with = "verbose", default_value_t = false)]
        quiet: bool,
        /// Disables display of implemented tables and only shows which unimplemented tables are present
        #[arg(long, default_value_t = false)]
        extra_only: bool,
    },
    /// Scans for the first font among the paths provided that contains the specified table
    Scan {
        /// Table to scan for
        #[arg(value_enum)]
        table: TableKind,
        #[command(flatten)]
        ver_filter: VerFilter,
    },
}

#[derive(Args)]
#[group(required = false)]
struct VerFilter {
    #[arg(long, conflicts_with = "major")]
    /// Version to select - does not apply to tables that use major/minor versions
    version: Option<u16>,
    /// Major and minor versions - specifies a major-minor version pair to select for
    #[arg(long)]
    major: Option<u16>,
    /// Minor version (optional)
    #[arg(long, requires("major"))]
    minor: Option<u16>,
}

type RunError = Box<dyn std::error::Error + Sync + Send + 'static>;
type RunResult<T> = Result<T, RunError>;

fn prepend_dir(dir: &'static str, file_name: OsString) -> OsString {
    let mut path = PathBuf::from(dir);
    path.push(file_name);
    path.into_os_string()
}

pub fn main() -> RunResult<()> {
    stderrlog::new()
        .module(module_path!())
        .module("doodle_gencode")
        .verbosity(log::Level::Info)
        .init()?;
    let cli = Cli::parse();

    let paths = if !cli.paths.is_empty() {
        cli.paths
    } else {
        std::fs::read_dir("test-fonts")?
            .flatten()
            .map(|entry| prepend_dir("test-fonts", entry.file_name()))
            .collect()
    };

    match cli.command {
        Command::Fast => do_fast(paths),
        Command::Scan { table, ver_filter } => {
            let selector = match ver_filter {
                VerFilter {
                    version: Some(version),
                    ..
                } => VersionSelector::Version(version),
                VerFilter {
                    major: Some(major),
                    minor: None,
                    ..
                } => VersionSelector::MajorOnly {
                    major_version: major,
                },
                VerFilter {
                    major: Some(major),
                    minor: Some(minor),
                    ..
                } => VersionSelector::MajorMinor {
                    major_version: major,
                    minor_version: minor,
                },
                _ => VersionSelector::Any,
            };
            do_scan(table, selector, paths);
        }
        Command::Lookups => {
            do_tabulate(paths);
        }
        Command::Analyze {
            verbose,
            quiet,
            extra_only,
        } => {
            let mut conf_builder = ConfigBuilder::default();
            conf_builder.extra_only(extra_only);
            if quiet {
                conf_builder.verbosity(Verbosity::Minimal);
            } else {
                conf_builder.verbosity(VerboseLevel::from(verbose));
            }
            let conf = conf_builder.build()?;
            do_analyze(paths, conf);
        }
    }
    Ok(())
}

fn do_scan(table: TableKind, selector: VersionSelector, iter: impl IntoIterator<Item = OsString>) {
    for path in iter {
        match font_has_table(&path, table, selector) {
            Ok(true) => {
                println!("{path}", path = path.to_string_lossy());
                return;
            }
            Ok(false) => {}
            Err(e) => eprintln!("[{path}]: Failed! ({e})", path = path.to_string_lossy()),
        }
    }
    print!("No font found containing table '{}'", table.to_string());
    match selector {
        VersionSelector::Any => println!(),
        VersionSelector::MajorOnly { major_version } => {
            println!(" with major version {major_version}")
        }
        VersionSelector::MajorMinor {
            major_version,
            minor_version,
        } => println!(" with major version {major_version} and minor version {minor_version}"),
        VersionSelector::Version(version) => println!(" with version {version}"),
    }
}

fn do_fast(iter: impl IntoIterator<Item = OsString>) {
    for name in iter {
        eprint!("[{name}]: ...", name = name.to_string_lossy());
        match analyze_font_fast(&name) {
            Ok(_) => {
                eprintln!("Success!");
            }
            Err(e) => {
                eprintln!("Failed! ({e})")
            }
        }
    }
}

fn do_tabulate(iter: impl IntoIterator<Item = OsString>) {
    let mut samples = Vec::new();
    for name in iter {
        match analyze_font_lookups(&name) {
            Ok(lookups) => {
                eprintln!("Success!");
                samples.push((name.to_string_lossy().into_owned(), lookups))
            }
            Err(e) => {
                eprintln!("Failed! ({e})")
            }
        }
    }
    collate_lookups_table(&samples);
}

fn do_analyze(iter: impl IntoIterator<Item = OsString>, conf: Config) {
    let mut accum = Vec::new();
    for name in iter {
        eprint!("[{name}]: ...", name = name.to_string_lossy());
        match analyze_font(&name, conf.extra_only) {
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
