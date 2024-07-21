
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetPerformanceV1Memory {
    // Returns process memory status
}

impl IsApiRequest for GetPerformanceV1Memory {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/performance/v1/memory".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_performance_v_1_memory() -> GetPerformanceV1Memory {
    GetPerformanceV1Memory {
        
    }
}


pub struct GetPerformanceV1Report {
    // Returns the various performance information for the cef processes
}

impl IsApiRequest for GetPerformanceV1Report {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<HashMap<String, String>>;

    fn get_url(&self) -> String {
        "/performance/v1/report".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_performance_v_1_report() -> GetPerformanceV1Report {
    GetPerformanceV1Report {
        
    }
}


pub struct GetPerformanceV1SystemInfo {
    // Returns hardware and software specs for the machine the client is running on.
    pub full: i32,
}

impl IsApiRequest for GetPerformanceV1SystemInfo {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/performance/v1/system-info".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "full" : self.full,
        }))
    }
}

pub fn get_performance_v_1_system_info(full: i32) -> GetPerformanceV1SystemInfo {
    GetPerformanceV1SystemInfo {
        full
    }
}


pub struct PostPerformanceV1ProcessByProcessId {
    // Registers the process and includes it with the performance information.
    pub process_id: u32,
}

impl IsApiRequest for PostPerformanceV1ProcessByProcessId {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/performance/v1/process/{}", self.process_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_performance_v_1_process_by_process_id(process_id: u32) -> PostPerformanceV1ProcessByProcessId {
    PostPerformanceV1ProcessByProcessId {
        process_id
    }
}


pub struct PostPerformanceV1ReportRestart {
    // Restarts the CPU timing information and returns the results from PerfReportProcesses
    pub sample_length: i32,
    pub sample_count: i32,
}

impl IsApiRequest for PostPerformanceV1ReportRestart {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<HashMap<String, String>>;

    fn get_url(&self) -> String {
        "/performance/v1/report/restart".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "sampleLength" : self.sample_length,
            "sampleCount" : self.sample_count,
        }))
    }
}

pub fn post_performance_v_1_report_restart(sample_length: i32, sample_count: i32) -> PostPerformanceV1ReportRestart {
    PostPerformanceV1ReportRestart {
        sample_length, sample_count
    }
}


// OBJECTS


// ENUMS

