use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LolGameFlowGetSession {
    pub phase: String,
    #[serde(rename = "gameData")]
    pub game_data: GameData,
    #[serde(rename = "gameClient")]
    pub game_client: GameClient,
    pub map: Map,
    #[serde(rename = "gameDodge")]
    pub game_dodge: GameDodge,
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub queue: Queue,
    #[serde(rename = "isCustomGame")]
    pub is_custom_game: bool,
    #[serde(rename = "gameName")]
    pub game_name: String,
    pub password: String,
    #[serde(rename = "teamOne")]
    pub team_one: Vec<Team>,
    #[serde(rename = "teamTwo")]
    pub team_two: Vec<Team>,
    #[serde(rename = "playerChampionSelections")]
    pub player_champion_selections: Vec<PlayerChampionSelection>,
    #[serde(rename = "spectatorsAllowed")]
    pub spectators_allowed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Queue {
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
    pub category: String,
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
    #[serde(rename = "queueAvailability")]
    pub queue_availability: String,
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
    #[serde(rename = "lastToggledOffTime")]
    pub last_toggled_off_time: i64,
    #[serde(rename = "lastToggledOnTime")]
    pub last_toggled_on_time: i64,
    #[serde(rename = "removalFromGameAllowed")]
    pub removal_from_game_allowed: bool,
    #[serde(rename = "removalFromGameDelayMinutes")]
    pub removal_from_game_delay_minutes: i32,
}

#[derive(Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Team {
    #[serde(flatten)]
    pub additional_props: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerChampionSelection {
    #[serde(flatten)]
    pub additional_props: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct GameClient {
    #[serde(rename = "serverIp")]
    pub server_ip: String,
    #[serde(rename = "serverPort")]
    pub server_port: i32,
    #[serde(rename = "observerServerIp")]
    pub observer_server_ip: String,
    #[serde(rename = "observerServerPort")]
    pub observer_server_port: i32,
    pub running: bool,
    pub visible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub id: i32,
    pub name: String,
    #[serde(rename = "mapStringId")]
    pub map_string_id: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameModeName")]
    pub game_mode_name: String,
    #[serde(rename = "gameModeShortName")]
    pub game_mode_short_name: String,
    #[serde(rename = "gameMutator")]
    pub game_mutator: String,
    #[serde(rename = "isRGM")]
    pub is_rgm: bool,
    pub description: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "platformName")]
    pub platform_name: String,
    pub assets: HashMap<String, String>,
    #[serde(rename = "categorizedContentBundles")]
    pub categorized_content_bundles: serde_json::Value,
    pub properties: serde_json::Value,
    #[serde(rename = "perPositionRequiredSummonerSpells")]
    pub per_position_required_summoner_spells: PerPositionSummonerSpells,
    #[serde(rename = "perPositionDisallowedSummonerSpells")]
    pub per_position_disallowed_summoner_spells: PerPositionSummonerSpells,
}

#[derive(Serialize, Deserialize)]
pub struct PerPositionSummonerSpells {
    #[serde(flatten)]
    pub spells: std::collections::HashMap<String, Spells>,
}

#[derive(Serialize, Deserialize)]
pub struct Spells {
    pub spells: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct GameDodge {
    pub state: String,
    #[serde(rename = "dodgeIds")]
    pub dodge_ids: Vec<i32>,
    pub phase: String,
}