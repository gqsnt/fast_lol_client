use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api_request;
use crate::client::plugin::LolApiPlugin;
use crate::client::query::Id;
use crate::client::query::IsQuery;
use crate::client::request::ApiRequest;

api_request!(
    LolApiPlugin::LolChampSelect,
    LolChampSelectPatchSessionAction,
    Method::PATCH,
    "/session/actions/{id}",
    query:Id,
    body:LolChampSelectPatchSessionActionBody,
    Value
);


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectPatchSessionActionBody {
    pub champion_id: u32,
    pub completed: bool,
}



