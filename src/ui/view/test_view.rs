use iced::Command;
use iced::widget::{Column, Container, container, scrollable};
use serde_json::Value;

use crate::AppResult;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityResponse;
use crate::client::apis::lol_game_flow::LolGameFlow;
use crate::client::client::perform_request;
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
    RequestResult(AppResult<LolGameFlowGetAvailabilityResponse>),
}

pub struct TestView {}

impl HasView for TestView {
    type State = TestState;
    type Message = TestMessage;

    fn update(message: Self::Message, connected_state: &mut Option<ConnectedState>) -> Command<Message> {
        if let Some(connected_state) = connected_state {
            match message {
                TestMessage::SendRequest => {
                    perform_request(connected_state, LolGameFlow::get_availability(), |r| TestMessage::RequestResult(r).into())
                    //perform_request(connected_state, LolGameFlowGetAvailability::new(), |r| TestMessage::RequestResult(r).into())
                }
                TestMessage::RequestResult(r) => {
                    match r {
                        Ok(v) => {
                            connected_state.test_state.result = serde_json::to_string_pretty(&v).unwrap();
                        }
                        Err(e) => {
                            connected_state.test_state.result = format!("Error: {}", e);
                        }
                    }
                    Command::none()
                }
                TestMessage::DefaultRequestResult(r) => {
                    match r {
                        Ok(v) => {
                            connected_state.test_state.result = serde_json::to_string_pretty(&v).unwrap();
                        }
                        Err(e) => {
                            connected_state.test_state.result = format!("Error: {}", e);
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
        container(Column::new()
            .push(iced::widget::Text::new("Test View").size(25))
            .push(custom_button("Send Request")
                .on_press(TestMessage::SendRequest.into())
            )
            .push(scrollable(iced::widget::Text::new(&connected_state.test_state.result)))
        ).center_x()
            .center_y()
    }
}