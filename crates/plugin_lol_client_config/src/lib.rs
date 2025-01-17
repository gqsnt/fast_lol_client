
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolClientConfigV3ClientConfigByName {
    pub name: String,
}

impl IsApiRequest for GetLolClientConfigV3ClientConfigByName {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-client-config/v3/client-config/{}", self.name)}
}

pub fn get_lol_client_config_v3_client_config_by_name(name: String) -> GetLolClientConfigV3ClientConfigByName {
    GetLolClientConfigV3ClientConfigByName{name}
}


pub struct GetLolClientConfigV3ClientConfigOperationalByName {
    pub name: String,
}

impl IsApiRequest for GetLolClientConfigV3ClientConfigOperationalByName {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-client-config/v3/client-config/operational/{}", self.name)}
}

pub fn get_lol_client_config_v3_client_config_operational_by_name(name: String) -> GetLolClientConfigV3ClientConfigOperationalByName {
    GetLolClientConfigV3ClientConfigOperationalByName{name}
}


// OBJECTS


// ENUMS

