
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolSpectatorV1Spectate {

}

impl IsApiRequest for GetLolSpectatorV1Spectate {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSpectatorSpectateGameInfo;

    fn get_url(&self) -> String {
        "/lol-spectator/v1/spectate".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_spectator_v_1_spectate() -> GetLolSpectatorV1Spectate {
    GetLolSpectatorV1Spectate {
        
    }
}


pub struct GetLolSpectatorV1SpectateConfig {

}

impl IsApiRequest for GetLolSpectatorV1SpectateConfig {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSpectatorSpectatorConfig;

    fn get_url(&self) -> String {
        "/lol-spectator/v1/spectate/config".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_spectator_v_1_spectate_config() -> GetLolSpectatorV1SpectateConfig {
    GetLolSpectatorV1SpectateConfig {
        
    }
}


pub struct PostLolSpectatorV1BuddySpectate {

    pub body: Vec<String>,
}

impl IsApiRequest for PostLolSpectatorV1BuddySpectate {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSpectatorSummonerOrTeamAvailabilty;

    fn get_url(&self) -> String {
        "/lol-spectator/v1/buddy/spectate".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_spectator_v_1_buddy_spectate(body: Vec<String>) -> PostLolSpectatorV1BuddySpectate {
    PostLolSpectatorV1BuddySpectate {
        body
    }
}


pub struct PostLolSpectatorV1SpectateLaunch {

    pub body: LolSpectatorSpectateGameInfo,
}

impl IsApiRequest for PostLolSpectatorV1SpectateLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-spectator/v1/spectate/launch".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_spectator_v_1_spectate_launch(body: LolSpectatorSpectateGameInfo) -> PostLolSpectatorV1SpectateLaunch {
    PostLolSpectatorV1SpectateLaunch {
        body
    }
}


pub struct PostLolSpectatorV2BuddySpectate {

    pub body: Vec<u64>,
}

impl IsApiRequest for PostLolSpectatorV2BuddySpectate {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSpectatorSummonerIdAvailability;

    fn get_url(&self) -> String {
        "/lol-spectator/v2/buddy/spectate".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_spectator_v_2_buddy_spectate(body: Vec<u64>) -> PostLolSpectatorV2BuddySpectate {
    PostLolSpectatorV2BuddySpectate {
        body
    }
}


pub struct PostLolSpectatorV3BuddySpectate {

    pub body: Vec<String>,
}

impl IsApiRequest for PostLolSpectatorV3BuddySpectate {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSpectatorSummonerPuuidsSpectateResource;

    fn get_url(&self) -> String {
        "/lol-spectator/v3/buddy/spectate".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_spectator_v_3_buddy_spectate(body: Vec<String>) -> PostLolSpectatorV3BuddySpectate {
    PostLolSpectatorV3BuddySpectate {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateGameInfo {
    pub drop_in_spectate_game_id: String,
    pub game_queue_type: String,
    pub allow_observe_mode: String,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectatorConfig {
    pub is_enabled: bool,
    pub is_spectator_delay_configurable: bool,
    pub is_bracket_spectating_enabled: bool,
    pub spectatable_queues: Vec<u32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerOrTeamAvailabilty {
    pub available_for_watching: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerIdAvailability {
    pub available_for_watching: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerPuuidsSpectateResource {
    pub available_for_watching: Vec<String>,
}


// ENUMS

