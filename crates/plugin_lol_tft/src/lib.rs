
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolTftV1TftBackgrounds {

}

impl IsApiRequest for GetLolTftV1TftBackgrounds {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftLolTftBackgrounds;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/backgrounds".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_backgrounds() -> GetLolTftV1TftBackgrounds {
    GetLolTftV1TftBackgrounds {
        
    }
}


pub struct GetLolTftV1TftBattlePassHub {

}

impl IsApiRequest for GetLolTftV1TftBattlePassHub {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftLolTftBattlePassHub;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/battlePassHub".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_battle_pass_hub() -> GetLolTftV1TftBattlePassHub {
    GetLolTftV1TftBattlePassHub {
        
    }
}


pub struct GetLolTftV1TftDirectToHub {

}

impl IsApiRequest for GetLolTftV1TftDirectToHub {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/directToHub".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_direct_to_hub() -> GetLolTftV1TftDirectToHub {
    GetLolTftV1TftDirectToHub {
        
    }
}


pub struct GetLolTftV1TftEvents {

}

impl IsApiRequest for GetLolTftV1TftEvents {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftLolTftEvents;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/events".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_events() -> GetLolTftV1TftEvents {
    GetLolTftV1TftEvents {
        
    }
}


pub struct GetLolTftV1TftHomeHub {

}

impl IsApiRequest for GetLolTftV1TftHomeHub {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftLolTftHomeHub;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/homeHub".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_home_hub() -> GetLolTftV1TftHomeHub {
    GetLolTftV1TftHomeHub {
        
    }
}


pub struct GetLolTftV1TftNewsHub {

}

impl IsApiRequest for GetLolTftV1TftNewsHub {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftLolTftNewsHub;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/newsHub".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_news_hub() -> GetLolTftV1TftNewsHub {
    GetLolTftV1TftNewsHub {
        
    }
}


pub struct GetLolTftV1TftPromoButtons {

}

impl IsApiRequest for GetLolTftV1TftPromoButtons {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftLolTftPromoButtons;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/promoButtons".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_promo_buttons() -> GetLolTftV1TftPromoButtons {
    GetLolTftV1TftPromoButtons {
        
    }
}


pub struct GetLolTftV1TftTencentEventhubConfigs {

}

impl IsApiRequest for GetLolTftV1TftTencentEventhubConfigs {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftLolTftTencentEventHubConfigs;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/tencentEventhubConfigs".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_v_1_tft_tencent_eventhub_configs() -> GetLolTftV1TftTencentEventhubConfigs {
    GetLolTftV1TftTencentEventhubConfigs {
        
    }
}


pub struct PostLolTftV1TftHomeHubRedirect {

}

impl IsApiRequest for PostLolTftV1TftHomeHubRedirect {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft/homeHub/redirect".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_tft_v_1_tft_home_hub_redirect() -> PostLolTftV1TftHomeHubRedirect {
    PostLolTftV1TftHomeHubRedirect {
        
    }
}


pub struct PutLolTftV1TftExperimentBucket {

    pub body: u8,
}

impl IsApiRequest for PutLolTftV1TftExperimentBucket {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-tft/v1/tft_experiment_bucket".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_tft_v_1_tft_experiment_bucket(body: u8) -> PutLolTftV1TftExperimentBucket {
    PutLolTftV1TftExperimentBucket {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftBackgrounds {
    pub backgrounds: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftBattlePassHub {
    pub battle_pass_xp_boosted: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftEvent {
    pub title_translation_key: String,
    pub enabled: bool,
    pub url: String,
    pub url_faq: String,
    pub start_date: String,
    pub end_date: String,
    pub series_id: String,
    pub daily_login_series_id: String,
    pub queue_ids: Vec<i32>,
    pub default_landing_page: bool,
    pub event_hub_template_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftEvents {
    pub sub_nav_tabs: Vec<LolTftLolTftEvent>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftHomeHub {
    pub enabled: bool,
    pub store_promo_offer_ids: Vec<String>,
    pub tactician_promo_offer_ids: Vec<String>,
    pub battle_pass_offer_ids: Vec<String>,
    pub fallback_store_promo_offer_ids: Vec<String>,
    pub prime_gaming_promo_offer: Option<LolTftLolTftPrimeGaming>,
    pub override_url: String,
    pub header_buttons_override_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftNewsHub {
    pub enabled: bool,
    pub url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftPrimeGaming {
    pub url: String,
    pub asset_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftPromoButton {
    pub enabled: bool,
    pub show_timer_while_event_active: bool,
    pub event_asset_id: String,
    pub event_key: String,
    pub url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftPromoButtons {
    pub promo_buttons: Vec<LolTftLolTftPromoButton>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftTencentEventHubConfig {
    pub trove_asset_id: String,
    pub trove_url: String,
    pub logo_asset_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftTencentEventHubConfigs {
    pub tencent_eventhub_configs: Vec<LolTftLolTftTencentEventHubConfig>,
}


// ENUMS

