use serde_json::Value;

use crate::client::apis::lol_lobby::local_member_player_slots::LolLobbyPutLocalMemberPlayerSlotsBody;
use crate::client::apis::lol_lobby::post_lobby::LolLobbyPostLobbyBody;
use crate::client::apis::plugin_macro::impl_api_plugin;
use crate::client::apis::is_api_request::IsApiRequest;
pub mod local_member_player_slots;
pub mod post_lobby;




impl_api_plugin!(
    "/lol-lobby/v1",
    PutLocalMemberPlayerSlots{
        put_local_member_player_slots, reqwest::Method::PUT, "/lobby/members/localMember/player-slots" => Value,
        body: LolLobbyPutLocalMemberPlayerSlotsBody,
    },
);




impl_api_plugin!(
    "/lol-lobby/v2",
    PostLobby{
        post_lobby, reqwest::Method::POST, "/lobby" => Value,
        body:LolLobbyPostLobbyBody,
    },
);


