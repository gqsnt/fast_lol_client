use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::apis::request::ApiRequest;
use crate::client::plugins::LolApiPlugin;

pub struct LolLobbyPutLocalMemberPlayerSlots {
    pub id: u32,
    pub body: LolLobbyPutLocalMemberPlayerSlotsBody,
}




impl ApiRequest for LolLobbyPutLocalMemberPlayerSlots{
    type ReturnType = Value;
    const PLUGIN: LolApiPlugin = LolApiPlugin::LolLobby;

    const METHOD: reqwest::Method = Method::PUT;

    fn get_path(&self) -> String {
        "/lobby/members/localMember/player-slots".to_string()
    }

    fn get_body(&self) -> Option<Value>{ serde_json::to_value(&self.body).ok()}


}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPutLocalMemberPlayerSlotsBody {
    champion_id: u32,
    position_preference: String,
    spell1: u32,
}

