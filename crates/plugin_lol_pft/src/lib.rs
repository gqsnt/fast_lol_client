
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolPftV2Survey {

}

impl IsApiRequest for GetLolPftV2Survey {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPftPftSurvey;

    fn get_url(&self) -> String {
        "/lol-pft/v2/survey".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_pft_v_2_survey() -> GetLolPftV2Survey {
    GetLolPftV2Survey {
        
    }
}


pub struct PostLolPftV2Survey {

    pub body: LolPftPftSurvey,
}

impl IsApiRequest for PostLolPftV2Survey {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-pft/v2/survey".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_pft_v_2_survey(body: LolPftPftSurvey) -> PostLolPftV2Survey {
    PostLolPftV2Survey {
        body
    }
}


pub struct PostLolPftV2Events {

    pub body: LolPftPftEvent,
}

impl IsApiRequest for PostLolPftV2Events {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-pft/v2/events".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_pft_v_2_events(body: LolPftPftEvent) -> PostLolPftV2Events {
    PostLolPftV2Events {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftEvent {
    pub player_survey_id: u64,
    pub action: String,
    pub data: Vec<HashMap<String, String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftSurvey {
    pub id: u64,
    pub title: String,
    pub caption: String,
    pub type_: String,
    pub display: String,
    pub data: HashMap<String, HashMap<String, String>>,
}


// ENUMS

