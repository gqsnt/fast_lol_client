
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolCatalogV1ItemDetails {
    pub inventory_type: String,
    pub item_id: i64,
}

impl IsApiRequest for GetLolCatalogV1ItemDetails {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCatalogCatalogPluginItemWithDetails;
    fn get_url(&self) -> String {"/lol-catalog/v1/item-details".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("inventoryType".to_string(), serde_json::to_string(&self.inventory_type).unwrap()),
            ("itemId".to_string(), serde_json::to_string(&self.item_id).unwrap())
        ])
    }
}

pub fn get_lol_catalog_v1_item_details(inventory_type: String, item_id: i64) -> GetLolCatalogV1ItemDetails {
    GetLolCatalogV1ItemDetails{inventory_type, item_id}
}


pub struct GetLolCatalogV1Items {
    pub inventory_type: String,
    pub item_ids: Vec<i64>,
}

impl IsApiRequest for GetLolCatalogV1Items {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogItemChoiceDetails>;
    fn get_url(&self) -> String {"/lol-catalog/v1/items".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("inventoryType".to_string(), serde_json::to_string(&self.inventory_type).unwrap()),
            ("itemIds".to_string(), serde_json::to_string(&self.item_ids).unwrap())
        ])
    }
}

pub fn get_lol_catalog_v1_items(inventory_type: String, item_ids: Vec<i64>) -> GetLolCatalogV1Items {
    GetLolCatalogV1Items{inventory_type, item_ids}
}


pub struct GetLolCatalogV1ItemsByInventoryType {
    pub inventory_type: String,
}

impl IsApiRequest for GetLolCatalogV1ItemsByInventoryType {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogCatalogPluginItem>;
    fn get_url(&self) -> String {format!("/lol-catalog/v1/items/{}", self.inventory_type)}
}

pub fn get_lol_catalog_v1_items_by_inventory_type(inventory_type: String) -> GetLolCatalogV1ItemsByInventoryType {
    GetLolCatalogV1ItemsByInventoryType{inventory_type}
}


pub struct GetLolCatalogV1ItemsListDetails {
    pub catalog_items_keys: Vec<LolCatalogItemKey>,
}

impl IsApiRequest for GetLolCatalogV1ItemsListDetails {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogCatalogPluginItemWithDetails>;
    fn get_url(&self) -> String {"/lol-catalog/v1/items-list-details".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("catalogItemsKeys".to_string(), serde_json::to_string(&self.catalog_items_keys).unwrap())
        ])
    }
}

pub fn get_lol_catalog_v1_items_list_details(catalog_items_keys: Vec<LolCatalogItemKey>) -> GetLolCatalogV1ItemsListDetails {
    GetLolCatalogV1ItemsListDetails{catalog_items_keys}
}


pub struct GetLolCatalogV1ItemsListDetailsSkipCache {
    pub catalog_items_keys: Vec<LolCatalogItemKey>,
}

impl IsApiRequest for GetLolCatalogV1ItemsListDetailsSkipCache {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolCatalogCatalogPluginItemWithDetails>;
    fn get_url(&self) -> String {"/lol-catalog/v1/items-list-details/skip-cache".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("catalogItemsKeys".to_string(), serde_json::to_string(&self.catalog_items_keys).unwrap())
        ])
    }
}

pub fn get_lol_catalog_v1_items_list_details_skip_cache(catalog_items_keys: Vec<LolCatalogItemKey>) -> GetLolCatalogV1ItemsListDetailsSkipCache {
    GetLolCatalogV1ItemsListDetailsSkipCache{catalog_items_keys}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItem {
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: String,
    pub owned: bool,
    #[serde(rename = "ownershipType")]
    pub ownership_type: Option<LolCatalogInventoryOwnership>,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "subInventoryType")]
    pub sub_inventory_type: String,
    pub name: String,
    #[serde(rename = "subTitle")]
    pub sub_title: String,
    pub description: String,
    #[serde(rename = "imagePath")]
    pub image_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: u64,
    #[serde(rename = "releaseDate")]
    pub release_date: u64,
    #[serde(rename = "inactiveDate")]
    pub inactive_date: u64,
    pub prices: Vec<LolCatalogCatalogPluginPrice>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Vec<LolCatalogItemMetadataEntry>>,
    pub active: bool,
    pub sale: Option<LolCatalogSale>,
    #[serde(rename = "questSkinInfo")]
    pub quest_skin_info: Option<LolCatalogSkinLineInfo>,
    #[serde(rename = "offerId")]
    pub offer_id: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemAssets {
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    pub emblems: Vec<LolCatalogChampionSkinEmblem>,
    pub colors: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemWithDetails {
    pub item: LolCatalogCatalogPluginItem,
    pub quantity: u32,
    #[serde(rename = "requiredItems")]
    pub required_items: Option<Vec<LolCatalogCatalogPluginItemWithDetails>>,
    #[serde(rename = "bundledItems")]
    pub bundled_items: Option<Vec<LolCatalogCatalogPluginItemWithDetails>>,
    #[serde(rename = "minimumBundlePrices")]
    pub minimum_bundle_prices: Option<Vec<LolCatalogCatalogPluginPrice>>,
    #[serde(rename = "bundledDiscountPrices")]
    pub bundled_discount_prices: Option<Vec<LolCatalogCatalogPluginPrice>>,
    pub assets: LolCatalogCatalogPluginItemAssets,
    pub metadata: Vec<LolCatalogItemMetadataEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginPrice {
    pub currency: String,
    pub cost: i64,
    #[serde(rename = "costType")]
    pub cost_type: Option<String>,
    pub sale: Option<LolCatalogCatalogPluginRetailDiscount>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginRetailDiscount {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub discount: Option<f32>,
    pub cost: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblem {
    pub name: String,
    #[serde(rename = "emblemPath")]
    pub emblem_path: LolCatalogChampionSkinEmblemPath,
    #[serde(rename = "emblemPosition")]
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
pub struct LolCatalogChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemChoiceDetails {
    pub item: LolCatalogCatalogPluginItem,
    #[serde(rename = "backgroundImage")]
    pub background_image: String,
    pub contents: Vec<LolCatalogItemDetails>,
    pub discount: String,
    #[serde(rename = "fullPrice")]
    pub full_price: i64,
    #[serde(rename = "displayType")]
    pub display_type: String,
    pub metadata: Value,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemDetails {
    pub title: String,
    #[serde(rename = "subTitle")]
    pub sub_title: String,
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemKey {
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemMetadataEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSale {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub prices: Vec<LolCatalogItemCost>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineDescriptionInfo {
    pub title: String,
    pub description: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineInfo {
    pub name: String,
    #[serde(rename = "descriptionInfo")]
    pub description_info: Vec<LolCatalogSkinLineDescriptionInfo>,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    #[serde(rename = "collectionCardPath")]
    pub collection_card_path: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "collectionDescription")]
    pub collection_description: String,
    pub tiers: Vec<LolCatalogSkinLineTier>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineTier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: Option<String>,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    #[serde(rename = "loadScreenPath")]
    pub load_screen_path: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: Option<String>,
    #[serde(rename = "collectionSplashVideoPath")]
    pub collection_splash_video_path: Option<String>,
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

