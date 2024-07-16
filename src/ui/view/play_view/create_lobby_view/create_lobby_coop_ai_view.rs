use iced::Command;
use iced::widget::{Column, Container, container, pick_list, text};
use crate::client::apis;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueue;
use crate::client::apis::lol_lobby::post_lobby::LolLobbyPostLobbyBody;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct CreateLobbyCoopAiState {
    selected_queue: Option<LolGameQueuesGetQueue>

}

#[derive(Debug, Clone)]
pub enum CreateLobbyCoopAiMessage {
    SelectQueue(LolGameQueuesGetQueue),
    StartQueue
}

pub struct CreateLobbyCoopAiView {}

impl HasView for CreateLobbyCoopAiView {
    type State = CreateLobbyCoopAiState;
    type Message = CreateLobbyCoopAiMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                CreateLobbyCoopAiMessage::SelectQueue(selected) => {
                    connected_state.play.create_lobby_state.coop_ai_state.selected_queue = Some(selected);
                }
                CreateLobbyCoopAiMessage::StartQueue => {
                    if let Some(queue) = &connected_state.play.create_lobby_state.coop_ai_state.selected_queue {
                        return perform_request(
                            connected_state,
                            apis::lol_lobby::post_lobby(
                                LolLobbyPostLobbyBody {
                                    queue_id: queue.id,
                                    ..Default::default()
                                }
                            ),
                            |r| Message::None
                        );
                    }
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(text("Create Lobby Coop AI"))
            .push(pick_list(
                connected_state.play.coop_vs_ai_queues.clone(),
                connected_state.play.create_lobby_state.coop_ai_state.selected_queue.as_ref(),
                |r| CreateLobbyCoopAiMessage::SelectQueue(r).into()
            ))
            .push_maybe(
                connected_state.play.create_lobby_state.coop_ai_state.selected_queue.as_ref().map(|q|{
                    custom_button("Confirm").on_press(CreateLobbyCoopAiMessage::StartQueue.into())
                })
            )
            .spacing(20)
        ).center_x()
            .center_y()
    }
}