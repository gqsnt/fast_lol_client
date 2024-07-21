
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolMatchmakingV1Search {}

impl IsApiRequest for DeleteLolMatchmakingV1Search {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/search".to_string()}
}

pub fn delete_lol_matchmaking_v_1_search() -> DeleteLolMatchmakingV1Search {
    DeleteLolMatchmakingV1Search{}
}


pub struct GetLolMatchmakingV1ReadyCheck {}

impl IsApiRequest for GetLolMatchmakingV1ReadyCheck {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchmakingMatchmakingReadyCheckResource;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/ready-check".to_string()}
}

pub fn get_lol_matchmaking_v_1_ready_check() -> GetLolMatchmakingV1ReadyCheck {
    GetLolMatchmakingV1ReadyCheck{}
}


pub struct GetLolMatchmakingV1Search {}

impl IsApiRequest for GetLolMatchmakingV1Search {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchmakingMatchmakingSearchResource;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/search".to_string()}
}

pub fn get_lol_matchmaking_v_1_search() -> GetLolMatchmakingV1Search {
    GetLolMatchmakingV1Search{}
}


pub struct GetLolMatchmakingV1SearchErrors {}

impl IsApiRequest for GetLolMatchmakingV1SearchErrors {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolMatchmakingMatchmakingSearchErrorResource>;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/search/errors".to_string()}
}

pub fn get_lol_matchmaking_v_1_search_errors() -> GetLolMatchmakingV1SearchErrors {
    GetLolMatchmakingV1SearchErrors{}
}


pub struct GetLolMatchmakingV1SearchErrorsById {
    pub id: i32,
}

impl IsApiRequest for GetLolMatchmakingV1SearchErrorsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchmakingMatchmakingSearchErrorResource;
    fn get_url(&self) -> String {format!("/lol-matchmaking/v1/search/errors/{}", self.id)}
}

pub fn get_lol_matchmaking_v_1_search_errors_by_id(id: i32) -> GetLolMatchmakingV1SearchErrorsById {
    GetLolMatchmakingV1SearchErrorsById{id}
}


pub struct PostLolMatchmakingV1ReadyCheckAccept {}

impl IsApiRequest for PostLolMatchmakingV1ReadyCheckAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/ready-check/accept".to_string()}
}

pub fn post_lol_matchmaking_v_1_ready_check_accept() -> PostLolMatchmakingV1ReadyCheckAccept {
    PostLolMatchmakingV1ReadyCheckAccept{}
}


pub struct PostLolMatchmakingV1ReadyCheckDecline {}

impl IsApiRequest for PostLolMatchmakingV1ReadyCheckDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/ready-check/decline".to_string()}
}

pub fn post_lol_matchmaking_v_1_ready_check_decline() -> PostLolMatchmakingV1ReadyCheckDecline {
    PostLolMatchmakingV1ReadyCheckDecline{}
}


pub struct PostLolMatchmakingV1Search {}

impl IsApiRequest for PostLolMatchmakingV1Search {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/search".to_string()}
}

pub fn post_lol_matchmaking_v_1_search() -> PostLolMatchmakingV1Search {
    PostLolMatchmakingV1Search{}
}


pub struct PutLolMatchmakingV1Search {
    pub body: LolMatchmakingMatchmakingSearchResource,
}

impl IsApiRequest for PutLolMatchmakingV1Search {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-matchmaking/v1/search".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_matchmaking_v_1_search(body: LolMatchmakingMatchmakingSearchResource) -> PutLolMatchmakingV1Search {
    PutLolMatchmakingV1Search{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingDodgeData {
    pub state: LolMatchmakingMatchmakingDodgeState,
    pub dodger_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingLowPriorityData {
    pub penalized_summoner_ids: Vec<u64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub busted_leaver_access_token: String,
    pub reason: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingReadyCheckResource {
    pub state: LolMatchmakingMatchmakingReadyCheckState,
    pub player_response: LolMatchmakingMatchmakingReadyCheckResponse,
    pub dodge_warning: LolMatchmakingMatchmakingDodgeWarning,
    pub timer: f32,
    pub decliner_ids: Vec<u64>,
    pub suppress_ux: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchErrorResource {
    pub id: i32,
    pub error_type: String,
    pub penalized_summoner_id: u64,
    pub penalty_time_remaining: f64,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchResource {
    pub queue_id: i32,
    pub is_currently_in_queue: bool,
    pub lobby_id: String,
    pub search_state: LolMatchmakingMatchmakingSearchState,
    pub time_in_queue: f32,
    pub estimated_queue_time: f32,
    pub ready_check: LolMatchmakingMatchmakingReadyCheckResource,
    pub dodge_data: LolMatchmakingMatchmakingDodgeData,
    pub low_priority_data: LolMatchmakingMatchmakingLowPriorityData,
    pub errors: Vec<LolMatchmakingMatchmakingSearchErrorResource>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolMatchmakingMatchmakingDodgeState {
    #[default]
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolMatchmakingMatchmakingDodgeWarning {
    #[default]
    Penalty,
    Warning,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolMatchmakingMatchmakingReadyCheckResponse {
    #[default]
    Declined,
    Accepted,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolMatchmakingMatchmakingReadyCheckState {
    #[default]
    Error,
    PartyNotReady,
    StrangerNotReady,
    EveryoneReady,
    InProgress,
    Invalid,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolMatchmakingMatchmakingSearchState {
    #[default]
    ServiceShutdown,
    ServiceError,
    Error,
    Found,
    Searching,
    Canceled,
    AbandonedLowPriorityQueue,
    Invalid,
}

