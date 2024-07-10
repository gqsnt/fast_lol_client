use iced::Command;
use iced::widget::{Column, Container, container, text};

use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;

#[derive(Debug, Clone, Default)]
pub struct SummonerInfoState {
    pub summoner_name: String,
    pub tag_line: String,
    pub summoner_level: u64,
    pub profile_icon_id: u64,
    pub number_of_rolls: u64,

}

#[derive(Debug, Clone)]
pub enum SummonerInfoMessage {}


pub struct SummonerInfoView {}

impl HasView for SummonerInfoView {
    type State = SummonerInfoState;
    type Message = SummonerInfoMessage;

    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message> {
        if let Some(_connected_state) = connected_state {
            match message {}
        } else {
            Command::none()
        }
    }
    fn view(connected_sate: &ConnectedState) -> Container<'_, Message> {
        let summoner_info = &connected_sate.summoner_info;
        container(
            Column::new()
                .push(text("Connected").size(25))
                .push(text(format!("{} #{}",&summoner_info.display_name,&summoner_info.tag_line)))
                .push(text(format!("Level: {}", &summoner_info.summoner_level)))
                .push(text(format!("Rolls: {}", &summoner_info.reroll_points.number_of_rolls)))
                .spacing(10)
        )
            .center_y()
    }
}
