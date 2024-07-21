
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolCatalogV1ItemDetails {

    pub inventory_type: String,
    pub item_id: i64,
}

impl IsApiRequest for GetLolCatalogV1ItemDetails {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCatalogCatalogPluginItemWithDetails;

    fn get_url(&self) -> String {
        "/lol-catalog/v1/item-details".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryType" : self.inventory_type,
            "itemId" : self.item_id,
        }))
    }
}

pub fn get_lol_catalog_v_1_item_details(inventory_type: String, item_id: i64) -> GetLolCatalogV1ItemDetails {
    GetLolCatalogV1ItemDetails {
        inventory_type, item_id
    }
}


pub struct GetLolCatalogV1Items {

    pub inventory_type: String,
    pub item_ids: Vec<i64>,
}

impl IsApiRequest for GetLolCatalogV1Items {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogItemChoiceDetails>;

    fn get_url(&self) -> String {
        "/lol-catalog/v1/items".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryType" : self.inventory_type,
            "itemIds" : self.item_ids,
        }))
    }
}

pub fn get_lol_catalog_v_1_items(inventory_type: String, item_ids: Vec<i64>) -> GetLolCatalogV1Items {
    GetLolCatalogV1Items {
        inventory_type, item_ids
    }
}


pub struct GetLolCatalogV1ItemsByInventoryType {

    pub inventory_type: String,
}

impl IsApiRequest for GetLolCatalogV1ItemsByInventoryType {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogCatalogPluginItem>;

    fn get_url(&self) -> String {
        format!("/lol-catalog/v1/items/{}", self.inventory_type)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_catalog_v_1_items_by_inventory_type(inventory_type: String) -> GetLolCatalogV1ItemsByInventoryType {
    GetLolCatalogV1ItemsByInventoryType {
        inventory_type
    }
}


pub struct GetLolCatalogV1ItemsListDetails {

    pub catalog_items_keys: Vec<LolCatalogItemKey>,
}

impl IsApiRequest for GetLolCatalogV1ItemsListDetails {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogCatalogPluginItemWithDetails>;

    fn get_url(&self) -> String {
        "/lol-catalog/v1/items-list-details".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "catalogItemsKeys" : self.catalog_items_keys,
        }))
    }
}

pub fn get_lol_catalog_v_1_items_list_details(catalog_items_keys: Vec<LolCatalogItemKey>) -> GetLolCatalogV1ItemsListDetails {
    GetLolCatalogV1ItemsListDetails {
        catalog_items_keys
    }
}


pub struct GetLolCatalogV1ItemsListDetailsSkipCache {

    pub catalog_items_keys: Vec<LolCatalogItemKey>,
}

impl IsApiRequest for GetLolCatalogV1ItemsListDetailsSkipCache {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogCatalogPluginItemWithDetails>;

    fn get_url(&self) -> String {
        "/lol-catalog/v1/items-list-details/skip-cache".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "catalogItemsKeys" : self.catalog_items_keys,
        }))
    }
}

pub fn get_lol_catalog_v_1_items_list_details_skip_cache(catalog_items_keys: Vec<LolCatalogItemKey>) -> GetLolCatalogV1ItemsListDetailsSkipCache {
    GetLolCatalogV1ItemsListDetailsSkipCache {
        catalog_items_keys
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSale {
    pub start_date: String,
    pub end_date: String,
    pub prices: Vec<LolCatalogItemCost>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolCatalogChampionSkinEmblemPath,
    pub emblem_position: LolCatalogChampionSkinEmblemPosition,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginRetailDiscount {
    pub start_date: String,
    pub end_date: String,
    pub discount: f32,
    pub cost: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginPrice {
    pub currency: String,
    pub cost: i64,
    pub cost_type: String,
    pub sale: LolCatalogCatalogPluginRetailDiscount,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemAssets {
    pub splash_path: String,
    pub icon_path: String,
    pub tile_path: String,
    pub emblems: Vec<LolCatalogChampionSkinEmblem>,
    pub colors: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemMetadataEntry {
    pub type_: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineTier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: String,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: String,
    pub collection_splash_video_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItem {
    pub item_id: i32,
    pub item_instance_id: String,
    pub owned: bool,
    pub ownership_type: LolCatalogInventoryOwnership,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub sub_title: String,
    pub description: String,
    pub image_path: String,
    pub tile_path: String,
    pub purchase_date: u64,
    pub release_date: u64,
    pub inactive_date: u64,
    pub prices: Vec<LolCatalogCatalogPluginPrice>,
    pub tags: Vec<String>,
    pub metadata: Vec<LolCatalogItemMetadataEntry>,
    pub active: bool,
    pub sale: LolCatalogSale,
    pub quest_skin_info: LolCatalogSkinLineInfo,
    pub offer_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineInfo {
    pub name: String,
    pub description_info: Vec<LolCatalogSkinLineDescriptionInfo>,
    pub splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub uncentered_splash_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolCatalogSkinLineTier>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemWithDetails {
    pub item: LolCatalogCatalogPluginItem,
    pub quantity: u32,
    pub required_items: Vec<LolCatalogCatalogPluginItemWithDetails>,
    pub bundled_items: Vec<LolCatalogCatalogPluginItemWithDetails>,
    pub minimum_bundle_prices: Vec<LolCatalogCatalogPluginPrice>,
    pub bundled_discount_prices: Vec<LolCatalogCatalogPluginPrice>,
    pub assets: LolCatalogCatalogPluginItemAssets,
    pub metadata: Vec<LolCatalogItemMetadataEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemChoiceDetails {
    pub item: LolCatalogCatalogPluginItem,
    pub background_image: String,
    pub contents: Vec<LolCatalogItemDetails>,
    pub discount: String,
    pub full_price: i64,
    pub display_type: String,
    pub metadata: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemDetails {
    pub title: String,
    pub sub_title: String,
    pub description: String,
    pub icon_url: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolCatalogInventoryOwnership {
    #[default]
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}

