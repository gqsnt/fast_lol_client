
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolSimpleDialogMessagesV1MessagesByMessageId {
    pub message_id: i64,
}

impl IsApiRequest for DeleteLolSimpleDialogMessagesV1MessagesByMessageId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-simple-dialog-messages/v1/messages/{}", self.message_id)}
}

pub fn delete_lol_simple_dialog_messages_v1_messages_by_message_id(message_id: i64) -> DeleteLolSimpleDialogMessagesV1MessagesByMessageId {
    DeleteLolSimpleDialogMessagesV1MessagesByMessageId{message_id}
}


pub struct GetLolSimpleDialogMessagesV1Messages {}

impl IsApiRequest for GetLolSimpleDialogMessagesV1Messages {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolSimpleDialogMessagesMessage>;
    fn get_url(&self) -> String {"/lol-simple-dialog-messages/v1/messages".to_string()}
}

pub fn get_lol_simple_dialog_messages_v1_messages() -> GetLolSimpleDialogMessagesV1Messages {
    GetLolSimpleDialogMessagesV1Messages{}
}


pub struct PostLolSimpleDialogMessagesV1Messages {
    pub body: LolSimpleDialogMessagesLocalMessageRequest,
}

impl IsApiRequest for PostLolSimpleDialogMessagesV1Messages {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-simple-dialog-messages/v1/messages".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_simple_dialog_messages_v1_messages(body: LolSimpleDialogMessagesLocalMessageRequest) -> PostLolSimpleDialogMessagesV1Messages {
    PostLolSimpleDialogMessagesV1Messages{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesLocalMessageRequest {
    #[serde(rename = "msgType")]
    pub msg_type: String,
    #[serde(rename = "msgBody")]
    pub msg_body: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesMessage {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub body: Value,
}


// ENUMS

