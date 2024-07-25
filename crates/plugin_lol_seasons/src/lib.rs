
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolSeasonsV1SeasonLOLCurrentSplitSeasons {}

impl IsApiRequest for GetLolSeasonsV1SeasonLOLCurrentSplitSeasons {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i64>;
    fn get_url(&self) -> String {"/lol-seasons/v1/season/LOL/current-split-seasons".to_string()}
}

pub fn get_lol_seasons_v1_season_lol_current_split_seasons() -> GetLolSeasonsV1SeasonLOLCurrentSplitSeasons {
    GetLolSeasonsV1SeasonLOLCurrentSplitSeasons{}
}


pub struct GetLolSeasonsV1SeasonProductByProduct {
    pub product: String,
}

impl IsApiRequest for GetLolSeasonsV1SeasonProductByProduct {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSeasonsAllSeasonsProduct;
    fn get_url(&self) -> String {format!("/lol-seasons/v1/season/product/{}", self.product)}
}

pub fn get_lol_seasons_v1_season_product_by_product(product: String) -> GetLolSeasonsV1SeasonProductByProduct {
    GetLolSeasonsV1SeasonProductByProduct{product}
}


pub struct GetLolSeasonsV1SeasonRecentFinalSplit {}

impl IsApiRequest for GetLolSeasonsV1SeasonRecentFinalSplit {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSeasonsAllSeasonsProduct;
    fn get_url(&self) -> String {"/lol-seasons/v1/season/recent-final-split".to_string()}
}

pub fn get_lol_seasons_v1_season_recent_final_split() -> GetLolSeasonsV1SeasonRecentFinalSplit {
    GetLolSeasonsV1SeasonRecentFinalSplit{}
}


pub struct PostLolSeasonsV1AllSeasonsProductByProduct {
    pub product: String,
    pub body: LolSeasonsAllProductSeasonQuery,
}

impl IsApiRequest for PostLolSeasonsV1AllSeasonsProductByProduct {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolSeasonsAllSeasonsProduct>;
    fn get_url(&self) -> String {format!("/lol-seasons/v1/allSeasons/product/{}", self.product)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_seasons_v1_all_seasons_product_by_product(product: String, body: LolSeasonsAllProductSeasonQuery) -> PostLolSeasonsV1AllSeasonsProductByProduct {
    PostLolSeasonsV1AllSeasonsProductByProduct{product, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsAllProductSeasonQuery {
    #[serde(rename = "lastNYears")]
    pub last_n_years: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsAllSeasonsProduct {
    #[serde(rename = "seasonId")]
    pub season_id: i32,
    #[serde(rename = "seasonStart")]
    pub season_start: i64,
    #[serde(rename = "seasonEnd")]
    pub season_end: i64,
    pub act: bool,
    pub metadata: LolSeasonsSeasonMetaData,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsSeasonMetaData {
    pub year: u16,
    #[serde(rename = "locKey")]
    pub loc_key: String,
    #[serde(rename = "publicName")]
    pub public_name: String,
    #[serde(rename = "currentSplit")]
    pub current_split: i32,
    #[serde(rename = "totalSplit")]
    pub total_split: i32,
}


// ENUMS

