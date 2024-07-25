use iced::Command;
use iced::widget::{Column, Container, container, pick_list, Row, text};
use plugin_lol_game_queues::LolGameQueuesQueue;
use plugin_lol_lobby::LolLobbyLobbyChangeGameDto;
use crate::assets::Assets;
use crate::client::apis;
use crate::client::apis::LolGameMode;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;

#[derive(Debug, Clone, Default)]
pub struct CreateLobbyPvpState {
    selected_mode: Option<LolGameMode>,
    selected_queue: Option<LolGameQueuesQueue>,
}

#[derive(Debug, Clone)]
pub enum CreateLobbyPvpMessage {
    StartQueue,
    SelectMode(LolGameMode),
    SelectQueue(LolGameQueuesQueue),
}

pub struct CreateLobbyPvpView {}

impl HasView for CreateLobbyPvpView {
    type State = CreateLobbyPvpState;
    type Message = CreateLobbyPvpMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                CreateLobbyPvpMessage::StartQueue => {
                    if let Some(queue) = &connected_state.play.create_lobby_state.pvp_state.selected_queue {
                        return perform_request(
                            connected_state,
                            plugin_lol_lobby::post_lol_lobby_v2_lobby(
                                LolLobbyLobbyChangeGameDto {
                                    queue_id: queue.id,
                                    ..Default::default()
                                }
                            ),
                            |r| Message::None
                        );
                    }
                }
                CreateLobbyPvpMessage::SelectQueue(selected) => {
                    connected_state.play.create_lobby_state.pvp_state.selected_queue = Some(selected);
                }
                CreateLobbyPvpMessage::SelectMode(selected) => {
                    connected_state.play.create_lobby_state.pvp_state.selected_mode = Some(selected);
                    connected_state.play.create_lobby_state.pvp_state.selected_queue = None;
                }
            }
        }
        Command::none()
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        container(Column::new()
            .push(text("Create Lobby PVP"))
            .push(
                Row::new()
                    .push(pick_list(
                        connected_state.play.pvp_queues.keys().collect::<Vec<&LolGameMode>>(),
                        connected_state.play.create_lobby_state.pvp_state.selected_mode.as_ref(),
                        |r| CreateLobbyPvpMessage::SelectMode(r.clone()).into(),
                    ))
                    .push_maybe
                    (
                        if let Some(mode) = &connected_state.play.create_lobby_state.pvp_state.selected_mode {
                            Some(pick_list(
                                connected_state.play.pvp_queues.get(mode).unwrap_or(&vec![]).clone(),
                                connected_state.play.create_lobby_state.pvp_state.selected_queue.as_ref(),
                                |r| CreateLobbyPvpMessage::SelectQueue(r).into(),
                            ))
                        } else {None}
                    )
                    .spacing(20)
            )
            .push_maybe(if let Some(queue) = &connected_state.play.create_lobby_state.pvp_state.selected_queue {
                Some(crate::ui::widget::custom_button::custom_button("Confirm").on_press(CreateLobbyPvpMessage::StartQueue.into()))
            } else {
                None
            })
            .spacing(20)
        ).center_x()
            .center_y()
    }
}