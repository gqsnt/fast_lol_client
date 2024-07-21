
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct AsyncDelete {
    // Cancels the asynchronous operation or removes its completion status.
    pub body: u32,
}

impl IsApiRequest for AsyncDelete {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/AsyncDelete".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn async_delete(body: u32) -> AsyncDelete {
    AsyncDelete {
        body
    }
}


pub struct AsyncResult {
    // Retrieves the result of a completed asynchronous operation.
    pub body: u32,
}

impl IsApiRequest for AsyncResult {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/AsyncResult".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn async_result(body: u32) -> AsyncResult {
    AsyncResult {
        body
    }
}


pub struct AsyncStatus {
    // Retrieves details on the current state of an asynchronous operation.
    pub body: u32,
}

impl IsApiRequest for AsyncStatus {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/AsyncStatus".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn async_status(body: u32) -> AsyncStatus {
    AsyncStatus {
        body
    }
}


pub struct Cancel {
    // Attempts to cancel an asynchronous operation
    pub body: u32,
}

impl IsApiRequest for Cancel {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/Cancel".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn cancel(body: u32) -> Cancel {
    Cancel {
        body
    }
}


pub struct Exit {
    // Closes the connection.
}

impl IsApiRequest for Exit {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/Exit".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn exit() -> Exit {
    Exit {
        
    }
}


pub struct Help {
    // Returns information on available functions and types
    pub target: String,
    pub format: RemotingHelpFormat,
}

impl IsApiRequest for Help {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/Help".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "target" : self.target,
            "format" : self.format,
        }))
    }
}

pub fn help(target: String, format: RemotingHelpFormat) -> Help {
    Help {
        target, format
    }
}


pub struct HttpAsyncStatus {
    // Retrieves details on the current state of an asynchronous operation.
    pub async_token: u32,
}

impl IsApiRequest for HttpAsyncStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/async/v1/status/{}", self.async_token)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn http_async_status(async_token: u32) -> HttpAsyncStatus {
    HttpAsyncStatus {
        async_token
    }
}


pub struct HttpAsyncDelete {
    // Cancels the asynchronous operation or removes its completion status.
    pub async_token: u32,
}

impl IsApiRequest for HttpAsyncDelete {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/async/v1/status/{}", self.async_token)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn http_async_delete(async_token: u32) -> HttpAsyncDelete {
    HttpAsyncDelete {
        async_token
    }
}


pub struct HttpAsyncResult {
    // Retrieves the result of a completed asynchronous operation.
    pub async_token: u32,
}

impl IsApiRequest for HttpAsyncResult {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/async/v1/result/{}", self.async_token)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn http_async_result(async_token: u32) -> HttpAsyncResult {
    HttpAsyncResult {
        async_token
    }
}


pub struct Subscribe {
    // Subscribes to a given event
    pub event_name: String,
    pub format: RemotingSerializedFormat,
}

impl IsApiRequest for Subscribe {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/Subscribe".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "eventName" : self.event_name,
            "format" : self.format,
        }))
    }
}

pub fn subscribe(event_name: String, format: RemotingSerializedFormat) -> Subscribe {
    Subscribe {
        event_name, format
    }
}


pub struct Unsubscribe {
    // Unsubscribes from a given event
    pub body: String,
}

impl IsApiRequest for Unsubscribe {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/Unsubscribe".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn unsubscribe(body: String) -> Unsubscribe {
    Unsubscribe {
        body
    }
}


pub struct WebSocketFormat {
    // Controls the console output format
    pub body: RemotingSerializedFormat,
}

impl IsApiRequest for WebSocketFormat {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/WebSocketFormat".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn web_socket_format(body: RemotingSerializedFormat) -> WebSocketFormat {
    WebSocketFormat {
        body
    }
}


// OBJECTS


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum RemotingHelpFormat {
    #[default]
    Console,
    Brief,
    Epytext,
    Full,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum RemotingSerializedFormat {
    #[default]
    MsgPack,
    #[serde(rename = "YAML")]
    Yaml,
    #[serde(rename = "JSON")]
    Json,
}

