
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolChampionMasteryV1RewardGrantsById {
    pub id: String,
}

impl IsApiRequest for DeleteLolChampionMasteryV1RewardGrantsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champion-mastery/v1/reward-grants/{}", self.id)}
}

pub fn delete_lol_champion_mastery_v1_reward_grants_by_id(id: String) -> DeleteLolChampionMasteryV1RewardGrantsById {
    DeleteLolChampionMasteryV1RewardGrantsById{id}
}


pub struct GetLolChampionMasteryV1ByPuuidChampionMastery {
    pub puuid: String,
}

impl IsApiRequest for GetLolChampionMasteryV1ByPuuidChampionMastery {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionMasteryChampionMastery>;
    fn get_url(&self) -> String {format!("/lol-champion-mastery/v1/{}/champion-mastery", self.puuid)}
}

pub fn get_lol_champion_mastery_v1_by_puuid_champion_mastery(puuid: String) -> GetLolChampionMasteryV1ByPuuidChampionMastery {
    GetLolChampionMasteryV1ByPuuidChampionMastery{puuid}
}


pub struct GetLolChampionMasteryV1LocalPlayerChampionMastery {}

impl IsApiRequest for GetLolChampionMasteryV1LocalPlayerChampionMastery {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionMasteryChampionMastery>;
    fn get_url(&self) -> String {"/lol-champion-mastery/v1/local-player/champion-mastery".to_string()}
}

pub fn get_lol_champion_mastery_v1_local_player_champion_mastery() -> GetLolChampionMasteryV1LocalPlayerChampionMastery {
    GetLolChampionMasteryV1LocalPlayerChampionMastery{}
}


pub struct GetLolChampionMasteryV1LocalPlayerChampionMasteryScore {}

impl IsApiRequest for GetLolChampionMasteryV1LocalPlayerChampionMasteryScore {
    const METHOD: Method = Method::GET;
    type ReturnType = u64;
    fn get_url(&self) -> String {"/lol-champion-mastery/v1/local-player/champion-mastery-score".to_string()}
}

pub fn get_lol_champion_mastery_v1_local_player_champion_mastery_score() -> GetLolChampionMasteryV1LocalPlayerChampionMasteryScore {
    GetLolChampionMasteryV1LocalPlayerChampionMasteryScore{}
}


pub struct GetLolChampionMasteryV1LocalPlayerChampionMasterySetsAndRewards {}

impl IsApiRequest for GetLolChampionMasteryV1LocalPlayerChampionMasterySetsAndRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampionMasteryUiAllChampionMasteryWithSets;
    fn get_url(&self) -> String {"/lol-champion-mastery/v1/local-player/champion-mastery-sets-and-rewards".to_string()}
}

pub fn get_lol_champion_mastery_v1_local_player_champion_mastery_sets_and_rewards() -> GetLolChampionMasteryV1LocalPlayerChampionMasterySetsAndRewards {
    GetLolChampionMasteryV1LocalPlayerChampionMasterySetsAndRewards{}
}


pub struct GetLolChampionMasteryV1Notifications {}

impl IsApiRequest for GetLolChampionMasteryV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampionMasteryChampionMasteryChangeNotification;
    fn get_url(&self) -> String {"/lol-champion-mastery/v1/notifications".to_string()}
}

pub fn get_lol_champion_mastery_v1_notifications() -> GetLolChampionMasteryV1Notifications {
    GetLolChampionMasteryV1Notifications{}
}


pub struct GetLolChampionMasteryV1RewardGrants {}

impl IsApiRequest for GetLolChampionMasteryV1RewardGrants {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionMasteryChampionMasteryRewardGrantNotification>;
    fn get_url(&self) -> String {"/lol-champion-mastery/v1/reward-grants".to_string()}
}

pub fn get_lol_champion_mastery_v1_reward_grants() -> GetLolChampionMasteryV1RewardGrants {
    GetLolChampionMasteryV1RewardGrants{}
}


pub struct PostLolChampionMasteryV1ByPuuidChampionMasteryTop {
    pub puuid: String,
    pub body: u32,
}

impl IsApiRequest for PostLolChampionMasteryV1ByPuuidChampionMasteryTop {
    const METHOD: Method = Method::POST;
    type ReturnType = LolChampionMasteryTopChampionMasteries;
    fn get_url(&self) -> String {format!("/lol-champion-mastery/v1/{}/champion-mastery/top", self.puuid)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_champion_mastery_v1_by_puuid_champion_mastery_top(puuid: String, body: u32) -> PostLolChampionMasteryV1ByPuuidChampionMasteryTop {
    PostLolChampionMasteryV1ByPuuidChampionMasteryTop{puuid, body}
}


pub struct PostLolChampionMasteryV1NotificationsAck {}

impl IsApiRequest for PostLolChampionMasteryV1NotificationsAck {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champion-mastery/v1/notifications/ack".to_string()}
}

pub fn post_lol_champion_mastery_v1_notifications_ack() -> PostLolChampionMasteryV1NotificationsAck {
    PostLolChampionMasteryV1NotificationsAck{}
}


pub struct PostLolChampionMasteryV1Scouting {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolChampionMasteryV1Scouting {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<RankedScoutingDto>;
    fn get_url(&self) -> String {"/lol-champion-mastery/v1/scouting".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_champion_mastery_v1_scouting(body: Vec<String>) -> PostLolChampionMasteryV1Scouting {
    PostLolChampionMasteryV1Scouting{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryPublicDto {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championLevel")]
    pub champion_level: i32,
    #[serde(rename = "championPoints")]
    pub champion_points: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionScoutingDto {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "winCount")]
    pub win_count: i32,
    #[serde(rename = "gameCount")]
    pub game_count: i32,
    pub kda: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMastery {
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championLevel")]
    pub champion_level: i32,
    #[serde(rename = "championPoints")]
    pub champion_points: i32,
    #[serde(rename = "lastPlayTime")]
    pub last_play_time: u64,
    #[serde(rename = "championPointsSinceLastLevel")]
    pub champion_points_since_last_level: i32,
    #[serde(rename = "championPointsUntilNextLevel")]
    pub champion_points_until_next_level: i32,
    #[serde(rename = "markRequiredForNextLevel")]
    pub mark_required_for_next_level: i32,
    #[serde(rename = "tokensEarned")]
    pub tokens_earned: i32,
    #[serde(rename = "championSeasonMilestone")]
    pub champion_season_milestone: i32,
    #[serde(rename = "milestoneGrades")]
    pub milestone_grades: Vec<String>,
    #[serde(rename = "nextSeasonMilestone")]
    pub next_season_milestone: LolChampionMasterySeasonMilestoneRequireAndRewards,
    #[serde(rename = "highestGrade")]
    pub highest_grade: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMasteryChangeNotification {
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championLevel")]
    pub champion_level: i32,
    #[serde(rename = "championPointsBeforeGame")]
    pub champion_points_before_game: i32,
    #[serde(rename = "championPointsGained")]
    pub champion_points_gained: i32,
    #[serde(rename = "championPointsGainedIndividualContribution")]
    pub champion_points_gained_individual_contribution: i32,
    #[serde(rename = "bonusChampionPointsGained")]
    pub bonus_champion_points_gained: i32,
    #[serde(rename = "playerGrade")]
    pub player_grade: String,
    #[serde(rename = "championPointsSinceLastLevelBeforeGame")]
    pub champion_points_since_last_level_before_game: i32,
    #[serde(rename = "championPointsUntilNextLevelBeforeGame")]
    pub champion_points_until_next_level_before_game: i32,
    #[serde(rename = "championPointsUntilNextLevelAfterGame")]
    pub champion_points_until_next_level_after_game: i32,
    #[serde(rename = "championLevelUp")]
    pub champion_level_up: bool,
    pub score: i32,
    #[serde(rename = "levelUpList")]
    pub level_up_list: Vec<LolChampionMasteryChampionMasteryMini>,
    #[serde(rename = "memberGrades")]
    pub member_grades: Vec<LolChampionMasteryChampionMasteryGrade>,
    pub win: bool,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "tokensEarned")]
    pub tokens_earned: i32,
    #[serde(rename = "tokenEarnedAfterGame")]
    pub token_earned_after_game: bool,
    #[serde(rename = "markRequiredForNextLevel")]
    pub mark_required_for_next_level: i32,
    #[serde(rename = "championSeasonMilestone")]
    pub champion_season_milestone: i32,
    #[serde(rename = "championSeasonMilestoneUp")]
    pub champion_season_milestone_up: bool,
    #[serde(rename = "milestoneGrades")]
    pub milestone_grades: Vec<String>,
    #[serde(rename = "seasonMilestone")]
    pub season_milestone: LolChampionMasterySeasonMilestoneRequireAndRewards,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMasteryGrade {
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    pub grade: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMasteryMini {
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championLevel")]
    pub champion_level: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMasteryRewardGrantNotification {
    pub id: String,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "playerGrade")]
    pub player_grade: String,
    #[serde(rename = "messageKey")]
    pub message_key: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionSet {
    pub champions: Vec<i32>,
    #[serde(rename = "totalMilestone")]
    pub total_milestone: i32,
    pub completed: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryRewardConfigurationEntry {
    #[serde(rename = "rewardValue")]
    pub reward_value: String,
    #[serde(rename = "maximumReward")]
    pub maximum_reward: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasterySeasonMilestoneRequireAndRewards {
    #[serde(rename = "requireGradeCounts")]
    pub require_grade_counts: HashMap<String, i16>,
    #[serde(rename = "rewardMarks")]
    pub reward_marks: u16,
    pub bonus: bool,
    #[serde(rename = "rewardConfig")]
    pub reward_config: LolChampionMasteryRewardConfigurationEntry,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasterySetRewardEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub value: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryTopChampionMasteries {
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub score: u64,
    pub masteries: Vec<LolChampionMasteryChampionMastery>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryUiAllChampionMasteryWithSets {
    #[serde(rename = "championMasteries")]
    pub champion_masteries: Vec<LolChampionMasteryChampionMastery>,
    #[serde(rename = "championSet")]
    pub champion_set: LolChampionMasteryChampionSet,
    #[serde(rename = "championSetRewards")]
    pub champion_set_rewards: HashMap<String, LolChampionMasterySetRewardEntry>,
    #[serde(rename = "seasonMilestoneRequireAndRewards")]
    pub season_milestone_require_and_rewards: HashMap<String, LolChampionMasterySeasonMilestoneRequireAndRewards>,
    #[serde(rename = "defaultChampionMastery")]
    pub default_champion_mastery: LolChampionMasteryChampionMastery,
    #[serde(rename = "customRewards")]
    pub custom_rewards: Vec<LolChampionMasteryUiChampionMasteryCustomReward>,
    #[serde(rename = "totalScore")]
    pub total_score: i32,
    #[serde(rename = "championCountByMilestone")]
    pub champion_count_by_milestone: HashMap<String, i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryUiChampionMasteryCustomReward {
    #[serde(rename = "type")]
    pub type_: String,
    pub level: i32,
    #[serde(rename = "rewardValue")]
    pub reward_value: String,
    pub quantity: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RankedScoutingDto {
    #[serde(rename = "playerId")]
    pub player_id: u64,
    pub puuid: String,
    #[serde(rename = "totalMasteryScore")]
    pub total_mastery_score: u64,
    #[serde(rename = "topMasteries")]
    pub top_masteries: Vec<ChampionMasteryPublicDto>,
    #[serde(rename = "topSeasonChampions")]
    pub top_season_champions: Vec<ChampionScoutingDto>,
}


// ENUMS

