use std::path::PathBuf;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{AppError, AppResult};
use request::ApiRequest;
use crate::client::client_type::ClientType;
pub mod apis;
mod client_type;
mod plugin;
pub mod request;
mod query;

#[derive(Clone, Default, Debug)]
pub struct LolClient {
    pub client: Client,
    pub port: String,
    pub client_type: ClientType,
}


impl LolClient {
    pub fn new(riot_path: &str) -> AppResult<Self> {
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
        let (port, password) = (lockfile_parts[2].to_string(), lockfile_parts[3]);

        let mut headers = reqwest::header::HeaderMap::new();
        let auth = BASE64_STANDARD.encode(format!("riot:{}", password).as_str());
        headers.insert("Authorization", format!("Basic {}", auth).parse().unwrap());

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
        let builder = self.client.request(S::METHOD, self.build_url(&request.build_url()));
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