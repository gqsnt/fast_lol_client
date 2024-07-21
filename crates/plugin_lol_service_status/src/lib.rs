
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolServiceStatusV1LcuStatus {

}

impl IsApiRequest for GetLolServiceStatusV1LcuStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolServiceStatusServiceStatusResource;

    fn get_url(&self) -> String {
        "/lol-service-status/v1/lcu-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_service_status_v_1_lcu_status() -> GetLolServiceStatusV1LcuStatus {
    GetLolServiceStatusV1LcuStatus {
        
    }
}


pub struct GetLolServiceStatusV1TickerMessages {

}

impl IsApiRequest for GetLolServiceStatusV1TickerMessages {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolServiceStatusTickerMessage>;

    fn get_url(&self) -> String {
        "/lol-service-status/v1/ticker-messages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_service_status_v_1_ticker_messages() -> GetLolServiceStatusV1TickerMessages {
    GetLolServiceStatusV1TickerMessages {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusTickerMessage {
    pub severity: String,
    pub created_at: String,
    pub updated_at: String,
    pub heading: String,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusServiceStatusResource {
    pub status: String,
    pub human_readable_url: String,
}


// ENUMS

