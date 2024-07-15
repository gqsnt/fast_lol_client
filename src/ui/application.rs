use iced::{Application, executor, Length, Subscription};
use iced::{Command, Element, Theme};
use iced::widget::{Column, Row, text};
use iced::widget::container;
use iced_box::icon::material::load_material_font;

use crate::client::apis;
use crate::client::utils::{perform_game_flow_update, perform_request, perform_request_with_delay};
use crate::config::Config;
use crate::ui::message::Message;
use crate::ui::state::{ConnectedState, wait_client_available};
use crate::ui::view::chat_view::ChatView;
use crate::ui::view::HasView;
use crate::ui::view::nav_bar_view::{NavBarMessage, NavBarView};
use crate::ui::view::play_view::{PlayMessage, PlayView};
use crate::ui::view::profile_view::ProfileView;
use crate::ui::view::test_view::TestView;

pub struct MainApp {
    state: AppState,
    config: Config,
}


impl Application for MainApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();


    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let config = Config::new();
        (Self {
            state: AppState::Disconnected,
            config: config.clone(),
        }, Command::batch(vec![
            load_material_font().map(Message::FontLoaded),
            Command::perform(wait_client_available(config.riot_path.to_string()), Message::ConnectResult),
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
                        perform_request(connected_state, apis::lol_game_flow::get_phase(), Message::GamFlowResult),
                        perform_request(connected_state, apis::lol_game_queues::get_queues(), |r| PlayMessage::RequestQueuesResult(r).into()),
                    ])
                } else {
                    Command::none()
                }
            }
            Message::Disconnected => {
                self.state = AppState::Disconnected;
                println!("Disconnected");
                Command::perform(wait_client_available(self.config.riot_path.to_string()), Message::ConnectResult)
            }
            Message::GamFlowResult(r) => {
                match &mut self.state {
                    AppState::Connected(connected_state) => {
                        connected_state.state = r.unwrap_or_default();
                        perform_game_flow_update(connected_state)
                    }
                    AppState::Disconnected => Command::none()
                }
            }
            Message::NavBar(message) => NavBarView::update(message, &mut self.state),
            Message::Play(message) => PlayView::update(message, &mut self.state),
            Message::Test(message) => TestView::update(message, &mut self.state),
            Message::Chat(message) => ChatView::update(message, &mut self.state),
            Message::Profile(message) => ProfileView::update(message, &mut self.state),
            Message::FontLoaded(_) => { Command::none() }
            Message::None => {Command::none()}
        }
    }


    fn view(&self) -> Element<'_, Message> {
        container(Column::new()
            .push(
                match &self.state {
                    AppState::Connected(connected_state) => {
                        container(Row::new()
                            .push(Column::new()
                                .push(NavBarView::view(connected_state))
                                .width(Length::Fixed(200.0))
                                .height(Length::Fill)
                                .spacing(30)
                            )
                            .push(match connected_state.nav_bar.state {
                                NavBarMessage::Profile => { ProfileView::view(connected_state) }
                                NavBarMessage::Play => { PlayView::view(connected_state) }
                                NavBarMessage::Test => { TestView::view(connected_state) }
                                NavBarMessage::Chat => { ChatView::view(connected_state) }
                            }).width(Length::Fill).height(Length::Fill))
                    }
                    AppState::Disconnected => {
                        container(Column::new().push(text("Waiting for client...")))
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
    Disconnected,
}
