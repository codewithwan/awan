//! Terminal output: paints a composed grid as text and runs the show loop.
//!
//! Colored output is run-length encoded — an SGR code is emitted only when
//! the color actually changes along a row, roughly quartering the bytes per
//! frame versus coloring every cell.

use std::io::Write;
use std::time::Duration;

use crate::character::Character;
use crate::grid::{CANVAS_W, Grid};
use crate::palette::Role;
use crate::stage::Stage;

pub(crate) fn render(grid: &Grid, ch: &Character, color: bool) -> String {
    let mut b = String::with_capacity(2048);
    for row in grid.rows() {
        b.push_str("  ");
        let mut active: Option<&str> = None;
        for cell in row {
            if cell.glyph.is_empty() {
                b.push_str("  ");
                continue;
            }
            if color {
                let code = cell.color.sgr(ch);
                if active != Some(code) {
                    b.push_str("\x1b[");
                    b.push_str(code);
                    b.push('m');
                    active = Some(code);
                }
            }
            b.push_str(cell.glyph);
        }
        if active.is_some() {
            b.push_str("\x1b[0m");
        }
        b.push('\n');
    }
    b.push_str("  ");
    if color {
        b.push_str("\x1b[38;5;242m");
    }
    for _ in 0..CANVAS_W * 2 {
        b.push('▔');
    }
    if color {
        b.push_str("\x1b[0m");
    }
    b
}

impl Stage {
    /// Run the show: the intro once, then `cycles` full loops — or forever
    /// when `cycles <= 0`. An optional caption ("baking…") animates under
    /// the canvas. Checks `stop` between frames so the caller can end the
    /// show (e.g. on Ctrl+C) with the cursor restored cleanly.
    pub fn play<W: Write>(
        &self,
        w: &mut W,
        color: bool,
        cycles: i32,
        delay: Duration,
        caption: Option<&str>,
        stop: &dyn Fn() -> bool,
    ) {
        let infinite = cycles <= 0;
        let total = self.intro_ticks() + cycles * self.cycle_ticks();

        let _ = write!(w, "\x1b[?25l\x1b[2J");
        let mut t = 0;
        while infinite || t < total {
            if stop() {
                break;
            }
            let _ = write!(w, "\x1b[H");
            for line in self.frame(t, color).split('\n') {
                let _ = writeln!(w, "{line}\x1b[K");
            }
            if let Some(label) = caption {
                let dots = ".".repeat(((t / 6) % 4) as usize);
                if color {
                    let _ = write!(w, "\x1b[{}m", Role::Dust.sgr(&self.character));
                }
                let _ = write!(w, "  {} {label}{dots}", self.character.name);
                if color {
                    let _ = write!(w, "\x1b[0m");
                }
                let _ = writeln!(w, "\x1b[K");
            }
            let _ = w.flush();
            if !delay.is_zero() {
                std::thread::sleep(delay);
            }
            t += 1;
        }
        let _ = writeln!(w, "\x1b[?25h");
    }
}
