
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolMarketingPreferencesV1PartitionByPartitionKey {
    pub partition_key: String,
}

impl IsApiRequest for GetLolMarketingPreferencesV1PartitionByPartitionKey {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-marketing-preferences/v1/partition/{}", self.partition_key)}
}

pub fn get_lol_marketing_preferences_v1_partition_by_partition_key(partition_key: String) -> GetLolMarketingPreferencesV1PartitionByPartitionKey {
    GetLolMarketingPreferencesV1PartitionByPartitionKey{partition_key}
}


pub struct GetLolMarketingPreferencesV1Ready {}

impl IsApiRequest for GetLolMarketingPreferencesV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-marketing-preferences/v1/ready".to_string()}
}

pub fn get_lol_marketing_preferences_v1_ready() -> GetLolMarketingPreferencesV1Ready {
    GetLolMarketingPreferencesV1Ready{}
}


pub struct PostLolMarketingPreferencesV1PartitionByPartitionKey {
    pub partition_key: String,
    pub body: HashMap<String, String>,
}

impl IsApiRequest for PostLolMarketingPreferencesV1PartitionByPartitionKey {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-marketing-preferences/v1/partition/{}", self.partition_key)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_marketing_preferences_v1_partition_by_partition_key(partition_key: String, body: HashMap<String, String>) -> PostLolMarketingPreferencesV1PartitionByPartitionKey {
    PostLolMarketingPreferencesV1PartitionByPartitionKey{partition_key, body}
}


// OBJECTS


// ENUMS

