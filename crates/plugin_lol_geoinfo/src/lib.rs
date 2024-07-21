
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolGeoinfoV1Getlocation {
    pub ip_address: String,
}

impl IsApiRequest for GetLolGeoinfoV1Getlocation {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGeoinfoGeoInfo;
    fn get_url(&self) -> String {"/lol-geoinfo/v1/getlocation".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "ip_address" : self.ip_address,
        }))
    }
}

pub fn get_lol_geoinfo_v_1_getlocation(ip_address: String) -> GetLolGeoinfoV1Getlocation {
    GetLolGeoinfoV1Getlocation{ip_address}
}


pub struct GetLolGeoinfoV1Whereami {}

impl IsApiRequest for GetLolGeoinfoV1Whereami {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGeoinfoGeoInfoResponse;
    fn get_url(&self) -> String {"/lol-geoinfo/v1/whereami".to_string()}
}

pub fn get_lol_geoinfo_v_1_whereami() -> GetLolGeoinfoV1Whereami {
    GetLolGeoinfoV1Whereami{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoGeoInfo {
    pub country: String,
    pub city: String,
    pub region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoGeoInfoResponse {
    pub success: bool,
    pub geo_info: LolGeoinfoGeoInfo,
    pub error_message: String,
    pub is_latest: bool,
    pub is_initialized: bool,
}


// ENUMS

