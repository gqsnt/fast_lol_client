
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct DeleteLolSimpleDialogMessagesV1MessagesByMessageId {

    pub message_id: i64,
}

impl IsApiRequest for DeleteLolSimpleDialogMessagesV1MessagesByMessageId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-simple-dialog-messages/v1/messages/{}", self.message_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_simple_dialog_messages_v_1_messages_by_message_id(message_id: i64) -> DeleteLolSimpleDialogMessagesV1MessagesByMessageId {
    DeleteLolSimpleDialogMessagesV1MessagesByMessageId {
        message_id
    }
}


pub struct GetLolSimpleDialogMessagesV1Messages {

}

impl IsApiRequest for GetLolSimpleDialogMessagesV1Messages {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolSimpleDialogMessagesMessage>;

    fn get_url(&self) -> String {
        "/lol-simple-dialog-messages/v1/messages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_simple_dialog_messages_v_1_messages() -> GetLolSimpleDialogMessagesV1Messages {
    GetLolSimpleDialogMessagesV1Messages {
        
    }
}


pub struct PostLolSimpleDialogMessagesV1Messages {

    pub body: LolSimpleDialogMessagesLocalMessageRequest,
}

impl IsApiRequest for PostLolSimpleDialogMessagesV1Messages {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-simple-dialog-messages/v1/messages".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_simple_dialog_messages_v_1_messages(body: LolSimpleDialogMessagesLocalMessageRequest) -> PostLolSimpleDialogMessagesV1Messages {
    PostLolSimpleDialogMessagesV1Messages {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesMessage {
    pub id: i64,
    pub type_: String,
    pub body: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesLocalMessageRequest {
    pub msg_type: String,
    pub msg_body: Vec<String>,
}


// ENUMS

