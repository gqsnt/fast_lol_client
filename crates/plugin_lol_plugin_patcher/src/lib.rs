
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeletePatcherV1NotificationsById {
    pub id: String,
}

impl IsApiRequest for DeletePatcherV1NotificationsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/notifications/{}", self.id)}
}

pub fn delete_patcher_v1_notifications_by_id(id: String) -> DeletePatcherV1NotificationsById {
    DeletePatcherV1NotificationsById{id}
}


pub struct DeletePatcherV1ProductsByProductId {
    pub product_id: String,
}

impl IsApiRequest for DeletePatcherV1ProductsByProductId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}", self.product_id)}
}

pub fn delete_patcher_v1_products_by_product_id(product_id: String) -> DeletePatcherV1ProductsByProductId {
    DeletePatcherV1ProductsByProductId{product_id}
}


pub struct GetPatcherV1Notifications {}

impl IsApiRequest for GetPatcherV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PatcherNotification>;
    fn get_url(&self) -> String {"/patcher/v1/notifications".to_string()}
}

pub fn get_patcher_v1_notifications() -> GetPatcherV1Notifications {
    GetPatcherV1Notifications{}
}


pub struct GetPatcherV1P2pStatus {}

impl IsApiRequest for GetPatcherV1P2pStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = PatcherP2PStatus;
    fn get_url(&self) -> String {"/patcher/v1/p2p/status".to_string()}
}

pub fn get_patcher_v1_p2_p_status() -> GetPatcherV1P2pStatus {
    GetPatcherV1P2pStatus{}
}


pub struct GetPatcherV1Products {}

impl IsApiRequest for GetPatcherV1Products {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/patcher/v1/products".to_string()}
}

pub fn get_patcher_v1_products() -> GetPatcherV1Products {
    GetPatcherV1Products{}
}


pub struct GetPatcherV1ProductsByProductIdPaths {
    pub product_id: String,
}

impl IsApiRequest for GetPatcherV1ProductsByProductIdPaths {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/paths", self.product_id)}
}

pub fn get_patcher_v1_products_by_product_id_paths(product_id: String) -> GetPatcherV1ProductsByProductIdPaths {
    GetPatcherV1ProductsByProductIdPaths{product_id}
}


pub struct GetPatcherV1ProductsByProductIdState {
    pub product_id: String,
}

impl IsApiRequest for GetPatcherV1ProductsByProductIdState {
    const METHOD: Method = Method::GET;
    type ReturnType = PatcherProductState;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/state", self.product_id)}
}

pub fn get_patcher_v1_products_by_product_id_state(product_id: String) -> GetPatcherV1ProductsByProductIdState {
    GetPatcherV1ProductsByProductIdState{product_id}
}


pub struct GetPatcherV1ProductsByProductIdTags {
    pub product_id: String,
}

impl IsApiRequest for GetPatcherV1ProductsByProductIdTags {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/tags", self.product_id)}
}

pub fn get_patcher_v1_products_by_product_id_tags(product_id: String) -> GetPatcherV1ProductsByProductIdTags {
    GetPatcherV1ProductsByProductIdTags{product_id}
}


pub struct GetPatcherV1Status {}

impl IsApiRequest for GetPatcherV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = PatcherStatus;
    fn get_url(&self) -> String {"/patcher/v1/status".to_string()}
}

pub fn get_patcher_v1_status() -> GetPatcherV1Status {
    GetPatcherV1Status{}
}


pub struct PatchPatcherV1P2pStatus {
    pub body: PatcherP2PStatusUpdate,
}

impl IsApiRequest for PatchPatcherV1P2pStatus {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/patcher/v1/p2p/status".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_patcher_v1_p2_p_status(body: PatcherP2PStatusUpdate) -> PatchPatcherV1P2pStatus {
    PatchPatcherV1P2pStatus{body}
}


pub struct PostPatcherV1Notifications {
    pub body: PatcherNotificationId,
}

impl IsApiRequest for PostPatcherV1Notifications {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/patcher/v1/notifications".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_patcher_v1_notifications(body: PatcherNotificationId) -> PostPatcherV1Notifications {
    PostPatcherV1Notifications{body}
}


pub struct PostPatcherV1ProductsByProductIdDetectCorruptionRequest {
    pub product_id: String,
}

impl IsApiRequest for PostPatcherV1ProductsByProductIdDetectCorruptionRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = PatcherProductState;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/detect-corruption-request", self.product_id)}
}

pub fn post_patcher_v1_products_by_product_id_detect_corruption_request(product_id: String) -> PostPatcherV1ProductsByProductIdDetectCorruptionRequest {
    PostPatcherV1ProductsByProductIdDetectCorruptionRequest{product_id}
}


pub struct PostPatcherV1ProductsByProductIdPartialRepairRequest {
    pub product_id: String,
}

impl IsApiRequest for PostPatcherV1ProductsByProductIdPartialRepairRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/partial-repair-request", self.product_id)}
}

pub fn post_patcher_v1_products_by_product_id_partial_repair_request(product_id: String) -> PostPatcherV1ProductsByProductIdPartialRepairRequest {
    PostPatcherV1ProductsByProductIdPartialRepairRequest{product_id}
}


pub struct PostPatcherV1ProductsByProductIdSignalStartPatchingDelayed {
    pub product_id: String,
}

impl IsApiRequest for PostPatcherV1ProductsByProductIdSignalStartPatchingDelayed {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/signal-start-patching-delayed", self.product_id)}
}

pub fn post_patcher_v1_products_by_product_id_signal_start_patching_delayed(product_id: String) -> PostPatcherV1ProductsByProductIdSignalStartPatchingDelayed {
    PostPatcherV1ProductsByProductIdSignalStartPatchingDelayed{product_id}
}


pub struct PostPatcherV1ProductsByProductIdStartCheckingRequest {
    pub product_id: String,
}

impl IsApiRequest for PostPatcherV1ProductsByProductIdStartCheckingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/start-checking-request", self.product_id)}
}

pub fn post_patcher_v1_products_by_product_id_start_checking_request(product_id: String) -> PostPatcherV1ProductsByProductIdStartCheckingRequest {
    PostPatcherV1ProductsByProductIdStartCheckingRequest{product_id}
}


pub struct PostPatcherV1ProductsByProductIdStartPatchingRequest {
    pub product_id: String,
}

impl IsApiRequest for PostPatcherV1ProductsByProductIdStartPatchingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/start-patching-request", self.product_id)}
}

pub fn post_patcher_v1_products_by_product_id_start_patching_request(product_id: String) -> PostPatcherV1ProductsByProductIdStartPatchingRequest {
    PostPatcherV1ProductsByProductIdStartPatchingRequest{product_id}
}


pub struct PostPatcherV1ProductsByProductIdStopCheckingRequest {
    pub product_id: String,
}

impl IsApiRequest for PostPatcherV1ProductsByProductIdStopCheckingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/stop-checking-request", self.product_id)}
}

pub fn post_patcher_v1_products_by_product_id_stop_checking_request(product_id: String) -> PostPatcherV1ProductsByProductIdStopCheckingRequest {
    PostPatcherV1ProductsByProductIdStopCheckingRequest{product_id}
}


pub struct PostPatcherV1ProductsByProductIdStopPatchingRequest {
    pub product_id: String,
}

impl IsApiRequest for PostPatcherV1ProductsByProductIdStopPatchingRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/patcher/v1/products/{}/stop-patching-request", self.product_id)}
}

pub fn post_patcher_v1_products_by_product_id_stop_patching_request(product_id: String) -> PostPatcherV1ProductsByProductIdStopPatchingRequest {
    PostPatcherV1ProductsByProductIdStopPatchingRequest{product_id}
}


pub struct PutPatcherV1Ux {
    pub body: PatcherUxResource,
}

impl IsApiRequest for PutPatcherV1Ux {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/patcher/v1/ux".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_patcher_v1_ux(body: PatcherUxResource) -> PutPatcherV1Ux {
    PutPatcherV1Ux{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentActionProgress {
    #[serde(rename = "currentItem")]
    pub current_item: String,
    pub total: PatcherComponentStateProgress,
    pub network: PatcherComponentStateProgress,
    #[serde(rename = "primaryWork")]
    pub primary_work: PatcherComponentStateWorkType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentState {
    pub id: String,
    pub action: PatcherComponentStateAction,
    #[serde(rename = "isUpToDate")]
    pub is_up_to_date: bool,
    #[serde(rename = "isUpdateAvailable")]
    pub is_update_available: bool,
    #[serde(rename = "timeOfLastUpToDateCheckISO8601")]
    pub time_of_last_up_to_date_check_iso8601: Option<String>,
    #[serde(rename = "isCorrupted")]
    pub is_corrupted: bool,
    pub progress: Option<PatcherComponentActionProgress>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentStateProgress {
    #[serde(rename = "bytesComplete")]
    pub bytes_complete: u64,
    #[serde(rename = "bytesRequired")]
    pub bytes_required: u64,
    #[serde(rename = "bytesPerSecond")]
    pub bytes_per_second: f64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherNotification {
    pub id: String,
    #[serde(rename = "notificationId")]
    pub notification_id: PatcherNotificationId,
    pub data: HashMap<String, Value>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherP2PStatus {
    #[serde(rename = "isEnabledForPatchline")]
    pub is_enabled_for_patchline: bool,
    #[serde(rename = "isAllowedByUser")]
    pub is_allowed_by_user: bool,
    #[serde(rename = "requiresRestart")]
    pub requires_restart: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherP2PStatusUpdate {
    #[serde(rename = "isAllowedByUser")]
    pub is_allowed_by_user: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherProductState {
    pub id: String,
    pub action: PatcherComponentStateAction,
    #[serde(rename = "isUpToDate")]
    pub is_up_to_date: bool,
    #[serde(rename = "isUpdateAvailable")]
    pub is_update_available: bool,
    #[serde(rename = "isCorrupted")]
    pub is_corrupted: bool,
    #[serde(rename = "isStopped")]
    pub is_stopped: bool,
    #[serde(rename = "percentPatched")]
    pub percent_patched: f64,
    pub components: Vec<PatcherComponentState>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherStatus {
    #[serde(rename = "connectedToPatchServer")]
    pub connected_to_patch_server: bool,
    #[serde(rename = "successfullyInstalledVersion")]
    pub successfully_installed_version: Option<u32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatcherUxResource {
    pub visible: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PatcherComponentStateAction {
    #[default]
    Migrating,
    Repairing,
    Patching,
    CheckingForUpdates,
    Idle,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PatcherComponentStateWorkType {
    #[default]
    Disk,
    Network,
    Scanning,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PatcherNotificationId {
    #[default]
    BrokenPermissions,
    NotEnoughDiskSpace,
    DidRestoreClientBackup,
    FailedToWriteError,
    MissingFilesError,
    ConnectionError,
    UnspecifiedError,
}

