use iced_box::icon::LoadingResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::AppResult;
use crate::client::apis::lol_game_flow::get_availability::LolGameFlowGetAvailabilityResponse;
use crate::ui::state::{ClientState, ConnectedState};
use crate::ui::view::chat_view::ChatMessage;
use crate::ui::view::nav_bar_view::NavBarMessage;
use crate::ui::view::play_view::PlayMessage;
use crate::ui::view::profile_view::ProfileMessage;
use crate::ui::view::test_view::TestMessage;

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(LoadingResult),
    Connect,
    ConnectResult(AppResult<ConnectedState>),
    Disconnected,
    ClientStateUpdated(AppResult<ClientState>),
    NavBar(NavBarMessage),
    SummonerInfo(ProfileMessage),
    Test(TestMessage),
    Play(PlayMessage),
    Chat(ChatMessage),

}






macro_rules! impl_from_for_message {
    ($source_type:ty => $enum_variant:ident) => {
        impl From<$source_type> for Message {
            fn from(value: $source_type) -> Self {
                Self::$enum_variant(value)
            }
        }
    };
}


impl_from_for_message!(ProfileMessage => SummonerInfo);
impl_from_for_message!(NavBarMessage => NavBar);
impl_from_for_message!(TestMessage => Test);
impl_from_for_message!(PlayMessage => Play);
impl_from_for_message!(ChatMessage => Chat);

