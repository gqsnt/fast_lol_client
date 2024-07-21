
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetRiotclientAffinity {
    // Get the current runtime affinity of the application.
}

impl IsApiRequest for GetRiotclientAffinity {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/riotclient/affinity".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_affinity() -> GetRiotclientAffinity {
    GetRiotclientAffinity {
        
    }
}


pub struct PostRiotclientAffinity {
    // Sets the current runtime affinity of the application.
    pub body: String,
}

impl IsApiRequest for PostRiotclientAffinity {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/affinity".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_affinity(body: String) -> PostRiotclientAffinity {
    PostRiotclientAffinity {
        body
    }
}


pub struct DeleteRiotclientAffinity {
    // Deletes the current runtime affinity of the application.
}

impl IsApiRequest for DeleteRiotclientAffinity {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/affinity".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_riotclient_affinity() -> DeleteRiotclientAffinity {
    DeleteRiotclientAffinity {
        
    }
}


pub struct PutRiotclientSplash {
    // Show the splash screen.
    pub body: String,
}

impl IsApiRequest for PutRiotclientSplash {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/splash".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_riotclient_splash(body: String) -> PutRiotclientSplash {
    PutRiotclientSplash {
        body
    }
}


pub struct DeleteRiotclientSplash {
    // Hide the splash screen.
}

impl IsApiRequest for DeleteRiotclientSplash {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/splash".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_riotclient_splash() -> DeleteRiotclientSplash {
    DeleteRiotclientSplash {
        
    }
}


pub struct PutRiotclientV1AuthTokensByAuthToken {
    // Register an auth token.  This is any alpha-numeric string that will be used as a password with the `riot` user when making requests.
    pub auth_token: String,
}

impl IsApiRequest for PutRiotclientV1AuthTokensByAuthToken {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/riotclient/v1/auth-tokens/{}", self.auth_token)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_riotclient_v_1_auth_tokens_by_auth_token(auth_token: String) -> PutRiotclientV1AuthTokensByAuthToken {
    PutRiotclientV1AuthTokensByAuthToken {
        auth_token
    }
}


pub struct DeleteRiotclientV1AuthTokensByAuthToken {
    // Unregister an existing auth token.
    pub auth_token: String,
}

impl IsApiRequest for DeleteRiotclientV1AuthTokensByAuthToken {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/riotclient/v1/auth-tokens/{}", self.auth_token)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_riotclient_v_1_auth_tokens_by_auth_token(auth_token: String) -> DeleteRiotclientV1AuthTokensByAuthToken {
    DeleteRiotclientV1AuthTokensByAuthToken {
        auth_token
    }
}


pub struct GetRiotclientAppName {
    // Application name without file extension
}

impl IsApiRequest for GetRiotclientAppName {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/riotclient/app-name".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_app_name() -> GetRiotclientAppName {
    GetRiotclientAppName {
        
    }
}


pub struct GetRiotclientAppPort {
    // Get the TCP port number that the remoting server is listening on.
}

impl IsApiRequest for GetRiotclientAppPort {
    const METHOD: Method = Method::GET;
    type ReturnType = u16;

    fn get_url(&self) -> String {
        "/riotclient/app-port".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_app_port() -> GetRiotclientAppPort {
    GetRiotclientAppPort {
        
    }
}


pub struct GetRiotclientAuthToken {
    // Return the auth token used by the remoting server
}

impl IsApiRequest for GetRiotclientAuthToken {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/riotclient/auth-token".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_auth_token() -> GetRiotclientAuthToken {
    GetRiotclientAuthToken {
        
    }
}


pub struct GetRiotclientCommandLineArgs {
    // Get the command line parameters for the application
}

impl IsApiRequest for GetRiotclientCommandLineArgs {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;

    fn get_url(&self) -> String {
        "/riotclient/command-line-args".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_command_line_args() -> GetRiotclientCommandLineArgs {
    GetRiotclientCommandLineArgs {
        
    }
}


pub struct GetRiotclientMachineId {
    // Base64 encoded uuid identifying the user's machine
}

impl IsApiRequest for GetRiotclientMachineId {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/riotclient/machine-id".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_machine_id() -> GetRiotclientMachineId {
    GetRiotclientMachineId {
        
    }
}


pub struct GetRiotclientRegionLocale {

}

impl IsApiRequest for GetRiotclientRegionLocale {
    const METHOD: Method = Method::GET;
    type ReturnType = LolL10NRegionLocale;

    fn get_url(&self) -> String {
        "/riotclient/region-locale".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_region_locale() -> GetRiotclientRegionLocale {
    GetRiotclientRegionLocale {
        
    }
}


pub struct GetRiotclientSystemInfoV1BasicInfo {
    // Get basic system information: OS, memory, processor speed, and number of physical cores
}

impl IsApiRequest for GetRiotclientSystemInfoV1BasicInfo {
    const METHOD: Method = Method::GET;
    type ReturnType = BasicSystemInfo;

    fn get_url(&self) -> String {
        "/riotclient/system-info/v1/basic-info".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_system_info_v_1_basic_info() -> GetRiotclientSystemInfoV1BasicInfo {
    GetRiotclientSystemInfoV1BasicInfo {
        
    }
}


pub struct GetRiotclientTrace {
    // Retrieves a completed scheduler trace.
}

impl IsApiRequest for GetRiotclientTrace {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/riotclient/trace".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_trace() -> GetRiotclientTrace {
    GetRiotclientTrace {
        
    }
}


pub struct GetRiotclientUxCrashCount {
    // Returns whether the ux has crashed or not
}

impl IsApiRequest for GetRiotclientUxCrashCount {
    const METHOD: Method = Method::GET;
    type ReturnType = u32;

    fn get_url(&self) -> String {
        "/riotclient/ux-crash-count".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_ux_crash_count() -> GetRiotclientUxCrashCount {
    GetRiotclientUxCrashCount {
        
    }
}


pub struct GetRiotclientUxState {
    // Get the current Ux state.
}

impl IsApiRequest for GetRiotclientUxState {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/riotclient/ux-state".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_ux_state() -> GetRiotclientUxState {
    GetRiotclientUxState {
        
    }
}


pub struct GetRiotclientV1CrashReportingEnvironment {
    // Get the crash reporting environment identifier.
}

impl IsApiRequest for GetRiotclientV1CrashReportingEnvironment {
    const METHOD: Method = Method::GET;
    type ReturnType = CrashReportingEnvironment;

    fn get_url(&self) -> String {
        "/riotclient/v1/crash-reporting/environment".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_v_1_crash_reporting_environment() -> GetRiotclientV1CrashReportingEnvironment {
    GetRiotclientV1CrashReportingEnvironment {
        
    }
}


pub struct PutRiotclientV1CrashReportingEnvironment {
    // Tags the crash with an environment so it can be filtered more easily.
    pub body: CrashReportingEnvironment,
}

impl IsApiRequest for PutRiotclientV1CrashReportingEnvironment {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/v1/crash-reporting/environment".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_riotclient_v_1_crash_reporting_environment(body: CrashReportingEnvironment) -> PutRiotclientV1CrashReportingEnvironment {
    PutRiotclientV1CrashReportingEnvironment {
        body
    }
}


pub struct GetRiotclientZoomScale {
    // Gets the last known posted zoom-scale value.
}

impl IsApiRequest for GetRiotclientZoomScale {
    const METHOD: Method = Method::GET;
    type ReturnType = f64;

    fn get_url(&self) -> String {
        "/riotclient/zoom-scale".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_riotclient_zoom_scale() -> GetRiotclientZoomScale {
    GetRiotclientZoomScale {
        
    }
}


pub struct PostRiotclientZoomScale {
    // Handles changing the zoom scale value.
    pub body: f64,
}

impl IsApiRequest for PostRiotclientZoomScale {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/zoom-scale".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_zoom_scale(body: f64) -> PostRiotclientZoomScale {
    PostRiotclientZoomScale {
        body
    }
}


pub struct PostRiotclientKillAndRestartUx {
    // Kills the ux process and restarts it. Used only when the ux process crashes.
}

impl IsApiRequest for PostRiotclientKillAndRestartUx {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/kill-and-restart-ux".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_kill_and_restart_ux() -> PostRiotclientKillAndRestartUx {
    PostRiotclientKillAndRestartUx {
        
    }
}


pub struct PostRiotclientKillUx {
    // Kills the ux process.
}

impl IsApiRequest for PostRiotclientKillUx {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/kill-ux".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_kill_ux() -> PostRiotclientKillUx {
    PostRiotclientKillUx {
        
    }
}


pub struct PostRiotclientLaunchUx {
    // Launches the ux process.
}

impl IsApiRequest for PostRiotclientLaunchUx {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/launch-ux".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_launch_ux() -> PostRiotclientLaunchUx {
    PostRiotclientLaunchUx {
        
    }
}


pub struct PostRiotclientNewArgs {
    // Endpoint for passing in new data.
    pub body: Vec<String>,
}

impl IsApiRequest for PostRiotclientNewArgs {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/new-args".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_new_args(body: Vec<String>) -> PostRiotclientNewArgs {
    PostRiotclientNewArgs {
        body
    }
}


pub struct PostRiotclientOpenUrlInBrowser {
    // Opens a URL in the player's system browser. The browser will use its existing cookies to determine if the player is logged in, meaning that using this API can result in three differenet cases:    //     // * The player is signed in with the current account    // * The player is signed in with a different account    // * The player is not signed in at all.    //     // In order to ensure the player is signed in with the same account as the client, leverage Player Platform's [authentication redirect](https://platform.riotgames.com/docs/authenticate-players/how-rso-works/sign-in-on-mobile/authentication-redirect) feature.
    pub body: String,
}

impl IsApiRequest for PostRiotclientOpenUrlInBrowser {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/open-url-in-browser".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_open_url_in_browser(body: String) -> PostRiotclientOpenUrlInBrowser {
    PostRiotclientOpenUrlInBrowser {
        body
    }
}


pub struct PostRiotclientShowSwagger {
    // Open swagger in the default browser.
}

impl IsApiRequest for PostRiotclientShowSwagger {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/show-swagger".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_show_swagger() -> PostRiotclientShowSwagger {
    PostRiotclientShowSwagger {
        
    }
}


pub struct PostRiotclientUnload {
    // Unloads the UX process
}

impl IsApiRequest for PostRiotclientUnload {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/unload".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_unload() -> PostRiotclientUnload {
    PostRiotclientUnload {
        
    }
}


pub struct PostRiotclientUxAllowForeground {
    // Allows the background process to launch the game into the foregound.
}

impl IsApiRequest for PostRiotclientUxAllowForeground {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/ux-allow-foreground".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_ux_allow_foreground() -> PostRiotclientUxAllowForeground {
    PostRiotclientUxAllowForeground {
        
    }
}


pub struct PostRiotclientUxFlash {
    // Flash the ux process' main window and the taskbar/dock icon, if they exist.
}

impl IsApiRequest for PostRiotclientUxFlash {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/ux-flash".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_ux_flash() -> PostRiotclientUxFlash {
    PostRiotclientUxFlash {
        
    }
}


pub struct PostRiotclientUxMinimize {
    // Minimize the ux process and all its windows if it exists. This does not kill the ux.
}

impl IsApiRequest for PostRiotclientUxMinimize {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/ux-minimize".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_ux_minimize() -> PostRiotclientUxMinimize {
    PostRiotclientUxMinimize {
        
    }
}


pub struct PostRiotclientUxShow {
    // Shows the ux process if it exists; create and show if it does not.
}

impl IsApiRequest for PostRiotclientUxShow {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/ux-show".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_ux_show() -> PostRiotclientUxShow {
    PostRiotclientUxShow {
        
    }
}


pub struct PostRiotclientV1CrashReportingLogs {
    // Adds the enclosed log to the app's crash report.
    pub body: String,
}

impl IsApiRequest for PostRiotclientV1CrashReportingLogs {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/v1/crash-reporting/logs".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_v_1_crash_reporting_logs(body: String) -> PostRiotclientV1CrashReportingLogs {
    PostRiotclientV1CrashReportingLogs {
        body
    }
}


pub struct PostRiotclientV1ElevationRequests {

    pub body: ElevationRequest,
}

impl IsApiRequest for PostRiotclientV1ElevationRequests {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/v1/elevation-requests".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_riotclient_v_1_elevation_requests(body: ElevationRequest) -> PostRiotclientV1ElevationRequests {
    PostRiotclientV1ElevationRequests {
        body
    }
}


pub struct PutRiotclientUxLoadComplete {
    // Ux notification that it has completed loading the main window.
}

impl IsApiRequest for PutRiotclientUxLoadComplete {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/ux-load-complete".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_riotclient_ux_load_complete() -> PutRiotclientUxLoadComplete {
    PutRiotclientUxLoadComplete {
        
    }
}


pub struct PutRiotclientUxStateAck {
    // Ux acknowledges the update to the Ux state.
    pub body: u32,
}

impl IsApiRequest for PutRiotclientUxStateAck {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/ux-state/ack".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_riotclient_ux_state_ack(body: u32) -> PutRiotclientUxStateAck {
    PutRiotclientUxStateAck {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolL10NRegionLocale {
    pub region: String,
    pub locale: String,
    pub web_region: String,
    pub web_language: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CrashReportingEnvironment {
    pub environment: String,
    pub user_name: String,
    pub user_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicSystemInfo {
    pub operating_system: BasicOperatingSystemInfo,
    pub physical_memory: u64,
    pub physical_processor_cores: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicOperatingSystemInfo {
    pub edition: String,
    pub platform: String,
    pub version_major: String,
    pub version_minor: String,
    pub build_number: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElevationRequest {
    pub action: ElevationAction,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ElevationAction {
    #[default]
    FixBrokenPermissions,
}

