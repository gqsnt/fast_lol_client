use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api_request;
use crate::client::api_data::ApiDataQueryBody;
use crate::client::request::ApiRequest;
use crate::client::query::IsQuery;
use crate::client::plugin::LolApiPlugin;
use crate::client::query::{Id};
use crate::client::query;
use crate::client::api_data::IsApiData;

api_request!(
    LolApiPlugin::LolChampSelect,
    LolChampSelectPatchSessionAction,
    Method::PATCH,
    "/session/actions/{id}",
    ApiDataQueryBody<Id, LolChampSelectPatchSessionActionBody>,
    Value
);


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectPatchSessionActionBody {
    champion_id: u32,
    completed: bool,
}



