use crate::client::apis::lol_game_queues::get_queues::LolGameQueuesGetQueues;
use crate::impl_api_plugin;

pub mod get_queues;


impl_api_plugin!(
    "/lol-game-queues",
    V1{
        GetQueues{
            fn:get_queues,method: reqwest::Method::GET,url: "/queues" => LolGameQueuesGetQueues
        }
    }
);