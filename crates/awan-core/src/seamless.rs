//! Seam-free full-size rendering: same height as the default block view, but
//! every pixel is painted as a solid background rectangle so the font's line
//! spacing is filled too — no gaps between rows on any terminal — while the
//! character keeps its normal proportions. The block dither of `▓`/`░` becomes
//! a flat shade instead. Always coloured.

use std::fmt::Write;

use crate::character::Character;
use crate::color::cell_rgb;
use crate::grid::{CANVAS_W, Grid};

/// Render the grid full height, painting each pixel as a bg-color rectangle.
pub(crate) fn render(grid: &Grid, ch: &Character) -> String {
    let mut b = String::with_capacity(4096);
    for row in grid.rows() {
        b.push_str("  ");
        for cell in row {
            match cell_rgb(cell, ch) {
                None => b.push_str("  "),
                Some((r, g, bl)) => {
                    let _ = write!(b, "\x1b[48;2;{r};{g};{bl}m  \x1b[0m");
                }
            }
        }
        b.push('\n');
    }
    let _ = write!(b, "  \x1b[38;5;242m");
    for _ in 0..CANVAS_W * 2 {
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
    fn seamless_is_full_height_bg_filled_and_mono_unaffected() {
        let big = Stage::show(Character::default()).frame(100, true);
        let seamless = Stage::show(Character::default())
            .with_size(Size::Seamless)
            .frame(100, true);
        // Same number of rows as the default look — proportions are preserved.
        assert_eq!(
            seamless.lines().count(),
            big.lines().count(),
            "seamless keeps full height"
        );
        assert!(
            seamless.contains("48;2"),
            "paints cell backgrounds (seam-free)"
        );

        // Colour-only: mono falls back to the full block render (Go parity holds).
        let mono = |s: Stage| s.frame(100, false);
        assert_eq!(
            mono(Stage::show(Character::default())),
            mono(Stage::show(Character::default()).with_size(Size::Seamless)),
        );
    }
}
