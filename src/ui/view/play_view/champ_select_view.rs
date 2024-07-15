
use iced::Command;
use iced::widget::{Column, Container, container, Row, text};
use serde_json::Value;
use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_champ_select::get_session::LolChampSelectChampSelectSession;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailability;
use crate::client::apis::lol_game_flow::get_session::LolGameFlowGetSession;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueues;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;

#[derive(Debug, Clone, Default)]
pub struct ChampSelectState {
    session: Option<LolChampSelectChampSelectSession>,
}

#[derive(Debug, Clone)]
pub enum ChampSelectMessage {
    ChampSelectSessionResult(AppResult<LolChampSelectChampSelectSession>),
}

pub struct ChampSelectView {}

impl HasView for ChampSelectView {
    type State = ChampSelectState;
    type Message = ChampSelectMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                ChampSelectMessage::ChampSelectSessionResult(result) => {
                    if let Ok(result) = result {
                        connected_state.play.champ_select_state.session = Some(result);
                    }
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(text("Champ Select").size(25))
        ).center_x()
            .center_y()
    }
}