use crate::AppResult;
use crate::client::apis::lol_champ_select::LolChampSelect;
use crate::client::apis::lol_game_flow::LolGameFlow;
use crate::client::apis::lol_summoner::current_summoner::{SummonerInfo};
use crate::client::apis::lol_summoner::LolSummoner;
use crate::client::client::LolClient;
use crate::ui::view::nav_bar_view::NavBarState;
use crate::ui::view::play_view::PlayState;
use crate::ui::view::test_view::TestState;

#[derive(Debug, Clone)]
pub struct ConnectedState {
    pub client: LolClient,
    pub nav_bar: NavBarState,
    pub summoner_info: SummonerInfo,
    pub test_state: TestState,
    pub play_state: PlayState,
}

pub async fn init_connected_state(riot_path: String) -> AppResult<ConnectedState> {
    let client = LolClient::new(riot_path.as_str()).await?;
    let summoner_info = client.execute(LolSummoner::get_current_summoner()).await?;

    Ok(ConnectedState {
        client,
        summoner_info,
        nav_bar: Default::default(),
        test_state: Default::default(),
        play_state: Default::default(),
    })
}

