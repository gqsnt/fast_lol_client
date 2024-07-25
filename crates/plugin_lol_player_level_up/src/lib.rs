
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolPlayerLevelUpV1LevelUp {}

impl IsApiRequest for GetLolPlayerLevelUpV1LevelUp {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerLevelUpPlayerLevelUpEvent;
    fn get_url(&self) -> String {"/lol-player-level-up/v1/level-up".to_string()}
}

pub fn get_lol_player_level_up_v1_level_up() -> GetLolPlayerLevelUpV1LevelUp {
    GetLolPlayerLevelUpV1LevelUp{}
}


pub struct GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    pub plugin_name: String,
}

impl IsApiRequest for GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerLevelUpPlayerLevelUpEventAck;
    fn get_url(&self) -> String {format!("/lol-player-level-up/v1/level-up-notifications/{}", self.plugin_name)}
}

pub fn get_lol_player_level_up_v1_level_up_notifications_by_plugin_name(plugin_name: String) -> GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName{plugin_name}
}


pub struct PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    pub plugin_name: String,
    pub body: LolPlayerLevelUpPlayerLevelUpEventAck,
}

impl IsApiRequest for PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-player-level-up/v1/level-up-notifications/{}", self.plugin_name)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_player_level_up_v1_level_up_notifications_by_plugin_name(plugin_name: String, body: LolPlayerLevelUpPlayerLevelUpEventAck) -> PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName{plugin_name, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEvent {
    #[serde(rename = "switchedToStandardFreeToPlayChampRotation")]
    pub switched_to_standard_free_to_play_champ_rotation: bool,
    #[serde(rename = "nowHasAccessToPublicChatRooms")]
    pub now_has_access_to_public_chat_rooms: bool,
    #[serde(rename = "nowHasAccessToLoot")]
    pub now_has_access_to_loot: bool,
    #[serde(rename = "leveledUp")]
    pub leveled_up: bool,
    #[serde(rename = "newSummonerLevel")]
    pub new_summoner_level: u32,
    #[serde(rename = "newRuneSlotUnlocked")]
    pub new_rune_slot_unlocked: bool,
    #[serde(rename = "rpEarned")]
    pub rp_earned: i32,
    #[serde(rename = "newSpells")]
    pub new_spells: Vec<u64>,
    #[serde(rename = "newQueues")]
    pub new_queues: Vec<i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEventAck {
    #[serde(rename = "seenThisEvent")]
    pub seen_this_event: bool,
    #[serde(rename = "newSummonerLevel")]
    pub new_summoner_level: u32,
}


// ENUMS

