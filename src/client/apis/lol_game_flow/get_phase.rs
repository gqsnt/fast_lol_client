

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub enum LolGameFlowPhase {
    #[default]
    None,
    Lobby,
    Matchmaking,
    ReadyCheck,// Needs to be confirmed
    ChampSelect,
    InProgress,//InGame
    WaitingForStats,
    PostGame,
}