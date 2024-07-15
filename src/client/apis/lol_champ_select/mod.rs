use reqwest::Method;
use serde_json::Value;
use crate::client::apis::lol_champ_select::get_session::LolChampSelectChampSelectSession;
use crate::client::apis::lol_champ_select::patch_session_action::LolChampSelectPatchSessionActionBody;
use crate::client::apis::plugin_macro::impl_api_plugin;

pub mod get_session;
pub mod patch_session_action;



impl_api_plugin!(
    "/lol-champ-select",
    V1{
        PatchSessionAction{
            patch_session_action, Method::PATCH, "/actions/{}",
            params: {id:u64},
            body: LolChampSelectPatchSessionActionBody
        } => Value
        GetSession{
            get_session, Method::GET, "/session"
        } => LolChampSelectChampSelectSession
    }
);

