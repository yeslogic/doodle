use doodle_gencode::api_helper::*;

pub fn main() -> std::io::Result<()> {
    let mut samples = Vec::new();
    for entry in std::fs::read_dir("test-images")?.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        match () {
            _ if name.contains("broken.png") => {
                continue;
            }
            _ if name.ends_with(".png") => {
                if let Ok(metrics) = analyze_png(format!("test-images/{}", name).as_str()) {
                    samples.push((name.to_string(), metrics))
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }
    collate_png_table(&samples);
    Ok(())
}
