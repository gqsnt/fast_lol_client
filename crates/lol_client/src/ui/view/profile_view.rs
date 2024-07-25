use std::collections::HashMap;
use iced::{Application, Command, Length};
use iced::widget::{Column, Container, container, Image, text};

use plugin_lol_inventory::LolInventoryInventoryItemWithPayload;
use plugin_lol_summoner::LolSummonerSummoner;
use crate::AppResult;
use crate::assets::Assets;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::profile_view::profile_icon_selector_view::{ProfileIconSelectorMessage, ProfileIconSelectorView, ProfileSelectorState};
use crate::ui::widget::custom_button::custom_button;
use crate::utils::draw_image;

pub mod profile_icon_selector_view;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ProfileStateType {
    #[default]
    Default,
    ProfileIconSelector,
}


#[derive(Debug, Clone, Default)]
pub struct ProfileState {
    pub state: ProfileStateType,
    pub summoner: LolSummonerSummoner,
    pub summoner_icons: HashMap<String, Vec<i32>>,
    pub profile_selector_state:ProfileSelectorState,
}

impl ProfileState {
    pub fn new(summoner:LolSummonerSummoner) -> Self {
        Self {
            state: ProfileStateType::Default,
            summoner,
            summoner_icons: HashMap::new(),
            profile_selector_state: Default::default(),
        }
    }

    pub fn set_summoner_icons(&mut self, summoner_icons: Vec<LolInventoryInventoryItemWithPayload>) {
        self.summoner_icons = HashMap::new();
        for icon in summoner_icons {
            let key = if icon.purchase_date.is_empty() {
                "Other".to_string()
            } else {
                // date is in format yyyyetc
                icon.purchase_date.chars().take(4).collect()
            };
            self.summoner_icons.entry(key).or_insert(vec!()).push(icon.item_id);
        }
    }
}


#[derive(Debug, Clone)]
pub enum ProfileMessage {
    ProfileIconSelector(ProfileIconSelectorMessage),
    SummonerUpdated(AppResult<LolSummonerSummoner>),
    OpenProfileIconSelector,
    CloseProfileIconSelector,
}


pub struct ProfileView {}

impl HasView for ProfileView {
    type State = ProfileState;
    type Message = ProfileMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        match message {
            ProfileMessage::ProfileIconSelector(message) => return ProfileIconSelectorView::update(message, state),
            ProfileMessage::OpenProfileIconSelector => {
                if let AppState::Connected(connected_state) = state {
                    connected_state.profile.state = ProfileStateType::ProfileIconSelector;
                }
            }
            ProfileMessage::CloseProfileIconSelector => {
                if let AppState::Connected(connected_state) = state {
                    connected_state.profile.state = ProfileStateType::Default;
                }
            }
            ProfileMessage::SummonerUpdated(summoner) => {
                if let AppState::Connected(connected_state) = state {
                    if let Ok(summoner) = summoner {
                        connected_state.profile.summoner = summoner;
                    }
                }
            }
        }
        Command::none()
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        container(
            if connected_state.profile.state == ProfileStateType::ProfileIconSelector {
                ProfileIconSelectorView::view(connected_state, assets)
            } else {
                container(Column::new()
                    .push(
                        custom_button(
                            Image::new(assets.get_profile_icon(connected_state.profile.summoner.profile_icon_id).unwrap())
                                .width(Length::Fixed(25.0))
                                .height(Length::Fixed(25.0))
                        )
                            .on_press(ProfileMessage::OpenProfileIconSelector.into()),
                    )
                    .push(text(format!("{} #{}", &connected_state.profile.summoner.display_name, &connected_state.profile.summoner.tag_line)))
                    .push(text(format!("Level: {}", &connected_state.profile.summoner.summoner_level)))
                    .push(text(format!("Rolls: {}", &connected_state.profile.summoner.reroll_points.number_of_rolls)))
                    .spacing(10))
            }
        )
            .center_y()
    }
}



macro_rules! impl_from_for_message_profile {
    ($source_type:ty => $enum_variant:ident) => {
        impl From<$source_type> for ProfileMessage {
            fn from(value: $source_type) -> Self {
                Self::$enum_variant(value)
            }
        }
    };
}

impl_from_for_message_profile!(ProfileIconSelectorMessage => ProfileIconSelector);
