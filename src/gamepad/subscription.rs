use crate::messages::Message;
use gilrs::{EventType, Gilrs};
use iced::futures::channel::mpsc;
use std::time::Duration;

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

    std::thread::spawn(move || {
        let mut gilrs = match Gilrs::new() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("[gamepad] Failed to initialize gilrs: {e}");
                return;
            }
        };

        // Report devices already connected at startup.
        for (id, gamepad) in gilrs.gamepads() {
            if tx
                .try_send(Message::GamepadConnected(id, gamepad.name().to_string()))
                .is_err()
            {
                return;
            }
        }

        loop {
            while let Some(gilrs::Event { id, event, .. }) = gilrs.next_event() {
                let msg = match event {
                    EventType::Connected => {
                        let name = gilrs
                            .connected_gamepad(id)
                            .map(|g| g.name().to_string())
                            .unwrap_or_else(|| format!("Gamepad #{}", usize::from(id)));
                        Message::GamepadConnected(id, name)
                    }
                    EventType::Disconnected => Message::GamepadDisconnected(id),
                    EventType::ButtonChanged(button, value, _) => {
                        Message::ButtonChanged(id, button, value)
                    }
                    EventType::AxisChanged(axis, value, _) => {
                        Message::AxisChanged(id, axis, value)
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
