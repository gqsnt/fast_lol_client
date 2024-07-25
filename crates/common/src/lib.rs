use serde_json::map::Values;
use serde_json::Value;

pub trait IsApiRequest {
    const METHOD: reqwest::Method;
    type ReturnType: serde::de::DeserializeOwned + serde::Serialize;
    fn get_url(&self) -> String;
    fn get_body(&self) -> Option<serde_json::Value> { None }

    fn get_query(&self) -> Option<Vec<(String,String)>> { None }
}

