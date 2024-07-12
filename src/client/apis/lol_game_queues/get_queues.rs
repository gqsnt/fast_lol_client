


use serde::{Deserialize, Deserializer, Serialize};


pub type LolGameQueuesGetQueues = Vec<LolGameQueuesGetQueue>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LolGameQueuesGetQueue {
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub description: String,
    #[serde(rename = "detailedDescription")]
    pub detailed_description: String,
    #[serde(rename = "type")]
    pub queue_type: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "assetMutator")]
    pub asset_mutator: String,
    #[serde(rename = "maxTierForPremadeSize2")]
    pub max_tier_for_premade_size_2: String,
    #[serde(rename = "maxDivisionForPremadeSize2")]
    pub max_division_for_premade_size_2: String,
    pub category: QueueCategory,
    #[serde(rename = "gameTypeConfig")]
    pub game_type_config: GameTypeConfig,
    #[serde(rename = "numPlayersPerTeam")]
    pub num_players_per_team: i32,
    #[serde(rename = "minimumParticipantListSize")]
    pub minimum_participant_list_size: i32,
    #[serde(rename = "maximumParticipantListSize")]
    pub maximum_participant_list_size: i32,
    #[serde(rename = "minLevel")]
    pub min_level: i32,
    #[serde(rename = "isRanked")]
    pub is_ranked: bool,
    #[serde(rename = "areFreeChampionsAllowed")]
    pub are_free_champions_allowed: bool,
    #[serde(rename = "isTeamBuilderManaged")]
    pub is_team_builder_managed: bool,
    #[serde(rename = "queueAvailability", deserialize_with = "queue_availability_to_bool")]
    pub is_available: bool,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    #[serde(rename = "queueRewards")]
    pub queue_rewards: QueueRewards,
    #[serde(rename = "spectatorEnabled")]
    pub spectator_enabled: bool,
    #[serde(rename = "championsRequiredToPlay")]
    pub champions_required_to_play: i32,
    #[serde(rename = "allowablePremadeSizes")]
    pub allowable_premade_sizes: Vec<i32>,
    #[serde(rename = "showPositionSelector")]
    pub show_position_selector: bool,
    #[serde(rename = "showQuickPlaySlotSelection")]
    pub show_quick_play_slot_selection: bool,
    #[serde(rename = "lastToggledOffTime")]
    pub last_toggled_off_time: i64,
    #[serde(rename = "lastToggledOnTime")]
    pub last_toggled_on_time: i64,
    #[serde(rename = "removalFromGameAllowed")]
    pub removal_from_game_allowed: bool,
    #[serde(rename = "removalFromGameDelayMinutes")]
    pub removal_from_game_delay_minutes: i32,
}

impl std::fmt::Display for LolGameQueuesGetQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.detailed_description.len() > 0 {self.detailed_description.as_str()} else {self.name.as_str()})
    }
}

impl PartialEq for LolGameQueuesGetQueue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}






#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameTypeConfig {
    pub id: i32,
    pub name: String,
    #[serde(rename = "maxAllowableBans")]
    pub max_allowable_bans: i32,
    #[serde(rename = "allowTrades")]
    pub allow_trades: bool,
    #[serde(rename = "exclusivePick")]
    pub exclusive_pick: bool,
    #[serde(rename = "duplicatePick")]
    pub duplicate_pick: bool,
    #[serde(rename = "teamChampionPool")]
    pub team_champion_pool: bool,
    #[serde(rename = "crossTeamChampionPool")]
    pub cross_team_champion_pool: bool,
    #[serde(rename = "advancedLearningQuests")]
    pub advanced_learning_quests: bool,
    #[serde(rename = "battleBoost")]
    pub battle_boost: bool,
    #[serde(rename = "deathMatch")]
    pub death_match: bool,
    #[serde(rename = "doNotRemove")]
    pub do_not_remove: bool,
    #[serde(rename = "learningQuests")]
    pub learning_quests: bool,
    #[serde(rename = "onboardCoopBeginner")]
    pub onboard_coop_beginner: bool,
    #[serde(rename = "reroll")]
    pub reroll: bool,
    #[serde(rename = "mainPickTimerDuration")]
    pub main_pick_timer_duration: i32,
    #[serde(rename = "postPickTimerDuration")]
    pub post_pick_timer_duration: i32,
    #[serde(rename = "banTimerDuration")]
    pub ban_timer_duration: i32,
    #[serde(rename = "pickMode")]
    pub pick_mode: String,
    #[serde(rename = "banMode")]
    pub ban_mode: String,
    #[serde(rename = "gameModeOverride")]
    pub game_mode_override: Option<String>,
    #[serde(rename = "numPlayersPerTeamOverride")]
    pub num_players_per_team_override: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueueRewards {
    #[serde(rename = "isIpEnabled")]
    pub is_ip_enabled: bool,
    #[serde(rename = "isXpEnabled")]
    pub is_xp_enabled: bool,
    #[serde(rename = "isChampionPointsEnabled")]
    pub is_champion_points_enabled: bool,
    #[serde(rename = "partySizeIpRewards")]
    pub party_size_ip_rewards: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QueueCategory{
    VersusAi,
    PvP,
    Alpha,
    Custom,
    None,
}


fn queue_availability_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s == "Available")
}