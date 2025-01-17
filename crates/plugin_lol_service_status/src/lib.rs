
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolServiceStatusV1LcuStatus {}

impl IsApiRequest for GetLolServiceStatusV1LcuStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolServiceStatusServiceStatusResource;
    fn get_url(&self) -> String {"/lol-service-status/v1/lcu-status".to_string()}
}

pub fn get_lol_service_status_v1_lcu_status() -> GetLolServiceStatusV1LcuStatus {
    GetLolServiceStatusV1LcuStatus{}
}


pub struct GetLolServiceStatusV1TickerMessages {}

impl IsApiRequest for GetLolServiceStatusV1TickerMessages {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolServiceStatusTickerMessage>;
    fn get_url(&self) -> String {"/lol-service-status/v1/ticker-messages".to_string()}
}

pub fn get_lol_service_status_v1_ticker_messages() -> GetLolServiceStatusV1TickerMessages {
    GetLolServiceStatusV1TickerMessages{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusServiceStatusResource {
    pub status: String,
    #[serde(rename = "humanReadableUrl")]
    pub human_readable_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusTickerMessage {
    pub severity: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub heading: String,
    pub message: String,
}


// ENUMS

