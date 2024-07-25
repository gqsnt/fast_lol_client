
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolEventHubV1Events {}

impl IsApiRequest for GetLolEventHubV1Events {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubActiveEventUiData>;
    fn get_url(&self) -> String {"/lol-event-hub/v1/events".to_string()}
}

pub fn get_lol_event_hub_v1_events() -> GetLolEventHubV1Events {
    GetLolEventHubV1Events{}
}


pub struct GetLolEventHubV1EventsByEventIdEventDetailsData {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdEventDetailsData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubEventDetailsUiData;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/event-details-data", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_event_details_data(event_id: String) -> GetLolEventHubV1EventsByEventIdEventDetailsData {
    GetLolEventHubV1EventsByEventIdEventDetailsData{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdInfo {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdInfo {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubEventInfoUiData;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/info", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_info(event_id: String) -> GetLolEventHubV1EventsByEventIdInfo {
    GetLolEventHubV1EventsByEventIdInfo{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdIsGracePeriod {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdIsGracePeriod {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/is-grace-period", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_is_grace_period(event_id: String) -> GetLolEventHubV1EventsByEventIdIsGracePeriod {
    GetLolEventHubV1EventsByEventIdIsGracePeriod{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdNarrative {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdNarrative {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubNarrativeElement>;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/narrative", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_narrative(event_id: String) -> GetLolEventHubV1EventsByEventIdNarrative {
    GetLolEventHubV1EventsByEventIdNarrative{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdPassBackgroundData {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdPassBackgroundData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubEventBackgroundUiData;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/pass-background-data", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_pass_background_data(event_id: String) -> GetLolEventHubV1EventsByEventIdPassBackgroundData {
    GetLolEventHubV1EventsByEventIdPassBackgroundData{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdPassBundles {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdPassBundles {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubBundleOfferUiData>;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/pass-bundles", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_pass_bundles(event_id: String) -> GetLolEventHubV1EventsByEventIdPassBundles {
    GetLolEventHubV1EventsByEventIdPassBundles{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdProgressInfoData {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdProgressInfoData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubProgressInfoUiData;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/progress-info-data", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_progress_info_data(event_id: String) -> GetLolEventHubV1EventsByEventIdProgressInfoData {
    GetLolEventHubV1EventsByEventIdProgressInfoData{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdProgressionPurchaseData {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdProgressionPurchaseData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubProgressionPurchaseUiData;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/progression-purchase-data", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_progression_purchase_data(event_id: String) -> GetLolEventHubV1EventsByEventIdProgressionPurchaseData {
    GetLolEventHubV1EventsByEventIdProgressionPurchaseData{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdRewardTrackBonusItems {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdRewardTrackBonusItems {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubRewardTrackItem>;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/bonus-items", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_reward_track_bonus_items(event_id: String) -> GetLolEventHubV1EventsByEventIdRewardTrackBonusItems {
    GetLolEventHubV1EventsByEventIdRewardTrackBonusItems{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdRewardTrackBonusProgress {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdRewardTrackBonusProgress {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubRewardTrackProgress;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/bonus-progress", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_reward_track_bonus_progress(event_id: String) -> GetLolEventHubV1EventsByEventIdRewardTrackBonusProgress {
    GetLolEventHubV1EventsByEventIdRewardTrackBonusProgress{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdRewardTrackFailure {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdRewardTrackFailure {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubEventHubError;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/failure", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_reward_track_failure(event_id: String) -> GetLolEventHubV1EventsByEventIdRewardTrackFailure {
    GetLolEventHubV1EventsByEventIdRewardTrackFailure{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdRewardTrackItems {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdRewardTrackItems {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubRewardTrackItem>;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/items", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_reward_track_items(event_id: String) -> GetLolEventHubV1EventsByEventIdRewardTrackItems {
    GetLolEventHubV1EventsByEventIdRewardTrackItems{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdRewardTrackProgress {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdRewardTrackProgress {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubRewardTrackProgress;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/progress", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_reward_track_progress(event_id: String) -> GetLolEventHubV1EventsByEventIdRewardTrackProgress {
    GetLolEventHubV1EventsByEventIdRewardTrackProgress{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdRewardTrackUnclaimedRewards {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdRewardTrackUnclaimedRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubUnclaimedRewardsUiData;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/unclaimed-rewards", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_reward_track_unclaimed_rewards(event_id: String) -> GetLolEventHubV1EventsByEventIdRewardTrackUnclaimedRewards {
    GetLolEventHubV1EventsByEventIdRewardTrackUnclaimedRewards{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdRewardTrackXp {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdRewardTrackXp {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubRewardTrackXp;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/xp", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_reward_track_xp(event_id: String) -> GetLolEventHubV1EventsByEventIdRewardTrackXp {
    GetLolEventHubV1EventsByEventIdRewardTrackXp{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdTokenShop {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdTokenShop {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEventHubTokenShopUiData;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/token-shop", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_token_shop(event_id: String) -> GetLolEventHubV1EventsByEventIdTokenShop {
    GetLolEventHubV1EventsByEventIdTokenShop{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdTokenShopCategoriesOffers {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdTokenShopCategoriesOffers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubCategoryOffersUiData>;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/token-shop/categories-offers", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_token_shop_categories_offers(event_id: String) -> GetLolEventHubV1EventsByEventIdTokenShopCategoriesOffers {
    GetLolEventHubV1EventsByEventIdTokenShopCategoriesOffers{event_id}
}


pub struct GetLolEventHubV1EventsByEventIdTokenShopTokenBalance {
    pub event_id: String,
}

impl IsApiRequest for GetLolEventHubV1EventsByEventIdTokenShopTokenBalance {
    const METHOD: Method = Method::GET;
    type ReturnType = u32;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/token-shop/token-balance", self.event_id)}
}

pub fn get_lol_event_hub_v1_events_by_event_id_token_shop_token_balance(event_id: String) -> GetLolEventHubV1EventsByEventIdTokenShopTokenBalance {
    GetLolEventHubV1EventsByEventIdTokenShopTokenBalance{event_id}
}


pub struct GetLolEventHubV1NavigationButtonData {}

impl IsApiRequest for GetLolEventHubV1NavigationButtonData {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubNavigationButtonUiData>;
    fn get_url(&self) -> String {"/lol-event-hub/v1/navigation-button-data".to_string()}
}

pub fn get_lol_event_hub_v1_navigation_button_data() -> GetLolEventHubV1NavigationButtonData {
    GetLolEventHubV1NavigationButtonData{}
}


pub struct GetLolEventHubV1Skins {}

impl IsApiRequest for GetLolEventHubV1Skins {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, LolEventHubEventPassInfo>;
    fn get_url(&self) -> String {"/lol-event-hub/v1/skins".to_string()}
}

pub fn get_lol_event_hub_v1_skins() -> GetLolEventHubV1Skins {
    GetLolEventHubV1Skins{}
}


pub struct GetLolEventHubV1TokenUpsell {}

impl IsApiRequest for GetLolEventHubV1TokenUpsell {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolEventHubTokenUpsell>;
    fn get_url(&self) -> String {"/lol-event-hub/v1/token-upsell".to_string()}
}

pub fn get_lol_event_hub_v1_token_upsell() -> GetLolEventHubV1TokenUpsell {
    GetLolEventHubV1TokenUpsell{}
}


pub struct PostLolEventHubV1EventsByEventIdPurchaseOffer {
    pub event_id: String,
    pub body: LolEventHubPurchaseOfferRequest,
}

impl IsApiRequest for PostLolEventHubV1EventsByEventIdPurchaseOffer {
    const METHOD: Method = Method::POST;
    type ReturnType = LolEventHubPurchaseOfferResponseV3;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/purchase-offer", self.event_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_event_hub_v1_events_by_event_id_purchase_offer(event_id: String, body: LolEventHubPurchaseOfferRequest) -> PostLolEventHubV1EventsByEventIdPurchaseOffer {
    PostLolEventHubV1EventsByEventIdPurchaseOffer{event_id, body}
}


pub struct PostLolEventHubV1EventsByEventIdRewardTrackClaimAll {
    pub event_id: String,
}

impl IsApiRequest for PostLolEventHubV1EventsByEventIdRewardTrackClaimAll {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-event-hub/v1/events/{}/reward-track/claim-all", self.event_id)}
}

pub fn post_lol_event_hub_v1_events_by_event_id_reward_track_claim_all(event_id: String) -> PostLolEventHubV1EventsByEventIdRewardTrackClaimAll {
    PostLolEventHubV1EventsByEventIdRewardTrackClaimAll{event_id}
}


pub struct PostLolEventHubV1PurchaseItem {
    pub body: LolEventHubItemOrderDto,
}

impl IsApiRequest for PostLolEventHubV1PurchaseItem {
    const METHOD: Method = Method::POST;
    type ReturnType = LolEventHubPurchaseOrderResponseDto;
    fn get_url(&self) -> String {"/lol-event-hub/v1/purchase-item".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_event_hub_v1_purchase_item(body: LolEventHubItemOrderDto) -> PostLolEventHubV1PurchaseItem {
    PostLolEventHubV1PurchaseItem{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubActiveEventUiData {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "eventInfo")]
    pub event_info: LolEventHubEventInfoUiData,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubBundleOfferUiData {
    pub details: LolEventHubBundledItemUiData,
    #[serde(rename = "initialPrice")]
    pub initial_price: i64,
    #[serde(rename = "finalPrice")]
    pub final_price: i64,
    #[serde(rename = "futureBalance")]
    pub future_balance: i64,
    #[serde(rename = "isPurchasable")]
    pub is_purchasable: bool,
    #[serde(rename = "discountPercentage")]
    pub discount_percentage: f64,
    #[serde(rename = "bundledItems")]
    pub bundled_items: Vec<LolEventHubBundledItemUiData>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubBundledItemUiData {
    pub name: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub description: String,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "subInventoryType")]
    pub sub_inventory_type: String,
    #[serde(rename = "splashImage")]
    pub splash_image: String,
    pub owned: bool,
    pub quantity: u32,
    #[serde(rename = "decoratorBadgeURL")]
    pub decorator_badge_url: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersDataDto {
    pub id: String,
    #[serde(rename = "subOrders")]
    pub sub_orders: Vec<LolEventHubCapOrdersSubOrderDto>,
    pub purchaser: LolEventHubCapOrdersTypedIdentifierDto,
    pub location: String,
    pub source: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersMetaDto {
    pub xid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersOfferContextDto {
    pub quantity: u32,
    #[serde(rename = "paymentOption")]
    pub payment_option: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersOfferDto {
    pub id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersOrderDto {
    pub data: LolEventHubCapOrdersDataDto,
    pub meta: LolEventHubCapOrdersMetaDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersSubOrderDto {
    #[serde(rename = "recipientId")]
    pub recipient_id: String,
    #[serde(rename = "offerContext")]
    pub offer_context: LolEventHubCapOrdersOfferContextDto,
    pub offer: LolEventHubCapOrdersOfferDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersTypedIdentifierDto {
    pub id: String,
    #[serde(rename = "typeId")]
    pub type_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogEntry {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "typeId")]
    pub type_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCategoryOffersUiData {
    pub category: LolEventHubOfferCategory,
    #[serde(rename = "categoryIconPath")]
    pub category_icon_path: String,
    pub offers: Vec<LolEventHubOfferUiData>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventBackgroundUiData {
    #[serde(rename = "backgroundImagePath")]
    pub background_image_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventDetailsUiData {
    #[serde(rename = "eventIconPath")]
    pub event_icon_path: String,
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(rename = "headerTitleImagePath")]
    pub header_title_image_path: String,
    #[serde(rename = "progressEndDate")]
    pub progress_end_date: String,
    #[serde(rename = "shopEndDate")]
    pub shop_end_date: String,
    #[serde(rename = "eventStartDate")]
    pub event_start_date: String,
    #[serde(rename = "helpModalImagePath")]
    pub help_modal_image_path: String,
    #[serde(rename = "inducteeName")]
    pub inductee_name: String,
    #[serde(rename = "promotionBannerImage")]
    pub promotion_banner_image: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventHubError {
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "errorId")]
    pub error_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventInfoUiData {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "eventIcon")]
    pub event_icon: String,
    #[serde(rename = "navBarIcon")]
    pub nav_bar_icon: String,
    #[serde(rename = "eventTokenImage")]
    pub event_token_image: String,
    #[serde(rename = "currentTokenBalance")]
    pub current_token_balance: i32,
    #[serde(rename = "lockedTokenCount")]
    pub locked_token_count: i32,
    #[serde(rename = "unclaimedRewardCount")]
    pub unclaimed_reward_count: i32,
    #[serde(rename = "timeOfLastUnclaimedReward")]
    pub time_of_last_unclaimed_reward: i64,
    #[serde(rename = "isPassPurchased")]
    pub is_pass_purchased: bool,
    #[serde(rename = "eventPassBundles")]
    pub event_pass_bundles: Vec<LolEventHubCatalogEntry>,
    #[serde(rename = "tokenBundles")]
    pub token_bundles: Vec<LolEventHubCatalogEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventPassInfo {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "isPassPurchased")]
    pub is_pass_purchased: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemOrderDto {
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub quantity: u32,
    #[serde(rename = "rpCost")]
    pub rp_cost: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemUiData {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    pub price: u32,
    pub quantity: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubNarrativeElement {
    #[serde(rename = "localizedNarrativeTitle")]
    pub localized_narrative_title: String,
    #[serde(rename = "localizedNarrativeDescription")]
    pub localized_narrative_description: String,
    #[serde(rename = "narrativeBackgroundImage")]
    pub narrative_background_image: String,
    #[serde(rename = "narrativeStartingTrackLevel")]
    pub narrative_starting_track_level: u16,
    #[serde(rename = "narrativeVideo")]
    pub narrative_video: LolEventHubNarrativeVideo,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubNarrativeVideo {
    #[serde(rename = "localizedNarrativeVideoUrl")]
    pub localized_narrative_video_url: String,
    #[serde(rename = "localizedPlayNarrativeButtonLabel")]
    pub localized_play_narrative_button_label: String,
    #[serde(rename = "narrativeVideoIsLockedOnLevel")]
    pub narrative_video_is_locked_on_level: Option<bool>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubNavigationButtonUiData {
    #[serde(rename = "activeEventId")]
    pub active_event_id: String,
    #[serde(rename = "showPip")]
    pub show_pip: bool,
    #[serde(rename = "showGlow")]
    pub show_glow: bool,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "eventName")]
    pub event_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubOfferUiData {
    pub id: String,
    #[serde(rename = "localizedTitle")]
    pub localized_title: String,
    #[serde(rename = "localizedDescription")]
    pub localized_description: String,
    pub image: String,
    pub highlighted: bool,
    #[serde(rename = "offerState")]
    pub offer_state: LolEventHubOfferStates,
    pub price: u32,
    #[serde(rename = "maxQuantity")]
    pub max_quantity: u32,
    pub items: Vec<LolEventHubItemUiData>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubProgressInfoUiData {
    #[serde(rename = "tokenImage")]
    pub token_image: String,
    #[serde(rename = "passPurchased")]
    pub pass_purchased: bool,
    #[serde(rename = "eventPassBundlesCatalogEntry")]
    pub event_pass_bundles_catalog_entry: Vec<LolEventHubCatalogEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubProgressionPurchaseUiData {
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "pricePerLevel")]
    pub price_per_level: i64,
    #[serde(rename = "rpBalance")]
    pub rp_balance: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOfferRequest {
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "purchaseQuantity")]
    pub purchase_quantity: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOfferResponseV3 {
    pub legacy: bool,
    #[serde(rename = "orderDto")]
    pub order_dto: Option<LolEventHubCapOrdersOrderDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOrderResponseDto {
    #[serde(rename = "rpBalance")]
    pub rp_balance: i64,
    #[serde(rename = "ipBalance")]
    pub ip_balance: i64,
    pub transactions: Vec<LolEventHubTransactionResponseDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrackItem {
    pub state: LolEventHubRewardTrackItemStates,
    #[serde(rename = "rewardOptions")]
    pub reward_options: Vec<LolEventHubRewardTrackItemOption>,
    #[serde(rename = "rewardTags")]
    pub reward_tags: Vec<LolEventHubRewardTrackItemTag>,
    #[serde(rename = "progressRequired")]
    pub progress_required: i64,
    pub threshold: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrackItemOption {
    pub state: LolEventHubRewardTrackItemStates,
    #[serde(rename = "thumbIconPath")]
    pub thumb_icon_path: String,
    #[serde(rename = "splashImagePath")]
    pub splash_image_path: String,
    pub selected: bool,
    #[serde(rename = "overrideFooter")]
    pub override_footer: String,
    #[serde(rename = "headerType")]
    pub header_type: LolEventHubRewardTrackItemHeaderType,
    #[serde(rename = "rewardName")]
    pub reward_name: String,
    #[serde(rename = "rewardDescription")]
    pub reward_description: String,
    #[serde(rename = "cardSize")]
    pub card_size: String,
    #[serde(rename = "rewardGroupId")]
    pub reward_group_id: String,
    #[serde(rename = "celebrationType")]
    pub celebration_type: LolEventHubCelebrationType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrackProgress {
    pub level: i16,
    #[serde(rename = "totalLevels")]
    pub total_levels: i16,
    #[serde(rename = "levelProgress")]
    pub level_progress: u16,
    #[serde(rename = "futureLevelProgress")]
    pub future_level_progress: u16,
    #[serde(rename = "passProgress")]
    pub pass_progress: i64,
    #[serde(rename = "currentLevelXP")]
    pub current_level_xp: i64,
    #[serde(rename = "totalLevelXP")]
    pub total_level_xp: i64,
    pub iteration: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrackXp {
    #[serde(rename = "currentLevel")]
    pub current_level: i64,
    #[serde(rename = "currentLevelXP")]
    pub current_level_xp: i64,
    #[serde(rename = "totalLevelXP")]
    pub total_level_xp: i64,
    #[serde(rename = "isBonusPhase")]
    pub is_bonus_phase: bool,
    pub iteration: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTokenShopUiData {
    #[serde(rename = "tokenName")]
    pub token_name: String,
    #[serde(rename = "tokenImage")]
    pub token_image: String,
    #[serde(rename = "tokenUuid")]
    pub token_uuid: String,
    #[serde(rename = "offersVersion")]
    pub offers_version: u32,
    #[serde(rename = "tokenBundlesCatalogEntry")]
    pub token_bundles_catalog_entry: Vec<LolEventHubCatalogEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTokenUpsell {
    pub id: String,
    #[serde(rename = "internalName")]
    pub internal_name: String,
    pub title: String,
    #[serde(rename = "buttonText")]
    pub button_text: String,
    #[serde(rename = "tooltipTitle")]
    pub tooltip_title: String,
    #[serde(rename = "tooltipDescription")]
    pub tooltip_description: String,
    #[serde(rename = "purchaseUrl")]
    pub purchase_url: String,
    #[serde(rename = "tooltipBackgroundUrl")]
    pub tooltip_background_url: String,
    #[serde(rename = "backgroundUrl")]
    pub background_url: String,
    #[serde(rename = "currencyUrl")]
    pub currency_url: String,
    #[serde(rename = "premiumCurrencyName")]
    pub premium_currency_name: String,
    #[serde(rename = "dependentInventoryType")]
    pub dependent_inventory_type: String,
    #[serde(rename = "dependentInventoryId")]
    pub dependent_inventory_id: i32,
    #[serde(rename = "currentlyLocked")]
    pub currently_locked: LolEventHubTokenUpsellLockedType,
    #[serde(rename = "lockedCount")]
    pub locked_count: i32,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTransactionResponseDto {
    pub id: String,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubUnclaimedRewardsUiData {
    #[serde(rename = "rewardsCount")]
    pub rewards_count: i32,
    #[serde(rename = "lockedTokensCount")]
    pub locked_tokens_count: i32,
    #[serde(rename = "timeOfLastUnclaimedReward")]
    pub time_of_last_unclaimed_reward: i64,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolEventHubCelebrationType {
    #[default]
    #[serde(rename = "FULLSCREEN")]
    Fullscreen,
    #[serde(rename = "TOAST")]
    Toast,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolEventHubOfferCategory {
    #[default]
    Currencies,
    Tft,
    Loot,
    Borders,
    Skins,
    Chromas,
    Featured,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolEventHubOfferStates {
    #[default]
    #[serde(rename = "kPurchasing")]
    KPurchasing,
    #[serde(rename = "kUnrevealed")]
    KUnrevealed,
    #[serde(rename = "kUnavailable")]
    KUnavailable,
    #[serde(rename = "kAvailable")]
    KAvailable,
    #[serde(rename = "kOwned")]
    KOwned,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolEventHubRewardTrackItemHeaderType {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "PREMIUM")]
    Premium,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolEventHubRewardTrackItemStates {
    #[default]
    Selected,
    Unselected,
    Unlocked,
    Locked,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolEventHubRewardTrackItemTag {
    #[default]
    Multiple,
    Choice,
    Instant,
    Free,
    Rare,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolEventHubTokenUpsellLockedType {
    #[default]
    #[serde(rename = "UNLOCKED")]
    Unlocked,
    #[serde(rename = "LOCKED")]
    Locked,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}

