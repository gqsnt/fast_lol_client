use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api_request_with_query_and_body;
use crate::client::request::ApiRequest;
use crate::client::plugin::LolApiPlugin;
use crate::client::query::{HasQuery, IdQuery};
use crate::client::query;

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



