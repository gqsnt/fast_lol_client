
use iced::Command;
use iced::widget::{Column, Container, container, Row, text};
use serde_json::Value;
use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailability;
use crate::client::apis::lol_game_flow::get_session::LolGameFlowGetSession;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueues;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;

#[derive(Debug, Clone, Default)]
pub struct PostGameState {

}

#[derive(Debug, Clone)]
pub enum PostGameMessage {
}

pub struct PostGameView {}

impl HasView for PostGameView {
    type State = PostGameState;
    type Message = PostGameMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {}
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(text("Game Done").size(25))
        ).center_x()
            .center_y()
    }
}