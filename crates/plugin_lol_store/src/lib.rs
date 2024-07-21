
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolStoreV1AliasChangeNotifications {

}

impl IsApiRequest for GetLolStoreV1AliasChangeNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreAliasChangeNotificationResource>;

    fn get_url(&self) -> String {
        "/lol-store/v1/alias-change-notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_alias_change_notifications() -> GetLolStoreV1AliasChangeNotifications {
    GetLolStoreV1AliasChangeNotifications {
        
    }
}


pub struct GetLolStoreV1ByPageType {

    pub page_type: String,
}

impl IsApiRequest for GetLolStoreV1ByPageType {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-store/v1/{}", self.page_type)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_by_page_type(page_type: String) -> GetLolStoreV1ByPageType {
    GetLolStoreV1ByPageType {
        page_type
    }
}


pub struct GetLolStoreV1Catalog {

    pub inventory_type: Option<Vec<String>>,
    pub item_id: Option<Vec<i32>>,
}

impl IsApiRequest for GetLolStoreV1Catalog {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreCatalogItem>;

    fn get_url(&self) -> String {
        "/lol-store/v1/catalog".to_string()
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

pub fn get_lol_store_v_1_catalog(inventory_type: Option<Vec<String>>, item_id: Option<Vec<i32>>) -> GetLolStoreV1Catalog {
    GetLolStoreV1Catalog {
        inventory_type, item_id
    }
}


pub struct GetLolStoreV1CatalogByInstanceIds {

    pub instance_ids: Vec<String>,
}

impl IsApiRequest for GetLolStoreV1CatalogByInstanceIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreCatalogItem>;

    fn get_url(&self) -> String {
        "/lol-store/v1/catalogByInstanceIds".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "instanceIds" : self.instance_ids,
        }))
    }
}

pub fn get_lol_store_v_1_catalog_by_instance_ids(instance_ids: Vec<String>) -> GetLolStoreV1CatalogByInstanceIds {
    GetLolStoreV1CatalogByInstanceIds {
        instance_ids
    }
}


pub struct GetLolStoreV1CatalogByInventoryType {

    pub inventory_type: String,
    pub item_ids: Vec<i32>,
}

impl IsApiRequest for GetLolStoreV1CatalogByInventoryType {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreCatalogItem>;

    fn get_url(&self) -> String {
        format!("/lol-store/v1/catalog/{}", self.inventory_type)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "itemIds" : self.item_ids,
        }))
    }
}

pub fn get_lol_store_v_1_catalog_by_inventory_type(inventory_type: String, item_ids: Vec<i32>) -> GetLolStoreV1CatalogByInventoryType {
    GetLolStoreV1CatalogByInventoryType {
        inventory_type, item_ids
    }
}


pub struct GetLolStoreV1CatalogItemsSkipCache {

    pub catalog_item_keys: Vec<LolStoreItemKey>,
}

impl IsApiRequest for GetLolStoreV1CatalogItemsSkipCache {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreCatalogItem>;

    fn get_url(&self) -> String {
        "/lol-store/v1/catalog/items/skip-cache".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "catalogItemKeys" : self.catalog_item_keys,
        }))
    }
}

pub fn get_lol_store_v_1_catalog_items_skip_cache(catalog_item_keys: Vec<LolStoreItemKey>) -> GetLolStoreV1CatalogItemsSkipCache {
    GetLolStoreV1CatalogItemsSkipCache {
        catalog_item_keys
    }
}


pub struct GetLolStoreV1CatalogSales {

}

impl IsApiRequest for GetLolStoreV1CatalogSales {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreItemSale>;

    fn get_url(&self) -> String {
        "/lol-store/v1/catalog/sales".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_catalog_sales() -> GetLolStoreV1CatalogSales {
    GetLolStoreV1CatalogSales {
        
    }
}


pub struct GetLolStoreV1GetStoreUrl {

}

impl IsApiRequest for GetLolStoreV1GetStoreUrl {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-store/v1/getStoreUrl".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_get_store_url() -> GetLolStoreV1GetStoreUrl {
    GetLolStoreV1GetStoreUrl {
        
    }
}


pub struct GetLolStoreV1Giftablefriends {

}

impl IsApiRequest for GetLolStoreV1Giftablefriends {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreGiftingFriend>;

    fn get_url(&self) -> String {
        "/lol-store/v1/giftablefriends".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_giftablefriends() -> GetLolStoreV1Giftablefriends {
    GetLolStoreV1Giftablefriends {
        
    }
}


pub struct GetLolStoreV1ItemKeysFromInstanceIds {

    pub instance_ids: Vec<String>,
}

impl IsApiRequest for GetLolStoreV1ItemKeysFromInstanceIds {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStoreItemKey;

    fn get_url(&self) -> String {
        "/lol-store/v1/itemKeysFromInstanceIds".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "instanceIds" : self.instance_ids,
        }))
    }
}

pub fn get_lol_store_v_1_item_keys_from_instance_ids(instance_ids: Vec<String>) -> GetLolStoreV1ItemKeysFromInstanceIds {
    GetLolStoreV1ItemKeysFromInstanceIds {
        instance_ids
    }
}


pub struct GetLolStoreV1ItemKeysFromOfferIds {

    pub offer_ids: Vec<String>,
}

impl IsApiRequest for GetLolStoreV1ItemKeysFromOfferIds {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStoreItemKey;

    fn get_url(&self) -> String {
        "/lol-store/v1/itemKeysFromOfferIds".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "offerIds" : self.offer_ids,
        }))
    }
}

pub fn get_lol_store_v_1_item_keys_from_offer_ids(offer_ids: Vec<String>) -> GetLolStoreV1ItemKeysFromOfferIds {
    GetLolStoreV1ItemKeysFromOfferIds {
        offer_ids
    }
}


pub struct GetLolStoreV1LastPage {

}

impl IsApiRequest for GetLolStoreV1LastPage {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-store/v1/lastPage".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_last_page() -> GetLolStoreV1LastPage {
    GetLolStoreV1LastPage {
        
    }
}


pub struct PostLolStoreV1LastPage {

    pub body: String,
}

impl IsApiRequest for PostLolStoreV1LastPage {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-store/v1/lastPage".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_store_v_1_last_page(body: String) -> PostLolStoreV1LastPage {
    PostLolStoreV1LastPage {
        body
    }
}


pub struct GetLolStoreV1Offers {

    pub inventory_type_uui_ds: Option<Vec<String>>,
}

impl IsApiRequest for GetLolStoreV1Offers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreCapOffer>;

    fn get_url(&self) -> String {
        "/lol-store/v1/offers".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryTypeUUIDs" : self.inventory_type_uui_ds,
        }))
    }
}

pub fn get_lol_store_v_1_offers(inventory_type_uui_ds: Option<Vec<String>>) -> GetLolStoreV1Offers {
    GetLolStoreV1Offers {
        inventory_type_uui_ds
    }
}


pub struct GetLolStoreV1OffersByOfferId {

    pub offer_id: String,
}

impl IsApiRequest for GetLolStoreV1OffersByOfferId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStoreCapOffer;

    fn get_url(&self) -> String {
        format!("/lol-store/v1/offers/{}", self.offer_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_offers_by_offer_id(offer_id: String) -> GetLolStoreV1OffersByOfferId {
    GetLolStoreV1OffersByOfferId {
        offer_id
    }
}


pub struct GetLolStoreV1OrderNotifications {

}

impl IsApiRequest for GetLolStoreV1OrderNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreOrderNotificationResource>;

    fn get_url(&self) -> String {
        "/lol-store/v1/order-notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_order_notifications() -> GetLolStoreV1OrderNotifications {
    GetLolStoreV1OrderNotifications {
        
    }
}


pub struct GetLolStoreV1OrderNotificationsById {

    pub id: u64,
}

impl IsApiRequest for GetLolStoreV1OrderNotificationsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStoreOrderNotificationResource;

    fn get_url(&self) -> String {
        format!("/lol-store/v1/order-notifications/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_order_notifications_by_id(id: u64) -> GetLolStoreV1OrderNotificationsById {
    GetLolStoreV1OrderNotificationsById {
        id
    }
}


pub struct GetLolStoreV1PaymentDetails {

    pub action: String,
    pub gift_recipient_account_id: Option<u64>,
    pub gift_message: Option<String>,
}

impl IsApiRequest for GetLolStoreV1PaymentDetails {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-store/v1/paymentDetails".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "action" : self.action,
            "giftRecipientAccountId" : self.gift_recipient_account_id,
            "giftMessage" : self.gift_message,
        }))
    }
}

pub fn get_lol_store_v_1_payment_details(action: String, gift_recipient_account_id: Option<u64>, gift_message: Option<String>) -> GetLolStoreV1PaymentDetails {
    GetLolStoreV1PaymentDetails {
        action, gift_recipient_account_id, gift_message
    }
}


pub struct GetLolStoreV1SkinsBySkinId {

    pub skin_id: i32,
}

impl IsApiRequest for GetLolStoreV1SkinsBySkinId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStoreCatalogItem;

    fn get_url(&self) -> String {
        format!("/lol-store/v1/skins/{}", self.skin_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_skins_by_skin_id(skin_id: i32) -> GetLolStoreV1SkinsBySkinId {
    GetLolStoreV1SkinsBySkinId {
        skin_id
    }
}


pub struct GetLolStoreV1Status {

}

impl IsApiRequest for GetLolStoreV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = LolStoreStoreStatus;

    fn get_url(&self) -> String {
        "/lol-store/v1/status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_status() -> GetLolStoreV1Status {
    GetLolStoreV1Status {
        
    }
}


pub struct GetLolStoreV1StoreReady {

}

impl IsApiRequest for GetLolStoreV1StoreReady {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-store/v1/store-ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_store_v_1_store_ready() -> GetLolStoreV1StoreReady {
    GetLolStoreV1StoreReady {
        
    }
}


pub struct GetLolStoreV2Offers {

    pub type_id: Option<String>,
}

impl IsApiRequest for GetLolStoreV2Offers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolStoreCapOffer>;

    fn get_url(&self) -> String {
        "/lol-store/v2/offers".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "typeId" : self.type_id,
        }))
    }
}

pub fn get_lol_store_v_2_offers(type_id: Option<String>) -> GetLolStoreV2Offers {
    GetLolStoreV2Offers {
        type_id
    }
}


pub struct PostLolStoreV1NotificationsAcknowledge {

    pub body: String,
}

impl IsApiRequest for PostLolStoreV1NotificationsAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-store/v1/notifications/acknowledge".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_store_v_1_notifications_acknowledge(body: String) -> PostLolStoreV1NotificationsAcknowledge {
    PostLolStoreV1NotificationsAcknowledge {
        body
    }
}


pub struct PostLolStoreV3Purchase {

    pub body: Vec<LolStoreItemOrderDto>,
}

impl IsApiRequest for PostLolStoreV3Purchase {
    const METHOD: Method = Method::POST;
    type ReturnType = LolStorePurchaseOrderResponseDto;

    fn get_url(&self) -> String {
        "/lol-store/v3/purchase".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_store_v_3_purchase(body: Vec<LolStoreItemOrderDto>) -> PostLolStoreV3Purchase {
    PostLolStoreV3Purchase {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreStoreStatus {
    pub storefront_is_running: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreOrderNotificationResource {
    pub id: u64,
    pub event_type_id: String,
    pub event_type: String,
    pub status: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundled {
    pub flexible: bool,
    pub items: Vec<LolStoreBundledItem>,
    pub minimum_prices: Vec<LolStoreBundledItemCost>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSale {
    pub start_date: String,
    pub end_date: String,
    pub prices: Vec<LolStoreItemCost>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGiftingFriend {
    pub friends_since: String,
    pub old_friends: bool,
    pub summoner_id: u64,
    pub nick: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreAliasDetail {
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundledItem {
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: u32,
    pub discount_prices: Vec<LolStoreBundledItemCost>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStorePurchaseOrderResponseDto {
    pub rp_balance: i64,
    pub ip_balance: i64,
    pub transactions: Vec<LolStoreTransactionResponseDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCapOffer {
    pub id: String,
    pub type_id: String,
    pub label: String,
    pub product_id: String,
    pub merchant_id: String,
    pub payload: HashMap<String, String>,
    pub active: bool,
    pub start_date: String,
    pub created_date: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemMetadataEntry {
    pub type_: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemOrderDto {
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: u32,
    pub rp_cost: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreTransactionResponseDto {
    pub id: String,
    pub inventory_type: String,
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreAliasChangeNotificationResource {
    pub id: u64,
    pub type_: String,
    pub details: LolStoreAliasDetail,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemLocalization {
    pub language: String,
    pub name: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCatalogItem {
    pub item_id: i32,
    pub inventory_type: String,
    pub icon_url: Option<String>,
    pub localizations: Option<LolStoreItemLocalization>,
    pub active: Option<bool>,
    pub bundled: Option<LolStoreBundled>,
    pub inactive_date: Option<String>,
    pub max_quantity: Option<i32>,
    pub prices: Option<Vec<LolStoreItemCost>>,
    pub release_date: Option<String>,
    pub sale: Option<LolStoreSale>,
    pub sub_inventory_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub item_requirements: Option<Vec<LolStoreItemKey>>,
    pub metadata: Option<Vec<LolStoreItemMetadataEntry>>,
    pub item_instance_id: Option<String>,
    pub offer_id: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundledItemCost {
    pub currency: String,
    pub cost: i64,
    pub discount: Option<f32>,
    pub cost_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemSale {
    pub id: u64,
    pub active: bool,
    pub item: LolStoreItemKey,
    pub sale: LolStoreSale,
}


// ENUMS

