use crate::{
    gamepad::{gamepad_subscription, DeviceInfo, GamepadState},
    messages::{Message, ViewMode},
    ui::{calibration_view, device_list, gamepad_view::GamepadCanvas, theme::system_theme},
};
use gilrs::GamepadId;
use iced::{
    widget::{button, column, container, row, text},
    Alignment, Element, Length, Subscription, Task, Theme,
};
use std::collections::HashMap;

// ── State ─────────────────────────────────────────────────────────────────────

pub struct GamepadApp {
    /// All currently connected gamepads.
    devices: Vec<DeviceInfo>,
    /// Per-device input state (buttons + axes).
    states: HashMap<GamepadId, GamepadState>,
    /// Which device is shown in the visualiser.
    selected: Option<GamepadId>,
    /// Current UI theme (light / dark).
    theme: Theme,
    /// Whether to show the controller canvas or the calibration panel.
    view_mode: ViewMode,
}

impl GamepadApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                devices: Vec::new(),
                states: HashMap::new(),
                selected: None,
                theme: system_theme(),
                view_mode: ViewMode::Controller,
            },
            Task::none(),
        )
    }

    // ── Title ─────────────────────────────────────────────────────────────────

    pub fn title(&self) -> String {
        "Gamepad".to_string()
    }

    // ── Update ────────────────────────────────────────────────────────────────

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GamepadConnected(id, name) => {
                // Avoid duplicates (gilrs can fire Connected twice on some platforms).
                if !self.devices.iter().any(|d| d.id == id) {
                    self.devices.push(DeviceInfo { id, name });
                    self.states.entry(id).or_default();

                    // Auto-select the first device.
                    if self.selected.is_none() {
                        self.selected = Some(id);
                    }
                }
            }

            Message::GamepadDisconnected(id) => {
                self.devices.retain(|d| d.id != id);
                self.states.remove(&id);

                if self.selected == Some(id) {
                    self.selected = self.devices.first().map(|d| d.id);
                }
            }

            Message::SelectDevice(id) => {
                self.selected = id;
            }

            Message::ButtonChanged(id, button, value) => {
                if let Some(state) = self.states.get_mut(&id) {
                    state.update_button(button, value);
                }
            }

            Message::AxisChanged(id, axis, value) => {
                if let Some(state) = self.states.get_mut(&id) {
                    state.update_axis(axis, value);
                }
            }

            Message::SetViewMode(mode) => {
                self.view_mode = mode;
            }

            Message::SetDeadzone(id, axis, deadzone) => {
                if let Some(state) = self.states.get_mut(&id) {
                    state.set_deadzone(axis, deadzone);
                }
            }

            Message::RecenterAxis(id, axis) => {
                if let Some(state) = self.states.get_mut(&id) {
                    state.recenter_axis(axis);
                }
            }

            Message::ResetAxisCalibration(id, axis) => {
                if let Some(state) = self.states.get_mut(&id) {
                    state.reset_axis_calibration(axis);
                }
            }

            Message::ResetAllCalibration(id) => {
                if let Some(state) = self.states.get_mut(&id) {
                    state.reset_all_calibration();
                }
            }
        }

        Task::none()
    }

    // ── View ──────────────────────────────────────────────────────────────────

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = device_list::view(&self.devices, self.selected);
        let content = self.view_content();

        row![sidebar, content].spacing(12).padding(12).height(Length::Fill).into()
    }

    fn view_content(&self) -> Element<'_, Message> {
        let Some(id) = self.selected else {
            return container(center_text(
                "No gamepad detected.\nConnect a USB or Bluetooth controller\nto begin testing.",
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::ui::styles::app_columns)
            .into();
        };

        let tabs = row![
            button("Control")
                .on_press(Message::SetViewMode(ViewMode::Controller))
                .style(if self.view_mode == ViewMode::Controller {
                    button::primary
                } else {
                    button::secondary
                }),
            button("Calibration")
                .on_press(Message::SetViewMode(ViewMode::Calibration))
                .style(if self.view_mode == ViewMode::Calibration {
                    button::primary
                } else {
                    button::secondary
                }),
        ]
        .spacing(6)
        .align_y(Alignment::Center)
        .padding([8, 12]);

        let inner: Element<'_, Message> = match self.states.get(&id) {
            Some(state) => match self.view_mode {
                ViewMode::Controller => GamepadCanvas::view(state),
                ViewMode::Calibration => calibration_view::view(id, state),
            },
            None => center_text("No state for selected device."),
        };

        container(
            column![tabs, inner].spacing(0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(crate::ui::styles::app_columns)
        .into()
    }

    // ── Theme ─────────────────────────────────────────────────────────────────

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    // ── Subscription ──────────────────────────────────────────────────────────

    pub fn subscription(&self) -> Subscription<Message> {
        gamepad_subscription()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn center_text(s: &str) -> Element<'_, Message> {
    container(text(s).size(15))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}
