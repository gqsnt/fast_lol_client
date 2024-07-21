
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolChallengesV1AvailableQueueIds {}

impl IsApiRequest for GetLolChallengesV1AvailableQueueIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-challenges/v1/available-queue-ids".to_string()}
}

pub fn get_lol_challenges_v_1_available_queue_ids() -> GetLolChallengesV1AvailableQueueIds {
    GetLolChallengesV1AvailableQueueIds{}
}


pub struct GetLolChallengesV1ChallengesCategoryData {}

impl IsApiRequest for GetLolChallengesV1ChallengesCategoryData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiChallenge;
    fn get_url(&self) -> String {"/lol-challenges/v1/challenges/category-data".to_string()}
}

pub fn get_lol_challenges_v_1_challenges_category_data() -> GetLolChallengesV1ChallengesCategoryData {
    GetLolChallengesV1ChallengesCategoryData{}
}


pub struct GetLolChallengesV1ChallengesLocalPlayer {}

impl IsApiRequest for GetLolChallengesV1ChallengesLocalPlayer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiChallenge;
    fn get_url(&self) -> String {"/lol-challenges/v1/challenges/local-player".to_string()}
}

pub fn get_lol_challenges_v_1_challenges_local_player() -> GetLolChallengesV1ChallengesLocalPlayer {
    GetLolChallengesV1ChallengesLocalPlayer{}
}


pub struct GetLolChallengesV1ClientState {}

impl IsApiRequest for GetLolChallengesV1ClientState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesClientState;
    fn get_url(&self) -> String {"/lol-challenges/v1/client-state".to_string()}
}

pub fn get_lol_challenges_v_1_client_state() -> GetLolChallengesV1ClientState {
    GetLolChallengesV1ClientState{}
}


pub struct GetLolChallengesV1LevelPoints {}

impl IsApiRequest for GetLolChallengesV1LevelPoints {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, i64>;
    fn get_url(&self) -> String {"/lol-challenges/v1/level-points".to_string()}
}

pub fn get_lol_challenges_v_1_level_points() -> GetLolChallengesV1LevelPoints {
    GetLolChallengesV1LevelPoints{}
}


pub struct GetLolChallengesV1MyUpdatedChallengesByGameId {
    pub game_id: u64,
}

impl IsApiRequest for GetLolChallengesV1MyUpdatedChallengesByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiChallenge;
    fn get_url(&self) -> String {format!("/lol-challenges/v1/my-updated-challenges/{}", self.game_id)}
}

pub fn get_lol_challenges_v_1_my_updated_challenges_by_game_id(game_id: u64) -> GetLolChallengesV1MyUpdatedChallengesByGameId {
    GetLolChallengesV1MyUpdatedChallengesByGameId{game_id}
}


pub struct GetLolChallengesV1Penalty {}

impl IsApiRequest for GetLolChallengesV1Penalty {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiChallengePenalty;
    fn get_url(&self) -> String {"/lol-challenges/v1/penalty".to_string()}
}

pub fn get_lol_challenges_v_1_penalty() -> GetLolChallengesV1Penalty {
    GetLolChallengesV1Penalty{}
}


pub struct GetLolChallengesV1Seasons {}

impl IsApiRequest for GetLolChallengesV1Seasons {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChallengesChallengeSeason>;
    fn get_url(&self) -> String {"/lol-challenges/v1/seasons".to_string()}
}

pub fn get_lol_challenges_v_1_seasons() -> GetLolChallengesV1Seasons {
    GetLolChallengesV1Seasons{}
}


pub struct GetLolChallengesV1SummaryPlayerDataLocalPlayer {}

impl IsApiRequest for GetLolChallengesV1SummaryPlayerDataLocalPlayer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiPlayerSummary;
    fn get_url(&self) -> String {"/lol-challenges/v1/summary-player-data/local-player".to_string()}
}

pub fn get_lol_challenges_v_1_summary_player_data_local_player() -> GetLolChallengesV1SummaryPlayerDataLocalPlayer {
    GetLolChallengesV1SummaryPlayerDataLocalPlayer{}
}


pub struct GetLolChallengesV1SummaryPlayerDataPlayerByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolChallengesV1SummaryPlayerDataPlayerByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiPlayerSummary;
    fn get_url(&self) -> String {format!("/lol-challenges/v1/summary-player-data/player/{}", self.puuid)}
}

pub fn get_lol_challenges_v_1_summary_player_data_player_by_puuid(puuid: String) -> GetLolChallengesV1SummaryPlayerDataPlayerByPuuid {
    GetLolChallengesV1SummaryPlayerDataPlayerByPuuid{puuid}
}


pub struct GetLolChallengesV1SummaryPlayersDataPlayers {
    pub puuids: Vec<String>,
}

impl IsApiRequest for GetLolChallengesV1SummaryPlayersDataPlayers {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiPlayerSummary;
    fn get_url(&self) -> String {"/lol-challenges/v1/summary-players-data/players".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "puuids" : self.puuids,
        }))
    }
}

pub fn get_lol_challenges_v_1_summary_players_data_players(puuids: Vec<String>) -> GetLolChallengesV1SummaryPlayersDataPlayers {
    GetLolChallengesV1SummaryPlayersDataPlayers{puuids}
}


pub struct GetLolChallengesV1UpdatedChallengesByGameIdByPuuid {
    pub game_id: u64,
    pub puuid: String,
}

impl IsApiRequest for GetLolChallengesV1UpdatedChallengesByGameIdByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiChallenge;
    fn get_url(&self) -> String {format!("/lol-challenges/v1/updated-challenges/{}/{}", self.game_id, self.puuid)}
}

pub fn get_lol_challenges_v_1_updated_challenges_by_game_id_by_puuid(game_id: u64, puuid: String) -> GetLolChallengesV1UpdatedChallengesByGameIdByPuuid {
    GetLolChallengesV1UpdatedChallengesByGameIdByPuuid{game_id, puuid}
}


pub struct GetLolChallengesV2TitlesAll {}

impl IsApiRequest for GetLolChallengesV2TitlesAll {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChallengesUiTitle;
    fn get_url(&self) -> String {"/lol-challenges/v2/titles/all".to_string()}
}

pub fn get_lol_challenges_v_2_titles_all() -> GetLolChallengesV2TitlesAll {
    GetLolChallengesV2TitlesAll{}
}


pub struct GetLolChallengesV2TitlesLocalPlayer {}

impl IsApiRequest for GetLolChallengesV2TitlesLocalPlayer {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChallengesUiTitle>;
    fn get_url(&self) -> String {"/lol-challenges/v2/titles/local-player".to_string()}
}

pub fn get_lol_challenges_v_2_titles_local_player() -> GetLolChallengesV2TitlesLocalPlayer {
    GetLolChallengesV2TitlesLocalPlayer{}
}


pub struct PostLolChallengesV1AckChallengeUpdateById {
    pub id: u64,
}

impl IsApiRequest for PostLolChallengesV1AckChallengeUpdateById {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-challenges/v1/ack-challenge-update/{}", self.id)}
}

pub fn post_lol_challenges_v_1_ack_challenge_update_by_id(id: u64) -> PostLolChallengesV1AckChallengeUpdateById {
    PostLolChallengesV1AckChallengeUpdateById{id}
}


pub struct PostLolChallengesV1UpdatePlayerPreferences {
    pub body: LolChallengesChallengesPlayerPreferences,
}

impl IsApiRequest for PostLolChallengesV1UpdatePlayerPreferences {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-challenges/v1/update-player-preferences".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_challenges_v_1_update_player_preferences(body: LolChallengesChallengesPlayerPreferences) -> PostLolChallengesV1UpdatePlayerPreferences {
    PostLolChallengesV1UpdatePlayerPreferences{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeSeason {
    pub season_id: i32,
    pub season_start: i64,
    pub season_end: i64,
    pub is_active: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeSignedUpdatePayload {
    pub tokens_by_type: HashMap<String, HashMap<String, String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeTitleData {
    pub challenge_id: i64,
    pub challenge_name: String,
    pub challenge_description: String,
    pub level: String,
    pub level_to_icon_path: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengesPlayerPreferences {
    pub banner_accent: String,
    pub title: String,
    pub challenge_ids: Vec<i64>,
    pub crest_border: String,
    pub prestige_crest_border_level: i32,
    pub signed_jwt_payload: LolChallengesChallengeSignedUpdatePayload,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesFriendLevelsData {
    pub level: String,
    pub friends: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiCategoryProgress {
    pub level: String,
    pub category: String,
    pub position_percentile: f64,
    pub current: i32,
    pub max: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiChallenge {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub description_short: String,
    pub icon_path: String,
    pub category: String,
    pub next_level_icon_path: String,
    pub current_level: String,
    pub next_level: String,
    pub previous_level: String,
    pub previous_value: f64,
    pub current_value: f64,
    pub current_threshold: f64,
    pub next_threshold: f64,
    pub points_awarded: i64,
    pub percentile: f64,
    pub current_level_achieved_time: i64,
    pub position: i32,
    pub players_in_level: i32,
    pub is_apex: bool,
    pub is_capstone: bool,
    pub game_modes: Vec<String>,
    pub friends_at_levels: Vec<LolChallengesFriendLevelsData>,
    pub parent_id: i64,
    pub parent_name: String,
    pub children_ids: Vec<i64>,
    pub capstone_group_id: i64,
    pub capstone_group_name: String,
    pub source: String,
    pub thresholds: LolChallengesUiChallengeThreshold,
    pub level_to_icon_path: HashMap<String, String>,
    pub value_mapping: String,
    pub has_leaderboard: bool,
    pub is_reverse_direction: bool,
    pub priority: f64,
    pub id_list_type: LolChallengesChallengeRequirementMappingName,
    pub available_ids: Vec<i32>,
    pub completed_ids: Vec<i32>,
    pub retire_timestamp: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiChallengePenalty {
    pub reason: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiChallengeReward {
    pub category: String,
    pub quantity: u64,
    pub name: String,
    pub asset: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiChallengeThreshold {
    pub value: f64,
    pub rewards: Vec<LolChallengesUiChallengeReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiPlayerSummary {
    pub total_challenge_score: i64,
    pub points_until_next_rank: i64,
    pub overall_challenge_level: String,
    pub position_percentile: f64,
    pub is_apex: bool,
    pub apex_leaderboard_position: i32,
    pub title: LolChallengesUiTitle,
    pub banner_id: String,
    pub crest_id: String,
    pub prestige_crest_border_level: i32,
    pub category_progress: Vec<LolChallengesUiCategoryProgress>,
    pub top_challenges: Vec<LolChallengesUiChallenge>,
    pub apex_ladder_update_time: i64,
    pub selected_challenges_string: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiTitle {
    pub item_id: i32,
    pub content_id: String,
    pub name: String,
    pub purchase_date: String,
    pub title_acquisition_type: String,
    pub title_acquisition_name: Option<String>,
    pub title_requirement_description: Option<String>,
    pub is_permanent_title: Option<bool>,
    pub challenge_title_data: Option<LolChallengesChallengeTitleData>,
    pub icon_path: Option<String>,
    pub background_image_path: Option<String>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChallengesChallengeRequirementMappingName {
    #[default]
    #[serde(rename = "ITEM")]
    Item,
    #[serde(rename = "CHAMPION_SKIN")]
    ChampionSkin,
    #[serde(rename = "CHAMPION")]
    Champion,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChallengesClientState {
    #[default]
    Enabled,
    DarkDisabled,
    DarkHidden,
    Disabled,
    Hidden,
}

