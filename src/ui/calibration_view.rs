use crate::{
    gamepad::GamepadState,
    messages::{Message},
};
use gilrs::{Axis, GamepadId};
use iced::{
    widget::{button, column, container, progress_bar, row, scrollable, slider, text, Column},
    Alignment, Element, Length,
};

const AXES: &[(Axis, &str)] = &[
    (Axis::LeftStickX,  "Left Stick X"),
    (Axis::LeftStickY,  "Left Stick Y"),
    (Axis::RightStickX, "Right Stick X"),
    (Axis::RightStickY, "Right Stick Y"),
    (Axis::LeftZ,       "Left Trigger"),
    (Axis::RightZ,      "Right Trigger"),
    (Axis::DPadX,       "D-Pad X"),
    (Axis::DPadY,       "D-Pad Y"),
];

pub fn view<'a>(id: GamepadId, state: &'a GamepadState) -> Element<'a, Message> {
    let header = row![
        text("Calibração de Eixos").size(17),
        iced::widget::horizontal_space(),
        button("Resetar Tudo")
            .on_press(Message::ResetAllCalibration(id))
            .style(button::danger),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .padding([0, 4]);

    let column_headers = row![
        text("Eixo").size(11).width(130),
        text("Bruto").size(11).width(130),
        text("Calibrado").size(11).width(130),
        text("Zona Morta").size(11).width(200),
        text("Ações").size(11),
    ]
    .spacing(12);

    let axis_rows: Vec<Element<'a, Message>> = AXES
        .iter()
        .map(|(axis, label)| axis_row(id, *axis, label, state))
        .collect();

    let table = Column::from_vec(axis_rows).spacing(10);

    let body = column![column_headers, table].spacing(8);

    container(
        column![
            header,
            scrollable(body).height(Length::Fill),
        ]
        .spacing(16)
        .padding(20),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn axis_row<'a>(id: GamepadId, axis: Axis, label: &'a str, state: &'a GamepadState) -> Element<'a, Message> {
    let raw = state.axis_value(axis);
    let calibrated = state.calibrated_axis(axis);
    let cal = state.calibrations.get(&axis).copied().unwrap_or_default();

    // Map -1..1 to 0..1 for the progress bars.
    let raw_bar = ((raw + 1.0) / 2.0).clamp(0.0, 1.0);
    let cal_bar = ((calibrated + 1.0) / 2.0).clamp(0.0, 1.0);

    let name_col = text(label).size(13).width(130);

    let raw_col = column![
        progress_bar(0.0_f32..=1.0_f32, raw_bar).height(8).width(110),
        text(format!("{:+.3}", raw)).size(11),
    ]
    .spacing(3)
    .width(130);

    let cal_col = column![
        progress_bar(0.0_f32..=1.0_f32, cal_bar).height(8).width(110),
        text(format!("{:+.3}", calibrated)).size(11),
    ]
    .spacing(3)
    .width(130);

    let dz_col = column![
        text(format!("{:.3}", cal.deadzone)).size(11),
        slider(0.0_f32..=0.30_f32, cal.deadzone, move |v| {
            Message::SetDeadzone(id, axis, v)
        })
        .step(0.001_f32)
        .width(180),
    ]
    .spacing(3)
    .width(200);

    let btns = row![
        button(text("Centro").size(11))
            .on_press(Message::RecenterAxis(id, axis))
            .padding([4, 8]),
        button(text("Reset").size(11))
            .on_press(Message::ResetAxisCalibration(id, axis))
            .padding([4, 8])
            .style(button::danger),
    ]
    .spacing(4);

    row![name_col, raw_col, cal_col, dz_col, btns]
        .spacing(12)
        .align_y(Alignment::Center)
        .into()
}
