use std::path::PathBuf;
use iced::widget::{combo_box, image};
use plugin_lol_champions::{GetLolChampionsV1OwnedChampionsMinimal, LolChampionsCollectionsChampionMinimal};
use plugin_lol_gameflow::{LolGameflowGameflowAvailabilityState, LolGameflowGameflowPhase};
use plugin_lol_summoner::LolSummonerSummoner;
use crate::AppResult;
use crate::client::LolClient;
use crate::ui::view::chat_view::ChatState;
use crate::ui::view::nav_bar_view::NavBarState;
use crate::ui::view::play_view::PlayState;
use crate::ui::view::profile_view::ProfileState;

#[derive(Clone, Debug)]
pub struct ConnectedState {
    pub client: LolClient,
    pub state: LolGameflowGameflowPhase,
    pub chat: ChatState,
    pub nav_bar: NavBarState,
    pub profile: ProfileState,
    pub play: PlayState,
    pub champions:Vec<LolChampionsCollectionsChampionMinimal>
}

pub async fn wait_client_available(riot_path: String) -> AppResult<ConnectedState> {
    let client = loop {
        if let Some(client) = LolClient::new(&riot_path).await.ok() {
            break client;
        }
        println!("Waiting for client...");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    };


    loop {
        let availability = client.execute(plugin_lol_gameflow::get_lol_gameflow_v1_availability()).await?;
        if availability.is_available &&
            matches!(availability.state, LolGameflowGameflowAvailabilityState::Available | LolGameflowGameflowAvailabilityState::InGameFlow) {
            break;
        }
        println!("Waiting for client to become available...");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    let chat = ChatState {
        me: client.execute(plugin_lol_chat::get_lol_chat_v1_me()).await?,
        friends: combo_box::State::new(client.execute(plugin_lol_chat::get_lol_chat_v1_friends()).await?),
        ..Default::default()
    };

    let queues = client.execute(plugin_lol_game_queues::get_lol_game_queues_v1_queues()).await?;
    let custom_queues = client.execute(plugin_lol_game_queues::get_lol_game_queues_v1_custom()).await?;
    let custom_non_default_queues = client.execute(plugin_lol_game_queues::get_lol_game_queues_v1_custom_non_default()).await?;

    let play = PlayState::new(queues, custom_queues, custom_non_default_queues);
    let summoner = client.execute(plugin_lol_summoner::get_lol_summoner_v1_current_summoner()).await?;
    let state = client.execute(plugin_lol_gameflow::get_lol_gameflow_v1_gameflow_phase()).await?;

    let champions = client.execute(plugin_lol_champions::get_lol_champions_v1_owned_champions_minimal()).await?;
    let summoner_icons = client.execute_and_save(plugin_lol_inventory::get_lol_inventory_v2_inventory_by_inventory_type("SUMMONER_ICON".to_string()), "icons").await?;
    let mut  profile = ProfileState::new(summoner);
    profile.set_summoner_icons(summoner_icons);
    Ok(ConnectedState {
        client,
        profile,
        state,
        chat,
        play,
        champions,
        nav_bar: Default::default(),
    })
}


