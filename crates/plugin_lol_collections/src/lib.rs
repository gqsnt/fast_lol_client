
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolCollectionsV1InventoriesBySummonerIdBackdrop {

    pub summoner_id: u64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdBackdrop {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCollectionsCollectionsSummonerBackdrop;

    fn get_url(&self) -> String {
        format!("/lol-collections/v1/inventories/{}/backdrop", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_collections_v_1_inventories_by_summoner_id_backdrop(summoner_id: u64) -> GetLolCollectionsV1InventoriesBySummonerIdBackdrop {
    GetLolCollectionsV1InventoriesBySummonerIdBackdrop {
        summoner_id
    }
}


pub struct GetLolCollectionsV1InventoriesBySummonerIdSpells {

    pub summoner_id: u64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdSpells {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCollectionsCollectionsSummonerSpells;

    fn get_url(&self) -> String {
        format!("/lol-collections/v1/inventories/{}/spells", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_collections_v_1_inventories_by_summoner_id_spells(summoner_id: u64) -> GetLolCollectionsV1InventoriesBySummonerIdSpells {
    GetLolCollectionsV1InventoriesBySummonerIdSpells {
        summoner_id
    }
}


pub struct GetLolCollectionsV1InventoriesBySummonerIdWardSkins {

    pub summoner_id: u64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdWardSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCollectionsCollectionsWardSkin>;

    fn get_url(&self) -> String {
        format!("/lol-collections/v1/inventories/{}/ward-skins", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_collections_v_1_inventories_by_summoner_id_ward_skins(summoner_id: u64) -> GetLolCollectionsV1InventoriesBySummonerIdWardSkins {
    GetLolCollectionsV1InventoriesBySummonerIdWardSkins {
        summoner_id
    }
}


pub struct GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId {

    pub summoner_id: u64,
    pub ward_skin_id: i64,
}

impl IsApiRequest for GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCollectionsCollectionsWardSkin;

    fn get_url(&self) -> String {
        format!("/lol-collections/v1/inventories/{}/ward-skins/{}", self.summoner_id, self.ward_skin_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_collections_v_1_inventories_by_summoner_id_ward_skins_by_ward_skin_id(summoner_id: u64, ward_skin_id: i64) -> GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId {
    GetLolCollectionsV1InventoriesBySummonerIdWardSkinsByWardSkinId {
        summoner_id, ward_skin_id
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsWardSkin {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub ownership: LolCollectionsCollectionsOwnership,
    pub ward_image_path: String,
    pub ward_shadow_image_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsOwnership {
    pub loyalty_reward: bool,
    pub xbox_gp_reward: bool,
    pub owned: bool,
    pub rental: LolCollectionsCollectionsRental,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerBackdropAugments {
    pub centered_lc_overlay_path: String,
    pub social_card_lc_overlay_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerSpells {
    pub summoner_id: u64,
    pub spells: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerBackdrop {
    pub summoner_id: u64,
    pub account_id: u64,
    pub profile_icon_id: i32,
    pub champion_id: i32,
    pub backdrop_type: LolCollectionsCollectionsSummonerBackdropType,
    pub backdrop_image: String,
    pub backdrop_augments: Vec<LolCollectionsCollectionsSummonerBackdropAugments>,
    pub backdrop_video: String,
    pub backdrop_mask_color: String,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsRental {
    pub end_date: u64,
    pub purchase_date: u64,
    pub win_count_remaining: i32,
    pub rented: bool,
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

