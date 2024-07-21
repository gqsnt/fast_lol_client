
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetLolPlayerLevelUpV1LevelUp {

}

impl IsApiRequest for GetLolPlayerLevelUpV1LevelUp {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerLevelUpPlayerLevelUpEvent;

    fn get_url(&self) -> String {
        "/lol-player-level-up/v1/level-up".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_player_level_up_v_1_level_up() -> GetLolPlayerLevelUpV1LevelUp {
    GetLolPlayerLevelUpV1LevelUp {
        
    }
}


pub struct GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName {

    pub plugin_name: String,
}

impl IsApiRequest for GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    const METHOD: Method = Method::GET;
    type ReturnType = LolPlayerLevelUpPlayerLevelUpEventAck;

    fn get_url(&self) -> String {
        format!("/lol-player-level-up/v1/level-up-notifications/{}", self.plugin_name)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_player_level_up_v_1_level_up_notifications_by_plugin_name(plugin_name: String) -> GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    GetLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
        plugin_name
    }
}


pub struct PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName {

    pub plugin_name: String,
    pub body: LolPlayerLevelUpPlayerLevelUpEventAck,
}

impl IsApiRequest for PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-player-level-up/v1/level-up-notifications/{}", self.plugin_name)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_player_level_up_v_1_level_up_notifications_by_plugin_name(plugin_name: String, body: LolPlayerLevelUpPlayerLevelUpEventAck) -> PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
    PostLolPlayerLevelUpV1LevelUpNotificationsByPluginName {
        plugin_name, body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEvent {
    pub switched_to_standard_free_to_play_champ_rotation: bool,
    pub now_has_access_to_public_chat_rooms: bool,
    pub now_has_access_to_loot: bool,
    pub leveled_up: bool,
    pub new_summoner_level: u32,
    pub new_rune_slot_unlocked: bool,
    pub rp_earned: i32,
    pub new_spells: Vec<u64>,
    pub new_queues: Vec<i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEventAck {
    pub seen_this_event: bool,
    pub new_summoner_level: u32,
}


// ENUMS

