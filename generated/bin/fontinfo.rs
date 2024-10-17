use clap::Parser;
use doodle_gencode::api_helper::otf_metrics::{Config, ConfigBuilder};
use doodle_gencode::api_helper::*;

#[derive(Parser)]
struct Params {
    #[arg(long, default_value_t = false)]
    extra_only: bool,
    paths: Vec<String>,
}

pub fn main() -> std::io::Result<()> {
    let mut conf_builder = ConfigBuilder::new();
    let params = Params::parse();
    if params.extra_only {
        conf_builder = conf_builder.extra_only(true);
    }
    let conf = conf_builder.build();

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
    do_work(iter, conf)
}

fn do_work(iter: impl Iterator<Item = String>, conf: Config) -> std::io::Result<()> {
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
    Ok(())
}
