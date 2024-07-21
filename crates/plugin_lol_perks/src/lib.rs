
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolPerksV1Pages {

}

impl IsApiRequest for DeleteLolPerksV1Pages {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/pages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_perks_v_1_pages() -> DeleteLolPerksV1Pages {
    DeleteLolPerksV1Pages {
        
    }
}


pub struct DeleteLolPerksV1PagesById {

    pub id: i32,
}

impl IsApiRequest for DeleteLolPerksV1PagesById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/pages/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_perks_v_1_pages_by_id(id: i32) -> DeleteLolPerksV1PagesById {
    DeleteLolPerksV1PagesById {
        id
    }
}


pub struct DeleteLolPerksV1PagesByIdAutoModifiedSelections {

    pub id: i32,
}

impl IsApiRequest for DeleteLolPerksV1PagesByIdAutoModifiedSelections {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/pages/{}/auto-modified-selections", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_perks_v_1_pages_by_id_auto_modified_selections(id: i32) -> DeleteLolPerksV1PagesByIdAutoModifiedSelections {
    DeleteLolPerksV1PagesByIdAutoModifiedSelections {
        id
    }
}


pub struct GetLolPerksV1Currentpage {

}

impl IsApiRequest for GetLolPerksV1Currentpage {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPerksPerkPageResource;

    fn get_url(&self) -> String {
        "/lol-perks/v1/currentpage".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_currentpage() -> GetLolPerksV1Currentpage {
    GetLolPerksV1Currentpage {
        
    }
}


pub struct GetLolPerksV1Inventory {

}

impl IsApiRequest for GetLolPerksV1Inventory {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPerksPlayerInventory;

    fn get_url(&self) -> String {
        "/lol-perks/v1/inventory".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_inventory() -> GetLolPerksV1Inventory {
    GetLolPerksV1Inventory {
        
    }
}


pub struct GetLolPerksV1Pages {

}

impl IsApiRequest for GetLolPerksV1Pages {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPerksPerkPageResource>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/pages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_pages() -> GetLolPerksV1Pages {
    GetLolPerksV1Pages {
        
    }
}


pub struct GetLolPerksV1PagesById {

    pub id: i32,
}

impl IsApiRequest for GetLolPerksV1PagesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPerksPerkPageResource;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/pages/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_pages_by_id(id: i32) -> GetLolPerksV1PagesById {
    GetLolPerksV1PagesById {
        id
    }
}


pub struct GetLolPerksV1Perks {

}

impl IsApiRequest for GetLolPerksV1Perks {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPerksPerkUiPerk>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/perks".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_perks() -> GetLolPerksV1Perks {
    GetLolPerksV1Perks {
        
    }
}


pub struct GetLolPerksV1PerksDisabled {

}

impl IsApiRequest for GetLolPerksV1PerksDisabled {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/perks/disabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_perks_disabled() -> GetLolPerksV1PerksDisabled {
    GetLolPerksV1PerksDisabled {
        
    }
}


pub struct GetLolPerksV1PerksGameplayUpdated {

}

impl IsApiRequest for GetLolPerksV1PerksGameplayUpdated {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/perks/gameplay-updated".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_perks_gameplay_updated() -> GetLolPerksV1PerksGameplayUpdated {
    GetLolPerksV1PerksGameplayUpdated {
        
    }
}


pub struct GetLolPerksV1QuickPlaySelectionsChampionByChampionIdPositionByPosition {

    pub champion_id: i32,
    pub position: String,
}

impl IsApiRequest for GetLolPerksV1QuickPlaySelectionsChampionByChampionIdPositionByPosition {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/quick-play-selections/champion/{}/position/{}", self.champion_id, self.position)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_quick_play_selections_champion_by_champion_id_position_by_position(champion_id: i32, position: String) -> GetLolPerksV1QuickPlaySelectionsChampionByChampionIdPositionByPosition {
    GetLolPerksV1QuickPlaySelectionsChampionByChampionIdPositionByPosition {
        champion_id, position
    }
}


pub struct GetLolPerksV1RecommendedChampionPositions {

}

impl IsApiRequest for GetLolPerksV1RecommendedChampionPositions {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPerksRecommendedPositionsMapResource;

    fn get_url(&self) -> String {
        "/lol-perks/v1/recommended-champion-positions".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_recommended_champion_positions() -> GetLolPerksV1RecommendedChampionPositions {
    GetLolPerksV1RecommendedChampionPositions {
        
    }
}


pub struct GetLolPerksV1RecommendedPagesChampionByChampionIdPositionByPositionMapByMapId {

    pub champion_id: i32,
    pub position: String,
    pub map_id: i32,
}

impl IsApiRequest for GetLolPerksV1RecommendedPagesChampionByChampionIdPositionByPositionMapByMapId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPerksPerkUiRecommendedPage>;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/recommended-pages/champion/{}/position/{}/map/{}", self.champion_id, self.position, self.map_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_recommended_pages_champion_by_champion_id_position_by_position_map_by_map_id(champion_id: i32, position: String, map_id: i32) -> GetLolPerksV1RecommendedPagesChampionByChampionIdPositionByPositionMapByMapId {
    GetLolPerksV1RecommendedPagesChampionByChampionIdPositionByPositionMapByMapId {
        champion_id, position, map_id
    }
}


pub struct GetLolPerksV1RecommendedPagesPositionChampionByChampionId {

    pub champion_id: i32,
}

impl IsApiRequest for GetLolPerksV1RecommendedPagesPositionChampionByChampionId {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/recommended-pages-position/champion/{}", self.champion_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_recommended_pages_position_champion_by_champion_id(champion_id: i32) -> GetLolPerksV1RecommendedPagesPositionChampionByChampionId {
    GetLolPerksV1RecommendedPagesPositionChampionByChampionId {
        champion_id
    }
}


pub struct GetLolPerksV1RuneRecommenderAutoSelect {

}

impl IsApiRequest for GetLolPerksV1RuneRecommenderAutoSelect {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-perks/v1/rune-recommender-auto-select".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_rune_recommender_auto_select() -> GetLolPerksV1RuneRecommenderAutoSelect {
    GetLolPerksV1RuneRecommenderAutoSelect {
        
    }
}


pub struct GetLolPerksV1Settings {

}

impl IsApiRequest for GetLolPerksV1Settings {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPerksUiSettings;

    fn get_url(&self) -> String {
        "/lol-perks/v1/settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_settings() -> GetLolPerksV1Settings {
    GetLolPerksV1Settings {
        
    }
}


pub struct GetLolPerksV1ShowAutoModifiedPagesNotification {

}

impl IsApiRequest for GetLolPerksV1ShowAutoModifiedPagesNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-perks/v1/show-auto-modified-pages-notification".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_show_auto_modified_pages_notification() -> GetLolPerksV1ShowAutoModifiedPagesNotification {
    GetLolPerksV1ShowAutoModifiedPagesNotification {
        
    }
}


pub struct GetLolPerksV1Styles {

}

impl IsApiRequest for GetLolPerksV1Styles {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPerksPerkUiStyle>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/styles".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_perks_v_1_styles() -> GetLolPerksV1Styles {
    GetLolPerksV1Styles {
        
    }
}


pub struct PostLolPerksV1Pages {

    pub body: LolPerksPerkPageResource,
}

impl IsApiRequest for PostLolPerksV1Pages {
    const METHOD: Method = Method::POST;
    type ReturnType = LolPerksPerkPageResource;

    fn get_url(&self) -> String {
        "/lol-perks/v1/pages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_perks_v_1_pages(body: LolPerksPerkPageResource) -> PostLolPerksV1Pages {
    PostLolPerksV1Pages {
        body
    }
}


pub struct PostLolPerksV1RecommendedPagesPositionChampionByChampionIdPositionByPosition {

    pub champion_id: i32,
    pub position: String,
}

impl IsApiRequest for PostLolPerksV1RecommendedPagesPositionChampionByChampionIdPositionByPosition {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/recommended-pages-position/champion/{}/position/{}", self.champion_id, self.position)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_perks_v_1_recommended_pages_position_champion_by_champion_id_position_by_position(champion_id: i32, position: String) -> PostLolPerksV1RecommendedPagesPositionChampionByChampionIdPositionByPosition {
    PostLolPerksV1RecommendedPagesPositionChampionByChampionIdPositionByPosition {
        champion_id, position
    }
}


pub struct PostLolPerksV1RuneRecommenderAutoSelect {

}

impl IsApiRequest for PostLolPerksV1RuneRecommenderAutoSelect {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/rune-recommender-auto-select".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_perks_v_1_rune_recommender_auto_select() -> PostLolPerksV1RuneRecommenderAutoSelect {
    PostLolPerksV1RuneRecommenderAutoSelect {
        
    }
}


pub struct PostLolPerksV1ShowAutoModifiedPagesNotification {

}

impl IsApiRequest for PostLolPerksV1ShowAutoModifiedPagesNotification {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/show-auto-modified-pages-notification".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_perks_v_1_show_auto_modified_pages_notification() -> PostLolPerksV1ShowAutoModifiedPagesNotification {
    PostLolPerksV1ShowAutoModifiedPagesNotification {
        
    }
}


pub struct PostLolPerksV1UpdatePageOrder {

    pub body: LolPerksUpdatePageOrderRequest,
}

impl IsApiRequest for PostLolPerksV1UpdatePageOrder {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/update-page-order".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_perks_v_1_update_page_order(body: LolPerksUpdatePageOrderRequest) -> PostLolPerksV1UpdatePageOrder {
    PostLolPerksV1UpdatePageOrder {
        body
    }
}


pub struct PutLolPerksV1Currentpage {

    pub body: i32,
}

impl IsApiRequest for PutLolPerksV1Currentpage {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/currentpage".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_perks_v_1_currentpage(body: i32) -> PutLolPerksV1Currentpage {
    PutLolPerksV1Currentpage {
        body
    }
}


pub struct PutLolPerksV1PagesById {

    pub id: i32,
    pub body: LolPerksPerkPageResource,
}

impl IsApiRequest for PutLolPerksV1PagesById {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolPerksPerkPageResource;

    fn get_url(&self) -> String {
        format!("/lol-perks/v1/pages/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_perks_v_1_pages_by_id(id: i32, body: LolPerksPerkPageResource) -> PutLolPerksV1PagesById {
    PutLolPerksV1PagesById {
        id, body
    }
}


pub struct PutLolPerksV1PagesValidate {

    pub body: LolPerksValidatePageNameData,
}

impl IsApiRequest for PutLolPerksV1PagesValidate {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolPerksValidateItemSetNameResponse;

    fn get_url(&self) -> String {
        "/lol-perks/v1/pages/validate".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_perks_v_1_pages_validate(body: LolPerksValidatePageNameData) -> PutLolPerksV1PagesValidate {
    PutLolPerksV1PagesValidate {
        body
    }
}


pub struct PutLolPerksV1PerksAckGameplayUpdated {

    pub body: Vec<i32>,
}

impl IsApiRequest for PutLolPerksV1PerksAckGameplayUpdated {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/perks/ack-gameplay-updated".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_perks_v_1_perks_ack_gameplay_updated(body: Vec<i32>) -> PutLolPerksV1PerksAckGameplayUpdated {
    PutLolPerksV1PerksAckGameplayUpdated {
        body
    }
}


pub struct PutLolPerksV1Settings {

    pub body: LolPerksUiSettings,
}

impl IsApiRequest for PutLolPerksV1Settings {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-perks/v1/settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_perks_v_1_settings(body: LolPerksUiSettings) -> PutLolPerksV1Settings {
    PutLolPerksV1Settings {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksNamecheckResponse {
    pub errors: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkPageResource {
    pub current: bool,
    pub id: i32,
    pub is_active: bool,
    pub is_valid: bool,
    pub is_editable: bool,
    pub is_deletable: bool,
    pub is_temporary: bool,
    pub name: String,
    pub order: i32,
    pub primary_style_id: i32,
    pub selected_perk_ids: Vec<i32>,
    pub sub_style_id: i32,
    pub auto_modified_selections: Vec<u32>,
    pub last_modified: u64,
    pub rune_recommendation_id: String,
    pub recommendation_index: i32,
    pub is_recommendation_override: bool,
    pub recommendation_champion_id: i32,
    pub quick_play_champion_ids: Vec<i32>,
    pub primary_style_name: String,
    pub secondary_style_name: String,
    pub primary_style_icon_path: String,
    pub secondary_style_icon_path: String,
    pub tooltip_bg_path: String,
    pub page_keystone: LolPerksUiPerkMinimal,
    pub ui_perks: Vec<LolPerksUiPerkMinimal>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkSubStyleBonusResource {
    pub perk_id: i32,
    pub style_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiPerk {
    pub icon_path: String,
    pub id: i32,
    pub style_id: i32,
    pub style_id_name: String,
    pub long_desc: String,
    pub name: String,
    pub short_desc: String,
    pub tooltip: String,
    pub recommendation_descriptor: String,
    pub slot_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiRecommendedPage {
    pub position: String,
    pub is_default_position: bool,
    pub keystone: LolPerksPerkUiPerk,
    pub perks: Vec<LolPerksPerkUiPerk>,
    pub primary_perk_style_id: i32,
    pub secondary_perk_style_id: i32,
    pub primary_recommendation_attribute: String,
    pub secondary_recommendation_attribute: String,
    pub summoner_spell_ids: Vec<i32>,
    pub recommendation_id: String,
    pub is_recommendation_override: bool,
    pub recommendation_champion_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiSlot {
    pub perks: Vec<i32>,
    pub type_: String,
    pub slot_label: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiStyle {
    pub allowed_sub_styles: Vec<i32>,
    pub icon_path: String,
    pub asset_map: HashMap<String, String>,
    pub id: i32,
    pub name: String,
    pub slots: Vec<LolPerksPerkUiSlot>,
    pub sub_style_bonus: Vec<LolPerksPerkSubStyleBonusResource>,
    pub tooltip: String,
    pub default_sub_style: i32,
    pub default_perks: Vec<i32>,
    pub default_page_name: String,
    pub id_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPlayerInventory {
    pub owned_page_count: u32,
    pub custom_page_count: u32,
    pub can_add_custom_page: bool,
    pub is_custom_page_creation_unlocked: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksRecommendedPositionsMapResource {
    pub recommended_positions: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUiPerkMinimal {
    pub id: i32,
    pub style_id: i32,
    pub name: String,
    pub slot_type: String,
    pub icon_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUiSettings {
    pub show_long_descriptions: bool,
    pub grid_mode_enabled: bool,
    pub show_preset_pages: bool,
    pub gameplay_patch_version_seen: String,
    pub gameplay_updated_perks_seen: Vec<i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUpdatePageOrderRequest {
    pub target_page_id: i32,
    pub destination_page_id: i32,
    pub offset: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksValidateItemSetNameResponse {
    pub success: bool,
    pub name_check_response: LolPerksNamecheckResponse,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksValidatePageNameData {
    pub id: i32,
    pub name: String,
}


// ENUMS

