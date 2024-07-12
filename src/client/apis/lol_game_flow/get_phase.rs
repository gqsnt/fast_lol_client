

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub enum LolGameFlowPhase {
    #[default]
    None,
    Lobby,
    Matchmaking,
    CheckedIntoTournament,
    ReadyCheck,// Needs to be confirmed
    ChampSelect,
    GameStart,
    FailedToLaunch,
    InProgress,//InGame
    Reconnect,
    WaitingForStats,
    PreEndOfGame,
    EndOfGame,
    TerminatedInError
}