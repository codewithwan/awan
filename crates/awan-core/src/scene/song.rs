//! The singing scene: he *walks* over to the right (no teleport), clearing the
//! left for a karaoke lyric panel, belts it out — mouth opening, notes drifting
//! — then walks back to centre. No mic, just vibes.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

const HOME: i32 = 11; // == MASCOT_HOME
pub(super) const DUR: i32 = 150;
/// How far right he stands while singing, and how long the walk on/off takes.
const SHIFT: i32 = 8;
const WALK: i32 = 16;

pub(super) fn sing(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let dx = if k < WALK {
        (k / 2).min(SHIFT) // stroll into place
    } else if k >= DUR - WALK {
        ((DUR - k).max(0) / 2).min(SHIFT) // stroll back to centre
    } else {
        SHIFT
    };
    let settled = dx == SHIFT;

    if settled {
        let y = 5 - (k / 2) % 6;
        grid.set(HOME + SHIFT + 6, y, "♪ ", Role::Butterfly);
    }
    Pose {
        dx,
        legs: if settled {
            LegsMode::Still
        } else {
            LegsMode::Walk
        },
        eyes: EyeMode::Happy,
        mouth_open: settled && k % 6 < 3,
        dy: if settled && (k / 3) % 2 == 0 { -1 } else { 0 },
        ..Pose::default()
    }
}
