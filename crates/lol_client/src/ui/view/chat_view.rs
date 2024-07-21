use iced::Command;
use iced::widget::{Column, combo_box, Container, container, pick_list, text};
use serde_json::Value;
use plugin_lol_chat::{LolChatConversationResource, LolChatFriendResource, LolChatUserResource};
use crate::AppResult;

use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::nav_bar_view::NavBarMessage;

#[derive(Debug, Clone)]
pub struct ChatState {
    pub me:LolChatUserResource,
    pub conversations: Vec<LolChatConversationResource>,
    pub friends: combo_box::State<LolChatFriendResource>,
    pub selected_conversation: Option<LolChatConversationResource>,
    pub selected_friend: Option<LolChatFriendResource>,
}

impl Default for ChatState{
    fn default() -> Self {
        Self {
            me: Default::default(),
            conversations: Default::default(),
            friends: combo_box::State::new(vec!()),
            selected_conversation: None,
            selected_friend: None,
        }
    }
}


#[derive(Debug, Clone)]
pub enum ChatMessage {
    ConversationsResult(AppResult<Vec<LolChatConversationResource>>),
    FriendsResult(AppResult<Vec<LolChatFriendResource>>),
    ConversationSelected(LolChatConversationResource),
    FriendSelected(LolChatFriendResource),
}


pub struct ChatView {}

impl HasView for ChatView {
    type State = ChatState;
    type Message = ChatMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                ChatMessage::ConversationsResult(result) => {
                    connected_state.chat.conversations = result.unwrap_or(vec!());
                    if connected_state.chat.conversations.len() == 0 {
                        connected_state.chat.selected_conversation = None;
                    }
                }
                ChatMessage::ConversationSelected(selected) => {
                    connected_state.chat.selected_conversation = Some(selected.clone());
                }
                ChatMessage::FriendsResult(result) => {
                    if let Ok(result) = result {
                        println!("Got friends: {}", result.len());
                        connected_state.chat.friends = combo_box::State::new(result);
                    }else{
                        println!("Error getting friends: {:?}", result);
                        connected_state.chat.friends = combo_box::State::new(vec!());
                    }
                }
                ChatMessage::FriendSelected(selected) => {
                    connected_state.chat.selected_friend = Some(selected.clone());
                    connected_state.chat.selected_conversation = connected_state.chat.conversations.iter().find(|c| c.pid == selected.pid).cloned();
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        let pick_list = pick_list(
            &*connected_state.chat.conversations,
            connected_state.chat.selected_conversation.as_ref(),
            |r|Self::Message::ConversationSelected(r).into(),
        )
            .placeholder("Choose a conversation");
        let friends = combo_box(
            &connected_state.chat.friends,
            "Choose a friend",
            connected_state.chat.selected_friend.as_ref(),
            |r| Self::Message::FriendSelected(r).into(),
        );
        container(
            Column::new()
                .push(pick_list)
                .push(friends)
        )
            .center_y()
    }
}
