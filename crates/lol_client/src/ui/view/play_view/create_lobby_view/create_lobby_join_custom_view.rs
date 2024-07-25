use iced::Command;
use iced::widget::{Column, Container, container, text};
use crate::assets::Assets;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;


#[derive(Debug, Clone, Default)]
pub struct CreateLobbyJoinCustomState {}

#[derive(Debug, Clone)]
pub enum CreateLobbyJoinCustomMessage {
}

pub struct CreateLobbyJoinCustomView {}

impl HasView for CreateLobbyJoinCustomView {
    type State = CreateLobbyJoinCustomState;
    type Message = CreateLobbyJoinCustomMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
            }
        }
        Command::none()
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        container(
            Column::new()
                .push(text("Join Custom Game"))
                .push(text("Not Implemented"))
        ).center_x()
            .center_y()
    }
}