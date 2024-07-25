use serde::{Deserialize, Serialize};

pub mod champion;
pub mod profile_icon;
pub mod summoner_spell;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}
