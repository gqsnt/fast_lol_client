
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolPlayerMessagingV1CelebrationNotificationByIdAcknowledge {
    pub id: u32,
}

impl IsApiRequest for DeleteLolPlayerMessagingV1CelebrationNotificationByIdAcknowledge {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-player-messaging/v1/celebration/notification/{}/acknowledge", self.id)}
}

pub fn delete_lol_player_messaging_v_1_celebration_notification_by_id_acknowledge(id: u32) -> DeleteLolPlayerMessagingV1CelebrationNotificationByIdAcknowledge {
    DeleteLolPlayerMessagingV1CelebrationNotificationByIdAcknowledge{id}
}


pub struct DeleteLolPlayerMessagingV1NotificationByIdAcknowledge {
    pub id: u32,
}

impl IsApiRequest for DeleteLolPlayerMessagingV1NotificationByIdAcknowledge {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-player-messaging/v1/notification/{}/acknowledge", self.id)}
}

pub fn delete_lol_player_messaging_v_1_notification_by_id_acknowledge(id: u32) -> DeleteLolPlayerMessagingV1NotificationByIdAcknowledge {
    DeleteLolPlayerMessagingV1NotificationByIdAcknowledge{id}
}


pub struct GetLolPlayerMessagingV1CelebrationNotification {}

impl IsApiRequest for GetLolPlayerMessagingV1CelebrationNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerMessagingDynamicCelebrationMessagingNotificationResource;
    fn get_url(&self) -> String {"/lol-player-messaging/v1/celebration/notification".to_string()}
}

pub fn get_lol_player_messaging_v_1_celebration_notification() -> GetLolPlayerMessagingV1CelebrationNotification {
    GetLolPlayerMessagingV1CelebrationNotification{}
}


pub struct GetLolPlayerMessagingV1Notification {}

impl IsApiRequest for GetLolPlayerMessagingV1Notification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerMessagingPlayerMessagingNotificationResource;
    fn get_url(&self) -> String {"/lol-player-messaging/v1/notification".to_string()}
}

pub fn get_lol_player_messaging_v_1_notification() -> GetLolPlayerMessagingV1Notification {
    GetLolPlayerMessagingV1Notification{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingDynamicCelebrationMessagingNotificationResource {
    pub id: i32,
    pub account_id: u64,
    pub msg_id: String,
    pub celebration_title: String,
    pub celebration_body: String,
    pub celebration_message: String,
    pub inventory_type: String,
    pub item_id: String,
    pub item_quantity: String,
    pub celebration_type: String,
    pub status: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingPlayerMessagingNotificationResource {
    pub id: i32,
    pub account_id: u64,
    pub msg_id: String,
    pub title: String,
    pub body: String,
    pub status: i32,
}


// ENUMS

