
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolAccountVerificationV1IsVerified {}

impl IsApiRequest for GetLolAccountVerificationV1IsVerified {
    const METHOD: Method = Method::GET;
    type ReturnType = LolAccountVerificationIsVerifiedResponse;
    fn get_url(&self) -> String {"/lol-account-verification/v1/is-verified".to_string()}
}

pub fn get_lol_account_verification_v_1_is_verified() -> GetLolAccountVerificationV1IsVerified {
    GetLolAccountVerificationV1IsVerified{}
}


pub struct GetLolAccountVerificationV1PhoneNumber {}

impl IsApiRequest for GetLolAccountVerificationV1PhoneNumber {
    const METHOD: Method = Method::GET;
    type ReturnType = LolAccountVerificationPhoneNumberResponse;
    fn get_url(&self) -> String {"/lol-account-verification/v1/phone-number".to_string()}
}

pub fn get_lol_account_verification_v_1_phone_number() -> GetLolAccountVerificationV1PhoneNumber {
    GetLolAccountVerificationV1PhoneNumber{}
}


pub struct PostLolAccountVerificationV1ConfirmActivationPin {
    pub body: LolAccountVerificationConfirmActivationPinRequest,
}

impl IsApiRequest for PostLolAccountVerificationV1ConfirmActivationPin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-account-verification/v1/confirmActivationPin".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_account_verification_v_1_confirm_activation_pin(body: LolAccountVerificationConfirmActivationPinRequest) -> PostLolAccountVerificationV1ConfirmActivationPin {
    PostLolAccountVerificationV1ConfirmActivationPin{body}
}


pub struct PostLolAccountVerificationV1ConfirmDeactivationPin {
    pub body: LolAccountVerificationConfirmDeactivationPinRequest,
}

impl IsApiRequest for PostLolAccountVerificationV1ConfirmDeactivationPin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-account-verification/v1/confirmDeactivationPin".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_account_verification_v_1_confirm_deactivation_pin(body: LolAccountVerificationConfirmDeactivationPinRequest) -> PostLolAccountVerificationV1ConfirmDeactivationPin {
    PostLolAccountVerificationV1ConfirmDeactivationPin{body}
}


pub struct PostLolAccountVerificationV1SendActivationPin {
    pub body: LolAccountVerificationSendActivationPinRequest,
}

impl IsApiRequest for PostLolAccountVerificationV1SendActivationPin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-account-verification/v1/sendActivationPin".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_account_verification_v_1_send_activation_pin(body: LolAccountVerificationSendActivationPinRequest) -> PostLolAccountVerificationV1SendActivationPin {
    PostLolAccountVerificationV1SendActivationPin{body}
}


pub struct PostLolAccountVerificationV1SendDeactivationPin {}

impl IsApiRequest for PostLolAccountVerificationV1SendDeactivationPin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-account-verification/v1/sendDeactivationPin".to_string()}
}

pub fn post_lol_account_verification_v_1_send_deactivation_pin() -> PostLolAccountVerificationV1SendDeactivationPin {
    PostLolAccountVerificationV1SendDeactivationPin{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmActivationPinRequest {
    pub one_time_pin: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmDeactivationPinRequest {
    pub one_time_pin: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationIsVerifiedResponse {
    pub success: bool,
    pub message: String,
    pub status: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberObfuscated {
    pub country_code: String,
    pub ends_with: String,
    pub length: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberResponse {
    pub data: LolAccountVerificationPhoneNumberResponseData,
    pub error: LolAccountVerificationResponseError,
    pub client_message_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberResponseData {
    pub phone_number_obfuscated: LolAccountVerificationPhoneNumberObfuscated,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationResponseError {
    pub error_code: String,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendActivationPinRequest {
    pub phone_number: String,
    pub locale: String,
}


// ENUMS

