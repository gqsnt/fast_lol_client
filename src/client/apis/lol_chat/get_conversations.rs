use std::fmt;
use serde::{Serialize, Deserialize};


pub type LolChatConversationResource = Vec<LolChatConversation>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversation {
    pub id: String,
    pub name: String,
    pub pid: String,
    pub game_name: String,
    pub game_tag: String,
    pub type_: String, // type is a reserved keyword in Rust, so using type_
    pub inviter_id: String,
    pub password: String,
    pub target_region: String,
    pub is_muted: bool,
    pub unread_message_count: u64,
    pub last_message: Option<LolChatConversationMessageResource>,
    pub muc_jwt_dto: Option<LolChatMucJwtDto>,
}

impl PartialEq for LolChatConversation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for LolChatConversation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, if self.unread_message_count > 0 {format!("({})", self.unread_message_count)} else {"".to_string()})
    }
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationMessageResource {
    pub id: String,
    pub type_: String, // type is a reserved keyword in Rust, so using type_
    pub from_summoner_id: u64,
    pub from_id: String,
    pub from_pid: String,
    pub from_obfuscated_summoner_id: u64,
    pub body: String,
    pub timestamp: String,
    pub is_historical: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
    pub domain: String,
    pub target_region: String,
}