use iced::{Color, Theme};

/// All semantic colors used by the controller canvas, resolved from the
/// active [`Theme`] at draw time.
///
/// # Usage
/// ```rust
/// let p = ControllerPalette::from_theme(theme);
/// // Every color decision is now a field access:
/// fill_rrect(frame, tl, w, h, r, p.body);
/// ```
///
/// To tweak colors, edit `dark()` or `light()` below — one place, one field.
#[derive(Debug, Clone, Copy)]
pub struct ControllerPalette {
    // ── Surface / structural ─────────────────────────────────────────────────
    /// Main controller body background.
    pub body: Color,
    /// Overall background used for app columns / panels.
    pub background: Color,
    /// Hand-grip extensions below the body.
    pub grip: Color,
    /// Shoulder bumper (LB / RB) at rest.
    pub bumper: Color,
    /// Generic idle element (D-pad, center buttons) at rest.
    pub idle: Color,
    /// Recessed well/track background (stick wells, trigger bar track).
    pub track: Color,

    // ── Text / decoration ────────────────────────────────────────────────────
    /// Labels drawn on canvas elements.
    pub label: Color,
    /// Analog-stick dot and ring while the stick is at rest.
    pub stick_dot: Color,

    // ── Interactive accent ────────────────────────────────────────────────────
    /// Highlight applied to every element that is currently pressed / active.
    pub accent: Color,

    // ── Face-button identity colors ───────────────────────────────────────────
    /// South button — A (Xbox) / Cross (PlayStation).
    pub face_a: Color,
    /// East button  — B (Xbox) / Circle (PlayStation).
    pub face_b: Color,
    /// West button  — X (Xbox) / Square (PlayStation).
    pub face_x: Color,
    /// North button — Y (Xbox) / Triangle (PlayStation).
    pub face_y: Color,
}

impl ControllerPalette {
    /// Derive the correct palette from the active iced [`Theme`].
    pub fn from_theme(theme: &Theme) -> Self {
        if theme.extended_palette().is_dark {
            Self::dark()
        } else {
            Self::light()
        }
    }

    // ── Dark palette ──────────────────────────────────────────────────────────

    fn dark() -> Self {
        Self {
            // Surfaces — darker cool purplish-grey scale
            body:      Color::from_rgb8(28,  28,  36),
            background: Color::from_rgb8(28, 28, 28),
            grip:      Color::from_rgb8(18,  18,  28),
            bumper:    Color::from_rgb8(42,  42,  55),
            idle:      Color::from_rgb8(48,  48,  60),
            track:     Color::from_rgb8(15,  15,  25),
            // Text / decoration
            label:     Color::from_rgb8(200, 200, 225),
            stick_dot: Color::from_rgb8(90,  90,  118),
            // Accent — vivid blue, consistent across themes
            accent:    Color::from_rgb8(80,  160, 255),
            // Face buttons — conventional cross-platform colors
            face_a:    Color::from_rgb8(0,   165, 75 ),
            face_b:    Color::from_rgb8(200, 30,  30 ),
            face_x:    Color::from_rgb8(30,  100, 220),
            face_y:    Color::from_rgb8(210, 170, 0  ),
        }
    }

    // ── Light palette ─────────────────────────────────────────────────────────

    fn light() -> Self {
        Self {
            // Surfaces — soft lavender-grey scale
            body:      Color::from_rgb8(200, 200, 215),
            background: Color::from_rgb8(245, 245, 250),
            grip:      Color::from_rgb8(183, 183, 203),
            bumper:    Color::from_rgb8(173, 173, 195),
            idle:      Color::from_rgb8(153, 153, 175),
            track:     Color::from_rgb8(155, 155, 180),
            // Text / decoration
            label:     Color::from_rgb8(45,  45,  68 ),
            stick_dot: Color::from_rgb8(100, 100, 132),
            // Accent — same vivid blue as dark theme
            accent:    Color::from_rgb8(80,  160, 255),
            // Face buttons — same identity colors, visible on light background
            face_a:    Color::from_rgb8(0,   165, 75 ),
            face_b:    Color::from_rgb8(200, 30,  30 ),
            face_x:    Color::from_rgb8(30,  100, 220),
            face_y:    Color::from_rgb8(210, 170, 0  ),
        }
    }
}
