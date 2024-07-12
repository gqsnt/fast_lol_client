use iced::Command;
use iced::widget::{Column, Container, container, text};

use crate::AppResult;
use crate::client::apis::lol_lobby::post_lobby::LolLobbySession;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;

#[derive(Debug, Clone, Default)]
pub struct LobbyState {
    pub session: Option<LolLobbySession>,
}

#[derive(Debug, Clone)]
pub enum LobbyMessage {
    RequestLobbyResult(AppResult<LolLobbySession>),
    IsReadyCheck,
    AcceptReadyCheck,
    AcceptReadyCheckResult,
    StartLobby,
    StartLobbyResult,
}

pub struct LobbyView {}

impl HasView for LobbyView {
    type State = LobbyState;
    type Message = LobbyMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                LobbyMessage::RequestLobbyResult(result) => {
                    connected_state.play.lobby_state.session = result.ok();
                }
                LobbyMessage::IsReadyCheck => {}
                LobbyMessage::AcceptReadyCheck => {}
                LobbyMessage::AcceptReadyCheckResult => {}
                LobbyMessage::StartLobby => {}
                LobbyMessage::StartLobbyResult => {}
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        let mut result = Column::new()
            .push(text("Lobby").size(25));

        if let Some(session) = &connected_state.play.lobby_state.session {
            result = result
                .push(text(format!("Game Mod: {}", session.game_config.game_mode)));
            for member in &session.members {
                result = result
                    .push(text(format!("Member: {}", member.summoner_name)));
            }

        }
        container(result)
            .center_x()
            .center_y()
    }
}