
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolTastesV1Ready {

}

impl IsApiRequest for GetLolTastesV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-tastes/v1/ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tastes_v_1_ready() -> GetLolTastesV1Ready {
    GetLolTastesV1Ready {
        
    }
}


pub struct GetLolTastesV1SkinsModel {

}

impl IsApiRequest for GetLolTastesV1SkinsModel {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTastesDataModelResponse;

    fn get_url(&self) -> String {
        "/lol-tastes/v1/skins-model".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tastes_v_1_skins_model() -> GetLolTastesV1SkinsModel {
    GetLolTastesV1SkinsModel {
        
    }
}


pub struct GetLolTastesV1TftOverviewModel {

}

impl IsApiRequest for GetLolTastesV1TftOverviewModel {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTastesDataModelResponse;

    fn get_url(&self) -> String {
        "/lol-tastes/v1/tft-overview-model".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tastes_v_1_tft_overview_model() -> GetLolTastesV1TftOverviewModel {
    GetLolTastesV1TftOverviewModel {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTastesDataModelResponse {
    pub response_code: i64,
    pub model_data: HashMap<String, String>,
}


// ENUMS

