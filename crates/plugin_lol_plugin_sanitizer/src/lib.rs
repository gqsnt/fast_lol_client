
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetSanitizerV1Status {

}

impl IsApiRequest for GetSanitizerV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = SanitizerSanitizerStatus;

    fn get_url(&self) -> String {
        "/sanitizer/v1/status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_sanitizer_v_1_status() -> GetSanitizerV1Status {
    GetSanitizerV1Status {
        
    }
}


pub struct PostSanitizerV1ContainsSanitized {

    pub body: SanitizerContainsSanitizedRequest,
}

impl IsApiRequest for PostSanitizerV1ContainsSanitized {
    const METHOD: Method = Method::POST;
    type ReturnType = SanitizerContainsSanitizedResponse;

    fn get_url(&self) -> String {
        "/sanitizer/v1/containsSanitized".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_sanitizer_v_1_contains_sanitized(body: SanitizerContainsSanitizedRequest) -> PostSanitizerV1ContainsSanitized {
    PostSanitizerV1ContainsSanitized {
        body
    }
}


pub struct PostSanitizerV1Sanitize {

    pub body: SanitizerSanitizeRequest,
}

impl IsApiRequest for PostSanitizerV1Sanitize {
    const METHOD: Method = Method::POST;
    type ReturnType = SanitizerSanitizeResponse;

    fn get_url(&self) -> String {
        "/sanitizer/v1/sanitize".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_sanitizer_v_1_sanitize(body: SanitizerSanitizeRequest) -> PostSanitizerV1Sanitize {
    PostSanitizerV1Sanitize {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizerStatus {
    pub ready: bool,
    pub region: String,
    pub locale: String,
    pub filtered_word_counts_by_level: HashMap<String, u32>,
    pub whitelisted_word_counts_by_level: HashMap<String, u32>,
    pub breaking_chars_count: u32,
    pub projected_chars_count: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerContainsSanitizedRequest {
    pub text: String,
    pub level: Option<u32>,
    pub aggressive_scan: Option<bool>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizeRequest {
    pub texts: Option<Vec<String>>,
    pub text: Option<String>,
    pub level: Option<u32>,
    pub aggressive_scan: Option<bool>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizeResponse {
    pub texts: Option<Vec<String>>,
    pub text: Option<String>,
    pub modified: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerContainsSanitizedResponse {
    pub contains: bool,
}


// ENUMS

