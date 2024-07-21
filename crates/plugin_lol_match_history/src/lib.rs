
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolMatchHistoryV1Delta {

}

impl IsApiRequest for GetLolMatchHistoryV1Delta {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryPlayerDelta;

    fn get_url(&self) -> String {
        "/lol-match-history/v1/delta".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_match_history_v_1_delta() -> GetLolMatchHistoryV1Delta {
    GetLolMatchHistoryV1Delta {
        
    }
}


pub struct GetLolMatchHistoryV1GameTimelinesByGameId {

    pub game_id: u64,
}

impl IsApiRequest for GetLolMatchHistoryV1GameTimelinesByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryTimelineFrames;

    fn get_url(&self) -> String {
        format!("/lol-match-history/v1/game-timelines/{}", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_match_history_v_1_game_timelines_by_game_id(game_id: u64) -> GetLolMatchHistoryV1GameTimelinesByGameId {
    GetLolMatchHistoryV1GameTimelinesByGameId {
        game_id
    }
}


pub struct GetLolMatchHistoryV1GamesByGameId {

    pub game_id: u64,
}

impl IsApiRequest for GetLolMatchHistoryV1GamesByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryGame;

    fn get_url(&self) -> String {
        format!("/lol-match-history/v1/games/{}", self.game_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_match_history_v_1_games_by_game_id(game_id: u64) -> GetLolMatchHistoryV1GamesByGameId {
    GetLolMatchHistoryV1GamesByGameId {
        game_id
    }
}


pub struct GetLolMatchHistoryV1ProductsLolByPuuidMatches {

    pub puuid: String,
    pub beg_index: Option<u32>,
    pub end_index: Option<u32>,
}

impl IsApiRequest for GetLolMatchHistoryV1ProductsLolByPuuidMatches {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryList;

    fn get_url(&self) -> String {
        format!("/lol-match-history/v1/products/lol/{}/matches", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "begIndex" : self.beg_index,
            "endIndex" : self.end_index,
        }))
    }
}

pub fn get_lol_match_history_v_1_products_lol_by_puuid_matches(puuid: String, beg_index: Option<u32>, end_index: Option<u32>) -> GetLolMatchHistoryV1ProductsLolByPuuidMatches {
    GetLolMatchHistoryV1ProductsLolByPuuidMatches {
        puuid, beg_index, end_index
    }
}


pub struct GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches {

    pub beg_index: Option<u32>,
    pub end_index: Option<u32>,
}

impl IsApiRequest for GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryList;

    fn get_url(&self) -> String {
        "/lol-match-history/v1/products/lol/current-summoner/matches".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "begIndex" : self.beg_index,
            "endIndex" : self.end_index,
        }))
    }
}

pub fn get_lol_match_history_v_1_products_lol_current_summoner_matches(beg_index: Option<u32>, end_index: Option<u32>) -> GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches {
    GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches {
        beg_index, end_index
    }
}


pub struct GetLolMatchHistoryV1ProductsTftByPuuidMatches {

    pub puuid: String,
    pub begin: Option<u32>,
    pub count: Option<u32>,
    pub tag: Option<String>,
}

impl IsApiRequest for GetLolMatchHistoryV1ProductsTftByPuuidMatches {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryGamhsMatchHistoryList;

    fn get_url(&self) -> String {
        format!("/lol-match-history/v1/products/tft/{}/matches", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "begin" : self.begin,
            "count" : self.count,
            "tag" : self.tag,
        }))
    }
}

pub fn get_lol_match_history_v_1_products_tft_by_puuid_matches(puuid: String, begin: Option<u32>, count: Option<u32>, tag: Option<String>) -> GetLolMatchHistoryV1ProductsTftByPuuidMatches {
    GetLolMatchHistoryV1ProductsTftByPuuidMatches {
        puuid, begin, count, tag
    }
}


pub struct GetLolMatchHistoryV1RecentlyPlayedSummoners {

}

impl IsApiRequest for GetLolMatchHistoryV1RecentlyPlayedSummoners {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolMatchHistoryRecentlyPlayedSummoner>;

    fn get_url(&self) -> String {
        "/lol-match-history/v1/recently-played-summoners".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_match_history_v_1_recently_played_summoners() -> GetLolMatchHistoryV1RecentlyPlayedSummoners {
    GetLolMatchHistoryV1RecentlyPlayedSummoners {
        
    }
}


pub struct GetLolMatchHistoryV1WebUrl {

}

impl IsApiRequest for GetLolMatchHistoryV1WebUrl {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-match-history/v1/web-url".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_match_history_v_1_web_url() -> GetLolMatchHistoryV1WebUrl {
    GetLolMatchHistoryV1WebUrl {
        
    }
}


pub struct GetLolMatchHistoryV3MatchlistAccountByAccountId {

    pub account_id: u64,
    pub beg_index: Option<u32>,
    pub end_index: Option<u32>,
}

impl IsApiRequest for GetLolMatchHistoryV3MatchlistAccountByAccountId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryList;

    fn get_url(&self) -> String {
        format!("/lol-match-history/v3/matchlist/account/{}", self.account_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "begIndex" : self.beg_index,
            "endIndex" : self.end_index,
        }))
    }
}

pub fn get_lol_match_history_v_3_matchlist_account_by_account_id(account_id: u64, beg_index: Option<u32>, end_index: Option<u32>) -> GetLolMatchHistoryV3MatchlistAccountByAccountId {
    GetLolMatchHistoryV3MatchlistAccountByAccountId {
        account_id, beg_index, end_index
    }
}


pub struct PostLolMatchHistoryV1AcsEndpointOverride {

    pub body: LolMatchHistoryAcsEndPoint,
}

impl IsApiRequest for PostLolMatchHistoryV1AcsEndpointOverride {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-match-history/v1/acs-endpoint-override".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_match_history_v_1_acs_endpoint_override(body: LolMatchHistoryAcsEndPoint) -> PostLolMatchHistoryV1AcsEndpointOverride {
    PostLolMatchHistoryV1AcsEndpointOverride {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryAcsEndPoint {
    pub url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryGamhsMatchHistoryData {
    pub metadata: LolMatchHistoryGamhsMatchHistoryMetadata,
    pub json: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryGamhsMatchHistoryList {
    pub games: Vec<LolMatchHistoryGamhsMatchHistoryData>,
    pub active_puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryGamhsMatchHistoryMetadata {
    pub product: String,
    pub data_version: u8,
    pub info_type: String,
    pub match_id: String,
    pub tags: Vec<String>,
    pub participants: Vec<String>,
    pub timestamp: u64,
    pub private: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryEvent {
    pub type_: String,
    pub timestamp: u64,
    pub participant_id: u16,
    pub team_id: u16,
    pub item_id: u16,
    pub killer_id: u16,
    pub victim_id: u16,
    pub skill_slot: u16,
    pub position: LolMatchHistoryMatchHistoryPosition,
    pub assisting_participant_ids: Vec<u16>,
    pub building_type: String,
    pub lane_type: String,
    pub tower_type: String,
    pub monster_type: String,
    pub monster_sub_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGame {
    pub end_of_game_result: String,
    pub game_id: u64,
    pub platform_id: String,
    pub game_creation: u64,
    pub game_creation_date: String,
    pub game_duration: u32,
    pub queue_id: i32,
    pub map_id: u16,
    pub season_id: u16,
    pub game_version: String,
    pub game_mode: String,
    pub game_type: String,
    pub teams: Vec<LolMatchHistoryMatchHistoryTeam>,
    pub participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    pub participant_identities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGameList {
    pub game_index_begin: u64,
    pub game_index_end: u64,
    pub game_begin_date: String,
    pub game_end_date: String,
    pub game_count: u64,
    pub games: Vec<LolMatchHistoryMatchHistoryGame>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryList {
    pub platform_id: String,
    pub account_id: u64,
    pub games: LolMatchHistoryMatchHistoryGameList,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipant {
    pub participant_id: u16,
    pub team_id: u16,
    pub champion_id: i32,
    pub spell_1_id: u16,
    pub spell_2_id: u16,
    pub highest_achieved_season_tier: String,
    pub stats: LolMatchHistoryMatchHistoryParticipantStatistics,
    pub timeline: LolMatchHistoryMatchHistoryTimeline,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantFrame {
    pub participant_id: u16,
    pub position: LolMatchHistoryMatchHistoryPosition,
    pub current_gold: i32,
    pub total_gold: i32,
    pub level: u16,
    pub xp: u32,
    pub minions_killed: u16,
    pub jungle_minions_killed: u16,
    pub dominion_score: u16,
    pub team_score: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    pub participant_id: u16,
    pub player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    pub puuid: String,
    pub platform_id: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub summoner_name: String,
    pub game_name: String,
    pub tag_line: String,
    pub current_platform_id: String,
    pub current_account_id: u64,
    pub match_history_uri: String,
    pub profile_icon: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    pub participant_id: u16,
    pub win: bool,
    pub item_0: i32,
    pub item_1: i32,
    pub item_2: i32,
    pub item_3: i32,
    pub item_4: i32,
    pub item_5: i32,
    pub item_6: i32,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
    pub killing_sprees: i64,
    pub longest_time_spent_living: i64,
    pub double_kills: i64,
    pub triple_kills: i64,
    pub quadra_kills: i64,
    pub penta_kills: i64,
    pub unreal_kills: i64,
    pub total_damage_dealt: i64,
    pub magic_damage_dealt: i64,
    pub physical_damage_dealt: i64,
    pub true_damage_dealt: i64,
    pub largest_critical_strike: i64,
    pub total_damage_dealt_to_champions: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub true_damage_dealt_to_champions: i64,
    pub total_heal: i64,
    pub total_units_healed: i64,
    pub total_damage_taken: i64,
    pub magical_damage_taken: i64,
    pub physical_damage_taken: i64,
    pub true_damage_taken: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub turret_kills: i64,
    pub inhibitor_kills: i64,
    pub total_minions_killed: i64,
    pub neutral_minions_killed: i64,
    pub neutral_minions_killed_team_jungle: i64,
    pub neutral_minions_killed_enemy_jungle: i64,
    pub total_time_crowd_control_dealt: i64,
    pub champ_level: i64,
    pub vision_wards_bought_in_game: i64,
    pub sight_wards_bought_in_game: i64,
    pub wards_placed: i64,
    pub wards_killed: i64,
    pub first_blood_kill: bool,
    pub first_blood_assist: bool,
    pub first_tower_kill: bool,
    pub first_tower_assist: bool,
    pub first_inhibitor_kill: bool,
    pub first_inhibitor_assist: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub caused_early_surrender: bool,
    pub early_surrender_accomplice: bool,
    pub team_early_surrendered: bool,
    pub combat_player_score: i64,
    pub objective_player_score: i64,
    pub total_player_score: i64,
    pub total_score_rank: i64,
    pub damage_self_mitigated: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub vision_score: i64,
    pub time_c_cing_others: i64,
    pub player_score_0: i64,
    pub player_score_1: i64,
    pub player_score_2: i64,
    pub player_score_3: i64,
    pub player_score_4: i64,
    pub player_score_5: i64,
    pub player_score_6: i64,
    pub player_score_7: i64,
    pub player_score_8: i64,
    pub player_score_9: i64,
    pub perk_primary_style: i64,
    pub perk_sub_style: i64,
    pub perk_0: i64,
    pub perk_0_var_1: i64,
    pub perk_0_var_2: i64,
    pub perk_0_var_3: i64,
    pub perk_1: i64,
    pub perk_1_var_1: i64,
    pub perk_1_var_2: i64,
    pub perk_1_var_3: i64,
    pub perk_2: i64,
    pub perk_2_var_1: i64,
    pub perk_2_var_2: i64,
    pub perk_2_var_3: i64,
    pub perk_3: i64,
    pub perk_3_var_1: i64,
    pub perk_3_var_2: i64,
    pub perk_3_var_3: i64,
    pub perk_4: i64,
    pub perk_4_var_1: i64,
    pub perk_4_var_2: i64,
    pub perk_4_var_3: i64,
    pub perk_5: i64,
    pub perk_5_var_1: i64,
    pub perk_5_var_2: i64,
    pub perk_5_var_3: i64,
    pub player_augment_1: i32,
    pub player_augment_2: i32,
    pub player_augment_3: i32,
    pub player_augment_4: i32,
    pub player_augment_5: i32,
    pub player_augment_6: i32,
    pub player_subteam_id: i32,
    pub subteam_placement: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerChampMasteryDelta {
    pub grade: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerDelta {
    pub original_account_id: u64,
    pub original_platform_id: String,
    pub deltas: Vec<LolMatchHistoryMatchHistoryPlayerGameDelta>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerGameDelta {
    pub game_platform_id: String,
    pub game_id: u64,
    pub platform_delta: LolMatchHistoryMatchHistoryPlayerPlatformDelta,
    pub league_delta: LolMatchHistoryMatchHistoryPlayerLeagueDelta,
    pub champ_mastery: LolMatchHistoryMatchHistoryPlayerChampMasteryDelta,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerLeagueDelta {
    pub league_point_delta: u64,
    pub reason: String,
    pub mini_series_progress: Vec<String>,
    pub timestamp: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerPlatformDelta {
    pub xp_delta: u64,
    pub ip_delta: u64,
    pub compensation_mode_enabled: bool,
    pub timestamp: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPosition {
    pub x: i16,
    pub y: i16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeam {
    pub team_id: u16,
    pub win: String,
    pub first_blood: bool,
    pub first_tower: bool,
    pub first_inhibitor: bool,
    pub first_baron: bool,
    pub first_dargon: bool,
    pub tower_kills: u32,
    pub inhibitor_kills: u32,
    pub baron_kills: u32,
    pub dragon_kills: u32,
    pub vilemaw_kills: u32,
    pub rift_herald_kills: u32,
    pub horde_kills: u32,
    pub dominion_victory_score: u32,
    pub bans: Vec<LolMatchHistoryMatchHistoryTeamBan>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    pub champion_id: i32,
    pub pick_turn: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimeline {
    pub participant_id: u16,
    pub role: String,
    pub lane: String,
    pub creeps_per_min_deltas: HashMap<String, f64>,
    pub xp_per_min_deltas: HashMap<String, f64>,
    pub gold_per_min_deltas: HashMap<String, f64>,
    pub cs_diff_per_min_deltas: HashMap<String, f64>,
    pub xp_diff_per_min_deltas: HashMap<String, f64>,
    pub damage_taken_per_min_deltas: HashMap<String, f64>,
    pub damage_taken_diff_per_min_deltas: HashMap<String, f64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimelineFrame {
    pub participant_frames: LolMatchHistoryMatchHistoryParticipantFrame,
    pub events: Vec<LolMatchHistoryMatchHistoryEvent>,
    pub timestamp: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimelineFrames {
    pub frames: Vec<LolMatchHistoryMatchHistoryTimelineFrame>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryRecentlyPlayedSummoner {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub game_name: String,
    pub tag_line: String,
    pub game_id: u64,
    pub game_creation_date: String,
    pub champion_id: u64,
    pub team_id: u64,
    pub puuid: String,
}


// ENUMS

