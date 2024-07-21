
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolPremadeVoiceV1MicTest {

}

impl IsApiRequest for DeleteLolPremadeVoiceV1MicTest {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/mic-test".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_premade_voice_v_1_mic_test() -> DeleteLolPremadeVoiceV1MicTest {
    DeleteLolPremadeVoiceV1MicTest {
        
    }
}


pub struct DeleteLolPremadeVoiceV1Session {

}

impl IsApiRequest for DeleteLolPremadeVoiceV1Session {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/session".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_premade_voice_v_1_session() -> DeleteLolPremadeVoiceV1Session {
    DeleteLolPremadeVoiceV1Session {
        
    }
}


pub struct GetLolPremadeVoiceV1Availability {

}

impl IsApiRequest for GetLolPremadeVoiceV1Availability {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPremadeVoiceVoiceAvailability;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/availability".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_premade_voice_v_1_availability() -> GetLolPremadeVoiceV1Availability {
    GetLolPremadeVoiceV1Availability {
        
    }
}


pub struct GetLolPremadeVoiceV1Capturedevices {

}

impl IsApiRequest for GetLolPremadeVoiceV1Capturedevices {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPremadeVoiceDeviceResource>;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/capturedevices".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_premade_voice_v_1_capturedevices() -> GetLolPremadeVoiceV1Capturedevices {
    GetLolPremadeVoiceV1Capturedevices {
        
    }
}


pub struct GetLolPremadeVoiceV1FirstExperience {

}

impl IsApiRequest for GetLolPremadeVoiceV1FirstExperience {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPremadeVoiceFirstExperience;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/first-experience".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_premade_voice_v_1_first_experience() -> GetLolPremadeVoiceV1FirstExperience {
    GetLolPremadeVoiceV1FirstExperience {
        
    }
}


pub struct GetLolPremadeVoiceV1MicTest {

}

impl IsApiRequest for GetLolPremadeVoiceV1MicTest {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPremadeVoiceAudioPropertiesResource;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/mic-test".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_premade_voice_v_1_mic_test() -> GetLolPremadeVoiceV1MicTest {
    GetLolPremadeVoiceV1MicTest {
        
    }
}


pub struct GetLolPremadeVoiceV1ParticipantRecords {

}

impl IsApiRequest for GetLolPremadeVoiceV1ParticipantRecords {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPremadeVoicePremadeVoiceParticipantDto>;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/participant-records".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_premade_voice_v_1_participant_records() -> GetLolPremadeVoiceV1ParticipantRecords {
    GetLolPremadeVoiceV1ParticipantRecords {
        
    }
}


pub struct GetLolPremadeVoiceV1Participants {

}

impl IsApiRequest for GetLolPremadeVoiceV1Participants {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolPremadeVoicePremadeVoiceParticipantDto>;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/participants".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_premade_voice_v_1_participants() -> GetLolPremadeVoiceV1Participants {
    GetLolPremadeVoiceV1Participants {
        
    }
}


pub struct GetLolPremadeVoiceV1Settings {

}

impl IsApiRequest for GetLolPremadeVoiceV1Settings {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPremadeVoiceSettingsResource;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_premade_voice_v_1_settings() -> GetLolPremadeVoiceV1Settings {
    GetLolPremadeVoiceV1Settings {
        
    }
}


pub struct PostLolPremadeVoiceV1FirstExperienceGame {

}

impl IsApiRequest for PostLolPremadeVoiceV1FirstExperienceGame {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/first-experience/game".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_first_experience_game() -> PostLolPremadeVoiceV1FirstExperienceGame {
    PostLolPremadeVoiceV1FirstExperienceGame {
        
    }
}


pub struct PostLolPremadeVoiceV1FirstExperienceLcu {

}

impl IsApiRequest for PostLolPremadeVoiceV1FirstExperienceLcu {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/first-experience/lcu".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_first_experience_lcu() -> PostLolPremadeVoiceV1FirstExperienceLcu {
    PostLolPremadeVoiceV1FirstExperienceLcu {
        
    }
}


pub struct PostLolPremadeVoiceV1FirstExperienceReset {

}

impl IsApiRequest for PostLolPremadeVoiceV1FirstExperienceReset {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/first-experience/reset".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_first_experience_reset() -> PostLolPremadeVoiceV1FirstExperienceReset {
    PostLolPremadeVoiceV1FirstExperienceReset {
        
    }
}


pub struct PostLolPremadeVoiceV1GameClientUpdatedPTTKey {

    pub body: String,
}

impl IsApiRequest for PostLolPremadeVoiceV1GameClientUpdatedPTTKey {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/gameClientUpdatedPTTKey".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_game_client_updated_ptt_key(body: String) -> PostLolPremadeVoiceV1GameClientUpdatedPTTKey {
    PostLolPremadeVoiceV1GameClientUpdatedPTTKey {
        body
    }
}


pub struct PostLolPremadeVoiceV1MicTest {

}

impl IsApiRequest for PostLolPremadeVoiceV1MicTest {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/mic-test".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_mic_test() -> PostLolPremadeVoiceV1MicTest {
    PostLolPremadeVoiceV1MicTest {
        
    }
}


pub struct PostLolPremadeVoiceV1PushToTalkCheckAvailable {

    pub body: i32,
}

impl IsApiRequest for PostLolPremadeVoiceV1PushToTalkCheckAvailable {
    const METHOD: Method = Method::POST;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/push-to-talk/check-available".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_push_to_talk_check_available(body: i32) -> PostLolPremadeVoiceV1PushToTalkCheckAvailable {
    PostLolPremadeVoiceV1PushToTalkCheckAvailable {
        body
    }
}


pub struct PostLolPremadeVoiceV1Session {

}

impl IsApiRequest for PostLolPremadeVoiceV1Session {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/session".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_session() -> PostLolPremadeVoiceV1Session {
    PostLolPremadeVoiceV1Session {
        
    }
}


pub struct PostLolPremadeVoiceV1SettingsReset {

}

impl IsApiRequest for PostLolPremadeVoiceV1SettingsReset {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/settings/reset".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_premade_voice_v_1_settings_reset() -> PostLolPremadeVoiceV1SettingsReset {
    PostLolPremadeVoiceV1SettingsReset {
        
    }
}


pub struct PutLolPremadeVoiceV1Capturedevices {

    pub body: String,
}

impl IsApiRequest for PutLolPremadeVoiceV1Capturedevices {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/capturedevices".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_premade_voice_v_1_capturedevices(body: String) -> PutLolPremadeVoiceV1Capturedevices {
    PutLolPremadeVoiceV1Capturedevices {
        body
    }
}


pub struct PutLolPremadeVoiceV1ParticipantsByPuuidMute {

    pub puuid: String,
    pub body: i32,
}

impl IsApiRequest for PutLolPremadeVoiceV1ParticipantsByPuuidMute {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-premade-voice/v1/participants/{}/mute", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_premade_voice_v_1_participants_by_puuid_mute(puuid: String, body: i32) -> PutLolPremadeVoiceV1ParticipantsByPuuidMute {
    PutLolPremadeVoiceV1ParticipantsByPuuidMute {
        puuid, body
    }
}


pub struct PutLolPremadeVoiceV1ParticipantsByPuuidVolume {

    pub puuid: String,
    pub body: i32,
}

impl IsApiRequest for PutLolPremadeVoiceV1ParticipantsByPuuidVolume {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-premade-voice/v1/participants/{}/volume", self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_premade_voice_v_1_participants_by_puuid_volume(puuid: String, body: i32) -> PutLolPremadeVoiceV1ParticipantsByPuuidVolume {
    PutLolPremadeVoiceV1ParticipantsByPuuidVolume {
        puuid, body
    }
}


pub struct PutLolPremadeVoiceV1SelfActivationSensitivity {

    pub body: i32,
}

impl IsApiRequest for PutLolPremadeVoiceV1SelfActivationSensitivity {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/self/activationSensitivity".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_premade_voice_v_1_self_activation_sensitivity(body: i32) -> PutLolPremadeVoiceV1SelfActivationSensitivity {
    PutLolPremadeVoiceV1SelfActivationSensitivity {
        body
    }
}


pub struct PutLolPremadeVoiceV1SelfInputMode {

    pub body: LolPremadeVoiceInputMode,
}

impl IsApiRequest for PutLolPremadeVoiceV1SelfInputMode {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/self/inputMode".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_premade_voice_v_1_self_input_mode(body: LolPremadeVoiceInputMode) -> PutLolPremadeVoiceV1SelfInputMode {
    PutLolPremadeVoiceV1SelfInputMode {
        body
    }
}


pub struct PutLolPremadeVoiceV1SelfMicLevel {

    pub body: i32,
}

impl IsApiRequest for PutLolPremadeVoiceV1SelfMicLevel {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/self/micLevel".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_premade_voice_v_1_self_mic_level(body: i32) -> PutLolPremadeVoiceV1SelfMicLevel {
    PutLolPremadeVoiceV1SelfMicLevel {
        body
    }
}


pub struct PutLolPremadeVoiceV1SelfMute {

    pub body: i32,
}

impl IsApiRequest for PutLolPremadeVoiceV1SelfMute {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-premade-voice/v1/self/mute".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_premade_voice_v_1_self_mute(body: i32) -> PutLolPremadeVoiceV1SelfMute {
    PutLolPremadeVoiceV1SelfMute {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceAudioPropertiesResource {
    pub is_loopback_enabled: bool,
    pub mic_energy: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceDeviceResource {
    pub handle: String,
    pub name: String,
    pub usable: bool,
    pub is_current_device: bool,
    pub is_default: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceFirstExperience {
    pub show_first_experience_in_lcu: bool,
    pub show_first_experience_in_game: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePremadeVoiceParticipantDto {
    pub participant_id: String,
    pub summoner_id: u64,
    pub puuid: String,
    pub display_name: String,
    pub volume: u32,
    pub energy: u32,
    pub is_muted: bool,
    pub is_speaking: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceSettingsResource {
    pub current_capture_device_handle: String,
    pub vad_hangover_time: u32,
    pub vad_sensitivity: u32,
    pub mic_level: u32,
    pub local_mic_muted: bool,
    pub loopback_enabled: bool,
    pub auto_join: bool,
    pub mute_on_connect: bool,
    pub vad_active: bool,
    pub ptt_active: bool,
    pub input_mode: LolPremadeVoiceInputMode,
    pub ptt_key: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceVoiceAvailability {
    pub enabled: bool,
    pub connected_to_voice_server: bool,
    pub voice_channel_available: bool,
    pub disabled_after_login: bool,
    pub show_ui: bool,
    pub show_disconnected_state: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolPremadeVoiceInputMode {
    #[default]
    #[serde(rename = "pushToTalk")]
    PushToTalk,
    #[serde(rename = "voiceActivity")]
    VoiceActivity,
}

