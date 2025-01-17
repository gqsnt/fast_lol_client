
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolLeagueSessionV1LeagueSessionToken {}

impl IsApiRequest for GetLolLeagueSessionV1LeagueSessionToken {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-league-session/v1/league-session-token".to_string()}
}

pub fn get_lol_league_session_v1_league_session_token() -> GetLolLeagueSessionV1LeagueSessionToken {
    GetLolLeagueSessionV1LeagueSessionToken{}
}


// OBJECTS


// ENUMS

