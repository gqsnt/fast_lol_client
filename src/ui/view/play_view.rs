use std::cmp::PartialEq;
use std::collections::HashMap;
use iced::Command;
use iced::widget::{Column, combo_box, Container, container, text};
use serde::{Deserialize, Serialize};
use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_game_queues::get_custom_queues::{LolGameQueuesQueueAvailability, LolGameQueuesQueueCustomGame, LolGameQueuesQueueCustomGameSubcategory};
use crate::client::apis::lol_game_queues::get_queues::{LolGameMode, LolGameQueuesGetQueue, LolGameQueuesGetQueues, QueueCategory};
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
    pub pvp_queues:HashMap<LolGameMode, Vec<LolGameQueuesGetQueue>>,
    pub coop_vs_ai_queues:Vec<LolGameQueuesGetQueue>,
    pub custom_queues: Vec<LolGameQueuesQueueCustomGameSubcategory>,
    pub practice_tool_queue: Option<LolGameQueuesQueueCustomGameSubcategory>,
    pub champ_select_state: ChampSelectState,
    pub create_lobby_state: CreateLobbyState,
    pub lobby_state: LobbyState,
    pub post_game_state: EndOfGameState,
    pub in_game_state: InGameState,
}



impl PlayState{
    pub fn new(
        queues: LolGameQueuesGetQueues,
        custom_queues: LolGameQueuesQueueCustomGame,
        custom_non_default_queues: LolGameQueuesQueueCustomGame,
    ) -> Self {
        let mut pvp_queues_result:HashMap<LolGameMode, Vec<LolGameQueuesGetQueue>> = HashMap::new();
        let mut custom_queues_result:Vec<LolGameQueuesQueueCustomGameSubcategory> = Vec::new();
        let mut coop_vs_ai_queues_result:Vec<LolGameQueuesGetQueue> = Vec::new();
        let mut practice_tool_queue = None;

        for queue in queues {
            if queue.is_available && queue.is_visible{
                match &queue.category{
                    QueueCategory::VersusAi => {
                        coop_vs_ai_queues_result.push(queue.into());
                    }
                    QueueCategory::PvP => {
                        pvp_queues_result.entry(queue.game_mode.clone()).or_insert_with(Vec::new).push(queue.into());
                    }
                    _ => {}
                }
            }
        }
        if custom_queues.queue_availability == LolGameQueuesQueueAvailability::Available{
            custom_queues_result =  custom_queues.subcategories.into_iter().map(|subcategory| subcategory.into()).collect();
        }
        if custom_non_default_queues.queue_availability == LolGameQueuesQueueAvailability::Available{
            let practice_tool = custom_non_default_queues.subcategories.iter().find(|subcategory| subcategory.game_mode == LolGameMode::OTHER("PRACTICETOOL".to_string()));
            if let Some(practice_tool) = practice_tool{
                practice_tool_queue = Some(practice_tool.clone());
            }
        }
        Self {
            pvp_queues: pvp_queues_result,
            coop_vs_ai_queues: coop_vs_ai_queues_result,
            custom_queues: custom_queues_result,
            practice_tool_queue,
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


impl_from_for_play_message!(ChampSelectMessage => ChampSelect);
impl_from_for_play_message!(CreateLobbyMessage => CreateLobby);
impl_from_for_play_message!(LobbyMessage => Lobby);
impl_from_for_play_message!(EndOfGameMessage => PostGame);
impl_from_for_play_message!(InGameMessage => InGame);