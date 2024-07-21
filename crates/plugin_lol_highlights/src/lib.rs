
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolHighlightsV1HighlightsById {
    pub id: u64,
}

impl IsApiRequest for DeleteLolHighlightsV1HighlightsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = LolHighlightsHighlight;
    fn get_url(&self) -> String {format!("/lol-highlights/v1/highlights/{}", self.id)}
}

pub fn delete_lol_highlights_v_1_highlights_by_id(id: u64) -> DeleteLolHighlightsV1HighlightsById {
    DeleteLolHighlightsV1HighlightsById{id}
}


pub struct GetLolHighlightsV1Config {}

impl IsApiRequest for GetLolHighlightsV1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHighlightsHighlightsConfig;
    fn get_url(&self) -> String {"/lol-highlights/v1/config".to_string()}
}

pub fn get_lol_highlights_v_1_config() -> GetLolHighlightsV1Config {
    GetLolHighlightsV1Config{}
}


pub struct GetLolHighlightsV1Highlights {}

impl IsApiRequest for GetLolHighlightsV1Highlights {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolHighlightsHighlight>;
    fn get_url(&self) -> String {"/lol-highlights/v1/highlights".to_string()}
}

pub fn get_lol_highlights_v_1_highlights() -> GetLolHighlightsV1Highlights {
    GetLolHighlightsV1Highlights{}
}


pub struct GetLolHighlightsV1HighlightsById {
    pub id: u64,
}

impl IsApiRequest for GetLolHighlightsV1HighlightsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHighlightsHighlight;
    fn get_url(&self) -> String {format!("/lol-highlights/v1/highlights/{}", self.id)}
}

pub fn get_lol_highlights_v_1_highlights_by_id(id: u64) -> GetLolHighlightsV1HighlightsById {
    GetLolHighlightsV1HighlightsById{id}
}


pub struct GetLolHighlightsV1HighlightsFolderPath {}

impl IsApiRequest for GetLolHighlightsV1HighlightsFolderPath {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-highlights/v1/highlights-folder-path".to_string()}
}

pub fn get_lol_highlights_v_1_highlights_folder_path() -> GetLolHighlightsV1HighlightsFolderPath {
    GetLolHighlightsV1HighlightsFolderPath{}
}


pub struct GetLolHighlightsV1HighlightsFolderPathDefault {}

impl IsApiRequest for GetLolHighlightsV1HighlightsFolderPathDefault {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-highlights/v1/highlights-folder-path/default".to_string()}
}

pub fn get_lol_highlights_v_1_highlights_folder_path_default() -> GetLolHighlightsV1HighlightsFolderPathDefault {
    GetLolHighlightsV1HighlightsFolderPathDefault{}
}


pub struct PostLolHighlightsV1FileBrowserByHighlightId {
    pub highlight_id: u64,
}

impl IsApiRequest for PostLolHighlightsV1FileBrowserByHighlightId {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-highlights/v1/file-browser/{}", self.highlight_id)}
}

pub fn post_lol_highlights_v_1_file_browser_by_highlight_id(highlight_id: u64) -> PostLolHighlightsV1FileBrowserByHighlightId {
    PostLolHighlightsV1FileBrowserByHighlightId{highlight_id}
}


pub struct PostLolHighlightsV1Highlights {}

impl IsApiRequest for PostLolHighlightsV1Highlights {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolHighlightsHighlight>;
    fn get_url(&self) -> String {"/lol-highlights/v1/highlights".to_string()}
}

pub fn post_lol_highlights_v_1_highlights() -> PostLolHighlightsV1Highlights {
    PostLolHighlightsV1Highlights{}
}


pub struct PutLolHighlightsV1HighlightsById {
    pub id: u64,
    pub body: LolHighlightsHighlight,
}

impl IsApiRequest for PutLolHighlightsV1HighlightsById {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolHighlightsHighlight;
    fn get_url(&self) -> String {format!("/lol-highlights/v1/highlights/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_highlights_v_1_highlights_by_id(id: u64, body: LolHighlightsHighlight) -> PutLolHighlightsV1HighlightsById {
    PutLolHighlightsV1HighlightsById{id, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlight {
    pub id: u64,
    pub name: String,
    pub filepath: String,
    pub url: String,
    pub mtime_ms_utc: u64,
    pub mtime_iso_8601: String,
    pub file_size_bytes: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlightsConfig {
    pub is_highlights_enabled: bool,
    pub invalid_highlight_name_characters: String,
}


// ENUMS

