use iced_box::icon::LoadingResult;

use crate::AppResult;
use crate::ui::state::ConnectedState;
use crate::ui::view::nav_bar_view::NavBarMessage;
use crate::ui::view::summoner_info_view::SummonerInfoMessage;

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(LoadingResult),
    Connect,
    ConnectResult(AppResult<ConnectedState>),
    NavBar(NavBarMessage),
    SummonerInfo(SummonerInfoMessage),
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


impl_from_for_message!(SummonerInfoMessage => SummonerInfo);
impl_from_for_message!(NavBarMessage => NavBar);

