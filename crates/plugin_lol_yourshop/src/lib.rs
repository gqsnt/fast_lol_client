
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolYourshopV1HasPermissions {}

impl IsApiRequest for GetLolYourshopV1HasPermissions {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-yourshop/v1/has-permissions".to_string()}
}

pub fn get_lol_yourshop_v1_has_permissions() -> GetLolYourshopV1HasPermissions {
    GetLolYourshopV1HasPermissions{}
}


pub struct GetLolYourshopV1Modal {}

impl IsApiRequest for GetLolYourshopV1Modal {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-yourshop/v1/modal".to_string()}
}

pub fn get_lol_yourshop_v1_modal() -> GetLolYourshopV1Modal {
    GetLolYourshopV1Modal{}
}


pub struct GetLolYourshopV1Offers {}

impl IsApiRequest for GetLolYourshopV1Offers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolYourshopUiOffer>;
    fn get_url(&self) -> String {"/lol-yourshop/v1/offers".to_string()}
}

pub fn get_lol_yourshop_v1_offers() -> GetLolYourshopV1Offers {
    GetLolYourshopV1Offers{}
}


pub struct GetLolYourshopV1Ready {}

impl IsApiRequest for GetLolYourshopV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-yourshop/v1/ready".to_string()}
}

pub fn get_lol_yourshop_v1_ready() -> GetLolYourshopV1Ready {
    GetLolYourshopV1Ready{}
}


pub struct GetLolYourshopV1Status {}

impl IsApiRequest for GetLolYourshopV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = LolYourshopUiStatus;
    fn get_url(&self) -> String {"/lol-yourshop/v1/status".to_string()}
}

pub fn get_lol_yourshop_v1_status() -> GetLolYourshopV1Status {
    GetLolYourshopV1Status{}
}


pub struct GetLolYourshopV1Themed {}

impl IsApiRequest for GetLolYourshopV1Themed {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-yourshop/v1/themed".to_string()}
}

pub fn get_lol_yourshop_v1_themed() -> GetLolYourshopV1Themed {
    GetLolYourshopV1Themed{}
}


pub struct PostLolYourshopV1OffersByIdPurchase {
    pub id: String,
}

impl IsApiRequest for PostLolYourshopV1OffersByIdPurchase {
    const METHOD: Method = Method::POST;
    type ReturnType = LolYourshopPurchaseResponse;
    fn get_url(&self) -> String {format!("/lol-yourshop/v1/offers/{}/purchase", self.id)}
}

pub fn post_lol_yourshop_v1_offers_by_id_purchase(id: String) -> PostLolYourshopV1OffersByIdPurchase {
    PostLolYourshopV1OffersByIdPurchase{id}
}


pub struct PostLolYourshopV1OffersByIdReveal {
    pub id: String,
}

impl IsApiRequest for PostLolYourshopV1OffersByIdReveal {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolYourshopUiOffer>;
    fn get_url(&self) -> String {format!("/lol-yourshop/v1/offers/{}/reveal", self.id)}
}

pub fn post_lol_yourshop_v1_offers_by_id_reveal(id: String) -> PostLolYourshopV1OffersByIdReveal {
    PostLolYourshopV1OffersByIdReveal{id}
}


pub struct PostLolYourshopV1Permissions {
    pub body: LolYourshopPlayerPermissions,
}

impl IsApiRequest for PostLolYourshopV1Permissions {
    const METHOD: Method = Method::POST;
    type ReturnType = LolYourshopPlayerPermissions;
    fn get_url(&self) -> String {"/lol-yourshop/v1/permissions".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_yourshop_v1_permissions(body: LolYourshopPlayerPermissions) -> PostLolYourshopV1Permissions {
    PostLolYourshopV1Permissions{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPlayerPermissions {
    #[serde(rename = "useData")]
    pub use_data: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPurchaseItem {
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "inventoryType")]
    pub inventory_type: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "pricePaid")]
    pub price_paid: i64,
    #[serde(rename = "orderId")]
    pub order_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPurchaseResponse {
    pub items: Vec<LolYourshopPurchaseItem>,
    pub wallet: LolYourshopWallet,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopUiOffer {
    pub id: String,
    #[serde(rename = "originalPrice")]
    pub original_price: i64,
    #[serde(rename = "discountPrice")]
    pub discount_price: i64,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "skinName")]
    pub skin_name: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    pub owned: bool,
    pub revealed: bool,
    pub purchasing: bool,
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopUiStatus {
    #[serde(rename = "hubEnabled")]
    pub hub_enabled: bool,
    pub name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopWallet {
    pub rp: i64,
}


// ENUMS

