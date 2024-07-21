
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct DeleteLolPatchV1NotificationsById {

    pub id: String,
}

impl IsApiRequest for DeleteLolPatchV1NotificationsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-patch/v1/notifications/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_patch_v_1_notifications_by_id(id: String) -> DeleteLolPatchV1NotificationsById {
    DeleteLolPatchV1NotificationsById {
        id
    }
}


pub struct GetLolPatchV1CheckingEnabled {

}

impl IsApiRequest for GetLolPatchV1CheckingEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-patch/v1/checking-enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_checking_enabled() -> GetLolPatchV1CheckingEnabled {
    GetLolPatchV1CheckingEnabled {
        
    }
}


pub struct GetLolPatchV1Environment {

}

impl IsApiRequest for GetLolPatchV1Environment {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPatchChunkingPatcherEnvironment;

    fn get_url(&self) -> String {
        "/lol-patch/v1/environment".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_environment() -> GetLolPatchV1Environment {
    GetLolPatchV1Environment {
        
    }
}


pub struct GetLolPatchV1GameVersion {

}

impl IsApiRequest for GetLolPatchV1GameVersion {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-patch/v1/game-version".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_game_version() -> GetLolPatchV1GameVersion {
    GetLolPatchV1GameVersion {
        
    }
}


pub struct GetLolPatchV1Notifications {

}

impl IsApiRequest for GetLolPatchV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPatchNotification>;

    fn get_url(&self) -> String {
        "/lol-patch/v1/notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_notifications() -> GetLolPatchV1Notifications {
    GetLolPatchV1Notifications {
        
    }
}


pub struct GetLolPatchV1ProductsLeagueOfLegendsInstallLocation {

}

impl IsApiRequest for GetLolPatchV1ProductsLeagueOfLegendsInstallLocation {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPatchInstallPaths;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/install-location".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_products_league_of_legends_install_location() -> GetLolPatchV1ProductsLeagueOfLegendsInstallLocation {
    GetLolPatchV1ProductsLeagueOfLegendsInstallLocation {
        
    }
}


pub struct GetLolPatchV1ProductsLeagueOfLegendsState {

}

impl IsApiRequest for GetLolPatchV1ProductsLeagueOfLegendsState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPatchProductState;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/state".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_products_league_of_legends_state() -> GetLolPatchV1ProductsLeagueOfLegendsState {
    GetLolPatchV1ProductsLeagueOfLegendsState {
        
    }
}


pub struct GetLolPatchV1ProductsLeagueOfLegendsSupportedGameReleases {

}

impl IsApiRequest for GetLolPatchV1ProductsLeagueOfLegendsSupportedGameReleases {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPatchSupportedGameReleases;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/supported-game-releases".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_products_league_of_legends_supported_game_releases() -> GetLolPatchV1ProductsLeagueOfLegendsSupportedGameReleases {
    GetLolPatchV1ProductsLeagueOfLegendsSupportedGameReleases {
        
    }
}


pub struct GetLolPatchV1Status {

}

impl IsApiRequest for GetLolPatchV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPatchStatus;

    fn get_url(&self) -> String {
        "/lol-patch/v1/status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_patch_v_1_status() -> GetLolPatchV1Status {
    GetLolPatchV1Status {
        
    }
}


pub struct PostLolPatchV1ProductsLeagueOfLegendsDetectCorruptionRequest {

}

impl IsApiRequest for PostLolPatchV1ProductsLeagueOfLegendsDetectCorruptionRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/detect-corruption-request".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_patch_v_1_products_league_of_legends_detect_corruption_request() -> PostLolPatchV1ProductsLeagueOfLegendsDetectCorruptionRequest {
    PostLolPatchV1ProductsLeagueOfLegendsDetectCorruptionRequest {
        
    }
}


pub struct PostLolPatchV1ProductsLeagueOfLegendsPartialRepairRequest {

}

impl IsApiRequest for PostLolPatchV1ProductsLeagueOfLegendsPartialRepairRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/partial-repair-request".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_patch_v_1_products_league_of_legends_partial_repair_request() -> PostLolPatchV1ProductsLeagueOfLegendsPartialRepairRequest {
    PostLolPatchV1ProductsLeagueOfLegendsPartialRepairRequest {
        
    }
}


pub struct PostLolPatchV1ProductsLeagueOfLegendsStartCheckingRequest {

}

impl IsApiRequest for PostLolPatchV1ProductsLeagueOfLegendsStartCheckingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/start-checking-request".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_patch_v_1_products_league_of_legends_start_checking_request() -> PostLolPatchV1ProductsLeagueOfLegendsStartCheckingRequest {
    PostLolPatchV1ProductsLeagueOfLegendsStartCheckingRequest {
        
    }
}


pub struct PostLolPatchV1ProductsLeagueOfLegendsStartPatchingRequest {

}

impl IsApiRequest for PostLolPatchV1ProductsLeagueOfLegendsStartPatchingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/start-patching-request".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_patch_v_1_products_league_of_legends_start_patching_request() -> PostLolPatchV1ProductsLeagueOfLegendsStartPatchingRequest {
    PostLolPatchV1ProductsLeagueOfLegendsStartPatchingRequest {
        
    }
}


pub struct PostLolPatchV1ProductsLeagueOfLegendsStopCheckingRequest {

}

impl IsApiRequest for PostLolPatchV1ProductsLeagueOfLegendsStopCheckingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/stop-checking-request".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_patch_v_1_products_league_of_legends_stop_checking_request() -> PostLolPatchV1ProductsLeagueOfLegendsStopCheckingRequest {
    PostLolPatchV1ProductsLeagueOfLegendsStopCheckingRequest {
        
    }
}


pub struct PostLolPatchV1ProductsLeagueOfLegendsStopPatchingRequest {

    pub body: bool,
}

impl IsApiRequest for PostLolPatchV1ProductsLeagueOfLegendsStopPatchingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/products/league_of_legends/stop-patching-request".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_patch_v_1_products_league_of_legends_stop_patching_request(body: bool) -> PostLolPatchV1ProductsLeagueOfLegendsStopPatchingRequest {
    PostLolPatchV1ProductsLeagueOfLegendsStopPatchingRequest {
        body
    }
}


pub struct PutLolPatchV1GamePatchUrl {

    pub body: String,
}

impl IsApiRequest for PutLolPatchV1GamePatchUrl {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/game-patch-url".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_patch_v_1_game_patch_url(body: String) -> PutLolPatchV1GamePatchUrl {
    PutLolPatchV1GamePatchUrl {
        body
    }
}


pub struct PutLolPatchV1Ux {

    pub body: LolPatchUxResource,
}

impl IsApiRequest for PutLolPatchV1Ux {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-patch/v1/ux".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_patch_v_1_ux(body: LolPatchUxResource) -> PutLolPatchV1Ux {
    PutLolPatchV1Ux {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchUxResource {
    pub visible: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentState {
    pub id: String,
    pub action: LolPatchComponentStateAction,
    pub is_up_to_date: bool,
    pub is_update_available: bool,
    pub time_of_last_up_to_date_check_iso_8601: String,
    pub is_corrupted: bool,
    pub progress: LolPatchComponentActionProgress,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchNotification {
    pub id: String,
    pub notification_id: LolPatchNotificationId,
    pub data: HashMap<String, HashMap<String, String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchStatus {
    pub connected_to_patch_server: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchInstallPaths {
    pub game_install_root: String,
    pub game_executable_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchSupportedGameRelease {
    pub artifact_id: String,
    pub download: LolPatchPatchSieveDownload,
    pub selected: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentActionProgress {
    pub current_item: String,
    pub total: LolPatchComponentStateProgress,
    pub network: LolPatchComponentStateProgress,
    pub primary_work: LolPatchComponentStateWorkType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchProductState {
    pub id: String,
    pub action: LolPatchComponentStateAction,
    pub is_up_to_date: bool,
    pub is_update_available: bool,
    pub is_corrupted: bool,
    pub is_stopped: bool,
    pub percent_patched: f64,
    pub components: Vec<LolPatchComponentState>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchPatchSieveDownload {
    pub url: String,
    pub scd_required: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchSupportedGameReleases {
    pub supported_game_releases: Vec<LolPatchSupportedGameRelease>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentStateProgress {
    pub bytes_complete: u64,
    pub bytes_required: u64,
    pub bytes_per_second: f64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchChunkingPatcherEnvironment {
    pub game_patcher_available: bool,
    pub game_patcher_enabled: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolPatchNotificationId {
    #[default]
    BrokenPermissions,
    NotEnoughDiskSpace,
    DidRestoreClientBackup,
    FailedToWriteError,
    MissingFilesError,
    ConnectionError,
    UnspecifiedError,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolPatchComponentStateAction {
    #[default]
    Migrating,
    Repairing,
    Patching,
    CheckingForUpdates,
    Idle,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolPatchComponentStateWorkType {
    #[default]
    Disk,
    Network,
    Scanning,
}

