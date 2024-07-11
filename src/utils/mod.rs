use serde_json::Value;

pub mod process_info;

pub fn save_json_to_file(name: &str, json: &Value) -> std::io::Result<()> {
    let temp = std::path::Path::new("temp");
    if !temp.exists() {
        std::fs::create_dir(temp)?;
    }
    let file = std::fs::File::create(temp.join(name))?;
    serde_json::to_writer_pretty(file, json)?;
    Ok(())
}

#[macro_export]
macro_rules! with_delay {
    (
        $delay:expr,
        $function:expr
    ) => {
        {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            $function
        }
    };
}

