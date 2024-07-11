use iced::Command;
use iced::widget::{Column, Container, container, scrollable};
use serde_json::Value;

use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailability;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueues;
use crate::client::utils::perform_save_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct TestState {
    result: String,
}

#[derive(Debug, Clone)]
pub enum TestMessage {
    SendRequest,
    DefaultRequestResult(AppResult<Value>),
    RequestResult(AppResult<LolGameFlowGetAvailability>),
    RequestQueuesResult(AppResult<LolGameQueuesGetQueues>),
}

pub struct TestView {}

impl HasView for TestView {
    type State = TestState;
    type Message = TestMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                TestMessage::SendRequest => {
                    return perform_save_request(connected_state, "queues", apis::lol_game_queues::get_queues(), |r| TestMessage::RequestQueuesResult(r).into());
                }
                TestMessage::DefaultRequestResult(r) => {
                    match r {
                        Ok(v) => {
                            connected_state.test.result = serde_json::to_string_pretty(&v).unwrap();
                        }
                        Err(e) => {
                            connected_state.test.result = format!("Error: {}", e);
                        }
                    }
                }
                TestMessage::RequestResult(r) => {
                    match r {
                        Ok(v) => {
                            connected_state.test.result = serde_json::to_string_pretty(&v).unwrap();
                        }
                        Err(e) => {
                            connected_state.test.result = format!("Error: {}", e);
                        }
                    }
                }
                TestMessage::RequestQueuesResult(r) => {
                    match r {
                        Ok(v) => {
                            connected_state.test.result = serde_json::to_string_pretty(&v).unwrap();
                        }
                        Err(e) => {
                            connected_state.test.result = format!("Error: {}", e);
                        }
                    }
                }
            }
        }
        Command::none()
    }
    fn view(connected_state: &ConnectedState) -> Container<'_, Message> {
        container(Column::new()
            .push(iced::widget::Text::new("Test View").size(25))
            .push(custom_button("Send Request")
                .on_press(TestMessage::SendRequest.into())
            )
            .push(scrollable(iced::widget::Text::new(&connected_state.test.result)))
        ).center_x()
            .center_y()
    }
}