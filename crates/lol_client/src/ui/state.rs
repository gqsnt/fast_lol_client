use std::path::PathBuf;
use iced::widget::combo_box;
use plugin_lol_gameflow::{LolGameflowGameflowAvailabilityState, LolGameflowGameflowPhase};
use plugin_lol_summoner::LolSummonerSummoner;
use crate::AppResult;
use crate::client::client::LolClient;
use crate::ui::view::chat_view::ChatState;
use crate::ui::view::nav_bar_view::NavBarState;
use crate::ui::view::play_view::PlayState;

#[derive(Clone, Debug)]
pub struct ConnectedState {
    pub client: LolClient,
    pub state: LolGameflowGameflowPhase,
    pub chat: ChatState,
    pub nav_bar: NavBarState,
    pub summoner_info: LolSummonerSummoner,
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
        let availability = client.execute(plugin_lol_gameflow::get_lol_gameflow_v_1_availability()).await;
        if let Ok(availability) = availability {
            if availability.is_available &&
                (availability.state == LolGameflowGameflowAvailabilityState::Available
                    || availability.state == LolGameflowGameflowAvailabilityState::InGameFlow
                ) {
                break;
            }
        }
        println!("Waiting client available ...");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }



    let chat = ChatState{
        me:client.execute(plugin_lol_chat::get_lol_chat_v_1_me()).await?,
        friends:combo_box::State::new(client.execute(plugin_lol_chat::get_lol_chat_v_1_friends() ).await?),
        ..Default::default()
    };

    let queues = client.execute(plugin_lol_game_queues::get_lol_game_queues_v_1_queues()).await?;
    let custom_queues = client.execute(plugin_lol_game_queues::get_lol_game_queues_v_1_custom()).await?;
    let custom_non_default_queues = client.execute(plugin_lol_game_queues::get_lol_game_queues_v_1_custom_non_default()).await?;

    let play = PlayState::new(queues, custom_queues, custom_non_default_queues);
    let summoner_info = client.execute(plugin_lol_summoner::get_lol_summoner_v_1_current_summoner()).await?;
    let state = client.execute(plugin_lol_gameflow::get_lol_gameflow_v_1_gameflow_phase()).await?;
    Ok(ConnectedState {
        client,
        summoner_info,
        state,
        chat,
        play,
        nav_bar: Default::default(),
    })
}


