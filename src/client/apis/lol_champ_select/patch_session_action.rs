use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api_request;
use crate::client::request::ApiRequest;
use crate::client::plugin::LolApiPlugin;
use crate::client::query::{HasQuery, IdQuery};
use crate::client::query;

api_request!(
    LolApiPlugin::LolChampSelect,
    LolChampSelectPatchSessionAction,
    Method::PATCH,
    "/session/actions/{id}",
    query:IdQuery,
    body:LolChampSelectPatchSessionActionBody,
    Value
);


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectPatchSessionActionBody {
    champion_id: u32,
    completed: bool,
}



