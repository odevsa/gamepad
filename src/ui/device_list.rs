use crate::{gamepad::DeviceInfo, messages::Message};
use gilrs::GamepadId;
use iced::{
    widget::{button, column, container, scrollable, text, Column},
    Element, Length,
};

/// Sidebar that lists every connected gamepad.
pub fn view<'a>(
    devices: &'a [DeviceInfo],
    selected: Option<GamepadId>,
) -> Element<'a, Message> {
    let header = container(
        text("Controllers")
            .size(15)
            .font(iced::Font::MONOSPACE),
    )
    .padding([14, 18]);

    let items: Vec<Element<'a, Message>> = if devices.is_empty() {
        vec![
            container(
                text("No gamepads detected")
                    .size(13),
            )
            .padding([12, 16])
            .into(),
        ]
    } else {
        devices
            .iter()
            .map(|device| {
                let is_selected = selected == Some(device.id);
                let id = device.id;
                let name = device.name.clone();

                button(
                    column![
                        text(name).size(13),
                        text(format!("id {:?}", id))
                            .size(10),
                    ]
                    .spacing(2),
                )
                .padding([8, 14])
                .width(Length::Fill)
                .style(if is_selected {
                    button::primary
                } else {
                    button::text
                })
                .on_press(Message::SelectDevice(Some(id)))
                .into()
            })
            .collect()
    };

    let list = scrollable(
        Column::from_vec(items)
            .spacing(4)
            .padding([4, 8])
            .width(Length::Fill),
    )
    .height(Length::Fill);

    container(column![header, list].spacing(0))
        .width(210)
        .height(Length::Fill)
        .style(crate::ui::styles::app_columns)
        .into()
}
