use reqwest::Method;
use serde_json::Value;
use crate::api_request_no_params;
use crate::client::request::ApiRequest;
use crate::client::plugins::LolApiPlugin;

api_request_no_params!(
    LolApiPlugin::LolChampSelect,
    LolChampSelectGetSession,
    Method::GET,
    "/session",
    Value
);