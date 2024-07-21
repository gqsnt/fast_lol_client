
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolLootV1LootGrantsById {

    pub id: i64,
}

impl IsApiRequest for DeleteLolLootV1LootGrantsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/loot-grants/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_loot_v_1_loot_grants_by_id(id: i64) -> DeleteLolLootV1LootGrantsById {
    DeleteLolLootV1LootGrantsById {
        id
    }
}


pub struct DeleteLolLootV1PlayerLootByLootIdNewNotification {

    pub loot_id: String,
}

impl IsApiRequest for DeleteLolLootV1PlayerLootByLootIdNewNotification {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/player-loot/{}/new-notification", self.loot_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_loot_v_1_player_loot_by_loot_id_new_notification(loot_id: String) -> DeleteLolLootV1PlayerLootByLootIdNewNotification {
    DeleteLolLootV1PlayerLootByLootIdNewNotification {
        loot_id
    }
}


pub struct GetLolLootV1CurrencyConfiguration {

}

impl IsApiRequest for GetLolLootV1CurrencyConfiguration {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-loot/v1/currency-configuration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_currency_configuration() -> GetLolLootV1CurrencyConfiguration {
    GetLolLootV1CurrencyConfiguration {
        
    }
}


pub struct GetLolLootV1Enabled {

}

impl IsApiRequest for GetLolLootV1Enabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-loot/v1/enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_enabled() -> GetLolLootV1Enabled {
    GetLolLootV1Enabled {
        
    }
}


pub struct GetLolLootV1LootGrants {

}

impl IsApiRequest for GetLolLootV1LootGrants {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootLootGrantNotification>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/loot-grants".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_loot_grants() -> GetLolLootV1LootGrants {
    GetLolLootV1LootGrants {
        
    }
}


pub struct GetLolLootV1LootItems {

}

impl IsApiRequest for GetLolLootV1LootItems {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootLootItem>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/loot-items".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_loot_items() -> GetLolLootV1LootItems {
    GetLolLootV1LootItems {
        
    }
}


pub struct GetLolLootV1LootOddsByRecipeName {

    pub recipe_name: String,
}

impl IsApiRequest for GetLolLootV1LootOddsByRecipeName {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootVerboseLootOddsResponse;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/loot-odds/{}", self.recipe_name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_loot_odds_by_recipe_name(recipe_name: String) -> GetLolLootV1LootOddsByRecipeName {
    GetLolLootV1LootOddsByRecipeName {
        recipe_name
    }
}


pub struct GetLolLootV1LootOddsByRecipeNameVisibility {

    pub recipe_name: String,
}

impl IsApiRequest for GetLolLootV1LootOddsByRecipeNameVisibility {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/loot-odds/{}/visibility", self.recipe_name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_loot_odds_by_recipe_name_visibility(recipe_name: String) -> GetLolLootV1LootOddsByRecipeNameVisibility {
    GetLolLootV1LootOddsByRecipeNameVisibility {
        recipe_name
    }
}


pub struct GetLolLootV1MassDisenchantConfiguration {

}

impl IsApiRequest for GetLolLootV1MassDisenchantConfiguration {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootMassDisenchantClientConfig;

    fn get_url(&self) -> String {
        "/lol-loot/v1/mass-disenchant/configuration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_mass_disenchant_configuration() -> GetLolLootV1MassDisenchantConfiguration {
    GetLolLootV1MassDisenchantConfiguration {
        
    }
}


pub struct GetLolLootV1MassDisenchantRecipes {

}

impl IsApiRequest for GetLolLootV1MassDisenchantRecipes {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LootLcdsRecipeClientDto>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/mass-disenchant-recipes".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_mass_disenchant_recipes() -> GetLolLootV1MassDisenchantRecipes {
    GetLolLootV1MassDisenchantRecipes {
        
    }
}


pub struct GetLolLootV1Milestones {

    pub minimize_response: bool,
}

impl IsApiRequest for GetLolLootV1Milestones {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootLootMilestones>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/milestones".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "minimizeResponse" : self.minimize_response,
        }))
    }
}

pub fn get_lol_loot_v_1_milestones(minimize_response: bool) -> GetLolLootV1Milestones {
    GetLolLootV1Milestones {
        minimize_response
    }
}


pub struct GetLolLootV1MilestonesByLootMilestonesId {

    pub loot_milestones_id: String,
}

impl IsApiRequest for GetLolLootV1MilestonesByLootMilestonesId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootLootMilestones;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/milestones/{}", self.loot_milestones_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_milestones_by_loot_milestones_id(loot_milestones_id: String) -> GetLolLootV1MilestonesByLootMilestonesId {
    GetLolLootV1MilestonesByLootMilestonesId {
        loot_milestones_id
    }
}


pub struct GetLolLootV1MilestonesByLootMilestonesIdClaimProgress {

    pub loot_milestones_id: String,
}

impl IsApiRequest for GetLolLootV1MilestonesByLootMilestonesIdClaimProgress {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootLootMilestonesClaimResponse;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/milestones/{}/claimProgress", self.loot_milestones_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_milestones_by_loot_milestones_id_claim_progress(loot_milestones_id: String) -> GetLolLootV1MilestonesByLootMilestonesIdClaimProgress {
    GetLolLootV1MilestonesByLootMilestonesIdClaimProgress {
        loot_milestones_id
    }
}


pub struct GetLolLootV1MilestonesByLootMilestonesIdCounter {

    pub loot_milestones_id: String,
}

impl IsApiRequest for GetLolLootV1MilestonesByLootMilestonesIdCounter {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootLootMilestonesCounter;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/milestones/{}/counter", self.loot_milestones_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_milestones_by_loot_milestones_id_counter(loot_milestones_id: String) -> GetLolLootV1MilestonesByLootMilestonesIdCounter {
    GetLolLootV1MilestonesByLootMilestonesIdCounter {
        loot_milestones_id
    }
}


pub struct GetLolLootV1MilestonesCounters {

}

impl IsApiRequest for GetLolLootV1MilestonesCounters {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootLootMilestonesCounter>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/milestones/counters".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_milestones_counters() -> GetLolLootV1MilestonesCounters {
    GetLolLootV1MilestonesCounters {
        
    }
}


pub struct GetLolLootV1MilestonesItems {

}

impl IsApiRequest for GetLolLootV1MilestonesItems {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/milestones/items".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_milestones_items() -> GetLolLootV1MilestonesItems {
    GetLolLootV1MilestonesItems {
        
    }
}


pub struct GetLolLootV1PlayerDisplayCategories {

}

impl IsApiRequest for GetLolLootV1PlayerDisplayCategories {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/player-display-categories".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_player_display_categories() -> GetLolLootV1PlayerDisplayCategories {
    GetLolLootV1PlayerDisplayCategories {
        
    }
}


pub struct GetLolLootV1PlayerLoot {

}

impl IsApiRequest for GetLolLootV1PlayerLoot {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootPlayerLoot>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/player-loot".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_player_loot() -> GetLolLootV1PlayerLoot {
    GetLolLootV1PlayerLoot {
        
    }
}


pub struct GetLolLootV1PlayerLootByLootId {

    pub loot_id: String,
}

impl IsApiRequest for GetLolLootV1PlayerLootByLootId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootPlayerLoot;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/player-loot/{}", self.loot_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_player_loot_by_loot_id(loot_id: String) -> GetLolLootV1PlayerLootByLootId {
    GetLolLootV1PlayerLootByLootId {
        loot_id
    }
}


pub struct GetLolLootV1PlayerLootByLootIdContextMenu {

    pub loot_id: String,
}

impl IsApiRequest for GetLolLootV1PlayerLootByLootIdContextMenu {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootContextMenu>;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/player-loot/{}/context-menu", self.loot_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_player_loot_by_loot_id_context_menu(loot_id: String) -> GetLolLootV1PlayerLootByLootIdContextMenu {
    GetLolLootV1PlayerLootByLootIdContextMenu {
        loot_id
    }
}


pub struct PostLolLootV1PlayerLootByLootIdContextMenu {

    pub loot_id: String,
}

impl IsApiRequest for PostLolLootV1PlayerLootByLootIdContextMenu {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLootContextMenu>;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/player-loot/{}/context-menu", self.loot_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loot_v_1_player_loot_by_loot_id_context_menu(loot_id: String) -> PostLolLootV1PlayerLootByLootIdContextMenu {
    PostLolLootV1PlayerLootByLootIdContextMenu {
        loot_id
    }
}


pub struct GetLolLootV1PlayerLootMap {

}

impl IsApiRequest for GetLolLootV1PlayerLootMap {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootPlayerLoot;

    fn get_url(&self) -> String {
        "/lol-loot/v1/player-loot-map".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_player_loot_map() -> GetLolLootV1PlayerLootMap {
    GetLolLootV1PlayerLootMap {
        
    }
}


pub struct GetLolLootV1PlayerLootNotifications {

}

impl IsApiRequest for GetLolLootV1PlayerLootNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootPlayerLootNotification>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/player-loot-notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_player_loot_notifications() -> GetLolLootV1PlayerLootNotifications {
    GetLolLootV1PlayerLootNotifications {
        
    }
}


pub struct GetLolLootV1Ready {

}

impl IsApiRequest for GetLolLootV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-loot/v1/ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_ready() -> GetLolLootV1Ready {
    GetLolLootV1Ready {
        
    }
}


pub struct GetLolLootV1RecipesConfiguration {

}

impl IsApiRequest for GetLolLootV1RecipesConfiguration {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-loot/v1/recipes/configuration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_recipes_configuration() -> GetLolLootV1RecipesConfiguration {
    GetLolLootV1RecipesConfiguration {
        
    }
}


pub struct GetLolLootV1RecipesInitialItemByLootId {

    pub loot_id: String,
}

impl IsApiRequest for GetLolLootV1RecipesInitialItemByLootId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLootRecipeWithMilestones>;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/recipes/initial-item/{}", self.loot_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_1_recipes_initial_item_by_loot_id(loot_id: String) -> GetLolLootV1RecipesInitialItemByLootId {
    GetLolLootV1RecipesInitialItemByLootId {
        loot_id
    }
}


pub struct PostLolLootV1RecipesInitialItemByLootId {

    pub loot_id: String,
}

impl IsApiRequest for PostLolLootV1RecipesInitialItemByLootId {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLootRecipeWithMilestones>;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/recipes/initial-item/{}", self.loot_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loot_v_1_recipes_initial_item_by_loot_id(loot_id: String) -> PostLolLootV1RecipesInitialItemByLootId {
    PostLolLootV1RecipesInitialItemByLootId {
        loot_id
    }
}


pub struct GetLolLootV2PlayerLootMap {

}

impl IsApiRequest for GetLolLootV2PlayerLootMap {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLootPlayerLootMap;

    fn get_url(&self) -> String {
        "/lol-loot/v2/player-loot-map".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loot_v_2_player_loot_map() -> GetLolLootV2PlayerLootMap {
    GetLolLootV2PlayerLootMap {
        
    }
}


pub struct PostLolLootV1CraftMass {

    pub body: Vec<CraftLootDto>,
}

impl IsApiRequest for PostLolLootV1CraftMass {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLootPlayerLootUpdate;

    fn get_url(&self) -> String {
        "/lol-loot/v1/craft/mass".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loot_v_1_craft_mass(body: Vec<CraftLootDto>) -> PostLolLootV1CraftMass {
    PostLolLootV1CraftMass {
        body
    }
}


pub struct PostLolLootV1MilestonesByLootMilestonesIdClaim {

    pub loot_milestones_id: String,
}

impl IsApiRequest for PostLolLootV1MilestonesByLootMilestonesIdClaim {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/milestones/{}/claim", self.loot_milestones_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loot_v_1_milestones_by_loot_milestones_id_claim(loot_milestones_id: String) -> PostLolLootV1MilestonesByLootMilestonesIdClaim {
    PostLolLootV1MilestonesByLootMilestonesIdClaim {
        loot_milestones_id
    }
}


pub struct PostLolLootV1PlayerLootByLootNameRedeem {

    pub loot_name: String,
}

impl IsApiRequest for PostLolLootV1PlayerLootByLootNameRedeem {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLootPlayerLootUpdate;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/player-loot/{}/redeem", self.loot_name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loot_v_1_player_loot_by_loot_name_redeem(loot_name: String) -> PostLolLootV1PlayerLootByLootNameRedeem {
    PostLolLootV1PlayerLootByLootNameRedeem {
        loot_name
    }
}


pub struct PostLolLootV1PlayerLootNotificationsByIdAcknowledge {

    pub id: String,
}

impl IsApiRequest for PostLolLootV1PlayerLootNotificationsByIdAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = String;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/player-loot-notifications/{}/acknowledge", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loot_v_1_player_loot_notifications_by_id_acknowledge(id: String) -> PostLolLootV1PlayerLootNotificationsByIdAcknowledge {
    PostLolLootV1PlayerLootNotificationsByIdAcknowledge {
        id
    }
}


pub struct PostLolLootV1RecipesByRecipeNameCraft {

    pub recipe_name: String,
    pub player_loot_list: Vec<String>,
    pub repeat: Option<i32>,
}

impl IsApiRequest for PostLolLootV1RecipesByRecipeNameCraft {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLootPlayerLootUpdate;

    fn get_url(&self) -> String {
        format!("/lol-loot/v1/recipes/{}/craft", self.recipe_name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "playerLootList" : self.player_loot_list,
            "repeat" : self.repeat,
        }))
    }
}

pub fn post_lol_loot_v_1_recipes_by_recipe_name_craft(recipe_name: String, player_loot_list: Vec<String>, repeat: Option<i32>) -> PostLolLootV1RecipesByRecipeNameCraft {
    PostLolLootV1RecipesByRecipeNameCraft {
        recipe_name, player_loot_list, repeat
    }
}


pub struct PostLolLootV1Refresh {

    pub body: bool,
}

impl IsApiRequest for PostLolLootV1Refresh {
    const METHOD: Method = Method::POST;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-loot/v1/refresh".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loot_v_1_refresh(body: bool) -> PostLolLootV1Refresh {
    PostLolLootV1Refresh {
        body
    }
}


pub struct PutLolLootV1LootOddsEvaluateQuery {

    pub body: QueryEvaluationRequestDto,
}

impl IsApiRequest for PutLolLootV1LootOddsEvaluateQuery {
    const METHOD: Method = Method::PUT;
    type ReturnType = Vec<LolLootQueryEvaluatedLootItem>;

    fn get_url(&self) -> String {
        "/lol-loot/v1/loot-odds/evaluateQuery".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_loot_v_1_loot_odds_evaluate_query(body: QueryEvaluationRequestDto) -> PutLolLootV1LootOddsEvaluateQuery {
    PutLolLootV1LootOddsEvaluateQuery {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeOutput {
    pub loot_name: String,
    pub quantity: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootMap {
    pub version: i64,
    pub player_loot: LolLootPlayerLoot,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootItem {
    pub loot_name: String,
    pub asset: String,
    pub type_: String,
    pub rarity: String,
    pub value: i32,
    pub store_item_id: i32,
    pub upgrade_loot_name: String,
    pub expiry_time: i64,
    pub tags: String,
    pub display_categories: String,
    pub rental_seconds: i64,
    pub rental_games: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMassDisenchantClientConfig {
    pub max_loot_items_size_mass_craft: i16,
    pub enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestones {
    pub id: String,
    pub progression_config_id: String,
    pub active: bool,
    pub start_date: String,
    pub end_date: String,
    pub store_group_title: String,
    pub repeat: LolLootLootMilestoneRepeat,
    pub loot_items: Vec<String>,
    pub recipes: Vec<String>,
    pub milestones: Vec<LolLootLootMilestone>,
    pub error_caching_milestone_set: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeSlot {
    pub slot_number: i32,
    pub loot_ids: Vec<String>,
    pub tags: String,
    pub quantity: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeWithMilestones {
    pub recipe_name: String,
    pub type_: String,
    pub description: String,
    pub context_menu_text: String,
    pub requirement_text: String,
    pub image_path: String,
    pub intro_video_path: String,
    pub loop_video_path: String,
    pub outro_video_path: String,
    pub display_categories: String,
    pub crafter_name: String,
    pub slots: Vec<LolLootRecipeSlot>,
    pub outputs: Vec<LolLootRecipeOutput>,
    pub metadata: LolLootRecipeMetadata,
    pub single_open: bool,
    pub has_visible_loot_odds: bool,
    pub loot_milestone_ids: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootDelta {
    pub delta_count: i32,
    pub player_loot: LolLootPlayerLoot,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryEvaluationRequestDto {
    pub query: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesClaimResponse {
    pub loot_milestone_set_id: String,
    pub claimed_milestones: Vec<String>,
    pub status: LolLootLootMilestoneClaimStatus,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLoot {
    pub loot_name: String,
    pub loot_id: String,
    pub ref_id: String,
    pub localized_name: String,
    pub localized_description: String,
    pub item_desc: String,
    pub display_categories: String,
    pub rarity: String,
    pub tags: String,
    pub type_: String,
    pub asset: String,
    pub tile_path: String,
    pub splash_path: String,
    pub shadow_path: String,
    pub upgrade_loot_name: String,
    pub upgrade_essence_name: String,
    pub disenchant_loot_name: String,
    pub localized_recipe_title: String,
    pub localized_recipe_subtitle: String,
    pub item_status: LolLootItemOwnershipStatus,
    pub parent_item_status: LolLootItemOwnershipStatus,
    pub redeemable_status: LolLootRedeemableStatus,
    pub count: i32,
    pub rental_games: i32,
    pub store_item_id: i32,
    pub parent_store_item_id: i32,
    pub value: i32,
    pub upgrade_essence_value: i32,
    pub disenchant_value: i32,
    pub disenchant_recipe_name: String,
    pub expiry_time: i64,
    pub rental_seconds: i64,
    pub is_new: bool,
    pub is_rental: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestoneRepeat {
    pub repeat_count: i32,
    pub repeat_scope: i32,
    pub multiplier: f32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootOddsResponse {
    pub loot_id: String,
    pub parent_id: String,
    pub drop_rate: f64,
    pub quantity: i32,
    pub label: String,
    pub query: String,
    pub loot_order: i32,
    pub children: Vec<LolLootLootOddsResponse>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CraftLootDto {
    pub recipe_name: String,
    pub loot_names: Vec<String>,
    pub repeat: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeMetadata {
    pub guaranteed_descriptions: Vec<LootLcdsLootDescriptionDto>,
    pub bonus_descriptions: Vec<LootLcdsLootDescriptionDto>,
    pub tooltips_disabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeSlotClientDto {
    pub slot_number: i32,
    pub query: String,
    pub quantity_expression: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootQueryEvaluatedLootItem {
    pub loot_name: String,
    pub localized_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootContextMenu {
    pub name: String,
    pub action_type: String,
    pub recipe_description: String,
    pub recipe_context_menu_action: String,
    pub enabled: bool,
    pub essence_type: String,
    pub essence_quantity: i32,
    pub required_tokens: String,
    pub required_others: String,
    pub required_others_name: String,
    pub required_others_count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeMetadata {
    pub guaranteed_descriptions: Vec<LolLootLootDescription>,
    pub bonus_descriptions: Vec<LolLootLootDescription>,
    pub tooltips_disabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootGrantNotification {
    pub id: i64,
    pub game_id: u64,
    pub player_id: u64,
    pub champion_id: i32,
    pub player_grade: String,
    pub loot_name: String,
    pub message_key: String,
    pub msg_id: String,
    pub account_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootDescription {
    pub loot_name: String,
    pub localized_description: String,
    pub localized_description_long: String,
    pub image_path: String,
    pub child_loot_table_names: Vec<String>,
    pub children_descriptions: Vec<LolLootLootDescription>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestone {
    pub id: String,
    pub threshold: u64,
    pub rewards: Vec<LolLootLootMilestoneReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootNotification {
    pub id: String,
    pub count: i32,
    pub acknowledged: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootUpdate {
    pub added: Vec<LolLootPlayerLootDelta>,
    pub removed: Vec<LolLootPlayerLootDelta>,
    pub redeemed: Vec<LolLootPlayerLootDelta>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesCounter {
    pub loot_milestones_id: String,
    pub counter_value: i64,
    pub completed_loops: i64,
    pub ready_to_claim_milestones: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeOutputDto {
    pub loot_name: String,
    pub quantity_expression: String,
    pub probability: f64,
    pub allow_duplicates: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestoneReward {
    pub reward_group_id: String,
    pub reward_type: String,
    pub item_instance_id: String,
    pub inventory_type: String,
    pub item_id: i32,
    pub quantity: i32,
    pub loot_item: Option<LolLootPlayerLoot>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLootVerboseLootOddsResponse {
    pub recipe_name: String,
    pub chance_to_contain: Vec<LolLootLootOddsResponse>,
    pub guaranteed_to_contain: Vec<LolLootLootOddsResponse>,
    pub loot_item: LolLootPlayerLoot,
    pub has_pity_rules: bool,
    pub checks_ownership: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeClientDto {
    pub recipe_name: String,
    pub type_: String,
    pub display_categories: String,
    pub crafter_name: String,
    pub slots: Vec<LootLcdsRecipeSlotClientDto>,
    pub outputs: Vec<LootLcdsRecipeOutputDto>,
    pub metadata: LootLcdsRecipeMetadata,
    pub single_open: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsLootDescriptionDto {
    pub loot_name: String,
    pub child_loot_table_names: Vec<String>,
    pub localization_map: HashMap<String, String>,
    pub localization_long_description_map: HashMap<String, String>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLootItemOwnershipStatus {
    #[default]
    #[serde(rename = "OWNED")]
    Owned,
    #[serde(rename = "RENTAL")]
    Rental,
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLootRedeemableStatus {
    #[default]
    #[serde(rename = "SKIN_NOT_OWNED")]
    SkinNotOwned,
    #[serde(rename = "CHAMPION_NOT_OWNED")]
    ChampionNotOwned,
    #[serde(rename = "ALREADY_RENTED")]
    AlreadyRented,
    #[serde(rename = "ALREADY_OWNED")]
    AlreadyOwned,
    #[serde(rename = "NOT_REDEEMABLE_RENTAL")]
    NotRedeemableRental,
    #[serde(rename = "NOT_REDEEMABLE")]
    NotRedeemable,
    #[serde(rename = "REDEEMABLE_RENTAL")]
    RedeemableRental,
    #[serde(rename = "REDEEMABLE")]
    Redeemable,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLootLootMilestoneClaimStatus {
    #[default]
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
}

