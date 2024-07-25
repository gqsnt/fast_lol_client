
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolDropsV1DropTables {}

impl IsApiRequest for GetLolDropsV1DropTables {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolDropsCapDropsDropTableWithPityDto>;
    fn get_url(&self) -> String {"/lol-drops/v1/drop-tables".to_string()}
}

pub fn get_lol_drops_v1_drop_tables() -> GetLolDropsV1DropTables {
    GetLolDropsV1DropTables{}
}


pub struct GetLolDropsV1DropTablesByDropTableId {
    pub drop_table_id: String,
}

impl IsApiRequest for GetLolDropsV1DropTablesByDropTableId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolDropsCapDropsDropTableWithPityDto;
    fn get_url(&self) -> String {format!("/lol-drops/v1/drop-tables/{}", self.drop_table_id)}
}

pub fn get_lol_drops_v1_drop_tables_by_drop_table_id(drop_table_id: String) -> GetLolDropsV1DropTablesByDropTableId {
    GetLolDropsV1DropTablesByDropTableId{drop_table_id}
}


pub struct GetLolDropsV1DropTablesByDropTableIdOddsList {
    pub drop_table_id: String,
}

impl IsApiRequest for GetLolDropsV1DropTablesByDropTableIdOddsList {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolDropsCapDropsOddsListEntryDto>;
    fn get_url(&self) -> String {format!("/lol-drops/v1/drop-tables/{}/odds-list", self.drop_table_id)}
}

pub fn get_lol_drops_v1_drop_tables_by_drop_table_id_odds_list(drop_table_id: String) -> GetLolDropsV1DropTablesByDropTableIdOddsList {
    GetLolDropsV1DropTablesByDropTableIdOddsList{drop_table_id}
}


pub struct GetLolDropsV1DropTablesByDropTableIdOddsTree {
    pub drop_table_id: String,
}

impl IsApiRequest for GetLolDropsV1DropTablesByDropTableIdOddsTree {
    const METHOD: Method = Method::GET;
    type ReturnType = LolDropsCapDropsOddsTreeNodeDto;
    fn get_url(&self) -> String {format!("/lol-drops/v1/drop-tables/{}/odds-tree", self.drop_table_id)}
}

pub fn get_lol_drops_v1_drop_tables_by_drop_table_id_odds_tree(drop_table_id: String) -> GetLolDropsV1DropTablesByDropTableIdOddsTree {
    GetLolDropsV1DropTablesByDropTableIdOddsTree{drop_table_id}
}


pub struct GetLolDropsV1DropTablesByDropTableIdPlayersByPlayerIdPityCount {
    pub drop_table_id: String,
    pub player_id: String,
}

impl IsApiRequest for GetLolDropsV1DropTablesByDropTableIdPlayersByPlayerIdPityCount {
    const METHOD: Method = Method::GET;
    type ReturnType = LolDropsCapDropTableCounterDto;
    fn get_url(&self) -> String {format!("/lol-drops/v1/drop-tables/{}/players/{}/pity-count", self.drop_table_id, self.player_id)}
}

pub fn get_lol_drops_v1_drop_tables_by_drop_table_id_players_by_player_id_pity_count(drop_table_id: String, player_id: String) -> GetLolDropsV1DropTablesByDropTableIdPlayersByPlayerIdPityCount {
    GetLolDropsV1DropTablesByDropTableIdPlayersByPlayerIdPityCount{drop_table_id, player_id}
}


pub struct GetLolDropsV1PlayersByPlayerIdPityCounts {
    pub player_id: String,
}

impl IsApiRequest for GetLolDropsV1PlayersByPlayerIdPityCounts {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolDropsCapDropTableCounterDto>;
    fn get_url(&self) -> String {format!("/lol-drops/v1/players/{}/pity-counts", self.player_id)}
}

pub fn get_lol_drops_v1_players_by_player_id_pity_counts(player_id: String) -> GetLolDropsV1PlayersByPlayerIdPityCounts {
    GetLolDropsV1PlayersByPlayerIdPityCounts{player_id}
}


pub struct GetLolDropsV1PlayersByPlayerIdTotalRollsCounts {
    pub player_id: String,
}

impl IsApiRequest for GetLolDropsV1PlayersByPlayerIdTotalRollsCounts {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolDropsCapDropTableCounterDto>;
    fn get_url(&self) -> String {format!("/lol-drops/v1/players/{}/total-rolls-counts", self.player_id)}
}

pub fn get_lol_drops_v1_players_by_player_id_total_rolls_counts(player_id: String) -> GetLolDropsV1PlayersByPlayerIdTotalRollsCounts {
    GetLolDropsV1PlayersByPlayerIdTotalRollsCounts{player_id}
}


pub struct GetLolDropsV1Ready {}

impl IsApiRequest for GetLolDropsV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-drops/v1/ready".to_string()}
}

pub fn get_lol_drops_v1_ready() -> GetLolDropsV1Ready {
    GetLolDropsV1Ready{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropTableCounterDto {
    #[serde(rename = "dropTableId")]
    pub drop_table_id: String,
    pub count: u8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsDropTableDisplayMetadata {
    #[serde(rename = "isCollectorsBounty")]
    pub is_collectors_bounty: bool,
    #[serde(rename = "dataAssetId")]
    pub data_asset_id: String,
    #[serde(rename = "nameTraKey")]
    pub name_tra_key: String,
    #[serde(rename = "mythicOfferId")]
    pub mythic_offer_id: String,
    #[serde(rename = "progressionId")]
    pub progression_id: String,
    pub priority: u8,
    pub tables: HashMap<String, LolDropsOddsTableDisplayMetadata>,
    pub version: u8,
    #[serde(rename = "chaseContentId")]
    pub chase_content_id: String,
    #[serde(rename = "oddsTree")]
    pub odds_tree: LolDropsCapDropsOddsTreeNodeDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsDropTablePityInfo {
    #[serde(rename = "pityLimit")]
    pub pity_limit: u8,
    #[serde(rename = "chaseContentIds")]
    pub chase_content_ids: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsDropTableWithPityDto {
    pub id: String,
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "currencyId")]
    pub currency_id: String,
    #[serde(rename = "rollOffer")]
    pub roll_offer: String,
    pub cost: u16,
    #[serde(rename = "totalRollsInfo")]
    pub total_rolls_info: LolDropsTotalRollsInfoDto,
    #[serde(rename = "pityInfo")]
    pub pity_info: LolDropsCapDropsDropTablePityInfo,
    #[serde(rename = "displayMetadata")]
    pub display_metadata: LolDropsCapDropsDropTableDisplayMetadata,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsOddsListEntryDto {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub odds: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsOddsTreeNodeDto {
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub odds: f32,
    pub children: Vec<LolDropsCapDropsOddsTreeNodeDto>,
    pub quantity: u16,
    #[serde(rename = "nameTraKey")]
    pub name_tra_key: String,
    pub priority: u8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsOddsTableDisplayMetadata {
    #[serde(rename = "nameTraKey")]
    pub name_tra_key: String,
    pub priority: u8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsTotalRollsInfoDto {
    #[serde(rename = "totalRollsCounterId")]
    pub total_rolls_counter_id: String,
    #[serde(rename = "maxTotalRolls")]
    pub max_total_rolls: u8,
}


// ENUMS

