use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::{api_request};
use crate::client::request::ApiRequest;
use crate::client::plugin::LolApiPlugin;


api_request!(
    LolApiPlugin::LolSummoner,
    LolSummonerGetCurrentSummoner,
    Method::GET,
    "/current-summoner",
    SummonerInfo
);


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SummonerInfo {
    #[serde(rename = "accountId")]
    pub account_id: i64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "internalName")]
    pub internal_name: String,
    #[serde(rename = "nameChangeFlag")]
    pub name_change_flag: bool,
    #[serde(rename = "percentCompleteForNextLevel")]
    pub percent_complete_for_next_level: i64,
    pub privacy: String,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i64,
    pub puuid: String,
    #[serde(rename = "rerollPoints")]
    pub reroll_points: RerollPoints,
    #[serde(rename = "summonerId")]
    pub summoner_id: i64,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    pub unnamed: bool,
    #[serde(rename = "xpSinceLastLevel")]
    pub xp_since_last_level: i64,
    #[serde(rename = "xpUntilNextLevel")]
    pub xp_until_next_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RerollPoints {
    #[serde(rename = "currentPoints")]
    pub current_points: i64,
    #[serde(rename = "maxRolls")]
    pub max_rolls: i64,
    #[serde(rename = "numberOfRolls")]
    pub number_of_rolls: i64,
    #[serde(rename = "pointsCostToRoll")]
    pub points_cost_to_roll: i64,
    #[serde(rename = "pointsToReroll")]
    pub points_to_reroll: i64,
}

