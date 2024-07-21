use std::fmt;
use crate::{LolChatConversationResource, LolChatFriendResource};

impl PartialEq for LolChatFriendResource {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for LolChatFriendResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


impl PartialEq for LolChatConversationResource{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl fmt::Display for LolChatConversationResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}