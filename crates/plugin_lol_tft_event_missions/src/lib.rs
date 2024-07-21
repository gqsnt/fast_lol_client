
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolEventMissionV1EventMission {}

impl IsApiRequest for GetLolEventMissionV1EventMission {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolTftEventTftEventMissionChain>;
    fn get_url(&self) -> String {"/lol-event-mission/v1/event-mission".to_string()}
}

pub fn get_lol_event_mission_v_1_event_mission() -> GetLolEventMissionV1EventMission {
    GetLolEventMissionV1EventMission{}
}


pub struct GetLolMissionsV1Data {}

impl IsApiRequest for GetLolMissionsV1Data {
    const METHOD: Method = Method::GET;
    type ReturnType = PlayerMissionEligibilityData;
    fn get_url(&self) -> String {"/lol-missions/v1/data".to_string()}
}

pub fn get_lol_missions_v_1_data() -> GetLolMissionsV1Data {
    GetLolMissionsV1Data{}
}


pub struct GetLolMissionsV1Missions {}

impl IsApiRequest for GetLolMissionsV1Missions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PlayerMissionDto>;
    fn get_url(&self) -> String {"/lol-missions/v1/missions".to_string()}
}

pub fn get_lol_missions_v_1_missions() -> GetLolMissionsV1Missions {
    GetLolMissionsV1Missions{}
}


pub struct GetLolMissionsV1MissionsSeriesBySeriesName {
    pub series_name: String,
}

impl IsApiRequest for GetLolMissionsV1MissionsSeriesBySeriesName {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PlayerMissionDto>;
    fn get_url(&self) -> String {format!("/lol-missions/v1/missions/series/{}", self.series_name)}
}

pub fn get_lol_missions_v_1_missions_series_by_series_name(series_name: String) -> GetLolMissionsV1MissionsSeriesBySeriesName {
    GetLolMissionsV1MissionsSeriesBySeriesName{series_name}
}


pub struct GetLolMissionsV1Series {}

impl IsApiRequest for GetLolMissionsV1Series {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<SeriesDto>;
    fn get_url(&self) -> String {"/lol-missions/v1/series".to_string()}
}

pub fn get_lol_missions_v_1_series() -> GetLolMissionsV1Series {
    GetLolMissionsV1Series{}
}


pub struct PostLolMissionsV1Force {}

impl IsApiRequest for PostLolMissionsV1Force {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-missions/v1/force".to_string()}
}

pub fn post_lol_missions_v_1_force() -> PostLolMissionsV1Force {
    PostLolMissionsV1Force{}
}


pub struct PutLolMissionsV1Player {
    pub body: IdsDto,
}

impl IsApiRequest for PutLolMissionsV1Player {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-missions/v1/player".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_missions_v_1_player(body: IdsDto) -> PutLolMissionsV1Player {
    PutLolMissionsV1Player{body}
}


pub struct PutLolMissionsV1PlayerByMissionId {
    pub mission_id: String,
    pub body: LolMissionsRewardGroupsSelection,
}

impl IsApiRequest for PutLolMissionsV1PlayerByMissionId {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-missions/v1/player/{}", self.mission_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_missions_v_1_player_by_mission_id(mission_id: String, body: LolMissionsRewardGroupsSelection) -> PutLolMissionsV1PlayerByMissionId {
    PutLolMissionsV1PlayerByMissionId{mission_id, body}
}


pub struct PutLolMissionsV2PlayerOpt {
    pub body: LolMissionsSeriesOpt,
}

impl IsApiRequest for PutLolMissionsV2PlayerOpt {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-missions/v2/player/opt".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_missions_v_2_player_opt(body: LolMissionsSeriesOpt) -> PutLolMissionsV2PlayerOpt {
    PutLolMissionsV2PlayerOpt{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertDto {
    pub alert_time: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdsDto {
    pub mission_ids: Vec<String>,
    pub series_ids: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGroupsSelection {
    pub reward_groups: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSeriesOpt {
    pub series_id: String,
    pub option: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftEventMissionChain {
    pub chain_index: i32,
    pub chain_size: u32,
    pub missions: Vec<PlayerMissionDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionAlertDto {
    pub type_: String,
    pub message: String,
    pub alert_time: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionDisplay {
    pub attributes: Vec<String>,
    pub locations: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionMetadata {
    pub tutorial: TutorialMetadata,
    pub npe_reward_pack: NpeRewardPackMetadata,
    pub mission_type: String,
    pub week_num: i32,
    pub xp_reward: i32,
    pub chain: i32,
    pub order: i32,
    pub chain_size: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionProgressDto {
    pub last_viewed_progress: i32,
    pub current_progress: i32,
    pub total_count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NpeReward {
    pub renderer: String,
    pub data: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NpeRewardPackMetadata {
    pub index: i32,
    pub premium_reward: bool,
    pub reward_key: String,
    pub major_reward: NpeReward,
    pub minor_rewards: Vec<NpeReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventory {
    pub ward_skins: Vec<i64>,
    pub champions: Vec<i32>,
    pub skins: Vec<i32>,
    pub icons: Vec<i32>,
    pub inventory_jwts: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionDto {
    pub id: String,
    pub title: String,
    pub helper_text: String,
    pub description: String,
    pub background_image_url: String,
    pub icon_image_url: String,
    pub series_name: String,
    pub locale: String,
    pub sequence: i32,
    pub metadata: MissionMetadata,
    pub start_time: i64,
    pub end_time: i64,
    pub last_updated_timestamp: i64,
    pub objectives: Vec<PlayerMissionObjectiveDto>,
    pub rewards: Vec<PlayerMissionRewardDto>,
    pub expiring_warnings: Vec<MissionAlertDto>,
    pub requirements: Vec<String>,
    pub reward_strategy: RewardStrategy,
    pub display: MissionDisplay,
    pub completion_expression: String,
    pub viewed: bool,
    pub is_new: bool,
    pub status: String,
    pub mission_type: String,
    pub display_type: String,
    pub earned_date: i64,
    pub completed_date: i64,
    pub cooldown_time_millis: i64,
    pub celebration_type: String,
    pub client_notify_level: String,
    pub internal_name: String,
    pub media: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionEligibilityData {
    pub level: i32,
    pub loyalty_enabled: bool,
    pub player_inventory: PlayerInventory,
    pub user_info_token: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionObjectiveDto {
    pub type_: String,
    pub description: String,
    pub progress: MissionProgressDto,
    pub sequence: i32,
    pub reward_groups: Vec<String>,
    pub has_objective_based_reward: bool,
    pub status: String,
    pub requirements: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionRewardDto {
    pub reward_type: String,
    pub reward_group: String,
    pub description: String,
    pub icon_url: String,
    pub small_icon_url: String,
    pub item_id: String,
    pub unique_name: String,
    pub reward_fulfilled: bool,
    pub reward_group_selected: bool,
    pub sequence: i32,
    pub quantity: i32,
    pub is_objective_based_reward: bool,
    pub media: HashMap<String, String>,
    pub icon_needs_frame: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardStrategy {
    pub group_strategy: String,
    pub select_max_group_count: u16,
    pub select_min_group_count: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeriesDto {
    pub id: String,
    pub internal_name: String,
    pub parent_internal_name: String,
    pub type_: String,
    pub eligibility_type: String,
    pub display_type: String,
    pub title: String,
    pub description: String,
    pub opt_in_button_text: String,
    pub opt_out_button_text: String,
    pub status: String,
    pub start_date: i64,
    pub end_date: i64,
    pub created_date: i64,
    pub last_updated_timestamp: i64,
    pub viewed: bool,
    pub media: SeriesMediaDto,
    pub tags: Vec<String>,
    pub warnings: Vec<AlertDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeriesMediaDto {
    pub background_url: String,
    pub background_image_large_url: String,
    pub background_image_small_url: String,
    pub tracker_icon_url: String,
    pub tracker_icon: String,
    pub accent_color: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TutorialMetadata {
    pub step_number: i32,
    pub queue_id: String,
    pub display_rewards: HashMap<String, String>,
    pub use_quick_search_matchmaking: bool,
    pub use_chosen_champion: bool,
}


// ENUMS

