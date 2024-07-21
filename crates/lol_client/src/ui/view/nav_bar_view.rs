use iced::{Command, Length};
use iced::widget::{Column, Container, container};
use plugin_lol_gameflow::LolGameflowGameflowPhase;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::{ConnectedState};
use crate::ui::view::HasView;
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::{custom_button, CustomButton};

#[derive(Debug, Clone, Default)]
pub struct NavBarState {
    pub state: NavBarMessage,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum NavBarMessage {
    #[default]
    Profile,
    Play,
    Chat,
}

pub struct NavBarView {}

impl HasView for NavBarView {
    type State = NavBarState;
    type Message = NavBarMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state{
            connected_state.nav_bar.state = message;
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        let nav_bar_state = &connected_state.nav_bar;
        container(Column::new()
            .push(nav_button("Profile", get_message_if_not_already(NavBarMessage::Profile, nav_bar_state.state.clone())))
            .push(
                nav_button(
                    "Play",
                    get_message_if_not_already(NavBarMessage::Play, nav_bar_state.state.clone())
                )
                    .style(
                        match connected_state.state {
                            LolGameflowGameflowPhase::None => custom_button::primary,
                            LolGameflowGameflowPhase::ReadyCheck |LolGameflowGameflowPhase::Reconnect => custom_button::danger,
                            LolGameflowGameflowPhase::WaitingForStats => custom_button::secondary,
                            _ => custom_button::success,
                        }
                    )
            )
            .push(nav_button("Chat", get_message_if_not_already(NavBarMessage::Chat, nav_bar_state.state.clone()))
                .style(custom_button::primary))
            .spacing(20)
        ).center_x()
            .center_y()
    }
}


pub fn nav_button(label: &str, message: Option<Message>) -> CustomButton<'_, Message> {
    custom_button(label).padding([12, 24]).width(Length::Fixed(175.0)).on_press_maybe(message)
}


pub fn get_message_if_not_already(message: NavBarMessage, current_state: NavBarMessage) -> Option<Message> {
    if message != current_state {
        Some(Message::NavBar(message))
    } else {
        None
    }
}

