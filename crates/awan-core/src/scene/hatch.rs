//! The hatch intro: on first run the buddy arrives in an egg — it wobbles,
//! cracks, the top pops off, and out he comes, blinking at the world.

use crate::character::Character;
use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{EGG, SHELL_BOTTOM, SHELL_LEFT, SHELL_RIGHT, SHELL_TOP};
use crate::stage::MASCOT_HOME;

pub(crate) const HATCH_TICKS: i32 = 42;
const POP_TICK: i32 = 18;

/// Draws the whole hatch frame (egg, shell, and the buddy once revealed).
pub(crate) fn hatch_frame(k: i32, grid: &mut Grid, ch: &Character) {
    let home = MASCOT_HOME;
    if k < POP_TICK {
        // The egg wobbles, then cracks spread.
        let wobble = if k >= 8 { (k / 2) % 2 } else { 0 };
        blit(grid, EGG, home + wobble, 6, Role::EyeWhite);
        if k >= 8 && k % 4 < 2 {
            grid.set(home - 1, 11, "░░", Role::Dust);
            grid.set(home + 10, 11, "░░", Role::Dust);
        }
        if k >= 10 {
            grid.set(home + wobble + 3, 8, "▓▓", Role::Eye); // crack
        }
        if k >= 12 {
            grid.set(home + wobble + 5, 9, "▓▓", Role::Eye);
        }
        if k >= 14 {
            grid.set(home + wobble + 6, 8, "▓▓", Role::Eye);
        }
        if k >= 16 {
            grid.set(home + wobble + 2, 9, "▓▓", Role::Eye);
        }
        return;
    }

    // POP — the buddy, revealed. He sits in the shell, then stands.
    let p = Pose {
        legs: match k {
            ..34 => LegsMode::Sit,
            34..40 => LegsMode::Still,
            _ => LegsMode::Walk,
        },
        eyes: match k {
            ..22 | 24 | 25 => EyeMode::Closed, // groggy… blink
            30..38 => EyeMode::Happy,
            _ => EyeMode::Auto,
        },
        dy: if k == 36 || k == 37 { -1 } else { 0 },
        mouth_open: k < 20,
        ..Pose::default()
    };
    blit(grid, &ch.mascot_rows(p, k), home, 6 + p.dy, Role::Body);

    if k < POP_TICK + 6 {
        // The top half of the shell sails up and away.
        blit(
            grid,
            SHELL_TOP,
            home,
            3 - (k - POP_TICK) * 2,
            Role::EyeWhite,
        );
    }
    if k < POP_TICK + 3 {
        grid.set(home + 1, 5, "░░", Role::Spark);
        grid.set(home + 8, 4, "▒▒", Role::Spark);
        grid.set(home + 4, 3, "░░", Role::Spark);
    }
    match k {
        ..28 => blit(grid, SHELL_BOTTOM, home, 10, Role::EyeWhite), // still in the shell
        28..34 => {
            // The bottom shell splits and the halves slide apart.
            let off = k - 28;
            blit(grid, SHELL_LEFT, home - off, 10, Role::EyeWhite);
            blit(grid, SHELL_RIGHT, home + 5 + off, 10, Role::EyeWhite);
        }
        _ => {}
    }
}
