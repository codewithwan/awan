//! Street skits: the crate bonk and the sudden sit-down doze.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{BANG_SPRITE, CRATE_FRAMES};

/// The knocked-away arc: up, over, one bounce, off-screen. `(x, y, variant)`.
pub(crate) const CRATE_FLIGHT: [(i32, i32, usize); 10] = [
    (22, 6, 1),
    (23, 5, 0),
    (24, 4, 1),
    (25, 4, 0),
    (26, 5, 1),
    (27, 6, 0),
    (28, 7, 1),
    (29, 8, 0),
    (30, 7, 1),
    (31, 8, 0),
];

pub(crate) const BONK_TICK: i32 = 24;

/// Maps a scene tick to the crate's position: sliding in from the right,
/// sitting at the contact point, then tumbling away after the bonk.
pub(crate) fn crate_at(k: i32) -> (i32, i32, usize, bool) {
    if k < BONK_TICK {
        return (33 - k / 2, 8, 0, true);
    }
    if k == BONK_TICK {
        return (21, 8, 0, true);
    }
    let f = (k - BONK_TICK - 1) as usize;
    if f < CRATE_FLIGHT.len() {
        let (x, y, v) = CRATE_FLIGHT[f];
        return (x, y, v, true);
    }
    (0, 0, 0, false)
}

pub(super) fn crate_bonk(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Walk,
        ..Pose::default()
    };
    let (x, y, v, draw) = crate_at(k);
    if draw {
        blit(grid, CRATE_FRAMES[v], x, y, Role::Crate);
    }
    if (18..BONK_TICK).contains(&k) {
        p.eyes = EyeMode::Right; // notices the crate coming
    }
    if (BONK_TICK..BONK_TICK + 4).contains(&k) {
        (p.eyes, p.mouth_open) = (EyeMode::Right, true);
    }
    if (BONK_TICK..BONK_TICK + 2).contains(&k) {
        p.dx = -1; // recoil
    }
    if (BONK_TICK..BONK_TICK + 3).contains(&k) {
        blit(grid, BANG_SPRITE, 22, 1, Role::Bang);
        grid.set(21, 6, "░░", Role::Spark);
        grid.set(20, 7, "▒▒", Role::Spark);
        grid.set(23, 7, "░░", Role::Spark);
    }
    p
}

pub(super) fn sit(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Sit,
        ..Pose::default()
    };
    if k < 2 {
        // plop dust
        grid.set(10, 11, "░░", Role::Dust);
        grid.set(21, 11, "░░", Role::Dust);
    }

    // Doze off: zzz drift up, then a groggy wake-up with rapid blinks.
    if (14..30).contains(&k) {
        p.eyes = EyeMode::Closed;
        if k >= 16 {
            grid.set(21, 5, "z ", Role::Dust);
        }
        if k >= 20 {
            grid.set(23, 4, "z ", Role::Dust);
        }
        if k >= 24 {
            grid.set(25, 3, "Z ", Role::Dust);
        }
    }
    match k {
        6..10 => p.eyes = EyeMode::Left, // glancing around before nodding off
        10..14 => p.eyes = EyeMode::Right,
        32 | 33 | 36 | 37 => p.eyes = EyeMode::Closed, // wake-up blink… blink
        38.. => p.legs = LegsMode::Still,              // back on his feet
        _ => {}
    }
    p
}
