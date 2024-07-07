use reqwest::Method;
use serde_json::Value;
use crate::client::apis::request::ApiRequest;
use crate::client::plugins::LolApiPlugin;

pub struct LolGameFlowGetSession{}




impl ApiRequest for LolGameFlowGetSession{
    type ReturnType = Value;
    const PLUGIN: LolApiPlugin = LolApiPlugin::LolGameFlow;

    const METHOD: reqwest::Method = Method::GET;

    fn get_path(&self) -> String {
        "/session".to_string()
    }
}
