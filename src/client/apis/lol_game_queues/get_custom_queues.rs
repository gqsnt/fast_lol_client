use std::fmt;
use serde::{Serialize, Deserialize};
use crate::client::apis::lol_game_queues::get_queues::LolGameMode;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGame {
    pub subcategories: Vec<LolGameQueuesQueueCustomGameSubcategory>,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
    pub spectator_slot_limit: u32,
    pub game_server_regions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGameSubcategory {
    pub map_id: i32,
    pub game_mode: LolGameMode,
    pub mutators: Vec<LolGameQueuesQueueGameTypeConfig>,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub max_player_count: i32,
    pub min_level: u32,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub custom_spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
}


impl fmt::Display for LolGameQueuesQueueCustomGameSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.game_mode)
    }
}


impl PartialEq for LolGameQueuesQueueCustomGameSubcategory {
    fn eq(&self, other: &Self) -> bool {
        self.game_mode == other.game_mode
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
    pub game_mode_override: Option<String>,
    pub num_players_per_team_override: Option<i32>,
}

impl fmt::Display for LolGameQueuesQueueGameTypeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for LolGameQueuesQueueGameTypeConfig {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}



#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum LolGameQueuesQueueAvailability {
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
    #[default]
    Invalid,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum LolGameQueuesQueueCustomGameSpectatorPolicy {
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
    #[default]
    Invalid,
}