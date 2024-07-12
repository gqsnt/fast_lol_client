use crate::client::apis::lol_summoner::current_summoner::SummonerInfo;
use crate::client::apis::plugin_macro::impl_api_plugin;

pub mod current_summoner;


impl_api_plugin!(
    "/lol-summoner",
    V1{
         CurrentSummoner{
            fn:get_current_summoner,method: reqwest::Method::GET,url: "/current-summoner" => SummonerInfo
        }
    }
);
