use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPutLocalMemberPlayerSlotsBody {
    champion_id: u32,
    position_preference: String,
    spell1: u32,
}

