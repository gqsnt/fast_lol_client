
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolLeagueSessionV1LeagueSessionToken {

}

impl IsApiRequest for GetLolLeagueSessionV1LeagueSessionToken {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-league-session/v1/league-session-token".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_league_session_v_1_league_session_token() -> GetLolLeagueSessionV1LeagueSessionToken {
    GetLolLeagueSessionV1LeagueSessionToken {
        
    }
}


// OBJECTS


// ENUMS

