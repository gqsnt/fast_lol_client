use iced::Command;
use iced::widget::{Column, Container, container, text};
use crate::client::api;
use crate::client::api::lol_summoner::CurrentSummoner;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;


#[derive(Debug, Clone)]
pub enum ProfileMessage {}


pub struct ProfileView {}

impl HasView for ProfileView {
    type State = CurrentSummoner;
    type Message = ProfileMessage;

    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message> {
        if let Some(_connected_state) = connected_state {
            match message {}
        } else {
            Command::none()
        }
    }
    fn view(connected_sate: &ConnectedState) -> Container<'_, Message> {
        container(
            Column::new()
                .push(text(format!("{} #{}", &connected_sate.summoner_info.display_name, &connected_sate.summoner_info.tag_line)))
                .push(text(format!("Level: {}", &connected_sate.summoner_info.summoner_level)))
                .push(text(format!("Rolls: {}", &connected_sate.summoner_info.reroll_points.number_of_rolls)))
                .spacing(10)
        )
            .center_y()
    }
}
