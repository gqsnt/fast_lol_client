use iced::Command;
use iced::widget::{Column, combo_box, Container, container};
use serde_json::Value;
use crate::AppResult;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_game_flow::get_session::LolGameFlowGetSession;
use crate::client::apis::lol_game_queues::get_queues::{LolGameQueuesGetQueue, LolGameQueuesGetQueues};
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::play_view::champ_select_view::{ChampSelectMessage, ChampSelectState, ChampSelectView};
use crate::ui::view::play_view::create_lobby_view::{CreateLobbyMessage, CreateLobbyState, CreateLobbyView};
use crate::ui::view::play_view::in_game_view::{InGameMessage, InGameState, InGameView};
use crate::ui::view::play_view::lobby_view::{LobbyMessage, LobbyState, LobbyView};
use crate::ui::view::play_view::post_game_view::{PostGameMessage, PostGameState, PostGameView};

mod create_lobby_view;
mod lobby_view;
mod post_game_view;
mod champ_select_view;

mod in_game_view;

#[derive(Debug, Clone)]
pub struct PlayState {
    pub session: Option<LolGameFlowGetSession>,
    pub queues: combo_box::State<LolGameQueuesGetQueue>,
    pub champ_select_state: Option<ChampSelectState>,
    pub create_lobby_state: CreateLobbyState,
    pub lobby_state: Option<LobbyState>,
    pub post_game_state: Option<PostGameState>,
    pub in_game_state: Option<InGameState>,
}

impl Default for PlayState{
    fn default() -> Self {
        Self {
            session: None,
            queues: combo_box::State::new(vec![]),
            champ_select_state: None,
            create_lobby_state: CreateLobbyState::default(),
            lobby_state: None,
            post_game_state: None,
            in_game_state: None,
        }
    }
}


#[derive(Debug, Clone)]
pub enum PlayMessage {
    RequestQueuesResult(AppResult<LolGameQueuesGetQueues>),
    ChampSelect(ChampSelectMessage),
    CreateLobby(CreateLobbyMessage),
    Lobby(LobbyMessage),
    PostGame(PostGameMessage),
    InGame(InGameMessage),
    StartQueueResult(AppResult<Value>),
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
            PlayMessage::PostGame(message) => PostGameView::update(message, state),
            PlayMessage::InGame(message) => InGameView::update(message, state),
            PlayMessage::StartQueueResult(r) => {
                println!("StartQueueResult: {:?}", r);
                Command::none()
            }
        }
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(match connected_state.state {
                LolGameFlowPhase::None => CreateLobbyView::view(connected_state),
                LolGameFlowPhase::Lobby => LobbyView::view(connected_state),
                LolGameFlowPhase::Matchmaking => LobbyView::view(connected_state),
                LolGameFlowPhase::ReadyCheck => LobbyView::view(connected_state),
                LolGameFlowPhase::ChampSelect => ChampSelectView::view(connected_state),
                LolGameFlowPhase::InProgress => InGameView::view(connected_state),
                LolGameFlowPhase::WaitingForStats => InGameView::view(connected_state),
                LolGameFlowPhase::PostGame => PostGameView::view(connected_state),
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
impl_from_for_play_message!(PostGameMessage => PostGame);
impl_from_for_play_message!(InGameMessage => InGame);