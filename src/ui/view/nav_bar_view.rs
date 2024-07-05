use iced::{Command, Length};
use iced::widget::{Column, Container, container, text};
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button::{custom_button, CustomButton};


#[derive(Debug, Clone, Default)]
pub struct NavBarState {
    pub state:NavBarMessage,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum NavBarMessage {
    #[default]
    Play,
}

pub struct NavBarView {}

impl HasView for NavBarView {
    type State = NavBarState;
    type Message = NavBarMessage;

    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message> {
        if let Some(connected_state) = connected_state {
            connected_state.nav_bar.state = message;
            Command::none()
        } else {
            Command::none()
        }
    }
     fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
         let nav_bar_state = &connected_state.nav_bar;
        container(Column::new()
            .push(nav_button("Play", Self::Message::Play, nav_bar_state.state.clone()))
            .spacing(20)
        ).center_x()
            .center_y()
    }
}


pub fn nav_button(label: &str, message: NavBarMessage, current_state: NavBarMessage) -> CustomButton<'_, Message> {
    custom_button(label).padding([12, 24]).width(Length::Fixed(175.0)).on_press_maybe(
        if message != current_state {
            Some(message.into())
        } else {
            None
        }
    )
}

