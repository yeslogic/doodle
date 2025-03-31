use doodle_gencode::api_helper::rle_scan::analyze_rle;

pub fn main() -> std::io::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let iter: Box<dyn Iterator<Item = String>> = if !args.is_empty() {
        Box::new(args.into_iter())
    } else {
        eprintln!("usage: rle_info <file>");
        std::process::exit(1);
    };
    do_work(iter)
}

fn do_work(iter: impl Iterator<Item = String>) -> std::io::Result<()> {
    for name in iter {
        if name.ends_with(".rle") {
            analyze_rle(&name).unwrap_or_else(|e| {
                eprintln!("[{name}]: Failed! ({e})");
            });
        } else {
            eprintln!("skipping non-rle file: {name}");
        }
    }
    Ok(())
}
