use crate::client::apis::lol_summoner::current_summoner::SummonerInfo;
use crate::client::apis::plugin_macro::impl_api_plugin;


pub mod current_summoner;


impl_api_plugin!(
    LolApiPlugin::LolSummoner,
    CurrentSummoner{
        get_current_summoner,reqwest::Method::GET,"/current-summoner" => SummonerInfo,
    },
);
