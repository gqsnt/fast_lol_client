use iced::{Application, Settings, Size};

use fast_lol_client::ui::application::MainApp;

fn main() -> iced::Result {
    MainApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(1280.0, 720.0),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}