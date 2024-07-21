
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolGameflowV1EarlyExitNotificationsEog {

}

impl IsApiRequest for GetLolGameflowV1EarlyExitNotificationsEog {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<HashMap<String, String>>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/early-exit-notifications/eog".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_early_exit_notifications_eog() -> GetLolGameflowV1EarlyExitNotificationsEog {
    GetLolGameflowV1EarlyExitNotificationsEog {
        
    }
}


pub struct DeleteLolGameflowV1EarlyExitNotificationsEog {

}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsEog {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/early-exit-notifications/eog".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_gameflow_v_1_early_exit_notifications_eog() -> DeleteLolGameflowV1EarlyExitNotificationsEog {
    DeleteLolGameflowV1EarlyExitNotificationsEog {
        
    }
}


pub struct DeleteLolGameflowV1EarlyExitNotificationsEogByKey {

    pub key: i32,
}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsEogByKey {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-gameflow/v1/early-exit-notifications/eog/{}", self.key)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_gameflow_v_1_early_exit_notifications_eog_by_key(key: i32) -> DeleteLolGameflowV1EarlyExitNotificationsEogByKey {
    DeleteLolGameflowV1EarlyExitNotificationsEogByKey {
        key
    }
}


pub struct GetLolGameflowV1EarlyExitNotificationsMissions {

}

impl IsApiRequest for GetLolGameflowV1EarlyExitNotificationsMissions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<HashMap<String, String>>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/early-exit-notifications/missions".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_early_exit_notifications_missions() -> GetLolGameflowV1EarlyExitNotificationsMissions {
    GetLolGameflowV1EarlyExitNotificationsMissions {
        
    }
}


pub struct DeleteLolGameflowV1EarlyExitNotificationsMissions {

}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsMissions {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/early-exit-notifications/missions".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_gameflow_v_1_early_exit_notifications_missions() -> DeleteLolGameflowV1EarlyExitNotificationsMissions {
    DeleteLolGameflowV1EarlyExitNotificationsMissions {
        
    }
}


pub struct DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey {

    pub key: i32,
}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-gameflow/v1/early-exit-notifications/missions/{}", self.key)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_gameflow_v_1_early_exit_notifications_missions_by_key(key: i32) -> DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey {
    DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey {
        key
    }
}


pub struct GetLolGameflowV1ActivePatcherLock {

}

impl IsApiRequest for GetLolGameflowV1ActivePatcherLock {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/active-patcher-lock".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_active_patcher_lock() -> GetLolGameflowV1ActivePatcherLock {
    GetLolGameflowV1ActivePatcherLock {
        
    }
}


pub struct GetLolGameflowV1Availability {

}

impl IsApiRequest for GetLolGameflowV1Availability {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowAvailability;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/availability".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_availability() -> GetLolGameflowV1Availability {
    GetLolGameflowV1Availability {
        
    }
}


pub struct GetLolGameflowV1BasicTutorial {

}

impl IsApiRequest for GetLolGameflowV1BasicTutorial {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/basic-tutorial".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_basic_tutorial() -> GetLolGameflowV1BasicTutorial {
    GetLolGameflowV1BasicTutorial {
        
    }
}


pub struct GetLolGameflowV1BattleTraining {

}

impl IsApiRequest for GetLolGameflowV1BattleTraining {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/battle-training".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_battle_training() -> GetLolGameflowV1BattleTraining {
    GetLolGameflowV1BattleTraining {
        
    }
}


pub struct GetLolGameflowV1EarlyExitEnabled {

}

impl IsApiRequest for GetLolGameflowV1EarlyExitEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/early-exit-enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_early_exit_enabled() -> GetLolGameflowV1EarlyExitEnabled {
    GetLolGameflowV1EarlyExitEnabled {
        
    }
}


pub struct GetLolGameflowV1EarlyExitQuitEnabled {

}

impl IsApiRequest for GetLolGameflowV1EarlyExitQuitEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/early-exit-quit-enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_early_exit_quit_enabled() -> GetLolGameflowV1EarlyExitQuitEnabled {
    GetLolGameflowV1EarlyExitQuitEnabled {
        
    }
}


pub struct GetLolGameflowV1ExtraGameClientArgs {

}

impl IsApiRequest for GetLolGameflowV1ExtraGameClientArgs {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/extra-game-client-args".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_extra_game_client_args() -> GetLolGameflowV1ExtraGameClientArgs {
    GetLolGameflowV1ExtraGameClientArgs {
        
    }
}


pub struct PostLolGameflowV1ExtraGameClientArgs {

    pub body: Vec<String>,
}

impl IsApiRequest for PostLolGameflowV1ExtraGameClientArgs {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/extra-game-client-args".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_extra_game_client_args(body: Vec<String>) -> PostLolGameflowV1ExtraGameClientArgs {
    PostLolGameflowV1ExtraGameClientArgs {
        body
    }
}


pub struct GetLolGameflowV1GameExitEarlyVanguard {

}

impl IsApiRequest for GetLolGameflowV1GameExitEarlyVanguard {
    const METHOD: Method = Method::GET;
    type ReturnType = u64;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/game-exit-early-vanguard".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_game_exit_early_vanguard() -> GetLolGameflowV1GameExitEarlyVanguard {
    GetLolGameflowV1GameExitEarlyVanguard {
        
    }
}


pub struct GetLolGameflowV1GameflowMetadataPlayerStatus {

}

impl IsApiRequest for GetLolGameflowV1GameflowMetadataPlayerStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowPlayerStatus;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/gameflow-metadata/player-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_gameflow_metadata_player_status() -> GetLolGameflowV1GameflowMetadataPlayerStatus {
    GetLolGameflowV1GameflowMetadataPlayerStatus {
        
    }
}


pub struct PostLolGameflowV1GameflowMetadataPlayerStatus {

    pub body: LolGameflowPlayerStatus,
}

impl IsApiRequest for PostLolGameflowV1GameflowMetadataPlayerStatus {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/gameflow-metadata/player-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_gameflow_metadata_player_status(body: LolGameflowPlayerStatus) -> PostLolGameflowV1GameflowMetadataPlayerStatus {
    PostLolGameflowV1GameflowMetadataPlayerStatus {
        body
    }
}


pub struct GetLolGameflowV1GameflowMetadataRegistrationStatus {

}

impl IsApiRequest for GetLolGameflowV1GameflowMetadataRegistrationStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowRegistrationStatus;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/gameflow-metadata/registration-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_gameflow_metadata_registration_status() -> GetLolGameflowV1GameflowMetadataRegistrationStatus {
    GetLolGameflowV1GameflowMetadataRegistrationStatus {
        
    }
}


pub struct PostLolGameflowV1GameflowMetadataRegistrationStatus {

    pub body: LolGameflowRegistrationStatus,
}

impl IsApiRequest for PostLolGameflowV1GameflowMetadataRegistrationStatus {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/gameflow-metadata/registration-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_gameflow_metadata_registration_status(body: LolGameflowRegistrationStatus) -> PostLolGameflowV1GameflowMetadataRegistrationStatus {
    PostLolGameflowV1GameflowMetadataRegistrationStatus {
        body
    }
}


pub struct GetLolGameflowV1GameflowPhase {

}

impl IsApiRequest for GetLolGameflowV1GameflowPhase {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowPhase;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/gameflow-phase".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_gameflow_phase() -> GetLolGameflowV1GameflowPhase {
    GetLolGameflowV1GameflowPhase {
        
    }
}


pub struct GetLolGameflowV1PlayerKickedVanguard {

}

impl IsApiRequest for GetLolGameflowV1PlayerKickedVanguard {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/player-kicked-vanguard".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_player_kicked_vanguard() -> GetLolGameflowV1PlayerKickedVanguard {
    GetLolGameflowV1PlayerKickedVanguard {
        
    }
}


pub struct GetLolGameflowV1Session {

}

impl IsApiRequest for GetLolGameflowV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowSession;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_session() -> GetLolGameflowV1Session {
    GetLolGameflowV1Session {
        
    }
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed {

}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameModeSpellList;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/per-position-summoner-spells/disallowed".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_session_per_position_summoner_spells_disallowed() -> GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed {
    GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed {
        
    }
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString {

}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/per-position-summoner-spells/disallowed/as-string".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_session_per_position_summoner_spells_disallowed_as_string() -> GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString {
    GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString {
        
    }
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsRequired {

}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsRequired {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameModeSpellList;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/per-position-summoner-spells/required".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_session_per_position_summoner_spells_required() -> GetLolGameflowV1SessionPerPositionSummonerSpellsRequired {
    GetLolGameflowV1SessionPerPositionSummonerSpellsRequired {
        
    }
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString {

}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/per-position-summoner-spells/required/as-string".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_session_per_position_summoner_spells_required_as_string() -> GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString {
    GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString {
        
    }
}


pub struct GetLolGameflowV1Spectate {

}

impl IsApiRequest for GetLolGameflowV1Spectate {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/spectate".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_spectate() -> GetLolGameflowV1Spectate {
    GetLolGameflowV1Spectate {
        
    }
}


pub struct GetLolGameflowV1SpectateDelayedLaunch {

}

impl IsApiRequest for GetLolGameflowV1SpectateDelayedLaunch {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/spectate/delayed-launch".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_spectate_delayed_launch() -> GetLolGameflowV1SpectateDelayedLaunch {
    GetLolGameflowV1SpectateDelayedLaunch {
        
    }
}


pub struct GetLolGameflowV1Watch {

}

impl IsApiRequest for GetLolGameflowV1Watch {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowWatchPhase;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/watch".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_gameflow_v_1_watch() -> GetLolGameflowV1Watch {
    GetLolGameflowV1Watch {
        
    }
}


pub struct PostLolGameflowV1AckFailedToLaunch {

}

impl IsApiRequest for PostLolGameflowV1AckFailedToLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/ack-failed-to-launch".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_ack_failed_to_launch() -> PostLolGameflowV1AckFailedToLaunch {
    PostLolGameflowV1AckFailedToLaunch {
        
    }
}


pub struct PostLolGameflowV1BasicTutorialStart {

}

impl IsApiRequest for PostLolGameflowV1BasicTutorialStart {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/basic-tutorial/start".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_basic_tutorial_start() -> PostLolGameflowV1BasicTutorialStart {
    PostLolGameflowV1BasicTutorialStart {
        
    }
}


pub struct PostLolGameflowV1BattleTrainingStart {

}

impl IsApiRequest for PostLolGameflowV1BattleTrainingStart {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/battle-training/start".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_battle_training_start() -> PostLolGameflowV1BattleTrainingStart {
    PostLolGameflowV1BattleTrainingStart {
        
    }
}


pub struct PostLolGameflowV1BattleTrainingStop {

}

impl IsApiRequest for PostLolGameflowV1BattleTrainingStop {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/battle-training/stop".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_battle_training_stop() -> PostLolGameflowV1BattleTrainingStop {
    PostLolGameflowV1BattleTrainingStop {
        
    }
}


pub struct PostLolGameflowV1EarlyExit {

}

impl IsApiRequest for PostLolGameflowV1EarlyExit {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/early-exit".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_early_exit() -> PostLolGameflowV1EarlyExit {
    PostLolGameflowV1EarlyExit {
        
    }
}


pub struct PostLolGameflowV1PreEndGameTransition {

    pub body: bool,
}

impl IsApiRequest for PostLolGameflowV1PreEndGameTransition {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/pre-end-game-transition".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_pre_end_game_transition(body: bool) -> PostLolGameflowV1PreEndGameTransition {
    PostLolGameflowV1PreEndGameTransition {
        body
    }
}


pub struct PostLolGameflowV1Reconnect {

}

impl IsApiRequest for PostLolGameflowV1Reconnect {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/reconnect".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_reconnect() -> PostLolGameflowV1Reconnect {
    PostLolGameflowV1Reconnect {
        
    }
}


pub struct PostLolGameflowV1SessionChampSelectPhaseTimeRemaining {

    pub body: u64,
}

impl IsApiRequest for PostLolGameflowV1SessionChampSelectPhaseTimeRemaining {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/champ-select/phase-time-remaining".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_champ_select_phase_time_remaining(body: u64) -> PostLolGameflowV1SessionChampSelectPhaseTimeRemaining {
    PostLolGameflowV1SessionChampSelectPhaseTimeRemaining {
        body
    }
}


pub struct PostLolGameflowV1SessionDodge {

    pub body: LolGameflowGameflowGameDodge,
}

impl IsApiRequest for PostLolGameflowV1SessionDodge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/dodge".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_dodge(body: LolGameflowGameflowGameDodge) -> PostLolGameflowV1SessionDodge {
    PostLolGameflowV1SessionDodge {
        body
    }
}


pub struct PostLolGameflowV1SessionEvent {

    pub body: String,
}

impl IsApiRequest for PostLolGameflowV1SessionEvent {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/event".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_event(body: String) -> PostLolGameflowV1SessionEvent {
    PostLolGameflowV1SessionEvent {
        body
    }
}


pub struct PostLolGameflowV1SessionGameConfiguration {

    pub body: LolGameflowQueue,
}

impl IsApiRequest for PostLolGameflowV1SessionGameConfiguration {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/game-configuration".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_game_configuration(body: LolGameflowQueue) -> PostLolGameflowV1SessionGameConfiguration {
    PostLolGameflowV1SessionGameConfiguration {
        body
    }
}


pub struct PostLolGameflowV1SessionRequestEnterGameflow {

    pub body: String,
}

impl IsApiRequest for PostLolGameflowV1SessionRequestEnterGameflow {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/request-enter-gameflow".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_request_enter_gameflow(body: String) -> PostLolGameflowV1SessionRequestEnterGameflow {
    PostLolGameflowV1SessionRequestEnterGameflow {
        body
    }
}


pub struct PostLolGameflowV1SessionRequestLobby {

}

impl IsApiRequest for PostLolGameflowV1SessionRequestLobby {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/request-lobby".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_request_lobby() -> PostLolGameflowV1SessionRequestLobby {
    PostLolGameflowV1SessionRequestLobby {
        
    }
}


pub struct PostLolGameflowV1SessionRequestTournamentCheckin {

}

impl IsApiRequest for PostLolGameflowV1SessionRequestTournamentCheckin {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/request-tournament-checkin".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_request_tournament_checkin() -> PostLolGameflowV1SessionRequestTournamentCheckin {
    PostLolGameflowV1SessionRequestTournamentCheckin {
        
    }
}


pub struct PostLolGameflowV1SessionTournamentEnded {

}

impl IsApiRequest for PostLolGameflowV1SessionTournamentEnded {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/session/tournament-ended".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_session_tournament_ended() -> PostLolGameflowV1SessionTournamentEnded {
    PostLolGameflowV1SessionTournamentEnded {
        
    }
}


pub struct PostLolGameflowV1SpectateLaunch {

    pub body: String,
}

impl IsApiRequest for PostLolGameflowV1SpectateLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/spectate/launch".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_spectate_launch(body: String) -> PostLolGameflowV1SpectateLaunch {
    PostLolGameflowV1SpectateLaunch {
        body
    }
}


pub struct PostLolGameflowV1SpectateQuit {

}

impl IsApiRequest for PostLolGameflowV1SpectateQuit {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/spectate/quit".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_spectate_quit() -> PostLolGameflowV1SpectateQuit {
    PostLolGameflowV1SpectateQuit {
        
    }
}


pub struct PostLolGameflowV1Tick {

}

impl IsApiRequest for PostLolGameflowV1Tick {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/tick".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_tick() -> PostLolGameflowV1Tick {
    PostLolGameflowV1Tick {
        
    }
}


pub struct PostLolGameflowV1WatchLaunch {

    pub body: Vec<String>,
}

impl IsApiRequest for PostLolGameflowV1WatchLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v1/watch/launch".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_1_watch_launch(body: Vec<String>) -> PostLolGameflowV1WatchLaunch {
    PostLolGameflowV1WatchLaunch {
        body
    }
}


pub struct PostLolGameflowV2SpectateLaunch {

    pub body: LolGameflowSpectateGameInfoResource,
}

impl IsApiRequest for PostLolGameflowV2SpectateLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-gameflow/v2/spectate/launch".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_gameflow_v_2_spectate_launch(body: LolGameflowSpectateGameInfoResource) -> PostLolGameflowV2SpectateLaunch {
    PostLolGameflowV2SpectateLaunch {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowSession {
    pub phase: LolGameflowGameflowPhase,
    pub game_data: LolGameflowGameflowGameData,
    pub game_client: LolGameflowGameflowGameClient,
    pub map: LolGameflowGameflowGameMap,
    pub game_dodge: LolGameflowGameflowGameDodge,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameMap {
    pub id: i64,
    pub name: String,
    pub map_string_id: String,
    pub game_mode: String,
    pub game_mode_name: String,
    pub game_mode_short_name: String,
    pub game_mutator: String,
    pub is_rgm: bool,
    pub description: String,
    pub platform_id: String,
    pub platform_name: String,
    pub assets: HashMap<String, String>,
    pub categorized_content_bundles: HashMap<String, String>,
    pub properties: HashMap<String, String>,
    pub per_position_required_summoner_spells: LolGameflowGameModeSpellList,
    pub per_position_disallowed_summoner_spells: LolGameflowGameModeSpellList,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowRegistrationStatus {
    pub complete: bool,
    pub error_codes: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameData {
    pub game_id: u64,
    pub queue: LolGameflowQueue,
    pub is_custom_game: bool,
    pub game_name: String,
    pub password: String,
    pub team_one: Vec<HashMap<String, String>>,
    pub team_two: Vec<HashMap<String, String>>,
    pub player_champion_selections: Vec<HashMap<String, String>>,
    pub spectators_allowed: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameDodge {
    pub state: LolGameflowGameflowGameDodgeState,
    pub dodge_ids: Vec<u64>,
    pub phase: LolGameflowGameflowPhase,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowPlayerStatus {
    pub current_lobby_status: Option<LolGameflowLobbyStatus>,
    pub last_queued_lobby_status: Option<LolGameflowLobbyStatus>,
    pub can_invite_others_at_eog: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameClient {
    pub server_ip: String,
    pub server_port: u16,
    pub observer_server_ip: String,
    pub observer_server_port: u16,
    pub running: bool,
    pub visible: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueue {
    pub id: i32,
    pub map_id: i32,
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub detailed_description: String,
    pub type_: String,
    pub game_mode: String,
    pub asset_mutator: String,
    pub category: LolGameflowQueueGameCategory,
    pub game_type_config: LolGameflowQueueGameTypeConfig,
    pub num_players_per_team: i32,
    pub minimum_participant_list_size: i32,
    pub maximum_participant_list_size: i32,
    pub min_level: u32,
    pub is_ranked: bool,
    pub are_free_champions_allowed: bool,
    pub is_team_builder_managed: bool,
    pub queue_availability: LolGameflowQueueAvailability,
    pub queue_rewards: LolGameflowQueueReward,
    pub spectator_enabled: bool,
    pub champions_required_to_play: u32,
    pub allowable_premade_sizes: Vec<i32>,
    pub show_position_selector: bool,
    pub last_toggled_off_time: u64,
    pub last_toggled_on_time: u64,
    pub removal_from_game_allowed: bool,
    pub removal_from_game_delay_minutes: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowSpectateGameInfoResource {
    pub drop_in_spectate_game_id: String,
    pub game_queue_type: String,
    pub allow_observe_mode: String,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowAvailability {
    pub is_available: bool,
    pub state: LolGameflowGameflowAvailabilityState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameModeSpellList {
    pub spells: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueReward {
    pub is_ip_enabled: bool,
    pub is_xp_enabled: bool,
    pub is_champion_points_enabled: bool,
    pub party_size_ip_rewards: Vec<i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowLobbyStatus {
    pub queue_id: i32,
    pub is_custom: bool,
    pub is_practice_tool: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub allowed_play_again: bool,
    pub member_summoner_ids: Vec<u64>,
    pub invited_summoner_ids: Vec<u64>,
    pub lobby_id: Option<String>,
    pub custom_spectator_policy: LolGameflowQueueCustomGameSpectatorPolicy,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowGameflowPhase {
    #[default]
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowGameflowAvailabilityState {
    #[default]
    EligibilityInfoMissing,
    Configuration,
    InGameFlow,
    PlayerBanned,
    Patching,
    Initializing,
    Available,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowQueueCustomGameSpectatorPolicy {
    #[default]
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowQueueGameCategory {
    #[default]
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowGameflowGameDodgeState {
    #[default]
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowQueueAvailability {
    #[default]
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowGameflowWatchPhase {
    #[default]
    WatchFailedToLaunch,
    WatchInProgress,
    WatchStarted,
    None,
}

