use iced::Command;
use iced::widget::{Column, Container, container, text};
use plugin_lol_lobby::{LolLobbyLobbyChangeGameDto, LolLobbyLobbyCustomGameConfiguration, LolLobbyLobbyCustomGameLobby, LolLobbyQueueCustomGameSpectatorPolicy, LolLobbyQueueGameTypeConfig};
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct CreateLobbyTrainingState {}

#[derive(Debug, Clone)]
pub enum CreateLobbyTrainingMessage {
    StartPracticeTool,
}

pub struct CreateLobbyTrainingView {}

impl HasView for CreateLobbyTrainingView {
    type State = CreateLobbyTrainingState;
    type Message = CreateLobbyTrainingMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                CreateLobbyTrainingMessage::StartPracticeTool => {
                    return perform_request(
                        connected_state,
                        plugin_lol_lobby::post_lol_lobby_v_2_lobby(LolLobbyLobbyChangeGameDto{
                            queue_id: 0,
                            is_custom: true,
                            custom_game_lobby: LolLobbyLobbyCustomGameLobby{
                                lobby_name:   "Practice Tool".to_string(),
                                lobby_password: "".to_string(),
                                configuration: LolLobbyLobbyCustomGameConfiguration{
                                    map_id: 11,
                                    game_mode: "PRACTICETOOL".to_string(),
                                    mutators: LolLobbyQueueGameTypeConfig{id:1, ..Default::default()},
                                    spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy::FriendsAllowed,
                                    team_size: 1,
                                    ..Default::default()
                                },
                                .. Default::default()
                            },
                            .. Default::default()
                        }),
                        |r| Message::None,
                    );
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(text("Training Mode"))
            .push(text("Practice Tool"))
            .push_maybe(if connected_state.play.practice_tool_queue.is_some(){
                Some(custom_button("Confirm").on_press(CreateLobbyTrainingMessage::StartPracticeTool.into()))
            }else{
                None
            })
            .spacing(20)
        ).center_x()
            .center_y()
    }
}