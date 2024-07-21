
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolPurchaseWidgetV1Configuration {

}

impl IsApiRequest for GetLolPurchaseWidgetV1Configuration {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetPurchaseWidgetConfig;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v1/configuration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_purchase_widget_v_1_configuration() -> GetLolPurchaseWidgetV1Configuration {
    GetLolPurchaseWidgetV1Configuration {
        
    }
}


pub struct GetLolPurchaseWidgetV1OrderNotifications {

}

impl IsApiRequest for GetLolPurchaseWidgetV1OrderNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPurchaseWidgetOrderNotificationResource>;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v1/order-notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_purchase_widget_v_1_order_notifications() -> GetLolPurchaseWidgetV1OrderNotifications {
    GetLolPurchaseWidgetV1OrderNotifications {
        
    }
}


pub struct GetLolPurchaseWidgetV1PurchasableItem {

    pub inventory_type: String,
    pub item_id: i64,
}

impl IsApiRequest for GetLolPurchaseWidgetV1PurchasableItem {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetPurchasableItem;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v1/purchasable-item".to_string()
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

pub fn get_lol_purchase_widget_v_1_purchasable_item(inventory_type: String, item_id: i64) -> GetLolPurchaseWidgetV1PurchasableItem {
    GetLolPurchaseWidgetV1PurchasableItem {
        inventory_type, item_id
    }
}


pub struct GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId {

    pub offer_id: String,
}

impl IsApiRequest for GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetBaseSkinLineDto;

    fn get_url(&self) -> String {
        format!("/lol-purchase-widget/v3/base-skin-line-data/{}", self.offer_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_purchase_widget_v_3_base_skin_line_data_by_offer_id(offer_id: String) -> GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId {
    GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId {
        offer_id
    }
}


pub struct GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses {

}

impl IsApiRequest for GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetPurchaseOfferOrderStatuses;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v3/purchase-offer-order-statuses".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_purchase_widget_v_3_purchase_offer_order_statuses() -> GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses {
    GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses {
        
    }
}


pub struct PostLolPurchaseWidgetV1PurchasableItemsByInventoryType {

    pub inventory_type: String,
    pub body: Vec<i64>,
}

impl IsApiRequest for PostLolPurchaseWidgetV1PurchasableItemsByInventoryType {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetItemChoices;

    fn get_url(&self) -> String {
        format!("/lol-purchase-widget/v1/purchasable-items/{}", self.inventory_type)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_purchase_widget_v_1_purchasable_items_by_inventory_type(inventory_type: String, body: Vec<i64>) -> PostLolPurchaseWidgetV1PurchasableItemsByInventoryType {
    PostLolPurchaseWidgetV1PurchasableItemsByInventoryType {
        inventory_type, body
    }
}


pub struct PostLolPurchaseWidgetV2PurchaseItems {

    pub body: LolPurchaseWidgetPurchaseRequest,
}

impl IsApiRequest for PostLolPurchaseWidgetV2PurchaseItems {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v2/purchaseItems".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_purchase_widget_v_2_purchase_items(body: LolPurchaseWidgetPurchaseRequest) -> PostLolPurchaseWidgetV2PurchaseItems {
    PostLolPurchaseWidgetV2PurchaseItems {
        body
    }
}


pub struct PostLolPurchaseWidgetV3PurchaseOffer {

    pub body: LolPurchaseWidgetPurchaseOfferRequestV3,
}

impl IsApiRequest for PostLolPurchaseWidgetV3PurchaseOffer {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetPurchaseOfferResponseV3;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v3/purchaseOffer".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_purchase_widget_v_3_purchase_offer(body: LolPurchaseWidgetPurchaseOfferRequestV3) -> PostLolPurchaseWidgetV3PurchaseOffer {
    PostLolPurchaseWidgetV3PurchaseOffer {
        body
    }
}


pub struct PostLolPurchaseWidgetV3PurchaseOfferViaCap {

    pub body: LolPurchaseWidgetPurchaseOfferRequestV3,
}

impl IsApiRequest for PostLolPurchaseWidgetV3PurchaseOfferViaCap {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetPurchaseOfferResponseV3;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v3/purchaseOfferViaCap".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_purchase_widget_v_3_purchase_offer_via_cap(body: LolPurchaseWidgetPurchaseOfferRequestV3) -> PostLolPurchaseWidgetV3PurchaseOfferViaCap {
    PostLolPurchaseWidgetV3PurchaseOfferViaCap {
        body
    }
}


pub struct PostLolPurchaseWidgetV3ValidateOffer {

    pub body: LolPurchaseWidgetValidateOfferRequestV3,
}

impl IsApiRequest for PostLolPurchaseWidgetV3ValidateOffer {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetValidateOfferResponseV3;

    fn get_url(&self) -> String {
        "/lol-purchase-widget/v3/validateOffer".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_purchase_widget_v_3_validate_offer(body: LolPurchaseWidgetValidateOfferRequestV3) -> PostLolPurchaseWidgetV3ValidateOffer {
    PostLolPurchaseWidgetV3ValidateOffer {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersDataDto {
    pub id: String,
    pub sub_orders: Vec<LolPurchaseWidgetCapOrdersSubOrderDto>,
    pub purchaser: LolPurchaseWidgetCapOrdersTypedIdentifierDto,
    pub location: String,
    pub source: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetDiscountPricingInfo {
    pub cost: i32,
    pub original_cost: i32,
    pub cost_type: String,
    pub currency: String,
    pub discount: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItemAssets {
    pub splash_path: String,
    pub icon_path: String,
    pub tile_path: String,
    pub emblems: Vec<LolPurchaseWidgetChampionSkinEmblem>,
    pub colors: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchasableItem {
    pub item: LolPurchaseWidgetItemDefinition,
    pub dependencies: Vec<LolPurchaseWidgetItemDefinition>,
    pub bundled_items: Vec<LolPurchaseWidgetItemDefinition>,
    pub sale: Option<LolPurchaseWidgetItemSale>,
    pub purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
    pub validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionDto {
    pub title: String,
    pub description: String,
    pub icon_image_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferDto {
    pub id: String,
    pub product_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationErrorEntry {
    pub id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseRequest {
    pub items: Vec<LolPurchaseWidgetPurchaseItem>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineItemDto {
    pub thumbnail_image_path: String,
    pub large_image_path: Option<String>,
    pub localized_long_name: String,
    pub localized_short_name: String,
    pub large_video_path: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferResponseV3 {
    pub validation_errors: Vec<LolPurchaseWidgetValidateOfferError>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginSale {
    pub start_date: String,
    pub end_date: String,
    pub discount: Option<f32>,
    pub cost: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersTypedIdentifierDto {
    pub id: String,
    pub type_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemSale {
    pub start_date: String,
    pub end_date: String,
    pub discount: Option<f32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersMetaDto {
    pub xid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDetails {
    pub title: String,
    pub sub_title: String,
    pub description: String,
    pub icon_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItem {
    pub item_id: i32,
    pub item_instance_id: String,
    pub owned: bool,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub sub_title: String,
    pub description: String,
    pub image_path: String,
    pub purchase_date: u64,
    pub release_date: u64,
    pub inactive_date: u64,
    pub prices: Vec<LolPurchaseWidgetCatalogPluginPrice>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Vec<LolPurchaseWidgetItemMetadataEntry>>,
    pub quest_skin_info: Option<LolPurchaseWidgetSkinLineInfo>,
    pub active: bool,
    pub ownership_type: Option<LolPurchaseWidgetInventoryOwnership>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemMetadataEntry {
    pub type_: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferResponseV3 {
    pub legacy: bool,
    pub order_dto: Option<LolPurchaseWidgetCapOrdersOrderDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblem {
    pub name: String,
    pub emblem_path: LolPurchaseWidgetChampionSkinEmblemPath,
    pub emblem_position: LolPurchaseWidgetChampionSkinEmblemPosition,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferRequestV3 {
    pub offer_id: String,
    pub currency_type: String,
    pub quantity: u32,
    pub price: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDefinition {
    pub item_id: i32,
    pub inventory_type: String,
    pub sub_inventory_type: String,
    pub name: String,
    pub description: String,
    pub sub_title: String,
    pub image_path: String,
    pub owned: bool,
    pub assets: LolPurchaseWidgetCatalogPluginItemAssets,
    pub tags: Vec<String>,
    pub metadata: Vec<LolPurchaseWidgetItemMetadataEntry>,
    pub bundled_item_price: Option<LolPurchaseWidgetBundledItemPricingInfo>,
    pub loyalty_unlocked: bool,
    pub has_visible_loot_odds: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceOptionDto {
    pub price: i64,
    pub currency_type: String,
    pub currency_payment_option: Option<String>,
    pub currency_name: Option<String>,
    pub currency_image_path: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginPrice {
    pub currency: String,
    pub cost: i64,
    pub cost_type: Option<String>,
    pub sale: Option<LolPurchaseWidgetCatalogPluginSale>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatus {
    pub order_state: LolPurchaseWidgetPurchaseOfferOrderStates,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseItem {
    pub item_key: LolPurchaseWidgetItemKey,
    pub quantity: i32,
    pub source: String,
    pub purchase_currency_info: LolPurchaseWidgetItemPrice,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersSubOrderDto {
    pub recipient_id: String,
    pub offer_context: LolPurchaseWidgetCapOrdersOfferContextDto,
    pub offer: LolPurchaseWidgetCapOrdersOfferDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferError {
    pub error_key: String,
    pub meta: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBaseSkinLineDto {
    pub items: Vec<LolPurchaseWidgetSkinLineItemDto>,
    pub localized_name: String,
    pub skin_line_descriptions: Vec<LolPurchaseWidgetSkinLineDescriptionDto>,
    pub pricing_options: Vec<LolPurchaseWidgetPriceOptionDto>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub collection_card_path: String,
    pub collection_description: String,
    pub tile_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseWidgetConfig {
    pub enabled: bool,
    pub non_refundable_disclaimer_enabled: bool,
    pub always_show_purchase_disclaimer: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferContextDto {
    pub quantity: u32,
    pub payment_option: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoices {
    pub choices: Vec<LolPurchaseWidgetItemChoiceDetails>,
    pub validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceDetail {
    pub item_key: LolPurchaseWidgetItemKey,
    pub price: LolPurchaseWidgetItemPrice,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetOrderNotificationResource {
    pub event_type_id: String,
    pub event_type: String,
    pub status: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOption {
    pub price_details: Vec<LolPurchaseWidgetPriceDetail>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineTier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: Option<String>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemKey {
    pub inventory_type: String,
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOrderDto {
    pub data: LolPurchaseWidgetCapOrdersDataDto,
    pub meta: LolPurchaseWidgetCapOrdersMetaDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferRequestV3 {
    pub offer_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatuses {
    pub statuses: LolPurchaseWidgetPurchaseOfferOrderStatus,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemPrice {
    pub currency_type: String,
    pub price: i64,
    pub purchasable: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineInfo {
    pub name: String,
    pub description_info: Vec<LolPurchaseWidgetSkinLineDescriptionInfo>,
    pub splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub uncentered_splash_path: String,
    pub collection_description: String,
    pub tiers: Vec<LolPurchaseWidgetSkinLineTier>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBundledItemPricingInfo {
    pub discount_prices: Vec<LolPurchaseWidgetDiscountPricingInfo>,
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoiceDetails {
    pub item: LolPurchaseWidgetCatalogPluginItem,
    pub background_image: String,
    pub contents: Vec<LolPurchaseWidgetItemDetails>,
    pub discount: String,
    pub full_price: u32,
    pub display_type: String,
    pub purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolPurchaseWidgetInventoryOwnership {
    #[default]
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolPurchaseWidgetPurchaseOfferOrderStates {
    #[default]
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
}

