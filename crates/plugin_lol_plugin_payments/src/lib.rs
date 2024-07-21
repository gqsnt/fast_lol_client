
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct PostPaymentsV1PmcStartUrl {
    pub body: PaymentsFrontEndRequest,
}

impl IsApiRequest for PostPaymentsV1PmcStartUrl {
    const METHOD: Method = Method::POST;
    type ReturnType = PaymentsFrontEndResult;
    fn get_url(&self) -> String {"/payments/v1/pmc-start-url".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_payments_v_1_pmc_start_url(body: PaymentsFrontEndRequest) -> PostPaymentsV1PmcStartUrl {
    PostPaymentsV1PmcStartUrl{body}
}


pub struct PostPaymentsV1UpdatePaymentTelemetryState {
    pub body: PaymentsPaymentsTelemetryTransitions,
}

impl IsApiRequest for PostPaymentsV1UpdatePaymentTelemetryState {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/payments/v1/updatePaymentTelemetryState".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_payments_v_1_update_payment_telemetry_state(body: PaymentsPaymentsTelemetryTransitions) -> PostPaymentsV1UpdatePaymentTelemetryState {
    PostPaymentsV1UpdatePaymentTelemetryState{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsFrontEndRequest {
    pub is_prepaid: bool,
    pub locale_id: String,
    pub summoner_level: i16,
    pub giftee_account_id: String,
    pub giftee_message: String,
    pub rso_token: String,
    pub use_pmc_sessions: bool,
    pub game: String,
    pub opened_from: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsFrontEndResult {
    pub url: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PaymentsPaymentsTelemetryTransitions {
    #[default]
    #[serde(rename = "PMCCompleteToIdle")]
    PmcCompleteToIdle,
    #[serde(rename = "PMCClosedToIdle")]
    PmcClosedToIdle,
    #[serde(rename = "PMCOpenToPMCComplete")]
    PmcOpenToPmcComplete,
    #[serde(rename = "PMCOpenToPMCClose")]
    PmcOpenToPmcClose,
    #[serde(rename = "IdleToPMCOpen")]
    IdleToPmcOpen,
}

