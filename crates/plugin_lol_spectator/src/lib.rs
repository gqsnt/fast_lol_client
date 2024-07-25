
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolSpectatorV1Spectate {}

impl IsApiRequest for GetLolSpectatorV1Spectate {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSpectatorSpectateGameInfo;
    fn get_url(&self) -> String {"/lol-spectator/v1/spectate".to_string()}
}

pub fn get_lol_spectator_v1_spectate() -> GetLolSpectatorV1Spectate {
    GetLolSpectatorV1Spectate{}
}


pub struct GetLolSpectatorV1SpectateConfig {}

impl IsApiRequest for GetLolSpectatorV1SpectateConfig {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSpectatorSpectatorConfig;
    fn get_url(&self) -> String {"/lol-spectator/v1/spectate/config".to_string()}
}

pub fn get_lol_spectator_v1_spectate_config() -> GetLolSpectatorV1SpectateConfig {
    GetLolSpectatorV1SpectateConfig{}
}


pub struct PostLolSpectatorV1BuddySpectate {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolSpectatorV1BuddySpectate {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSpectatorSummonerOrTeamAvailabilty;
    fn get_url(&self) -> String {"/lol-spectator/v1/buddy/spectate".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_spectator_v1_buddy_spectate(body: Vec<String>) -> PostLolSpectatorV1BuddySpectate {
    PostLolSpectatorV1BuddySpectate{body}
}


pub struct PostLolSpectatorV1SpectateLaunch {
    pub body: LolSpectatorSpectateGameInfo,
}

impl IsApiRequest for PostLolSpectatorV1SpectateLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-spectator/v1/spectate/launch".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_spectator_v1_spectate_launch(body: LolSpectatorSpectateGameInfo) -> PostLolSpectatorV1SpectateLaunch {
    PostLolSpectatorV1SpectateLaunch{body}
}


pub struct PostLolSpectatorV2BuddySpectate {
    pub body: Vec<u64>,
}

impl IsApiRequest for PostLolSpectatorV2BuddySpectate {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSpectatorSummonerIdAvailability;
    fn get_url(&self) -> String {"/lol-spectator/v2/buddy/spectate".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_spectator_v2_buddy_spectate(body: Vec<u64>) -> PostLolSpectatorV2BuddySpectate {
    PostLolSpectatorV2BuddySpectate{body}
}


pub struct PostLolSpectatorV3BuddySpectate {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolSpectatorV3BuddySpectate {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSpectatorSummonerPuuidsSpectateResource;
    fn get_url(&self) -> String {"/lol-spectator/v3/buddy/spectate".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_spectator_v3_buddy_spectate(body: Vec<String>) -> PostLolSpectatorV3BuddySpectate {
    PostLolSpectatorV3BuddySpectate{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateGameInfo {
    #[serde(rename = "dropInSpectateGameId")]
    pub drop_in_spectate_game_id: String,
    #[serde(rename = "gameQueueType")]
    pub game_queue_type: String,
    #[serde(rename = "allowObserveMode")]
    pub allow_observe_mode: String,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectatorConfig {
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "isSpectatorDelayConfigurable")]
    pub is_spectator_delay_configurable: bool,
    #[serde(rename = "isBracketSpectatingEnabled")]
    pub is_bracket_spectating_enabled: bool,
    #[serde(rename = "spectatableQueues")]
    pub spectatable_queues: Vec<u32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerIdAvailability {
    #[serde(rename = "availableForWatching")]
    pub available_for_watching: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerOrTeamAvailabilty {
    #[serde(rename = "availableForWatching")]
    pub available_for_watching: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerPuuidsSpectateResource {
    #[serde(rename = "availableForWatching")]
    pub available_for_watching: Vec<String>,
}


// ENUMS

