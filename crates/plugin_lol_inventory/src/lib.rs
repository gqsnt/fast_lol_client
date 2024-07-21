
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolInventoryV1ChampSelectInventory {}

impl IsApiRequest for GetLolInventoryV1ChampSelectInventory {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-inventory/v1/champSelectInventory".to_string()}
}

pub fn get_lol_inventory_v_1_champ_select_inventory() -> GetLolInventoryV1ChampSelectInventory {
    GetLolInventoryV1ChampSelectInventory{}
}


pub struct GetLolInventoryV1InitialConfigurationComplete {}

impl IsApiRequest for GetLolInventoryV1InitialConfigurationComplete {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-inventory/v1/initial-configuration-complete".to_string()}
}

pub fn get_lol_inventory_v_1_initial_configuration_complete() -> GetLolInventoryV1InitialConfigurationComplete {
    GetLolInventoryV1InitialConfigurationComplete{}
}


pub struct GetLolInventoryV1Inventory {
    pub inventory_types: Vec<String>,
}

impl IsApiRequest for GetLolInventoryV1Inventory {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolInventoryInventoryItemWithPayload>;
    fn get_url(&self) -> String {"/lol-inventory/v1/inventory".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryTypes" : self.inventory_types,
        }))
    }
}

pub fn get_lol_inventory_v_1_inventory(inventory_types: Vec<String>) -> GetLolInventoryV1Inventory {
    GetLolInventoryV1Inventory{inventory_types}
}


pub struct GetLolInventoryV1InventoryEmotes {}

impl IsApiRequest for GetLolInventoryV1InventoryEmotes {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolInventoryInventoryItemWithPayload>;
    fn get_url(&self) -> String {"/lol-inventory/v1/inventory/emotes".to_string()}
}

pub fn get_lol_inventory_v_1_inventory_emotes() -> GetLolInventoryV1InventoryEmotes {
    GetLolInventoryV1InventoryEmotes{}
}


pub struct GetLolInventoryV1InventoryWithF2P {
    pub inventory_types: Vec<String>,
}

impl IsApiRequest for GetLolInventoryV1InventoryWithF2P {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolInventoryInventoryItemWithPayload>;
    fn get_url(&self) -> String {"/lol-inventory/v1/inventoryWithF2P".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryTypes" : self.inventory_types,
        }))
    }
}

pub fn get_lol_inventory_v_1_inventory_with_f_2_p(inventory_types: Vec<String>) -> GetLolInventoryV1InventoryWithF2P {
    GetLolInventoryV1InventoryWithF2P{inventory_types}
}


pub struct GetLolInventoryV1NotificationsByInventoryType {
    pub inventory_type: String,
}

impl IsApiRequest for GetLolInventoryV1NotificationsByInventoryType {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolInventoryInventoryNotification>;
    fn get_url(&self) -> String {format!("/lol-inventory/v1/notifications/{}", self.inventory_type)}
}

pub fn get_lol_inventory_v_1_notifications_by_inventory_type(inventory_type: String) -> GetLolInventoryV1NotificationsByInventoryType {
    GetLolInventoryV1NotificationsByInventoryType{inventory_type}
}


pub struct GetLolInventoryV1PlayersByPuuidInventory {
    pub puuid: String,
    pub inventory_types: Vec<String>,
}

impl IsApiRequest for GetLolInventoryV1PlayersByPuuidInventory {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolInventoryInventoryItemWithPayload>;
    fn get_url(&self) -> String {format!("/lol-inventory/v1/players/{}/inventory", self.puuid)}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryTypes" : self.inventory_types,
        }))
    }
}

pub fn get_lol_inventory_v_1_players_by_puuid_inventory(puuid: String, inventory_types: Vec<String>) -> GetLolInventoryV1PlayersByPuuidInventory {
    GetLolInventoryV1PlayersByPuuidInventory{puuid, inventory_types}
}


pub struct GetLolInventoryV1SignedInventory {
    pub inventory_types: Vec<String>,
}

impl IsApiRequest for GetLolInventoryV1SignedInventory {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-inventory/v1/signedInventory".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryTypes" : self.inventory_types,
        }))
    }
}

pub fn get_lol_inventory_v_1_signed_inventory(inventory_types: Vec<String>) -> GetLolInventoryV1SignedInventory {
    GetLolInventoryV1SignedInventory{inventory_types}
}


pub struct GetLolInventoryV1SignedInventoryCache {}

impl IsApiRequest for GetLolInventoryV1SignedInventoryCache {
    const METHOD: Method = Method::GET;
    type ReturnType = LolInventoryInventoryCacheEntry;
    fn get_url(&self) -> String {"/lol-inventory/v1/signedInventoryCache".to_string()}
}

pub fn get_lol_inventory_v_1_signed_inventory_cache() -> GetLolInventoryV1SignedInventoryCache {
    GetLolInventoryV1SignedInventoryCache{}
}


pub struct GetLolInventoryV1SignedInventorySimple {
    pub inventory_types: Vec<String>,
    pub query_params: Option<HashMap<String, String>>,
}

impl IsApiRequest for GetLolInventoryV1SignedInventorySimple {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-inventory/v1/signedInventory/simple".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "inventoryTypes" : self.inventory_types,
            "queryParams" : self.query_params,
        }))
    }
}

pub fn get_lol_inventory_v_1_signed_inventory_simple(inventory_types: Vec<String>, query_params: Option<HashMap<String, String>>) -> GetLolInventoryV1SignedInventorySimple {
    GetLolInventoryV1SignedInventorySimple{inventory_types, query_params}
}


pub struct GetLolInventoryV1SignedInventoryTournamentlogos {}

impl IsApiRequest for GetLolInventoryV1SignedInventoryTournamentlogos {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-inventory/v1/signedInventory/tournamentlogos".to_string()}
}

pub fn get_lol_inventory_v_1_signed_inventory_tournamentlogos() -> GetLolInventoryV1SignedInventoryTournamentlogos {
    GetLolInventoryV1SignedInventoryTournamentlogos{}
}


pub struct GetLolInventoryV1SignedWallet {
    pub currency_types: Vec<String>,
}

impl IsApiRequest for GetLolInventoryV1SignedWallet {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-inventory/v1/signedWallet".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "currencyTypes" : self.currency_types,
        }))
    }
}

pub fn get_lol_inventory_v_1_signed_wallet(currency_types: Vec<String>) -> GetLolInventoryV1SignedWallet {
    GetLolInventoryV1SignedWallet{currency_types}
}


pub struct GetLolInventoryV1SignedWalletByCurrencyType {
    pub currency_type: String,
}

impl IsApiRequest for GetLolInventoryV1SignedWalletByCurrencyType {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-inventory/v1/signedWallet/{}", self.currency_type)}
}

pub fn get_lol_inventory_v_1_signed_wallet_by_currency_type(currency_type: String) -> GetLolInventoryV1SignedWalletByCurrencyType {
    GetLolInventoryV1SignedWalletByCurrencyType{currency_type}
}


pub struct GetLolInventoryV1StrawberryInventory {}

impl IsApiRequest for GetLolInventoryV1StrawberryInventory {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-inventory/v1/strawberryInventory".to_string()}
}

pub fn get_lol_inventory_v_1_strawberry_inventory() -> GetLolInventoryV1StrawberryInventory {
    GetLolInventoryV1StrawberryInventory{}
}


pub struct GetLolInventoryV1Wallet {
    pub currency_types: Vec<String>,
}

impl IsApiRequest for GetLolInventoryV1Wallet {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, i32>;
    fn get_url(&self) -> String {"/lol-inventory/v1/wallet".to_string()}
    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "currencyTypes" : self.currency_types,
        }))
    }
}

pub fn get_lol_inventory_v_1_wallet(currency_types: Vec<String>) -> GetLolInventoryV1Wallet {
    GetLolInventoryV1Wallet{currency_types}
}


pub struct GetLolInventoryV1WalletByCurrencyType {
    pub currency_type: String,
}

impl IsApiRequest for GetLolInventoryV1WalletByCurrencyType {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, i32>;
    fn get_url(&self) -> String {format!("/lol-inventory/v1/wallet/{}", self.currency_type)}
}

pub fn get_lol_inventory_v_1_wallet_by_currency_type(currency_type: String) -> GetLolInventoryV1WalletByCurrencyType {
    GetLolInventoryV1WalletByCurrencyType{currency_type}
}


pub struct GetLolInventoryV1XboxSubscriptionStatus {}

impl IsApiRequest for GetLolInventoryV1XboxSubscriptionStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolInventoryXboxSubscriptionStatus;
    fn get_url(&self) -> String {"/lol-inventory/v1/xbox-subscription-status".to_string()}
}

pub fn get_lol_inventory_v_1_xbox_subscription_status() -> GetLolInventoryV1XboxSubscriptionStatus {
    GetLolInventoryV1XboxSubscriptionStatus{}
}


pub struct GetLolInventoryV2InventoryByInventoryType {
    pub inventory_type: String,
}

impl IsApiRequest for GetLolInventoryV2InventoryByInventoryType {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolInventoryInventoryItemWithPayload>;
    fn get_url(&self) -> String {format!("/lol-inventory/v2/inventory/{}", self.inventory_type)}
}

pub fn get_lol_inventory_v_2_inventory_by_inventory_type(inventory_type: String) -> GetLolInventoryV2InventoryByInventoryType {
    GetLolInventoryV2InventoryByInventoryType{inventory_type}
}


pub struct PostLolInventoryV1NotificationAcknowledge {
    pub body: i64,
}

impl IsApiRequest for PostLolInventoryV1NotificationAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-inventory/v1/notification/acknowledge".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_inventory_v_1_notification_acknowledge(body: i64) -> PostLolInventoryV1NotificationAcknowledge {
    PostLolInventoryV1NotificationAcknowledge{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryCacheEntry {
    pub signed_inventory_jwt: String,
    pub expiration_ms: u64,
    pub issued_at_ms: u64,
    pub received_at_ms: u64,
    pub valid: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryItemWithPayload {
    pub uuid: String,
    pub item_id: i32,
    pub inventory_type: String,
    pub purchase_date: String,
    pub quantity: u64,
    pub ownership_type: LolInventoryItemOwnershipType,
    pub expiration_date: String,
    pub f_2_p: bool,
    pub rental: bool,
    pub loyalty: bool,
    pub loyalty_sources: Vec<String>,
    pub owned: bool,
    pub wins: u64,
    pub payload: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryNotification {
    pub id: i64,
    pub item_id: i32,
    pub inventory_type: String,
    pub type_: String,
    pub acknowledged: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryXboxSubscriptionStatus {
    pub active: String,
    pub subscription_id: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolInventoryItemOwnershipType {
    #[default]
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}

