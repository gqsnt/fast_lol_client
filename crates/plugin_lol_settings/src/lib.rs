
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolSettingsV1AccountByCategory {
    pub category: String,
}

impl IsApiRequest for GetLolSettingsV1AccountByCategory {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v1/account/{}", self.category)}
}

pub fn get_lol_settings_v_1_account_by_category(category: String) -> GetLolSettingsV1AccountByCategory {
    GetLolSettingsV1AccountByCategory{category}
}


pub struct GetLolSettingsV1AccountDidreset {}

impl IsApiRequest for GetLolSettingsV1AccountDidreset {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-settings/v1/account/didreset".to_string()}
}

pub fn get_lol_settings_v_1_account_didreset() -> GetLolSettingsV1AccountDidreset {
    GetLolSettingsV1AccountDidreset{}
}


pub struct GetLolSettingsV1LocalByCategory {
    pub category: String,
}

impl IsApiRequest for GetLolSettingsV1LocalByCategory {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v1/local/{}", self.category)}
}

pub fn get_lol_settings_v_1_local_by_category(category: String) -> GetLolSettingsV1LocalByCategory {
    GetLolSettingsV1LocalByCategory{category}
}


pub struct GetLolSettingsV2AccountByPpTypeByCategory {
    pub pp_type: String,
    pub category: String,
}

impl IsApiRequest for GetLolSettingsV2AccountByPpTypeByCategory {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v2/account/{}/{}", self.pp_type, self.category)}
}

pub fn get_lol_settings_v_2_account_by_pp_type_by_category(pp_type: String, category: String) -> GetLolSettingsV2AccountByPpTypeByCategory {
    GetLolSettingsV2AccountByPpTypeByCategory{pp_type, category}
}


pub struct GetLolSettingsV2Config {}

impl IsApiRequest for GetLolSettingsV2Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSettingsSettingsConfig;
    fn get_url(&self) -> String {"/lol-settings/v2/config".to_string()}
}

pub fn get_lol_settings_v_2_config() -> GetLolSettingsV2Config {
    GetLolSettingsV2Config{}
}


pub struct GetLolSettingsV2DidresetByPpType {
    pub pp_type: String,
}

impl IsApiRequest for GetLolSettingsV2DidresetByPpType {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {format!("/lol-settings/v2/didreset/{}", self.pp_type)}
}

pub fn get_lol_settings_v_2_didreset_by_pp_type(pp_type: String) -> GetLolSettingsV2DidresetByPpType {
    GetLolSettingsV2DidresetByPpType{pp_type}
}


pub struct GetLolSettingsV2LocalByCategory {
    pub category: String,
}

impl IsApiRequest for GetLolSettingsV2LocalByCategory {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v2/local/{}", self.category)}
}

pub fn get_lol_settings_v_2_local_by_category(category: String) -> GetLolSettingsV2LocalByCategory {
    GetLolSettingsV2LocalByCategory{category}
}


pub struct GetLolSettingsV2Ready {}

impl IsApiRequest for GetLolSettingsV2Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-settings/v2/ready".to_string()}
}

pub fn get_lol_settings_v_2_ready() -> GetLolSettingsV2Ready {
    GetLolSettingsV2Ready{}
}


pub struct PatchLolSettingsV1AccountByCategory {
    pub category: String,
    pub body: LolSettingsSettingCategory,
}

impl IsApiRequest for PatchLolSettingsV1AccountByCategory {
    const METHOD: Method = Method::PATCH;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v1/account/{}", self.category)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_settings_v_1_account_by_category(category: String, body: LolSettingsSettingCategory) -> PatchLolSettingsV1AccountByCategory {
    PatchLolSettingsV1AccountByCategory{category, body}
}


pub struct PatchLolSettingsV1LocalByCategory {
    pub category: String,
    pub body: LolSettingsSettingCategory,
}

impl IsApiRequest for PatchLolSettingsV1LocalByCategory {
    const METHOD: Method = Method::PATCH;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v1/local/{}", self.category)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_settings_v_1_local_by_category(category: String, body: LolSettingsSettingCategory) -> PatchLolSettingsV1LocalByCategory {
    PatchLolSettingsV1LocalByCategory{category, body}
}


pub struct PatchLolSettingsV2AccountByPpTypeByCategory {
    pub pp_type: String,
    pub category: String,
    pub body: LolSettingsSettingCategory,
}

impl IsApiRequest for PatchLolSettingsV2AccountByPpTypeByCategory {
    const METHOD: Method = Method::PATCH;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v2/account/{}/{}", self.pp_type, self.category)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_settings_v_2_account_by_pp_type_by_category(pp_type: String, category: String, body: LolSettingsSettingCategory) -> PatchLolSettingsV2AccountByPpTypeByCategory {
    PatchLolSettingsV2AccountByPpTypeByCategory{pp_type, category, body}
}


pub struct PatchLolSettingsV2LocalByCategory {
    pub category: String,
    pub body: LolSettingsSettingCategory,
}

impl IsApiRequest for PatchLolSettingsV2LocalByCategory {
    const METHOD: Method = Method::PATCH;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v2/local/{}", self.category)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_settings_v_2_local_by_category(category: String, body: LolSettingsSettingCategory) -> PatchLolSettingsV2LocalByCategory {
    PatchLolSettingsV2LocalByCategory{category, body}
}


pub struct PostLolSettingsV1AccountSave {}

impl IsApiRequest for PostLolSettingsV1AccountSave {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-settings/v1/account/save".to_string()}
}

pub fn post_lol_settings_v_1_account_save() -> PostLolSettingsV1AccountSave {
    PostLolSettingsV1AccountSave{}
}


pub struct PostLolSettingsV2ReloadByPpType {
    pub pp_type: String,
}

impl IsApiRequest for PostLolSettingsV2ReloadByPpType {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-settings/v2/reload/{}", self.pp_type)}
}

pub fn post_lol_settings_v_2_reload_by_pp_type(pp_type: String) -> PostLolSettingsV2ReloadByPpType {
    PostLolSettingsV2ReloadByPpType{pp_type}
}


pub struct PutLolSettingsV1AccountByCategory {
    pub category: String,
    pub body: LolSettingsSettingCategory,
}

impl IsApiRequest for PutLolSettingsV1AccountByCategory {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v1/account/{}", self.category)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_settings_v_1_account_by_category(category: String, body: LolSettingsSettingCategory) -> PutLolSettingsV1AccountByCategory {
    PutLolSettingsV1AccountByCategory{category, body}
}


pub struct PutLolSettingsV2AccountByPpTypeByCategory {
    pub pp_type: String,
    pub category: String,
    pub body: LolSettingsSettingCategory,
}

impl IsApiRequest for PutLolSettingsV2AccountByPpTypeByCategory {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-settings/v2/account/{}/{}", self.pp_type, self.category)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_settings_v_2_account_by_pp_type_by_category(pp_type: String, category: String, body: LolSettingsSettingCategory) -> PutLolSettingsV2AccountByPpTypeByCategory {
    PutLolSettingsV2AccountByPpTypeByCategory{pp_type, category, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsSettingCategory {
    pub schema_version: i32,
    pub data: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsSettingsConfig {
    pub is_hotkeys_enabled: bool,
    pub is_sound_enabled: bool,
    pub is_interface_enabled: bool,
    pub is_gameplay_enabled: bool,
    pub is_replays_enabled: bool,
    pub is_privacy_notice_enabled: bool,
    pub is_terms_enabled: bool,
    pub is_legal_statements_enabled: bool,
    pub localized_licenses_url: String,
}


// ENUMS

