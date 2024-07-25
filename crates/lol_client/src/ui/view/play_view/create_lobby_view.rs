use std::borrow::Borrow;
use std::fmt;

use iced::Command;
use iced::widget::{Column, Container, container, pick_list, Row, text};
use crate::assets::Assets;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::play_view::create_lobby_view::create_lobby_coop_ai_view::{CreateLobbyCoopAiMessage, CreateLobbyCoopAiView};
use crate::ui::view::play_view::create_lobby_view::create_lobby_custom_view::{CreateLobbyCustomMessage, CreateLobbyCustomView};
use crate::ui::view::play_view::create_lobby_view::create_lobby_join_custom_view::{CreateLobbyJoinCustomMessage, CreateLobbyJoinCustomView};
use crate::ui::view::play_view::create_lobby_view::create_lobby_pvp_view::{CreateLobbyPvpMessage, CreateLobbyPvpView};
use crate::ui::view::play_view::create_lobby_view::create_lobby_training_view::{CreateLobbyTrainingMessage, CreateLobbyTrainingView};
use crate::ui::view::play_view::lobby_view::{LobbyMessage, LobbyState};
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

pub mod create_lobby_pvp_view;
pub mod create_lobby_coop_ai_view;
pub mod create_lobby_custom_view;
pub mod create_lobby_join_custom_view;
pub mod create_lobby_training_view;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CreateLobbyViewType {
    #[default]
    Pvp,
    CoopVsAi,
    Training,
    Custom,
    JoinCustom,
}


impl CreateLobbyViewType {
    pub fn cases() -> Vec<Self> {
        vec![Self::Pvp, Self::CoopVsAi,Self::Training, Self::Custom, Self::JoinCustom]
    }
}


impl fmt::Display for CreateLobbyViewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pvp => write!(f, "Pvp"),
            Self::CoopVsAi => write!(f, "CoopVsAi"),
            Self::Custom => write!(f, "Custom"),
            Self::JoinCustom => write!(f, "JoinCustom"),
            Self::Training => write!(f, "Training"),
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct CreateLobbyState {
    view_type: CreateLobbyViewType,
    pvp_state: create_lobby_pvp_view::CreateLobbyPvpState,
    coop_ai_state: create_lobby_coop_ai_view::CreateLobbyCoopAiState,
    training_state: create_lobby_training_view::CreateLobbyTrainingState,
    custom_state: create_lobby_custom_view::CreateLobbyCustomState,
    join_custom_state: create_lobby_join_custom_view::CreateLobbyJoinCustomState,
}

#[derive(Debug, Clone)]
pub enum CreateLobbyMessage {
    SelectedViewType(CreateLobbyViewType),
    DeleteLobbyResult,
    Pvp(CreateLobbyPvpMessage),
    CoopAi(CreateLobbyCoopAiMessage),
    Training(CreateLobbyTrainingMessage),
    Custom(CreateLobbyCustomMessage),
    JoinCustom(CreateLobbyJoinCustomMessage)
}

pub struct CreateLobbyView {}

impl HasView for CreateLobbyView {
    type State = CreateLobbyState;
    type Message = CreateLobbyMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        return match message{
            CreateLobbyMessage::SelectedViewType(selected_type) => {
                if let AppState::Connected(connected_state) = state {
                    connected_state.play.create_lobby_state.view_type = selected_type;
                }
                Command::none()
            }
            CreateLobbyMessage::DeleteLobbyResult => {
                if let AppState::Connected(connected_state) = state {
                    connected_state.play.lobby_state = LobbyState::default();
                }
                Command::none()
            }
            CreateLobbyMessage::Pvp(message) => CreateLobbyPvpView::update(message, state),
            CreateLobbyMessage::CoopAi(message) => CreateLobbyCoopAiView::update(message, state),
            CreateLobbyMessage::Training(message) => CreateLobbyTrainingView::update(message, state),
            CreateLobbyMessage::Custom(message) => CreateLobbyCustomView::update(message, state),
            CreateLobbyMessage::JoinCustom(message) => CreateLobbyJoinCustomView::update(message, state),
        };
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        container(Column::new()
            .push(text("Create Lobby").size(25))
            .push(
                pick_list(
                    CreateLobbyViewType::cases(),
                    Some(connected_state.play.create_lobby_state.view_type.clone()),
                    |r| CreateLobbyMessage::SelectedViewType(r).into()
                )
            )
            .push(match connected_state.play.create_lobby_state.view_type {
                CreateLobbyViewType::Pvp => CreateLobbyPvpView::view(connected_state, assets),
                CreateLobbyViewType::CoopVsAi => CreateLobbyCoopAiView::view(connected_state,assets ),
                CreateLobbyViewType::Training => CreateLobbyTrainingView::view(connected_state, assets),
                CreateLobbyViewType::Custom => CreateLobbyCustomView::view(connected_state, assets),
                CreateLobbyViewType::JoinCustom => CreateLobbyJoinCustomView::view(connected_state, assets),
            })
            .spacing(20)
        )
            .center_x()
            .center_y()
    }
}


macro_rules! impl_from_for_create_lobby_message {
    ($source_type:ty => $enum_variant:ident) => {
        impl From<$source_type> for Message {
            fn from(value: $source_type) -> Self {
                CreateLobbyMessage::$enum_variant(value).into()
            }
        }
    };
}

impl_from_for_create_lobby_message!(CreateLobbyPvpMessage => Pvp);
impl_from_for_create_lobby_message!(CreateLobbyCoopAiMessage => CoopAi);
impl_from_for_create_lobby_message!(CreateLobbyTrainingMessage => Training);
impl_from_for_create_lobby_message!(CreateLobbyCustomMessage => Custom);
impl_from_for_create_lobby_message!(CreateLobbyJoinCustomMessage => JoinCustom);
