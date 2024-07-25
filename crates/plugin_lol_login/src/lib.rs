
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName {
    pub service_name: String,
    pub method_name: String,
    pub plugin_id: u32,
}

impl IsApiRequest for DeleteLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-login/v1/service-proxy-async-requests/{}/{}", self.service_name, self.method_name)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("pluginId".to_string(), serde_json::to_string(&self.plugin_id).unwrap())
        ])
    }
}

pub fn delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(service_name: String, method_name: String, plugin_id: u32) -> DeleteLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName {
    DeleteLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName{service_name, method_name, plugin_id}
}


pub struct DeleteLolLoginV1Session {}

impl IsApiRequest for DeleteLolLoginV1Session {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/session".to_string()}
}

pub fn delete_lol_login_v1_session() -> DeleteLolLoginV1Session {
    DeleteLolLoginV1Session{}
}


pub struct DeleteLolLoginV1ShutdownLocksByLockName {
    pub lock_name: String,
}

impl IsApiRequest for DeleteLolLoginV1ShutdownLocksByLockName {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-login/v1/shutdown-locks/{}", self.lock_name)}
}

pub fn delete_lol_login_v1_shutdown_locks_by_lock_name(lock_name: String) -> DeleteLolLoginV1ShutdownLocksByLockName {
    DeleteLolLoginV1ShutdownLocksByLockName{lock_name}
}


pub struct GetLolLoginV1AccountState {}

impl IsApiRequest for GetLolLoginV1AccountState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoginAccountStateResource;
    fn get_url(&self) -> String {"/lol-login/v1/account-state".to_string()}
}

pub fn get_lol_login_v1_account_state() -> GetLolLoginV1AccountState {
    GetLolLoginV1AccountState{}
}


pub struct GetLolLoginV1LoginConnectionState {}

impl IsApiRequest for GetLolLoginV1LoginConnectionState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoginLoginConnectionState;
    fn get_url(&self) -> String {"/lol-login/v1/login-connection-state".to_string()}
}

pub fn get_lol_login_v1_login_connection_state() -> GetLolLoginV1LoginConnectionState {
    GetLolLoginV1LoginConnectionState{}
}


pub struct GetLolLoginV1LoginDataPacket {}

impl IsApiRequest for GetLolLoginV1LoginDataPacket {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/login-data-packet".to_string()}
}

pub fn get_lol_login_v1_login_data_packet() -> GetLolLoginV1LoginDataPacket {
    GetLolLoginV1LoginDataPacket{}
}


pub struct GetLolLoginV1LoginInGameCreds {}

impl IsApiRequest for GetLolLoginV1LoginInGameCreds {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/login-in-game-creds".to_string()}
}

pub fn get_lol_login_v1_login_in_game_creds() -> GetLolLoginV1LoginInGameCreds {
    GetLolLoginV1LoginInGameCreds{}
}


pub struct GetLolLoginV1LoginPlatformCredentials {}

impl IsApiRequest for GetLolLoginV1LoginPlatformCredentials {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoginPlatformGeneratedCredentials;
    fn get_url(&self) -> String {"/lol-login/v1/login-platform-credentials".to_string()}
}

pub fn get_lol_login_v1_login_platform_credentials() -> GetLolLoginV1LoginPlatformCredentials {
    GetLolLoginV1LoginPlatformCredentials{}
}


pub struct GetLolLoginV1LoginQueueState {}

impl IsApiRequest for GetLolLoginV1LoginQueueState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoginLoginQueue;
    fn get_url(&self) -> String {"/lol-login/v1/login-queue-state".to_string()}
}

pub fn get_lol_login_v1_login_queue_state() -> GetLolLoginV1LoginQueueState {
    GetLolLoginV1LoginQueueState{}
}


pub struct GetLolLoginV1Session {}

impl IsApiRequest for GetLolLoginV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoginLoginSession;
    fn get_url(&self) -> String {"/lol-login/v1/session".to_string()}
}

pub fn get_lol_login_v1_session() -> GetLolLoginV1Session {
    GetLolLoginV1Session{}
}


pub struct GetLolLoginV1Wallet {}

impl IsApiRequest for GetLolLoginV1Wallet {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoginLoginSessionWallet;
    fn get_url(&self) -> String {"/lol-login/v1/wallet".to_string()}
}

pub fn get_lol_login_v1_wallet() -> GetLolLoginV1Wallet {
    GetLolLoginV1Wallet{}
}


pub struct GetLolLoginV2LeagueSessionInitToken {}

impl IsApiRequest for GetLolLoginV2LeagueSessionInitToken {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoginLeagueSessionTokenEnvelope;
    fn get_url(&self) -> String {"/lol-login/v2/league-session-init-token".to_string()}
}

pub fn get_lol_login_v2_league_session_init_token() -> GetLolLoginV2LeagueSessionInitToken {
    GetLolLoginV2LeagueSessionInitToken{}
}


pub struct PostLolLoginV1AccountState {}

impl IsApiRequest for PostLolLoginV1AccountState {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/account-state".to_string()}
}

pub fn post_lol_login_v1_account_state() -> PostLolLoginV1AccountState {
    PostLolLoginV1AccountState{}
}


pub struct PostLolLoginV1ChangeSummonerName {
    pub body: String,
}

impl IsApiRequest for PostLolLoginV1ChangeSummonerName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/change-summoner-name".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_login_v1_change_summoner_name(body: String) -> PostLolLoginV1ChangeSummonerName {
    PostLolLoginV1ChangeSummonerName{body}
}


pub struct PostLolLoginV1DeleteRsoOnClose {}

impl IsApiRequest for PostLolLoginV1DeleteRsoOnClose {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/delete-rso-on-close".to_string()}
}

pub fn post_lol_login_v1_delete_rso_on_close() -> PostLolLoginV1DeleteRsoOnClose {
    PostLolLoginV1DeleteRsoOnClose{}
}


pub struct PostLolLoginV1LeagueSessionStatus {
    pub body: LolLoginLeagueSessionStatus,
}

impl IsApiRequest for PostLolLoginV1LeagueSessionStatus {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/leagueSessionStatus".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_login_v1_league_session_status(body: LolLoginLeagueSessionStatus) -> PostLolLoginV1LeagueSessionStatus {
    PostLolLoginV1LeagueSessionStatus{body}
}


pub struct PostLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName {
    pub service_name: String,
    pub method_name: String,
    pub body: u32,
}

impl IsApiRequest for PostLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-login/v1/service-proxy-async-requests/{}/{}", self.service_name, self.method_name)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(service_name: String, method_name: String, body: u32) -> PostLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName {
    PostLolLoginV1ServiceProxyAsyncRequestsByServiceNameByMethodName{service_name, method_name, body}
}


pub struct PostLolLoginV1ServiceProxyUuidRequests {
    pub service_name: String,
    pub method_name: String,
    pub plugin_id: u32,
    pub timeout_millis: u64,
    pub payload: String,
}

impl IsApiRequest for PostLolLoginV1ServiceProxyUuidRequests {
    const METHOD: Method = Method::POST;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-login/v1/service-proxy-uuid-requests".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("serviceName".to_string(), serde_json::to_string(&self.service_name).unwrap()),
            ("methodName".to_string(), serde_json::to_string(&self.method_name).unwrap()),
            ("pluginId".to_string(), serde_json::to_string(&self.plugin_id).unwrap()),
            ("timeoutMillis".to_string(), serde_json::to_string(&self.timeout_millis).unwrap()),
            ("payload".to_string(), serde_json::to_string(&self.payload).unwrap())
        ])
    }
}

pub fn post_lol_login_v1_service_proxy_uuid_requests(service_name: String, method_name: String, plugin_id: u32, timeout_millis: u64, payload: String) -> PostLolLoginV1ServiceProxyUuidRequests {
    PostLolLoginV1ServiceProxyUuidRequests{service_name, method_name, plugin_id, timeout_millis, payload}
}


pub struct PostLolLoginV1SessionInvoke {
    pub destination: String,
    pub method: String,
    pub args: Vec<Value>,
}

impl IsApiRequest for PostLolLoginV1SessionInvoke {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLoginLcdsResponse;
    fn get_url(&self) -> String {"/lol-login/v1/session/invoke".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("destination".to_string(), serde_json::to_string(&self.destination).unwrap()),
            ("method".to_string(), serde_json::to_string(&self.method).unwrap()),
            ("args".to_string(), serde_json::to_string(&self.args).unwrap())
        ])
    }
}

pub fn post_lol_login_v1_session_invoke(destination: String, method: String, args: Vec<Value>) -> PostLolLoginV1SessionInvoke {
    PostLolLoginV1SessionInvoke{destination, method, args}
}


pub struct PostLolLoginV1SummonerSession {
    pub body: LolLoginSummonerSessionResource,
}

impl IsApiRequest for PostLolLoginV1SummonerSession {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/summoner-session".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_login_v1_summoner_session(body: LolLoginSummonerSessionResource) -> PostLolLoginV1SummonerSession {
    PostLolLoginV1SummonerSession{body}
}


pub struct PostLolLoginV1SummonerSessionFailed {
    pub body: i32,
}

impl IsApiRequest for PostLolLoginV1SummonerSessionFailed {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-login/v1/summoner-session-failed".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_login_v1_summoner_session_failed(body: i32) -> PostLolLoginV1SummonerSessionFailed {
    PostLolLoginV1SummonerSessionFailed{body}
}


pub struct PutLolLoginV1ShutdownLocksByLockName {
    pub lock_name: String,
}

impl IsApiRequest for PutLolLoginV1ShutdownLocksByLockName {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-login/v1/shutdown-locks/{}", self.lock_name)}
}

pub fn put_lol_login_v1_shutdown_locks_by_lock_name(lock_name: String) -> PutLolLoginV1ShutdownLocksByLockName {
    PutLolLoginV1ShutdownLocksByLockName{lock_name}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginAccountStateResource {
    pub state: LolLoginAccountStateType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLcdsResponse {
    #[serde(rename = "typeName")]
    pub type_name: String,
    pub body: Value,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLeagueSessionTokenEnvelope {
    pub token: Option<String>,
    #[serde(rename = "logoutOnFailure")]
    pub logout_on_failure: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginConnectionState {
    pub mode: LolLoginLoginConnectionMode,
    #[serde(rename = "isUsingDeveloperAuthToken")]
    pub is_using_developer_auth_token: bool,
    #[serde(rename = "isPartnerRiotClient")]
    pub is_partner_riot_client: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginError {
    pub id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginQueue {
    #[serde(rename = "estimatedPositionInQueue")]
    pub estimated_position_in_queue: u64,
    #[serde(rename = "approximateWaitTimeSeconds")]
    pub approximate_wait_time_seconds: Option<u64>,
    #[serde(rename = "maxDisplayedPosition")]
    pub max_displayed_position: Option<u64>,
    #[serde(rename = "maxDisplayedWaitTimeSeconds")]
    pub max_displayed_wait_time_seconds: Option<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginSession {
    pub state: LolLoginLoginSessionStates,
    pub username: String,
    #[serde(rename = "userAuthToken")]
    pub user_auth_token: String,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "summonerId")]
    pub summoner_id: Option<u64>,
    #[serde(rename = "isInLoginQueue")]
    pub is_in_login_queue: bool,
    pub error: Option<LolLoginLoginError>,
    #[serde(rename = "idToken")]
    pub id_token: String,
    pub puuid: String,
    #[serde(rename = "isNewPlayer")]
    pub is_new_player: bool,
    pub connected: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginSessionWallet {
    pub ip: i64,
    pub rp: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginPlatformGeneratedCredentials {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginSummonerSessionResource {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isNewPlayer")]
    pub is_new_player: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLoginAccountStateType {
    #[default]
    #[serde(rename = "GENERATING")]
    Generating,
    #[serde(rename = "TRANSFERRED_OUT")]
    TransferredOut,
    #[serde(rename = "TRANSFERRING_IN")]
    TransferringIn,
    #[serde(rename = "TRANSFERRING_OUT")]
    TransferringOut,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "CREATING")]
    Creating,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLoginLeagueSessionStatus {
    #[default]
    #[serde(rename = "ANTI_ADDICTION_EXPIRED")]
    AntiAddictionExpired,
    #[serde(rename = "DUPLICATED")]
    Duplicated,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "INITIALIZED")]
    Initialized,
    #[serde(rename = "UNINITIALIZED")]
    Uninitialized,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLoginLoginConnectionMode {
    #[default]
    RiotClient,
    Partner,
    Legacy,
    Preparing,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLoginLoginSessionStates {
    #[default]
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "LOGGING_OUT")]
    LoggingOut,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
}

