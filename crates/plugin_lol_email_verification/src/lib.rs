
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolEmailVerificationV1Email {

}

impl IsApiRequest for GetLolEmailVerificationV1Email {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEmailVerificationEmailVerificationSession;

    fn get_url(&self) -> String {
        "/lol-email-verification/v1/email".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_email_verification_v_1_email() -> GetLolEmailVerificationV1Email {
    GetLolEmailVerificationV1Email {
        
    }
}


pub struct PutLolEmailVerificationV1Email {

    pub body: LolEmailVerificationEmailUpdate,
}

impl IsApiRequest for PutLolEmailVerificationV1Email {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-email-verification/v1/email".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_email_verification_v_1_email(body: LolEmailVerificationEmailUpdate) -> PutLolEmailVerificationV1Email {
    PutLolEmailVerificationV1Email {
        body
    }
}


pub struct PostLolEmailVerificationV1ConfirmEmail {

}

impl IsApiRequest for PostLolEmailVerificationV1ConfirmEmail {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-email-verification/v1/confirm-email".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_email_verification_v_1_confirm_email() -> PostLolEmailVerificationV1ConfirmEmail {
    PostLolEmailVerificationV1ConfirmEmail {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationEmailVerificationSession {
    pub email: String,
    pub email_verified: bool,
    pub fatal_error: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationEmailUpdate {
    pub email: String,
    pub password: String,
}


// ENUMS

