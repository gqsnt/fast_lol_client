
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetMemoryV1FeProcessesReady {
    /// Returns whether or not the frontend processes are ready

}

impl IsApiRequest for GetMemoryV1FeProcessesReady {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/memory/v1/fe-processes-ready".to_string()}
}

pub fn get_memory_v1_fe_processes_ready() -> GetMemoryV1FeProcessesReady {
    GetMemoryV1FeProcessesReady{}
}


pub struct PostMemoryV1NotifyFeProcessesReady {
    /// Sends an event that the frontend processes are ready

}

impl IsApiRequest for PostMemoryV1NotifyFeProcessesReady {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/memory/v1/notify-fe-processes-ready".to_string()}
}

pub fn post_memory_v1_notify_fe_processes_ready() -> PostMemoryV1NotifyFeProcessesReady {
    PostMemoryV1NotifyFeProcessesReady{}
}


pub struct PostMemoryV1Snapshot {
    /// Sends current memory usage info to telemetry.
    pub process_ids: Vec<u32>,
    pub context: String,
}

impl IsApiRequest for PostMemoryV1Snapshot {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/memory/v1/snapshot".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("processIds".to_string(), serde_json::to_string(&self.process_ids).unwrap()),
            ("context".to_string(), serde_json::to_string(&self.context).unwrap())
        ])
    }
}

pub fn post_memory_v1_snapshot(process_ids: Vec<u32>, context: String) -> PostMemoryV1Snapshot {
    PostMemoryV1Snapshot{process_ids, context}
}


// OBJECTS


// ENUMS

