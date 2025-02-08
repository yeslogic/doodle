use doodle_gencode::api_helper::*;

pub fn main() -> std::io::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let iter: Box<dyn Iterator<Item = String>> = if !args.is_empty() {
        Box::new(args.into_iter())
    } else {
        Box::new(
            std::fs::read_dir("test-images")?
                .flatten()
                .map(|entry| format!("test-images/{}", entry.file_name().to_string_lossy())),
        )
    };
    do_work(iter)
}

fn do_work(iter: impl Iterator<Item = String>) -> std::io::Result<()> {
    let mut samples = Vec::new();
    for name in iter {
        match () {
            _ if name.contains("broken.png") => {
                continue;
            }
            _ if name.ends_with(".png") => {
                eprint!("[{name}]: ...");
                match analyze_png(name.as_str()) {
                    Ok(metrics) => {
                        eprintln!("Success!");
                        samples.push((name.to_string(), metrics))
                    }
                    Err(e) => {
                        eprintln!("Failed! ({e})")
                    }
                }
            }
            _ => {
                eprintln!("Skipping: {name}");
            }
        }
    }
    collate_png_table(&samples);
    Ok(())
}

#[cfg(test)]
#[test]
fn run_errant() -> std::io::Result<()> {
    do_work(std::iter::once(
        "/home/peter/yeslogic/dev/prince/tests/w3c-svg-11se/images/20x20.png".to_owned(),
    ))
}
