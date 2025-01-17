
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolPlayerBehaviorV1CodeOfConductNotification {}

impl IsApiRequest for DeleteLolPlayerBehaviorV1CodeOfConductNotification {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/code-of-conduct-notification".to_string()}
}

pub fn delete_lol_player_behavior_v1_code_of_conduct_notification() -> DeleteLolPlayerBehaviorV1CodeOfConductNotification {
    DeleteLolPlayerBehaviorV1CodeOfConductNotification{}
}


pub struct DeleteLolPlayerBehaviorV1ReporterFeedbackById {
    pub id: String,
}

impl IsApiRequest for DeleteLolPlayerBehaviorV1ReporterFeedbackById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = LolPlayerBehaviorReporterFeedback;
    fn get_url(&self) -> String {format!("/lol-player-behavior/v1/reporter-feedback/{}", self.id)}
}

pub fn delete_lol_player_behavior_v1_reporter_feedback_by_id(id: String) -> DeleteLolPlayerBehaviorV1ReporterFeedbackById {
    DeleteLolPlayerBehaviorV1ReporterFeedbackById{id}
}


pub struct GetLolPlayerBehaviorV1Ban {}

impl IsApiRequest for GetLolPlayerBehaviorV1Ban {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerBehaviorBanNotification;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/ban".to_string()}
}

pub fn get_lol_player_behavior_v1_ban() -> GetLolPlayerBehaviorV1Ban {
    GetLolPlayerBehaviorV1Ban{}
}


pub struct GetLolPlayerBehaviorV1ChatRestriction {}

impl IsApiRequest for GetLolPlayerBehaviorV1ChatRestriction {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerBehaviorRestrictionNotification;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/chat-restriction".to_string()}
}

pub fn get_lol_player_behavior_v1_chat_restriction() -> GetLolPlayerBehaviorV1ChatRestriction {
    GetLolPlayerBehaviorV1ChatRestriction{}
}


pub struct GetLolPlayerBehaviorV1CodeOfConductNotification {}

impl IsApiRequest for GetLolPlayerBehaviorV1CodeOfConductNotification {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerBehaviorCodeOfConductNotification;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/code-of-conduct-notification".to_string()}
}

pub fn get_lol_player_behavior_v1_code_of_conduct_notification() -> GetLolPlayerBehaviorV1CodeOfConductNotification {
    GetLolPlayerBehaviorV1CodeOfConductNotification{}
}


pub struct GetLolPlayerBehaviorV1Config {}

impl IsApiRequest for GetLolPlayerBehaviorV1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerBehaviorPlayerBehaviorConfig;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/config".to_string()}
}

pub fn get_lol_player_behavior_v1_config() -> GetLolPlayerBehaviorV1Config {
    GetLolPlayerBehaviorV1Config{}
}


pub struct GetLolPlayerBehaviorV1CredibilityBehaviorWarnings {}

impl IsApiRequest for GetLolPlayerBehaviorV1CredibilityBehaviorWarnings {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/credibility-behavior-warnings".to_string()}
}

pub fn get_lol_player_behavior_v1_credibility_behavior_warnings() -> GetLolPlayerBehaviorV1CredibilityBehaviorWarnings {
    GetLolPlayerBehaviorV1CredibilityBehaviorWarnings{}
}


pub struct GetLolPlayerBehaviorV1ReformCard {}

impl IsApiRequest for GetLolPlayerBehaviorV1ReformCard {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerBehaviorReformCard;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/reform-card".to_string()}
}

pub fn get_lol_player_behavior_v1_reform_card() -> GetLolPlayerBehaviorV1ReformCard {
    GetLolPlayerBehaviorV1ReformCard{}
}


pub struct GetLolPlayerBehaviorV1ReporterFeedback {}

impl IsApiRequest for GetLolPlayerBehaviorV1ReporterFeedback {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPlayerBehaviorReporterFeedback>;
    fn get_url(&self) -> String {"/lol-player-behavior/v1/reporter-feedback".to_string()}
}

pub fn get_lol_player_behavior_v1_reporter_feedback() -> GetLolPlayerBehaviorV1ReporterFeedback {
    GetLolPlayerBehaviorV1ReporterFeedback{}
}


pub struct GetLolPlayerBehaviorV1ReporterFeedbackById {
    pub id: String,
}

impl IsApiRequest for GetLolPlayerBehaviorV1ReporterFeedbackById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerBehaviorReporterFeedback;
    fn get_url(&self) -> String {format!("/lol-player-behavior/v1/reporter-feedback/{}", self.id)}
}

pub fn get_lol_player_behavior_v1_reporter_feedback_by_id(id: String) -> GetLolPlayerBehaviorV1ReporterFeedbackById {
    GetLolPlayerBehaviorV1ReporterFeedbackById{id}
}


pub struct GetLolPlayerBehaviorV2ReformCard {}

impl IsApiRequest for GetLolPlayerBehaviorV2ReformCard {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerBehaviorReformCardV2;
    fn get_url(&self) -> String {"/lol-player-behavior/v2/reform-card".to_string()}
}

pub fn get_lol_player_behavior_v2_reform_card() -> GetLolPlayerBehaviorV2ReformCard {
    GetLolPlayerBehaviorV2ReformCard{}
}


pub struct GetLolPlayerBehaviorV2ReporterFeedback {}

impl IsApiRequest for GetLolPlayerBehaviorV2ReporterFeedback {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPlayerBehaviorReporterFeedbackMessage>;
    fn get_url(&self) -> String {"/lol-player-behavior/v2/reporter-feedback".to_string()}
}

pub fn get_lol_player_behavior_v2_reporter_feedback() -> GetLolPlayerBehaviorV2ReporterFeedback {
    GetLolPlayerBehaviorV2ReporterFeedback{}
}


pub struct GetLolPlayerBehaviorV3ReformCards {}

impl IsApiRequest for GetLolPlayerBehaviorV3ReformCards {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-player-behavior/v3/reform-cards".to_string()}
}

pub fn get_lol_player_behavior_v3_reform_cards() -> GetLolPlayerBehaviorV3ReformCards {
    GetLolPlayerBehaviorV3ReformCards{}
}


pub struct PostLolPlayerBehaviorV2ReporterFeedbackByKey {
    pub key: String,
}

impl IsApiRequest for PostLolPlayerBehaviorV2ReporterFeedbackByKey {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-player-behavior/v2/reporter-feedback/{}", self.key)}
}

pub fn post_lol_player_behavior_v2_reporter_feedback_by_key(key: String) -> PostLolPlayerBehaviorV2ReporterFeedbackByKey {
    PostLolPlayerBehaviorV2ReporterFeedbackByKey{key}
}


pub struct PutLolPlayerBehaviorV1AckCredibilityBehaviorWarningByMailId {
    pub mail_id: String,
}

impl IsApiRequest for PutLolPlayerBehaviorV1AckCredibilityBehaviorWarningByMailId {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-player-behavior/v1/ack-credibility-behavior-warning/{}", self.mail_id)}
}

pub fn put_lol_player_behavior_v1_ack_credibility_behavior_warning_by_mail_id(mail_id: String) -> PutLolPlayerBehaviorV1AckCredibilityBehaviorWarningByMailId {
    PutLolPlayerBehaviorV1AckCredibilityBehaviorWarningByMailId{mail_id}
}


pub struct PutLolPlayerBehaviorV3ReformCardById {
    pub id: String,
}

impl IsApiRequest for PutLolPlayerBehaviorV3ReformCardById {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-player-behavior/v3/reform-card/{}", self.id)}
}

pub fn put_lol_player_behavior_v3_reform_card_by_id(id: String) -> PutLolPlayerBehaviorV3ReformCardById {
    PutLolPlayerBehaviorV3ReformCardById{id}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorBanNotification {
    pub id: u64,
    pub source: LolPlayerBehaviorNotificationSource,
    pub reason: String,
    #[serde(rename = "timeUntilBanExpires")]
    pub time_until_ban_expires: u64,
    #[serde(rename = "isPermaBan")]
    pub is_perma_ban: bool,
    #[serde(rename = "displayReformCard")]
    pub display_reform_card: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorCodeOfConductNotification {
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorPlayerBehaviorConfig {
    #[serde(rename = "IsLoaded")]
    pub is_loaded: bool,
    #[serde(rename = "CodeOfConductEnabled")]
    pub code_of_conduct_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCard {
    pub id: u64,
    #[serde(rename = "punishmentType")]
    pub punishment_type: String,
    pub reason: String,
    #[serde(rename = "timeWhenPunishmentExpires")]
    pub time_when_punishment_expires: u64,
    #[serde(rename = "punishmentLengthTime")]
    pub punishment_length_time: u64,
    #[serde(rename = "punishmentLengthGames")]
    pub punishment_length_games: i64,
    #[serde(rename = "restrictedChatGamesRemaining")]
    pub restricted_chat_games_remaining: i64,
    #[serde(rename = "chatLogs")]
    pub chat_logs: Vec<String>,
    #[serde(rename = "gameIds")]
    pub game_ids: Vec<u64>,
    #[serde(rename = "playerFacingMessage")]
    pub player_facing_message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCardChatLogs {
    #[serde(rename = "preGameChatLogs")]
    pub pre_game_chat_logs: Vec<String>,
    #[serde(rename = "inGameChatLogs")]
    pub in_game_chat_logs: Vec<String>,
    #[serde(rename = "postGameChatLogs")]
    pub post_game_chat_logs: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCardV2 {
    pub id: u64,
    #[serde(rename = "punishmentType")]
    pub punishment_type: String,
    #[serde(rename = "punishmentReason")]
    pub punishment_reason: String,
    #[serde(rename = "punishedUntilDateMillis")]
    pub punished_until_date_millis: u64,
    #[serde(rename = "punishmentLengthMillis")]
    pub punishment_length_millis: u64,
    #[serde(rename = "punishmentLengthGames")]
    pub punishment_length_games: i64,
    #[serde(rename = "punishedForReformCardChatLogs")]
    pub punished_for_reform_card_chat_logs: Vec<LolPlayerBehaviorReformCardChatLogs>,
    #[serde(rename = "punishedForGameIds")]
    pub punished_for_game_ids: Vec<u64>,
    #[serde(rename = "playerFacingMessage")]
    pub player_facing_message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReporterFeedback {
    pub id: u64,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReporterFeedbackMessage {
    pub title: String,
    pub message: String,
    pub category: String,
    pub locale: String,
    pub key: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorRestrictionNotification {
    pub id: u64,
    pub source: LolPlayerBehaviorNotificationSource,
    #[serde(rename = "gamesRemaining")]
    pub games_remaining: i64,
    #[serde(rename = "expirationMillis")]
    pub expiration_millis: u64,
    #[serde(rename = "displayReformCard")]
    pub display_reform_card: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolPlayerBehaviorNotificationSource {
    #[default]
    Message,
    ForcedShutdown,
    Login,
    Invalid,
}

