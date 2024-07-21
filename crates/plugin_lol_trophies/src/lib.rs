
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolTrophiesV1CurrentSummonerTrophiesProfile {

}

impl IsApiRequest for GetLolTrophiesV1CurrentSummonerTrophiesProfile {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTrophiesTrophyProfileData;

    fn get_url(&self) -> String {
        "/lol-trophies/v1/current-summoner/trophies/profile".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_trophies_v_1_current_summoner_trophies_profile() -> GetLolTrophiesV1CurrentSummonerTrophiesProfile {
    GetLolTrophiesV1CurrentSummonerTrophiesProfile {
        
    }
}


pub struct GetLolTrophiesV1PlayersByPuuidTrophiesProfile {

    pub puuid: String,
}

impl IsApiRequest for GetLolTrophiesV1PlayersByPuuidTrophiesProfile {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTrophiesTrophyProfileData;

    fn get_url(&self) -> String {
        format!("/lol-trophies/v1/players/{}/trophies/profile", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_trophies_v_1_players_by_puuid_trophies_profile(puuid: String) -> GetLolTrophiesV1PlayersByPuuidTrophiesProfile {
    GetLolTrophiesV1PlayersByPuuidTrophiesProfile {
        puuid
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesTrophyProfileData {
    pub theme: String,
    pub tier: i64,
    pub bracket: i64,
    pub season_id: i64,
    pub pedestal: String,
    pub cup: String,
    pub gem: String,
}


// ENUMS

