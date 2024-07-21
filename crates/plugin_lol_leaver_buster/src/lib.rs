
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolLeaverBusterV1NotificationsById {

    pub id: u32,
}

impl IsApiRequest for GetLolLeaverBusterV1NotificationsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLeaverBusterLeaverBusterNotificationResource;

    fn get_url(&self) -> String {
        format!("/lol-leaver-buster/v1/notifications/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_leaver_buster_v_1_notifications_by_id(id: u32) -> GetLolLeaverBusterV1NotificationsById {
    GetLolLeaverBusterV1NotificationsById {
        id
    }
}


pub struct DeleteLolLeaverBusterV1NotificationsById {

    pub id: u32,
}

impl IsApiRequest for DeleteLolLeaverBusterV1NotificationsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-leaver-buster/v1/notifications/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_leaver_buster_v_1_notifications_by_id(id: u32) -> DeleteLolLeaverBusterV1NotificationsById {
    DeleteLolLeaverBusterV1NotificationsById {
        id
    }
}


pub struct GetLolLeaverBusterV1Notifications {

}

impl IsApiRequest for GetLolLeaverBusterV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLeaverBusterLeaverBusterNotificationResource>;

    fn get_url(&self) -> String {
        "/lol-leaver-buster/v1/notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_leaver_buster_v_1_notifications() -> GetLolLeaverBusterV1Notifications {
    GetLolLeaverBusterV1Notifications {
        
    }
}


pub struct GetLolLeaverBusterV1RankedRestriction {

}

impl IsApiRequest for GetLolLeaverBusterV1RankedRestriction {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLeaverBusterRankedRestrictionInfo;

    fn get_url(&self) -> String {
        "/lol-leaver-buster/v1/ranked-restriction".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_leaver_buster_v_1_ranked_restriction() -> GetLolLeaverBusterV1RankedRestriction {
    GetLolLeaverBusterV1RankedRestriction {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLeaverBusterNotificationResource {
    pub id: u32,
    pub msg_id: String,
    pub account_id: u64,
    pub type_: LolLeaverBusterLeaverBusterNotificationType,
    pub punished_games_remaining: i32,
    pub queue_lockout_timer_expiry_utc_millis_diff: u64,
    pub from_rms: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterRankedRestrictionInfo {
    pub punished_games_remaining: i32,
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

