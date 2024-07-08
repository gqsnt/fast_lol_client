use iced::Command;
use iced::widget::Container;

use crate::ui::message::Message;
use crate::ui::state::ConnectedState;

pub mod summoner_info_view;
pub mod nav_bar_view;
pub mod play_view;
pub mod test_view;

pub trait HasView {
    type State;
    type Message;
    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message>;
    fn view(connected_sate: &ConnectedState) -> Container<'_, Message>;
}