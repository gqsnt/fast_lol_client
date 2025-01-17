
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolTrophiesV1CurrentSummonerTrophiesProfile {}

impl IsApiRequest for GetLolTrophiesV1CurrentSummonerTrophiesProfile {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTrophiesTrophyProfileData;
    fn get_url(&self) -> String {"/lol-trophies/v1/current-summoner/trophies/profile".to_string()}
}

pub fn get_lol_trophies_v1_current_summoner_trophies_profile() -> GetLolTrophiesV1CurrentSummonerTrophiesProfile {
    GetLolTrophiesV1CurrentSummonerTrophiesProfile{}
}


pub struct GetLolTrophiesV1PlayersByPuuidTrophiesProfile {
    pub puuid: String,
}

impl IsApiRequest for GetLolTrophiesV1PlayersByPuuidTrophiesProfile {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTrophiesTrophyProfileData;
    fn get_url(&self) -> String {format!("/lol-trophies/v1/players/{}/trophies/profile", self.puuid)}
}

pub fn get_lol_trophies_v1_players_by_puuid_trophies_profile(puuid: String) -> GetLolTrophiesV1PlayersByPuuidTrophiesProfile {
    GetLolTrophiesV1PlayersByPuuidTrophiesProfile{puuid}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesTrophyProfileData {
    pub theme: String,
    pub tier: i64,
    pub bracket: i64,
    #[serde(rename = "seasonId")]
    pub season_id: i64,
    pub pedestal: String,
    pub cup: String,
    pub gem: String,
}


// ENUMS

