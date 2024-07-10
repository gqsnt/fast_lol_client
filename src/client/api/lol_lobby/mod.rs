use serde_json::Value;

use crate::client::api::lol_lobby::local_member_player_slots::LolLobbyPutLocalMemberPlayerSlotsBody;
use crate::client::api::plugin_macro::impl_api_plugin;

pub mod local_member_player_slots;


impl_api_plugin!(
    LolLobby,
    PutLocalMemberPlayerSlots{
        put_local_member_player_slots, reqwest::Method::PUT, "/lobby/members/localMember/player-slots" => Value,
        body: LolLobbyPutLocalMemberPlayerSlotsBody,
    },
);



