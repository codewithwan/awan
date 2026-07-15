//! The singing scene: he steps to the right — clearing the left for a karaoke
//! lyric panel — and belts it out, mouth opening and closing, notes drifting
//! up beside him. No mic, just vibes.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

const HOME: i32 = 11; // == MASCOT_HOME
/// How far right he stands while singing, leaving the left for the lyrics.
const SHIFT: i32 = 6;

pub(super) fn sing(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    // Notes rise and fade beside him (on the right, over the empty space).
    let y = 5 - (k / 2) % 6;
    let x = HOME + SHIFT + if (k / 2) % 2 == 0 { -1 } else { 8 };
    grid.set(x, y, "♪ ", Role::Butterfly);
    Pose {
        dx: SHIFT,
        legs: LegsMode::Still,
        eyes: EyeMode::Happy,
        mouth_open: k % 6 < 3, // opening and closing to the beat
        dy: if (k / 3) % 2 == 0 { -1 } else { 0 },
        ..Pose::default()
    }
}
