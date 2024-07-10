use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use iced::Command;
use iced::futures::{SinkExt, StreamExt};
use reqwest::Client;
use tokio::sync::Mutex;
use tokio_tungstenite::Connector;
use tokio_tungstenite::Connector::NativeTls;
use tungstenite::{handshake::client::Request, protocol::WebSocketConfig};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::HeaderValue;
use crate::{AppError, AppResult};
use crate::client::client_type::ClientType;
use crate::client::request::ApiRequest;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;

#[derive(Debug, Clone)]
pub struct LolClient {
    pub client: Client,
    pub port: String,
    pub client_type: ClientType,
}

impl LolClient {
    pub async fn new(riot_path: &str) -> AppResult<Self> {
        let (client_type, port, auth_token) = get_lockfile_info(riot_path)?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Basic {}", auth_token).parse().unwrap());
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client_type,
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
        Ok(response.json().await?)
    }

    pub async fn execute_and_save<S: ApiRequest>(&self, request: S, file_name: &str) -> AppResult<S::ReturnType> {
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
    Command::perform(async move { client.execute(request).await }, move |r| map_response(r).into())
}

pub fn perform_save_request<R: ApiRequest + Send + 'static>(
    connected_state: &mut ConnectedState,
    file_name: String,
    request: R,
    map_response: fn(AppResult<R::ReturnType>) -> Message,
) -> Command<Message>
{
    let client = connected_state.client.clone();
    Command::perform(async move { client.execute_and_save(request, file_name.as_str()).await }, move |r| map_response(r).into())
}


pub fn get_lockfile_info(riot_path: &str) -> AppResult<(ClientType, String, String)> {
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
        client_type,
        lockfile_parts[2].to_string(),
        BASE64_STANDARD.encode(format!("riot:{}", lockfile_parts[3].to_string()).as_str())
    ))
}