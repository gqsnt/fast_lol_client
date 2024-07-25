
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetPerformanceV1Memory {
    /// Returns process memory status

}

impl IsApiRequest for GetPerformanceV1Memory {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/performance/v1/memory".to_string()}
}

pub fn get_performance_v1_memory() -> GetPerformanceV1Memory {
    GetPerformanceV1Memory{}
}


pub struct GetPerformanceV1Report {
    /// Returns the various performance information for the cef processes

}

impl IsApiRequest for GetPerformanceV1Report {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<Value>;
    fn get_url(&self) -> String {"/performance/v1/report".to_string()}
}

pub fn get_performance_v1_report() -> GetPerformanceV1Report {
    GetPerformanceV1Report{}
}


pub struct GetPerformanceV1SystemInfo {
    /// Returns hardware and software specs for the machine the client is running on.
    pub full: Option<i32>,
}

impl IsApiRequest for GetPerformanceV1SystemInfo {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/performance/v1/system-info".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("full".to_string(), serde_json::to_string(&self.full).unwrap())
        ])
    }
}

pub fn get_performance_v1_system_info(full: Option<i32>) -> GetPerformanceV1SystemInfo {
    GetPerformanceV1SystemInfo{full}
}


pub struct PostPerformanceV1ProcessByProcessId {
    /// Registers the process and includes it with the performance information.
    pub process_id: u32,
}

impl IsApiRequest for PostPerformanceV1ProcessByProcessId {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/performance/v1/process/{}", self.process_id)}
}

pub fn post_performance_v1_process_by_process_id(process_id: u32) -> PostPerformanceV1ProcessByProcessId {
    PostPerformanceV1ProcessByProcessId{process_id}
}


pub struct PostPerformanceV1ReportRestart {
    /// Restarts the CPU timing information and returns the results from PerfReportProcesses
    pub sample_length: Option<i32>,
    pub sample_count: Option<i32>,
}

impl IsApiRequest for PostPerformanceV1ReportRestart {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<Value>;
    fn get_url(&self) -> String {"/performance/v1/report/restart".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("sampleLength".to_string(), serde_json::to_string(&self.sample_length).unwrap()),
            ("sampleCount".to_string(), serde_json::to_string(&self.sample_count).unwrap())
        ])
    }
}

pub fn post_performance_v1_report_restart(sample_length: Option<i32>, sample_count: Option<i32>) -> PostPerformanceV1ReportRestart {
    PostPerformanceV1ReportRestart{sample_length, sample_count}
}


// OBJECTS


// ENUMS

