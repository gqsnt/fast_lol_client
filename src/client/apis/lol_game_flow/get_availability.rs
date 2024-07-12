use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameFlowGetAvailability {
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    pub state: LolGameFlowGetAvailabilityState,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum LolGameFlowGetAvailabilityState {
    #[default]
    EligibilityInfoMissing,
    Configuration,
    PlayerBanned,
    Patching,
    Initializing,
    Available,
    InGameFlow,
}



