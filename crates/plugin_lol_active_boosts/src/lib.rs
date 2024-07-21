
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolActiveBoostsV1ActiveBoosts {}

impl IsApiRequest for GetLolActiveBoostsV1ActiveBoosts {
    const METHOD: Method = Method::GET;
    type ReturnType = LolActiveBoostsActiveBoosts;
    fn get_url(&self) -> String {"/lol-active-boosts/v1/active-boosts".to_string()}
}

pub fn get_lol_active_boosts_v_1_active_boosts() -> GetLolActiveBoostsV1ActiveBoosts {
    GetLolActiveBoostsV1ActiveBoosts{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsActiveBoosts {
    pub xp_boost_end_date: String,
    pub xp_boost_per_win_count: u64,
    pub xp_loyalty_boost: i32,
    pub first_win_of_the_day_start_time: String,
}


// ENUMS

