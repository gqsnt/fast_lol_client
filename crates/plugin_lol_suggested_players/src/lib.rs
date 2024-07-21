
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct DeleteLolSuggestedPlayersV1SuggestedPlayersBySummonerId {

    pub summoner_id: u64,
}

impl IsApiRequest for DeleteLolSuggestedPlayersV1SuggestedPlayersBySummonerId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-suggested-players/v1/suggested-players/{}", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_suggested_players_v_1_suggested_players_by_summoner_id(summoner_id: u64) -> DeleteLolSuggestedPlayersV1SuggestedPlayersBySummonerId {
    DeleteLolSuggestedPlayersV1SuggestedPlayersBySummonerId {
        summoner_id
    }
}


pub struct GetLolSuggestedPlayersV1SuggestedPlayers {

}

impl IsApiRequest for GetLolSuggestedPlayersV1SuggestedPlayers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolSuggestedPlayersSuggestedPlayersSuggestedPlayer>;

    fn get_url(&self) -> String {
        "/lol-suggested-players/v1/suggested-players".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_suggested_players_v_1_suggested_players() -> GetLolSuggestedPlayersV1SuggestedPlayers {
    GetLolSuggestedPlayersV1SuggestedPlayers {
        
    }
}


pub struct PostLolSuggestedPlayersV1ReportedPlayer {

    pub body: LolSuggestedPlayersSuggestedPlayersReportedPlayer,
}

impl IsApiRequest for PostLolSuggestedPlayersV1ReportedPlayer {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-suggested-players/v1/reported-player".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_suggested_players_v_1_reported_player(body: LolSuggestedPlayersSuggestedPlayersReportedPlayer) -> PostLolSuggestedPlayersV1ReportedPlayer {
    PostLolSuggestedPlayersV1ReportedPlayer {
        body
    }
}


pub struct PostLolSuggestedPlayersV1VictoriousComrade {

    pub body: LolSuggestedPlayersSuggestedPlayersVictoriousComrade,
}

impl IsApiRequest for PostLolSuggestedPlayersV1VictoriousComrade {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-suggested-players/v1/victorious-comrade".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_suggested_players_v_1_victorious_comrade(body: LolSuggestedPlayersSuggestedPlayersVictoriousComrade) -> PostLolSuggestedPlayersV1VictoriousComrade {
    PostLolSuggestedPlayersV1VictoriousComrade {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersVictoriousComrade {
    pub summoner_id: u64,
    pub summoner_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersReportedPlayer {
    pub reported_summoner_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersSuggestedPlayer {
    pub summoner_name: String,
    pub summoner_id: u64,
    pub common_friend_name: String,
    pub common_friend_id: u64,
    pub reason: LolSuggestedPlayersSuggestedPlayersReason,
    pub game_id: u64,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolSuggestedPlayersSuggestedPlayersReason {
    #[default]
    LegacyPlayAgain,
    HonorInteractions,
    VictoriousComrade,
    FriendOfFriend,
    OnlineFriend,
    PreviousPremade,
}

