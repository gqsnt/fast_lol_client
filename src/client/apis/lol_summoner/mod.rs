use crate::client::apis::lol_summoner::current_summoner::SummonerInfo;
use crate::impl_api_plugin;

pub mod current_summoner;


impl_api_plugin!(
    LolSummoner,
    CurrentSummoner{
         get_current_summoner,reqwest::Method::GET,"/current-summoner" => SummonerInfo,
    },
);
