
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolLeaverBusterV1NotificationsById {
    pub id: u32,
}

impl IsApiRequest for DeleteLolLeaverBusterV1NotificationsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-leaver-buster/v1/notifications/{}", self.id)}
}

pub fn delete_lol_leaver_buster_v1_notifications_by_id(id: u32) -> DeleteLolLeaverBusterV1NotificationsById {
    DeleteLolLeaverBusterV1NotificationsById{id}
}


pub struct GetLolLeaverBusterV1Notifications {}

impl IsApiRequest for GetLolLeaverBusterV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLeaverBusterLeaverBusterNotificationResource>;
    fn get_url(&self) -> String {"/lol-leaver-buster/v1/notifications".to_string()}
}

pub fn get_lol_leaver_buster_v1_notifications() -> GetLolLeaverBusterV1Notifications {
    GetLolLeaverBusterV1Notifications{}
}


pub struct GetLolLeaverBusterV1NotificationsById {
    pub id: u32,
}

impl IsApiRequest for GetLolLeaverBusterV1NotificationsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLeaverBusterLeaverBusterNotificationResource;
    fn get_url(&self) -> String {format!("/lol-leaver-buster/v1/notifications/{}", self.id)}
}

pub fn get_lol_leaver_buster_v1_notifications_by_id(id: u32) -> GetLolLeaverBusterV1NotificationsById {
    GetLolLeaverBusterV1NotificationsById{id}
}


pub struct GetLolLeaverBusterV1RankedRestriction {}

impl IsApiRequest for GetLolLeaverBusterV1RankedRestriction {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLeaverBusterRankedRestrictionInfo;
    fn get_url(&self) -> String {"/lol-leaver-buster/v1/ranked-restriction".to_string()}
}

pub fn get_lol_leaver_buster_v1_ranked_restriction() -> GetLolLeaverBusterV1RankedRestriction {
    GetLolLeaverBusterV1RankedRestriction{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLeaverBusterNotificationResource {
    pub id: u32,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "type")]
    pub type_: LolLeaverBusterLeaverBusterNotificationType,
    #[serde(rename = "punishedGamesRemaining")]
    pub punished_games_remaining: i32,
    #[serde(rename = "queueLockoutTimerExpiryUtcMillisDiff")]
    pub queue_lockout_timer_expiry_utc_millis_diff: u64,
    #[serde(rename = "isWinRequired")]
    pub is_win_required: bool,
    #[serde(rename = "fromRms")]
    pub from_rms: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterRankedRestrictionInfo {
    #[serde(rename = "punishedGamesRemaining")]
    pub punished_games_remaining: i32,
    #[serde(rename = "isWinRequired")]
    pub is_win_required: bool,
    #[serde(rename = "needsAck")]
    pub needs_ack: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLeaverBusterLeaverBusterNotificationType {
    #[default]
    RankedRestrictedGames,
    OnLockoutWarning,
    PreLockoutWarning,
    Reforming,
    PunishedGamesRemaining,
    PunishmentIncurred,
    TaintedWarning,
    Invalid,
}

