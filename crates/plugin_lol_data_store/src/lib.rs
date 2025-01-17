
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetDataStoreV1InstallDir {
    /// Gets the current install directory (used internally.)

}

impl IsApiRequest for GetDataStoreV1InstallDir {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/data-store/v1/install-dir".to_string()}
}

pub fn get_data_store_v1_install_dir() -> GetDataStoreV1InstallDir {
    GetDataStoreV1InstallDir{}
}


pub struct GetDataStoreV1InstallSettingsByPath {
    /// Get the data for the specified key from the install settings.
    pub path_: String,
}

impl IsApiRequest for GetDataStoreV1InstallSettingsByPath {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/data-store/v1/install-settings/{}", self.path_)}
}

pub fn get_data_store_v1_install_settings_by_path(path_: String) -> GetDataStoreV1InstallSettingsByPath {
    GetDataStoreV1InstallSettingsByPath{path_}
}


pub struct GetDataStoreV1SystemSettingsByPath {
    /// Get the setting for the specified key.
    pub path_: String,
}

impl IsApiRequest for GetDataStoreV1SystemSettingsByPath {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/data-store/v1/system-settings/{}", self.path_)}
}

pub fn get_data_store_v1_system_settings_by_path(path_: String) -> GetDataStoreV1SystemSettingsByPath {
    GetDataStoreV1SystemSettingsByPath{path_}
}


pub struct PostDataStoreV1InstallSettingsByPath {
    /// Set the data for the specified key from the install settings.
    pub path_: String,
    pub body: Value,
}

impl IsApiRequest for PostDataStoreV1InstallSettingsByPath {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/data-store/v1/install-settings/{}", self.path_)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_data_store_v1_install_settings_by_path(path_: String, body: Value) -> PostDataStoreV1InstallSettingsByPath {
    PostDataStoreV1InstallSettingsByPath{path_, body}
}


// OBJECTS


// ENUMS

