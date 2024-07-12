use reqwest::Method;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailability;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_game_flow::get_session::LolGameFlowGetSession;
use crate::client::apis::plugin_macro::impl_api_plugin;
pub mod get_session;
pub mod get_availability;
pub mod get_phase;

impl_api_plugin!(
    "/lol-gameflow",
    V1{
        GetAvailability{
            fn:get_availability,method:Method::GET,url:"/availability" => LolGameFlowGetAvailability
        }
        GetSession{
            fn:get_session,method: Method::GET, url:"/session" => LolGameFlowGetSession
        }
        GetPhase{
            fn:get_phase,method: Method::GET, url:"/gameflow-phase" => LolGameFlowPhase
        }
    }
);


