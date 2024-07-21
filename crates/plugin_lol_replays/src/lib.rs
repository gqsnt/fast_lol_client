
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolReplaysV1Configuration {

}

impl IsApiRequest for GetLolReplaysV1Configuration {
    const METHOD: Method = Method::GET;
    type ReturnType = LolReplaysReplaysConfiguration;

    fn get_url(&self) -> String {
        "/lol-replays/v1/configuration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_replays_v_1_configuration() -> GetLolReplaysV1Configuration {
    GetLolReplaysV1Configuration {
        
    }
}


pub struct GetLolReplaysV1MetadataByGameId {

    pub game_id: u64,
}

impl IsApiRequest for GetLolReplaysV1MetadataByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolReplaysReplayMetadata;

    fn get_url(&self) -> String {
        format!("/lol-replays/v1/metadata/{}", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_replays_v_1_metadata_by_game_id(game_id: u64) -> GetLolReplaysV1MetadataByGameId {
    GetLolReplaysV1MetadataByGameId {
        game_id
    }
}


pub struct GetLolReplaysV1RoflsPath {

}

impl IsApiRequest for GetLolReplaysV1RoflsPath {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-replays/v1/rofls/path".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_replays_v_1_rofls_path() -> GetLolReplaysV1RoflsPath {
    GetLolReplaysV1RoflsPath {
        
    }
}


pub struct GetLolReplaysV1RoflsPathDefault {

}

impl IsApiRequest for GetLolReplaysV1RoflsPathDefault {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-replays/v1/rofls/path/default".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_replays_v_1_rofls_path_default() -> GetLolReplaysV1RoflsPathDefault {
    GetLolReplaysV1RoflsPathDefault {
        
    }
}


pub struct PostLolReplaysV1MetadataByGameIdCreateGameVersionByGameVersionGameTypeByGameTypeQueueIdByQueueId {

    pub game_id: u64,
    pub game_version: String,
    pub game_type: String,
    pub queue_id: i32,
}

impl IsApiRequest for PostLolReplaysV1MetadataByGameIdCreateGameVersionByGameVersionGameTypeByGameTypeQueueIdByQueueId {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-replays/v1/metadata/{}/create/gameVersion/{}/gameType/{}/queueId/{}", self.game_id, self.game_version, self.game_type, self.queue_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_replays_v_1_metadata_by_game_id_create_game_version_by_game_version_game_type_by_game_type_queue_id_by_queue_id(game_id: u64, game_version: String, game_type: String, queue_id: i32) -> PostLolReplaysV1MetadataByGameIdCreateGameVersionByGameVersionGameTypeByGameTypeQueueIdByQueueId {
    PostLolReplaysV1MetadataByGameIdCreateGameVersionByGameVersionGameTypeByGameTypeQueueIdByQueueId {
        game_id, game_version, game_type, queue_id
    }
}


pub struct PostLolReplaysV1RoflsByGameIdDownload {

    pub game_id: u64,
    pub body: LolReplaysReplayContextData,
}

impl IsApiRequest for PostLolReplaysV1RoflsByGameIdDownload {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-replays/v1/rofls/{}/download", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_replays_v_1_rofls_by_game_id_download(game_id: u64, body: LolReplaysReplayContextData) -> PostLolReplaysV1RoflsByGameIdDownload {
    PostLolReplaysV1RoflsByGameIdDownload {
        game_id, body
    }
}


pub struct PostLolReplaysV1RoflsByGameIdDownloadGraceful {

    pub game_id: u64,
    pub body: LolReplaysReplayContextData,
}

impl IsApiRequest for PostLolReplaysV1RoflsByGameIdDownloadGraceful {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-replays/v1/rofls/{}/download/graceful", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_replays_v_1_rofls_by_game_id_download_graceful(game_id: u64, body: LolReplaysReplayContextData) -> PostLolReplaysV1RoflsByGameIdDownloadGraceful {
    PostLolReplaysV1RoflsByGameIdDownloadGraceful {
        game_id, body
    }
}


pub struct PostLolReplaysV1RoflsByGameIdWatch {

    pub game_id: u64,
    pub body: LolReplaysReplayContextData,
}

impl IsApiRequest for PostLolReplaysV1RoflsByGameIdWatch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-replays/v1/rofls/{}/watch", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_replays_v_1_rofls_by_game_id_watch(game_id: u64, body: LolReplaysReplayContextData) -> PostLolReplaysV1RoflsByGameIdWatch {
    PostLolReplaysV1RoflsByGameIdWatch {
        game_id, body
    }
}


pub struct PostLolReplaysV1RoflsScan {

}

impl IsApiRequest for PostLolReplaysV1RoflsScan {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-replays/v1/rofls/scan".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_replays_v_1_rofls_scan() -> PostLolReplaysV1RoflsScan {
    PostLolReplaysV1RoflsScan {
        
    }
}


pub struct PostLolReplaysV2MetadataByGameIdCreate {

    pub game_id: u64,
    pub body: LolReplaysReplayCreateMetadata,
}

impl IsApiRequest for PostLolReplaysV2MetadataByGameIdCreate {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-replays/v2/metadata/{}/create", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_replays_v_2_metadata_by_game_id_create(game_id: u64, body: LolReplaysReplayCreateMetadata) -> PostLolReplaysV2MetadataByGameIdCreate {
    PostLolReplaysV2MetadataByGameIdCreate {
        game_id, body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayContextData {
    pub component_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayCreateMetadata {
    pub game_version: String,
    pub game_type: String,
    pub queue_id: i32,
    pub game_end: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplaysConfiguration {
    pub is_replays_enabled: bool,
    pub is_replays_for_end_of_game_enabled: bool,
    pub is_replays_for_match_history_enabled: bool,
    pub is_patching: bool,
    pub is_in_tournament: bool,
    pub is_playing_game: bool,
    pub is_playing_replay: bool,
    pub is_logged_in: bool,
    pub game_version: String,
    pub min_server_version: String,
    pub minutes_until_replay_considered_lost: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayMetadata {
    pub state: LolReplaysMetadataState,
    pub game_id: u64,
    pub download_progress: u32,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolReplaysMetadataState {
    #[default]
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "retryDownload")]
    RetryDownload,
    #[serde(rename = "missingOrExpired")]
    MissingOrExpired,
    #[serde(rename = "incompatible")]
    Incompatible,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "watch")]
    Watch,
    #[serde(rename = "found")]
    Found,
    #[serde(rename = "checking")]
    Checking,
}

