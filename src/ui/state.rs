use crate::AppResult;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityState;
use crate::client::apis::lol_game_flow::LolGameFlow;
use crate::client::apis::lol_summoner::current_summoner::SummonerInfo;
use crate::client::apis::lol_summoner::LolSummoner;
use crate::client::client::LolClient;
use crate::ui::view::nav_bar_view::NavBarState;
use crate::ui::view::play_view::PlayState;
use crate::ui::view::test_view::TestState;


#[derive(Debug, Clone, PartialEq, Default)]
pub enum ClientGameFlowState {
    #[default]
    Lobby,
    Queue,
    ChampSelect,
    InGame,
    PostGame,
}


#[derive(Debug, Clone, PartialEq, Default)]
pub enum ClientState {
    #[default]
    NotAvailable,
    Available,
    InGameFlow(ClientGameFlowState),
}



#[derive(Debug, Clone, Default)]
pub struct ConnectedState {
    pub client: LolClient,
    pub state: ClientState,
    pub nav_bar: NavBarState,
    pub summoner_info: SummonerInfo,
    pub test: TestState,
    pub play: PlayState,
}

pub async fn init_connected_state(riot_path: String) -> AppResult<ConnectedState> {
    let client = LolClient::new(riot_path.as_str()).await?;
    let summoner_info = client.execute(LolSummoner::get_current_summoner()).await?;

    Ok(ConnectedState {
        client,
        summoner_info,
        ..Default::default()
    })
}


