use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use crate::client::plugin::LolApiPlugin;
pub trait ApiRequest {
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type ReturnType: DeserializeOwned + Serialize;

    const PLUGIN: LolApiPlugin;


    fn get_path(&self) -> String;

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn build_url(&self) -> String {
        format!("{}{}", Self::PLUGIN.get_path(), self.get_path())
    }
}





#[macro_export]
macro_rules! api_request {
    // No params
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, $return_type:ty) => {
        pub struct $struct_name {}

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                $url.to_string()
            }
        }
    };

    // With query
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, query: $query_type:ty, $return_type:ty) => {
        pub struct $struct_name {
            pub query: $query_type,
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                self.query.to_path_string($url.to_string())
            }
        }
    };

    // With body
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, body: $body_type:ty, $return_type:ty) => {
        pub struct $struct_name {
            pub body: $body_type,
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                $url.to_string()
            }

            fn get_body(&self) -> Option<Value> {
                serde_json::to_value(&self.body).ok()
            }
        }
    };

    // With query and body
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, query: $query_type:ty, body: $body_type:ty, $return_type:ty) => {
        pub struct $struct_name {
            pub query: $query_type,
            pub body: $body_type,
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                 self.query.to_path_string($url.to_string())
            }

            fn get_body(&self) -> Option<Value> {
                serde_json::to_value(&self.body).ok()
            }
        }
    };
}
