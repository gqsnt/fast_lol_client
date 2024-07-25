
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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

pub fn get_lol_event_mission_v1_event_mission() -> GetLolEventMissionV1EventMission {
    GetLolEventMissionV1EventMission{}
}


pub struct GetLolMissionsV1Data {}

impl IsApiRequest for GetLolMissionsV1Data {
    const METHOD: Method = Method::GET;
    type ReturnType = PlayerMissionEligibilityData;
    fn get_url(&self) -> String {"/lol-missions/v1/data".to_string()}
}

pub fn get_lol_missions_v1_data() -> GetLolMissionsV1Data {
    GetLolMissionsV1Data{}
}


pub struct GetLolMissionsV1Missions {}

impl IsApiRequest for GetLolMissionsV1Missions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PlayerMissionDto>;
    fn get_url(&self) -> String {"/lol-missions/v1/missions".to_string()}
}

pub fn get_lol_missions_v1_missions() -> GetLolMissionsV1Missions {
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

pub fn get_lol_missions_v1_missions_series_by_series_name(series_name: String) -> GetLolMissionsV1MissionsSeriesBySeriesName {
    GetLolMissionsV1MissionsSeriesBySeriesName{series_name}
}


pub struct GetLolMissionsV1Series {}

impl IsApiRequest for GetLolMissionsV1Series {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<SeriesDto>;
    fn get_url(&self) -> String {"/lol-missions/v1/series".to_string()}
}

pub fn get_lol_missions_v1_series() -> GetLolMissionsV1Series {
    GetLolMissionsV1Series{}
}


pub struct PostLolMissionsV1Force {}

impl IsApiRequest for PostLolMissionsV1Force {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-missions/v1/force".to_string()}
}

pub fn post_lol_missions_v1_force() -> PostLolMissionsV1Force {
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

pub fn put_lol_missions_v1_player(body: IdsDto) -> PutLolMissionsV1Player {
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

pub fn put_lol_missions_v1_player_by_mission_id(mission_id: String, body: LolMissionsRewardGroupsSelection) -> PutLolMissionsV1PlayerByMissionId {
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

pub fn put_lol_missions_v2_player_opt(body: LolMissionsSeriesOpt) -> PutLolMissionsV2PlayerOpt {
    PutLolMissionsV2PlayerOpt{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertDto {
    #[serde(rename = "alertTime")]
    pub alert_time: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdsDto {
    #[serde(rename = "missionIds")]
    pub mission_ids: Vec<String>,
    #[serde(rename = "seriesIds")]
    pub series_ids: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGroupsSelection {
    #[serde(rename = "rewardGroups")]
    pub reward_groups: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSeriesOpt {
    #[serde(rename = "seriesId")]
    pub series_id: String,
    pub option: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftEventMissionChain {
    #[serde(rename = "chainIndex")]
    pub chain_index: i32,
    #[serde(rename = "chainSize")]
    pub chain_size: u32,
    pub missions: Vec<PlayerMissionDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionAlertDto {
    #[serde(rename = "type")]
    pub type_: String,
    pub message: String,
    #[serde(rename = "alertTime")]
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
    #[serde(rename = "npeRewardPack")]
    pub npe_reward_pack: NpeRewardPackMetadata,
    #[serde(rename = "missionType")]
    pub mission_type: String,
    #[serde(rename = "weekNum")]
    pub week_num: i32,
    #[serde(rename = "xpReward")]
    pub xp_reward: i32,
    pub chain: i32,
    pub order: i32,
    #[serde(rename = "chainSize")]
    pub chain_size: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MissionProgressDto {
    #[serde(rename = "lastViewedProgress")]
    pub last_viewed_progress: i32,
    #[serde(rename = "currentProgress")]
    pub current_progress: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NpeReward {
    pub renderer: String,
    pub data: Value,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NpeRewardPackMetadata {
    pub index: i32,
    #[serde(rename = "premiumReward")]
    pub premium_reward: bool,
    #[serde(rename = "rewardKey")]
    pub reward_key: String,
    #[serde(rename = "majorReward")]
    pub major_reward: NpeReward,
    #[serde(rename = "minorRewards")]
    pub minor_rewards: Vec<NpeReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventory {
    #[serde(rename = "wardSkins")]
    pub ward_skins: Vec<i64>,
    pub champions: Vec<i32>,
    pub skins: Vec<i32>,
    pub icons: Vec<i32>,
    #[serde(rename = "inventoryJwts")]
    pub inventory_jwts: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionDto {
    pub id: String,
    pub title: String,
    #[serde(rename = "helperText")]
    pub helper_text: String,
    pub description: String,
    #[serde(rename = "backgroundImageUrl")]
    pub background_image_url: String,
    #[serde(rename = "iconImageUrl")]
    pub icon_image_url: String,
    #[serde(rename = "seriesName")]
    pub series_name: String,
    pub locale: String,
    pub sequence: i32,
    pub metadata: MissionMetadata,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "lastUpdatedTimestamp")]
    pub last_updated_timestamp: i64,
    pub objectives: Vec<PlayerMissionObjectiveDto>,
    pub rewards: Vec<PlayerMissionRewardDto>,
    #[serde(rename = "expiringWarnings")]
    pub expiring_warnings: Vec<MissionAlertDto>,
    pub requirements: Vec<String>,
    #[serde(rename = "rewardStrategy")]
    pub reward_strategy: RewardStrategy,
    pub display: MissionDisplay,
    #[serde(rename = "completionExpression")]
    pub completion_expression: String,
    pub viewed: bool,
    #[serde(rename = "isNew")]
    pub is_new: bool,
    pub status: String,
    #[serde(rename = "missionType")]
    pub mission_type: String,
    #[serde(rename = "displayType")]
    pub display_type: String,
    #[serde(rename = "earnedDate")]
    pub earned_date: i64,
    #[serde(rename = "completedDate")]
    pub completed_date: i64,
    #[serde(rename = "cooldownTimeMillis")]
    pub cooldown_time_millis: i64,
    #[serde(rename = "celebrationType")]
    pub celebration_type: String,
    #[serde(rename = "clientNotifyLevel")]
    pub client_notify_level: String,
    #[serde(rename = "internalName")]
    pub internal_name: String,
    pub media: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionEligibilityData {
    pub level: i32,
    #[serde(rename = "loyaltyEnabled")]
    pub loyalty_enabled: bool,
    #[serde(rename = "playerInventory")]
    pub player_inventory: PlayerInventory,
    #[serde(rename = "userInfoToken")]
    pub user_info_token: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionObjectiveDto {
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub progress: MissionProgressDto,
    pub sequence: i32,
    #[serde(rename = "rewardGroups")]
    pub reward_groups: Vec<String>,
    #[serde(rename = "hasObjectiveBasedReward")]
    pub has_objective_based_reward: bool,
    pub status: String,
    pub requirements: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionRewardDto {
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    #[serde(rename = "rewardGroup")]
    pub reward_group: String,
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "smallIconUrl")]
    pub small_icon_url: String,
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "uniqueName")]
    pub unique_name: String,
    #[serde(rename = "rewardFulfilled")]
    pub reward_fulfilled: bool,
    #[serde(rename = "rewardGroupSelected")]
    pub reward_group_selected: bool,
    pub sequence: i32,
    pub quantity: i32,
    #[serde(rename = "isObjectiveBasedReward")]
    pub is_objective_based_reward: bool,
    pub media: HashMap<String, String>,
    #[serde(rename = "iconNeedsFrame")]
    pub icon_needs_frame: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardStrategy {
    #[serde(rename = "groupStrategy")]
    pub group_strategy: String,
    #[serde(rename = "selectMaxGroupCount")]
    pub select_max_group_count: u16,
    #[serde(rename = "selectMinGroupCount")]
    pub select_min_group_count: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeriesDto {
    pub id: String,
    #[serde(rename = "internalName")]
    pub internal_name: String,
    #[serde(rename = "parentInternalName")]
    pub parent_internal_name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "eligibilityType")]
    pub eligibility_type: String,
    #[serde(rename = "displayType")]
    pub display_type: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "optInButtonText")]
    pub opt_in_button_text: String,
    #[serde(rename = "optOutButtonText")]
    pub opt_out_button_text: String,
    pub status: String,
    #[serde(rename = "startDate")]
    pub start_date: i64,
    #[serde(rename = "endDate")]
    pub end_date: i64,
    #[serde(rename = "createdDate")]
    pub created_date: i64,
    #[serde(rename = "lastUpdatedTimestamp")]
    pub last_updated_timestamp: i64,
    pub viewed: bool,
    pub media: SeriesMediaDto,
    pub tags: Vec<String>,
    pub warnings: Vec<AlertDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeriesMediaDto {
    #[serde(rename = "backgroundUrl")]
    pub background_url: String,
    #[serde(rename = "backgroundImageLargeUrl")]
    pub background_image_large_url: String,
    #[serde(rename = "backgroundImageSmallUrl")]
    pub background_image_small_url: String,
    #[serde(rename = "trackerIconUrl")]
    pub tracker_icon_url: String,
    #[serde(rename = "trackerIcon")]
    pub tracker_icon: String,
    #[serde(rename = "accentColor")]
    pub accent_color: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TutorialMetadata {
    #[serde(rename = "stepNumber")]
    pub step_number: i32,
    #[serde(rename = "queueId")]
    pub queue_id: String,
    #[serde(rename = "displayRewards")]
    pub display_rewards: HashMap<String, String>,
    #[serde(rename = "useQuickSearchMatchmaking")]
    pub use_quick_search_matchmaking: bool,
    #[serde(rename = "useChosenChampion")]
    pub use_chosen_champion: bool,
}


// ENUMS

