
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetDeepLinksV1Settings {

}

impl IsApiRequest for GetDeepLinksV1Settings {
    const METHOD: Method = Method::GET;
    type ReturnType = DeepLinksDeepLinksSettings;

    fn get_url(&self) -> String {
        "/deep-links/v1/settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_deep_links_v_1_settings() -> GetDeepLinksV1Settings {
    GetDeepLinksV1Settings {
        
    }
}


pub struct PostDeepLinksV1LaunchLorLink {

}

impl IsApiRequest for PostDeepLinksV1LaunchLorLink {
    const METHOD: Method = Method::POST;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/deep-links/v1/launch-lor-link".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_deep_links_v_1_launch_lor_link() -> PostDeepLinksV1LaunchLorLink {
    PostDeepLinksV1LaunchLorLink {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeepLinksDeepLinksSettings {
    pub is_scheme_ready: bool,
    pub external_client_scheme: String,
    pub launch_lor_enabled: bool,
    pub launch_lor_url: String,
}


// ENUMS

