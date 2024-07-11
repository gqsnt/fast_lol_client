use reqwest::Method;
use serde_json::Value;

use crate::client::apis::plugin_macro::impl_api_plugin;
use crate::client::apis::is_api_request::IsApiRequest;
pub mod me;

impl_api_plugin!(
    "/lol-chat/v1",
    GetMe {
        get_me,Method::GET,"/me" =>Value,
    },
);