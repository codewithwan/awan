//! The singing scene: he sways and belts it out — mouth opening and closing,
//! music notes drifting up on both sides — no mic, just vibes.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

const HOME: i32 = 11; // == MASCOT_HOME

pub(super) fn sing(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    // Notes rise and fade on alternating sides.
    let rise = k / 2;
    if (k / 2) % 2 == 0 {
        let y = 6 - (rise % 6);
        grid.set(HOME - 2, y, "♪ ", Role::Butterfly);
    } else {
        let y = 5 - (rise % 6);
        grid.set(HOME + 7, y, " ♪", Role::Butterfly);
    }
    Pose {
        legs: LegsMode::Walk, // a little step-sway so he stays lively
        eyes: EyeMode::Happy,
        mouth_open: k % 6 < 3, // opening and closing to the beat
        dy: if (k / 3) % 2 == 0 { -1 } else { 0 },
        ..Pose::default()
    }
}
