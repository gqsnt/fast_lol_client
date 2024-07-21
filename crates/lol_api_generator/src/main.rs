use std::path::PathBuf;
use serde_json;
use openapiv3::OpenAPI;
use lol_api_generator::generator::ConvertOpenApiToRust;

#[tokio::main]
async fn main() ->Result<(), reqwest::Error> {

    // let response = reqwest::get("https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json").await?;
    // if !response.status().is_success() {
    //     panic!("Failed to get swagger.json from hasagi-types");
    // }
    //let data = response.text().await?;

    let path = PathBuf::from("openapi.json");
    if !path.exists() {
        panic!("File not found: {:?}", path);
    }
    let data=  std::fs::read_to_string(&path).unwrap();
    let spec : OpenAPI =serde_json::from_str(&data).unwrap();
    let mut open_api = ConvertOpenApiToRust::new(spec);
    open_api.parse_all().unwrap();
    open_api.generate_all().unwrap();
    Ok(())
}