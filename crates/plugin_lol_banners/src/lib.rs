
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolBannersV1CurrentSummonerFlags {

}

impl IsApiRequest for GetLolBannersV1CurrentSummonerFlags {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolBannersBannerFlag>;

    fn get_url(&self) -> String {
        "/lol-banners/v1/current-summoner/flags".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_banners_v_1_current_summoner_flags() -> GetLolBannersV1CurrentSummonerFlags {
    GetLolBannersV1CurrentSummonerFlags {
        
    }
}


pub struct GetLolBannersV1CurrentSummonerFlagsEquipped {

}

impl IsApiRequest for GetLolBannersV1CurrentSummonerFlagsEquipped {
    const METHOD: Method = Method::GET;
    type ReturnType = LolBannersBannerFlag;

    fn get_url(&self) -> String {
        "/lol-banners/v1/current-summoner/flags/equipped".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_banners_v_1_current_summoner_flags_equipped() -> GetLolBannersV1CurrentSummonerFlagsEquipped {
    GetLolBannersV1CurrentSummonerFlagsEquipped {
        
    }
}


pub struct GetLolBannersV1CurrentSummonerFramesEquipped {

}

impl IsApiRequest for GetLolBannersV1CurrentSummonerFramesEquipped {
    const METHOD: Method = Method::GET;
    type ReturnType = LolBannersBannerFrame;

    fn get_url(&self) -> String {
        "/lol-banners/v1/current-summoner/frames/equipped".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_banners_v_1_current_summoner_frames_equipped() -> GetLolBannersV1CurrentSummonerFramesEquipped {
    GetLolBannersV1CurrentSummonerFramesEquipped {
        
    }
}


pub struct GetLolBannersV1PlayersByPuuidFlagsEquipped {

    pub puuid: String,
}

impl IsApiRequest for GetLolBannersV1PlayersByPuuidFlagsEquipped {
    const METHOD: Method = Method::GET;
    type ReturnType = LolBannersBannerFlag;

    fn get_url(&self) -> String {
        format!("/lol-banners/v1/players/{}/flags/equipped", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_banners_v_1_players_by_puuid_flags_equipped(puuid: String) -> GetLolBannersV1PlayersByPuuidFlagsEquipped {
    GetLolBannersV1PlayersByPuuidFlagsEquipped {
        puuid
    }
}


pub struct PutLolBannersV1CurrentSummonerFlagsEquipped {

    pub body: LolBannersBannerFlag,
}

impl IsApiRequest for PutLolBannersV1CurrentSummonerFlagsEquipped {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolBannersBannerFlag;

    fn get_url(&self) -> String {
        "/lol-banners/v1/current-summoner/flags/equipped".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_banners_v_1_current_summoner_flags_equipped(body: LolBannersBannerFlag) -> PutLolBannersV1CurrentSummonerFlagsEquipped {
    PutLolBannersV1CurrentSummonerFlagsEquipped {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersBannerFlag {
    pub item_id: i32,
    pub theme: String,
    pub level: i64,
    pub season_id: i64,
    pub earned_date_iso_8601: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersBannerFrame {
    pub level: i64,
}


// ENUMS

