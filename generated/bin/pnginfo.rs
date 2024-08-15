use doodle_gencode::api_helper::*;

pub fn main() -> std::io::Result<()> {
    let mut samples = Vec::new();
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let iter: Box<dyn Iterator<Item = String>> = if !args.is_empty() {
        Box::new(args.into_iter())
    } else {
        Box::new(
            std::fs::read_dir("test-images")?
                .flatten()
                .into_iter()
                .map(|entry| format!("test-images/{}", entry.file_name().to_string_lossy())),
        )
    };
    for name in iter {
        match () {
            _ if name.contains("broken.png") => {
                continue;
            }
            _ if name.ends_with(".png") => {
                // eprintln!("Analyzing: {name}");
                match analyze_png(name.as_str()) {
                    Ok(metrics) => samples.push((name.to_string(), metrics)),
                    Err(e) => {
                        eprintln!("Failed to analyze {name}: {e}")
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
