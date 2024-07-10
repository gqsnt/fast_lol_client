use serde_json::Value;

pub mod process_info;

pub fn save_json_to_file(name: &str, json: &Value) -> std::io::Result<()> {
    let file = std::fs::File::create(std::path::Path::new("temp").join(name))?;
    serde_json::to_writer_pretty(file, json)?;
    Ok(())
}