use iced::Command;
use iced::widget::{Column, Container, container, text};
use crate::client::apis;
use crate::client::apis::lol_game_queues::get_queues::LolGameMode;
use crate::client::apis::lol_lobby::post_lobby::{LolLobbyLobbyCustomGameConfiguration, LolLobbyLobbyCustomGameLobby, LolLobbyPostLobbyBody, LolLobbyQueueCustomGameSpectatorPolicy, LolLobbyQueueGameTypeConfig};
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
                        apis::lol_lobby::post_lobby(LolLobbyPostLobbyBody{
                            queue_id: 0,
                            is_custom: true,
                            custom_game_lobby: Some(LolLobbyLobbyCustomGameLobby{
                                lobby_name:   "Practice Tool".to_string(),
                                lobby_password: None,
                                configuration: LolLobbyLobbyCustomGameConfiguration{
                                    map_id: 11,
                                    game_mode: LolGameMode::OTHER("PRACTICETOOL".to_string()),
                                    mutators: LolLobbyQueueGameTypeConfig{id:1},
                                    spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy::FriendsAllowed,
                                    team_size: 1,
                                },
                            }),
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