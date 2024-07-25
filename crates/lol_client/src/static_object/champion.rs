use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use toml::Value;
use crate::static_object::Image;


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Champions{
    pub format: String,
    pub version: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data: HashMap<String, Champion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Champion {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Stats {
//     pub hp: i64,
//     pub hpperlevel: i64,
//     pub mp: i64,
//     pub mpperlevel: i64,
//     pub movespeed: i64,
//     pub armor: i64,
//     pub armorperlevel: f64,
//     pub spellblock: i64,
//     pub spellblockperlevel: f64,
//     pub attackrange: i64,
//     pub hpregen: i64,
//     pub hpregenperlevel: f64,
//     pub mpregen: i64,
//     pub mpregenperlevel: i64,
//     pub crit: i64,
//     pub critperlevel: i64,
//     pub attackdamage: i64,
//     pub attackdamageperlevel: f64,
//     pub attackspeedperlevel: f64,
//     pub attackspeed: f64,
// }