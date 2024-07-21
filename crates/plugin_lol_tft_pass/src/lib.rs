
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolTftPassV1BattlePass {

}

impl IsApiRequest for GetLolTftPassV1BattlePass {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftPassTftBattlepass;

    fn get_url(&self) -> String {
        "/lol-tft-pass/v1/battle-pass".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_pass_v_1_battle_pass() -> GetLolTftPassV1BattlePass {
    GetLolTftPassV1BattlePass {
        
    }
}


pub struct GetLolTftPassV1DailyLoginPass {

}

impl IsApiRequest for GetLolTftPassV1DailyLoginPass {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftPassTftBattlepass;

    fn get_url(&self) -> String {
        "/lol-tft-pass/v1/daily-login-pass".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_pass_v_1_daily_login_pass() -> GetLolTftPassV1DailyLoginPass {
    GetLolTftPassV1DailyLoginPass {
        
    }
}


pub struct GetLolTftPassV1Enabled {

}

impl IsApiRequest for GetLolTftPassV1Enabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-tft-pass/v1/enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_pass_v_1_enabled() -> GetLolTftPassV1Enabled {
    GetLolTftPassV1Enabled {
        
    }
}


pub struct GetLolTftPassV1EventPass {

}

impl IsApiRequest for GetLolTftPassV1EventPass {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftPassTftBattlepass;

    fn get_url(&self) -> String {
        "/lol-tft-pass/v1/event-pass".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_pass_v_1_event_pass() -> GetLolTftPassV1EventPass {
    GetLolTftPassV1EventPass {
        
    }
}


pub struct GetLolTftPassV1RewardNotification {

}

impl IsApiRequest for GetLolTftPassV1RewardNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftPassTftPassRewardNotification;

    fn get_url(&self) -> String {
        "/lol-tft-pass/v1/reward-notification".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_tft_pass_v_1_reward_notification() -> GetLolTftPassV1RewardNotification {
    GetLolTftPassV1RewardNotification {
        
    }
}


pub struct PostLolTftPassV1PassById {

    pub id: String,
}

impl IsApiRequest for PostLolTftPassV1PassById {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-tft-pass/v1/pass/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_tft_pass_v_1_pass_by_id(id: String) -> PostLolTftPassV1PassById {
    PostLolTftPassV1PassById {
        id
    }
}


pub struct PostLolTftPassV1Passes {

}

impl IsApiRequest for PostLolTftPassV1Passes {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-tft-pass/v1/passes".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_tft_pass_v_1_passes() -> PostLolTftPassV1Passes {
    PostLolTftPassV1Passes {
        
    }
}


pub struct PutLolTftPassV1PassByIdMilestoneByMilestoneIdReward {

    pub id: String,
    pub milestone_id: String,
}

impl IsApiRequest for PutLolTftPassV1PassByIdMilestoneByMilestoneIdReward {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-tft-pass/v1/pass/{}/milestone/{}/reward", self.id, self.milestone_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_tft_pass_v_1_pass_by_id_milestone_by_milestone_id_reward(id: String, milestone_id: String) -> PutLolTftPassV1PassByIdMilestoneByMilestoneIdReward {
    PutLolTftPassV1PassByIdMilestoneByMilestoneIdReward {
        id, milestone_id
    }
}


pub struct PutLolTftPassV1PassByIdMilestoneClaimAllRewards {

    pub id: String,
}

impl IsApiRequest for PutLolTftPassV1PassByIdMilestoneClaimAllRewards {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-tft-pass/v1/pass/{}/milestone/claimAllRewards", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_tft_pass_v_1_pass_by_id_milestone_claim_all_rewards(id: String) -> PutLolTftPassV1PassByIdMilestoneClaimAllRewards {
    PutLolTftPassV1PassByIdMilestoneClaimAllRewards {
        id
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftBattlepass {
    pub total_points_earned: i32,
    pub milestones: Vec<LolTftPassTftBattlepassMilestone>,
    pub bonuses: Vec<LolTftPassTftBattlepassMilestone>,
    pub active_milestone: LolTftPassTftBattlepassMilestone,
    pub info: LolTftPassTftBattlepassInfo,
    pub last_viewed_progress: i32,
    pub last_viewed_milestone: LolTftPassTftBattlepassMilestone,
    pub current_level: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftBattlepassInfo {
    pub title: String,
    pub description: String,
    pub start_date: u64,
    pub end_date: u64,
    pub premium: bool,
    pub premium_title: String,
    pub premium_entitlement_id: String,
    pub pc_purchase_requirement: String,
    pub pass_id: String,
    pub media: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftBattlepassMilestone {
    pub milestone_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub points_needed_for_milestone: i32,
    pub points_earned_for_milestone: i32,
    pub total_points_for_milestone: i32,
    pub level: i32,
    pub icon_image_url: String,
    pub icon_needs_frame: bool,
    pub rewards: Vec<LolTftPassTftBattlepassReward>,
    pub is_paid: bool,
    pub is_locked: bool,
    pub is_keystone: bool,
    pub is_bonus: bool,
    pub is_claim_request_pending: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftBattlepassReward {
    pub description: String,
    pub icon_url: String,
    pub icon_needs_frame: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPassRewardNotification {
    pub title: String,
    pub description: String,
    pub icon_url: String,
    pub framed_icon: bool,
}


// ENUMS

