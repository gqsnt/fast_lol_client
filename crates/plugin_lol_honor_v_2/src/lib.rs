
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolHonorV2V1Ballot {}

impl IsApiRequest for GetLolHonorV2V1Ballot {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHonorV2Ballot;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/ballot".to_string()}
}

pub fn get_lol_honor_v2_v1_ballot() -> GetLolHonorV2V1Ballot {
    GetLolHonorV2V1Ballot{}
}


pub struct GetLolHonorV2V1Config {}

impl IsApiRequest for GetLolHonorV2V1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHonorV2HonorConfig;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/config".to_string()}
}

pub fn get_lol_honor_v2_v1_config() -> GetLolHonorV2V1Config {
    GetLolHonorV2V1Config{}
}


pub struct GetLolHonorV2V1LateRecognition {}

impl IsApiRequest for GetLolHonorV2V1LateRecognition {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolHonorV2Honor>;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/late-recognition".to_string()}
}

pub fn get_lol_honor_v2_v1_late_recognition() -> GetLolHonorV2V1LateRecognition {
    GetLolHonorV2V1LateRecognition{}
}


pub struct GetLolHonorV2V1LatestEligibleGame {}

impl IsApiRequest for GetLolHonorV2V1LatestEligibleGame {
    const METHOD: Method = Method::GET;
    type ReturnType = u64;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/latest-eligible-game".to_string()}
}

pub fn get_lol_honor_v2_v1_latest_eligible_game() -> GetLolHonorV2V1LatestEligibleGame {
    GetLolHonorV2V1LatestEligibleGame{}
}


pub struct GetLolHonorV2V1LevelChange {}

impl IsApiRequest for GetLolHonorV2V1LevelChange {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHonorV2VendedHonorChange;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/level-change".to_string()}
}

pub fn get_lol_honor_v2_v1_level_change() -> GetLolHonorV2V1LevelChange {
    GetLolHonorV2V1LevelChange{}
}


pub struct GetLolHonorV2V1MutualHonor {}

impl IsApiRequest for GetLolHonorV2V1MutualHonor {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHonorV2MutualHonor;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/mutual-honor".to_string()}
}

pub fn get_lol_honor_v2_v1_mutual_honor() -> GetLolHonorV2V1MutualHonor {
    GetLolHonorV2V1MutualHonor{}
}


pub struct GetLolHonorV2V1Profile {}

impl IsApiRequest for GetLolHonorV2V1Profile {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHonorV2ProfileInfo;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/profile".to_string()}
}

pub fn get_lol_honor_v2_v1_profile() -> GetLolHonorV2V1Profile {
    GetLolHonorV2V1Profile{}
}


pub struct GetLolHonorV2V1Recognition {}

impl IsApiRequest for GetLolHonorV2V1Recognition {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolHonorV2Honor>;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/recognition".to_string()}
}

pub fn get_lol_honor_v2_v1_recognition() -> GetLolHonorV2V1Recognition {
    GetLolHonorV2V1Recognition{}
}


pub struct GetLolHonorV2V1RecognitionHistory {}

impl IsApiRequest for GetLolHonorV2V1RecognitionHistory {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolHonorV2HonorInteraction>;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/recognition-history".to_string()}
}

pub fn get_lol_honor_v2_v1_recognition_history() -> GetLolHonorV2V1RecognitionHistory {
    GetLolHonorV2V1RecognitionHistory{}
}


pub struct GetLolHonorV2V1RewardGranted {}

impl IsApiRequest for GetLolHonorV2V1RewardGranted {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHonorV2VendedReward;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/reward-granted".to_string()}
}

pub fn get_lol_honor_v2_v1_reward_granted() -> GetLolHonorV2V1RewardGranted {
    GetLolHonorV2V1RewardGranted{}
}


pub struct GetLolHonorV2V1TeamChoices {}

impl IsApiRequest for GetLolHonorV2V1TeamChoices {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/team-choices".to_string()}
}

pub fn get_lol_honor_v2_v1_team_choices() -> GetLolHonorV2V1TeamChoices {
    GetLolHonorV2V1TeamChoices{}
}


pub struct GetLolHonorV2V1VoteCompletion {}

impl IsApiRequest for GetLolHonorV2V1VoteCompletion {
    const METHOD: Method = Method::GET;
    type ReturnType = LolHonorV2VoteCompletion;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/vote-completion".to_string()}
}

pub fn get_lol_honor_v2_v1_vote_completion() -> GetLolHonorV2V1VoteCompletion {
    GetLolHonorV2V1VoteCompletion{}
}


pub struct PostLolHonorV1Ballot {}

impl IsApiRequest for PostLolHonorV1Ballot {
    const METHOD: Method = Method::POST;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-honor/v1/ballot".to_string()}
}

pub fn post_lol_honor_v1_ballot() -> PostLolHonorV1Ballot {
    PostLolHonorV1Ballot{}
}


pub struct PostLolHonorV1Honor {
    pub body: LolHonorV2ApiHonorPlayerServerRequestV3,
}

impl IsApiRequest for PostLolHonorV1Honor {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-honor/v1/honor".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_honor_v1_honor(body: LolHonorV2ApiHonorPlayerServerRequestV3) -> PostLolHonorV1Honor {
    PostLolHonorV1Honor{body}
}


pub struct PostLolHonorV2V1HonorPlayer {
    pub body: LolHonorV2ApiHonorPlayerServerRequest,
}

impl IsApiRequest for PostLolHonorV2V1HonorPlayer {
    const METHOD: Method = Method::POST;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/honor-player".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_honor_v2_v1_honor_player(body: LolHonorV2ApiHonorPlayerServerRequest) -> PostLolHonorV2V1HonorPlayer {
    PostLolHonorV2V1HonorPlayer{body}
}


pub struct PostLolHonorV2V1LateRecognitionAck {}

impl IsApiRequest for PostLolHonorV2V1LateRecognitionAck {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/late-recognition/ack".to_string()}
}

pub fn post_lol_honor_v2_v1_late_recognition_ack() -> PostLolHonorV2V1LateRecognitionAck {
    PostLolHonorV2V1LateRecognitionAck{}
}


pub struct PostLolHonorV2V1LevelChangeAck {}

impl IsApiRequest for PostLolHonorV2V1LevelChangeAck {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/level-change/ack".to_string()}
}

pub fn post_lol_honor_v2_v1_level_change_ack() -> PostLolHonorV2V1LevelChangeAck {
    PostLolHonorV2V1LevelChangeAck{}
}


pub struct PostLolHonorV2V1MutualHonorAck {}

impl IsApiRequest for PostLolHonorV2V1MutualHonorAck {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/mutual-honor/ack".to_string()}
}

pub fn post_lol_honor_v2_v1_mutual_honor_ack() -> PostLolHonorV2V1MutualHonorAck {
    PostLolHonorV2V1MutualHonorAck{}
}


pub struct PostLolHonorV2V1RewardGrantedAck {}

impl IsApiRequest for PostLolHonorV2V1RewardGrantedAck {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-honor-v2/v1/reward-granted/ack".to_string()}
}

pub fn post_lol_honor_v2_v1_reward_granted_ack() -> PostLolHonorV2V1RewardGrantedAck {
    PostLolHonorV2V1RewardGrantedAck{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ApiHonorPlayerServerRequest {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub puuid: String,
    #[serde(rename = "honorType")]
    pub honor_type: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ApiHonorPlayerServerRequestV3 {
    pub puuid: String,
    #[serde(rename = "honorType")]
    pub honor_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Ballot {
    #[serde(rename = "eligibleAllies")]
    pub eligible_allies: Vec<LolHonorV2EligiblePlayer>,
    #[serde(rename = "eligibleOpponents")]
    pub eligible_opponents: Vec<LolHonorV2EligiblePlayer>,
    #[serde(rename = "numVotes")]
    pub num_votes: u32,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "honoredPlayers")]
    pub honored_players: Vec<LolHonorV2ApiHonorPlayerServerRequestV3>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2DynamicHonorMessage {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub value: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EligiblePlayer {
    pub puuid: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "championName")]
    pub champion_name: String,
    #[serde(rename = "skinSplashPath")]
    pub skin_splash_path: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Honor {
    #[serde(rename = "honorCategory")]
    pub honor_category: String,
    #[serde(rename = "voterRelationship")]
    pub voter_relationship: String,
    #[serde(rename = "senderPuuid")]
    pub sender_puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorConfig {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "SecondsToVote")]
    pub seconds_to_vote: i32,
    #[serde(rename = "HonorVisibilityEnabled")]
    pub honor_visibility_enabled: bool,
    #[serde(rename = "HonorSuggestionsEnabled")]
    pub honor_suggestions_enabled: bool,
    #[serde(rename = "honorEndpointsV2Enabled")]
    pub honor_endpoints_v2_enabled: bool,
    #[serde(rename = "ceremonyV3Enabled")]
    pub ceremony_v3_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorInteraction {
    pub puuid: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2MutualHonor {
    #[serde(rename = "gameId")]
    pub game_id: u64,
    pub summoners: Vec<LolHonorV2MutualHonorPlayer>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2MutualHonorPlayer {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ProfileInfo {
    #[serde(rename = "honorLevel")]
    pub honor_level: i32,
    pub checkpoint: i32,
    #[serde(rename = "rewardsLocked")]
    pub rewards_locked: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Reward {
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    pub quantity: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedHonorChange {
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(rename = "previousState")]
    pub previous_state: LolHonorV2VendedHonorState,
    #[serde(rename = "currentState")]
    pub current_state: LolHonorV2VendedHonorState,
    pub reward: LolHonorV2Reward,
    #[serde(rename = "dynamicHonorMessage")]
    pub dynamic_honor_message: LolHonorV2DynamicHonorMessage,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedHonorState {
    pub level: i32,
    pub checkpoint: i32,
    #[serde(rename = "rewardsLocked")]
    pub rewards_locked: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedReward {
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    pub quantity: i32,
    #[serde(rename = "dynamicHonorMessage")]
    pub dynamic_honor_message: LolHonorV2DynamicHonorMessage,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VoteCompletion {
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "fullTeamVote")]
    pub full_team_vote: bool,
}


// ENUMS

