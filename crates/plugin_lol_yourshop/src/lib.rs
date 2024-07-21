
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolYourshopV1HasPermissions {

}

impl IsApiRequest for GetLolYourshopV1HasPermissions {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-yourshop/v1/has-permissions".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_yourshop_v_1_has_permissions() -> GetLolYourshopV1HasPermissions {
    GetLolYourshopV1HasPermissions {
        
    }
}


pub struct GetLolYourshopV1Modal {

}

impl IsApiRequest for GetLolYourshopV1Modal {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-yourshop/v1/modal".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_yourshop_v_1_modal() -> GetLolYourshopV1Modal {
    GetLolYourshopV1Modal {
        
    }
}


pub struct GetLolYourshopV1Offers {

}

impl IsApiRequest for GetLolYourshopV1Offers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolYourshopUiOffer>;

    fn get_url(&self) -> String {
        "/lol-yourshop/v1/offers".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_yourshop_v_1_offers() -> GetLolYourshopV1Offers {
    GetLolYourshopV1Offers {
        
    }
}


pub struct GetLolYourshopV1Ready {

}

impl IsApiRequest for GetLolYourshopV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-yourshop/v1/ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_yourshop_v_1_ready() -> GetLolYourshopV1Ready {
    GetLolYourshopV1Ready {
        
    }
}


pub struct GetLolYourshopV1Status {

}

impl IsApiRequest for GetLolYourshopV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = LolYourshopUiStatus;

    fn get_url(&self) -> String {
        "/lol-yourshop/v1/status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_yourshop_v_1_status() -> GetLolYourshopV1Status {
    GetLolYourshopV1Status {
        
    }
}


pub struct GetLolYourshopV1Themed {

}

impl IsApiRequest for GetLolYourshopV1Themed {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-yourshop/v1/themed".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_yourshop_v_1_themed() -> GetLolYourshopV1Themed {
    GetLolYourshopV1Themed {
        
    }
}


pub struct PostLolYourshopV1OffersByIdPurchase {

    pub id: String,
}

impl IsApiRequest for PostLolYourshopV1OffersByIdPurchase {
    const METHOD: Method = Method::POST;
    type ReturnType = LolYourshopPurchaseResponse;

    fn get_url(&self) -> String {
        format!("/lol-yourshop/v1/offers/{}/purchase", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_yourshop_v_1_offers_by_id_purchase(id: String) -> PostLolYourshopV1OffersByIdPurchase {
    PostLolYourshopV1OffersByIdPurchase {
        id
    }
}


pub struct PostLolYourshopV1OffersByIdReveal {

    pub id: String,
}

impl IsApiRequest for PostLolYourshopV1OffersByIdReveal {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolYourshopUiOffer>;

    fn get_url(&self) -> String {
        format!("/lol-yourshop/v1/offers/{}/reveal", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_yourshop_v_1_offers_by_id_reveal(id: String) -> PostLolYourshopV1OffersByIdReveal {
    PostLolYourshopV1OffersByIdReveal {
        id
    }
}


pub struct PostLolYourshopV1Permissions {

    pub body: LolYourshopPlayerPermissions,
}

impl IsApiRequest for PostLolYourshopV1Permissions {
    const METHOD: Method = Method::POST;
    type ReturnType = LolYourshopPlayerPermissions;

    fn get_url(&self) -> String {
        "/lol-yourshop/v1/permissions".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_yourshop_v_1_permissions(body: LolYourshopPlayerPermissions) -> PostLolYourshopV1Permissions {
    PostLolYourshopV1Permissions {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPlayerPermissions {
    pub use_data: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPurchaseItem {
    pub offer_id: String,
    pub inventory_type: String,
    pub item_id: i32,
    pub price_paid: i64,
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
    pub original_price: i64,
    pub discount_price: i64,
    pub type_: String,
    pub skin_name: String,
    pub champion_id: i32,
    pub skin_id: i32,
    pub owned: bool,
    pub revealed: bool,
    pub purchasing: bool,
    pub expiration_date: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopUiStatus {
    pub hub_enabled: bool,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopWallet {
    pub rp: i64,
}


// ENUMS

