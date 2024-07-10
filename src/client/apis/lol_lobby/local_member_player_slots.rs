use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::plugin::LolApiPlugin;
use crate::client::request::ApiRequest;



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPutLocalMemberPlayerSlotsBody {
    champion_id: u32,
    position_preference: String,
    spell1: u32,
}

