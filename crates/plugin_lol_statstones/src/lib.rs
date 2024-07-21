
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolStatstonesV1VignetteNotifications {

}

impl IsApiRequest for GetLolStatstonesV1VignetteNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<HashMap<String, String>>;

    fn get_url(&self) -> String {
        "/lol-statstones/v1/vignette-notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_1_vignette_notifications() -> GetLolStatstonesV1VignetteNotifications {
    GetLolStatstonesV1VignetteNotifications {
        
    }
}


pub struct DeleteLolStatstonesV1VignetteNotifications {

}

impl IsApiRequest for DeleteLolStatstonesV1VignetteNotifications {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-statstones/v1/vignette-notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_statstones_v_1_vignette_notifications() -> DeleteLolStatstonesV1VignetteNotifications {
    DeleteLolStatstonesV1VignetteNotifications {
        
    }
}


pub struct DeleteLolStatstonesV1VignetteNotificationsByKey {

    pub key: i32,
}

impl IsApiRequest for DeleteLolStatstonesV1VignetteNotificationsByKey {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-statstones/v1/vignette-notifications/{}", self.key)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_statstones_v_1_vignette_notifications_by_key(key: i32) -> DeleteLolStatstonesV1VignetteNotificationsByKey {
    DeleteLolStatstonesV1VignetteNotificationsByKey {
        key
    }
}


pub struct GetLolStatstonesV1EogNotificationsByGameId {

    pub game_id: u64,
}

impl IsApiRequest for GetLolStatstonesV1EogNotificationsByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStatstonesEogNotificationEnvelope;

    fn get_url(&self) -> String {
        format!("/lol-statstones/v1/eog-notifications/{}", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_1_eog_notifications_by_game_id(game_id: u64) -> GetLolStatstonesV1EogNotificationsByGameId {
    GetLolStatstonesV1EogNotificationsByGameId {
        game_id
    }
}


pub struct GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId {

    pub champion_item_id: i32,
}

impl IsApiRequest for GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesStatstone>;

    fn get_url(&self) -> String {
        format!("/lol-statstones/v1/featured-champion-statstones/{}", self.champion_item_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_1_featured_champion_statstones_by_champion_item_id(champion_item_id: i32) -> GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId {
    GetLolStatstonesV1FeaturedChampionStatstonesByChampionItemId {
        champion_item_id
    }
}


pub struct GetLolStatstonesV1ProfileSummaryByPuuid {

    pub puuid: String,
}

impl IsApiRequest for GetLolStatstonesV1ProfileSummaryByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesProfileStatstoneSummary>;

    fn get_url(&self) -> String {
        format!("/lol-statstones/v1/profile-summary/{}", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_1_profile_summary_by_puuid(puuid: String) -> GetLolStatstonesV1ProfileSummaryByPuuid {
    GetLolStatstonesV1ProfileSummaryByPuuid {
        puuid
    }
}


pub struct GetLolStatstonesV1StatstoneByContentIdOwned {

    pub content_id: String,
}

impl IsApiRequest for GetLolStatstonesV1StatstoneByContentIdOwned {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        format!("/lol-statstones/v1/statstone/{}/owned", self.content_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_1_statstone_by_content_id_owned(content_id: String) -> GetLolStatstonesV1StatstoneByContentIdOwned {
    GetLolStatstonesV1StatstoneByContentIdOwned {
        content_id
    }
}


pub struct GetLolStatstonesV1StatstonesEnabledQueueIds {

}

impl IsApiRequest for GetLolStatstonesV1StatstonesEnabledQueueIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u32>;

    fn get_url(&self) -> String {
        "/lol-statstones/v1/statstones-enabled-queue-ids".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_1_statstones_enabled_queue_ids() -> GetLolStatstonesV1StatstonesEnabledQueueIds {
    GetLolStatstonesV1StatstonesEnabledQueueIds {
        
    }
}


pub struct GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId {

    pub champion_item_id: i32,
}

impl IsApiRequest for GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesStatstoneSet>;

    fn get_url(&self) -> String {
        format!("/lol-statstones/v2/player-statstones-self/{}", self.champion_item_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_2_player_statstones_self_by_champion_item_id(champion_item_id: i32) -> GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId {
    GetLolStatstonesV2PlayerStatstonesSelfByChampionItemId {
        champion_item_id
    }
}


pub struct GetLolStatstonesV2PlayerSummarySelf {

}

impl IsApiRequest for GetLolStatstonesV2PlayerSummarySelf {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStatstonesChampionStatstoneSummary>;

    fn get_url(&self) -> String {
        "/lol-statstones/v2/player-summary-self".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_statstones_v_2_player_summary_self() -> GetLolStatstonesV2PlayerSummarySelf {
    GetLolStatstonesV2PlayerSummarySelf {
        
    }
}


pub struct PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId {

    pub champion_item_id: i32,
    pub statstone_id: String,
    pub body: LolStatstonesStatstoneFeaturedRequest,
}

impl IsApiRequest for PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-statstones/v1/featured-champion-statstones/{}/{}", self.champion_item_id, self.statstone_id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_statstones_v_1_featured_champion_statstones_by_champion_item_id_by_statstone_id(champion_item_id: i32, statstone_id: String, body: LolStatstonesStatstoneFeaturedRequest) -> PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId {
    PostLolStatstonesV1FeaturedChampionStatstonesByChampionItemIdByStatstoneId {
        champion_item_id, statstone_id, body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstone {
    pub name: String,
    pub statstone_id: String,
    pub bound_champion_item_id: u32,
    pub next_milestone: String,
    pub completion_value: f32,
    pub is_complete: bool,
    pub is_featured: bool,
    pub is_epic: bool,
    pub is_retired: bool,
    pub category: String,
    pub image_url: String,
    pub description: String,
    pub formatted_value: String,
    pub formatted_personal_best: String,
    pub formatted_milestone_level: String,
    pub player_record: Option<LolStatstonesStatstonePlayerRecord>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesPersonalBestNotification {
    pub summoner: LolStatstonesSummoner,
    pub statstone_id: String,
    pub statstone_name: String,
    pub personal_best: String,
    pub image_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstonePlayerRecord {
    pub puuid: String,
    pub statstone_id: String,
    pub value: u32,
    pub personal_best: u32,
    pub milestone_level: u32,
    pub date_acquired: String,
    pub date_modified: String,
    pub date_completed: String,
    pub date_archived: String,
    pub entitled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesSummoner {
    pub summoner_id: u64,
    pub puuid: String,
    pub display_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesEogNotificationEnvelope {
    pub self_statstone_progress: Vec<LolStatstonesStatstoneProgress>,
    pub self_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
    pub self_milestone_progress: Vec<LolStatstonesMilestoneProgressNotification>,
    pub others_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneSet {
    pub name: String,
    pub statstones: Vec<LolStatstonesStatstone>,
    pub stones_owned: u32,
    pub milestones_passed: u32,
    pub item_id: u32,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub item_instance_id: String,
    pub prices: Vec<LolStatstonesPriceInfo>,
    pub owned_from_packs: Vec<LolStatstonesGameDataStatstonePack>,
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
    pub champion_id: i32,
    pub name: String,
    pub value: String,
    pub image_url: String,
    pub category: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSummary {
    pub champion_id: i32,
    pub stones_available: u32,
    pub stones_owned: u32,
    pub stones_illuminated: u32,
    pub milestones_passed: u32,
    pub sets: Vec<LolStatstonesChampionStatstoneSetSummary>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstonePack {
    pub name: String,
    pub description: String,
    pub content_id: String,
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneFeaturedRequest {
    pub index: i32,
    pub existing_featured: Vec<LolStatstonesStatstone>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSetSummary {
    pub name: String,
    pub stones_available: u32,
    pub stones_owned: u32,
    pub stones_illuminated: u32,
    pub milestones_passed: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesMilestoneProgressNotification {
    pub statstone_id: String,
    pub statstone_name: String,
    pub threshold: i32,
    pub image_url: String,
    pub level: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneProgress {
    pub statstone_id: String,
    pub statstone_name: String,
    pub description: String,
    pub image_url: String,
    pub delta: String,
    pub value: String,
    pub next_milestone: String,
    pub existing_progress_percent: String,
    pub new_progress_percent: String,
    pub new_milestone_difference: String,
    pub total_progress_percent: String,
    pub category: String,
    pub level: i32,
    pub best: i32,
    pub is_new_best: bool,
}


// ENUMS

