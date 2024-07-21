
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolPlayerPreferencesV1PlayerPreferencesReady {

}

impl IsApiRequest for GetLolPlayerPreferencesV1PlayerPreferencesReady {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-player-preferences/v1/player-preferences-ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_player_preferences_v_1_player_preferences_ready() -> GetLolPlayerPreferencesV1PlayerPreferencesReady {
    GetLolPlayerPreferencesV1PlayerPreferencesReady {
        
    }
}


pub struct GetLolPlayerPreferencesV1PreferenceByType {

    pub type_: String,
}

impl IsApiRequest for GetLolPlayerPreferencesV1PreferenceByType {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-player-preferences/v1/preference/{}", self.type_)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_player_preferences_v_1_preference_by_type(type_: String) -> GetLolPlayerPreferencesV1PreferenceByType {
    GetLolPlayerPreferencesV1PreferenceByType {
        type_
    }
}


pub struct PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride {

    pub body: LolPlayerPreferencesPlayerPreferencesEndpoint,
}

impl IsApiRequest for PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-player-preferences/v1/player-preferences-endpoint-override".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_player_preferences_v_1_player_preferences_endpoint_override(body: LolPlayerPreferencesPlayerPreferencesEndpoint) -> PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride {
    PostLolPlayerPreferencesV1PlayerPreferencesEndpointOverride {
        body
    }
}


pub struct PutLolPlayerPreferencesV1Preference {

    pub body: LolPlayerPreferencesPlayerPreferences,
}

impl IsApiRequest for PutLolPlayerPreferencesV1Preference {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-player-preferences/v1/preference".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_player_preferences_v_1_preference(body: LolPlayerPreferencesPlayerPreferences) -> PutLolPlayerPreferencesV1Preference {
    PutLolPlayerPreferencesV1Preference {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesPlayerPreferences {
    pub type_: String,
    pub version: String,
    pub data: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesPlayerPreferencesEndpoint {
    pub enabled: bool,
    pub enforce_https: bool,
    pub service_endpoint: String,
    pub version: String,
    pub retries: i64,
}


// ENUMS

