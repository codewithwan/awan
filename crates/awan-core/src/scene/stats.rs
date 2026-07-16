//! The stats readout: he strolls left, a little terminal window opens beside
//! him, and the numbers *print* into it line by line — which is what a terminal
//! character should do. Then the window closes and he strolls back, so the beat
//! drops into any reel.
//!
//! The engine draws the *window*; the renderer prints the lines using
//! [`PANEL`] and [`chars_at`], which says how much of each line has typed out.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

pub(super) const DUR: i32 = 150;
const SHIFT: i32 = 10; // how far left he stands
/// A cell every other tick, so the stroll lasts exactly twice the distance.
const WALK: i32 = SHIFT * 2;
const BACK: i32 = DUR - SHIFT * 2;
const OPEN: i32 = 22; // the window unfolds
const GROW: i32 = 6; // unfold/fold length
const FIRST: i32 = 32; // first line starts printing
const STEP: i32 = 14; // gap between lines
const TYPE: i32 = 10; // ticks to type one line
const CLOSE: i32 = 120; // the window folds away

/// How many lines the readout prints.
pub const SLOTS: usize = 5;
/// The window, in cells: `(x, y, w, h)`.
pub const PANEL: (i32, i32, i32, i32) = (12, 1, 20, 10);

/// The window's rectangle at tick `k` — it unfolds from the middle and folds
/// back — or `None` while it's shut.
pub fn panel_at(k: i32) -> Option<(i32, i32, i32, i32)> {
    let (x, y, w, h) = PANEL;
    let pct = match k {
        k if k < OPEN => 0,
        k if k < OPEN + GROW => 100 * (k - OPEN) / GROW,
        k if k < CLOSE => 100,
        k if k < CLOSE + GROW => 100 - 100 * (k - CLOSE) / GROW,
        _ => 0,
    };
    if pct <= 0 {
        return None;
    }
    let ch = (h * pct / 100).max(1);
    Some((x, y + (h - ch) / 2, w, ch))
}

/// How many characters of line `i` have printed at tick `k`. Lines type out one
/// after another, and clear when the window folds away.
pub fn chars_at(k: i32, i: usize) -> usize {
    let start = FIRST + i as i32 * STEP;
    if k < start || k >= CLOSE {
        return 0;
    }
    ((k - start) * 32 / TYPE).max(0) as usize
}

/// True while line `i` is still typing — the renderer parks the cursor there.
pub fn typing(k: i32, i: usize) -> bool {
    let start = FIRST + i as i32 * STEP;
    (start..start + TYPE).contains(&k)
}

/// The window: a soft frame with three little traffic lights in the title bar,
/// around an interior wiped back to the page's own dark background.
fn window(grid: &mut Grid, x: i32, y: i32, w: i32, h: i32, full: bool) {
    // wipe the interior so clouds never drift through the terminal
    for cy in y + 1..y + h - 1 {
        for cx in x + 1..x + w - 1 {
            grid.set(cx, cy, "", Role::Body);
        }
    }
    for cx in x..x + w {
        grid.set(cx, y, "██", Role::Tool);
        grid.set(cx, y + h - 1, "▒▒", Role::Tool);
    }
    for cy in y + 1..y + h - 1 {
        grid.set(x, cy, "▒▒", Role::Tool);
        grid.set(x + w - 1, cy, "▒▒", Role::Tool);
    }
    if full {
        for (i, role) in [Role::Bang, Role::Spark, Role::Gem].iter().enumerate() {
            grid.set(x + 1 + i as i32, y, "██", *role);
        }
    }
}

pub(super) fn stats(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    if let Some((x, y, w, h)) = panel_at(k) {
        window(grid, x, y, w, h, h == PANEL.3);
    }

    let dx = if k < WALK {
        -(k / 2).min(SHIFT)
    } else if k >= BACK {
        -((DUR - k).max(0) / 2).min(SHIFT)
    } else {
        -SHIFT
    };
    let working = (FIRST..CLOSE).contains(&k);
    Pose {
        dx,
        // a small bob while he taps away at it
        dy: if working && (k / 3) % 4 == 0 { -1 } else { 0 },
        legs: if dx == -SHIFT {
            LegsMode::Still
        } else {
            LegsMode::Walk
        },
        eyes: EyeMode::Right, // reading his own output
        ..Pose::default()
    }
}
