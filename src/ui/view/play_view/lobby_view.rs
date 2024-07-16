use iced::advanced::Widget;
use iced::Alignment::Center;
use iced::Command;
use iced::widget::{Column, Container, container, Row, text};
use iced_box::icon::material::Material;
use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_lobby::post_lobby::LolLobbySession;
use crate::client::apis::lol_matchmaking::get_search::{LolMatchmakingMatchmakingReadyCheckResponse, LolMatchmakingMatchmakingSearchResource, LolMatchmakingMatchmakingSearchState};
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::play_view::create_lobby_view::CreateLobbyMessage;
use crate::ui::widget::{custom_button, icons_builder};
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct LobbyState {
    pub session: Option<LolLobbySession>,
    pub matchmaking_session: Option<LolMatchmakingMatchmakingSearchResource>,
}

#[derive(Debug, Clone)]
pub enum LobbyMessage {
    LobbySessionResult(AppResult<LolLobbySession>),
    MatchmakingSearchResult(AppResult<LolMatchmakingMatchmakingSearchResource>),
    PromoteLobbySummoner(i64),
    QuitLobby,
    FindMatch,
    CancelMatchmaking,
    AcceptReadyCheck,
    DeclineReadyCheck,

}

pub struct LobbyView {}

impl HasView for LobbyView {
    type State = LobbyState;
    type Message = LobbyMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                LobbyMessage::LobbySessionResult(result) => {
                    connected_state.play.lobby_state.session = result.ok();
                }
                LobbyMessage::MatchmakingSearchResult(result) => {
                    connected_state.play.lobby_state.matchmaking_session = result.ok();
                }
                LobbyMessage::QuitLobby => {
                    return perform_request(
                        connected_state,
                        apis::lol_lobby::delete_lobby(),
                        |r| CreateLobbyMessage::DeleteLobbyResult.into(),
                    );
                }
                LobbyMessage::FindMatch => {
                    if let Some(session) = &connected_state.play.lobby_state.session {
                        return if !session.game_config.is_custom {
                            perform_request(
                                connected_state,
                                apis::lol_lobby::post_matchmaking_search(),
                                |r| Message::None,
                            )
                        } else {
                            perform_request(
                                connected_state,
                                apis::lol_lobby::post_start_custom_lobby(),
                                |r| Message::None,
                            )
                        };
                    }
                }
                LobbyMessage::CancelMatchmaking => {
                    if let Some(session) = &connected_state.play.lobby_state.session {
                        if !session.game_config.is_custom {
                            return perform_request(
                                connected_state,
                                apis::lol_lobby::delete_matchmaking_search(),
                                |r| Message::None,
                            );
                        }
                    }
                }
                LobbyMessage::AcceptReadyCheck => {
                    return perform_request(
                        connected_state,
                        apis::lol_matchmaking::post_ready_check_accept(),
                        |r| Message::None,
                    );
                }
                LobbyMessage::DeclineReadyCheck => {
                    return perform_request(
                        connected_state,
                        apis::lol_matchmaking::post_ready_check_decline(),
                        |r| Message::None,
                    );
                }
                LobbyMessage::PromoteLobbySummoner(summoner_id) => {
                    return perform_request(
                        connected_state,
                        apis::lol_lobby::promote_lobby_summoner(summoner_id),
                        |r| Message::None,
                    );
                }
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
                    .push(
                        Row::new()
                            .push(
                                text(member.summoner_name.as_str()).size(25)
                            )
                            .push_maybe(
                                if session.local_member.is_leader && member.summoner_id != session.local_member.summoner_id {
                                    Some(
                                        custom_button("Promote")
                                            .on_press(LobbyMessage::PromoteLobbySummoner(member.summoner_id).into())
                                            .style(custom_button::primary)
                                    )
                                } else if member.is_leader {
                                    Some(custom_button(icons_builder(Material::Star).build()))
                                }else{
                                    None
                                }
                            )
                            .spacing(20)
                            .align_items(Center)
                    )
            }
            result = result.push(if let Some(matchmaking_session) = &connected_state.play.lobby_state.matchmaking_session {
                match matchmaking_session.search_state {
                    LolMatchmakingMatchmakingSearchState::Searching => {
                        Column::new()
                            .push(text("Searching..."))
                            .push(
                                custom_button("Cancel")
                                    .on_press(Self::Message::CancelMatchmaking.into())
                                    .style(custom_button::danger)
                            )
                    }
                    LolMatchmakingMatchmakingSearchState::Found => {
                        Column::new()
                            .push(text(format!("Ready Check: {:?}", matchmaking_session.ready_check.state)))
                            .push(text(format!("Time Remaining: {}", 12.0 - matchmaking_session.ready_check.timer)))
                            .push(
                                if matchmaking_session.ready_check.player_response == LolMatchmakingMatchmakingReadyCheckResponse::None {
                                    Row::new()
                                        .push(
                                            custom_button("Accept")
                                                .on_press(Self::Message::AcceptReadyCheck.into())
                                                .style(custom_button::primary)
                                        )
                                        .push(
                                            custom_button("Decline")
                                                .on_press(Self::Message::DeclineReadyCheck.into())
                                                .style(custom_button::danger)
                                        )
                                } else {
                                    Row::new()
                                        .push(
                                            text(format!("Player Choice: {:?}", matchmaking_session.ready_check.player_response))
                                        )
                                }
                            )
                    }
                    LolMatchmakingMatchmakingSearchState::Canceled => {
                        Column::new()
                            .push(text("Cancelled"))
                    }
                    LolMatchmakingMatchmakingSearchState::Invalid
                    | LolMatchmakingMatchmakingSearchState::AbandonedLowPriorityQueue
                    | LolMatchmakingMatchmakingSearchState::Error
                    | LolMatchmakingMatchmakingSearchState::ServiceError
                    | LolMatchmakingMatchmakingSearchState::ServiceShutdown => Column::new()
                }
            } else {
                Column::new().push(
                    Row::new()
                        .push_maybe(
                            if session.local_member.is_leader {
                                Some(custom_button("Find Match")
                                    .on_press(Self::Message::FindMatch.into())
                                    .style(custom_button::primary))
                            } else { None }
                        )
                        .push(
                            custom_button("Quit")
                                .on_press(Self::Message::QuitLobby.into())
                                .style(custom_button::danger)
                        )
                )
            });
        }
        container(result)
            .center_x()
            .center_y()
    }
}