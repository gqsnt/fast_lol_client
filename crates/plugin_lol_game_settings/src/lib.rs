
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolGameSettingsV1Didreset {

}

impl IsApiRequest for GetLolGameSettingsV1Didreset {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/didreset".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_settings_v_1_didreset() -> GetLolGameSettingsV1Didreset {
    GetLolGameSettingsV1Didreset {
        
    }
}


pub struct GetLolGameSettingsV1GameSettings {

}

impl IsApiRequest for GetLolGameSettingsV1GameSettings {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/game-settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_settings_v_1_game_settings() -> GetLolGameSettingsV1GameSettings {
    GetLolGameSettingsV1GameSettings {
        
    }
}


pub struct PatchLolGameSettingsV1GameSettings {

    pub body: HashMap<String, String>,
}

impl IsApiRequest for PatchLolGameSettingsV1GameSettings {
    const METHOD: Method = Method::PATCH;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/game-settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn patch_lol_game_settings_v_1_game_settings(body: HashMap<String, String>) -> PatchLolGameSettingsV1GameSettings {
    PatchLolGameSettingsV1GameSettings {
        body
    }
}


pub struct GetLolGameSettingsV1GameSettingsSchema {

}

impl IsApiRequest for GetLolGameSettingsV1GameSettingsSchema {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/game-settings-schema".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_settings_v_1_game_settings_schema() -> GetLolGameSettingsV1GameSettingsSchema {
    GetLolGameSettingsV1GameSettingsSchema {
        
    }
}


pub struct GetLolGameSettingsV1InputSettings {

}

impl IsApiRequest for GetLolGameSettingsV1InputSettings {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/input-settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_settings_v_1_input_settings() -> GetLolGameSettingsV1InputSettings {
    GetLolGameSettingsV1InputSettings {
        
    }
}


pub struct PatchLolGameSettingsV1InputSettings {

    pub body: HashMap<String, String>,
}

impl IsApiRequest for PatchLolGameSettingsV1InputSettings {
    const METHOD: Method = Method::PATCH;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/input-settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn patch_lol_game_settings_v_1_input_settings(body: HashMap<String, String>) -> PatchLolGameSettingsV1InputSettings {
    PatchLolGameSettingsV1InputSettings {
        body
    }
}


pub struct GetLolGameSettingsV1InputSettingsSchema {

}

impl IsApiRequest for GetLolGameSettingsV1InputSettingsSchema {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/input-settings-schema".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_settings_v_1_input_settings_schema() -> GetLolGameSettingsV1InputSettingsSchema {
    GetLolGameSettingsV1InputSettingsSchema {
        
    }
}


pub struct GetLolGameSettingsV1Ready {

}

impl IsApiRequest for GetLolGameSettingsV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_settings_v_1_ready() -> GetLolGameSettingsV1Ready {
    GetLolGameSettingsV1Ready {
        
    }
}


pub struct PostLolGameSettingsV1ReloadPostGame {

}

impl IsApiRequest for PostLolGameSettingsV1ReloadPostGame {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/reload-post-game".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_game_settings_v_1_reload_post_game() -> PostLolGameSettingsV1ReloadPostGame {
    PostLolGameSettingsV1ReloadPostGame {
        
    }
}


pub struct PostLolGameSettingsV1Save {

}

impl IsApiRequest for PostLolGameSettingsV1Save {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-game-settings/v1/save".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_game_settings_v_1_save() -> PostLolGameSettingsV1Save {
    PostLolGameSettingsV1Save {
        
    }
}


// OBJECTS


// ENUMS

