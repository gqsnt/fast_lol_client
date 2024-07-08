use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;



pub trait UrlParameters: Serialize+DeserializeOwned {
    fn get_path(&self,url:String) -> String;
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id {
    pub id: u32,
}

impl UrlParameters for Id {
    fn get_path(&self, url: String) -> String {
        url.replace("{id}", &self.id.to_string())
    }
}

