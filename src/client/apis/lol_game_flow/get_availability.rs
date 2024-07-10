use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::plugin::LolApiPlugin;
use crate::client::request::ApiRequest;

#[derive(Default, Debug, Clone,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameFlowGetAvailabilityResponse{
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    pub state: LolGameFlowGetAvailabilityState,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LolGameFlowGetAvailabilityState {
    #[default]
    EligibilityInfoMissing,
    Available,
    InGameFlow,
}



