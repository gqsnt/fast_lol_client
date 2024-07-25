
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId {
    pub cosmetic_type: String,
    pub content_id: String,
}

impl IsApiRequest for DeleteLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-cosmetics/v1/favorites/tft/{}/{}", self.cosmetic_type, self.content_id)}
}

pub fn delete_lol_cosmetics_v1_favorites_tft_by_cosmetic_type_by_content_id(cosmetic_type: String, content_id: String) -> DeleteLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId {
    DeleteLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId{cosmetic_type, content_id}
}


pub struct DeleteLolCosmeticsV1SelectionCompanion {}

impl IsApiRequest for DeleteLolCosmeticsV1SelectionCompanion {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/companion".to_string()}
}

pub fn delete_lol_cosmetics_v1_selection_companion() -> DeleteLolCosmeticsV1SelectionCompanion {
    DeleteLolCosmeticsV1SelectionCompanion{}
}


pub struct DeleteLolCosmeticsV1SelectionPlaybook {}

impl IsApiRequest for DeleteLolCosmeticsV1SelectionPlaybook {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/playbook".to_string()}
}

pub fn delete_lol_cosmetics_v1_selection_playbook() -> DeleteLolCosmeticsV1SelectionPlaybook {
    DeleteLolCosmeticsV1SelectionPlaybook{}
}


pub struct DeleteLolCosmeticsV1SelectionTftDamageSkin {}

impl IsApiRequest for DeleteLolCosmeticsV1SelectionTftDamageSkin {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/tft-damage-skin".to_string()}
}

pub fn delete_lol_cosmetics_v1_selection_tft_damage_skin() -> DeleteLolCosmeticsV1SelectionTftDamageSkin {
    DeleteLolCosmeticsV1SelectionTftDamageSkin{}
}


pub struct DeleteLolCosmeticsV1SelectionTftMapSkin {}

impl IsApiRequest for DeleteLolCosmeticsV1SelectionTftMapSkin {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/tft-map-skin".to_string()}
}

pub fn delete_lol_cosmetics_v1_selection_tft_map_skin() -> DeleteLolCosmeticsV1SelectionTftMapSkin {
    DeleteLolCosmeticsV1SelectionTftMapSkin{}
}


pub struct GetLolCosmeticsV1FavoritesTftCompanions {}

impl IsApiRequest for GetLolCosmeticsV1FavoritesTftCompanions {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCosmeticsCompanionsFavoritesViewModel;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/favorites/tft/companions".to_string()}
}

pub fn get_lol_cosmetics_v1_favorites_tft_companions() -> GetLolCosmeticsV1FavoritesTftCompanions {
    GetLolCosmeticsV1FavoritesTftCompanions{}
}


pub struct GetLolCosmeticsV1FavoritesTftDamageSkins {}

impl IsApiRequest for GetLolCosmeticsV1FavoritesTftDamageSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCosmeticsTftDamageSkinFavoritesViewModel;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/favorites/tft/damage-skins".to_string()}
}

pub fn get_lol_cosmetics_v1_favorites_tft_damage_skins() -> GetLolCosmeticsV1FavoritesTftDamageSkins {
    GetLolCosmeticsV1FavoritesTftDamageSkins{}
}


pub struct GetLolCosmeticsV1FavoritesTftMapSkins {}

impl IsApiRequest for GetLolCosmeticsV1FavoritesTftMapSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCosmeticsTftMapSkinFavoritesViewModel;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/favorites/tft/map-skins".to_string()}
}

pub fn get_lol_cosmetics_v1_favorites_tft_map_skins() -> GetLolCosmeticsV1FavoritesTftMapSkins {
    GetLolCosmeticsV1FavoritesTftMapSkins{}
}


pub struct GetLolCosmeticsV1InventoriesBySetNameCompanions {
    pub set_name: String,
}

impl IsApiRequest for GetLolCosmeticsV1InventoriesBySetNameCompanions {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCosmeticsCompanionsGroupedViewModel;
    fn get_url(&self) -> String {format!("/lol-cosmetics/v1/inventories/{}/companions", self.set_name)}
}

pub fn get_lol_cosmetics_v1_inventories_by_set_name_companions(set_name: String) -> GetLolCosmeticsV1InventoriesBySetNameCompanions {
    GetLolCosmeticsV1InventoriesBySetNameCompanions{set_name}
}


pub struct GetLolCosmeticsV1InventoriesBySetNameDamageSkins {
    pub set_name: String,
}

impl IsApiRequest for GetLolCosmeticsV1InventoriesBySetNameDamageSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCosmeticsTftDamageSkinGroupedViewModel;
    fn get_url(&self) -> String {format!("/lol-cosmetics/v1/inventories/{}/damage-skins", self.set_name)}
}

pub fn get_lol_cosmetics_v1_inventories_by_set_name_damage_skins(set_name: String) -> GetLolCosmeticsV1InventoriesBySetNameDamageSkins {
    GetLolCosmeticsV1InventoriesBySetNameDamageSkins{set_name}
}


pub struct GetLolCosmeticsV1InventoriesBySetNameMapSkins {
    pub set_name: String,
}

impl IsApiRequest for GetLolCosmeticsV1InventoriesBySetNameMapSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCosmeticsTftMapSkinGroupedViewModel;
    fn get_url(&self) -> String {format!("/lol-cosmetics/v1/inventories/{}/map-skins", self.set_name)}
}

pub fn get_lol_cosmetics_v1_inventories_by_set_name_map_skins(set_name: String) -> GetLolCosmeticsV1InventoriesBySetNameMapSkins {
    GetLolCosmeticsV1InventoriesBySetNameMapSkins{set_name}
}


pub struct GetLolCosmeticsV1InventoriesBySetNamePlaybooks {
    pub set_name: String,
}

impl IsApiRequest for GetLolCosmeticsV1InventoriesBySetNamePlaybooks {
    const METHOD: Method = Method::GET;
    type ReturnType = LolCosmeticsTftPlaybookGroupedViewModel;
    fn get_url(&self) -> String {format!("/lol-cosmetics/v1/inventories/{}/playbooks", self.set_name)}
}

pub fn get_lol_cosmetics_v1_inventories_by_set_name_playbooks(set_name: String) -> GetLolCosmeticsV1InventoriesBySetNamePlaybooks {
    GetLolCosmeticsV1InventoriesBySetNamePlaybooks{set_name}
}


pub struct PatchLolCosmeticsV1RecentByType {
    pub type_: String,
    pub body: Vec<String>,
}

impl IsApiRequest for PatchLolCosmeticsV1RecentByType {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-cosmetics/v1/recent/{}", self.type_)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_cosmetics_v1_recent_by_type(type_: String, body: Vec<String>) -> PatchLolCosmeticsV1RecentByType {
    PatchLolCosmeticsV1RecentByType{type_, body}
}


pub struct PutLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId {
    pub cosmetic_type: String,
    pub content_id: String,
}

impl IsApiRequest for PutLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-cosmetics/v1/favorites/tft/{}/{}", self.cosmetic_type, self.content_id)}
}

pub fn put_lol_cosmetics_v1_favorites_tft_by_cosmetic_type_by_content_id(cosmetic_type: String, content_id: String) -> PutLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId {
    PutLolCosmeticsV1FavoritesTftByCosmeticTypeByContentId{cosmetic_type, content_id}
}


pub struct PutLolCosmeticsV1FavoritesTftSave {}

impl IsApiRequest for PutLolCosmeticsV1FavoritesTftSave {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/favorites/tft/save".to_string()}
}

pub fn put_lol_cosmetics_v1_favorites_tft_save() -> PutLolCosmeticsV1FavoritesTftSave {
    PutLolCosmeticsV1FavoritesTftSave{}
}


pub struct PutLolCosmeticsV1SelectionCompanion {
    pub body: i32,
}

impl IsApiRequest for PutLolCosmeticsV1SelectionCompanion {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/companion".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_cosmetics_v1_selection_companion(body: i32) -> PutLolCosmeticsV1SelectionCompanion {
    PutLolCosmeticsV1SelectionCompanion{body}
}


pub struct PutLolCosmeticsV1SelectionPlaybook {
    pub body: i32,
}

impl IsApiRequest for PutLolCosmeticsV1SelectionPlaybook {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/playbook".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_cosmetics_v1_selection_playbook(body: i32) -> PutLolCosmeticsV1SelectionPlaybook {
    PutLolCosmeticsV1SelectionPlaybook{body}
}


pub struct PutLolCosmeticsV1SelectionTftDamageSkin {
    pub body: i32,
}

impl IsApiRequest for PutLolCosmeticsV1SelectionTftDamageSkin {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/tft-damage-skin".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_cosmetics_v1_selection_tft_damage_skin(body: i32) -> PutLolCosmeticsV1SelectionTftDamageSkin {
    PutLolCosmeticsV1SelectionTftDamageSkin{body}
}


pub struct PutLolCosmeticsV1SelectionTftMapSkin {
    pub body: i32,
}

impl IsApiRequest for PutLolCosmeticsV1SelectionTftMapSkin {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-cosmetics/v1/selection/tft-map-skin".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_cosmetics_v1_selection_tft_map_skin(body: i32) -> PutLolCosmeticsV1SelectionTftMapSkin {
    PutLolCosmeticsV1SelectionTftMapSkin{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCapOffer {
    pub id: String,
    #[serde(rename = "typeId")]
    pub type_id: String,
    pub label: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "merchantId")]
    pub merchant_id: String,
    pub payload: Vec<Value>,
    pub active: bool,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "createdDate")]
    pub created_date: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsFavoritesViewModel {
    #[serde(rename = "favoriteItems")]
    pub favorite_items: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsGroupViewModel {
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(rename = "numOwned")]
    pub num_owned: u32,
    #[serde(rename = "numAvailable")]
    pub num_available: u32,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: i64,
    pub items: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsGroupedViewModel {
    #[serde(rename = "selectedLoadoutItem")]
    pub selected_loadout_item: LolCosmeticsCosmeticsCompanionViewModel,
    #[serde(rename = "defaultItemId")]
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsCompanionsGroupViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsCompanionViewModel {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "loadoutsIcon")]
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub favorited: bool,
    pub loyalty: bool,
    #[serde(rename = "f2p")]
    pub f_2_p: bool,
    #[serde(rename = "rarityValue")]
    pub rarity_value: u32,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: String,
    #[serde(rename = "isRecentItem")]
    pub is_recent_item: bool,
    pub species: String,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    pub color: String,
    pub level: u32,
    pub upgrades: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
    #[serde(rename = "offerData")]
    pub offer_data: Option<LolCosmeticsCapOffer>,
    #[serde(rename = "starShardsPrice")]
    pub star_shards_price: LolCosmeticsCosmeticsOfferPrice,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsOfferPrice {
    #[serde(rename = "offerId")]
    pub offer_id: String,
    pub price: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTftDamageSkinViewModel {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "loadoutsIcon")]
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub favorited: bool,
    pub loyalty: bool,
    #[serde(rename = "f2p")]
    pub f_2_p: bool,
    #[serde(rename = "rarityValue")]
    pub rarity_value: u32,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: String,
    #[serde(rename = "isRecentItem")]
    pub is_recent_item: bool,
    pub level: u32,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(rename = "groupName")]
    pub group_name: String,
    pub upgrades: Vec<LolCosmeticsCosmeticsTftDamageSkinViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTftMapSkinViewModel {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "loadoutsIcon")]
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub favorited: bool,
    pub loyalty: bool,
    #[serde(rename = "f2p")]
    pub f_2_p: bool,
    #[serde(rename = "rarityValue")]
    pub rarity_value: u32,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: String,
    #[serde(rename = "isRecentItem")]
    pub is_recent_item: bool,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(rename = "groupName")]
    pub group_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTftPlaybookAugment {
    pub name: String,
    pub description: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "effectAmounts")]
    pub effect_amounts: Vec<LolCosmeticsCosmeticsTftPlaybookAugmentEffectAmount>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTftPlaybookAugmentEffectAmount {
    pub name: String,
    pub value: f32,
    #[serde(rename = "formatString")]
    pub format_string: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTftPlaybookViewModel {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "loadoutsIcon")]
    pub loadouts_icon: String,
    pub owned: bool,
    pub selected: bool,
    pub loyalty: bool,
    #[serde(rename = "f2p")]
    pub f_2_p: bool,
    #[serde(rename = "rarityValue")]
    pub rarity_value: u32,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: String,
    #[serde(rename = "isRecentItem")]
    pub is_recent_item: bool,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "iconPathSmall")]
    pub icon_path_small: String,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "earlyAugments")]
    pub early_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "midAugments")]
    pub mid_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "lateAugments")]
    pub late_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "isDisabledInDoubleUp")]
    pub is_disabled_in_double_up: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftDamageSkinFavoritesViewModel {
    #[serde(rename = "favoriteItems")]
    pub favorite_items: Vec<LolCosmeticsCosmeticsTftDamageSkinViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftDamageSkinGroupViewModel {
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(rename = "numOwned")]
    pub num_owned: u32,
    #[serde(rename = "numAvailable")]
    pub num_available: u32,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: i64,
    pub items: Vec<LolCosmeticsCosmeticsTftDamageSkinViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftDamageSkinGroupedViewModel {
    #[serde(rename = "selectedLoadoutItem")]
    pub selected_loadout_item: LolCosmeticsCosmeticsTftDamageSkinViewModel,
    #[serde(rename = "defaultItemId")]
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsTftDamageSkinGroupViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftMapSkinFavoritesViewModel {
    #[serde(rename = "favoriteItems")]
    pub favorite_items: Vec<LolCosmeticsCosmeticsTftMapSkinViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftMapSkinGroupViewModel {
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(rename = "numOwned")]
    pub num_owned: u32,
    #[serde(rename = "numAvailable")]
    pub num_available: u32,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: i64,
    pub items: Vec<LolCosmeticsCosmeticsTftMapSkinViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftMapSkinGroupedViewModel {
    #[serde(rename = "selectedLoadoutItem")]
    pub selected_loadout_item: LolCosmeticsCosmeticsTftMapSkinViewModel,
    #[serde(rename = "defaultItemId")]
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsTftMapSkinGroupViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftPlaybookGroupViewModel {
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(rename = "numOwned")]
    pub num_owned: u32,
    #[serde(rename = "numAvailable")]
    pub num_available: u32,
    pub items: Vec<LolCosmeticsCosmeticsTftPlaybookViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftPlaybookGroupedViewModel {
    #[serde(rename = "selectedLoadoutItem")]
    pub selected_loadout_item: LolCosmeticsCosmeticsTftPlaybookViewModel,
    #[serde(rename = "defaultItemId")]
    pub default_item_id: i32,
    pub groups: Vec<LolCosmeticsTftPlaybookGroupViewModel>,
}


// ENUMS

