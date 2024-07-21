use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use chrono::TimeZone;
use iced::futures::{SinkExt, StreamExt, TryFutureExt};
use reqwest::Client;
use serde_json::Value;
use common::IsApiRequest;

use crate::{AppError, AppResult};
use crate::client::client_type::ClientType;

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


    pub async fn execute<S: IsApiRequest>(&self, request: S) -> AppResult<S::ReturnType> {
        let mut request_builder = self.client.request(S::METHOD, self.build_url(&request.get_url()));
        if let Some(body) = request.get_body() {
            request_builder = request_builder.json(&body)
        }
        if let Some(query) = request.get_query_params() {
            request_builder = request_builder.query(&query)
        }
        let response = request_builder
            .send()
            .await
            .map_err(|e| AppError::DisconnectedError(e.to_string()))?;

        if !response.status().is_success(){
            return Err(AppError::ApiRequestError(format!("API request failed with status: {:?}", response.text().await?)));
        }

        let mut response_text = response.text().await?;
        if response_text.is_empty() {
            response_text = "{}".to_string();
        }
        serde_json::from_str::<S::ReturnType>(&response_text).or_else(|e| {
            println!("Error parsing response: {:?}", e);
            println!("Response: {:?}", response_text);
            Err(AppError::ParsingError(e.to_string()))
        })
    }

    pub async fn execute_with_delay<S: IsApiRequest>(&self, request: S, delay_ms: u64) -> AppResult<S::ReturnType> {
        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
        self.execute(request).await
    }

    pub async fn get_help(&self) -> AppResult<Value> {
        self.client.post(self.build_url("/Help?format=Full")).send().await.map_err(|e| AppError::DisconnectedError(e.to_string()))?.json().await.map_err(|e| AppError::ParsingError(e.to_string()))
    }


    pub async fn execute_and_save<S: IsApiRequest>(&self, request: S, file_name: &str) -> AppResult<S::ReturnType> {
        let response = self.execute(request).await;
        let timestamp = chrono::Utc::now().timestamp_millis();
        let file_path = PathBuf::from("temp").join(format!("{}_{}.json", file_name, timestamp));
        let path = std::fs::File::create(&file_path)?;
        match &response {
            Ok(response) => {
                serde_json::to_writer_pretty(&path, &response).unwrap();
            }
            Err(er) => {
                let err = Value::String(er.to_string());
                serde_json::to_writer_pretty(&path, &err).unwrap();
            }
        }
        response
    }
}


pub fn get_lockfile_info(riot_path: &str) -> AppResult<(String, String)> {
    let lockfile_path = [ClientType::Live, ClientType::Pbe]
        .iter()
        .find_map(|client_type| client_type.get_lock_file_path(riot_path).exists().then(|| client_type.get_lock_file_path(riot_path)))
        .ok_or_else(|| AppError::IoError("lockfile not found".to_string()))?;
    let lockfile_content = std::fs::read_to_string(lockfile_path)?;
    let parts: Vec<&str> = lockfile_content.split(':').collect();
    Ok((
        parts[2].to_string(),
        BASE64_STANDARD.encode(format!("riot:{}", parts[3]).as_str())
    ))
}