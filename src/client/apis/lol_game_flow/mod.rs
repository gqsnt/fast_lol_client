use reqwest::Method;
use serde_json::Value;
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
            get_availability,Method::GET,"/availability"
        } => LolGameFlowGetAvailability
        GetSession{
            get_session,Method::GET,"/session"
        } => LolGameFlowGetSession
        GetPhase{
            get_phase,Method::GET,"/gameflow-phase"
        } => LolGameFlowPhase
        PostReconnect{
            post_reconnect,Method::POST,"/reconnect"
        } => Value
        PostPreEndGameTransition{
            post_pre_end_game_transition,Method::POST,"/pre-end-game-transition"
        } => Value
    }
);


