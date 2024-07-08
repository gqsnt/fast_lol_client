use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;



pub trait IsQuery: Serialize+DeserializeOwned {
    fn to_path_string(&self, url: String) -> Result<String, String>;
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id {
    pub id: u32,
}

impl IsQuery for Id {
    fn to_path_string(&self, url: String) -> Result<String, String>{
        if url.contains("{id}") {
           Ok(url.replace("{id}", &self.id.to_string()))
        } else {
            Err("Placeholder {id} not found in url".to_string())
        }
    }
}

