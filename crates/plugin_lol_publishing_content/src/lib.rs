
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolPublishingContentV1ListenersAllowListByRegion {
    pub region: String,
}

impl IsApiRequest for GetLolPublishingContentV1ListenersAllowListByRegion {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {format!("/lol-publishing-content/v1/listeners/allow-list/{}", self.region)}
}

pub fn get_lol_publishing_content_v_1_listeners_allow_list_by_region(region: String) -> GetLolPublishingContentV1ListenersAllowListByRegion {
    GetLolPublishingContentV1ListenersAllowListByRegion{region}
}


pub struct GetLolPublishingContentV1ListenersClientData {}

impl IsApiRequest for GetLolPublishingContentV1ListenersClientData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPublishingContentClientData;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/listeners/client-data".to_string()}
}

pub fn get_lol_publishing_content_v_1_listeners_client_data() -> GetLolPublishingContentV1ListenersClientData {
    GetLolPublishingContentV1ListenersClientData{}
}


pub struct GetLolPublishingContentV1ListenersPubhubConfig {}

impl IsApiRequest for GetLolPublishingContentV1ListenersPubhubConfig {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPublishingContentPubHubConfig;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/listeners/pubhub-config".to_string()}
}

pub fn get_lol_publishing_content_v_1_listeners_pubhub_config() -> GetLolPublishingContentV1ListenersPubhubConfig {
    GetLolPublishingContentV1ListenersPubhubConfig{}
}


pub struct GetLolPublishingContentV1Ready {}

impl IsApiRequest for GetLolPublishingContentV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/ready".to_string()}
}

pub fn get_lol_publishing_content_v_1_ready() -> GetLolPublishingContentV1Ready {
    GetLolPublishingContentV1Ready{}
}


pub struct GetLolPublishingContentV1Settings {}

impl IsApiRequest for GetLolPublishingContentV1Settings {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPublishingContentPublishingSettings;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/settings".to_string()}
}

pub fn get_lol_publishing_content_v_1_settings() -> GetLolPublishingContentV1Settings {
    GetLolPublishingContentV1Settings{}
}


pub struct GetLolPublishingContentV1TftHubCards {}

impl IsApiRequest for GetLolPublishingContentV1TftHubCards {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/tft-hub-cards".to_string()}
}

pub fn get_lol_publishing_content_v_1_tft_hub_cards() -> GetLolPublishingContentV1TftHubCards {
    GetLolPublishingContentV1TftHubCards{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentClientData {
    pub puuid: String,
    pub account_id: u64,
    pub env: String,
    pub web_region: String,
    pub locale: String,
    pub summoner_level: u16,
    pub summoner_name: String,
    pub app_name: String,
    pub app_version: String,
    pub system_os: String,
    pub protocol: String,
    pub port: u16,
    pub asset_urls: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfig {
    pub edge: LolPublishingContentPubHubConfigEdge,
    pub app_context: LolPublishingContentPubHubConfigAppContext,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigAppContext {
    pub user_id: String,
    pub user_region: String,
    pub device_category: String,
    pub device_operating_system: String,
    pub device_operating_system_version: String,
    pub app_id: String,
    pub app_version: String,
    pub app_locale: String,
    pub app_language: String,
    pub publishing_locale: String,
    pub app_session_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigEdge {
    pub client_id: String,
    pub client_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPublishingSettings {
    pub region: String,
    pub locale: String,
    pub web_region: String,
    pub web_locale: String,
    pub publishing_locale: String,
    pub rso_platform_id: String,
}


// ENUMS

