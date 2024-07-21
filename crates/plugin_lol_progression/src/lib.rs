
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolProgressionV1GroupsByGroupIdConfiguration {

    pub group_id: String,
}

impl IsApiRequest for GetLolProgressionV1GroupsByGroupIdConfiguration {
    const METHOD: Method = Method::GET;
    type ReturnType = LolProgressionGroup;

    fn get_url(&self) -> String {
        format!("/lol-progression/v1/groups/{}/configuration", self.group_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_progression_v_1_groups_by_group_id_configuration(group_id: String) -> GetLolProgressionV1GroupsByGroupIdConfiguration {
    GetLolProgressionV1GroupsByGroupIdConfiguration {
        group_id
    }
}


pub struct GetLolProgressionV1GroupsByGroupIdInstanceData {

    pub group_id: String,
}

impl IsApiRequest for GetLolProgressionV1GroupsByGroupIdInstanceData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolProgressionEntityInstance;

    fn get_url(&self) -> String {
        format!("/lol-progression/v1/groups/{}/instanceData", self.group_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_progression_v_1_groups_by_group_id_instance_data(group_id: String) -> GetLolProgressionV1GroupsByGroupIdInstanceData {
    GetLolProgressionV1GroupsByGroupIdInstanceData {
        group_id
    }
}


pub struct GetLolProgressionV1GroupsConfiguration {

}

impl IsApiRequest for GetLolProgressionV1GroupsConfiguration {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolProgressionGroup>;

    fn get_url(&self) -> String {
        "/lol-progression/v1/groups/configuration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_progression_v_1_groups_configuration() -> GetLolProgressionV1GroupsConfiguration {
    GetLolProgressionV1GroupsConfiguration {
        
    }
}


pub struct GetLolProgressionV1Ready {

}

impl IsApiRequest for GetLolProgressionV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-progression/v1/ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_progression_v_1_ready() -> GetLolProgressionV1Ready {
    GetLolProgressionV1Ready {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounter {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub direction: String,
    pub start_value: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounterInstance {
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub counter_value: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionEntityInstance {
    pub group_id: String,
    pub counters: Vec<LolProgressionCounterInstance>,
    pub milestones: Vec<LolProgressionMilestoneInstance>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestone {
    pub id: String,
    pub name: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub properties: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionRepeatGroupTrigger {
    pub type_: String,
    pub counter_id: String,
    pub start_trigger_value: u16,
    pub increase_by: u16,
    pub multiplier: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionRepeat {
    pub count: i32,
    pub scope: u32,
    pub multiplier: f32,
    pub milestones: Vec<LolProgressionMilestone>,
    pub repeat_triggers: Vec<LolProgressionRepeatGroupTrigger>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionGroup {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub repeat: LolProgressionRepeat,
    pub counters: Vec<LolProgressionCounter>,
    pub milestones: Vec<LolProgressionMilestone>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestoneInstance {
    pub milestone_id: String,
    pub instance_id: String,
    pub owner_id: String,
    pub product_id: String,
    pub group_id: String,
    pub counter_id: String,
    pub trigger_value: i64,
    pub repeat_sequence: u32,
    pub triggered: bool,
    pub triggered_timestamp: String,
    pub triggers: Vec<LolProgressionTrigger>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionTrigger {
    pub type_: String,
    pub counter_id: String,
    pub trigger_value: u64,
}


// ENUMS

