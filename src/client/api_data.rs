use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::query::IsQuery;



pub trait IsApiData {
    fn to_path_string(&self, url: String) -> Result<String, String>;
    fn get_body(&self) -> Option<Value>;
}

pub struct ApiDataQuery<Q: IsQuery> {
    pub query: Q,
}

impl<Q: IsQuery> IsApiData for ApiDataQuery<Q> {
    fn to_path_string(&self, url: String) -> Result<String, String> {
        self.query.to_path_string(url)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }
}


pub struct ApiDataBody<B: Serialize + ?Sized> {
    pub body: B,
}

impl<B: Serialize> IsApiData for ApiDataBody<B> {
    fn to_path_string(&self, url: String) -> Result<String, String> {
        Ok(url)
    }

    fn get_body(&self) -> Option<Value> {
        serde_json::to_value(&self.body).ok()
    }
}


pub struct ApiDataQueryBody<Q: IsQuery, B: Serialize + ?Sized> {
    pub query: Q,
    pub body: B,
}

impl<Q: IsQuery, B: Serialize> IsApiData for ApiDataQueryBody<Q, B> {
    fn to_path_string(&self, url: String) -> Result<String, String> {
        self.query.to_path_string(url)
    }

    fn get_body(&self) -> Option<Value> {
        serde_json::to_value(&self.body).ok()
    }
}
