
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolLoyaltyV1StatusNotification {

}

impl IsApiRequest for GetLolLoyaltyV1StatusNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoyaltyLoyaltyStatusNotification;

    fn get_url(&self) -> String {
        "/lol-loyalty/v1/status-notification".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loyalty_v_1_status_notification() -> GetLolLoyaltyV1StatusNotification {
    GetLolLoyaltyV1StatusNotification {
        
    }
}


pub struct PostLolLoyaltyV1UpdateLoyaltyInventory {

    pub body: LolLoyaltyLoyaltyRewards,
}

impl IsApiRequest for PostLolLoyaltyV1UpdateLoyaltyInventory {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-loyalty/v1/updateLoyaltyInventory".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loyalty_v_1_update_loyalty_inventory(body: LolLoyaltyLoyaltyRewards) -> PostLolLoyaltyV1UpdateLoyaltyInventory {
    PostLolLoyaltyV1UpdateLoyaltyInventory {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoyaltyStatusNotification {
    pub status: LolLoyaltyLoyaltyStatus,
    pub rewards: LolLoyaltyLoyaltyRewardsSimplified,
    pub reload_inventory: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoyaltyRewards {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub global: LolLoyaltyGlobalRewards,
    pub ip_boost: i32,
    pub xp_boost: HashMap<String, i32>,
    pub loyalty_tft_map_skin_count: i32,
    pub loyalty_tft_companion_count: i32,
    pub loyalty_tft_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoyaltyRewardsSimplified {
    pub free_rewarded_champions_count: i32,
    pub champion_ids: Vec<i32>,
    pub free_rewarded_skins_count: i32,
    pub skin_ids: Vec<i32>,
    pub global: LolLoyaltyGlobalRewards,
    pub ip_boost: i32,
    pub xp_boost: i32,
    pub loyalty_tft_map_skin_count: i32,
    pub loyalty_tft_companion_count: i32,
    pub loyalty_tft_damage_skin_count: i32,
    pub loyalty_sources: HashMap<String, bool>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyGlobalRewards {
    pub all_champions: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLoyaltyLoyaltyStatus {
    #[default]
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "REVOKE")]
    Revoke,
    #[serde(rename = "CHANGE")]
    Change,
    #[serde(rename = "EXPIRY")]
    Expiry,
    #[serde(rename = "REWARDS_GRANT")]
    RewardsGrant,
    #[serde(rename = "LEGACY")]
    Legacy,
}

