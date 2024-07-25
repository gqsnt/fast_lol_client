
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolEsportStreamNotificationsV1LiveStreams {}

impl IsApiRequest for GetLolEsportStreamNotificationsV1LiveStreams {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEsportStreamNotificationsESportsLiveStreams;
    fn get_url(&self) -> String {"/lol-esport-stream-notifications/v1/live-streams".to_string()}
}

pub fn get_lol_esport_stream_notifications_v1_live_streams() -> GetLolEsportStreamNotificationsV1LiveStreams {
    GetLolEsportStreamNotificationsV1LiveStreams{}
}


pub struct GetLolEsportStreamNotificationsV1StreamUrl {}

impl IsApiRequest for GetLolEsportStreamNotificationsV1StreamUrl {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-esport-stream-notifications/v1/stream-url".to_string()}
}

pub fn get_lol_esport_stream_notifications_v1_stream_url() -> GetLolEsportStreamNotificationsV1StreamUrl {
    GetLolEsportStreamNotificationsV1StreamUrl{}
}


pub struct PostLolEsportStreamNotificationsV1SendStats {
    pub event_type: String,
    pub match_id: String,
}

impl IsApiRequest for PostLolEsportStreamNotificationsV1SendStats {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-esport-stream-notifications/v1/send-stats".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("eventType".to_string(), serde_json::to_string(&self.event_type).unwrap()),
            ("matchId".to_string(), serde_json::to_string(&self.match_id).unwrap())
        ])
    }
}

pub fn post_lol_esport_stream_notifications_v1_send_stats(event_type: String, match_id: String) -> PostLolEsportStreamNotificationsV1SendStats {
    PostLolEsportStreamNotificationsV1SendStats{event_type, match_id}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsLiveStreams {
    #[serde(rename = "liveStreams")]
    pub live_streams: Vec<LolEsportStreamNotificationsESportsStreams>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsStreams {
    pub title: String,
    #[serde(rename = "tournamentDescription")]
    pub tournament_description: String,
    #[serde(rename = "teamAGuid")]
    pub team_a_guid: String,
    #[serde(rename = "teamAId")]
    pub team_a_id: i64,
    #[serde(rename = "teamBGuid")]
    pub team_b_guid: String,
    #[serde(rename = "teamBId")]
    pub team_b_id: i64,
    #[serde(rename = "teamAName")]
    pub team_a_name: String,
    #[serde(rename = "teamBName")]
    pub team_b_name: String,
    #[serde(rename = "teamAAcronym")]
    pub team_a_acronym: String,
    #[serde(rename = "teamBAcronym")]
    pub team_b_acronym: String,
    #[serde(rename = "teamALogoUrl")]
    pub team_a_logo_url: String,
    #[serde(rename = "teamBLogoUrl")]
    pub team_b_logo_url: String,
}


// ENUMS

