use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchResource {
    pub queue_id: i32,
    pub is_currently_in_queue: bool,
    pub lobby_id: String,
    pub search_state: LolMatchmakingMatchmakingSearchState,
    pub time_in_queue: f64,
    pub estimated_queue_time: f64,
    pub ready_check: LolMatchmakingMatchmakingReadyCheckResource,
    pub dodge_data: LolMatchmakingMatchmakingDodgeData,
    pub low_priority_data: LolMatchmakingMatchmakingLowPriorityData,
    pub errors: Vec<LolMatchmakingMatchmakingSearchErrorResource>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum LolMatchmakingMatchmakingSearchState {
    ServiceShutdown,
    ServiceError,
    Error,
    Found,
    Searching,
    Canceled,
    AbandonedLowPriorityQueue,
    #[default]
    Invalid,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingReadyCheckResource {
    pub state: LolMatchmakingMatchmakingReadyCheckState,
    pub player_response: LolMatchmakingMatchmakingReadyCheckResponse,
    pub dodge_warning: LolMatchmakingMatchmakingDodgeWarning,
    pub timer: f64,
    pub decliner_ids: Vec<u64>,
    pub suppress_ux: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum LolMatchmakingMatchmakingReadyCheckState {
    #[default]
    Invalid,
    Error,
    PartyNotReady,
    StrangerNotReady,
    EveryoneReady,
    InProgress,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum LolMatchmakingMatchmakingReadyCheckResponse {
    Declined,
    Accepted,
    #[default]
    None,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum LolMatchmakingMatchmakingDodgeWarning {
    Penalty,
    Warning,
    #[default]
    None,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingDodgeData {
    pub state: LolMatchmakingMatchmakingDodgeState,
    pub dodger_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum LolMatchmakingMatchmakingDodgeState {
    #[default]
    Invalid,
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingLowPriorityData {
    pub penalized_summoner_ids: Vec<u64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub busted_leaver_access_token: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchErrorResource {
    pub id: i32,
    pub error_type: String,
    pub penalized_summoner_id: u64,
    pub penalty_time_remaining: f64,
    pub message: String,
}
