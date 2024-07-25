
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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

pub fn get_lol_publishing_content_v1_listeners_allow_list_by_region(region: String) -> GetLolPublishingContentV1ListenersAllowListByRegion {
    GetLolPublishingContentV1ListenersAllowListByRegion{region}
}


pub struct GetLolPublishingContentV1ListenersClientData {}

impl IsApiRequest for GetLolPublishingContentV1ListenersClientData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPublishingContentClientData;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/listeners/client-data".to_string()}
}

pub fn get_lol_publishing_content_v1_listeners_client_data() -> GetLolPublishingContentV1ListenersClientData {
    GetLolPublishingContentV1ListenersClientData{}
}


pub struct GetLolPublishingContentV1ListenersPubhubConfig {}

impl IsApiRequest for GetLolPublishingContentV1ListenersPubhubConfig {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPublishingContentPubHubConfig;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/listeners/pubhub-config".to_string()}
}

pub fn get_lol_publishing_content_v1_listeners_pubhub_config() -> GetLolPublishingContentV1ListenersPubhubConfig {
    GetLolPublishingContentV1ListenersPubhubConfig{}
}


pub struct GetLolPublishingContentV1Ready {}

impl IsApiRequest for GetLolPublishingContentV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/ready".to_string()}
}

pub fn get_lol_publishing_content_v1_ready() -> GetLolPublishingContentV1Ready {
    GetLolPublishingContentV1Ready{}
}


pub struct GetLolPublishingContentV1Settings {}

impl IsApiRequest for GetLolPublishingContentV1Settings {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPublishingContentPublishingSettings;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/settings".to_string()}
}

pub fn get_lol_publishing_content_v1_settings() -> GetLolPublishingContentV1Settings {
    GetLolPublishingContentV1Settings{}
}


pub struct GetLolPublishingContentV1TftHubCards {}

impl IsApiRequest for GetLolPublishingContentV1TftHubCards {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-publishing-content/v1/tft-hub-cards".to_string()}
}

pub fn get_lol_publishing_content_v1_tft_hub_cards() -> GetLolPublishingContentV1TftHubCards {
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
    #[serde(rename = "assetUrls")]
    pub asset_urls: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfig {
    pub edge: LolPublishingContentPubHubConfigEdge,
    #[serde(rename = "appContext")]
    pub app_context: LolPublishingContentPubHubConfigAppContext,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigAppContext {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userRegion")]
    pub user_region: String,
    #[serde(rename = "deviceCategory")]
    pub device_category: String,
    #[serde(rename = "deviceOperatingSystem")]
    pub device_operating_system: String,
    #[serde(rename = "deviceOperatingSystemVersion")]
    pub device_operating_system_version: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "appVersion")]
    pub app_version: String,
    #[serde(rename = "appLocale")]
    pub app_locale: String,
    #[serde(rename = "appLanguage")]
    pub app_language: String,
    #[serde(rename = "publishingLocale")]
    pub publishing_locale: String,
    #[serde(rename = "appSessionId")]
    pub app_session_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigEdge {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientRegion")]
    pub client_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPublishingSettings {
    pub region: String,
    pub locale: String,
    #[serde(rename = "webRegion")]
    pub web_region: String,
    #[serde(rename = "webLocale")]
    pub web_locale: String,
    #[serde(rename = "publishingLocale")]
    pub publishing_locale: String,
    #[serde(rename = "rsoPlatformId")]
    pub rso_platform_id: String,
}


// ENUMS

