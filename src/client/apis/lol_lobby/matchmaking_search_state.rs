use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LolLobbyMatchmakingSearchState {
    #[serde(rename = "searchState")]
    pub state: MatchMakingSearchState,
    #[serde(rename = "lowPriorityData")]
    pub low_priority_data: LowPriorityData,
    pub errors: Vec<Error>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatchMakingSearchState {
    #[default]
    Invalid,
    Searching,
    AbandonedLowPriorityQueue,
    Found,
    Error,
    ServiceError,
    ServiceShutdown,
    Cancelled,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowPriorityData {
    #[serde(rename = "penalizedSummonerIds")]
    pub penalized_summoner_ids: Vec<i64>,
    #[serde(rename = "penaltyTime")]
    pub penalty_time: f64,
    #[serde(rename = "penaltyTimeRemaining")]
    pub penalty_time_remaining: f64,
    #[serde(rename = "bustedLeaverAccessToken")]
    pub busted_leaver_access_token: String,
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub id: i64,
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[serde(rename = "penalizedSummonerId")]
    pub penalized_summoner_id: i64,
    #[serde(rename = "penaltyTimeRemaining")]
    pub penalty_time_remaining: f64,
    pub message: String,
}