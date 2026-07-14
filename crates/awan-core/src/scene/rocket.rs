//! The grand finale: he hammers a little rocket together, launches it,
//! watches it explode mid-air, and stands there charred, blinking. Ship it
//! again.

use crate::grid::{CANVAS_H, GROUND_Y, Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::ROCKET_ROWS;

const ROCKET_X: i32 = 24;

/// How many rocket rows exist at build tick `k` (bottom-up).
pub(crate) fn build_stage(k: i32) -> usize {
    match k {
        ..8 => 1,
        8..18 => 2,
        18..28 => 4,
        _ => ROCKET_ROWS.len(),
    }
}

pub(super) fn build(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Right,
        ..Pose::default()
    };

    let n = build_stage(k);
    blit(
        grid,
        &ROCKET_ROWS[ROCKET_ROWS.len() - n..],
        ROCKET_X,
        CANVAS_H - n as i32,
        Role::Rocket,
    );

    if k < 30 {
        // hammer time: swing up… and down — clank!
        if k % 4 < 2 {
            grid.set(21, 7, "▓▓", Role::Tool);
            grid.set(22, 6, "██", Role::Tool);
        } else {
            grid.set(21, 8, "▓▓", Role::Tool);
            grid.set(22, 9, "██", Role::Tool);
            if k % 4 == 2 {
                grid.set(23, 9, "▒▒", Role::Spark);
            }
        }
        return p;
    }

    // Step back and admire the work.
    p.dx = -1;
    if k >= 34 {
        p.eyes = EyeMode::Happy;
    }
    if k == 36 || k == 37 {
        p.dy = -1; // proud hop
    }
    p
}

/// What's left of the rocket, raining down after the blast. `(x, y)`.
const DEBRIS: [(i32, i32); 4] = [(23, 4), (27, 3), (25, 5), (28, 5)];

fn draw_blast(grid: &mut Grid, f: i32) {
    match f {
        0 => {
            grid.set(25, 2, "▒▒", Role::Bang);
            grid.set(26, 2, "▒▒", Role::Spark);
            grid.set(24, 3, "░░", Role::Spark);
            grid.set(27, 3, "░░", Role::Bang);
        }
        1 => {
            grid.set(24, 1, "░░", Role::Bang);
            grid.set(27, 1, "▒▒", Role::Spark);
            grid.set(23, 2, "▒▒", Role::Bang);
            grid.set(28, 2, "░░", Role::Spark);
            grid.set(25, 3, "▒▒", Role::Spark);
            grid.set(26, 4, "░░", Role::Bang);
        }
        _ => {
            grid.set(25, 0, "░░", Role::Dust);
            grid.set(23, 1, "░░", Role::Dust);
            grid.set(28, 1, "░░", Role::Dust);
            grid.set(26, 3, "░░", Role::Dust);
        }
    }
}

pub(crate) fn launch(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Right,
        ..Pose::default()
    };

    match k {
        ..8 => {
            // engine rumble — the rocket shakes on the pad
            let jx = if k >= 4 && k % 2 == 1 { 1 } else { 0 };
            blit(grid, ROCKET_ROWS, ROCKET_X + jx, 7, Role::Rocket);
            if k >= 4 {
                grid.set(23, 11, "░░", Role::Dust);
                grid.set(28, 11, "░░", Role::Dust);
            }
            if k >= 6 {
                p.dx = -1; // step back
            }
        }
        8..13 => {
            // liftoff!
            let y = 7 - (k - 7);
            blit(grid, ROCKET_ROWS, ROCKET_X, y, Role::Rocket);
            grid.set(25, y + 5, "▒▒", Role::Flame);
            grid.set(26, y + 5, "░░", Role::Flame);
            grid.set(25 + k % 2, y + 6, "░░", Role::Flame);
            grid.set(23 + (k % 2) * 5, 11, "░░", Role::Dust); // pad smoke
            p.dx = -1;
        }
        13..16 => {
            // …BOOM
            draw_blast(grid, k - 13);
            (p.dx, p.mouth_open) = (-1, true);
            p.charred = k >= 14;
        }
        16..43 => {
            // charred, frozen, blinking it off
            (p.dx, p.charred) = (-1, true);
            if k < 20 {
                // debris rains down
                for &(x, y0) in &DEBRIS {
                    let y = y0 + (k - 16);
                    if y < GROUND_Y {
                        grid.set(x, y, "▓▓", Role::Crate);
                    }
                }
            }
            if k < 26 {
                p.mouth_open = true;
            }
            p.eyes = EyeMode::Auto;
            if let 20 | 21 | 24 | 25 | 30 | 31 = k {
                p.eyes = EyeMode::Closed; // blink… blink-blink
            }
            if k % 4 < 2 {
                // smoke wisps rising off him
                grid.set(14, 5, "░░", Role::Dust);
            } else {
                grid.set(17, 4, "░░", Role::Dust);
            }
        }
        _ => {
            // shakes the soot off and moves on
            if k < 46 {
                p.dx = -1 + (k % 2) * 2; // shake
                grid.set(9, 9, "░░", Role::Dust);
                grid.set(21, 8, "░░", Role::Dust);
            }
            if k >= 47 {
                p.legs = LegsMode::Walk;
            }
        }
    }
    p
}
