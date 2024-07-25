
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolGameflowV1EarlyExitNotificationsEog {}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsEog {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/early-exit-notifications/eog".to_string()}
}

pub fn delete_lol_gameflow_v1_early_exit_notifications_eog() -> DeleteLolGameflowV1EarlyExitNotificationsEog {
    DeleteLolGameflowV1EarlyExitNotificationsEog{}
}


pub struct DeleteLolGameflowV1EarlyExitNotificationsEogByKey {
    pub key: i32,
}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsEogByKey {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-gameflow/v1/early-exit-notifications/eog/{}", self.key)}
}

pub fn delete_lol_gameflow_v1_early_exit_notifications_eog_by_key(key: i32) -> DeleteLolGameflowV1EarlyExitNotificationsEogByKey {
    DeleteLolGameflowV1EarlyExitNotificationsEogByKey{key}
}


pub struct DeleteLolGameflowV1EarlyExitNotificationsMissions {}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsMissions {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/early-exit-notifications/missions".to_string()}
}

pub fn delete_lol_gameflow_v1_early_exit_notifications_missions() -> DeleteLolGameflowV1EarlyExitNotificationsMissions {
    DeleteLolGameflowV1EarlyExitNotificationsMissions{}
}


pub struct DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey {
    pub key: i32,
}

impl IsApiRequest for DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-gameflow/v1/early-exit-notifications/missions/{}", self.key)}
}

pub fn delete_lol_gameflow_v1_early_exit_notifications_missions_by_key(key: i32) -> DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey {
    DeleteLolGameflowV1EarlyExitNotificationsMissionsByKey{key}
}


pub struct GetLolGameflowV1ActivePatcherLock {}

impl IsApiRequest for GetLolGameflowV1ActivePatcherLock {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/active-patcher-lock".to_string()}
}

pub fn get_lol_gameflow_v1_active_patcher_lock() -> GetLolGameflowV1ActivePatcherLock {
    GetLolGameflowV1ActivePatcherLock{}
}


pub struct GetLolGameflowV1Availability {}

impl IsApiRequest for GetLolGameflowV1Availability {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowAvailability;
    fn get_url(&self) -> String {"/lol-gameflow/v1/availability".to_string()}
}

pub fn get_lol_gameflow_v1_availability() -> GetLolGameflowV1Availability {
    GetLolGameflowV1Availability{}
}


pub struct GetLolGameflowV1BasicTutorial {}

impl IsApiRequest for GetLolGameflowV1BasicTutorial {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/basic-tutorial".to_string()}
}

pub fn get_lol_gameflow_v1_basic_tutorial() -> GetLolGameflowV1BasicTutorial {
    GetLolGameflowV1BasicTutorial{}
}


pub struct GetLolGameflowV1BattleTraining {}

impl IsApiRequest for GetLolGameflowV1BattleTraining {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/battle-training".to_string()}
}

pub fn get_lol_gameflow_v1_battle_training() -> GetLolGameflowV1BattleTraining {
    GetLolGameflowV1BattleTraining{}
}


pub struct GetLolGameflowV1EarlyExitEnabled {}

impl IsApiRequest for GetLolGameflowV1EarlyExitEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/early-exit-enabled".to_string()}
}

pub fn get_lol_gameflow_v1_early_exit_enabled() -> GetLolGameflowV1EarlyExitEnabled {
    GetLolGameflowV1EarlyExitEnabled{}
}


pub struct GetLolGameflowV1EarlyExitNotificationsEog {}

impl IsApiRequest for GetLolGameflowV1EarlyExitNotificationsEog {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<Value>;
    fn get_url(&self) -> String {"/lol-gameflow/v1/early-exit-notifications/eog".to_string()}
}

pub fn get_lol_gameflow_v1_early_exit_notifications_eog() -> GetLolGameflowV1EarlyExitNotificationsEog {
    GetLolGameflowV1EarlyExitNotificationsEog{}
}


pub struct GetLolGameflowV1EarlyExitNotificationsMissions {}

impl IsApiRequest for GetLolGameflowV1EarlyExitNotificationsMissions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<Value>;
    fn get_url(&self) -> String {"/lol-gameflow/v1/early-exit-notifications/missions".to_string()}
}

pub fn get_lol_gameflow_v1_early_exit_notifications_missions() -> GetLolGameflowV1EarlyExitNotificationsMissions {
    GetLolGameflowV1EarlyExitNotificationsMissions{}
}


pub struct GetLolGameflowV1EarlyExitQuitEnabled {}

impl IsApiRequest for GetLolGameflowV1EarlyExitQuitEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/early-exit-quit-enabled".to_string()}
}

pub fn get_lol_gameflow_v1_early_exit_quit_enabled() -> GetLolGameflowV1EarlyExitQuitEnabled {
    GetLolGameflowV1EarlyExitQuitEnabled{}
}


pub struct GetLolGameflowV1ExtraGameClientArgs {}

impl IsApiRequest for GetLolGameflowV1ExtraGameClientArgs {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/lol-gameflow/v1/extra-game-client-args".to_string()}
}

pub fn get_lol_gameflow_v1_extra_game_client_args() -> GetLolGameflowV1ExtraGameClientArgs {
    GetLolGameflowV1ExtraGameClientArgs{}
}


pub struct GetLolGameflowV1GameExitEarlyVanguard {}

impl IsApiRequest for GetLolGameflowV1GameExitEarlyVanguard {
    const METHOD: Method = Method::GET;
    type ReturnType = u64;
    fn get_url(&self) -> String {"/lol-gameflow/v1/game-exit-early-vanguard".to_string()}
}

pub fn get_lol_gameflow_v1_game_exit_early_vanguard() -> GetLolGameflowV1GameExitEarlyVanguard {
    GetLolGameflowV1GameExitEarlyVanguard{}
}


pub struct GetLolGameflowV1GameflowMetadataPlayerStatus {}

impl IsApiRequest for GetLolGameflowV1GameflowMetadataPlayerStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowPlayerStatus;
    fn get_url(&self) -> String {"/lol-gameflow/v1/gameflow-metadata/player-status".to_string()}
}

pub fn get_lol_gameflow_v1_gameflow_metadata_player_status() -> GetLolGameflowV1GameflowMetadataPlayerStatus {
    GetLolGameflowV1GameflowMetadataPlayerStatus{}
}


pub struct GetLolGameflowV1GameflowMetadataRegistrationStatus {}

impl IsApiRequest for GetLolGameflowV1GameflowMetadataRegistrationStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowRegistrationStatus;
    fn get_url(&self) -> String {"/lol-gameflow/v1/gameflow-metadata/registration-status".to_string()}
}

pub fn get_lol_gameflow_v1_gameflow_metadata_registration_status() -> GetLolGameflowV1GameflowMetadataRegistrationStatus {
    GetLolGameflowV1GameflowMetadataRegistrationStatus{}
}


pub struct GetLolGameflowV1GameflowPhase {}

impl IsApiRequest for GetLolGameflowV1GameflowPhase {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowPhase;
    fn get_url(&self) -> String {"/lol-gameflow/v1/gameflow-phase".to_string()}
}

pub fn get_lol_gameflow_v1_gameflow_phase() -> GetLolGameflowV1GameflowPhase {
    GetLolGameflowV1GameflowPhase{}
}


pub struct GetLolGameflowV1PlayerKickedVanguard {}

impl IsApiRequest for GetLolGameflowV1PlayerKickedVanguard {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/player-kicked-vanguard".to_string()}
}

pub fn get_lol_gameflow_v1_player_kicked_vanguard() -> GetLolGameflowV1PlayerKickedVanguard {
    GetLolGameflowV1PlayerKickedVanguard{}
}


pub struct GetLolGameflowV1Session {}

impl IsApiRequest for GetLolGameflowV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowSession;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session".to_string()}
}

pub fn get_lol_gameflow_v1_session() -> GetLolGameflowV1Session {
    GetLolGameflowV1Session{}
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed {}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, LolGameflowGameModeSpellList>;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/per-position-summoner-spells/disallowed".to_string()}
}

pub fn get_lol_gameflow_v1_session_per_position_summoner_spells_disallowed() -> GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed {
    GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowed{}
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString {}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/per-position-summoner-spells/disallowed/as-string".to_string()}
}

pub fn get_lol_gameflow_v1_session_per_position_summoner_spells_disallowed_as_string() -> GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString {
    GetLolGameflowV1SessionPerPositionSummonerSpellsDisallowedAsString{}
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsRequired {}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsRequired {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, LolGameflowGameModeSpellList>;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/per-position-summoner-spells/required".to_string()}
}

pub fn get_lol_gameflow_v1_session_per_position_summoner_spells_required() -> GetLolGameflowV1SessionPerPositionSummonerSpellsRequired {
    GetLolGameflowV1SessionPerPositionSummonerSpellsRequired{}
}


pub struct GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString {}

impl IsApiRequest for GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/per-position-summoner-spells/required/as-string".to_string()}
}

pub fn get_lol_gameflow_v1_session_per_position_summoner_spells_required_as_string() -> GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString {
    GetLolGameflowV1SessionPerPositionSummonerSpellsRequiredAsString{}
}


pub struct GetLolGameflowV1Spectate {}

impl IsApiRequest for GetLolGameflowV1Spectate {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/spectate".to_string()}
}

pub fn get_lol_gameflow_v1_spectate() -> GetLolGameflowV1Spectate {
    GetLolGameflowV1Spectate{}
}


pub struct GetLolGameflowV1SpectateDelayedLaunch {}

impl IsApiRequest for GetLolGameflowV1SpectateDelayedLaunch {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/spectate/delayed-launch".to_string()}
}

pub fn get_lol_gameflow_v1_spectate_delayed_launch() -> GetLolGameflowV1SpectateDelayedLaunch {
    GetLolGameflowV1SpectateDelayedLaunch{}
}


pub struct GetLolGameflowV1Watch {}

impl IsApiRequest for GetLolGameflowV1Watch {
    const METHOD: Method = Method::GET;
    type ReturnType = LolGameflowGameflowWatchPhase;
    fn get_url(&self) -> String {"/lol-gameflow/v1/watch".to_string()}
}

pub fn get_lol_gameflow_v1_watch() -> GetLolGameflowV1Watch {
    GetLolGameflowV1Watch{}
}


pub struct PostLolGameflowV1AckFailedToLaunch {}

impl IsApiRequest for PostLolGameflowV1AckFailedToLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/ack-failed-to-launch".to_string()}
}

pub fn post_lol_gameflow_v1_ack_failed_to_launch() -> PostLolGameflowV1AckFailedToLaunch {
    PostLolGameflowV1AckFailedToLaunch{}
}


pub struct PostLolGameflowV1BasicTutorialStart {}

impl IsApiRequest for PostLolGameflowV1BasicTutorialStart {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/basic-tutorial/start".to_string()}
}

pub fn post_lol_gameflow_v1_basic_tutorial_start() -> PostLolGameflowV1BasicTutorialStart {
    PostLolGameflowV1BasicTutorialStart{}
}


pub struct PostLolGameflowV1BattleTrainingStart {}

impl IsApiRequest for PostLolGameflowV1BattleTrainingStart {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/battle-training/start".to_string()}
}

pub fn post_lol_gameflow_v1_battle_training_start() -> PostLolGameflowV1BattleTrainingStart {
    PostLolGameflowV1BattleTrainingStart{}
}


pub struct PostLolGameflowV1BattleTrainingStop {}

impl IsApiRequest for PostLolGameflowV1BattleTrainingStop {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/battle-training/stop".to_string()}
}

pub fn post_lol_gameflow_v1_battle_training_stop() -> PostLolGameflowV1BattleTrainingStop {
    PostLolGameflowV1BattleTrainingStop{}
}


pub struct PostLolGameflowV1EarlyExit {}

impl IsApiRequest for PostLolGameflowV1EarlyExit {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/early-exit".to_string()}
}

pub fn post_lol_gameflow_v1_early_exit() -> PostLolGameflowV1EarlyExit {
    PostLolGameflowV1EarlyExit{}
}


pub struct PostLolGameflowV1ExtraGameClientArgs {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolGameflowV1ExtraGameClientArgs {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/extra-game-client-args".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_extra_game_client_args(body: Vec<String>) -> PostLolGameflowV1ExtraGameClientArgs {
    PostLolGameflowV1ExtraGameClientArgs{body}
}


pub struct PostLolGameflowV1GameflowMetadataPlayerStatus {
    pub body: LolGameflowPlayerStatus,
}

impl IsApiRequest for PostLolGameflowV1GameflowMetadataPlayerStatus {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/gameflow-metadata/player-status".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_gameflow_metadata_player_status(body: LolGameflowPlayerStatus) -> PostLolGameflowV1GameflowMetadataPlayerStatus {
    PostLolGameflowV1GameflowMetadataPlayerStatus{body}
}


pub struct PostLolGameflowV1GameflowMetadataRegistrationStatus {
    pub body: LolGameflowRegistrationStatus,
}

impl IsApiRequest for PostLolGameflowV1GameflowMetadataRegistrationStatus {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/gameflow-metadata/registration-status".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_gameflow_metadata_registration_status(body: LolGameflowRegistrationStatus) -> PostLolGameflowV1GameflowMetadataRegistrationStatus {
    PostLolGameflowV1GameflowMetadataRegistrationStatus{body}
}


pub struct PostLolGameflowV1PreEndGameTransition {
    pub body: bool,
}

impl IsApiRequest for PostLolGameflowV1PreEndGameTransition {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/pre-end-game-transition".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_pre_end_game_transition(body: bool) -> PostLolGameflowV1PreEndGameTransition {
    PostLolGameflowV1PreEndGameTransition{body}
}


pub struct PostLolGameflowV1Reconnect {}

impl IsApiRequest for PostLolGameflowV1Reconnect {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/reconnect".to_string()}
}

pub fn post_lol_gameflow_v1_reconnect() -> PostLolGameflowV1Reconnect {
    PostLolGameflowV1Reconnect{}
}


pub struct PostLolGameflowV1SessionChampSelectPhaseTimeRemaining {
    pub body: u64,
}

impl IsApiRequest for PostLolGameflowV1SessionChampSelectPhaseTimeRemaining {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/champ-select/phase-time-remaining".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_session_champ_select_phase_time_remaining(body: u64) -> PostLolGameflowV1SessionChampSelectPhaseTimeRemaining {
    PostLolGameflowV1SessionChampSelectPhaseTimeRemaining{body}
}


pub struct PostLolGameflowV1SessionDodge {
    pub body: LolGameflowGameflowGameDodge,
}

impl IsApiRequest for PostLolGameflowV1SessionDodge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/dodge".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_session_dodge(body: LolGameflowGameflowGameDodge) -> PostLolGameflowV1SessionDodge {
    PostLolGameflowV1SessionDodge{body}
}


pub struct PostLolGameflowV1SessionEvent {
    pub body: String,
}

impl IsApiRequest for PostLolGameflowV1SessionEvent {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/event".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_session_event(body: String) -> PostLolGameflowV1SessionEvent {
    PostLolGameflowV1SessionEvent{body}
}


pub struct PostLolGameflowV1SessionGameConfiguration {
    pub body: LolGameflowQueue,
}

impl IsApiRequest for PostLolGameflowV1SessionGameConfiguration {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/game-configuration".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_session_game_configuration(body: LolGameflowQueue) -> PostLolGameflowV1SessionGameConfiguration {
    PostLolGameflowV1SessionGameConfiguration{body}
}


pub struct PostLolGameflowV1SessionRequestEnterGameflow {
    pub body: String,
}

impl IsApiRequest for PostLolGameflowV1SessionRequestEnterGameflow {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/request-enter-gameflow".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_session_request_enter_gameflow(body: String) -> PostLolGameflowV1SessionRequestEnterGameflow {
    PostLolGameflowV1SessionRequestEnterGameflow{body}
}


pub struct PostLolGameflowV1SessionRequestLobby {}

impl IsApiRequest for PostLolGameflowV1SessionRequestLobby {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/request-lobby".to_string()}
}

pub fn post_lol_gameflow_v1_session_request_lobby() -> PostLolGameflowV1SessionRequestLobby {
    PostLolGameflowV1SessionRequestLobby{}
}


pub struct PostLolGameflowV1SessionRequestTournamentCheckin {}

impl IsApiRequest for PostLolGameflowV1SessionRequestTournamentCheckin {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/request-tournament-checkin".to_string()}
}

pub fn post_lol_gameflow_v1_session_request_tournament_checkin() -> PostLolGameflowV1SessionRequestTournamentCheckin {
    PostLolGameflowV1SessionRequestTournamentCheckin{}
}


pub struct PostLolGameflowV1SessionTournamentEnded {}

impl IsApiRequest for PostLolGameflowV1SessionTournamentEnded {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/session/tournament-ended".to_string()}
}

pub fn post_lol_gameflow_v1_session_tournament_ended() -> PostLolGameflowV1SessionTournamentEnded {
    PostLolGameflowV1SessionTournamentEnded{}
}


pub struct PostLolGameflowV1SpectateLaunch {
    pub body: String,
}

impl IsApiRequest for PostLolGameflowV1SpectateLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/spectate/launch".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_spectate_launch(body: String) -> PostLolGameflowV1SpectateLaunch {
    PostLolGameflowV1SpectateLaunch{body}
}


pub struct PostLolGameflowV1SpectateQuit {}

impl IsApiRequest for PostLolGameflowV1SpectateQuit {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/spectate/quit".to_string()}
}

pub fn post_lol_gameflow_v1_spectate_quit() -> PostLolGameflowV1SpectateQuit {
    PostLolGameflowV1SpectateQuit{}
}


pub struct PostLolGameflowV1Tick {}

impl IsApiRequest for PostLolGameflowV1Tick {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/tick".to_string()}
}

pub fn post_lol_gameflow_v1_tick() -> PostLolGameflowV1Tick {
    PostLolGameflowV1Tick{}
}


pub struct PostLolGameflowV1WatchLaunch {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolGameflowV1WatchLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v1/watch/launch".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v1_watch_launch(body: Vec<String>) -> PostLolGameflowV1WatchLaunch {
    PostLolGameflowV1WatchLaunch{body}
}


pub struct PostLolGameflowV2SpectateLaunch {
    pub body: LolGameflowSpectateGameInfoResource,
}

impl IsApiRequest for PostLolGameflowV2SpectateLaunch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-gameflow/v2/spectate/launch".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_gameflow_v2_spectate_launch(body: LolGameflowSpectateGameInfoResource) -> PostLolGameflowV2SpectateLaunch {
    PostLolGameflowV2SpectateLaunch{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameModeSpellList {
    pub spells: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowAvailability {
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    pub state: LolGameflowGameflowAvailabilityState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameClient {
    #[serde(rename = "serverIp")]
    pub server_ip: String,
    #[serde(rename = "serverPort")]
    pub server_port: u16,
    #[serde(rename = "observerServerIp")]
    pub observer_server_ip: String,
    #[serde(rename = "observerServerPort")]
    pub observer_server_port: u16,
    pub running: bool,
    pub visible: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameData {
    #[serde(rename = "gameId")]
    pub game_id: u64,
    pub queue: LolGameflowQueue,
    #[serde(rename = "isCustomGame")]
    pub is_custom_game: bool,
    #[serde(rename = "gameName")]
    pub game_name: String,
    pub password: String,
    #[serde(rename = "teamOne")]
    pub team_one: Vec<Value>,
    #[serde(rename = "teamTwo")]
    pub team_two: Vec<Value>,
    #[serde(rename = "playerChampionSelections")]
    pub player_champion_selections: Vec<Value>,
    #[serde(rename = "spectatorsAllowed")]
    pub spectators_allowed: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameDodge {
    pub state: LolGameflowGameflowGameDodgeState,
    #[serde(rename = "dodgeIds")]
    pub dodge_ids: Vec<u64>,
    pub phase: LolGameflowGameflowPhase,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameMap {
    pub id: i64,
    pub name: String,
    #[serde(rename = "mapStringId")]
    pub map_string_id: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameModeName")]
    pub game_mode_name: String,
    #[serde(rename = "gameModeShortName")]
    pub game_mode_short_name: String,
    #[serde(rename = "gameMutator")]
    pub game_mutator: String,
    #[serde(rename = "isRGM")]
    pub is_rgm: bool,
    pub description: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "platformName")]
    pub platform_name: String,
    pub assets: Value,
    #[serde(rename = "categorizedContentBundles")]
    pub categorized_content_bundles: Value,
    pub properties: Value,
    #[serde(rename = "perPositionRequiredSummonerSpells")]
    pub per_position_required_summoner_spells: HashMap<String, LolGameflowGameModeSpellList>,
    #[serde(rename = "perPositionDisallowedSummonerSpells")]
    pub per_position_disallowed_summoner_spells: HashMap<String, LolGameflowGameModeSpellList>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowSession {
    pub phase: LolGameflowGameflowPhase,
    #[serde(rename = "gameData")]
    pub game_data: LolGameflowGameflowGameData,
    #[serde(rename = "gameClient")]
    pub game_client: LolGameflowGameflowGameClient,
    pub map: LolGameflowGameflowGameMap,
    #[serde(rename = "gameDodge")]
    pub game_dodge: LolGameflowGameflowGameDodge,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowLobbyStatus {
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    #[serde(rename = "isPracticeTool")]
    pub is_practice_tool: bool,
    #[serde(rename = "isLeader")]
    pub is_leader: bool,
    #[serde(rename = "isSpectator")]
    pub is_spectator: bool,
    #[serde(rename = "allowedPlayAgain")]
    pub allowed_play_again: bool,
    #[serde(rename = "memberSummonerIds")]
    pub member_summoner_ids: Vec<u64>,
    #[serde(rename = "invitedSummonerIds")]
    pub invited_summoner_ids: Vec<u64>,
    #[serde(rename = "lobbyId")]
    pub lobby_id: Option<String>,
    #[serde(rename = "customSpectatorPolicy")]
    pub custom_spectator_policy: LolGameflowQueueCustomGameSpectatorPolicy,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowPlayerStatus {
    #[serde(rename = "currentLobbyStatus")]
    pub current_lobby_status: Option<LolGameflowLobbyStatus>,
    #[serde(rename = "lastQueuedLobbyStatus")]
    pub last_queued_lobby_status: Option<LolGameflowLobbyStatus>,
    #[serde(rename = "canInviteOthersAtEog")]
    pub can_invite_others_at_eog: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueue {
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub description: String,
    #[serde(rename = "detailedDescription")]
    pub detailed_description: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "assetMutator")]
    pub asset_mutator: String,
    pub category: LolGameflowQueueGameCategory,
    #[serde(rename = "gameTypeConfig")]
    pub game_type_config: LolGameflowQueueGameTypeConfig,
    #[serde(rename = "numPlayersPerTeam")]
    pub num_players_per_team: i32,
    #[serde(rename = "minimumParticipantListSize")]
    pub minimum_participant_list_size: i32,
    #[serde(rename = "maximumParticipantListSize")]
    pub maximum_participant_list_size: i32,
    #[serde(rename = "minLevel")]
    pub min_level: u32,
    #[serde(rename = "isRanked")]
    pub is_ranked: bool,
    #[serde(rename = "areFreeChampionsAllowed")]
    pub are_free_champions_allowed: bool,
    #[serde(rename = "isTeamBuilderManaged")]
    pub is_team_builder_managed: bool,
    #[serde(rename = "queueAvailability")]
    pub queue_availability: LolGameflowQueueAvailability,
    #[serde(rename = "queueRewards")]
    pub queue_rewards: LolGameflowQueueReward,
    #[serde(rename = "spectatorEnabled")]
    pub spectator_enabled: bool,
    #[serde(rename = "championsRequiredToPlay")]
    pub champions_required_to_play: u32,
    #[serde(rename = "allowablePremadeSizes")]
    pub allowable_premade_sizes: Vec<i32>,
    #[serde(rename = "showPositionSelector")]
    pub show_position_selector: bool,
    #[serde(rename = "lastToggledOffTime")]
    pub last_toggled_off_time: u64,
    #[serde(rename = "lastToggledOnTime")]
    pub last_toggled_on_time: u64,
    #[serde(rename = "removalFromGameAllowed")]
    pub removal_from_game_allowed: bool,
    #[serde(rename = "removalFromGameDelayMinutes")]
    pub removal_from_game_delay_minutes: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    #[serde(rename = "maxAllowableBans")]
    pub max_allowable_bans: i32,
    #[serde(rename = "allowTrades")]
    pub allow_trades: bool,
    #[serde(rename = "exclusivePick")]
    pub exclusive_pick: bool,
    #[serde(rename = "duplicatePick")]
    pub duplicate_pick: bool,
    #[serde(rename = "teamChampionPool")]
    pub team_champion_pool: bool,
    #[serde(rename = "crossTeamChampionPool")]
    pub cross_team_champion_pool: bool,
    #[serde(rename = "advancedLearningQuests")]
    pub advanced_learning_quests: bool,
    #[serde(rename = "battleBoost")]
    pub battle_boost: bool,
    #[serde(rename = "deathMatch")]
    pub death_match: bool,
    #[serde(rename = "doNotRemove")]
    pub do_not_remove: bool,
    #[serde(rename = "learningQuests")]
    pub learning_quests: bool,
    #[serde(rename = "onboardCoopBeginner")]
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    #[serde(rename = "mainPickTimerDuration")]
    pub main_pick_timer_duration: i32,
    #[serde(rename = "postPickTimerDuration")]
    pub post_pick_timer_duration: i32,
    #[serde(rename = "banTimerDuration")]
    pub ban_timer_duration: i32,
    #[serde(rename = "pickMode")]
    pub pick_mode: String,
    #[serde(rename = "banMode")]
    pub ban_mode: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueReward {
    #[serde(rename = "isIpEnabled")]
    pub is_ip_enabled: bool,
    #[serde(rename = "isXpEnabled")]
    pub is_xp_enabled: bool,
    #[serde(rename = "isChampionPointsEnabled")]
    pub is_champion_points_enabled: bool,
    #[serde(rename = "partySizeIpRewards")]
    pub party_size_ip_rewards: Vec<i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowRegistrationStatus {
    pub complete: bool,
    #[serde(rename = "errorCodes")]
    pub error_codes: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowSpectateGameInfoResource {
    #[serde(rename = "dropInSpectateGameId")]
    pub drop_in_spectate_game_id: String,
    #[serde(rename = "gameQueueType")]
    pub game_queue_type: String,
    #[serde(rename = "allowObserveMode")]
    pub allow_observe_mode: String,
    pub puuid: String,
}


// ENUMS

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
pub enum LolGameflowGameflowGameDodgeState {
    #[default]
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}


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
pub enum LolGameflowGameflowWatchPhase {
    #[default]
    WatchFailedToLaunch,
    WatchInProgress,
    WatchStarted,
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolGameflowQueueAvailability {
    #[default]
    DoesntMeetRequirements,
    PlatformDisabled,
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

