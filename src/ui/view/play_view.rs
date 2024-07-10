use iced::Command;
use iced::widget::{Container, container, Row};
use serde_json::Value;
use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailability;
use crate::client::apis::lol_game_flow::get_session::LolGameFlowGetSession;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueues;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;

#[derive(Debug, Clone, Default)]
pub struct PlayState {
    pub session:Option<LolGameFlowGetSession>,
    pub queues:LolGameQueuesGetQueues

}

#[derive(Debug, Clone)]
pub enum PlayMessage {
    RequestQueuesResult(AppResult<LolGameQueuesGetQueues>),
}

pub struct PlayView {}

impl HasView for PlayView {
    type State = PlayState;
    type Message = PlayMessage;

    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message> {
        if let Some(connected_state) = connected_state {
            match message {
                PlayMessage::RequestQueuesResult(r) => {
                    match r {
                        Ok(v) => {
                            connected_state.play.queues = v.into_iter().filter(|queue|queue.is_available && queue.is_visible).collect();
                            println!("Found {} queues", connected_state.play.queues.len());
                            for queue in &connected_state.play.queues {
                                println!("Queue: {}", if queue.detailed_description.len() == 0{ &queue.description} else {&queue.detailed_description})
                            }
                        }
                        Err(e) => {
                            connected_state.play.queues = LolGameQueuesGetQueues::default();
                        }
                    }
                    Command::none()
                }
            }
        } else {
            Command::none()
        }
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Row::new()).center_x()
            .center_y()
    }
}