use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api_request_with_body;
use crate::client::request::ApiRequest;
use crate::client::plugins::LolApiPlugin;

api_request_with_body!(
    LolApiPlugin::LolLobby,
    LolLobbyPutLocalMemberPlayerSlots,
    Method::PUT,
    "/lobby/members/localMember/player-slots",
    LolLobbyPutLocalMemberPlayerSlotsBody,
    Value
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPutLocalMemberPlayerSlotsBody {
    champion_id: u32,
    position_preference: String,
    spell1: u32,
}

