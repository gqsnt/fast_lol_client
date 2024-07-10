use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityState;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_summoner::current_summoner::SummonerInfo;
use crate::client::client::LolClient;
use crate::ui::view::nav_bar_view::NavBarState;
use crate::ui::view::play_view::PlayState;
use crate::ui::view::test_view::TestState;





#[derive(Debug, Clone, Default)]
pub struct ConnectedState {
    pub client: LolClient,
    pub state: Option<LolGameFlowPhase>,
    pub nav_bar: NavBarState,
    pub summoner_info: SummonerInfo,
    pub test: TestState,
    pub play: PlayState,
}

pub async fn init_connected_state(riot_path: String) -> AppResult<ConnectedState> {
    let client = LolClient::new(riot_path.as_str()).await?;
    let summoner_info = client.execute(apis::lol_summoner::get_current_summoner()).await?;

    Ok(ConnectedState {
        client,
        summoner_info,
        ..Default::default()
    })
}


