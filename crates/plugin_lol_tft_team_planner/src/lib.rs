
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolTftTeamPlannerV1TeamChampions {}

impl IsApiRequest for DeleteLolTftTeamPlannerV1TeamChampions {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team/champions".to_string()}
}

pub fn delete_lol_tft_team_planner_v1_team_champions() -> DeleteLolTftTeamPlannerV1TeamChampions {
    DeleteLolTftTeamPlannerV1TeamChampions{}
}


pub struct DeleteLolTftTeamPlannerV1TeamChampionsByIdByChampionName {
    pub champion_name: String,
}

impl IsApiRequest for DeleteLolTftTeamPlannerV1TeamChampionsByIdByChampionName {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-tft-team-planner/v1/team/championsById/{}", self.champion_name)}
}

pub fn delete_lol_tft_team_planner_v1_team_champions_by_id_by_champion_name(champion_name: String) -> DeleteLolTftTeamPlannerV1TeamChampionsByIdByChampionName {
    DeleteLolTftTeamPlannerV1TeamChampionsByIdByChampionName{champion_name}
}


pub struct DeleteLolTftTeamPlannerV1TeamChampionsByIndex {
    pub index: u64,
}

impl IsApiRequest for DeleteLolTftTeamPlannerV1TeamChampionsByIndex {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-tft-team-planner/v1/team/champions/{}", self.index)}
}

pub fn delete_lol_tft_team_planner_v1_team_champions_by_index(index: u64) -> DeleteLolTftTeamPlannerV1TeamChampionsByIndex {
    DeleteLolTftTeamPlannerV1TeamChampionsByIndex{index}
}


pub struct DeleteLolTftTeamPlannerV1TeamDirty {}

impl IsApiRequest for DeleteLolTftTeamPlannerV1TeamDirty {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team/dirty".to_string()}
}

pub fn delete_lol_tft_team_planner_v1_team_dirty() -> DeleteLolTftTeamPlannerV1TeamDirty {
    DeleteLolTftTeamPlannerV1TeamDirty{}
}


pub struct GetLolTftTeamPlannerV1Config {}

impl IsApiRequest for GetLolTftTeamPlannerV1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTeamPlannerTftTeamPlannerConfig;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/config".to_string()}
}

pub fn get_lol_tft_team_planner_v1_config() -> GetLolTftTeamPlannerV1Config {
    GetLolTftTeamPlannerV1Config{}
}


pub struct GetLolTftTeamPlannerV1FtueHasViewed {}

impl IsApiRequest for GetLolTftTeamPlannerV1FtueHasViewed {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/ftue/hasViewed".to_string()}
}

pub fn get_lol_tft_team_planner_v1_ftue_has_viewed() -> GetLolTftTeamPlannerV1FtueHasViewed {
    GetLolTftTeamPlannerV1FtueHasViewed{}
}


pub struct GetLolTftTeamPlannerV1TeamDirty {}

impl IsApiRequest for GetLolTftTeamPlannerV1TeamDirty {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTeamPlannerTeamPlan;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team/dirty".to_string()}
}

pub fn get_lol_tft_team_planner_v1_team_dirty() -> GetLolTftTeamPlannerV1TeamDirty {
    GetLolTftTeamPlannerV1TeamDirty{}
}


pub struct GetLolTftTeamPlannerV1TeamLocal {}

impl IsApiRequest for GetLolTftTeamPlannerV1TeamLocal {
    const METHOD: Method = Method::GET;
    type ReturnType = LolTftTeamPlannerTeamSettings;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team/local".to_string()}
}

pub fn get_lol_tft_team_planner_v1_team_local() -> GetLolTftTeamPlannerV1TeamLocal {
    GetLolTftTeamPlannerV1TeamLocal{}
}


pub struct GetLolTftTeamPlannerV1TeamReminders {}

impl IsApiRequest for GetLolTftTeamPlannerV1TeamReminders {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team/reminders".to_string()}
}

pub fn get_lol_tft_team_planner_v1_team_reminders() -> GetLolTftTeamPlannerV1TeamReminders {
    GetLolTftTeamPlannerV1TeamReminders{}
}


pub struct PatchLolTftTeamPlannerV1FtueHasViewed {
    pub body: bool,
}

impl IsApiRequest for PatchLolTftTeamPlannerV1FtueHasViewed {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/ftue/hasViewed".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_tft_team_planner_v1_ftue_has_viewed(body: bool) -> PatchLolTftTeamPlannerV1FtueHasViewed {
    PatchLolTftTeamPlannerV1FtueHasViewed{body}
}


pub struct PatchLolTftTeamPlannerV1Set {
    pub body: String,
}

impl IsApiRequest for PatchLolTftTeamPlannerV1Set {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/set".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_tft_team_planner_v1_set(body: String) -> PatchLolTftTeamPlannerV1Set {
    PatchLolTftTeamPlannerV1Set{body}
}


pub struct PatchLolTftTeamPlannerV1TeamChampions {
    pub body: Vec<u64>,
}

impl IsApiRequest for PatchLolTftTeamPlannerV1TeamChampions {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team/champions".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_tft_team_planner_v1_team_champions(body: Vec<u64>) -> PatchLolTftTeamPlannerV1TeamChampions {
    PatchLolTftTeamPlannerV1TeamChampions{body}
}


pub struct PatchLolTftTeamPlannerV1TeamReminders {
    pub body: bool,
}

impl IsApiRequest for PatchLolTftTeamPlannerV1TeamReminders {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team/reminders".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_tft_team_planner_v1_team_reminders(body: bool) -> PatchLolTftTeamPlannerV1TeamReminders {
    PatchLolTftTeamPlannerV1TeamReminders{body}
}


pub struct PostLolTftTeamPlannerV1TeamChampionsByIdByChampionName {
    pub champion_name: String,
}

impl IsApiRequest for PostLolTftTeamPlannerV1TeamChampionsByIdByChampionName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-tft-team-planner/v1/team/championsById/{}", self.champion_name)}
}

pub fn post_lol_tft_team_planner_v1_team_champions_by_id_by_champion_name(champion_name: String) -> PostLolTftTeamPlannerV1TeamChampionsByIdByChampionName {
    PostLolTftTeamPlannerV1TeamChampionsByIdByChampionName{champion_name}
}


pub struct PostLolTftTeamPlannerV1TeamChampionsByIndex {
    pub index: u64,
    pub body: String,
}

impl IsApiRequest for PostLolTftTeamPlannerV1TeamChampionsByIndex {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-tft-team-planner/v1/team/champions/{}", self.index)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_tft_team_planner_v1_team_champions_by_index(index: u64, body: String) -> PostLolTftTeamPlannerV1TeamChampionsByIndex {
    PostLolTftTeamPlannerV1TeamChampionsByIndex{index, body}
}


pub struct PutLolTftTeamPlannerV1Team {}

impl IsApiRequest for PutLolTftTeamPlannerV1Team {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-tft-team-planner/v1/team".to_string()}
}

pub fn put_lol_tft_team_planner_v1_team() -> PutLolTftTeamPlannerV1Team {
    PutLolTftTeamPlannerV1Team{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerChampion {
    #[serde(rename = "championId")]
    pub champion_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTeamPlan {
    pub champions: Vec<LolTftTeamPlannerChampion>,
    #[serde(rename = "setName")]
    pub set_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTeamSettings {
    pub teams: Vec<LolTftTeamPlannerTeamPlan>,
    #[serde(rename = "remindersEnabled")]
    pub reminders_enabled: bool,
    #[serde(rename = "registeredTeamIndex")]
    pub registered_team_index: u64,
    #[serde(rename = "hasViewedTeamPlanner")]
    pub has_viewed_team_planner: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTftTeamPlannerConfig {
    pub enabled: bool,
    #[serde(rename = "multipleSetsEnabled")]
    pub multiple_sets_enabled: bool,
    #[serde(rename = "multipleTeamsEnabled")]
    pub multiple_teams_enabled: bool,
}


// ENUMS

