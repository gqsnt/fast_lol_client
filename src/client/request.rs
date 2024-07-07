use std::collections::HashMap;
use regex::Regex;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::client::plugins::LolApiPlugin;


pub fn format_string_with_query(url: &str, query: &HashMap<String, String>) -> String {
    let mut url = url.to_string();
    let re = Regex::new(r"\{(\w+)\}").unwrap();
    for cap in re.captures_iter(&url.clone()) {
        let key = cap.get(1).unwrap().as_str();
        let value = query.get(key).unwrap();
        url = url.replace(&format!("{{{}}}", key), value);
    }
    url
}

pub trait HasQuery: Serialize + DeserializeOwned{
    fn get_query(&self) -> HashMap<String, String>{
        return  serde_json::from_value(serde_json::to_value(&self).unwrap()).unwrap();
    }
}


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
macro_rules! api_request_no_params {
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr,$return_type:ty) => {
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
}


#[macro_export]
macro_rules! api_request_with_query {
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr,$query_type:ty ,$return_type:ty) => {
        pub struct $struct_name {
            pub query: $query_type,
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                format_string_with_query($url, &self.query.get_query())
            }

            fn get_body(&self) -> Option<Value> {
                serde_json::to_value(&self.query).ok()
            }
        }
    };
}



#[macro_export]
macro_rules! api_request_with_body {
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr,$body_type:ty ,$return_type:ty) => {
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
}


#[macro_export]
macro_rules! api_request_with_query_and_body {
    ($plugin:expr, $struct_name:ident, $method:expr, $url:expr,$query_type:ty,$body_type:ty ,$return_type:ty) => {
        pub struct $struct_name {
            pub query: $query_type,
            pub body: $body_type,
        }

        impl ApiRequest for $struct_name {
            const METHOD: reqwest::Method = $method;
            type ReturnType = $return_type;
            const PLUGIN: LolApiPlugin = $plugin;

            fn get_path(&self) -> String {
                format_string_with_query($url, &self.query.get_query())
            }

            fn get_body(&self) -> Option<Value> {
                serde_json::to_value(&self.body).ok()
            }
        }
    };
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdQuery {
    pub id: u32,
}
impl HasQuery for IdQuery{}

pub fn id_query(id: u32) -> IdQuery {
    IdQuery { id }
}




