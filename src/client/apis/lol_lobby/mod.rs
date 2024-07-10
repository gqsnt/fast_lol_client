use serde_json::Value;

use crate::client::apis::lol_lobby::local_member_player_slots::LolLobbyPutLocalMemberPlayerSlotsBody;
use crate::client::apis::plugin_macro::impl_api_plugin;

pub mod local_member_player_slots;


impl_api_plugin!(
    LolLobby,
    lol_lobby,
    PutLocalMemberPlayerSlots{
        put_local_member_player_slots, reqwest::Method::PUT, "/lobby/members/localMember/player-slots" => Value,
        body: LolLobbyPutLocalMemberPlayerSlotsBody,
    },
);



