
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolPlayerPreferencesV1PlayerPreferencesReady {}

impl IsApiRequest for GetLolPlayerPreferencesV1PlayerPreferencesReady {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-player-preferences/v1/player-preferences-ready".to_string()}
}

pub fn get_lol_player_preferences_v1_player_preferences_ready() -> GetLolPlayerPreferencesV1PlayerPreferencesReady {
    GetLolPlayerPreferencesV1PlayerPreferencesReady{}
}


pub struct GetLolPlayerPreferencesV1PreferenceByType {
    pub type_: String,
}

impl IsApiRequest for GetLolPlayerPreferencesV1PreferenceByType {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-player-preferences/v1/preference/{}", self.type_)}
}

pub fn get_lol_player_preferences_v1_preference_by_type(type_: String) -> GetLolPlayerPreferencesV1PreferenceByType {
    GetLolPlayerPreferencesV1PreferenceByType{type_}
}


pub struct PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride {
    pub body: LolPlayerPreferencesPlayerPreferencesEndpoint,
}

impl IsApiRequest for PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-player-preferences/v1/player-preferences-endpoint-override".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_player_preferences_v1_player_preferences_endpoint_override(body: LolPlayerPreferencesPlayerPreferencesEndpoint) -> PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride {
    PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride{body}
}


pub struct PutLolPlayerPreferencesV1Preference {
    pub body: LolPlayerPreferencesPlayerPreferences,
}

impl IsApiRequest for PutLolPlayerPreferencesV1Preference {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-player-preferences/v1/preference".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_player_preferences_v1_preference(body: LolPlayerPreferencesPlayerPreferences) -> PutLolPlayerPreferencesV1Preference {
    PutLolPlayerPreferencesV1Preference{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesPlayerPreferences {
    #[serde(rename = "type")]
    pub type_: String,
    pub version: String,
    pub data: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesPlayerPreferencesEndpoint {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "EnforceHttps")]
    pub enforce_https: bool,
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Retries")]
    pub retries: i64,
}


// ENUMS

