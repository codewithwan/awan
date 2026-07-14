//! Compact, seam-free rendering: two vertical pixels are packed into one
//! terminal cell with the upper-half block `▀` (foreground = top pixel,
//! background = bottom pixel). Because the whole cell rect — including the
//! font's line spacing — is painted, there are no gaps between rows on any
//! terminal (macOS Terminal.app included), and the character is half as tall.

use std::fmt::Write;

use crate::character::Character;
use crate::color::cell_rgb;
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
            match (cell_rgb(&top[x], ch), cell_rgb(&bot[x], ch)) {
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
