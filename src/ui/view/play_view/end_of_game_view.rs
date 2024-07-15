use iced::Command;
use iced::widget::{Column, Container, container, text};

use crate::client::apis;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct EndOfGameState {}

#[derive(Debug, Clone)]
pub enum EndOfGameMessage {
    ContinuePreEndGame,
    DismissStats,
}

pub struct EndOfGameView {}

impl HasView for EndOfGameView {
    type State = EndOfGameState;
    type Message = EndOfGameMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                EndOfGameMessage::ContinuePreEndGame => {
                    return perform_request(connected_state, apis::lol_game_flow::post_pre_end_game_transition(), |r| Message::None);
                }
                EndOfGameMessage::DismissStats => {
                    return perform_request(connected_state, apis::lol_end_of_game::post_dismiss_stats(), |r| Message::None);
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(text("End Of Game").size(25))
            .push_maybe(match connected_state.state {
                LolGameFlowPhase::WaitingForStats => {
                    Some(Column::new().push(text("Waiting for stats to be available")))
                }
                LolGameFlowPhase::PreEndOfGame => {
                    Some(Column::new()
                        .push(text("Pre end game"))
                        .push(
                            custom_button("Continue")
                                .style(custom_button::primary)
                                .on_press(Self::Message::ContinuePreEndGame.into())
                        )
                    )
                }
                LolGameFlowPhase::EndOfGame => {
                    Some(Column::new()
                        .push(
                            text("Stats not implemented")
                        )
                        .push(
                            custom_button("Continue")
                                .style(custom_button::primary)
                                .on_press(Self::Message::DismissStats.into())
                        )
                    )
                }
                _ => None
            })
        ).center_x()
            .center_y()
    }
}