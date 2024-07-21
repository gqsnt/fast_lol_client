
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolPreEndOfGameV1RegistrationBySequenceEventName {
    pub sequence_event_name: String,
}

impl IsApiRequest for DeleteLolPreEndOfGameV1RegistrationBySequenceEventName {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-pre-end-of-game/v1/registration/{}", self.sequence_event_name)}
}

pub fn delete_lol_pre_end_of_game_v_1_registration_by_sequence_event_name(sequence_event_name: String) -> DeleteLolPreEndOfGameV1RegistrationBySequenceEventName {
    DeleteLolPreEndOfGameV1RegistrationBySequenceEventName{sequence_event_name}
}


pub struct GetLolPreEndOfGameV1CurrentSequenceEvent {}

impl IsApiRequest for GetLolPreEndOfGameV1CurrentSequenceEvent {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPreEndOfGameSequenceEvent;
    fn get_url(&self) -> String {"/lol-pre-end-of-game/v1/currentSequenceEvent".to_string()}
}

pub fn get_lol_pre_end_of_game_v_1_current_sequence_event() -> GetLolPreEndOfGameV1CurrentSequenceEvent {
    GetLolPreEndOfGameV1CurrentSequenceEvent{}
}


pub struct PostLolPreEndOfGameV1CompleteBySequenceEventName {
    pub sequence_event_name: String,
}

impl IsApiRequest for PostLolPreEndOfGameV1CompleteBySequenceEventName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-pre-end-of-game/v1/complete/{}", self.sequence_event_name)}
}

pub fn post_lol_pre_end_of_game_v_1_complete_by_sequence_event_name(sequence_event_name: String) -> PostLolPreEndOfGameV1CompleteBySequenceEventName {
    PostLolPreEndOfGameV1CompleteBySequenceEventName{sequence_event_name}
}


pub struct PostLolPreEndOfGameV1RegistrationBySequenceEventNameByPriority {
    pub sequence_event_name: String,
    pub priority: i32,
}

impl IsApiRequest for PostLolPreEndOfGameV1RegistrationBySequenceEventNameByPriority {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-pre-end-of-game/v1/registration/{}/{}", self.sequence_event_name, self.priority)}
}

pub fn post_lol_pre_end_of_game_v_1_registration_by_sequence_event_name_by_priority(sequence_event_name: String, priority: i32) -> PostLolPreEndOfGameV1RegistrationBySequenceEventNameByPriority {
    PostLolPreEndOfGameV1RegistrationBySequenceEventNameByPriority{sequence_event_name, priority}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPreEndOfGameSequenceEvent {
    pub name: String,
    pub priority: i32,
}


// ENUMS

