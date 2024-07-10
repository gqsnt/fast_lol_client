use reqwest::Method;
use serde_json::Value;

use crate::client::apis::impl_api_plugin;

pub mod me;

impl_api_plugin!(
    LolChat,
    lol_chat,
    GetMe {
        get_me,Method::GET,"/me" =>Value,
    },
);