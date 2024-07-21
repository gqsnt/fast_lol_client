
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolGameClientChatV1Buddies {

}

impl IsApiRequest for GetLolGameClientChatV1Buddies {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v1/buddies".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_client_chat_v_1_buddies() -> GetLolGameClientChatV1Buddies {
    GetLolGameClientChatV1Buddies {
        
    }
}


pub struct GetLolGameClientChatV1IgnoredSummoners {

}

impl IsApiRequest for GetLolGameClientChatV1IgnoredSummoners {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u64>;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v1/ignored-summoners".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_client_chat_v_1_ignored_summoners() -> GetLolGameClientChatV1IgnoredSummoners {
    GetLolGameClientChatV1IgnoredSummoners {
        
    }
}


pub struct GetLolGameClientChatV1MutedSummoners {

}

impl IsApiRequest for GetLolGameClientChatV1MutedSummoners {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u64>;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v1/muted-summoners".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_client_chat_v_1_muted_summoners() -> GetLolGameClientChatV1MutedSummoners {
    GetLolGameClientChatV1MutedSummoners {
        
    }
}


pub struct GetLolGameClientChatV2Buddies {

}

impl IsApiRequest for GetLolGameClientChatV2Buddies {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolGameClientChatBuddy>;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v2/buddies".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_client_chat_v_2_buddies() -> GetLolGameClientChatV2Buddies {
    GetLolGameClientChatV2Buddies {
        
    }
}


pub struct GetLolGameClientChatV2IgnoredPlayers {

}

impl IsApiRequest for GetLolGameClientChatV2IgnoredPlayers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u64>;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v2/ignored-players".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_client_chat_v_2_ignored_players() -> GetLolGameClientChatV2IgnoredPlayers {
    GetLolGameClientChatV2IgnoredPlayers {
        
    }
}


pub struct GetLolGameClientChatV2MutedPlayers {

}

impl IsApiRequest for GetLolGameClientChatV2MutedPlayers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u64>;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v2/muted-players".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_client_chat_v_2_muted_players() -> GetLolGameClientChatV2MutedPlayers {
    GetLolGameClientChatV2MutedPlayers {
        
    }
}


pub struct PostLolGameClientChatV1InstantMessages {

    pub summoner_name: String,
    pub message: String,
}

impl IsApiRequest for PostLolGameClientChatV1InstantMessages {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v1/instant-messages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "summonerName" : self.summoner_name,
            "message" : self.message,
        }))
    }
}

pub fn post_lol_game_client_chat_v_1_instant_messages(summoner_name: String, message: String) -> PostLolGameClientChatV1InstantMessages {
    PostLolGameClientChatV1InstantMessages {
        summoner_name, message
    }
}


pub struct PostLolGameClientChatV2InstantMessages {

    pub body: LolGameClientChatMessageToPlayer,
}

impl IsApiRequest for PostLolGameClientChatV2InstantMessages {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-game-client-chat/v2/instant-messages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_game_client_chat_v_2_instant_messages(body: LolGameClientChatMessageToPlayer) -> PostLolGameClientChatV2InstantMessages {
    PostLolGameClientChatV2InstantMessages {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatBuddy {
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub summoner_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatMessageToPlayer {
    pub game_name: String,
    pub tag_line: String,
    pub body: String,
}


// ENUMS

