
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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

pub fn get_lol_marketplace_v1_products_by_product_stores(product: String) -> GetLolMarketplaceV1ProductsByProductStores {
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

pub fn get_lol_marketplace_v1_purchases_by_purchase_id(purchase_id: String) -> GetLolMarketplaceV1PurchasesByPurchaseId {
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

pub fn post_lol_marketplace_v1_products_by_product_purchases(product: String, body: LolMarketplacePurchaseRequest) -> PostLolMarketplaceV1ProductsByProductPurchases {
    PostLolMarketplaceV1ProductsByProductPurchases{product, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceCatalogEntryDto {
    pub id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "purchaseUnits")]
    pub purchase_units: Vec<LolMarketplacePurchaseUnitDto>,
    #[serde(rename = "displayMetadata")]
    pub display_metadata: Value,
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
    #[serde(rename = "finalDelta")]
    pub final_delta: i64,
    pub name: String,
    #[serde(rename = "maxQuantity")]
    pub max_quantity: i64,
    #[serde(rename = "ownedQuantity")]
    pub owned_quantity: u64,
    #[serde(rename = "itemTypeId")]
    pub item_type_id: String,
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "currencyId")]
    pub currency_id: String,
    #[serde(rename = "subCurrencyDeltas")]
    pub sub_currency_deltas: HashMap<String, i64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePagination {
    pub offset: u32,
    pub limit: u32,
    #[serde(rename = "maxLimit")]
    pub max_limit: u32,
    pub total: u32,
    pub previous: String,
    pub next: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePaymentDto {
    pub delta: i64,
    #[serde(rename = "finalDelta")]
    pub final_delta: i64,
    pub name: String,
    #[serde(rename = "discountedDelta")]
    pub discounted_delta: i64,
    #[serde(rename = "discountPercent")]
    pub discount_percent: f64,
    #[serde(rename = "itemTypeId")]
    pub item_type_id: String,
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "currencyId")]
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
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "storeId")]
    pub store_id: String,
    #[serde(rename = "catalogEntryId")]
    pub catalog_entry_id: String,
    #[serde(rename = "purchaserId")]
    pub purchaser_id: String,
    #[serde(rename = "recipientId")]
    pub recipient_id: String,
    #[serde(rename = "purchaseUnits")]
    pub purchase_units: Vec<LolMarketplaceFinalPurchaseUnitDto>,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "completedTime")]
    pub completed_time: String,
    #[serde(rename = "isReverted")]
    pub is_reverted: bool,
    #[serde(rename = "revertedTime")]
    pub reverted_time: String,
    #[serde(rename = "purchaseState")]
    pub purchase_state: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePurchaseRequest {
    #[serde(rename = "storeId")]
    pub store_id: String,
    #[serde(rename = "catalogEntryId")]
    pub catalog_entry_id: String,
    #[serde(rename = "paymentOptionsKeys")]
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
    #[serde(rename = "purchaseId")]
    pub purchase_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "storeId")]
    pub store_id: String,
    #[serde(rename = "catalogEntryId")]
    pub catalog_entry_id: String,
    #[serde(rename = "purchaseState")]
    pub purchase_state: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplacePurchaseUnitDto {
    #[serde(rename = "paymentOptions")]
    pub payment_options: Vec<LolMarketplacePaymentOptionDto>,
    pub payment: Vec<LolMarketplacePaymentDto>,
    pub fulfillment: LolMarketplaceFulfillmentDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceResponseError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub code: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceResponseStats {
    #[serde(rename = "durationMs")]
    pub duration_ms: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolMarketplaceStoreDto {
    pub id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    pub name: String,
    #[serde(rename = "catalogEntries")]
    pub catalog_entries: Vec<LolMarketplaceCatalogEntryDto>,
    #[serde(rename = "displayMetadata")]
    pub display_metadata: Value,
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

