
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolSummonerV1AliasLookup {
    pub game_name: String,
    pub tag_line: String,
}

impl IsApiRequest for GetLolSummonerV1AliasLookup {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerAliasLookupResponse;
    fn get_url(&self) -> String {"/lol-summoner/v1/alias/lookup".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("gameName".to_string(), serde_json::to_string(&self.game_name).unwrap()),
            ("tagLine".to_string(), serde_json::to_string(&self.tag_line).unwrap())
        ])
    }
}

pub fn get_lol_summoner_v1_alias_lookup(game_name: String, tag_line: String) -> GetLolSummonerV1AliasLookup {
    GetLolSummonerV1AliasLookup{game_name, tag_line}
}


pub struct GetLolSummonerV1CheckNameAvailabilityByName {
    pub name: String,
}

impl IsApiRequest for GetLolSummonerV1CheckNameAvailabilityByName {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {format!("/lol-summoner/v1/check-name-availability/{}", self.name)}
}

pub fn get_lol_summoner_v1_check_name_availability_by_name(name: String) -> GetLolSummonerV1CheckNameAvailabilityByName {
    GetLolSummonerV1CheckNameAvailabilityByName{name}
}


pub struct GetLolSummonerV1CheckNameAvailabilityNewSummonersByName {
    pub name: String,
}

impl IsApiRequest for GetLolSummonerV1CheckNameAvailabilityNewSummonersByName {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {format!("/lol-summoner/v1/check-name-availability-new-summoners/{}", self.name)}
}

pub fn get_lol_summoner_v1_check_name_availability_new_summoners_by_name(name: String) -> GetLolSummonerV1CheckNameAvailabilityNewSummonersByName {
    GetLolSummonerV1CheckNameAvailabilityNewSummonersByName{name}
}


pub struct GetLolSummonerV1CurrentSummoner {}

impl IsApiRequest for GetLolSummonerV1CurrentSummoner {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner".to_string()}
}

pub fn get_lol_summoner_v1_current_summoner() -> GetLolSummonerV1CurrentSummoner {
    GetLolSummonerV1CurrentSummoner{}
}


pub struct GetLolSummonerV1CurrentSummonerAccountAndSummonerIds {}

impl IsApiRequest for GetLolSummonerV1CurrentSummonerAccountAndSummonerIds {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerAccountIdAndSummonerId;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/account-and-summoner-ids".to_string()}
}

pub fn get_lol_summoner_v1_current_summoner_account_and_summoner_ids() -> GetLolSummonerV1CurrentSummonerAccountAndSummonerIds {
    GetLolSummonerV1CurrentSummonerAccountAndSummonerIds{}
}


pub struct GetLolSummonerV1CurrentSummonerAutofill {}

impl IsApiRequest for GetLolSummonerV1CurrentSummonerAutofill {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolSummonerAutoFillQueueDto>;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/autofill".to_string()}
}

pub fn get_lol_summoner_v1_current_summoner_autofill() -> GetLolSummonerV1CurrentSummonerAutofill {
    GetLolSummonerV1CurrentSummonerAutofill{}
}


pub struct GetLolSummonerV1CurrentSummonerJwt {}

impl IsApiRequest for GetLolSummonerV1CurrentSummonerJwt {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/jwt".to_string()}
}

pub fn get_lol_summoner_v1_current_summoner_jwt() -> GetLolSummonerV1CurrentSummonerJwt {
    GetLolSummonerV1CurrentSummonerJwt{}
}


pub struct GetLolSummonerV1CurrentSummonerProfilePrivacy {}

impl IsApiRequest for GetLolSummonerV1CurrentSummonerProfilePrivacy {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerProfilePrivacy;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/profile-privacy".to_string()}
}

pub fn get_lol_summoner_v1_current_summoner_profile_privacy() -> GetLolSummonerV1CurrentSummonerProfilePrivacy {
    GetLolSummonerV1CurrentSummonerProfilePrivacy{}
}


pub struct GetLolSummonerV1CurrentSummonerRerollPoints {}

impl IsApiRequest for GetLolSummonerV1CurrentSummonerRerollPoints {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerSummonerRerollPoints;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/rerollPoints".to_string()}
}

pub fn get_lol_summoner_v1_current_summoner_reroll_points() -> GetLolSummonerV1CurrentSummonerRerollPoints {
    GetLolSummonerV1CurrentSummonerRerollPoints{}
}


pub struct GetLolSummonerV1CurrentSummonerSummonerProfile {}

impl IsApiRequest for GetLolSummonerV1CurrentSummonerSummonerProfile {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/summoner-profile".to_string()}
}

pub fn get_lol_summoner_v1_current_summoner_summoner_profile() -> GetLolSummonerV1CurrentSummonerSummonerProfile {
    GetLolSummonerV1CurrentSummonerSummonerProfile{}
}


pub struct GetLolSummonerV1PlayerAliasState {}

impl IsApiRequest for GetLolSummonerV1PlayerAliasState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerPlayerNameState;
    fn get_url(&self) -> String {"/lol-summoner/v1/player-alias-state".to_string()}
}

pub fn get_lol_summoner_v1_player_alias_state() -> GetLolSummonerV1PlayerAliasState {
    GetLolSummonerV1PlayerAliasState{}
}


pub struct GetLolSummonerV1PlayerNameMode {}

impl IsApiRequest for GetLolSummonerV1PlayerNameMode {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerPlayerNameMode;
    fn get_url(&self) -> String {"/lol-summoner/v1/player-name-mode".to_string()}
}

pub fn get_lol_summoner_v1_player_name_mode() -> GetLolSummonerV1PlayerNameMode {
    GetLolSummonerV1PlayerNameMode{}
}


pub struct GetLolSummonerV1ProfilePrivacyEnabled {}

impl IsApiRequest for GetLolSummonerV1ProfilePrivacyEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerProfilePrivacyEnabledState;
    fn get_url(&self) -> String {"/lol-summoner/v1/profile-privacy-enabled".to_string()}
}

pub fn get_lol_summoner_v1_profile_privacy_enabled() -> GetLolSummonerV1ProfilePrivacyEnabled {
    GetLolSummonerV1ProfilePrivacyEnabled{}
}


pub struct GetLolSummonerV1RiotAliasFreeEligibility {}

impl IsApiRequest for GetLolSummonerV1RiotAliasFreeEligibility {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-summoner/v1/riot-alias-free-eligibility".to_string()}
}

pub fn get_lol_summoner_v1_riot_alias_free_eligibility() -> GetLolSummonerV1RiotAliasFreeEligibility {
    GetLolSummonerV1RiotAliasFreeEligibility{}
}


pub struct GetLolSummonerV1RiotAliasPurchaseEligibility {}

impl IsApiRequest for GetLolSummonerV1RiotAliasPurchaseEligibility {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-summoner/v1/riot-alias-purchase-eligibility".to_string()}
}

pub fn get_lol_summoner_v1_riot_alias_purchase_eligibility() -> GetLolSummonerV1RiotAliasPurchaseEligibility {
    GetLolSummonerV1RiotAliasPurchaseEligibility{}
}


pub struct GetLolSummonerV1Status {}

impl IsApiRequest for GetLolSummonerV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerStatus;
    fn get_url(&self) -> String {"/lol-summoner/v1/status".to_string()}
}

pub fn get_lol_summoner_v1_status() -> GetLolSummonerV1Status {
    GetLolSummonerV1Status{}
}


pub struct GetLolSummonerV1SummonerProfile {
    pub puuid: String,
}

impl IsApiRequest for GetLolSummonerV1SummonerProfile {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-summoner/v1/summoner-profile".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("puuid".to_string(), serde_json::to_string(&self.puuid).unwrap())
        ])
    }
}

pub fn get_lol_summoner_v1_summoner_profile(puuid: String) -> GetLolSummonerV1SummonerProfile {
    GetLolSummonerV1SummonerProfile{puuid}
}


pub struct GetLolSummonerV1SummonerRequestsReady {}

impl IsApiRequest for GetLolSummonerV1SummonerRequestsReady {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-summoner/v1/summoner-requests-ready".to_string()}
}

pub fn get_lol_summoner_v1_summoner_requests_ready() -> GetLolSummonerV1SummonerRequestsReady {
    GetLolSummonerV1SummonerRequestsReady{}
}


pub struct GetLolSummonerV1Summoners {
    pub name: String,
}

impl IsApiRequest for GetLolSummonerV1Summoners {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {"/lol-summoner/v1/summoners".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("name".to_string(), serde_json::to_string(&self.name).unwrap())
        ])
    }
}

pub fn get_lol_summoner_v1_summoners(name: String) -> GetLolSummonerV1Summoners {
    GetLolSummonerV1Summoners{name}
}


pub struct GetLolSummonerV1SummonersById {
    pub id: u64,
}

impl IsApiRequest for GetLolSummonerV1SummonersById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {format!("/lol-summoner/v1/summoners/{}", self.id)}
}

pub fn get_lol_summoner_v1_summoners_by_id(id: u64) -> GetLolSummonerV1SummonersById {
    GetLolSummonerV1SummonersById{id}
}


pub struct GetLolSummonerV1SummonersByPuuidCachedByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolSummonerV1SummonersByPuuidCachedByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {format!("/lol-summoner/v1/summoners-by-puuid-cached/{}", self.puuid)}
}

pub fn get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid(puuid: String) -> GetLolSummonerV1SummonersByPuuidCachedByPuuid {
    GetLolSummonerV1SummonersByPuuidCachedByPuuid{puuid}
}


pub struct GetLolSummonerV2SummonerIcons {
    pub ids: Vec<u64>,
}

impl IsApiRequest for GetLolSummonerV2SummonerIcons {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolSummonerSummonerIdAndIcon>;
    fn get_url(&self) -> String {"/lol-summoner/v2/summoner-icons".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("ids".to_string(), serde_json::to_string(&self.ids).unwrap())
        ])
    }
}

pub fn get_lol_summoner_v2_summoner_icons(ids: Vec<u64>) -> GetLolSummonerV2SummonerIcons {
    GetLolSummonerV2SummonerIcons{ids}
}


pub struct GetLolSummonerV2SummonerNames {
    pub ids: Vec<u64>,
}

impl IsApiRequest for GetLolSummonerV2SummonerNames {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolSummonerSummonerIdAndName>;
    fn get_url(&self) -> String {"/lol-summoner/v2/summoner-names".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("ids".to_string(), serde_json::to_string(&self.ids).unwrap())
        ])
    }
}

pub fn get_lol_summoner_v2_summoner_names(ids: Vec<u64>) -> GetLolSummonerV2SummonerNames {
    GetLolSummonerV2SummonerNames{ids}
}


pub struct GetLolSummonerV2Summoners {
    pub ids: Option<Vec<u64>>,
}

impl IsApiRequest for GetLolSummonerV2Summoners {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolSummonerSummoner>;
    fn get_url(&self) -> String {"/lol-summoner/v2/summoners".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("ids".to_string(), serde_json::to_string(&self.ids).unwrap())
        ])
    }
}

pub fn get_lol_summoner_v2_summoners(ids: Option<Vec<u64>>) -> GetLolSummonerV2Summoners {
    GetLolSummonerV2Summoners{ids}
}


pub struct GetLolSummonerV2SummonersPuuidByPuuid {
    pub puuid: String,
}

impl IsApiRequest for GetLolSummonerV2SummonersPuuidByPuuid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {format!("/lol-summoner/v2/summoners/puuid/{}", self.puuid)}
}

pub fn get_lol_summoner_v2_summoners_puuid_by_puuid(puuid: String) -> GetLolSummonerV2SummonersPuuidByPuuid {
    GetLolSummonerV2SummonersPuuidByPuuid{puuid}
}


pub struct PostLolSummonerV1CurrentSummonerName {
    pub body: String,
}

impl IsApiRequest for PostLolSummonerV1CurrentSummonerName {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/name".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_current_summoner_name(body: String) -> PostLolSummonerV1CurrentSummonerName {
    PostLolSummonerV1CurrentSummonerName{body}
}


pub struct PostLolSummonerV1CurrentSummonerSummonerProfile {
    pub body: LolSummonerSummonerProfileUpdate,
}

impl IsApiRequest for PostLolSummonerV1CurrentSummonerSummonerProfile {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/summoner-profile".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_current_summoner_summoner_profile(body: LolSummonerSummonerProfileUpdate) -> PostLolSummonerV1CurrentSummonerSummonerProfile {
    PostLolSummonerV1CurrentSummonerSummonerProfile{body}
}


pub struct PostLolSummonerV1SaveAlias {
    pub body: LolSummonerAlias,
}

impl IsApiRequest for PostLolSummonerV1SaveAlias {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSummonerAliasAvailability;
    fn get_url(&self) -> String {"/lol-summoner/v1/save-alias".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_save_alias(body: LolSummonerAlias) -> PostLolSummonerV1SaveAlias {
    PostLolSummonerV1SaveAlias{body}
}


pub struct PostLolSummonerV1SummonerAliasesByIds {
    pub body: Vec<u64>,
}

impl IsApiRequest for PostLolSummonerV1SummonerAliasesByIds {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, LolSummonerAlias>;
    fn get_url(&self) -> String {"/lol-summoner/v1/summoner-aliases-by-ids".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_summoner_aliases_by_ids(body: Vec<u64>) -> PostLolSummonerV1SummonerAliasesByIds {
    PostLolSummonerV1SummonerAliasesByIds{body}
}


pub struct PostLolSummonerV1SummonerAliasesByPuuids {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolSummonerV1SummonerAliasesByPuuids {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, LolSummonerAlias>;
    fn get_url(&self) -> String {"/lol-summoner/v1/summoner-aliases-by-puuids".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_summoner_aliases_by_puuids(body: Vec<String>) -> PostLolSummonerV1SummonerAliasesByPuuids {
    PostLolSummonerV1SummonerAliasesByPuuids{body}
}


pub struct PostLolSummonerV1Summoners {
    pub body: LolSummonerSummonerRequestedName,
}

impl IsApiRequest for PostLolSummonerV1Summoners {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {"/lol-summoner/v1/summoners".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_summoners(body: LolSummonerSummonerRequestedName) -> PostLolSummonerV1Summoners {
    PostLolSummonerV1Summoners{body}
}


pub struct PostLolSummonerV1SummonersAliases {
    pub body: Vec<LolSummonerAlias>,
}

impl IsApiRequest for PostLolSummonerV1SummonersAliases {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolSummonerSummoner>;
    fn get_url(&self) -> String {"/lol-summoner/v1/summoners/aliases".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_summoners_aliases(body: Vec<LolSummonerAlias>) -> PostLolSummonerV1SummonersAliases {
    PostLolSummonerV1SummonersAliases{body}
}


pub struct PostLolSummonerV1ValidateAlias {
    pub body: LolSummonerAlias,
}

impl IsApiRequest for PostLolSummonerV1ValidateAlias {
    const METHOD: Method = Method::POST;
    type ReturnType = LolSummonerAliasAvailability;
    fn get_url(&self) -> String {"/lol-summoner/v1/validate-alias".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v1_validate_alias(body: LolSummonerAlias) -> PostLolSummonerV1ValidateAlias {
    PostLolSummonerV1ValidateAlias{body}
}


pub struct PostLolSummonerV2SummonersNames {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolSummonerV2SummonersNames {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolSummonerSummoner>;
    fn get_url(&self) -> String {"/lol-summoner/v2/summoners/names".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v2_summoners_names(body: Vec<String>) -> PostLolSummonerV2SummonersNames {
    PostLolSummonerV2SummonersNames{body}
}


pub struct PostLolSummonerV2SummonersPuuid {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolSummonerV2SummonersPuuid {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolSummonerSummoner>;
    fn get_url(&self) -> String {"/lol-summoner/v2/summoners/puuid".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_summoner_v2_summoners_puuid(body: Vec<String>) -> PostLolSummonerV2SummonersPuuid {
    PostLolSummonerV2SummonersPuuid{body}
}


pub struct PutLolSummonerV1CurrentSummonerIcon {
    pub body: LolSummonerSummonerIcon,
}

impl IsApiRequest for PutLolSummonerV1CurrentSummonerIcon {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolSummonerSummoner;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/icon".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_summoner_v1_current_summoner_icon(body: LolSummonerSummonerIcon) -> PutLolSummonerV1CurrentSummonerIcon {
    PutLolSummonerV1CurrentSummonerIcon{body}
}


pub struct PutLolSummonerV1CurrentSummonerProfilePrivacy {
    pub body: LolSummonerProfilePrivacySetting,
}

impl IsApiRequest for PutLolSummonerV1CurrentSummonerProfilePrivacy {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-summoner/v1/current-summoner/profile-privacy".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_summoner_v1_current_summoner_profile_privacy(body: LolSummonerProfilePrivacySetting) -> PutLolSummonerV1CurrentSummonerProfilePrivacy {
    PutLolSummonerV1CurrentSummonerProfilePrivacy{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAccountIdAndSummonerId {
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAlias {
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAliasAvailability {
    pub alias: LolSummonerAlias,
    #[serde(rename = "errorCode")]
    pub error_code: LolSummonerAliasAvailabilityCode,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "isSuccess")]
    pub is_success: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAliasLookupResponse {
    pub alias: LolSummonerAlias,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAutoFillQueueDto {
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "autoFillEligible")]
    pub auto_fill_eligible: bool,
    #[serde(rename = "autoFillProtectedForStreaking")]
    pub auto_fill_protected_for_streaking: bool,
    #[serde(rename = "autoFillProtectedForPromos")]
    pub auto_fill_protected_for_promos: bool,
    #[serde(rename = "autoFillProtectedForRemedy")]
    pub auto_fill_protected_for_remedy: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerPlayerNameState {
    #[serde(rename = "isAliasChangeRequired")]
    pub is_alias_change_required: bool,
    #[serde(rename = "isAliasMissing")]
    pub is_alias_missing: bool,
    #[serde(rename = "isTaglineCustomizable")]
    pub is_tagline_customizable: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerProfilePrivacy {
    #[serde(rename = "enabledState")]
    pub enabled_state: LolSummonerProfilePrivacyEnabledState,
    pub setting: LolSummonerProfilePrivacySetting,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerStatus {
    pub ready: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummoner {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "internalName")]
    pub internal_name: String,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: u32,
    #[serde(rename = "xpSinceLastLevel")]
    pub xp_since_last_level: u64,
    #[serde(rename = "xpUntilNextLevel")]
    pub xp_until_next_level: u64,
    #[serde(rename = "percentCompleteForNextLevel")]
    pub percent_complete_for_next_level: u32,
    #[serde(rename = "rerollPoints")]
    pub reroll_points: LolSummonerSummonerRerollPoints,
    pub puuid: String,
    #[serde(rename = "nameChangeFlag")]
    pub name_change_flag: bool,
    pub unnamed: bool,
    pub privacy: LolSummonerProfilePrivacySetting,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIcon {
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename = "inventoryToken")]
    pub inventory_token: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIdAndIcon {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIdAndName {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerProfileUpdate {
    pub key: String,
    pub value: Value,
    pub inventory: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerRequestedName {
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerRerollPoints {
    #[serde(rename = "pointsToReroll")]
    pub points_to_reroll: u32,
    #[serde(rename = "currentPoints")]
    pub current_points: u32,
    #[serde(rename = "numberOfRolls")]
    pub number_of_rolls: u32,
    #[serde(rename = "maxRolls")]
    pub max_rolls: u32,
    #[serde(rename = "pointsCostToRoll")]
    pub points_cost_to_roll: u32,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolSummonerAliasAvailabilityCode {
    #[default]
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limited")]
    RateLimited,
    #[serde(rename = "name_not_available")]
    NameNotAvailable,
    #[serde(rename = "name_change_forbidden")]
    NameChangeForbidden,
    #[serde(rename = "no_error")]
    NoError,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolSummonerPlayerNameMode {
    #[default]
    #[serde(rename = "ALIAS")]
    Alias,
    #[serde(rename = "DARKMODE")]
    Darkmode,
    #[serde(rename = "SUMMONER")]
    Summoner,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolSummonerProfilePrivacyEnabledState {
    #[default]
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolSummonerProfilePrivacySetting {
    #[default]
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PRIVATE")]
    Private,
}

