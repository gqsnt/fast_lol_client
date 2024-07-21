
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetAntiAddictionV1PoliciesByPolicyTypeAntiAddictionState {

    pub policy_type: LolAntiAddictionPolicyType,
}

impl IsApiRequest for GetAntiAddictionV1PoliciesByPolicyTypeAntiAddictionState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolAntiAddictionAntiAddictionState;

    fn get_url(&self) -> String {
        format!("/anti-addiction/v1/policies/{}/anti-addiction-state", serde_json::to_string(&self.policy_type).unwrap())
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_anti_addiction_v_1_policies_by_policy_type_anti_addiction_state(policy_type: LolAntiAddictionPolicyType) -> GetAntiAddictionV1PoliciesByPolicyTypeAntiAddictionState {
    GetAntiAddictionV1PoliciesByPolicyTypeAntiAddictionState {
        policy_type
    }
}


pub struct GetLolAntiAddictionV1AntiAddictionToken {

}

impl IsApiRequest for GetLolAntiAddictionV1AntiAddictionToken {
    const METHOD: Method = Method::GET;
    type ReturnType = LolAntiAddictionAntiAddictionToken;

    fn get_url(&self) -> String {
        "/lol-anti-addiction/v1/anti-addiction-token".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_anti_addiction_v_1_anti_addiction_token() -> GetLolAntiAddictionV1AntiAddictionToken {
    GetLolAntiAddictionV1AntiAddictionToken {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAntiAddictionAntiAddictionToken {
    pub anti_addiction_token: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolAntiAddictionAntiAddictionState {
    pub policy_type: LolAntiAddictionPolicyType,
    pub localization_key: String,
    pub anti_addiction_token: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolAntiAddictionPolicyType {
    #[default]
    #[serde(rename = "antiAddictionHeartbeat")]
    AntiAddictionHeartbeat,
    #[serde(rename = "antiAddictionShutdown")]
    AntiAddictionShutdown,
    #[serde(rename = "antiAddictionWarning")]
    AntiAddictionWarning,
}

