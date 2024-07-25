
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolItemSetsV1ItemSetsBySummonerIdSets {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolItemSetsV1ItemSetsBySummonerIdSets {
    const METHOD: Method = Method::GET;
    type ReturnType = LolItemSetsItemSets;
    fn get_url(&self) -> String {format!("/lol-item-sets/v1/item-sets/{}/sets", self.summoner_id)}
}

pub fn get_lol_item_sets_v1_item_sets_by_summoner_id_sets(summoner_id: u64) -> GetLolItemSetsV1ItemSetsBySummonerIdSets {
    GetLolItemSetsV1ItemSetsBySummonerIdSets{summoner_id}
}


pub struct PostLolItemSetsV1ItemSetsBySummonerIdSets {
    pub summoner_id: u64,
    pub body: LolItemSetsItemSet,
}

impl IsApiRequest for PostLolItemSetsV1ItemSetsBySummonerIdSets {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-item-sets/v1/item-sets/{}/sets", self.summoner_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_item_sets_v1_item_sets_by_summoner_id_sets(summoner_id: u64, body: LolItemSetsItemSet) -> PostLolItemSetsV1ItemSetsBySummonerIdSets {
    PostLolItemSetsV1ItemSetsBySummonerIdSets{summoner_id, body}
}


pub struct PostLolItemSetsV1ItemSetsValidate {
    pub body: LolItemSetsValidateItemSetNameInput,
}

impl IsApiRequest for PostLolItemSetsV1ItemSetsValidate {
    const METHOD: Method = Method::POST;
    type ReturnType = LolItemSetsValidateItemSetNameResponse;
    fn get_url(&self) -> String {"/lol-item-sets/v1/item-sets/validate".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_item_sets_v1_item_sets_validate(body: LolItemSetsValidateItemSetNameInput) -> PostLolItemSetsV1ItemSetsValidate {
    PostLolItemSetsV1ItemSetsValidate{body}
}


pub struct PutLolItemSetsV1ItemSetsBySummonerIdSets {
    pub summoner_id: u64,
    pub body: LolItemSetsItemSets,
}

impl IsApiRequest for PutLolItemSetsV1ItemSetsBySummonerIdSets {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-item-sets/v1/item-sets/{}/sets", self.summoner_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_item_sets_v1_item_sets_by_summoner_id_sets(summoner_id: u64, body: LolItemSetsItemSets) -> PutLolItemSetsV1ItemSetsBySummonerIdSets {
    PutLolItemSetsV1ItemSetsBySummonerIdSets{summoner_id, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSet {
    pub uid: String,
    pub title: String,
    pub mode: String,
    pub map: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub sortrank: i32,
    #[serde(rename = "startedFrom")]
    pub started_from: String,
    #[serde(rename = "associatedChampions")]
    pub associated_champions: Vec<i32>,
    #[serde(rename = "associatedMaps")]
    pub associated_maps: Vec<i32>,
    pub blocks: Vec<LolItemSetsItemSetBlock>,
    #[serde(rename = "preferredItemSlots")]
    pub preferred_item_slots: Vec<LolItemSetsPreferredItemSlot>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSetBlock {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "hideIfSummonerSpell")]
    pub hide_if_summoner_spell: String,
    #[serde(rename = "showIfSummonerSpell")]
    pub show_if_summoner_spell: String,
    pub items: Vec<LolItemSetsItemSetItem>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSetItem {
    pub id: String,
    pub count: u16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSets {
    pub timestamp: u64,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "itemSets")]
    pub item_sets: Vec<LolItemSetsItemSet>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsNamecheckResponse {
    pub errors: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsPreferredItemSlot {
    pub id: String,
    #[serde(rename = "preferredItemSlot")]
    pub preferred_item_slot: i16,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsValidateItemSetNameInput {
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsValidateItemSetNameResponse {
    pub success: bool,
    #[serde(rename = "nameCheckResponse")]
    pub name_check_response: LolItemSetsNamecheckResponse,
}


// ENUMS

