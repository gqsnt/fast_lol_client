
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolPlatformConfigV1InitialConfigurationComplete {

}

impl IsApiRequest for GetLolPlatformConfigV1InitialConfigurationComplete {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-platform-config/v1/initial-configuration-complete".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_platform_config_v_1_initial_configuration_complete() -> GetLolPlatformConfigV1InitialConfigurationComplete {
    GetLolPlatformConfigV1InitialConfigurationComplete {
        
    }
}


pub struct GetLolPlatformConfigV1Namespaces {

}

impl IsApiRequest for GetLolPlatformConfigV1Namespaces {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-platform-config/v1/namespaces".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_platform_config_v_1_namespaces() -> GetLolPlatformConfigV1Namespaces {
    GetLolPlatformConfigV1Namespaces {
        
    }
}


pub struct GetLolPlatformConfigV1NamespacesByNs {

    pub ns: String,
}

impl IsApiRequest for GetLolPlatformConfigV1NamespacesByNs {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-platform-config/v1/namespaces/{}", self.ns)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_platform_config_v_1_namespaces_by_ns(ns: String) -> GetLolPlatformConfigV1NamespacesByNs {
    GetLolPlatformConfigV1NamespacesByNs {
        ns
    }
}


pub struct GetLolPlatformConfigV1NamespacesByNsByKey {

    pub ns: String,
    pub key: String,
}

impl IsApiRequest for GetLolPlatformConfigV1NamespacesByNsByKey {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-platform-config/v1/namespaces/{}/{}", self.ns, self.key)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_platform_config_v_1_namespaces_by_ns_by_key(ns: String, key: String) -> GetLolPlatformConfigV1NamespacesByNsByKey {
    GetLolPlatformConfigV1NamespacesByNsByKey {
        ns, key
    }
}


// OBJECTS


// ENUMS

