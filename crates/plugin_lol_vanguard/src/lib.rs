
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolVanguardV1ConfigDaysToReshowModal {}

impl IsApiRequest for GetLolVanguardV1ConfigDaysToReshowModal {
    const METHOD: Method = Method::GET;
    type ReturnType = f32;
    fn get_url(&self) -> String {"/lol-vanguard/v1/config/days-to-reshow-modal".to_string()}
}

pub fn get_lol_vanguard_v1_config_days_to_reshow_modal() -> GetLolVanguardV1ConfigDaysToReshowModal {
    GetLolVanguardV1ConfigDaysToReshowModal{}
}


pub struct GetLolVanguardV1ConfigEnabled {}

impl IsApiRequest for GetLolVanguardV1ConfigEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-vanguard/v1/config/enabled".to_string()}
}

pub fn get_lol_vanguard_v1_config_enabled() -> GetLolVanguardV1ConfigEnabled {
    GetLolVanguardV1ConfigEnabled{}
}


pub struct GetLolVanguardV1IsPlayingInPcb {}

impl IsApiRequest for GetLolVanguardV1IsPlayingInPcb {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-vanguard/v1/is-playing-in-pcb".to_string()}
}

pub fn get_lol_vanguard_v1_is_playing_in_pcb() -> GetLolVanguardV1IsPlayingInPcb {
    GetLolVanguardV1IsPlayingInPcb{}
}


pub struct GetLolVanguardV1MachineSpecs {}

impl IsApiRequest for GetLolVanguardV1MachineSpecs {
    const METHOD: Method = Method::GET;
    type ReturnType = LolVanguardVanguardMachineSpecs;
    fn get_url(&self) -> String {"/lol-vanguard/v1/machine-specs".to_string()}
}

pub fn get_lol_vanguard_v1_machine_specs() -> GetLolVanguardV1MachineSpecs {
    GetLolVanguardV1MachineSpecs{}
}


pub struct GetLolVanguardV1Session {}

impl IsApiRequest for GetLolVanguardV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolVanguardVanguardSession;
    fn get_url(&self) -> String {"/lol-vanguard/v1/session".to_string()}
}

pub fn get_lol_vanguard_v1_session() -> GetLolVanguardV1Session {
    GetLolVanguardV1Session{}
}


pub struct PostLolVanguardV1TelemetrySystemCheck {
    pub body: LolVanguardVanguardSystemCheckTelemetryEvent,
}

impl IsApiRequest for PostLolVanguardV1TelemetrySystemCheck {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-vanguard/v1/telemetry/system-check".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_vanguard_v1_telemetry_system_check(body: LolVanguardVanguardSystemCheckTelemetryEvent) -> PostLolVanguardV1TelemetrySystemCheck {
    PostLolVanguardV1TelemetrySystemCheck{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolVanguardVanguardMachineSpecs {
    #[serde(rename = "tpm2Enabled")]
    pub tpm_2_enabled: bool,
    #[serde(rename = "secureBootEnabled")]
    pub secure_boot_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolVanguardVanguardSession {
    pub state: LolVanguardVanguardSessionState,
    #[serde(rename = "vanguardStatus")]
    pub vanguard_status: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolVanguardVanguardSystemCheckTelemetryEvent {
    #[serde(rename = "passedOsCheck")]
    pub passed_os_check: bool,
    #[serde(rename = "passedSecureFeaturesCheck")]
    pub passed_secure_features_check: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolVanguardVanguardSessionState {
    #[default]
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "CONNECTED")]
    Connected,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
}

