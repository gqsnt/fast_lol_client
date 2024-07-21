
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetEntitlementsV1Token {}

impl IsApiRequest for GetEntitlementsV1Token {
    const METHOD: Method = Method::GET;
    type ReturnType = EntitlementsToken;
    fn get_url(&self) -> String {"/entitlements/v1/token".to_string()}
}

pub fn get_entitlements_v_1_token() -> GetEntitlementsV1Token {
    GetEntitlementsV1Token{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsToken {
    pub access_token: String,
    pub token: String,
    pub subject: String,
    pub issuer: String,
    pub entitlements: Vec<String>,
}


// ENUMS

