use iced::widget::combo_box;
use crate::AppResult;
use crate::client::apis;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityState;
use crate::client::apis::lol_game_flow::get_phase::LolGameFlowPhase;
use crate::client::apis::lol_summoner::current_summoner::SummonerInfo;
use crate::client::client::LolClient;
use crate::ui::view::chat_view::ChatState;
use crate::ui::view::nav_bar_view::NavBarState;
use crate::ui::view::play_view::PlayState;
use crate::ui::view::test_view::TestState;

#[derive(Debug, Clone, Default)]
pub struct ConnectedState {
    pub client: LolClient,
    pub state: LolGameFlowPhase,
    pub chat: ChatState,
    pub nav_bar: NavBarState,
    pub summoner_info: SummonerInfo,
    pub test: TestState,
    pub play: PlayState,
}

pub async fn wait_client_available(riot_path: String) -> AppResult<ConnectedState> {
    let mut client: Option<LolClient> = None;
    loop {
        client = LolClient::new(riot_path.as_str()).await.ok();
        if client.is_some() {
            break;
        }
        println!("Waiting client...");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    let client = client.unwrap();

    loop {
        let availability = client.execute(apis::lol_game_flow::get_availability()).await;
        if let Ok(availability) = availability {
            if availability.is_available &&
                (availability.state == LolGameFlowGetAvailabilityState::Available
                    || availability.state == LolGameFlowGetAvailabilityState::InGameFlow
                ) {
                break;
            }
        }
        println!("Waiting client available ...");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    let summoner_info = client.execute(apis::lol_summoner::get_current_summoner()).await?;
    let me = client.execute(apis::lol_chat::get_me()).await?;
    let friends = client.execute(apis::lol_chat::get_friends() ).await?;
    let state = client.execute(apis::lol_game_flow::get_phase()).await?;
    println!("Client is available");
    Ok(ConnectedState {
        client,
        summoner_info,
        state,
        chat:ChatState{
            me,
            friends:combo_box::State::new(friends),
            ..Default::default()
        },
        ..Default::default()
    })
}


