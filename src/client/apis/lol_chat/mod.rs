use reqwest::Method;
use serde_json::Value;

use crate::client::apis::plugin_macro::impl_api_plugin;

pub mod me;

impl_api_plugin!(
    LolApiPlugin::LolChat,
    GetMe {
        get_me,Method::GET,"/me" =>Value,
    },
);