
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolChatV1BlockedPlayersById {
    pub id: String,
}

impl IsApiRequest for DeleteLolChatV1BlockedPlayersById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/blocked-players/{}", self.id)}
}

pub fn delete_lol_chat_v1_blocked_players_by_id(id: String) -> DeleteLolChatV1BlockedPlayersById {
    DeleteLolChatV1BlockedPlayersById{id}
}


pub struct DeleteLolChatV1ConversationsActive {}

impl IsApiRequest for DeleteLolChatV1ConversationsActive {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/conversations/active".to_string()}
}

pub fn delete_lol_chat_v1_conversations_active() -> DeleteLolChatV1ConversationsActive {
    DeleteLolChatV1ConversationsActive{}
}


pub struct DeleteLolChatV1ConversationsById {
    pub id: String,
}

impl IsApiRequest for DeleteLolChatV1ConversationsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}", self.id)}
}

pub fn delete_lol_chat_v1_conversations_by_id(id: String) -> DeleteLolChatV1ConversationsById {
    DeleteLolChatV1ConversationsById{id}
}


pub struct DeleteLolChatV1ConversationsByIdMessages {
    pub id: String,
}

impl IsApiRequest for DeleteLolChatV1ConversationsByIdMessages {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}/messages", self.id)}
}

pub fn delete_lol_chat_v1_conversations_by_id_messages(id: String) -> DeleteLolChatV1ConversationsByIdMessages {
    DeleteLolChatV1ConversationsByIdMessages{id}
}


pub struct DeleteLolChatV1ErrorsById {
    pub id: u64,
}

impl IsApiRequest for DeleteLolChatV1ErrorsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/errors/{}", self.id)}
}

pub fn delete_lol_chat_v1_errors_by_id(id: u64) -> DeleteLolChatV1ErrorsById {
    DeleteLolChatV1ErrorsById{id}
}


pub struct DeleteLolChatV1FriendGroupsById {
    pub id: u32,
}

impl IsApiRequest for DeleteLolChatV1FriendGroupsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/friend-groups/{}", self.id)}
}

pub fn delete_lol_chat_v1_friend_groups_by_id(id: u32) -> DeleteLolChatV1FriendGroupsById {
    DeleteLolChatV1FriendGroupsById{id}
}


pub struct DeleteLolChatV1FriendsById {
    pub id: String,
}

impl IsApiRequest for DeleteLolChatV1FriendsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/friends/{}", self.id)}
}

pub fn delete_lol_chat_v1_friends_by_id(id: String) -> DeleteLolChatV1FriendsById {
    DeleteLolChatV1FriendsById{id}
}


pub struct DeleteLolChatV1PlayerMutes {}

impl IsApiRequest for DeleteLolChatV1PlayerMutes {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/player-mutes".to_string()}
}

pub fn delete_lol_chat_v1_player_mutes() -> DeleteLolChatV1PlayerMutes {
    DeleteLolChatV1PlayerMutes{}
}


pub struct DeleteLolChatV2FriendRequestsById {
    pub id: String,
}

impl IsApiRequest for DeleteLolChatV2FriendRequestsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v2/friend-requests/{}", self.id)}
}

pub fn delete_lol_chat_v2_friend_requests_by_id(id: String) -> DeleteLolChatV2FriendRequestsById {
    DeleteLolChatV2FriendRequestsById{id}
}


pub struct GetLolChatV1BlockedPlayers {}

impl IsApiRequest for GetLolChatV1BlockedPlayers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatBlockedPlayerResource>;
    fn get_url(&self) -> String {"/lol-chat/v1/blocked-players".to_string()}
}

pub fn get_lol_chat_v1_blocked_players() -> GetLolChatV1BlockedPlayers {
    GetLolChatV1BlockedPlayers{}
}


pub struct GetLolChatV1BlockedPlayersById {
    pub id: String,
}

impl IsApiRequest for GetLolChatV1BlockedPlayersById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatBlockedPlayerResource;
    fn get_url(&self) -> String {format!("/lol-chat/v1/blocked-players/{}", self.id)}
}

pub fn get_lol_chat_v1_blocked_players_by_id(id: String) -> GetLolChatV1BlockedPlayersById {
    GetLolChatV1BlockedPlayersById{id}
}


pub struct GetLolChatV1Config {}

impl IsApiRequest for GetLolChatV1Config {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatChatServiceDynamicClientConfig;
    fn get_url(&self) -> String {"/lol-chat/v1/config".to_string()}
}

pub fn get_lol_chat_v1_config() -> GetLolChatV1Config {
    GetLolChatV1Config{}
}


pub struct GetLolChatV1Conversations {}

impl IsApiRequest for GetLolChatV1Conversations {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatConversationResource>;
    fn get_url(&self) -> String {"/lol-chat/v1/conversations".to_string()}
}

pub fn get_lol_chat_v1_conversations() -> GetLolChatV1Conversations {
    GetLolChatV1Conversations{}
}


pub struct GetLolChatV1ConversationsActive {}

impl IsApiRequest for GetLolChatV1ConversationsActive {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatActiveConversationResource;
    fn get_url(&self) -> String {"/lol-chat/v1/conversations/active".to_string()}
}

pub fn get_lol_chat_v1_conversations_active() -> GetLolChatV1ConversationsActive {
    GetLolChatV1ConversationsActive{}
}


pub struct GetLolChatV1ConversationsById {
    pub id: String,
}

impl IsApiRequest for GetLolChatV1ConversationsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatConversationResource;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}", self.id)}
}

pub fn get_lol_chat_v1_conversations_by_id(id: String) -> GetLolChatV1ConversationsById {
    GetLolChatV1ConversationsById{id}
}


pub struct GetLolChatV1ConversationsByIdMessages {
    pub id: String,
}

impl IsApiRequest for GetLolChatV1ConversationsByIdMessages {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatConversationMessageResource>;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}/messages", self.id)}
}

pub fn get_lol_chat_v1_conversations_by_id_messages(id: String) -> GetLolChatV1ConversationsByIdMessages {
    GetLolChatV1ConversationsByIdMessages{id}
}


pub struct GetLolChatV1ConversationsByIdParticipants {
    pub id: String,
}

impl IsApiRequest for GetLolChatV1ConversationsByIdParticipants {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatUserResource>;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}/participants", self.id)}
}

pub fn get_lol_chat_v1_conversations_by_id_participants(id: String) -> GetLolChatV1ConversationsByIdParticipants {
    GetLolChatV1ConversationsByIdParticipants{id}
}


pub struct GetLolChatV1ConversationsByIdParticipantsByPid {
    pub id: String,
    pub pid: String,
}

impl IsApiRequest for GetLolChatV1ConversationsByIdParticipantsByPid {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatUserResource;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}/participants/{}", self.id, self.pid)}
}

pub fn get_lol_chat_v1_conversations_by_id_participants_by_pid(id: String, pid: String) -> GetLolChatV1ConversationsByIdParticipantsByPid {
    GetLolChatV1ConversationsByIdParticipantsByPid{id, pid}
}


pub struct GetLolChatV1ConversationsNotify {}

impl IsApiRequest for GetLolChatV1ConversationsNotify {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-chat/v1/conversations/notify".to_string()}
}

pub fn get_lol_chat_v1_conversations_notify() -> GetLolChatV1ConversationsNotify {
    GetLolChatV1ConversationsNotify{}
}


pub struct GetLolChatV1Errors {}

impl IsApiRequest for GetLolChatV1Errors {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatErrorResource>;
    fn get_url(&self) -> String {"/lol-chat/v1/errors".to_string()}
}

pub fn get_lol_chat_v1_errors() -> GetLolChatV1Errors {
    GetLolChatV1Errors{}
}


pub struct GetLolChatV1FriendCounts {}

impl IsApiRequest for GetLolChatV1FriendCounts {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatFriendCountsResource;
    fn get_url(&self) -> String {"/lol-chat/v1/friend-counts".to_string()}
}

pub fn get_lol_chat_v1_friend_counts() -> GetLolChatV1FriendCounts {
    GetLolChatV1FriendCounts{}
}


pub struct GetLolChatV1FriendExistsBySummonerId {
    pub summoner_id: u64,
}

impl IsApiRequest for GetLolChatV1FriendExistsBySummonerId {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {format!("/lol-chat/v1/friend-exists/{}", self.summoner_id)}
}

pub fn get_lol_chat_v1_friend_exists_by_summoner_id(summoner_id: u64) -> GetLolChatV1FriendExistsBySummonerId {
    GetLolChatV1FriendExistsBySummonerId{summoner_id}
}


pub struct GetLolChatV1FriendGroups {}

impl IsApiRequest for GetLolChatV1FriendGroups {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatGroupResource>;
    fn get_url(&self) -> String {"/lol-chat/v1/friend-groups".to_string()}
}

pub fn get_lol_chat_v1_friend_groups() -> GetLolChatV1FriendGroups {
    GetLolChatV1FriendGroups{}
}


pub struct GetLolChatV1FriendGroupsById {
    pub id: u32,
}

impl IsApiRequest for GetLolChatV1FriendGroupsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatGroupResource;
    fn get_url(&self) -> String {format!("/lol-chat/v1/friend-groups/{}", self.id)}
}

pub fn get_lol_chat_v1_friend_groups_by_id(id: u32) -> GetLolChatV1FriendGroupsById {
    GetLolChatV1FriendGroupsById{id}
}


pub struct GetLolChatV1Friends {}

impl IsApiRequest for GetLolChatV1Friends {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatFriendResource>;
    fn get_url(&self) -> String {"/lol-chat/v1/friends".to_string()}
}

pub fn get_lol_chat_v1_friends() -> GetLolChatV1Friends {
    GetLolChatV1Friends{}
}


pub struct GetLolChatV1FriendsById {
    pub id: String,
}

impl IsApiRequest for GetLolChatV1FriendsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatFriendResource;
    fn get_url(&self) -> String {format!("/lol-chat/v1/friends/{}", self.id)}
}

pub fn get_lol_chat_v1_friends_by_id(id: String) -> GetLolChatV1FriendsById {
    GetLolChatV1FriendsById{id}
}


pub struct GetLolChatV1Me {}

impl IsApiRequest for GetLolChatV1Me {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatUserResource;
    fn get_url(&self) -> String {"/lol-chat/v1/me".to_string()}
}

pub fn get_lol_chat_v1_me() -> GetLolChatV1Me {
    GetLolChatV1Me{}
}


pub struct GetLolChatV1PlayerMutes {}

impl IsApiRequest for GetLolChatV1PlayerMutes {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, LolChatPlayerMuteStatus>;
    fn get_url(&self) -> String {"/lol-chat/v1/player-mutes".to_string()}
}

pub fn get_lol_chat_v1_player_mutes() -> GetLolChatV1PlayerMutes {
    GetLolChatV1PlayerMutes{}
}


pub struct GetLolChatV1Resources {}

impl IsApiRequest for GetLolChatV1Resources {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatProductMetadataMap;
    fn get_url(&self) -> String {"/lol-chat/v1/resources".to_string()}
}

pub fn get_lol_chat_v1_resources() -> GetLolChatV1Resources {
    GetLolChatV1Resources{}
}


pub struct GetLolChatV1Session {}

impl IsApiRequest for GetLolChatV1Session {
    const METHOD: Method = Method::GET;
    type ReturnType = LolChatSessionResource;
    fn get_url(&self) -> String {"/lol-chat/v1/session".to_string()}
}

pub fn get_lol_chat_v1_session() -> GetLolChatV1Session {
    GetLolChatV1Session{}
}


pub struct GetLolChatV1Settings {}

impl IsApiRequest for GetLolChatV1Settings {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/settings".to_string()}
}

pub fn get_lol_chat_v1_settings() -> GetLolChatV1Settings {
    GetLolChatV1Settings{}
}


pub struct GetLolChatV2FriendRequests {}

impl IsApiRequest for GetLolChatV2FriendRequests {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolChatFriendRequestResource>;
    fn get_url(&self) -> String {"/lol-chat/v2/friend-requests".to_string()}
}

pub fn get_lol_chat_v2_friend_requests() -> GetLolChatV2FriendRequests {
    GetLolChatV2FriendRequests{}
}


pub struct PostLolChatV1BlockedPlayers {
    pub body: LolChatBlockedPlayerResource,
}

impl IsApiRequest for PostLolChatV1BlockedPlayers {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/blocked-players".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v1_blocked_players(body: LolChatBlockedPlayerResource) -> PostLolChatV1BlockedPlayers {
    PostLolChatV1BlockedPlayers{body}
}


pub struct PostLolChatV1Conversations {
    pub body: LolChatConversationResource,
}

impl IsApiRequest for PostLolChatV1Conversations {
    const METHOD: Method = Method::POST;
    type ReturnType = LolChatConversationResource;
    fn get_url(&self) -> String {"/lol-chat/v1/conversations".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v1_conversations(body: LolChatConversationResource) -> PostLolChatV1Conversations {
    PostLolChatV1Conversations{body}
}


pub struct PostLolChatV1ConversationsByIdMessages {
    pub id: String,
    pub body: LolChatConversationMessageResource,
}

impl IsApiRequest for PostLolChatV1ConversationsByIdMessages {
    const METHOD: Method = Method::POST;
    type ReturnType = LolChatConversationMessageResource;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}/messages", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v1_conversations_by_id_messages(id: String, body: LolChatConversationMessageResource) -> PostLolChatV1ConversationsByIdMessages {
    PostLolChatV1ConversationsByIdMessages{id, body}
}


pub struct PostLolChatV1ConversationsEogChatToggle {
    pub body: bool,
}

impl IsApiRequest for PostLolChatV1ConversationsEogChatToggle {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/conversations/eog-chat-toggle".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v1_conversations_eog_chat_toggle(body: bool) -> PostLolChatV1ConversationsEogChatToggle {
    PostLolChatV1ConversationsEogChatToggle{body}
}


pub struct PostLolChatV1FriendGroups {
    pub body: LolChatGroupResource,
}

impl IsApiRequest for PostLolChatV1FriendGroups {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/friend-groups".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v1_friend_groups(body: LolChatGroupResource) -> PostLolChatV1FriendGroups {
    PostLolChatV1FriendGroups{body}
}


pub struct PostLolChatV1PlayerMutes {
    pub body: LolChatPlayerMuteUpdate,
}

impl IsApiRequest for PostLolChatV1PlayerMutes {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/player-mutes".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v1_player_mutes(body: LolChatPlayerMuteUpdate) -> PostLolChatV1PlayerMutes {
    PostLolChatV1PlayerMutes{body}
}


pub struct PostLolChatV1SystemMutes {
    pub body: LolChatPlayerMuteUpdate,
}

impl IsApiRequest for PostLolChatV1SystemMutes {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/system-mutes".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v1_system_mutes(body: LolChatPlayerMuteUpdate) -> PostLolChatV1SystemMutes {
    PostLolChatV1SystemMutes{body}
}


pub struct PostLolChatV2FriendRequests {
    pub body: LolChatFriendRequestResource,
}

impl IsApiRequest for PostLolChatV2FriendRequests {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v2/friend-requests".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_chat_v2_friend_requests(body: LolChatFriendRequestResource) -> PostLolChatV2FriendRequests {
    PostLolChatV2FriendRequests{body}
}


pub struct PutLolChatV1ConversationsActive {
    pub body: LolChatActiveConversationResource,
}

impl IsApiRequest for PutLolChatV1ConversationsActive {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/conversations/active".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_chat_v1_conversations_active(body: LolChatActiveConversationResource) -> PutLolChatV1ConversationsActive {
    PutLolChatV1ConversationsActive{body}
}


pub struct PutLolChatV1ConversationsById {
    pub id: String,
    pub body: LolChatConversationResource,
}

impl IsApiRequest for PutLolChatV1ConversationsById {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolChatConversationResource;
    fn get_url(&self) -> String {format!("/lol-chat/v1/conversations/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_chat_v1_conversations_by_id(id: String, body: LolChatConversationResource) -> PutLolChatV1ConversationsById {
    PutLolChatV1ConversationsById{id, body}
}


pub struct PutLolChatV1FriendGroupsById {
    pub id: u32,
    pub body: LolChatGroupResource,
}

impl IsApiRequest for PutLolChatV1FriendGroupsById {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/friend-groups/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_chat_v1_friend_groups_by_id(id: u32, body: LolChatGroupResource) -> PutLolChatV1FriendGroupsById {
    PutLolChatV1FriendGroupsById{id, body}
}


pub struct PutLolChatV1FriendGroupsOrder {
    pub body: LolChatFriendGroupOrder,
}

impl IsApiRequest for PutLolChatV1FriendGroupsOrder {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/friend-groups/order".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_chat_v1_friend_groups_order(body: LolChatFriendGroupOrder) -> PutLolChatV1FriendGroupsOrder {
    PutLolChatV1FriendGroupsOrder{body}
}


pub struct PutLolChatV1FriendsById {
    pub id: String,
    pub body: LolChatFriendResource,
}

impl IsApiRequest for PutLolChatV1FriendsById {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v1/friends/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_chat_v1_friends_by_id(id: String, body: LolChatFriendResource) -> PutLolChatV1FriendsById {
    PutLolChatV1FriendsById{id, body}
}


pub struct PutLolChatV1Me {
    pub body: LolChatUserResource,
}

impl IsApiRequest for PutLolChatV1Me {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolChatUserResource;
    fn get_url(&self) -> String {"/lol-chat/v1/me".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_chat_v1_me(body: LolChatUserResource) -> PutLolChatV1Me {
    PutLolChatV1Me{body}
}


pub struct PutLolChatV1Settings {
    pub data: Value,
    pub do_async: Option<bool>,
}

impl IsApiRequest for PutLolChatV1Settings {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-chat/v1/settings".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("data".to_string(), serde_json::to_string(&self.data).unwrap()),
            ("doAsync".to_string(), serde_json::to_string(&self.do_async).unwrap())
        ])
    }
}

pub fn put_lol_chat_v1_settings(data: Value, do_async: Option<bool>) -> PutLolChatV1Settings {
    PutLolChatV1Settings{data, do_async}
}


pub struct PutLolChatV2FriendRequestsById {
    pub id: String,
    pub body: LolChatFriendRequestResource,
}

impl IsApiRequest for PutLolChatV2FriendRequestsById {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-chat/v2/friend-requests/{}", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_chat_v2_friend_requests_by_id(id: String, body: LolChatFriendRequestResource) -> PutLolChatV2FriendRequestsById {
    PutLolChatV2FriendRequestsById{id, body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatActiveConversationResource {
    pub id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatBlockedPlayerResource {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub icon: i32,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameTag")]
    pub game_tag: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatDomainConfig {
    #[serde(rename = "P2PDomainName")]
    pub p2_p_domain_name: Option<String>,
    #[serde(rename = "CustomGameDomainName")]
    pub custom_game_domain_name: Option<String>,
    #[serde(rename = "ChampSelectDomainName")]
    pub champ_select_domain_name: Option<String>,
    #[serde(rename = "PostGameDomainName")]
    pub post_game_domain_name: Option<String>,
    #[serde(rename = "ClubDomainName")]
    pub club_domain_name: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatServiceDynamicClientConfig {
    #[serde(rename = "LcuSocial")]
    pub lcu_social: Option<LolChatLcuSocialConfig>,
    #[serde(rename = "ChatDomain")]
    pub chat_domain: Option<LolChatChatDomainConfig>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatContentCookies {
    pub content_id: String,
    pub content_path: String,
    pub cookies: Vec<LolChatcookie>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationMessageResource {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "fromSummonerId")]
    pub from_summoner_id: u64,
    #[serde(rename = "fromId")]
    pub from_id: String,
    #[serde(rename = "fromPid")]
    pub from_pid: String,
    #[serde(rename = "fromObfuscatedSummonerId")]
    pub from_obfuscated_summoner_id: u64,
    pub body: String,
    pub timestamp: String,
    #[serde(rename = "isHistorical")]
    pub is_historical: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationResource {
    pub id: String,
    pub name: String,
    pub pid: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameTag")]
    pub game_tag: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "inviterId")]
    pub inviter_id: String,
    pub password: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
    #[serde(rename = "unreadMessageCount")]
    pub unread_message_count: u64,
    #[serde(rename = "lastMessage")]
    pub last_message: Option<LolChatConversationMessageResource>,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolChatMucJwtDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatErrorResource {
    pub id: u64,
    pub from: String,
    pub code: u64,
    pub message: String,
    pub text: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendCountsResource {
    #[serde(rename = "numFriends")]
    pub num_friends: u32,
    #[serde(rename = "numFriendsOnline")]
    pub num_friends_online: u32,
    #[serde(rename = "numFriendsAvailable")]
    pub num_friends_available: u32,
    #[serde(rename = "numFriendsAway")]
    pub num_friends_away: u32,
    #[serde(rename = "numFriendsInQueue")]
    pub num_friends_in_queue: u32,
    #[serde(rename = "numFriendsInChampSelect")]
    pub num_friends_in_champ_select: u32,
    #[serde(rename = "numFriendsInGame")]
    pub num_friends_in_game: u32,
    #[serde(rename = "numFriendsMobile")]
    pub num_friends_mobile: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendGroupOrder {
    pub groups: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequestResource {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub icon: i32,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    pub note: String,
    pub direction: LolChatFriendRequestDirection,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendResource {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameTag")]
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    #[serde(rename = "statusMessage")]
    pub status_message: String,
    pub note: String,
    #[serde(rename = "lastSeenOnlineTimestamp")]
    pub last_seen_online_timestamp: Option<String>,
    #[serde(rename = "isP2PConversationMuted")]
    pub is_p2_p_conversation_muted: bool,
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(rename = "displayGroupId")]
    pub display_group_id: u32,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "displayGroupName")]
    pub display_group_name: String,
    pub lol: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGroupResource {
    pub id: u32,
    pub name: String,
    #[serde(rename = "isMetaGroup")]
    pub is_meta_group: bool,
    #[serde(rename = "isLocalized")]
    pub is_localized: bool,
    pub priority: i32,
    pub collapsed: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLcuSocialConfig {
    #[serde(rename = "ForceChatFilter")]
    pub force_chat_filter: bool,
    #[serde(rename = "QueueJobGraceSeconds")]
    pub queue_job_grace_seconds: u64,
    #[serde(rename = "SilenceChatWhileInGame")]
    pub silence_chat_while_in_game: bool,
    #[serde(rename = "AggressiveScanning")]
    pub aggressive_scanning: bool,
    #[serde(rename = "ReplaceRichMessages")]
    pub replace_rich_messages: bool,
    #[serde(rename = "gameNameTaglineEnabled")]
    pub game_name_tagline_enabled: bool,
    #[serde(rename = "allowGroupByGame")]
    pub allow_group_by_game: bool,
    #[serde(rename = "platformToRegionMap")]
    pub platform_to_region_map: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPatchlineMetadata {
    pub product_id: String,
    pub id: String,
    pub content_paths: HashMap<String, String>,
    pub content_cookies: Vec<LolChatContentCookies>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPlayerMuteStatus {
    pub puuid: String,
    #[serde(rename = "obfuscatedPuuid")]
    pub obfuscated_puuid: String,
    #[serde(rename = "isPlayerMuted")]
    pub is_player_muted: bool,
    #[serde(rename = "isSettingsMuted")]
    pub is_settings_muted: bool,
    #[serde(rename = "isSystemMuted")]
    pub is_system_muted: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPlayerMuteUpdate {
    pub puuids: Vec<String>,
    #[serde(rename = "isMuted")]
    pub is_muted: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatProductMetadata {
    pub id: String,
    pub name: String,
    pub patchlines: HashMap<String, LolChatPatchlineMetadata>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatProductMetadataMap {
    pub products: HashMap<String, LolChatProductMetadata>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSessionResource {
    #[serde(rename = "sessionState")]
    pub session_state: LolChatSessionState,
    #[serde(rename = "sessionExpire")]
    pub session_expire: u32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatUserResource {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    #[serde(rename = "obfuscatedSummonerId")]
    pub obfuscated_summoner_id: u64,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "gameTag")]
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    #[serde(rename = "statusMessage")]
    pub status_message: Option<String>,
    #[serde(rename = "lastSeenOnlineTimestamp")]
    pub last_seen_online_timestamp: Option<String>,
    pub lol: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatcookie {
    pub url: String,
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub httponly: bool,
    pub expires: Option<i64>,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChatFriendRequestDirection {
    #[default]
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "in")]
    In,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolChatSessionState {
    #[default]
    #[serde(rename = "shuttingdown")]
    Shuttingdown,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "initializing")]
    Initializing,
}

