
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("queueType".to_string(), serde_json::to_string(&self.queue_type).unwrap())
        ])
    }
}

pub fn get_lol_social_leaderboard_v1_leaderboard_next_update_time(queue_type: Option<LolSocialLeaderboardLeagueQueueType>) -> GetLolSocialLeaderboardV1LeaderboardNextUpdateTime {
    GetLolSocialLeaderboardV1LeaderboardNextUpdateTime{queue_type}
}


pub struct GetLolSocialLeaderboardV1SocialLeaderboardData {
    pub queue_type: Option<LolSocialLeaderboardLeagueQueueType>,
}

impl IsApiRequest for GetLolSocialLeaderboardV1SocialLeaderboardData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSocialLeaderboardSocialLeaderboardData;
    fn get_url(&self) -> String {"/lol-social-leaderboard/v1/social-leaderboard-data".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("queueType".to_string(), serde_json::to_string(&self.queue_type).unwrap())
        ])
    }
}

pub fn get_lol_social_leaderboard_v1_social_leaderboard_data(queue_type: Option<LolSocialLeaderboardLeagueQueueType>) -> GetLolSocialLeaderboardV1SocialLeaderboardData {
    GetLolSocialLeaderboardV1SocialLeaderboardData{queue_type}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardData {
    #[serde(rename = "rowData")]
    pub row_data: Vec<LolSocialLeaderboardSocialLeaderboardRowData>,
    #[serde(rename = "nextUpdateTime")]
    pub next_update_time: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardRowData {
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    #[serde(rename = "provisionalGamesRemaining")]
    pub provisional_games_remaining: i32,
    #[serde(rename = "isProvisional")]
    pub is_provisional: bool,
    pub tier: String,
    pub division: LolSocialLeaderboardLeagueDivision,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    pub wins: i32,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i32,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    pub availability: String,
    #[serde(rename = "leaderboardPosition")]
    pub leaderboard_position: i32,
    #[serde(rename = "isGiftable")]
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

