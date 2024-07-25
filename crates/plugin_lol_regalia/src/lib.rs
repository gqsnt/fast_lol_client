
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolRegaliaV2Config {}

impl IsApiRequest for GetLolRegaliaV2Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRegaliaRegaliaFrontendConfig;
    fn get_url(&self) -> String {"/lol-regalia/v2/config".to_string()}
}

pub fn get_lol_regalia_v2_config() -> GetLolRegaliaV2Config {
    GetLolRegaliaV2Config{}
}


pub struct GetLolRegaliaV2CurrentSummonerRegalia {}

impl IsApiRequest for GetLolRegaliaV2CurrentSummonerRegalia {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRegaliaRegaliaWithPreferences;
    fn get_url(&self) -> String {"/lol-regalia/v2/current-summoner/regalia".to_string()}
}

pub fn get_lol_regalia_v2_current_summoner_regalia() -> GetLolRegaliaV2CurrentSummonerRegalia {
    GetLolRegaliaV2CurrentSummonerRegalia{}
}


pub struct GetLolRegaliaV2SummonersBySummonerIdQueuesByQueuePositionsByPositionRegalia {
    pub summoner_id: u64,
    pub queue: String,
    pub position: String,
}

impl IsApiRequest for GetLolRegaliaV2SummonersBySummonerIdQueuesByQueuePositionsByPositionRegalia {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRegaliaRegalia;
    fn get_url(&self) -> String {format!("/lol-regalia/v2/summoners/{}/queues/{}/positions/{}/regalia", self.summoner_id, self.queue, self.position)}
}

pub fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia(summoner_id: u64, queue: String, position: String) -> GetLolRegaliaV2SummonersBySummonerIdQueuesByQueuePositionsByPositionRegalia {
    GetLolRegaliaV2SummonersBySummonerIdQueuesByQueuePositionsByPositionRegalia{summoner_id, queue, position}
}


pub struct GetLolRegaliaV2SummonersBySummonerIdQueuesByQueueRegalia {
    pub summoner_id: u64,
    pub queue: String,
}

impl IsApiRequest for GetLolRegaliaV2SummonersBySummonerIdQueuesByQueueRegalia {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRegaliaRegalia;
    fn get_url(&self) -> String {format!("/lol-regalia/v2/summoners/{}/queues/{}/regalia", self.summoner_id, self.queue)}
}

pub fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia(summoner_id: u64, queue: String) -> GetLolRegaliaV2SummonersBySummonerIdQueuesByQueueRegalia {
    GetLolRegaliaV2SummonersBySummonerIdQueuesByQueueRegalia{summoner_id, queue}
}


pub struct GetLolRegaliaV2SummonersBySummonerIdRegalia {
    pub summoner_id: u64,
    pub hovercard: bool,
}

impl IsApiRequest for GetLolRegaliaV2SummonersBySummonerIdRegalia {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRegaliaRegalia;
    fn get_url(&self) -> String {format!("/lol-regalia/v2/summoners/{}/regalia", self.summoner_id)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("hovercard".to_string(), serde_json::to_string(&self.hovercard).unwrap())
        ])
    }
}

pub fn get_lol_regalia_v2_summoners_by_summoner_id_regalia(summoner_id: u64, hovercard: bool) -> GetLolRegaliaV2SummonersBySummonerIdRegalia {
    GetLolRegaliaV2SummonersBySummonerIdRegalia{summoner_id, hovercard}
}


pub struct GetLolRegaliaV2SummonersBySummonerIdRegaliaAsync {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolRegaliaV2SummonersBySummonerIdRegaliaAsync {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRegaliaRegaliaAsync;
    fn get_url(&self) -> String {format!("/lol-regalia/v2/summoners/{}/regalia/async", self.summoner_id)}
}

pub fn get_lol_regalia_v2_summoners_by_summoner_id_regalia_async(summoner_id: u64) -> GetLolRegaliaV2SummonersBySummonerIdRegaliaAsync {
    GetLolRegaliaV2SummonersBySummonerIdRegaliaAsync{summoner_id}
}


pub struct GetLolRegaliaV3InventoryByInventoryType {
    pub inventory_type: String,
}

impl IsApiRequest for GetLolRegaliaV3InventoryByInventoryType {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, LolRegaliaRegaliaInventoryItem>;
    fn get_url(&self) -> String {format!("/lol-regalia/v3/inventory/{}", self.inventory_type)}
}

pub fn get_lol_regalia_v3_inventory_by_inventory_type(inventory_type: String) -> GetLolRegaliaV3InventoryByInventoryType {
    GetLolRegaliaV3InventoryByInventoryType{inventory_type}
}


pub struct GetLolRegaliaV3SummonersBySummonerIdRegalia {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolRegaliaV3SummonersBySummonerIdRegalia {
    const METHOD: Method = Method::GET;
    type ReturnType = LolRegaliaRegalia;
    fn get_url(&self) -> String {format!("/lol-regalia/v3/summoners/{}/regalia", self.summoner_id)}
}

pub fn get_lol_regalia_v3_summoners_by_summoner_id_regalia(summoner_id: u64) -> GetLolRegaliaV3SummonersBySummonerIdRegalia {
    GetLolRegaliaV3SummonersBySummonerIdRegalia{summoner_id}
}


pub struct PutLolRegaliaV2CurrentSummonerRegalia {
    pub body: LolRegaliaRegaliaPreferences,
}

impl IsApiRequest for PutLolRegaliaV2CurrentSummonerRegalia {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolRegaliaRegaliaWithPreferences;
    fn get_url(&self) -> String {"/lol-regalia/v2/current-summoner/regalia".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_regalia_v2_current_summoner_regalia(body: LolRegaliaRegaliaPreferences) -> PutLolRegaliaV2CurrentSummonerRegalia {
    PutLolRegaliaV2CurrentSummonerRegalia{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaGameDataRegalia {
    pub id: String,
    #[serde(rename = "idSecondary")]
    pub id_secondary: String,
    #[serde(rename = "assetPath")]
    pub asset_path: String,
    #[serde(rename = "isSelectable")]
    pub is_selectable: bool,
    #[serde(rename = "regaliaType")]
    pub regalia_type: String,
    #[serde(rename = "localizedName")]
    pub localized_name: String,
    #[serde(rename = "localizedDescription")]
    pub localized_description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegalia {
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename = "crestType")]
    pub crest_type: String,
    #[serde(rename = "bannerType")]
    pub banner_type: String,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: u32,
    #[serde(rename = "lastSeasonHighestRank")]
    pub last_season_highest_rank: Option<String>,
    #[serde(rename = "highestRankedEntry")]
    pub highest_ranked_entry: Option<LolRegaliaRegaliaRankedEntry>,
    #[serde(rename = "selectedPrestigeCrest")]
    pub selected_prestige_crest: u8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaAsync {
    #[serde(rename = "md5")]
    pub md_5: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaFrontendConfig {
    #[serde(rename = "hovercardEnabled")]
    pub hovercard_enabled: bool,
    #[serde(rename = "selectionsEnabled")]
    pub selections_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaInventoryItem {
    pub items: Vec<LolRegaliaGameDataRegalia>,
    #[serde(rename = "isOwned")]
    pub is_owned: bool,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaPreferences {
    #[serde(rename = "preferredCrestType")]
    pub preferred_crest_type: String,
    #[serde(rename = "preferredBannerType")]
    pub preferred_banner_type: String,
    #[serde(rename = "selectedPrestigeCrest")]
    pub selected_prestige_crest: u8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaRankedEntry {
    #[serde(rename = "queueType")]
    pub queue_type: LolRegaliaLeagueQueueType,
    pub tier: String,
    pub division: LolRegaliaLeagueDivision,
    #[serde(rename = "splitRewardLevel")]
    pub split_reward_level: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaWithPreferences {
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename = "crestType")]
    pub crest_type: String,
    #[serde(rename = "bannerType")]
    pub banner_type: String,
    #[serde(rename = "preferredCrestType")]
    pub preferred_crest_type: String,
    #[serde(rename = "preferredBannerType")]
    pub preferred_banner_type: String,
    #[serde(rename = "selectedPrestigeCrest")]
    pub selected_prestige_crest: u8,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: u32,
    #[serde(rename = "lastSeasonHighestRank")]
    pub last_season_highest_rank: Option<String>,
    #[serde(rename = "highestRankedEntry")]
    pub highest_ranked_entry: Option<LolRegaliaRegaliaRankedEntry>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolRegaliaLeagueDivision {
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
pub enum LolRegaliaLeagueQueueType {
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

