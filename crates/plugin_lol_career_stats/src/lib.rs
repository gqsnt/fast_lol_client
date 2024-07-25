
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolCareerStatsV1ChampionAveragesByChampionIdByPositionByTierByQueue {
    pub champion_id: i32,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub tier: String,
    pub queue: LolCareerStatsCareerStatsQueueType,
}

impl IsApiRequest for GetLolCareerStatsV1ChampionAveragesByChampionIdByPositionByTierByQueue {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCareerStatsChampionQueueStatsResponse;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/champion-averages/{}/{}/{}/{}", self.champion_id, serde_json::to_string(&self.position).unwrap(), self.tier, serde_json::to_string(&self.queue).unwrap())}
}

pub fn get_lol_career_stats_v1_champion_averages_by_champion_id_by_position_by_tier_by_queue(champion_id: i32, position: LolCareerStatsSummonersRiftPosition, tier: String, queue: LolCareerStatsCareerStatsQueueType) -> GetLolCareerStatsV1ChampionAveragesByChampionIdByPositionByTierByQueue {
    GetLolCareerStatsV1ChampionAveragesByChampionIdByPositionByTierByQueue{champion_id, position, tier, queue}
}


pub struct GetLolCareerStatsV1ChampionAveragesSeasonBySeasonByChampionIdByPositionByTierByQueue {
    pub season: u32,
    pub champion_id: i32,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub tier: String,
    pub queue: LolCareerStatsCareerStatsQueueType,
}

impl IsApiRequest for GetLolCareerStatsV1ChampionAveragesSeasonBySeasonByChampionIdByPositionByTierByQueue {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCareerStatsChampionQueueStatsResponse;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/champion-averages/season/{}/{}/{}/{}/{}", self.season, self.champion_id, serde_json::to_string(&self.position).unwrap(), self.tier, serde_json::to_string(&self.queue).unwrap())}
}

pub fn get_lol_career_stats_v1_champion_averages_season_by_season_by_champion_id_by_position_by_tier_by_queue(season: u32, champion_id: i32, position: LolCareerStatsSummonersRiftPosition, tier: String, queue: LolCareerStatsCareerStatsQueueType) -> GetLolCareerStatsV1ChampionAveragesSeasonBySeasonByChampionIdByPositionByTierByQueue {
    GetLolCareerStatsV1ChampionAveragesSeasonBySeasonByChampionIdByPositionByTierByQueue{season, champion_id, position, tier, queue}
}


pub struct GetLolCareerStatsV1ChampionExpertsByChampionIdByPosition {
    pub champion_id: i32,
    pub position: LolCareerStatsSummonersRiftPosition,
}

impl IsApiRequest for GetLolCareerStatsV1ChampionExpertsByChampionIdByPosition {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCareerStatsExpertPlayer>;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/champion-experts/{}/{}", self.champion_id, serde_json::to_string(&self.position).unwrap())}
}

pub fn get_lol_career_stats_v1_champion_experts_by_champion_id_by_position(champion_id: i32, position: LolCareerStatsSummonersRiftPosition) -> GetLolCareerStatsV1ChampionExpertsByChampionIdByPosition {
    GetLolCareerStatsV1ChampionExpertsByChampionIdByPosition{champion_id, position}
}


pub struct GetLolCareerStatsV1ChampionExpertsSeasonBySeasonByChampionIdByPosition {
    pub season: u32,
    pub champion_id: i32,
    pub position: LolCareerStatsSummonersRiftPosition,
}

impl IsApiRequest for GetLolCareerStatsV1ChampionExpertsSeasonBySeasonByChampionIdByPosition {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCareerStatsExpertPlayer>;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/champion-experts/season/{}/{}/{}", self.season, self.champion_id, serde_json::to_string(&self.position).unwrap())}
}

pub fn get_lol_career_stats_v1_champion_experts_season_by_season_by_champion_id_by_position(season: u32, champion_id: i32, position: LolCareerStatsSummonersRiftPosition) -> GetLolCareerStatsV1ChampionExpertsSeasonBySeasonByChampionIdByPosition {
    GetLolCareerStatsV1ChampionExpertsSeasonBySeasonByChampionIdByPosition{season, champion_id, position}
}


pub struct GetLolCareerStatsV1PositionAveragesByPositionByTierByQueue {
    pub position: LolCareerStatsSummonersRiftPosition,
    pub tier: String,
    pub queue: LolCareerStatsCareerStatsQueueType,
}

impl IsApiRequest for GetLolCareerStatsV1PositionAveragesByPositionByTierByQueue {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCareerStatsChampionQueueStatsResponse;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/position-averages/{}/{}/{}", serde_json::to_string(&self.position).unwrap(), self.tier, serde_json::to_string(&self.queue).unwrap())}
}

pub fn get_lol_career_stats_v1_position_averages_by_position_by_tier_by_queue(position: LolCareerStatsSummonersRiftPosition, tier: String, queue: LolCareerStatsCareerStatsQueueType) -> GetLolCareerStatsV1PositionAveragesByPositionByTierByQueue {
    GetLolCareerStatsV1PositionAveragesByPositionByTierByQueue{position, tier, queue}
}


pub struct GetLolCareerStatsV1PositionAveragesSeasonBySeasonByPositionByTierByQueue {
    pub season: u32,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub tier: String,
    pub queue: LolCareerStatsCareerStatsQueueType,
}

impl IsApiRequest for GetLolCareerStatsV1PositionAveragesSeasonBySeasonByPositionByTierByQueue {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCareerStatsChampionQueueStatsResponse;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/position-averages/season/{}/{}/{}/{}", self.season, serde_json::to_string(&self.position).unwrap(), self.tier, serde_json::to_string(&self.queue).unwrap())}
}

pub fn get_lol_career_stats_v1_position_averages_season_by_season_by_position_by_tier_by_queue(season: u32, position: LolCareerStatsSummonersRiftPosition, tier: String, queue: LolCareerStatsCareerStatsQueueType) -> GetLolCareerStatsV1PositionAveragesSeasonBySeasonByPositionByTierByQueue {
    GetLolCareerStatsV1PositionAveragesSeasonBySeasonByPositionByTierByQueue{season, position, tier, queue}
}


pub struct GetLolCareerStatsV1PositionExpertsByPosition {
    pub position: LolCareerStatsSummonersRiftPosition,
}

impl IsApiRequest for GetLolCareerStatsV1PositionExpertsByPosition {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCareerStatsExpertPlayer>;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/position-experts/{}", serde_json::to_string(&self.position).unwrap())}
}

pub fn get_lol_career_stats_v1_position_experts_by_position(position: LolCareerStatsSummonersRiftPosition) -> GetLolCareerStatsV1PositionExpertsByPosition {
    GetLolCareerStatsV1PositionExpertsByPosition{position}
}


pub struct GetLolCareerStatsV1PositionExpertsSeasonBySeasonByPosition {
    pub season: u32,
    pub position: LolCareerStatsSummonersRiftPosition,
}

impl IsApiRequest for GetLolCareerStatsV1PositionExpertsSeasonBySeasonByPosition {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCareerStatsExpertPlayer>;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/position-experts/season/{}/{}", self.season, serde_json::to_string(&self.position).unwrap())}
}

pub fn get_lol_career_stats_v1_position_experts_season_by_season_by_position(season: u32, position: LolCareerStatsSummonersRiftPosition) -> GetLolCareerStatsV1PositionExpertsSeasonBySeasonByPosition {
    GetLolCareerStatsV1PositionExpertsSeasonBySeasonByPosition{season, position}
}


pub struct GetLolCareerStatsV1SummonerGamesByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolCareerStatsV1SummonerGamesByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/summoner-games/{}", self.puuid)}
}

pub fn get_lol_career_stats_v1_summoner_games_by_puuid(puuid: String) -> GetLolCareerStatsV1SummonerGamesByPuuid {
    GetLolCareerStatsV1SummonerGamesByPuuid{puuid}
}


pub struct GetLolCareerStatsV1SummonerGamesByPuuidSeasonBySeason {
    pub puuid: String,
    pub season: u32,
}

impl IsApiRequest for GetLolCareerStatsV1SummonerGamesByPuuidSeasonBySeason {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/summoner-games/{}/season/{}", self.puuid, self.season)}
}

pub fn get_lol_career_stats_v1_summoner_games_by_puuid_season_by_season(puuid: String, season: u32) -> GetLolCareerStatsV1SummonerGamesByPuuidSeasonBySeason {
    GetLolCareerStatsV1SummonerGamesByPuuidSeasonBySeason{puuid, season}
}


pub struct GetLolCareerStatsV1SummonerStatsByPuuidBySeasonByQueueByPosition {
    pub puuid: String,
    pub season: u32,
    pub queue: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    pub champion_id: Option<i32>,
}

impl IsApiRequest for GetLolCareerStatsV1SummonerStatsByPuuidBySeasonByQueueByPosition {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-career-stats/v1/summoner-stats/{}/{}/{}/{}", self.puuid, self.season, serde_json::to_string(&self.queue).unwrap(), serde_json::to_string(&self.position).unwrap())}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("championId".to_string(), serde_json::to_string(&self.champion_id).unwrap())
        ])
    }
}

pub fn get_lol_career_stats_v1_summoner_stats_by_puuid_by_season_by_queue_by_position(puuid: String, season: u32, queue: LolCareerStatsCareerStatsQueueType, position: LolCareerStatsSummonersRiftPosition, champion_id: Option<i32>) -> GetLolCareerStatsV1SummonerStatsByPuuidBySeasonByQueueByPosition {
    GetLolCareerStatsV1SummonerStatsByPuuidBySeasonByQueueByPosition{puuid, season, queue, position, champion_id}
}


pub struct PostLolCareerStatsV1ChampionStatsPercentiles {
    pub body: Vec<LolCareerStatsStatsQueryRequest>,
}

impl IsApiRequest for PostLolCareerStatsV1ChampionStatsPercentiles {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolCareerStatsStatisticsPercentilesResponse>;
    fn get_url(&self) -> String {"/lol-career-stats/v1/champion-stats-percentiles".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_career_stats_v1_champion_stats_percentiles(body: Vec<LolCareerStatsStatsQueryRequest>) -> PostLolCareerStatsV1ChampionStatsPercentiles {
    PostLolCareerStatsV1ChampionStatsPercentiles{body}
}


pub struct PostLolCareerStatsV1PositionStatsPercentiles {
    pub body: Vec<LolCareerStatsPositionStatsQueryRequest>,
}

impl IsApiRequest for PostLolCareerStatsV1PositionStatsPercentiles {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolCareerStatsStatisticsPercentilesResponse>;
    fn get_url(&self) -> String {"/lol-career-stats/v1/position-stats-percentiles".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_career_stats_v1_position_stats_percentiles(body: Vec<LolCareerStatsPositionStatsQueryRequest>) -> PostLolCareerStatsV1PositionStatsPercentiles {
    PostLolCareerStatsV1PositionStatsPercentiles{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsChampionQueueStatsResponse {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "queueType")]
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    #[serde(rename = "rankTier")]
    pub rank_tier: String,
    pub stats: Value,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsExpertPlayer {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    pub position: LolCareerStatsSummonersRiftPosition,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "numOfGames")]
    pub num_of_games: i32,
    #[serde(rename = "winRate")]
    pub win_rate: f32,
    #[serde(rename = "expertRank")]
    pub expert_rank: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsPositionStatsQueryRequest {
    #[serde(rename = "queueType")]
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    #[serde(rename = "rankTier")]
    pub rank_tier: String,
    pub season: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatisticsPercentilesResponse {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "queueType")]
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    #[serde(rename = "rankTier")]
    pub rank_tier: String,
    pub season: u32,
    pub stats: Value,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatsQueryRequest {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "queueType")]
    pub queue_type: LolCareerStatsCareerStatsQueueType,
    pub position: LolCareerStatsSummonersRiftPosition,
    #[serde(rename = "rankTier")]
    pub rank_tier: String,
    pub season: u32,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolCareerStatsCareerStatsQueueType {
    #[default]
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "quickplay5")]
    Quickplay5,
    #[serde(rename = "rank3flex")]
    Rank3Flex,
    #[serde(rename = "blind3")]
    Blind3,
    #[serde(rename = "aram")]
    Aram,
    #[serde(rename = "blind5")]
    Blind5,
    #[serde(rename = "rank5solo")]
    Rank5Solo,
    #[serde(rename = "rank5flex")]
    Rank5Flex,
    #[serde(rename = "draft5")]
    Draft5,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolCareerStatsSummonersRiftPosition {
    #[default]
    #[serde(rename = "SUPPORT")]
    Support,
    #[serde(rename = "BOTTOM")]
    Bottom,
    #[serde(rename = "MID")]
    Mid,
    #[serde(rename = "JUNGLE")]
    Jungle,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ALL")]
    All,
}

