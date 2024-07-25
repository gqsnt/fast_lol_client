
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolGameSettingsV1Didreset {}

impl IsApiRequest for GetLolGameSettingsV1Didreset {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-game-settings/v1/didreset".to_string()}
}

pub fn get_lol_game_settings_v1_didreset() -> GetLolGameSettingsV1Didreset {
    GetLolGameSettingsV1Didreset{}
}


pub struct GetLolGameSettingsV1GameSettings {}

impl IsApiRequest for GetLolGameSettingsV1GameSettings {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-game-settings/v1/game-settings".to_string()}
}

pub fn get_lol_game_settings_v1_game_settings() -> GetLolGameSettingsV1GameSettings {
    GetLolGameSettingsV1GameSettings{}
}


pub struct GetLolGameSettingsV1GameSettingsSchema {}

impl IsApiRequest for GetLolGameSettingsV1GameSettingsSchema {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-game-settings/v1/game-settings-schema".to_string()}
}

pub fn get_lol_game_settings_v1_game_settings_schema() -> GetLolGameSettingsV1GameSettingsSchema {
    GetLolGameSettingsV1GameSettingsSchema{}
}


pub struct GetLolGameSettingsV1InputSettings {}

impl IsApiRequest for GetLolGameSettingsV1InputSettings {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-game-settings/v1/input-settings".to_string()}
}

pub fn get_lol_game_settings_v1_input_settings() -> GetLolGameSettingsV1InputSettings {
    GetLolGameSettingsV1InputSettings{}
}


pub struct GetLolGameSettingsV1InputSettingsSchema {}

impl IsApiRequest for GetLolGameSettingsV1InputSettingsSchema {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-game-settings/v1/input-settings-schema".to_string()}
}

pub fn get_lol_game_settings_v1_input_settings_schema() -> GetLolGameSettingsV1InputSettingsSchema {
    GetLolGameSettingsV1InputSettingsSchema{}
}


pub struct GetLolGameSettingsV1Ready {}

impl IsApiRequest for GetLolGameSettingsV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-game-settings/v1/ready".to_string()}
}

pub fn get_lol_game_settings_v1_ready() -> GetLolGameSettingsV1Ready {
    GetLolGameSettingsV1Ready{}
}


pub struct PatchLolGameSettingsV1GameSettings {
    pub body: Value,
}

impl IsApiRequest for PatchLolGameSettingsV1GameSettings {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-game-settings/v1/game-settings".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_game_settings_v1_game_settings(body: Value) -> PatchLolGameSettingsV1GameSettings {
    PatchLolGameSettingsV1GameSettings{body}
}


pub struct PatchLolGameSettingsV1InputSettings {
    pub body: Value,
}

impl IsApiRequest for PatchLolGameSettingsV1InputSettings {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-game-settings/v1/input-settings".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_game_settings_v1_input_settings(body: Value) -> PatchLolGameSettingsV1InputSettings {
    PatchLolGameSettingsV1InputSettings{body}
}


pub struct PostLolGameSettingsV1ReloadPostGame {}

impl IsApiRequest for PostLolGameSettingsV1ReloadPostGame {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-game-settings/v1/reload-post-game".to_string()}
}

pub fn post_lol_game_settings_v1_reload_post_game() -> PostLolGameSettingsV1ReloadPostGame {
    PostLolGameSettingsV1ReloadPostGame{}
}


pub struct PostLolGameSettingsV1Save {}

impl IsApiRequest for PostLolGameSettingsV1Save {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-game-settings/v1/save".to_string()}
}

pub fn post_lol_game_settings_v1_save() -> PostLolGameSettingsV1Save {
    PostLolGameSettingsV1Save{}
}


// OBJECTS


// ENUMS

