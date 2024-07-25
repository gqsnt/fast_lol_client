
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolChampionsV1InventoriesBySummonerIdChampions {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdChampions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionsCollectionsChampion>;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/champions", self.summoner_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_champions(summoner_id: u64) -> GetLolChampionsV1InventoriesBySummonerIdChampions {
    GetLolChampionsV1InventoriesBySummonerIdChampions{summoner_id}
}


pub struct GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionId {
    pub summoner_id: u64,
    pub champion_id: i32,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampionsCollectionsChampion;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/champions/{}", self.summoner_id, self.champion_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id(summoner_id: u64, champion_id: i32) -> GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionId {
    GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionId{summoner_id, champion_id}
}


pub struct GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkins {
    pub summoner_id: u64,
    pub champion_id: i32,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionsCollectionsChampionSkin>;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/champions/{}/skins", self.summoner_id, self.champion_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins(summoner_id: u64, champion_id: i32) -> GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkins {
    GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkins{summoner_id, champion_id}
}


pub struct GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsByChampionSkinId {
    pub summoner_id: u64,
    pub champion_id: i32,
    pub champion_skin_id: i32,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsByChampionSkinId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampionsCollectionsChampionSkin;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/champions/{}/skins/{}", self.summoner_id, self.champion_id, self.champion_skin_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_champion_skin_id(summoner_id: u64, champion_id: i32, champion_skin_id: i32) -> GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsByChampionSkinId {
    GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsByChampionSkinId{summoner_id, champion_id, champion_skin_id}
}


pub struct GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsBySkinIdChromas {
    pub summoner_id: u64,
    pub champion_id: i32,
    pub skin_id: i32,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsBySkinIdChromas {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionsCollectionsChampionChroma>;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/champions/{}/skins/{}/chromas", self.summoner_id, self.champion_id, self.skin_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_skin_id_chromas(summoner_id: u64, champion_id: i32, skin_id: i32) -> GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsBySkinIdChromas {
    GetLolChampionsV1InventoriesBySummonerIdChampionsByChampionIdSkinsBySkinIdChromas{summoner_id, champion_id, skin_id}
}


pub struct GetLolChampionsV1InventoriesBySummonerIdChampionsMinimal {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdChampionsMinimal {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionsCollectionsChampionMinimal>;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/champions-minimal", self.summoner_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_champions_minimal(summoner_id: u64) -> GetLolChampionsV1InventoriesBySummonerIdChampionsMinimal {
    GetLolChampionsV1InventoriesBySummonerIdChampionsMinimal{summoner_id}
}


pub struct GetLolChampionsV1InventoriesBySummonerIdChampionsPlayableCount {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdChampionsPlayableCount {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampionsCollectionsChampionPlayableCounts;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/champions-playable-count", self.summoner_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_champions_playable_count(summoner_id: u64) -> GetLolChampionsV1InventoriesBySummonerIdChampionsPlayableCount {
    GetLolChampionsV1InventoriesBySummonerIdChampionsPlayableCount{summoner_id}
}


pub struct GetLolChampionsV1InventoriesBySummonerIdSkinsMinimal {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolChampionsV1InventoriesBySummonerIdSkinsMinimal {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionsCollectionsChampionSkinMinimal>;
    fn get_url(&self) -> String {format!("/lol-champions/v1/inventories/{}/skins-minimal", self.summoner_id)}
}

pub fn get_lol_champions_v1_inventories_by_summoner_id_skins_minimal(summoner_id: u64) -> GetLolChampionsV1InventoriesBySummonerIdSkinsMinimal {
    GetLolChampionsV1InventoriesBySummonerIdSkinsMinimal{summoner_id}
}


pub struct GetLolChampionsV1OwnedChampionsMinimal {}

impl IsApiRequest for GetLolChampionsV1OwnedChampionsMinimal {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampionsCollectionsChampionMinimal>;
    fn get_url(&self) -> String {"/lol-champions/v1/owned-champions-minimal".to_string()}
}

pub fn get_lol_champions_v1_owned_champions_minimal() -> GetLolChampionsV1OwnedChampionsMinimal {
    GetLolChampionsV1OwnedChampionsMinimal{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsChampionQuestSkinInfo {
    pub name: String,
    #[serde(rename = "descriptionInfo")]
    pub description_info: Vec<LolChampionsQuestSkinDescriptionInfo>,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    #[serde(rename = "collectionCardPath")]
    pub collection_card_path: String,
    #[serde(rename = "collectionDescription")]
    pub collection_description: String,
    pub tiers: Vec<LolChampionsCollectionsChampionQuestSkin>,
    #[serde(rename = "productType")]
    pub product_type: Option<LolChampionsQuestSkinProductType>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampion {
    pub alias: String,
    pub title: String,
    #[serde(rename = "banVoPath")]
    pub ban_vo_path: String,
    #[serde(rename = "chooseVoPath")]
    pub choose_vo_path: String,
    #[serde(rename = "disabledQueues")]
    pub disabled_queues: Vec<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub purchased: u64,
    pub roles: Vec<String>,
    #[serde(rename = "squarePortraitPath")]
    pub square_portrait_path: String,
    #[serde(rename = "stingerSfxPath")]
    pub stinger_sfx_path: String,
    #[serde(rename = "baseLoadScreenPath")]
    pub base_load_screen_path: String,
    #[serde(rename = "baseSplashPath")]
    pub base_splash_path: String,
    pub active: bool,
    #[serde(rename = "botEnabled")]
    pub bot_enabled: bool,
    #[serde(rename = "freeToPlay")]
    pub free_to_play: bool,
    #[serde(rename = "rankedPlayEnabled")]
    pub ranked_play_enabled: bool,
    pub passive: LolChampionsCollectionsChampionSpell,
    pub skins: Vec<LolChampionsCollectionsChampionSkin>,
    pub spells: Vec<LolChampionsCollectionsChampionSpell>,
    #[serde(rename = "tacticalInfo")]
    pub tactical_info: LolChampionsCollectionsChampionTacticalInfo,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionChroma {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "chromaPath")]
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub disabled: bool,
    #[serde(rename = "stillObtainable")]
    pub still_obtainable: bool,
    #[serde(rename = "lastSelected")]
    pub last_selected: bool,
    #[serde(rename = "skinAugments")]
    pub skin_augments: LolChampionsCollectionsChampionSkinAugments,
    pub colors: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionMinimal {
    pub alias: String,
    pub title: String,
    #[serde(rename = "banVoPath")]
    pub ban_vo_path: String,
    #[serde(rename = "chooseVoPath")]
    pub choose_vo_path: String,
    #[serde(rename = "disabledQueues")]
    pub disabled_queues: Vec<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    pub purchased: u64,
    pub roles: Vec<String>,
    #[serde(rename = "squarePortraitPath")]
    pub square_portrait_path: String,
    #[serde(rename = "stingerSfxPath")]
    pub stinger_sfx_path: String,
    #[serde(rename = "baseLoadScreenPath")]
    pub base_load_screen_path: String,
    #[serde(rename = "baseSplashPath")]
    pub base_splash_path: String,
    pub active: bool,
    #[serde(rename = "botEnabled")]
    pub bot_enabled: bool,
    #[serde(rename = "freeToPlay")]
    pub free_to_play: bool,
    #[serde(rename = "rankedPlayEnabled")]
    pub ranked_play_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionPlayableCounts {
    #[serde(rename = "championsOwned")]
    pub champions_owned: u32,
    #[serde(rename = "championsRented")]
    pub champions_rented: u32,
    #[serde(rename = "championsFreeToPlay")]
    pub champions_free_to_play: u32,
    #[serde(rename = "championsLoyaltyReward")]
    pub champions_loyalty_reward: u32,
    #[serde(rename = "championsXboxGPReward")]
    pub champions_xbox_gp_reward: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionQuestSkin {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "chromaPath")]
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    #[serde(rename = "isBase")]
    pub is_base: bool,
    pub disabled: bool,
    #[serde(rename = "stillObtainable")]
    pub still_obtainable: bool,
    #[serde(rename = "lastSelected")]
    pub last_selected: bool,
    #[serde(rename = "skinAugments")]
    pub skin_augments: LolChampionsCollectionsChampionSkinAugments,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    pub stage: u64,
    pub description: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "loadScreenPath")]
    pub load_screen_path: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: String,
    #[serde(rename = "collectionSplashVideoPath")]
    pub collection_splash_video_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkin {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "chromaPath")]
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    #[serde(rename = "isBase")]
    pub is_base: bool,
    pub disabled: bool,
    #[serde(rename = "stillObtainable")]
    pub still_obtainable: bool,
    #[serde(rename = "lastSelected")]
    pub last_selected: bool,
    #[serde(rename = "skinAugments")]
    pub skin_augments: LolChampionsCollectionsChampionSkinAugments,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    pub chromas: Vec<LolChampionsCollectionsChampionChroma>,
    #[serde(rename = "questSkinInfo")]
    pub quest_skin_info: LolChampionsChampionQuestSkinInfo,
    pub emblems: Vec<LolChampionsCollectionsChampionSkinEmblem>,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "loadScreenPath")]
    pub load_screen_path: String,
    #[serde(rename = "rarityGemPath")]
    pub rarity_gem_path: String,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: Option<String>,
    #[serde(rename = "collectionSplashVideoPath")]
    pub collection_splash_video_path: Option<String>,
    #[serde(rename = "skinType")]
    pub skin_type: Option<String>,
    #[serde(rename = "featuresText")]
    pub features_text: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinAugment {
    #[serde(rename = "contentId")]
    pub content_id: String,
    pub overlays: Vec<LolChampionsCollectionsChampionSkinAugmentOverlays>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinAugmentOverlays {
    #[serde(rename = "centeredLCOverlayPath")]
    pub centered_lc_overlay_path: String,
    #[serde(rename = "socialCardLCOverlayPath")]
    pub social_card_lc_overlay_path: String,
    #[serde(rename = "tileLCOverlayPath")]
    pub tile_lc_overlay_path: String,
    #[serde(rename = "uncenteredLCOverlayPath")]
    pub uncentered_lc_overlay_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinAugments {
    pub augments: Vec<LolChampionsCollectionsChampionSkinAugment>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinEmblem {
    pub name: String,
    #[serde(rename = "emblemPath")]
    pub emblem_path: LolChampionsCollectionsChampionSkinEmblemPath,
    pub positions: LolChampionsCollectionsChampionSkinEmblemPosition,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinMinimal {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "chromaPath")]
    pub chroma_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampionsCollectionsOwnership,
    #[serde(rename = "isBase")]
    pub is_base: bool,
    pub disabled: bool,
    #[serde(rename = "stillObtainable")]
    pub still_obtainable: bool,
    #[serde(rename = "lastSelected")]
    pub last_selected: bool,
    #[serde(rename = "skinAugments")]
    pub skin_augments: LolChampionsCollectionsChampionSkinAugments,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSpell {
    pub name: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionTacticalInfo {
    pub style: u32,
    pub difficulty: u32,
    #[serde(rename = "damageType")]
    pub damage_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsOwnership {
    #[serde(rename = "loyaltyReward")]
    pub loyalty_reward: bool,
    #[serde(rename = "xboxGPReward")]
    pub xbox_gp_reward: bool,
    pub owned: bool,
    pub rental: LolChampionsCollectionsRental,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsRental {
    #[serde(rename = "endDate")]
    pub end_date: u64,
    #[serde(rename = "purchaseDate")]
    pub purchase_date: u64,
    #[serde(rename = "winCountRemaining")]
    pub win_count_remaining: i32,
    pub rented: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsQuestSkinDescriptionInfo {
    pub title: String,
    pub description: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChampionsQuestSkinProductType {
    #[default]
    #[serde(rename = "kTieredSkin")]
    KTieredSkin,
    #[serde(rename = "kQuestSkin")]
    KQuestSkin,
}

