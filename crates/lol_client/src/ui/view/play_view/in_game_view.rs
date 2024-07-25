
use iced::Command;
use iced::widget::{Column, Container, container, Row, text};
use serde_json::Value;
use plugin_lol_gameflow::LolGameflowGameflowPhase;
use crate::AppResult;
use crate::assets::Assets;
use crate::client::apis;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct InGameState {

}

#[derive(Debug, Clone)]
pub enum InGameMessage {
    Reconnect
}

pub struct InGameView {}

impl HasView for InGameView {
    type State = InGameState;
    type Message = InGameMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                InGameMessage::Reconnect => {
                    return perform_request(connected_state, plugin_lol_gameflow::post_lol_gameflow_v1_reconnect(), |r| Message::None);
                }
            }
        }
        Command::none()
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        container(Column::new()
            .push(
                text("In Game").size(25)
            )
            .push_maybe(if connected_state.state == LolGameflowGameflowPhase::Reconnect{
                Some(custom_button("Reconnect").style(custom_button::primary).on_press(Self::Message::Reconnect.into()))
            }else{
                None
            })
        ).center_x()
            .center_y()
    }
}