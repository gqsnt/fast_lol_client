
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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

pub fn delete_lol_tft_troves_v1_roll_rewards() -> DeleteLolTftTrovesV1RollRewards {
    DeleteLolTftTrovesV1RollRewards{}
}


pub struct GetLolTftTrovesV1Banners {}

impl IsApiRequest for GetLolTftTrovesV1Banners {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolTftTrovesTrovesBanner>;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/banners".to_string()}
}

pub fn get_lol_tft_troves_v1_banners() -> GetLolTftTrovesV1Banners {
    GetLolTftTrovesV1Banners{}
}


pub struct GetLolTftTrovesV1Config {}

impl IsApiRequest for GetLolTftTrovesV1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTrovesTroves;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/config".to_string()}
}

pub fn get_lol_tft_troves_v1_config() -> GetLolTftTrovesV1Config {
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

pub fn get_lol_tft_troves_v1_loot_odds_by_drop_table_id(drop_table_id: String) -> GetLolTftTrovesV1LootOddsByDropTableId {
    GetLolTftTrovesV1LootOddsByDropTableId{drop_table_id}
}


pub struct GetLolTftTrovesV1MilestoneNotifications {}

impl IsApiRequest for GetLolTftTrovesV1MilestoneNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/milestone-notifications".to_string()}
}

pub fn get_lol_tft_troves_v1_milestone_notifications() -> GetLolTftTrovesV1MilestoneNotifications {
    GetLolTftTrovesV1MilestoneNotifications{}
}


pub struct GetLolTftTrovesV1Milestones {}

impl IsApiRequest for GetLolTftTrovesV1Milestones {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTrovesTrovesMilestones;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/milestones".to_string()}
}

pub fn get_lol_tft_troves_v1_milestones() -> GetLolTftTrovesV1Milestones {
    GetLolTftTrovesV1Milestones{}
}


pub struct GetLolTftTrovesV1RollRewards {}

impl IsApiRequest for GetLolTftTrovesV1RollRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/roll-rewards".to_string()}
}

pub fn get_lol_tft_troves_v1_roll_rewards() -> GetLolTftTrovesV1RollRewards {
    GetLolTftTrovesV1RollRewards{}
}


pub struct GetLolTftTrovesV1StatusNotifications {}

impl IsApiRequest for GetLolTftTrovesV1StatusNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-troves/v1/status-notifications".to_string()}
}

pub fn get_lol_tft_troves_v1_status_notifications() -> GetLolTftTrovesV1StatusNotifications {
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

pub fn post_lol_tft_troves_v1_purchase(body: LolTftTrovesTrovesPurchaseRequest) -> PostLolTftTrovesV1Purchase {
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

pub fn post_lol_tft_troves_v1_roll(body: LolTftTrovesTrovesRollRequest) -> PostLolTftTrovesV1Roll {
    PostLolTftTrovesV1Roll{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesCapOrdersResponseDto {
    pub data: Value,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesLootOddsResponse {
    #[serde(rename = "lootId")]
    pub loot_id: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "dropRate")]
    pub drop_rate: f64,
    pub quantity: i32,
    pub label: String,
    pub query: String,
    #[serde(rename = "displayPriority")]
    pub display_priority: i32,
    pub children: Vec<LolTftTrovesLootOddsResponse>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesPlayerLoot {
    #[serde(rename = "lootName")]
    pub loot_name: String,
    #[serde(rename = "localizedName")]
    pub localized_name: String,
    #[serde(rename = "itemDesc")]
    pub item_desc: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTroves {
    pub enabled: bool,
    #[serde(rename = "useDisplayMetadata")]
    pub use_display_metadata: bool,
    #[serde(rename = "v2Enabled")]
    pub v_2_enabled: bool,
    #[serde(rename = "bannerList")]
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
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "pityLimit")]
    pub pity_limit: u32,
    #[serde(rename = "rollOffer")]
    pub roll_offer: String,
    #[serde(rename = "mythicOffer")]
    pub mythic_offer: String,
    #[serde(rename = "bannerTexture")]
    pub banner_texture: String,
    #[serde(rename = "thumbnailTexture")]
    pub thumbnail_texture: String,
    #[serde(rename = "backgroundTexture")]
    pub background_texture: String,
    #[serde(rename = "platformTexture")]
    pub platform_texture: String,
    #[serde(rename = "eventHubBannerTexture")]
    pub event_hub_banner_texture: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "pityCounterId")]
    pub pity_counter_id: String,
    #[serde(rename = "isCollectorBounty")]
    pub is_collector_bounty: bool,
    #[serde(rename = "maxTotalRolls")]
    pub max_total_rolls: u32,
    #[serde(rename = "pullCost")]
    pub pull_cost: u32,
    #[serde(rename = "chaseContentId")]
    pub chase_content_id: String,
    #[serde(rename = "celebrationTheme")]
    pub celebration_theme: LolTftTrovesTrovesCelebrationThemeData,
    pub status: LolTftTrovesTrovesStatus,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationCurrencySegmentData {
    #[serde(rename = "lottieJsonPath")]
    pub lottie_json_path: String,
    #[serde(rename = "singlePullSoundPath")]
    pub single_pull_sound_path: String,
    #[serde(rename = "multiPullSoundPath")]
    pub multi_pull_sound_path: String,
    #[serde(rename = "mythicPullSoundPath")]
    pub mythic_pull_sound_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationHighlightSegmentData {
    #[serde(rename = "lottieJsonPath")]
    pub lottie_json_path: String,
    #[serde(rename = "revealSoundPath")]
    pub reveal_sound_path: String,
    #[serde(rename = "transitionWipeSoundPath")]
    pub transition_wipe_sound_path: String,
    #[serde(rename = "promiseTokenTitle")]
    pub promise_token_title: String,
    #[serde(rename = "promiseTokenDescription")]
    pub promise_token_description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationPortalSegmentData {
    #[serde(rename = "singlePullRareWebmPath")]
    pub single_pull_rare_webm_path: String,
    #[serde(rename = "singlePullEpicWebmPath")]
    pub single_pull_epic_webm_path: String,
    #[serde(rename = "singlePullLegendaryWebmPath")]
    pub single_pull_legendary_webm_path: String,
    #[serde(rename = "singlePullMythicWebmPath")]
    pub single_pull_mythic_webm_path: String,
    #[serde(rename = "singlePullSoundPath")]
    pub single_pull_sound_path: String,
    #[serde(rename = "multiPullRareWebmPath")]
    pub multi_pull_rare_webm_path: String,
    #[serde(rename = "multiPullEpicWebmPath")]
    pub multi_pull_epic_webm_path: String,
    #[serde(rename = "multiPullLegendaryWebmPath")]
    pub multi_pull_legendary_webm_path: String,
    #[serde(rename = "multiPullMythicWebmPath")]
    pub multi_pull_mythic_webm_path: String,
    #[serde(rename = "multiPullSoundPath")]
    pub multi_pull_sound_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationStandardSegmentData {
    #[serde(rename = "FirstItemTimingOffset")]
    pub first_item_timing_offset: f32,
    #[serde(rename = "InterItemTimingOffset")]
    pub inter_item_timing_offset: f32,
    #[serde(rename = "PCHeaderText")]
    pub pc_header_text: String,
    #[serde(rename = "PCButtonText")]
    pub pc_button_text: String,
    #[serde(rename = "PCBackgroundPath")]
    pub pc_background_path: String,
    #[serde(rename = "PCRewardFramePath")]
    pub pc_reward_frame_path: String,
    #[serde(rename = "PCRewardOneStarPath")]
    pub pc_reward_one_star_path: String,
    #[serde(rename = "PCRewardTwoStarPath")]
    pub pc_reward_two_star_path: String,
    #[serde(rename = "PCRewardThreeStarPath")]
    pub pc_reward_three_star_path: String,
    #[serde(rename = "PCRewardRareGemPath")]
    pub pc_reward_rare_gem_path: String,
    #[serde(rename = "PCRewardEpicGemPath")]
    pub pc_reward_epic_gem_path: String,
    #[serde(rename = "PCRewardLegendaryGemPath")]
    pub pc_reward_legendary_gem_path: String,
    #[serde(rename = "PCRewardMythicGemPath")]
    pub pc_reward_mythic_gem_path: String,
    #[serde(rename = "pullSingleIndividualGlintSoundPath")]
    pub pull_single_individual_glint_sound_path: String,
    #[serde(rename = "pullSingleIndividualGlintLegendarySoundPath")]
    pub pull_single_individual_glint_legendary_sound_path: String,
    #[serde(rename = "revealGlobalSoundPath")]
    pub reveal_global_sound_path: String,
    #[serde(rename = "revealEpicSoundPath")]
    pub reveal_epic_sound_path: String,
    #[serde(rename = "revealMythicSoundPath")]
    pub reveal_mythic_sound_path: String,
    #[serde(rename = "revealRareSoundPath")]
    pub reveal_rare_sound_path: String,
    #[serde(rename = "PCRewardFadeInDuration")]
    pub pc_reward_fade_in_duration: f32,
    #[serde(rename = "PCRewardFadeInDelay")]
    pub pc_reward_fade_in_delay: f32,
    #[serde(rename = "PCThumbnailFadeInDuration")]
    pub pc_thumbnail_fade_in_duration: f32,
    #[serde(rename = "PCThumbnailFadeInDelay")]
    pub pc_thumbnail_fade_in_delay: f32,
    #[serde(rename = "PCRewardSheenPath")]
    pub pc_reward_sheen_path: String,
    #[serde(rename = "PCRewardSheenDuration")]
    pub pc_reward_sheen_duration: f32,
    #[serde(rename = "PCRewardSheenDelay")]
    pub pc_reward_sheen_delay: f32,
    #[serde(rename = "PCGlintSprite")]
    pub pc_glint_sprite: LolTftTrovesTrovesPcSpriteAnimation,
    #[serde(rename = "PCLegendarySparkSprite")]
    pub pc_legendary_spark_sprite: LolTftTrovesTrovesPcSpriteAnimation,
    #[serde(rename = "PCLegendaryHitSprite")]
    pub pc_legendary_hit_sprite: LolTftTrovesTrovesPcSpriteAnimation,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationThemeData {
    #[serde(rename = "currencySegmentData")]
    pub currency_segment_data: LolTftTrovesTrovesCelebrationCurrencySegmentData,
    #[serde(rename = "portalSegmentData")]
    pub portal_segment_data: LolTftTrovesTrovesCelebrationPortalSegmentData,
    #[serde(rename = "highlightSegmentData")]
    pub highlight_segment_data: LolTftTrovesTrovesCelebrationHighlightSegmentData,
    #[serde(rename = "standardSegmentData")]
    pub standard_segment_data: LolTftTrovesTrovesCelebrationStandardSegmentData,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesMilestone {
    #[serde(rename = "milestoneId")]
    pub milestone_id: String,
    #[serde(rename = "currencyId")]
    pub currency_id: String,
    #[serde(rename = "currencyAmount")]
    pub currency_amount: u32,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "triggerValue")]
    pub trigger_value: u64,
    #[serde(rename = "repeatSequence")]
    pub repeat_sequence: u32,
    #[serde(rename = "triggeredTimestamp")]
    pub triggered_timestamp: String,
    pub triggered: bool,
    pub name: String,
    #[serde(rename = "iconURL")]
    pub icon_url: String,
    #[serde(rename = "resetValue")]
    pub reset_value: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesMilestoneCounter {
    #[serde(rename = "counterId")]
    pub counter_id: String,
    #[serde(rename = "counterValue")]
    pub counter_value: u64,
    #[serde(rename = "startTriggerValue")]
    pub start_trigger_value: u16,
    #[serde(rename = "increaseBy")]
    pub increase_by: u16,
    pub multiplier: f32,
    #[serde(rename = "resetValue")]
    pub reset_value: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesMilestones {
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub name: String,
    pub milestones: Vec<LolTftTrovesTrovesMilestone>,
    pub counter: LolTftTrovesTrovesMilestoneCounter,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesPcSpriteAnimation {
    #[serde(rename = "spritesheetPath")]
    pub spritesheet_path: String,
    pub duration: f32,
    pub delay: f32,
    #[serde(rename = "numRows")]
    pub num_rows: u32,
    #[serde(rename = "numCols")]
    pub num_cols: u32,
    #[serde(rename = "numFrames")]
    pub num_frames: u32,
    #[serde(rename = "startFrame")]
    pub start_frame: u32,
    pub fps: u32,
    #[serde(rename = "maxPlayCount")]
    pub max_play_count: i32,
    #[serde(rename = "playAtInsert")]
    pub play_at_insert: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesPurchaseRequest {
    #[serde(rename = "offerId")]
    pub offer_id: String,
    pub quantity: u32,
    #[serde(rename = "paymentOption")]
    pub payment_option: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesRollRequest {
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "numberOfRolls")]
    pub number_of_rolls: u32,
    #[serde(rename = "isMythic")]
    pub is_mythic: bool,
    #[serde(rename = "dropTableId")]
    pub drop_table_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesStatus {
    pub owned: bool,
    #[serde(rename = "availableContents")]
    pub available_contents: u32,
    #[serde(rename = "pityCount")]
    pub pity_count: u8,
    #[serde(rename = "dropTableId")]
    pub drop_table_id: String,
    #[serde(rename = "hasPullError")]
    pub has_pull_error: bool,
    #[serde(rename = "totalRollsCount")]
    pub total_rolls_count: u16,
    #[serde(rename = "isCollectorBountyMaxRollsMet")]
    pub is_collector_bounty_max_rolls_met: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesVerboseLootOddsResponse {
    #[serde(rename = "recipeName")]
    pub recipe_name: String,
    #[serde(rename = "chanceToContain")]
    pub chance_to_contain: Vec<LolTftTrovesLootOddsResponse>,
    #[serde(rename = "guaranteedToContain")]
    pub guaranteed_to_contain: Vec<LolTftTrovesLootOddsResponse>,
    #[serde(rename = "lootItem")]
    pub loot_item: LolTftTrovesPlayerLoot,
    #[serde(rename = "hasPityRules")]
    pub has_pity_rules: bool,
    #[serde(rename = "checksOwnership")]
    pub checks_ownership: bool,
}


// ENUMS

