use iced::Command;
use iced::widget::{Column, combo_box, Container, container, text};

use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_game_queues::get_queues::{LolGameQueuesGetQueue, LolGameQueuesGetQueues};
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::play_view::champ_select_view::{ChampSelectMessage, ChampSelectState, ChampSelectView};
use crate::ui::view::play_view::create_lobby_view::{CreateLobbyMessage, CreateLobbyState, CreateLobbyView};
use crate::ui::view::play_view::end_of_game_view::{EndOfGameMessage, EndOfGameState, EndOfGameView};
use crate::ui::view::play_view::in_game_view::{InGameMessage, InGameState, InGameView};
use crate::ui::view::play_view::lobby_view::{LobbyMessage, LobbyState, LobbyView};
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

pub mod create_lobby_view;
pub mod lobby_view;
pub mod end_of_game_view;
pub mod champ_select_view;

pub mod in_game_view;

#[derive(Debug, Clone)]
pub struct PlayState {
    pub queues: combo_box::State<LolGameQueuesGetQueue>,
    pub champ_select_state: ChampSelectState,
    pub create_lobby_state: CreateLobbyState,
    pub lobby_state: LobbyState,
    pub post_game_state: EndOfGameState,
    pub in_game_state: InGameState,
}

impl Default for PlayState {
    fn default() -> Self {
        Self {
            queues: combo_box::State::new(vec![]),
            champ_select_state: ChampSelectState::default(),
            create_lobby_state: CreateLobbyState::default(),
            lobby_state: LobbyState::default(),
            post_game_state: EndOfGameState::default(),
            in_game_state: InGameState::default(),
        }
    }
}


#[derive(Debug, Clone)]
pub enum PlayMessage {
    RequestQueuesResult(AppResult<LolGameQueuesGetQueues>),
    ChampSelect(ChampSelectMessage),
    CreateLobby(CreateLobbyMessage),
    Lobby(LobbyMessage),
    PostGame(EndOfGameMessage),
    InGame(InGameMessage),

}


pub struct PlayView {}

impl HasView for PlayView {
    type State = PlayState;
    type Message = PlayMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        match message {
            PlayMessage::RequestQueuesResult(r) => {
                if let AppState::Connected(connected_state) = state {
                    match r {
                        Ok(v) => {
                            connected_state.play.queues = combo_box::State::new(v.into_iter().filter(|queue| queue.is_available && queue.is_visible).collect());
                        }
                        Err(e) => {
                            connected_state.play.queues = combo_box::State::new(vec![]);
                        }
                    }
                }
                Command::none()
            }
            PlayMessage::ChampSelect(message) => ChampSelectView::update(message, state),
            PlayMessage::CreateLobby(message) => CreateLobbyView::update(message, state),
            PlayMessage::Lobby(message) => LobbyView::update(message, state),
            PlayMessage::PostGame(message) => EndOfGameView::update(message, state),
            PlayMessage::InGame(message) => InGameView::update(message, state),
        }
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(match connected_state.state {
                LolGameFlowPhase::None => CreateLobbyView::view(connected_state),
                LolGameFlowPhase::Lobby
                | LolGameFlowPhase::Matchmaking
                | LolGameFlowPhase::ReadyCheck => LobbyView::view(connected_state),
                LolGameFlowPhase::ChampSelect => ChampSelectView::view(connected_state),
                LolGameFlowPhase::GameStart => container(Column::new().push(text("Game Starting"))),
                LolGameFlowPhase::Reconnect
                | LolGameFlowPhase::InProgress => InGameView::view(connected_state),
                LolGameFlowPhase::WaitingForStats
                |LolGameFlowPhase::PreEndOfGame
                |LolGameFlowPhase::EndOfGame => EndOfGameView::view(connected_state),
                LolGameFlowPhase::FailedToLaunch
                | LolGameFlowPhase::CheckedIntoTournament
                | LolGameFlowPhase::TerminatedInError => container(Column::new().push(text("Not Implemented/Error"))),
            })
        )
            .center_x()
            .center_y()
    }
}


macro_rules! impl_from_for_play_message {
    ($source_type:ty => $enum_variant:ident) => {
        impl From<$source_type> for Message {
            fn from(value: $source_type) -> Self {
                Message::Play(PlayMessage::$enum_variant(value))
            }
        }
    };
}

impl_from_for_play_message!(AppResult<LolGameQueuesGetQueues> => RequestQueuesResult);
impl_from_for_play_message!(ChampSelectMessage => ChampSelect);
impl_from_for_play_message!(CreateLobbyMessage => CreateLobby);
impl_from_for_play_message!(LobbyMessage => Lobby);
impl_from_for_play_message!(EndOfGameMessage => PostGame);
impl_from_for_play_message!(InGameMessage => InGame);