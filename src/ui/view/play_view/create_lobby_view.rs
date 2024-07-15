use iced::Command;
use iced::widget::{Column, combo_box, Container, container, text};
use serde_json::Value;
use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueue;
use crate::client::apis::lol_lobby::post_lobby::LolLobbyPostLobbyBody;
use crate::client::utils::{disconnect_if_disconnect_error, perform_request};
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::play_view::lobby_view::{LobbyMessage, LobbyState, LobbyView};
use crate::ui::view::play_view::PlayMessage;
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct CreateLobbyState {
    selected_queue: Option<LolGameQueuesGetQueue>,
}

#[derive(Debug, Clone)]
pub enum CreateLobbyMessage {
    Selected(LolGameQueuesGetQueue),
    DeleteLobbyResult,
    StartQueue,
}

pub struct CreateLobbyView {}

impl HasView for CreateLobbyView {
    type State = CreateLobbyState;
    type Message = CreateLobbyMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                CreateLobbyMessage::Selected(queue) => {
                    connected_state.play.create_lobby_state.selected_queue = Some(queue);
                }

                CreateLobbyMessage::StartQueue => {
                    if let Some(queue) = &connected_state.play.create_lobby_state.selected_queue {
                        let queue = queue.clone();
                        return perform_request(
                            connected_state,
                            apis::lol_lobby::post_lobby(LolLobbyPostLobbyBody{queue_id:queue.id}),
                            |r|LobbyMessage::LobbySessionResult(r).into()
                        );
                    }
                }
                CreateLobbyMessage::DeleteLobbyResult => {
                    connected_state.play.lobby_state = LobbyState::default();
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        let combo_box = combo_box(
            &connected_state.play.queues,
            "Type a Queue...",
            connected_state.play.create_lobby_state.selected_queue.as_ref(),
            |r|CreateLobbyMessage::Selected(r).into(),
        ).width(250);
        container(Column::new()
            .push(text("Create Lobby").size(25))
            .push(combo_box)
            .push(
                custom_button("Confirm")
                    .on_press(Self::Message::StartQueue.into())
                    .style(custom_button::primary)
            )
        ).center_x()
            .center_y()
    }
}