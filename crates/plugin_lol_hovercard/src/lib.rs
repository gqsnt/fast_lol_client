
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolHovercardV1FriendInfoByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolHovercardV1FriendInfoByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHovercardHovercardUserInfo;
    fn get_url(&self) -> String {format!("/lol-hovercard/v1/friend-info/{}", self.puuid)}
}

pub fn get_lol_hovercard_v1_friend_info_by_puuid(puuid: String) -> GetLolHovercardV1FriendInfoByPuuid {
    GetLolHovercardV1FriendInfoByPuuid{puuid}
}


pub struct GetLolHovercardV1FriendInfoBySummonerBySummonerId {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolHovercardV1FriendInfoBySummonerBySummonerId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHovercardHovercardUserInfo;
    fn get_url(&self) -> String {format!("/lol-hovercard/v1/friend-info-by-summoner/{}", self.summoner_id)}
}

pub fn get_lol_hovercard_v1_friend_info_by_summoner_by_summoner_id(summoner_id: u64) -> GetLolHovercardV1FriendInfoBySummonerBySummonerId {
    GetLolHovercardV1FriendInfoBySummonerBySummonerId{summoner_id}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardHovercardUserInfo {
    pub id: String,
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub name: String,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    pub icon: i32,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameTag")]
    pub game_tag: String,
    pub availability: String,
    pub note: String,
    #[serde(rename = "masteryScore")]
    pub mastery_score: u64,
    #[serde(rename = "legendaryMasteryScore")]
    pub legendary_mastery_score: u64,
    pub patchline: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    pub product: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "statusMessage")]
    pub status_message: String,
    #[serde(rename = "summonerIcon")]
    pub summoner_icon: i32,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: u32,
    #[serde(rename = "remoteProduct")]
    pub remote_product: bool,
    #[serde(rename = "remotePlatform")]
    pub remote_platform: bool,
    #[serde(rename = "remoteProductIconUrl")]
    pub remote_product_icon_url: String,
    #[serde(rename = "remoteProductBackdropUrl")]
    pub remote_product_backdrop_url: String,
    #[serde(rename = "partySummoners")]
    pub party_summoners: Vec<String>,
    pub lol: HashMap<String, String>,
}


// ENUMS

