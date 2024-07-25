
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolMapsV1MapById {
    pub id: i64,
}

impl IsApiRequest for GetLolMapsV1MapById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMapsMaps;
    fn get_url(&self) -> String {format!("/lol-maps/v1/map/{}", self.id)}
}

pub fn get_lol_maps_v1_map_by_id(id: i64) -> GetLolMapsV1MapById {
    GetLolMapsV1MapById{id}
}


pub struct GetLolMapsV1Maps {}

impl IsApiRequest for GetLolMapsV1Maps {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolMapsMaps>;
    fn get_url(&self) -> String {"/lol-maps/v1/maps".to_string()}
}

pub fn get_lol_maps_v1_maps() -> GetLolMapsV1Maps {
    GetLolMapsV1Maps{}
}


pub struct GetLolMapsV2MapByIdByGameMode {
    pub id: i64,
    pub game_mode: String,
}

impl IsApiRequest for GetLolMapsV2MapByIdByGameMode {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMapsMaps;
    fn get_url(&self) -> String {format!("/lol-maps/v2/map/{}/{}", self.id, self.game_mode)}
}

pub fn get_lol_maps_v2_map_by_id_by_game_mode(id: i64, game_mode: String) -> GetLolMapsV2MapByIdByGameMode {
    GetLolMapsV2MapByIdByGameMode{id, game_mode}
}


pub struct GetLolMapsV2MapByIdByGameModeByGameMutator {
    pub id: i64,
    pub game_mode: String,
    pub game_mutator: String,
}

impl IsApiRequest for GetLolMapsV2MapByIdByGameModeByGameMutator {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMapsMaps;
    fn get_url(&self) -> String {format!("/lol-maps/v2/map/{}/{}/{}", self.id, self.game_mode, self.game_mutator)}
}

pub fn get_lol_maps_v2_map_by_id_by_game_mode_by_game_mutator(id: i64, game_mode: String, game_mutator: String) -> GetLolMapsV2MapByIdByGameModeByGameMutator {
    GetLolMapsV2MapByIdByGameModeByGameMutator{id, game_mode, game_mutator}
}


pub struct GetLolMapsV2Maps {}

impl IsApiRequest for GetLolMapsV2Maps {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolMapsMaps>;
    fn get_url(&self) -> String {"/lol-maps/v2/maps".to_string()}
}

pub fn get_lol_maps_v2_maps() -> GetLolMapsV2Maps {
    GetLolMapsV2Maps{}
}


pub struct PostLolMapsV1Map {
    pub body: LolMapsMaps,
}

impl IsApiRequest for PostLolMapsV1Map {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-maps/v1/map".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_maps_v1_map(body: LolMapsMaps) -> PostLolMapsV1Map {
    PostLolMapsV1Map{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMapsGameModeSpellList {
    pub spells: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMapsMaps {
    pub id: i64,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameModeName")]
    pub game_mode_name: String,
    #[serde(rename = "gameModeShortName")]
    pub game_mode_short_name: String,
    #[serde(rename = "gameModeDescription")]
    pub game_mode_description: String,
    #[serde(rename = "gameMutator")]
    pub game_mutator: String,
    #[serde(rename = "isRGM")]
    pub is_rgm: bool,
    pub name: String,
    pub description: String,
    #[serde(rename = "mapStringId")]
    pub map_string_id: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "platformName")]
    pub platform_name: String,
    pub assets: HashMap<String, String>,
    #[serde(rename = "locStrings")]
    pub loc_strings: HashMap<String, String>,
    #[serde(rename = "categorizedContentBundles")]
    pub categorized_content_bundles: Value,
    #[serde(rename = "tutorialCards")]
    pub tutorial_cards: Vec<LolMapsTutorialCard>,
    pub properties: Value,
    #[serde(rename = "perPositionRequiredSummonerSpells")]
    pub per_position_required_summoner_spells: HashMap<String, LolMapsGameModeSpellList>,
    #[serde(rename = "perPositionDisallowedSummonerSpells")]
    pub per_position_disallowed_summoner_spells: HashMap<String, LolMapsGameModeSpellList>,
    #[serde(rename = "tftSetOverride")]
    pub tft_set_override: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMapsTutorialCard {
    pub header: Option<String>,
    pub footer: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "imagePath")]
    pub image_path: String,
}


// ENUMS

