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

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                $url.to_string()
            }
        }
    };

    // With data
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr, $api_data_type:ty, $return_type:ty) => {
        pub struct $struct_name {
            pub data: $api_data_type,
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                self.data.to_path_string($url.to_string()).unwrap()
            }

            fn get_body(&self) -> Option<Value> {
                self.data.get_body()
            }
        }
    };
}