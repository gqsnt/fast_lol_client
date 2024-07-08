use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api_request;
use crate::client::request::ApiRequest;
use crate::client::url_parameters::UrlParameters;
use crate::client::plugin::LolApiPlugin;
use crate::client::url_parameters::{Id};
use crate::client::url_parameters;

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
    champion_id: u32,
    completed: bool,
}



