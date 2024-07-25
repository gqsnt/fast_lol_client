
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetCrashReportingV1CrashStatus {
    /// Returns whether or not the game or client have crashed

}

impl IsApiRequest for GetCrashReportingV1CrashStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/crash-reporting/v1/crash-status".to_string()}
}

pub fn get_crash_reporting_v1_crash_status() -> GetCrashReportingV1CrashStatus {
    GetCrashReportingV1CrashStatus{}
}


pub struct GetSystemV1Builds {
    /// Information about the current artifacts that make up this build

}

impl IsApiRequest for GetSystemV1Builds {
    const METHOD: Method = Method::GET;
    type ReturnType = BuildInfo;
    fn get_url(&self) -> String {"/system/v1/builds".to_string()}
}

pub fn get_system_v1_builds() -> GetSystemV1Builds {
    GetSystemV1Builds{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub branch: String,
    pub patchline: String,
    pub version: String,
    #[serde(rename = "patchlineVisibleName")]
    pub patchline_visible_name: String,
}


// ENUMS

