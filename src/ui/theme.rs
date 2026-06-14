use iced::Theme;

/// Detect the OS-level dark/light preference and return the matching theme.
pub fn system_theme() -> Theme {
    match dark_light::detect() {
        dark_light::Mode::Light => Theme::Light,
        dark_light::Mode::Dark | dark_light::Mode::Default => Theme::Dark,
    }
}
