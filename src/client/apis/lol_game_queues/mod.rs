use serde_json::Value;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueues;
use crate::impl_api_plugin;

pub mod get_queues;


impl_api_plugin!(
    LolApiPlugin::LolGameQueues,
    GetQueues{
        get_queues,reqwest::Method::GET,"/queues" => LolGameQueuesGetQueues,
    },
);