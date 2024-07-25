
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetTelemetryV1ApplicationStartTime {
    /// Gets the millisecond UNIX timestamp of when the application was started.

}

impl IsApiRequest for GetTelemetryV1ApplicationStartTime {
    const METHOD: Method = Method::GET;
    type ReturnType = u64;
    fn get_url(&self) -> String {"/telemetry/v1/application-start-time".to_string()}
}

pub fn get_telemetry_v1_application_start_time() -> GetTelemetryV1ApplicationStartTime {
    GetTelemetryV1ApplicationStartTime{}
}


pub struct PatchTelemetryV3SlisAddBoolDiagnostic {
    /// Add bool diagnostic to be sent with SLIs.
    pub body: SliBoolDiagnostic,
}

impl IsApiRequest for PatchTelemetryV3SlisAddBoolDiagnostic {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/slis/add-bool-diagnostic".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_telemetry_v3_slis_add_bool_diagnostic(body: SliBoolDiagnostic) -> PatchTelemetryV3SlisAddBoolDiagnostic {
    PatchTelemetryV3SlisAddBoolDiagnostic{body}
}


pub struct PatchTelemetryV3SlisAddDoubleDiagnostic {
    /// Add double diagnostic to be sent with SLIs.
    pub body: SliDoubleDiagnostic,
}

impl IsApiRequest for PatchTelemetryV3SlisAddDoubleDiagnostic {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/slis/add-double-diagnostic".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_telemetry_v3_slis_add_double_diagnostic(body: SliDoubleDiagnostic) -> PatchTelemetryV3SlisAddDoubleDiagnostic {
    PatchTelemetryV3SlisAddDoubleDiagnostic{body}
}


pub struct PatchTelemetryV3SlisAddIntDiagnostic {
    /// Add int diagnostic to be sent with SLIs.
    pub body: SliIntDiagnostic,
}

impl IsApiRequest for PatchTelemetryV3SlisAddIntDiagnostic {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/slis/add-int-diagnostic".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_telemetry_v3_slis_add_int_diagnostic(body: SliIntDiagnostic) -> PatchTelemetryV3SlisAddIntDiagnostic {
    PatchTelemetryV3SlisAddIntDiagnostic{body}
}


pub struct PatchTelemetryV3SlisAddLabel {
    /// Add label to be sent with SLIs.
    pub body: SliLabel,
}

impl IsApiRequest for PatchTelemetryV3SlisAddLabel {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/slis/add-label".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_telemetry_v3_slis_add_label(body: SliLabel) -> PatchTelemetryV3SlisAddLabel {
    PatchTelemetryV3SlisAddLabel{body}
}


pub struct PatchTelemetryV3SlisAddStringDiagnostic {
    /// Add string diagnostic to be sent with SLIs.
    pub body: SliStringDiagnostic,
}

impl IsApiRequest for PatchTelemetryV3SlisAddStringDiagnostic {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/slis/add-string-diagnostic".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn patch_telemetry_v3_slis_add_string_diagnostic(body: SliStringDiagnostic) -> PatchTelemetryV3SlisAddStringDiagnostic {
    PatchTelemetryV3SlisAddStringDiagnostic{body}
}


pub struct PostTelemetryV1CommonDataByKey {
    /// Adds/updates a common data key and value to be sent with every subsequent event.
    pub key: String,
    pub body: String,
}

impl IsApiRequest for PostTelemetryV1CommonDataByKey {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/telemetry/v1/common-data/{}", self.key)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v1_common_data_by_key(key: String, body: String) -> PostTelemetryV1CommonDataByKey {
    PostTelemetryV1CommonDataByKey{key, body}
}


pub struct PostTelemetryV1EventsByEventType {
    /// Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks. All events will have their eventType prefixed with "riot__rclient__"
    pub event_type: String,
    pub body: HashMap<String, Value>,
}

impl IsApiRequest for PostTelemetryV1EventsByEventType {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/telemetry/v1/events/{}", self.event_type)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v1_events_by_event_type(event_type: String, body: HashMap<String, Value>) -> PostTelemetryV1EventsByEventType {
    PostTelemetryV1EventsByEventType{event_type, body}
}


pub struct PostTelemetryV1EventsWithPerfInfoByEventType {
    /// Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks. This will include current performance information along with the passed in data. Each call will record the performance counters then reset them for use in the next call. All events will have their eventType prefixed with "riot__rclient__"
    pub event_type: String,
    pub body: HashMap<String, Value>,
}

impl IsApiRequest for PostTelemetryV1EventsWithPerfInfoByEventType {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/telemetry/v1/events-with-perf-info/{}", self.event_type)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v1_events_with_perf_info_by_event_type(event_type: String, body: HashMap<String, Value>) -> PostTelemetryV1EventsWithPerfInfoByEventType {
    PostTelemetryV1EventsWithPerfInfoByEventType{event_type, body}
}


pub struct PostTelemetryV3EventsByEventType {
    /// Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks using the Riot Data API. All events will have their eventType prefixed with "riot__rclient__"
    pub event_type: String,
    pub body: HashMap<String, Value>,
}

impl IsApiRequest for PostTelemetryV3EventsByEventType {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/telemetry/v3/events/{}", self.event_type)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v3_events_by_event_type(event_type: String, body: HashMap<String, Value>) -> PostTelemetryV3EventsByEventType {
    PostTelemetryV3EventsByEventType{event_type, body}
}


pub struct PostTelemetryV3EventsOnceByEventTypeByOnceTag {
    /// Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks using the Riot Data API that will be sent only once during this client executable run regardless of any javascript frontend restarts. All events will have their eventType prefixed with "riot__rclient__"
    pub event_type: String,
    pub once_tag: String,
    pub body: HashMap<String, String>,
}

impl IsApiRequest for PostTelemetryV3EventsOnceByEventTypeByOnceTag {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/telemetry/v3/events-once/{}/{}", self.event_type, self.once_tag)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v3_events_once_by_event_type_by_once_tag(event_type: String, once_tag: String, body: HashMap<String, String>) -> PostTelemetryV3EventsOnceByEventTypeByOnceTag {
    PostTelemetryV3EventsOnceByEventTypeByOnceTag{event_type, once_tag, body}
}


pub struct PostTelemetryV3SlisCounts {
    /// Report an SLI to the collector
    pub body: SliCount,
}

impl IsApiRequest for PostTelemetryV3SlisCounts {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/slis/counts".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v3_slis_counts(body: SliCount) -> PostTelemetryV3SlisCounts {
    PostTelemetryV3SlisCounts{body}
}


pub struct PostTelemetryV3UptimeTrackingNotifyFailure {
    /// Mark an availability item's operation as failure on uptime tracking mechanism.
    pub body: NotifyFailureRequest,
}

impl IsApiRequest for PostTelemetryV3UptimeTrackingNotifyFailure {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/uptime-tracking/notify-failure".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v3_uptime_tracking_notify_failure(body: NotifyFailureRequest) -> PostTelemetryV3UptimeTrackingNotifyFailure {
    PostTelemetryV3UptimeTrackingNotifyFailure{body}
}


pub struct PostTelemetryV3UptimeTrackingNotifySuccess {
    /// Mark an availability item's operation as success on uptime tracking mechanism.
    pub body: NotifySuccessRequest,
}

impl IsApiRequest for PostTelemetryV3UptimeTrackingNotifySuccess {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/telemetry/v3/uptime-tracking/notify-success".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_telemetry_v3_uptime_tracking_notify_success(body: NotifySuccessRequest) -> PostTelemetryV3UptimeTrackingNotifySuccess {
    PostTelemetryV3UptimeTrackingNotifySuccess{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotifyFailureRequest {
    #[serde(rename = "availabilityItemName")]
    pub availability_item_name: String,
    #[serde(rename = "failureInfo")]
    pub failure_info: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotifySuccessRequest {
    #[serde(rename = "availabilityItemName")]
    pub availability_item_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SliBoolDiagnostic {
    pub key: String,
    pub value: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SliCount {
    #[serde(rename = "sliName")]
    pub sli_name: String,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,
    pub successes: f64,
    pub failures: f64,
    #[serde(rename = "startTimeEpochMs")]
    pub start_time_epoch_ms: i64,
    #[serde(rename = "endTimeEpochMs")]
    pub end_time_epoch_ms: i64,
    pub labels: HashMap<String, String>,
    #[serde(rename = "boolDiagnostics")]
    pub bool_diagnostics: HashMap<String, bool>,
    #[serde(rename = "doubleDiagnostics")]
    pub double_diagnostics: HashMap<String, f64>,
    #[serde(rename = "intDiagnostics")]
    pub int_diagnostics: HashMap<String, i64>,
    #[serde(rename = "stringDiagnostics")]
    pub string_diagnostics: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SliDoubleDiagnostic {
    pub key: String,
    pub value: f64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SliIntDiagnostic {
    pub key: String,
    pub value: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SliLabel {
    pub key: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SliStringDiagnostic {
    pub key: String,
    pub value: String,
}


// ENUMS

