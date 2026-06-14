mod app;
mod gamepad;
mod messages;
mod ui;

use iced::{Size, window};

fn main() -> iced::Result {
    iced::application(
        app::GamepadApp::title,
        app::GamepadApp::update,
        app::GamepadApp::view,
    )
    .theme(app::GamepadApp::theme)
    .subscription(app::GamepadApp::subscription)
    .window(window::Settings {
        size: Size::new(1100.0, 700.0),
        min_size: Some(Size::new(800.0, 530.0)),
        ..Default::default()
    })
    .run_with(app::GamepadApp::new)
}

