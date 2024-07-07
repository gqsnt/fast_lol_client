use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::apis::request::ApiRequest;
use crate::client::plugins::LolApiPlugin;

pub struct LolChampSelectPatchSessionAction{
    pub id: u32,
    pub body: LolChampSelectPatchSessionActionBody,
}



impl ApiRequest for LolChampSelectPatchSessionAction{
    type ReturnType = Value;

    const PLUGIN: LolApiPlugin = LolApiPlugin::LolChampSelect;

    const METHOD: reqwest::Method = Method::PATCH;
    fn get_path(&self) -> String {
        format!("/session/actions/{}",self.id)
    }

    fn get_body(&self) -> Option<Value>{ serde_json::to_value(&self.body).ok()}

}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectPatchSessionActionBody {
    champion_id: u32,
    completed: bool,
}



