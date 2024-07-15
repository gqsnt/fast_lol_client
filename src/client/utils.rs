use iced::Command;

use crate::{AppError, AppResult};
use crate::client::apis;
use crate::client::apis::is_api_request::IsApiRequest;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_lobby::post_lobby::LolLobbySession;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::chat_view::ChatMessage;
use crate::ui::view::play_view::champ_select_view::ChampSelectMessage;
use crate::ui::view::play_view::lobby_view::LobbyMessage;

pub fn perform_request<R: IsApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    request: R,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute(request).await },
        move |r| disconnect_if_disconnect_error::<R::ReturnType>(r, map_response),
    )
}

pub fn perform_request_with_delay<R: IsApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    request: R,
    delay_ms: u64,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute_with_delay(request, delay_ms).await },
        move |r| disconnect_if_disconnect_error::<R::ReturnType>(r, map_response),
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
        async move { client.execute_and_save(request, file_name).await },
        move |r| disconnect_if_disconnect_error::<R::ReturnType>(r, map_response),
    )
}

pub fn perform_save_request_with_delay<R: IsApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    file_name: &'static str,
    request: R,
    delay_ms: u64,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            client.execute_and_save(request, file_name).await
        },
        move |r| disconnect_if_disconnect_error::<R::ReturnType>(r, map_response),
    )
}


pub fn perform_game_flow_update(
    connected_state: &mut ConnectedState
) -> Command<Message>
{
    let current_state = connected_state.state.clone();
    println!("perform_states_update: {:?}", current_state);
    let mut all_commands = vec![
        perform_request_with_delay(connected_state, apis::lol_game_flow::get_phase(), 500, |r| Message::GamFlowResult(r)),
        perform_request(connected_state,apis::lol_chat::get_conversations(),|r|ChatMessage::ConversationsResult(r).into())
    ];
    match current_state {
        LolGameFlowPhase::Lobby
        | LolGameFlowPhase::Matchmaking
        | LolGameFlowPhase::ReadyCheck => {
            all_commands.push(
                perform_save_request(connected_state, "lol_lobby",apis::lol_lobby::get_lobby(), |r| LobbyMessage::LobbySessionResult(r).into())
            );
            if matches!( current_state,LolGameFlowPhase::Matchmaking| LolGameFlowPhase::ReadyCheck ) {
                all_commands.push(
                    perform_request(connected_state, apis::lol_matchmaking::get_search(), |r| LobbyMessage::MatchmakingSearchResult(r).into())
                );
            } else {
                connected_state.play.lobby_state.matchmaking_session = None;
            }
        }
        LolGameFlowPhase::ChampSelect => {
            all_commands.push(
                perform_request(connected_state, apis::lol_champ_select::get_session(), |r| ChampSelectMessage::ChampSelectSessionResult(r).into())
            );
        }
        LolGameFlowPhase::GameStart
        | LolGameFlowPhase::CheckedIntoTournament
        | LolGameFlowPhase::FailedToLaunch
        | LolGameFlowPhase::InProgress
        | LolGameFlowPhase::Reconnect
        | LolGameFlowPhase::WaitingForStats
        | LolGameFlowPhase::PreEndOfGame
        | LolGameFlowPhase::EndOfGame
        | LolGameFlowPhase::TerminatedInError
        | LolGameFlowPhase::None => {}
    }
    Command::batch(all_commands)
}


pub fn perform_message_command(message: Message) -> Command<Message> {
    Command::perform(async move {}, |r| message)
}



pub(crate) fn disconnect_if_disconnect_error<T>(result: AppResult<T>, map_response: fn(AppResult<T>) -> Message) -> Message {
    match result {
        Ok(v) => map_response(Ok(v)),
        Err(e) => match e {
            AppError::DisconnectedError(_) => Message::Disconnected,
            _ => { map_response(Err(e)) }
        }
    }
}

