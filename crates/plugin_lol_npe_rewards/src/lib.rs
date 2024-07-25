
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolNpeRewardsV1ChallengesProgress {}

impl IsApiRequest for GetLolNpeRewardsV1ChallengesProgress {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsChallengesProgress;
    fn get_url(&self) -> String {"/lol-npe-rewards/v1/challenges/progress".to_string()}
}

pub fn get_lol_npe_rewards_v1_challenges_progress() -> GetLolNpeRewardsV1ChallengesProgress {
    GetLolNpeRewardsV1ChallengesProgress{}
}


pub struct GetLolNpeRewardsV1LevelRewards {}

impl IsApiRequest for GetLolNpeRewardsV1LevelRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeries;
    fn get_url(&self) -> String {"/lol-npe-rewards/v1/level-rewards".to_string()}
}

pub fn get_lol_npe_rewards_v1_level_rewards() -> GetLolNpeRewardsV1LevelRewards {
    GetLolNpeRewardsV1LevelRewards{}
}


pub struct GetLolNpeRewardsV1LevelRewardsState {}

impl IsApiRequest for GetLolNpeRewardsV1LevelRewardsState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeriesState;
    fn get_url(&self) -> String {"/lol-npe-rewards/v1/level-rewards/state".to_string()}
}

pub fn get_lol_npe_rewards_v1_level_rewards_state() -> GetLolNpeRewardsV1LevelRewardsState {
    GetLolNpeRewardsV1LevelRewardsState{}
}


pub struct GetLolNpeRewardsV1LoginRewards {}

impl IsApiRequest for GetLolNpeRewardsV1LoginRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeries;
    fn get_url(&self) -> String {"/lol-npe-rewards/v1/login-rewards".to_string()}
}

pub fn get_lol_npe_rewards_v1_login_rewards() -> GetLolNpeRewardsV1LoginRewards {
    GetLolNpeRewardsV1LoginRewards{}
}


pub struct GetLolNpeRewardsV1LoginRewardsState {}

impl IsApiRequest for GetLolNpeRewardsV1LoginRewardsState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeriesState;
    fn get_url(&self) -> String {"/lol-npe-rewards/v1/login-rewards/state".to_string()}
}

pub fn get_lol_npe_rewards_v1_login_rewards_state() -> GetLolNpeRewardsV1LoginRewardsState {
    GetLolNpeRewardsV1LoginRewardsState{}
}


pub struct PostLolNpeRewardsV1ChallengesOpt {}

impl IsApiRequest for PostLolNpeRewardsV1ChallengesOpt {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-npe-rewards/v1/challenges/opt".to_string()}
}

pub fn post_lol_npe_rewards_v1_challenges_opt() -> PostLolNpeRewardsV1ChallengesOpt {
    PostLolNpeRewardsV1ChallengesOpt{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsChallengesProgress {
    pub progress: LolNpeRewardsProgress,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsProgress {
    #[serde(rename = "lastViewedProgress")]
    pub last_viewed_progress: i32,
    #[serde(rename = "currentProgress")]
    pub current_progress: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRequirements {
    pub level: u32,
    pub day: u32,
    #[serde(rename = "missionInternalName")]
    pub mission_internal_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsReward {
    pub renderer: String,
    pub data: Value,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardPack {
    pub index: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub requirements: LolNpeRewardsRequirements,
    pub state: String,
    #[serde(rename = "premiumReward")]
    pub premium_reward: bool,
    #[serde(rename = "rewardKey")]
    pub reward_key: String,
    #[serde(rename = "majorReward")]
    pub major_reward: LolNpeRewardsReward,
    #[serde(rename = "minorRewards")]
    pub minor_rewards: Vec<LolNpeRewardsReward>,
    pub delay: i64,
    #[serde(rename = "unlockTime")]
    pub unlock_time: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeries {
    #[serde(rename = "rewardPacks")]
    pub reward_packs: Vec<LolNpeRewardsRewardPack>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeriesState {
    #[serde(rename = "allRewardsClaimed")]
    pub all_rewards_claimed: bool,
}


// ENUMS

