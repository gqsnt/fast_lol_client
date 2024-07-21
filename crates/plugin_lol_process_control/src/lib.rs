
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetProcessControlV1Process {
    // Returns information about the process-control.
}

impl IsApiRequest for GetProcessControlV1Process {
    const METHOD: Method = Method::GET;
    type ReturnType = ProcessControlProcess;

    fn get_url(&self) -> String {
        "/process-control/v1/process".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_process_control_v_1_process() -> GetProcessControlV1Process {
    GetProcessControlV1Process {
        
    }
}


pub struct PostProcessControlV1ProcessQuit {
    // Quits the application.
}

impl IsApiRequest for PostProcessControlV1ProcessQuit {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/process-control/v1/process/quit".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_process_control_v_1_process_quit() -> PostProcessControlV1ProcessQuit {
    PostProcessControlV1ProcessQuit {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessControlProcess {
    pub status: String,
}


// ENUMS

