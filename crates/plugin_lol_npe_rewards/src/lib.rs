
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolNpeRewardsV1ChallengesProgress {

}

impl IsApiRequest for GetLolNpeRewardsV1ChallengesProgress {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsChallengesProgress;

    fn get_url(&self) -> String {
        "/lol-npe-rewards/v1/challenges/progress".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_rewards_v_1_challenges_progress() -> GetLolNpeRewardsV1ChallengesProgress {
    GetLolNpeRewardsV1ChallengesProgress {
        
    }
}


pub struct GetLolNpeRewardsV1LevelRewards {

}

impl IsApiRequest for GetLolNpeRewardsV1LevelRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeries;

    fn get_url(&self) -> String {
        "/lol-npe-rewards/v1/level-rewards".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_rewards_v_1_level_rewards() -> GetLolNpeRewardsV1LevelRewards {
    GetLolNpeRewardsV1LevelRewards {
        
    }
}


pub struct GetLolNpeRewardsV1LevelRewardsState {

}

impl IsApiRequest for GetLolNpeRewardsV1LevelRewardsState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeriesState;

    fn get_url(&self) -> String {
        "/lol-npe-rewards/v1/level-rewards/state".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_rewards_v_1_level_rewards_state() -> GetLolNpeRewardsV1LevelRewardsState {
    GetLolNpeRewardsV1LevelRewardsState {
        
    }
}


pub struct GetLolNpeRewardsV1LoginRewards {

}

impl IsApiRequest for GetLolNpeRewardsV1LoginRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeries;

    fn get_url(&self) -> String {
        "/lol-npe-rewards/v1/login-rewards".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_rewards_v_1_login_rewards() -> GetLolNpeRewardsV1LoginRewards {
    GetLolNpeRewardsV1LoginRewards {
        
    }
}


pub struct GetLolNpeRewardsV1LoginRewardsState {

}

impl IsApiRequest for GetLolNpeRewardsV1LoginRewardsState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeRewardsRewardSeriesState;

    fn get_url(&self) -> String {
        "/lol-npe-rewards/v1/login-rewards/state".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_rewards_v_1_login_rewards_state() -> GetLolNpeRewardsV1LoginRewardsState {
    GetLolNpeRewardsV1LoginRewardsState {
        
    }
}


pub struct PostLolNpeRewardsV1ChallengesOpt {

}

impl IsApiRequest for PostLolNpeRewardsV1ChallengesOpt {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-npe-rewards/v1/challenges/opt".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_npe_rewards_v_1_challenges_opt() -> PostLolNpeRewardsV1ChallengesOpt {
    PostLolNpeRewardsV1ChallengesOpt {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeriesState {
    pub all_rewards_claimed: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeries {
    pub reward_packs: Vec<LolNpeRewardsRewardPack>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRequirements {
    pub level: u32,
    pub day: u32,
    pub mission_internal_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsChallengesProgress {
    pub progress: LolNpeRewardsProgress,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsProgress {
    pub last_viewed_progress: i32,
    pub current_progress: i32,
    pub total_count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsReward {
    pub renderer: String,
    pub data: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardPack {
    pub index: i32,
    pub type_: String,
    pub requirements: LolNpeRewardsRequirements,
    pub state: String,
    pub premium_reward: bool,
    pub reward_key: String,
    pub major_reward: LolNpeRewardsReward,
    pub minor_rewards: Vec<LolNpeRewardsReward>,
    pub delay: i64,
    pub unlock_time: i64,
}


// ENUMS

