use iced::Command;
use crate::{AppError, AppResult};
use crate::client::apis;
use crate::client::apis::is_api_request::IsApiRequest;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityState;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;

pub fn perform_request<R: IsApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    request: R,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute(request).await },
        move |r|disconnect_if_disconnect_error::<R::ReturnType>(r, map_response)
    )
}

pub fn perform_request_with_delay<R: IsApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    request: R,
    delay_ms:u64,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute_with_delay(request,delay_ms).await },
        move |r|disconnect_if_disconnect_error::<R::ReturnType>(r, map_response)
    )
}

pub fn perform_save_request<R: IsApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    file_name: &'static str,
    request: R,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute_and_save(request,file_name).await },
        move |r|disconnect_if_disconnect_error::<R::ReturnType>(r, map_response)
    )
}



pub fn perform_game_flow_state_update(
    connected_state: &mut ConnectedState
) -> Command<Message>
{
    let client = connected_state.client.clone();
    let current_state=  connected_state.state.clone();
    Command::perform(
        async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            client.execute(apis::lol_game_flow::get_phase()).await
        },
        move |r|disconnect_if_disconnect_error::<LolGameFlowPhase>(r, Message::ClientStateUpdated)
    )
}

pub(crate) fn disconnect_if_disconnect_error<T>(result:AppResult<T>, map_response: fn(AppResult<T>) -> Message) -> Message {
    match result {
        Ok(v) => map_response(Ok(v)),
        Err(e) => match e {
            AppError::DisconnectedError(e) => {
                Message::Disconnected
            }
            _ => { map_response(Err(e)) }
        }
    }
}