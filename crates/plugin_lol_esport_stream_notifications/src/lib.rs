
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolEsportStreamNotificationsV1LiveStreams {

}

impl IsApiRequest for GetLolEsportStreamNotificationsV1LiveStreams {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEsportStreamNotificationsESportsLiveStreams;

    fn get_url(&self) -> String {
        "/lol-esport-stream-notifications/v1/live-streams".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_esport_stream_notifications_v_1_live_streams() -> GetLolEsportStreamNotificationsV1LiveStreams {
    GetLolEsportStreamNotificationsV1LiveStreams {
        
    }
}


pub struct GetLolEsportStreamNotificationsV1StreamUrl {

}

impl IsApiRequest for GetLolEsportStreamNotificationsV1StreamUrl {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-esport-stream-notifications/v1/stream-url".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_esport_stream_notifications_v_1_stream_url() -> GetLolEsportStreamNotificationsV1StreamUrl {
    GetLolEsportStreamNotificationsV1StreamUrl {
        
    }
}


pub struct PostLolEsportStreamNotificationsV1SendStats {

    pub event_type: String,
    pub match_id: String,
}

impl IsApiRequest for PostLolEsportStreamNotificationsV1SendStats {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-esport-stream-notifications/v1/send-stats".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "eventType" : self.event_type,
            "matchId" : self.match_id,
        }))
    }
}

pub fn post_lol_esport_stream_notifications_v_1_send_stats(event_type: String, match_id: String) -> PostLolEsportStreamNotificationsV1SendStats {
    PostLolEsportStreamNotificationsV1SendStats {
        event_type, match_id
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsLiveStreams {
    pub live_streams: Vec<LolEsportStreamNotificationsESportsStreams>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsStreams {
    pub title: String,
    pub tournament_description: String,
    pub team_a_guid: String,
    pub team_a_id: i64,
    pub team_b_guid: String,
    pub team_b_id: i64,
    pub team_a_name: String,
    pub team_b_name: String,
    pub team_a_acronym: String,
    pub team_b_acronym: String,
    pub team_a_logo_url: String,
    pub team_b_logo_url: String,
}


// ENUMS

