
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetClientConfigV1StatusByType {
    pub type_: ClientConfigConfigType,
}

impl IsApiRequest for GetClientConfigV1StatusByType {
    const METHOD: Method = Method::GET;
    type ReturnType = ClientConfigConfigStatus;
    fn get_url(&self) -> String {format!("/client-config/v1/status/{}", serde_json::to_string(&self.type_).unwrap())}
}

pub fn get_client_config_v1_status_by_type(type_: ClientConfigConfigType) -> GetClientConfigV1StatusByType {
    GetClientConfigV1StatusByType{type_}
}


pub struct GetClientConfigV2ConfigByName {
    pub name: String,
}

impl IsApiRequest for GetClientConfigV2ConfigByName {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/client-config/v2/config/{}", self.name)}
}

pub fn get_client_config_v2_config_by_name(name: String) -> GetClientConfigV2ConfigByName {
    GetClientConfigV2ConfigByName{name}
}


pub struct GetClientConfigV2NamespaceByNamespace {
    pub namespace: String,
}

impl IsApiRequest for GetClientConfigV2NamespaceByNamespace {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, Value>;
    fn get_url(&self) -> String {format!("/client-config/v2/namespace/{}", self.namespace)}
}

pub fn get_client_config_v2_namespace_by_namespace(namespace: String) -> GetClientConfigV2NamespaceByNamespace {
    GetClientConfigV2NamespaceByNamespace{namespace}
}


pub struct GetClientConfigV2NamespaceByNamespacePlayer {
    pub namespace: String,
}

impl IsApiRequest for GetClientConfigV2NamespaceByNamespacePlayer {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, Value>;
    fn get_url(&self) -> String {format!("/client-config/v2/namespace/{}/player", self.namespace)}
}

pub fn get_client_config_v2_namespace_by_namespace_player(namespace: String) -> GetClientConfigV2NamespaceByNamespacePlayer {
    GetClientConfigV2NamespaceByNamespacePlayer{namespace}
}


pub struct GetClientConfigV2NamespaceByNamespacePublic {
    pub namespace: String,
}

impl IsApiRequest for GetClientConfigV2NamespaceByNamespacePublic {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, Value>;
    fn get_url(&self) -> String {format!("/client-config/v2/namespace/{}/public", self.namespace)}
}

pub fn get_client_config_v2_namespace_by_namespace_public(namespace: String) -> GetClientConfigV2NamespaceByNamespacePublic {
    GetClientConfigV2NamespaceByNamespacePublic{namespace}
}


pub struct PutClientConfigV1EntitlementsToken {
    pub body: ClientConfigEntitlementsUpdate,
}

impl IsApiRequest for PutClientConfigV1EntitlementsToken {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/client-config/v1/entitlements-token".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_client_config_v1_entitlements_token(body: ClientConfigEntitlementsUpdate) -> PutClientConfigV1EntitlementsToken {
    PutClientConfigV1EntitlementsToken{body}
}


pub struct PutClientConfigV1RefreshConfigStatus {}

impl IsApiRequest for PutClientConfigV1RefreshConfigStatus {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/client-config/v1/refresh-config-status".to_string()}
}

pub fn put_client_config_v1_refresh_config_status() -> PutClientConfigV1RefreshConfigStatus {
    PutClientConfigV1RefreshConfigStatus{}
}


pub struct PutClientConfigV2NamespaceChanges {
    pub body: ClientConfigConfigNamespaceUpdate,
}

impl IsApiRequest for PutClientConfigV2NamespaceChanges {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/client-config/v2/namespace-changes".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_client_config_v2_namespace_changes(body: ClientConfigConfigNamespaceUpdate) -> PutClientConfigV2NamespaceChanges {
    PutClientConfigV2NamespaceChanges{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigConfigNamespaceUpdate {
    pub public: Vec<String>,
    pub player: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigConfigStatus {
    pub readiness: ClientConfigConfigReadinessEnum,
    #[serde(rename = "updateId")]
    pub update_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigEntitlements {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub token: String,
    pub subject: String,
    pub issuer: String,
    pub entitlements: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigEntitlementsUpdate {
    #[serde(rename = "UpdateType")]
    pub update_type: ClientConfigUpdateType,
    #[serde(rename = "EntitlementsTokenResource")]
    pub entitlements_token_resource: ClientConfigEntitlements,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ClientConfigConfigReadinessEnum {
    #[default]
    Disabled,
    Ready,
    NotReady,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ClientConfigConfigType {
    #[default]
    #[serde(rename = "player")]
    Player,
    #[serde(rename = "public")]
    Public,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ClientConfigUpdateType {
    #[default]
    Delete,
    Update,
    Create,
}

