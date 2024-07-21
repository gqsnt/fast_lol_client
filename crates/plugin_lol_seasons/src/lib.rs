
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
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

pub fn get_lol_seasons_v_1_season_lol_current_split_seasons() -> GetLolSeasonsV1SeasonLOLCurrentSplitSeasons {
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

pub fn get_lol_seasons_v_1_season_product_by_product(product: String) -> GetLolSeasonsV1SeasonProductByProduct {
    GetLolSeasonsV1SeasonProductByProduct{product}
}


pub struct GetLolSeasonsV1SeasonRecentFinalSplit {}

impl IsApiRequest for GetLolSeasonsV1SeasonRecentFinalSplit {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSeasonsAllSeasonsProduct;
    fn get_url(&self) -> String {"/lol-seasons/v1/season/recent-final-split".to_string()}
}

pub fn get_lol_seasons_v_1_season_recent_final_split() -> GetLolSeasonsV1SeasonRecentFinalSplit {
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

pub fn post_lol_seasons_v_1_all_seasons_product_by_product(product: String, body: LolSeasonsAllProductSeasonQuery) -> PostLolSeasonsV1AllSeasonsProductByProduct {
    PostLolSeasonsV1AllSeasonsProductByProduct{product, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsAllProductSeasonQuery {
    pub last_n_years: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsAllSeasonsProduct {
    pub season_id: i32,
    pub season_start: i64,
    pub season_end: i64,
    pub act: bool,
    pub metadata: LolSeasonsSeasonMetaData,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsSeasonMetaData {
    pub year: u16,
    pub loc_key: String,
    pub public_name: String,
    pub current_split: i32,
    pub total_split: i32,
}


// ENUMS

