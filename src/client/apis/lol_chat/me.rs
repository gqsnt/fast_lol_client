use reqwest::Method;
use serde_json::Value;
use crate::client::apis::request::ApiRequest;
use crate::client::plugins::LolApiPlugin;

pub struct LolChatGetMe{}




impl ApiRequest for LolChatGetMe{
    type ReturnType = Value;
    const PLUGIN: LolApiPlugin = LolApiPlugin::LolChat;

    const METHOD: reqwest::Method = Method::GET;

    fn get_path(&self) -> String {
        "/me".to_string()
    }
}
