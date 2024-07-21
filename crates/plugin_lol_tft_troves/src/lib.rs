
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolTftTrovesV1RollRewards {}

impl IsApiRequest for DeleteLolTftTrovesV1RollRewards {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/roll-rewards".to_string()}
}

pub fn delete_lol_tft_troves_v_1_roll_rewards() -> DeleteLolTftTrovesV1RollRewards {
    DeleteLolTftTrovesV1RollRewards{}
}


pub struct GetLolTftTrovesV1Banners {}

impl IsApiRequest for GetLolTftTrovesV1Banners {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolTftTrovesTrovesBanner>;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/banners".to_string()}
}

pub fn get_lol_tft_troves_v_1_banners() -> GetLolTftTrovesV1Banners {
    GetLolTftTrovesV1Banners{}
}


pub struct GetLolTftTrovesV1Config {}

impl IsApiRequest for GetLolTftTrovesV1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTrovesTroves;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/config".to_string()}
}

pub fn get_lol_tft_troves_v_1_config() -> GetLolTftTrovesV1Config {
    GetLolTftTrovesV1Config{}
}


pub struct GetLolTftTrovesV1LootOddsByDropTableId {
    pub drop_table_id: String,
}

impl IsApiRequest for GetLolTftTrovesV1LootOddsByDropTableId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTrovesVerboseLootOddsResponse;
    fn get_url(&self) -> String {format!("/lol-tft-troves/v1/loot-odds/{}", self.drop_table_id)}
}

pub fn get_lol_tft_troves_v_1_loot_odds_by_drop_table_id(drop_table_id: String) -> GetLolTftTrovesV1LootOddsByDropTableId {
    GetLolTftTrovesV1LootOddsByDropTableId{drop_table_id}
}


pub struct GetLolTftTrovesV1MilestoneNotifications {}

impl IsApiRequest for GetLolTftTrovesV1MilestoneNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/milestone-notifications".to_string()}
}

pub fn get_lol_tft_troves_v_1_milestone_notifications() -> GetLolTftTrovesV1MilestoneNotifications {
    GetLolTftTrovesV1MilestoneNotifications{}
}


pub struct GetLolTftTrovesV1Milestones {}

impl IsApiRequest for GetLolTftTrovesV1Milestones {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTrovesTrovesMilestones;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/milestones".to_string()}
}

pub fn get_lol_tft_troves_v_1_milestones() -> GetLolTftTrovesV1Milestones {
    GetLolTftTrovesV1Milestones{}
}


pub struct GetLolTftTrovesV1RollRewards {}

impl IsApiRequest for GetLolTftTrovesV1RollRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/roll-rewards".to_string()}
}

pub fn get_lol_tft_troves_v_1_roll_rewards() -> GetLolTftTrovesV1RollRewards {
    GetLolTftTrovesV1RollRewards{}
}


pub struct GetLolTftTrovesV1StatusNotifications {}

impl IsApiRequest for GetLolTftTrovesV1StatusNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/status-notifications".to_string()}
}

pub fn get_lol_tft_troves_v_1_status_notifications() -> GetLolTftTrovesV1StatusNotifications {
    GetLolTftTrovesV1StatusNotifications{}
}


pub struct PostLolTftTrovesV1Purchase {
    pub body: LolTftTrovesTrovesPurchaseRequest,
}

impl IsApiRequest for PostLolTftTrovesV1Purchase {
    const METHOD: Method = Method::POST;
    type ReturnType = LolTftTrovesCapOrdersResponseDto;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/purchase".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_tft_troves_v_1_purchase(body: LolTftTrovesTrovesPurchaseRequest) -> PostLolTftTrovesV1Purchase {
    PostLolTftTrovesV1Purchase{body}
}


pub struct PostLolTftTrovesV1Roll {
    pub body: LolTftTrovesTrovesRollRequest,
}

impl IsApiRequest for PostLolTftTrovesV1Roll {
    const METHOD: Method = Method::POST;
    type ReturnType = LolTftTrovesCapOrdersResponseDto;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/roll".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_tft_troves_v_1_roll(body: LolTftTrovesTrovesRollRequest) -> PostLolTftTrovesV1Roll {
    PostLolTftTrovesV1Roll{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesCapOrdersResponseDto {
    pub data: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesLootOddsResponse {
    pub loot_id: String,
    pub parent_id: String,
    pub drop_rate: f64,
    pub quantity: i32,
    pub label: String,
    pub query: String,
    pub display_priority: i32,
    pub children: Vec<LolTftTrovesLootOddsResponse>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesPlayerLoot {
    pub loot_name: String,
    pub localized_name: String,
    pub item_desc: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTroves {
    pub enabled: bool,
    pub use_display_metadata: bool,
    pub v_2_enabled: bool,
    pub banner_list: Option<Vec<LolTftTrovesTrovesActiveBanner>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesActiveBanner {
    pub id: String,
    pub version: u8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesBanner {
    pub id: String,
    pub version: u8,
    pub source_id: String,
    pub start_date: String,
    pub end_date: String,
    pub pity_limit: u32,
    pub roll_offer: String,
    pub mythic_offer: String,
    pub banner_texture: String,
    pub thumbnail_texture: String,
    pub background_texture: String,
    pub platform_texture: String,
    pub event_hub_banner_texture: String,
    pub name: String,
    pub description: String,
    pub pity_counter_id: String,
    pub is_collector_bounty: bool,
    pub max_total_rolls: u32,
    pub pull_cost: u32,
    pub chase_content_id: String,
    pub celebration_theme: LolTftTrovesTrovesCelebrationThemeData,
    pub status: LolTftTrovesTrovesStatus,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationCurrencySegmentData {
    pub lottie_json_path: String,
    pub single_pull_sound_path: String,
    pub multi_pull_sound_path: String,
    pub mythic_pull_sound_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationHighlightSegmentData {
    pub lottie_json_path: String,
    pub reveal_sound_path: String,
    pub transition_wipe_sound_path: String,
    pub promise_token_title: String,
    pub promise_token_description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationPortalSegmentData {
    pub single_pull_rare_webm_path: String,
    pub single_pull_epic_webm_path: String,
    pub single_pull_legendary_webm_path: String,
    pub single_pull_mythic_webm_path: String,
    pub single_pull_sound_path: String,
    pub multi_pull_rare_webm_path: String,
    pub multi_pull_epic_webm_path: String,
    pub multi_pull_legendary_webm_path: String,
    pub multi_pull_mythic_webm_path: String,
    pub multi_pull_sound_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationStandardSegmentData {
    pub first_item_timing_offset: f32,
    pub inter_item_timing_offset: f32,
    pub pc_header_text: String,
    pub pc_button_text: String,
    pub pc_background_path: String,
    pub pc_reward_frame_path: String,
    pub pc_reward_one_star_path: String,
    pub pc_reward_two_star_path: String,
    pub pc_reward_three_star_path: String,
    pub pc_reward_rare_gem_path: String,
    pub pc_reward_epic_gem_path: String,
    pub pc_reward_legendary_gem_path: String,
    pub pc_reward_mythic_gem_path: String,
    pub pull_single_individual_glint_sound_path: String,
    pub pull_single_individual_glint_legendary_sound_path: String,
    pub reveal_global_sound_path: String,
    pub reveal_epic_sound_path: String,
    pub reveal_mythic_sound_path: String,
    pub reveal_rare_sound_path: String,
    pub pc_reward_fade_in_duration: f32,
    pub pc_reward_fade_in_delay: f32,
    pub pc_thumbnail_fade_in_duration: f32,
    pub pc_thumbnail_fade_in_delay: f32,
    pub pc_reward_sheen_path: String,
    pub pc_reward_sheen_duration: f32,
    pub pc_reward_sheen_delay: f32,
    pub pc_glint_sprite: LolTftTrovesTrovesPcSpriteAnimation,
    pub pc_legendary_spark_sprite: LolTftTrovesTrovesPcSpriteAnimation,
    pub pc_legendary_hit_sprite: LolTftTrovesTrovesPcSpriteAnimation,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationThemeData {
    pub currency_segment_data: LolTftTrovesTrovesCelebrationCurrencySegmentData,
    pub portal_segment_data: LolTftTrovesTrovesCelebrationPortalSegmentData,
    pub highlight_segment_data: LolTftTrovesTrovesCelebrationHighlightSegmentData,
    pub standard_segment_data: LolTftTrovesTrovesCelebrationStandardSegmentData,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesMilestone {
    pub milestone_id: String,
    pub currency_id: String,
    pub currency_amount: u32,
    pub instance_id: String,
    pub trigger_value: u64,
    pub repeat_sequence: u32,
    pub triggered_timestamp: String,
    pub triggered: bool,
    pub name: String,
    pub icon_url: String,
    pub reset_value: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesMilestoneCounter {
    pub counter_id: String,
    pub counter_value: u64,
    pub start_trigger_value: u16,
    pub increase_by: u16,
    pub multiplier: f32,
    pub reset_value: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesMilestones {
    pub group_id: String,
    pub name: String,
    pub milestones: Vec<LolTftTrovesTrovesMilestone>,
    pub counter: LolTftTrovesTrovesMilestoneCounter,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesPcSpriteAnimation {
    pub spritesheet_path: String,
    pub duration: f32,
    pub delay: f32,
    pub num_rows: u32,
    pub num_cols: u32,
    pub num_frames: u32,
    pub start_frame: u32,
    pub fps: u32,
    pub max_play_count: i32,
    pub play_at_insert: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesPurchaseRequest {
    pub offer_id: String,
    pub quantity: u32,
    pub payment_option: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesRollRequest {
    pub offer_id: String,
    pub number_of_rolls: u32,
    pub is_mythic: bool,
    pub drop_table_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesStatus {
    pub owned: bool,
    pub available_contents: u32,
    pub pity_count: u8,
    pub drop_table_id: String,
    pub has_pull_error: bool,
    pub total_rolls_count: u16,
    pub is_collector_bounty_max_rolls_met: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesVerboseLootOddsResponse {
    pub recipe_name: String,
    pub chance_to_contain: Vec<LolTftTrovesLootOddsResponse>,
    pub guaranteed_to_contain: Vec<LolTftTrovesLootOddsResponse>,
    pub loot_item: LolTftTrovesPlayerLoot,
    pub has_pity_rules: bool,
    pub checks_ownership: bool,
}


// ENUMS

