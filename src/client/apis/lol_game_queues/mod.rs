use serde_json::Value;
use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueues;
use crate::impl_api_plugin;
use crate::client::apis::is_api_request::IsApiRequest;
pub mod get_queues;


impl_api_plugin!(
    "/lol-game-queues/v1",
    GetQueues{
        get_queues,reqwest::Method::GET,"/queues" => LolGameQueuesGetQueues,
    },
);