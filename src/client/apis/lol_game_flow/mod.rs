use serde_json::Value;

use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityResponse;
use crate::client::apis::plugin_macro::impl_api_plugin;

pub mod get_session;
pub mod get_availability;

impl_api_plugin!(
    LolGameFlow,
    lol_game_flow,
    GetAvailability{
        get_availability,reqwest::Method::GET,"/availability" => LolGameFlowGetAvailabilityResponse,
    },
    GetSession{
        get_session,reqwest::Method::GET,"/session" => Value,
    },
);


