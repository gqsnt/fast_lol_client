use iced::Command;
use iced::widget::{Container, container, Row};

use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;

#[derive(Debug, Clone, Default)]
pub struct PlayState {}

#[derive(Debug, Clone)]
pub enum PlayMessage {}

pub struct PlayView {}

impl HasView for PlayView {
    type State = PlayState;
    type Message = PlayMessage;

    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message> {
        if let Some(connected_state) = connected_state {
            Command::none()
        } else {
            Command::none()
        }
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Row::new()).center_x()
            .center_y()
    }
}