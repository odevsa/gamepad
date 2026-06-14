use crate::{gamepad::GamepadState, messages::Message, ui::palette::ControllerPalette};
use gilrs::{Axis, Button};
use iced::{
    alignment::{Horizontal, Vertical},
    mouse,
    widget::{
        canvas::{self, Frame, Geometry, Path, Program, Stroke, Text},
        text::{LineHeight, Shaping},
    },
    Color, Element, Font, Length, Pixels, Point, Rectangle, Renderer, Size, Theme,
};

// ── Public entry ─────────────────────────────────────────────────────────────

pub struct GamepadCanvas<'a> {
    state: &'a GamepadState,
}

impl<'a> GamepadCanvas<'a> {
    pub fn view(state: &'a GamepadState) -> Element<'a, Message> {
        iced::widget::canvas(GamepadCanvas { state })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// ── Program impl ─────────────────────────────────────────────────────────────

impl<'a> Program<Message> for GamepadCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        draw_controller(&mut frame, theme, bounds.size(), self.state);
        vec![frame.into_geometry()]
    }
}

// ── Main drawing function ─────────────────────────────────────────────────────

fn draw_controller(frame: &mut Frame, theme: &Theme, size: Size, state: &GamepadState) {
    // All color decisions live in one place: the palette.
    let p = ControllerPalette::from_theme(theme);

    // Virtual canvas: 700 x 430 units
    const VW: f32 = 700.0;
    const VH: f32 = 430.0;
    let scale = (size.width / VW).min(size.height / VH) * 0.9;
    let ox = (size.width - VW * scale) / 2.0;
    let oy = (size.height - VH * scale) / 2.0;

    let sc = |v: f32| v * scale;
    let pt = |vx: f32, vy: f32| Point::new(ox + vx * scale, oy + vy * scale);

    // Grips (behind body)
    fill_rrect(frame, pt(70.0, 275.0), sc(145.0), sc(155.0), sc(40.0), p.grip);
    fill_rrect(frame, pt(485.0, 275.0), sc(145.0), sc(155.0), sc(40.0), p.grip);

    // Main body
    fill_rrect(frame, pt(45.0, 80.0), sc(610.0), sc(250.0), sc(45.0), p.body);

    // Triggers (LT / RT)
    let lt_v = normalize_trigger(state.axis_value(Axis::LeftZ));
    let rt_v = normalize_trigger(state.axis_value(Axis::RightZ));
    draw_trigger(frame, pt(72.0, 40.0), sc(145.0), sc(40.0), sc(11.0),
        lt_v, p.track, p.accent, p.label, "LT", scale);
    draw_trigger(frame, pt(483.0, 40.0), sc(145.0), sc(40.0), sc(11.0),
        rt_v, p.track, p.accent, p.label, "RT", scale);

    // Bumpers (LB / RB)
    let lb = state.is_pressed(Button::LeftTrigger);
    let rb = state.is_pressed(Button::RightTrigger);
    fill_rrect(frame, pt(72.0, 78.0), sc(145.0), sc(30.0), sc(10.0),
        if lb { p.accent } else { p.bumper });
    fill_rrect(frame, pt(483.0, 78.0), sc(145.0), sc(30.0), sc(10.0),
        if rb { p.accent } else { p.bumper });
    draw_label(frame, pt(144.0, 93.0), "LB", sc(11.0), p.label);
    draw_label(frame, pt(555.0, 93.0), "RB", sc(11.0), p.label);

    // D-pad
    let dcx = ox + 240.0 * scale;
    let dcy = oy + 260.0 * scale;
    let aw = sc(29.0);
    let al = sc(40.0);
    let dr = sc(7.0);

    let dup    = state.is_pressed(Button::DPadUp);
    let ddown  = state.is_pressed(Button::DPadDown);
    let dleft  = state.is_pressed(Button::DPadLeft);
    let dright = state.is_pressed(Button::DPadRight);

    fill_rrect(frame, Point::new(dcx - aw / 2.0, dcy - aw / 2.0), aw, aw, dr, p.idle);
    fill_rrect(frame, Point::new(dcx - aw / 2.0, dcy - aw / 2.0 - al), aw, al, dr,
        if dup    { p.accent } else { p.idle });
    fill_rrect(frame, Point::new(dcx - aw / 2.0, dcy + aw / 2.0), aw, al, dr,
        if ddown  { p.accent } else { p.idle });
    fill_rrect(frame, Point::new(dcx - aw / 2.0 - al, dcy - aw / 2.0), al, aw, dr,
        if dleft  { p.accent } else { p.idle });
    fill_rrect(frame, Point::new(dcx + aw / 2.0, dcy - aw / 2.0), al, aw, dr,
        if dright { p.accent } else { p.idle });

    // Left analog stick
    let ls_c = pt(130.0, 180.0);
    let ls_r  = sc(50.0);
    draw_stick(frame, ls_c, ls_r, sc(20.0),
        state.axis_value(Axis::LeftStickX), state.axis_value(Axis::LeftStickY),
        state.is_pressed(Button::LeftThumb), p.track, p.stick_dot, p.accent);
    draw_label(frame, Point::new(ls_c.x, ls_c.y + ls_r + sc(14.0)), "L3", sc(10.0), p.label);

    // Right analog stick
    let rs_c = pt(450.0, 250.0);
    let rs_r  = sc(50.0);
    draw_stick(frame, rs_c, rs_r, sc(20.0),
        state.axis_value(Axis::RightStickX), state.axis_value(Axis::RightStickY),
        state.is_pressed(Button::RightThumb), p.track, p.stick_dot, p.accent);
    draw_label(frame, Point::new(rs_c.x, rs_c.y + rs_r + sc(14.0)), "R3", sc(10.0), p.label);

    // Face buttons
    let fc = pt(560.0, 195.0);
    let br = sc(20.0);
    let fs = sc(45.0);

    draw_face_button(frame, Point::new(fc.x, fc.y + fs), br,
        state.is_pressed(Button::South), p.face_a, "A", scale);
    draw_face_button(frame, Point::new(fc.x + fs, fc.y), br,
        state.is_pressed(Button::East), p.face_b, "B", scale);
    draw_face_button(frame, Point::new(fc.x - fs, fc.y), br,
        state.is_pressed(Button::West), p.face_x, "X", scale);
    draw_face_button(frame, Point::new(fc.x, fc.y - fs), br,
        state.is_pressed(Button::North), p.face_y, "Y", scale);

    // Center buttons
    let sel = state.is_pressed(Button::Select);
    let gui = state.is_pressed(Button::Mode);
    let sta = state.is_pressed(Button::Start);
    draw_center_btn(frame, pt(310.0, 162.0), sc(16.0), sel, p.idle, p.accent, p.label, "-", scale);
    draw_center_btn(frame, pt(350.0, 144.0), sc(21.0), gui, p.idle, p.accent, p.label, "\u{23FB}", scale);
    draw_center_btn(frame, pt(390.0, 162.0), sc(16.0), sta, p.idle, p.accent, p.label, "+", scale);
}

// ── Component drawing helpers ────────────────────────────────────────────────

fn draw_trigger(
    frame: &mut Frame,
    tl: Point, w: f32, h: f32, r: f32,
    fill_ratio: f32,
    track: Color, accent: Color, label_color: Color,
    label: &str, scale: f32,
) {
    fill_rrect(frame, tl, w, h, r, track);
    if fill_ratio > 0.005 {
        let fh = (h * fill_ratio).max(r * 2.0).min(h);
        let tla = Point::new(tl.x, tl.y + h - fh);
        fill_rrect(frame, tla, w, fh, r, Color { a: 0.85, ..accent });
    }
    draw_label(frame, Point::new(tl.x + w / 2.0, tl.y + h / 2.0), label, scale * 11.0, label_color);
}

fn draw_stick(
    frame: &mut Frame,
    center: Point, well_r: f32, dot_r: f32,
    ax: f32, ay: f32,
    pressed: bool,
    well_color: Color, dot_idle: Color, accent: Color,
) {
    let well = Path::circle(center, well_r);
    frame.fill(&well, well_color);
    frame.stroke(&well, Stroke {
        style: canvas::stroke::Style::Solid(if pressed { accent } else { dot_idle }),
        width: 2.5,
        ..Default::default()
    });

    let max_offset = well_r - dot_r;
    let dot = Path::circle(
        Point::new(
            center.x + ax.clamp(-1.0, 1.0) * max_offset,
            center.y - ay.clamp(-1.0, 1.0) * max_offset,
        ),
        dot_r,
    );
    frame.fill(&dot, if pressed { accent } else { dot_idle });
}

fn draw_face_button(
    frame: &mut Frame,
    center: Point, radius: f32,
    pressed: bool, color: Color,
    label: &str, scale: f32,
) {
    let path = Path::circle(center, radius);
    frame.fill(&path, if pressed { color } else { Color { a: 0.3, ..color } });
    frame.stroke(&path, Stroke {
        style: canvas::stroke::Style::Solid(color),
        width: 1.8,
        ..Default::default()
    });
    draw_label(frame, center, label, scale * 13.0, Color::WHITE);
}

fn draw_center_btn(
    frame: &mut Frame,
    center: Point, radius: f32,
    pressed: bool,
    idle: Color, accent: Color, label_color: Color,
    label: &str, scale: f32,
) {
    frame.fill(&Path::circle(center, radius), if pressed { accent } else { idle });
    draw_label(frame, center, label, scale * 11.0, label_color);
}

// ── Primitive helpers ────────────────────────────────────────────────────────

fn fill_rrect(frame: &mut Frame, tl: Point, w: f32, h: f32, r: f32, color: Color) {
    frame.fill(&Path::new(|b| rrect(b, tl.x, tl.y, w, h, r)), color);
}

fn draw_label(frame: &mut Frame, position: Point, content: &str, size: f32, color: Color) {
    frame.fill_text(Text {
        content: content.to_string(),
        position,
        color,
        size: Pixels(size.max(1.0)),
        horizontal_alignment: Horizontal::Center,
        vertical_alignment: Vertical::Center,
        font: Font::DEFAULT,
        line_height: LineHeight::default(),
        shaping: Shaping::Basic,
    });
}

/// Build a rounded-rectangle path using cubic-bezier corner approximations.
fn rrect(b: &mut canvas::path::Builder, x: f32, y: f32, w: f32, h: f32, r: f32) {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    const K: f32 = 0.5523; // bezier ≈ quarter-circle
    let kr = K * r;

    b.move_to(Point::new(x + r, y));
    b.line_to(Point::new(x + w - r, y));
    b.bezier_curve_to(
        Point::new(x + w - r + kr, y),
        Point::new(x + w, y + r - kr),
        Point::new(x + w, y + r),
    );
    b.line_to(Point::new(x + w, y + h - r));
    b.bezier_curve_to(
        Point::new(x + w, y + h - r + kr),
        Point::new(x + w - r + kr, y + h),
        Point::new(x + w - r, y + h),
    );
    b.line_to(Point::new(x + r, y + h));
    b.bezier_curve_to(
        Point::new(x + r - kr, y + h),
        Point::new(x, y + h - r + kr),
        Point::new(x, y + h - r),
    );
    b.line_to(Point::new(x, y + r));
    b.bezier_curve_to(
        Point::new(x, y + r - kr),
        Point::new(x + r - kr, y),
        Point::new(x + r, y),
    );
    b.close();
}

/// Normalize a trigger axis value (which can be -1..1 or 0..1) to 0..1.
fn normalize_trigger(v: f32) -> f32 {
    ((v + 1.0) / 2.0).clamp(0.0, 1.0)
}
