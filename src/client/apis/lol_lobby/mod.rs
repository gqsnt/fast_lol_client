use serde_json::Value;

use crate::client::apis::lol_lobby::local_member_player_slots::LolLobbyPutLocalMemberPlayerSlotsBody;
use crate::client::apis::lol_lobby::post_lobby::{LolLobbyPostLobbyBody, LolLobbySession};
use crate::client::apis::plugin_macro::impl_api_plugin;

pub mod local_member_player_slots;
pub mod post_lobby;




impl_api_plugin!(
    "/lol-lobby",
    V1{
        PutLocalMemberPlayerSlots{
            fn:put_local_member_player_slots,
            method: reqwest::Method::PUT,
            url:  "/lobby/members/localMember/player-slots",
            body:LolLobbyPutLocalMemberPlayerSlotsBody
            => Value
        }
    }
    V2{
         PostLobby{
            fn:post_lobby, method:reqwest::Method::POST, url:  "/lobby", body:LolLobbyPostLobbyBody => LolLobbySession
        }
        GetLobby{
            fn:get_lobby, method: reqwest::Method::GET,url: "/lobby" => LolLobbySession
        }
    }
);





