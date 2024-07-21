
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolKickoutV1Notification {}

impl IsApiRequest for GetLolKickoutV1Notification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolKickoutKickoutMessage;
    fn get_url(&self) -> String {"/lol-kickout/v1/notification".to_string()}
}

pub fn get_lol_kickout_v_1_notification() -> GetLolKickoutV1Notification {
    GetLolKickoutV1Notification{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolKickoutKickoutMessage {
    pub message: String,
}


// ENUMS

