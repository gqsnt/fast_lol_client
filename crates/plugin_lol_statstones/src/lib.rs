
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolStatstonesV1VignetteNotifications {}

impl IsApiRequest for DeleteLolStatstonesV1VignetteNotifications {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-statstones/v1/vignette-notifications".to_string()}
}

pub fn delete_lol_statstones_v1_vignette_notifications() -> DeleteLolStatstonesV1VignetteNotifications {
    DeleteLolStatstonesV1VignetteNotifications{}
}


pub struct DeleteLolStatstonesV1VignetteNotificationsByKey {
    pub key: i32,
}

impl IsApiRequest for DeleteLolStatstonesV1VignetteNotificationsByKey {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-statstones/v1/vignette-notifications/{}", self.key)}
}

pub fn delete_lol_statstones_v1_vignette_notifications_by_key(key: i32) -> DeleteLolStatstonesV1VignetteNotificationsByKey {
    DeleteLolStatstonesV1VignetteNotificationsByKey{key}
}


pub struct GetLolStatstonesV1EogNotificationsByGameId {
    pub game_id: u64,
}

impl IsApiRequest for GetLolStatstonesV1EogNotificationsByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStatstonesEogNotificationEnvelope;
    fn get_url(&self) -> String {format!("/lol-statstones/v1/eog-notifications/{}", self.game_id)}
}

pub fn get_lol_statstones_v1_eog_notifications_by_game_id(game_id: u64) -> GetLolStatstonesV1EogNotificationsByGameId {
    GetLolStatstonesV1EogNotificationsByGameId{game_id}
}


pub struct GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId {
    pub champion_item_id: i32,
}

impl IsApiRequest for GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesStatstone>;
    fn get_url(&self) -> String {format!("/lol-statstones/v1/featured-champion-statstones/{}", self.champion_item_id)}
}

pub fn get_lol_statstones_v1_featured_champion_statstones_by_champion_item_id(champion_item_id: i32) -> GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId {
    GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId{champion_item_id}
}


pub struct GetLolStatstonesV1ProfileSummaryByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolStatstonesV1ProfileSummaryByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesProfileStatstoneSummary>;
    fn get_url(&self) -> String {format!("/lol-statstones/v1/profile-summary/{}", self.puuid)}
}

pub fn get_lol_statstones_v1_profile_summary_by_puuid(puuid: String) -> GetLolStatstonesV1ProfileSummaryByPuuid {
    GetLolStatstonesV1ProfileSummaryByPuuid{puuid}
}


pub struct GetLolStatstonesV1StatstoneByContentIdOwned {
    pub content_id: String,
}

impl IsApiRequest for GetLolStatstonesV1StatstoneByContentIdOwned {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {format!("/lol-statstones/v1/statstone/{}/owned", self.content_id)}
}

pub fn get_lol_statstones_v1_statstone_by_content_id_owned(content_id: String) -> GetLolStatstonesV1StatstoneByContentIdOwned {
    GetLolStatstonesV1StatstoneByContentIdOwned{content_id}
}


pub struct GetLolStatstonesV1StatstonesEnabledQueueIds {}

impl IsApiRequest for GetLolStatstonesV1StatstonesEnabledQueueIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u32>;
    fn get_url(&self) -> String {"/lol-statstones/v1/statstones-enabled-queue-ids".to_string()}
}

pub fn get_lol_statstones_v1_statstones_enabled_queue_ids() -> GetLolStatstonesV1StatstonesEnabledQueueIds {
    GetLolStatstonesV1StatstonesEnabledQueueIds{}
}


pub struct GetLolStatstonesV1VignetteNotifications {}

impl IsApiRequest for GetLolStatstonesV1VignetteNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<Value>;
    fn get_url(&self) -> String {"/lol-statstones/v1/vignette-notifications".to_string()}
}

pub fn get_lol_statstones_v1_vignette_notifications() -> GetLolStatstonesV1VignetteNotifications {
    GetLolStatstonesV1VignetteNotifications{}
}


pub struct GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId {
    pub champion_item_id: i32,
}

impl IsApiRequest for GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesStatstoneSet>;
    fn get_url(&self) -> String {format!("/lol-statstones/v2/player-statstones-self/{}", self.champion_item_id)}
}

pub fn get_lol_statstones_v2_player_statstones_self_by_champion_item_id(champion_item_id: i32) -> GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId {
    GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId{champion_item_id}
}


pub struct GetLolStatstonesV2PlayerSummarySelf {}

impl IsApiRequest for GetLolStatstonesV2PlayerSummarySelf {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesChampionStatstoneSummary>;
    fn get_url(&self) -> String {"/lol-statstones/v2/player-summary-self".to_string()}
}

pub fn get_lol_statstones_v2_player_summary_self() -> GetLolStatstonesV2PlayerSummarySelf {
    GetLolStatstonesV2PlayerSummarySelf{}
}


pub struct PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId {
    pub champion_item_id: i32,
    pub statstone_id: String,
    pub body: LolStatstonesStatstoneFeaturedRequest,
}

impl IsApiRequest for PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-statstones/v1/featured-champion-statstones/{}/{}", self.champion_item_id, self.statstone_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_statstones_v1_featured_champion_statstones_by_champion_item_id_by_statstone_id(champion_item_id: i32, statstone_id: String, body: LolStatstonesStatstoneFeaturedRequest) -> PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId {
    PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId{champion_item_id, statstone_id, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSetSummary {
    pub name: String,
    #[serde(rename = "stonesAvailable")]
    pub stones_available: u32,
    #[serde(rename = "stonesOwned")]
    pub stones_owned: u32,
    #[serde(rename = "stonesIlluminated")]
    pub stones_illuminated: u32,
    #[serde(rename = "milestonesPassed")]
    pub milestones_passed: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSummary {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "stonesAvailable")]
    pub stones_available: u32,
    #[serde(rename = "stonesOwned")]
    pub stones_owned: u32,
    #[serde(rename = "stonesIlluminated")]
    pub stones_illuminated: u32,
    #[serde(rename = "milestonesPassed")]
    pub milestones_passed: u32,
    pub sets: Vec<LolStatstonesChampionStatstoneSetSummary>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesEogNotificationEnvelope {
    #[serde(rename = "selfStatstoneProgress")]
    pub self_statstone_progress: Vec<LolStatstonesStatstoneProgress>,
    #[serde(rename = "selfPersonalBests")]
    pub self_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
    #[serde(rename = "selfMilestoneProgress")]
    pub self_milestone_progress: Vec<LolStatstonesMilestoneProgressNotification>,
    #[serde(rename = "othersPersonalBests")]
    pub others_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstonePack {
    pub name: String,
    pub description: String,
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesMilestoneProgressNotification {
    #[serde(rename = "statstoneId")]
    pub statstone_id: String,
    #[serde(rename = "statstoneName")]
    pub statstone_name: String,
    pub threshold: i32,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub level: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesPersonalBestNotification {
    pub summoner: LolStatstonesSummoner,
    #[serde(rename = "statstoneId")]
    pub statstone_id: String,
    #[serde(rename = "statstoneName")]
    pub statstone_name: String,
    #[serde(rename = "personalBest")]
    pub personal_best: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesPriceInfo {
    pub currency: String,
    pub price: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesProfileStatstoneSummary {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    pub name: String,
    pub value: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub category: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstone {
    pub name: String,
    #[serde(rename = "statstoneId")]
    pub statstone_id: String,
    #[serde(rename = "boundChampionItemId")]
    pub bound_champion_item_id: u32,
    #[serde(rename = "nextMilestone")]
    pub next_milestone: String,
    #[serde(rename = "completionValue")]
    pub completion_value: f32,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
    #[serde(rename = "isFeatured")]
    pub is_featured: bool,
    #[serde(rename = "isEpic")]
    pub is_epic: bool,
    #[serde(rename = "isRetired")]
    pub is_retired: bool,
    pub category: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub description: String,
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "formattedPersonalBest")]
    pub formatted_personal_best: String,
    #[serde(rename = "formattedMilestoneLevel")]
    pub formatted_milestone_level: String,
    #[serde(rename = "playerRecord")]
    pub player_record: Option<LolStatstonesStatstonePlayerRecord>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneFeaturedRequest {
    pub index: i32,
    #[serde(rename = "existingFeatured")]
    pub existing_featured: Vec<LolStatstonesStatstone>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstonePlayerRecord {
    pub puuid: String,
    #[serde(rename = "statstoneId")]
    pub statstone_id: String,
    pub value: u32,
    #[serde(rename = "personalBest")]
    pub personal_best: u32,
    #[serde(rename = "milestoneLevel")]
    pub milestone_level: u32,
    #[serde(rename = "dateAcquired")]
    pub date_acquired: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "dateCompleted")]
    pub date_completed: String,
    #[serde(rename = "dateArchived")]
    pub date_archived: String,
    pub entitled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneProgress {
    #[serde(rename = "statstoneId")]
    pub statstone_id: String,
    #[serde(rename = "statstoneName")]
    pub statstone_name: String,
    pub description: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub delta: String,
    pub value: String,
    #[serde(rename = "nextMilestone")]
    pub next_milestone: String,
    #[serde(rename = "existingProgressPercent")]
    pub existing_progress_percent: String,
    #[serde(rename = "newProgressPercent")]
    pub new_progress_percent: String,
    #[serde(rename = "newMilestoneDifference")]
    pub new_milestone_difference: String,
    #[serde(rename = "totalProgressPercent")]
    pub total_progress_percent: String,
    pub category: String,
    pub level: i32,
    pub best: i32,
    #[serde(rename = "isNewBest")]
    pub is_new_best: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneSet {
    pub name: String,
    pub statstones: Vec<LolStatstonesStatstone>,
    #[serde(rename = "stonesOwned")]
    pub stones_owned: u32,
    #[serde(rename = "milestonesPassed")]
    pub milestones_passed: u32,
    #[serde(rename = "itemId")]
    pub item_id: u32,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "subInventoryType")]
    pub sub_inventory_type: String,
    #[serde(rename = "itemInstanceID")]
    pub item_instance_id: String,
    pub prices: Vec<LolStatstonesPriceInfo>,
    #[serde(rename = "ownedFromPacks")]
    pub owned_from_packs: Vec<LolStatstonesGameDataStatstonePack>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesSummoner {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub puuid: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}


// ENUMS

