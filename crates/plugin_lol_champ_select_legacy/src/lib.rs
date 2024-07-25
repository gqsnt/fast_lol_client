
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolChampSelectLegacyV1BannableChampionIds {}

impl IsApiRequest for GetLolChampSelectLegacyV1BannableChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/bannable-champion-ids".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_bannable_champion_ids() -> GetLolChampSelectLegacyV1BannableChampionIds {
    GetLolChampSelectLegacyV1BannableChampionIds{}
}


pub struct GetLolChampSelectLegacyV1CurrentChampion {}

impl IsApiRequest for GetLolChampSelectLegacyV1CurrentChampion {
    const METHOD: Method = Method::GET;
    type ReturnType = i32;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/current-champion".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_current_champion() -> GetLolChampSelectLegacyV1CurrentChampion {
    GetLolChampSelectLegacyV1CurrentChampion{}
}


pub struct GetLolChampSelectLegacyV1DisabledChampionIds {}

impl IsApiRequest for GetLolChampSelectLegacyV1DisabledChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/disabled-champion-ids".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_disabled_champion_ids() -> GetLolChampSelectLegacyV1DisabledChampionIds {
    GetLolChampSelectLegacyV1DisabledChampionIds{}
}


pub struct GetLolChampSelectLegacyV1ImplementationActive {}

impl IsApiRequest for GetLolChampSelectLegacyV1ImplementationActive {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/implementation-active".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_implementation_active() -> GetLolChampSelectLegacyV1ImplementationActive {
    GetLolChampSelectLegacyV1ImplementationActive{}
}


pub struct GetLolChampSelectLegacyV1PickableChampionIds {}

impl IsApiRequest for GetLolChampSelectLegacyV1PickableChampionIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/pickable-champion-ids".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_pickable_champion_ids() -> GetLolChampSelectLegacyV1PickableChampionIds {
    GetLolChampSelectLegacyV1PickableChampionIds{}
}


pub struct GetLolChampSelectLegacyV1PickableSkinIds {}

impl IsApiRequest for GetLolChampSelectLegacyV1PickableSkinIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i32>;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/pickable-skin-ids".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_pickable_skin_ids() -> GetLolChampSelectLegacyV1PickableSkinIds {
    GetLolChampSelectLegacyV1PickableSkinIds{}
}


pub struct GetLolChampSelectLegacyV1Session {}

impl IsApiRequest for GetLolChampSelectLegacyV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectLegacyChampSelectSession;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/session".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_session() -> GetLolChampSelectLegacyV1Session {
    GetLolChampSelectLegacyV1Session{}
}


pub struct GetLolChampSelectLegacyV1SessionMySelection {}

impl IsApiRequest for GetLolChampSelectLegacyV1SessionMySelection {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectLegacyChampSelectPlayerSelection;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/session/my-selection".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_session_my_selection() -> GetLolChampSelectLegacyV1SessionMySelection {
    GetLolChampSelectLegacyV1SessionMySelection{}
}


pub struct GetLolChampSelectLegacyV1SessionTimer {}

impl IsApiRequest for GetLolChampSelectLegacyV1SessionTimer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectLegacyChampSelectTimer;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/session/timer".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_session_timer() -> GetLolChampSelectLegacyV1SessionTimer {
    GetLolChampSelectLegacyV1SessionTimer{}
}


pub struct GetLolChampSelectLegacyV1SessionTrades {}

impl IsApiRequest for GetLolChampSelectLegacyV1SessionTrades {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChampSelectLegacyChampSelectTradeContract>;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/session/trades".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_session_trades() -> GetLolChampSelectLegacyV1SessionTrades {
    GetLolChampSelectLegacyV1SessionTrades{}
}


pub struct GetLolChampSelectLegacyV1SessionTradesById {
    pub id: i64,
}

impl IsApiRequest for GetLolChampSelectLegacyV1SessionTradesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectLegacyChampSelectTradeContract;
    fn get_url(&self) -> String {format!("/lol-champ-select-legacy/v1/session/trades/{}", self.id)}
}

pub fn get_lol_champ_select_legacy_v1_session_trades_by_id(id: i64) -> GetLolChampSelectLegacyV1SessionTradesById {
    GetLolChampSelectLegacyV1SessionTradesById{id}
}


pub struct GetLolChampSelectLegacyV1TeamBoost {}

impl IsApiRequest for GetLolChampSelectLegacyV1TeamBoost {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChampSelectLegacyTeamBoost;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/team-boost".to_string()}
}

pub fn get_lol_champ_select_legacy_v1_team_boost() -> GetLolChampSelectLegacyV1TeamBoost {
    GetLolChampSelectLegacyV1TeamBoost{}
}


pub struct PatchLolChampSelectLegacyV1SessionActionsById {
    pub id: u64,
    pub body: LolChampSelectLegacyChampSelectAction,
}

impl IsApiRequest for PatchLolChampSelectLegacyV1SessionActionsById {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select-legacy/v1/session/actions/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_champ_select_legacy_v1_session_actions_by_id(id: u64, body: LolChampSelectLegacyChampSelectAction) -> PatchLolChampSelectLegacyV1SessionActionsById {
    PatchLolChampSelectLegacyV1SessionActionsById{id, body}
}


pub struct PatchLolChampSelectLegacyV1SessionMySelection {
    pub body: LolChampSelectLegacyChampSelectMySelection,
}

impl IsApiRequest for PatchLolChampSelectLegacyV1SessionMySelection {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/session/my-selection".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_lol_champ_select_legacy_v1_session_my_selection(body: LolChampSelectLegacyChampSelectMySelection) -> PatchLolChampSelectLegacyV1SessionMySelection {
    PatchLolChampSelectLegacyV1SessionMySelection{body}
}


pub struct PostLolChampSelectLegacyV1BattleTrainingLaunch {}

impl IsApiRequest for PostLolChampSelectLegacyV1BattleTrainingLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/battle-training/launch".to_string()}
}

pub fn post_lol_champ_select_legacy_v1_battle_training_launch() -> PostLolChampSelectLegacyV1BattleTrainingLaunch {
    PostLolChampSelectLegacyV1BattleTrainingLaunch{}
}


pub struct PostLolChampSelectLegacyV1SessionActionsByIdComplete {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectLegacyV1SessionActionsByIdComplete {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select-legacy/v1/session/actions/{}/complete", self.id)}
}

pub fn post_lol_champ_select_legacy_v1_session_actions_by_id_complete(id: i64) -> PostLolChampSelectLegacyV1SessionActionsByIdComplete {
    PostLolChampSelectLegacyV1SessionActionsByIdComplete{id}
}


pub struct PostLolChampSelectLegacyV1SessionMySelectionReroll {}

impl IsApiRequest for PostLolChampSelectLegacyV1SessionMySelectionReroll {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-champ-select-legacy/v1/session/my-selection/reroll".to_string()}
}

pub fn post_lol_champ_select_legacy_v1_session_my_selection_reroll() -> PostLolChampSelectLegacyV1SessionMySelectionReroll {
    PostLolChampSelectLegacyV1SessionMySelectionReroll{}
}


pub struct PostLolChampSelectLegacyV1SessionTradesByIdAccept {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectLegacyV1SessionTradesByIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select-legacy/v1/session/trades/{}/accept", self.id)}
}

pub fn post_lol_champ_select_legacy_v1_session_trades_by_id_accept(id: i64) -> PostLolChampSelectLegacyV1SessionTradesByIdAccept {
    PostLolChampSelectLegacyV1SessionTradesByIdAccept{id}
}


pub struct PostLolChampSelectLegacyV1SessionTradesByIdCancel {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectLegacyV1SessionTradesByIdCancel {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select-legacy/v1/session/trades/{}/cancel", self.id)}
}

pub fn post_lol_champ_select_legacy_v1_session_trades_by_id_cancel(id: i64) -> PostLolChampSelectLegacyV1SessionTradesByIdCancel {
    PostLolChampSelectLegacyV1SessionTradesByIdCancel{id}
}


pub struct PostLolChampSelectLegacyV1SessionTradesByIdDecline {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectLegacyV1SessionTradesByIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-champ-select-legacy/v1/session/trades/{}/decline", self.id)}
}

pub fn post_lol_champ_select_legacy_v1_session_trades_by_id_decline(id: i64) -> PostLolChampSelectLegacyV1SessionTradesByIdDecline {
    PostLolChampSelectLegacyV1SessionTradesByIdDecline{id}
}


pub struct PostLolChampSelectLegacyV1SessionTradesByIdRequest {
    pub id: i64,
}

impl IsApiRequest for PostLolChampSelectLegacyV1SessionTradesByIdRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = LolChampSelectLegacyChampSelectTradeContract;
    fn get_url(&self) -> String {format!("/lol-champ-select-legacy/v1/session/trades/{}/request", self.id)}
}

pub fn post_lol_champ_select_legacy_v1_session_trades_by_id_request(id: i64) -> PostLolChampSelectLegacyV1SessionTradesByIdRequest {
    PostLolChampSelectLegacyV1SessionTradesByIdRequest{id}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectAction {
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
    #[serde(rename = "pickTurn")]
    pub pick_turn: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectBannedChampions {
    #[serde(rename = "myTeamBans")]
    pub my_team_bans: Vec<i32>,
    #[serde(rename = "theirTeamBans")]
    pub their_team_bans: Vec<i32>,
    #[serde(rename = "numBans")]
    pub num_bans: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectChatRoomDetails {
    #[serde(rename = "multiUserChatId")]
    pub multi_user_chat_id: String,
    #[serde(rename = "multiUserChatPassword")]
    pub multi_user_chat_password: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolChampSelectLegacyMucJwtDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectMySelection {
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
pub struct LolChampSelectLegacyChampSelectPlayerSelection {
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
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectSession {
    pub timer: LolChampSelectLegacyChampSelectTimer,
    #[serde(rename = "chatDetails")]
    pub chat_details: LolChampSelectLegacyChampSelectChatRoomDetails,
    #[serde(rename = "myTeam")]
    pub my_team: Vec<LolChampSelectLegacyChampSelectPlayerSelection>,
    #[serde(rename = "theirTeam")]
    pub their_team: Vec<LolChampSelectLegacyChampSelectPlayerSelection>,
    pub trades: Vec<LolChampSelectLegacyChampSelectTradeContract>,
    pub actions: Vec<Value>,
    pub bans: LolChampSelectLegacyChampSelectBannedChampions,
    #[serde(rename = "localPlayerCellId")]
    pub local_player_cell_id: i64,
    #[serde(rename = "isSpectating")]
    pub is_spectating: bool,
    #[serde(rename = "allowSkinSelection")]
    pub allow_skin_selection: bool,
    #[serde(rename = "allowBattleBoost")]
    pub allow_battle_boost: bool,
    #[serde(rename = "allowRerolling")]
    pub allow_rerolling: bool,
    #[serde(rename = "rerollsRemaining")]
    pub rerolls_remaining: u64,
    #[serde(rename = "hasSimultaneousBans")]
    pub has_simultaneous_bans: bool,
    #[serde(rename = "hasSimultaneousPicks")]
    pub has_simultaneous_picks: bool,
    #[serde(rename = "isCustomGame")]
    pub is_custom_game: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectTimer {
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
pub struct LolChampSelectLegacyChampSelectTradeContract {
    pub id: i64,
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    pub state: LolChampSelectLegacyChampSelectTradeState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyMucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyTeamBoost {
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
pub enum LolChampSelectLegacyChampSelectTradeState {
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

