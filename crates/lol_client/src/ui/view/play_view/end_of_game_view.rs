use iced::Command;
use iced::widget::{Column, Container, container, Row, text};
use iced_box::icon::material::Material;
use plugin_lol_gameflow::LolGameflowGameflowPhase;
use crate::assets::Assets;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::{custom_button, icons_builder};
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct EndOfGameState {}

#[derive(Debug, Clone)]
pub enum EndOfGameMessage {
    DeclinePlayAgain,
    PlayAgain,
    DismissStats,
}

pub struct EndOfGameView {}

impl HasView for EndOfGameView {
    type State = EndOfGameState;
    type Message = EndOfGameMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                EndOfGameMessage::DeclinePlayAgain => {
                    return perform_request(connected_state,plugin_lol_lobby::post_lol_lobby_v2_play_again_decline(), |r| Message::None);
                }
                EndOfGameMessage::DismissStats => {
                    return perform_request(connected_state, plugin_lol_end_of_game::post_lol_end_of_game_v1_state_dismiss_stats(), |r| Message::None);
                }
                EndOfGameMessage::PlayAgain => {
                    return perform_request(connected_state, plugin_lol_lobby::post_lol_lobby_v2_play_again(), |r| Message::None);
                }
            }
        }
        Command::none()
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        container(Column::new()
            .push(text("End Of Game").size(25))
            .push_maybe(match connected_state.state {
                LolGameflowGameflowPhase::WaitingForStats => {
                    Some(Column::new().push(text("Waiting for stats to be available")))
                }
                LolGameflowGameflowPhase::PreEndOfGame => {
                    Some(Column::new()
                        .push(text("Pre end game"))
                        .push(
                            custom_button("Continue")
                                .style(custom_button::primary)
                                .on_press(Self::Message::DeclinePlayAgain.into())
                        )
                    )
                }
                LolGameflowGameflowPhase::EndOfGame => {
                    Some(Column::new()
                        .push(
                            text("Stats not implemented")
                        )
                        .push(
                            Row::new()
                                .push(
                                    custom_button(icons_builder(Material::Close).size(15).build())
                                    .style(custom_button::danger)
                                    .on_press(Self::Message::DismissStats.into())
                                )
                                .push(
                                    custom_button("Play Again")
                                        .style(custom_button::primary)
                                        .on_press(Self::Message::PlayAgain.into())
                                )
                                .spacing(20)
                        )
                        .spacing(20)
                    )
                }
                _ => None
            })
        ).center_x()
            .center_y()
    }
}