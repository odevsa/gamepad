use gilrs::{Axis, Button, GamepadId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewMode {
    Controller,
    Calibration,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// A gamepad was connected (id, name)
    GamepadConnected(GamepadId, String),
    /// A gamepad was disconnected
    GamepadDisconnected(GamepadId),
    /// User selected a device from the sidebar (None = deselect)
    SelectDevice(Option<GamepadId>),
    /// A button value changed on a gamepad
    ButtonChanged(GamepadId, Button, f32),
    /// An axis value changed on a gamepad
    AxisChanged(GamepadId, Axis, f32),
    /// Switch between controller view and calibration view
    SetViewMode(ViewMode),
    /// Set dead zone for a specific axis
    SetDeadzone(GamepadId, Axis, f32),
    /// Record current axis position as center / zero point
    RecenterAxis(GamepadId, Axis),
    /// Reset calibration for one axis
    ResetAxisCalibration(GamepadId, Axis),
    /// Reset all calibration data for a gamepad
    ResetAllCalibration(GamepadId),
}
