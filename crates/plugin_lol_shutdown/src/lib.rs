
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolShutdownV1Notification {

}

impl IsApiRequest for GetLolShutdownV1Notification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolShutdownShutdownNotification;

    fn get_url(&self) -> String {
        "/lol-shutdown/v1/notification".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_shutdown_v_1_notification() -> GetLolShutdownV1Notification {
    GetLolShutdownV1Notification {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolShutdownShutdownNotification {
    pub reason: LolShutdownShutdownReason,
    pub countdown: f32,
    pub additional_info: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolShutdownShutdownReason {
    #[default]
    PlayerBanned,
    LcuAlphaDisabled,
    PlatformMaintenance,
    Invalid,
}

