use iced::{Application, Command};
use iced::widget::{Column, Container, container, text};
use plugin_lol_champ_select::LolChampSelectChampSelectSession;
use crate::AppResult;
use crate::assets::Assets;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct ChampSelectState {
    session: Option<LolChampSelectChampSelectSession>,
}

#[derive(Debug, Clone)]
pub enum ChampSelectMessage {
    ChampSelectSessionResult(AppResult<LolChampSelectChampSelectSession>),
    QuitCustomLobby,
}

pub struct ChampSelectView {}

impl HasView for ChampSelectView {
    type State = ChampSelectState;
    type Message = ChampSelectMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                ChampSelectMessage::ChampSelectSessionResult(result) => {
                    if let Ok(result) = result {
                        connected_state.play.champ_select_state.session = Some(result);
                    }
                }
                ChampSelectMessage::QuitCustomLobby => {
                    return perform_request(
                        connected_state,
                        plugin_lol_lobby::post_lol_lobby_v1_lobby_custom_cancel_champ_select(),
                        |r| Message::None,
                    )
                }
            }
        }
        Command::none()
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        container(Column::new()
            .push(text("Champ Select").size(25))
        ).center_x()
            .center_y()
    }
}