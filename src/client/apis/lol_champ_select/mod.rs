use reqwest::Method;
use serde_json::Value;

use crate::client::apis::lol_champ_select::patch_session_action::LolChampSelectPatchSessionActionBody;
use crate::client::apis::plugin_macro::impl_api_plugin;
use crate::client::apis::is_api_request::IsApiRequest;
pub mod get_session;
pub mod patch_session_action;

impl_api_plugin!(
    "/lol-champ-select/v1",
    PatchSessionAction{
        patch_session_action,Method::PATCH,"/actions/{id}" => Value,
        route_params:{id:u64},
        body:LolChampSelectPatchSessionActionBody,
    },
    GetSession{
        get_session, Method::GET, "/session" => Value,
    },
);

