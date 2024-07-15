use std::fmt;

pub mod lol_summoner;
pub mod lol_chat;
pub mod lol_game_flow;
pub mod lol_lobby;
pub mod lol_champ_select;

pub mod is_api_request;

mod plugin_macro;
pub mod lol_game_queues;
pub mod lol_matchmaking;
mod lol_end_of_game;

pub enum ApiVersion{
    None,
    V1,
    V2,
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiVersion::None => write!(f, ""),
            ApiVersion::V1 => write!(f, "/v1"),
            ApiVersion::V2 => write!(f, "/v2"),
        }
    }
}







