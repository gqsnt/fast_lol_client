
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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

pub fn get_lol_ranked_v1_apex_leagues_by_queue_type_by_tier(queue_type: LolRankedLeagueQueueType, tier: String) -> GetLolRankedV1ApexLeaguesByQueueTypeByTier {
    GetLolRankedV1ApexLeaguesByQueueTypeByTier{queue_type, tier}
}


pub struct GetLolRankedV1ChallengerLaddersEnabled {}

impl IsApiRequest for GetLolRankedV1ChallengerLaddersEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/lol-ranked/v1/challenger-ladders-enabled".to_string()}
}

pub fn get_lol_ranked_v1_challenger_ladders_enabled() -> GetLolRankedV1ChallengerLaddersEnabled {
    GetLolRankedV1ChallengerLaddersEnabled{}
}


pub struct GetLolRankedV1CurrentLpChangeNotification {}

impl IsApiRequest for GetLolRankedV1CurrentLpChangeNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedLcuLeagueNotification;
    fn get_url(&self) -> String {"/lol-ranked/v1/current-lp-change-notification".to_string()}
}

pub fn get_lol_ranked_v1_current_lp_change_notification() -> GetLolRankedV1CurrentLpChangeNotification {
    GetLolRankedV1CurrentLpChangeNotification{}
}


pub struct GetLolRankedV1CurrentRankedStats {}

impl IsApiRequest for GetLolRankedV1CurrentRankedStats {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedRankedStats;
    fn get_url(&self) -> String {"/lol-ranked/v1/current-ranked-stats".to_string()}
}

pub fn get_lol_ranked_v1_current_ranked_stats() -> GetLolRankedV1CurrentRankedStats {
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

pub fn get_lol_ranked_v1_eligible_tiers_queue_type_by_queue_type(queue_type: LolRankedLeagueQueueType) -> GetLolRankedV1EligibleTiersQueueTypeByQueueType {
    GetLolRankedV1EligibleTiersQueueTypeByQueueType{queue_type}
}


pub struct GetLolRankedV1EosNotifications {}

impl IsApiRequest for GetLolRankedV1EosNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRankedEosNotificationResource>;
    fn get_url(&self) -> String {"/lol-ranked/v1/eos-notifications".to_string()}
}

pub fn get_lol_ranked_v1_eos_notifications() -> GetLolRankedV1EosNotifications {
    GetLolRankedV1EosNotifications{}
}


pub struct GetLolRankedV1EosRewards {}

impl IsApiRequest for GetLolRankedV1EosRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedEosRewardsConfig;
    fn get_url(&self) -> String {"/lol-ranked/v1/eos-rewards".to_string()}
}

pub fn get_lol_ranked_v1_eos_rewards() -> GetLolRankedV1EosRewards {
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

pub fn get_lol_ranked_v1_league_ladders_by_puuid(puuid: String) -> GetLolRankedV1LeagueLaddersByPuuid {
    GetLolRankedV1LeagueLaddersByPuuid{puuid}
}


pub struct GetLolRankedV1Notifications {}

impl IsApiRequest for GetLolRankedV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRankedLcuLeagueNotification>;
    fn get_url(&self) -> String {"/lol-ranked/v1/notifications".to_string()}
}

pub fn get_lol_ranked_v1_notifications() -> GetLolRankedV1Notifications {
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

pub fn get_lol_ranked_v1_ranked_stats_by_puuid(puuid: String) -> GetLolRankedV1RankedStatsByPuuid {
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

pub fn get_lol_ranked_v1_rated_ladder_by_queue_type(queue_type: LolRankedLeagueQueueType) -> GetLolRankedV1RatedLadderByQueueType {
    GetLolRankedV1RatedLadderByQueueType{queue_type}
}


pub struct GetLolRankedV1SignedRankedStats {}

impl IsApiRequest for GetLolRankedV1SignedRankedStats {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedSignedRankedStatsDto;
    fn get_url(&self) -> String {"/lol-ranked/v1/signed-ranked-stats".to_string()}
}

pub fn get_lol_ranked_v1_signed_ranked_stats() -> GetLolRankedV1SignedRankedStats {
    GetLolRankedV1SignedRankedStats{}
}


pub struct GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids {
    pub queue_type: LolRankedLeagueQueueType,
    pub puuids: Vec<String>,
}

impl IsApiRequest for GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, LolRankedSocialLeaderboardRankedQueueStats>;
    fn get_url(&self) -> String {"/lol-ranked/v1/social-leaderboard-ranked-queue-stats-for-puuids".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("queueType".to_string(), serde_json::to_string(&self.queue_type).unwrap()),
            ("puuids".to_string(), serde_json::to_string(&self.puuids).unwrap())
        ])
    }
}

pub fn get_lol_ranked_v1_social_leaderboard_ranked_queue_stats_for_puuids(queue_type: LolRankedLeagueQueueType, puuids: Vec<String>) -> GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids {
    GetLolRankedV1SocialLeaderboardRankedQueueStatsForPuuids{queue_type, puuids}
}


pub struct GetLolRankedV1SplitsConfig {}

impl IsApiRequest for GetLolRankedV1SplitsConfig {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRankedRewardsInfo;
    fn get_url(&self) -> String {"/lol-ranked/v1/splits-config".to_string()}
}

pub fn get_lol_ranked_v1_splits_config() -> GetLolRankedV1SplitsConfig {
    GetLolRankedV1SplitsConfig{}
}


pub struct GetLolRankedV1TopRatedLaddersEnabled {}

impl IsApiRequest for GetLolRankedV1TopRatedLaddersEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/lol-ranked/v1/top-rated-ladders-enabled".to_string()}
}

pub fn get_lol_ranked_v1_top_rated_ladders_enabled() -> GetLolRankedV1TopRatedLaddersEnabled {
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
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("summonerIds".to_string(), serde_json::to_string(&self.summoner_ids).unwrap()),
            ("queueTypes".to_string(), serde_json::to_string(&self.queue_types).unwrap())
        ])
    }
}

pub fn get_lol_ranked_v2_tiers(summoner_ids: Vec<u64>, queue_types: Vec<LolRankedLeagueQueueType>) -> GetLolRankedV2Tiers {
    GetLolRankedV2Tiers{summoner_ids, queue_types}
}


pub struct PostLolRankedV1EosNotificationsByIdAcknowledge {
    pub id: String,
}

impl IsApiRequest for PostLolRankedV1EosNotificationsByIdAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/eos-notifications/{}/acknowledge", self.id)}
}

pub fn post_lol_ranked_v1_eos_notifications_by_id_acknowledge(id: String) -> PostLolRankedV1EosNotificationsByIdAcknowledge {
    PostLolRankedV1EosNotificationsByIdAcknowledge{id}
}


pub struct PostLolRankedV1NotificationsByIdAcknowledge {
    pub id: u64,
}

impl IsApiRequest for PostLolRankedV1NotificationsByIdAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-ranked/v1/notifications/{}/acknowledge", self.id)}
}

pub fn post_lol_ranked_v1_notifications_by_id_acknowledge(id: u64) -> PostLolRankedV1NotificationsByIdAcknowledge {
    PostLolRankedV1NotificationsByIdAcknowledge{id}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedAchievedTier {
    #[serde(rename = "queueType")]
    pub queue_type: LolRankedLeagueQueueType,
    pub tier: String,
    pub division: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosNotificationResource {
    #[serde(rename = "notificationName")]
    pub notification_name: String,
    #[serde(rename = "notificationType")]
    pub notification_type: String,
    #[serde(rename = "seasonEndTime")]
    pub season_end_time: i64,
    pub queue: String,
    pub tier: String,
    pub division: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "overrideImagePath")]
    pub override_image_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardsConfig {
    pub seasons: HashMap<String, LolRankedEosRewardsConfigEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardsConfigEntry {
    pub rewards: HashMap<String, LolRankedEosRewardData>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLcuLeagueNotification {
    pub id: u64,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "displayType")]
    pub display_type: LolRankedNotificationDisplayType,
    #[serde(rename = "notifyReason")]
    pub notify_reason: String,
    #[serde(rename = "changeReason")]
    pub change_reason: String,
    #[serde(rename = "queueType")]
    pub queue_type: LolRankedLeagueQueueType,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "provisionalGamesRemaining")]
    pub provisional_games_remaining: i32,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    #[serde(rename = "numberOfPromotions")]
    pub number_of_promotions: u64,
    #[serde(rename = "miniseriesProgress")]
    pub miniseries_progress: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    #[serde(rename = "leaguePointsDelta")]
    pub league_points_delta: i32,
    #[serde(rename = "ratedTier")]
    pub rated_tier: LolRankedRatedTier,
    #[serde(rename = "ratedRating")]
    pub rated_rating: i32,
    #[serde(rename = "ratedRatingDelta")]
    pub rated_rating_delta: i32,
    #[serde(rename = "eligibleForPromoHelper")]
    pub eligible_for_promo_helper: bool,
    #[serde(rename = "miniseriesWins")]
    pub miniseries_wins: i32,
    #[serde(rename = "timeUntilInactivityStatusChanges")]
    pub time_until_inactivity_status_changes: i64,
    #[serde(rename = "rewardEarnedId")]
    pub reward_earned_id: String,
    #[serde(rename = "rewardEarnedType")]
    pub reward_earned_type: String,
    #[serde(rename = "rewardOverrideImagePath")]
    pub reward_override_image_path: String,
    #[serde(rename = "splitPointsNotification")]
    pub split_points_notification: Option<LolRankedSplitPointsNotification>,
    #[serde(rename = "promoSeriesForRanksEnabled")]
    pub promo_series_for_ranks_enabled: bool,
    #[serde(rename = "consolationLpUsed")]
    pub consolation_lp_used: i32,
    #[serde(rename = "afkLpPenaltyAmount")]
    pub afk_lp_penalty_amount: i32,
    #[serde(rename = "afkLpPenaltyLevel")]
    pub afk_lp_penalty_level: i32,
    #[serde(rename = "wasAfkOrLeaver")]
    pub was_afk_or_leaver: bool,
    #[serde(rename = "canDemoteFromTier")]
    pub can_demote_from_tier: bool,
    #[serde(rename = "winStreak")]
    pub win_streak: i32,
    pub wins: i32,
    pub losses: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueDivisionInfo {
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    #[serde(rename = "maxLeagueSize")]
    pub max_league_size: i32,
    #[serde(rename = "apexUnlockTimeMillis")]
    pub apex_unlock_time_millis: i64,
    #[serde(rename = "minLpForApexTier")]
    pub min_lp_for_apex_tier: i32,
    #[serde(rename = "topNumberOfPlayers")]
    pub top_number_of_players: i64,
    pub standings: Vec<LolRankedLeagueStanding>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderInfo {
    #[serde(rename = "queueType")]
    pub queue_type: LolRankedLeagueQueueType,
    pub tier: String,
    #[serde(rename = "provisionalGameThreshold")]
    pub provisional_game_threshold: i32,
    pub divisions: Vec<LolRankedLeagueDivisionInfo>,
    #[serde(rename = "nextApexUpdateMillis")]
    pub next_apex_update_millis: i64,
    #[serde(rename = "requestedRankedEntry")]
    pub requested_ranked_entry: Option<LolRankedLeagueStanding>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueStanding {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub puuid: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    pub position: i32,
    #[serde(rename = "positionDelta")]
    pub position_delta: i32,
    #[serde(rename = "previousPosition")]
    pub previous_position: i32,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    #[serde(rename = "leaguePoints")]
    pub league_points: i64,
    #[serde(rename = "miniseriesResults")]
    pub miniseries_results: Vec<LolRankedMiniseries>,
    pub wins: u64,
    pub losses: u64,
    #[serde(rename = "provisionalGamesRemaining")]
    pub provisional_games_remaining: i32,
    #[serde(rename = "isProvisional")]
    pub is_provisional: bool,
    #[serde(rename = "previousSeasonEndTier")]
    pub previous_season_end_tier: String,
    #[serde(rename = "previousSeasonEndDivision")]
    pub previous_season_end_division: LolRankedLeagueDivision,
    #[serde(rename = "earnedRegaliaRewardIds")]
    pub earned_regalia_reward_ids: Vec<String>,
    #[serde(rename = "rankedRegaliaLevel")]
    pub ranked_regalia_level: i32,
    #[serde(rename = "pendingPromotion")]
    pub pending_promotion: bool,
    #[serde(rename = "pendingDemotion")]
    pub pending_demotion: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedParticipantTiers {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "achievedTiers")]
    pub achieved_tiers: Vec<LolRankedAchievedTier>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStats {
    #[serde(rename = "queueType")]
    pub queue_type: LolRankedLeagueQueueType,
    #[serde(rename = "provisionalGameThreshold")]
    pub provisional_game_threshold: i32,
    #[serde(rename = "provisionalGamesRemaining")]
    pub provisional_games_remaining: i32,
    #[serde(rename = "isProvisional")]
    pub is_provisional: bool,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    #[serde(rename = "miniSeriesProgress")]
    pub mini_series_progress: String,
    #[serde(rename = "ratedTier")]
    pub rated_tier: LolRankedRatedTier,
    #[serde(rename = "ratedRating")]
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
    #[serde(rename = "highestTier")]
    pub highest_tier: String,
    #[serde(rename = "highestDivision")]
    pub highest_division: LolRankedLeagueDivision,
    #[serde(rename = "previousSeasonEndTier")]
    pub previous_season_end_tier: String,
    #[serde(rename = "previousSeasonEndDivision")]
    pub previous_season_end_division: LolRankedLeagueDivision,
    #[serde(rename = "previousSeasonHighestTier")]
    pub previous_season_highest_tier: String,
    #[serde(rename = "previousSeasonHighestDivision")]
    pub previous_season_highest_division: LolRankedLeagueDivision,
    pub warnings: Option<LolRankedRankedQueueWarnings>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStatsDto {
    #[serde(rename = "queueType")]
    pub queue_type: String,
    #[serde(rename = "provisionalGameThreshold")]
    pub provisional_game_threshold: i32,
    #[serde(rename = "provisionalGamesRemaining")]
    pub provisional_games_remaining: i32,
    pub tier: String,
    pub rank: String,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    #[serde(rename = "miniSeriesProgress")]
    pub mini_series_progress: String,
    #[serde(rename = "ratedTier")]
    pub rated_tier: String,
    #[serde(rename = "ratedRating")]
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
    #[serde(rename = "highestTier")]
    pub highest_tier: String,
    #[serde(rename = "highestRank")]
    pub highest_rank: String,
    #[serde(rename = "previousSeasonEndTier")]
    pub previous_season_end_tier: String,
    #[serde(rename = "previousSeasonEndRank")]
    pub previous_season_end_rank: String,
    #[serde(rename = "previousSeasonHighestTier")]
    pub previous_season_highest_tier: String,
    #[serde(rename = "previousSeasonHighestRank")]
    pub previous_season_highest_rank: String,
    pub warnings: Option<LolRankedRankedQueueWarningsDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarnings {
    #[serde(rename = "displayDecayWarning")]
    pub display_decay_warning: bool,
    #[serde(rename = "timeUntilInactivityStatusChanges")]
    pub time_until_inactivity_status_changes: i64,
    #[serde(rename = "demotionWarning")]
    pub demotion_warning: i32,
    #[serde(rename = "daysUntilDecay")]
    pub days_until_decay: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarningsDto {
    #[serde(rename = "displayDecayWarning")]
    pub display_decay_warning: bool,
    #[serde(rename = "timeUntilInactivityStatusChanges")]
    pub time_until_inactivity_status_changes: i64,
    #[serde(rename = "demotionWarning")]
    pub demotion_warning: i32,
    #[serde(rename = "apexDaysUntilDecay")]
    pub apex_days_until_decay: i32,
    #[serde(rename = "daysUntilDecay")]
    pub days_until_decay: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedStats {
    pub queues: Vec<LolRankedRankedQueueStats>,
    #[serde(rename = "queueMap")]
    pub queue_map: HashMap<String, LolRankedRankedQueueStats>,
    #[serde(rename = "highestRankedEntry")]
    pub highest_ranked_entry: Option<LolRankedRankedQueueStats>,
    #[serde(rename = "highestRankedEntrySR")]
    pub highest_ranked_entry_sr: Option<LolRankedRankedQueueStats>,
    #[serde(rename = "earnedRegaliaRewardIds")]
    pub earned_regalia_reward_ids: Vec<String>,
    #[serde(rename = "rankedRegaliaLevel")]
    pub ranked_regalia_level: i32,
    #[serde(rename = "highestCurrentSeasonReachedTierSR")]
    pub highest_current_season_reached_tier_sr: String,
    #[serde(rename = "highestPreviousSeasonEndTier")]
    pub highest_previous_season_end_tier: String,
    #[serde(rename = "highestPreviousSeasonEndDivision")]
    pub highest_previous_season_end_division: LolRankedLeagueDivision,
    #[serde(rename = "splitsProgress")]
    pub splits_progress: HashMap<String, i32>,
    #[serde(rename = "currentSeasonSplitPoints")]
    pub current_season_split_points: i32,
    #[serde(rename = "previousSeasonSplitPoints")]
    pub previous_season_split_points: i32,
    pub seasons: HashMap<String, LolRankedSeasonDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderInfo {
    #[serde(rename = "queueType")]
    pub queue_type: LolRankedLeagueQueueType,
    pub standings: Vec<LolRankedRatedLadderStanding>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderStanding {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub puuid: String,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "ratedTier")]
    pub rated_tier: LolRankedRatedTier,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    pub wins: i32,
    pub position: i32,
    #[serde(rename = "positionDelta")]
    pub position_delta: i32,
    #[serde(rename = "previousPosition")]
    pub previous_position: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRewardsInfo {
    pub splits: Vec<LolRankedSeasonSplit>,
    #[serde(rename = "currentSplit")]
    pub current_split: Option<LolRankedSeasonSplit>,
    #[serde(rename = "rewardInfoByRewardId")]
    pub reward_info_by_reward_id: HashMap<String, LolRankedSplitReward>,
    #[serde(rename = "currentSplitId")]
    pub current_split_id: i32,
    #[serde(rename = "currentSeasonId")]
    pub current_season_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonDto {
    #[serde(rename = "currentSeasonId")]
    pub current_season_id: i32,
    #[serde(rename = "currentSeasonEnd")]
    pub current_season_end: i64,
    #[serde(rename = "nextSeasonStart")]
    pub next_season_start: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonSplit {
    #[serde(rename = "splitId")]
    pub split_id: i32,
    #[serde(rename = "seasonId")]
    pub season_id: i32,
    #[serde(rename = "startTimeMillis")]
    pub start_time_millis: u64,
    #[serde(rename = "endTimeMillis")]
    pub end_time_millis: u64,
    #[serde(rename = "rewardTrack")]
    pub reward_track: Vec<LolRankedSplitRewardGroup>,
    #[serde(rename = "victoriousSkinRewardGroup")]
    pub victorious_skin_reward_group: LolRankedVictoriousSkin,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSignedRankedStatsDto {
    pub queues: Vec<LolRankedRankedQueueStatsDto>,
    #[serde(rename = "earnedRegaliaRewardIds")]
    pub earned_regalia_reward_ids: Vec<String>,
    #[serde(rename = "highestPreviousSeasonEndTier")]
    pub highest_previous_season_end_tier: String,
    #[serde(rename = "highestPreviousSeasonEndRank")]
    pub highest_previous_season_end_rank: String,
    #[serde(rename = "splitsProgress")]
    pub splits_progress: HashMap<String, i32>,
    #[serde(rename = "currentSeasonSplitPoints")]
    pub current_season_split_points: i32,
    #[serde(rename = "previousSeasonSplitPoints")]
    pub previous_season_split_points: i32,
    pub seasons: HashMap<String, LolRankedSeasonDto>,
    pub jwt: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSocialLeaderboardRankedQueueStats {
    #[serde(rename = "queueType")]
    pub queue_type: LolRankedLeagueQueueType,
    #[serde(rename = "provisionalGameThreshold")]
    pub provisional_game_threshold: i32,
    #[serde(rename = "provisionalGamesRemaining")]
    pub provisional_games_remaining: i32,
    #[serde(rename = "isProvisional")]
    pub is_provisional: bool,
    pub tier: String,
    pub division: LolRankedLeagueDivision,
    #[serde(rename = "leaguePoints")]
    pub league_points: i32,
    #[serde(rename = "miniSeriesProgress")]
    pub mini_series_progress: String,
    #[serde(rename = "ratedTier")]
    pub rated_tier: LolRankedRatedTier,
    #[serde(rename = "ratedRating")]
    pub rated_rating: i32,
    pub wins: i32,
    pub losses: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitPointsNotification {
    #[serde(rename = "splitPointsDelta")]
    pub split_points_delta: i32,
    #[serde(rename = "splitPointsBeforeGame")]
    pub split_points_before_game: i32,
    #[serde(rename = "splitPointsAfterGame")]
    pub split_points_after_game: i32,
    #[serde(rename = "previousSplitPointsRequired")]
    pub previous_split_points_required: i32,
    #[serde(rename = "splitPointsRequired")]
    pub split_points_required: i32,
    #[serde(rename = "nextRewardId")]
    pub next_reward_id: String,
    #[serde(rename = "nextRewardType")]
    pub next_reward_type: String,
    #[serde(rename = "splitPointsBreakdown")]
    pub split_points_breakdown: HashMap<String, i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitReward {
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    pub quantity: i32,
    pub description: String,
    pub id: String,
    #[serde(rename = "regaliaLevel")]
    pub regalia_level: Option<i32>,
    #[serde(rename = "pointsRequired")]
    pub points_required: i32,
    #[serde(rename = "splitId")]
    pub split_id: i32,
    #[serde(rename = "championId")]
    pub champion_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardGroup {
    #[serde(rename = "splitPoints")]
    pub split_points: i32,
    pub rewards: Vec<LolRankedSplitReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedVictoriousSkin {
    #[serde(rename = "splitPointsByHighestSeasonEndTier")]
    pub split_points_by_highest_season_end_tier: HashMap<String, i32>,
    #[serde(rename = "itemInstanceId")]
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

