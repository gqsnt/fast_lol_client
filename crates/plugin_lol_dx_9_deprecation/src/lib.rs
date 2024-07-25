
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetDx9DeprecationNotificationType {}

impl IsApiRequest for GetDx9DeprecationNotificationType {
    const METHOD: Method = Method::GET;
    type ReturnType = LolDx9DeprecationDx9DeprecationNotificationType;
    fn get_url(&self) -> String {"/dx9-deprecation/notification-type".to_string()}
}

pub fn get_dx_9_deprecation_notification_type() -> GetDx9DeprecationNotificationType {
    GetDx9DeprecationNotificationType{}
}


pub struct GetLolDx9DeprecationNeedsHardwareUpgrade {}

impl IsApiRequest for GetLolDx9DeprecationNeedsHardwareUpgrade {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-dx9-deprecation/needs-hardware-upgrade".to_string()}
}

pub fn get_lol_dx_9_deprecation_needs_hardware_upgrade() -> GetLolDx9DeprecationNeedsHardwareUpgrade {
    GetLolDx9DeprecationNeedsHardwareUpgrade{}
}


pub struct PostDx9DeprecationLegacyModeNotificationAck {}

impl IsApiRequest for PostDx9DeprecationLegacyModeNotificationAck {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/dx9-deprecation/legacy-mode-notification/ack".to_string()}
}

pub fn post_dx_9_deprecation_legacy_mode_notification_ack() -> PostDx9DeprecationLegacyModeNotificationAck {
    PostDx9DeprecationLegacyModeNotificationAck{}
}


// OBJECTS


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolDx9DeprecationDx9DeprecationNotificationType {
    #[default]
    #[serde(rename = "TURN_OFF_DX9_LEGACY_MODE")]
    TurnOffDx9LegacyMode,
    #[serde(rename = "HARDWARE_UPGRADE")]
    HardwareUpgrade,
    #[serde(rename = "NONE")]
    None,
}

