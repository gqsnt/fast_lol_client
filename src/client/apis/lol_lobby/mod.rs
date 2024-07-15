use serde_json::Value;
use crate::client::apis::lol_lobby::local_member_player_slots::LolLobbyPutLocalMemberPlayerSlotsBody;
use crate::client::apis::lol_lobby::matchmaking_search_state::LolLobbyMatchmakingSearchState;
use crate::client::apis::lol_lobby::post_lobby::{LolLobbyPostLobbyBody, LolLobbySession};
use crate::client::apis::plugin_macro::impl_api_plugin;

pub mod local_member_player_slots;
pub mod post_lobby;
pub mod matchmaking_search_state;

impl_api_plugin!(
    "/lol-lobby",
    V1{
        PutLocalMemberPlayerSlots{
            put_local_member_player_slots, reqwest::Method::PUT, "/lobby/members/localMember/player-slots",
            body:LolLobbyPutLocalMemberPlayerSlotsBody
        } => Value
    }
    V2{
        PostLobby{
            post_lobby, reqwest::Method::POST, "/lobby", body:LolLobbyPostLobbyBody
        } => LolLobbySession
        GetLobby{
            get_lobby, reqwest::Method::GET, "/lobby"
        } => LolLobbySession
        DeleteLobby{
            delete_lobby, reqwest::Method::DELETE, "/lobby"
        } => Value
        PostMatchmakingSearch{
            post_matchmaking_search, reqwest::Method::POST, "/lobby/matchmaking/search"
        } => Value
        DeleteMatchmakingSearch{
            delete_matchmaking_search, reqwest::Method::DELETE, "/lobby/matchmaking/search"
        } => Value
        GetMatchmakingSearchState{
            get_matchmaking_search_state, reqwest::Method::GET, "/lobby/matchmaking/search-state"
        } => LolLobbyMatchmakingSearchState

    }
);





