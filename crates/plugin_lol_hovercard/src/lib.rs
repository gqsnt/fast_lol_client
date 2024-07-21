
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolHovercardV1FriendInfoByPuuid {

    pub puuid: String,
}

impl IsApiRequest for GetLolHovercardV1FriendInfoByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHovercardHovercardUserInfo;

    fn get_url(&self) -> String {
        format!("/lol-hovercard/v1/friend-info/{}", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_hovercard_v_1_friend_info_by_puuid(puuid: String) -> GetLolHovercardV1FriendInfoByPuuid {
    GetLolHovercardV1FriendInfoByPuuid {
        puuid
    }
}


pub struct GetLolHovercardV1FriendInfoBySummonerBySummonerId {

    pub summoner_id: u64,
}

impl IsApiRequest for GetLolHovercardV1FriendInfoBySummonerBySummonerId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHovercardHovercardUserInfo;

    fn get_url(&self) -> String {
        format!("/lol-hovercard/v1/friend-info-by-summoner/{}", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_hovercard_v_1_friend_info_by_summoner_by_summoner_id(summoner_id: u64) -> GetLolHovercardV1FriendInfoBySummonerBySummonerId {
    GetLolHovercardV1FriendInfoBySummonerBySummonerId {
        summoner_id
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardHovercardUserInfo {
    pub id: String,
    pub puuid: String,
    pub summoner_id: u64,
    pub name: String,
    pub account_id: u64,
    pub icon: i32,
    pub game_name: String,
    pub game_tag: String,
    pub availability: String,
    pub note: String,
    pub mastery_score: u64,
    pub legendary_mastery_score: u64,
    pub patchline: String,
    pub platform_id: String,
    pub product: String,
    pub product_name: String,
    pub status_message: String,
    pub summoner_icon: i32,
    pub summoner_level: u32,
    pub remote_product: bool,
    pub remote_platform: bool,
    pub remote_product_icon_url: String,
    pub remote_product_backdrop_url: String,
    pub party_summoners: Vec<String>,
    pub lol: HashMap<String, String>,
}


// ENUMS

