
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolPurchaseWidgetV1Configuration {}

impl IsApiRequest for GetLolPurchaseWidgetV1Configuration {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetPurchaseWidgetConfig;
    fn get_url(&self) -> String {"/lol-purchase-widget/v1/configuration".to_string()}
}

pub fn get_lol_purchase_widget_v1_configuration() -> GetLolPurchaseWidgetV1Configuration {
    GetLolPurchaseWidgetV1Configuration{}
}


pub struct GetLolPurchaseWidgetV1OrderNotifications {}

impl IsApiRequest for GetLolPurchaseWidgetV1OrderNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPurchaseWidgetOrderNotificationResource>;
    fn get_url(&self) -> String {"/lol-purchase-widget/v1/order-notifications".to_string()}
}

pub fn get_lol_purchase_widget_v1_order_notifications() -> GetLolPurchaseWidgetV1OrderNotifications {
    GetLolPurchaseWidgetV1OrderNotifications{}
}


pub struct GetLolPurchaseWidgetV1PurchasableItem {
    pub inventory_type: String,
    pub item_id: i64,
}

impl IsApiRequest for GetLolPurchaseWidgetV1PurchasableItem {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetPurchasableItem;
    fn get_url(&self) -> String {"/lol-purchase-widget/v1/purchasable-item".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("inventoryType".to_string(), serde_json::to_string(&self.inventory_type).unwrap()),
            ("itemId".to_string(), serde_json::to_string(&self.item_id).unwrap())
        ])
    }
}

pub fn get_lol_purchase_widget_v1_purchasable_item(inventory_type: String, item_id: i64) -> GetLolPurchaseWidgetV1PurchasableItem {
    GetLolPurchaseWidgetV1PurchasableItem{inventory_type, item_id}
}


pub struct GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId {
    pub offer_id: String,
}

impl IsApiRequest for GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetBaseSkinLineDto;
    fn get_url(&self) -> String {format!("/lol-purchase-widget/v3/base-skin-line-data/{}", self.offer_id)}
}

pub fn get_lol_purchase_widget_v3_base_skin_line_data_by_offer_id(offer_id: String) -> GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId {
    GetLolPurchaseWidgetV3BaseSkinLineDataByOfferId{offer_id}
}


pub struct GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses {}

impl IsApiRequest for GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPurchaseWidgetPurchaseOfferOrderStatuses;
    fn get_url(&self) -> String {"/lol-purchase-widget/v3/purchase-offer-order-statuses".to_string()}
}

pub fn get_lol_purchase_widget_v3_purchase_offer_order_statuses() -> GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses {
    GetLolPurchaseWidgetV3PurchaseOfferOrderStatuses{}
}


pub struct PostLolPurchaseWidgetV1PurchasableItemsByInventoryType {
    pub inventory_type: String,
    pub body: Vec<i64>,
}

impl IsApiRequest for PostLolPurchaseWidgetV1PurchasableItemsByInventoryType {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetItemChoices;
    fn get_url(&self) -> String {format!("/lol-purchase-widget/v1/purchasable-items/{}", self.inventory_type)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_purchase_widget_v1_purchasable_items_by_inventory_type(inventory_type: String, body: Vec<i64>) -> PostLolPurchaseWidgetV1PurchasableItemsByInventoryType {
    PostLolPurchaseWidgetV1PurchasableItemsByInventoryType{inventory_type, body}
}


pub struct PostLolPurchaseWidgetV2PurchaseItems {
    pub body: LolPurchaseWidgetPurchaseRequest,
}

impl IsApiRequest for PostLolPurchaseWidgetV2PurchaseItems {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-purchase-widget/v2/purchaseItems".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_purchase_widget_v2_purchase_items(body: LolPurchaseWidgetPurchaseRequest) -> PostLolPurchaseWidgetV2PurchaseItems {
    PostLolPurchaseWidgetV2PurchaseItems{body}
}


pub struct PostLolPurchaseWidgetV3PurchaseOffer {
    pub body: LolPurchaseWidgetPurchaseOfferRequestV3,
}

impl IsApiRequest for PostLolPurchaseWidgetV3PurchaseOffer {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetPurchaseOfferResponseV3;
    fn get_url(&self) -> String {"/lol-purchase-widget/v3/purchaseOffer".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_purchase_widget_v3_purchase_offer(body: LolPurchaseWidgetPurchaseOfferRequestV3) -> PostLolPurchaseWidgetV3PurchaseOffer {
    PostLolPurchaseWidgetV3PurchaseOffer{body}
}


pub struct PostLolPurchaseWidgetV3PurchaseOfferViaCap {
    pub body: LolPurchaseWidgetPurchaseOfferRequestV3,
}

impl IsApiRequest for PostLolPurchaseWidgetV3PurchaseOfferViaCap {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetPurchaseOfferResponseV3;
    fn get_url(&self) -> String {"/lol-purchase-widget/v3/purchaseOfferViaCap".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_purchase_widget_v3_purchase_offer_via_cap(body: LolPurchaseWidgetPurchaseOfferRequestV3) -> PostLolPurchaseWidgetV3PurchaseOfferViaCap {
    PostLolPurchaseWidgetV3PurchaseOfferViaCap{body}
}


pub struct PostLolPurchaseWidgetV3ValidateOffer {
    pub body: LolPurchaseWidgetValidateOfferRequestV3,
}

impl IsApiRequest for PostLolPurchaseWidgetV3ValidateOffer {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPurchaseWidgetValidateOfferResponseV3;
    fn get_url(&self) -> String {"/lol-purchase-widget/v3/validateOffer".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_purchase_widget_v3_validate_offer(body: LolPurchaseWidgetValidateOfferRequestV3) -> PostLolPurchaseWidgetV3ValidateOffer {
    PostLolPurchaseWidgetV3ValidateOffer{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBaseSkinLineDto {
    pub items: Vec<LolPurchaseWidgetSkinLineItemDto>,
    #[serde(rename = "localizedName")]
    pub localized_name: String,
    #[serde(rename = "skinLineDescriptions")]
    pub skin_line_descriptions: Vec<LolPurchaseWidgetSkinLineDescriptionDto>,
    #[serde(rename = "pricingOptions")]
    pub pricing_options: Vec<LolPurchaseWidgetPriceOptionDto>,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "collectionCardPath")]
    pub collection_card_path: String,
    #[serde(rename = "collectionDescription")]
    pub collection_description: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBundledItemPricingInfo {
    #[serde(rename = "discountPrices")]
    pub discount_prices: Vec<LolPurchaseWidgetDiscountPricingInfo>,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub quantity: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersDataDto {
    pub id: String,
    #[serde(rename = "subOrders")]
    pub sub_orders: Vec<LolPurchaseWidgetCapOrdersSubOrderDto>,
    pub purchaser: LolPurchaseWidgetCapOrdersTypedIdentifierDto,
    pub location: String,
    pub source: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersMetaDto {
    pub xid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferContextDto {
    pub quantity: u32,
    #[serde(rename = "paymentOption")]
    pub payment_option: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferDto {
    pub id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOrderDto {
    pub data: LolPurchaseWidgetCapOrdersDataDto,
    pub meta: LolPurchaseWidgetCapOrdersMetaDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersSubOrderDto {
    #[serde(rename = "recipientId")]
    pub recipient_id: String,
    #[serde(rename = "offerContext")]
    pub offer_context: LolPurchaseWidgetCapOrdersOfferContextDto,
    pub offer: LolPurchaseWidgetCapOrdersOfferDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersTypedIdentifierDto {
    pub id: String,
    #[serde(rename = "typeId")]
    pub type_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItem {
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "itemInstanceId")]
    pub item_instance_id: String,
    pub owned: bool,
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
    #[serde(rename = "purchaseDate")]
    pub purchase_date: u64,
    #[serde(rename = "releaseDate")]
    pub release_date: u64,
    #[serde(rename = "inactiveDate")]
    pub inactive_date: u64,
    pub prices: Vec<LolPurchaseWidgetCatalogPluginPrice>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<Vec<LolPurchaseWidgetItemMetadataEntry>>,
    #[serde(rename = "questSkinInfo")]
    pub quest_skin_info: Option<LolPurchaseWidgetSkinLineInfo>,
    pub active: bool,
    #[serde(rename = "ownershipType")]
    pub ownership_type: Option<LolPurchaseWidgetInventoryOwnership>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItemAssets {
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    pub emblems: Vec<LolPurchaseWidgetChampionSkinEmblem>,
    pub colors: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginPrice {
    pub currency: String,
    pub cost: i64,
    #[serde(rename = "costType")]
    pub cost_type: Option<String>,
    pub sale: Option<LolPurchaseWidgetCatalogPluginSale>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginSale {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub discount: Option<f32>,
    pub cost: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblem {
    pub name: String,
    #[serde(rename = "emblemPath")]
    pub emblem_path: LolPurchaseWidgetChampionSkinEmblemPath,
    #[serde(rename = "emblemPosition")]
    pub emblem_position: LolPurchaseWidgetChampionSkinEmblemPosition,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetDiscountPricingInfo {
    pub cost: i32,
    #[serde(rename = "originalCost")]
    pub original_cost: i32,
    #[serde(rename = "costType")]
    pub cost_type: String,
    pub currency: String,
    pub discount: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoiceDetails {
    pub item: LolPurchaseWidgetCatalogPluginItem,
    #[serde(rename = "backgroundImage")]
    pub background_image: String,
    pub contents: Vec<LolPurchaseWidgetItemDetails>,
    pub discount: String,
    #[serde(rename = "fullPrice")]
    pub full_price: u32,
    #[serde(rename = "displayType")]
    pub display_type: String,
    #[serde(rename = "purchaseOptions")]
    pub purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoices {
    pub choices: Vec<LolPurchaseWidgetItemChoiceDetails>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDefinition {
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "subInventoryType")]
    pub sub_inventory_type: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "subTitle")]
    pub sub_title: String,
    #[serde(rename = "imagePath")]
    pub image_path: String,
    pub owned: bool,
    pub assets: LolPurchaseWidgetCatalogPluginItemAssets,
    pub tags: Vec<String>,
    pub metadata: Vec<LolPurchaseWidgetItemMetadataEntry>,
    #[serde(rename = "bundledItemPrice")]
    pub bundled_item_price: Option<LolPurchaseWidgetBundledItemPricingInfo>,
    #[serde(rename = "loyaltyUnlocked")]
    pub loyalty_unlocked: bool,
    #[serde(rename = "hasVisibleLootOdds")]
    pub has_visible_loot_odds: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDetails {
    pub title: String,
    #[serde(rename = "subTitle")]
    pub sub_title: String,
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemKey {
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemMetadataEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemPrice {
    #[serde(rename = "currencyType")]
    pub currency_type: String,
    pub price: i64,
    pub purchasable: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemSale {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub discount: Option<f32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetOrderNotificationResource {
    #[serde(rename = "eventTypeId")]
    pub event_type_id: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub status: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceDetail {
    #[serde(rename = "itemKey")]
    pub item_key: LolPurchaseWidgetItemKey,
    pub price: LolPurchaseWidgetItemPrice,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceOptionDto {
    pub price: i64,
    #[serde(rename = "currencyType")]
    pub currency_type: String,
    #[serde(rename = "currencyPaymentOption")]
    pub currency_payment_option: Option<String>,
    #[serde(rename = "currencyName")]
    pub currency_name: Option<String>,
    #[serde(rename = "currencyImagePath")]
    pub currency_image_path: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchasableItem {
    pub item: LolPurchaseWidgetItemDefinition,
    pub dependencies: Vec<LolPurchaseWidgetItemDefinition>,
    #[serde(rename = "bundledItems")]
    pub bundled_items: Vec<LolPurchaseWidgetItemDefinition>,
    pub sale: Option<LolPurchaseWidgetItemSale>,
    #[serde(rename = "purchaseOptions")]
    pub purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseItem {
    #[serde(rename = "itemKey")]
    pub item_key: LolPurchaseWidgetItemKey,
    pub quantity: i32,
    pub source: String,
    #[serde(rename = "purchaseCurrencyInfo")]
    pub purchase_currency_info: LolPurchaseWidgetItemPrice,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatus {
    #[serde(rename = "orderState")]
    pub order_state: LolPurchaseWidgetPurchaseOfferOrderStates,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatuses {
    pub statuses: HashMap<String, LolPurchaseWidgetPurchaseOfferOrderStatus>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferRequestV3 {
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "currencyType")]
    pub currency_type: String,
    pub quantity: u32,
    pub price: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferResponseV3 {
    pub legacy: bool,
    #[serde(rename = "orderDto")]
    pub order_dto: Option<LolPurchaseWidgetCapOrdersOrderDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOption {
    #[serde(rename = "priceDetails")]
    pub price_details: Vec<LolPurchaseWidgetPriceDetail>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseRequest {
    pub items: Vec<LolPurchaseWidgetPurchaseItem>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseWidgetConfig {
    pub enabled: bool,
    #[serde(rename = "nonRefundableDisclaimerEnabled")]
    pub non_refundable_disclaimer_enabled: bool,
    #[serde(rename = "alwaysShowPurchaseDisclaimer")]
    pub always_show_purchase_disclaimer: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionDto {
    pub title: String,
    pub description: String,
    #[serde(rename = "iconImagePath")]
    pub icon_image_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionInfo {
    pub title: String,
    pub description: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineInfo {
    pub name: String,
    #[serde(rename = "descriptionInfo")]
    pub description_info: Vec<LolPurchaseWidgetSkinLineDescriptionInfo>,
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
    pub tiers: Vec<LolPurchaseWidgetSkinLineTier>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineItemDto {
    #[serde(rename = "thumbnailImagePath")]
    pub thumbnail_image_path: String,
    #[serde(rename = "largeImagePath")]
    pub large_image_path: Option<String>,
    #[serde(rename = "localizedLongName")]
    pub localized_long_name: String,
    #[serde(rename = "localizedShortName")]
    pub localized_short_name: String,
    #[serde(rename = "largeVideoPath")]
    pub large_video_path: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineTier {
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


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferError {
    #[serde(rename = "errorKey")]
    pub error_key: String,
    pub meta: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferRequestV3 {
    #[serde(rename = "offerId")]
    pub offer_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferResponseV3 {
    #[serde(rename = "validationErrors")]
    pub validation_errors: Vec<LolPurchaseWidgetValidateOfferError>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationErrorEntry {
    pub id: String,
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

