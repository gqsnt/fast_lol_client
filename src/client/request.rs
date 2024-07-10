use serde::Serialize;

pub trait ApiRequest {
    const METHOD: reqwest::Method;
    type ReturnType: serde::de::DeserializeOwned + Serialize;
    const PLUGIN: crate::client::plugin::LolApiPlugin;
    const ENDPOINT: &'static str;
    fn get_path(&self) -> String{
        Self::ENDPOINT.to_string()
    }
    fn get_body(&self) -> Option<serde_json::Value> {
        None
    }

    fn get_url(&self) -> String {
        format!("{}{}", Self::PLUGIN.get_path(), self.get_path())
    }

}

