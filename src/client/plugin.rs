

pub(crate) enum LolApiPlugin {
    LolSummoner,
    LolChat,
    LolLobby,
    LolChampSelect,
    LolGameFlow,
}


impl LolApiPlugin {
    pub fn get_path(&self) -> &str {
        match self {
            Self::LolSummoner => "/lol-summoner/v1",
            Self::LolChat => "/lol-chat/v1",
            Self::LolLobby => "/lol-lobby/v1",
            Self::LolChampSelect => "/lol-champ-select/v1",
            Self::LolGameFlow => "/lol-gameflow/v1",
        }
    }
}


