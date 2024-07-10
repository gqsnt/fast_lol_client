use reqwest::Method;
use serde_json::Value;

use crate::client::api::plugin_macro::impl_api_plugin;

pub mod me;

impl_api_plugin!(
    LolChat,
    GetMe {
        get_me,Method::GET,"/me" =>Value,
    },
);