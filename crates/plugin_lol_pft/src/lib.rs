
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolPftV2Survey {}

impl IsApiRequest for GetLolPftV2Survey {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPftPftSurvey;
    fn get_url(&self) -> String {"/lol-pft/v2/survey".to_string()}
}

pub fn get_lol_pft_v2_survey() -> GetLolPftV2Survey {
    GetLolPftV2Survey{}
}


pub struct PostLolPftV2Events {
    pub body: LolPftPftEvent,
}

impl IsApiRequest for PostLolPftV2Events {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-pft/v2/events".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_pft_v2_events(body: LolPftPftEvent) -> PostLolPftV2Events {
    PostLolPftV2Events{body}
}


pub struct PostLolPftV2Survey {
    pub body: LolPftPftSurvey,
}

impl IsApiRequest for PostLolPftV2Survey {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-pft/v2/survey".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_pft_v2_survey(body: LolPftPftSurvey) -> PostLolPftV2Survey {
    PostLolPftV2Survey{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftEvent {
    #[serde(rename = "playerSurveyId")]
    pub player_survey_id: u64,
    pub action: String,
    pub data: Vec<Value>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftSurvey {
    pub id: u64,
    pub title: String,
    pub caption: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub display: String,
    pub data: HashMap<String, Value>,
}


// ENUMS

