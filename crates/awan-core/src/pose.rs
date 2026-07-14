//! Poses — everything a scene decides about the character for one frame.
//!
//! Ported 1:1 from the original Go engine.

/// Eye expression for a single frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EyeMode {
    /// Open, with an automatic idle blink handled by the engine.
    #[default]
    Auto,
    /// Closed lids (blink, sleep).
    Closed,
    /// Glancing left.
    Left,
    /// Glancing right.
    Right,
    /// Happy closed eyes (`^^`).
    Happy,
}

/// Leg state for a single frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegsMode {
    /// Walking cycle (animated).
    #[default]
    Walk,
    /// Standing still.
    Still,
    /// Plopped down on the ground, legs tucked.
    Sit,
}

/// Everything a scene decides about the character for one frame.
#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    /// Horizontal offset from the character's home position, in pixels.
    pub dx: i32,
    /// Vertical offset (negative = hop), in pixels.
    pub dy: i32,
    /// Leg state.
    pub legs: LegsMode,
    /// Eye expression.
    pub eyes: EyeMode,
    /// Startled open mouth (`O`).
    pub mouth_open: bool,
    /// Covered in soot after the rocket explosion, with wide white eyes.
    pub charred: bool,
}
