use iced::Command;
use iced::widget::{Column, Container, container, text};
use crate::client::apis;
use crate::client::apis::lol_summoner::CurrentSummoner;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;


#[derive(Debug, Clone)]
pub enum ProfileMessage {}


pub struct ProfileView {}

impl HasView for ProfileView {
    type State = CurrentSummoner;
    type Message = ProfileMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {}
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(
            Column::new()
                .push(text(format!("{} #{}", &connected_state.summoner_info.display_name, &connected_state.summoner_info.tag_line)))
                .push(text(format!("Level: {}", &connected_state.summoner_info.summoner_level)))
                .push(text(format!("Rolls: {}", &connected_state.summoner_info.reroll_points.number_of_rolls)))
                .spacing(10)
        )
            .center_y()
    }
}
