use iced::{Application, executor, Length, Subscription};
use iced::{Command, Element, Theme};
use iced::widget::{Column, Row, text};
use iced::widget::container;
use iced_box::icon::material::load_material_font;
use crate::client::utils::{perform_game_flow_update, perform_request, perform_request_with_delay};
use crate::config::Config;
use crate::ui::message::Message;
use crate::ui::state::{ConnectedState, wait_client_available};
use crate::ui::view::chat_view::ChatView;
use crate::ui::view::HasView;
use crate::ui::view::nav_bar_view::{NavBarMessage, NavBarView};
use crate::ui::view::play_view::{PlayMessage, PlayView};
use crate::ui::view::profile_view::ProfileView;
use plugin_lol_gameflow::LolGameflowGameflowPhase;
use crate::assets::{Assets, load_assets};

pub struct MainApp {
    state: AppState,
    config: Config,
    assets:Assets,
}


impl Application for MainApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();


    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let config = Config::new();
        (Self {
            state: AppState::LoadingAsset,
            config: config.clone(),
            assets:Assets::default(),
        }, Command::batch(vec![
            load_material_font().map(Message::FontLoaded),
            Command::perform(load_assets(), Message::AssetsLoaded),
        ]))
    }

    fn title(&self) -> String {
        "Lol Client".to_string()
    }


    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ConnectResult(mut result) => {
                if let Ok(connected_state) = &mut result {
                    self.state = AppState::Connected(connected_state.clone());
                    Command::batch(vec![
                        perform_request(connected_state, plugin_lol_gameflow::get_lol_gameflow_v1_gameflow_phase(), Message::GamFlowResult),
                    ])
                } else {
                    Command::none()
                }
            }
            Message::Disconnected => {
                self.state = AppState::Disconnected;
                Command::perform(wait_client_available(self.config.riot_path.to_string()), Message::ConnectResult)
            }
            Message::GamFlowResult(r) => {
                match &mut self.state {
                    AppState::Connected(connected_state) => {
                        connected_state.state = r.unwrap_or(LolGameflowGameflowPhase::None);
                        perform_game_flow_update(connected_state)
                    }
                   _ => Command::none(),
                }
            }
            Message::NavBar(message) => NavBarView::update(message, &mut self.state),
            Message::Play(message) => PlayView::update(message, &mut self.state),
            Message::Chat(message) => ChatView::update(message, &mut self.state),
            Message::Profile(message) => ProfileView::update(message, &mut self.state),
            Message::FontLoaded(_) => { Command::none() }
            Message::None => {Command::none()}
            Message::AssetsLoaded(assets) => {
                self.assets = assets.unwrap();
                Command::perform(async {}, |_|Message::Disconnected )
            }
        }
    }


    fn view(&self) -> Element<'_, Message> {
        container(Column::new()
            .push(
                match &self.state {
                    AppState::Connected(connected_state) => {
                        container(Row::new()
                            .push(Column::new()
                                .push(NavBarView::view(connected_state, &self.assets))
                                .width(Length::Fixed(200.0))
                                .height(Length::Fill)
                                .spacing(30)
                            )
                            .push(match connected_state.nav_bar.state {
                                NavBarMessage::Profile => { ProfileView::view(connected_state, &self.assets) }
                                NavBarMessage::Play => { PlayView::view(connected_state, &self.assets) }
                                NavBarMessage::Chat => { ChatView::view(connected_state, &self.assets) }
                            }).width(Length::Fill).height(Length::Fill))
                    }
                    AppState::Disconnected => {
                        container(Column::new().push(text("Waiting for client...")))
                    }
                    AppState::LoadingAsset => {
                        container(Column::new().push(text("Loading asset...")))
                    }
                })
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}


#[derive(Debug, Clone)]
pub enum AppState {
    Connected(ConnectedState),
    LoadingAsset,
    Disconnected,
}
