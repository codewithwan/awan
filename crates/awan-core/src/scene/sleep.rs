//! The sleepy scene: a big yawn, then he curls up and dozes with `z`s drifting
//! up, then blinks awake and stretches before moving on.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

const HOME: i32 = 11; // == MASCOT_HOME
pub(super) const DUR: i32 = 80;
const YAWN: i32 = 14; // getting sleepy
const WAKE: i32 = DUR - 14; // blinking awake

pub(super) fn sleep(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    if k < YAWN {
        // a drowsy yawn
        return Pose {
            legs: LegsMode::Still,
            eyes: EyeMode::Closed,
            mouth_open: k % 8 < 5,
            ..Pose::default()
        };
    }
    if k >= WAKE {
        // waking up, a little stretch
        return Pose {
            legs: LegsMode::Sit,
            eyes: EyeMode::Happy,
            dy: -((k - WAKE) % 2),
            ..Pose::default()
        };
    }
    // fast asleep — a trail of z's floats up
    let z = (k - YAWN) / 3;
    grid.set(HOME + 6, (5 - z % 5).max(0), "z ", Role::Dust);
    Pose {
        legs: LegsMode::Sit,
        eyes: EyeMode::Closed,
        ..Pose::default()
    }
}
