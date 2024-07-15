use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::apis::lol_chat::get_me::LolDetails;

pub type LolChatFriendsResource = Vec<LolChatFriendResource>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendResource {
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    pub status_message: String,
    pub note: String,
    pub last_seen_online_timestamp: Option<String>,
    #[serde(rename = "isP2PConversationMuted")]
    pub is_p2p_conversation_muted: bool,
    pub group_id: u32,
    pub display_group_id: u32,
    pub group_name: String,
    pub display_group_name: String,
    pub lol: Option<LolDetails>,
}

impl PartialEq for LolChatFriendResource {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for LolChatFriendResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let in_game_status = if let Some(lol) = &self.lol {
            format!("({})",lol.game_status.clone().unwrap_or_default())
        } else {
            "".to_string()
        };
        write!(f, "{} {}", &self.game_name, in_game_status)
    }
}

