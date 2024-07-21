
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolRsoAuthV1Authorization {

}

impl IsApiRequest for GetLolRsoAuthV1Authorization {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRsoAuthAuthorization;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_v_1_authorization() -> GetLolRsoAuthV1Authorization {
    GetLolRsoAuthV1Authorization {
        
    }
}


pub struct DeleteLolRsoAuthV1Authorization {

}

impl IsApiRequest for DeleteLolRsoAuthV1Authorization {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_rso_auth_v_1_authorization() -> DeleteLolRsoAuthV1Authorization {
    DeleteLolRsoAuthV1Authorization {
        
    }
}


pub struct DeleteLolRsoAuthV1Session {

}

impl IsApiRequest for DeleteLolRsoAuthV1Session {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/session".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_rso_auth_v_1_session() -> DeleteLolRsoAuthV1Session {
    DeleteLolRsoAuthV1Session {
        
    }
}


pub struct PostLolRsoAuthV2Config {

    pub body: LolRsoAuthPublicClientConfig,
}

impl IsApiRequest for PostLolRsoAuthV2Config {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v2/config".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rso_auth_v_2_config(body: LolRsoAuthPublicClientConfig) -> PostLolRsoAuthV2Config {
    PostLolRsoAuthV2Config {
        body
    }
}


pub struct DeleteLolRsoAuthV2Config {

}

impl IsApiRequest for DeleteLolRsoAuthV2Config {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v2/config".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_rso_auth_v_2_config() -> DeleteLolRsoAuthV2Config {
    DeleteLolRsoAuthV2Config {
        
    }
}


pub struct GetLolRsoAuthConfigurationV3ReadyState {

}

impl IsApiRequest for GetLolRsoAuthConfigurationV3ReadyState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRsoAuthRsoConfigReadyState;

    fn get_url(&self) -> String {
        "/lol-rso-auth/configuration/v3/ready-state".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_configuration_v_3_ready_state() -> GetLolRsoAuthConfigurationV3ReadyState {
    GetLolRsoAuthConfigurationV3ReadyState {
        
    }
}


pub struct GetLolRsoAuthV1AuthorizationAccessToken {

}

impl IsApiRequest for GetLolRsoAuthV1AuthorizationAccessToken {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRsoAuthAccessToken;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/access-token".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_v_1_authorization_access_token() -> GetLolRsoAuthV1AuthorizationAccessToken {
    GetLolRsoAuthV1AuthorizationAccessToken {
        
    }
}


pub struct GetLolRsoAuthV1AuthorizationCountry {

}

impl IsApiRequest for GetLolRsoAuthV1AuthorizationCountry {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/country".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_v_1_authorization_country() -> GetLolRsoAuthV1AuthorizationCountry {
    GetLolRsoAuthV1AuthorizationCountry {
        
    }
}


pub struct GetLolRsoAuthV1AuthorizationError {

}

impl IsApiRequest for GetLolRsoAuthV1AuthorizationError {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRsoAuthAuthError;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/error".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_v_1_authorization_error() -> GetLolRsoAuthV1AuthorizationError {
    GetLolRsoAuthV1AuthorizationError {
        
    }
}


pub struct GetLolRsoAuthV1AuthorizationIdToken {

}

impl IsApiRequest for GetLolRsoAuthV1AuthorizationIdToken {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRsoAuthIdToken;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/id-token".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_v_1_authorization_id_token() -> GetLolRsoAuthV1AuthorizationIdToken {
    GetLolRsoAuthV1AuthorizationIdToken {
        
    }
}


pub struct GetLolRsoAuthV1AuthorizationUserinfo {

}

impl IsApiRequest for GetLolRsoAuthV1AuthorizationUserinfo {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRsoAuthUserInfo;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/userinfo".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_v_1_authorization_userinfo() -> GetLolRsoAuthV1AuthorizationUserinfo {
    GetLolRsoAuthV1AuthorizationUserinfo {
        
    }
}


pub struct PostLolRsoAuthV1AuthorizationUserinfo {

}

impl IsApiRequest for PostLolRsoAuthV1AuthorizationUserinfo {
    const METHOD: Method = Method::POST;
    type ReturnType = LolRsoAuthUserInfo;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/userinfo".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rso_auth_v_1_authorization_userinfo() -> PostLolRsoAuthV1AuthorizationUserinfo {
    PostLolRsoAuthV1AuthorizationUserinfo {
        
    }
}


pub struct GetLolRsoAuthV1StatusByPlatformId {

    pub platform_id: String,
}

impl IsApiRequest for GetLolRsoAuthV1StatusByPlatformId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRsoAuthRegionStatus;

    fn get_url(&self) -> String {
        format!("/lol-rso-auth/v1/status/{}", self.platform_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_rso_auth_v_1_status_by_platform_id(platform_id: String) -> GetLolRsoAuthV1StatusByPlatformId {
    GetLolRsoAuthV1StatusByPlatformId {
        platform_id
    }
}


pub struct PostLolRsoAuthV1AuthorizationGas {

    pub body: LolRsoAuthRsoPlayerCredentials,
}

impl IsApiRequest for PostLolRsoAuthV1AuthorizationGas {
    const METHOD: Method = Method::POST;
    type ReturnType = LolRsoAuthAuthorization;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/gas".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rso_auth_v_1_authorization_gas(body: LolRsoAuthRsoPlayerCredentials) -> PostLolRsoAuthV1AuthorizationGas {
    PostLolRsoAuthV1AuthorizationGas {
        body
    }
}


pub struct PostLolRsoAuthV1AuthorizationRefresh {

}

impl IsApiRequest for PostLolRsoAuthV1AuthorizationRefresh {
    const METHOD: Method = Method::POST;
    type ReturnType = LolRsoAuthAuthorization;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/authorization/refresh".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rso_auth_v_1_authorization_refresh() -> PostLolRsoAuthV1AuthorizationRefresh {
    PostLolRsoAuthV1AuthorizationRefresh {
        
    }
}


pub struct PostLolRsoAuthV1DeviceId {

}

impl IsApiRequest for PostLolRsoAuthV1DeviceId {
    const METHOD: Method = Method::POST;
    type ReturnType = LolRsoAuthDeviceId;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/device-id".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rso_auth_v_1_device_id() -> PostLolRsoAuthV1DeviceId {
    PostLolRsoAuthV1DeviceId {
        
    }
}


pub struct PostLolRsoAuthV1ExternalSessionConfig {

    pub body: HashMap<String, String>,
}

impl IsApiRequest for PostLolRsoAuthV1ExternalSessionConfig {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-rso-auth/v1/external-session-config".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rso_auth_v_1_external_session_config(body: HashMap<String, String>) -> PostLolRsoAuthV1ExternalSessionConfig {
    PostLolRsoAuthV1ExternalSessionConfig {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthPublicClientConfig {
    pub url: String,
    pub client_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRsoConfigReadyState {
    pub ready: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthDeviceId {
    pub collector_server_name: String,
    pub merchant_id: String,
    pub session_id: String,
    pub install_id: String,
    pub frame_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthIdToken {
    pub token: String,
    pub expiry: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthError {
    pub error: String,
    pub error_description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthorization {
    pub current_platform_id: String,
    pub current_account_id: u64,
    pub subject: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRegionStatus {
    pub platform_id: String,
    pub enabled: bool,
    pub is_lq_fallback_allowed: bool,
    pub is_user_info_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthUserInfo {
    pub user_info: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRsoPlayerCredentials {
    pub username: String,
    pub password: String,
    pub platform_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAccessToken {
    pub token: String,
    pub scopes: Vec<String>,
    pub expiry: u64,
}


// ENUMS

