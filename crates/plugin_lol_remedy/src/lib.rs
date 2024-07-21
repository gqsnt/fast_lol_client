
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolRemedyV1ConfigIsVerbalAbuseRemedyModalEnabled {

}

impl IsApiRequest for GetLolRemedyV1ConfigIsVerbalAbuseRemedyModalEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-remedy/v1/config/is-verbal-abuse-remedy-modal-enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_remedy_v_1_config_is_verbal_abuse_remedy_modal_enabled() -> GetLolRemedyV1ConfigIsVerbalAbuseRemedyModalEnabled {
    GetLolRemedyV1ConfigIsVerbalAbuseRemedyModalEnabled {
        
    }
}


pub struct GetLolRemedyV1RemedyNotifications {

}

impl IsApiRequest for GetLolRemedyV1RemedyNotifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolRemedyMail>;

    fn get_url(&self) -> String {
        "/lol-remedy/v1/remedy-notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_remedy_v_1_remedy_notifications() -> GetLolRemedyV1RemedyNotifications {
    GetLolRemedyV1RemedyNotifications {
        
    }
}


pub struct PutLolRemedyV1AckRemedyNotificationByMailId {

    pub mail_id: String,
}

impl IsApiRequest for PutLolRemedyV1AckRemedyNotificationByMailId {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-remedy/v1/ack-remedy-notification/{}", self.mail_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_remedy_v_1_ack_remedy_notification_by_mail_id(mail_id: String) -> PutLolRemedyV1AckRemedyNotificationByMailId {
    PutLolRemedyV1AckRemedyNotificationByMailId {
        mail_id
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolRemedyMail {
    pub mail_id: String,
    pub message: String,
    pub state: String,
    pub created_at: u64,
}


// ENUMS

