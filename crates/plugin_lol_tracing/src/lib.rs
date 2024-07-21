
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteTracingV1PerformanceByName {
    // Ends recording of a performance metric.
    pub name: String,
}

impl IsApiRequest for DeleteTracingV1PerformanceByName {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/tracing/v1/performance/{}", self.name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_tracing_v_1_performance_by_name(name: String) -> DeleteTracingV1PerformanceByName {
    DeleteTracingV1PerformanceByName {
        name
    }
}


pub struct DeleteTracingV1TraceTimeSeriesEventByEventName {
    // Record the ending of a time series event.
    pub event_name: String,
    pub when: u64,
    pub suffix: Option<String>,
}

impl IsApiRequest for DeleteTracingV1TraceTimeSeriesEventByEventName {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/tracing/v1/trace/time-series-event/{}", self.event_name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "when" : self.when,
            "suffix" : self.suffix,
        }))
    }
}

pub fn delete_tracing_v_1_trace_time_series_event_by_event_name(event_name: String, when: u64, suffix: Option<String>) -> DeleteTracingV1TraceTimeSeriesEventByEventName {
    DeleteTracingV1TraceTimeSeriesEventByEventName {
        event_name, when, suffix
    }
}


pub struct GetTracingV1TracePayloadsEnabled {
    // Returns whether payloads are included in the tracing log.
}

impl IsApiRequest for GetTracingV1TracePayloadsEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/tracing/v1/trace/payloads/enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_tracing_v_1_trace_payloads_enabled() -> GetTracingV1TracePayloadsEnabled {
    GetTracingV1TracePayloadsEnabled {
        
    }
}


pub struct PostTracingV1PerformanceByName {
    // Starts recording of a performance metric.
    pub name: String,
}

impl IsApiRequest for PostTracingV1PerformanceByName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/tracing/v1/performance/{}", self.name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_performance_by_name(name: String) -> PostTracingV1PerformanceByName {
    PostTracingV1PerformanceByName {
        name
    }
}


pub struct PostTracingV1TraceCriticalFlow {
    // Record a crtical flow event.
    pub body: TracingCriticalFlowEventV1,
}

impl IsApiRequest for PostTracingV1TraceCriticalFlow {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/tracing/v1/trace/critical-flow".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_critical_flow(body: TracingCriticalFlowEventV1) -> PostTracingV1TraceCriticalFlow {
    PostTracingV1TraceCriticalFlow {
        body
    }
}


pub struct PostTracingV1TraceEvent {
    // Record a tracing event.
    pub body: TracingEventV1,
}

impl IsApiRequest for PostTracingV1TraceEvent {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/tracing/v1/trace/event".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_event(body: TracingEventV1) -> PostTracingV1TraceEvent {
    PostTracingV1TraceEvent {
        body
    }
}


pub struct PostTracingV1TraceModule {
    // Record a module description.
    pub body: TracingModuleV1,
}

impl IsApiRequest for PostTracingV1TraceModule {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/tracing/v1/trace/module".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_module(body: TracingModuleV1) -> PostTracingV1TraceModule {
    PostTracingV1TraceModule {
        body
    }
}


pub struct PostTracingV1TraceNonTimingEventByEventName {
    // Record a non timing telemetry event.
    pub event_name: String,
    pub when: u64,
    pub value: String,
    pub unit: String,
}

impl IsApiRequest for PostTracingV1TraceNonTimingEventByEventName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/tracing/v1/trace/non-timing-event/{}", self.event_name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "when" : self.when,
            "value" : self.value,
            "unit" : self.unit,
        }))
    }
}

pub fn post_tracing_v_1_trace_non_timing_event_by_event_name(event_name: String, when: u64, value: String, unit: String) -> PostTracingV1TraceNonTimingEventByEventName {
    PostTracingV1TraceNonTimingEventByEventName {
        event_name, when, value, unit
    }
}


pub struct PostTracingV1TracePhaseBegin {
    // Record a tracing phase beginning.
    pub body: TracingPhaseBeginV1,
}

impl IsApiRequest for PostTracingV1TracePhaseBegin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/tracing/v1/trace/phase/begin".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_phase_begin(body: TracingPhaseBeginV1) -> PostTracingV1TracePhaseBegin {
    PostTracingV1TracePhaseBegin {
        body
    }
}


pub struct PostTracingV1TracePhaseEnd {
    // Record a tracing phase ending.
    pub body: TracingPhaseEndV1,
}

impl IsApiRequest for PostTracingV1TracePhaseEnd {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/tracing/v1/trace/phase/end".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_phase_end(body: TracingPhaseEndV1) -> PostTracingV1TracePhaseEnd {
    PostTracingV1TracePhaseEnd {
        body
    }
}


pub struct PostTracingV1TraceStepEvent {
    // Record a tracing step event.
    pub body: String,
}

impl IsApiRequest for PostTracingV1TraceStepEvent {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/tracing/v1/trace/step-event".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_step_event(body: String) -> PostTracingV1TraceStepEvent {
    PostTracingV1TraceStepEvent {
        body
    }
}


pub struct PostTracingV1TraceTimeSeriesEventByEventName {
    // Record the beginning of a time series event.
    pub event_name: String,
    pub body: u64,
}

impl IsApiRequest for PostTracingV1TraceTimeSeriesEventByEventName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/tracing/v1/trace/time-series-event/{}", self.event_name)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_time_series_event_by_event_name(event_name: String, body: u64) -> PostTracingV1TraceTimeSeriesEventByEventName {
    PostTracingV1TraceTimeSeriesEventByEventName {
        event_name, body
    }
}


pub struct PostTracingV1TraceTimeSeriesEventByEventNameMarkerByMarkerName {
    // Record a marker within a time series event.
    pub event_name: String,
    pub marker_name: String,
    pub body: u64,
}

impl IsApiRequest for PostTracingV1TraceTimeSeriesEventByEventNameMarkerByMarkerName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/tracing/v1/trace/time-series-event/{}/marker/{}", self.event_name, self.marker_name)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_tracing_v_1_trace_time_series_event_by_event_name_marker_by_marker_name(event_name: String, marker_name: String, body: u64) -> PostTracingV1TraceTimeSeriesEventByEventNameMarkerByMarkerName {
    PostTracingV1TraceTimeSeriesEventByEventNameMarkerByMarkerName {
        event_name, marker_name, body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TracingCriticalFlowEventV1 {
    pub when: u64,
    pub event_id: String,
    pub succeeded: bool,
    pub payload_string: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TracingEventV1 {
    pub when: u64,
    pub name: String,
    pub src: String,
    pub dest: String,
    pub tags: String,
    pub details: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TracingModuleV1 {
    pub module_id: u32,
    pub name: String,
    pub type_: TracingModuleTypeV1,
    pub threading_model: TracingModuleThreadingModelV1,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TracingPhaseBeginV1 {
    pub when: u64,
    pub name: String,
    pub importance: TracingPhaseImportanceV1,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TracingPhaseEndV1 {
    pub when: u64,
    pub name: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum TracingModuleThreadingModelV1 {
    #[default]
    #[serde(rename = "kParallel")]
    KParallel,
    #[serde(rename = "kConcurrent")]
    KConcurrent,
    #[serde(rename = "kSequential")]
    KSequential,
    #[serde(rename = "kDedicated")]
    KDedicated,
    #[serde(rename = "kMainThread")]
    KMainThread,
    #[serde(rename = "kNone")]
    KNone,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum TracingModuleTypeV1 {
    #[default]
    #[serde(rename = "kRemotingSource")]
    KRemotingSource,
    #[serde(rename = "kFrontEndPlugin")]
    KFrontEndPlugin,
    #[serde(rename = "kBackendOther")]
    KBackendOther,
    #[serde(rename = "kBackEndPlugin")]
    KBackEndPlugin,
    #[serde(rename = "kRemoteAppModule")]
    KRemoteAppModule,
    #[serde(rename = "kUnknown")]
    KUnknown,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum TracingPhaseImportanceV1 {
    #[default]
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "trivial")]
    Trivial,
}

