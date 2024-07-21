
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolSocialLeaderboardV1LeaderboardNextUpdateTime {
    pub queue_type: Option<LolSocialLeaderboardLeagueQueueType>,
}

impl IsApiRequest for GetLolSocialLeaderboardV1LeaderboardNextUpdateTime {
    const METHOD: Method = Method::GET;
    type ReturnType = i64;
    fn get_url(&self) -> String {"/lol-social-leaderboard/v1/leaderboard-next-update-time".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "queueType" : self.queue_type,
        }))
    }
}

pub fn get_lol_social_leaderboard_v_1_leaderboard_next_update_time(queue_type: Option<LolSocialLeaderboardLeagueQueueType>) -> GetLolSocialLeaderboardV1LeaderboardNextUpdateTime {
    GetLolSocialLeaderboardV1LeaderboardNextUpdateTime{queue_type}
}


pub struct GetLolSocialLeaderboardV1SocialLeaderboardData {
    pub queue_type: Option<LolSocialLeaderboardLeagueQueueType>,
}

impl IsApiRequest for GetLolSocialLeaderboardV1SocialLeaderboardData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSocialLeaderboardSocialLeaderboardData;
    fn get_url(&self) -> String {"/lol-social-leaderboard/v1/social-leaderboard-data".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "queueType" : self.queue_type,
        }))
    }
}

pub fn get_lol_social_leaderboard_v_1_social_leaderboard_data(queue_type: Option<LolSocialLeaderboardLeagueQueueType>) -> GetLolSocialLeaderboardV1SocialLeaderboardData {
    GetLolSocialLeaderboardV1SocialLeaderboardData{queue_type}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardData {
    pub row_data: Vec<LolSocialLeaderboardSocialLeaderboardRowData>,
    pub next_update_time: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardRowData {
    pub puuid: String,
    pub summoner_id: u64,
    pub summoner_name: String,
    pub game_name: String,
    pub tag_line: String,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub tier: String,
    pub division: LolSocialLeaderboardLeagueDivision,
    pub league_points: i32,
    pub wins: i32,
    pub summoner_level: i32,
    pub profile_icon_id: i32,
    pub availability: String,
    pub leaderboard_position: i32,
    pub is_giftable: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolSocialLeaderboardLeagueDivision {
    #[default]
    #[serde(rename = "NA")]
    Na,
    V,
    #[serde(rename = "IV")]
    Iv,
    #[serde(rename = "III")]
    Iii,
    #[serde(rename = "II")]
    Ii,
    I,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolSocialLeaderboardLeagueQueueType {
    #[default]
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    RankedTftDoubleUp,
    #[serde(rename = "RANKED_TFT_PAIRS")]
    RankedTftPairs,
    #[serde(rename = "RANKED_TFT_TURBO")]
    RankedTftTurbo,
    #[serde(rename = "RANKED_TFT")]
    RankedTft,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTt,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RankedSolo5X5,
    #[serde(rename = "NONE")]
    None,
}

