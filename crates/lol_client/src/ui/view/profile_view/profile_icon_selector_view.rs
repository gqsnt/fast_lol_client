use iced::{Application, Command, Length};
use iced::widget::{Column, Container, container, Image, pick_list, Row, scrollable};

use plugin_lol_summoner::LolSummonerSummonerIcon;

use crate::assets::Assets;
use crate::client::utils::perform_request;
use crate::ui::application::AppState;
use crate::ui::message::Message;
use crate::ui::state::ConnectedState;
use crate::ui::view::HasView;
use crate::ui::view::profile_view::ProfileMessage;
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

#[derive(Debug, Clone, Default)]
pub struct ProfileSelectorState {
    pub selected_icon_key: Option<String>,
}


#[derive(Debug, Clone)]
pub enum ProfileIconSelectorMessage {
    SelectProfileIcon(i32),
    SelectProfileIconKey(String),
}


pub struct ProfileIconSelectorView {}

impl HasView for ProfileIconSelectorView {
    type State = ProfileSelectorState;
    type Message = ProfileIconSelectorMessage;

    fn update(message: Self::Message, state: &mut AppState) -> Command<Message> {
        if let AppState::Connected(connected_state) = state {
            match message {
                ProfileIconSelectorMessage::SelectProfileIcon(profile_icon_id) => {
                    println!("Selecting profile icon: {}", profile_icon_id);
                    return Command::batch(vec![
                        perform_request(connected_state, plugin_lol_summoner::put_lol_summoner_v1_current_summoner_icon(LolSummonerSummonerIcon {
                            profile_icon_id,
                            inventory_token: "".to_string(),
                        }), |r| ProfileMessage::SummonerUpdated(r).into()),
                        Command::perform(async {}, |_| ProfileMessage::CloseProfileIconSelector.into()),
                    ]);
                }
                ProfileIconSelectorMessage::SelectProfileIconKey(new_key) => {
                    connected_state.profile.profile_selector_state.selected_icon_key = Some(new_key);
                }
            }
        }
        Command::none()
    }
    fn view<'a>(connected_state: &'a ConnectedState, assets: &'a Assets) -> Container<'a, Message> {
        let mut keys = connected_state.profile.summoner_icons.keys().cloned().collect::<Vec<String>>();
        keys.sort();
        let mut inner = Column::new()
            .push(
                Row::new()
                    .push(
                        pick_list(
                            keys,
                            connected_state.profile.profile_selector_state.selected_icon_key.as_ref(),
                            |r| ProfileMessage::ProfileIconSelector(ProfileIconSelectorMessage::SelectProfileIconKey(r)).into(),
                        )
                    )
                    .push(
                        custom_button("Close")
                            .on_press(ProfileMessage::CloseProfileIconSelector.into())
                            .style(custom_button::danger)
                    )
                    .spacing(20)
            );
        if let Some(key) = &connected_state.profile.profile_selector_state.selected_icon_key {
            let icons = connected_state.profile.summoner_icons.get(key).unwrap();
            let rows = icons.chunks(7).map(|icons| {
                let mut row = Row::new();
                for icon in icons {
                    row = row.push(
                        custom_button(
                            Image::new(assets.get_profile_icon(*icon).unwrap())
                                .width(Length::Fixed(50.0))
                                .height(Length::Fixed(50.0))
                        )
                            .style(if *icon == connected_state.profile.summoner.profile_icon_id { custom_button::success } else { custom_button::primary })
                            .on_press(ProfileMessage::ProfileIconSelector(ProfileIconSelectorMessage::SelectProfileIcon(*icon)).into())
                    );
                }
                row.spacing(20)
                    .padding([0.0, 20.0])
            });
            for row in rows {
                inner = inner.push(row);
            }
        }

        container(scrollable(inner.spacing(20)))
            .center_y()
    }
}


