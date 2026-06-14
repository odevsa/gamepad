use crate::ui::palette::ControllerPalette;
use iced::{Theme};
use iced::widget::container;
use iced::Background;

/// Container style for app columns using palette.background.
pub fn app_columns(theme: &Theme) -> container::Style {
    let p = ControllerPalette::from_theme(theme);
    container::Style::default().background(Background::Color(p.background))
}
