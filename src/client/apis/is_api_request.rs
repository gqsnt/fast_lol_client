
pub trait IsApiRequest {
    const METHOD: reqwest::Method;
    type ReturnType: serde::de::DeserializeOwned + serde::Serialize;
    const PLUGIN_URL: &'static str;
    const REQUEST_URL: &'static str;
    fn get_path(&self) -> String{
        Self::REQUEST_URL.to_string()
    }
    fn get_body(&self) -> Option<serde_json::Value> {
        None
    }

    fn get_url(&self) -> String {
        format!("{}{}", Self::PLUGIN_URL, self.get_path())
    }

}

