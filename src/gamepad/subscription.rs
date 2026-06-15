use crate::messages::Message;
use gilrs::{EventType, Gilrs, Axis, GamepadId};
use iced::futures::channel::mpsc;
use std::time::Duration;
use std::collections::HashMap;

use crate::gamepad::mappings::{load_mappings_from_dir, find_mapping, Mapping};

struct GamepadWorker;

/// Returns a subscription that drives gilrs in a background OS thread and
/// yields gamepad events into the iced message loop.
///
/// In iced 0.13 the API is `Subscription::run_with_id(id, stream)` where the
/// stream is any `Stream<Item = Message> + Send + 'static`.
/// We use `futures::channel::mpsc::Receiver` which implements `Stream`.
pub fn gamepad_subscription() -> iced::Subscription<Message> {
    iced::Subscription::run_with_id(
        std::any::TypeId::of::<GamepadWorker>(),
        gamepad_stream(),
    )
}

fn gamepad_stream() -> mpsc::Receiver<Message> {
    // Buffer of 1024 so the input thread never blocks even under load.
    let (mut tx, rx) = mpsc::channel::<Message>(1024);

    // Parse a debug-printed evdev code like `EvCode(EvCode { kind: 3, code: 10 })`.
    fn parse_code_number(s: &str) -> Option<i32> {
        if let Some(idx) = s.rfind("code: ") {
            let tail = &s[idx + 6..];
            let num_str = tail.chars().take_while(|c| c.is_ascii_digit()).collect::<String>();
            if let Ok(n) = num_str.parse::<i32>() {
                return Some(n);
            }
        }
        None
    }

    std::thread::spawn(move || {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("[gamepad] Failed to initialize gilrs: {e}");
                return;
            }
        };

        // Load mapping files from the `mappings/` directory (optional).
        let mappings = load_mappings_from_dir("mappings");
        
        // Track per-device mapping once connected.
        let mut device_mappings: HashMap<GamepadId, Mapping> = HashMap::new();

        // Report devices already connected at startup.
        for (id, gamepad) in gilrs.gamepads() {
            let name = gamepad.name().to_string();
            if tx
                .try_send(Message::GamepadConnected(id, name.clone()))
                .is_err()
            {
                return;
            }

            if let Some(m) = find_mapping(&mappings, &name) {
                // eprintln!("[gamepad] applied mapping '{}' to id={:?}", m.matcher, id);
                device_mappings.insert(id, m.clone());
            }
        }

        loop {
            while let Some(gilrs::Event { id, event, .. }) = gilrs.next_event() {
                // eprintln!("[gamepad] raw event: id={:?} event={:?}", id, event);
                let msg = match event {
                    EventType::Connected => {
                        let name = gilrs
                            .connected_gamepad(id)
                            .map(|g| g.name().to_string())
                            .unwrap_or_else(|| format!("Gamepad #{}", usize::from(id)));

                        // Apply mapping if a file matches this device name.
                        if let Some(m) = find_mapping(&mappings, &name) {
                            // eprintln!("[gamepad] applied mapping '{}' to id={:?}", m.matcher, id);
                            device_mappings.insert(id, m.clone());
                        }

                        Message::GamepadConnected(id, name)
                    }
                    EventType::Disconnected => {
                        device_mappings.remove(&id);
                        Message::GamepadDisconnected(id)
                    }
                    EventType::ButtonChanged(button, value, _code) => {
                        // Remap button name if mapping exists for this device.
                        if let Some(m) = device_mappings.get(&id) {
                            let reported = format!("{:?}", button);
                            if let Some(&dst) = m.button_names.get(&reported) {
                                // eprintln!("[gamepad] remapped button {:?} -> {:?} for id={:?}", reported, dst, id);
                                Message::ButtonChanged(id, dst, value)
                            } else {
                                Message::ButtonChanged(id, button, value)
                            }
                        } else {
                            Message::ButtonChanged(id, button, value)
                        }
                    }
                    EventType::AxisChanged(axis, value, code) => {
                        // Try to remap using per-device mapping rules.
                        let mut mapped_axis = axis;
                        let mut mapped_value = value;

                        if let Some(m) = device_mappings.get(&id) {
                            // Try mapping by evdev code number first.
                            if let Some(n) = parse_code_number(&format!("{:?}", code)) {
                                if let Some(ax) = m.axis_codes.get(&n) {
                                    mapped_axis = *ax;
                                }
                            }
                            // Next try mapping by reported axis name.
                            if mapped_axis == axis {
                                let reported = format!("{:?}", axis);
                                if let Some(ax) = m.axis_names.get(&reported) {
                                    mapped_axis = *ax;
                                }
                            }

                            // If this mapping requests inversion for this axis, apply it.
                            let rep_mapped = format!("{:?}", mapped_axis);
                            if m.axis_invert.contains(&rep_mapped) {
                                mapped_value = -mapped_value;
                            }
                        }

                        // Fallback: generic Unknown -> RightStickX/RightStickY heuristics.
                        if mapped_axis == Axis::Unknown {
                            if let Some(n) = parse_code_number(&format!("{:?}", code)) {
                                mapped_axis = match n {
                                    9 => Axis::RightStickX,
                                    10 => Axis::RightStickY,
                                    3 => Axis::RightStickX,
                                    4 => Axis::RightStickY,
                                    16 => Axis::DPadX,
                                    17 => Axis::DPadY,
                                    _ => Axis::Unknown,
                                }
                            }
                        }

                        // eprintln!("[gamepad] mapped event: id={:?} axis={:?} value={:.3} (raw={:?})", id, mapped_axis, mapped_value, code);
                        Message::AxisChanged(id, mapped_axis, mapped_value)
                    }
                    _ => continue,
                };

                if let Err(e) = tx.try_send(msg) {
                    if e.is_disconnected() {
                        return; // iced dropped the subscription – shut down
                    }
                    // Buffer full: drop this event and continue
                }
            }

            std::thread::sleep(Duration::from_millis(8));
        }
    });

    rx
}
