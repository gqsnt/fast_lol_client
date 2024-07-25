
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeletePlayerNotificationsV1NotificationsById {
    pub id: u64,
}

impl IsApiRequest for DeletePlayerNotificationsV1NotificationsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/player-notifications/v1/notifications/{}", self.id)}
}

pub fn delete_player_notifications_v1_notifications_by_id(id: u64) -> DeletePlayerNotificationsV1NotificationsById {
    DeletePlayerNotificationsV1NotificationsById{id}
}


pub struct GetPlayerNotificationsV1Config {}

impl IsApiRequest for GetPlayerNotificationsV1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = PlayerNotificationsPlayerNotificationConfigResource;
    fn get_url(&self) -> String {"/player-notifications/v1/config".to_string()}
}

pub fn get_player_notifications_v1_config() -> GetPlayerNotificationsV1Config {
    GetPlayerNotificationsV1Config{}
}


pub struct GetPlayerNotificationsV1Notifications {}

impl IsApiRequest for GetPlayerNotificationsV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PlayerNotificationsPlayerNotificationResource>;
    fn get_url(&self) -> String {"/player-notifications/v1/notifications".to_string()}
}

pub fn get_player_notifications_v1_notifications() -> GetPlayerNotificationsV1Notifications {
    GetPlayerNotificationsV1Notifications{}
}


pub struct GetPlayerNotificationsV1NotificationsById {
    pub id: u64,
}

impl IsApiRequest for GetPlayerNotificationsV1NotificationsById {
    const METHOD: Method = Method::GET;
    type ReturnType = PlayerNotificationsPlayerNotificationResource;
    fn get_url(&self) -> String {format!("/player-notifications/v1/notifications/{}", self.id)}
}

pub fn get_player_notifications_v1_notifications_by_id(id: u64) -> GetPlayerNotificationsV1NotificationsById {
    GetPlayerNotificationsV1NotificationsById{id}
}


pub struct PostPlayerNotificationsV1Notifications {
    pub body: PlayerNotificationsPlayerNotificationResource,
}

impl IsApiRequest for PostPlayerNotificationsV1Notifications {
    const METHOD: Method = Method::POST;
    type ReturnType = PlayerNotificationsPlayerNotificationResource;
    fn get_url(&self) -> String {"/player-notifications/v1/notifications".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_player_notifications_v1_notifications(body: PlayerNotificationsPlayerNotificationResource) -> PostPlayerNotificationsV1Notifications {
    PostPlayerNotificationsV1Notifications{body}
}


pub struct PutPlayerNotificationsV1Config {
    pub body: PlayerNotificationsPlayerNotificationConfigResource,
}

impl IsApiRequest for PutPlayerNotificationsV1Config {
    const METHOD: Method = Method::PUT;
    type ReturnType = PlayerNotificationsPlayerNotificationConfigResource;
    fn get_url(&self) -> String {"/player-notifications/v1/config".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_player_notifications_v1_config(body: PlayerNotificationsPlayerNotificationConfigResource) -> PutPlayerNotificationsV1Config {
    PutPlayerNotificationsV1Config{body}
}


pub struct PutPlayerNotificationsV1NotificationsById {
    pub id: u64,
    pub body: Value,
}

impl IsApiRequest for PutPlayerNotificationsV1NotificationsById {
    const METHOD: Method = Method::PUT;
    type ReturnType = PlayerNotificationsPlayerNotificationResource;
    fn get_url(&self) -> String {format!("/player-notifications/v1/notifications/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_player_notifications_v1_notifications_by_id(id: u64, body: Value) -> PutPlayerNotificationsV1NotificationsById {
    PutPlayerNotificationsV1NotificationsById{id, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerNotificationsPlayerNotificationConfigResource {
    #[serde(rename = "ExpirationCheckFrequency")]
    pub expiration_check_frequency: Option<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerNotificationsPlayerNotificationResource {
    #[serde(rename = "backgroundUrl")]
    pub background_url: String,
    pub created: String,
    pub critical: bool,
    pub data: HashMap<String, String>,
    #[serde(rename = "detailKey")]
    pub detail_key: String,
    pub expires: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: u64,
    pub source: String,
    pub state: String,
    #[serde(rename = "titleKey")]
    pub title_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub dismissible: bool,
}


// ENUMS

