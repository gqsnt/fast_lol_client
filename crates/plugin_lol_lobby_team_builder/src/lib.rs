
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolLobbyTeamBuilderChampSelectV1BannableChampionIds {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1BannableChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/bannable-champion-ids".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_bannable_champion_ids() -> GetLolLobbyTeamBuilderChampSelectV1BannableChampionIds {
    GetLolLobbyTeamBuilderChampSelectV1BannableChampionIds{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1CurrentChampion {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1CurrentChampion {
    const METHOD: Method = Method::GET;
    type ReturnType = i32;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/current-champion".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_current_champion() -> GetLolLobbyTeamBuilderChampSelectV1CurrentChampion {
    GetLolLobbyTeamBuilderChampSelectV1CurrentChampion{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1DisabledChampionIds {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1DisabledChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/disabled-champion-ids".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_disabled_champion_ids() -> GetLolLobbyTeamBuilderChampSelectV1DisabledChampionIds {
    GetLolLobbyTeamBuilderChampSelectV1DisabledChampionIds{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1HasAutoAssignedSmite {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1HasAutoAssignedSmite {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/has-auto-assigned-smite".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_has_auto_assigned_smite() -> GetLolLobbyTeamBuilderChampSelectV1HasAutoAssignedSmite {
    GetLolLobbyTeamBuilderChampSelectV1HasAutoAssignedSmite{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1ImplementationActive {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1ImplementationActive {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/implementation-active".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_implementation_active() -> GetLolLobbyTeamBuilderChampSelectV1ImplementationActive {
    GetLolLobbyTeamBuilderChampSelectV1ImplementationActive{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1PickableChampionIds {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1PickableChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/pickable-champion-ids".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_pickable_champion_ids() -> GetLolLobbyTeamBuilderChampSelectV1PickableChampionIds {
    GetLolLobbyTeamBuilderChampSelectV1PickableChampionIds{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1PickableSkinIds {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1PickableSkinIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/pickable-skin-ids".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_pickable_skin_ids() -> GetLolLobbyTeamBuilderChampSelectV1PickableSkinIds {
    GetLolLobbyTeamBuilderChampSelectV1PickableSkinIds{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1Preferences {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1Preferences {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderChampionSelectPreferences;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/preferences".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_preferences() -> GetLolLobbyTeamBuilderChampSelectV1Preferences {
    GetLolLobbyTeamBuilderChampSelectV1Preferences{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SendingLoadoutsGcosEnabled {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SendingLoadoutsGcosEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/sending-loadouts-gcos-enabled".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_sending_loadouts_gcos_enabled() -> GetLolLobbyTeamBuilderChampSelectV1SendingLoadoutsGcosEnabled {
    GetLolLobbyTeamBuilderChampSelectV1SendingLoadoutsGcosEnabled{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1Session {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderChampSelectSession;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session() -> GetLolLobbyTeamBuilderChampSelectV1Session {
    GetLolLobbyTeamBuilderChampSelectV1Session{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SessionMySelection {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SessionMySelection {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderChampSelectPlayerSelection;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session/my-selection".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session_my_selection() -> GetLolLobbyTeamBuilderChampSelectV1SessionMySelection {
    GetLolLobbyTeamBuilderChampSelectV1SessionMySelection{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SessionObfuscatedSummonerIds {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SessionObfuscatedSummonerIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u64>;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session/obfuscated-summoner-ids".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session_obfuscated_summoner_ids() -> GetLolLobbyTeamBuilderChampSelectV1SessionObfuscatedSummonerIds {
    GetLolLobbyTeamBuilderChampSelectV1SessionObfuscatedSummonerIds{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SessionSwaps {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SessionSwaps {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyTeamBuilderChampSelectSwapContract>;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session/swaps".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session_swaps() -> GetLolLobbyTeamBuilderChampSelectV1SessionSwaps {
    GetLolLobbyTeamBuilderChampSelectV1SessionSwaps{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SessionSwapsById {
    pub id: i64,
}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SessionSwapsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderChampSelectSwapContract;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/swaps/{}", self.id)}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id(id: i64) -> GetLolLobbyTeamBuilderChampSelectV1SessionSwapsById {
    GetLolLobbyTeamBuilderChampSelectV1SessionSwapsById{id}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SessionTimer {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SessionTimer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderChampSelectTimer;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session/timer".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session_timer() -> GetLolLobbyTeamBuilderChampSelectV1SessionTimer {
    GetLolLobbyTeamBuilderChampSelectV1SessionTimer{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SessionTrades {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SessionTrades {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyTeamBuilderChampSelectTradeContract>;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session/trades".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session_trades() -> GetLolLobbyTeamBuilderChampSelectV1SessionTrades {
    GetLolLobbyTeamBuilderChampSelectV1SessionTrades{}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1SessionTradesById {
    pub id: i64,
}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1SessionTradesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderChampSelectTradeContract;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/trades/{}", self.id)}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_session_trades_by_id(id: i64) -> GetLolLobbyTeamBuilderChampSelectV1SessionTradesById {
    GetLolLobbyTeamBuilderChampSelectV1SessionTradesById{id}
}


pub struct GetLolLobbyTeamBuilderChampSelectV1TeamBoost {}

impl IsApiRequest for GetLolLobbyTeamBuilderChampSelectV1TeamBoost {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderTeamBoost;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/team-boost".to_string()}
}

pub fn get_lol_lobby_team_builder_champ_select_v1_team_boost() -> GetLolLobbyTeamBuilderChampSelectV1TeamBoost {
    GetLolLobbyTeamBuilderChampSelectV1TeamBoost{}
}


pub struct GetLolLobbyTeamBuilderV1Matchmaking {}

impl IsApiRequest for GetLolLobbyTeamBuilderV1Matchmaking {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyTeamBuilderMatchmakingSearchResource;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/v1/matchmaking".to_string()}
}

pub fn get_lol_lobby_team_builder_v1_matchmaking() -> GetLolLobbyTeamBuilderV1Matchmaking {
    GetLolLobbyTeamBuilderV1Matchmaking{}
}


pub struct PatchLolLobbyTeamBuilderChampSelectV1SessionActionsById {
    pub id: i32,
    pub body: LolLobbyTeamBuilderChampSelectAction,
}

impl IsApiRequest for PatchLolLobbyTeamBuilderChampSelectV1SessionActionsById {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/actions/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_lobby_team_builder_champ_select_v1_session_actions_by_id(id: i32, body: LolLobbyTeamBuilderChampSelectAction) -> PatchLolLobbyTeamBuilderChampSelectV1SessionActionsById {
    PatchLolLobbyTeamBuilderChampSelectV1SessionActionsById{id, body}
}


pub struct PatchLolLobbyTeamBuilderChampSelectV1SessionMySelection {
    pub body: LolLobbyTeamBuilderChampSelectMySelection,
}

impl IsApiRequest for PatchLolLobbyTeamBuilderChampSelectV1SessionMySelection {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session/my-selection".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_lobby_team_builder_champ_select_v1_session_my_selection(body: LolLobbyTeamBuilderChampSelectMySelection) -> PatchLolLobbyTeamBuilderChampSelectV1SessionMySelection {
    PatchLolLobbyTeamBuilderChampSelectV1SessionMySelection{body}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1RetrieveLatestGameDto {}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1RetrieveLatestGameDto {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/retrieve-latest-game-dto".to_string()}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_retrieve_latest_game_dto() -> PostLolLobbyTeamBuilderChampSelectV1RetrieveLatestGameDto {
    PostLolLobbyTeamBuilderChampSelectV1RetrieveLatestGameDto{}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionActionsByIdComplete {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionActionsByIdComplete {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/actions/{}/complete", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_actions_by_id_complete(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionActionsByIdComplete {
    PostLolLobbyTeamBuilderChampSelectV1SessionActionsByIdComplete{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionBenchSwapByChampionId {
    pub champion_id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionBenchSwapByChampionId {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/bench/swap/{}", self.champion_id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_bench_swap_by_champion_id(champion_id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionBenchSwapByChampionId {
    PostLolLobbyTeamBuilderChampSelectV1SessionBenchSwapByChampionId{champion_id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionMySelectionReroll {}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionMySelectionReroll {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/session/my-selection/reroll".to_string()}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_my_selection_reroll() -> PostLolLobbyTeamBuilderChampSelectV1SessionMySelectionReroll {
    PostLolLobbyTeamBuilderChampSelectV1SessionMySelectionReroll{}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdAccept {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/swaps/{}/accept", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_accept(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdAccept {
    PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdAccept{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdCancel {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdCancel {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/swaps/{}/cancel", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_cancel(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdCancel {
    PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdCancel{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdDecline {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/swaps/{}/decline", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_decline(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdDecline {
    PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdDecline{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdRequest {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyTeamBuilderChampSelectSwapContract;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/swaps/{}/request", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_swaps_by_id_request(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdRequest {
    PostLolLobbyTeamBuilderChampSelectV1SessionSwapsByIdRequest{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdAccept {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/trades/{}/accept", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_accept(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdAccept {
    PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdAccept{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdCancel {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdCancel {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/trades/{}/cancel", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_cancel(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdCancel {
    PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdCancel{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdDecline {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/trades/{}/decline", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_decline(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdDecline {
    PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdDecline{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdRequest {
    pub id: i32,
}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyTeamBuilderChampSelectTradeContract;
    fn get_url(&self) -> String {format!("/lol-lobby-team-builder/champ-select/v1/session/trades/{}/request", self.id)}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_session_trades_by_id_request(id: i32) -> PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdRequest {
    PostLolLobbyTeamBuilderChampSelectV1SessionTradesByIdRequest{id}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1SimpleInventory {}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1SimpleInventory {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/simple-inventory".to_string()}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_simple_inventory() -> PostLolLobbyTeamBuilderChampSelectV1SimpleInventory {
    PostLolLobbyTeamBuilderChampSelectV1SimpleInventory{}
}


pub struct PostLolLobbyTeamBuilderChampSelectV1TeamBoostPurchase {}

impl IsApiRequest for PostLolLobbyTeamBuilderChampSelectV1TeamBoostPurchase {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/champ-select/v1/team-boost/purchase".to_string()}
}

pub fn post_lol_lobby_team_builder_champ_select_v1_team_boost_purchase() -> PostLolLobbyTeamBuilderChampSelectV1TeamBoostPurchase {
    PostLolLobbyTeamBuilderChampSelectV1TeamBoostPurchase{}
}


pub struct PostLolLobbyTeamBuilderV1ReadyCheckAccept {}

impl IsApiRequest for PostLolLobbyTeamBuilderV1ReadyCheckAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/v1/ready-check/accept".to_string()}
}

pub fn post_lol_lobby_team_builder_v1_ready_check_accept() -> PostLolLobbyTeamBuilderV1ReadyCheckAccept {
    PostLolLobbyTeamBuilderV1ReadyCheckAccept{}
}


pub struct PostLolLobbyTeamBuilderV1ReadyCheckDecline {}

impl IsApiRequest for PostLolLobbyTeamBuilderV1ReadyCheckDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby-team-builder/v1/ready-check/decline".to_string()}
}

pub fn post_lol_lobby_team_builder_v1_ready_check_decline() -> PostLolLobbyTeamBuilderV1ReadyCheckDecline {
    PostLolLobbyTeamBuilderV1ReadyCheckDecline{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderBenchChampion {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "isPriority")]
    pub is_priority: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectAction {
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
    #[serde(rename = "isInProgress")]
    pub is_in_progress: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectChatRoomDetails {
    #[serde(rename = "multiUserChatId")]
    pub multi_user_chat_id: String,
    #[serde(rename = "multiUserChatPassword")]
    pub multi_user_chat_password: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolLobbyTeamBuilderMucJwtDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectMySelection {
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
pub struct LolLobbyTeamBuilderChampSelectPlayerSelection {
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
    #[serde(rename = "playerType")]
    pub player_type: String,
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
pub struct LolLobbyTeamBuilderChampSelectSession {
    #[serde(rename = "gameId")]
    pub game_id: u64,
    pub timer: LolLobbyTeamBuilderChampSelectTimer,
    #[serde(rename = "chatDetails")]
    pub chat_details: LolLobbyTeamBuilderChampSelectChatRoomDetails,
    #[serde(rename = "myTeam")]
    pub my_team: Vec<LolLobbyTeamBuilderChampSelectPlayerSelection>,
    #[serde(rename = "theirTeam")]
    pub their_team: Vec<LolLobbyTeamBuilderChampSelectPlayerSelection>,
    pub trades: Vec<LolLobbyTeamBuilderChampSelectTradeContract>,
    #[serde(rename = "pickOrderSwaps")]
    pub pick_order_swaps: Vec<LolLobbyTeamBuilderChampSelectSwapContract>,
    pub actions: Vec<Value>,
    #[serde(rename = "localPlayerCellId")]
    pub local_player_cell_id: i64,
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
    pub bench_champions: Vec<LolLobbyTeamBuilderBenchChampion>,
    pub counter: i64,
    #[serde(rename = "recoveryCounter")]
    pub recovery_counter: i64,
    #[serde(rename = "skipChampionSelect")]
    pub skip_champion_select: bool,
    #[serde(rename = "isSpectating")]
    pub is_spectating: bool,
    #[serde(rename = "hasSimultaneousBans")]
    pub has_simultaneous_bans: bool,
    #[serde(rename = "hasSimultaneousPicks")]
    pub has_simultaneous_picks: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectSwapContract {
    pub id: i64,
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    pub state: LolLobbyTeamBuilderChampSelectSwapState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectTimer {
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
pub struct LolLobbyTeamBuilderChampSelectTradeContract {
    pub id: i64,
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    pub state: LolLobbyTeamBuilderChampSelectTradeState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionSelectPreferences {
    pub skins: HashMap<String, i32>,
    pub spells: HashMap<String, Value>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingDodgeData {
    pub state: LolLobbyTeamBuilderMatchmakingDodgeState,
    #[serde(rename = "dodgerId")]
    pub dodger_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingLowPriorityData {
    #[serde(rename = "penalizedSummonerIds")]
    pub penalized_summoner_ids: Vec<u64>,
    #[serde(rename = "penaltyTime")]
    pub penalty_time: f64,
    #[serde(rename = "penaltyTimeRemaining")]
    pub penalty_time_remaining: f64,
    #[serde(rename = "bustedLeaverAccessToken")]
    pub busted_leaver_access_token: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingReadyCheckResource {
    pub state: LolLobbyTeamBuilderMatchmakingReadyCheckState,
    #[serde(rename = "playerResponse")]
    pub player_response: LolLobbyTeamBuilderMatchmakingReadyCheckResponse,
    #[serde(rename = "dodgeWarning")]
    pub dodge_warning: LolLobbyTeamBuilderMatchmakingDodgeWarning,
    pub timer: f32,
    #[serde(rename = "declinerIds")]
    pub decliner_ids: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearchErrorResource {
    pub id: i32,
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[serde(rename = "penalizedSummonerId")]
    pub penalized_summoner_id: u64,
    #[serde(rename = "penaltyTimeRemaining")]
    pub penalty_time_remaining: f64,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearchResource {
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "isCurrentlyInQueue")]
    pub is_currently_in_queue: bool,
    #[serde(rename = "lobbyId")]
    pub lobby_id: String,
    #[serde(rename = "searchState")]
    pub search_state: LolLobbyTeamBuilderMatchmakingSearchState,
    #[serde(rename = "timeInQueue")]
    pub time_in_queue: f32,
    #[serde(rename = "estimatedQueueTime")]
    pub estimated_queue_time: f32,
    #[serde(rename = "readyCheck")]
    pub ready_check: LolLobbyTeamBuilderMatchmakingReadyCheckResource,
    #[serde(rename = "dodgeData")]
    pub dodge_data: LolLobbyTeamBuilderMatchmakingDodgeData,
    #[serde(rename = "lowPriorityData")]
    pub low_priority_data: LolLobbyTeamBuilderMatchmakingLowPriorityData,
    pub errors: Vec<LolLobbyTeamBuilderMatchmakingSearchErrorResource>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTeamBoost {
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
pub enum LolLobbyTeamBuilderChampSelectSwapState {
    #[default]
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
pub enum LolLobbyTeamBuilderChampSelectTradeState {
    #[default]
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
pub enum LolLobbyTeamBuilderMatchmakingDodgeState {
    #[default]
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyTeamBuilderMatchmakingDodgeWarning {
    #[default]
    Penalty,
    Warning,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyTeamBuilderMatchmakingReadyCheckResponse {
    #[default]
    Declined,
    Accepted,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyTeamBuilderMatchmakingReadyCheckState {
    #[default]
    Error,
    PartyNotReady,
    StrangerNotReady,
    EveryoneReady,
    InProgress,
    Invalid,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyTeamBuilderMatchmakingSearchState {
    #[default]
    ServiceShutdown,
    ServiceError,
    Error,
    Found,
    Searching,
    Canceled,
    AbandonedLowPriorityQueue,
    Invalid,
}

