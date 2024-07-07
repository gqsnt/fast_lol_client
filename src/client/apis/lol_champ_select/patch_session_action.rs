use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api_request_with_query_and_body;
use crate::client::request::{ApiRequest, IdQuery,HasQuery};
use crate::client::plugins::LolApiPlugin;
use crate::client::request::format_string_with_query;

api_request_with_query_and_body!(
    LolApiPlugin::LolChampSelect,
    LolChampSelectPatchSessionAction,
    Method::PATCH,
    "/session/actions/{id}",
    IdQuery,
    LolChampSelectPatchSessionActionBody,
    Value
);


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectPatchSessionActionBody {
    champion_id: u32,
    completed: bool,
}



