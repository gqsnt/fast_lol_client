
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolChampSelectV1AllGridChampions {}

impl IsApiRequest for GetLolChampSelectV1AllGridChampions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampSelectChampGridChampion>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/all-grid-champions".to_string()}
}

pub fn get_lol_champ_select_v1_all_grid_champions() -> GetLolChampSelectV1AllGridChampions {
    GetLolChampSelectV1AllGridChampions{}
}


pub struct GetLolChampSelectV1BannableChampionIds {}

impl IsApiRequest for GetLolChampSelectV1BannableChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/bannable-champion-ids".to_string()}
}

pub fn get_lol_champ_select_v1_bannable_champion_ids() -> GetLolChampSelectV1BannableChampionIds {
    GetLolChampSelectV1BannableChampionIds{}
}


pub struct GetLolChampSelectV1CurrentChampion {}

impl IsApiRequest for GetLolChampSelectV1CurrentChampion {
    const METHOD: Method = Method::GET;
    type ReturnType = i32;
    fn get_url(&self) -> String {"/lol-champ-select/v1/current-champion".to_string()}
}

pub fn get_lol_champ_select_v1_current_champion() -> GetLolChampSelectV1CurrentChampion {
    GetLolChampSelectV1CurrentChampion{}
}


pub struct GetLolChampSelectV1DisabledChampionIds {}

impl IsApiRequest for GetLolChampSelectV1DisabledChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/disabled-champion-ids".to_string()}
}

pub fn get_lol_champ_select_v1_disabled_champion_ids() -> GetLolChampSelectV1DisabledChampionIds {
    GetLolChampSelectV1DisabledChampionIds{}
}


pub struct GetLolChampSelectV1GridChampionsByChampionId {
    pub champion_id: i32,
}

impl IsApiRequest for GetLolChampSelectV1GridChampionsByChampionId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampGridChampion;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/grid-champions/{}", self.champion_id)}
}

pub fn get_lol_champ_select_v1_grid_champions_by_champion_id(champion_id: i32) -> GetLolChampSelectV1GridChampionsByChampionId {
    GetLolChampSelectV1GridChampionsByChampionId{champion_id}
}


pub struct GetLolChampSelectV1MutedPlayers {}

impl IsApiRequest for GetLolChampSelectV1MutedPlayers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampSelectMutedPlayerInfo>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/muted-players".to_string()}
}

pub fn get_lol_champ_select_v1_muted_players() -> GetLolChampSelectV1MutedPlayers {
    GetLolChampSelectV1MutedPlayers{}
}


pub struct GetLolChampSelectV1OngoingSwap {}

impl IsApiRequest for GetLolChampSelectV1OngoingSwap {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectSwapNotification;
    fn get_url(&self) -> String {"/lol-champ-select/v1/ongoing-swap".to_string()}
}

pub fn get_lol_champ_select_v1_ongoing_swap() -> GetLolChampSelectV1OngoingSwap {
    GetLolChampSelectV1OngoingSwap{}
}


pub struct GetLolChampSelectV1OngoingTrade {}

impl IsApiRequest for GetLolChampSelectV1OngoingTrade {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectTradeNotification;
    fn get_url(&self) -> String {"/lol-champ-select/v1/ongoing-trade".to_string()}
}

pub fn get_lol_champ_select_v1_ongoing_trade() -> GetLolChampSelectV1OngoingTrade {
    GetLolChampSelectV1OngoingTrade{}
}


pub struct GetLolChampSelectV1PickableChampionIds {}

impl IsApiRequest for GetLolChampSelectV1PickableChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/pickable-champion-ids".to_string()}
}

pub fn get_lol_champ_select_v1_pickable_champion_ids() -> GetLolChampSelectV1PickableChampionIds {
    GetLolChampSelectV1PickableChampionIds{}
}


pub struct GetLolChampSelectV1PickableSkinIds {}

impl IsApiRequest for GetLolChampSelectV1PickableSkinIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/pickable-skin-ids".to_string()}
}

pub fn get_lol_champ_select_v1_pickable_skin_ids() -> GetLolChampSelectV1PickableSkinIds {
    GetLolChampSelectV1PickableSkinIds{}
}


pub struct GetLolChampSelectV1PinDropNotification {}

impl IsApiRequest for GetLolChampSelectV1PinDropNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectPinDropNotification;
    fn get_url(&self) -> String {"/lol-champ-select/v1/pin-drop-notification".to_string()}
}

pub fn get_lol_champ_select_v1_pin_drop_notification() -> GetLolChampSelectV1PinDropNotification {
    GetLolChampSelectV1PinDropNotification{}
}


pub struct GetLolChampSelectV1Session {}

impl IsApiRequest for GetLolChampSelectV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectSession;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session".to_string()}
}

pub fn get_lol_champ_select_v1_session() -> GetLolChampSelectV1Session {
    GetLolChampSelectV1Session{}
}


pub struct GetLolChampSelectV1SessionMySelection {}

impl IsApiRequest for GetLolChampSelectV1SessionMySelection {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectPlayerSelection;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session/my-selection".to_string()}
}

pub fn get_lol_champ_select_v1_session_my_selection() -> GetLolChampSelectV1SessionMySelection {
    GetLolChampSelectV1SessionMySelection{}
}


pub struct GetLolChampSelectV1SessionSwaps {}

impl IsApiRequest for GetLolChampSelectV1SessionSwaps {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampSelectChampSelectSwapContract>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session/swaps".to_string()}
}

pub fn get_lol_champ_select_v1_session_swaps() -> GetLolChampSelectV1SessionSwaps {
    GetLolChampSelectV1SessionSwaps{}
}


pub struct GetLolChampSelectV1SessionSwapsById {
    pub id: i64,
}

impl IsApiRequest for GetLolChampSelectV1SessionSwapsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectSwapContract;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/swaps/{}", self.id)}
}

pub fn get_lol_champ_select_v1_session_swaps_by_id(id: i64) -> GetLolChampSelectV1SessionSwapsById {
    GetLolChampSelectV1SessionSwapsById{id}
}


pub struct GetLolChampSelectV1SessionTimer {}

impl IsApiRequest for GetLolChampSelectV1SessionTimer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectTimer;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session/timer".to_string()}
}

pub fn get_lol_champ_select_v1_session_timer() -> GetLolChampSelectV1SessionTimer {
    GetLolChampSelectV1SessionTimer{}
}


pub struct GetLolChampSelectV1SessionTrades {}

impl IsApiRequest for GetLolChampSelectV1SessionTrades {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampSelectChampSelectTradeContract>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session/trades".to_string()}
}

pub fn get_lol_champ_select_v1_session_trades() -> GetLolChampSelectV1SessionTrades {
    GetLolChampSelectV1SessionTrades{}
}


pub struct GetLolChampSelectV1SessionTradesById {
    pub id: i64,
}

impl IsApiRequest for GetLolChampSelectV1SessionTradesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectTradeContract;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/trades/{}", self.id)}
}

pub fn get_lol_champ_select_v1_session_trades_by_id(id: i64) -> GetLolChampSelectV1SessionTradesById {
    GetLolChampSelectV1SessionTradesById{id}
}


pub struct GetLolChampSelectV1SfxNotifications {}

impl IsApiRequest for GetLolChampSelectV1SfxNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampSelectSfxNotification>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/sfx-notifications".to_string()}
}

pub fn get_lol_champ_select_v1_sfx_notifications() -> GetLolChampSelectV1SfxNotifications {
    GetLolChampSelectV1SfxNotifications{}
}


pub struct GetLolChampSelectV1SkinCarouselSkins {}

impl IsApiRequest for GetLolChampSelectV1SkinCarouselSkins {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampSelectSkinSelectorSkin>;
    fn get_url(&self) -> String {"/lol-champ-select/v1/skin-carousel-skins".to_string()}
}

pub fn get_lol_champ_select_v1_skin_carousel_skins() -> GetLolChampSelectV1SkinCarouselSkins {
    GetLolChampSelectV1SkinCarouselSkins{}
}


pub struct GetLolChampSelectV1SkinSelectorInfo {}

impl IsApiRequest for GetLolChampSelectV1SkinSelectorInfo {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectSkinSelectorInfo;
    fn get_url(&self) -> String {"/lol-champ-select/v1/skin-selector-info".to_string()}
}

pub fn get_lol_champ_select_v1_skin_selector_info() -> GetLolChampSelectV1SkinSelectorInfo {
    GetLolChampSelectV1SkinSelectorInfo{}
}


pub struct GetLolChampSelectV1SummonersBySlotId {
    pub slot_id: u64,
}

impl IsApiRequest for GetLolChampSelectV1SummonersBySlotId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectChampSelectSummoner;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/summoners/{}", self.slot_id)}
}

pub fn get_lol_champ_select_v1_summoners_by_slot_id(slot_id: u64) -> GetLolChampSelectV1SummonersBySlotId {
    GetLolChampSelectV1SummonersBySlotId{slot_id}
}


pub struct GetLolChampSelectV1TeamBoost {}

impl IsApiRequest for GetLolChampSelectV1TeamBoost {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectTeamBoost;
    fn get_url(&self) -> String {"/lol-champ-select/v1/team-boost".to_string()}
}

pub fn get_lol_champ_select_v1_team_boost() -> GetLolChampSelectV1TeamBoost {
    GetLolChampSelectV1TeamBoost{}
}


pub struct PatchLolChampSelectV1SessionActionsById {
    pub id: i64,
    pub body: LolChampSelectChampSelectAction,
}

impl IsApiRequest for PatchLolChampSelectV1SessionActionsById {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/actions/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_champ_select_v1_session_actions_by_id(id: i64, body: LolChampSelectChampSelectAction) -> PatchLolChampSelectV1SessionActionsById {
    PatchLolChampSelectV1SessionActionsById{id, body}
}


pub struct PatchLolChampSelectV1SessionMySelection {
    pub body: LolChampSelectChampSelectMySelection,
}

impl IsApiRequest for PatchLolChampSelectV1SessionMySelection {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session/my-selection".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_champ_select_v1_session_my_selection(body: LolChampSelectChampSelectMySelection) -> PatchLolChampSelectV1SessionMySelection {
    PatchLolChampSelectV1SessionMySelection{body}
}


pub struct PostLolChampSelectV1BattleTrainingLaunch {}

impl IsApiRequest for PostLolChampSelectV1BattleTrainingLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select/v1/battle-training/launch".to_string()}
}

pub fn post_lol_champ_select_v1_battle_training_launch() -> PostLolChampSelectV1BattleTrainingLaunch {
    PostLolChampSelectV1BattleTrainingLaunch{}
}


pub struct PostLolChampSelectV1OngoingSwapByIdClear {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1OngoingSwapByIdClear {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/ongoing-swap/{}/clear", self.id)}
}

pub fn post_lol_champ_select_v1_ongoing_swap_by_id_clear(id: i64) -> PostLolChampSelectV1OngoingSwapByIdClear {
    PostLolChampSelectV1OngoingSwapByIdClear{id}
}


pub struct PostLolChampSelectV1OngoingTradeByIdClear {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1OngoingTradeByIdClear {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/ongoing-trade/{}/clear", self.id)}
}

pub fn post_lol_champ_select_v1_ongoing_trade_by_id_clear(id: i64) -> PostLolChampSelectV1OngoingTradeByIdClear {
    PostLolChampSelectV1OngoingTradeByIdClear{id}
}


pub struct PostLolChampSelectV1RetrieveLatestGameDto {}

impl IsApiRequest for PostLolChampSelectV1RetrieveLatestGameDto {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select/v1/retrieve-latest-game-dto".to_string()}
}

pub fn post_lol_champ_select_v1_retrieve_latest_game_dto() -> PostLolChampSelectV1RetrieveLatestGameDto {
    PostLolChampSelectV1RetrieveLatestGameDto{}
}


pub struct PostLolChampSelectV1SessionActionsByIdComplete {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionActionsByIdComplete {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/actions/{}/complete", self.id)}
}

pub fn post_lol_champ_select_v1_session_actions_by_id_complete(id: i64) -> PostLolChampSelectV1SessionActionsByIdComplete {
    PostLolChampSelectV1SessionActionsByIdComplete{id}
}


pub struct PostLolChampSelectV1SessionBenchSwapByChampionId {
    pub champion_id: i32,
}

impl IsApiRequest for PostLolChampSelectV1SessionBenchSwapByChampionId {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/bench/swap/{}", self.champion_id)}
}

pub fn post_lol_champ_select_v1_session_bench_swap_by_champion_id(champion_id: i32) -> PostLolChampSelectV1SessionBenchSwapByChampionId {
    PostLolChampSelectV1SessionBenchSwapByChampionId{champion_id}
}


pub struct PostLolChampSelectV1SessionMySelectionReroll {}

impl IsApiRequest for PostLolChampSelectV1SessionMySelectionReroll {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session/my-selection/reroll".to_string()}
}

pub fn post_lol_champ_select_v1_session_my_selection_reroll() -> PostLolChampSelectV1SessionMySelectionReroll {
    PostLolChampSelectV1SessionMySelectionReroll{}
}


pub struct PostLolChampSelectV1SessionSimpleInventory {}

impl IsApiRequest for PostLolChampSelectV1SessionSimpleInventory {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select/v1/session/simple-inventory".to_string()}
}

pub fn post_lol_champ_select_v1_session_simple_inventory() -> PostLolChampSelectV1SessionSimpleInventory {
    PostLolChampSelectV1SessionSimpleInventory{}
}


pub struct PostLolChampSelectV1SessionSwapsByIdAccept {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionSwapsByIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/swaps/{}/accept", self.id)}
}

pub fn post_lol_champ_select_v1_session_swaps_by_id_accept(id: i64) -> PostLolChampSelectV1SessionSwapsByIdAccept {
    PostLolChampSelectV1SessionSwapsByIdAccept{id}
}


pub struct PostLolChampSelectV1SessionSwapsByIdCancel {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionSwapsByIdCancel {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/swaps/{}/cancel", self.id)}
}

pub fn post_lol_champ_select_v1_session_swaps_by_id_cancel(id: i64) -> PostLolChampSelectV1SessionSwapsByIdCancel {
    PostLolChampSelectV1SessionSwapsByIdCancel{id}
}


pub struct PostLolChampSelectV1SessionSwapsByIdDecline {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionSwapsByIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/swaps/{}/decline", self.id)}
}

pub fn post_lol_champ_select_v1_session_swaps_by_id_decline(id: i64) -> PostLolChampSelectV1SessionSwapsByIdDecline {
    PostLolChampSelectV1SessionSwapsByIdDecline{id}
}


pub struct PostLolChampSelectV1SessionSwapsByIdRequest {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionSwapsByIdRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = LolChampSelectChampSelectSwapContract;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/swaps/{}/request", self.id)}
}

pub fn post_lol_champ_select_v1_session_swaps_by_id_request(id: i64) -> PostLolChampSelectV1SessionSwapsByIdRequest {
    PostLolChampSelectV1SessionSwapsByIdRequest{id}
}


pub struct PostLolChampSelectV1SessionTradesByIdAccept {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionTradesByIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/trades/{}/accept", self.id)}
}

pub fn post_lol_champ_select_v1_session_trades_by_id_accept(id: i64) -> PostLolChampSelectV1SessionTradesByIdAccept {
    PostLolChampSelectV1SessionTradesByIdAccept{id}
}


pub struct PostLolChampSelectV1SessionTradesByIdCancel {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionTradesByIdCancel {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/trades/{}/cancel", self.id)}
}

pub fn post_lol_champ_select_v1_session_trades_by_id_cancel(id: i64) -> PostLolChampSelectV1SessionTradesByIdCancel {
    PostLolChampSelectV1SessionTradesByIdCancel{id}
}


pub struct PostLolChampSelectV1SessionTradesByIdDecline {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionTradesByIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/trades/{}/decline", self.id)}
}

pub fn post_lol_champ_select_v1_session_trades_by_id_decline(id: i64) -> PostLolChampSelectV1SessionTradesByIdDecline {
    PostLolChampSelectV1SessionTradesByIdDecline{id}
}


pub struct PostLolChampSelectV1SessionTradesByIdRequest {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectV1SessionTradesByIdRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = LolChampSelectChampSelectTradeContract;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/session/trades/{}/request", self.id)}
}

pub fn post_lol_champ_select_v1_session_trades_by_id_request(id: i64) -> PostLolChampSelectV1SessionTradesByIdRequest {
    PostLolChampSelectV1SessionTradesByIdRequest{id}
}


pub struct PostLolChampSelectV1TeamBoostPurchase {}

impl IsApiRequest for PostLolChampSelectV1TeamBoostPurchase {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select/v1/team-boost/purchase".to_string()}
}

pub fn post_lol_champ_select_v1_team_boost_purchase() -> PostLolChampSelectV1TeamBoostPurchase {
    PostLolChampSelectV1TeamBoostPurchase{}
}


pub struct PostLolChampSelectV1ToggleFavoriteByChampionIdByPosition {
    pub champion_id: i64,
    pub position: String,
}

impl IsApiRequest for PostLolChampSelectV1ToggleFavoriteByChampionIdByPosition {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select/v1/toggle-favorite/{}/{}", self.champion_id, self.position)}
}

pub fn post_lol_champ_select_v1_toggle_favorite_by_champion_id_by_position(champion_id: i64, position: String) -> PostLolChampSelectV1ToggleFavoriteByChampionIdByPosition {
    PostLolChampSelectV1ToggleFavoriteByChampionIdByPosition{champion_id, position}
}


pub struct PostLolChampSelectV1TogglePlayerMuted {
    pub body: LolChampSelectMutedPlayerInfo,
}

impl IsApiRequest for PostLolChampSelectV1TogglePlayerMuted {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select/v1/toggle-player-muted".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_champ_select_v1_toggle_player_muted(body: LolChampSelectMutedPlayerInfo) -> PostLolChampSelectV1TogglePlayerMuted {
    PostLolChampSelectV1TogglePlayerMuted{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectBenchChampion {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "isPriority")]
    pub is_priority: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampGridChampion {
    pub id: i32,
    pub name: String,
    #[serde(rename = "squarePortraitPath")]
    pub square_portrait_path: String,
    #[serde(rename = "freeToPlay")]
    pub free_to_play: bool,
    #[serde(rename = "loyaltyReward")]
    pub loyalty_reward: bool,
    #[serde(rename = "xboxGPReward")]
    pub xbox_gp_reward: bool,
    #[serde(rename = "freeToPlayForQueue")]
    pub free_to_play_for_queue: bool,
    pub owned: bool,
    pub rented: bool,
    pub disabled: bool,
    pub roles: Vec<String>,
    #[serde(rename = "masteryPoints")]
    pub mastery_points: i32,
    #[serde(rename = "masteryLevel")]
    pub mastery_level: i32,
    #[serde(rename = "selectionStatus")]
    pub selection_status: LolChampSelectChampionSelection,
    #[serde(rename = "positionsFavorited")]
    pub positions_favorited: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectAction {
    pub id: i64,
    #[serde(rename = "actorCellId")]
    pub actor_cell_id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub completed: bool,
    #[serde(rename = "isAllyAction")]
    pub is_ally_action: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectBannedChampions {
    #[serde(rename = "myTeamBans")]
    pub my_team_bans: Vec<i32>,
    #[serde(rename = "theirTeamBans")]
    pub their_team_bans: Vec<i32>,
    #[serde(rename = "numBans")]
    pub num_bans: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectChatRoomDetails {
    #[serde(rename = "multiUserChatId")]
    pub multi_user_chat_id: String,
    #[serde(rename = "multiUserChatPassword")]
    pub multi_user_chat_password: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolChampSelectMucJwtDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectMySelection {
    #[serde(rename = "selectedSkinId")]
    pub selected_skin_id: Option<i32>,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: Option<u64>,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: Option<u64>,
    #[serde(rename = "wardSkinId")]
    pub ward_skin_id: Option<i64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPinDropNotification {
    #[serde(rename = "pinDropSummoners")]
    pub pin_drop_summoners: Vec<LolChampSelectChampSelectPinDropSummoner>,
    #[serde(rename = "mapSide")]
    pub map_side: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPinDropSummoner {
    #[serde(rename = "slotId")]
    pub slot_id: u64,
    pub position: String,
    pub lane: String,
    #[serde(rename = "lanePosition")]
    pub lane_position: u64,
    #[serde(rename = "isLocalSummoner")]
    pub is_local_summoner: bool,
    #[serde(rename = "isPlaceholder")]
    pub is_placeholder: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPlayerSelection {
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "selectedSkinId")]
    pub selected_skin_id: i32,
    #[serde(rename = "wardSkinId")]
    pub ward_skin_id: i64,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: u64,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: u64,
    pub team: i32,
    #[serde(rename = "assignedPosition")]
    pub assigned_position: String,
    #[serde(rename = "championPickIntent")]
    pub champion_pick_intent: i32,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub puuid: String,
    #[serde(rename = "nameVisibilityType")]
    pub name_visibility_type: String,
    #[serde(rename = "obfuscatedSummonerId")]
    pub obfuscated_summoner_id: u64,
    #[serde(rename = "obfuscatedPuuid")]
    pub obfuscated_puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSession {
    #[serde(rename = "gameId")]
    pub game_id: u64,
    pub timer: LolChampSelectChampSelectTimer,
    #[serde(rename = "chatDetails")]
    pub chat_details: LolChampSelectChampSelectChatRoomDetails,
    #[serde(rename = "myTeam")]
    pub my_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    #[serde(rename = "theirTeam")]
    pub their_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub trades: Vec<LolChampSelectChampSelectTradeContract>,
    #[serde(rename = "pickOrderSwaps")]
    pub pick_order_swaps: Vec<LolChampSelectChampSelectSwapContract>,
    pub actions: Vec<Value>,
    pub bans: LolChampSelectChampSelectBannedChampions,
    #[serde(rename = "localPlayerCellId")]
    pub local_player_cell_id: i64,
    #[serde(rename = "isSpectating")]
    pub is_spectating: bool,
    #[serde(rename = "allowSkinSelection")]
    pub allow_skin_selection: bool,
    #[serde(rename = "allowDuplicatePicks")]
    pub allow_duplicate_picks: bool,
    #[serde(rename = "allowBattleBoost")]
    pub allow_battle_boost: bool,
    #[serde(rename = "boostableSkinCount")]
    pub boostable_skin_count: i32,
    #[serde(rename = "allowRerolling")]
    pub allow_rerolling: bool,
    #[serde(rename = "rerollsRemaining")]
    pub rerolls_remaining: u64,
    #[serde(rename = "allowLockedEvents")]
    pub allow_locked_events: bool,
    #[serde(rename = "lockedEventIndex")]
    pub locked_event_index: i32,
    #[serde(rename = "benchEnabled")]
    pub bench_enabled: bool,
    #[serde(rename = "benchChampions")]
    pub bench_champions: Vec<LolChampSelectBenchChampion>,
    pub counter: i64,
    #[serde(rename = "recoveryCounter")]
    pub recovery_counter: i64,
    #[serde(rename = "skipChampionSelect")]
    pub skip_champion_select: bool,
    #[serde(rename = "hasSimultaneousBans")]
    pub has_simultaneous_bans: bool,
    #[serde(rename = "hasSimultaneousPicks")]
    pub has_simultaneous_picks: bool,
    #[serde(rename = "isCustomGame")]
    pub is_custom_game: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSummoner {
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    #[serde(rename = "slotId")]
    pub slot_id: u64,
    #[serde(rename = "spell1IconPath")]
    pub spell_1_icon_path: String,
    #[serde(rename = "spell2IconPath")]
    pub spell_2_icon_path: String,
    #[serde(rename = "assignedPosition")]
    pub assigned_position: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub puuid: String,
    #[serde(rename = "nameVisibilityType")]
    pub name_visibility_type: String,
    #[serde(rename = "obfuscatedSummonerId")]
    pub obfuscated_summoner_id: u64,
    #[serde(rename = "obfuscatedPuuid")]
    pub obfuscated_puuid: String,
    #[serde(rename = "activeActionType")]
    pub active_action_type: String,
    #[serde(rename = "championIconStyle")]
    pub champion_icon_style: String,
    #[serde(rename = "skinSplashPath")]
    pub skin_splash_path: String,
    #[serde(rename = "actingBackgroundAnimationState")]
    pub acting_background_animation_state: String,
    #[serde(rename = "statusMessageKey")]
    pub status_message_key: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championName")]
    pub champion_name: String,
    #[serde(rename = "pickSnipedClass")]
    pub pick_sniped_class: String,
    #[serde(rename = "currentChampionVotePercentInteger")]
    pub current_champion_vote_percent_integer: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "banIntentSquarePortratPath")]
    pub ban_intent_square_portrat_path: String,
    #[serde(rename = "isOnPlayersTeam")]
    pub is_on_players_team: bool,
    #[serde(rename = "shouldShowSelectedSkin")]
    pub should_show_selected_skin: bool,
    #[serde(rename = "shouldShowExpanded")]
    pub should_show_expanded: bool,
    #[serde(rename = "isActingNow")]
    pub is_acting_now: bool,
    #[serde(rename = "shouldShowActingBar")]
    pub should_show_acting_bar: bool,
    #[serde(rename = "isSelf")]
    pub is_self: bool,
    #[serde(rename = "shouldShowBanIntentIcon")]
    pub should_show_ban_intent_icon: bool,
    #[serde(rename = "isPickIntenting")]
    pub is_pick_intenting: bool,
    #[serde(rename = "isDonePicking")]
    pub is_done_picking: bool,
    #[serde(rename = "isPlaceholder")]
    pub is_placeholder: bool,
    #[serde(rename = "shouldShowSpells")]
    pub should_show_spells: bool,
    #[serde(rename = "shouldShowRingAnimations")]
    pub should_show_ring_animations: bool,
    #[serde(rename = "areSummonerActionsComplete")]
    pub are_summoner_actions_complete: bool,
    #[serde(rename = "tradeId")]
    pub trade_id: i64,
    #[serde(rename = "swapId")]
    pub swap_id: i64,
    #[serde(rename = "showTrades")]
    pub show_trades: bool,
    #[serde(rename = "showSwaps")]
    pub show_swaps: bool,
    #[serde(rename = "showMuted")]
    pub show_muted: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSwapContract {
    pub id: i64,
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectSwapState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSwapNotification {
    pub id: i64,
    #[serde(rename = "requestorIndex")]
    pub requestor_index: i64,
    #[serde(rename = "responderIndex")]
    pub responder_index: i64,
    pub state: LolChampSelectChampSelectSwapState,
    #[serde(rename = "otherSummonerIndex")]
    pub other_summoner_index: i64,
    #[serde(rename = "initiatedByLocalPlayer")]
    pub initiated_by_local_player: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTimer {
    #[serde(rename = "adjustedTimeLeftInPhase")]
    pub adjusted_time_left_in_phase: i64,
    #[serde(rename = "totalTimeInPhase")]
    pub total_time_in_phase: i64,
    pub phase: String,
    #[serde(rename = "isInfinite")]
    pub is_infinite: bool,
    #[serde(rename = "internalNowInEpochMs")]
    pub internal_now_in_epoch_ms: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTradeContract {
    pub id: i64,
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectTradeState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTradeNotification {
    pub id: i64,
    #[serde(rename = "responderIndex")]
    pub responder_index: i64,
    pub state: LolChampSelectChampSelectTradeState,
    #[serde(rename = "otherSummonerIndex")]
    pub other_summoner_index: i64,
    #[serde(rename = "responderChampionName")]
    pub responder_champion_name: String,
    #[serde(rename = "requesterChampionName")]
    pub requester_champion_name: String,
    #[serde(rename = "requesterChampionSplashPath")]
    pub requester_champion_splash_path: String,
    #[serde(rename = "initiatedByLocalPlayer")]
    pub initiated_by_local_player: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionSelection {
    #[serde(rename = "selectedByMe")]
    pub selected_by_me: bool,
    #[serde(rename = "banIntentedByMe")]
    pub ban_intented_by_me: bool,
    #[serde(rename = "banIntented")]
    pub ban_intented: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "pickIntented")]
    pub pick_intented: bool,
    #[serde(rename = "pickIntentedByMe")]
    pub pick_intented_by_me: bool,
    #[serde(rename = "pickIntentedPosition")]
    pub pick_intented_position: String,
    #[serde(rename = "pickedByOtherOrBanned")]
    pub picked_by_other_or_banned: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionSkinAugmentOverlays {
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
pub struct LolChampSelectCollectionsChampionSkinEmblem {
    pub name: String,
    #[serde(rename = "emblemPath")]
    pub emblem_path: LolChampSelectCollectionsChampionSkinEmblemPath,
    pub positions: LolChampSelectCollectionsChampionSkinEmblemPosition,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinEmblemPath {
    pub large: String,
    pub small: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinEmblemPosition {
    pub vertical: String,
    pub horizontal: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsOwnership {
    #[serde(rename = "loyaltyReward")]
    pub loyalty_reward: bool,
    #[serde(rename = "xboxGPReward")]
    pub xbox_gp_reward: bool,
    pub owned: bool,
    pub rental: LolChampSelectCollectionsRental,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsRental {
    pub rented: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectMucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectMutedPlayerInfo {
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "obfuscatedPuuid")]
    pub obfuscated_puuid: String,
    #[serde(rename = "obfuscatedSummonerId")]
    pub obfuscated_summoner_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSfxNotification {
    #[serde(rename = "delayMillis")]
    pub delay_millis: i64,
    pub path: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorChildSkin {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "chromaPreviewPath")]
    pub chroma_preview_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    #[serde(rename = "isBase")]
    pub is_base: bool,
    pub disabled: bool,
    #[serde(rename = "stillObtainable")]
    pub still_obtainable: bool,
    #[serde(rename = "isChampionUnlocked")]
    pub is_champion_unlocked: bool,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: Option<String>,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    pub unlocked: bool,
    #[serde(rename = "skinAugments")]
    pub skin_augments: HashMap<String, LolChampSelectChampionSkinAugmentOverlays>,
    #[serde(rename = "parentSkinId")]
    pub parent_skin_id: i32,
    pub colors: Vec<String>,
    pub stage: u64,
    #[serde(rename = "shortName")]
    pub short_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorInfo {
    #[serde(rename = "selectedSkinId")]
    pub selected_skin_id: i32,
    #[serde(rename = "isSkinGrantedFromBoost")]
    pub is_skin_granted_from_boost: bool,
    #[serde(rename = "selectedChampionId")]
    pub selected_champion_id: i32,
    #[serde(rename = "championName")]
    pub champion_name: String,
    #[serde(rename = "skinSelectionDisabled")]
    pub skin_selection_disabled: bool,
    #[serde(rename = "showSkinSelector")]
    pub show_skin_selector: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorSkin {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "chromaPreviewPath")]
    pub chroma_preview_path: Option<String>,
    pub id: i32,
    pub name: String,
    pub ownership: LolChampSelectCollectionsOwnership,
    #[serde(rename = "isBase")]
    pub is_base: bool,
    pub disabled: bool,
    #[serde(rename = "stillObtainable")]
    pub still_obtainable: bool,
    #[serde(rename = "isChampionUnlocked")]
    pub is_champion_unlocked: bool,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: Option<String>,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    pub unlocked: bool,
    #[serde(rename = "skinAugments")]
    pub skin_augments: HashMap<String, LolChampSelectChampionSkinAugmentOverlays>,
    #[serde(rename = "childSkins")]
    pub child_skins: Vec<LolChampSelectSkinSelectorChildSkin>,
    pub emblems: Vec<LolChampSelectCollectionsChampionSkinEmblem>,
    #[serde(rename = "rarityGemPath")]
    pub rarity_gem_path: String,
    #[serde(rename = "groupSplash")]
    pub group_splash: String,
    #[serde(rename = "productType")]
    pub product_type: Option<LolChampSelectQuestSkinProductType>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectTeamBoost {
    #[serde(rename = "summonerId")]
    pub summoner_id: i64,
    pub puuid: String,
    #[serde(rename = "skinUnlockMode")]
    pub skin_unlock_mode: String,
    pub price: i64,
    #[serde(rename = "ipReward")]
    pub ip_reward: i64,
    #[serde(rename = "ipRewardForPurchaser")]
    pub ip_reward_for_purchaser: i64,
    #[serde(rename = "availableSkins")]
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChampSelectChampSelectSwapState {
    #[default]
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "SENT")]
    Sent,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "AVAILABLE")]
    Available,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChampSelectChampSelectTradeState {
    #[default]
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "SENT")]
    Sent,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "AVAILABLE")]
    Available,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChampSelectQuestSkinProductType {
    #[default]
    #[serde(rename = "kTieredSkin")]
    KTieredSkin,
    #[serde(rename = "kQuestSkin")]
    KQuestSkin,
}

