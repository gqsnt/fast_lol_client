use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::client::apis::lol_game_queues::get_custom_queues::{LolGameQueuesQueueCustomGame, LolGameQueuesQueueCustomGameSubcategory};
use crate::client::apis::lol_game_queues::get_queues::{LolGameQueuesGetQueue, LolGameQueuesGetQueues};
use crate::impl_api_plugin;

pub mod get_queues;
pub mod get_custom_queues;

impl_api_plugin!(
    "/lol-game-queues",
    V1{
        GetQueues{
            get_queues,reqwest::Method::GET, "/queues"
        } => LolGameQueuesGetQueues

        GetCustomQueues{
            get_custom_queues,reqwest::Method::GET, "/custom"
        } => LolGameQueuesQueueCustomGame

        GetCustomNonDefaultQueues{
            get_custom_non_default_queues,reqwest::Method::GET, "/custom-non-default"
        } => LolGameQueuesQueueCustomGame
    }
);
