use iced::Command;
use iced::widget::{Column, Container, container, pick_list, text, text_input};
use plugin_lol_game_queues::{LolGameQueuesQueueCustomGameSubcategory, LolGameQueuesQueueGameTypeConfig};
use plugin_lol_lobby::{LolLobbyLobbyChangeGameDto, LolLobbyLobbyCustomGameConfiguration, LolLobbyLobbyCustomGameLobby, LolLobbyQueueCustomGameSpectatorPolicy, LolLobbyQueueGameTypeConfig};
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone)]
pub struct CreateLobbyCustomState {
    selected_queue: Option<LolGameQueuesQueueCustomGameSubcategory>,
    selected_mutator: Option<LolGameQueuesQueueGameTypeConfig>,
    name: String,
    password: String,
    team_size: i32,
    spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
}

impl Default for CreateLobbyCustomState{
    fn default() -> Self {
        Self {
            selected_queue: None,
            name: format!("My custom game {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            password: "".to_string(),
            team_size: 5,
            spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy::LobbyAllowed,
            selected_mutator : None,
        }
    }
}


#[derive(Debug, Clone)]
pub enum CreateLobbyCustomMessage {
    SelectQueue(LolGameQueuesQueueCustomGameSubcategory),
    SelectMutator(LolGameQueuesQueueGameTypeConfig),
    SelectSpectatorPolicy(LolLobbyQueueCustomGameSpectatorPolicy),
    UpdateName(String),
    UpdatePassword(String),
    UpdateTeamSize(i32),
    StartQueue,
}

pub struct CreateLobbyCustomView {}

impl HasView for CreateLobbyCustomView {
    type State = CreateLobbyCustomState;
    type Message = CreateLobbyCustomMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                CreateLobbyCustomMessage::SelectQueue(queue) => {
                    connected_state.play.create_lobby_state.custom_state.selected_queue = Some(queue.clone());
                    if let Some(mutator) = &mut connected_state.play.create_lobby_state.custom_state.selected_mutator{
                        if !queue.mutators.contains(mutator){
                            connected_state.play.create_lobby_state.custom_state.selected_mutator = None;
                        }
                    }
                }
                CreateLobbyCustomMessage::SelectMutator(mutator) => {
                    connected_state.play.create_lobby_state.custom_state.selected_mutator = Some(mutator);
                }
                CreateLobbyCustomMessage::SelectSpectatorPolicy(spectator_policy) => {
                    connected_state.play.create_lobby_state.custom_state.spectator_policy = spectator_policy;
                }
                CreateLobbyCustomMessage::UpdateName(name) => {
                    connected_state.play.create_lobby_state.custom_state.name = name;
                }
                CreateLobbyCustomMessage::UpdatePassword(password) => {
                    connected_state.play.create_lobby_state.custom_state.password = password;
                }
                CreateLobbyCustomMessage::UpdateTeamSize(team_size) => {
                    connected_state.play.create_lobby_state.custom_state.team_size = team_size;
                }
                CreateLobbyCustomMessage::StartQueue => {
                    if let Some(queue) = &connected_state.play.create_lobby_state.custom_state.selected_queue{
                        if let Some(mutator) = &connected_state.play.create_lobby_state.custom_state.selected_mutator {
                            return crate::client::utils::perform_request(
                                connected_state,
                                plugin_lol_lobby::post_lol_lobby_v_2_lobby(
                                    LolLobbyLobbyChangeGameDto {
                                        queue_id: 0,
                                        is_custom: true,
                                        custom_game_lobby: LolLobbyLobbyCustomGameLobby{
                                            lobby_name: connected_state.play.create_lobby_state.custom_state.name.clone(),
                                            lobby_password: connected_state.play.create_lobby_state.custom_state.password.clone(),
                                            configuration: LolLobbyLobbyCustomGameConfiguration {
                                                map_id: queue.map_id,
                                                game_mode: queue.game_mode.clone(),
                                                mutators: LolLobbyQueueGameTypeConfig { id: mutator.id, ..Default::default() },
                                                spectator_policy: connected_state.play.create_lobby_state.custom_state.spectator_policy.clone(),
                                                team_size: connected_state.play.create_lobby_state.custom_state.team_size,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }
                                ),
                                |r| Message::None
                            );
                        }
                    }
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(
            Column::new()
                .push(text("Create Custom Lobby"))
                .push(
                    pick_list(
                        connected_state.play.custom_queues.clone(),
                        connected_state.play.create_lobby_state.custom_state.selected_queue.as_ref(),
                        |r| CreateLobbyCustomMessage::SelectQueue(r).into()
                    )
                )
                .push_maybe(
                    if let Some(queue) = &connected_state.play.create_lobby_state.custom_state.selected_queue{
                        Some(pick_list(
                            queue.mutators.iter().collect::<Vec<&LolGameQueuesQueueGameTypeConfig>>(),
                            connected_state.play.create_lobby_state.custom_state.selected_mutator.as_ref(),
                            |r| CreateLobbyCustomMessage::SelectMutator(r.clone()).into()
                        ))
                    } else{None}
                )
                .push(
                    text_input(
                        "Name",
                        &connected_state.play.create_lobby_state.custom_state.name,
                    ).on_input(|s|CreateLobbyCustomMessage::UpdateName(s).into())
                )
                .push(
                    text_input(
                        "Password",
                        &connected_state.play.create_lobby_state.custom_state.password,
                    ).on_input(|s|CreateLobbyCustomMessage::UpdatePassword(s).into())
                )
                .push(
                    text_input(
                        "Team Size",
                        &connected_state.play.create_lobby_state.custom_state.team_size.to_string(),
                    ).on_input(|s| CreateLobbyCustomMessage::UpdateTeamSize(s.parse().unwrap_or(0)).into())
                )
                .push(
                    pick_list(
                        LolLobbyQueueCustomGameSpectatorPolicy::cases(),
                        Some(&connected_state.play.create_lobby_state.custom_state.spectator_policy),
                        |r| CreateLobbyCustomMessage::SelectSpectatorPolicy(r).into()
                    )
                )
                .push_maybe(
                    connected_state.play.create_lobby_state.custom_state.selected_queue.as_ref().map(|q|{
                        custom_button("Confirm").on_press(CreateLobbyCustomMessage::StartQueue.into())
                    })
                )
                .spacing(20)
        ).center_x()
            .center_y()
    }
}