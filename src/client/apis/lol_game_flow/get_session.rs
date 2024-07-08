use reqwest::Method;
use serde_json::Value;

use crate::api_request;
use crate::client::plugin::LolApiPlugin;
use crate::client::request::ApiRequest;

api_request!(
    LolApiPlugin::LolGameFlow,
    LolGameFlowGetSession,
    Method::GET,
    "/session",
    Value
);