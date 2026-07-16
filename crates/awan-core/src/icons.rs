//! Tiny 8×8 pixel icons a renderer can draw beside a line. Each `u8` is a row;
//! bit `1 << col` is the pixel at column `col` (0 = left), matching font8x8's
//! byte order so they rasterise with the same routine.
//!
//! They live in the engine rather than next to one renderer because there are
//! two now — the GIF encoder and the browser preview — and a preview that
//! draws its own idea of these icons is a preview that lies.

pub struct Icon(pub [u8; 8]);

/// A gem/marker for the name line.
pub const DIAMOND: Icon = Icon([0x08, 0x1C, 0x3E, 0x7F, 0x3E, 0x1C, 0x08, 0x00]);
/// A briefcase for the role.
pub const BRIEFCASE: Icon = Icon([0x3C, 0x7E, 0xFF, 0xFF, 0xC3, 0xFF, 0xFF, 0x00]);
/// A map pin for the location.
pub const PIN: Icon = Icon([0x3C, 0x7E, 0xC3, 0xC3, 0x7E, 0x3C, 0x18, 0x10]);
/// A `>_` prompt for the tech stack.
pub const CODE: Icon = Icon([0x00, 0x01, 0x02, 0x04, 0x02, 0x01, 0x7C, 0x00]);
/// A globe for the site/handle.
pub const GLOBE: Icon = Icon([0x3C, 0x42, 0x99, 0xA5, 0xA5, 0x99, 0x42, 0x3C]);
/// A heart for the greeting and sign-off.
pub const HEART: Icon = Icon([0x36, 0x7F, 0x7F, 0x3E, 0x1C, 0x08, 0x00, 0x00]);
/// A sparkle/star for the hobby line.
pub const STAR: Icon = Icon([0x08, 0x2A, 0x1C, 0x7F, 0x1C, 0x2A, 0x08, 0x00]);
/// A flame for the coding-streak badge.
pub const FIRE: Icon = Icon([0x08, 0x0C, 0x1C, 0x3E, 0x7F, 0x7F, 0x3E, 0x00]);
