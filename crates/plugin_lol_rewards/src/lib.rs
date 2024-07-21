
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolRewardsV1Grants {

    pub status: Option<LolRewardsGrantStatus>,
}

impl IsApiRequest for GetLolRewardsV1Grants {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRewardsRewardGrant>;

    fn get_url(&self) -> String {
        "/lol-rewards/v1/grants".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "status" : self.status,
        }))
    }
}

pub fn get_lol_rewards_v_1_grants(status: Option<LolRewardsGrantStatus>) -> GetLolRewardsV1Grants {
    GetLolRewardsV1Grants {
        status
    }
}


pub struct GetLolRewardsV1Groups {

    pub types: Option<Vec<String>>,
}

impl IsApiRequest for GetLolRewardsV1Groups {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRewardsSvcRewardGroup>;

    fn get_url(&self) -> String {
        "/lol-rewards/v1/groups".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "types" : self.types,
        }))
    }
}

pub fn get_lol_rewards_v_1_groups(types: Option<Vec<String>>) -> GetLolRewardsV1Groups {
    GetLolRewardsV1Groups {
        types
    }
}


pub struct PatchLolRewardsV1GrantsView {

    pub body: Vec<String>,
}

impl IsApiRequest for PatchLolRewardsV1GrantsView {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-rewards/v1/grants/view".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn patch_lol_rewards_v_1_grants_view(body: Vec<String>) -> PatchLolRewardsV1GrantsView {
    PatchLolRewardsV1GrantsView {
        body
    }
}


pub struct PostLolRewardsV1GrantsByGrantIdSelect {

    pub grant_id: String,
    pub body: LolRewardsSelectionRequestDto,
}

impl IsApiRequest for PostLolRewardsV1GrantsByGrantIdSelect {
    const METHOD: Method = Method::POST;
    type ReturnType = LolRewardsRewardGrant;

    fn get_url(&self) -> String {
        format!("/lol-rewards/v1/grants/{}/select", self.grant_id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rewards_v_1_grants_by_grant_id_select(grant_id: String, body: LolRewardsSelectionRequestDto) -> PostLolRewardsV1GrantsByGrantIdSelect {
    PostLolRewardsV1GrantsByGrantIdSelect {
        grant_id, body
    }
}


pub struct PostLolRewardsV1RewardReplay {

    pub body: String,
}

impl IsApiRequest for PostLolRewardsV1RewardReplay {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-rewards/v1/reward/replay".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rewards_v_1_reward_replay(body: String) -> PostLolRewardsV1RewardReplay {
    PostLolRewardsV1RewardReplay {
        body
    }
}


pub struct PostLolRewardsV1SelectBulk {

    pub body: Vec<LolRewardsSelectionRequestDto>,
}

impl IsApiRequest for PostLolRewardsV1SelectBulk {
    const METHOD: Method = Method::POST;
    type ReturnType = LolRewardsSelectGrantStatusResponse;

    fn get_url(&self) -> String {
        "/lol-rewards/v1/select-bulk".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_rewards_v_1_select_bulk(body: Vec<LolRewardsSelectionRequestDto>) -> PostLolRewardsV1SelectBulk {
    PostLolRewardsV1SelectBulk {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRewardGrant {
    pub info: LolRewardsSvcRewardGrant,
    pub reward_group: LolRewardsSvcRewardGroup,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGrant {
    pub id: String,
    pub grantee_id: String,
    pub reward_group_id: String,
    pub date_created: String,
    pub status: LolRewardsGrantStatus,
    pub grant_elements: Vec<LolRewardsSvcRewardGrantElement>,
    pub selected_ids: Vec<String>,
    pub viewed: bool,
    pub grantor_description: LolRewardsGrantorDescription,
    pub message_parameters: HashMap<String, HashMap<String, String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSelectionRequestDto {
    pub grant_id: String,
    pub reward_group_id: String,
    pub selections: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGroup {
    pub id: String,
    pub product_id: String,
    pub types: Vec<String>,
    pub rewards: Vec<LolRewardsReward>,
    pub child_reward_group_ids: Vec<String>,
    pub reward_strategy: LolRewardsRewardStrategy,
    pub selection_strategy_config: Option<LolRewardsSelectionStrategyConfig>,
    pub active: bool,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
    pub celebration_type: LolRewardsCelebrationType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsGrantorDescription {
    pub app_name: String,
    pub entity_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsReward {
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub quantity: i32,
    pub fulfillment_source: String,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSelectionStrategyConfig {
    pub min_selections_allowed: u32,
    pub max_selections_allowed: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGrantElement {
    pub element_id: String,
    pub item_id: String,
    pub item_type: String,
    pub fulfillment_source: String,
    pub status: LolRewardsRewardStatus,
    pub quantity: i32,
    pub media: HashMap<String, String>,
    pub localizations: HashMap<String, String>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRewardsGrantStatus {
    #[default]
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "FULFILLED")]
    Fulfilled,
    #[serde(rename = "PENDING_SELECTION")]
    PendingSelection,
    #[serde(rename = "PENDING_FULFILLMENT")]
    PendingFulfillment,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRewardsCelebrationType {
    #[default]
    #[serde(rename = "FULLSCREEN")]
    Fullscreen,
    #[serde(rename = "TOAST")]
    Toast,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRewardsRewardStatus {
    #[default]
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "FULFILLED")]
    Fulfilled,
    #[serde(rename = "PENDING")]
    Pending,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRewardsSelectGrantStatusResponse {
    #[default]
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "SELECTED")]
    Selected,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRewardsRewardStrategy {
    #[default]
    #[serde(rename = "SELECTION")]
    Selection,
    #[serde(rename = "RANDOM")]
    Random,
    #[serde(rename = "ALL")]
    All,
}

