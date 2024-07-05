use std::path::PathBuf;

use base64::{alphabet, Engine, engine};
use base64::prelude::BASE64_STANDARD;
use reqwest::Client;
use serde_json::Value;
use crate::{AppError, AppResult};
use crate::ui::view::summoner_info_view::SummonerInfoState;

#[derive(Clone, Default, Debug)]
pub enum ClientType{
    #[default]
    Live,
    Pbe,
}

impl ClientType{
    pub fn get_lock_file_path(&self, riot_path:&str) -> PathBuf {
        match self {
            ClientType::Live => PathBuf::from(format!("{}\\League of Legends\\lockfile", riot_path)),
            ClientType::Pbe => PathBuf::from(format!("{}\\League of Legends (PBE)\\lockfile", riot_path)),
        }
    }

}


#[derive(Clone, Default, Debug)]
pub struct LolClient {
    pub client: Client,
    pub port:String,
    pub client_type :ClientType,
}


impl LolClient {
    pub fn new(riot_path:&str) -> AppResult<Self> {
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
            port:port.to_string(),
            client,
        })
    }

    pub fn build_url(&self, endpoint:&str) -> String {
        format!("https://127.0.0.1:{}{}", &self.port, endpoint)
    }


    pub async fn get_summoner(&self) -> AppResult<SummonerInfoState> {
        let url = self.build_url("/lol-summoner/v1/current-summoner");

        let response = self.client.get(url).send().await?;
        let json:Value = response.json().await?;
        let profile_icon_id:u64 = json["profileIconId"].as_u64().unwrap();
        let summoner_level:u64 = json["summonerLevel"].as_u64().unwrap();
        let summoner_name:String = json["displayName"].as_str().unwrap().to_string();
        let tag_line :String = json["tagLine"].as_str().unwrap().to_string();
        println!("{:?}",json);
        let number_of_rolls :u64= json["rerollPoints"]["numberOfRolls"].as_u64().unwrap();
        Ok(SummonerInfoState{
            summoner_name,
            tag_line,
            summoner_level,
            profile_icon_id,
            number_of_rolls,
        })
    }

    pub async fn get_game_flow_session(&self) -> AppResult<serde_json::Value> {
        let url = self.build_url("/lol-gameflow/v1/session");
        println!("game_flow_session:{:?}",url);
        let response = self.client.get(url).send().await?;
        let json = response.json().await?;
        println!("{:?}",json);
        Ok(json)
    }
}