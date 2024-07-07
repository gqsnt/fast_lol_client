use reqwest::Method;
use serde_json::Value;
use crate::client::apis::request::ApiRequest;
use crate::client::plugins::LolApiPlugin;

pub struct LolChampSelectGetSession{}




impl ApiRequest for LolChampSelectGetSession{
    type ReturnType = Value;
    const PLUGIN: LolApiPlugin = LolApiPlugin::LolChampSelect;

    const METHOD: reqwest::Method = Method::GET;

    fn get_path(&self) -> String {
        "/session".to_string()
    }
}
