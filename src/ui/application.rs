use iced::{Application, executor, Length};
use iced::{Command, Element, Theme};
use iced::widget::{Column, Row};
use iced::widget::container;
use iced_box::icon::material::load_material_font;
use crate::client::perform_request;
use crate::config::Config;
use crate::ui::message::Message;
use crate::ui::state::{ConnectedState, init_connected_state};
use crate::ui::view::HasView;
use crate::ui::view::nav_bar_view::{NavBarMessage, NavBarView};
use crate::ui::view::play_view::PlayView;
use crate::ui::view::summoner_info_view::SummonerInfoView;
use crate::ui::view::test_view::TestView;
use crate::ui::widget::custom_button;
use crate::ui::widget::custom_button::custom_button;

pub struct MainApp {
    connected_state: Option<ConnectedState>,
    config: Config,
}


impl Application for MainApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();


    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self {
            connected_state: None,
            config: Config::new(),
        }, Command::batch(vec![
            load_material_font().map(Message::FontLoaded),
        ]))
    }

    fn title(&self) -> String {
        "Lol Client".to_string()
    }


    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Connect => {
                Command::perform(init_connected_state(self.config.riot_path.to_string()), Message::ConnectResult)
            }
            Message::ConnectResult(result) => {
                match result {
                    Ok(connected_state) => {
                        self.connected_state = Some(connected_state);
                    }
                    Err(e) => {
                        self.connected_state = None;
                    }
                }
                Command::none()
            }
            Message::NavBar(message) => NavBarView::update(message, &mut self.connected_state),
            Message::Play(message) => PlayView::update(message, &mut self.connected_state),
            Message::Test(message) => TestView::update(message, &mut self.connected_state),
            _ => { Command::none() }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let rest_row = if let Some(connected_state) = &self.connected_state {
            Row::new()
                .push(Column::new()
                    .push(SummonerInfoView::view(connected_state))
                    .push(NavBarView::view(connected_state))
                    .width(Length::Fixed(200.0))
                    .height(Length::Fill)
                    .spacing(30)
                )
                .push(match connected_state.nav_bar.state {
                    NavBarMessage::Play => { PlayView::view(connected_state) }
                    NavBarMessage::Test => {TestView::view(connected_state)}
                }).width(Length::Fill).height(Length::Fill)
        } else {
            Row::new()
        };
        container(Column::new()
            .push(Column::new()
                .push_maybe(if self.connected_state.is_none() {
                    Some(custom_button("Connect")
                        .style(custom_button::success)
                        .on_press(Message::Connect))
                } else {
                    None
                })
                .spacing(20))
            .push(rest_row)
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

