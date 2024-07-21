
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolPlayerReportSenderV1ReportedPlayersGameIdByGameId {
    pub game_id: u64,
}

impl IsApiRequest for DeleteLolPlayerReportSenderV1ReportedPlayersGameIdByGameId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {format!("/lol-player-report-sender/v1/reported-players/gameId/{}", self.game_id)}
}

pub fn delete_lol_player_report_sender_v_1_reported_players_game_id_by_game_id(game_id: u64) -> DeleteLolPlayerReportSenderV1ReportedPlayersGameIdByGameId {
    DeleteLolPlayerReportSenderV1ReportedPlayersGameIdByGameId{game_id}
}


pub struct GetLolPlayerReportSenderV1GameIdsWithVerbalAbuseReport {}

impl IsApiRequest for GetLolPlayerReportSenderV1GameIdsWithVerbalAbuseReport {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<u64>;
    fn get_url(&self) -> String {"/lol-player-report-sender/v1/game-ids-with-verbal-abuse-report".to_string()}
}

pub fn get_lol_player_report_sender_v_1_game_ids_with_verbal_abuse_report() -> GetLolPlayerReportSenderV1GameIdsWithVerbalAbuseReport {
    GetLolPlayerReportSenderV1GameIdsWithVerbalAbuseReport{}
}


pub struct GetLolPlayerReportSenderV1InGameReports {}

impl IsApiRequest for GetLolPlayerReportSenderV1InGameReports {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPlayerReportSenderPlayerReport>;
    fn get_url(&self) -> String {"/lol-player-report-sender/v1/in-game-reports".to_string()}
}

pub fn get_lol_player_report_sender_v_1_in_game_reports() -> GetLolPlayerReportSenderV1InGameReports {
    GetLolPlayerReportSenderV1InGameReports{}
}


pub struct GetLolPlayerReportSenderV1ReportedPlayersGameIdByGameId {
    pub game_id: u64,
}

impl IsApiRequest for GetLolPlayerReportSenderV1ReportedPlayersGameIdByGameId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {format!("/lol-player-report-sender/v1/reported-players/gameId/{}", self.game_id)}
}

pub fn get_lol_player_report_sender_v_1_reported_players_game_id_by_game_id(game_id: u64) -> GetLolPlayerReportSenderV1ReportedPlayersGameIdByGameId {
    GetLolPlayerReportSenderV1ReportedPlayersGameIdByGameId{game_id}
}


pub struct PostLolPlayerReportSenderV1ChampSelectReports {
    pub body: LolPlayerReportSenderPlayerReport,
}

impl IsApiRequest for PostLolPlayerReportSenderV1ChampSelectReports {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-player-report-sender/v1/champ-select-reports".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_player_report_sender_v_1_champ_select_reports(body: LolPlayerReportSenderPlayerReport) -> PostLolPlayerReportSenderV1ChampSelectReports {
    PostLolPlayerReportSenderV1ChampSelectReports{body}
}


pub struct PostLolPlayerReportSenderV1EndOfGameReports {
    pub body: LolPlayerReportSenderPlayerReport,
}

impl IsApiRequest for PostLolPlayerReportSenderV1EndOfGameReports {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-player-report-sender/v1/end-of-game-reports".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_player_report_sender_v_1_end_of_game_reports(body: LolPlayerReportSenderPlayerReport) -> PostLolPlayerReportSenderV1EndOfGameReports {
    PostLolPlayerReportSenderV1EndOfGameReports{body}
}


pub struct PostLolPlayerReportSenderV1InGameReports {
    pub body: LolPlayerReportSenderPlayerReport,
}

impl IsApiRequest for PostLolPlayerReportSenderV1InGameReports {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-player-report-sender/v1/in-game-reports".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_player_report_sender_v_1_in_game_reports(body: LolPlayerReportSenderPlayerReport) -> PostLolPlayerReportSenderV1InGameReports {
    PostLolPlayerReportSenderV1InGameReports{body}
}


pub struct PostLolPlayerReportSenderV1MatchHistoryReports {
    pub body: LolPlayerReportSenderPlayerReport,
}

impl IsApiRequest for PostLolPlayerReportSenderV1MatchHistoryReports {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-player-report-sender/v1/match-history-reports".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_player_report_sender_v_1_match_history_reports(body: LolPlayerReportSenderPlayerReport) -> PostLolPlayerReportSenderV1MatchHistoryReports {
    PostLolPlayerReportSenderV1MatchHistoryReports{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderPlayerReport {
    pub offender_puuid: String,
    pub obfuscated_offender_puuid: String,
    pub categories: Vec<String>,
    pub game_id: u64,
    pub offender_summoner_id: u64,
    pub comment: String,
}


// ENUMS

