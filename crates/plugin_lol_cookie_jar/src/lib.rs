
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetCookieJarV1Cookies {
    /// Get all cookies.

}

impl IsApiRequest for GetCookieJarV1Cookies {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<Cookie>;
    fn get_url(&self) -> String {"/cookie-jar/v1/cookies".to_string()}
}

pub fn get_cookie_jar_v1_cookies() -> GetCookieJarV1Cookies {
    GetCookieJarV1Cookies{}
}


pub struct PostCookieJarV1Cookies {
    /// Set a cookie.
    pub body: Vec<Cookie>,
}

impl IsApiRequest for PostCookieJarV1Cookies {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/cookie-jar/v1/cookies".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_cookie_jar_v1_cookies(body: Vec<Cookie>) -> PostCookieJarV1Cookies {
    PostCookieJarV1Cookies{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub url: String,
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub httponly: bool,
    pub expires: Option<i64>,
}


// ENUMS

