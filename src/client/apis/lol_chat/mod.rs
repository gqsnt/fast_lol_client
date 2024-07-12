use reqwest::Method;
use serde_json::Value;
use crate::client::apis::plugin_macro::impl_api_plugin;
pub mod me;

impl_api_plugin!(
    "/lol-chat",
    V1{
        GetMe {
            fn:get_me,method: Method::GET,url: "/me" =>Value
        }
    }
);