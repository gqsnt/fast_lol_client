use iced::Command;
use iced::widget::Container;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;

pub mod profile_view;
pub mod nav_bar_view;
pub mod play_view;
pub mod test_view;
pub mod chat_view;

pub trait HasView {
    type State;
    type Message;
    fn update(message: Self::Message, state: &mut AppState) -> Command<Message>;
    fn view(connected_state: &ConnectedState) -> Container<'_, Message>;
}