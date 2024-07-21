
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolHoneyfruitV1AccountClaimMigration {

}

impl IsApiRequest for DeleteLolHoneyfruitV1AccountClaimMigration {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/account-claim/migration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_honeyfruit_v_1_account_claim_migration() -> DeleteLolHoneyfruitV1AccountClaimMigration {
    DeleteLolHoneyfruitV1AccountClaimMigration {
        
    }
}


pub struct GetLolHoneyfruitV1AccountClaimAccountStatusByPuuid {

    pub puuid: String,
}

impl IsApiRequest for GetLolHoneyfruitV1AccountClaimAccountStatusByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHoneyfruitAccountClaimStatus;

    fn get_url(&self) -> String {
        format!("/lol-honeyfruit/v1/account-claim/account-status/{}", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_honeyfruit_v_1_account_claim_account_status_by_puuid(puuid: String) -> GetLolHoneyfruitV1AccountClaimAccountStatusByPuuid {
    GetLolHoneyfruitV1AccountClaimAccountStatusByPuuid {
        puuid
    }
}


pub struct GetLolHoneyfruitV1AccountClaimAutoDismiss {

}

impl IsApiRequest for GetLolHoneyfruitV1AccountClaimAutoDismiss {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/account-claim/auto-dismiss".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_honeyfruit_v_1_account_claim_auto_dismiss() -> GetLolHoneyfruitV1AccountClaimAutoDismiss {
    GetLolHoneyfruitV1AccountClaimAutoDismiss {
        
    }
}


pub struct GetLolHoneyfruitV1AccountClaimMigration {

}

impl IsApiRequest for GetLolHoneyfruitV1AccountClaimMigration {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/account-claim/migration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_honeyfruit_v_1_account_claim_migration() -> GetLolHoneyfruitV1AccountClaimMigration {
    GetLolHoneyfruitV1AccountClaimMigration {
        
    }
}


pub struct GetLolHoneyfruitV1LinkingSettingsButtonAvailable {

}

impl IsApiRequest for GetLolHoneyfruitV1LinkingSettingsButtonAvailable {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/linking-settings-button-available".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_honeyfruit_v_1_linking_settings_button_available() -> GetLolHoneyfruitV1LinkingSettingsButtonAvailable {
    GetLolHoneyfruitV1LinkingSettingsButtonAvailable {
        
    }
}


pub struct GetLolHoneyfruitV1VngPublisherSettings {

}

impl IsApiRequest for GetLolHoneyfruitV1VngPublisherSettings {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHoneyfruitHoneyfruitVngPublisherSettings;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/vng-publisher-settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_honeyfruit_v_1_vng_publisher_settings() -> GetLolHoneyfruitV1VngPublisherSettings {
    GetLolHoneyfruitV1VngPublisherSettings {
        
    }
}


pub struct PostLolHoneyfruitV1AccountClaimLinkingRedirect {

}

impl IsApiRequest for PostLolHoneyfruitV1AccountClaimLinkingRedirect {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/account-claim/linking-redirect".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_honeyfruit_v_1_account_claim_linking_redirect() -> PostLolHoneyfruitV1AccountClaimLinkingRedirect {
    PostLolHoneyfruitV1AccountClaimLinkingRedirect {
        
    }
}


pub struct PostLolHoneyfruitV1AccountClaimMigration {

}

impl IsApiRequest for PostLolHoneyfruitV1AccountClaimMigration {
    const METHOD: Method = Method::POST;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/account-claim/migration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_honeyfruit_v_1_account_claim_migration() -> PostLolHoneyfruitV1AccountClaimMigration {
    PostLolHoneyfruitV1AccountClaimMigration {
        
    }
}


pub struct PostLolHoneyfruitV1VngPublisherSettings {

}

impl IsApiRequest for PostLolHoneyfruitV1VngPublisherSettings {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/vng-publisher-settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_honeyfruit_v_1_vng_publisher_settings() -> PostLolHoneyfruitV1VngPublisherSettings {
    PostLolHoneyfruitV1VngPublisherSettings {
        
    }
}


pub struct PutLolHoneyfruitV1AccountClaimAutoDismiss {

    pub body: bool,
}

impl IsApiRequest for PutLolHoneyfruitV1AccountClaimAutoDismiss {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-honeyfruit/v1/account-claim/auto-dismiss".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_honeyfruit_v_1_account_claim_auto_dismiss(body: bool) -> PutLolHoneyfruitV1AccountClaimAutoDismiss {
    PutLolHoneyfruitV1AccountClaimAutoDismiss {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitAccountClaimStatus {
    pub linking_status: Option<LolHoneyfruitHoneyfruitLinkingServiceResponse>,
    pub migration_status: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitGarenaRegionLeagueAccount {
    pub garena_puuid: String,
    pub platform_id: String,
    pub summoner_name: String,
    pub summoner_level: u32,
    pub summoner_icon_id: i32,
    pub garena_id: u64,
    pub is_reserved_summoner_name: bool,
    pub has_played_a_game: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitLinkingServiceResponse {
    pub eligible: bool,
    pub reason_code: LolHoneyfruitHoneyfruitLinkingFailureReason,
    pub email: String,
    pub account_details: Option<LolHoneyfruitGarenaRegionLeagueAccount>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitVngPublisherSettings {
    pub visible: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolHoneyfruitHoneyfruitLinkingFailureReason {
    #[default]
    #[serde(rename = "UNHANDLED_SERVER_SIDE_ERROR")]
    UnhandledServerSideError,
    #[serde(rename = "REQUEST_FAILURE")]
    RequestFailure,
    #[serde(rename = "NOT_LINKED")]
    NotLinked,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "DEGRADED")]
    Degraded,
    #[serde(rename = "BAD_AUTHORIZATION_PARAM")]
    BadAuthorizationParam,
    #[serde(rename = "ACCESS_TOKEN_EXPIRED")]
    AccessTokenExpired,
    #[serde(rename = "ALREADY_LINKED")]
    AlreadyLinked,
}

