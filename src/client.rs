use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use crate::{AppError, AppResult};
use crate::client::apis::lol_summoner::current_summoner::{LolSummonerGetCurrentSummoner, SummonerInfo};

use crate::client::apis::request::{ApiRequest};
use crate::client::client_type::ClientType;
use crate::utils::save_json_to_file;
pub mod apis;
mod client_type;
mod plugins;

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



    pub async fn execute< S:ApiRequest>(&self, request: S) -> AppResult<S::ReturnType> {
        let builder = self.client.request(S::METHOD, self.build_url(&request.build_url()));
        let response = if let Some(body) = request.get_body() {
            builder.json(&body)
        }else{
            builder
        }.send().await?;
        Ok(response.json().await?)
    }


    pub async fn get_game_flow_session(&self) -> AppResult<serde_json::Value> {
        let url = self.build_url("/lol-gameflow/v1/session");
        println!("game_flow_session:{:?}", url);
        let response = self.client.get(url).send().await?;
        let json = response.json().await?;
        save_json_to_file("game_flow_session.json", &json);
        println!("{:?}", json);
        Ok(json)
    }
}