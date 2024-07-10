use iced::Command;
use iced::widget::{Column, Container, container, text};
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::nav_bar_view::NavBarMessage;

#[derive(Debug, Clone, Default)]
pub struct ChatState {
}

#[derive(Debug, Clone)]
pub enum ChatMessage {}


pub struct ChatView {}

impl HasView for ChatView {
    type State = ChatState;
    type Message = ChatMessage;

    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message> {
        if let Some(_connected_state) = connected_state {
            match message {}
        } else {
            Command::none()
        }
    }
    fn view(connected_sate: &ConnectedState) -> Container<'_, Message> {
        container(
            Column::new()
        )
            .center_y()
    }
}
