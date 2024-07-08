use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api_request;
use crate::client::api_data::ApiDataBody;
use crate::client::plugin::LolApiPlugin;
use crate::client::request::ApiRequest;
use crate::client::api_data::IsApiData;

api_request!(
    LolApiPlugin::LolLobby,
    LolLobbyPutLocalMemberPlayerSlots,
    Method::PUT,
    "/lobby/members/localMember/player-slots",
    ApiDataBody<LolLobbyPutLocalMemberPlayerSlotsBody>,
    Value
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPutLocalMemberPlayerSlotsBody {
    champion_id: u32,
    position_preference: String,
    spell1: u32,
}

