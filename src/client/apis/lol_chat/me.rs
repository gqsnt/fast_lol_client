use reqwest::Method;
use serde_json::Value;
use crate::api_request;
use crate::client::request::ApiRequest;
use crate::client::plugin::LolApiPlugin;



api_request!(
    LolApiPlugin::LolChat,
    LolChatGetMe,
    Method::GET,
    "/me",
    Value
);