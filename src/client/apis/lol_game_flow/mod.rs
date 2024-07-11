use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailability;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_game_flow::get_session::LolGameFlowGetSession;
use crate::client::apis::plugin_macro::impl_api_plugin;
use crate::client::apis::is_api_request::IsApiRequest;
pub mod get_session;
pub mod get_availability;
pub mod get_phase;

impl_api_plugin!(
    "/lol-gameflow/v1",
    GetAvailability{
        get_availability,reqwest::Method::GET,"/availability" => LolGameFlowGetAvailability,
    },
    GetSession{
        get_session,reqwest::Method::GET,"/session" => LolGameFlowGetSession,
    },
    GetPhase{
        get_phase,reqwest::Method::GET,"/gameflow-phase" => LolGameFlowPhase,
    },
);


