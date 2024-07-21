
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteRiotMessagingServiceV1Connect {

}

impl IsApiRequest for DeleteRiotMessagingServiceV1Connect {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riot-messaging-service/v1/connect".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_riot_messaging_service_v_1_connect() -> DeleteRiotMessagingServiceV1Connect {
    DeleteRiotMessagingServiceV1Connect {
        
    }
}


pub struct DeleteRiotMessagingServiceV1Entitlements {

}

impl IsApiRequest for DeleteRiotMessagingServiceV1Entitlements {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riot-messaging-service/v1/entitlements".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_riot_messaging_service_v_1_entitlements() -> DeleteRiotMessagingServiceV1Entitlements {
    DeleteRiotMessagingServiceV1Entitlements {
        
    }
}


pub struct DeleteRiotMessagingServiceV1Session {

}

impl IsApiRequest for DeleteRiotMessagingServiceV1Session {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riot-messaging-service/v1/session".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_riot_messaging_service_v_1_session() -> DeleteRiotMessagingServiceV1Session {
    DeleteRiotMessagingServiceV1Session {
        
    }
}


pub struct GetRiotMessagingServiceV1MessageByA {

    pub a: String,
}

impl IsApiRequest for GetRiotMessagingServiceV1MessageByA {
    const METHOD: Method = Method::GET;
    type ReturnType = RmsMessage;

    fn get_url(&self) -> String {
        format!("/riot-messaging-service/v1/message/{}", self.a)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_message_by_a(a: String) -> GetRiotMessagingServiceV1MessageByA {
    GetRiotMessagingServiceV1MessageByA {
        a
    }
}


pub struct GetRiotMessagingServiceV1MessageByAByB {

    pub a: String,
    pub b: String,
}

impl IsApiRequest for GetRiotMessagingServiceV1MessageByAByB {
    const METHOD: Method = Method::GET;
    type ReturnType = RmsMessage;

    fn get_url(&self) -> String {
        format!("/riot-messaging-service/v1/message/{}/{}", self.a, self.b)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_message_by_a_by_b(a: String, b: String) -> GetRiotMessagingServiceV1MessageByAByB {
    GetRiotMessagingServiceV1MessageByAByB {
        a, b
    }
}


pub struct GetRiotMessagingServiceV1MessageByAByBByC {

    pub a: String,
    pub b: String,
    pub c: String,
}

impl IsApiRequest for GetRiotMessagingServiceV1MessageByAByBByC {
    const METHOD: Method = Method::GET;
    type ReturnType = RmsMessage;

    fn get_url(&self) -> String {
        format!("/riot-messaging-service/v1/message/{}/{}/{}", self.a, self.b, self.c)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c(a: String, b: String, c: String) -> GetRiotMessagingServiceV1MessageByAByBByC {
    GetRiotMessagingServiceV1MessageByAByBByC {
        a, b, c
    }
}


pub struct GetRiotMessagingServiceV1MessageByAByBByCByD {

    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
}

impl IsApiRequest for GetRiotMessagingServiceV1MessageByAByBByCByD {
    const METHOD: Method = Method::GET;
    type ReturnType = RmsMessage;

    fn get_url(&self) -> String {
        format!("/riot-messaging-service/v1/message/{}/{}/{}/{}", self.a, self.b, self.c, self.d)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c_by_d(a: String, b: String, c: String, d: String) -> GetRiotMessagingServiceV1MessageByAByBByCByD {
    GetRiotMessagingServiceV1MessageByAByBByCByD {
        a, b, c, d
    }
}


pub struct GetRiotMessagingServiceV1MessageByAByBByCByDByE {

    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
    pub e: String,
}

impl IsApiRequest for GetRiotMessagingServiceV1MessageByAByBByCByDByE {
    const METHOD: Method = Method::GET;
    type ReturnType = RmsMessage;

    fn get_url(&self) -> String {
        format!("/riot-messaging-service/v1/message/{}/{}/{}/{}/{}", self.a, self.b, self.c, self.d, self.e)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c_by_d_by_e(a: String, b: String, c: String, d: String, e: String) -> GetRiotMessagingServiceV1MessageByAByBByCByDByE {
    GetRiotMessagingServiceV1MessageByAByBByCByDByE {
        a, b, c, d, e
    }
}


pub struct GetRiotMessagingServiceV1MessageByAByBByCByDByEByF {

    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
    pub e: String,
    pub f: String,
}

impl IsApiRequest for GetRiotMessagingServiceV1MessageByAByBByCByDByEByF {
    const METHOD: Method = Method::GET;
    type ReturnType = RmsMessage;

    fn get_url(&self) -> String {
        format!("/riot-messaging-service/v1/message/{}/{}/{}/{}/{}/{}", self.a, self.b, self.c, self.d, self.e, self.f)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c_by_d_by_e_by_f(a: String, b: String, c: String, d: String, e: String, f: String) -> GetRiotMessagingServiceV1MessageByAByBByCByDByEByF {
    GetRiotMessagingServiceV1MessageByAByBByCByDByEByF {
        a, b, c, d, e, f
    }
}


pub struct GetRiotMessagingServiceV1Session {

}

impl IsApiRequest for GetRiotMessagingServiceV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = RiotMessagingServiceSession;

    fn get_url(&self) -> String {
        "/riot-messaging-service/v1/session".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_session() -> GetRiotMessagingServiceV1Session {
    GetRiotMessagingServiceV1Session {
        
    }
}


pub struct GetRiotMessagingServiceV1State {

}

impl IsApiRequest for GetRiotMessagingServiceV1State {
    const METHOD: Method = Method::GET;
    type ReturnType = RiotMessagingServiceState;

    fn get_url(&self) -> String {
        "/riot-messaging-service/v1/state".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riot_messaging_service_v_1_state() -> GetRiotMessagingServiceV1State {
    GetRiotMessagingServiceV1State {
        
    }
}


pub struct PostRiotMessagingServiceV1Connect {

    pub body: String,
}

impl IsApiRequest for PostRiotMessagingServiceV1Connect {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riot-messaging-service/v1/connect".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riot_messaging_service_v_1_connect(body: String) -> PostRiotMessagingServiceV1Connect {
    PostRiotMessagingServiceV1Connect {
        body
    }
}


pub struct PostRiotMessagingServiceV1Entitlements {

    pub body: RiotMessagingServiceEntitlementsToken,
}

impl IsApiRequest for PostRiotMessagingServiceV1Entitlements {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riot-messaging-service/v1/entitlements".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riot_messaging_service_v_1_entitlements(body: RiotMessagingServiceEntitlementsToken) -> PostRiotMessagingServiceV1Entitlements {
    PostRiotMessagingServiceV1Entitlements {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServiceEntitlementsToken {
    pub access_token: String,
    pub token: String,
    pub subject: String,
    pub issuer: String,
    pub entitlements: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServiceSession {
    pub state: RiotMessagingServiceState,
    pub token: String,
    pub token_type: RiotMessagingServiceTokenType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RmsMessage {
    pub id: String,
    pub resource: String,
    pub service: String,
    pub version: String,
    pub timestamp: i64,
    pub payload: String,
    pub ack_required: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum RiotMessagingServiceState {
    #[default]
    Connected,
    Connecting,
    Disconnected,
    Disconnecting,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum RiotMessagingServiceTokenType {
    #[default]
    Identity,
    Access,
    Unavailable,
}

