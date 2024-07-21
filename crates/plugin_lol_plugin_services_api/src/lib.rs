
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct PostServicesApiConfigV1ClientConfig {

    pub body: HashMap<String, String>,
}

impl IsApiRequest for PostServicesApiConfigV1ClientConfig {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/services-api/config/v1/client-config".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_services_api_config_v_1_client_config(body: HashMap<String, String>) -> PostServicesApiConfigV1ClientConfig {
    PostServicesApiConfigV1ClientConfig {
        body
    }
}


pub struct PutServicesApiGameSessionV1GameSessionToken {

    pub body: String,
}

impl IsApiRequest for PutServicesApiGameSessionV1GameSessionToken {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/services-api/game-session/v1/game-session-token".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_services_api_game_session_v_1_game_session_token(body: String) -> PutServicesApiGameSessionV1GameSessionToken {
    PutServicesApiGameSessionV1GameSessionToken {
        body
    }
}


// OBJECTS


// ENUMS

