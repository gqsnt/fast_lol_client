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

#[derive(Debug, Clone)]
pub struct ConnectedState {
    pub client: LolClient,
    pub state: LolGameFlowPhase,
    pub chat: ChatState,
    pub nav_bar: NavBarState,
    pub summoner_info: SummonerInfo,
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



    let chat = ChatState{
        me:client.execute(apis::lol_chat::get_me()).await?,
        friends:combo_box::State::new(client.execute(apis::lol_chat::get_friends() ).await?),
        ..Default::default()
    };

    let queues = client.execute_and_save(apis::lol_game_queues::get_queues(), "queues").await?;
    let custom_queues = client.execute_and_save(apis::lol_game_queues::get_custom_queues(), "custom_queues").await?;
    let custom_non_default_queues = client.execute_and_save(apis::lol_game_queues::get_custom_non_default_queues(), "custom_non_default_queues").await?;

    let play = PlayState::new(queues, custom_queues, custom_non_default_queues);
    let summoner_info = client.execute(apis::lol_summoner::get_current_summoner()).await?;
    let state = client.execute(apis::lol_game_flow::get_phase()).await?;
    Ok(ConnectedState {
        client,
        summoner_info,
        state,
        chat,
        play,
        nav_bar: Default::default(),
    })
}


