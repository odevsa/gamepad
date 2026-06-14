use gilrs::{Axis, Button, GamepadId};
use std::collections::HashMap;

/// Metadata about a connected gamepad device.
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: GamepadId,
    pub name: String,
}

/// Per-axis calibration data.
#[derive(Debug, Clone, Copy)]
pub struct AxisCalibration {
    /// Dead-zone radius: axis values within `[-deadzone, +deadzone]` are clamped to zero.
    pub deadzone: f32,
    /// Center offset: subtracted from the raw value before applying the dead zone.
    pub center: f32,
}

impl Default for AxisCalibration {
    fn default() -> Self {
        Self { deadzone: 0.05, center: 0.0 }
    }
}

/// The live input state of a single gamepad.
#[derive(Debug, Clone, Default)]
pub struct GamepadState {
    pub buttons: HashMap<Button, f32>,
    pub axes: HashMap<Axis, f32>,
    pub calibrations: HashMap<Axis, AxisCalibration>,
}

impl GamepadState {
    /// Returns `true` when the button value exceeds the pressed threshold.
    pub fn is_pressed(&self, button: Button) -> bool {
        self.buttons.get(&button).copied().unwrap_or(0.0) > 0.5
    }

    /// Returns the raw (uncalibrated) axis value in –1.0 … 1.0, defaulting to 0.
    pub fn axis_value(&self, axis: Axis) -> f32 {
        self.axes.get(&axis).copied().unwrap_or(0.0)
    }

    /// Returns the calibrated axis value with center offset and dead zone applied.
    pub fn calibrated_axis(&self, axis: Axis) -> f32 {
        let raw = self.axis_value(axis);
        let cal = self.calibrations.get(&axis).copied().unwrap_or_default();
        let v = (raw - cal.center).clamp(-1.0, 1.0);
        if v.abs() < cal.deadzone { 0.0 } else { v }
    }

    pub fn update_button(&mut self, button: Button, value: f32) {
        self.buttons.insert(button, value);
    }

    pub fn update_axis(&mut self, axis: Axis, value: f32) {
        self.axes.insert(axis, value);
    }

    /// Set the dead zone for a specific axis (clamped to 0.0–1.0).
    pub fn set_deadzone(&mut self, axis: Axis, deadzone: f32) {
        self.calibrations.entry(axis).or_default().deadzone = deadzone.clamp(0.0, 1.0);
    }

    /// Record the current axis position as the zero/center point.
    pub fn recenter_axis(&mut self, axis: Axis) {
        let raw = self.axis_value(axis);
        self.calibrations.entry(axis).or_default().center = raw;
    }

    /// Reset calibration for one axis to defaults.
    pub fn reset_axis_calibration(&mut self, axis: Axis) {
        self.calibrations.remove(&axis);
    }

    /// Reset all calibration data for this gamepad.
    pub fn reset_all_calibration(&mut self) {
        self.calibrations.clear();
    }
}
