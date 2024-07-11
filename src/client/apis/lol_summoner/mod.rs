use crate::client::apis::lol_summoner::current_summoner::SummonerInfo;
use crate::client::apis::plugin_macro::impl_api_plugin;
use crate::client::apis::is_api_request::IsApiRequest;

pub mod current_summoner;


impl_api_plugin!(
    "/lol-summoner/v1",
    CurrentSummoner{
        get_current_summoner,reqwest::Method::GET,"/current-summoner" => SummonerInfo,
    },
);
