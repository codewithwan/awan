//! Shared pixel colour for the block renderers: a cell's palette role is
//! resolved to an SGR code, converted to RGB, then dimmed by the glyph's
//! block density so `░` reads fainter than `█`.

use crate::character::Character;
use crate::grid::Cell;

/// A pixel's colour, dimmed by its glyph's density; `None` when the cell is empty.
pub(crate) fn cell_rgb(cell: &Cell, ch: &Character) -> Option<(u8, u8, u8)> {
    if cell.glyph.is_empty() {
        return None;
    }
    let f = density(cell.glyph);
    let (r, g, b) = sgr_to_rgb(cell.color.sgr(ch));
    let s = |v: u8| (v as f32 * f) as u8;
    Some((s(r), s(g), s(b)))
}

/// The block glyph's coverage as a brightness factor (`░` fainter than `█`).
fn density(glyph: &str) -> f32 {
    match glyph.chars().next() {
        Some('█' | '▀' | '▄') => 1.0,
        Some('▓') => 0.75,
        Some('▒') => 0.55,
        Some('░') => 0.42,
        _ => 0.7, // decorative text (notes, dust, zZ)
    }
}

/// Parse an SGR foreground code (`38;2;r;g;b` or `38;5;n`) to RGB.
fn sgr_to_rgb(sgr: &str) -> (u8, u8, u8) {
    let mut it = sgr.split(';');
    match (it.next(), it.next()) {
        (Some("38"), Some("2")) => {
            let mut next = || it.next().and_then(|s| s.parse().ok()).unwrap_or(200);
            (next(), next(), next())
        }
        (Some("38"), Some("5")) => xterm256(it.next().and_then(|s| s.parse().ok()).unwrap_or(244)),
        _ => (200, 200, 200),
    }
}

/// Convert an xterm-256 index to RGB (the 6×6×6 cube and grayscale ramp).
fn xterm256(n: u8) -> (u8, u8, u8) {
    match n {
        16..=231 => {
            let n = n - 16;
            let c = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            (c(n / 36), c((n / 6) % 6), c(n % 6))
        }
        232..=255 => {
            let g = 8 + (n - 232) * 10;
            (g, g, g)
        }
        _ => (128, 128, 128),
    }
}
