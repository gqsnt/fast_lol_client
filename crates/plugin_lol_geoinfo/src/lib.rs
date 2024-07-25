
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("ip_address".to_string(), serde_json::to_string(&self.ip_address).unwrap())
        ])
    }
}

pub fn get_lol_geoinfo_v1_getlocation(ip_address: String) -> GetLolGeoinfoV1Getlocation {
    GetLolGeoinfoV1Getlocation{ip_address}
}


pub struct GetLolGeoinfoV1Whereami {}

impl IsApiRequest for GetLolGeoinfoV1Whereami {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGeoinfoGeoInfoResponse;
    fn get_url(&self) -> String {"/lol-geoinfo/v1/whereami".to_string()}
}

pub fn get_lol_geoinfo_v1_whereami() -> GetLolGeoinfoV1Whereami {
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
    #[serde(rename = "geoInfo")]
    pub geo_info: LolGeoinfoGeoInfo,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "isLatest")]
    pub is_latest: bool,
    #[serde(rename = "isInitialized")]
    pub is_initialized: bool,
}


// ENUMS

