use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LolLobbyPostLobbyBody{
    #[serde(rename = "queueId")]
    pub queue_id: i32,
}