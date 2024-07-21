
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolMarketplaceV1ProductsByProductStores {
    pub product: String,
}

impl IsApiRequest for GetLolMarketplaceV1ProductsByProductStores {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMarketplaceStoresResponse;
    fn get_url(&self) -> String {format!("/lol-marketplace/v1/products/{}/stores", self.product)}
}

pub fn get_lol_marketplace_v_1_products_by_product_stores(product: String) -> GetLolMarketplaceV1ProductsByProductStores {
    GetLolMarketplaceV1ProductsByProductStores{product}
}


pub struct GetLolMarketplaceV1PurchasesByPurchaseId {
    pub purchase_id: String,
}

impl IsApiRequest for GetLolMarketplaceV1PurchasesByPurchaseId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolMarketplacePurchaseTransaction;
    fn get_url(&self) -> String {format!("/lol-marketplace/v1/purchases/{}", self.purchase_id)}
}

pub fn get_lol_marketplace_v_1_purchases_by_purchase_id(purchase_id: String) -> GetLolMarketplaceV1PurchasesByPurchaseId {
    GetLolMarketplaceV1PurchasesByPurchaseId{purchase_id}
}


pub struct PostLolMarketplaceV1ProductsByProductPurchases {
    pub product: String,
    pub body: LolMarketplacePurchaseRequest,
}

impl IsApiRequest for PostLolMarketplaceV1ProductsByProductPurchases {
    const METHOD: Method = Method::POST;
    type ReturnType = LolMarketplacePurchaseResponse;
    fn get_url(&self) -> String {format!("/lol-marketplace/v1/products/{}/purchases", self.product)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_marketplace_v_1_products_by_product_purchases(product: String, body: LolMarketplacePurchaseRequest) -> PostLolMarketplaceV1ProductsByProductPurchases {
    PostLolMarketplaceV1ProductsByProductPurchases{product, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceCatalogEntryDto {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub description: String,
    pub end_time: String,
    pub purchase_units: Vec<LolMarketplacePurchaseUnitDto>,
    pub display_metadata: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceFinalPurchaseUnitDto {
    pub payments: Vec<LolMarketplacePaymentDto>,
    pub fulfillment: LolMarketplaceFulfillmentDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceFulfillmentDto {
    pub delta: i64,
    pub final_delta: i64,
    pub name: String,
    pub max_quantity: i64,
    pub owned_quantity: u64,
    pub item_type_id: String,
    pub item_id: String,
    pub currency_id: String,
    pub sub_currency_deltas: HashMap<String, i64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePagination {
    pub offset: u32,
    pub limit: u32,
    pub max_limit: u32,
    pub total: u32,
    pub previous: String,
    pub next: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePaymentDto {
    pub delta: i64,
    pub final_delta: i64,
    pub name: String,
    pub discounted_delta: i64,
    pub discount_percent: f64,
    pub item_type_id: String,
    pub item_id: String,
    pub currency_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePaymentOptionDto {
    pub key: String,
    pub payments: Vec<LolMarketplacePaymentDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePurchaseDto {
    pub id: String,
    pub product_id: String,
    pub store_id: String,
    pub catalog_entry_id: String,
    pub purchaser_id: String,
    pub recipient_id: String,
    pub purchase_units: Vec<LolMarketplaceFinalPurchaseUnitDto>,
    pub created_time: String,
    pub completed_time: String,
    pub is_reverted: bool,
    pub reverted_time: String,
    pub purchase_state: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePurchaseRequest {
    pub store_id: String,
    pub catalog_entry_id: String,
    pub payment_options_keys: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePurchaseResponse {
    pub data: LolMarketplacePurchaseDto,
    pub paging: LolMarketplacePagination,
    pub stats: LolMarketplaceResponseStats,
    pub notes: Vec<String>,
    pub errors: Vec<LolMarketplaceResponseError>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePurchaseTransaction {
    pub purchase_id: String,
    pub product_id: String,
    pub store_id: String,
    pub catalog_entry_id: String,
    pub purchase_state: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePurchaseUnitDto {
    pub payment_options: Vec<LolMarketplacePaymentOptionDto>,
    pub payment: Vec<LolMarketplacePaymentDto>,
    pub fulfillment: LolMarketplaceFulfillmentDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceResponseError {
    pub message: String,
    pub type_: String,
    pub code: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceResponseStats {
    pub duration_ms: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceStoreDto {
    pub id: String,
    pub product_id: String,
    pub name: String,
    pub catalog_entries: Vec<LolMarketplaceCatalogEntryDto>,
    pub display_metadata: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceStoresResponse {
    pub data: Vec<LolMarketplaceStoreDto>,
    pub paging: LolMarketplacePagination,
    pub stats: LolMarketplaceResponseStats,
    pub notes: Vec<String>,
    pub errors: Vec<LolMarketplaceResponseError>,
}


// ENUMS

