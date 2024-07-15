pub mod get_search;

use reqwest::Method;
use serde_json::Value;
use crate::client::apis::lol_matchmaking::get_search::{LolMatchmakingMatchmakingReadyCheckResource, LolMatchmakingMatchmakingSearchResource};
use crate::client::apis::plugin_macro::impl_api_plugin;


impl_api_plugin!(
    "/lol-matchmaking",
    V1{
        GetSearch{
            get_search,Method::GET,"/search"
        } => LolMatchmakingMatchmakingSearchResource
        GetReadyCheck {
            get_ready_check,Method::GET,"/ready-check"
        } =>LolMatchmakingMatchmakingReadyCheckResource
        PostReadyCheckAccept {
            post_ready_check_accept,Method::POST,"/ready-check/accept"
        } =>Value
        PostReadyCheckDecline {
            post_ready_check_decline,Method::POST,"/ready-check/decline"
        } =>Value

    }
);