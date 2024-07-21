
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolKrShutdownLawV1CustomStatus {

}

impl IsApiRequest for GetLolKrShutdownLawV1CustomStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolKrShutdownLawQueueShutdownStatus;

    fn get_url(&self) -> String {
        "/lol-kr-shutdown-law/v1/custom-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_kr_shutdown_law_v_1_custom_status() -> GetLolKrShutdownLawV1CustomStatus {
    GetLolKrShutdownLawV1CustomStatus {
        
    }
}


pub struct GetLolKrShutdownLawV1DisabledQueues {

}

impl IsApiRequest for GetLolKrShutdownLawV1DisabledQueues {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;

    fn get_url(&self) -> String {
        "/lol-kr-shutdown-law/v1/disabled-queues".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_kr_shutdown_law_v_1_disabled_queues() -> GetLolKrShutdownLawV1DisabledQueues {
    GetLolKrShutdownLawV1DisabledQueues {
        
    }
}


pub struct GetLolKrShutdownLawV1IsEnabled {

}

impl IsApiRequest for GetLolKrShutdownLawV1IsEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-kr-shutdown-law/v1/is-enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_kr_shutdown_law_v_1_is_enabled() -> GetLolKrShutdownLawV1IsEnabled {
    GetLolKrShutdownLawV1IsEnabled {
        
    }
}


pub struct GetLolKrShutdownLawV1Notification {

}

impl IsApiRequest for GetLolKrShutdownLawV1Notification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolKrShutdownLawShutdownLawNotification;

    fn get_url(&self) -> String {
        "/lol-kr-shutdown-law/v1/notification".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_kr_shutdown_law_v_1_notification() -> GetLolKrShutdownLawV1Notification {
    GetLolKrShutdownLawV1Notification {
        
    }
}


pub struct GetLolKrShutdownLawV1QueueStatusByQueueId {

    pub queue_id: i32,
}

impl IsApiRequest for GetLolKrShutdownLawV1QueueStatusByQueueId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolKrShutdownLawQueueShutdownStatus;

    fn get_url(&self) -> String {
        format!("/lol-kr-shutdown-law/v1/queue-status/{}", self.queue_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_kr_shutdown_law_v_1_queue_status_by_queue_id(queue_id: i32) -> GetLolKrShutdownLawV1QueueStatusByQueueId {
    GetLolKrShutdownLawV1QueueStatusByQueueId {
        queue_id
    }
}


pub struct GetLolKrShutdownLawV1RatingScreen {

}

impl IsApiRequest for GetLolKrShutdownLawV1RatingScreen {
    const METHOD: Method = Method::GET;
    type ReturnType = LolKrShutdownLawRatingScreenInfo;

    fn get_url(&self) -> String {
        "/lol-kr-shutdown-law/v1/rating-screen".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_kr_shutdown_law_v_1_rating_screen() -> GetLolKrShutdownLawV1RatingScreen {
    GetLolKrShutdownLawV1RatingScreen {
        
    }
}


pub struct GetLolKrShutdownLawV1Status {

}

impl IsApiRequest for GetLolKrShutdownLawV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = LolKrShutdownLawAllQueueShutdownStatus;

    fn get_url(&self) -> String {
        "/lol-kr-shutdown-law/v1/status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_kr_shutdown_law_v_1_status() -> GetLolKrShutdownLawV1Status {
    GetLolKrShutdownLawV1Status {
        
    }
}


pub struct PostLolKrShutdownLawV1RatingScreenAcknowledge {

}

impl IsApiRequest for PostLolKrShutdownLawV1RatingScreenAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-kr-shutdown-law/v1/rating-screen/acknowledge".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_kr_shutdown_law_v_1_rating_screen_acknowledge() -> PostLolKrShutdownLawV1RatingScreenAcknowledge {
    PostLolKrShutdownLawV1RatingScreenAcknowledge {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawAllQueueShutdownStatus {
    pub is_all_queues_disabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawQueueShutdownStatus {
    pub is_disabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawRatingScreenInfo {
    pub shown: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawShutdownLawNotification {
    pub type_: LolKrShutdownLawShutdownLawStatus,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolKrShutdownLawShutdownLawStatus {
    #[default]
    #[serde(rename = "CUT_OFF")]
    CutOff,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "NONE")]
    None,
}

