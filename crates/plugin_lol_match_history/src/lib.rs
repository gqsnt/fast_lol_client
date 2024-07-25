
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolMatchHistoryV1Delta {}

impl IsApiRequest for GetLolMatchHistoryV1Delta {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryPlayerDelta;
    fn get_url(&self) -> String {"/lol-match-history/v1/delta".to_string()}
}

pub fn get_lol_match_history_v1_delta() -> GetLolMatchHistoryV1Delta {
    GetLolMatchHistoryV1Delta{}
}


pub struct GetLolMatchHistoryV1GameTimelinesByGameId {
    pub game_id: u64,
}

impl IsApiRequest for GetLolMatchHistoryV1GameTimelinesByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryTimelineFrames;
    fn get_url(&self) -> String {format!("/lol-match-history/v1/game-timelines/{}", self.game_id)}
}

pub fn get_lol_match_history_v1_game_timelines_by_game_id(game_id: u64) -> GetLolMatchHistoryV1GameTimelinesByGameId {
    GetLolMatchHistoryV1GameTimelinesByGameId{game_id}
}


pub struct GetLolMatchHistoryV1GamesByGameId {
    pub game_id: u64,
}

impl IsApiRequest for GetLolMatchHistoryV1GamesByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryGame;
    fn get_url(&self) -> String {format!("/lol-match-history/v1/games/{}", self.game_id)}
}

pub fn get_lol_match_history_v1_games_by_game_id(game_id: u64) -> GetLolMatchHistoryV1GamesByGameId {
    GetLolMatchHistoryV1GamesByGameId{game_id}
}


pub struct GetLolMatchHistoryV1ProductsLolByPuuidMatches {
    pub puuid: String,
    pub beg_index: Option<u32>,
    pub end_index: Option<u32>,
}

impl IsApiRequest for GetLolMatchHistoryV1ProductsLolByPuuidMatches {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryList;
    fn get_url(&self) -> String {format!("/lol-match-history/v1/products/lol/{}/matches", self.puuid)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("begIndex".to_string(), serde_json::to_string(&self.beg_index).unwrap()),
            ("endIndex".to_string(), serde_json::to_string(&self.end_index).unwrap())
        ])
    }
}

pub fn get_lol_match_history_v1_products_lol_by_puuid_matches(puuid: String, beg_index: Option<u32>, end_index: Option<u32>) -> GetLolMatchHistoryV1ProductsLolByPuuidMatches {
    GetLolMatchHistoryV1ProductsLolByPuuidMatches{puuid, beg_index, end_index}
}


pub struct GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches {
    pub beg_index: Option<u32>,
    pub end_index: Option<u32>,
}

impl IsApiRequest for GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryList;
    fn get_url(&self) -> String {"/lol-match-history/v1/products/lol/current-summoner/matches".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("begIndex".to_string(), serde_json::to_string(&self.beg_index).unwrap()),
            ("endIndex".to_string(), serde_json::to_string(&self.end_index).unwrap())
        ])
    }
}

pub fn get_lol_match_history_v1_products_lol_current_summoner_matches(beg_index: Option<u32>, end_index: Option<u32>) -> GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches {
    GetLolMatchHistoryV1ProductsLolCurrentSummonerMatches{beg_index, end_index}
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
    fn get_url(&self) -> String {format!("/lol-match-history/v1/products/tft/{}/matches", self.puuid)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("begin".to_string(), serde_json::to_string(&self.begin).unwrap()),
            ("count".to_string(), serde_json::to_string(&self.count).unwrap()),
            ("tag".to_string(), serde_json::to_string(&self.tag).unwrap())
        ])
    }
}

pub fn get_lol_match_history_v1_products_tft_by_puuid_matches(puuid: String, begin: Option<u32>, count: Option<u32>, tag: Option<String>) -> GetLolMatchHistoryV1ProductsTftByPuuidMatches {
    GetLolMatchHistoryV1ProductsTftByPuuidMatches{puuid, begin, count, tag}
}


pub struct GetLolMatchHistoryV1RecentlyPlayedSummoners {}

impl IsApiRequest for GetLolMatchHistoryV1RecentlyPlayedSummoners {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolMatchHistoryRecentlyPlayedSummoner>;
    fn get_url(&self) -> String {"/lol-match-history/v1/recently-played-summoners".to_string()}
}

pub fn get_lol_match_history_v1_recently_played_summoners() -> GetLolMatchHistoryV1RecentlyPlayedSummoners {
    GetLolMatchHistoryV1RecentlyPlayedSummoners{}
}


pub struct GetLolMatchHistoryV1WebUrl {}

impl IsApiRequest for GetLolMatchHistoryV1WebUrl {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-match-history/v1/web-url".to_string()}
}

pub fn get_lol_match_history_v1_web_url() -> GetLolMatchHistoryV1WebUrl {
    GetLolMatchHistoryV1WebUrl{}
}


pub struct GetLolMatchHistoryV3MatchlistAccountByAccountId {
    pub account_id: u64,
    pub beg_index: Option<u32>,
    pub end_index: Option<u32>,
}

impl IsApiRequest for GetLolMatchHistoryV3MatchlistAccountByAccountId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMatchHistoryMatchHistoryList;
    fn get_url(&self) -> String {format!("/lol-match-history/v3/matchlist/account/{}", self.account_id)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("begIndex".to_string(), serde_json::to_string(&self.beg_index).unwrap()),
            ("endIndex".to_string(), serde_json::to_string(&self.end_index).unwrap())
        ])
    }
}

pub fn get_lol_match_history_v3_matchlist_account_by_account_id(account_id: u64, beg_index: Option<u32>, end_index: Option<u32>) -> GetLolMatchHistoryV3MatchlistAccountByAccountId {
    GetLolMatchHistoryV3MatchlistAccountByAccountId{account_id, beg_index, end_index}
}


pub struct PostLolMatchHistoryV1AcsEndpointOverride {
    pub body: LolMatchHistoryAcsEndPoint,
}

impl IsApiRequest for PostLolMatchHistoryV1AcsEndpointOverride {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-match-history/v1/acs-endpoint-override".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_match_history_v1_acs_endpoint_override(body: LolMatchHistoryAcsEndPoint) -> PostLolMatchHistoryV1AcsEndpointOverride {
    PostLolMatchHistoryV1AcsEndpointOverride{body}
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
    pub json: Value,
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
    #[serde(rename = "type")]
    pub type_: String,
    pub timestamp: u64,
    #[serde(rename = "participantId")]
    pub participant_id: u16,
    #[serde(rename = "teamId")]
    pub team_id: u16,
    #[serde(rename = "itemId")]
    pub item_id: u16,
    #[serde(rename = "killerId")]
    pub killer_id: u16,
    #[serde(rename = "victimId")]
    pub victim_id: u16,
    #[serde(rename = "skillSlot")]
    pub skill_slot: u16,
    pub position: LolMatchHistoryMatchHistoryPosition,
    #[serde(rename = "assistingParticipantIds")]
    pub assisting_participant_ids: Vec<u16>,
    #[serde(rename = "buildingType")]
    pub building_type: String,
    #[serde(rename = "laneType")]
    pub lane_type: String,
    #[serde(rename = "towerType")]
    pub tower_type: String,
    #[serde(rename = "monsterType")]
    pub monster_type: String,
    #[serde(rename = "monsterSubType")]
    pub monster_sub_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGame {
    #[serde(rename = "endOfGameResult")]
    pub end_of_game_result: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "gameCreation")]
    pub game_creation: u64,
    #[serde(rename = "gameCreationDate")]
    pub game_creation_date: String,
    #[serde(rename = "gameDuration")]
    pub game_duration: u32,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "mapId")]
    pub map_id: u16,
    #[serde(rename = "seasonId")]
    pub season_id: u16,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameType")]
    pub game_type: String,
    pub teams: Vec<LolMatchHistoryMatchHistoryTeam>,
    pub participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    #[serde(rename = "participantIdentities")]
    pub participant_identities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGameList {
    #[serde(rename = "gameIndexBegin")]
    pub game_index_begin: u64,
    #[serde(rename = "gameIndexEnd")]
    pub game_index_end: u64,
    #[serde(rename = "gameBeginDate")]
    pub game_begin_date: String,
    #[serde(rename = "gameEndDate")]
    pub game_end_date: String,
    #[serde(rename = "gameCount")]
    pub game_count: u64,
    pub games: Vec<LolMatchHistoryMatchHistoryGame>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryList {
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    pub games: LolMatchHistoryMatchHistoryGameList,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipant {
    #[serde(rename = "participantId")]
    pub participant_id: u16,
    #[serde(rename = "teamId")]
    pub team_id: u16,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: u16,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: u16,
    #[serde(rename = "highestAchievedSeasonTier")]
    pub highest_achieved_season_tier: String,
    pub stats: LolMatchHistoryMatchHistoryParticipantStatistics,
    pub timeline: LolMatchHistoryMatchHistoryTimeline,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantFrame {
    #[serde(rename = "participantId")]
    pub participant_id: u16,
    pub position: LolMatchHistoryMatchHistoryPosition,
    #[serde(rename = "currentGold")]
    pub current_gold: i32,
    #[serde(rename = "totalGold")]
    pub total_gold: i32,
    pub level: u16,
    pub xp: u32,
    #[serde(rename = "minionsKilled")]
    pub minions_killed: u16,
    #[serde(rename = "jungleMinionsKilled")]
    pub jungle_minions_killed: u16,
    #[serde(rename = "dominionScore")]
    pub dominion_score: u16,
    #[serde(rename = "teamScore")]
    pub team_score: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    #[serde(rename = "participantId")]
    pub participant_id: u16,
    pub player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    pub puuid: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    #[serde(rename = "currentPlatformId")]
    pub current_platform_id: String,
    #[serde(rename = "currentAccountId")]
    pub current_account_id: u64,
    #[serde(rename = "matchHistoryUri")]
    pub match_history_uri: String,
    #[serde(rename = "profileIcon")]
    pub profile_icon: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    #[serde(rename = "participantId")]
    pub participant_id: u16,
    pub win: bool,
    #[serde(rename = "item0")]
    pub item_0: i32,
    #[serde(rename = "item1")]
    pub item_1: i32,
    #[serde(rename = "item2")]
    pub item_2: i32,
    #[serde(rename = "item3")]
    pub item_3: i32,
    #[serde(rename = "item4")]
    pub item_4: i32,
    #[serde(rename = "item5")]
    pub item_5: i32,
    #[serde(rename = "item6")]
    pub item_6: i32,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    #[serde(rename = "largestKillingSpree")]
    pub largest_killing_spree: i64,
    #[serde(rename = "largestMultiKill")]
    pub largest_multi_kill: i64,
    #[serde(rename = "killingSprees")]
    pub killing_sprees: i64,
    #[serde(rename = "longestTimeSpentLiving")]
    pub longest_time_spent_living: i64,
    #[serde(rename = "doubleKills")]
    pub double_kills: i64,
    #[serde(rename = "tripleKills")]
    pub triple_kills: i64,
    #[serde(rename = "quadraKills")]
    pub quadra_kills: i64,
    #[serde(rename = "pentaKills")]
    pub penta_kills: i64,
    #[serde(rename = "unrealKills")]
    pub unreal_kills: i64,
    #[serde(rename = "totalDamageDealt")]
    pub total_damage_dealt: i64,
    #[serde(rename = "magicDamageDealt")]
    pub magic_damage_dealt: i64,
    #[serde(rename = "physicalDamageDealt")]
    pub physical_damage_dealt: i64,
    #[serde(rename = "trueDamageDealt")]
    pub true_damage_dealt: i64,
    #[serde(rename = "largestCriticalStrike")]
    pub largest_critical_strike: i64,
    #[serde(rename = "totalDamageDealtToChampions")]
    pub total_damage_dealt_to_champions: i64,
    #[serde(rename = "magicDamageDealtToChampions")]
    pub magic_damage_dealt_to_champions: i64,
    #[serde(rename = "physicalDamageDealtToChampions")]
    pub physical_damage_dealt_to_champions: i64,
    #[serde(rename = "trueDamageDealtToChampions")]
    pub true_damage_dealt_to_champions: i64,
    #[serde(rename = "totalHeal")]
    pub total_heal: i64,
    #[serde(rename = "totalUnitsHealed")]
    pub total_units_healed: i64,
    #[serde(rename = "totalDamageTaken")]
    pub total_damage_taken: i64,
    #[serde(rename = "magicalDamageTaken")]
    pub magical_damage_taken: i64,
    #[serde(rename = "physicalDamageTaken")]
    pub physical_damage_taken: i64,
    #[serde(rename = "trueDamageTaken")]
    pub true_damage_taken: i64,
    #[serde(rename = "goldEarned")]
    pub gold_earned: i64,
    #[serde(rename = "goldSpent")]
    pub gold_spent: i64,
    #[serde(rename = "turretKills")]
    pub turret_kills: i64,
    #[serde(rename = "inhibitorKills")]
    pub inhibitor_kills: i64,
    #[serde(rename = "totalMinionsKilled")]
    pub total_minions_killed: i64,
    #[serde(rename = "neutralMinionsKilled")]
    pub neutral_minions_killed: i64,
    #[serde(rename = "neutralMinionsKilledTeamJungle")]
    pub neutral_minions_killed_team_jungle: i64,
    #[serde(rename = "neutralMinionsKilledEnemyJungle")]
    pub neutral_minions_killed_enemy_jungle: i64,
    #[serde(rename = "totalTimeCrowdControlDealt")]
    pub total_time_crowd_control_dealt: i64,
    #[serde(rename = "champLevel")]
    pub champ_level: i64,
    #[serde(rename = "visionWardsBoughtInGame")]
    pub vision_wards_bought_in_game: i64,
    #[serde(rename = "sightWardsBoughtInGame")]
    pub sight_wards_bought_in_game: i64,
    #[serde(rename = "wardsPlaced")]
    pub wards_placed: i64,
    #[serde(rename = "wardsKilled")]
    pub wards_killed: i64,
    #[serde(rename = "firstBloodKill")]
    pub first_blood_kill: bool,
    #[serde(rename = "firstBloodAssist")]
    pub first_blood_assist: bool,
    #[serde(rename = "firstTowerKill")]
    pub first_tower_kill: bool,
    #[serde(rename = "firstTowerAssist")]
    pub first_tower_assist: bool,
    #[serde(rename = "firstInhibitorKill")]
    pub first_inhibitor_kill: bool,
    #[serde(rename = "firstInhibitorAssist")]
    pub first_inhibitor_assist: bool,
    #[serde(rename = "gameEndedInEarlySurrender")]
    pub game_ended_in_early_surrender: bool,
    #[serde(rename = "gameEndedInSurrender")]
    pub game_ended_in_surrender: bool,
    #[serde(rename = "causedEarlySurrender")]
    pub caused_early_surrender: bool,
    #[serde(rename = "earlySurrenderAccomplice")]
    pub early_surrender_accomplice: bool,
    #[serde(rename = "teamEarlySurrendered")]
    pub team_early_surrendered: bool,
    #[serde(rename = "combatPlayerScore")]
    pub combat_player_score: i64,
    #[serde(rename = "objectivePlayerScore")]
    pub objective_player_score: i64,
    #[serde(rename = "totalPlayerScore")]
    pub total_player_score: i64,
    #[serde(rename = "totalScoreRank")]
    pub total_score_rank: i64,
    #[serde(rename = "damageSelfMitigated")]
    pub damage_self_mitigated: i64,
    #[serde(rename = "damageDealtToObjectives")]
    pub damage_dealt_to_objectives: i64,
    #[serde(rename = "damageDealtToTurrets")]
    pub damage_dealt_to_turrets: i64,
    #[serde(rename = "visionScore")]
    pub vision_score: i64,
    #[serde(rename = "timeCCingOthers")]
    pub time_c_cing_others: i64,
    #[serde(rename = "playerScore0")]
    pub player_score_0: i64,
    #[serde(rename = "playerScore1")]
    pub player_score_1: i64,
    #[serde(rename = "playerScore2")]
    pub player_score_2: i64,
    #[serde(rename = "playerScore3")]
    pub player_score_3: i64,
    #[serde(rename = "playerScore4")]
    pub player_score_4: i64,
    #[serde(rename = "playerScore5")]
    pub player_score_5: i64,
    #[serde(rename = "playerScore6")]
    pub player_score_6: i64,
    #[serde(rename = "playerScore7")]
    pub player_score_7: i64,
    #[serde(rename = "playerScore8")]
    pub player_score_8: i64,
    #[serde(rename = "playerScore9")]
    pub player_score_9: i64,
    #[serde(rename = "perkPrimaryStyle")]
    pub perk_primary_style: i64,
    #[serde(rename = "perkSubStyle")]
    pub perk_sub_style: i64,
    #[serde(rename = "perk0")]
    pub perk_0: i64,
    #[serde(rename = "perk0Var1")]
    pub perk_0_var_1: i64,
    #[serde(rename = "perk0Var2")]
    pub perk_0_var_2: i64,
    #[serde(rename = "perk0Var3")]
    pub perk_0_var_3: i64,
    #[serde(rename = "perk1")]
    pub perk_1: i64,
    #[serde(rename = "perk1Var1")]
    pub perk_1_var_1: i64,
    #[serde(rename = "perk1Var2")]
    pub perk_1_var_2: i64,
    #[serde(rename = "perk1Var3")]
    pub perk_1_var_3: i64,
    #[serde(rename = "perk2")]
    pub perk_2: i64,
    #[serde(rename = "perk2Var1")]
    pub perk_2_var_1: i64,
    #[serde(rename = "perk2Var2")]
    pub perk_2_var_2: i64,
    #[serde(rename = "perk2Var3")]
    pub perk_2_var_3: i64,
    #[serde(rename = "perk3")]
    pub perk_3: i64,
    #[serde(rename = "perk3Var1")]
    pub perk_3_var_1: i64,
    #[serde(rename = "perk3Var2")]
    pub perk_3_var_2: i64,
    #[serde(rename = "perk3Var3")]
    pub perk_3_var_3: i64,
    #[serde(rename = "perk4")]
    pub perk_4: i64,
    #[serde(rename = "perk4Var1")]
    pub perk_4_var_1: i64,
    #[serde(rename = "perk4Var2")]
    pub perk_4_var_2: i64,
    #[serde(rename = "perk4Var3")]
    pub perk_4_var_3: i64,
    #[serde(rename = "perk5")]
    pub perk_5: i64,
    #[serde(rename = "perk5Var1")]
    pub perk_5_var_1: i64,
    #[serde(rename = "perk5Var2")]
    pub perk_5_var_2: i64,
    #[serde(rename = "perk5Var3")]
    pub perk_5_var_3: i64,
    #[serde(rename = "playerAugment1")]
    pub player_augment_1: i32,
    #[serde(rename = "playerAugment2")]
    pub player_augment_2: i32,
    #[serde(rename = "playerAugment3")]
    pub player_augment_3: i32,
    #[serde(rename = "playerAugment4")]
    pub player_augment_4: i32,
    #[serde(rename = "playerAugment5")]
    pub player_augment_5: i32,
    #[serde(rename = "playerAugment6")]
    pub player_augment_6: i32,
    #[serde(rename = "playerSubteamId")]
    pub player_subteam_id: i32,
    #[serde(rename = "subteamPlacement")]
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
    #[serde(rename = "originalAccountId")]
    pub original_account_id: u64,
    #[serde(rename = "originalPlatformId")]
    pub original_platform_id: String,
    pub deltas: Vec<LolMatchHistoryMatchHistoryPlayerGameDelta>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerGameDelta {
    #[serde(rename = "gamePlatformId")]
    pub game_platform_id: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "platformDelta")]
    pub platform_delta: LolMatchHistoryMatchHistoryPlayerPlatformDelta,
    #[serde(rename = "leagueDelta")]
    pub league_delta: LolMatchHistoryMatchHistoryPlayerLeagueDelta,
    #[serde(rename = "champMastery")]
    pub champ_mastery: LolMatchHistoryMatchHistoryPlayerChampMasteryDelta,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerLeagueDelta {
    #[serde(rename = "leaguePointDelta")]
    pub league_point_delta: u64,
    pub reason: String,
    #[serde(rename = "miniSeriesProgress")]
    pub mini_series_progress: Vec<String>,
    pub timestamp: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerPlatformDelta {
    #[serde(rename = "xpDelta")]
    pub xp_delta: u64,
    #[serde(rename = "ipDelta")]
    pub ip_delta: u64,
    #[serde(rename = "compensationModeEnabled")]
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
    #[serde(rename = "teamId")]
    pub team_id: u16,
    pub win: String,
    #[serde(rename = "firstBlood")]
    pub first_blood: bool,
    #[serde(rename = "firstTower")]
    pub first_tower: bool,
    #[serde(rename = "firstInhibitor")]
    pub first_inhibitor: bool,
    #[serde(rename = "firstBaron")]
    pub first_baron: bool,
    #[serde(rename = "firstDargon")]
    pub first_dargon: bool,
    #[serde(rename = "towerKills")]
    pub tower_kills: u32,
    #[serde(rename = "inhibitorKills")]
    pub inhibitor_kills: u32,
    #[serde(rename = "baronKills")]
    pub baron_kills: u32,
    #[serde(rename = "dragonKills")]
    pub dragon_kills: u32,
    #[serde(rename = "vilemawKills")]
    pub vilemaw_kills: u32,
    #[serde(rename = "riftHeraldKills")]
    pub rift_herald_kills: u32,
    #[serde(rename = "hordeKills")]
    pub horde_kills: u32,
    #[serde(rename = "dominionVictoryScore")]
    pub dominion_victory_score: u32,
    pub bans: Vec<LolMatchHistoryMatchHistoryTeamBan>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "pickTurn")]
    pub pick_turn: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimeline {
    #[serde(rename = "participantId")]
    pub participant_id: u16,
    pub role: String,
    pub lane: String,
    #[serde(rename = "creepsPerMinDeltas")]
    pub creeps_per_min_deltas: HashMap<String, f64>,
    #[serde(rename = "xpPerMinDeltas")]
    pub xp_per_min_deltas: HashMap<String, f64>,
    #[serde(rename = "goldPerMinDeltas")]
    pub gold_per_min_deltas: HashMap<String, f64>,
    #[serde(rename = "csDiffPerMinDeltas")]
    pub cs_diff_per_min_deltas: HashMap<String, f64>,
    #[serde(rename = "xpDiffPerMinDeltas")]
    pub xp_diff_per_min_deltas: HashMap<String, f64>,
    #[serde(rename = "damageTakenPerMinDeltas")]
    pub damage_taken_per_min_deltas: HashMap<String, f64>,
    #[serde(rename = "damageTakenDiffPerMinDeltas")]
    pub damage_taken_diff_per_min_deltas: HashMap<String, f64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimelineFrame {
    #[serde(rename = "participantFrames")]
    pub participant_frames: HashMap<String, LolMatchHistoryMatchHistoryParticipantFrame>,
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
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "gameCreationDate")]
    pub game_creation_date: String,
    #[serde(rename = "championId")]
    pub champion_id: u64,
    #[serde(rename = "teamId")]
    pub team_id: u64,
    pub puuid: String,
}


// ENUMS

