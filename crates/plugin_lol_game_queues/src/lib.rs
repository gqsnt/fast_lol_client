
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolGameQueuesV1Custom {

}

impl IsApiRequest for GetLolGameQueuesV1Custom {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueCustomGame;

    fn get_url(&self) -> String {
        "/lol-game-queues/v1/custom".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_queues_v_1_custom() -> GetLolGameQueuesV1Custom {
    GetLolGameQueuesV1Custom {
        
    }
}


pub struct GetLolGameQueuesV1CustomNonDefault {

}

impl IsApiRequest for GetLolGameQueuesV1CustomNonDefault {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueCustomGame;

    fn get_url(&self) -> String {
        "/lol-game-queues/v1/custom-non-default".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_queues_v_1_custom_non_default() -> GetLolGameQueuesV1CustomNonDefault {
    GetLolGameQueuesV1CustomNonDefault {
        
    }
}


pub struct GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId {

    pub game_type_config_id: u32,
}

impl IsApiRequest for GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueGameTypeConfig;

    fn get_url(&self) -> String {
        format!("/lol-game-queues/v1/game-type-config/{}", self.game_type_config_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_queues_v_1_game_type_config_by_game_type_config_id(game_type_config_id: u32) -> GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId {
    GetLolGameQueuesV1GameTypeConfigByGameTypeConfigId {
        game_type_config_id
    }
}


pub struct GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId {

    pub game_type_config_id: u32,
    pub map_id: i32,
}

impl IsApiRequest for GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueueGameTypeConfig;

    fn get_url(&self) -> String {
        format!("/lol-game-queues/v1/game-type-config/{}/map/{}", self.game_type_config_id, self.map_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_queues_v_1_game_type_config_by_game_type_config_id_map_by_map_id(game_type_config_id: u32, map_id: i32) -> GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId {
    GetLolGameQueuesV1GameTypeConfigByGameTypeConfigIdMapByMapId {
        game_type_config_id, map_id
    }
}


pub struct GetLolGameQueuesV1Queues {

}

impl IsApiRequest for GetLolGameQueuesV1Queues {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolGameQueuesQueue>;

    fn get_url(&self) -> String {
        "/lol-game-queues/v1/queues".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_queues_v_1_queues() -> GetLolGameQueuesV1Queues {
    GetLolGameQueuesV1Queues {
        
    }
}


pub struct GetLolGameQueuesV1QueuesById {

    pub id: i32,
}

impl IsApiRequest for GetLolGameQueuesV1QueuesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueue;

    fn get_url(&self) -> String {
        format!("/lol-game-queues/v1/queues/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_queues_v_1_queues_by_id(id: i32) -> GetLolGameQueuesV1QueuesById {
    GetLolGameQueuesV1QueuesById {
        id
    }
}


pub struct GetLolGameQueuesV1QueuesTypeByQueueType {

    pub queue_type: String,
}

impl IsApiRequest for GetLolGameQueuesV1QueuesTypeByQueueType {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameQueuesQueue;

    fn get_url(&self) -> String {
        format!("/lol-game-queues/v1/queues/type/{}", self.queue_type)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_game_queues_v_1_queues_type_by_queue_type(queue_type: String) -> GetLolGameQueuesV1QueuesTypeByQueueType {
    GetLolGameQueuesV1QueuesTypeByQueueType {
        queue_type
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueReward {
    pub is_ip_enabled: bool,
    pub is_xp_enabled: bool,
    pub is_champion_points_enabled: bool,
    pub party_size_ip_rewards: Vec<i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGameSubcategory {
    pub map_id: i32,
    pub game_mode: String,
    pub mutators: Vec<LolGameQueuesQueueGameTypeConfig>,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub max_player_count: i32,
    pub min_level: u32,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub custom_spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
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


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGame {
    pub subcategories: Vec<LolGameQueuesQueueCustomGameSubcategory>,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
    pub spectator_slot_limit: u32,
    pub game_server_regions: Option<Vec<String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
    pub type_: String,
    pub game_mode: String,
    pub asset_mutator: String,
    pub max_tier_for_premade_size_2: String,
    pub max_division_for_premade_size_2: String,
    pub category: LolGameQueuesQueueGameCategory,
    pub game_type_config: LolGameQueuesQueueGameTypeConfig,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub min_level: u32,
    pub is_ranked: bool,
    pub are_free_champions_allowed: bool,
    pub is_team_builder_managed: bool,
    pub queue_availability: LolGameQueuesQueueAvailability,
    pub is_visible: bool,
    pub queue_rewards: LolGameQueuesQueueReward,
    pub spectator_enabled: bool,
    pub champions_required_to_play: u32,
    pub allowable_premade_sizes: Vec<i32>,
    pub show_position_selector: bool,
    pub show_quick_play_slot_selection: bool,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
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
pub enum LolGameQueuesQueueGameCategory {
    #[default]
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameQueuesQueueCustomGameSpectatorPolicy {
    #[default]
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}

