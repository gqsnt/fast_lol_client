use crate::LolLobbyQueueCustomGameSpectatorPolicy;

impl LolLobbyQueueCustomGameSpectatorPolicy{
    pub fn cases() -> Vec<Self>{
        vec![
            LolLobbyQueueCustomGameSpectatorPolicy::AllAllowed,
            LolLobbyQueueCustomGameSpectatorPolicy::LobbyAllowed,
            LolLobbyQueueCustomGameSpectatorPolicy::NotAllowed,
            LolLobbyQueueCustomGameSpectatorPolicy::FriendsAllowed,
        ]
    }
}

impl std::fmt::Display for LolLobbyQueueCustomGameSpectatorPolicy{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).unwrap().fmt(f)
    }
}
