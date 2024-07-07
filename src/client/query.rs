use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdQuery {
    pub id: u32,
}

impl HasQuery for IdQuery{}

pub fn id_query(id: u32) -> IdQuery {
    IdQuery { id }
}