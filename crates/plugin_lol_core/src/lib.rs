
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct LoggingGetEntries {
    // Gets all buffered log entries since the last call.
}

impl IsApiRequest for LoggingGetEntries {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LogEvent>;

    fn get_url(&self) -> String {
        "/LoggingGetEntries".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn logging_get_entries() -> LoggingGetEntries {
    LoggingGetEntries {
        
    }
}


pub struct LoggingMetrics {
    // Returns all metrics
}

impl IsApiRequest for LoggingMetrics {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/LoggingMetrics".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn logging_metrics() -> LoggingMetrics {
    LoggingMetrics {
        
    }
}


pub struct LoggingMetricsMetadata {
    // Returns metadata for all metrics
}

impl IsApiRequest for LoggingMetricsMetadata {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/LoggingMetricsMetadata".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn logging_metrics_metadata() -> LoggingMetricsMetadata {
    LoggingMetricsMetadata {
        
    }
}


pub struct LoggingStart {
    // Initializes the logging system.
    pub buffered: bool,
    pub severity: LogSeverityLevels,
}

impl IsApiRequest for LoggingStart {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/LoggingStart".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "buffered" : self.buffered,
            "severity" : self.severity,
        }))
    }
}

pub fn logging_start(buffered: bool, severity: LogSeverityLevels) -> LoggingStart {
    LoggingStart {
        buffered, severity
    }
}


pub struct LoggingStop {
    // Finalizes the logging system.
}

impl IsApiRequest for LoggingStop {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/LoggingStop".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn logging_stop() -> LoggingStop {
    LoggingStop {
        
    }
}


pub struct PostRiotclientAddorupdatemetric {
    // Adds or Updates a Metric
    pub group: String,
    pub object: String,
    pub name: String,
    pub value: u64,
}

impl IsApiRequest for PostRiotclientAddorupdatemetric {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/riotclient/addorupdatemetric".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "group" : self.group,
            "object" : self.object,
            "name" : self.name,
            "value" : self.value,
        }))
    }
}

pub fn post_riotclient_addorupdatemetric(group: String, object: String, name: String, value: u64) -> PostRiotclientAddorupdatemetric {
    PostRiotclientAddorupdatemetric {
        group, object, name, value
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogEvent {
    pub severity: LogSeverityLevels,
    pub message: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LogSeverityLevels {
    #[default]
    Always,
    Error,
    Warning,
    Okay,
}

