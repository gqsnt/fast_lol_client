use serde_json::Value;

use crate::client::api::lol_game_flow::get_availability::LolGameFlowGetAvailabilityResponse;
use crate::client::api::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::api::lol_game_flow::get_session::LolGameFlowGetSession;
use crate::client::api::plugin_macro::impl_api_plugin;

pub mod get_session;
pub mod get_availability;
pub mod get_phase;

impl_api_plugin!(
    LolGameFlow,
    GetAvailability{
        get_availability,reqwest::Method::GET,"/availability" => LolGameFlowGetAvailabilityResponse,
    },
    GetSession{
        get_session,reqwest::Method::GET,"/session" => LolGameFlowGetSession,
    },
    GetPhase{
        get_phase,reqwest::Method::GET,"/gameflow-phase" => LolGameFlowPhase,
    },
);

