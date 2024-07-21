
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolRmsV1ChampionMasteryLeaveupUpdateById {
    pub id: u64,
}

impl IsApiRequest for DeleteLolRmsV1ChampionMasteryLeaveupUpdateById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-rms/v1/champion-mastery-leaveup-update/{}", self.id)}
}

pub fn delete_lol_rms_v_1_champion_mastery_leaveup_update_by_id(id: u64) -> DeleteLolRmsV1ChampionMasteryLeaveupUpdateById {
    DeleteLolRmsV1ChampionMasteryLeaveupUpdateById{id}
}


pub struct GetLolRmsV1ChampionMasteryLeaveupUpdate {}

impl IsApiRequest for GetLolRmsV1ChampionMasteryLeaveupUpdate {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRiotMessagingServiceChampionMasteryLevelUp>;
    fn get_url(&self) -> String {"/lol-rms/v1/champion-mastery-leaveup-update".to_string()}
}

pub fn get_lol_rms_v_1_champion_mastery_leaveup_update() -> GetLolRmsV1ChampionMasteryLeaveupUpdate {
    GetLolRmsV1ChampionMasteryLeaveupUpdate{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRiotMessagingServiceChampionMasteryLevelUp {
    pub id: u64,
    pub puuid: String,
    pub champion_id: i32,
    pub has_leveled_up: bool,
    pub champion_level: i64,
}


// ENUMS

