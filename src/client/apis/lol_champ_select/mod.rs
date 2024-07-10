use reqwest::Method;
use serde_json::Value;
use crate::client::apis::lol_champ_select::patch_session_action::LolChampSelectPatchSessionActionBody;
use crate::client::request::impl_api_plugin;

pub mod get_session;
pub mod patch_session_action;


impl_api_plugin!(
    LolChampSelect,
    PatchSessionAction{
        patch_session_action,Method::PATCH,"/actions/{id}" => Value,
        route_params:{id:u64},
        body:LolChampSelectPatchSessionActionBody,
    },
    GetSession{
        get_session, Method::GET, "/session" => Value,
    },
);

