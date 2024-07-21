use std::path::PathBuf;
use serde_json;
use openapiv3::OpenAPI;
use lol_api_generator::generator::ConvertOpenApiToRust;

fn main() {
    let path = PathBuf::from("openapi.json");
    if !path.exists() {
        panic!("File not found: {:?}", path);
    }
    let data=  std::fs::read_to_string(&path).unwrap();
    println!("data len: {}", data.len());
    let spec : OpenAPI =serde_json::from_str(&data).unwrap();
    let mut open_api = ConvertOpenApiToRust::new(spec);
    open_api.parse_all().unwrap();
    open_api.generate_all().unwrap();

}