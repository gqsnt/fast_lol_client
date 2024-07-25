
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolCollectionsV1InventoriesBySummonerIdBackdrop {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdBackdrop {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCollectionsCollectionsSummonerBackdrop;
    fn get_url(&self) -> String {format!("/lol-collections/v1/inventories/{}/backdrop", self.summoner_id)}
}

pub fn get_lol_collections_v1_inventories_by_summoner_id_backdrop(summoner_id: u64) -> GetLolCollectionsV1InventoriesBySummonerIdBackdrop {
    GetLolCollectionsV1InventoriesBySummonerIdBackdrop{summoner_id}
}


pub struct GetLolCollectionsV1InventoriesBySummonerIdSpells {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdSpells {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCollectionsCollectionsSummonerSpells;
    fn get_url(&self) -> String {format!("/lol-collections/v1/inventories/{}/spells", self.summoner_id)}
}

pub fn get_lol_collections_v1_inventories_by_summoner_id_spells(summoner_id: u64) -> GetLolCollectionsV1InventoriesBySummonerIdSpells {
    GetLolCollectionsV1InventoriesBySummonerIdSpells{summoner_id}
}


pub struct GetLolCollectionsV1InventoriesBySummonerIdWardSkins {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdWardSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCollectionsCollectionsWardSkin>;
    fn get_url(&self) -> String {format!("/lol-collections/v1/inventories/{}/ward-skins", self.summoner_id)}
}

pub fn get_lol_collections_v1_inventories_by_summoner_id_ward_skins(summoner_id: u64) -> GetLolCollectionsV1InventoriesBySummonerIdWardSkins {
    GetLolCollectionsV1InventoriesBySummonerIdWardSkins{summoner_id}
}


pub struct GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId {
    pub summoner_id: u64,
    pub ward_skin_id: i64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCollectionsCollectionsWardSkin;
    fn get_url(&self) -> String {format!("/lol-collections/v1/inventories/{}/ward-skins/{}", self.summoner_id, self.ward_skin_id)}
}

pub fn get_lol_collections_v1_inventories_by_summoner_id_ward_skins_by_ward_skin_id(summoner_id: u64, ward_skin_id: i64) -> GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId {
    GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId{summoner_id, ward_skin_id}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsOwnership {
    #[serde(rename = "loyaltyReward")]
    pub loyalty_reward: bool,
    #[serde(rename = "xboxGPReward")]
    pub xbox_gp_reward: bool,
    pub owned: bool,
    pub rental: LolCollectionsCollectionsRental,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsRental {
    #[serde(rename = "endDate")]
    pub end_date: u64,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: u64,
    #[serde(rename = "winCountRemaining")]
    pub win_count_remaining: i32,
    pub rented: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerBackdrop {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "backdropType")]
    pub backdrop_type: LolCollectionsCollectionsSummonerBackdropType,
    #[serde(rename = "backdropImage")]
    pub backdrop_image: String,
    #[serde(rename = "backdropAugments")]
    pub backdrop_augments: Vec<LolCollectionsCollectionsSummonerBackdropAugments>,
    #[serde(rename = "backdropVideo")]
    pub backdrop_video: String,
    #[serde(rename = "backdropMaskColor")]
    pub backdrop_mask_color: String,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerBackdropAugments {
    #[serde(rename = "centeredLCOverlayPath")]
    pub centered_lc_overlay_path: String,
    #[serde(rename = "socialCardLCOverlayPath")]
    pub social_card_lc_overlay_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerSpells {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub spells: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsWardSkin {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub ownership: LolCollectionsCollectionsOwnership,
    #[serde(rename = "wardImagePath")]
    pub ward_image_path: String,
    #[serde(rename = "wardShadowImagePath")]
    pub ward_shadow_image_path: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolCollectionsCollectionsSummonerBackdropType {
    #[default]
    #[serde(rename = "specified-skin")]
    SpecifiedSkin,
    #[serde(rename = "highest-mastery")]
    HighestMastery,
    #[serde(rename = "summoner-icon")]
    SummonerIcon,
    #[serde(rename = "default")]
    Default,
}

