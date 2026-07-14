//! Compact, seam-free rendering: two vertical pixels are packed into one
//! terminal cell with the upper-half block `▀` (foreground = top pixel,
//! background = bottom pixel). Because the whole cell rect — including the
//! font's line spacing — is painted, there are no gaps between rows on any
//! terminal (macOS Terminal.app included), and the character is half as tall.

use std::fmt::Write;

use crate::character::Character;
use crate::grid::{Cell, Grid};

/// Render the grid at half height using bg-color half-blocks. Always coloured.
pub(crate) fn render(grid: &Grid, ch: &Character) -> String {
    let rows: Vec<&[Cell]> = grid.rows().collect();
    let width = rows.first().map_or(0, |r| r.len());
    let mut b = String::with_capacity(2048);
    for pair in rows.chunks_exact(2) {
        b.push_str("  ");
        let (top, bot) = (pair[0], pair[1]);
        for x in 0..width {
            match (rgb(&top[x], ch), rgb(&bot[x], ch)) {
                (None, None) => b.push_str("  "),
                (Some(t), None) => {
                    let _ = write!(b, "\x1b[38;2;{};{};{}m▀▀\x1b[0m", t.0, t.1, t.2);
                }
                (None, Some(l)) => {
                    let _ = write!(b, "\x1b[38;2;{};{};{}m▄▄\x1b[0m", l.0, l.1, l.2);
                }
                (Some(t), Some(l)) => {
                    let _ = write!(
                        b,
                        "\x1b[38;2;{};{};{};48;2;{};{};{}m▀▀\x1b[0m",
                        t.0, t.1, t.2, l.0, l.1, l.2
                    );
                }
            }
        }
        b.push('\n');
    }
    let _ = write!(b, "  \x1b[38;5;242m");
    for _ in 0..width * 2 {
        b.push('▔');
    }
    b.push_str("\x1b[0m");
    b
}

/// A pixel's colour, dimmed by its glyph's density; `None` when empty.
fn rgb(cell: &Cell, ch: &Character) -> Option<(u8, u8, u8)> {
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

#[cfg(test)]
mod tests {
    use crate::character::Character;
    use crate::stage::{Size, Stage};

    #[test]
    fn compact_is_half_height_seam_free_and_mono_unaffected() {
        let big = Stage::show(Character::default()).frame(100, true);
        let compact = Stage::show(Character::default())
            .with_size(Size::Compact)
            .frame(100, true);
        assert!(
            compact.lines().count() < big.lines().count(),
            "compact is shorter"
        );
        assert!(compact.contains('▀'), "uses half-blocks");
        assert!(
            compact.contains("48;2"),
            "paints cell backgrounds (seam-free)"
        );

        // Compact only changes coloured output; mono falls back to the full render.
        let mono = |s: Stage| s.frame(100, false);
        assert_eq!(
            mono(Stage::show(Character::default())),
            mono(Stage::show(Character::default()).with_size(Size::Compact)),
        );
    }
}
