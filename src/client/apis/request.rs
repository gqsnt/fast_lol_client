use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::client::plugins::LolApiPlugin;

pub trait ApiRequest {

    type ReturnType : DeserializeOwned;

    const PLUGIN: LolApiPlugin;
    const METHOD:reqwest::Method;

    fn get_path(&self) -> String;

    fn get_body(&self) -> Option<Value> {
        None
    }



    fn build_url(&self) -> String {
        format!("{}{}", Self::PLUGIN.get_path(), self.get_path())
    }

}




