use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::plugin::LolApiPlugin;
use crate::client::query::IsQuery;

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

        impl $struct_name{
            pub fn new() -> Self {
                Self{}
            }
        }

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
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, query:$query_type:ty, $return_type:ty) => {
        pub struct $struct_name {
            pub query: $query_type,
        }

        impl $struct_name{
            pub fn new(query: $query_type) -> Self {
                Self{
                    query
                }
            }
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                self.query.to_path_string($url.to_string()).unwrap()
            }

            fn get_body(&self) -> Option<Value> {
                None
            }
        }
    };


    // With body
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, body:$body_type:ty, $return_type:ty) => {
        pub struct $struct_name {
            pub body: $body_type,
        }

        impl $struct_name{
            pub fn new(body: $body_type) -> Self {
                Self{
                    body
                }
            }
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                $url.to_string()
            }

            fn get_body(&self) -> Option<Value> {
                Some(serde_json::to_value(&self.body).unwrap())
            }
        }
    };

    // With query and body
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, query:$query_type:ty, body:$body_type:ty, $return_type:ty) => {
        pub struct $struct_name {
            pub query: $query_type,
            pub body: $body_type,
        }

        impl $struct_name{
            pub fn new(query: $query_type, body: $body_type) -> Self {
                Self{
                    query,
                    body
                }
            }
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                self.query.to_path_string($url.to_string()).unwrap()
            }

            fn get_body(&self) -> Option<Value> {
                Some(serde_json::to_value(&self.body).unwrap())
            }
        }
    };
}