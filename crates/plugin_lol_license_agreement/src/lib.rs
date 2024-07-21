
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolLicenseAgreementV1Agreement {

}

impl IsApiRequest for GetLolLicenseAgreementV1Agreement {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-license-agreement/v1/agreement".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_license_agreement_v_1_agreement() -> GetLolLicenseAgreementV1Agreement {
    GetLolLicenseAgreementV1Agreement {
        
    }
}


pub struct GetLolLicenseAgreementV1Agreements {

}

impl IsApiRequest for GetLolLicenseAgreementV1Agreements {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLicenseAgreementLicenseAgreement>;

    fn get_url(&self) -> String {
        "/lol-license-agreement/v1/agreements".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_license_agreement_v_1_agreements() -> GetLolLicenseAgreementV1Agreements {
    GetLolLicenseAgreementV1Agreements {
        
    }
}


pub struct GetLolLicenseAgreementV1AllAgreements {

}

impl IsApiRequest for GetLolLicenseAgreementV1AllAgreements {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLicenseAgreementLicenseAgreement>;

    fn get_url(&self) -> String {
        "/lol-license-agreement/v1/all-agreements".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_license_agreement_v_1_all_agreements() -> GetLolLicenseAgreementV1AllAgreements {
    GetLolLicenseAgreementV1AllAgreements {
        
    }
}


pub struct GetLolLicenseAgreementV1PrivacyPolicy {

}

impl IsApiRequest for GetLolLicenseAgreementV1PrivacyPolicy {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-license-agreement/v1/privacy-policy".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_license_agreement_v_1_privacy_policy() -> GetLolLicenseAgreementV1PrivacyPolicy {
    GetLolLicenseAgreementV1PrivacyPolicy {
        
    }
}


pub struct GetLolLicenseAgreementV1ServeLocation {

}

impl IsApiRequest for GetLolLicenseAgreementV1ServeLocation {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLicenseAgreementLicenseServeLocation;

    fn get_url(&self) -> String {
        "/lol-license-agreement/v1/serve-location".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_license_agreement_v_1_serve_location() -> GetLolLicenseAgreementV1ServeLocation {
    GetLolLicenseAgreementV1ServeLocation {
        
    }
}


pub struct PostLolLicenseAgreementV1AgreementsByIdAccept {

    pub id: String,
}

impl IsApiRequest for PostLolLicenseAgreementV1AgreementsByIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-license-agreement/v1/agreements/{}/accept", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_license_agreement_v_1_agreements_by_id_accept(id: String) -> PostLolLicenseAgreementV1AgreementsByIdAccept {
    PostLolLicenseAgreementV1AgreementsByIdAccept {
        id
    }
}


pub struct PostLolLicenseAgreementV1AgreementsByIdDecline {

    pub id: String,
}

impl IsApiRequest for PostLolLicenseAgreementV1AgreementsByIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-license-agreement/v1/agreements/{}/decline", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_license_agreement_v_1_agreements_by_id_decline(id: String) -> PostLolLicenseAgreementV1AgreementsByIdDecline {
    PostLolLicenseAgreementV1AgreementsByIdDecline {
        id
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLicenseAgreementLicenseAgreement {
    pub id: String,
    pub license_type: LolLicenseAgreementLicenseAgreementType,
    pub text: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLicenseAgreementLicenseAgreementType {
    #[default]
    TermsOfUse,
    PrivacyNotice,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLicenseAgreementLicenseServeLocation {
    #[default]
    External,
    Local,
    Preparing,
}

