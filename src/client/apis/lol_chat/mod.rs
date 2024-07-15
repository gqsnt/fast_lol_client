use reqwest::Method;
use serde_json::Value;
use crate::client::apis::lol_chat::get_conversations::LolChatConversationResource;
use crate::client::apis::lol_chat::get_friends::LolChatFriendsResource;
use crate::client::apis::lol_chat::get_me::LolChatUserResource;
use crate::client::apis::plugin_macro::impl_api_plugin;
pub mod get_me;
pub mod get_conversations;
pub mod get_friends;

impl_api_plugin!(
    "/lol-chat",
    V1{
        GetMe {
            get_me,Method::GET,"/me"
        } =>LolChatUserResource
        GetConversations{
            get_conversations,Method::GET,"/conversations"
        } =>LolChatConversationResource
        GetFriends{
            get_friends,Method::GET,"/friends"
        } => LolChatFriendsResource
    }
);