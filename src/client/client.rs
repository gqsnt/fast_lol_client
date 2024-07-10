use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use iced::Command;
use iced::futures::{SinkExt, StreamExt};
use reqwest::Client;
use reqwest::header::HeaderName;
use crate::{AppError, AppResult};
use crate::client::apis::{API};
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityState;
use crate::client::client_type::ClientType;
use crate::client::apis::request::ApiRequest;
use crate::ui::message::Message;
use crate::ui::state::{ClientGameFlowState, ClientState, ConnectedState};

#[derive(Debug, Clone, Default)]
pub struct LolClient {
    pub client: Client,
    pub port: String,
}

impl LolClient {
    pub async fn new(riot_path: &str) -> AppResult<Self> {
        let (port, auth_token) = get_lockfile_info(riot_path)?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Basic {}", auth_token).parse().unwrap());
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            port: port.to_string(),
            client,
        })
    }

    pub fn build_url(&self, endpoint: &str) -> String {
        format!("https://127.0.0.1:{}{}", &self.port, endpoint)
    }






    pub async fn execute<S: ApiRequest>(&self, request: S) -> AppResult<S::ReturnType> {
        let builder = self.client.request(S::METHOD, self.build_url(&request.get_url()));
        let response = if let Some(body) = request.get_body() {
            builder.json(&body)
        } else {
            builder
        }.send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(AppError::ApiRequestError(format!("API request failed with status: {:?}", response)))
        }
    }

    pub async fn execute_with_delay<S: ApiRequest>(&self, request: S, delay_ms:u64) -> AppResult<S::ReturnType> {
        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
        self.execute(request).await
    }


    pub async fn execute_and_save<S: ApiRequest>(&self, request: S,file_name: &str) -> AppResult<S::ReturnType> {
        let response = self.execute(request).await?;
        serde_json::to_writer_pretty(&std::fs::File::create(PathBuf::from("temp").join(format!("{}.json", file_name)))?, &response).unwrap();
        Ok(response)
    }
}

pub fn perform_request<R: ApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    request: R,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute(request).await },
        move |r|disconnect_if_reqwest_error::<R::ReturnType>(r, map_response)
    )
}

pub fn perform_request_with_delay<R: ApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    request: R,
    delay_ms:u64,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute_with_delay(request,delay_ms).await },
        move |r|disconnect_if_reqwest_error::<R::ReturnType>(r, map_response)
    )
}

pub fn perform_save_request<R: ApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    file_name: String,
    request: R,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move { client.execute_and_save(request,file_name.as_str()).await },
        move |r|disconnect_if_reqwest_error::<R::ReturnType>(r, map_response)
    )
}



pub fn perform_state_update(
    connected_state: &mut ConnectedState,
    delay_ms:Option<u64>,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(
        async move {
            if let Some(delay_ms) = delay_ms {
                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            }
            let state = client.execute(API::lol_game_flow().get_availability()).await?;
            Ok(match (state.state, state.is_available) {
                (_, false)|  (LolGameFlowGetAvailabilityState::EligibilityInfoMissing, true) => {
                    ClientState::NotAvailable
                }
                (LolGameFlowGetAvailabilityState::Available, true) => {
                    ClientState::Available
                }
                (LolGameFlowGetAvailabilityState::InGameFlow, true) => {
                    let game_flow = client.execute_and_save(API::lol_game_flow().get_session(), "game_flow_session").await?;

                    ClientState::InGameFlow(ClientGameFlowState::default())
                }

            })
        },
        move |r|disconnect_if_reqwest_error::<ClientState>(r, Message::ClientStateUpdated)
    )
}

fn disconnect_if_reqwest_error<T>(result:AppResult<T>, map_response: fn(AppResult<T>) -> Message) -> Message {
    match result {
        Ok(v) => map_response(Ok(v)),
        Err(e) => match e {
            AppError::ReqwestError(_) => {
                Message::Disconnected
            }
            _ => { map_response(Err(e)) }
        }
    }
}


pub fn get_lockfile_info(riot_path: &str) -> AppResult<(String, String)> {
    let live_lockfile_path = ClientType::Live.get_lock_file_path(riot_path);
    let pbe_lockfile_path = ClientType::Pbe.get_lock_file_path(riot_path);
    let (lockfile_path, client_type) = if live_lockfile_path.exists() {
        (live_lockfile_path, ClientType::Live)
    } else if pbe_lockfile_path.exists() {
        (pbe_lockfile_path, ClientType::Pbe)
    } else {
        return Err(AppError::IoError("lockfile not found".to_string()));
    };
    let lockfile = std::fs::read_to_string(lockfile_path)?;
    let lockfile_parts: Vec<&str> = lockfile.split(':').collect();
    Ok((
        lockfile_parts[2].to_string(),
        BASE64_STANDARD.encode(format!("riot:{}", lockfile_parts[3].to_string()).as_str())
    ))
}