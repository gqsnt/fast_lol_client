
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolProgressionV1GroupsByGroupIdConfiguration {
    pub group_id: String,
}

impl IsApiRequest for GetLolProgressionV1GroupsByGroupIdConfiguration {
    const METHOD: Method = Method::GET;
    type ReturnType = LolProgressionGroup;
    fn get_url(&self) -> String {format!("/lol-progression/v1/groups/{}/configuration", self.group_id)}
}

pub fn get_lol_progression_v1_groups_by_group_id_configuration(group_id: String) -> GetLolProgressionV1GroupsByGroupIdConfiguration {
    GetLolProgressionV1GroupsByGroupIdConfiguration{group_id}
}


pub struct GetLolProgressionV1GroupsByGroupIdInstanceData {
    pub group_id: String,
}

impl IsApiRequest for GetLolProgressionV1GroupsByGroupIdInstanceData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolProgressionEntityInstance;
    fn get_url(&self) -> String {format!("/lol-progression/v1/groups/{}/instanceData", self.group_id)}
}

pub fn get_lol_progression_v1_groups_by_group_id_instance_data(group_id: String) -> GetLolProgressionV1GroupsByGroupIdInstanceData {
    GetLolProgressionV1GroupsByGroupIdInstanceData{group_id}
}


pub struct GetLolProgressionV1GroupsConfiguration {}

impl IsApiRequest for GetLolProgressionV1GroupsConfiguration {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolProgressionGroup>;
    fn get_url(&self) -> String {"/lol-progression/v1/groups/configuration".to_string()}
}

pub fn get_lol_progression_v1_groups_configuration() -> GetLolProgressionV1GroupsConfiguration {
    GetLolProgressionV1GroupsConfiguration{}
}


pub struct GetLolProgressionV1Ready {}

impl IsApiRequest for GetLolProgressionV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-progression/v1/ready".to_string()}
}

pub fn get_lol_progression_v1_ready() -> GetLolProgressionV1Ready {
    GetLolProgressionV1Ready{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounter {
    pub id: String,
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub direction: String,
    #[serde(rename = "startValue")]
    pub start_value: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounterInstance {
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "counterId")]
    pub counter_id: String,
    #[serde(rename = "counterValue")]
    pub counter_value: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionEntityInstance {
    #[serde(rename = "groupId")]
    pub group_id: String,
    pub counters: Vec<LolProgressionCounterInstance>,
    pub milestones: Vec<LolProgressionMilestoneInstance>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionGroup {
    pub id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    pub name: String,
    pub repeat: LolProgressionRepeat,
    pub counters: Vec<LolProgressionCounter>,
    pub milestones: Vec<LolProgressionMilestone>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestone {
    pub id: String,
    pub name: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "counterId")]
    pub counter_id: String,
    #[serde(rename = "triggerValue")]
    pub trigger_value: i64,
    pub properties: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestoneInstance {
    #[serde(rename = "milestoneId")]
    pub milestone_id: String,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "counterId")]
    pub counter_id: String,
    #[serde(rename = "triggerValue")]
    pub trigger_value: i64,
    #[serde(rename = "repeatSequence")]
    pub repeat_sequence: u32,
    pub triggered: bool,
    #[serde(rename = "triggeredTimestamp")]
    pub triggered_timestamp: String,
    pub triggers: Vec<LolProgressionTrigger>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionRepeat {
    pub count: i32,
    pub scope: u32,
    pub multiplier: f32,
    pub milestones: Vec<LolProgressionMilestone>,
    #[serde(rename = "repeatTriggers")]
    pub repeat_triggers: Vec<LolProgressionRepeatGroupTrigger>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionRepeatGroupTrigger {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "counterId")]
    pub counter_id: String,
    #[serde(rename = "startTriggerValue")]
    pub start_trigger_value: u16,
    #[serde(rename = "increaseBy")]
    pub increase_by: u16,
    pub multiplier: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionTrigger {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "counterId")]
    pub counter_id: String,
    #[serde(rename = "triggerValue")]
    pub trigger_value: u64,
}


// ENUMS

