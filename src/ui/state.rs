use serde::{Deserialize, Serialize};
use crate::AppResult;
use crate::client::LolClient;
use crate::ui::view::summoner_info_view::SummonerInfoState;
use crate::ui::view::nav_bar_view::NavBarState;


#[derive(Debug, Clone, Default)]
pub struct ConnectedState {
    pub lol_client:LolClient,
    pub nav_bar:NavBarState,
    pub summoner_info:SummonerInfoState,
}

pub async fn init_connected_state(riot_path:String) ->AppResult<ConnectedState>{
    let client = LolClient::new(riot_path.as_str())?;
    let summoner_info = client.get_summoner().await?;
    Ok(ConnectedState{
        lol_client:client,
        summoner_info,
        ..Default::default()
    })
}

