use serde::{Deserialize, Serialize};
use crate::AppResult;
use crate::client::apis::lol_summoner::current_summoner::{LolSummonerGetCurrentSummoner, SummonerInfo};
use crate::client::LolClient;
use crate::ui::view::summoner_info_view::SummonerInfoState;
use crate::ui::view::nav_bar_view::NavBarState;


#[derive(Debug, Clone, Default)]
pub struct ConnectedState {
    pub client:LolClient,
    pub nav_bar:NavBarState,
    pub summoner_info:SummonerInfo,
}

pub async fn init_connected_state(riot_path:String) ->AppResult<ConnectedState>{
    let client = LolClient::new(riot_path.as_str())?;
    let summoner_info = client.execute(LolSummonerGetCurrentSummoner{}).await?;
    Ok(ConnectedState{
        client,
        summoner_info,
        ..Default::default()
    })
}

