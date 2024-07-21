
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolRankedV1ApexLeaguesByQueueTypeByTier {
    pub queue_type: LolRankedLeagueQueueType,
    pub tier: String,
}

impl IsApiRequest for GetLolRankedV1ApexLeaguesByQueueTypeByTier {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedLeagueLadderInfo;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/apex-leagues/{}/{}", serde_json::to_string(&self.queue_type).unwrap(), self.tier)}
}

pub fn get_lol_ranked_v_1_apex_leagues_by_queue_type_by_tier(queue_type: LolRankedLeagueQueueType, tier: String) -> GetLolRankedV1ApexLeaguesByQueueTypeByTier {
    GetLolRankedV1ApexLeaguesByQueueTypeByTier{queue_type, tier}
}


pub struct GetLolRankedV1ChallengerLaddersEnabled {}

impl IsApiRequest for GetLolRankedV1ChallengerLaddersEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/lol-ranked/v1/challenger-ladders-enabled".to_string()}
}

pub fn get_lol_ranked_v_1_challenger_ladders_enabled() -> GetLolRankedV1ChallengerLaddersEnabled {
    GetLolRankedV1ChallengerLaddersEnabled{}
}


pub struct GetLolRankedV1CurrentLpChangeNotification {}

impl IsApiRequest for GetLolRankedV1CurrentLpChangeNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedLcuLeagueNotification;
    fn get_url(&self) -> String {"/lol-ranked/v1/current-lp-change-notification".to_string()}
}

pub fn get_lol_ranked_v_1_current_lp_change_notification() -> GetLolRankedV1CurrentLpChangeNotification {
    GetLolRankedV1CurrentLpChangeNotification{}
}


pub struct GetLolRankedV1CurrentRankedStats {}

impl IsApiRequest for GetLolRankedV1CurrentRankedStats {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedRankedStats;
    fn get_url(&self) -> String {"/lol-ranked/v1/current-ranked-stats".to_string()}
}

pub fn get_lol_ranked_v_1_current_ranked_stats() -> GetLolRankedV1CurrentRankedStats {
    GetLolRankedV1CurrentRankedStats{}
}


pub struct GetLolRankedV1EligibleTiersQueueTypeByQueueType {
    pub queue_type: LolRankedLeagueQueueType,
}

impl IsApiRequest for GetLolRankedV1EligibleTiersQueueTypeByQueueType {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/eligibleTiers/queueType/{}", serde_json::to_string(&self.queue_type).unwrap())}
}

pub fn get_lol_ranked_v_1_eligible_tiers_queue_type_by_queue_type(queue_type: LolRankedLeagueQueueType) -> GetLolRankedV1EligibleTiersQueueTypeByQueueType {
    GetLolRankedV1EligibleTiersQueueTypeByQueueType{queue_type}
}


pub struct GetLolRankedV1EosNotifications {}

impl IsApiRequest for GetLolRankedV1EosNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRankedEosNotificationResource>;
    fn get_url(&self) -> String {"/lol-ranked/v1/eos-notifications".to_string()}
}

pub fn get_lol_ranked_v_1_eos_notifications() -> GetLolRankedV1EosNotifications {
    GetLolRankedV1EosNotifications{}
}


pub struct GetLolRankedV1EosRewards {}

impl IsApiRequest for GetLolRankedV1EosRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedEosRewardsConfig;
    fn get_url(&self) -> String {"/lol-ranked/v1/eos-rewards".to_string()}
}

pub fn get_lol_ranked_v_1_eos_rewards() -> GetLolRankedV1EosRewards {
    GetLolRankedV1EosRewards{}
}


pub struct GetLolRankedV1LeagueLaddersByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolRankedV1LeagueLaddersByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRankedLeagueLadderInfo>;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/league-ladders/{}", self.puuid)}
}

pub fn get_lol_ranked_v_1_league_ladders_by_puuid(puuid: String) -> GetLolRankedV1LeagueLaddersByPuuid {
    GetLolRankedV1LeagueLaddersByPuuid{puuid}
}


pub struct GetLolRankedV1Notifications {}

impl IsApiRequest for GetLolRankedV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRankedLcuLeagueNotification>;
    fn get_url(&self) -> String {"/lol-ranked/v1/notifications".to_string()}
}

pub fn get_lol_ranked_v_1_notifications() -> GetLolRankedV1Notifications {
    GetLolRankedV1Notifications{}
}


pub struct GetLolRankedV1RankedStatsByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolRankedV1RankedStatsByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedRankedStats;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/ranked-stats/{}", self.puuid)}
}

pub fn get_lol_ranked_v_1_ranked_stats_by_puuid(puuid: String) -> GetLolRankedV1RankedStatsByPuuid {
    GetLolRankedV1RankedStatsByPuuid{puuid}
}


pub struct GetLolRankedV1RatedLadderByQueueType {
    pub queue_type: LolRankedLeagueQueueType,
}

impl IsApiRequest for GetLolRankedV1RatedLadderByQueueType {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedRatedLadderInfo;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/rated-ladder/{}", serde_json::to_string(&self.queue_type).unwrap())}
}

pub fn get_lol_ranked_v_1_rated_ladder_by_queue_type(queue_type: LolRankedLeagueQueueType) -> GetLolRankedV1RatedLadderByQueueType {
    GetLolRankedV1RatedLadderByQueueType{queue_type}
}


pub struct GetLolRankedV1SignedRankedStats {}

impl IsApiRequest for GetLolRankedV1SignedRankedStats {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedSignedRankedStatsDto;
    fn get_url(&self) -> String {"/lol-ranked/v1/signed-ranked-stats".to_string()}
}

pub fn get_lol_ranked_v_1_signed_ranked_stats() -> GetLolRankedV1SignedRankedStats {
    GetLolRankedV1SignedRankedStats{}
}


pub struct GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids {
    pub queue_type: LolRankedLeagueQueueType,
    pub puuids: Vec<String>,
}

impl IsApiRequest for GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedSocialLeaderboardRankedQueueStats;
    fn get_url(&self) -> String {"/lol-ranked/v1/social-leaderboard-ranked-queue-stats-for-puuids".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "queueType" : self.queue_type,
            "puuids" : self.puuids,
        }))
    }
}

pub fn get_lol_ranked_v_1_social_leaderboard_ranked_queue_stats_for_puuids(queue_type: LolRankedLeagueQueueType, puuids: Vec<String>) -> GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids {
    GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids{queue_type, puuids}
}


pub struct GetLolRankedV1SplitsConfig {}

impl IsApiRequest for GetLolRankedV1SplitsConfig {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedRewardsInfo;
    fn get_url(&self) -> String {"/lol-ranked/v1/splits-config".to_string()}
}

pub fn get_lol_ranked_v_1_splits_config() -> GetLolRankedV1SplitsConfig {
    GetLolRankedV1SplitsConfig{}
}


pub struct GetLolRankedV1TopRatedLaddersEnabled {}

impl IsApiRequest for GetLolRankedV1TopRatedLaddersEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/lol-ranked/v1/top-rated-ladders-enabled".to_string()}
}

pub fn get_lol_ranked_v_1_top_rated_ladders_enabled() -> GetLolRankedV1TopRatedLaddersEnabled {
    GetLolRankedV1TopRatedLaddersEnabled{}
}


pub struct GetLolRankedV2Tiers {
    pub summoner_ids: Vec<u64>,
    pub queue_types: Vec<LolRankedLeagueQueueType>,
}

impl IsApiRequest for GetLolRankedV2Tiers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRankedParticipantTiers>;
    fn get_url(&self) -> String {"/lol-ranked/v2/tiers".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "summonerIds" : self.summoner_ids,
            "queueTypes" : self.queue_types,
        }))
    }
}

pub fn get_lol_ranked_v_2_tiers(summoner_ids: Vec<u64>, queue_types: Vec<LolRankedLeagueQueueType>) -> GetLolRankedV2Tiers {
    GetLolRankedV2Tiers{summoner_ids, queue_types}
}


pub struct PostLolRankedV1EosNotificationsByIdAcknowledge {
    pub id: String,
}

impl IsApiRequest for PostLolRankedV1EosNotificationsByIdAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/eos-notifications/{}/acknowledge", self.id)}
}

pub fn post_lol_ranked_v_1_eos_notifications_by_id_acknowledge(id: String) -> PostLolRankedV1EosNotificationsByIdAcknowledge {
    PostLolRankedV1EosNotificationsByIdAcknowledge{id}
}


pub struct PostLolRankedV1NotificationsByIdAcknowledge {
    pub id: u64,
}

impl IsApiRequest for PostLolRankedV1NotificationsByIdAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/notifications/{}/acknowledge", self.id)}
}

pub fn post_lol_ranked_v_1_notifications_by_id_acknowledge(id: u64) -> PostLolRankedV1NotificationsByIdAcknowledge {
    PostLolRankedV1NotificationsByIdAcknowledge{id}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedAchievedTier {
    pub queue_type: LolRankedLeagueQueueType,
    pub tier: String,
    pub division: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosNotificationResource {
    pub notification_name: String,
    pub notification_type: String,
    pub season_end_time: i64,
    pub queue: String,
    pub tier: String,
    pub division: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardData {
    pub id: String,
    pub type_: String,
    pub override_image_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardsConfig {
    pub seasons: LolRankedEosRewardsConfigEntry,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardsConfigEntry {
    pub rewards: LolRankedEosRewardData,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLcuLeagueNotification {
    pub id: u64,
    pub msg_id: String,
    pub display_type: LolRankedNotificationDisplayType,
    pub notify_reason: String,
    pub change_reason: String,
    pub queue_type: LolRankedLeagueQueueType,
    pub game_id: u64,
    pub provisional_games_remaining: i32,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    pub number_of_promotions: u64,
    pub miniseries_progress: String,
    pub league_points: i32,
    pub league_points_delta: i32,
    pub rated_tier: LolRankedRatedTier,
    pub rated_rating: i32,
    pub rated_rating_delta: i32,
    pub eligible_for_promo_helper: bool,
    pub miniseries_wins: i32,
    pub time_until_inactivity_status_changes: i64,
    pub reward_earned_id: String,
    pub reward_earned_type: String,
    pub reward_override_image_path: String,
    pub split_points_notification: Option<LolRankedSplitPointsNotification>,
    pub promo_series_for_ranks_enabled: bool,
    pub consolation_lp_used: i32,
    pub afk_lp_penalty_amount: i32,
    pub afk_lp_penalty_level: i32,
    pub was_afk_or_leaver: bool,
    pub can_demote_from_tier: bool,
    pub win_streak: i32,
    pub wins: i32,
    pub losses: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueDivisionInfo {
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    pub max_league_size: i32,
    pub apex_unlock_time_millis: i64,
    pub min_lp_for_apex_tier: i32,
    pub top_number_of_players: i64,
    pub standings: Vec<LolRankedLeagueStanding>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderInfo {
    pub queue_type: LolRankedLeagueQueueType,
    pub tier: String,
    pub provisional_game_threshold: i32,
    pub divisions: Vec<LolRankedLeagueDivisionInfo>,
    pub next_apex_update_millis: i64,
    pub requested_ranked_entry: Option<LolRankedLeagueStanding>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueStanding {
    pub summoner_id: u64,
    pub puuid: String,
    pub summoner_name: String,
    pub position: i32,
    pub position_delta: i32,
    pub previous_position: i32,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    pub league_points: i64,
    pub miniseries_results: Vec<LolRankedMiniseries>,
    pub wins: u64,
    pub losses: u64,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub previous_season_end_tier: String,
    pub previous_season_end_division: LolRankedLeagueDivision,
    pub earned_regalia_reward_ids: Vec<String>,
    pub ranked_regalia_level: i32,
    pub pending_promotion: bool,
    pub pending_demotion: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedParticipantTiers {
    pub summoner_id: u64,
    pub achieved_tiers: Vec<LolRankedAchievedTier>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStats {
    pub queue_type: LolRankedLeagueQueueType,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub rated_tier: LolRankedRatedTier,
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
    pub highest_tier: String,
    pub highest_division: LolRankedLeagueDivision,
    pub previous_season_end_tier: String,
    pub previous_season_end_division: LolRankedLeagueDivision,
    pub previous_season_highest_tier: String,
    pub previous_season_highest_division: LolRankedLeagueDivision,
    pub warnings: Option<LolRankedRankedQueueWarnings>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStatsDto {
    pub queue_type: String,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub tier: String,
    pub rank: String,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub rated_tier: String,
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
    pub highest_tier: String,
    pub highest_rank: String,
    pub previous_season_end_tier: String,
    pub previous_season_end_rank: String,
    pub previous_season_highest_tier: String,
    pub previous_season_highest_rank: String,
    pub warnings: Option<LolRankedRankedQueueWarningsDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarnings {
    pub display_decay_warning: bool,
    pub time_until_inactivity_status_changes: i64,
    pub demotion_warning: i32,
    pub days_until_decay: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarningsDto {
    pub display_decay_warning: bool,
    pub time_until_inactivity_status_changes: i64,
    pub demotion_warning: i32,
    pub apex_days_until_decay: i32,
    pub days_until_decay: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedStats {
    pub queues: Vec<LolRankedRankedQueueStats>,
    pub queue_map: LolRankedRankedQueueStats,
    pub highest_ranked_entry: Option<LolRankedRankedQueueStats>,
    pub highest_ranked_entry_sr: Option<LolRankedRankedQueueStats>,
    pub earned_regalia_reward_ids: Vec<String>,
    pub ranked_regalia_level: i32,
    pub highest_current_season_reached_tier_sr: String,
    pub highest_previous_season_end_tier: String,
    pub highest_previous_season_end_division: LolRankedLeagueDivision,
    pub splits_progress: HashMap<String, i32>,
    pub current_season_split_points: i32,
    pub previous_season_split_points: i32,
    pub seasons: LolRankedSeasonDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderInfo {
    pub queue_type: LolRankedLeagueQueueType,
    pub standings: Vec<LolRankedRatedLadderStanding>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderStanding {
    pub summoner_id: u64,
    pub puuid: String,
    pub summoner_name: String,
    pub rated_tier: LolRankedRatedTier,
    pub league_points: i32,
    pub wins: i32,
    pub position: i32,
    pub position_delta: i32,
    pub previous_position: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRewardsInfo {
    pub splits: Vec<LolRankedSeasonSplit>,
    pub current_split: Option<LolRankedSeasonSplit>,
    pub reward_info_by_reward_id: LolRankedSplitReward,
    pub current_split_id: i32,
    pub current_season_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonDto {
    pub current_season_id: i32,
    pub current_season_end: i64,
    pub next_season_start: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonSplit {
    pub split_id: i32,
    pub season_id: i32,
    pub start_time_millis: u64,
    pub end_time_millis: u64,
    pub reward_track: Vec<LolRankedSplitRewardGroup>,
    pub victorious_skin_reward_group: LolRankedVictoriousSkin,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSignedRankedStatsDto {
    pub queues: Vec<LolRankedRankedQueueStatsDto>,
    pub earned_regalia_reward_ids: Vec<String>,
    pub highest_previous_season_end_tier: String,
    pub highest_previous_season_end_rank: String,
    pub splits_progress: HashMap<String, i32>,
    pub current_season_split_points: i32,
    pub previous_season_split_points: i32,
    pub seasons: LolRankedSeasonDto,
    pub jwt: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSocialLeaderboardRankedQueueStats {
    pub queue_type: LolRankedLeagueQueueType,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub is_provisional: bool,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    pub league_points: i32,
    pub mini_series_progress: String,
    pub rated_tier: LolRankedRatedTier,
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitPointsNotification {
    pub split_points_delta: i32,
    pub split_points_before_game: i32,
    pub split_points_after_game: i32,
    pub previous_split_points_required: i32,
    pub split_points_required: i32,
    pub next_reward_id: String,
    pub next_reward_type: String,
    pub split_points_breakdown: HashMap<String, i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitReward {
    pub reward_type: String,
    pub quantity: i32,
    pub description: String,
    pub id: String,
    pub regalia_level: Option<i32>,
    pub points_required: i32,
    pub split_id: i32,
    pub champion_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardGroup {
    pub split_points: i32,
    pub rewards: Vec<LolRankedSplitReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedVictoriousSkin {
    pub split_points_by_highest_season_end_tier: HashMap<String, i32>,
    pub item_instance_id: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRankedLeagueDivision {
    #[default]
    #[serde(rename = "NA")]
    Na,
    V,
    #[serde(rename = "IV")]
    Iv,
    #[serde(rename = "III")]
    Iii,
    #[serde(rename = "II")]
    Ii,
    I,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRankedLeagueQueueType {
    #[default]
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    RankedTftDoubleUp,
    #[serde(rename = "RANKED_TFT_PAIRS")]
    RankedTftPairs,
    #[serde(rename = "RANKED_TFT_TURBO")]
    RankedTftTurbo,
    #[serde(rename = "RANKED_TFT")]
    RankedTft,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTt,
    #[serde(rename = "CHERRY")]
    Cherry,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RankedSolo5X5,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRankedMiniseries {
    #[default]
    N,
    L,
    W,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRankedNotificationDisplayType {
    #[default]
    #[serde(rename = "VIGNETTE")]
    Vignette,
    #[serde(rename = "MODAL")]
    Modal,
    #[serde(rename = "TOAST")]
    Toast,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRankedRatedTier {
    #[default]
    #[serde(rename = "ORANGE")]
    Orange,
    #[serde(rename = "PURPLE")]
    Purple,
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "GRAY")]
    Gray,
    #[serde(rename = "NONE")]
    None,
}

