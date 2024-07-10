use crate::client::apis::lol_champ_select::LolChampSelect;
use crate::client::apis::lol_chat::LolChat;
use crate::client::apis::lol_game_flow::LolGameFlow;
use crate::client::apis::lol_lobby::LolLobby;
use crate::client::apis::lol_summoner::LolSummoner;

pub mod lol_summoner;
pub mod lol_chat;
pub mod lol_game_flow;
pub mod lol_lobby;
pub mod lol_champ_select;
mod plugin_macro;
mod plugin;
pub mod request;

pub struct API;








