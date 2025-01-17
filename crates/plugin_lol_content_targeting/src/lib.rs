
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolContentTargetingV1Filters {}

impl IsApiRequest for GetLolContentTargetingV1Filters {
    const METHOD: Method = Method::GET;
    type ReturnType = LolContentTargetingContentTargetingFilterResponse;
    fn get_url(&self) -> String {"/lol-content-targeting/v1/filters".to_string()}
}

pub fn get_lol_content_targeting_v1_filters() -> GetLolContentTargetingV1Filters {
    GetLolContentTargetingV1Filters{}
}


pub struct GetLolContentTargetingV1Locale {}

impl IsApiRequest for GetLolContentTargetingV1Locale {
    const METHOD: Method = Method::GET;
    type ReturnType = LolContentTargetingContentTargetingLocaleResponse;
    fn get_url(&self) -> String {"/lol-content-targeting/v1/locale".to_string()}
}

pub fn get_lol_content_targeting_v1_locale() -> GetLolContentTargetingV1Locale {
    GetLolContentTargetingV1Locale{}
}


pub struct GetLolContentTargetingV1ProtectedFilters {}

impl IsApiRequest for GetLolContentTargetingV1ProtectedFilters {
    const METHOD: Method = Method::GET;
    type ReturnType = LolContentTargetingContentTargetingFilterResponse;
    fn get_url(&self) -> String {"/lol-content-targeting/v1/protected_filters".to_string()}
}

pub fn get_lol_content_targeting_v1_protected_filters() -> GetLolContentTargetingV1ProtectedFilters {
    GetLolContentTargetingV1ProtectedFilters{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingContentTargetingFilterResponse {
    pub filters: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingContentTargetingLocaleResponse {
    pub locale: String,
}


// ENUMS

