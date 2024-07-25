
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolGameQueuesV1Custom {}

impl IsApiRequest for GetLolGameQueuesV1Custom {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueCustomGame;
    fn get_url(&self) -> String {"/lol-game-queues/v1/custom".to_string()}
}

pub fn get_lol_game_queues_v1_custom() -> GetLolGameQueuesV1Custom {
    GetLolGameQueuesV1Custom{}
}


pub struct GetLolGameQueuesV1CustomNonDefault {}

impl IsApiRequest for GetLolGameQueuesV1CustomNonDefault {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueCustomGame;
    fn get_url(&self) -> String {"/lol-game-queues/v1/custom-non-default".to_string()}
}

pub fn get_lol_game_queues_v1_custom_non_default() -> GetLolGameQueuesV1CustomNonDefault {
    GetLolGameQueuesV1CustomNonDefault{}
}


pub struct GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId {
    pub game_type_config_id: u32,
}

impl IsApiRequest for GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueGameTypeConfig;
    fn get_url(&self) -> String {format!("/lol-game-queues/v1/game-type-config/{}", self.game_type_config_id)}
}

pub fn get_lol_game_queues_v1_game_type_config_by_game_type_config_id(game_type_config_id: u32) -> GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId {
    GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId{game_type_config_id}
}


pub struct GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId {
    pub game_type_config_id: u32,
    pub map_id: i32,
}

impl IsApiRequest for GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueGameTypeConfig;
    fn get_url(&self) -> String {format!("/lol-game-queues/v1/game-type-config/{}/map/{}", self.game_type_config_id, self.map_id)}
}

pub fn get_lol_game_queues_v1_game_type_config_by_game_type_config_id_map_by_map_id(game_type_config_id: u32, map_id: i32) -> GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId {
    GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId{game_type_config_id, map_id}
}


pub struct GetLolGameQueuesV1Queues {}

impl IsApiRequest for GetLolGameQueuesV1Queues {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolGameQueuesQueue>;
    fn get_url(&self) -> String {"/lol-game-queues/v1/queues".to_string()}
}

pub fn get_lol_game_queues_v1_queues() -> GetLolGameQueuesV1Queues {
    GetLolGameQueuesV1Queues{}
}


pub struct GetLolGameQueuesV1QueuesById {
    pub id: i32,
}

impl IsApiRequest for GetLolGameQueuesV1QueuesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueue;
    fn get_url(&self) -> String {format!("/lol-game-queues/v1/queues/{}", self.id)}
}

pub fn get_lol_game_queues_v1_queues_by_id(id: i32) -> GetLolGameQueuesV1QueuesById {
    GetLolGameQueuesV1QueuesById{id}
}


pub struct GetLolGameQueuesV1QueuesTypeByQueueType {
    pub queue_type: String,
}

impl IsApiRequest for GetLolGameQueuesV1QueuesTypeByQueueType {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueue;
    fn get_url(&self) -> String {format!("/lol-game-queues/v1/queues/type/{}", self.queue_type)}
}

pub fn get_lol_game_queues_v1_queues_type_by_queue_type(queue_type: String) -> GetLolGameQueuesV1QueuesTypeByQueueType {
    GetLolGameQueuesV1QueuesTypeByQueueType{queue_type}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueue {
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
    pub type_: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "assetMutator")]
    pub asset_mutator: String,
    #[serde(rename = "maxTierForPremadeSize2")]
    pub max_tier_for_premade_size_2: String,
    #[serde(rename = "maxDivisionForPremadeSize2")]
    pub max_division_for_premade_size_2: String,
    pub category: LolGameQueuesQueueGameCategory,
    #[serde(rename = "gameTypeConfig")]
    pub game_type_config: LolGameQueuesQueueGameTypeConfig,
    #[serde(rename = "numPlayersPerTeam")]
    pub num_players_per_team: i32,
    #[serde(rename = "minimumParticipantListSize")]
    pub minimum_participant_list_size: i32,
    #[serde(rename = "maximumParticipantListSize")]
    pub maximum_participant_list_size: i32,
    #[serde(rename = "minLevel")]
    pub min_level: u32,
    #[serde(rename = "isRanked")]
    pub is_ranked: bool,
    #[serde(rename = "areFreeChampionsAllowed")]
    pub are_free_champions_allowed: bool,
    #[serde(rename = "isTeamBuilderManaged")]
    pub is_team_builder_managed: bool,
    #[serde(rename = "queueAvailability")]
    pub queue_availability: LolGameQueuesQueueAvailability,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    #[serde(rename = "queueRewards")]
    pub queue_rewards: LolGameQueuesQueueReward,
    #[serde(rename = "spectatorEnabled")]
    pub spectator_enabled: bool,
    #[serde(rename = "championsRequiredToPlay")]
    pub champions_required_to_play: u32,
    #[serde(rename = "allowablePremadeSizes")]
    pub allowable_premade_sizes: Vec<i32>,
    #[serde(rename = "showPositionSelector")]
    pub show_position_selector: bool,
    #[serde(rename = "showQuickPlaySlotSelection")]
    pub show_quick_play_slot_selection: bool,
    #[serde(rename = "lastToggledOffTime")]
    pub last_toggled_off_time: u64,
    #[serde(rename = "lastToggledOnTime")]
    pub last_toggled_on_time: u64,
    #[serde(rename = "removalFromGameAllowed")]
    pub removal_from_game_allowed: bool,
    #[serde(rename = "removalFromGameDelayMinutes")]
    pub removal_from_game_delay_minutes: i32,
    #[serde(rename = "gameSelectModeGroup")]
    pub game_select_mode_group: String,
    #[serde(rename = "gameSelectCategory")]
    pub game_select_category: String,
    #[serde(rename = "gameSelectPriority")]
    pub game_select_priority: u8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGame {
    pub subcategories: Vec<LolGameQueuesQueueCustomGameSubcategory>,
    #[serde(rename = "queueAvailability")]
    pub queue_availability: LolGameQueuesQueueAvailability,
    #[serde(rename = "spectatorPolicies")]
    pub spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
    #[serde(rename = "spectatorSlotLimit")]
    pub spectator_slot_limit: u32,
    #[serde(rename = "gameServerRegions")]
    pub game_server_regions: Option<Vec<String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGameSubcategory {
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    pub mutators: Vec<LolGameQueuesQueueGameTypeConfig>,
    #[serde(rename = "numPlayersPerTeam")]
    pub num_players_per_team: i32,
    #[serde(rename = "minimumParticipantListSize")]
    pub minimum_participant_list_size: i32,
    #[serde(rename = "maximumParticipantListSize")]
    pub maximum_participant_list_size: i32,
    #[serde(rename = "maxPlayerCount")]
    pub max_player_count: i32,
    #[serde(rename = "minLevel")]
    pub min_level: u32,
    #[serde(rename = "queueAvailability")]
    pub queue_availability: LolGameQueuesQueueAvailability,
    #[serde(rename = "customSpectatorPolicies")]
    pub custom_spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueGameTypeConfig {
    pub id: i64,
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


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueReward {
    #[serde(rename = "isIpEnabled")]
    pub is_ip_enabled: bool,
    #[serde(rename = "isXpEnabled")]
    pub is_xp_enabled: bool,
    #[serde(rename = "isChampionPointsEnabled")]
    pub is_champion_points_enabled: bool,
    #[serde(rename = "partySizeIpRewards")]
    pub party_size_ip_rewards: Vec<i32>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameQueuesQueueAvailability {
    #[default]
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameQueuesQueueCustomGameSpectatorPolicy {
    #[default]
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameQueuesQueueGameCategory {
    #[default]
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}

